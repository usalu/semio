#nullable enable

using System;
using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;
using Semio.Store;
using Formatting = Newtonsoft.Json.Formatting;

namespace Semio;

/// <summary>Kit diff validation, <see cref="AreKitsEqual"/> (normalized JSON), and canonical <see cref="KitDiff"/> JSON comparison.</summary>
public static class SemioDiff
{
    public static DesignChange GetDesignChange(Design before, Design after, string? author = null, DateTime? time = null)
    {
        var forward = Design.GetDesignDiff(before, after) ?? new DesignDiff();
        var backward = Design.GetDesignDiff(after, before) ?? new DesignDiff();
        return new DesignChange { Forward = forward, Backward = backward, Author = author, Time = time, Before = before, After = after };
    }

    public static bool AreKitsEqual(Kit a, Kit b) => StoreKitIO.KitsEqual(a, b);

    public static KitDiffValidationResult ValidateKitDiff(Kit kit, KitDiff diff, bool heal = false)
    {
        var ctx = new KitDiffValidationContext { Heal = heal };
        var json = KitDiffValidationJson;
        var kitObj = JObject.Parse(JsonConvert.SerializeObject(kit, json));
        var diffObj = JObject.Parse(JsonConvert.SerializeObject(diff, json));
        JObject? outDiff = heal ? (JObject)diffObj.DeepClone() : null;
        var refs = RefSets.FromKit(kitObj);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "types", "type", "types", null, refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "designs", "design", "designs",
            (c, km, item, dm, p, r) => ValidateDesignDiffNested(c, km, item, dm, p, r), refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "tags", "tag", "tags", null, refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "concepts", "concept", "concepts", null, refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "ports", "port", "ports", null, refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "qualities", "quality", "qualities", null, refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "files", "file", "files", null, refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "folders", "folder", "folders", null, refs);
        RunTopLevelIdCollection(ctx, kitObj, diffObj, outDiff, heal, "authors", "author", "authors", null, refs);
        if (diffObj["attributes"] is JObject attrPart)
        {
            var baseAttrs = kitObj["attributes"] as JArray;
            _ = ValidateIdCollectionDiff(ctx, "kit.attributes", "attribute", baseAttrs, attrPart, null);
        }
        var ok = ctx.Errors.Count == 0;
        KitDiff? diffOut = null;
        if (heal && outDiff != null)
        {
            if (outDiff.Properties().Any())
                diffOut = JsonConvert.DeserializeObject<KitDiff>(outDiff.ToString(Formatting.None), json);
            else
                diffOut = new KitDiff();
        }
        return new KitDiffValidationResult
        {
            Ok = ok,
            Errors = ctx.Errors,
            Warnings = ctx.Warnings,
            Diff = diffOut
        };
    }

    private static readonly JsonSerializerSettings KitDiffValidationJson = new()
    {
        ContractResolver = new CamelCasePropertyNamesContractResolver(),
        NullValueHandling = NullValueHandling.Include
    };

    private sealed class KitDiffValidationContext
    {
        public bool Heal { get; init; }
        public List<KitDiffValidationNote> Errors { get; } = new();
        public List<KitDiffValidationNote> Warnings { get; } = new();
        public void Push(string kind, string code, string message)
        {
            var n = new KitDiffValidationNote { Code = string.IsNullOrEmpty(code) ? null : code, Message = message };
            if (kind == "errors") Errors.Add(n); else Warnings.Add(n);
        }
    }

    private readonly struct RefSets
    {
        public HashSet<string> TypeIds { get; }
        public HashSet<string> DesignIds { get; }
        public HashSet<string> AuthorIds { get; }

        private RefSets(HashSet<string> t, HashSet<string> d, HashSet<string> a)
        {
            TypeIds = t; DesignIds = d; AuthorIds = a;
        }

        public static RefSets FromKit(JObject kitObj) => new(
            IdSetFromEntities(kitObj["types"]),
            IdSetFromEntities(kitObj["designs"]),
            IdSetFromEntities(kitObj["authors"]));
    }

    private static HashSet<string> IdSetFromEntities(JToken? v)
    {
        var s = new HashSet<string>();
        if (v is not JArray arr) return s;
        foreach (var x in arr)
            if (x is JObject o && o["id"]?.Value<string>() is { } g && !string.IsNullOrEmpty(g))
                s.Add(g);
        return s;
    }

    private static List<JObject> ToJObjectList(JArray? arr)
    {
        var list = new List<JObject>();
        if (arr == null) return list;
        foreach (var x in arr)
            if (x is JObject o) list.Add(o);
        return list;
    }

    private static JArray? DiffUpdatesArray(JObject raw) => raw["updated"] as JArray ?? raw["modified"] as JArray;

    private static bool KitDiffDeepEqual(JToken a, JToken b) => JToken.DeepEquals(a, b);

    private delegate void DesignNestedHandler(KitDiffValidationContext ctx, JObject kitMap, JObject designItem, JObject? diffMap, string path, RefSets refs);

    private static void RunTopLevelIdCollection(KitDiffValidationContext ctx, JObject kitObj, JObject diffObj, JObject? outDiff, bool heal,
        string key, string idKey, string kitArrayKey, DesignNestedHandler? onDesign, RefSets refs)
    {
        if (diffObj[key] is not JObject part) return;
        var baseArr = kitObj[kitArrayKey] as JArray;
        var fixedObj = ValidateIdCollectionDiff(ctx, key, idKey, baseArr, part,
            onDesign != null ? (c, item, dm, p) => onDesign(c, kitObj, item, dm, p, refs) : null);
        if (!heal || outDiff == null) return;
        if (fixedObj != null && fixedObj.Properties().Any()) outDiff[key] = fixedObj;
        else outDiff.Remove(key);
    }

    private static JArray? FilterUpdatesById(JArray updates, string idKey, string gid)
    {
        var na = new JArray();
        foreach (var u in updates)
        {
            if (u is not JObject um) { na.Add(u); continue; }
            if (um[idKey] is JObject idObj && idObj["id"]?.Value<string>() == gid) continue;
            na.Add(u);
        }
        return na;
    }

    private static JObject? ValidateIdCollectionDiff(KitDiffValidationContext ctx, string path, string idKey, JArray? baseEntities, JObject? raw,
        Action<KitDiffValidationContext, JObject, JObject?, string>? onUpdated)
    {
        if (raw == null) return null;
        var baseBy = new Dictionary<string, JObject>();
        foreach (var ent in ToJObjectList(baseEntities))
            if (ent["id"]?.Value<string>() is { } bg && !string.IsNullOrEmpty(bg))
                baseBy[bg] = ent;
        var removedSet = new HashSet<string>();
        if (raw["removed"] is JArray remArr)
            foreach (var r in remArr)
                if (r is JObject rm && rm["id"]?.Value<string>() is { } rg)
                    removedSet.Add(rg);
        var afterRemove = new HashSet<string>();
        foreach (var g in baseBy.Keys)
            if (!removedSet.Contains(g)) afterRemove.Add(g);
        JArray? hRem = null, hUpd = null, hAdd = null;
        if (ctx.Heal)
        {
            if (raw["removed"] is JArray r0) hRem = (JArray)r0.DeepClone();
            if (DiffUpdatesArray(raw) is JArray u0) hUpd = (JArray)u0.DeepClone();
            if (raw["added"] is JArray a0) hAdd = (JArray)a0.DeepClone();
        }
        if (raw["removed"] is JArray removedTok)
            foreach (var r in removedTok)
                if (r is JObject rm)
                {
                    var rg = rm["id"]?.Value<string>() ?? "";
                    if (!baseBy.ContainsKey(rg))
                    {
                        ctx.Push("warnings", "kitdiff.remove.missing-target", $"{path}: remove references missing {idKey} {rg}");
                        if (hRem != null)
                        {
                            var nr = new JArray();
                            foreach (var x in hRem)
                                if (x is JObject xm && xm["id"]?.Value<string>() == rg) continue;
                                else nr.Add(x);
                            hRem = nr;
                        }
                    }
                }
        var addBy = new Dictionary<string, JObject>();
        if (raw["added"] is JArray addArr)
            foreach (var a in addArr)
                if (a is JObject am && am["id"]?.Value<string>() is { } ag)
                    addBy[ag] = am;
        if (raw["removed"] is JArray removedTok2)
            foreach (var r in removedTok2)
                if (r is JObject rm)
                {
                    var rg = rm["id"]?.Value<string>() ?? "";
                    if (baseBy.TryGetValue(rg, out var orig) && addBy.TryGetValue(rg, out var add) && KitDiffDeepEqual(orig, add))
                    {
                        ctx.Push("warnings", "kitdiff.cycle.noop-restore", $"{path}: removed and re-added {idKey} {rg} are deeply equal (no effective change)");
                        if (ctx.Heal)
                        {
                            if (hRem != null)
                            {
                                var nr = new JArray();
                                foreach (var x in hRem)
                                    if (x is JObject xm && xm["id"]?.Value<string>() == rg) continue;
                                    else nr.Add(x);
                                hRem = nr;
                            }
                            if (hAdd != null)
                            {
                                var na = new JArray();
                                foreach (var x in hAdd)
                                    if (x is JObject xm && xm["id"]?.Value<string>() == rg) continue;
                                    else na.Add(x);
                                hAdd = na;
                            }
                        }
                    }
                }
        var seenAdd = new HashSet<string>();
        if (raw["added"] is JArray addedTok)
            foreach (var a in addedTok)
                if (a is JObject am)
                {
                    var ag = am["id"]?.Value<string>() ?? "";
                    if (seenAdd.Contains(ag))
                    {
                        ctx.Push("errors", "kitdiff.add.duplicate-in-diff", $"{path}: duplicate added {idKey} id {ag}");
                        if (hAdd != null)
                        {
                            var first = true;
                            var na = new JArray();
                            foreach (var x in hAdd)
                            {
                                if (x is JObject xm && xm["id"]?.Value<string>() == ag)
                                {
                                    if (first) { na.Add(x); first = false; }
                                    continue;
                                }
                                na.Add(x);
                            }
                            hAdd = na;
                        }
                    }
                    seenAdd.Add(ag);
                    if (afterRemove.Contains(ag))
                    {
                        ctx.Push("errors", "kitdiff.add.duplicate-id", $"{path}: cannot add {idKey} {ag} that still exists after removes");
                        if (hAdd != null)
                        {
                            var na = new JArray();
                            foreach (var x in hAdd)
                                if (x is JObject xm && xm["id"]?.Value<string>() == ag) continue;
                                else na.Add(x);
                            hAdd = na;
                        }
                    }
                }
        if (DiffUpdatesArray(raw) is JArray updTok)
            foreach (var u in updTok)
                if (u is JObject um && um[idKey] is JObject idObj)
                {
                    var gid = idObj["id"]?.Value<string>() ?? "";
                    var p = $"{path}.{idKey}[{gid}]";
                    if (string.IsNullOrEmpty(gid))
                    {
                        ctx.Push("errors", "kitdiff.update.bad-id", $"{p}: missing {idKey} id");
                        if (hUpd != null) hUpd = FilterUpdatesById(hUpd, idKey, gid);
                        continue;
                    }
                    if (!afterRemove.Contains(gid))
                    {
                        ctx.Push("errors", "kitdiff.update.missing-target", $"{p}: update targets {idKey} not present after removes");
                        if (hUpd != null) hUpd = FilterUpdatesById(hUpd, idKey, gid);
                        continue;
                    }
                    if (!baseBy.TryGetValue(gid, out var item))
                    {
                        ctx.Push("errors", "kitdiff.update.missing-base", $"{p}: {idKey} not found in base kit");
                        if (hUpd != null) hUpd = FilterUpdatesById(hUpd, idKey, gid);
                        continue;
                    }
                    var dm = um["diff"] as JObject;
                    onUpdated?.Invoke(ctx, item, dm, p);
                }
        if (!ctx.Heal) return null;
        var o = new JObject();
        if (hRem is { Count: > 0 }) o["removed"] = hRem;
        if (hUpd is { Count: > 0 }) o["updated"] = hUpd;
        if (hAdd is { Count: > 0 }) o["added"] = hAdd;
        return o.Properties().Any() ? o : null;
    }

    private static void ValidateDesignDiffNested(KitDiffValidationContext ctx, JObject kitMap, JObject design, JObject? diff, string path, RefSets refs)
    {
        if (diff == null) return;
        if (diff["parent"] is JObject pObj && pObj["id"]?.Value<string>() is { } pg)
        {
            if (!string.IsNullOrEmpty(pg) && !refs.DesignIds.Contains(pg))
                ctx.Push("errors", "kitdiff.ref.design-parent-missing", $"{path}: parent design {pg} not in kit");
            if (design["id"]?.Value<string>() is { } dg && pg == dg)
                ctx.Push("errors", "kitdiff.ref.design-parent-self", $"{path}: design cannot be its own parent");
        }
        if (diff["authors"] != null)
        {
            var da = diff["authors"]!;
            if (da is JArray authArr)
                foreach (var a in authArr)
                    if (a is JObject am && am["id"]?.Value<string>() is { } g && !string.IsNullOrEmpty(g) && !refs.AuthorIds.Contains(g))
                        ctx.Push("errors", "kitdiff.ref.author-missing", $"{path}: author {g} not in kit");
            else if (da is JObject authObj)
            {
                var authBase = kitMap["authors"] as JArray;
                _ = ValidateIdCollectionDiff(ctx, $"{path}.authors", "author", authBase, authObj, null);
            }
        }
        if (diff["pieces"] is JObject pd)
        {
            var piecesBase = design["pieces"] as JArray;
            _ = ValidateIdCollectionDiff(ctx, $"{path}.pieces", "piece", piecesBase, pd, null);
            if (pd["added"] is JArray pAdd)
                foreach (var a in pAdd)
                    if (a is JObject am)
                    {
                        var tg = (am["type"] as JObject)?["id"]?.Value<string>() ?? "";
                        if (!string.IsNullOrEmpty(tg) && !refs.TypeIds.Contains(tg))
                            ctx.Push("errors", "kitdiff.ref.piece-type-missing", $"{path}.pieces.added: type {tg} not in kit");
                        var dsg = (am["design"] as JObject)?["id"]?.Value<string>() ?? "";
                        if (!string.IsNullOrEmpty(dsg) && !refs.DesignIds.Contains(dsg))
                            ctx.Push("errors", "kitdiff.ref.piece-design-missing", $"{path}.pieces.added: subdesign {dsg} not in kit");
                    }
        }
    }

    public static bool AreKitDiffsEqual(KitDiff a, KitDiff b)
    {
        var jsonA = Utility.Serialize(a);
        var jsonB = Utility.Serialize(b);
        if (jsonA == jsonB) return true;
        var tokenA = CanonicalizeToken(JToken.Parse(jsonA));
        var tokenB = CanonicalizeToken(JToken.Parse(jsonB));
        return JToken.DeepEquals(tokenA ?? JValue.CreateNull(), tokenB ?? JValue.CreateNull());
    }

    private static readonly HashSet<string> DefaultZeroKeys = new() { "x", "y", "z", "u", "v", "gap", "shift", "rise", "rotation", "turn", "tilt", "t" };
    private static readonly HashSet<string> DefaultFalseKeys = new() { "mandatory", "isHidden", "isLocked", "isAbstract", "virtual" };

    private static string? GetComparableId(JToken? token)
    {
        if (token == null || token.Type != JTokenType.Object) return null;
        var obj = (JObject)token;
        return (string?)(obj["id"]
            ?? (obj["type"] as JObject)?["id"]
            ?? (obj["design"] as JObject)?["id"]
            ?? (obj["piece"] as JObject)?["id"]
            ?? (obj["connection"] as JObject)?["id"]
            ?? (obj["representation"] as JObject)?["id"]
            ?? (obj["port"] as JObject)?["id"]
            ?? (obj["connector"] as JObject)?["id"]
            ?? (obj["prop"] as JObject)?["id"]
            ?? (obj["attribute"] as JObject)?["id"]);
    }

    private static JToken? CanonicalizeToken(JToken? token, string key = "")
    {
        if (token == null || token.Type == JTokenType.Null) return null;
        switch (token.Type)
        {
            case JTokenType.String:
                var s = token.Value<string>();
                return string.IsNullOrEmpty(s) ? null : token;
            case JTokenType.Integer:
            case JTokenType.Float:
                var n = token.Value<double>();
                return DefaultZeroKeys.Contains(key) && n == 0 ? null : token;
            case JTokenType.Boolean:
                var b = token.Value<bool>();
                return DefaultFalseKeys.Contains(key) && !b ? null : token;
            case JTokenType.Array:
                var items = token.Children()
                    .Select(c => CanonicalizeToken(c, key))
                    .Where(c => c != null)
                    .OrderBy(c => GetComparableId(c) ?? c!.ToString(Formatting.None))
                    .ToList();
                return items.Count > 0 ? new JArray(items) : null;
            case JTokenType.Object:
                var result = new JObject();
                foreach (var prop in ((JObject)token).Properties().OrderBy(p => p.Name))
                {
                    var val = CanonicalizeToken(prop.Value, prop.Name);
                    if (val != null) result[prop.Name] = val;
                }
                return result.Count > 0 ? result : null;
            default:
                return token;
        }
    }
}
