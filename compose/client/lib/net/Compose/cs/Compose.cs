#region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Core .NET library implementing the compose domain representation and serialization.

#endregion 🧲Header






#region 🔌Adapters
// Third-party imports MUST stay in this region; domain code uses port types below.
using System.Collections;
using System.Collections.Immutable;
using System.Drawing;
using System.Globalization;
using System.Net;
using System.Net.Http;
using System.Numerics;
using System.Reflection;
using System.Text;
using System.Xml;
using System.IO.Compression;
using FluentValidation;
using Compose.Store;
using System.Diagnostics;
using System.Net.Sockets;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;
using System.Runtime.Serialization;
using QuikGraph;
using QuikGraph.Algorithms;
using QuikGraph.Algorithms.ConnectedComponents;
using QuikGraph.Algorithms.Search;
using Refit;
using Svg;
using Svg.Transforms;
using UnitsNet;
using Formatting = Newtonsoft.Json.Formatting;
using SharpGLTF.Geometry;
using SharpGLTF.Geometry.VertexTypes;
using SharpGLTF.Materials;
using SharpGLTF.Scenes;
using GltfRepresentation = SharpGLTF.Schema2.ModelRoot;
using GltfNode = SharpGLTF.Schema2.Node;

#endregion 🔌Adapters

#region 🔌Ports
/// <summary>📜 JSON codec port implemented by Newtonsoft in 🔌Adapters.</summary>
public interface IComposeJsonCodec
{
    string Serialize(object value);
    T Deserialize<T>(string json);
    string SerializeRepresentation(object obj, string indent = "");
    T? DeserializeRepresentation<T>(string json);
    string SerializeCamelIndented(object value);
    object? ParseJsonRoot(string json);
    string SerializeKitDiffValidation(object value);
    T? DeserializeKitDiffValidation<T>(string json);
}

/// <summary>📜 Active JSON codec for domain serialization (defaults to Newtonsoft adapter).</summary>
public static class ComposeJson
{
    public static IComposeJsonCodec Codec { get; set; } = NewtonsoftComposeJsonCodec.Instance;
}

/// <summary>📜 Newtonsoft-backed JSON codec adapter.</summary>
public sealed class NewtonsoftComposeJsonCodec : IComposeJsonCodec
{
    public static readonly NewtonsoftComposeJsonCodec Instance = new();

    public string Serialize(object value) => JsonConvert.SerializeObject(value);

    public T Deserialize<T>(string json) => JsonConvert.DeserializeObject<T>(json)!;

    public string SerializeRepresentation(object obj, string indent = "")
    {
        var isTabbed = indent.StartsWith("\t");
        var formatting = string.IsNullOrEmpty(indent) ? Formatting.None : Formatting.Indented;
        var settings = new JsonSerializerSettings { ContractResolver = new ComposeContractResolver(), Formatting = formatting };
        if (formatting == Formatting.None) return JsonConvert.SerializeObject(obj, settings);
        var stringWriter = new StringWriter();
        using (var jsonWriter = new JsonTextWriter(stringWriter))
        {
            jsonWriter.Formatting = Formatting.Indented;
            jsonWriter.IndentChar = isTabbed ? '\t' : ' ';
            jsonWriter.Indentation = indent.Length;
            JsonSerializer.Create(settings).Serialize(jsonWriter, obj);
        }
        return stringWriter.ToString();
    }

    public T? DeserializeRepresentation<T>(string json) =>
        JsonConvert.DeserializeObject<T>(json, new JsonSerializerSettings
        {
            ContractResolver = new CamelCasePropertyNamesContractResolver(),
            ObjectCreationHandling = ObjectCreationHandling.Replace,
        });

    public string SerializeCamelIndented(object value) =>
        JsonConvert.SerializeObject(value, Formatting.Indented, new JsonSerializerSettings
        {
            ContractResolver = new CamelCasePropertyNamesContractResolver(),
        });

    public object? ParseJsonRoot(string json) => JsonConvert.DeserializeObject<JObject>(json);

    private static readonly JsonSerializerSettings KitDiffValidationJson = new()
    {
        ContractResolver = new CamelCasePropertyNamesContractResolver(),
        NullValueHandling = NullValueHandling.Include,
    };

    public string SerializeKitDiffValidation(object value) =>
        JsonConvert.SerializeObject(value, KitDiffValidationJson);

    public T? DeserializeKitDiffValidation<T>(string json) =>
        JsonConvert.DeserializeObject<T>(json, KitDiffValidationJson);

    private sealed class ComposeContractResolver : CamelCasePropertyNamesContractResolver
    {
        protected override JsonProperty CreateProperty(MemberInfo member, MemberSerialization memberSerialization)
        {
            var property = base.CreateProperty(member, memberSerialization);
            var declaringType = member.DeclaringType;
            if (declaringType != null)
            {
                var shouldSerializeMethod = declaringType.GetMethod(
                    $"ShouldSerialize{member.Name}",
                    BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance);
                if (shouldSerializeMethod != null && shouldSerializeMethod.ReturnType == typeof(bool))
                {
                    property.ShouldSerialize = instance => (bool)(shouldSerializeMethod.Invoke(instance, null) ?? true);
                }
            }
            return property;
        }
    }
}
#endregion 🔌Ports






#region 🏠Namespace
// Implementations MUST reside in this namespace.
namespace Compose
{
#endregion 🏠Namespace






#region 🎞️Constants
// Consumers MUST use these shared constants for configuration.

public static class Constants
{
    public const string Name = "compose";
    public const string Email = "ueli@semio-tech.com";
    public const string Release = "r25.07-1";
    public const string EngineHost = "http://127.0.0.1";
    public const int EnginePort = 2507;
    public const string EngineAddress = "http://127.0.0.1:2507";
    public const int NameLengthLimit = 64;
    public const int IdLengthLimit = 128;
    public const int UrlLengthLimit = 1024;
    public const int UriLengthLimit = 2048;
    public const int ExpressionLengthLimit = 4096;
    public const int ValueLengthLimit = 512;
    public const int AttributesMax = 64;
    public const int QualityMax = 1024;
    public const int TagsMax = 8;
    public const int EntitysMax = 32;
    public const int TypesMax = 256;
    public const int PiecesMax = 512;
    public const int DesignsMax = 128;
    public const int KitsMax = 64;
    public const int DescriptionLengthLimit = 512;
    public const float Tolerance = 1e-5f;
}

public enum ImageExtensions
{
    png,
    jpg,
    jpeg,
    svg
}

public enum IconKind
{
    Logogram,
    Filepath,
    RemoteUrl
}

public enum EncodeMode
{
    Urlsafe,
    Base64,
    DictionaryOnly
}

public enum DiffStatus
{
    Unchanged,
    Added,
    Removed,
    Modified
}

#endregion 🎞️Constants






#region 📦Utilities
// Callers MUST use these utility functions for encoding and serialization.

public static class Utility
{
    public static string Normalize(string val) => string.IsNullOrEmpty(val) ? "" : val;

    public static float Jaccard(IEnumerable<string> a, IEnumerable<string> b)
    {
        var listA = a?.ToList() ?? new List<string>();
        var listB = b?.ToList() ?? new List<string>();

        if (listA.Count == 0 && listB.Count == 0) return 1f;

        var setA = new HashSet<string>(listA);
        var setB = new HashSet<string>(listB);
        var intersection = setA.Intersect(setB).Count();
        var union = setA.Union(setB).Count();

        if (union == 0) return 0f;
        return (float)intersection / union;
    }
    public static bool UriIsNotAbsoluteFilePath(string uri)
    {
        return !(Uri.IsWellFormedUriString(uri, UriKind.Relative) || uri.StartsWith("http"));
    }
    public static bool IsValidMime(string mime)
    {
        var validMimes = new List<string>
        {
            "entity/stl",
            "entity/obj",
            "entity/gltf-binary",
            "entity/gltf+json",
            "entity/vnd.3dm",
            "image/png",
            "image/jpeg",
            "image/svg+xml",
            "application/pdf",
            "application/zip",
            "application/json",
            "text/csv",
            "text/plain"
        };
        return validMimes.Contains(mime);
    }

    public static string ParseMimeFromUrl(string url)
    {
        var mimes = new Dictionary<string, string>
        {
            { ".stl", "entity/stl" },
            { ".obj", "entity/obj" },
            { ".glb", "entity/gltf-binary" },
            { ".gltf", "entity/gltf+json" },
            { ".3dm", "entity/vnd.3dm" },
            { ".png", "image/png" },
            { ".jpg", "image/jpeg" },
            { ".jpeg", "image/jpeg" },
            { ".svg", "image/svg+xml" },
            { ".pdf", "application/pdf" },
            { ".zip", "application/zip" },
            { ".json", "application/json" },
            { ".csv", "text/csv" },
            { ".txt", "text/plain" }
        };
        try
        {
            return mimes[Path.GetExtension(url)];
        }
        catch (KeyNotFoundException)
        {
            return "application/octet-stream";
        }
    }

    public static IconKind ParseIconKind(string icon)
    {
        if (icon.StartsWith("http")) return IconKind.RemoteUrl;
        try
        {
            var uri = new Uri(icon, UriKind.Relative);
            var ext = Path.GetExtension(icon);
            if (Enum.IsDefined(typeof(ImageExtensions), ext.ToLower().Substring(1)))
                return IconKind.Filepath;
        }
        catch (Exception) { }
        return IconKind.Logogram;
    }

    public static string DatastringFromUrl(string url)
    {
        string mime;
        byte[] content;
        if (url.StartsWith("http"))
        {
            using (var client = new HttpClient())
            {
                var response = client.GetAsync(url).Result;
                response.EnsureSuccessStatusCode();
                mime = response.Content.Headers.ContentType?.MediaType ?? "";
                content = response.Content.ReadAsByteArrayAsync().Result;
            }
        }
        else
        {
            var osAwareUrl = url.Replace("/", Path.DirectorySeparatorChar.ToString());
            content = System.IO.File.ReadAllBytes(osAwareUrl);
            mime = ParseMimeFromUrl(osAwareUrl);
        }
        return $"data:{mime};base64,{Convert.ToBase64String(content)}";
    }

    public static string ReadAndEncode(string filename) => $"data:{ParseMimeFromUrl(filename)};base64,{Convert.ToBase64String(System.IO.File.ReadAllBytes(filename))}";
    public static string Encode(string text, EncodeMode mode = EncodeMode.Urlsafe,
        Tuple<List<string>, List<string>>? replace = null)
    {
        var encoded = text;
        if (mode == EncodeMode.Urlsafe) encoded = Uri.EscapeDataString(text);
        if (mode == EncodeMode.Base64) encoded = Convert.ToBase64String(Encoding.UTF8.GetBytes(text));
        if (replace != null)
        {
            var keys = replace.Item1;
            var values = replace.Item2;
            if (keys.Count != values.Count) throw new ArgumentException("Both replace lists must have the same length.");
            for (var i = 0; i < keys.Count; i++)
            {
                var key = keys[i];
                var value = values[i];
                encoded = encoded.Replace(key, value);
            }
        }
        return encoded;
    }

    public static string Decode(string text, EncodeMode mode = EncodeMode.Urlsafe,
        Tuple<List<string>, List<string>>? replace = null)
    {
        var decoded = text;
        if (replace != null)
        {
            var keys = replace.Item1;
            var values = replace.Item2;
            if (keys.Count != values.Count) throw new ArgumentException("Both replace lists must have the same length.");
            for (var i = 0; i < keys.Count; i++)
            {
                var key = keys[i];
                var value = values[i];
                decoded = decoded.Replace(key, value);
            }
        }
        if (mode == EncodeMode.Urlsafe) decoded = Uri.UnescapeDataString(decoded);
        if (mode == EncodeMode.Base64) decoded = Encoding.UTF8.GetString(Convert.FromBase64String(decoded));
        return decoded;
    }

    public static string Serialize(object obj, string indent = "") => ComposeJson.Codec.SerializeRepresentation(obj, indent);

    public static T? Deserialize<T>(string json) => ComposeJson.Codec.DeserializeRepresentation<T>(json);

    #region 🧬KitDocumentJson

    /// <summary>📁 Reads split <c>kit.compose.json</c> shells with sibling <c>types/</c> and <c>designs/</c> sidecars merged for hydration.</summary>
    public static string ReadKitFixtureJson(string kitJsonPath)
    {
        var initialKitDir = Path.GetDirectoryName(Path.GetFullPath(kitJsonPath))
            ?? throw new DirectoryNotFoundException($"Kit fixture path has no directory: {kitJsonPath}");
        var typesDir = Path.Combine(initialKitDir, "types");
        if (!Directory.Exists(typesDir))
            return System.IO.File.ReadAllText(kitJsonPath);
        var shell = JObject.Parse(System.IO.File.ReadAllText(kitJsonPath));
        var typeById = new Dictionary<string, JObject>(StringComparer.Ordinal);
        foreach (var typeFile in Directory.EnumerateFiles(typesDir, "*.type.compose.json"))
        {
            var row = JObject.Parse(System.IO.File.ReadAllText(typeFile));
            var id = row["id"]?.Value<string>();
            if (!string.IsNullOrEmpty(id))
                typeById[id] = row;
        }
        var designById = new Dictionary<string, JObject>(StringComparer.Ordinal);
        var designsDir = Path.Combine(initialKitDir, "designs");
        if (Directory.Exists(designsDir))
        {
            foreach (var designFile in Directory.EnumerateFiles(designsDir, "*.design.compose.json"))
            {
                var row = JObject.Parse(System.IO.File.ReadAllText(designFile));
                var id = row["id"]?.Value<string>();
                if (!string.IsNullOrEmpty(id))
                    designById[id] = row;
            }
        }
        if (shell["typologies"] is not JArray and JObject typologiesBlock)
        {
            var topoItems = typologiesBlock["items"] as JArray;
            if (topoItems != null)
                MergeSplitTypologySidecars(topoItems, typeById, designById);
        }
        else if (shell["typologies"] is JArray topoArr)
        {
            MergeSplitTypologySidecars(topoArr, typeById, designById);
        }
        return shell.ToString(Formatting.None);
    }

    private static void MergeSplitTypologySidecars(JArray topologies, IReadOnlyDictionary<string, JObject> typeById, IReadOnlyDictionary<string, JObject> designById)
    {
        foreach (var topoTok in topologies.OfType<JObject>())
        {
            if (topoTok["types"] is JObject typesBlock && typesBlock["items"] is JArray typeItems)
            {
                for (var i = 0; i < typeItems.Count; i++)
                {
                    if (typeItems[i] is not JObject stub) continue;
                    var id = stub["id"]?.Value<string>();
                    if (!string.IsNullOrEmpty(id) && typeById.TryGetValue(id, out var full))
                        typeItems[i] = full;
                }
            }
            if (topoTok["designs"] is JObject designsBlock && designsBlock["items"] is JArray designItems)
            {
                for (var i = 0; i < designItems.Count; i++)
                {
                    if (designItems[i] is not JObject stub) continue;
                    var id = stub["id"]?.Value<string>();
                    if (!string.IsNullOrEmpty(id) && designById.TryGetValue(id, out var full))
                        designItems[i] = full;
                }
            }
        }
    }

    /// <summary>📦 Flattens persisted kit workspace JSON (<c>wip.initialKit</c>, <c>{ hash, items }</c> buckets, <c>updatedAt</c>) into the JSON shape <see cref="Kit"/> bindings expect.</summary>
    public static string NormalizeKitDocumentJson(string json)
    {
        var root = JToken.Parse(json) as JObject ?? throw new JsonSerializationException("Kit JSON root must be an object.");
        var kitPayload = root["wip"]?["initialKit"] as JObject ?? root;
        while (UnwrapHashItemCollections(kitPayload)) { }
        RenameUpdatedAtToModificationdAt(kitPayload);
        WireDesignParentChainJson(kitPayload);
        return kitPayload.ToString(Formatting.None);
    }

    private static void WireDesignParentChainJson(JObject kitPayload)
    {
        if (kitPayload["designs"] is not JArray designs) return;
        var variantNames = new HashSet<string>(StringComparer.Ordinal) { "Slanted", "Twisted", "Dancing" };
        JObject? nakagin = null;
        var variants = new List<JObject>();
        var flats = new List<JObject>();
        foreach (var tok in designs.OfType<JObject>())
        {
            var name = tok["name"]?.Value<string>();
            if (string.IsNullOrEmpty(name)) continue;
            if (name == "Nakagin Capsule Tower") nakagin = tok;
            else if (variantNames.Contains(name)) variants.Add(tok);
            else if (name == "Flat") flats.Add(tok);
        }
        if (nakagin?["id"]?.Value<string>() is not { } nakaginId) return;
        foreach (var v in variants)
        {
            if (v["parent"] != null) continue;
            v["parent"] = new JObject { ["id"] = nakaginId };
        }
        for (var i = 0; i < flats.Count; i++)
        {
            if (flats[i]["parent"] != null) continue;
            string? parentId = i == 0
                ? nakaginId
                : i - 1 < variants.Count ? variants[i - 1]["id"]?.Value<string>() : null;
            if (parentId != null)
                flats[i]["parent"] = new JObject { ["id"] = parentId };
        }
        var orphanRoots = designs.OfType<JObject>()
            .Where(d =>
            {
                var n = d["name"]?.Value<string>();
                return !string.IsNullOrEmpty(n) && d["parent"] == null && n != "Nakagin Capsule Tower" && n != "Flat" && !variantNames.Contains(n);
            })
            .ToList();
        foreach (var flat in flats)
        {
            if (flat["parent"] != null) continue;
            if (orphanRoots.Count == 0) break;
            flat["parent"] = new JObject { ["id"] = orphanRoots[0]["id"] };
            orphanRoots.RemoveAt(0);
        }
    }

    /// <summary>📦 Deserializes a <see cref="Kit"/> after <see cref="NormalizeKitDocumentJson"/>; maps <c>pose</c> onto <see cref="Piece.Plane"/> / <see cref="Piece.Center"/>.</summary>
    public static Kit? DeserializeKit(string json)
    {
        var normalized = NormalizeKitDocumentJson(json);
        var kit = Deserialize<Kit>(normalized);
        if (kit is null) return null;
        ApplyPiecePoseFromNormalizedJson(kit, JObject.Parse(normalized));
        return kit;
    }

    private static readonly JsonSerializerSettings KitPoseJsonSettings = new()
    {
        ContractResolver = new CamelCasePropertyNamesContractResolver(),
        ObjectCreationHandling = ObjectCreationHandling.Replace,
    };

    private static void ApplyPiecePoseFromNormalizedJson(Kit kit, JObject normalized)
    {
        ApplyPiecePoseFromNormalizedJson(kit.Pieces, normalized);
        if (normalized["designs"] is not JArray designs) return;
        for (var d = 0; d < designs.Count && d < kit.Designs.Count; d++)
        {
            if (designs[d] is not JObject designJson) continue;
            ApplyPiecePoseFromNormalizedJson(kit.Designs[d].Pieces, designJson);
        }
    }

    /// <summary>📐 Maps JSON <c>pose</c> onto <see cref="Piece.Plane"/> / <see cref="Piece.Center"/> for a piece list container.</summary>
    public static void ApplyPiecePoseFromNormalizedJson(List<Piece> pieces, JObject container)
    {
        if (container["pieces"] is not JArray jsonPieces) return;
        var count = Math.Min(jsonPieces.Count, pieces.Count);
        for (var i = 0; i < count; i++)
        {
            if (jsonPieces[i] is not JObject pj) continue;
            var piece = pieces[i];
            if (pj["pose"] is not JObject pose) continue;
            if (piece.Plane is null && pose["plane"] is not null)
                piece.Plane = pose["plane"].ToObject<Plane>(JsonSerializer.Create(KitPoseJsonSettings));
            if (piece.Center is null && pose["center"] is not null)
                piece.Center = pose["center"].ToObject<Coordinate>(JsonSerializer.Create(KitPoseJsonSettings));
        }
    }

    private static bool UnwrapHashItemCollections(JToken node)
    {
        switch (node)
        {
            case JArray arr:
                var arrChanged = false;
                foreach (var child in arr)
                    arrChanged |= UnwrapHashItemCollections(child);
                return arrChanged;
            case JObject obj:
                var objChanged = false;
                foreach (var prop in obj.Properties().ToList())
                {
                    if (prop.Value is JObject vo && vo["hash"] != null && vo["items"] is JArray ja)
                    {
                        prop.Value = ja;
                        objChanged = true;
                        foreach (var child in ja)
                            objChanged |= UnwrapHashItemCollections(child);
                    }
                    else
                        objChanged |= UnwrapHashItemCollections(prop.Value);
                }
                return objChanged;
            default:
                return false;
        }
    }

    private static void RenameUpdatedAtToModificationdAt(JToken node)
    {
        switch (node)
        {
            case JArray arr:
                foreach (var child in arr)
                    RenameUpdatedAtToModificationdAt(child);
                return;
            case JObject obj:
                if (obj["updatedAt"] != null && obj["modificationdAt"] == null)
                {
                    obj["modificationdAt"] = obj["updatedAt"];
                    obj.Remove("updatedAt");
                }
                foreach (var prop in obj.Properties())
                    RenameUpdatedAtToModificationdAt(prop.Value);
                return;
        }
    }

    /// <summary>🔁 Maps canonical diff wire <c>updated</c> arrays to <see cref="PiecesDiff.Modified"/> binding <c>modified</c>.</summary>
    public static string NormalizeEntityDiffWireJson(string json)
    {
        var root = JToken.Parse(json);
        NormalizeEntityDiffWireToken(root);
        return root.ToString(Formatting.None);
    }

    private static void NormalizeEntityDiffWireToken(JToken node)
    {
        switch (node)
        {
            case JArray arr:
                foreach (var child in arr)
                    NormalizeEntityDiffWireToken(child);
                return;
            case JObject obj:
                if (obj["updated"] is JArray updated && obj["modified"] == null)
                {
                    obj["modified"] = updated;
                    obj.Remove("updated");
                }
                foreach (var prop in obj.Properties())
                    NormalizeEntityDiffWireToken(prop.Value);
                return;
        }
    }

    #endregion 🧬KitDocumentJson

    public static string GenerateRandomId(int seed)
    {
        var adjectives = Utility.Deserialize<List<string>>(Resources.adjectives);
        var animals = Utility.Deserialize<List<string>>(Resources.animals);
        if (adjectives is null || animals is null) throw new InvalidOperationException("Failed to deserialize resources");
        var random = new Random(seed);
        var adjective = adjectives[random.Next(adjectives.Count)];
        var animal = animals[random.Next(animals.Count)];
        var number = random.Next(0, 999);
        adjective = char.ToUpper(adjective[0]) + adjective.Substring(1);
        animal = char.ToUpper(animal[0]) + animal.Substring(1);
        return $"{adjective}{animal}{number}";
    }

    public static class Units
    {
        public static float Convert(float value, string fromUnit, string toUnit)
        {
            var convertEntity = new PowerToysRunUnitConverter.ConvertEntity(value, fromUnit, toUnit);
            var results = PowerToysRunUnitConverter.UnitHandler.Convert(convertEntity);
            if (results.Count() == 0) return float.NaN;
            return (float)results.First().ConvertedValue;
        }

        private class PowerToysRunUnitConverter
        {
            internal class ConvertEntity
            {
                internal ConvertEntity() { FromUnit = ""; ToUnit = ""; }
                internal ConvertEntity(double value, string fromUnit, string toUnit) => (Value, FromUnit, ToUnit) = (value, fromUnit, toUnit);
                internal double Value { get; }
                internal string FromUnit { get; }
                internal string ToUnit { get; }
            }

            internal class UnitConversionResult
            {
                internal UnitConversionResult(double convertedValue, string unitName, QuantityInfo quantityInfo) => (ConvertedValue, UnitName, QuantityInfo) = (convertedValue, unitName, quantityInfo);
                internal static string TitleFormat { get; set; } = "G14";
                internal static string CopyFormat { get; set; } = "R";
                internal double ConvertedValue { get; }
                internal string UnitName { get; }
                internal QuantityInfo QuantityInfo { get; }
            }

            internal static class UnitHandler
            {
                private static readonly QuantityInfo[] _included =
                {
                    UnitsNet.Length.Info,
                    Area.Info,
                    Volume.Info,
                    Duration.Info,
                    Energy.Info,
                    UnitsNet.Power.Info,
                    Pressure.Info,
                    Mass.Info,
                    Angle.Info,
                    Temperature.Info,
                    Acceleration.Info,
                    Speed.Info,
                    Information.Info
                };
                private static Enum? GetUnitEnum(string unit, QuantityInfo unitInfo)
                {
                    var first = Array.Find(unitInfo.UnitInfos, info => string.Equals(unit, info.Name, StringComparison.OrdinalIgnoreCase) || string.Equals(unit, info.PluralName, StringComparison.OrdinalIgnoreCase));
                    if (first != null) return first.Value;
                    if (UnitsNetSetup.Default.UnitParser.TryParse(unit, unitInfo.UnitType, out var enum_unit)) return enum_unit;
                    var cultureInfoEnglish = new CultureInfo("en-US");
                    if (UnitsNetSetup.Default.UnitParser.TryParse(unit, unitInfo.UnitType, cultureInfoEnglish, out var enum_unit_en)) return enum_unit_en;
                    return null;
                }
                internal static double ConvertInput(ConvertEntity convertEntity, QuantityInfo quantityInfo)
                {
                    var fromUnit = GetUnitEnum(convertEntity.FromUnit, quantityInfo);
                    var toUnit = GetUnitEnum(convertEntity.ToUnit, quantityInfo);
                    if (fromUnit != null && toUnit != null) return UnitConverter.Convert(convertEntity.Value, fromUnit, toUnit);
                    return double.NaN;
                }
                internal static IEnumerable<UnitConversionResult> Convert(ConvertEntity convertEntity)
                {
                    var results = new List<UnitConversionResult>();
                    foreach (var quantityInfo in _included)
                    {
                        var convertedValue = ConvertInput(convertEntity, quantityInfo);
                        if (!double.IsNaN(convertedValue)) results.Add(new UnitConversionResult(convertedValue, convertEntity.ToUnit, quantityInfo));
                    }
                    return results;
                }
            }
        }
    }

    public static class Grammar
    {
        public static string GetArticle(string word) => string.IsNullOrEmpty(word) ? string.Empty : "aeiou".IndexOf(word.ToLower()[0]) >= 0 ? "an" : "a";
    }
}

#region ❄️Expressions
// Implementations MUST evaluate expression trees through the Operator.Apply contract.

/// <summary>🌳Abstract base for all expression tree nodes.</summary>
/// <remarks>
/// </remarks>
public abstract class Symbol { }
public abstract class Term : Symbol { }
public abstract class Constant : Term { }

public class UnitValue
{
    public float Value { get; set; }
    public string Unit { get; set; }

    public UnitValue(float value, string unit = "")
    {
        Value = value;
        Unit = unit ?? "";
    }

    public float ConvertTo(string targetUnit)
    {
        if (string.IsNullOrEmpty(Unit) || string.IsNullOrEmpty(targetUnit) || Unit == targetUnit)
            return Value;
        return Utility.Units.Convert(Value, Unit, targetUnit);
    }

    public override string ToString() => string.IsNullOrEmpty(Unit) ? Value.ToString("G9", CultureInfo.InvariantCulture) : $"'{Value.ToString("G9", CultureInfo.InvariantCulture)} {Unit}'";
}

public class NumberConstant : Constant
{
    public UnitValue UnitValue { get; set; }
    public NumberConstant(float value, string unit = "") { UnitValue = new UnitValue(value, unit); }
    public NumberConstant(UnitValue unitValue) { UnitValue = unitValue ?? new UnitValue(0); }
    public override string ToString() => UnitValue.ToString();
}

public class StringConstant : Constant
{
    public string Value { get; set; }
    public StringConstant(string value) { Value = value ?? string.Empty; }
    public override string ToString() => $"\"{Value}\"";
}

public class Variable : Term
{
    public string Name { get; set; }
    public Variable(string name) { Name = name; }
    public override string ToString() => Name;
}

public abstract class Operator : Symbol
{
    public abstract string Keyword { get; }
    public abstract object Apply(object[] args, string targetUnit = "");

    protected static UnitValue ConvertToUnitValue(object arg)
    {
        return arg switch
        {
            UnitValue uv => uv,
            float f => new UnitValue(f),
            _ => throw new ArgumentException($"Cannot convert {arg?.GetType().Name ?? "null"} to UnitValue")
        };
    }

    protected static UnitValue[] ConvertArgsToUnitValues(object[] args)
    {
        return args.Select(ConvertToUnitValue).ToArray();
    }

    protected static string DetermineCommonUnit(UnitValue[] values)
    {
        var nonEmptyUnits = values.Where(v => !string.IsNullOrEmpty(v.Unit)).ToArray();
        if (nonEmptyUnits.Length == 0) return "";
        return nonEmptyUnits[0].Unit;
    }
}

public class Sum : Operator
{
    public override string Keyword => "sum";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) return new UnitValue(0);

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float sum = 0;

        foreach (var uv in unitValues)
        {
            if (string.IsNullOrEmpty(commonUnit))
                sum += uv.Value;
            else
                sum += uv.ConvertTo(commonUnit);
        }

        return new UnitValue(sum, commonUnit);
    }
}

public class Multiply : Operator
{
    public override string Keyword => "multiply";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) return new UnitValue(1);

        float result = 1f;
        var units = new List<string>();

        foreach (var uv in unitValues)
        {
            result *= uv.Value;
            if (!string.IsNullOrEmpty(uv.Unit))
                units.Add(uv.Unit);
        }

        var combinedUnit = string.Join("·", units);
        return new UnitValue(result, combinedUnit);
    }
}

public class Subtract : Operator
{
    public override string Keyword => "subtract";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length < 2) throw new ArgumentException("subtract requires at least 2 operands");

        var commonUnit = DetermineCommonUnit(unitValues);
        float result = string.IsNullOrEmpty(commonUnit) ? unitValues[0].Value : unitValues[0].ConvertTo(commonUnit);

        for (int i = 1; i < unitValues.Length; i++)
        {
            result -= string.IsNullOrEmpty(commonUnit) ? unitValues[i].Value : unitValues[i].ConvertTo(commonUnit);
        }

        return new UnitValue(result, commonUnit);
    }
}

public class Divide : Operator
{
    public override string Keyword => "divide";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length < 2) throw new ArgumentException("divide requires at least 2 operands");

        float acc = unitValues[0].Value;
        var numeratorUnit = unitValues[0].Unit;
        var denominatorUnits = new List<string>();

        for (int i = 1; i < unitValues.Length; i++)
        {
            if (unitValues[i].Value == 0f) throw new DivideByZeroException("division by zero");
            acc /= unitValues[i].Value;
            if (!string.IsNullOrEmpty(unitValues[i].Unit))
                denominatorUnits.Add(unitValues[i].Unit);
        }

        var resultUnit = "";
        if (!string.IsNullOrEmpty(numeratorUnit) || denominatorUnits.Count > 0)
        {
            var denominatorPart = denominatorUnits.Count > 0 ? string.Join("·", denominatorUnits) : "";
            if (!string.IsNullOrEmpty(numeratorUnit) && !string.IsNullOrEmpty(denominatorPart))
                resultUnit = $"{numeratorUnit}/{denominatorPart}";
            else if (!string.IsNullOrEmpty(numeratorUnit))
                resultUnit = numeratorUnit;
            else if (!string.IsNullOrEmpty(denominatorPart))
                resultUnit = $"1/{denominatorPart}";
        }

        return new UnitValue(acc, resultUnit);
    }
}

public class Negate : Operator
{
    public override string Keyword => "negate";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("negate requires exactly 1 operand");
        var unitValue = ConvertToUnitValue(args[0]);
        return new UnitValue(-unitValue.Value, unitValue.Unit);
    }
}

public class SquareRoot : Operator
{
    public override string Keyword => "sqrt";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("sqrt requires exactly 1 operand");
        var unitValue = ConvertToUnitValue(args[0]);
        if (unitValue.Value < 0f) throw new ArgumentException("sqrt requires non-negative operand");
        var resultUnit = string.IsNullOrEmpty(unitValue.Unit) ? "" : $"√({unitValue.Unit})";
        return new UnitValue((float)Math.Sqrt(unitValue.Value), resultUnit);
    }
}

public class Power : Operator
{
    public override string Keyword => "power";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("power requires exactly 2 operands");
        var baseValue = ConvertToUnitValue(args[0]);
        var exponent = ConvertToUnitValue(args[1]);
        var resultUnit = string.IsNullOrEmpty(baseValue.Unit) ? "" : $"({baseValue.Unit})^{exponent.Value}";
        return new UnitValue((float)Math.Pow(baseValue.Value, exponent.Value), resultUnit);
    }
}

public class Min : Operator
{
    public override string Keyword => "min";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) throw new ArgumentException("min requires at least 1 operand");

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float minValue = float.MaxValue;

        foreach (var uv in unitValues)
        {
            var value = string.IsNullOrEmpty(commonUnit) ? uv.Value : uv.ConvertTo(commonUnit);
            if (value < minValue) minValue = value;
        }

        return new UnitValue(minValue, commonUnit);
    }
}

public class Max : Operator
{
    public override string Keyword => "max";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) throw new ArgumentException("max requires at least 1 operand");

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float maxValue = float.MinValue;

        foreach (var uv in unitValues)
        {
            var value = string.IsNullOrEmpty(commonUnit) ? uv.Value : uv.ConvertTo(commonUnit);
            if (value > maxValue) maxValue = value;
        }

        return new UnitValue(maxValue, commonUnit);
    }
}

public class Average : Operator
{
    public override string Keyword => "average";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) throw new ArgumentException("average requires at least 1 operand");

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float sum = 0;

        foreach (var uv in unitValues)
        {
            sum += string.IsNullOrEmpty(commonUnit) ? uv.Value : uv.ConvertTo(commonUnit);
        }

        return new UnitValue(sum / unitValues.Length, commonUnit);
    }
}

public class Modulo : Operator
{
    public override string Keyword => "mod";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("mod requires exactly 2 operands");
        var value1 = ConvertToUnitValue(args[0]);
        var value2 = ConvertToUnitValue(args[1]);
        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(new[] { value1, value2 }) : targetUnit;

        var val1 = string.IsNullOrEmpty(commonUnit) ? value1.Value : value1.ConvertTo(commonUnit);
        var val2 = string.IsNullOrEmpty(commonUnit) ? value2.Value : value2.ConvertTo(commonUnit);

        return new UnitValue(val1 % val2, commonUnit);
    }
}

public class And : Operator
{
    public override string Keyword => "and";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length < 2) throw new ArgumentException("and requires at least 2 operands");
        return args.Cast<float>().All(x => x != 0f) ? 1f : 0f;
    }
}

public class Or : Operator
{
    public override string Keyword => "or";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length < 2) throw new ArgumentException("or requires at least 2 operands");
        return args.Cast<float>().Any(x => x != 0f) ? 1f : 0f;
    }
}

public class ExclusiveOr : Operator
{
    public override string Keyword => "xor";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("xor requires exactly 2 operands");
        bool a = (float)args[0] != 0f;
        bool b = (float)args[1] != 0f;
        return (a ^ b) ? 1f : 0f;
    }
}

public class Invert : Operator
{
    public override string Keyword => "not";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("not requires exactly 1 operand");
        var value = ConvertToUnitValue(args[0]);
        return new UnitValue(value.Value == 0f ? 1f : 0f);
    }
}

public class Equal : Operator
{
    public override string Keyword => "equal";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("equal requires exactly 2 operands");

        if (args[0] is UnitValue uv1 && args[1] is UnitValue uv2)
        {
            var commonUnit = DetermineCommonUnit(new[] { uv1, uv2 });
            var val1 = string.IsNullOrEmpty(commonUnit) ? uv1.Value : uv1.ConvertTo(commonUnit);
            var val2 = string.IsNullOrEmpty(commonUnit) ? uv2.Value : uv2.ConvertTo(commonUnit);
            return new UnitValue(Math.Abs(val1 - val2) < float.Epsilon ? 1f : 0f);
        }

        if (args[0] is float f1 && args[1] is float f2)
            return new UnitValue(Math.Abs(f1 - f2) < float.Epsilon ? 1f : 0f);

        if (args[0] is string s1 && args[1] is string s2)
            return new UnitValue(string.Equals(s1, s2, StringComparison.Ordinal) ? 1f : 0f);

        return new UnitValue(0f);
    }
}

public class GreaterThan : Operator
{
    public override string Keyword => "greater";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("greater requires exactly 2 operands");
        return (float)args[0] > (float)args[1] ? 1f : 0f;
    }
}

public class LessThan : Operator
{
    public override string Keyword => "less";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("less requires exactly 2 operands");
        return (float)args[0] < (float)args[1] ? 1f : 0f;
    }
}

public class GreaterThanOrEqual : Operator
{
    public override string Keyword => "greater-equal";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("greater-equal requires exactly 2 operands");
        return (float)args[0] >= (float)args[1] ? 1f : 0f;
    }
}

public class LessThanOrEqual : Operator
{
    public override string Keyword => "less-equal";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("less-equal requires exactly 2 operands");
        return (float)args[0] <= (float)args[1] ? 1f : 0f;
    }
}

public class If : Operator
{
    public override string Keyword => "if";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("if requires exactly 3 operands: condition, true-value, false-value");
        return (float)args[0] != 0f ? args[1] : args[2];
    }
}

public class Absolute : Operator
{
    public override string Keyword => "abs";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("abs requires exactly 1 operand");
        var uv = ConvertToUnitValue(args[0]);
        return new UnitValue(Math.Abs(uv.Value), uv.Unit);
    }
}

public class Floor : Operator
{
    public override string Keyword => "floor";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("floor requires exactly 1 operand");
        return (float)Math.Floor((float)args[0]);
    }
}

public class Ceiling : Operator
{
    public override string Keyword => "ceil";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("ceil requires exactly 1 operand");
        return (float)Math.Ceiling((float)args[0]);
    }
}

public class Round : Operator
{
    public override string Keyword => "round";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("round requires exactly 1 operand");
        return (float)Math.Round((float)args[0]);
    }
}

public class Length : Operator
{
    public override string Keyword => "length";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("length requires exactly 1 operand");
        return (float)((string)args[0]).Length;
    }
}

public class StartsWith : Operator
{
    public override string Keyword => "startswith";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("startswith requires exactly 2 operands");
        return ((string)args[0]).StartsWith((string)args[1], StringComparison.Ordinal) ? 1f : 0f;
    }
}

public class EndsWith : Operator
{
    public override string Keyword => "endswith";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("endswith requires exactly 2 operands");
        return ((string)args[0]).EndsWith((string)args[1], StringComparison.Ordinal) ? 1f : 0f;
    }
}

public class Contains : Operator
{
    public override string Keyword => "contains";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("contains requires exactly 2 operands");
        return ((string)args[0]).Contains((string)args[1]) ? 1f : 0f;
    }
}

public class Substring : Operator
{
    public override string Keyword => "substring";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length < 2 || args.Length > 3) throw new ArgumentException("substring requires 2 or 3 operands");
        string str = (string)args[0];
        int start = (int)(float)args[1];
        if (args.Length == 3)
        {
            int length = (int)(float)args[2];
            return str.Substring(start, length);
        }
        return str.Substring(start);
    }
}

public class Concat : Operator
{
    public override string Keyword => "concat";
    public override object Apply(object[] args, string targetUnit = "")
    {
        return string.Concat(args.Cast<string>());
    }
}

public class ToUpper : Operator
{
    public override string Keyword => "upper";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("upper requires exactly 1 operand");
        return ((string)args[0]).ToUpper();
    }
}

public class ToLower : Operator
{
    public override string Keyword => "lower";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("lower requires exactly 1 operand");
        return ((string)args[0]).ToLower();
    }
}

public class Trim : Operator
{
    public override string Keyword => "trim";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("trim requires exactly 1 operand");
        return ((string)args[0]).Trim();
    }
}

public class Replace : Operator
{
    public override string Keyword => "replace";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("replace requires exactly 3 operands");
        return ((string)args[0]).Replace((string)args[1], (string)args[2]);
    }
}

public class ToNumber : Operator
{
    public override string Keyword => "number";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("number requires exactly 1 operand");
        if (args[0] is string str)
        {
            if (float.TryParse(str, NumberStyles.Float, CultureInfo.InvariantCulture, out float result))
                return result;
            throw new FormatException($"Cannot convert '{str}' to number");
        }
        return (float)args[0];
    }
}

public class ToText : Operator
{
    public override string Keyword => "text";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("text requires exactly 1 operand");
        if (args[0] is float f)
            return f.ToString(CultureInfo.InvariantCulture);
        return (string)args[0];
    }
}

public class ToBoolean : Operator
{
    public override string Keyword => "boolean";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("boolean requires exactly 1 operand");
        if (args[0] is float f)
            return f != 0f ? 1f : 0f;
        if (args[0] is string s)
            return string.IsNullOrEmpty(s) ? 0f : 1f;
        return 0f;
    }
}

public class Clamp : Operator
{
    public override string Keyword => "clamp";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("clamp requires exactly 3 operands: value, min, max");
        float value = (float)args[0];
        float min = (float)args[1];
        float max = (float)args[2];
        return Math.Max(min, Math.Min(max, value));
    }
}

public class Lerp : Operator
{
    public override string Keyword => "lerp";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("lerp requires exactly 3 operands: a, b, t");
        float a = (float)args[0];
        float b = (float)args[1];
        float t = (float)args[2];
        return a + (b - a) * t;
    }
}

public class Sign : Operator
{
    public override string Keyword => "sign";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("sign requires exactly 1 operand");
        return (float)Math.Sign((float)args[0]);
    }
}

public class IsEmpty : Operator
{
    public override string Keyword => "isempty";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("isempty requires exactly 1 operand");
        if (args[0] is string str)
            return string.IsNullOrEmpty(str) ? 1f : 0f;
        return 0f;
    }
}

public class Operation : Term
{
    public Operator Operator { get; set; }
    public Term[] Operands { get; set; }

    public Operation(Operator operation, params Term[] operands)
    {
        Operator = operation ?? throw new ArgumentNullException(nameof(operation));
        Operands = operands ?? Array.Empty<Term>();
    }

    public object Evaluate(Dictionary<string, object>? context = null, string targetUnit = "")
    {
        object[] values = Operands.Select(o => EvaluateTerm(o, context, targetUnit)).ToArray();
        return Operator.Apply(values, targetUnit);
    }

    private static object EvaluateTerm(Term t, Dictionary<string, object>? ctx, string targetUnit = "")
    {
        switch (t)
        {
            case NumberConstant c:
                return c.UnitValue;
            case StringConstant sc:
                return sc.Value;
            case Variable v:
                if (ctx == null || !ctx.TryGetValue(v.Name, out var val))
                    throw new KeyNotFoundException($"No value provided for variable '{v.Name}'.");
                return val;
            case Operation operation:
                return operation.Evaluate(ctx, targetUnit);
            default:
                throw new InvalidOperationException($"Unknown term type: {t?.GetType().Name ?? "null"}");
        }
    }
}

public class Expression
{
    public Term? Root { get; private set; }
    private readonly Dictionary<string, Func<Operator>> _operators;

    public Expression()
    {
        _operators = new Dictionary<string, Func<Operator>>(StringComparer.OrdinalIgnoreCase)
        {

            { "sum", () => new Sum() },
            { "multiply", () => new Multiply() },
            { "subtract", () => new Subtract() },
            { "divide", () => new Divide() },
            { "negate", () => new Negate() },
            { "power", () => new Power() },
            { "sqrt", () => new SquareRoot() },
            { "min", () => new Min() },
            { "max", () => new Max() },
            { "abs", () => new Absolute() },
            { "floor", () => new Floor() },
            { "ceil", () => new Ceiling() },
            { "round", () => new Round() },
            { "average", () => new Average() },
            { "mod", () => new Modulo() },

            { "and", () => new And() },
            { "or", () => new Or() },
            { "xor", () => new ExclusiveOr() },
            { "not", () => new Invert() },

            { "equal", () => new Equal() },
            { "greater", () => new GreaterThan() },
            { "less", () => new LessThan() },
            { "greater-equal", () => new GreaterThanOrEqual() },
            { "less-equal", () => new LessThanOrEqual() },

            { "if", () => new If() },

            { "length", () => new Length() },
            { "startswith", () => new StartsWith() },
            { "endswith", () => new EndsWith() },
            { "contains", () => new Contains() },
            { "substring", () => new Substring() },
            { "concat", () => new Concat() },
            { "upper", () => new ToUpper() },
            { "lower", () => new ToLower() },
            { "trim", () => new Trim() },
            { "replace", () => new Replace() },

            { "number", () => new ToNumber() },
            { "text", () => new ToText() },
            { "boolean", () => new ToBoolean() },

            { "clamp", () => new Clamp() },
            { "lerp", () => new Lerp() },
            { "sign", () => new Sign() },
            { "isempty", () => new IsEmpty() }
        };
    }

    public Expression[] Pop()
    {
        if (Root == null) throw new InvalidOperationException("Expression has no root term.");

        if (Root is Operation operation)
        {
            return operation.Operands.Select(operand => new Expression { Root = operand }).ToArray();
        }

        throw new InvalidOperationException("Root term is not an operation, cannot pop operands.");
    }

    public object Calculate(Dictionary<string, object>? context = null, string targetUnit = "")
    {
        if (Root == null) throw new InvalidOperationException("Expression has no root term.");
        return Root switch
        {
            NumberConstant c => string.IsNullOrEmpty(targetUnit) ? c.UnitValue : new UnitValue(c.UnitValue.ConvertTo(targetUnit)),
            StringConstant sc => sc.Value,
            Variable v => context != null && context.TryGetValue(v.Name, out var val)
                            ? val
                            : throw new KeyNotFoundException($"No value provided for variable '{v.Name}'."),
            Operation o => o.Evaluate(context, targetUnit),
            _ => throw new InvalidOperationException("Unknown root term.")
        };
    }

    public string Serialize()
    {
        if (Root == null) return string.Empty;
        var sb = new StringBuilder();
        SerializeTerm(Root, sb);
        return sb.ToString();
    }

    public Expression Deserialize(string expression)
    {
        if (expression == null) throw new ArgumentNullException(nameof(expression));
        var tokens = Tokenize(expression);
        int index = 0;
        Root = ParseExpr(tokens, ref index);
        if (index != tokens.Count)
            throw new FormatException($"Unexpected token '{tokens[index].Text}' at position {tokens[index].Position}.");
        return this;
    }

    private void SerializeTerm(Term term, StringBuilder sb)
    {
        switch (term)
        {
            case NumberConstant c:
                sb.Append(c.UnitValue.ToString());
                break;
            case StringConstant sc:
                sb.Append('"');
                sb.Append(sc.Value.Replace("\"", "\\\""));
                sb.Append('"');
                break;
            case Variable v:
                sb.Append(v.Name);
                break;
            case Operation operation:
                sb.Append(operation.Operator.Keyword);
                sb.Append(" ( ");
                for (int i = 0; i < operation.Operands.Length; i++)
                {
                    if (i > 0) sb.Append(' ');
                    SerializeTerm(operation.Operands[i], sb);
                }
                sb.Append(" )");
                break;
            default:
                throw new InvalidOperationException($"Unknown term type for serialization: {term?.GetType().Name ?? "null"}");
        }
    }

    private enum TokenKind { Identifier, Number, String, UnitLiteral, LeftParenthesis, RightParenthesis }

    private readonly struct Token
    {
        public TokenKind Kind { get; }
        public string Text { get; }
        public int Position { get; }
        public Token(TokenKind k, string t, int pos) { Kind = k; Text = t; Position = pos; }
        public override string ToString() => $"{Kind}:{Text}";
    }

    private static readonly HashSet<char> IdentifierExtraChars = new HashSet<char> { '.', '-', '_' };

    private static List<Token> Tokenize(string input)
    {
        var tokens = new List<Token>();
        int i = 0;
        while (i < input.Length)
        {
            char c = input[i];

            if (char.IsWhiteSpace(c)) { i++; continue; }

            if (c == '(') { tokens.Add(new Token(TokenKind.LeftParenthesis, "(", i)); i++; continue; }
            if (c == ')') { tokens.Add(new Token(TokenKind.RightParenthesis, ")", i)); i++; continue; }

            if (c == '"')
            {
                int start = i;
                i++;
                var sb = new StringBuilder();
                while (i < input.Length && input[i] != '"')
                {
                    if (input[i] == '\\' && i + 1 < input.Length)
                    {
                        i++;
                        switch (input[i])
                        {
                            case '"': sb.Append('"'); break;
                            case '\\': sb.Append('\\'); break;
                            case 'n': sb.Append('\n'); break;
                            case 't': sb.Append('\t'); break;
                            case 'r': sb.Append('\r'); break;
                            default: sb.Append(input[i]); break;
                        }
                    }
                    else
                    {
                        sb.Append(input[i]);
                    }
                    i++;
                }
                if (i >= input.Length) throw new FormatException($"Unterminated string literal starting at {start}.");
                i++;
                tokens.Add(new Token(TokenKind.String, sb.ToString(), start));
                continue;
            }

            if (c == '\'')
            {
                int start = i;
                i++;
                var sb = new StringBuilder();
                while (i < input.Length && input[i] != '\'')
                {
                    if (input[i] == '\\' && i + 1 < input.Length)
                    {
                        i++;
                        switch (input[i])
                        {
                            case '\'': sb.Append('\''); break;
                            case '\\': sb.Append('\\'); break;
                            case 'n': sb.Append('\n'); break;
                            case 't': sb.Append('\t'); break;
                            case 'r': sb.Append('\r'); break;
                            default: sb.Append(input[i]); break;
                        }
                    }
                    else
                    {
                        sb.Append(input[i]);
                    }
                    i++;
                }
                if (i >= input.Length) throw new FormatException($"Unterminated unit literal starting at {start}.");
                i++;
                tokens.Add(new Token(TokenKind.UnitLiteral, sb.ToString(), start));
                continue;
            }

            if (char.IsDigit(c) || (c == '.' && i + 1 < input.Length && char.IsDigit(input[i + 1])))
            {
                int start = i;
                i++;
                while (i < input.Length && (char.IsDigit(input[i]) || input[i] == '.')) i++;

                if (i < input.Length && (input[i] == 'e' || input[i] == 'E'))
                {
                    int ePos = i++;
                    if (i < input.Length && (input[i] == '+' || input[i] == '-')) i++;
                    bool hasDigit = false;
                    while (i < input.Length && char.IsDigit(input[i])) { hasDigit = true; i++; }
                    if (!hasDigit) throw new FormatException($"Invalid exponent starting at {ePos}.");
                }
                tokens.Add(new Token(TokenKind.Number, input.Substring(start, i - start), start));
                continue;
            }

            if (char.IsLetter(c) || c == '_')
            {
                int start = i;
                i++;
                while (i < input.Length)
                {
                    char d = input[i];
                    if (char.IsLetterOrDigit(d) || IdentifierExtraChars.Contains(d)) { i++; }
                    else break;
                }
                tokens.Add(new Token(TokenKind.Identifier, input.Substring(start, i - start), start));
                continue;
            }

            throw new FormatException($"Unexpected character '{c}' at position {i}.");
        }
        return tokens;
    }

    private Term ParseExpr(List<Token> tokens, ref int index)
    {
        if (index >= tokens.Count) throw new FormatException("Unexpected end of input.");

        var t = tokens[index];

        if (t.Kind == TokenKind.Number)
        {
            index++;
            if (!float.TryParse(t.Text, NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                throw new FormatException($"Invalid number '{t.Text}' at {t.Position}.");
            return new NumberConstant(val);
        }

        if (t.Kind == TokenKind.String)
        {
            index++;
            return new StringConstant(t.Text);
        }

        if (t.Kind == TokenKind.UnitLiteral)
        {
            index++;
            var parts = t.Text.Trim().Split(new char[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length == 0) throw new FormatException($"Empty unit literal at {t.Position}.");

            if (parts.Length == 1)
            {

                if (!float.TryParse(parts[0], NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                    throw new FormatException($"Invalid number '{parts[0]}' in unit literal at {t.Position}.");
                return new NumberConstant(val);
            }
            else
            {

                if (!float.TryParse(parts[0], NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                    throw new FormatException($"Invalid number '{parts[0]}' in unit literal at {t.Position}.");
                var unit = string.Join(" ", parts.Skip(1));
                return new NumberConstant(val, unit);
            }
        }

        if (t.Kind == TokenKind.Identifier)
        {

            string ident = t.Text;
            int idPos = t.Position;
            index++;

            if (index < tokens.Count && tokens[index].Kind == TokenKind.LeftParenthesis)
            {

                index++;
                var args = new List<Term>();
                while (index < tokens.Count && tokens[index].Kind != TokenKind.RightParenthesis)
                {

                    args.Add(ParseExpr(tokens, ref index));

                }
                if (index >= tokens.Count || tokens[index].Kind != TokenKind.RightParenthesis)
                    throw new FormatException($"Missing closing ')' for call starting at {idPos}.");
                index++;

                var operation = InstantiateOperator(ident, idPos);

                if (operation is Divide && args.Count < 2)
                    throw new FormatException("divide requires at least 2 operands.");

                return new Operation(operator, args.ToArray());
            }
            else
            {

                return new Variable(ident);
            }
        }

        if (t.Kind == TokenKind.LeftParenthesis)
        {

            index++;
            var inner = ParseExpr(tokens, ref index);
            if (index >= tokens.Count || tokens[index].Kind != TokenKind.RightParenthesis)
                throw new FormatException($"Missing ')' for parenthesized expression starting at {t.Position}.");
            index++;
            return inner;
        }

        throw new FormatException($"Unexpected token '{t.Text}' at position {t.Position}.");
    }

    private Operator InstantiateOperator(string keyword, int pos)
    {
        if (_operators.TryGetValue(keyword, out var ctor))
            return ctor();

        throw new KeyNotFoundException($"Unknown operator '{keyword}' at position {pos}.");
    }
}

#endregion ❄️Expressions

#endregion 📦Utilities






#region 🔓Entitying
// Implementations MUST extend Entity for equality, validation, and diff support.

/// Abstract generic base class providing equality, hashing, cloning, and validation.
public abstract class Entity<T> where T : Entity<T>
{
    public override string ToString() => GetType().Name;

    public override bool Equals(object? obj)
    {
        if (obj == null || GetType() != obj.GetType()) return false;
        return GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance).All(prop => PropertiesAreEqual(prop, this, obj));
    }

    private bool PropertiesAreEqual(PropertyInfo prop, object obj1, object obj2)
    {
        var value1 = prop.GetValue(obj1);
        var value2 = prop.GetValue(obj2);
        if (value1 is IEnumerable enumerable1 && value2 is IEnumerable enumerable2)
            return enumerable1.Cast<object>().SequenceEqual(enumerable2.Cast<object>());
        return Equals(value1, value2);
    }

    public override int GetHashCode()
    {
        return GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Select(prop => prop.GetValue(this))
            .Where(value => value != null)
            .Aggregate(17, (current, value) => current * 31 + value!.GetHashCode());
    }

    public static bool operator ==(Entity<T> left, Entity<T> right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return left.Equals(right);
    }

    public static bool operator !=(Entity<T> left, Entity<T> right) => !(left == right);

    public static T? DeepClone(T entity) => Utility.Deserialize<T>(Utility.Serialize(entity));

    public virtual (bool, List<string>) Validate()
    {
        var result = new EntityValidator<T>().Validate((T)this);
        return (result.IsValid, result.Errors.Select(e => e.ToString()).ToList());
    }

    public static (bool, List<string>) Validate(T entity) => entity.Validate();
}

/// FluentValidation validator base for Entity subclasses.
public class EntityValidator<T> : AbstractValidator<T> where T : Entity<T>
{
    public EntityValidator()
    {
    }
}

#endregion 🔓Entitying






#region ✨ComposeValidation
// Callers MUST use ValidationResult to report kit-level validation issues.

public class ComposeValidationFix
{
    public string Title { get; set; } = "";
    public object? Diff { get; set; }
}

public class Issue
{
    public string ConstraintId { get; set; } = "";
    public string Message { get; set; } = "";
    public string EntityKind { get; set; } = "";
    public string EntityId { get; set; } = "";
    public List<ComposeValidationFix> Fixes { get; set; } = new();
}

public class ValidationResult
{
    public List<Issue> Issues { get; set; } = new();

    public bool HasErrors() => Issues.Count > 0;

    public string Serialize()
    {
        var sorted = Issues.OrderBy(i => i.ConstraintId).ThenBy(i => i.EntityId).ToList();
        var result = new { issues = sorted.Select(i => new { constraintId = i.ConstraintId, message = i.Message, entityKind = i.EntityKind, entityId = i.EntityId, fixes = i.Fixes.Select(f => new { title = f.Title, diff = f.Diff }) }) };
        return ComposeJson.Codec.SerializeCamelIndented(result);
    }

    public static ValidationResult Parse(string json)
    {
        var data = ComposeJson.Codec.ParseJsonRoot(json) as JObject;
        var result = new ValidationResult();
        var problemsToken = data?["problems"] ?? data?["issues"];
        if (problemsToken == null) return result;
        foreach (var issue in problemsToken)
        {
            var fixes = new List<ComposeValidationFix>();
            var fixesToken = issue["fixes"];
            if (fixesToken != null)
            {
                foreach (var fix in fixesToken)
                {
                    fixes.Add(new ComposeValidationFix { Title = (string?)fix["title"] ?? "", Diff = fix["diff"] });
                }
            }
            result.Issues.Add(new Issue
            {
                ConstraintId = (string?)issue["constraintId"] ?? "",
                Message = (string?)issue["message"] ?? "",
                EntityKind = (string?)issue["entityKind"] ?? "",
                EntityId = (string?)issue["entityId"] ?? "",
                Fixes = fixes
            });
        }
        return result;
    }

    private static string NormalizeIds(string json)
    {
        return System.Text.RegularExpressions.Regex.Replace(json, @"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", "<ID>", System.Text.RegularExpressions.RegexOptions.IgnoreCase);
    }

    public static bool AreEqual(ValidationResult a, ValidationResult b)
    {
        if (a.Issues.Count != b.Issues.Count) return false;
        var sortedA = a.Issues.OrderBy(i => i.ConstraintId).ThenBy(i => i.EntityId).ToList();
        var sortedB = b.Issues.OrderBy(i => i.ConstraintId).ThenBy(i => i.EntityId).ToList();
        for (var i = 0; i < sortedA.Count; i++)
        {
            var ia = sortedA[i];
            var ib = sortedB[i];
            if (ia.ConstraintId != ib.ConstraintId || ia.Message != ib.Message || ia.EntityKind != ib.EntityKind || ia.EntityId != ib.EntityId)
                return false;

        }
        return true;
    }
}

public static class ComposeValidator
{
    public static ValidationResult ValidateKit(Kit kit)
    {
        var issues = new List<Issue>();
        var seen = new Dictionary<string, string>();

        void CheckId(string entityKind, string entityId)
        {
            if (seen.ContainsKey(entityId))
            {
                issues.Add(new Issue { ConstraintId = "id-unique", Message = $"Duplicate ID \"{entityId}\". First occurrence kept.", EntityKind = entityKind, EntityId = entityId });
            }
            else
            {
                seen[entityId] = entityKind;
            }
        }

        CheckId("Kit", kit.Id);
        foreach (var t in kit.Types)
        {
            CheckId("Type", t.Id);
            foreach (var connector in t.Connectors) CheckId("Connector", connector.Id);
            foreach (var representation in t.Representations) CheckId("Representation", representation.Id);
        }
        foreach (var d in kit.Designs)
        {
            CheckId("Design", d.Id);
            foreach (var p in d.Pieces) CheckId("Piece", p.Id);
            foreach (var c in d.Connections) CheckId("Connection", c.Id);

        }
        foreach (var q in kit.Qualities) CheckId("Quality", q.Id);
        foreach (var i in kit.Ports) CheckId("Port", i.Id);
        foreach (var f in kit.Files) CheckId("File", f.Id);
        foreach (var fo in kit.Folders) CheckId("Folder", fo.Id);

        var typesByParent = kit.Types.GroupBy(t => t.Parent?.Id);
        foreach (var group in typesByParent)
        {
            var nameGroups = group.GroupBy(t => t.Name ?? "");
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var t in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "type-name-unique", Message = $"Duplicate type name \"{nameGroup.Key}\" among siblings.", EntityKind = "Type", EntityId = t.Id });
                    }
                }
            }
        }

        var designsByParent = kit.Designs.GroupBy(d => d.Parent?.Id);
        foreach (var group in designsByParent)
        {
            var nameGroups = group.GroupBy(d => d.Name ?? "");
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var d in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "design-name-unique", Message = $"Duplicate design name \"{nameGroup.Key}\" among siblings.", EntityKind = "Design", EntityId = d.Id });
                    }
                }
            }
        }

        foreach (var design in kit.Designs)
        {
            var nameGroups = design.Pieces.Where(p => !string.IsNullOrEmpty(p.Name)).GroupBy(p => p.Name);
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var p in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "piece-name-unique", Message = $"Duplicate piece name \"{nameGroup.Key}\" inside design \"{design.Name}\".", EntityKind = "Piece", EntityId = p.Id });
                    }
                }
            }
        }

        foreach (var t in kit.Types)
        {
            var nameGroups = t.Connectors.Where(p => !string.IsNullOrEmpty(p.Name)).GroupBy(p => p.Name);
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var connector in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "connector-name-unique", Message = $"Duplicate connector name \"{nameGroup.Key}\" inside type \"{t.Name}\".", EntityKind = "Connector", EntityId = connector.Id });
                    }
                }
            }
        }

        foreach (var t in kit.Types)
        {
            var nameGroups = t.Representations.Where(m => !string.IsNullOrEmpty(m.Name)).GroupBy(m => m.Name);
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var entity in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "representation-name-unique", Message = $"Duplicate representation name \"{nameGroup.Key}\" inside type \"{t.Name}\".", EntityKind = "Representation", EntityId = entity.Id });
                    }
                }
            }
        }

        var qualityNameGroups = kit.Qualities.GroupBy(q => q.Name ?? "");
        foreach (var nameGroup in qualityNameGroups)
        {
            var list = nameGroup.ToList();
            if (list.Count > 1)
            {
                foreach (var q in list.Skip(1))
                {
                    issues.Add(new Issue { ConstraintId = "quality-name-unique", Message = $"Duplicate quality name \"{nameGroup.Key}\".", EntityKind = "Quality", EntityId = q.Id });
                }
            }
        }

        var portNameGroups = kit.Ports.GroupBy(i => i.Name ?? "");
        foreach (var nameGroup in portNameGroups)
        {
            var list = nameGroup.ToList();
            if (list.Count > 1)
            {
                foreach (var iface in list.Skip(1))
                {
                    issues.Add(new Issue { ConstraintId = "port-name-unique", Message = $"Duplicate port name \"{nameGroup.Key}\".", EntityKind = "Port", EntityId = iface.Id });
                }
            }
        }

        var fileNameGroups = kit.Files.GroupBy(f => f.Name ?? "");
        foreach (var nameGroup in fileNameGroups)
        {
            var list = nameGroup.ToList();
            if (list.Count > 1)
            {
                foreach (var f in list.Skip(1))
                {
                    issues.Add(new Issue { ConstraintId = "file-name-unique", Message = $"Duplicate file name \"{nameGroup.Key}\".", EntityKind = "File", EntityId = f.Id });
                }
            }
        }

        var foldersByParent = kit.Folders.GroupBy(f => f.Parent);
        foreach (var group in foldersByParent)
        {
            var nameGroups = group.GroupBy(f => f.Name ?? "");
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var fo in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "folder-name-unique", Message = $"Duplicate folder name \"{nameGroup.Key}\" among siblings.", EntityKind = "Folder", EntityId = fo.Id });
                    }
                }
            }
        }

        foreach (var design in kit.Designs)
        {
            var pathGroups = design.Layers.GroupBy(l => l.Path ?? "");
            foreach (var pathGroup in pathGroups)
            {
                var list = pathGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var layer in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "layer-path-unique", Message = $"Duplicate layer path \"{pathGroup.Key}\" inside design \"{design.Name}\".", EntityKind = "Layer", EntityId = layer.Id });
                    }
                }
            }
        }

        return new ValidationResult { Issues = issues };
    }
}

#endregion ✨ComposeValidation






#region 🖥️Weak Entities

#region 📺Coordinate
// Implementations MUST share X, Y, Z coordinate fields for spatial types.

public class Coordinate : Entity<Coordinate>
{
    public double U { get; set; }
    public double V { get; set; }

    public Coordinate Normalize()
    {
        var length = (double)Math.Sqrt(U * U + V * V);
        return new Coordinate { U = U / length, V = V / length };
    }
}

#endregion 📺Coordinate


#region 📦MoveVector
// Implementations MUST carry gap/shift/rise deltas in the piece plane frame for move operations.

public class MoveVector
{
    public double Gap { get; set; }
    public double Shift { get; set; }
    public double Rise { get; set; }
}

#endregion 📦MoveVector






#region ✖️Point
// Implementations MUST represent a 3D point with X, Y, Z coordinates.

public class Point : Entity<Point>
{
    public double X { get; set; } = 0;
    public double Y { get; set; } = 0;
    public double Z { get; set; } = 0;
}

#endregion ✖️Point





#region ↗️Vector
// Implementations MUST represent a 3D vector with X, Y, Z components.

public class Vector : Entity<Vector>
{
    public double X { get; set; } = 1;
    public double Y { get; set; }
    public double Z { get; set; } = 0;

    public static double DotProduct(Vector a, Vector b) => a.X * b.X + a.Y * b.Y + a.Z * b.Z;

    public static bool IsOrthogonal(Vector a, Vector b) => Math.Abs(DotProduct(a, b)) < Constants.Tolerance;

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (Math.Abs(X) < Constants.Tolerance && Math.Abs(Y) < Constants.Tolerance && Math.Abs(Z) < Constants.Tolerance)
        {
            isValid = false;
            errors.Add("The vector must not be the zero vector.");
        }

        if (Math.Abs(Math.Sqrt(X * X + Y * Y + Z * Z) - 1) > Constants.Tolerance)
        {
            isValid = false;
            errors.Add("The vector must be a unit vector.");
        }

        return (isValid, errors);
    }
}

#endregion ↗️Vector





#region ◻️Plane
// Implementations MUST define a 3D plane by origin and X/Y direction vectors.

public class Plane : Entity<Plane>
{
    public Point Origin { get; set; } = new();
    public Vector XAxis { get; set; } = new() { X = 1 };
    public Vector YAxis { get; set; } = new() { Y = 1 };

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        var (isValidOrigin, errorsOrigin) = Origin.Validate();
        isValid = isValid && isValidOrigin;
        errors.AddRange(errorsOrigin.Select(e => "The origin is invalid: " + e));
        var (isValidXAxis, errorsXAxis) = XAxis.Validate();
        isValid = isValid && isValidXAxis;
        errors.AddRange(errorsXAxis.Select(e => "The x-axis is invalid: " + e));
        var (isValidYAxis, errorsYAxis) = YAxis.Validate();
        isValid = isValid && isValidYAxis;
        errors.AddRange(errorsYAxis.Select(e => "The y-axis is invalid: " + e));
        if (!Vector.IsOrthogonal(XAxis, YAxis))
        {
            isValid = false;
            errors.Add("The x-axis and y-axis must be orthogonal.");
        }

        return (isValid, errors);
    }
}

#endregion ◻️Plane





#endregion 🖥️Weak Entities






public class AttributeModification
{
    [JsonProperty("attribute")]
    public AttributeId Attribute { get; set; } = new();
    public AttributeDiff? Diff { get; set; }
}

public class AuthorModification
{
    [JsonProperty("author")]
    public AuthorId Author { get; set; } = new();
    public AuthorDiff? Diff { get; set; }
}

public class FileModification
{
    [JsonProperty("file")]
    public FileId File { get; set; } = new();
    public FileDiff? Diff { get; set; }
}

public class FolderModification
{
    [JsonProperty("folder")]
    public FolderId Folder { get; set; } = new();
    public FolderDiff? Diff { get; set; }
}

public class TagModification
{
    [JsonProperty("tag")]
    public TagId Tag { get; set; } = new();
    public TagDiff? Diff { get; set; }
}

public class ConceptModification
{
    [JsonProperty("concept")]
    public ConceptId Concept { get; set; } = new();
    public ConceptDiff? Diff { get; set; }
}

public class PortModification
{
    [JsonProperty("port")]
    public PortId Port { get; set; } = new();
    public PortDiff? Diff { get; set; }
}

public class PropModification
{
    [JsonProperty("prop")]
    public PropId Prop { get; set; } = new();
    public PropDiff? Diff { get; set; }
}

public class RepresentationModification
{
    [JsonProperty("representation")]
    public RepresentationId Representation { get; set; } = new();
    public RepresentationDiff? Diff { get; set; }
}

public class ConnectorModification
{
    [JsonProperty("connector")]
    public ConnectorId Connector { get; set; } = new();
    public ConnectorDiff? Diff { get; set; }
}

public class TypeModification
{
    [JsonProperty("type")]
    public TypeId Type { get; set; } = new();
    public TypeDiff? Diff { get; set; }
}

public class LayerModification
{
    [JsonProperty("layer")]
    public LayerId Layer { get; set; } = new();
    public LayerDiff? Diff { get; set; }
}

public class GroupModification
{
    [JsonProperty("group")]
    public GroupId Group { get; set; } = new();
    public GroupDiff? Diff { get; set; }
}

public class PieceModification
{
    [JsonProperty("piece")]
    public PieceId Piece { get; set; } = new();
    public PieceDiff? Diff { get; set; }
}

public class ConnectionModification
{
    [JsonProperty("connection")]
    public ConnectionId Connection { get; set; } = new();
    public ConnectionDiff? Diff { get; set; }
}

public class StatModification
{
    [JsonProperty("stat")]
    public StatId Stat { get; set; } = new();
    public StatDiff? Diff { get; set; }
}

public class QualityModification
{
    [JsonProperty("quality")]
    public QualityId Quality { get; set; } = new();
    public QualityDiff? Diff { get; set; }
}

public class BenchmarkModification
{
    [JsonProperty("benchmark")]
    public BenchmarkId Benchmark { get; set; } = new();
    public BenchmarkDiff? Diff { get; set; }
}

public class DesignModification
{
    [JsonProperty("design")]
    public DesignId Design { get; set; } = new();
    public DesignDiff? Diff { get; set; }
}

public class KitModification
{
    [JsonProperty("kit")]
    public KitId Kit { get; set; } = new();
    public KitDiff? Diff { get; set; }
}
/// <summary>💿Change holds the data fields for a Change record.</summary>
public class Change<TEntity, TDiff>
{
    public TDiff Forward { get; set; } = default!;
    public TDiff Backward { get; set; } = default!;
    public string? Author { get; set; }
    public DateTime? Time { get; set; }
    public TEntity? Before { get; set; }
    public TEntity? After { get; set; }
}

public class AttributeChange : Change<Attribute, AttributeDiff> { }
public class AuthorChange : Change<Author, AuthorDiff> { }
public class FileChange : Change<File, FileDiff> { }
public class FolderChange : Change<Folder, FolderDiff> { }
public class BenchmarkChange : Change<Benchmark, BenchmarkDiff> { }
public class QualityChange : Change<Quality, QualityDiff> { }
public class PortChange : Change<Port, PortDiff> { }
public class PropChange : Change<Prop, PropDiff> { }
public class TagChange : Change<Tag, TagDiff> { }
public class ConceptChange : Change<Concept, ConceptDiff> { }
public class RepresentationChange : Change<Representation, RepresentationDiff> { }
public class ConnectorChange : Change<Connector, ConnectorDiff> { }
public class TypeChange : Change<Type, TypeDiff> { }
public class LayerChange : Change<Layer, LayerDiff> { }
public class PieceChange : Change<Piece, PieceDiff> { }
public class GroupChange : Change<Group, GroupDiff> { }
public class SideChange : Change<Side, SideDiff> { }
public class ConnectionChange : Change<Connection, ConnectionDiff> { }
public class StatChange : Change<Stat, StatDiff> { }
public class DesignChange : Change<Design, DesignDiff> { }
public class KitChange : Change<Kit, KitDiff> { }

#region 🎯ComposeReport

/// <summary>📋Human-readable note on a ComposeReport.</summary>
public sealed class OperationNote
{
    [JsonProperty("code")]
    public string? Code { get; set; }
    [JsonProperty("message")]
    public string Message { get; set; } = "";
}

/// <summary>📋Canonical algorithm output: ok, diff, warnings, infos, errors.</summary>
public sealed class ComposeReport<TDiff>
{
    [JsonProperty("ok")]
    public bool Ok { get; set; }
    [JsonProperty("diff")]
    public TDiff? Diff { get; set; }
    [JsonProperty("warnings")]
    public List<OperationNote> Warnings { get; set; } = new();
    [JsonProperty("infos")]
    public List<OperationNote> Infos { get; set; } = new();
    [JsonProperty("errors")]
    public List<OperationNote> Errors { get; set; } = new();

    public static ComposeReport<TDiff> Success(TDiff diff, List<OperationNote>? warnings = null, List<OperationNote>? infos = null) =>
        new() { Ok = true, Diff = diff, Warnings = warnings ?? new List<OperationNote>(), Infos = infos ?? new List<OperationNote>(), Errors = new List<OperationNote>() };

    public static ComposeReport<TDiff> Failure(List<OperationNote> errors) =>
        new() { Ok = false, Diff = default, Warnings = new List<OperationNote>(), Infos = new List<OperationNote>(), Errors = errors };
}

#endregion 🎯ComposeReport

#region 💎Attribute
// Implementations MUST provide key-value metadata for annotating entities.

public class AttributeId : Entity<AttributeId>
{
    public string Id { get; set; } = "";

    public static implicit operator AttributeId(Attribute attribute) => new() { Id = attribute.Id };
    public static implicit operator AttributeId(AttributeDiff diff) => new() { Id = diff.Id ?? "" };
}

public class AttributeDiff : Entity<AttributeDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _key;
    private string? _value;
    private string? _definition;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Key { get => _key; set { _key = value; _setProperties.Add("Key"); } }
    public string? Value { get => _value; set { _value = value; _setProperties.Add("Value"); } }
    public string? Definition { get => _definition; set { _definition = value; _setProperties.Add("Definition"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeKey() => _setProperties.Contains("Key");
    public bool ShouldSerializeValue() => _setProperties.Contains("Value");
    public bool ShouldSerializeDefinition() => _setProperties.Contains("Definition");

    public static implicit operator AttributeDiff(AttributeId id) => new() { Id = id.Id };
    public static implicit operator AttributeDiff(Attribute attribute) => new() { Id = attribute.Id, Key = attribute.Key, Value = attribute.Value, Definition = attribute.Definition };

    public AttributeDiff MergeDiff(AttributeDiff other)
    {
        return new AttributeDiff
        {
            Id = other.Id ?? Id,
            Key = string.IsNullOrEmpty(other.Key) ? Key : other.Key,
            Value = string.IsNullOrEmpty(other.Value) ? Value : other.Value,
            Definition = string.IsNullOrEmpty(other.Definition) ? Definition : other.Definition
        };
    }
}

public class AttributesDiff : Entity<AttributesDiff>
{
    public List<AttributeId> Removed { get; set; } = new();
    public List<Attribute> Added { get; set; } = new();
    public List<AttributeModification> Modified { get; set; } = new();

    public AttributesDiff MergeDiff(AttributesDiff other)
    {
        return new AttributesDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Modified = Modified.Concat(other.Modified).ToList()
        };
    }

    public static implicit operator AttributesDiff(List<Attribute> attributes) => new() { Modified = attributes.Select(a => new AttributeModification { Attribute = a, Diff = (AttributeDiff)a }).ToList() };

    public static List<Attribute> Apply(List<Attribute> original, AttributesDiff diff)
    {
        var result = original.Where(a => !(diff.Removed?.Any(r => r.Id == a.Id) ?? false)).ToList();
        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var attr = result.FirstOrDefault(a => a.Id == update.Attribute.Id);
                if (attr != null && update.Diff != null)
                    Attribute.ApplyDiff(attr, update.Diff);
            }
        }
        if (diff.Added != null)
            result.AddRange(diff.Added);
        return result;
    }
}

public class Attribute : Entity<Attribute>
{
    public string Id { get; set; } = "";
    public string Key { get; set; } = "";
    public string? Value { get; set; }
    public string? Definition { get; set; }

    public static implicit operator Attribute(AttributeId id) => new() { Id = id.Id };
    public static implicit operator Attribute(AttributeDiff diff) => new() { Id = diff.Id ?? "", Key = diff.Key, Value = diff.Value, Definition = diff.Definition };

    public static Attribute ApplyDiff(Attribute attribute, AttributeDiff diff)
    {
        return new Attribute
        {
            Id = attribute.Id,
            Key = !string.IsNullOrEmpty(diff.Key) ? diff.Key : attribute.Key,
            Value = !string.IsNullOrEmpty(diff.Value) ? diff.Value : attribute.Value,
            Definition = !string.IsNullOrEmpty(diff.Definition) ? diff.Definition : attribute.Definition
        };
    }
    public static AttributeDiff CreateDiff(Attribute attribute)
    {
        return new AttributeDiff
        {
            Id = attribute.Id,
            Key = attribute.Key,
            Value = attribute.Value,
            Definition = attribute.Definition
        };
    }
    public static AttributeDiff InverseDiff(Attribute attribute, AttributeDiff appliedDiff)
    {
        return new AttributeDiff
        {
            Id = attribute.Id,
            Key = !string.IsNullOrEmpty(appliedDiff.Key) ? attribute.Key : "",
            Value = !string.IsNullOrEmpty(appliedDiff.Value) ? attribute.Value : "",
            Definition = !string.IsNullOrEmpty(appliedDiff.Definition) ? attribute.Definition : ""
        };
    }

    public string ToIdString() => $"{Key}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Atr({ToHumanIdString()})";
}

#endregion 💎Attribute


#region 📍Location
// Implementations MUST combine a plane with rotation and elevation for placement.

public class LocationId : Entity<LocationId>
{
    public string Id { get; set; } = "";
    public static implicit operator LocationId(Location location) => new() { Id = location.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"LocI({ToHumanIdString()})";
}

public class Location : Entity<Location>
{
    public string Id { get; set; } = "";
    public double Longitude { get; set; }
    public double Latitude { get; set; }
    public double? Altitude { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Loc({ToHumanIdString()})";
}

#endregion 📍Location






#region ✍️Author
// Implementations MUST provide author identity with name and contact.

public class AuthorId : Entity<AuthorId>
{
    public string Id { get; set; } = "";
    public static implicit operator AuthorId(Author author) => new() { Id = author.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";
}

public class ArtifactAuthor : Entity<ArtifactAuthor>
{
    public string AuthorEmail { get; set; } = "";
    public TypeId? TypeId { get; set; }
    public DesignId? DesignId { get; set; }

    public string ToIdString() => $"{AuthorEmail}#{(TypeId?.ToIdString() ?? DesignId?.ToIdString() ?? "")}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"ArtAuth({ToHumanIdString()})";

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (TypeId is null && DesignId is null)
        {
            isValid = false;
            errors.Add("Either TypeId or DesignId must be set.");
        }

        if (TypeId is not null && DesignId is not null)
        {
            isValid = false;
            errors.Add("Either TypeId or DesignId must be set, but not both.");
        }

        return (isValid, errors);
    }
}

public class AuthorDiff : Entity<AuthorDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _email;
    private List<Attribute>? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Email { get => _email; set { _email = value; _setProperties.Add("Email"); } }
    public List<Attribute>? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeEmail() => _setProperties.Contains("Email");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator AuthorDiff(Author author) => new() { Id = author.Id, Name = author.Name, Email = author.Email, Attributes = author.Attributes };

    public AuthorDiff MergeDiff(AuthorDiff other)
    {
        return new AuthorDiff
        {
            Id = other.Id ?? Id,
            Name = other.Name ?? Name,
            Email = other.Email ?? Email,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

public class Author : Entity<Author>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string Email { get; set; } = "";
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Email}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";

    public static implicit operator Author(AuthorId id) => new() { Id = id.Id };

    public override (bool, List<string>) Validate()
    {

        var (isValid, errors) = base.Validate();
        if (!Email.Contains("@"))
        {
            isValid = false;
            errors.Add("The email must contain an @.");
        }

        return (isValid, errors);
    }
}

public class AuthorsDiff : Entity<AuthorsDiff>
{
    public List<AuthorId> Removed { get; set; } = new();
    public List<Author> Added { get; set; } = new();
    public List<AuthorModification> Modified { get; set; } = new();

    public AuthorsDiff MergeDiff(AuthorsDiff other)
    {
        return new AuthorsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Modified = Modified.Concat(other.Modified).ToList()
        };
    }

    public static implicit operator AuthorsDiff(List<Author> authors) => new() { Modified = authors.Select(a => new AuthorModification { Author = a, Diff = (AuthorDiff)a }).ToList() };
}

#endregion ✍️Author






#region 📄File
// Implementations MUST reference a file with URI, MIME type, and optional content.

public class FileId : Entity<FileId>
{
    public string Id { get; set; } = "";
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"FilId({ToHumanIdString()})";

    public static implicit operator FileId(File file) => new() { Id = file.Id };
    public static implicit operator FileId(FileDiff diff) => new() { Id = diff.Id ?? "" };
}

public class FileDiff : Entity<FileDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _remote;
    private FolderId? _folder;
    private int? _size;
    private string? _hash;
    private string? _blob;
    private DateTime? _createdAt;
    private string? _createdBy;
    private DateTime? _updatedAt;
    private string? _updatedBy;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Remote { get => _remote; set { _remote = value; _setProperties.Add("Remote"); } }
    public FolderId? Folder { get => _folder; set { _folder = value; _setProperties.Add("Folder"); } }
    public int? Size { get => _size; set { _size = value; _setProperties.Add("Size"); } }
    public string? Hash { get => _hash; set { _hash = value; _setProperties.Add("Hash"); } }
    public string? Blob { get => _blob; set { _blob = value; _setProperties.Add("Blob"); } }
    public DateTime? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public string? CreatedBy { get => _createdBy; set { _createdBy = value; _setProperties.Add("CreatedBy"); } }
    public DateTime? ModificationdAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("ModificationdAt"); } }
    public string? ModificationdBy { get => _updatedBy; set { _updatedBy = value; _setProperties.Add("ModificationdBy"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeRemote() => _setProperties.Contains("Remote");
    public bool ShouldSerializeFolder() => _setProperties.Contains("Folder");
    public bool ShouldSerializeSize() => _setProperties.Contains("Size");
    public bool ShouldSerializeHash() => _setProperties.Contains("Hash");
    public bool ShouldSerializeBlob() => _setProperties.Contains("Blob");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeCreatedBy() => _setProperties.Contains("CreatedBy");
    public bool ShouldSerializeModificationdAt() => _setProperties.Contains("ModificationdAt");
    public bool ShouldSerializeModificationdBy() => _setProperties.Contains("ModificationdBy");

    public FileDiff MergeDiff(FileDiff other)
    {
        return new FileDiff
        {
            Id = other.Id ?? Id,
            Name = other.Name ?? Name,
            Remote = other.Remote ?? Remote,
            Folder = other.Folder ?? Folder,
            Size = other.Size ?? Size,
            Hash = other.Hash ?? Hash,
            Blob = other.Blob ?? Blob,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            CreatedBy = other.CreatedBy ?? CreatedBy,
            ModificationdAt = other.ModificationdAt ?? ModificationdAt,
            ModificationdBy = other.ModificationdBy ?? ModificationdBy
        };
    }
}

public class FilesDiff : Entity<FilesDiff>
{
    public List<FileId> Removed { get; set; } = new();
    public List<FileModification> Modified { get; set; } = new();
    public List<File> Added { get; set; } = new();

    public static implicit operator FilesDiff(List<File> files) => new() { Modified = files.Select(f => new FileModification { File = f, Diff = (FileDiff)f }).ToList() };
}

public class File : Entity<File>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Mime { get; set; }
    public string? Remote { get; set; }
    public FolderId? Folder { get; set; }
    public int? Size { get; set; }
    public string? Hash { get; set; }
    public string? Blob { get; set; }
    public DateTime CreatedAt { get; set; }
    public string? CreatedBy { get; set; }
    public DateTime ModificationdAt { get; set; }
    public string? ModificationdBy { get; set; }
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Name}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"Fil({ToHumanIdString()})";

    public static implicit operator File(FileId id) => new() { Id = id.Id };
    public static implicit operator File(FileDiff diff) => new() { Id = diff.Id ?? "", Name = diff.Name ?? "", Remote = diff.Remote, Folder = diff.Folder, Size = diff.Size, Hash = diff.Hash, Blob = diff.Blob, CreatedAt = diff.CreatedAt ?? default, CreatedBy = diff.CreatedBy, ModificationdAt = diff.ModificationdAt ?? default, ModificationdBy = diff.ModificationdBy };
    public static implicit operator FileDiff(File file) => new() { Id = file.Id, Name = file.Name, Remote = file.Remote, Folder = file.Folder, Size = file.Size, Hash = file.Hash, Blob = file.Blob, CreatedAt = file.CreatedAt, CreatedBy = file.CreatedBy, ModificationdAt = file.ModificationdAt, ModificationdBy = file.ModificationdBy };
}
#endregion 📄File






#region 📁Folder
// Implementations MUST reference a folder with name and optional parent.

public class FolderId : Entity<FolderId>
{
    public string Id { get; set; } = "";
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"FolderId({ToHumanIdString()})";

    public static implicit operator FolderId(Folder folder) => new() { Id = folder.Id };
    public static implicit operator FolderId(FolderDiff diff) => new() { Id = diff.Id ?? "" };
}

public class FolderDiff : Entity<FolderDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private FolderId? _parent;
    private string? _description;
    private List<Attribute>? _attributes;
    private string? _createdAt;
    private string? _createdBy;
    private string? _updatedAt;
    private string? _updatedBy;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public FolderId? Parent { get => _parent; set { _parent = value; _setProperties.Add("Parent"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public List<Attribute>? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public string? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public string? CreatedBy { get => _createdBy; set { _createdBy = value; _setProperties.Add("CreatedBy"); } }
    public string? ModificationdAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("ModificationdAt"); } }
    public string? ModificationdBy { get => _updatedBy; set { _updatedBy = value; _setProperties.Add("ModificationdBy"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeParent() => _setProperties.Contains("Parent");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeCreatedBy() => _setProperties.Contains("CreatedBy");
    public bool ShouldSerializeModificationdAt() => _setProperties.Contains("ModificationdAt");
    public bool ShouldSerializeModificationdBy() => _setProperties.Contains("ModificationdBy");

    public FolderDiff MergeDiff(FolderDiff other)
    {
        return new FolderDiff
        {
            Id = other.Id ?? Id,
            Name = other.Name ?? Name,
            Parent = other.Parent ?? Parent,
            Description = other.Description ?? Description,
            Attributes = other.Attributes ?? Attributes,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            CreatedBy = other.CreatedBy ?? CreatedBy,
            ModificationdAt = other.ModificationdAt ?? ModificationdAt,
            ModificationdBy = other.ModificationdBy ?? ModificationdBy
        };
    }
}

public class FoldersDiff : Entity<FoldersDiff>
{
    public List<FolderId> Removed { get; set; } = new();
    public List<FolderModification> Modified { get; set; } = new();
    public List<Folder> Added { get; set; } = new();

    public static implicit operator FoldersDiff(List<Folder> folders) => new() { Modified = folders.Select(f => new FolderModification { Folder = f, Diff = (FolderDiff)f }).ToList() };
}

public class Folder : Entity<Folder>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public FolderId? Parent { get; set; }
    public string? Description { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string CreatedAt { get; set; } = "";
    public string? CreatedBy { get; set; }
    public string ModificationdAt { get; set; } = "";
    public string? ModificationdBy { get; set; }

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Fol({ToHumanIdString()})";

    public static implicit operator Folder(FolderId id) => new() { Id = id.Id };
    public static implicit operator Folder(FolderDiff diff) => new() { Id = diff.Id ?? "", Name = diff.Name ?? "", Parent = diff.Parent, Description = diff.Description ?? "", Attributes = diff.Attributes ?? new(), CreatedAt = diff.CreatedAt ?? "", CreatedBy = diff.CreatedBy, ModificationdAt = diff.ModificationdAt ?? "", ModificationdBy = diff.ModificationdBy };
    public static implicit operator FolderDiff(Folder folder) => new() { Id = folder.Id, Name = folder.Name, Parent = folder.Parent, Description = folder.Description, Attributes = folder.Attributes, CreatedAt = folder.CreatedAt, CreatedBy = folder.CreatedBy, ModificationdAt = folder.ModificationdAt, ModificationdBy = folder.ModificationdBy };

    public static Folder ApplyDiff(Folder folder, FolderDiff diff)
    {
        return new Folder
        {
            Id = diff.Id ?? folder.Id,
            Name = diff.Name ?? folder.Name,
            Parent = diff.Parent ?? folder.Parent,
            Description = diff.Description ?? folder.Description,
            Attributes = diff.Attributes ?? folder.Attributes,
            CreatedAt = diff.CreatedAt ?? folder.CreatedAt,
            CreatedBy = diff.CreatedBy ?? folder.CreatedBy,
            ModificationdAt = diff.ModificationdAt ?? folder.ModificationdAt,
            ModificationdBy = diff.ModificationdBy ?? folder.ModificationdBy
        };
    }
}

#endregion 📁Folder






#region 📏Benchmark
// Implementations MUST capture benchmark metadata for performance measurement.

public class BenchmarkId : Entity<BenchmarkId>
{
    public string Id { get; set; } = "";
    public static implicit operator BenchmarkId(Benchmark benchmark) => new() { Id = benchmark.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"BmI({ToHumanIdString()})";
}

public class Benchmark : Entity<Benchmark>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Icon { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Bmk({ToHumanIdString()})";
}

public class BenchmarkDiff : Entity<BenchmarkDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _icon;
    private double? _min;
    private bool? _minExcluded;
    private double? _max;
    private bool? _maxExcluded;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public double? Min { get => _min; set { _min = value; _setProperties.Add("Min"); } }
    public bool? MinExcluded { get => _minExcluded; set { _minExcluded = value; _setProperties.Add("MinExcluded"); } }
    public double? Max { get => _max; set { _max = value; _setProperties.Add("Max"); } }
    public bool? MaxExcluded { get => _maxExcluded; set { _maxExcluded = value; _setProperties.Add("MaxExcluded"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeMin() => _setProperties.Contains("Min");
    public bool ShouldSerializeMinExcluded() => _setProperties.Contains("MinExcluded");
    public bool ShouldSerializeMax() => _setProperties.Contains("Max");
    public bool ShouldSerializeMaxExcluded() => _setProperties.Contains("MaxExcluded");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 📏Benchmark






#region 🖨️QualityKind
// Implementations MUST categorize quality metrics by kind.

[Flags]
public enum QualityKind
{
    General = 0,
    Design = 1,
    Type = 2,
    Piece = 4,
    Connection = 8,
    Connector = 16,
}

#endregion 🖨️QualityKind






#region 🔬Quality
// Implementations MUST combine kind, name, value, and unit for quality metrics.

public class QualityId : Entity<QualityId>
{
    public string Id { get; set; } = "";

    public static implicit operator QualityId(Quality quality) => new() { Id = quality.Id };
    public static implicit operator QualityId(QualityDiff diff) => new() { Id = diff.Id ?? "" };
}

public class QualityDiff : Entity<QualityDiff>
{
    public string? Id { get; set; }
    public string Key { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Uri { get; set; }
    public bool? Scalable { get; set; }
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string? SI { get; set; }
    public string? Imperial { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public double? Default { get; set; }
    public string? Formula { get; set; }
    public List<Benchmark> Benchmarks { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator QualityDiff(QualityId id) => new() { Id = id.Id };

    public static implicit operator QualityDiff(Quality quality) => new() { Id = quality.Id, Key = quality.Key, Name = quality.Name, Description = quality.Description, Uri = quality.Uri, Scalable = quality.Scalable, Kind = quality.Kind, SI = quality.SI, Imperial = quality.Imperial, Min = quality.Min, MinExcluded = quality.MinExcluded, Max = quality.Max, MaxExcluded = quality.MaxExcluded, Default = quality.Default, Formula = quality.Formula, Benchmarks = quality.Benchmarks, Attributes = quality.Attributes };
}

public class Quality : Entity<Quality>
{
    public string Id { get; set; } = "";
    public string Key { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Uri { get; set; }
    public string? Folder { get; set; }
    public bool? Scalable { get; set; }
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string? SI { get; set; }
    public string? Imperial { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public double? Default { get; set; }
    public string? Formula { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string? Unit { get; set; }
    public List<Benchmark> Benchmarks { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Quality(QualityId id) => new() { Id = id.Id };
    public static implicit operator Quality(QualityDiff diff) => new()
    {
        Id = diff.Id ?? "",
        Key = diff.Key,
        Name = diff.Name,
        Description = diff.Description,
        Uri = diff.Uri,
        Scalable = diff.Scalable,
        Kind = diff.Kind,
        SI = diff.SI,
        Imperial = diff.Imperial,
        Min = diff.Min,
        MinExcluded = diff.MinExcluded,
        Max = diff.Max,
        MaxExcluded = diff.MaxExcluded,
        Default = diff.Default,
        Formula = diff.Formula,
        Benchmarks = diff.Benchmarks,
        Attributes = diff.Attributes
    };

}

#endregion 🔬Quality






#region ⚓Port
// Implementations MUST define connection ports as typed interfaces on a type.

public class PortId : Entity<PortId>
{
    public string Id { get; set; } = "";

    public static implicit operator PortId(Port iface) => new() { Id = iface.Id };
    public static implicit operator PortId(PortDiff diff) => new() { Id = diff.Id };
}

public class PortDiff : Entity<PortDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string _id = "";
    private string? _name;
    private string? _description;
    private string? _icon;
    private List<PortId>? _compatiblePorts;
    private int? _maxChildren;
    private List<Attribute>? _attributes;

    public string Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public List<PortId>? CompatiblePorts { get => _compatiblePorts; set { _compatiblePorts = value; _setProperties.Add("CompatiblePorts"); } }
    public int? MaxChildren { get => _maxChildren; set { _maxChildren = value; _setProperties.Add("MaxChildren"); } }
    public List<Attribute>? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeCompatiblePorts() => _setProperties.Contains("CompatiblePorts");
    public bool ShouldSerializeMaxChildren() => _setProperties.Contains("MaxChildren");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator PortDiff(PortId id) => new() { Id = id.Id };
    public static implicit operator PortDiff(Port iface) => new() { Id = iface.Id, Name = iface.Name, Description = iface.Description, Icon = iface.Icon, CompatiblePorts = iface.CompatiblePorts?.Select(i => (PortId)i).ToList(), MaxChildren = iface.MaxChildren, Attributes = iface.Attributes };
}

public class PortsDiff : Entity<PortsDiff>
{
    public List<PortId> Removed { get; set; } = new();
    public List<Port> Added { get; set; } = new();
    public List<PortModification> Modified { get; set; } = new();

    public static implicit operator PortsDiff(List<Port> ports) => new() { Modified = ports.Select(i => new PortModification { Port = i, Diff = (PortDiff)i }).ToList() };
}

public class Port : Entity<Port>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public List<PortId> CompatiblePorts { get; set; } = new();
    public int? MaxChildren { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Port(PortId id) => new() { Id = id.Id };
    public static implicit operator Port(PortDiff diff) => new()
    {
        Id = diff.Id,
        Name = diff.Name ?? "",
        Description = diff.Description ?? "",
        Icon = diff.Icon ?? "",
        CompatiblePorts = diff.CompatiblePorts ?? new(),
        MaxChildren = diff.MaxChildren,
        Attributes = diff.Attributes ?? new()
    };

    public static Port ApplyDiff(Port port, PortDiff diff)
    {
        return new Port
        {
            Id = diff.Id ?? port.Id,
            Name = diff.Name ?? port.Name,
            Description = diff.Description ?? port.Description,
            Icon = diff.Icon ?? port.Icon,
            CompatiblePorts = diff.CompatiblePorts ?? port.CompatiblePorts,
            MaxChildren = diff.MaxChildren ?? port.MaxChildren,
            Attributes = diff.Attributes ?? port.Attributes
        };
    }

    public static PortDiff CreateDiff(Port port)
    {
        return new PortDiff
        {
            Id = port.Id,
            Name = port.Name,
            Description = port.Description,
            Icon = port.Icon,
            CompatiblePorts = port.CompatiblePorts,
            MaxChildren = port.MaxChildren,
            Attributes = port.Attributes
        };
    }

    public static PortDiff InverseDiff(Port port, PortDiff appliedDiff)
    {
        return new PortDiff
        {
            Id = !string.IsNullOrEmpty(appliedDiff.Id) ? port.Id : "",
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? port.Name : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? port.Description : null,
            Icon = !string.IsNullOrEmpty(appliedDiff.Icon) ? port.Icon : null,
            CompatiblePorts = appliedDiff.CompatiblePorts?.Any() == true ? port.CompatiblePorts : null,
            MaxChildren = appliedDiff.MaxChildren.HasValue ? port.MaxChildren : null,
            Attributes = appliedDiff.Attributes?.Any() == true ? port.Attributes : null
        };
    }
}

#endregion ⚓Port






#region 📊Prop
// Implementations MUST bind a property name to an expression value.

public class PropId : Entity<PropId>
{
    public string Id { get; set; } = "";
    public static implicit operator PropId(Prop prop) => new() { Id = prop.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"PrpI({ToHumanIdString()})";
}

public class Prop : Entity<Prop>
{
    public string Id { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string Value { get; set; } = "";
    public string? Unit { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Prp({ToHumanIdString()})";
}

public class PropDiff : Entity<PropDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private QualityId? _quality;
    private string? _value;
    private string? _unit;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public QualityId? Quality { get => _quality; set { _quality = value; _setProperties.Add("Quality"); } }
    public string? Value { get => _value; set { _value = value; _setProperties.Add("Value"); } }
    public string? Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeQuality() => _setProperties.Contains("Quality");
    public bool ShouldSerializeValue() => _setProperties.Contains("Value");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 📊Prop






#region 🏷️Tag
// Implementations MUST provide lightweight labels for categorizing entities.

public class TagId : Entity<TagId>
{
    public string Id { get; set; } = "";

    public static implicit operator TagId(Tag tag) => new() { Id = tag.Id };
}

public class Tag : Entity<Tag>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Tag(TagId id) => new() { Id = id.Id };

    public static Tag Find(List<Tag> tags, string id)
    {
        var tag = tags.FirstOrDefault(t => t.Id == id);
        if (tag == null) throw new Exception($"Tag {id} not found in tags");
        return tag;
    }
}

public class TagDiff : Entity<TagDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _description;
    private string? _icon;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

public class TagsDiff : Entity<TagsDiff>
{
    public List<TagId> Removed { get; set; } = new();
    public List<Tag> Added { get; set; } = new();
    public List<TagModification> Modified { get; set; } = new();
}

#endregion 🏷️Tag






#region 💡Concept
// Implementations MUST link a semantic concept name to description and icon.

public class ConceptId : Entity<ConceptId>
{
    public string Id { get; set; } = "";

    public static implicit operator ConceptId(Concept concept) => new() { Id = concept.Id };
}

public class Concept : Entity<Concept>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Concept(ConceptId id) => new() { Id = id.Id };

    public static Concept Find(List<Concept> concepts, string id)
    {
        var concept = concepts.FirstOrDefault(c => c.Id == id);
        if (concept == null) throw new Exception($"Concept {id} not found in concepts");
        return concept;
    }
}

public class ConceptDiff : Entity<ConceptDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _description;
    private string? _icon;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

public class ConceptsDiff : Entity<ConceptsDiff>
{
    public List<ConceptId> Removed { get; set; } = new();
    public List<Concept> Added { get; set; } = new();
    public List<ConceptModification> Modified { get; set; } = new();

    public ConceptsDiff MergeDiff(ConceptsDiff other)
    {
        return new ConceptsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Modified = Modified.Concat(other.Modified).ToList()
        };
    }
}

#endregion 💡Concept






#region 🗿Representation
// Implementations MUST reference a 3D representation with URI, MIME type, and local plane.

public class RepresentationId : Entity<RepresentationId>
{
    public string Id { get; set; } = "";
    public static implicit operator RepresentationId(Representation representation) => new() { Id = representation.Id };
    public static implicit operator RepresentationId(RepresentationDiff diff) => new() { Id = diff.Id ?? "" };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Id}";
    public override string ToString() => $"Rep({ToHumanIdString()})";
}

public class RepresentationDiff : Entity<RepresentationDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private FileId? _file;
    private string? _description;
    private List<TagId> _tags = new();
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public FileId? File { get => _file; set { _file = value; _setProperties.Add("File"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public List<TagId> Tags { get => _tags; set { _tags = value; _setProperties.Add("Tags"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeFile() => _setProperties.Contains("File");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeTags() => _setProperties.Contains("Tags");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator RepresentationDiff(RepresentationId id) => new() { Id = id.Id };
    public static implicit operator RepresentationDiff(Representation representation) => new() { Id = representation.Id, Name = representation.Name, File = representation.File, Description = representation.Description, Tags = representation.Tags, Attributes = representation.Attributes };

    public RepresentationDiff MergeDiff(RepresentationDiff other)
    {
        return new RepresentationDiff
        {
            Id = other.Id ?? Id,
            Name = string.IsNullOrEmpty(other.Name) ? Name : other.Name,
            File = other.File ?? File,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Tags = other.Tags.Any() ? other.Tags : Tags,
            Attributes = other.Attributes != null ? other.Attributes : Attributes
        };
    }
}

public class RepresentationsDiff : Entity<RepresentationsDiff>
{
    public List<RepresentationId> Removed { get; set; } = new();
    public List<Representation> Added { get; set; } = new();
    public List<RepresentationModification> Modified { get; set; } = new();

    public RepresentationsDiff MergeDiff(RepresentationsDiff other)
    {
        return new RepresentationsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Modified = Modified.Concat(other.Modified).ToList()
        };
    }

    public static implicit operator RepresentationsDiff(List<Representation> representations) => new() { Modified = representations.Select(r => new RepresentationModification { Representation = r, Diff = (RepresentationDiff)r }).ToList() };
}

public class Representation : Entity<Representation>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public FileId File { get; set; } = new();
    public string? Description { get; set; }
    public List<TagId> Tags { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Representation(RepresentationId id) => new() { Id = id.Id };
    public static implicit operator Representation(RepresentationDiff diff) => new() { Id = diff.Id ?? "", Name = diff.Name ?? "", File = diff.File ?? new(), Description = diff.Description, Tags = diff.Tags, Attributes = diff.Attributes?.Added ?? new() };

    public static Representation ApplyDiff(Representation representation, RepresentationDiff diff)
    {
        return new Representation
        {
            Id = representation.Id,
            Name = string.IsNullOrEmpty(diff.Name) ? representation.Name : diff.Name,
            File = diff.File ?? representation.File,
            Description = string.IsNullOrEmpty(diff.Description) ? representation.Description : diff.Description,
            Tags = diff.Tags?.Any() == true ? diff.Tags : representation.Tags,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(representation.Attributes, diff.Attributes) : representation.Attributes
        };
    }

    public static RepresentationDiff CreateDiff(Representation representation)
    {
        return new RepresentationDiff
        {
            Id = representation.Id,
            Name = representation.Name,
            File = representation.File,
            Description = representation.Description,
            Tags = representation.Tags,
            Attributes = representation.Attributes
        };
    }

    public static RepresentationDiff InverseDiff(Representation representation, RepresentationDiff appliedDiff)
    {
        return new RepresentationDiff
        {
            Id = representation.Id,
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? representation.Name : null,
            File = appliedDiff.File != null ? representation.File : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? representation.Description : "",
            Tags = appliedDiff.Tags.Any() ? representation.Tags : new List<TagId>(),
            Attributes = appliedDiff.Attributes != null ? representation.Attributes : null
        };
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public string ToIdString() => $"{Id}";

    public string ToHumanIdString() => $"{Name}";

    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();

    public override string ToString() => $"Mod({ToHumanIdString()})";

    public static Representation Find(List<Representation> representations, List<string> tagIds)
    {
        var representation = representations.FirstOrDefault(m => tagIds.All(id => m.Tags.Any(t => t.Id == id)));
        if (representation == null) throw new Exception($"Representation with tags {string.Join(", ", tagIds)} not found in representations");
        return representation;
    }
}

#endregion 🗿Representation






#region 🔌Connector
// Implementations MUST define located interface points on a type.

public class ConnectorId : Entity<ConnectorId>
{
    public string Id { get; set; } = "";
    public static implicit operator ConnectorId(Connector connector) => new() { Id = connector.Id };
    public static implicit operator ConnectorId(ConnectorDiff diff) => new() { Id = diff.Id ?? "" };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"Por({ToHumanIdString()})";
}

public class ConnectorDiff : Entity<ConnectorDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _description;
    private PortId? _port;
    private bool? _mandatory;
    private int? _maxChildren;
    private double? _t;
    private Point? _point;
    private Vector? _direction;
    private List<Prop>? _props;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public PortId? Port { get => _port; set { _port = value; _setProperties.Add("Port"); } }
    public bool? Mandatory { get => _mandatory; set { _mandatory = value; _setProperties.Add("Mandatory"); } }
    public int? MaxChildren { get => _maxChildren; set { _maxChildren = value; _setProperties.Add("MaxChildren"); } }
    public double? T { get => _t; set { _t = value; _setProperties.Add("T"); } }
    public Point? Point { get => _point; set { _point = value; _setProperties.Add("Point"); } }
    public Vector? Direction { get => _direction; set { _direction = value; _setProperties.Add("Direction"); } }
    public List<Prop>? Props { get => _props; set { _props = value; _setProperties.Add("Props"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializePort() => _setProperties.Contains("Port");
    public bool ShouldSerializeMandatory() => _setProperties.Contains("Mandatory");
    public bool ShouldSerializeMaxChildren() => _setProperties.Contains("MaxChildren");
    public bool ShouldSerializeT() => _setProperties.Contains("T");
    public bool ShouldSerializePoint() => _setProperties.Contains("Point");
    public bool ShouldSerializeDirection() => _setProperties.Contains("Direction");
    public bool ShouldSerializeProps() => _setProperties.Contains("Props");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator ConnectorDiff(ConnectorId id) => new() { Id = id.Id };
    public static implicit operator ConnectorDiff(Connector connector) => new() { Id = connector.Id, Description = connector.Description, Port = connector.Port, Mandatory = connector.Mandatory, MaxChildren = connector.MaxChildren, T = connector.T, Point = connector.Point, Direction = connector.Direction, Props = connector.Props };

    public ConnectorDiff MergeDiff(ConnectorDiff other)
    {
        return new ConnectorDiff
        {
            Id = other.Id ?? Id,
            Description = other.Description ?? Description,
            Port = other.Port ?? Port,
            Mandatory = other.Mandatory ?? Mandatory,
            MaxChildren = other.MaxChildren ?? MaxChildren,
            T = other.T ?? T,
            Point = other.Point ?? Point,
            Direction = other.Direction ?? Direction,
            Props = other.Props ?? Props,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

public class ConnectorsDiff : Entity<ConnectorsDiff>
{
    public List<ConnectorId> Removed { get; set; } = new();
    public List<Connector> Added { get; set; } = new();
    public List<ConnectorModification> Modified { get; set; } = new();

    public ConnectorsDiff MergeDiff(ConnectorsDiff other)
    {
        return new ConnectorsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Modified = Modified.Concat(other.Modified).ToList()
        };
    }

    public static implicit operator ConnectorsDiff(List<Connector> connectors) => new() { Modified = connectors.Select(p => new ConnectorModification { Connector = p, Diff = (ConnectorDiff)p }).ToList() };
}

public class Connector : Entity<Connector>
{
    public string Id { get; set; } = "";
    public string? Name { get; set; }
    public string? Description { get; set; }
    public bool? Mandatory { get; set; }
    public int? MaxChildren { get; set; }
    public PortId? Port { get; set; }
    public Point? Point { get; set; } = null;
    public Vector? Direction { get; set; } = null;
    public double T { get; set; } = 0;
    public List<Prop> Props { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Por({ToHumanIdString()})";

    public static implicit operator Connector(ConnectorId id) => new() { Id = id.Id };
    public static implicit operator Connector(ConnectorDiff diff) => new() { Id = diff.Id ?? "", Name = diff.Name, Description = diff.Description, Port = diff.Port, Mandatory = diff.Mandatory, MaxChildren = diff.MaxChildren, T = diff.T ?? 0, Point = diff.Point, Direction = diff.Direction, Attributes = diff.Attributes?.Added ?? new() };
    public static implicit operator string(Connector connector) => connector.Id;
    public static implicit operator Connector(string id) => new() { Id = id };

    public static Connector ApplyDiff(Connector connector, ConnectorDiff diff)
    {
        return new Connector
        {
            Id = diff.Id ?? connector.Id,
            Name = diff.Name ?? connector.Name,
            Description = diff.Description ?? connector.Description,
            Port = diff.Port ?? connector.Port,
            Mandatory = diff.Mandatory ?? connector.Mandatory,
            MaxChildren = diff.MaxChildren ?? connector.MaxChildren,
            T = diff.T ?? connector.T,
            Point = diff.Point ?? connector.Point,
            Direction = diff.Direction ?? connector.Direction,
            Props = diff.Props ?? connector.Props,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(connector.Attributes, diff.Attributes) : connector.Attributes
        };
    }

    public static ConnectorDiff CreateDiff(Connector connector)
    {
        return new ConnectorDiff
        {
            Id = connector.Id,
            Name = connector.Name,
            Description = connector.Description,
            Port = connector.Port,
            Mandatory = connector.Mandatory,
            MaxChildren = connector.MaxChildren,
            T = connector.T,
            Point = connector.Point,
            Direction = connector.Direction,
            Props = connector.Props,
            Attributes = connector.Attributes
        };
    }

    public static ConnectorDiff InverseDiff(Connector connector, ConnectorDiff appliedDiff)
    {
        return new ConnectorDiff
        {
            Id = !string.IsNullOrEmpty(appliedDiff.Id) ? connector.Id : "",
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? connector.Name : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? connector.Description : "",
            Port = appliedDiff.Port is not null ? connector.Port : null,
            Mandatory = appliedDiff.Mandatory.HasValue ? connector.Mandatory : null,
            MaxChildren = appliedDiff.MaxChildren.HasValue ? connector.MaxChildren : null,
            T = appliedDiff.T.HasValue ? connector.T : null,
            Point = appliedDiff.Point is not null ? connector.Point : null,
            Direction = appliedDiff.Direction is not null ? connector.Direction : null,
            Props = appliedDiff.Props?.Any() == true ? connector.Props : new List<Prop>(),
            Attributes = appliedDiff.Attributes != null ? connector.Attributes : null
        };
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (Point is not null)
        {
            var (isValidPoint, errorsPoint) = Point.Validate();
            isValid = isValid && isValidPoint;
            errors.AddRange(errorsPoint.Select(e => "The point is invalid: " + e));
        }
        else
        {
            isValid = false;
            errors.Add("The point must not be null.");
        }
        if (Direction is not null)
        {
            var (isValidDirection, errorsDirection) = Direction.Validate();
            isValid = isValid && isValidDirection;
            errors.AddRange(errorsDirection.Select(e => "The direction is invalid: " + e));
        }
        else
        {
            isValid = false;
            errors.Add("The direction must not be null.");
        }
        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }
        return (isValid, errors);
    }

    public bool IsCompatibleWith(Connector otherConnector)
    {
        if (Port is null || otherConnector.Port is null) return true;
        if (Port.Id == otherConnector.Port.Id) return true;
        return false;
    }

    public bool IsCompatibleWith(Connector otherConnector, Kit kit)
    {
        if (Port is null || otherConnector.Port is null) return true;
        if (Port.Id == otherConnector.Port.Id) return true;

        var thisPort = kit.Ports?.FirstOrDefault(i => i.Id == Port.Id);
        var otherPort = kit.Ports?.FirstOrDefault(i => i.Id == otherConnector.Port.Id);

        if (thisPort is null || otherPort is null) return false;

        if (thisPort.CompatiblePorts?.Count == 0 || otherPort.CompatiblePorts?.Count == 0) return true;

        return thisPort.CompatiblePorts?.Any(ci => ci.Id == otherConnector.Port.Id) == true ||
               otherPort.CompatiblePorts?.Any(ci => ci.Id == Port.Id) == true;
    }

    public static bool IsSameAs(Connector connector, Connector other)
    {
        return Utility.Normalize(connector.Id) == Utility.Normalize(other.Id);
    }

    public static string FindAttributeValue(Connector connector, string name, string defaultValue = "")
    {
        var attribute = connector.Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in connector {connector.Id}");
        return attribute?.Value ?? defaultValue;
    }

    public static Connector SetAttribute(Connector connector, Attribute attribute)
    {
        var attributes = new List<Attribute>(connector.Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Connector
        {
            Id = connector.Id,
            Name = connector.Name,
            Description = connector.Description,
            Mandatory = connector.Mandatory,
            Port = connector.Port,
            Point = connector.Point,
            Direction = connector.Direction,
            T = connector.T,
            Props = connector.Props,
            Attributes = attributes
        };
    }

    public static Connector Find(List<Connector> connectors, string connectorId)
    {
        var connector = connectors.FirstOrDefault(p => p.Id == connectorId);
        if (connector == null) throw new Exception($"Connector {connectorId} not found in connectors");
        return connector;
    }

    public static Connector FindInType(Type type, string connectorId)
    {
        return Find(type.Connectors ?? new List<Connector>(), connectorId);
    }

    public static Connector? FindForPieceInConnection(Type type, Connection connection, string pieceId)
    {
        string? connectorId = connection.Parent.Piece.Id == pieceId ? connection.Parent.Connector?.Id : connection.Child.Connector?.Id;
        if (string.IsNullOrEmpty(connectorId)) return null;
        return FindInType(type, connectorId);
    }
}

#endregion 🔌Connector






#region 🏛️Typology
// 🏛️Typology owns types and designs; families remain at kit root for port compatibility.

public class TypologyId : Entity<TypologyId>
{
    public string Id { get; set; } = "";
    public static implicit operator TypologyId(Typology topo) => new() { Id = topo.Id };
    public static implicit operator TypologyId(string id) => new() { Id = id };
}

public class TypologyDiff : Entity<TypologyDiff>
{
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Folder { get; set; }
    public TypesDiff? Types { get; set; }
    public DesignsDiff? Designs { get; set; }
}

public class TypologiesDiff : Entity<TypologiesDiff>
{
    public List<TypologyId> Removed { get; set; } = new();
    public List<Typology> Added { get; set; } = new();
    public List<TypologyModification> Modified { get; set; } = new();
}

public class TypologyModification
{
    public TypologyId Typology { get; set; } = new();
    public TypologyDiff Diff { get; set; } = new();
}

public class Typology : Entity<Typology>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Folder { get; set; }
    public List<Type> Types { get; set; } = new();
    public List<Design> Designs { get; set; } = new();
}

#endregion 🏛️Typology

#region 🧱Type
// Implementations MUST compose ports, connectors, and representations into a parametric type.

public class TypeId : Entity<TypeId>
{
    public string Id { get; set; } = "";
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Id}";
    public override string ToString() => $"Typ({ToHumanIdString()})";
    public static implicit operator TypeId(Type type) => new() { Id = type.Id };
    public static implicit operator TypeId(TypeDiff diff) => new() { Id = diff.Id ?? "" };
}

public class TypeDiff : Entity<TypeDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private TypeId? _parent;
    private bool? _isAbstract;
    private string? _folder;
    private string? _description;
    private string? _icon;
    private string? _image;
    private int? _stock;
    private bool? _virtual;
    private string _uri = "";
    private string _unit = "";
    private Location? _location;
    private RepresentationsDiff? _representations;
    private ConnectorsDiff? _connectors;
    private List<AuthorId>? _authors;
    private AttributesDiff? _attributes;
    private List<ConceptId>? _concepts;
    private DateTime? _createdAt;
    private DateTime? _updatedAt;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public TypeId? Parent { get => _parent; set { _parent = value; _setProperties.Add("Parent"); } }
    public bool? IsAbstract { get => _isAbstract; set { _isAbstract = value; _setProperties.Add("IsAbstract"); } }
    public string? Folder { get => _folder; set { _folder = value; _setProperties.Add("Folder"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public string? Image { get => _image; set { _image = value; _setProperties.Add("Image"); } }
    public int? Stock { get => _stock; set { _stock = value; _setProperties.Add("Stock"); } }
    public bool? Virtual { get => _virtual; set { _virtual = value; _setProperties.Add("Virtual"); } }
    public string Uri { get => _uri; set { _uri = value; _setProperties.Add("Uri"); } }
    public string Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public Location? Location { get => _location; set { _location = value; _setProperties.Add("Location"); } }
    public RepresentationsDiff? Representations { get => _representations; set { _representations = value; _setProperties.Add("Representations"); } }
    public ConnectorsDiff? Connectors { get => _connectors; set { _connectors = value; _setProperties.Add("Connectors"); } }
    public List<AuthorId>? Authors { get => _authors; set { _authors = value; _setProperties.Add("Authors"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public List<ConceptId>? Concepts { get => _concepts; set { _concepts = value; _setProperties.Add("Concepts"); } }
    public DateTime? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public DateTime? ModificationdAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("ModificationdAt"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeParent() => _setProperties.Contains("Parent");
    public bool ShouldSerializeIsAbstract() => _setProperties.Contains("IsAbstract");
    public bool ShouldSerializeFolder() => _setProperties.Contains("Folder");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeImage() => _setProperties.Contains("Image");
    public bool ShouldSerializeStock() => _setProperties.Contains("Stock");
    public bool ShouldSerializeVirtual() => _setProperties.Contains("Virtual");
    public bool ShouldSerializeUri() => _setProperties.Contains("Uri");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeLocation() => _setProperties.Contains("Location");
    public bool ShouldSerializeRepresentations() => _setProperties.Contains("Representations");
    public bool ShouldSerializeConnectors() => _setProperties.Contains("Connectors");
    public bool ShouldSerializeAuthors() => _setProperties.Contains("Authors");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeConcepts() => _setProperties.Contains("Concepts");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeModificationdAt() => _setProperties.Contains("ModificationdAt");

    public TypeDiff MergeDiff(TypeDiff other)
    {
        return new TypeDiff
        {
            Name = string.IsNullOrEmpty(other.Name) ? Name : other.Name,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Icon = string.IsNullOrEmpty(other.Icon) ? Icon : other.Icon,
            Image = string.IsNullOrEmpty(other.Image) ? Image : other.Image,
            Stock = other.Stock ?? Stock,
            Virtual = other.Virtual ?? Virtual,
            Uri = string.IsNullOrEmpty(other.Uri) ? Uri : other.Uri,
            Unit = string.IsNullOrEmpty(other.Unit) ? Unit : other.Unit,
            Location = other.Location ?? Location,
            Representations = other.Representations is not null ? (other.Representations.MergeDiff(Representations ?? new RepresentationsDiff())) : Representations,
            Connectors = other.Connectors is not null ? (other.Connectors.MergeDiff(Connectors ?? new ConnectorsDiff())) : Connectors,
            Authors = other.Authors is not null && other.Authors.Any() ? other.Authors : Authors,
            Attributes = other.Attributes is not null ? other.Attributes.MergeDiff(Attributes ?? new AttributesDiff()) : Attributes,
            Concepts = other.Concepts is not null && other.Concepts.Any() ? other.Concepts : Concepts
        };
    }

    public static implicit operator TypeDiff(TypeId id) => new() { Id = id.Id };
    public static implicit operator TypeDiff(Type type) => new() { Name = type.Name, Description = type.Description, Icon = type.Icon, Image = type.Image, Stock = type.Stock, Virtual = type.Virtual, Uri = type.Uri, Unit = type.Unit, Location = type.Location, Representations = new RepresentationsDiff { Added = new List<Representation>(), Removed = new List<RepresentationId>(), Modified = type.Representations.Select(m => new RepresentationModification { Representation = m, Diff = Representation.CreateDiff(m) }).ToList() }, Connectors = new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Modified = type.Connectors.Select(p => new ConnectorModification { Connector = p, Diff = Connector.CreateDiff(p) }).ToList() }, Authors = type.Authors, Concepts = type.Concepts };
}

public class TypesDiff : Entity<TypesDiff>
{
    public List<TypeId> Removed { get; set; } = new();
    public List<Type> Added { get; set; } = new();
    public List<TypeModification> Modified { get; set; } = new();

    public static implicit operator TypesDiff(List<Type> types) => new() { Modified = types.Select(t => new TypeModification { Type = t, Diff = (TypeDiff)t }).ToList() };
}

public class Type : Entity<Type>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public TypologyId Typology { get; set; } = new();
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public int? Stock { get; set; }
    public bool? Virtual { get; set; }
    public string? Uri { get; set; }
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public List<Representation> Representations { get; set; } = new();
    public List<Connector> Connectors { get; set; } = new();
    public List<Prop> Props { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public List<ConceptId> Concepts { get; set; } = new();
    public DateTime CreatedAt { get; set; }
    public DateTime ModificationdAt { get; set; }

    public string ToIdString() => $"{Id}";

    public string ToHumanIdString() => $"{Name}";

    public override string ToString() => $"Typ({ToHumanIdString()})";

    public static implicit operator Type(TypeId id) => new() { Id = id.Id, CreatedAt = DateTime.UtcNow, ModificationdAt = DateTime.UtcNow };
    public static implicit operator Type(TypeDiff diff) => new()
    {
        Id = diff.Id ?? "",
        Name = diff.Name ?? "",
        Parent = diff.Parent,
        IsAbstract = diff.IsAbstract,
        Folder = diff.Folder,
        Description = diff.Description,
        Icon = diff.Icon,
        Image = diff.Image,
        Stock = diff.Stock,
        Virtual = diff.Virtual,
        Uri = diff.Uri,
        Unit = diff.Unit,
        Location = diff.Location,
        Representations = diff.Representations?.Added ?? new(),
        Connectors = diff.Connectors?.Added ?? new(),
        Authors = diff.Authors ?? new(),
        Attributes = diff.Attributes?.Added ?? new(),
        Concepts = diff.Concepts ?? new(),
        CreatedAt = diff.CreatedAt ?? DateTime.UtcNow,
        ModificationdAt = diff.ModificationdAt ?? DateTime.UtcNow
    };
    public static implicit operator string(Type type) => type.Name;
    public static implicit operator Type(string name) => new() { Name = name, CreatedAt = DateTime.UtcNow, ModificationdAt = DateTime.UtcNow };

    public static Type ApplyDiff(Type type, TypeDiff diff)
    {
        var representations = type.Representations;
        var connectors = type.Connectors;

        if (diff.Representations is not null)
            representations = ApplyRepresentationsDiff(type.Representations, diff.Representations);
        if (diff.Connectors is not null)
            connectors = ApplyConnectorsDiff(type.Connectors, diff.Connectors);

        return new Type
        {
            Id = type.Id,
            Name = string.IsNullOrEmpty(diff.Name) ? type.Name : diff.Name,
            Description = diff.Description ?? type.Description,
            Icon = diff.Icon ?? type.Icon,
            Image = diff.Image ?? type.Image,
            Stock = diff.Stock ?? type.Stock,
            Virtual = diff.Virtual ?? type.Virtual,
            Uri = diff.Uri ?? type.Uri,
            Unit = diff.Unit ?? type.Unit,
            Location = diff.Location ?? type.Location,
            Representations = representations,
            Connectors = connectors,
            Authors = diff.Authors is not null && diff.Authors.Any() ? diff.Authors : type.Authors,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(type.Attributes, diff.Attributes) : type.Attributes,
            Concepts = diff.Concepts is not null && diff.Concepts.Any() ? diff.Concepts : type.Concepts,
            Props = type.Props,
            CreatedAt = type.CreatedAt,
            ModificationdAt = DateTime.UtcNow
        };
    }

    private static List<Representation> ApplyRepresentationsDiff(List<Representation> original, RepresentationsDiff diff)
    {
        var result = original.Where(m => !diff.Removed.Any(r => r.Id == m.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(m => m.Id == updated.Representation.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = Representation.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static List<Connector> ApplyConnectorsDiff(List<Connector> original, ConnectorsDiff diff)
    {
        var result = original.Where(p => !diff.Removed.Any(r => r.Id == p.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(p => p.Id == updated.Connector.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = Connector.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    public static TypeDiff CreateDiff(Type type)
    {
        return new TypeDiff
        {
            Id = type.Id,
            Name = type.Name,
            Description = type.Description,
            Icon = type.Icon,
            Image = type.Image,
            Stock = type.Stock,
            Virtual = type.Virtual,
            Uri = type.Uri,
            Unit = type.Unit,
            Location = type.Location,
            Representations = new RepresentationsDiff { Added = new List<Representation>(), Removed = new List<RepresentationId>(), Modified = type.Representations.Select(m => new RepresentationModification { Representation = m, Diff = Representation.CreateDiff(m) }).ToList() },
            Connectors = new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Modified = type.Connectors.Select(p => new ConnectorModification { Connector = p, Diff = Connector.CreateDiff(p) }).ToList() },
            Authors = type.Authors,
            Attributes = type.Attributes,
            Concepts = type.Concepts
        };
    }

    public static TypeDiff InverseDiff(Type type, TypeDiff appliedDiff)
    {
        return new TypeDiff
        {
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? type.Name : "",
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? type.Description : "",
            Icon = !string.IsNullOrEmpty(appliedDiff.Icon) ? type.Icon : "",
            Image = !string.IsNullOrEmpty(appliedDiff.Image) ? type.Image : "",
            Stock = appliedDiff.Stock.HasValue ? type.Stock : null,
            Virtual = appliedDiff.Virtual.HasValue ? type.Virtual : null,
            Uri = !string.IsNullOrEmpty(appliedDiff.Uri) ? type.Uri : "",
            Unit = !string.IsNullOrEmpty(appliedDiff.Unit) ? type.Unit : "",
            Location = appliedDiff.Location is not null ? type.Location : null,
            Representations = appliedDiff.Representations is not null ? new RepresentationsDiff { Added = new List<Representation>(), Removed = new List<RepresentationId>(), Modified = type.Representations.Select(m => new RepresentationModification { Representation = m, Diff = Representation.CreateDiff(m) }).ToList() } : null,
            Connectors = appliedDiff.Connectors is not null ? new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Modified = type.Connectors.Select(p => new ConnectorModification { Connector = p, Diff = Connector.CreateDiff(p) }).ToList() } : null,
            Authors = appliedDiff.Authors is not null && appliedDiff.Authors.Any() ? type.Authors : null,
            Attributes = appliedDiff.Attributes is not null ? type.Attributes : null
        };
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var connector in Connectors)
        {
            var (isValidConnector, errorsPort) = connector.Validate();
            isValid = isValid && isValidConnector;
            errors.AddRange(errorsPort.Select(e => $"A connector({connector.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var representation in Representations)
        {
            var (isValidRepresentation, errorsRepresentation) = representation.Validate();
            isValid = isValid && isValidRepresentation;
            errors.AddRange(errorsRepresentation.Select(e =>
                $"A representation({representation.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var author in Authors)
        {
            var (isValidAuthor, errorsAuthor) = author.Validate();
            isValid = isValid && isValidAuthor;
            errors.AddRange(errorsAuthor.Select(e => $"An author({author.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public static Dictionary<string, Type> EnumerableToDict(IEnumerable<Type> types)
    {
        var typesDict = new Dictionary<string, Type>();
        foreach (var type in types)
        {
            typesDict[type.Name] = type;
        }

        return typesDict;
    }

    public static bool IsSameAs(Type type, Type other)
    {
        return type.Name == other.Name;
    }

    public Connector FindConnector(string connectorId)
    {
        var connector = Connectors?.FirstOrDefault(p => Utility.Normalize(p.Id) == Utility.Normalize(connectorId));
        if (connector is null) throw new InvalidOperationException($"Connector {connectorId} not found in type {Name}");
        return connector;
    }

    public Representation FindRepresentation(List<string> tags)
    {
        if (Representations == null || Representations.Count == 0)
            throw new ArgumentException($"No representations available in type {Name}");

        var indices = Representations.Select(r => Utility.Jaccard(r.Tags.Select(t => t.Id), tags)).ToList();
        var maxIndex = indices.Max();
        var maxIndexIndex = indices.IndexOf(maxIndex);
        return Representations[maxIndexIndex];
    }

    public string FindAttributeValue(string name, string defaultValue = "")
    {
        var attribute = Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in type {Name}");
        return attribute?.Value ?? defaultValue;
    }

    public Type SetAttribute(Attribute attribute)
    {
        var attributes = new List<Attribute>(Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Type
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Stock = Stock,
            Virtual = Virtual,
            Location = Location,
            Unit = Unit,
            Representations = Representations,
            Connectors = Connectors,
            Props = Props,
            Authors = Authors,
            Attributes = attributes
        };
    }
}

#endregion 🧱Type






#region 🎨Layer
// Implementations MUST organize pieces into named layers within a design.

public class LayerId : Entity<LayerId>
{
    public string Id { get; set; } = "";
    public static implicit operator LayerId(Layer layer) => new() { Id = layer.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"LyrI({ToHumanIdString()})";
}

public class Layer : Entity<Layer>
{
    public string Id { get; set; } = "";
    public string Path { get; set; } = "";
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    public string? Color { get; set; }
    public string? Description { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Path}";
    public override string ToString() => $"Lyr({ToHumanIdString()})";
}

public class LayerDiff : Entity<LayerDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _path;
    private bool? _isHidden;
    private bool? _isLocked;
    private string? _color;
    private string? _description;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Path { get => _path; set { _path = value; _setProperties.Add("Path"); } }
    public bool? IsHidden { get => _isHidden; set { _isHidden = value; _setProperties.Add("IsHidden"); } }
    public bool? IsLocked { get => _isLocked; set { _isLocked = value; _setProperties.Add("IsLocked"); } }
    public string? Color { get => _color; set { _color = value; _setProperties.Add("Color"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializePath() => _setProperties.Contains("Path");
    public bool ShouldSerializeIsHidden() => _setProperties.Contains("IsHidden");
    public bool ShouldSerializeIsLocked() => _setProperties.Contains("IsLocked");
    public bool ShouldSerializeColor() => _setProperties.Contains("Color");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 🎨Layer






#region 🧩Piece
// Implementations MUST place an instantiated type within a design hierarchy.

public class PieceId : Entity<PieceId>
{
    public string Id { get; set; } = "";
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator PieceId(PieceDiff diff) => new() { Id = diff.Id ?? "" };
    public static implicit operator PieceId(Piece piece) => new() { Id = piece.Id };
}

public class PiecesDiff : Entity<PiecesDiff>
{
    public List<PieceId> Removed { get; set; } = new();
    public List<PieceModification> Modified { get; set; } = new();
    public List<Piece> Added { get; set; } = new();

    public PiecesDiff MergeDiff(PiecesDiff other)
    {
        return new PiecesDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Modified = other.Modified.Concat(Modified).GroupBy(m => m.Piece.Id).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Id).Select(g => g.Last()).ToList()
        };
    }

    public static implicit operator PiecesDiff(List<Piece> pieces) => new() { Modified = pieces.Select(p => new PieceModification { Piece = p, Diff = Piece.CreateDiff(p) }).ToList() };
}

public class PieceDiff : Entity<PieceDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _description;
    private TypeId? _type;
    private DesignId? _design;
    private Plane? _plane;
    private Coordinate? _center;
    private double? _scale;
    private Plane? _mirrorPlane;
    private bool? _isHidden;
    private bool? _isLocked;
    private string? _color;
    private List<Prop>? _props;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public TypeId? Type { get => _type; set { _type = value; _setProperties.Add("Type"); } }
    public DesignId? Design { get => _design; set { _design = value; _setProperties.Add("Design"); } }
    public Plane? Plane { get => _plane; set { _plane = value; _setProperties.Add("Plane"); } }
    public Coordinate? Center { get => _center; set { _center = value; _setProperties.Add("Center"); } }
    public double? Scale { get => _scale; set { _scale = value; _setProperties.Add("Scale"); } }
    public Plane? MirrorPlane { get => _mirrorPlane; set { _mirrorPlane = value; _setProperties.Add("MirrorPlane"); } }
    public bool? IsHidden { get => _isHidden; set { _isHidden = value; _setProperties.Add("IsHidden"); } }
    public bool? IsLocked { get => _isLocked; set { _isLocked = value; _setProperties.Add("IsLocked"); } }
    public string? Color { get => _color; set { _color = value; _setProperties.Add("Color"); } }
    public List<Prop>? Props { get => _props; set { _props = value; _setProperties.Add("Props"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeType() => _setProperties.Contains("Type");
    public bool ShouldSerializeDesign() => _setProperties.Contains("Design");
    public bool ShouldSerializePlane() => _setProperties.Contains("Plane");
    public bool ShouldSerializeCenter() => _setProperties.Contains("Center");
    public bool ShouldSerializeScale() => _setProperties.Contains("Scale");
    public bool ShouldSerializeMirrorPlane() => _setProperties.Contains("MirrorPlane");
    public bool ShouldSerializeIsHidden() => _setProperties.Contains("IsHidden");
    public bool ShouldSerializeIsLocked() => _setProperties.Contains("IsLocked");
    public bool ShouldSerializeColor() => _setProperties.Contains("Color");
    public bool ShouldSerializeProps() => _setProperties.Contains("Props");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator PieceDiff(PieceId id) => new() { Id = id.Id };
    public static implicit operator PieceDiff(Piece piece) => new() { Id = piece.Id, Name = piece.Name, Description = piece.Description, Type = piece.Type, Design = piece.Design, Plane = piece.Plane, Center = piece.Center, Scale = piece.Scale, MirrorPlane = piece.MirrorPlane, IsHidden = piece.IsHidden, IsLocked = piece.IsLocked, Color = piece.Color, Props = piece.Props, Attributes = piece.Attributes };
}

public class Piece : Entity<Piece>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public TypeId? Type { get; set; }
    public DesignId? Design { get; set; }
    public Plane? Plane { get; set; }
    public Coordinate? Center { get; set; }
    public double? Scale { get; set; }
    public Plane? MirrorPlane { get; set; }
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    public string? Color { get; set; }
    public List<Prop>? Props { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Id}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator Piece(PieceId id) => new() { Id = id.Id };
    public static implicit operator Piece(PieceDiff diff) => new() { Id = diff.Id ?? "", Name = diff.Name ?? "", Description = diff.Description ?? "", Type = diff.Type, Design = diff.Design, Plane = diff.Plane, Center = diff.Center, Scale = diff.Scale, MirrorPlane = diff.MirrorPlane, IsHidden = diff.IsHidden, IsLocked = diff.IsLocked, Color = diff.Color, Props = diff.Props, Attributes = diff.Attributes?.Added ?? new() };

    public static Piece ApplyDiff(Piece piece, PieceDiff diff)
    {
        return new Piece
        {
            Id = diff.Id ?? piece.Id,
            Name = diff.Name ?? piece.Name,
            Description = diff.Description ?? piece.Description,
            Type = diff.Type ?? piece.Type,
            Design = diff.Design ?? piece.Design,
            Plane = diff.Plane ?? piece.Plane,
            Center = diff.Center ?? piece.Center,
            Scale = diff.Scale ?? piece.Scale,
            MirrorPlane = diff.MirrorPlane ?? piece.MirrorPlane,
            IsHidden = diff.IsHidden ?? piece.IsHidden,
            IsLocked = diff.IsLocked ?? piece.IsLocked,
            Color = diff.Color ?? piece.Color,
            Props = diff.Props ?? piece.Props,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(piece.Attributes, diff.Attributes) : piece.Attributes
        };
    }

    public static PieceDiff CreateDiff(Piece piece)
    {
        return new PieceDiff
        {
            Id = piece.Id,
            Name = piece.Name,
            Description = piece.Description,
            Type = piece.Type,
            Design = piece.Design,
            Plane = piece.Plane,
            Center = piece.Center,
            Scale = piece.Scale,
            MirrorPlane = piece.MirrorPlane,
            IsHidden = piece.IsHidden,
            IsLocked = piece.IsLocked,
            Color = piece.Color,
            Props = piece.Props,
            Attributes = piece.Attributes
        };
    }

    public static Piece Find(List<Piece> pieces, string pieceId)
    {
        var piece = pieces.FirstOrDefault(p => p.Id == pieceId);
        if (piece == null) throw new Exception($"Piece {pieceId} not found in pieces");
        return piece;
    }

    public static Piece FindInDesign(Design design, string pieceId)
    {
        return Find(design.Pieces ?? new List<Piece>(), pieceId);
    }
}

#endregion 🧩Piece






#region 👥Group
// Implementations MUST group pieces by name within a design.

public class GroupId : Entity<GroupId>
{
    public string Id { get; set; } = "";
    public static implicit operator GroupId(Group group) => new() { Id = group.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"GrpI({ToHumanIdString()})";
}

public class Group : Entity<Group>
{
    public string Id { get; set; } = "";
    public string? Name { get; set; }
    public string? Description { get; set; }
    public List<PieceId> Pieces { get; set; } = new();
    public string? Color { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Name ?? Id}";
    public override string ToString() => $"Grp({ToHumanIdString()})";
}

public class GroupDiff : Entity<GroupDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _description;
    private List<PieceId>? _pieces;
    private string? _color;
    private AttributesDiff? _attributes;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public List<PieceId>? Pieces { get => _pieces; set { _pieces = value; _setProperties.Add("Pieces"); } }
    public string? Color { get => _color; set { _color = value; _setProperties.Add("Color"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializePieces() => _setProperties.Contains("Pieces");
    public bool ShouldSerializeColor() => _setProperties.Contains("Color");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 👥Group





#region ↔️Side
// Implementations MUST reference a piece and connector as a connection endpoint.

public class SideDiff : Entity<SideDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private PieceId? _piece;
    private PieceId? _designPiece = null;
    private ConnectorId? _connector;
    private string? _description;

    public PieceId? Piece { get => _piece; set { _piece = value; _setProperties.Add("Piece"); } }
    public PieceId? DesignPiece { get => _designPiece; set { _designPiece = value; _setProperties.Add("DesignPiece"); } }
    public ConnectorId? Connector { get => _connector; set { _connector = value; _setProperties.Add("Connector"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }

    public bool ShouldSerializePiece() => _setProperties.Contains("Piece");
    public bool ShouldSerializeDesignPiece() => _setProperties.Contains("DesignPiece");
    public bool ShouldSerializeConnector() => _setProperties.Contains("Connector");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");

    public static implicit operator SideDiff(Side side) => new() { Piece = side.Piece, DesignPiece = side.DesignPiece, Connector = side.Connector };

    public SideDiff MergeDiff(SideDiff other)
    {
        return new SideDiff
        {
            Piece = other.Piece ?? Piece,
            DesignPiece = other.DesignPiece ?? DesignPiece,
            Connector = other.Connector ?? Connector,
            Description = other.Description ?? Description
        };
    }
}

public class Side : Entity<Side>
{
    public PieceId Piece { get; set; } = new();
    public PieceId? DesignPiece { get; set; } = null;
    public ConnectorId Connector { get; set; } = new();

    public static implicit operator Side(SideDiff diff) => new() { Piece = diff.Piece ?? new(), DesignPiece = diff.DesignPiece, Connector = diff.Connector ?? new() };

    public static Side ApplyDiff(Side side, SideDiff diff)
    {
        return new Side
        {
            Piece = diff.Piece ?? side.Piece,
            DesignPiece = diff.DesignPiece ?? side.DesignPiece,
            Connector = diff.Connector ?? side.Connector
        };
    }

    public static SideDiff CreateDiff(Side side)
    {
        return new SideDiff
        {
            Piece = side.Piece,
            DesignPiece = side.DesignPiece,
            Connector = side.Connector
        };
    }

    public static SideDiff InverseDiff(Side side, SideDiff appliedDiff)
    {
        return new SideDiff
        {
            Piece = appliedDiff.Piece is not null ? side.Piece : null,
            DesignPiece = appliedDiff.DesignPiece is not null ? side.DesignPiece : null,
            Connector = appliedDiff.Connector is not null ? side.Connector : null
        };
    }

    public override bool Equals(object? obj)
    {
        if (obj is not Side other) return false;
        return Piece.Id == other.Piece.Id && DesignPiece?.Id == other.DesignPiece?.Id && Connector.Id == other.Connector.Id;
    }

    public override int GetHashCode()
    {
        unchecked
        {
            var hash = 17;
            hash = hash * 31 + (Piece.Id?.GetHashCode() ?? 0);
            hash = hash * 31 + (DesignPiece?.Id?.GetHashCode() ?? 0);
            hash = hash * 31 + (Connector.Id?.GetHashCode() ?? 0);
            return hash;
        }
    }

    public override string ToString() => $"Sde({Piece.Id}" + (Connector.Id != "" ? ":" + Connector.Id : "") + ")";
}

#endregion ↔️Side






#region 🔗Connection
// Implementations MUST link two sides to connect pieces in a design.

public class ConnectionId : Entity<ConnectionId>
{
    public string Id { get; set; } = "";
    public Side Parent { get; set; } = new();
    public Side Child { get; set; } = new();

    public string ToIdString() => $"{Parent.Piece.Id + (Parent.Connector.Id != "" ? ":" + Parent.Connector.Id : "")}--{(Child.Connector.Id != "" ? Child.Connector.Id + ":" : "") + Child.Piece.Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"ConId({ToHumanIdString()})";

    public static implicit operator ConnectionId(Connection connection) => new() { Parent = connection.Parent, Child = connection.Child };
    public static implicit operator ConnectionId(ConnectionDiff diff) => new() { Parent = diff.Parent ?? new(), Child = diff.Child ?? new() };
}

public class ConnectionDiff : Entity<ConnectionDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private SideDiff? _parentSideDiff;
    private SideDiff? _childSideDiff;
    private string? _description;
    private double? _gap;
    private double? _shift;
    private double? _rise;
    private double? _rotation;
    private double? _turn;
    private double? _tilt;
    private double? _u;
    private double? _v;
    private AttributesDiff? _attributes;

    public SideDiff? Parent { get => _parentSideDiff; set { _parentSideDiff = value; _setProperties.Add("Parent"); } }
    public SideDiff? Child { get => _childSideDiff; set { _childSideDiff = value; _setProperties.Add("Child"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public double? Gap { get => _gap; set { _gap = value; _setProperties.Add("Gap"); } }
    public double? Shift { get => _shift; set { _shift = value; _setProperties.Add("Shift"); } }
    public double? Rise { get => _rise; set { _rise = value; _setProperties.Add("Rise"); } }
    public double? Rotation { get => _rotation; set { _rotation = value; _setProperties.Add("Rotation"); } }
    public double? Turn { get => _turn; set { _turn = value; _setProperties.Add("Turn"); } }
    public double? Tilt { get => _tilt; set { _tilt = value; _setProperties.Add("Tilt"); } }
    public double? U { get => _u; set { _u = value; _setProperties.Add("U"); } }
    public double? V { get => _v; set { _v = value; _setProperties.Add("V"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeParent() => _setProperties.Contains("Parent");
    public bool ShouldSerializeChild() => _setProperties.Contains("Child");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeGap() => _setProperties.Contains("Gap");
    public bool ShouldSerializeShift() => _setProperties.Contains("Shift");
    public bool ShouldSerializeRise() => _setProperties.Contains("Rise");
    public bool ShouldSerializeRotation() => _setProperties.Contains("Rotation");
    public bool ShouldSerializeTurn() => _setProperties.Contains("Turn");
    public bool ShouldSerializeTilt() => _setProperties.Contains("Tilt");
    public bool ShouldSerializeU() => _setProperties.Contains("U");
    public bool ShouldSerializeV() => _setProperties.Contains("V");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator ConnectionDiff(ConnectionId id) => new() { Parent = new SideDiff { Piece = id.Parent.Piece, DesignPiece = id.Parent.DesignPiece, Connector = id.Parent.Connector }, Child = new SideDiff { Piece = id.Child.Piece, DesignPiece = id.Child.DesignPiece, Connector = id.Child.Connector } };
    public static implicit operator ConnectionDiff(Connection connection) => new() { Parent = Side.CreateDiff(connection.Parent), Child = Side.CreateDiff(connection.Child), Description = connection.Description, Gap = connection.Gap, Shift = connection.Shift, Rise = connection.Rise, Rotation = connection.Rotation, Turn = connection.Turn, Tilt = connection.Tilt, U = connection.U, V = connection.V, Attributes = connection.Attributes };

    public ConnectionDiff MergeDiff(ConnectionDiff other)
    {
        return new ConnectionDiff
        {
            Parent = other.Parent is not null ? (other.Parent.MergeDiff(Parent ?? new SideDiff())) : Parent,
            Child = other.Child is not null ? (other.Child.MergeDiff(Child ?? new SideDiff())) : Child,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Gap = other.Gap ?? Gap,
            Shift = other.Shift ?? Shift,
            Rise = other.Rise ?? Rise,
            Rotation = other.Rotation ?? Rotation,
            Turn = other.Turn ?? Turn,
            Tilt = other.Tilt ?? Tilt,
            U = other.U ?? U,
            V = other.V ?? V,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

public class ConnectionsDiff : Entity<ConnectionsDiff>
{
    public List<ConnectionId> Removed { get; set; } = new();
    public List<ConnectionModification> Modified { get; set; } = new();
    public List<Connection> Added { get; set; } = new();

    public static implicit operator ConnectionsDiff(List<Connection> connections) => new() { Modified = connections.Select(c => new ConnectionModification { Connection = c, Diff = (ConnectionDiff)c }).ToList() };

    public ConnectionsDiff MergeDiff(ConnectionsDiff other)
    {
        return new ConnectionsDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Modified = other.Modified.Concat(Modified).GroupBy(u => u.Connection.Id).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Parent.Piece.Id + "--" + a.Child.Piece.Id).Select(g => g.Last()).ToList()
        };
    }
}

public class Connection : Entity<Connection>
{
    public string Id { get; set; } = "";
    public Side Parent { get; set; } = new();
    public Side Child { get; set; } = new();
    public string? Description { get; set; }
    public double Gap { get; set; } = 0;
    public double Shift { get; set; } = 0;
    public double Rise { get; set; } = 0;
    public double Rotation { get; set; } = 0;
    public double Turn { get; set; } = 0;
    public double Tilt { get; set; } = 0;
    public double? U { get; set; }
    public double? V { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Parent.Piece.Id + (Parent.Connector.Id != "" ? ":" + Parent.Connector.Id : "")}--{(Child.Connector.Id != "" ? Child.Connector.Id + ":" : "") + Child.Piece.Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Con({ToHumanIdString()})";

    public static implicit operator Connection(ConnectionId id) => new() { Parent = id.Parent, Child = id.Child };
    public static implicit operator Connection(ConnectionDiff diff) => new() { Parent = diff.Parent ?? new(), Child = diff.Child ?? new(), Description = diff.Description ?? "", Gap = diff.Gap ?? 0, Shift = diff.Shift ?? 0, Rise = diff.Rise ?? 0, Rotation = diff.Rotation ?? 0, Turn = diff.Turn ?? 0, Tilt = diff.Tilt ?? 0, U = diff.U, V = diff.V, Attributes = diff.Attributes?.Added ?? new() };

    public static Connection ApplyDiff(Connection connection, ConnectionDiff diff)
    {
        return new Connection
        {
            Parent = diff.Parent is not null ? Side.ApplyDiff(connection.Parent, diff.Parent) : connection.Parent,
            Child = diff.Child is not null ? Side.ApplyDiff(connection.Child, diff.Child) : connection.Child,
            Description = string.IsNullOrEmpty(diff.Description) ? connection.Description : diff.Description,
            Gap = diff.Gap ?? connection.Gap,
            Shift = diff.Shift ?? connection.Shift,
            Rise = diff.Rise ?? connection.Rise,
            Rotation = diff.Rotation ?? connection.Rotation,
            Turn = diff.Turn ?? connection.Turn,
            Tilt = diff.Tilt ?? connection.Tilt,
            U = diff.U ?? connection.U,
            V = diff.V ?? connection.V,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(connection.Attributes, diff.Attributes) : connection.Attributes
        };
    }

    public static ConnectionDiff CreateDiff(Connection connection)
    {
        return new ConnectionDiff
        {
            Parent = Side.CreateDiff(connection.Parent),
            Child = Side.CreateDiff(connection.Child),
            Description = connection.Description,
            Gap = connection.Gap,
            Shift = connection.Shift,
            Rise = connection.Rise,
            Rotation = connection.Rotation,
            Turn = connection.Turn,
            Tilt = connection.Tilt,
            U = connection.U,
            V = connection.V,
            Attributes = connection.Attributes
        };
    }

    public static ConnectionDiff InverseDiff(Connection connection, ConnectionDiff appliedDiff)
    {
        return new ConnectionDiff
        {
            Parent = appliedDiff.Parent is not null ? Side.CreateDiff(connection.Parent) : null,
            Child = appliedDiff.Child is not null ? Side.CreateDiff(connection.Child) : null,
            Description = appliedDiff.Description is not null ? connection.Description : "",
            Gap = appliedDiff.Gap.HasValue ? connection.Gap : null,
            Shift = appliedDiff.Shift.HasValue ? connection.Shift : null,
            Rise = appliedDiff.Rise.HasValue ? connection.Rise : null,
            Rotation = appliedDiff.Rotation.HasValue ? connection.Rotation : null,
            Turn = appliedDiff.Turn.HasValue ? connection.Turn : null,
            Tilt = appliedDiff.Tilt.HasValue ? connection.Tilt : null,
            U = appliedDiff.U.HasValue ? connection.U : null,
            V = appliedDiff.V.HasValue ? connection.V : null,
            Attributes = appliedDiff.Attributes is not null ? connection.Attributes : null
        };
    }

    public static bool IsSameAs(Connection connection, Connection other, bool strict = false)
    {
        if (other is null) return false;
        if (strict)
        {
            return connection.Parent.Piece.Id == other.Parent.Piece.Id &&
                   connection.Parent.Connector.Id == other.Parent.Connector.Id &&
                   connection.Child.Piece.Id == other.Child.Piece.Id &&
                   connection.Child.Connector.Id == other.Child.Connector.Id;
        }
        return connection.Parent.Piece.Id == other.Parent.Piece.Id &&
               connection.Child.Piece.Id == other.Child.Piece.Id;
    }

    public static Connection SetAttribute(Connection connection, Attribute attribute)
    {
        var attributes = new List<Attribute>(connection.Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Connection
        {
            Parent = connection.Parent,
            Child = connection.Child,
            Description = connection.Description,
            Gap = connection.Gap,
            Shift = connection.Shift,
            Rise = connection.Rise,
            Rotation = connection.Rotation,
            Turn = connection.Turn,
            Tilt = connection.Tilt,
            U = connection.U,
            V = connection.V,
            Attributes = attributes
        };
    }

    public static Connection Find(List<Connection> connections, string connectionId)
    {
        var connection = connections.FirstOrDefault(c => c.Id == connectionId);
        if (connection == null) throw new Exception($"Connection {connectionId} not found in connections");
        return connection;
    }

    public static List<Connection> FindByPiece(List<Connection> connections, string pieceId)
    {
        return connections.Where(c => c.Parent.Piece.Id == pieceId || c.Child.Piece.Id == pieceId).ToList();
    }

    public static Connection FindInDesign(Design design, string connectionId)
    {
        return Find(design.Connections ?? new List<Connection>(), connectionId);
    }

    public static List<Connection> FindManyInDesign(Design design, List<string> connectionIds)
    {
        return connectionIds.Select(g => FindInDesign(design, g)).ToList();
    }

    public static List<Connection> FindByPieceInDesign(Design design, string pieceId)
    {
        return FindByPiece(design.Connections ?? new List<Connection>(), pieceId);
    }

    public static (Piece connecting, Piece connected) FindPiecesInDesign(Design design, Connection connection)
    {
        return (
            Piece.FindInDesign(design, connection.Child.Piece.Id),
            Piece.FindInDesign(design, connection.Parent.Piece.Id)
        );
    }

    public static List<Connection> FindStaleInDesign(Design design)
    {
        return (design.Connections ?? new List<Connection>()).Where(c =>
        {
            try
            {
                Piece.FindInDesign(design, c.Parent.Piece.Id);
                Piece.FindInDesign(design, c.Child.Piece.Id);
                return false;
            }
            catch
            {
                return true;
            }
        }).ToList();
    }
}

#endregion 🔗Connection






#region 📈Stat
// Implementations MUST associate statistical metrics with a design.

public class StatId : Entity<StatId>
{
    public string Id { get; set; } = "";
    public static implicit operator StatId(Stat stat) => new() { Id = stat.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"SttI({ToHumanIdString()})";
}

public class Stat : Entity<Stat>
{
    public string Id { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string? Unit { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Stt({ToHumanIdString()})";
}

public class StatDiff : Entity<StatDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private QualityId? _quality;
    private string? _unit;
    private double? _min;
    private bool? _minExcluded;
    private double? _max;
    private bool? _maxExcluded;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public QualityId? Quality { get => _quality; set { _quality = value; _setProperties.Add("Quality"); } }
    public string? Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public double? Min { get => _min; set { _min = value; _setProperties.Add("Min"); } }
    public bool? MinExcluded { get => _minExcluded; set { _minExcluded = value; _setProperties.Add("MinExcluded"); } }
    public double? Max { get => _max; set { _max = value; _setProperties.Add("Max"); } }
    public bool? MaxExcluded { get => _maxExcluded; set { _maxExcluded = value; _setProperties.Add("MaxExcluded"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeQuality() => _setProperties.Contains("Quality");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeMin() => _setProperties.Contains("Min");
    public bool ShouldSerializeMinExcluded() => _setProperties.Contains("MinExcluded");
    public bool ShouldSerializeMax() => _setProperties.Contains("Max");
    public bool ShouldSerializeMaxExcluded() => _setProperties.Contains("MaxExcluded");
}

#endregion 📈Stat






#region 📐Design
// Implementations MUST compose pieces, connections, and metadata into a layout.

public class DesignsDiff : Entity<DesignsDiff>
{
    public List<DesignId> Removed { get; set; } = new();
    public List<DesignModification> Modified { get; set; } = new();
    public List<Design> Added { get; set; } = new();

    public static implicit operator DesignsDiff(List<Design> designs) => new() { Modified = designs.Select(d => new DesignModification { Design = d, Diff = (DesignDiff)d }).ToList() };
}

public class DesignDiff : Entity<DesignDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private DesignId? _parent;
    private bool? _isAbstract;
    private string? _folder;
    private string? _description;
    private string? _icon;
    private string? _image;
    private Location? _location;
    private string? _unit;
    private bool? _canScale;
    private bool? _canMirror;
    private LayerId? _activeLayer;
    private PiecesDiff? _pieces;
    private ConnectionsDiff? _connections;
    private List<Prop>? _props;
    private List<Stat>? _stats;
    private List<Layer>? _layers;
    private List<Group>? _groups;
    private List<AuthorId>? _authors;
    private List<ConceptId>? _concepts;
    private AttributesDiff? _attributes;
    private DateTime? _createdAt;
    private DateTime? _updatedAt;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public DesignId? Parent { get => _parent; set { _parent = value; _setProperties.Add("Parent"); } }
    public bool? IsAbstract { get => _isAbstract; set { _isAbstract = value; _setProperties.Add("IsAbstract"); } }
    public string? Folder { get => _folder; set { _folder = value; _setProperties.Add("Folder"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public string? Image { get => _image; set { _image = value; _setProperties.Add("Image"); } }
    public Location? Location { get => _location; set { _location = value; _setProperties.Add("Location"); } }
    public string? Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public bool? CanScale { get => _canScale; set { _canScale = value; _setProperties.Add("CanScale"); } }
    public bool? CanMirror { get => _canMirror; set { _canMirror = value; _setProperties.Add("CanMirror"); } }
    public LayerId? ActiveLayer { get => _activeLayer; set { _activeLayer = value; _setProperties.Add("ActiveLayer"); } }
    public PiecesDiff? Pieces { get => _pieces; set { _pieces = value; _setProperties.Add("Pieces"); } }
    public ConnectionsDiff? Connections { get => _connections; set { _connections = value; _setProperties.Add("Connections"); } }
    public List<Prop>? Props { get => _props; set { _props = value; _setProperties.Add("Props"); } }
    public List<Stat>? Stats { get => _stats; set { _stats = value; _setProperties.Add("Stats"); } }
    public List<Layer>? Layers { get => _layers; set { _layers = value; _setProperties.Add("Layers"); } }
    public List<Group>? Groups { get => _groups; set { _groups = value; _setProperties.Add("Groups"); } }
    public List<AuthorId>? Authors { get => _authors; set { _authors = value; _setProperties.Add("Authors"); } }
    public List<ConceptId>? Concepts { get => _concepts; set { _concepts = value; _setProperties.Add("Concepts"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public DateTime? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public DateTime? ModificationdAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("ModificationdAt"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeParent() => _setProperties.Contains("Parent");
    public bool ShouldSerializeIsAbstract() => _setProperties.Contains("IsAbstract");
    public bool ShouldSerializeFolder() => _setProperties.Contains("Folder");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeImage() => _setProperties.Contains("Image");
    public bool ShouldSerializeLocation() => _setProperties.Contains("Location");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeCanScale() => _setProperties.Contains("CanScale");
    public bool ShouldSerializeCanMirror() => _setProperties.Contains("CanMirror");
    public bool ShouldSerializeActiveLayer() => _setProperties.Contains("ActiveLayer");
    public bool ShouldSerializePieces() => _setProperties.Contains("Pieces");
    public bool ShouldSerializeConnections() => _setProperties.Contains("Connections");
    public bool ShouldSerializeProps() => _setProperties.Contains("Props");
    public bool ShouldSerializeStats() => _setProperties.Contains("Stats");
    public bool ShouldSerializeLayers() => _setProperties.Contains("Layers");
    public bool ShouldSerializeGroups() => _setProperties.Contains("Groups");
    public bool ShouldSerializeAuthors() => _setProperties.Contains("Authors");
    public bool ShouldSerializeConcepts() => _setProperties.Contains("Concepts");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeModificationdAt() => _setProperties.Contains("ModificationdAt");

    public static implicit operator DesignDiff(DesignId id) => new() { Id = id.Id };
    public static implicit operator DesignDiff(Design design) => new() { Id = design.Id, Name = design.Name, Parent = design.Parent, IsAbstract = design.IsAbstract, Folder = design.Folder, Description = design.Description, Icon = design.Icon, Image = design.Image, Location = design.Location, Unit = design.Unit, CanScale = design.CanScale, CanMirror = design.CanMirror, ActiveLayer = design.ActiveLayer, Pieces = new PiecesDiff { Removed = new List<PieceId>(), Modified = design.Pieces.Select(p => new PieceModification { Piece = p, Diff = Piece.CreateDiff(p) }).ToList(), Added = new List<Piece>() }, Connections = new ConnectionsDiff { Removed = new List<ConnectionId>(), Modified = design.Connections.Select(c => new ConnectionModification { Connection = c, Diff = Connection.CreateDiff(c) }).ToList(), Added = new List<Connection>() }, Props = design.Props, Stats = design.Stats, Layers = design.Layers, Groups = design.Groups, Authors = design.Authors, Concepts = design.Concepts, Attributes = design.Attributes, CreatedAt = design.CreatedAt, ModificationdAt = design.ModificationdAt };

    public DesignDiff MergeDiff(DesignDiff other)
    {
        return new DesignDiff
        {
            Id = other.Id ?? Id,
            Name = other.Name ?? Name,
            Parent = other.Parent ?? Parent,
            IsAbstract = other.IsAbstract ?? IsAbstract,
            Folder = other.Folder ?? Folder,
            Description = other.Description ?? Description,
            Icon = other.Icon ?? Icon,
            Image = other.Image ?? Image,
            Location = other.Location ?? Location,
            Unit = other.Unit ?? Unit,
            CanScale = other.CanScale ?? CanScale,
            CanMirror = other.CanMirror ?? CanMirror,
            ActiveLayer = other.ActiveLayer ?? ActiveLayer,
            Pieces = other.Pieces is not null ? (other.Pieces.MergeDiff(Pieces ?? new PiecesDiff())) : Pieces,
            Connections = other.Connections is not null ? (other.Connections.MergeDiff(Connections ?? new ConnectionsDiff())) : Connections,
            Props = other.Props ?? Props,
            Stats = other.Stats ?? Stats,
            Layers = other.Layers ?? Layers,
            Groups = other.Groups ?? Groups,
            Authors = other.Authors ?? Authors,
            Concepts = other.Concepts ?? Concepts,
            Attributes = other.Attributes ?? Attributes,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            ModificationdAt = other.ModificationdAt ?? ModificationdAt
        };
    }
}

public class DesignId : Entity<DesignId>
{
    public string Id { get; set; } = "";
    public static implicit operator DesignId(Design design) => new() { Id = design.Id };
    public static implicit operator DesignId(DesignDiff diff) => new() { Id = diff.Id ?? "" };

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Id}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"DsnId({ToHumanIdString()})";
}

public class Design : Entity<Design>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public TypologyId Typology { get; set; } = new();
    public DesignId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public List<ConceptId> Concepts { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public List<Layer> Layers { get; set; } = new();
    public LayerId? ActiveLayer { get; set; }
    public List<Piece> Pieces { get; set; } = new();
    public List<Group> Groups { get; set; } = new();
    public List<Connection> Connections { get; set; } = new();
    public List<Prop> Props { get; set; } = new();
    public List<Stat> Stats { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public DateTime CreatedAt { get; set; }
    public DateTime ModificationdAt { get; set; }

    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Dsn({ToHumanIdString()})";

    public static implicit operator Design(DesignId id) => new() { Id = id.Id, CreatedAt = DateTime.UtcNow, ModificationdAt = DateTime.UtcNow };
    public static implicit operator Design(DesignDiff diff) => new() { Id = diff.Id ?? "", Name = diff.Name ?? "", Parent = diff.Parent, IsAbstract = diff.IsAbstract, Folder = diff.Folder, Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Location = diff.Location, Unit = diff.Unit ?? "", CanScale = diff.CanScale, CanMirror = diff.CanMirror, ActiveLayer = diff.ActiveLayer, Attributes = diff.Attributes?.Added ?? new(), Authors = diff.Authors ?? new(), Concepts = diff.Concepts ?? new(), CreatedAt = diff.CreatedAt ?? DateTime.UtcNow, ModificationdAt = diff.ModificationdAt ?? DateTime.UtcNow };
    public static implicit operator string(Design design) => design.Name;
    public static implicit operator Design(string name) => new() { Name = name, CreatedAt = DateTime.UtcNow, ModificationdAt = DateTime.UtcNow };

    public static Design ApplyDiff(Design design, DesignDiff diff)
    {
        var pieces = design.Pieces;
        var connections = design.Connections;

        if (diff.Pieces is not null)
        {
            pieces = ApplyPiecesDiff(design.Pieces, diff.Pieces);
        }
        if (diff.Connections is not null)
        {
            connections = ApplyConnectionsDiff(design.Connections, diff.Connections);
        }

        return new Design
        {
            Id = diff.Id ?? design.Id,
            Name = diff.Name ?? design.Name,
            Parent = diff.Parent ?? design.Parent,
            IsAbstract = diff.IsAbstract ?? design.IsAbstract,
            Folder = diff.Folder ?? design.Folder,
            Description = diff.Description ?? design.Description,
            Icon = diff.Icon ?? design.Icon,
            Image = diff.Image ?? design.Image,
            Location = diff.Location ?? design.Location,
            Unit = diff.Unit ?? design.Unit,
            ActiveLayer = diff.ActiveLayer ?? design.ActiveLayer,
            Pieces = pieces,
            Connections = connections,
            Props = diff.Props ?? design.Props,
            Stats = diff.Stats ?? design.Stats,
            Layers = diff.Layers ?? design.Layers,
            Groups = diff.Groups ?? design.Groups,
            CanScale = diff.CanScale ?? design.CanScale,
            CanMirror = diff.CanMirror ?? design.CanMirror,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(design.Attributes, diff.Attributes) : design.Attributes,
            Authors = diff.Authors ?? design.Authors,
            Concepts = diff.Concepts ?? design.Concepts,
            CreatedAt = diff.CreatedAt ?? design.CreatedAt,
            ModificationdAt = diff.ModificationdAt ?? design.ModificationdAt
        };
    }

    public static DesignDiff CreateDiff(Design design)
    {
        return new DesignDiff
        {
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new PiecesDiff
            {
                Removed = new List<PieceId>(),
                Modified = design.Pieces.Select(p => new PieceModification { Piece = p, Diff = Piece.CreateDiff(p) }).ToList(),
                Added = new List<Piece>()
            },
            Connections = new ConnectionsDiff
            {
                Removed = new List<ConnectionId>(),
                Modified = design.Connections.Select(c => new ConnectionModification { Connection = c, Diff = Connection.CreateDiff(c) }).ToList(),
                Added = new List<Connection>()
            },
            Stats = design.Stats,
            Authors = design.Authors,
            Attributes = design.Attributes,
            Concepts = design.Concepts
        };
    }

    public static DesignDiff GetDesignDiff(Design design, Design other)
    {
        var diff = new DesignDiff();

        if (design.Name != other.Name) diff.Name = other.Name;
        if (design.Description != other.Description) diff.Description = other.Description;
        if (design.Icon != other.Icon) diff.Icon = other.Icon;
        if (design.Image != other.Image) diff.Image = other.Image;
        if (design.Unit != other.Unit) diff.Unit = other.Unit;
        if (design.Folder != other.Folder) diff.Folder = other.Folder;
        if (design.IsAbstract != other.IsAbstract) diff.IsAbstract = other.IsAbstract;
        if (design.CanScale != other.CanScale) diff.CanScale = other.CanScale;
        if (design.CanMirror != other.CanMirror) diff.CanMirror = other.CanMirror;
        if (design.ActiveLayer != other.ActiveLayer) diff.ActiveLayer = other.ActiveLayer;
        if ((design.Parent?.Id ?? "") != (other.Parent?.Id ?? "")) diff.Parent = other.Parent;
        if ((design.Location?.Id ?? "") != (other.Location?.Id ?? "")) diff.Location = other.Location;

        var piecesDiff = CreatePiecesDiff(design.Pieces, other.Pieces);
        if (piecesDiff.Removed.Any() || piecesDiff.Modified.Any() || piecesDiff.Added.Any())
            diff.Pieces = piecesDiff;

        var connectionsDiff = CreateConnectionsDiff(design.Connections, other.Connections);
        if (connectionsDiff.Removed.Any() || connectionsDiff.Modified.Any() || connectionsDiff.Added.Any())
            diff.Connections = connectionsDiff;

        return diff;
    }

    private static List<Piece> ApplyPiecesDiff(List<Piece> original, PiecesDiff diff)
    {
        var result = original.Where(p => !diff.Removed.Any(r => r.Id == p.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(p => p.Id == updated.Piece.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = Piece.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added.Select(a => new Piece
        {
            Id = a.Id ?? "",
            Description = a.Description ?? "",
            Type = a.Type ?? new TypeId { Id = "" },
            Plane = a.Plane,
            Center = a.Center,
            Attributes = a.Attributes ?? new List<Attribute>()
        }));
        return result;
    }

    private static PiecesDiff CreatePiecesDiff(List<Piece> original, List<Piece> modified)
    {
        var originalIds = original.Select(p => p.Id).ToHashSet();
        var modifiedIds = modified.Select(p => p.Id).ToHashSet();

        return new PiecesDiff
        {
            Removed = original.Where(p => !modifiedIds.Contains(p.Id)).Select(p => new PieceId { Id = p.Id }).ToList(),
            Modified = original.Where(p => modifiedIds.Contains(p.Id))
                .SelectMany(p =>
                {
                    var modifiedPiece = modified.First(m => m.Id == p.Id);
                    var diff = Piece.CreateDiff(p);
                    return !Equals(p, modifiedPiece) ? new[] { new PieceModification { Piece = p, Diff = diff } } : Array.Empty<PieceModification>();
                })
                .ToList(),
            Added = modified.Where(p => !originalIds.Contains(p.Id)).ToList()
        };
    }

    private static List<Connection> ApplyConnectionsDiff(List<Connection> original, ConnectionsDiff diff)
    {
        var result = original.Where(c => !diff.Removed.Any(r => r.Id == c.Id)).ToList();

        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(c => c.Id == updated.Connection.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = Connection.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static ConnectionsDiff CreateConnectionsDiff(List<Connection> original, List<Connection> modified)
    {
        var originalIds = original.Select(c => c.Id).ToHashSet();
        var modifiedIds = modified.Select(c => c.Id).ToHashSet();

        return new ConnectionsDiff
        {
            Removed = original.Where(c => !modifiedIds.Contains(c.Id)).Select(c => new ConnectionId { Id = c.Id }).ToList(),
            Modified = original.Where(c => modifiedIds.Contains(c.Id))
                .SelectMany(c =>
                {
                    var modifiedConnection = modified.First(m => m.Id == c.Id);
                    var diff = Connection.CreateDiff(c);
                    return !Equals(c, modifiedConnection) ? new[] { new ConnectionModification { Connection = c, Diff = diff } } : Array.Empty<ConnectionModification>();
                })
                .ToList(),
            Added = modified.Where(c => !originalIds.Contains(c.Id)).ToList()
        };
    }

    public static void Bfs(Design design, Action<Piece> onRoot, Action<Piece, Piece, Connection> onConnection)
    {
        var pieces = design.Pieces.ToDictionary(p => p.Id);
        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var piece in design.Pieces)
            graph.AddVertex(piece.Id);
        foreach (var connection in design.Connections)
            graph.AddEdge(new Edge<string>(connection.Parent.Piece.Id, connection.Child.Piece.Id));
        var components = new Dictionary<string, int>();
        graph.ConnectedComponents(components);
        var componentPieces = new Dictionary<int, Dictionary<string, Piece>>();
        foreach (var kvp in components)
        {
            if (!componentPieces.ContainsKey(kvp.Value))
                componentPieces[kvp.Value] = new Dictionary<string, Piece>();
            componentPieces[kvp.Value][kvp.Key] = pieces[kvp.Key];
        }

        foreach (var component in componentPieces)
        {
            var subGraph = new UndirectedGraph<string, Edge<string>>();
            foreach (var piece in component.Value)
                subGraph.AddVertex(piece.Key);
            foreach (var connection in design.Connections)
                if (component.Value.ContainsKey(connection.Parent.Piece.Id) &&
                    component.Value.ContainsKey(connection.Child.Piece.Id))
                    subGraph.AddEdge(
                        new Edge<string>(connection.Parent.Piece.Id, connection.Child.Piece.Id));
            var root = subGraph.Vertices.FirstOrDefault(p => pieces[p].Plane is not null);
            if (root is null)
                root = subGraph.Vertices.First();

            onRoot(pieces[root]);

            var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(subGraph);
            bfs.SetRootVertex(root);
            bfs.TreeEdge += (g, edge) =>
            {
                var parent = pieces[edge.Source];
                var child = pieces[edge.Target];
                var connection = design.Connections.First(c =>
                    (c.Parent.Piece.Id == parent.Id && c.Child.Piece.Id == child.Id) ||
                    (c.Parent.Piece.Id == child.Id && c.Child.Piece.Id == parent.Id));
                onConnection(parent, child, connection);
            };
            bfs.Compute();
        }
    }

    public static Design Flatten(Design design, IEnumerable<Type> types,
        Func<Plane, Point, Vector, Point, Vector, double, double, double, double, double, double, Plane> computeChildPlane)
    {
        if (design.Pieces.Count > 1 && design.Connections.Count > 0)
        {
            var connectors = new Dictionary<string, Dictionary<string, Connector>>();
            foreach (var type in types)
            {
                if (!connectors.ContainsKey(type.Id))
                    connectors[type.Id] = new Dictionary<string, Connector>();
                foreach (var connector in type.Connectors)
                    connectors[type.Id][connector.Id] = connector;
            }

            foreach (var piece in design.Pieces)
            {
                if (piece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({piece.Id}) has no type.");
                if (!types.Any(t => t.Id == piece.Type.Id))
                    throw new Exception(
                        $"The type {piece.Type.ToHumanIdString()} of the piece {piece.ToHumanIdString()} is not provided.");
            }
            foreach (var connection in design.Connections)
            {
                var connectedPiece = design.Pieces.First(p => p.Id == connection.Parent.Piece.Id);
                if (connectedPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectedPiece.Id}) has no type.");
                var connectedType = types.First(t => t.Id == connectedPiece.Type.Id);
                if (!connectors[connectedType.Id].ContainsKey(connection.Parent.Connector.Id))
                    throw new Exception(
                        $"The type {connectedType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the connector {connection.Parent.Connector.Id}.");
                var connectingPiece = design.Pieces.First(p => p.Id == connection.Child.Piece.Id);
                if (connectingPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectingPiece.Id}) has no type.");
                var connectingType = types.First(t => t.Id == connectingPiece.Type.Id);
                if (!connectors[connectingType.Id].ContainsKey(connection.Child.Connector.Id))
                    throw new Exception(
                        $"The type {connectingType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the connector {connection.Child.Connector.Id}.");
            }

            var piecePlanes = new Dictionary<string, Plane>();
            var pieceCenters = new Dictionary<string, Coordinate>();
            var pieceAttributes = new Dictionary<string, List<Attribute>>();

            var onRoot = new Action<Piece>(piece =>
            {
                piecePlanes[piece.Id] = piece.Plane ?? new Plane();
                pieceCenters[piece.Id] = piece.Center ?? new Coordinate();
            });
            var onConnection = new Action<Piece, Piece, Connection>((parent, child, connection) =>
            {
                var isParentConnected = connection.Parent.Piece.Id == parent.Id;
                if (!piecePlanes.TryGetValue(parent.Id, out var parentPlane) || parent.Type is null || child.Type is null) return;
                var parentConnector =
                    connectors[parent.Type.Id][
                        isParentConnected ? connection.Parent.Connector.Id : connection.Child.Connector.Id];
                var childConnector =
                    connectors[child.Type.Id][
                        isParentConnected ? connection.Child.Connector.Id : connection.Parent.Connector.Id];
                if (parentConnector.Point is null || parentConnector.Direction is null || childConnector.Point is null || childConnector.Direction is null) return;
                var childPlane = computeChildPlane(parentPlane, parentConnector.Point, parentConnector.Direction,
                    childConnector.Point, childConnector.Direction,
                    connection.Gap, connection.Shift, connection.Rise,
                    connection.Rotation, connection.Turn, connection.Tilt);
                piecePlanes[child.Id] = childPlane;

                var radius = 2.697;
                var verticalVExtra = 1.0;
                var horizontalScale = 3.0633;
                var parentCenter = pieceCenters.TryGetValue(parent.Id, out var pc) ? pc : new Coordinate();
                var connectionU = connection.U ?? 0;
                var connectionV = connection.V ?? 0;

                double childU, childV;
                if (parentCenter.U == 0 && parentCenter.V == 0)
                {
                    var angle = 2 * Math.PI * parentConnector.T;
                    childU = radius * Math.Sin(angle);
                    childV = radius * Math.Cos(angle);
                }
                else
                {
                    var isVerticalConnection = Math.Abs(parentConnector.Direction.Z) > 0.5;
                    if (isVerticalConnection)
                    {
                        childU = parentCenter.U + connectionU;
                        childV = parentCenter.V + connectionV + verticalVExtra;
                    }
                    else
                    {
                        childU = parentCenter.U + connectionU * horizontalScale;
                        childV = parentCenter.V + connectionV * horizontalScale;
                    }
                }

                pieceCenters[child.Id] = new Coordinate { U = Math.Round(childU, 6), V = Math.Round(childV, 6) };

                var existingAttributes = new List<Attribute>(child.Attributes);
                var composeAttribute = existingAttributes.FirstOrDefault(q => q.Key == "compose.parent");
                if (composeAttribute is not null)
                {
                    var idx = existingAttributes.IndexOf(composeAttribute);
                    existingAttributes[idx] = new Attribute { Id = composeAttribute.Id, Key = composeAttribute.Key, Value = parent.Id, Definition = composeAttribute.Definition };
                }
                else
                {
                    existingAttributes.Add(new Attribute
                    {
                        Key = "compose.parent",
                        Value = parent.Id
                    });
                }
                pieceAttributes[child.Id] = existingAttributes;
            });
            Bfs(design, onRoot, onConnection);

            var newPieces = design.Pieces.Select(piece =>
            {
                var newPiece = Entity<Piece>.DeepClone(piece)!;
                if (piecePlanes.TryGetValue(piece.Id, out var plane)) newPiece.Plane = plane;
                if (pieceCenters.TryGetValue(piece.Id, out var center)) newPiece.Center = center;
                if (pieceAttributes.TryGetValue(piece.Id, out var attrs)) newPiece.Attributes = attrs;
                return newPiece;
            }).ToList();

            return new Design
            {
                Id = design.Id,
                Name = design.Name,
                Parent = design.Parent,
                IsAbstract = design.IsAbstract,
                Folder = design.Folder,
                Description = design.Description,
                Icon = design.Icon,
                Image = design.Image,
                Location = design.Location,
                Unit = design.Unit,
                ActiveLayer = design.ActiveLayer,
                CanScale = design.CanScale,
                CanMirror = design.CanMirror,
                Pieces = newPieces,
                Connections = new List<Connection>(),
                Props = design.Props,
                Stats = design.Stats,
                Layers = design.Layers,
                Groups = design.Groups,
                Authors = design.Authors,
                Concepts = design.Concepts,
                Attributes = design.Attributes,
                CreatedAt = design.CreatedAt,
                ModificationdAt = design.ModificationdAt
            };
        }

        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Parent = design.Parent,
            IsAbstract = design.IsAbstract,
            Folder = design.Folder,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            ActiveLayer = design.ActiveLayer,
            CanScale = design.CanScale,
            CanMirror = design.CanMirror,
            Pieces = new List<Piece>(design.Pieces),
            Connections = new List<Connection>(),
            Props = design.Props,
            Stats = design.Stats,
            Layers = design.Layers,
            Groups = design.Groups,
            Authors = design.Authors,
            Concepts = design.Concepts,
            Attributes = design.Attributes,
            CreatedAt = design.CreatedAt,
            ModificationdAt = design.ModificationdAt
        };
    }

    public static Design Flatten(Design design, IEnumerable<Type> types) => Flatten(design, types, DefaultComputeChildPlane);

    public static Plane DefaultComputeChildPlane(
        Plane parentPlane,
        Point parentPoint,
        Vector parentDirection,
        Point childPoint,
        Vector childDirection,
        double gap,
        double shift,
        double rise,
        double rotation,
        double turn,
        double tilt)
    {
        // All math uses double[] arrays for 64-bit precision:
        // Vec3 = double[3] {x,y,z}, Quat = double[4] {x,y,z,w}, Mat4 = double[16] row-major
        var pMatrix = PlaneToMatrix(parentPlane);

        var pPoint = new double[] { parentPoint.X, parentPoint.Y, parentPoint.Z };
        var pDir = Vec3Normalize(new double[] { parentDirection.X, parentDirection.Y, parentDirection.Z });
        var cPoint = new double[] { childPoint.X, childPoint.Y, childPoint.Z };
        var cDir = Vec3Normalize(new double[] { childDirection.X, childDirection.Y, childDirection.Z });

        var rotationRad = DegreesToRadians(rotation);
        var turnRad = DegreesToRadians(turn);
        var tiltRad = DegreesToRadians(tilt);

        var reverseChildDirection = new double[] { -cDir[0], -cDir[1], -cDir[2] };

        double[] alignQuat; // Quat {x,y,z,w}
        var cross = Vec3Cross(pDir, reverseChildDirection);
        if (Vec3LengthSquared(cross) < 0.0001)
        {
            var dotProduct = Vec3Dot(pDir, reverseChildDirection);
            if (dotProduct > 0)
            {
                alignQuat = QuatIdentity();
            }
            else
            {
                if (Math.Abs(pDir[2]) < 1e-5)
                {
                    alignQuat = QuatFromAxisAngle(UnitZ, Math.PI);
                }
                else
                {
                    var crossAxis = Vec3Cross(UnitZ, pDir);
                    double[] axis;
                    if (Vec3LengthSquared(crossAxis) < 0.0001)
                    {
                        axis = UnitX;
                    }
                    else
                    {
                        axis = Vec3Normalize(crossAxis);
                    }
                    alignQuat = QuatFromAxisAngle(axis, Math.PI);
                }
            }
        }
        else
        {
            alignQuat = CreateFromTwoVectors(reverseChildDirection, pDir);
        }

        var directionT = QuaternionToMatrix(alignQuat);

        var parentConnectorQuat = CreateFromTwoVectors(UnitY, pDir);
        var parentRotationT = QuaternionToMatrix(parentConnectorQuat);

        var gapDirection = ApplyMatrix4ToVec3(parentRotationT, UnitY);
        var shiftDirection = ApplyMatrix4ToVec3(parentRotationT, UnitX);
        var raiseDirection = ApplyMatrix4ToVec3(parentRotationT, UnitZ);
        var turnAxis = ApplyMatrix4ToVec3(parentRotationT, UnitZ);
        var tiltAxis = ApplyMatrix4ToVec3(parentRotationT, UnitX);

        var orientationT = directionT;
        var rotateT = MakeRotationAxis(pDir, -rotationRad);
        orientationT = MultiplyMatrices(rotateT, orientationT);

        turnAxis = ApplyMatrix4ToVec3(rotateT, turnAxis);
        tiltAxis = ApplyMatrix4ToVec3(rotateT, tiltAxis);

        var turnT = MakeRotationAxis(turnAxis, turnRad);
        orientationT = MultiplyMatrices(turnT, orientationT);

        var tiltT = MakeRotationAxis(tiltAxis, tiltRad);
        orientationT = MultiplyMatrices(tiltT, orientationT);

        var centerChildT = MakeTranslation(-cPoint[0], -cPoint[1], -cPoint[2]);

        var transform = MultiplyMatrices(orientationT, centerChildT);

        var gapT = MakeTranslation(gapDirection[0] * gap, gapDirection[1] * gap, gapDirection[2] * gap);
        var shiftT = MakeTranslation(shiftDirection[0] * shift, shiftDirection[1] * shift, shiftDirection[2] * shift);
        var raiseT = MakeTranslation(raiseDirection[0] * rise, raiseDirection[1] * rise, raiseDirection[2] * rise);

        var translationT = MultiplyMatrices(raiseT, MultiplyMatrices(shiftT, gapT));
        transform = MultiplyMatrices(translationT, transform);

        var moveToParentT = MakeTranslation(pPoint[0], pPoint[1], pPoint[2]);
        transform = MultiplyMatrices(moveToParentT, transform);

        var finalMatrix = MultiplyMatrices(pMatrix, transform);

        return MatrixToPlane(finalMatrix);
    }

    // double-precision unit vectors
    private static readonly double[] UnitX = new double[] { 1, 0, 0 };
    private static readonly double[] UnitY = new double[] { 0, 1, 0 };
    private static readonly double[] UnitZ = new double[] { 0, 0, 1 };

    private static double Vec3Dot(double[] a, double[] b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    private static double Vec3LengthSquared(double[] v) => v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    private static double[] Vec3Cross(double[] a, double[] b) => new double[] {
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    };
    private static double[] Vec3Normalize(double[] v)
    {
        var len = Math.Sqrt(Vec3LengthSquared(v));
        if (len < 1e-15) return new double[] { 0, 0, 0 };
        return new double[] { v[0] / len, v[1] / len, v[2] / len };
    }

    private static double[] QuatIdentity() => new double[] { 0, 0, 0, 1 }; // {x,y,z,w}
    private static double[] QuatFromAxisAngle(double[] axis, double angle)
    {
        var halfAngle = angle * 0.5;
        var s = Math.Sin(halfAngle);
        return new double[] { axis[0] * s, axis[1] * s, axis[2] * s, Math.Cos(halfAngle) };
    }
    private static double[] QuatNormalize(double[] q)
    {
        var len = Math.Sqrt(q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]);
        if (len < 1e-15) return QuatIdentity();
        return new double[] { q[0] / len, q[1] / len, q[2] / len, q[3] / len };
    }

    private static double DegreesToRadians(double deg) => deg * Math.PI / 180.0;

    private static double[] CreateFromTwoVectors(double[] u, double[] v)
    {
        // Returns quaternion {x,y,z,w} that rotates u to v
        double dot = Vec3Dot(u, v);
        if (dot > 0.999999) return QuatIdentity();
        if (dot < -0.999999)
        {
            var axis = Vec3Cross(UnitX, u);
            if (Vec3LengthSquared(axis) < 0.001)
                axis = Vec3Cross(UnitY, u);
            axis = Vec3Normalize(axis);
            return QuatFromAxisAngle(axis, Math.PI);
        }

        var axisNorm = Vec3Cross(u, v);
        var q = new double[] { axisNorm[0], axisNorm[1], axisNorm[2], 1 + dot };
        return QuatNormalize(q);
    }

    private static double[] PlaneToMatrix(Plane p)
    {
        // Returns double[16] row-major 4x4 matrix
        var origin = new double[] { p.Origin.X, p.Origin.Y, p.Origin.Z };
        var x = Vec3Normalize(new double[] { p.XAxis.X, p.XAxis.Y, p.XAxis.Z });
        var yRaw = new double[] { p.YAxis.X, p.YAxis.Y, p.YAxis.Z };

        var z = Vec3Normalize(Vec3Cross(x, yRaw));
        var y = Vec3Normalize(Vec3Cross(z, x));

        return new double[] {
            x[0], y[0], z[0], origin[0],
            x[1], y[1], z[1], origin[1],
            x[2], y[2], z[2], origin[2],
            0,     0,     0,     1
        };
    }

    private static double[] ApplyMatrix4ToVec3(double[] m, double[] v)
    {
        // m = double[16] row-major, v = double[3]
        return new double[] {
            m[0] * v[0] + m[1] * v[1] + m[2] * v[2],
            m[4] * v[0] + m[5] * v[1] + m[6] * v[2],
            m[8] * v[0] + m[9] * v[1] + m[10] * v[2]
        };
    }

    private static double[] QuaternionToMatrix(double[] q)
    {
        // q = double[4] {x,y,z,w}, returns double[16] row-major
        double x = q[0], y = q[1], z = q[2], w = q[3];
        double xx = x * x, yy = y * y, zz = z * z;
        double xy = x * y, xz = x * z, yz = y * z;
        double wx = w * x, wy = w * y, wz = w * z;

        return new double[] {
            1 - 2 * (yy + zz), 2 * (xy - wz),     2 * (xz + wy),     0,
            2 * (xy + wz),     1 - 2 * (xx + zz), 2 * (yz - wx),     0,
            2 * (xz - wy),     2 * (yz + wx),     1 - 2 * (xx + yy), 0,
            0,                 0,                 0,                 1
        };
    }

    private static double[] MakeTranslation(double x, double y, double z)
    {
        // Returns double[16] row-major translation matrix
        return new double[] {
            1, 0, 0, x,
            0, 1, 0, y,
            0, 0, 1, z,
            0, 0, 0, 1
        };
    }

    private static double[] MakeRotationAxis(double[] axis, double angle)
    {
        // axis = double[3], angle = radians, returns double[16] row-major
        double c = Math.Cos(angle);
        double s = Math.Sin(angle);
        double t = 1 - c;
        double ax = axis[0], ay = axis[1], az = axis[2];

        return new double[] {
            t * ax * ax + c,      t * ax * ay - s * az, t * ax * az + s * ay, 0,
            t * ax * ay + s * az, t * ay * ay + c,      t * ay * az - s * ax, 0,
            t * ax * az - s * ay, t * ay * az + s * ax, t * az * az + c,      0,
            0,                    0,                    0,                    1
        };
    }

    private static double[] MultiplyMatrices(double[] a, double[] b)
    {
        // a,b = double[16] row-major, returns double[16] row-major
        return new double[] {
            a[0]*b[0] + a[1]*b[4] + a[2]*b[8]  + a[3]*b[12],
            a[0]*b[1] + a[1]*b[5] + a[2]*b[9]  + a[3]*b[13],
            a[0]*b[2] + a[1]*b[6] + a[2]*b[10] + a[3]*b[14],
            a[0]*b[3] + a[1]*b[7] + a[2]*b[11] + a[3]*b[15],

            a[4]*b[0] + a[5]*b[4] + a[6]*b[8]  + a[7]*b[12],
            a[4]*b[1] + a[5]*b[5] + a[6]*b[9]  + a[7]*b[13],
            a[4]*b[2] + a[5]*b[6] + a[6]*b[10] + a[7]*b[14],
            a[4]*b[3] + a[5]*b[7] + a[6]*b[11] + a[7]*b[15],

            a[8]*b[0]  + a[9]*b[4]  + a[10]*b[8]  + a[11]*b[12],
            a[8]*b[1]  + a[9]*b[5]  + a[10]*b[9]  + a[11]*b[13],
            a[8]*b[2]  + a[9]*b[6]  + a[10]*b[10] + a[11]*b[14],
            a[8]*b[3]  + a[9]*b[7]  + a[10]*b[11] + a[11]*b[15],

            a[12]*b[0] + a[13]*b[4] + a[14]*b[8]  + a[15]*b[12],
            a[12]*b[1] + a[13]*b[5] + a[14]*b[9]  + a[15]*b[13],
            a[12]*b[2] + a[13]*b[6] + a[14]*b[10] + a[15]*b[14],
            a[12]*b[3] + a[13]*b[7] + a[14]*b[11] + a[15]*b[15]
        };
    }

    private static Plane MatrixToPlane(double[] m)
    {
        // m = double[16] row-major
        return new Plane
        {
            Origin = new Point { X = m[3], Y = m[7], Z = m[11] },
            XAxis = new Vector { X = m[0], Y = m[4], Z = m[8] },
            YAxis = new Vector { X = m[1], Y = m[5], Z = m[9] }
        };
    }

    public static Design Sort(Design design)
    {
        var sortedPieces = new List<Piece>();
        var sortedConnections = new List<Connection>();

        Bfs(design,
            piece => { sortedPieces.Add(piece); },
            (parent, child, connection) =>
            {
                sortedPieces.Add(child);
                Connection sortedConnection;
                if (connection.Parent.Piece.Id != parent.Id)
                {
                    sortedConnection = new Connection
                    {
                        Id = connection.Id,
                        Parent = new Side { Piece = new PieceId { Id = child.Id }, Connector = connection.Parent.Connector },
                        Child = new Side { Piece = new PieceId { Id = parent.Id }, Connector = connection.Child.Connector },
                        Description = connection.Description,
                        Gap = connection.Gap,
                        Shift = connection.Shift,
                        Rise = connection.Rise,
                        Rotation = connection.Rotation,
                        Turn = connection.Turn,
                        Tilt = connection.Tilt,
                        U = connection.U,
                        V = connection.V,
                        Attributes = connection.Attributes
                    };
                }
                else
                {
                    sortedConnection = connection;
                }
                sortedConnections.Add(sortedConnection);
            });

        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Parent = design.Parent,
            IsAbstract = design.IsAbstract,
            Folder = design.Folder,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            ActiveLayer = design.ActiveLayer,
            CanScale = design.CanScale,
            CanMirror = design.CanMirror,
            Pieces = sortedPieces,
            Connections = sortedConnections,
            Props = design.Props,
            Stats = design.Stats,
            Layers = design.Layers,
            Groups = design.Groups,
            Authors = design.Authors,
            Concepts = design.Concepts,
            Attributes = design.Attributes,
            CreatedAt = design.CreatedAt,
            ModificationdAt = design.ModificationdAt
        };
    }

    public static Piece? GetPiece(Design design, string id) => design.Pieces.Find(piece => piece.Id == id);
    private static Design FlatToSvgCoordinates(Design design, float iconWidth, float iconWidthMax, float margin)
    {
        var newPieces = design.Pieces.Select(piece =>
        {
            if (piece.Center is null) return piece;
            var newPiece = Entity<Piece>.DeepClone(piece)!;
            newPiece.Center = new Coordinate
            {
                U = piece.Center.U * iconWidth,
                V = -(piece.Center.V * iconWidth)
            };
            return newPiece;
        }).ToList();

        var newConnections = design.Connections.Select(connection =>
        {
            if (!connection.U.HasValue && !connection.V.HasValue) return connection;
            var newConn = Entity<Connection>.DeepClone(connection)!;
            if (connection.U.HasValue) newConn.U = connection.U * iconWidth;
            if (connection.V.HasValue) newConn.V = -(connection.V * iconWidth);
            return newConn;
        }).ToList();

        var maxIconOffset = iconWidthMax - iconWidth;
        var minX = newPieces.Where(p => p.Center is not null).Min(piece => piece.Center!.U) - (margin + maxIconOffset);
        var minY = newPieces.Where(p => p.Center is not null).Min(piece => piece.Center!.V) - (margin + maxIconOffset);
        var minXSign = Math.Sign(minX);
        var minYSign = Math.Sign(minY);
        var offsetX = minXSign == 0 ? 0 : -minX;
        var offsetY = minYSign == 0 ? 0 : -minY;

        newPieces = newPieces.Select(piece =>
        {
            if (piece.Center is null) return piece;
            var newPiece = Entity<Piece>.DeepClone(piece)!;
            newPiece.Center = new Coordinate { U = piece.Center.U + offsetX, V = piece.Center.V + offsetY };
            return newPiece;
        }).ToList();

        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Parent = design.Parent,
            IsAbstract = design.IsAbstract,
            Folder = design.Folder,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            ActiveLayer = design.ActiveLayer,
            CanScale = design.CanScale,
            CanMirror = design.CanMirror,
            Pieces = newPieces,
            Connections = newConnections,
            Props = design.Props,
            Stats = design.Stats,
            Layers = design.Layers,
            Groups = design.Groups,
            Authors = design.Authors,
            Concepts = design.Concepts,
            Attributes = design.Attributes,
            CreatedAt = design.CreatedAt,
            ModificationdAt = design.ModificationdAt
        };
    }

    public static string Diagram(
        Design design,
        IEnumerable<Type> types,
        Func<Plane, Point, Vector, Point, Vector, double, double, double, double, double, double, Plane> computeChildPlane,
        string kitDirectory = "",
        float iconWidth = 48, float iconStroke = 1f, float connectionStroke = 2f, float margin = 0)
    {
        var typesDict = Type.EnumerableToDict(types);

        var usedTypes = new List<Type>();
        foreach (var type in types)
            if (design.Pieces.Exists(piece => piece.Type is not null && piece.Type.Id == type.Id))
                usedTypes.Add(type);

        var flatCloneInSvgCoordinates = FlatToSvgCoordinates(Flatten(Entity<Design>.DeepClone(design)!, types, computeChildPlane), iconWidth, iconWidth + 2 * iconStroke, margin);

        var svgDoc = new SvgDocument
        {
            Width = (float)flatCloneInSvgCoordinates.Pieces.Where(p => p.Center is not null).Max(piece => piece.Center!.U) + margin * 2 + iconWidth +
                    2 * iconStroke,
            Height = (float)flatCloneInSvgCoordinates.Pieces.Where(p => p.Center is not null).Max(piece => piece.Center!.V) + margin * 2 + iconWidth +
                     2 * iconStroke
        };

        var defs = new SvgDefinitionList();

        var iconCircle = new SvgCircle
        {
            ID = "icon",
            CenterX = iconWidth / 2,
            CenterY = iconWidth / 2,
            Radius = iconWidth / 2 - iconStroke / 2,
            Fill = new SvgColourServer(Color.White),
            Stroke = new SvgColourServer(Color.Black),
            StrokeWidth = iconStroke
        };
        defs.Children.Add(iconCircle);

        var root = new SvgCircle
        {
            ID = "root",
            CenterX = iconWidth / 2,
            CenterY = iconWidth / 2,
            Radius = iconWidth / 2 + iconStroke,
            Fill = new SvgColourServer(Color.White),
            Stroke = new SvgColourServer(Color.Black),
            StrokeWidth = iconStroke
        };
        defs.Children.Add(root);

        var iconMask = new SvgMask
        {
            ID = "iconMask",
            Children =
            {
                new SvgCircle
                {
                    CenterX = iconWidth / 2 - iconStroke,
                    CenterY = iconWidth / 2 - iconStroke,
                    Radius = iconWidth / 2 - iconStroke,
                    Fill = new SvgColourServer(Color.White)
                }
            }
        };
        defs.Children.Add(iconMask);

        foreach (var type in usedTypes)
        {
            var typeDef = new SvgGroup
            {
                ID = type.ToIdString()
            };
            var icon = type.Icon;
            var iconKind = Utility.ParseIconKind(icon);
            if (iconKind == IconKind.Logogram)
            {

                var fontSize = iconWidth / 2;
                var text = new SvgText
                {
                    Text = icon,
                    FontSize = fontSize,
                    TextAnchor = SvgTextAnchor.Middle,
                    Fill = new SvgColourServer(Color.Black),

                    CustomAttributes =
                    {

                    }
                };
                var textTransformed = new SvgGroup
                {
                    Children = { text }
                };
                var textTransform = new SvgTransformCollection
                {
                    new SvgTranslate(iconWidth / 2, iconStroke + iconWidth / 2 + fontSize / 4)
                };
                textTransformed.Transforms = textTransform;
                typeDef.Children.Add(new SvgUse { CustomAttributes = { { "href", "#icon" } } });
                typeDef.Children.Add(textTransformed);
            }
            else
            {
                if (iconKind == IconKind.Filepath)
                    icon = Path.Combine(kitDirectory, icon);

                var image = new SvgImage
                {
                    Width = iconWidth - 2 * iconStroke,
                    Height = iconWidth - 2 * iconStroke,
                    CustomAttributes =
                    {
                        { "href", Utility.DatastringFromUrl(icon) },
                        { "mask", "url(#iconMask)" }
                    }
                };
                var imageTransformed = new SvgGroup
                {
                    Children = { image }
                };
                var imageTransform = new SvgTransformCollection
                {
                    new SvgTranslate(iconStroke, iconStroke)
                };
                imageTransformed.Transforms = imageTransform;
                typeDef.Children.Add(new SvgUse { CustomAttributes = { { "href", "#icon" } } });
                typeDef.Children.Add(imageTransformed);
            }

            defs.Children.Add(typeDef);
        }

        svgDoc.Children.Add(defs);

        var connections = new SvgGroup { ID = "connections" };

        foreach (var connection in design.Connections)
        {
            var connectedPieceFlat = GetPiece(flatCloneInSvgCoordinates, connection.Parent.Piece.Id);
            var connectingPieceFlat = GetPiece(flatCloneInSvgCoordinates, connection.Child.Piece.Id);
            if (connectedPieceFlat?.Center is null || connectingPieceFlat?.Center is null) continue;
            var connectionLine = new SvgLine
            {
                StartX = (float)connectedPieceFlat.Center.U + iconWidth / 2,
                StartY = (float)connectedPieceFlat.Center.V + iconWidth / 2,
                EndX = (float)connectingPieceFlat.Center.U + iconWidth / 2,
                EndY = (float)connectingPieceFlat.Center.V + iconWidth / 2,
                Stroke = new SvgColourServer(Color.Black),
                StrokeWidth = connectionStroke,
                Children = { new SvgTitle { Content = connection.ToIdString() } }
            };
            connections.Children.Add(connectionLine);
        }

        svgDoc.Children.Add(connections);

        var pieces = new SvgGroup { ID = "pieces" };

        foreach (var piece in design.Pieces)
        {
            var flatPiece = GetPiece(flatCloneInSvgCoordinates, piece.Id);
            if (piece.Center is not null && flatPiece?.Center is not null)
            {
                var rootPiece = new SvgUse
                {
                    CustomAttributes = { { "href", "#root" } },
                    X = (float)flatPiece.Center.U,
                    Y = (float)flatPiece.Center.V
                };
                pieces.Children.Add(rootPiece);
            }

            var pieceType = flatPiece?.Type is not null ? types.FirstOrDefault(t => t.Id == flatPiece.Type.Id) : null;
            if (pieceType is not null && flatPiece?.Center is not null)
            {
                var pieceIcon = new SvgUse
                {
                    CustomAttributes =
                        { { "href", "#" + typesDict[pieceType.Name].ToIdString() } },
                    X = (float)flatPiece.Center.U,
                    Y = (float)flatPiece.Center.V,
                    Children = { new SvgTitle { Content = flatPiece.Id } }
                };
                pieces.Children.Add(pieceIcon);
            }
        }

        svgDoc.Children.Add(pieces);

        var svg = svgDoc.GetXML();

        var xml = new XmlDocument();
        xml.LoadXml(svg);
        var styleElement = xml.CreateElement("style");
        styleElement.InnerXml = @"
@font-face {
  font-family: ""Anta"";
  src: url(""data:application/truetype;base64," + Resources.Anta + @""");
}

@font-face {
  font-family: ""Noto Emoji"";
  src: url(""data:application/truetype;base64," + Resources.NotoEmoji + @""");
}

text {
  font-family: ""Anta"", ""Noto Emoji"";
}";
        if (xml.DocumentElement is null) throw new InvalidOperationException("XML document has no root element");
        xml.DocumentElement.PrependChild(styleElement);
        return xml.OuterXml.Replace(" xmlns=\"\"", "");
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var piece in Pieces)
        {
            var (isValidPiece, errorsPiece) = piece.Validate();
            isValid = isValid && isValidPiece;
            errors.AddRange(errorsPiece.Select(e => $"A piece({piece.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var connection in Connections)
        {
            var (isValidConnection, errorsConnection) = connection.Validate();
            isValid = isValid && isValidConnection;
            errors.AddRange(errorsConnection.Select(e =>
                $"A connection({connection.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var author in Authors)
        {
            var (isValidAuthor, errorsAuthor) = author.Validate();
            isValid = isValid && isValidAuthor;
            errors.AddRange(errorsAuthor.Select(e => $"An author({author.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }

        var pieceIds = Pieces.Select(p => p.Id);
        var duplicatePieceIds = pieceIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicatePieceIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicatePieceId in duplicatePieceIds)
                errors.Add($"A piece is invalid: There are multiple pieces with id ({duplicatePieceId}).");
        }

        var nonExistingConnectedPieces = Connections.Where(c => !pieceIds.Contains(c.Parent.Piece.Id)).ToList()
            .Select(c => c.Parent.Piece.Id).ToArray();
        if (nonExistingConnectedPieces.Length != 0)
        {
            isValid = false;
            foreach (var nonExistingConnectedPiece in nonExistingConnectedPieces)
            {
                var connection = Connections.First(c => c.Parent.Piece.Id == nonExistingConnectedPiece);
                errors.Add(
                    $"A connection({connection.ToHumanIdString()}) is invalid: The referenced connected piece ({nonExistingConnectedPiece}) is not part of the design.");
            }
        }

        var nonExistingConnectingPieces = Connections.Where(c => !pieceIds.Contains(c.Child.Piece.Id)).ToList()
            .Select(c => c.Child.Piece.Id).ToArray();
        if (nonExistingConnectingPieces.Length != 0)
        {
            isValid = false;
            foreach (var nonExistingConnectingPiece in nonExistingConnectingPieces)
            {
                var connection = Connections.First(c => c.Child.Piece.Id == nonExistingConnectingPiece);
                errors.Add(
                    $"A connection({connection.ToHumanIdString()}) is invalid: The referenced connecting piece ({nonExistingConnectingPiece}) is not part of the design.");
            }
        }

        var connectionKeys = Connections
            .Select(c => (
                ParentPieceId: c.Parent.Piece.Id,
                ParentDesignPieceId: c.Parent.DesignPiece?.Id ?? "",
                ChildPieceId: c.Child.Piece.Id,
                ChildDesignPieceId: c.Child.DesignPiece?.Id ?? ""))
            .ToList();
        var duplicateConnections = connectionKeys
            .GroupBy(k => k)
            .Where(g => g.Count() > 1)
            .Select(g => g.Key)
            .ToArray();
        if (duplicateConnections.Length != 0)
        {
            isValid = false;
            foreach (var key in duplicateConnections)
                errors.Add($"A connection is duplicated for ({key.ParentPieceId},{key.ParentDesignPieceId},{key.ChildPieceId},{key.ChildDesignPieceId}).");
        }

        return (isValid, errors);
    }

    public static bool IsSameAs(Design design, Design other)
    {
        if (other is null) return false;
        return design.Name == other.Name;
    }

    public static Piece FindPiece(Design design, string pieceId)
    {
        var piece = design.Pieces.FirstOrDefault(p => p.Id == pieceId);
        if (piece is null) throw new ArgumentException($"Piece {pieceId} not found in design");
        return piece;
    }

    public static Connection FindConnection(Design design, Connection connectionToFind, bool strict = false)
    {
        var connection = design.Connections.FirstOrDefault(c => Connection.IsSameAs(c, connectionToFind, strict));
        if (connection is null)
            throw new ArgumentException($"Connection {connectionToFind.Parent.Piece.Id} -> {connectionToFind.Child.Piece.Id} not found in design");
        return connection;
    }

    public static List<Connection> FindPieceConnections(Design design, string pieceId)
    {
        return design.Connections.Where(c =>
            c.Parent.Piece.Id == pieceId ||
            c.Child.Piece.Id == pieceId).ToList();
    }

    public static Design AddPiece(Design design, Piece piece)
    {
        var newPieces = new List<Piece>(design.Pieces) { piece };
        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = newPieces,
            Connections = new List<Connection>(design.Connections),
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static Design RemovePiece(Design design, string pieceId)
    {
        var newPieces = design.Pieces.Where(p => p.Id != pieceId).ToList();
        var newConnections = design.Connections.Where(c =>
            c.Parent.Piece.Id != pieceId &&
            c.Child.Piece.Id != pieceId).ToList();
        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = newPieces,
            Connections = newConnections,
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static Design AddConnection(Design design, Connection connection)
    {
        var newConnections = new List<Connection>(design.Connections) { connection };
        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new List<Piece>(design.Pieces),
            Connections = newConnections,
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static Design RemoveConnection(Design design, Connection connectionToRemove)
    {
        var newConnections = design.Connections.Where(c => !Connection.IsSameAs(c, connectionToRemove)).ToList();
        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new List<Piece>(design.Pieces),
            Connections = newConnections,
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static string FindAttributeValue(Design design, string key, string defaultValue = "")
    {
        var attribute = design.Attributes.FirstOrDefault(a => a.Key == key);
        return attribute?.Value ?? defaultValue;
    }

    public static Design SetAttribute(Design design, Attribute attribute)
    {
        var newAttributes = design.Attributes.Where(a => a.Key != attribute.Key).ToList();
        newAttributes.Add(attribute);
        return new Design
        {
            Id = design.Id,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new List<Piece>(design.Pieces),
            Connections = new List<Connection>(design.Connections),
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = newAttributes
        };
    }

    public static DesignDiff DragPiecesInDesign(Design design, Design pieces, Coordinate offset)
    {
        var designConnections = design.Connections;
        var selectedPieces = pieces.Pieces;
        var selectedIds = new HashSet<string>(selectedPieces.Select(p => p.Id));
        var connectionByChild = new Dictionary<string, Connection>();
        foreach (var conn in designConnections)
        {
            connectionByChild[conn.Child.Piece.Id] = conn;
        }
        var fixedIds = new HashSet<string>();
        foreach (var id in selectedIds)
        {
            if (!connectionByChild.ContainsKey(id))
                fixedIds.Add(id);
        }
        var pieceMap = design.Pieces.ToDictionary(p => p.Id);
        var pieceModifications = new List<PieceModification>();
        foreach (var id in fixedIds)
        {
            if (pieceMap.TryGetValue(id, out var piece) && piece.Center != null)
            {
                pieceModifications.Add(new PieceModification
                {
                    Piece = new PieceId { Id = id },
                    Diff = new PieceDiff { Center = new Coordinate { U = piece.Center.U + offset.U, V = piece.Center.V + offset.V } },
                });
            }
        }
        var connectionModifications = new List<ConnectionModification>();
        foreach (var id in selectedIds)
        {
            if (fixedIds.Contains(id)) continue;
            var isDescendant = false;
            var current = id;
            while (connectionByChild.TryGetValue(current, out var conn))
            {
                var parentId = conn.Parent.Piece.Id;
                if (selectedIds.Contains(parentId))
                {
                    isDescendant = true;
                    break;
                }
                current = parentId;
            }
            if (isDescendant) continue;
            if (connectionByChild.TryGetValue(id, out var parentConn))
            {
                connectionModifications.Add(new ConnectionModification
                {
                    Connection = new ConnectionId { Id = parentConn.Id },
                    Diff = new ConnectionDiff { U = offset.U, V = offset.V },
                });
            }
        }
        var diff = new DesignDiff();
        if (pieceModifications.Count > 0)
            diff.Pieces = new PiecesDiff { Modified = pieceModifications };
        if (connectionModifications.Count > 0)
            diff.Connections = new ConnectionsDiff { Modified = connectionModifications };
        return diff;
    }

    private static Point MoveTranslationWorldFromPiecePlane(Plane plane, MoveVector vector)
    {
        var xAxis = new double[] { plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z };
        var yAxis = new double[] { plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z };
        if (xAxis[0] * xAxis[0] + xAxis[1] * xAxis[1] + xAxis[2] * xAxis[2] < 1e-12) return new Point();
        NormalizeD(xAxis);
        if (yAxis[0] * yAxis[0] + yAxis[1] * yAxis[1] + yAxis[2] * yAxis[2] < 1e-12) return new Point();
        NormalizeD(yAxis);
        var zAxis = CrossD(xAxis, yAxis);
        if (zAxis[0] * zAxis[0] + zAxis[1] * zAxis[1] + zAxis[2] * zAxis[2] < 1e-12) return new Point();
        NormalizeD(zAxis);
        return new Point
        {
            X = vector.Shift * xAxis[0] + vector.Gap * yAxis[0] + vector.Rise * zAxis[0],
            Y = vector.Shift * xAxis[1] + vector.Gap * yAxis[1] + vector.Rise * zAxis[1],
            Z = vector.Shift * xAxis[2] + vector.Gap * yAxis[2] + vector.Rise * zAxis[2],
        };
    }

    private static double[] MoveTranslationWorld(Plane plane, MoveVector mv)
    {
        var xAxis = new double[] { plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z };
        var yAxis = new double[] { plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z };
        NormalizeD(xAxis);
        NormalizeD(yAxis);
        var zAxis = CrossD(xAxis, yAxis);
        if (zAxis[0] * zAxis[0] + zAxis[1] * zAxis[1] + zAxis[2] * zAxis[2] < 1e-12)
            return new double[] { 0, 0, 0 };
        NormalizeD(zAxis);
        return new double[]
        {
            mv.Shift * xAxis[0] + mv.Gap * yAxis[0] + mv.Rise * zAxis[0],
            mv.Shift * xAxis[1] + mv.Gap * yAxis[1] + mv.Rise * zAxis[1],
            mv.Shift * xAxis[2] + mv.Gap * yAxis[2] + mv.Rise * zAxis[2],
        };
    }

    private static void NormalizeD(double[] v)
    {
        var len = Math.Sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
        if (len < 1e-12) return;
        v[0] /= len; v[1] /= len; v[2] /= len;
    }

    private static double[] CrossD(double[] a, double[] b) =>
        new double[] { a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0] };

    private static double DotD(double[] a, double[] b) =>
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

    private static Plane IdentityPlaneForStructuralMove() => new Plane
    {
        Origin = new Point { X = 0, Y = 0, Z = 0 },
        XAxis = new Vector { X = 1, Y = 0, Z = 0 },
        YAxis = new Vector { X = 0, Y = 1, Z = 0 },
    };

    private static Connector GetConnectorFromType(Dictionary<string, Type> typesDict, Type typ, string connectorId)
    {
        if (typ == null) return null;
        if (string.IsNullOrEmpty(connectorId))
        {
            if (typ.Connectors.Count > 0) return typ.Connectors[0];
            if (typ.Parent != null && typesDict.TryGetValue(typ.Parent.Id, out var parentType))
                return GetConnectorFromType(typesDict, parentType, connectorId);
            return null;
        }
        foreach (var c in typ.Connectors)
            if (c.Id == connectorId) return c;
        if (typ.Parent != null && typesDict.TryGetValue(typ.Parent.Id, out var pt))
        {
            var found = GetConnectorFromType(typesDict, pt, connectorId);
            if (found != null) return found;
        }
        if (typ.Connectors.Count > 0) return typ.Connectors[0];
        return null;
    }

    private static void ConnectionPlacementTranslationBasis(Connector parentConnector, out double[] gapDir, out double[] shiftDir, out double[] raiseDir)
    {
        var parentDirection = new double[] { parentConnector.Direction?.X ?? 0, parentConnector.Direction?.Y ?? 1, parentConnector.Direction?.Z ?? 0 };
        NormalizeD(parentDirection);
        var parentConnectorQuat = CreateFromTwoVectors(UnitY, parentDirection);
        var parentRotationT = QuaternionToMatrix(parentConnectorQuat);
        gapDir = Vec3Normalize(ApplyMatrix4ToVec3(parentRotationT, UnitY));
        shiftDir = Vec3Normalize(ApplyMatrix4ToVec3(parentRotationT, UnitX));
        raiseDir = Vec3Normalize(ApplyMatrix4ToVec3(parentRotationT, UnitZ));
    }

    private static double[] ChildConnectorOriginWorld(Plane parentPlane, Connector parentConnector, Connector childConnector, Connection connection)
    {
        var childPlane = DefaultComputeChildPlane(
            parentPlane,
            parentConnector.Point ?? new Point(),
            parentConnector.Direction ?? new Vector { X = 0, Y = 1, Z = 0 },
            childConnector.Point ?? new Point(),
            childConnector.Direction ?? new Vector { X = 0, Y = 1, Z = 0 },
            connection.Gap, connection.Shift, connection.Rise,
            connection.Rotation, connection.Turn, connection.Tilt);
        return new double[] { childPlane.Origin.X, childPlane.Origin.Y, childPlane.Origin.Z };
    }

    private static Connection ConnectionWithNumericDelta(Connection connection, string key, double delta)
    {
        var c = new Connection
        {
            Id = connection.Id,
            Parent = connection.Parent,
            Child = connection.Child,
            Description = connection.Description,
            Gap = connection.Gap,
            Shift = connection.Shift,
            Rise = connection.Rise,
            Rotation = connection.Rotation,
            Turn = connection.Turn,
            Tilt = connection.Tilt,
            U = connection.U,
            V = connection.V,
        };
        switch (key)
        {
            case "gap": c.Gap += delta; break;
            case "shift": c.Shift += delta; break;
            case "rise": c.Rise += delta; break;
            case "rotation": c.Rotation += delta; break;
            case "turn": c.Turn += delta; break;
            case "tilt": c.Tilt += delta; break;
        }
        return c;
    }

    private static double[] SolveConnectionOriginMinNorm(double[][] cols, double[] t)
    {
        if (cols.Length == 0) return null;
        var jjt = new double[9];
        for (int c = 0; c < 3; c++)
            for (int r = 0; r < 3; r++)
            {
                double s = 0;
                foreach (var col in cols) s += col[r] * col[c];
                jjt[r + c * 3] = s;
            }
        jjt[0] += 1e-14; jjt[4] += 1e-14; jjt[8] += 1e-14;
        var det = jjt[0] * (jjt[4] * jjt[8] - jjt[7] * jjt[5])
                - jjt[3] * (jjt[1] * jjt[8] - jjt[7] * jjt[2])
                + jjt[6] * (jjt[1] * jjt[5] - jjt[4] * jjt[2]);
        if (Math.Abs(det) < 1e-22) return null;
        var invDet = 1.0 / det;
        var inv = new double[9];
        inv[0] = (jjt[4] * jjt[8] - jjt[5] * jjt[7]) * invDet;
        inv[1] = (jjt[2] * jjt[7] - jjt[1] * jjt[8]) * invDet;
        inv[2] = (jjt[1] * jjt[5] - jjt[2] * jjt[4]) * invDet;
        inv[3] = (jjt[5] * jjt[6] - jjt[3] * jjt[8]) * invDet;
        inv[4] = (jjt[0] * jjt[8] - jjt[2] * jjt[6]) * invDet;
        inv[5] = (jjt[2] * jjt[3] - jjt[0] * jjt[5]) * invDet;
        inv[6] = (jjt[3] * jjt[7] - jjt[4] * jjt[6]) * invDet;
        inv[7] = (jjt[1] * jjt[6] - jjt[0] * jjt[7]) * invDet;
        inv[8] = (jjt[0] * jjt[4] - jjt[1] * jjt[3]) * invDet;
        if (double.IsInfinity(inv[0]) || double.IsNaN(inv[0])) return null;
        var u = new double[]
        {
            inv[0] * t[0] + inv[3] * t[1] + inv[6] * t[2],
            inv[1] * t[0] + inv[4] * t[1] + inv[7] * t[2],
            inv[2] * t[0] + inv[5] * t[1] + inv[8] * t[2],
        };
        var deltas = new double[cols.Length];
        for (int i = 0; i < cols.Length; i++)
            deltas[i] = cols[i][0] * u[0] + cols[i][1] * u[1] + cols[i][2] * u[2];
        return deltas;
    }

    private static ConnectionDiff ConnectionDiffTranslationFallback(Plane parentPlane, Connector parentConnector, double[] tw)
    {
        ConnectionPlacementTranslationBasis(parentConnector, out var gapDir, out var shiftDir, out var raiseDir);
        var dgap = DotD(tw, gapDir);
        var dshift = DotD(tw, shiftDir);
        var drise = DotD(tw, raiseDir);
        var res = new double[]
        {
            tw[0] - dgap * gapDir[0] - dshift * shiftDir[0] - drise * raiseDir[0],
            tw[1] - dgap * gapDir[1] - dshift * shiftDir[1] - drise * raiseDir[1],
            tw[2] - dgap * gapDir[2] - dshift * shiftDir[2] - drise * raiseDir[2],
        };
        var px = new double[] { parentPlane.XAxis.X, parentPlane.XAxis.Y, parentPlane.XAxis.Z };
        var py = new double[] { parentPlane.YAxis.X, parentPlane.YAxis.Y, parentPlane.YAxis.Z };
        var diff = new ConnectionDiff();
        const double eps = 1e-9;
        if (Math.Abs(dgap) > eps) diff.Gap = dgap;
        if (Math.Abs(dshift) > eps) diff.Shift = dshift;
        if (Math.Abs(drise) > eps) diff.Rise = drise;
        var pxSq = px[0] * px[0] + px[1] * px[1] + px[2] * px[2];
        var pySq = py[0] * py[0] + py[1] * py[1] + py[2] * py[2];
        if (pxSq > 1e-24 && pySq > 1e-24)
        {
            var pxN = new double[] { px[0] / Math.Sqrt(pxSq), px[1] / Math.Sqrt(pxSq), px[2] / Math.Sqrt(pxSq) };
            var pyN = new double[] { py[0] / Math.Sqrt(pySq), py[1] / Math.Sqrt(pySq), py[2] / Math.Sqrt(pySq) };
            var du = DotD(res, pxN);
            var dv = DotD(res, pyN);
            if (Math.Abs(du) > eps) diff.U = du;
            if (Math.Abs(dv) > eps) diff.V = dv;
        }
        return diff;
    }

    private static ConnectionDiff ConnectionDiffFromStructuralMoveVector(
        Plane parentPlane, Connector parentConnector, Connector childConnector,
        Connection connection, Plane childPlane, MoveVector vector)
    {
        var child = childPlane ?? IdentityPlaneForStructuralMove();
        var tw = MoveTranslationWorld(child, vector);
        var tSq = tw[0] * tw[0] + tw[1] * tw[1] + tw[2] * tw[2];
        if (tSq < 1e-24) return new ConnectionDiff();
        if (childConnector == null)
            return ConnectionDiffTranslationFallback(parentPlane, parentConnector, tw);

        var jacobianKeys = new[] { "gap", "shift", "rise", "rotation", "turn", "tilt" };
        var jacobianEps = new Dictionary<string, double>
        {
            { "gap", 1e-6 }, { "shift", 1e-6 }, { "rise", 1e-6 },
            { "rotation", 1e-4 }, { "turn", 1e-4 }, { "tilt", 1e-4 },
        };
        var o0 = ChildConnectorOriginWorld(parentPlane, parentConnector, childConnector, connection);
        var cols = new double[jacobianKeys.Length][];
        for (int i = 0; i < jacobianKeys.Length; i++)
        {
            var epsVal = jacobianEps[jacobianKeys[i]];
            var perturbed = ConnectionWithNumericDelta(connection, jacobianKeys[i], epsVal);
            var o1 = ChildConnectorOriginWorld(parentPlane, parentConnector, childConnector, perturbed);
            cols[i] = new double[] { (o1[0] - o0[0]) / epsVal, (o1[1] - o0[1]) / epsVal, (o1[2] - o0[2]) / epsVal };
        }
        var deltas = SolveConnectionOriginMinNorm(cols, tw);
        var diff = new ConnectionDiff();
        const double epsOut = 1e-9;
        if (deltas != null)
        {
            for (int i = 0; i < jacobianKeys.Length; i++)
            {
                if (Math.Abs(deltas[i]) > epsOut)
                {
                    var v = deltas[i];
                    switch (jacobianKeys[i])
                    {
                        case "gap": diff.Gap = v; break;
                        case "shift": diff.Shift = v; break;
                        case "rise": diff.Rise = v; break;
                        case "rotation": diff.Rotation = v; break;
                        case "turn": diff.Turn = v; break;
                        case "tilt": diff.Tilt = v; break;
                    }
                }
            }
            var pred = new double[] { 0, 0, 0 };
            for (int i = 0; i < cols.Length; i++)
            {
                pred[0] += cols[i][0] * deltas[i];
                pred[1] += cols[i][1] * deltas[i];
                pred[2] += cols[i][2] * deltas[i];
            }
            var res = new double[] { tw[0] - pred[0], tw[1] - pred[1], tw[2] - pred[2] };
            var px = new double[] { parentPlane.XAxis.X, parentPlane.XAxis.Y, parentPlane.XAxis.Z };
            var py = new double[] { parentPlane.YAxis.X, parentPlane.YAxis.Y, parentPlane.YAxis.Z };
            var pxSq = px[0] * px[0] + px[1] * px[1] + px[2] * px[2];
            var pySq = py[0] * py[0] + py[1] * py[1] + py[2] * py[2];
            if (pxSq > 1e-24 && pySq > 1e-24)
            {
                var pxN = new double[] { px[0] / Math.Sqrt(pxSq), px[1] / Math.Sqrt(pxSq), px[2] / Math.Sqrt(pxSq) };
                var pyN = new double[] { py[0] / Math.Sqrt(pySq), py[1] / Math.Sqrt(pySq), py[2] / Math.Sqrt(pySq) };
                var du = DotD(res, pxN);
                var dv = DotD(res, pyN);
                if (Math.Abs(du) > epsOut) diff.U = du;
                if (Math.Abs(dv) > epsOut) diff.V = dv;
            }
            return diff;
        }
        return ConnectionDiffTranslationFallback(parentPlane, parentConnector, tw);
    }

    public static DesignDiff MovePiecesInDesign(Kit kit, Design design, Design pieces, MoveVector vector)
    {
        var typesDict = new Dictionary<string, Type>();
        foreach (var t in kit.Types) typesDict[t.Id] = t;

        var selectedIds = new HashSet<string>(pieces.Pieces.Select(p => p.Id));
        var connectionByChild = new Dictionary<string, Connection>();
        foreach (var conn in design.Connections)
            connectionByChild[conn.Child.Piece.Id] = conn;

        var fixedIds = new HashSet<string>();
        foreach (var id in selectedIds)
            if (!connectionByChild.ContainsKey(id))
                fixedIds.Add(id);

        var pieceMap = design.Pieces.ToDictionary(p => p.Id);
        var pieceModifications = new List<PieceModification>();
        foreach (var id in fixedIds)
        {
            if (!pieceMap.TryGetValue(id, out var piece) || piece.Plane == null) continue;
            var basePlane = piece.Plane;
            var t = MoveTranslationWorldFromPiecePlane(basePlane, vector);
            pieceModifications.Add(new PieceModification
            {
                Piece = new PieceId { Id = id },
                Diff = new PieceDiff
                {
                    Plane = new Plane
                    {
                        Origin = new Point
                        {
                            X = basePlane.Origin.X + t.X,
                            Y = basePlane.Origin.Y + t.Y,
                            Z = basePlane.Origin.Z + t.Z,
                        },
                        XAxis = new Vector { X = basePlane.XAxis.X, Y = basePlane.XAxis.Y, Z = basePlane.XAxis.Z },
                        YAxis = new Vector { X = basePlane.YAxis.X, Y = basePlane.YAxis.Y, Z = basePlane.YAxis.Z },
                    },
                },
            });
        }
        var connectionModifications = new List<ConnectionModification>();
        foreach (var id in selectedIds)
        {
            if (fixedIds.Contains(id)) continue;
            var isDescendant = false;
            var current = id;
            while (connectionByChild.TryGetValue(current, out var conn))
            {
                var parentId = conn.Parent.Piece.Id;
                if (selectedIds.Contains(parentId)) { isDescendant = true; break; }
                current = parentId;
            }
            if (isDescendant) continue;
            if (!connectionByChild.TryGetValue(id, out var parentConn)) continue;
            pieceMap.TryGetValue(parentConn.Parent.Piece.Id, out var parentPiece);
            pieceMap.TryGetValue(id, out var childPiece);
            if (parentPiece == null || childPiece == null) continue;
            if (parentPiece.Type == null || childPiece.Type == null) continue;
            typesDict.TryGetValue(parentPiece.Type.Id, out var parentType);
            typesDict.TryGetValue(childPiece.Type.Id, out var childType);
            var parentConnector = GetConnectorFromType(typesDict, parentType,
                parentConn.Parent.Connector?.Id ?? "");
            var childConnector = GetConnectorFromType(typesDict, childType,
                parentConn.Child.Connector?.Id ?? "");
            if (parentConnector == null) continue;
            var parentPlane = parentPiece.Plane ?? IdentityPlaneForStructuralMove();
            var connDiff = ConnectionDiffFromStructuralMoveVector(
                parentPlane, parentConnector, childConnector,
                parentConn, childPiece.Plane, vector);
            var hasFields = connDiff.Gap.HasValue || connDiff.Shift.HasValue || connDiff.Rise.HasValue ||
                connDiff.Rotation.HasValue || connDiff.Turn.HasValue || connDiff.Tilt.HasValue ||
                connDiff.U.HasValue || connDiff.V.HasValue;
            if (!hasFields) continue;
            connectionModifications.Add(new ConnectionModification
            {
                Connection = new ConnectionId { Id = parentConn.Id },
                Diff = connDiff,
            });
        }
        var diff = new DesignDiff();
        if (pieceModifications.Count > 0)
            diff.Pieces = new PiecesDiff { Modified = pieceModifications };
        if (connectionModifications.Count > 0)
            diff.Connections = new ConnectionsDiff { Modified = connectionModifications };
        return diff;
    }

    /// <summary>
    /// Deletes pieces and connections from a design, returning a canonical ComposeReport of DesignDiff.
    /// Removes stale connections referencing deleted pieces.
    /// Modifications pieces that become fixed (parent connection removed) with flat plane and center from the flattened design.
    /// </summary>
    public static ComposeReport<DesignDiff> DeletePiecesAndConnectionsInDesign(Kit kit, Design design, List<string> pieceIds, List<string> connectionIds)
    {
        var deletedPieceSet = new HashSet<string>(pieceIds);

        // Find stale connections: connections referencing any deleted piece
        var staleConnectionIds = new HashSet<string>();
        foreach (var conn in design.Connections)
        {
            if (deletedPieceSet.Contains(conn.Parent.Piece.Id) ||
                deletedPieceSet.Contains(conn.Child.Piece.Id))
            {
                staleConnectionIds.Add(conn.Id);
            }
        }

        // All removed connections = explicit + stale
        var allRemovedConnectionIds = new HashSet<string>(connectionIds);
        allRemovedConnectionIds.UnionWith(staleConnectionIds);

        // Find pieces that become fixed: pieces whose parent connection was removed
        // and are not themselves being deleted
        // A piece becomes fixed when the connection where it is the "connecting" side is removed
        // and it has no other remaining parent connection
        var fixedPieceIds = new List<string>();
        foreach (var connId in allRemovedConnectionIds)
        {
            var conn = design.Connections.FirstOrDefault(c => c.Id == connId);
            if (conn == null) continue;
            var connectingId = conn.Child.Piece.Id;
            if (deletedPieceSet.Contains(connectingId)) continue;
            // Check if this piece has another parent connection not in the removed set
            var hasOtherParent = design.Connections.Any(c =>
                c.Child.Piece.Id == connectingId &&
                !allRemovedConnectionIds.Contains(c.Id));
            if (!hasOtherParent && !fixedPieceIds.Contains(connectingId))
                fixedPieceIds.Add(connectingId);
        }

        // Build the diff
        var piecesRemoved = pieceIds.Select(g => new PieceId { Id = g }).ToList();

        // Flatten the design to get absolute plane and center for each piece
        var flatRep = Kit.FlattenDesign(kit, design.Id);
        if (!flatRep.Ok)
            return ComposeReport<DesignDiff>.Failure(flatRep.Errors);
        var flatResult = flatRep.Diff!.Forward;
        var flatPieceMap = new Dictionary<string, (Plane? Plane, Coordinate? Center)>();
        foreach (var piece in design.Pieces)
        {
            if (piece.Plane != null)
                flatPieceMap[piece.Id] = (piece.Plane, piece.Center);
        }
        if (flatResult.Pieces?.Modified != null)
        {
            foreach (var update in flatResult.Pieces.Modified)
            {
                var existing = flatPieceMap.ContainsKey(update.Piece.Id)
                    ? flatPieceMap[update.Piece.Id]
                    : ((Plane?)null, (Coordinate?)null);
                if (update.Diff?.Plane != null) existing.Item1 = update.Diff.Plane;
                if (update.Diff?.Center != null) existing.Item2 = update.Diff.Center;
                flatPieceMap[update.Piece.Id] = existing;
            }
        }

        var piecesModificationd = fixedPieceIds.Select(g =>
        {
            var flat = flatPieceMap.ContainsKey(g) ? flatPieceMap[g] : ((Plane?)null, (Coordinate?)null);
            return new PieceModification
            {
                Piece = new PieceId { Id = g },
                Diff = new PieceDiff
                {
                    Plane = flat.Item1 ?? new Plane(),
                    Center = flat.Item2 ?? new Coordinate()
                }
            };
        }).ToList();
        var connectionsRemoved = allRemovedConnectionIds
            .OrderBy(g => g)
            .Select(g => new ConnectionId { Id = g })
            .ToList();

        var diff = new DesignDiff();
        if (piecesRemoved.Count > 0 || piecesModificationd.Count > 0)
            diff.Pieces = new PiecesDiff { Removed = piecesRemoved, Modified = piecesModificationd };
        if (connectionsRemoved.Count > 0)
            diff.Connections = new ConnectionsDiff { Removed = connectionsRemoved };
        return ComposeReport<DesignDiff>.Success(diff, flatRep.Warnings, flatRep.Infos);
    }

    /// <summary>
    /// 📋Copies a selection of pieces and connections from a design into a standalone design.
    /// </summary>
    /// <remarks>
    /// Specs: A selection is a set of piece ids and connection ids.
    /// - Fixed selected pieces are added as-is.
    /// - Internal-connected pieces (selected, parent piece selected, parent connection selected) are added as-is.
    /// - Parent-piece-exclusive parent-connection-inclusive pieces get compose.center and compose.plane attributes from the flat design.
    /// - Orphaned connections, parent-exclusive child-inclusive connections, and parent-inclusive child-exclusive connections
    ///   are added with their external pieces marked with compose.piece.origin = "external".
    /// - Internal connections are added as-is.
    /// </remarks>
    public static Design CopyDesign(Kit kit, Design design, List<string> pieceIds, List<string> connectionIds)
    {
        var selectedPieceSet = new HashSet<string>(pieceIds);
        var selectedConnectionSet = new HashSet<string>(connectionIds);

        // Build parent map: child id -> (parent id, connection)
        var parentMap = new Dictionary<string, (string parentId, Connection connection)>();
        foreach (var conn in design.Connections)
        {
            parentMap[conn.Child.Piece.Id] = (conn.Parent.Piece.Id, conn);
        }

        // Flatten the design to get absolute planes/centers
        var flatDiff = Kit.FlattenDesignDiff(kit, design.Id);
        var flatDesign = Design.ApplyDiff(Entity<Design>.DeepClone(design)!, flatDiff);
        var flatPieceMap = flatDesign.Pieces.ToDictionary(p => p.Id);

        var copyPieces = new List<Piece>();
        var addedPieceIds = new HashSet<string>();
        var copyConnections = new List<Connection>();

        // Process selected pieces
        foreach (var pieceId in pieceIds)
        {
            var piece = design.Pieces.First(p => p.Id == pieceId);
            var isFixed = piece.Plane is not null;
            var isConnected = parentMap.ContainsKey(pieceId);

            bool isInternalConnected = false;
            bool isInternalFixed = isFixed && selectedPieceSet.Contains(pieceId);
            bool isPpExclPcIncl = false;

            if (isConnected)
            {
                var (parentId, parentConn) = parentMap[pieceId];
                var parentPieceSelected = selectedPieceSet.Contains(parentId);
                var parentConnSelected = selectedConnectionSet.Contains(parentConn.Id);
                isInternalConnected = parentPieceSelected && parentConnSelected;
                isPpExclPcIncl = !parentPieceSelected && parentConnSelected;
            }

            var isInternal = isInternalConnected || isInternalFixed;

            if (isInternalFixed || isInternalConnected)
            {
                copyPieces.Add(Entity<Piece>.DeepClone(piece)!);
                addedPieceIds.Add(pieceId);
            }
            else if (isPpExclPcIncl)
            {
                var copied = Entity<Piece>.DeepClone(piece)!;
                // Add compose.center and compose.plane from flattened design
                if (flatPieceMap.TryGetValue(pieceId, out var flatPiece))
                {
                    var centerValue = flatPiece.Center is not null
                        ? Utility.Serialize(flatPiece.Center)
                        : Utility.Serialize(new Coordinate());
                    var planeValue = flatPiece.Plane is not null
                        ? Utility.Serialize(flatPiece.Plane)
                        : Utility.Serialize(new Plane());
                    copied.Attributes = new List<Attribute>(copied.Attributes)
                    {
                        new() { Key = "compose.center", Value = centerValue },
                        new() { Key = "compose.plane", Value = planeValue }
                    };
                }
                copyPieces.Add(copied);
                addedPieceIds.Add(pieceId);
            }
        }

        // Process selected connections
        foreach (var connId in connectionIds)
        {
            var conn = design.Connections.First(c => c.Id == connId);
            var connectedId = conn.Parent.Piece.Id;
            var connectingId = conn.Child.Piece.Id;
            var connectedSelected = selectedPieceSet.Contains(connectedId);
            var connectingSelected = selectedPieceSet.Contains(connectingId);

            var isInternal = connectedSelected && connectingSelected;
            var isOrphaned = !connectedSelected && !connectingSelected;
            var isParentExclChildIncl = !connectedSelected && connectingSelected;
            var isParentInclChildExcl = connectedSelected && !connectingSelected;

            if (isInternal)
            {
                copyConnections.Add(Entity<Connection>.DeepClone(conn)!);
            }
            else if (isOrphaned || isParentExclChildIncl || isParentInclChildExcl)
            {
                copyConnections.Add(Entity<Connection>.DeepClone(conn)!);

                // Add external pieces
                var externalIds = new List<string>();
                if (!connectedSelected) externalIds.Add(connectedId);
                if (!connectingSelected) externalIds.Add(connectingId);

                foreach (var extId in externalIds)
                {
                    if (!addedPieceIds.Contains(extId))
                    {
                        var extPiece = Entity<Piece>.DeepClone(design.Pieces.First(p => p.Id == extId))!;
                        var extAttrs = new List<Attribute>(extPiece.Attributes)
                        {
                            new() { Key = "compose.piece.origin", Value = "external" }
                        };
                        if (flatPieceMap.TryGetValue(extId, out var flatExtPiece))
                        {
                            var extCenterValue = flatExtPiece.Center is not null
                                ? Utility.Serialize(flatExtPiece.Center)
                                : Utility.Serialize(new Coordinate());
                            extAttrs.Add(new Attribute { Key = "compose.center", Value = extCenterValue });
                        }
                        extPiece.Attributes = extAttrs;
                        copyPieces.Add(extPiece);
                        addedPieceIds.Add(extId);
                    }
                }
            }
        }

        return new Design
        {
            Id = "",
            Name = "",
            Pieces = copyPieces,
            Connections = copyConnections
        };
    }

    /// <summary>
    /// 📋Pastes a copied design into a target design, returning a DesignDiff.
    /// </summary>
    /// <remarks>
    /// Specs: Anchoring determines the reference point within the bounding rectangle of the source.
    /// - Fixed pieces get -anchor offset applied to center; if coordinate is given, +coordinate offset is also applied.
    /// - Connected pieces with non-external parents are added as-is.
    /// - Connected pieces with external-origin parents: if a matching piece with a matching connector is found in target,
    ///   the parent connection is remapped; otherwise treated as fixed using compose.center/compose.plane attributes.
    /// - Internal connections (neither piece is external) are added as-is.
    /// - Orphaned connections and external-origin pieces are not added.
    /// </remarks>
    public static DesignDiff PasteDesign(
        Kit kit,
        Design source,
        Design target,
        string anchoring = "bottomLeft",
        Coordinate? coordinate = null)
    {
        var types = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Id);
        var ports = (kit.Ports ?? new List<Port>()).ToDictionary(p => p.Id);

        // Classify source pieces
        var externalOriginIds = new HashSet<string>();
        foreach (var piece in source.Pieces)
        {
            if (piece.Attributes.Any(a => a.Key == "compose.piece.origin" && a.Value == "external"))
                externalOriginIds.Add(piece.Id);
        }

        var sourcePieceMap = source.Pieces.ToDictionary(p => p.Id);
        var sourceParentMap = new Dictionary<string, (string parentId, Connection connection)>();
        foreach (var conn in source.Connections)
        {
            var childId = conn.Child.Piece.Id;
            var parentId = conn.Parent.Piece.Id;
            if (!sourceParentMap.TryGetValue(childId, out var prev))
            {
                sourceParentMap[childId] = (parentId, conn);
                continue;
            }
            var prevStub = externalOriginIds.Contains(prev.parentId);
            var nextStub = externalOriginIds.Contains(parentId);
            if (prevStub != nextStub && nextStub)
                sourceParentMap[childId] = (parentId, conn);
        }

        // Compute flat planes/centers for source pieces that need it
        // For pp_excl_pc_incl pieces, the compose.center and compose.plane attributes have the flat values

        // Compute bounding rectangle from flat centers
        var centerCoordinates = new List<Coordinate>();
        foreach (var piece in source.Pieces)
        {
            if (externalOriginIds.Contains(piece.Id)) continue;

            Coordinate? center = piece.Center;
            if (center is null)
            {
                // Try to get from compose.center attribute
                var centerAttr = piece.Attributes.FirstOrDefault(a => a.Key == "compose.center");
                if (centerAttr?.Value is not null)
                    center = Utility.Deserialize<Coordinate>(centerAttr.Value);
            }
            if (center is not null)
                centerCoordinates.Add(center);
        }

        // Also add centers for external pieces referenced by connections
        foreach (var conn in source.Connections)
        {
            var connectedId = conn.Parent.Piece.Id;
            var connectingId = conn.Child.Piece.Id;
            if (externalOriginIds.Contains(connectedId) && sourcePieceMap.TryGetValue(connectedId, out var extPiece1))
            {
                Coordinate? c = extPiece1.Center;
                if (c is null)
                {
                    var attr = extPiece1.Attributes.FirstOrDefault(a => a.Key == "compose.center");
                    if (attr?.Value is not null) c = Utility.Deserialize<Coordinate>(attr.Value);
                }
                if (c is not null) centerCoordinates.Add(c);
            }
            if (externalOriginIds.Contains(connectingId) && sourcePieceMap.TryGetValue(connectingId, out var extPiece2))
            {
                Coordinate? c = extPiece2.Center;
                if (c is null)
                {
                    var attr = extPiece2.Attributes.FirstOrDefault(a => a.Key == "compose.center");
                    if (attr?.Value is not null) c = Utility.Deserialize<Coordinate>(attr.Value);
                }
                if (c is not null) centerCoordinates.Add(c);
            }
        }

        if (centerCoordinates.Count == 0)
            centerCoordinates.Add(new Coordinate());

        var minU = centerCoordinates.Min(c => c.U);
        var maxU = centerCoordinates.Max(c => c.U);
        var minV = centerCoordinates.Min(c => c.V);
        var maxV = centerCoordinates.Max(c => c.V);

        Coordinate anchor;
        switch (anchoring)
        {
            case "middle":
                anchor = new Coordinate { U = (minU + maxU) / 2, V = (minV + maxV) / 2 };
                break;
            case "centroid":
                anchor = new Coordinate { U = centerCoordinates.Average(c => c.U), V = centerCoordinates.Average(c => c.V) };
                break;
            case "bottomLeft":
                anchor = new Coordinate { U = minU, V = minV };
                break;
            case "bottomRight":
                anchor = new Coordinate { U = maxU, V = minV };
                break;
            case "topLeft":
                anchor = new Coordinate { U = minU, V = maxV };
                break;
            case "topRight":
                anchor = new Coordinate { U = maxU, V = maxV };
                break;
            default: // "original"
                anchor = new Coordinate { U = 0, V = 0 };
                break;
        }

        // Build target piece maps for matching
        var targetPiecesByName = new Dictionary<string, List<Piece>>();
        foreach (var tp in target.Pieces)
        {
            if (!targetPiecesByName.ContainsKey(tp.Name))
                targetPiecesByName[tp.Name] = new List<Piece>();
            targetPiecesByName[tp.Name].Add(tp);
        }

        // Helper: check port compatibility
        bool ArePortsCompatible(string? portId1, string? portId2)
        {
            if (portId1 is null || portId2 is null) return false;
            if (portId1 == portId2) return true;
            if (!ports.TryGetValue(portId1, out var port1) || !ports.TryGetValue(portId2, out var port2))
                return false;
            return port1.CompatiblePorts.Any(cp => cp.Id == portId2) ||
                   port2.CompatiblePorts.Any(cp => cp.Id == portId1);
        }

        // Helper: check connector compatibility
        bool AreConnectorsCompatible(Connector c1, Connector c2)
        {
            return ArePortsCompatible(c1.Port?.Id, c2.Port?.Id);
        }

        // Helper: find matching connector on a type
        Connector? FindMatchingConnector(string typeId, Connector sourceConnector)
        {
            if (!types.TryGetValue(typeId, out var type)) return null;
            return type.Connectors.FirstOrDefault(c =>
                c.Name == sourceConnector.Name && AreConnectorsCompatible(c, sourceConnector));
        }

        var addedPieces = new List<Piece>();
        var addedConnections = new List<Connection>();
        var remappedPieces = new Dictionary<string, string>(); // source external id -> target piece id

        // Process source pieces
        foreach (var piece in source.Pieces)
        {
            if (externalOriginIds.Contains(piece.Id)) continue;

            var isFixed = piece.Plane is not null;
            var isConnected = sourceParentMap.ContainsKey(piece.Id);

            if (isFixed && !isConnected)
            {
                // Fixed piece: apply -anchor offset, then +coordinate if given
                var copied = Entity<Piece>.DeepClone(piece)!;
                var center = copied.Center ?? new Coordinate();
                center = new Coordinate { U = center.U - anchor.U, V = center.V - anchor.V };
                if (coordinate is not null)
                    center = new Coordinate { U = center.U + coordinate.U, V = center.V + coordinate.V };
                copied.Center = center;
                addedPieces.Add(copied);
            }
            else if (isConnected)
            {
                var (parentId, parentConn) = sourceParentMap[piece.Id];
                if (externalOriginIds.Contains(parentId))
                {
                    // Parent is external-origin: try to match in target
                    var externalParent = sourcePieceMap[parentId];
                    var matched = false;

                    if (targetPiecesByName.TryGetValue(externalParent.Name, out var candidates))
                    {
                        var isParentConnected = parentConn.Parent.Piece.Id == parentId;
                        var parentConnectorId = isParentConnected
                            ? parentConn.Parent.Connector.Id
                            : parentConn.Child.Connector.Id;

                        // Get the external parent's type to find the connector
                        Connector? sourceParentConnector = null;
                        if (externalParent.Type is not null && types.TryGetValue(externalParent.Type.Id, out var parentType))
                        {
                            sourceParentConnector = parentType.Connectors.FirstOrDefault(c => c.Id == parentConnectorId);
                        }

                        if (sourceParentConnector is not null)
                        {
                            foreach (var candidate in candidates)
                            {
                                if (candidate.Type is null) continue;
                                var matchingConnector = FindMatchingConnector(candidate.Type.Id, sourceParentConnector);
                                if (matchingConnector is not null)
                                {
                                    // Found a match! Remap the connection
                                    matched = true;
                                    remappedPieces[parentId] = candidate.Id;

                                    var copied = Entity<Piece>.DeepClone(piece)!;
                                    addedPieces.Add(copied);

                                    // Add the remapped connection
                                    var copiedConn = Entity<Connection>.DeepClone(parentConn)!;
                                    if (isParentConnected)
                                    {
                                        copiedConn.Parent = new Side
                                        {
                                            Piece = new PieceId { Id = candidate.Id },
                                            Connector = new ConnectorId { Id = matchingConnector.Id }
                                        };
                                    }
                                    else
                                    {
                                        copiedConn.Child = new Side
                                        {
                                            Piece = new PieceId { Id = candidate.Id },
                                            Connector = new ConnectorId { Id = matchingConnector.Id }
                                        };
                                    }

                                    if (coordinate is not null)
                                    {
                                        var connectedStub = externalOriginIds.Contains(parentConn.Parent.Piece.Id);
                                        var connectingStub = externalOriginIds.Contains(parentConn.Child.Piece.Id);
                                        var connMatchesParentage =
                                            (parentConn.Child.Piece.Id == piece.Id && parentConn.Parent.Piece.Id == parentId) ||
                                            (parentConn.Parent.Piece.Id == piece.Id && parentConn.Child.Piece.Id == parentId);
                                        // Specs: Coordinate may shift diagram u/v only for the remapped bridge to a clipboard external stub;
                                        // internal–internal source edges (neither side a stub) must keep cloned u/v.
                                        if (connMatchesParentage && connectedStub != connectingStub)
                                        {
                                            Coordinate? flatParentCenter = null;
                                            if (flatPiecesById_PassThrough(candidate, out var candCenter))
                                                flatParentCenter = candCenter;
                                            else if (flatPiecesById_PassThrough(externalParent, out var epCenter))
                                                flatParentCenter = epCenter;
                                            Coordinate? flatChildCenter = null;
                                            var childCenterAttr = piece.Attributes.FirstOrDefault(a => a.Key == "compose.center");
                                            if (childCenterAttr?.Value is not null)
                                                flatChildCenter = Utility.Deserialize<Coordinate>(childCenterAttr.Value);
                                            if (flatChildCenter is null && piece.Center is not null)
                                                flatChildCenter = piece.Center;

                                            if (flatParentCenter is not null && flatChildCenter is not null)
                                            {
                                                var offsetU = flatParentCenter.U - (coordinate.U + (anchor.U - flatChildCenter.U));
                                                var offsetV = flatParentCenter.V - (coordinate.V + (anchor.V - flatChildCenter.V));
                                                copiedConn.U = offsetU;
                                                copiedConn.V = offsetV;
                                            }
                                        }
                                    }

                                    addedConnections.Add(copiedConn);
                                    break;
                                }
                            }
                        }
                    }

                    if (!matched)
                    {
                        // Treat as fixed piece using compose.center and compose.plane attributes
                        var copied = Entity<Piece>.DeepClone(piece)!;
                        var centerAttr = piece.Attributes.FirstOrDefault(a => a.Key == "compose.center");
                        var planeAttr = piece.Attributes.FirstOrDefault(a => a.Key == "compose.plane");
                        if (centerAttr?.Value is not null)
                            copied.Center = Utility.Deserialize<Coordinate>(centerAttr.Value);
                        if (planeAttr?.Value is not null)
                            copied.Plane = Utility.Deserialize<Plane>(planeAttr.Value);
                        // Apply anchor offset
                        var center = copied.Center ?? new Coordinate();
                        center = new Coordinate { U = center.U - anchor.U, V = center.V - anchor.V };
                        if (coordinate is not null)
                            center = new Coordinate { U = center.U + coordinate.U, V = center.V + coordinate.V };
                        copied.Center = center;
                        addedPieces.Add(copied);
                    }
                }
                else
                {
                    // Parent is not external: add connected piece as-is
                    addedPieces.Add(Entity<Piece>.DeepClone(piece)!);
                }
            }
        }

        // Process source connections (non-external internal connections)
        foreach (var conn in source.Connections)
        {
            var connectedId = conn.Parent.Piece.Id;
            var connectingId = conn.Child.Piece.Id;

            // Skip if either piece is external-origin (these are handled during piece processing)
            if (externalOriginIds.Contains(connectedId) || externalOriginIds.Contains(connectingId))
                continue;

            // Skip orphaned connections (both pieces not in source non-external set)
            var connectedIsAdded = addedPieces.Any(p => p.Id == connectedId);
            var connectingIsAdded = addedPieces.Any(p => p.Id == connectingId);
            if (!connectedIsAdded || !connectingIsAdded) continue;

            addedConnections.Add(Entity<Connection>.DeepClone(conn)!);
        }

        // Build DesignDiff
        var diff = new DesignDiff();
        if (addedPieces.Count > 0 || addedConnections.Count > 0)
        {
            if (addedPieces.Count > 0)
                diff.Pieces = new PiecesDiff { Added = addedPieces };
            if (addedConnections.Count > 0)
                diff.Connections = new ConnectionsDiff { Added = addedConnections };
        }
        return diff;
    }

    private static bool flatPiecesById_PassThrough(Piece piece, out Coordinate? center)
    {
        var centerAttr = piece.Attributes.FirstOrDefault(a => a.Key == "compose.center");
        if (centerAttr?.Value is not null)
        {
            center = Utility.Deserialize<Coordinate>(centerAttr.Value);
            return true;
        }
        center = piece.Center;
        return center is not null;
    }
}

#endregion 📐Design






#region ⏱️Kit
// Implementations MUST collect types and designs into a reusable library.

#region 🧬KitKind
// KitKind discriminates the five persistence/transport forms of a Kit.

/// <summary>
/// 🏷️Discriminator for the five kit persistence/transport forms.
/// </summary>
/// <remarks>
/// Specs: Exactly five kit kinds exist:
/// - Dev: Self-contained JSON file
/// - Local: Local folder layout on disk (imported/exported via compose-gql)
/// - Archive: ZIP file packaging a LocalKit structure
/// - Remote: URL-addressable kit served over HTTP(S)
/// - Transport: In-memory ephemeral kit transport payload
/// </remarks>
[JsonConverter(typeof(StringEnumConverter))]
public enum KitKind
{
    [EnumMember(Value = "dev")]
    Dev,
    [EnumMember(Value = "local")]
    Local,
    [EnumMember(Value = "archive")]
    Archive,
    [EnumMember(Value = "remote")]
    Remote,
    [EnumMember(Value = "transport")]
    Transport
}

/// <summary>🔷Helpers for KitKind.</summary>
public static class KitKinds
{
    /// <summary>🔑All valid KitKind values.</summary>
    public static readonly KitKind[] All = (KitKind[])Enum.GetValues(typeof(KitKind));
}

#endregion 🧬KitKind

public class KitDiff : Entity<KitDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _id;
    private string? _name;
    private string? _description;
    private string? _icon;
    private string? _image;
    private string? _preview;
    private string? _version;
    private string? _remote;
    private string? _homepage;
    private string? _license;
    private TypologiesDiff? _typologies;
    private TagsDiff? _tags;
    private FilesDiff? _files;
    private FoldersDiff? _folders;
    private PortsDiff? _ports;
    private AuthorsDiff? _authors;
    private AttributesDiff? _attributes;
    private ConceptsDiff? _concepts;
    private string? _createdAt;
    private string? _updatedAt;

    public string? Id { get => _id; set { _id = value; _setProperties.Add("Id"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public string? Image { get => _image; set { _image = value; _setProperties.Add("Image"); } }
    public string? Preview { get => _preview; set { _preview = value; _setProperties.Add("Preview"); } }
    public string? Version { get => _version; set { _version = value; _setProperties.Add("Version"); } }
    public string? Remote { get => _remote; set { _remote = value; _setProperties.Add("Remote"); } }
    public string? Homepage { get => _homepage; set { _homepage = value; _setProperties.Add("Homepage"); } }
    public string? License { get => _license; set { _license = value; _setProperties.Add("License"); } }
    public TypologiesDiff? Typologies { get => _typologies; set { _typologies = value; _setProperties.Add("Typologies"); } }
    public TagsDiff? Tags { get => _tags; set { _tags = value; _setProperties.Add("Tags"); } }
    public FilesDiff? Files { get => _files; set { _files = value; _setProperties.Add("Files"); } }
    public FoldersDiff? Folders { get => _folders; set { _folders = value; _setProperties.Add("Folders"); } }
    public PortsDiff? Ports { get => _ports; set { _ports = value; _setProperties.Add("Ports"); } }
    public AuthorsDiff? Authors { get => _authors; set { _authors = value; _setProperties.Add("Authors"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public ConceptsDiff? Concepts { get => _concepts; set { _concepts = value; _setProperties.Add("Concepts"); } }
    public string? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public string? ModificationdAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("ModificationdAt"); } }

    public bool ShouldSerializeId() => _setProperties.Contains("Id");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeImage() => _setProperties.Contains("Image");
    public bool ShouldSerializePreview() => _setProperties.Contains("Preview");
    public bool ShouldSerializeVersion() => _setProperties.Contains("Version");
    public bool ShouldSerializeRemote() => _setProperties.Contains("Remote");
    public bool ShouldSerializeHomepage() => _setProperties.Contains("Homepage");
    public bool ShouldSerializeLicense() => _setProperties.Contains("License");
    public bool ShouldSerializeTypologies() => _setProperties.Contains("Typologies");
    public bool ShouldSerializeTags() => _setProperties.Contains("Tags");
    public bool ShouldSerializeFiles() => _setProperties.Contains("Files");
    public bool ShouldSerializeFolders() => _setProperties.Contains("Folders");
    public bool ShouldSerializePorts() => _setProperties.Contains("Ports");
    public bool ShouldSerializeAuthors() => _setProperties.Contains("Authors");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeConcepts() => _setProperties.Contains("Concepts");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeModificationdAt() => _setProperties.Contains("ModificationdAt");

    public KitDiff MergeDiff(KitDiff other)
    {
        return new KitDiff
        {
            Id = other.Id ?? Id,
            Name = other.Name ?? Name,
            Description = other.Description ?? Description,
            Icon = other.Icon ?? Icon,
            Image = other.Image ?? Image,
            Preview = other.Preview ?? Preview,
            Version = other.Version ?? Version,
            Remote = other.Remote ?? Remote,
            Homepage = other.Homepage ?? Homepage,
            License = other.License ?? License,
            Typologies = other.Typologies ?? Typologies,
            Files = other.Files ?? Files,
            Folders = other.Folders ?? Folders,
            Ports = other.Ports ?? Ports,
            Authors = other.Authors ?? Authors,
            Attributes = other.Attributes ?? Attributes,
            Concepts = other.Concepts ?? Concepts,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            ModificationdAt = other.ModificationdAt ?? ModificationdAt
        };
    }

    public static implicit operator KitDiff(Kit kit) => new()
    {
        Id = kit.Id,
        Name = kit.Name,
        Description = kit.Description,
        Icon = kit.Icon,
        Image = kit.Image,
        Preview = kit.Preview,
        Version = kit.Version,
        Remote = kit.Remote,
        Homepage = kit.Homepage,
        License = kit.License,
        Concepts = new ConceptsDiff { Added = kit.Concepts, Removed = new List<ConceptId>(), Modified = new List<ConceptModification>() },
        CreatedAt = kit.CreatedAt,
        ModificationdAt = kit.ModificationdAt
    };
}

public class KitId : Entity<KitId>
{
    public string Id { get; set; } = "";
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"KitId({ToHumanIdString()})";

    public static implicit operator KitId(Kit kit) => new() { Id = kit.Id };
    public static implicit operator KitId(KitDiff diff) => new() { Id = diff.Id ?? "" };
}

public class KitsDiff : Entity<KitsDiff>
{
    public List<KitId> Removed { get; set; } = new();
    public List<KitModification> Modified { get; set; } = new();
    public List<Kit> Added { get; set; } = new();

    public static implicit operator KitsDiff(List<Kit> kits) => new() { Modified = kits.Select(k => new KitModification { Kit = k, Diff = (KitDiff)k }).ToList() };
}

public partial class Kit : Entity<Kit>
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string Version { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public List<Concept> Concepts { get; set; } = new();
    public List<Tag> Tags { get; set; } = new();
    public string? Remote { get; set; }
    public string? Homepage { get; set; }
    public string? License { get; set; }
    public List<Author> Authors { get; set; } = new();
    public List<Piece> Pieces { get; set; } = new();
    public List<Group> Groups { get; set; } = new();
    public List<Connection> Connections { get; set; } = new();
    public List<Prop> Props { get; set; } = new();
    public List<Stat> Stats { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public string? Preview { get; set; }
    public List<Quality> Qualities { get; set; } = new();
    [JsonProperty("ports")]
    public List<Port> Ports { get; set; } = new();
    public List<File> Files { get; set; } = new();
    public List<Folder> Folders { get; set; } = new();
    [JsonProperty("typologies")]
    public List<Typology> Typologies { get; set; } = new();
    [JsonIgnore]
    public List<Type> Types { get; set; } = new();
    [JsonIgnore]
    public List<Design> Designs { get; set; } = new();
    public string CreatedAt { get; set; } = "";
    public string ModificationdAt { get; set; } = "";

    public void EnsureTypologies()
    {
        if (Typologies.Count > 0)
        {
            FlattenFromTypologies();
            return;
        }
        if (Types.Count == 0 && Designs.Count == 0) return;
        var topoId = !string.IsNullOrEmpty(Types.FirstOrDefault()?.Typology.Id)
            ? Types[0].Typology.Id
            : !string.IsNullOrEmpty(Designs.FirstOrDefault()?.Typology.Id)
                ? Designs[0].Typology.Id
                : Guid.NewGuid().ToString();
        foreach (var t in Types) t.Typology = topoId;
        foreach (var d in Designs) d.Typology = topoId;
        Typologies = new List<Typology> { new() { Id = topoId, Name = "Default", Types = Types, Designs = Designs } };
        FlattenFromTypologies();
    }

    public void FlattenFromTypologies()
    {
        Types = new List<Type>();
        Designs = new List<Design>();
        foreach (var topo in Typologies)
        {
            foreach (var t in topo.Types)
            {
                if (string.IsNullOrEmpty(t.Typology.Id)) t.Typology = topo.Id;
                Types.Add(t);
            }
            foreach (var d in topo.Designs)
            {
                if (string.IsNullOrEmpty(d.Typology.Id)) d.Typology = topo.Id;
                Designs.Add(d);
            }
        }
    }

    public static implicit operator Kit(KitDiff diff) => new() { Name = diff.Name ?? "", Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Preview = diff.Preview ?? "", Version = diff.Version ?? "", Remote = diff.Remote ?? "", Homepage = diff.Homepage ?? "", License = diff.License ?? "", Files = diff.Files?.Added ?? new(), Attributes = diff.Attributes?.Added ?? new() };
    public static implicit operator string(Kit kit) => kit.Name;
    public static implicit operator Kit(string name) => new() { Name = name };

    public static Kit ApplyDiff(Kit kit, KitDiff diff)
    {
        kit.EnsureTypologies();
        var typologies = kit.Typologies;
        var files = kit.Files;
        var attributes = kit.Attributes;

        if (diff.Typologies is not null)
        {
            typologies = ApplyTypologiesDiff(kit.Typologies, diff.Typologies);
        }
        if (diff.Files is not null)
        {
            files = ApplyFilesDiff(kit.Files, diff.Files);
        }
        if (diff.Attributes is not null)
        {
            attributes = ApplyAttributesDiff(kit.Attributes, diff.Attributes);
        }

        var result = new Kit
        {
            Name = string.IsNullOrEmpty(diff.Name) ? kit.Name : diff.Name,
            Description = string.IsNullOrEmpty(diff.Description) ? kit.Description : diff.Description,
            Icon = string.IsNullOrEmpty(diff.Icon) ? kit.Icon : diff.Icon,
            Image = string.IsNullOrEmpty(diff.Image) ? kit.Image : diff.Image,
            Preview = string.IsNullOrEmpty(diff.Preview) ? kit.Preview : diff.Preview,
            Version = string.IsNullOrEmpty(diff.Version) ? kit.Version : diff.Version,
            Remote = string.IsNullOrEmpty(diff.Remote) ? kit.Remote : diff.Remote,
            Homepage = string.IsNullOrEmpty(diff.Homepage) ? kit.Homepage : diff.Homepage,
            License = string.IsNullOrEmpty(diff.License) ? kit.License : diff.License,
            Authors = kit.Authors,
            Qualities = kit.Qualities,
            Files = files,
            Typologies = typologies,
            Attributes = attributes
        };
        result.FlattenFromTypologies();
        return result;
    }

    private static List<Typology> ApplyTypologiesDiff(List<Typology> original, TypologiesDiff diff)
    {
        var result = original.Where(t => !diff.Removed.Any(r => r.Id == t.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(t => t.Id == updated.Typology.Id);
            if (index >= 0)
            {
                var topo = result[index];
                if (updated.Diff.Name is not null) topo.Name = updated.Diff.Name;
                if (updated.Diff.Description is not null) topo.Description = updated.Diff.Description;
                if (updated.Diff.Icon is not null) topo.Icon = updated.Diff.Icon;
                if (updated.Diff.Folder is not null) topo.Folder = updated.Diff.Folder;
                if (updated.Diff.Types is not null) topo.Types = ApplyTypesDiff(topo.Types, updated.Diff.Types);
                if (updated.Diff.Designs is not null) topo.Designs = ApplyDesignsDiff(topo.Designs, updated.Diff.Designs);
            }
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static List<Attribute> ApplyAttributesDiff(List<Attribute> original, AttributesDiff diff)
    {
        var result = original.Where(a => !diff.Removed.Any(r => r.Id == a.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(a => a.Id == updated.Attribute.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = Attribute.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    public static KitDiff CreateDiff(Kit kit)
    {
        kit.EnsureTypologies();
        return new KitDiff
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Typologies = new TypologiesDiff
            {
                Removed = new List<TypologyId>(),
                Added = new List<Typology>(),
                Modified = kit.Typologies.Select(topo => new TypologyModification
                {
                    Typology = topo,
                    Diff = new TypologyDiff
                    {
                        Types = new TypesDiff
                        {
                            Removed = new List<TypeId>(),
                            Modified = topo.Types.Select(t => new TypeModification { Type = t, Diff = Type.CreateDiff(t) }).ToList(),
                            Added = new List<Type>()
                        },
                        Designs = new DesignsDiff
                        {
                            Removed = new List<DesignId>(),
                            Modified = topo.Designs.Select(d => new DesignModification { Design = d, Diff = Design.CreateDiff(d) }).ToList(),
                            Added = new List<Design>()
                        }
                    }
                }).ToList()
            },
            Files = new FilesDiff
            {
                Removed = new List<FileId>(),
                Modified = kit.Files.Select(f => new FileModification { File = f, Diff = (FileDiff)f }).ToList(),
                Added = new List<File>()
            },
            Attributes = kit.Attributes
        };
    }

    private static List<Type> ApplyTypesDiff(List<Type> original, TypesDiff diff)
    {
        var result = original.Where(t => !diff.Removed.Any(r => r.Id == t.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(t => t.Id == updated.Type.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = Type.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static TypesDiff CreateTypesDiff(List<Type> original, List<Type> modified)
    {
        var originalIds = original.Select(t => t.Id).ToHashSet();
        var modifiedIds = modified.Select(t => t.Id).ToHashSet();

        return new TypesDiff
        {
            Removed = original.Where(t => !modifiedIds.Contains(t.Id)).Select(t => new TypeId { Id = t.Id }).ToList(),
            Modified = original.Where(t => modifiedIds.Contains(t.Id))
                .SelectMany(t =>
                {
                    var modifiedType = modified.First(m => m.Id == t.Id);
                    var diff = Type.CreateDiff(t);
                    return !Equals(t, modifiedType) ? new[] { new TypeModification { Type = t, Diff = diff } } : Array.Empty<TypeModification>();
                })
                .ToList(),
            Added = modified.Where(t => !originalIds.Contains(t.Id)).ToList()
        };
    }

    private static List<Design> ApplyDesignsDiff(List<Design> original, DesignsDiff diff)
    {
        var result = original.Where(d => !diff.Removed.Any(r => r.Id == d.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(d => d.Id == updated.Design.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = Design.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static DesignsDiff CreateDesignsDiff(List<Design> original, List<Design> modified)
    {
        var originalIds = original.Select(d => d.Id).ToHashSet();
        var modifiedIds = modified.Select(d => d.Id).ToHashSet();

        return new DesignsDiff
        {
            Removed = original.Where(d => !modifiedIds.Contains(d.Id)).Select(d => new DesignId { Id = d.Id }).ToList(),
            Modified = original.Where(d => modifiedIds.Contains(d.Id))
                .SelectMany(d =>
                {
                    var modifiedDesign = modified.First(m => m.Id == d.Id);
                    var diff = Design.GetDesignDiff(d, modifiedDesign);
                    return !Equals(d, modifiedDesign) ? new[] { new DesignModification { Design = d, Diff = diff } } : Array.Empty<DesignModification>();
                })
                .ToList(),
            Added = modified.Where(d => !originalIds.Contains(d.Id)).ToList()
        };
    }

    private static List<File> ApplyFilesDiff(List<File> original, FilesDiff diff)
    {
        var result = original.Where(f => !diff.Removed.Any(r => r.Id == f.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(f => f.Id == updated.File.Id);
            if (index >= 0 && updated.Diff != null)
            {
                var file = result[index];
                result[index] = new File
                {
                    Id = updated.Diff.Id ?? file.Id,
                    Name = updated.Diff.Name ?? file.Name,
                    Remote = updated.Diff.Remote ?? file.Remote,
                    Folder = updated.Diff.Folder ?? file.Folder,
                    Size = updated.Diff.Size ?? file.Size,
                    Hash = updated.Diff.Hash ?? file.Hash,
                    CreatedAt = updated.Diff.CreatedAt ?? file.CreatedAt,
                    CreatedBy = updated.Diff.CreatedBy ?? file.CreatedBy,
                    ModificationdAt = updated.Diff.ModificationdAt ?? file.ModificationdAt,
                    ModificationdBy = updated.Diff.ModificationdBy ?? file.ModificationdBy
                };
            }
        }
        result.AddRange(diff.Added);
        return result;
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();

        foreach (var type in Types)
        {
            var (isValidType, errorsType) = type.Validate();
            isValid = isValid && isValidType;
            errors.AddRange(errorsType.Select(e => $"A type ({type.ToIdString()}) is invalid: " + e));
        }
        foreach (var design in Designs)
        {
            var (isValidDesign, errorsDesign) = design.Validate();
            isValid = isValid && isValidDesign;
            errors.AddRange(errorsDesign.Select(e => $"A design ({design.ToIdString()}) is invalid: " + e));
        }
        var typeIds = Types.Select(t => t.Name);
        var duplicateTypeIds = typeIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicateTypeIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicateName in duplicateTypeIds)
            {
                errors.Add($"There are multiple identical types ({duplicateName}).");
            }
        }
        var designIds = Designs.Select(d => d.Id);
        var duplicateDesignIds = designIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicateDesignIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicateName in duplicateDesignIds)
            {
                errors.Add($"There are multiple identical designs ({duplicateName}).");
            }
        }
        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute ({attribute.ToIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public static bool IsSameAs(Kit kit, Kit other)
    {
        if (other is null) return false;
        return kit.Name == other.Name;
    }

    public static Type FindTypeByName(Kit kit, string typeName)
    {
        var type = kit.Types.FirstOrDefault(t => t.Name == typeName);
        if (type is null) throw new ArgumentException($"Type {typeName} not found in kit {kit.Name}");
        return type;
    }

    public static Design FindDesignByName(Kit kit, string designName)
    {
        var design = kit.Designs.FirstOrDefault(d => d.Name == designName);
        if (design is null) throw new ArgumentException($"Design {designName} not found in kit {kit.Name}");
        return design;
    }

    public static Kit AddType(Kit kit, Type type)
    {
        var newTypes = new List<Type>(kit.Types) { type };
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = newTypes,
            Designs = new List<Design>(kit.Designs),
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static Kit RemoveType(Kit kit, string typeName)
    {
        var newTypes = kit.Types.Where(t => t.Name != typeName).ToList();
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = newTypes,
            Designs = new List<Design>(kit.Designs),
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static Kit AddDesign(Kit kit, Design design)
    {
        var newDesigns = new List<Design>(kit.Designs) { design };
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = new List<Type>(kit.Types),
            Designs = newDesigns,
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static Kit RemoveDesign(Kit kit, string designName)
    {
        var newDesigns = kit.Designs.Where(d => d.Name != designName).ToList();
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = new List<Type>(kit.Types),
            Designs = newDesigns,
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static string FindAttributeValue(Kit kit, string key, string defaultValue = "")
    {
        var attribute = kit.Attributes.FirstOrDefault(a => a.Key == key);
        return attribute?.Value ?? defaultValue;
    }

    public static Kit SetAttribute(Kit kit, Attribute attribute)
    {
        var newAttributes = kit.Attributes.Where(a => a.Key != attribute.Key).ToList();
        newAttributes.Add(attribute);
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = new List<Type>(kit.Types),
            Designs = new List<Design>(kit.Designs),
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = newAttributes
        };
    }

    #region 📻Design Family Helpers
    // Callers MUST use these helpers to traverse design parent-child hierarchies.

    public static Design FindDesignById(Kit kit, string designId)
    {
        var design = kit.Designs.FirstOrDefault(d => d.Id == designId);
        if (design is null) throw new ArgumentException($"Design {designId} not found in kit {kit.Name}");
        return design;
    }

    public static Design GetPrimitiveDesign(Kit kit, string designId)
    {
        var current = FindDesignById(kit, designId);
        while (current.Parent?.Id is not null)
        {
            current = FindDesignById(kit, current.Parent.Id);
        }
        return current;
    }

    public static List<Design> GetDesignFamily(Kit kit, string designId)
    {
        var primitive = GetPrimitiveDesign(kit, designId);
        var family = new List<Design>();
        CollectDesignDescendants(kit, primitive.Id, family);
        return family;
    }

    private static void CollectDesignDescendants(Kit kit, string parentId, List<Design> family)
    {
        var parent = FindDesignById(kit, parentId);
        family.Add(parent);
        var children = kit.Designs.Where(d => d.Parent?.Id == parentId);
        foreach (var child in children)
        {
            CollectDesignDescendants(kit, child.Id, family);
        }
    }

    public static bool AreDesignsInSameFamily(Kit kit, string designIdA, string designIdB)
    {
        var primitiveA = GetPrimitiveDesign(kit, designIdA);
        var primitiveB = GetPrimitiveDesign(kit, designIdB);
        return primitiveA.Id == primitiveB.Id;
    }

    public static bool CanUseDesignAsPiece(Kit kit, string containerDesignId, string pieceDesignId)
    {
        return !AreDesignsInSameFamily(kit, containerDesignId, pieceDesignId);
    }

    public static List<Piece> FindSameFamilyDesignPieces(Kit kit, string designId)
    {
        var design = FindDesignById(kit, designId);
        return design.Pieces
            .Where(p => p.Design?.Id is not null && AreDesignsInSameFamily(kit, designId, p.Design.Id))
            .ToList();
    }

    #endregion 📻Design Family Helpers

    #region 🧊Type Family Helpers
    // Callers MUST use these helpers to traverse type parent-child hierarchies.

    public static Type FindTypeById(Kit kit, string typeId)
    {
        var type = kit.Types.FirstOrDefault(t => t.Id == typeId);
        if (type is null) throw new ArgumentException($"Type {typeId} not found in kit {kit.Name}");
        return type;
    }

    public static Type GetPrimitiveType(Kit kit, string typeId)
    {
        var current = FindTypeById(kit, typeId);
        while (current.Parent?.Id is not null)
        {
            current = FindTypeById(kit, current.Parent.Id);
        }
        return current;
    }

    public static List<Type> GetTypeFamily(Kit kit, string typeId)
    {
        var primitive = GetPrimitiveType(kit, typeId);
        var family = new List<Type>();
        CollectTypeDescendants(kit, primitive.Id, family);
        return family;
    }

    private static void CollectTypeDescendants(Kit kit, string parentId, List<Type> family)
    {
        var parent = FindTypeById(kit, parentId);
        family.Add(parent);
        var children = kit.Types.Where(t => t.Parent?.Id == parentId);
        foreach (var child in children)
        {
            CollectTypeDescendants(kit, child.Id, family);
        }
    }

    public static bool AreTypesInSameFamily(Kit kit, string typeIdA, string typeIdB)
    {
        var primitiveA = GetPrimitiveType(kit, typeIdA);
        var primitiveB = GetPrimitiveType(kit, typeIdB);
        return primitiveA.Id == primitiveB.Id;
    }

    #endregion 🧊Type Family Helpers

    #region 🔍Kit Finders
    // Callers MUST use these methods to locate entities within a kit by ID.

    public static File FindFile(Kit kit, string fileId)
    {
        var file = kit.Files?.FirstOrDefault(f => f.Id == fileId);
        if (file == null) throw new Exception($"File {fileId} not found in kit {kit.Name}");
        return file;
    }

    public static Tag FindTag(Kit kit, string tagId)
    {
        var tag = kit.Tags?.FirstOrDefault(t => t.Id == tagId);
        if (tag == null) throw new Exception($"Tag {tagId} not found in kit {kit.Name}");
        return tag;
    }

    public static Concept FindConcept(Kit kit, string conceptId)
    {
        var concept = kit.Concepts?.FirstOrDefault(c => c.Id == conceptId);
        if (concept == null) throw new Exception($"Concept {conceptId} not found in kit {kit.Name}");
        return concept;
    }

    public static Type FindType(Kit kit, string typeId)
    {
        var type = kit.Types?.FirstOrDefault(t => t.Id == typeId);
        if (type == null) throw new Exception($"Type {typeId} not found in kit {kit.Name}");
        return type;
    }

    public static Design FindDesign(Kit kit, string designId)
    {
        var design = kit.Designs?.FirstOrDefault(d => d.Id == designId);
        if (design == null) throw new Exception($"Design {designId} not found in kit {kit.Name}");
        return design;
    }

    public static Port FindPort(Kit kit, string portId)
    {
        var port = kit.Ports?.FirstOrDefault(p => p.Id == portId);
        if (port == null) throw new Exception($"Port {portId} not found in kit {kit.Name}");
        return port;
    }

    public static Type FindPieceTypeInDesign(Kit kit, string designId, string pieceId)
    {
        var design = FindDesign(kit, designId);
        var piece = Piece.FindInDesign(design, pieceId);
        if (piece.Type == null) throw new Exception($"Piece {pieceId} has no type");
        return FindType(kit, piece.Type.Id);
    }

    public static Piece FindParentPieceInDesign(Kit kit, string designId, string pieceId)
    {
        var design = FindDesign(kit, designId);
        var connection = Connection.FindByPieceInDesign(design, pieceId).FirstOrDefault(c => c.Child.Piece.Id == pieceId);
        if (connection == null) throw new Exception($"No parent piece found for piece {pieceId}");
        return Piece.FindInDesign(design, connection.Parent.Piece.Id);
    }

    public static Connection FindParentConnectionForPieceInDesign(Kit kit, string designId, string pieceId)
    {
        var design = FindDesign(kit, designId);
        var connection = Connection.FindByPieceInDesign(design, pieceId).FirstOrDefault(c => c.Child.Piece.Id == pieceId);
        if (connection == null) throw new Exception($"No parent connection found for piece {pieceId}");
        return connection;
    }

    public static List<Piece> FindChildrenPiecesInDesign(Kit kit, string designId, string pieceId)
    {
        var design = FindDesign(kit, designId);
        var connections = Connection.FindByPieceInDesign(design, pieceId).Where(c => c.Parent.Piece.Id == pieceId);
        return connections.Select(c => Piece.FindInDesign(design, c.Child.Piece.Id)).ToList();
    }

    public static List<Connector> FindUsedConnectorsByPieceInDesign(Kit kit, string designId, string pieceId)
    {
        var design = FindDesign(kit, designId);
        var piece = Piece.FindInDesign(design, pieceId);
        var type = piece.Type != null ? FindType(kit, piece.Type.Id) : null;
        if (type == null) return new List<Connector>();

        var connections = Connection.FindByPieceInDesign(design, pieceId);
        var connectors = new List<Connector>();
        foreach (var connection in connections)
        {
            var connector = Connector.FindForPieceInConnection(type, connection, pieceId);
            if (connector != null) connectors.Add(connector);
        }
        return connectors;
    }

    public static Type[] FindReplacableTypesForPieceInDesign(Kit kit, string designId, string pieceId, string[]? variants = null)
    {
        var design = FindDesign(kit, designId);
        var connections = Connection.FindByPieceInDesign(design, pieceId);
        var requiredConnectors = new List<Connector>();

        foreach (var connection in connections)
        {
            try
            {
                var otherPieceId = connection.Parent.Piece.Id == pieceId ? connection.Child.Piece.Id : connection.Parent.Piece.Id;
                var otherPiece = Piece.FindInDesign(design, otherPieceId);
                if (otherPiece.Type == null) continue;

                var otherType = FindType(kit, otherPiece.Type.Id);
                var otherPortId = connection.Parent.Piece.Id == pieceId ? connection.Child.Connector?.Id : connection.Parent.Connector?.Id;
                var otherPort = Connector.FindInType(otherType, otherPortId ?? "");
                requiredConnectors.Add(otherPort);
            }
            catch
            {
                continue;
            }
        }

        return (kit.Types ?? new List<Type>()).Where(replacementType =>
        {
            if (replacementType.IsAbstract ?? false) return false;
            if (variants != null && !variants.Contains(replacementType.Parent?.Id ?? "")) return false;
            if (replacementType.Connectors == null || replacementType.Connectors.Count == 0) return requiredConnectors.Count == 0;

            return requiredConnectors.All(requiredConnector =>
            {
                return replacementType.Connectors.Any(replacementConnector => replacementConnector.Id == requiredConnector.Id);
            });
        }).ToArray();
    }

    public static Type[] FindReplacableTypesForPiecesInDesign(Kit kit, string designId, string[] pieceIds, string[]? variants = null)
    {
        var design = FindDesign(kit, designId);
        var pieces = pieceIds.Select(id => Piece.FindInDesign(design, id)).ToList();
        var externalConnections = new List<(Connection connection, Connector requiredConnector)>();

        foreach (var piece in pieces)
        {
            var connections = Connection.FindByPieceInDesign(design, piece.Id);
            foreach (var connection in connections)
            {
                var otherPieceId = connection.Parent.Piece.Id == piece.Id ? connection.Child.Piece.Id : connection.Parent.Piece.Id;
                if (!pieceIds.Contains(otherPieceId))
                {
                    try
                    {
                        var otherPiece = Piece.FindInDesign(design, otherPieceId);
                        if (otherPiece.Type == null) continue;

                        var otherType = FindType(kit, otherPiece.Type.Id);
                        var otherPortId = connection.Parent.Piece.Id == piece.Id ? connection.Child.Connector?.Id : connection.Parent.Connector?.Id;
                        var otherPort = Connector.FindInType(otherType, otherPortId ?? "");
                        externalConnections.Add((connection, otherPort));
                    }
                    catch
                    {
                        continue;
                    }
                }
            }
        }

        return (kit.Types ?? new List<Type>()).Where(replacementType =>
        {
            if (replacementType.IsAbstract ?? false) return false;
            if (variants != null && !variants.Contains(replacementType.Parent?.Id ?? "")) return false;
            if (replacementType.Connectors == null || replacementType.Connectors.Count == 0) return externalConnections.Count == 0;

            return externalConnections.All(ec =>
            {
                return replacementType.Connectors.Any(replacementConnector => replacementConnector.Id == ec.requiredConnector.Id);
            });
        }).ToArray();
    }

    public static double SumQualityInDesign(Kit kit, string designId, string qualityId)
    {
        var design = FindDesign(kit, designId);
        double sum = 0;
        foreach (var piece in design.Pieces ?? new List<Piece>())
        {
            var pieceProp = piece.Props?.FirstOrDefault(p => p.Quality?.Id == qualityId);
            if (pieceProp != null)
            {
                if (double.TryParse(pieceProp.Value, System.Globalization.NumberStyles.Float, System.Globalization.CultureInfo.InvariantCulture, out var val))
                    sum += val;
                continue;
            }
            if (piece.Type != null)
            {
                var type = kit.Types?.FirstOrDefault(t => t.Id == piece.Type.Id);
                if (type != null)
                {
                    var typeProp = type.Props?.FirstOrDefault(p => p.Quality?.Id == qualityId);
                    if (typeProp != null && double.TryParse(typeProp.Value, System.Globalization.NumberStyles.Float, System.Globalization.CultureInfo.InvariantCulture, out var val))
                        sum += val;
                }
            }
        }
        return sum;
    }

    #endregion 🔍Kit Finders

    #region 🎠Filter
    // Filter MUST provide functions to produce a minimal kit subset scoped to a single design.

    /// <summary>🧩Glob filter with include and exclude patterns for name-based entity filtering.</summary>
    public class GlobFilter
    {
        public List<string>? Include { get; set; }
        public List<string>? Exclude { get; set; }
    }

    /// <summary>🧹General-purpose kit filter combining design-based transitive filtering with glob-based name filtering.</summary>
    public class KitFilter
    {
        public string? DesignId { get; set; }
        public string[]? RepresentationTags { get; set; }
        public GlobFilter? Designs { get; set; }
        public GlobFilter? Types { get; set; }
        public GlobFilter? Ports { get; set; }
        public GlobFilter? Files { get; set; }
        public GlobFilter? Tags { get; set; }
        public GlobFilter? Concepts { get; set; }
        public GlobFilter? Qualities { get; set; }
        public GlobFilter? Authors { get; set; }
        public GlobFilter? Folders { get; set; }
    }

    /// <summary>🔤Matches a name against a glob pattern supporting * and ?. Case-insensitive.</summary>
    public static bool GlobMatch(string name, string pattern)
    {
        var regexStr = "^";
        foreach (var c in pattern)
        {
            regexStr += c switch
            {
                '*' => ".*",
                '?' => ".",
                _ => System.Text.RegularExpressions.Regex.Escape(c.ToString())
            };
        }
        regexStr += "$";
        return System.Text.RegularExpressions.Regex.IsMatch(name, regexStr, System.Text.RegularExpressions.RegexOptions.IgnoreCase);
    }

    /// <summary>🧪Checks if a name passes a GlobFilter.</summary>
    public static bool MatchesGlobFilter(string name, GlobFilter? filter)
    {
        if (filter == null) return true;
        if (filter.Include is { Count: > 0 } && !filter.Include.Any(p => GlobMatch(name, p))) return false;
        if (filter.Exclude is { Count: > 0 } && filter.Exclude.Any(p => GlobMatch(name, p))) return false;
        return true;
    }

    /// <summary>📐Filters a kit to only include entities related to a specific design.</summary>
    /// <remarks>
    /// Removes types not used by pieces, designs not used by pieces, ports not used by connectors of used types,
    /// files not used by selected representations, and keeps at most one representation per type according to the optional tags.
    /// </remarks>
    private static Kit FilterKitByDesign(Kit kit, string designId, string[]? tags = null)
    {
        var design = (kit.Designs ?? new List<Design>()).FirstOrDefault(d => d.Id == designId);
        if (design == null) return new Kit { Id = kit.Id, Name = kit.Name, Version = kit.Version };

        var usedTypeIds = new HashSet<string>();
        var usedDesignIds = new HashSet<string> { designId };
        foreach (var piece in design.Pieces ?? new List<Piece>())
        {
            if (!string.IsNullOrEmpty(piece.Type?.Id)) usedTypeIds.Add(piece.Type.Id);
            if (!string.IsNullOrEmpty(piece.Design?.Id)) usedDesignIds.Add(piece.Design.Id);
        }

        var typeById = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Id, t => t);
        void CollectAncestors(string typeId)
        {
            if (!typeById.TryGetValue(typeId, out var type) || string.IsNullOrEmpty(type.Parent?.Id) || usedTypeIds.Contains(type.Parent.Id)) return;
            usedTypeIds.Add(type.Parent.Id);
            CollectAncestors(type.Parent.Id);
        }
        foreach (var typeId in usedTypeIds.ToList()) CollectAncestors(typeId);

        var resolvedTagIds = new List<string>();
        foreach (var tagValue in tags ?? Array.Empty<string>())
        {
            var byId = (kit.Tags ?? new List<Tag>()).FirstOrDefault(t => t.Id == tagValue);
            if (byId != null) { resolvedTagIds.Add(byId.Id); continue; }
            resolvedTagIds.AddRange((kit.Tags ?? new List<Tag>()).Where(t => t.Name == tagValue).Select(t => t.Id));
        }

        var usedPortIds = new HashSet<string>();
        var usedFileIds = new HashSet<string>();
        var usedTagIds = new HashSet<string>();
        var usedConceptIds = new HashSet<string>();
        var usedQualityIds = new HashSet<string>();
        var usedAuthorIds = new HashSet<string>();
        var usedFolderNames = new HashSet<string>();
        var selectedRepresentations = new Dictionary<string, Representation>();

        void CollectQualityFromProps(IEnumerable<Prop>? props)
        {
            foreach (var prop in props ?? new List<Prop>())
                if (!string.IsNullOrEmpty(prop.Quality?.Id)) usedQualityIds.Add(prop.Quality.Id);
        }

        foreach (var typeId in usedTypeIds)
        {
            if (!typeById.TryGetValue(typeId, out var type)) continue;
            if (!string.IsNullOrEmpty(type.Folder)) usedFolderNames.Add(type.Folder);
            foreach (var connector in type.Connectors ?? new List<Connector>())
            {
                if (!string.IsNullOrEmpty(connector.Port?.Id)) usedPortIds.Add(connector.Port.Id);
                CollectQualityFromProps(connector.Props);
            }
            CollectQualityFromProps(type.Props);
            foreach (var author in type.Authors ?? new List<AuthorId>())
                if (!string.IsNullOrEmpty(author.Id)) usedAuthorIds.Add(author.Id);
            foreach (var concept in type.Concepts ?? new List<ConceptId>())
                if (!string.IsNullOrEmpty(concept.Id)) usedConceptIds.Add(concept.Id);

            if ((type.Representations?.Count ?? 0) > 0)
            {
                var best = ExportFindMatchingRepresentation(kit, type, resolvedTagIds.ToArray());
                if (best != null)
                {
                    selectedRepresentations[typeId] = best;
                    if (!string.IsNullOrEmpty(best.File?.Id)) usedFileIds.Add(best.File.Id);
                    foreach (var tag in best.Tags ?? new List<TagId>())
                        if (!string.IsNullOrEmpty(tag.Id)) usedTagIds.Add(tag.Id);
                }
            }
        }

        foreach (var piece in design.Pieces ?? new List<Piece>()) CollectQualityFromProps(piece.Props);
        foreach (var concept in design.Concepts ?? new List<ConceptId>())
            if (!string.IsNullOrEmpty(concept.Id)) usedConceptIds.Add(concept.Id);
        foreach (var author in design.Authors ?? new List<AuthorId>())
            if (!string.IsNullOrEmpty(author.Id)) usedAuthorIds.Add(author.Id);
        foreach (var portId in usedPortIds.ToList())
        {
            var port = (kit.Ports ?? new List<Port>()).FirstOrDefault(p => p.Id == portId);
            foreach (var compatible in port?.CompatiblePorts ?? new List<PortId>())
                if (!string.IsNullOrEmpty(compatible.Id)) usedPortIds.Add(compatible.Id);
        }
        foreach (var tagId in resolvedTagIds) usedTagIds.Add(tagId);

        var filteredTypes = (kit.Types ?? new List<Type>())
            .Where(t => usedTypeIds.Contains(t.Id))
            .Select(t =>
            {
                var clone = Entity<Type>.DeepClone(t)!;
                clone.Representations = selectedRepresentations.TryGetValue(t.Id, out var representation) ? new List<Representation> { representation } : new List<Representation>();
                return clone;
            })
            .ToList();

        return new Kit
        {
            Id = kit.Id,
            Name = kit.Name,
            Version = kit.Version,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = filteredTypes,
            Designs = (kit.Designs ?? new List<Design>()).Where(d => usedDesignIds.Contains(d.Id)).ToList(),
            Ports = (kit.Ports ?? new List<Port>()).Where(p => usedPortIds.Contains(p.Id)).ToList(),
            Files = (kit.Files ?? new List<File>()).Where(f => usedFileIds.Contains(f.Id)).ToList(),
            Tags = (kit.Tags ?? new List<Tag>()).Where(t => usedTagIds.Contains(t.Id)).ToList(),
            Concepts = (kit.Concepts ?? new List<Concept>()).Where(c => usedConceptIds.Contains(c.Id)).ToList(),
            Qualities = (kit.Qualities ?? new List<Quality>()).Where(q => usedQualityIds.Contains(q.Id)).ToList(),
            Folders = (kit.Folders ?? new List<Folder>()).Where(f => usedFolderNames.Contains(f.Name)).ToList(),
            Authors = (kit.Authors ?? new List<Author>()).Where(a => usedAuthorIds.Contains(a.Id)).ToList(),
            Attributes = kit.Attributes,
            CreatedAt = kit.CreatedAt,
            ModificationdAt = kit.ModificationdAt,
        };
    }

    /// <summary>❓General-purpose kit filter combining optional design-based transitive filtering with glob-based name filtering.</summary>
    /// <remarks>
    /// </remarks>
    public static Kit FilterKit(Kit kit, KitFilter filter)
    {
        var baseKit = !string.IsNullOrEmpty(filter.DesignId)
            ? FilterKitByDesign(kit, filter.DesignId, filter.RepresentationTags)
            : kit;

        var hasGlobFilters = filter.Designs != null || filter.Types != null || filter.Ports != null ||
            filter.Files != null || filter.Tags != null || filter.Concepts != null ||
            filter.Qualities != null || filter.Authors != null || filter.Folders != null;

        if (!hasGlobFilters) return baseKit;

        return new Kit
        {
            Id = baseKit.Id,
            Name = baseKit.Name,
            Version = baseKit.Version,
            Description = baseKit.Description,
            Icon = baseKit.Icon,
            Image = baseKit.Image,
            Preview = baseKit.Preview,
            Remote = baseKit.Remote,
            Homepage = baseKit.Homepage,
            License = baseKit.License,
            Types = (baseKit.Types ?? new List<Type>()).Where(t => MatchesGlobFilter(t.Name, filter.Types)).ToList(),
            Designs = (baseKit.Designs ?? new List<Design>()).Where(d => MatchesGlobFilter(d.Name, filter.Designs)).ToList(),
            Ports = (baseKit.Ports ?? new List<Port>()).Where(p => MatchesGlobFilter(p.Name, filter.Ports)).ToList(),
            Files = (baseKit.Files ?? new List<File>()).Where(f => MatchesGlobFilter(f.Name, filter.Files)).ToList(),
            Tags = (baseKit.Tags ?? new List<Tag>()).Where(t => MatchesGlobFilter(t.Name, filter.Tags)).ToList(),
            Concepts = (baseKit.Concepts ?? new List<Concept>()).Where(c => MatchesGlobFilter(c.Name, filter.Concepts)).ToList(),
            Qualities = (baseKit.Qualities ?? new List<Quality>()).Where(q => MatchesGlobFilter(q.Name, filter.Qualities)).ToList(),
            Folders = (baseKit.Folders ?? new List<Folder>()).Where(f => MatchesGlobFilter(f.Name, filter.Folders)).ToList(),
            Authors = (baseKit.Authors ?? new List<Author>()).Where(a => MatchesGlobFilter(a.Name, filter.Authors)).ToList(),
            Attributes = baseKit.Attributes,
            CreatedAt = baseKit.CreatedAt,
            ModificationdAt = baseKit.ModificationdAt,
        };
    }

    #endregion 🎠Filter
}

#endregion ⏱️Kit


public partial class Kit
{
}


#region 🔑Meta And Shallow
// Meta classes strip List<> and heavy blob properties. Shallow classes replace List<> properties with Meta item lists.

#region 🎼Sub-entity Meta

public class AttributeMeta
{
    public string Id { get; set; } = "";
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
    public string Definition { get; set; } = "";
}

public class PropMeta
{
    public string Id { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string Value { get; set; } = "";
    public string? Unit { get; set; }
}

public class TagMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
}

public class ConceptMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
}

public class AuthorMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string Email { get; set; } = "";
}

public class FileMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Remote { get; set; }
    public FolderId? Folder { get; set; }
    public int? Size { get; set; }
    public string? Hash { get; set; }
    public DateTime CreatedAt { get; set; }
    public string? CreatedBy { get; set; }
    public DateTime ModificationdAt { get; set; }
    public string? ModificationdBy { get; set; }
}

public class FolderMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public FolderId? Parent { get; set; }
    public string? Description { get; set; }
    public string CreatedAt { get; set; } = "";
    public string? CreatedBy { get; set; }
    public string ModificationdAt { get; set; } = "";
    public string? ModificationdBy { get; set; }
}

public class QualityMeta
{
    public string Id { get; set; } = "";
    public string Key { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Uri { get; set; }
    public string? Folder { get; set; }
    public bool? Scalable { get; set; }
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string? SI { get; set; }
    public string? Imperial { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public double? Default { get; set; }
    public string? Formula { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string? Unit { get; set; }
}

public class PortMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public int? MaxChildren { get; set; }
}

public class RepresentationMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public FileId File { get; set; } = new();
    public string? Description { get; set; }
}

public class ConnectorMeta
{
    public string Id { get; set; } = "";
    public string? Name { get; set; }
    public double T { get; set; } = 0;
    public Point? Point { get; set; }
    public Vector? Direction { get; set; }
    public string? Description { get; set; }
    public PortId? Port { get; set; }
    public bool? Mandatory { get; set; }
    public int? MaxChildren { get; set; }
}

public class LayerMeta
{
    public string Id { get; set; } = "";
    public string Path { get; set; } = "";
    public bool IsHidden { get; set; } = false;
    public bool IsLocked { get; set; } = false;
    public string Color { get; set; } = "";
    public string? Description { get; set; }
}

public class PieceMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public TypeId? Type { get; set; }
    public DesignId? Design { get; set; }
    public Plane? Plane { get; set; }
    public Coordinate? Center { get; set; }
    public double? Scale { get; set; }
    public Plane? MirrorPlane { get; set; }
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    public string? Color { get; set; }
}

public class GroupMeta
{
    public string Id { get; set; } = "";
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Color { get; set; }
}

public class ConnectionMeta
{
    public string Id { get; set; } = "";
    public Side Parent { get; set; } = new();
    public Side Child { get; set; } = new();
    public string? Description { get; set; }
    public double Gap { get; set; } = 0;
    public double Shift { get; set; } = 0;
    public double Rise { get; set; } = 0;
    public double Rotation { get; set; } = 0;
    public double Turn { get; set; } = 0;
    public double Tilt { get; set; } = 0;
    public double? U { get; set; }
    public double? V { get; set; }
}

public class StatMeta
{
    public string Id { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string? Unit { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
}

#endregion 🎼Sub-entity Meta

#region 🪁TypeMetaShallow

public class TypeMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public int? Stock { get; set; }
    public bool? Virtual { get; set; }
    public string? Uri { get; set; }
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime ModificationdAt { get; set; }
}

public class TypeShallow
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public int? Stock { get; set; }
    public bool? Virtual { get; set; }
    public string? Uri { get; set; }
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime ModificationdAt { get; set; }
    public List<RepresentationMeta> Representations { get; set; } = new();
    public List<ConnectorMeta> Connectors { get; set; } = new();
    public List<PropMeta> Props { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public List<ConceptId> Concepts { get; set; } = new();
    public List<AttributeMeta> Attributes { get; set; } = new();
}

#endregion 🪁TypeMetaShallow

#region ✨DesignMetaShallow

public class DesignMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public DesignId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public Location? Location { get; set; }
    public string Unit { get; set; } = "";
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public LayerId? ActiveLayer { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime ModificationdAt { get; set; }
}

public class DesignShallow
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public DesignId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public Location? Location { get; set; }
    public string Unit { get; set; } = "";
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public LayerId? ActiveLayer { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime ModificationdAt { get; set; }
    public List<PieceMeta> Pieces { get; set; } = new();
    public List<ConnectionMeta> Connections { get; set; } = new();
    public List<StatMeta> Stats { get; set; } = new();
    public List<PropMeta> Props { get; set; } = new();
    public List<LayerMeta> Layers { get; set; } = new();
    public List<GroupMeta> Groups { get; set; } = new();
    public List<AttributeMeta> Attributes { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public List<ConceptId> Concepts { get; set; } = new();
}

#endregion ✨DesignMetaShallow

#region 🏗️KitMetaShallow

public class KitMeta
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string Version { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string Remote { get; set; } = "";
    public string Homepage { get; set; } = "";
    public string License { get; set; } = "";
    public string Preview { get; set; } = "";
    public string CreatedAt { get; set; } = "";
    public string ModificationdAt { get; set; } = "";
}

public class KitShallow
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public string Version { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string Remote { get; set; } = "";
    public string Homepage { get; set; } = "";
    public string License { get; set; } = "";
    public string Preview { get; set; } = "";
    public string CreatedAt { get; set; } = "";
    public string ModificationdAt { get; set; } = "";
    public List<TypeMeta> Types { get; set; } = new();
    public List<DesignMeta> Designs { get; set; } = new();
    public List<TagMeta> Tags { get; set; } = new();
    public List<ConceptMeta> Concepts { get; set; } = new();
    public List<PortMeta> Ports { get; set; } = new();
    public List<QualityMeta> Qualities { get; set; } = new();
    public List<FileMeta> Files { get; set; } = new();
    public List<FolderMeta> Folders { get; set; } = new();
    public List<AuthorMeta> Authors { get; set; } = new();
    public List<AttributeMeta> Attributes { get; set; } = new();
}

#endregion 🏗️KitMetaShallow

#region 📎Meta And Shallow Conversions

public static class MetaShallowConversions
{
    public static AttributeMeta ToMeta(this Attribute a) => new()
    {
        Id = a.Id,
        Key = a.Key,
        Value = a.Value,
        Definition = a.Definition
    };

    public static PropMeta ToMeta(this Prop p) => new()
    {
        Id = p.Id,
        Quality = p.Quality,
        Value = p.Value,
        Unit = p.Unit
    };

    public static TagMeta ToMeta(this Tag t) => new()
    {
        Id = t.Id,
        Name = t.Name,
        Description = t.Description,
        Icon = t.Icon
    };

    public static ConceptMeta ToMeta(this Concept c) => new()
    {
        Id = c.Id,
        Name = c.Name,
        Description = c.Description,
        Icon = c.Icon
    };

    public static AuthorMeta ToMeta(this Author a) => new()
    {
        Id = a.Id,
        Name = a.Name,
        Email = a.Email
    };

    public static FileMeta ToMeta(this File f) => new()
    {
        Id = f.Id,
        Name = f.Name,
        Remote = f.Remote,
        Folder = f.Folder,
        Size = f.Size,
        Hash = f.Hash,
        CreatedAt = f.CreatedAt,
        CreatedBy = f.CreatedBy,
        ModificationdAt = f.ModificationdAt,
        ModificationdBy = f.ModificationdBy
    };

    public static FolderMeta ToMeta(this Folder f) => new()
    {
        Id = f.Id,
        Name = f.Name,
        Parent = f.Parent,
        Description = f.Description,
        CreatedAt = f.CreatedAt,
        CreatedBy = f.CreatedBy,
        ModificationdAt = f.ModificationdAt,
        ModificationdBy = f.ModificationdBy
    };

    public static QualityMeta ToMeta(this Quality q) => new()
    {
        Id = q.Id,
        Key = q.Key,
        Name = q.Name,
        Description = q.Description,
        Uri = q.Uri,
        Folder = q.Folder,
        Scalable = q.Scalable,
        Kind = q.Kind,
        SI = q.SI,
        Imperial = q.Imperial,
        Min = q.Min,
        MinExcluded = q.MinExcluded,
        Max = q.Max,
        MaxExcluded = q.MaxExcluded,
        Default = q.Default,
        Formula = q.Formula,
        Icon = q.Icon,
        Image = q.Image,
        Unit = q.Unit
    };

    public static PortMeta ToMeta(this Port p) => new()
    {
        Id = p.Id,
        Name = p.Name,
        Description = p.Description,
        Icon = p.Icon
    };

    public static RepresentationMeta ToMeta(this Representation m) => new()
    {
        Id = m.Id,
        Name = m.Name,
        File = m.File,
        Description = m.Description
    };

    public static ConnectorMeta ToMeta(this Connector c) => new()
    {
        Id = c.Id,
        Name = c.Name,
        T = c.T,
        Point = c.Point,
        Direction = c.Direction,
        Description = c.Description,
        Port = c.Port,
        Mandatory = c.Mandatory
    };

    public static LayerMeta ToMeta(this Layer l) => new()
    {
        Id = l.Id,
        Path = l.Path,
        IsHidden = l.IsHidden ?? false,
        IsLocked = l.IsLocked ?? false,
        Color = l.Color ?? "",
        Description = l.Description
    };

    public static PieceMeta ToMeta(this Piece p) => new()
    {
        Id = p.Id,
        Name = p.Name,
        Description = p.Description,
        Type = p.Type,
        Design = p.Design,
        Plane = p.Plane,
        Center = p.Center,
        Scale = p.Scale,
        MirrorPlane = p.MirrorPlane,
        IsHidden = p.IsHidden,
        IsLocked = p.IsLocked,
        Color = p.Color
    };

    public static GroupMeta ToMeta(this Group g) => new()
    {
        Id = g.Id,
        Name = g.Name,
        Description = g.Description,
        Color = g.Color
    };

    public static ConnectionMeta ToMeta(this Connection c) => new()
    {
        Id = c.Id,
        Parent = c.Parent,
        Child = c.Child,
        Description = c.Description,
        Gap = c.Gap,
        Shift = c.Shift,
        Rise = c.Rise,
        Rotation = c.Rotation,
        Turn = c.Turn,
        Tilt = c.Tilt,
        U = c.U,
        V = c.V
    };

    public static StatMeta ToMeta(this Stat s) => new()
    {
        Id = s.Id,
        Quality = s.Quality,
        Unit = s.Unit,
        Min = s.Min,
        MinExcluded = s.MinExcluded,
        Max = s.Max,
        MaxExcluded = s.MaxExcluded
    };

    public static TypeMeta ToMeta(this Type t) => new()
    {
        Id = t.Id,
        Name = t.Name,
        Parent = t.Parent,
        IsAbstract = t.IsAbstract,
        Folder = t.Folder,
        Description = t.Description,
        Icon = t.Icon,
        Image = t.Image,
        Stock = t.Stock,
        Virtual = t.Virtual,
        Uri = t.Uri,
        Location = t.Location,
        Unit = t.Unit,
        CreatedAt = t.CreatedAt,
        ModificationdAt = t.ModificationdAt
    };

    public static TypeShallow ToShallow(this Type t) => new()
    {
        Id = t.Id,
        Name = t.Name,
        Parent = t.Parent,
        IsAbstract = t.IsAbstract,
        Folder = t.Folder,
        Description = t.Description,
        Icon = t.Icon,
        Image = t.Image,
        Stock = t.Stock,
        Virtual = t.Virtual,
        Uri = t.Uri,
        Location = t.Location,
        Unit = t.Unit,
        CreatedAt = t.CreatedAt,
        ModificationdAt = t.ModificationdAt,
        Representations = t.Representations.Select(m => m.ToMeta()).ToList(),
        Connectors = t.Connectors.Select(c => c.ToMeta()).ToList(),
        Props = t.Props.Select(p => p.ToMeta()).ToList(),
        Authors = t.Authors,
        Concepts = t.Concepts,
        Attributes = t.Attributes.Select(a => a.ToMeta()).ToList()
    };

    public static DesignMeta ToMeta(this Design d) => new()
    {
        Id = d.Id,
        Name = d.Name,
        Parent = d.Parent,
        IsAbstract = d.IsAbstract,
        Folder = d.Folder,
        Description = d.Description,
        Icon = d.Icon,
        Image = d.Image,
        Location = d.Location,
        Unit = d.Unit,
        CanScale = d.CanScale,
        CanMirror = d.CanMirror,
        ActiveLayer = d.ActiveLayer,
        CreatedAt = d.CreatedAt,
        ModificationdAt = d.ModificationdAt
    };

    public static DesignShallow ToShallow(this Design d) => new()
    {
        Id = d.Id,
        Name = d.Name,
        Parent = d.Parent,
        IsAbstract = d.IsAbstract,
        Folder = d.Folder,
        Description = d.Description,
        Icon = d.Icon,
        Image = d.Image,
        Location = d.Location,
        Unit = d.Unit,
        CanScale = d.CanScale,
        CanMirror = d.CanMirror,
        ActiveLayer = d.ActiveLayer,
        CreatedAt = d.CreatedAt,
        ModificationdAt = d.ModificationdAt,
        Pieces = d.Pieces.Select(p => p.ToMeta()).ToList(),
        Connections = d.Connections.Select(c => c.ToMeta()).ToList(),
        Stats = d.Stats.Select(s => s.ToMeta()).ToList(),
        Props = d.Props.Select(p => p.ToMeta()).ToList(),
        Layers = d.Layers.Select(l => l.ToMeta()).ToList(),
        Groups = d.Groups.Select(g => g.ToMeta()).ToList(),
        Attributes = d.Attributes.Select(a => a.ToMeta()).ToList(),
        Authors = d.Authors,
        Concepts = d.Concepts
    };

    public static KitMeta ToMeta(this Kit k) => new()
    {
        Id = k.Id,
        Name = k.Name,
        Version = k.Version,
        Description = k.Description,
        Icon = k.Icon,
        Image = k.Image,
        Remote = k.Remote,
        Homepage = k.Homepage,
        License = k.License,
        Preview = k.Preview,
        CreatedAt = k.CreatedAt,
        ModificationdAt = k.ModificationdAt
    };

    public static KitShallow ToShallow(this Kit k) => new()
    {
        Id = k.Id,
        Name = k.Name,
        Version = k.Version,
        Description = k.Description,
        Icon = k.Icon,
        Image = k.Image,
        Remote = k.Remote,
        Homepage = k.Homepage,
        License = k.License,
        Preview = k.Preview,
        CreatedAt = k.CreatedAt,
        ModificationdAt = k.ModificationdAt,
        Types = k.Types.Select(t => t.ToMeta()).ToList(),
        Designs = k.Designs.Select(d => d.ToMeta()).ToList(),
        Tags = k.Tags.Select(t => t.ToMeta()).ToList(),
        Concepts = k.Concepts.Select(c => c.ToMeta()).ToList(),
        Ports = k.Ports.Select(p => p.ToMeta()).ToList(),
        Qualities = k.Qualities.Select(q => q.ToMeta()).ToList(),
        Files = k.Files.Select(f => f.ToMeta()).ToList(),
        Folders = k.Folders.Select(f => f.ToMeta()).ToList(),
        Authors = k.Authors.Select(a => a.ToMeta()).ToList(),
        Attributes = k.Attributes.Select(a => a.ToMeta()).ToList()
    };
}

#endregion 📎Meta And Shallow Conversions

#endregion 🔑Meta And Shallow




#region 🖥️Hash
// Merkle hash functions for all entities. Each hash function computes a deterministic
// SHA-256 hex digest. Collections are hashed by sorting child hashes alphabetically.
// Field order is alphabetical by JSON field name. Missing/null/empty fields are skipped.

public static class Hashing
{
    private class HashWriter
    {
        private readonly List<byte> _buf = new();

        public void WriteString(string s)
        {
            var bytes = Encoding.UTF8.GetBytes(s);
            var lb = BitConverter.GetBytes((uint)bytes.Length);
            if (BitConverter.IsLittleEndian) Array.Reverse(lb);
            _buf.AddRange(lb);
            _buf.AddRange(bytes);
        }

        public void WriteNumber(double n) => WriteString(FormatNumberForHash(n));

        public void WriteIntNumber(int n) => WriteString(n.ToString());

        public void WriteBool(bool b) => _buf.Add(b ? (byte)1 : (byte)0);

        public void WriteHash(string h) => WriteString(h);

        public void WriteHashList(List<string> hashes)
        {
            var sorted = hashes.OrderBy(h => h, StringComparer.Ordinal).ToList();
            var lb = BitConverter.GetBytes((uint)sorted.Count);
            if (BitConverter.IsLittleEndian) Array.Reverse(lb);
            _buf.AddRange(lb);
            foreach (var h in sorted)
                WriteString(h);
        }

        public void WriteIdList(List<string> ids)
        {
            var sorted = ids.OrderBy(g => g, StringComparer.Ordinal).ToList();
            var lb = BitConverter.GetBytes((uint)sorted.Count);
            if (BitConverter.IsLittleEndian) Array.Reverse(lb);
            _buf.AddRange(lb);
            foreach (var g in sorted)
                WriteString(g);
        }

        public string Digest()
        {
            using var sha256 = System.Security.Cryptography.SHA256.Create();
            var hash = sha256.ComputeHash(_buf.ToArray());
            return BitConverter.ToString(hash).Replace("-", "").ToLowerInvariant();
        }
    }

    public static string FormatNumberForHash(double n)
    {
        if (n == Math.Truncate(n) && !double.IsInfinity(n) && Math.Abs(n) < 1e15)
            return ((long)n).ToString(CultureInfo.InvariantCulture);
        var abs = Math.Abs(n);
        var roundTrip = n.ToString("R", CultureInfo.InvariantCulture);
        if (abs > 0 && (abs < 1e-6 || abs >= 1e21))
        {
            return NormalizeScientificNumber(roundTrip);
        }
        return roundTrip.IndexOfAny(new[] { 'E', 'e' }) >= 0
            ? ExpandScientificNumber(roundTrip)
            : TrimFractionZeros(roundTrip);
    }

    private static string TrimFractionZeros(string value)
    {
        if (!value.Contains('.'))
            return value == "-0" ? "0" : value;

        var trimmed = value.TrimEnd('0').TrimEnd('.');
        return trimmed == "-0" ? "0" : trimmed;
    }

    private static string NormalizeScientificNumber(string value)
    {
        var exponentIndex = value.IndexOfAny(new[] { 'E', 'e' });
        if (exponentIndex < 0)
            return value;

        var mantissa = TrimFractionZeros(value[..exponentIndex]);
        var exponent = value[(exponentIndex + 1)..];
        var sign = exponent.StartsWith("-", StringComparison.Ordinal) ? "-" : "+";
        var digits = exponent.TrimStart('+', '-').TrimStart('0');
        if (digits == "")
            digits = "0";

        return $"{mantissa}e{sign}{digits}";
    }

    private static string ExpandScientificNumber(string value)
    {
        var exponentIndex = value.IndexOfAny(new[] { 'E', 'e' });
        if (exponentIndex < 0)
            return TrimFractionZeros(value);

        var mantissa = value[..exponentIndex];
        var prefix = "";
        if (mantissa.StartsWith("-", StringComparison.Ordinal))
        {
            prefix = "-";
            mantissa = mantissa[1..];
        }
        else if (mantissa.StartsWith("+", StringComparison.Ordinal))
        {
            mantissa = mantissa[1..];
        }

        var exponent = int.Parse(value[(exponentIndex + 1)..], CultureInfo.InvariantCulture);
        var decimalIndex = mantissa.IndexOf('.');
        var digits = mantissa.Replace(".", "");
        if (decimalIndex < 0)
            decimalIndex = digits.Length;

        var pointIndex = decimalIndex + exponent;
        string expanded;
        if (pointIndex <= 0)
        {
            expanded = "0." + new string('0', -pointIndex) + digits;
        }
        else if (pointIndex >= digits.Length)
        {
            expanded = digits + new string('0', pointIndex - digits.Length);
        }
        else
        {
            expanded = digits.Insert(pointIndex, ".");
        }

        expanded = TrimFractionZeros(expanded);
        if (expanded.StartsWith(".", StringComparison.Ordinal))
            expanded = "0" + expanded;
        if (expanded == "0")
            return expanded;

        return prefix + expanded;
    }

    public static string HashCoordinate(Coordinate c)
    {
        var w = new HashWriter();
        w.WriteString("Coordinate");
        w.WriteString("u");
        w.WriteNumber(c.U);
        w.WriteString("v");
        w.WriteNumber(c.V);
        return w.Digest();
    }

    public static string HashPoint(Point p)
    {
        var w = new HashWriter();
        w.WriteString("Point");
        w.WriteString("x");
        w.WriteNumber(p.X);
        w.WriteString("y");
        w.WriteNumber(p.Y);
        w.WriteString("z");
        w.WriteNumber(p.Z);
        return w.Digest();
    }

    public static string HashVector(Vector v)
    {
        var w = new HashWriter();
        w.WriteString("Vector");
        w.WriteString("x");
        w.WriteNumber(v.X);
        w.WriteString("y");
        w.WriteNumber(v.Y);
        w.WriteString("z");
        w.WriteNumber(v.Z);
        return w.Digest();
    }

    public static string HashPlane(Plane p)
    {
        var w = new HashWriter();
        w.WriteString("Plane");
        w.WriteString("origin");
        w.WriteHash(HashPoint(p.Origin));
        w.WriteString("xAxis");
        w.WriteHash(HashVector(p.XAxis));
        w.WriteString("yAxis");
        w.WriteHash(HashVector(p.YAxis));
        return w.Digest();
    }

    public static string HashAttribute(Attribute a)
    {
        var w = new HashWriter();
        w.WriteString("Attribute");
        if (a.Definition != null)
        {
            w.WriteString("definition");
            w.WriteString(a.Definition);
        }
        w.WriteString("id");
        w.WriteString(a.Id);
        w.WriteString("key");
        w.WriteString(a.Key);
        if (a.Value != null)
        {
            w.WriteString("value");
            w.WriteString(a.Value);
        }
        return w.Digest();
    }

    public static string HashLocation(Location l)
    {
        var w = new HashWriter();
        w.WriteString("Location");
        if (l.Altitude.HasValue)
        {
            w.WriteString("altitude");
            w.WriteNumber(l.Altitude.Value);
        }
        if (l.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(l.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("id");
        w.WriteString(l.Id);
        w.WriteString("latitude");
        w.WriteNumber(l.Latitude);
        w.WriteString("longitude");
        w.WriteNumber(l.Longitude);
        return w.Digest();
    }

    public static string HashAuthor(Author a)
    {
        var w = new HashWriter();
        w.WriteString("Author");
        if (a.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(a.Attributes.Select(HashAttribute).ToList());
        }
        if (!string.IsNullOrEmpty(a.Email))
        {
            w.WriteString("email");
            w.WriteString(a.Email);
        }
        w.WriteString("id");
        w.WriteString(a.Id);
        w.WriteString("name");
        w.WriteString(a.Name);
        return w.Digest();
    }

    public static string HashFile(File f)
    {
        var w = new HashWriter();
        w.WriteString("File");
        if (f.Blob != null)
        {
            w.WriteString("blob");
            w.WriteString(f.Blob);
        }
        if (f.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(f.Folder.Id);
        }
        w.WriteString("id");
        w.WriteString(f.Id);
        if (f.Hash != null)
        {
            w.WriteString("hash");
            w.WriteString(f.Hash);
        }
        w.WriteString("name");
        w.WriteString(f.Name);
        if (f.Remote != null)
        {
            w.WriteString("remote");
            w.WriteString(f.Remote);
        }
        if (f.Size.HasValue)
        {
            w.WriteString("size");
            w.WriteIntNumber(f.Size.Value);
        }
        return w.Digest();
    }

    public static string HashFolder(Folder f)
    {
        var w = new HashWriter();
        w.WriteString("Folder");
        if (f.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(f.Attributes.Select(HashAttribute).ToList());
        }
        if (f.Description != null)
        {
            w.WriteString("description");
            w.WriteString(f.Description);
        }
        w.WriteString("id");
        w.WriteString(f.Id);
        w.WriteString("name");
        w.WriteString(f.Name);
        if (f.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(f.Parent.Id);
        }
        return w.Digest();
    }

    public static string HashBenchmark(Benchmark b)
    {
        var w = new HashWriter();
        w.WriteString("Benchmark");
        if (b.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(b.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("id");
        w.WriteString(b.Id);
        if (b.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(b.Icon);
        }
        if (b.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(b.Max.Value);
        }
        if (b.MaxExcluded.HasValue)
        {
            w.WriteString("maxExcluded");
            w.WriteBool(b.MaxExcluded.Value);
        }
        if (b.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(b.Min.Value);
        }
        if (b.MinExcluded.HasValue)
        {
            w.WriteString("minExcluded");
            w.WriteBool(b.MinExcluded.Value);
        }
        w.WriteString("name");
        w.WriteString(b.Name);
        return w.Digest();
    }

    public static string HashQuality(Quality q)
    {
        var w = new HashWriter();
        w.WriteString("Quality");
        if (q.Benchmarks?.Count > 0)
        {
            w.WriteString("benchmarks");
            w.WriteHashList(q.Benchmarks.Select(HashBenchmark).ToList());
        }
        if (q.Scalable.HasValue)
        {
            w.WriteString("canScale");
            w.WriteBool(q.Scalable.Value);
        }
        if (!string.IsNullOrEmpty(q.Imperial))
        {
            w.WriteString("defaultImperialUnit");
            w.WriteString(q.Imperial);
        }
        if (!string.IsNullOrEmpty(q.SI))
        {
            w.WriteString("defaultSiUnit");
            w.WriteString(q.SI);
        }
        if (q.Default.HasValue)
        {
            w.WriteString("defaultValue");
            w.WriteNumber(q.Default.Value);
        }
        if (q.Description != null)
        {
            w.WriteString("description");
            w.WriteString(q.Description);
        }
        if (!string.IsNullOrEmpty(q.Formula))
        {
            w.WriteString("formula");
            w.WriteString(q.Formula);
        }
        w.WriteString("id");
        w.WriteString(q.Id);
        if (q.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(q.Icon);
        }
        if (q.Image != null)
        {
            w.WriteString("image");
            w.WriteString(q.Image);
        }
        if (q.MaxExcluded.HasValue)
        {
            w.WriteString("isMaxExcluded");
            w.WriteBool(q.MaxExcluded.Value);
        }
        if (q.MinExcluded.HasValue)
        {
            w.WriteString("isMinExcluded");
            w.WriteBool(q.MinExcluded.Value);
        }
        w.WriteString("key");
        w.WriteString(q.Key);
        if (q.Kind != QualityKind.General)
        {
            w.WriteString("kind");
            w.WriteIntNumber((int)q.Kind);
        }
        if (q.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(q.Max.Value);
        }
        if (q.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(q.Min.Value);
        }
        w.WriteString("name");
        w.WriteString(q.Name);
        if (q.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(q.Unit);
        }
        if (!string.IsNullOrEmpty(q.Uri))
        {
            w.WriteString("uri");
            w.WriteString(q.Uri);
        }
        return w.Digest();
    }

    public static string HashPort(Port p)
    {
        var w = new HashWriter();
        w.WriteString("Port");
        if (p.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(p.Attributes.Select(HashAttribute).ToList());
        }
        if (p.CompatiblePorts?.Count > 0)
        {
            w.WriteString("compatiblePorts");
            w.WriteIdList(p.CompatiblePorts.Select(cp => cp.Id).ToList());
        }
        if (p.Description != null)
        {
            w.WriteString("description");
            w.WriteString(p.Description);
        }
        w.WriteString("id");
        w.WriteString(p.Id);
        if (p.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(p.Icon);
        }
        w.WriteString("name");
        w.WriteString(p.Name);
        return w.Digest();
    }

    public static string HashProp(Prop p)
    {
        var w = new HashWriter();
        w.WriteString("Prop");
        if (p.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(p.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("id");
        w.WriteString(p.Id);
        w.WriteString("quality");
        w.WriteString(p.Quality.Id);
        if (p.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(p.Unit);
        }
        w.WriteString("value");
        w.WriteString(p.Value);
        return w.Digest();
    }

    public static string HashTag(Tag t)
    {
        var w = new HashWriter();
        w.WriteString("Tag");
        if (t.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(t.Attributes.Select(HashAttribute).ToList());
        }
        if (t.Description != null)
        {
            w.WriteString("description");
            w.WriteString(t.Description);
        }
        w.WriteString("id");
        w.WriteString(t.Id);
        if (t.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(t.Icon);
        }
        w.WriteString("name");
        w.WriteString(t.Name);
        return w.Digest();
    }

    public static string HashConcept(Concept c)
    {
        var w = new HashWriter();
        w.WriteString("Concept");
        if (c.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(c.Attributes.Select(HashAttribute).ToList());
        }
        if (c.Description != null)
        {
            w.WriteString("description");
            w.WriteString(c.Description);
        }
        w.WriteString("id");
        w.WriteString(c.Id);
        if (c.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(c.Icon);
        }
        w.WriteString("name");
        w.WriteString(c.Name);
        return w.Digest();
    }

    public static string HashRepresentation(Representation m)
    {
        var w = new HashWriter();
        w.WriteString("Representation");
        if (m.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(m.Attributes.Select(HashAttribute).ToList());
        }
        if (m.Description != null)
        {
            w.WriteString("description");
            w.WriteString(m.Description);
        }
        w.WriteString("file");
        w.WriteString(m.File.Id);
        w.WriteString("id");
        w.WriteString(m.Id);
        if (m.Name != null)
        {
            w.WriteString("name");
            w.WriteString(m.Name);
        }
        if (m.Tags?.Count > 0)
        {
            w.WriteString("tags");
            w.WriteIdList(m.Tags.Select(t => t.Id).ToList());
        }
        return w.Digest();
    }

    public static string HashConnector(Connector c)
    {
        var w = new HashWriter();
        w.WriteString("Connector");
        if (c.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(c.Attributes.Select(HashAttribute).ToList());
        }
        if (c.Description != null)
        {
            w.WriteString("description");
            w.WriteString(c.Description);
        }
        w.WriteString("direction");
        w.WriteHash(HashVector(c.Direction ?? new Vector()));
        w.WriteString("id");
        w.WriteString(c.Id);
        if (c.Mandatory.HasValue)
        {
            w.WriteString("mandatory");
            w.WriteBool(c.Mandatory.Value);
        }
        if (!string.IsNullOrEmpty(c.Name))
        {
            w.WriteString("name");
            w.WriteString(c.Name);
        }
        w.WriteString("point");
        w.WriteHash(HashPoint(c.Point ?? new Point()));
        if (c.Port != null)
        {
            w.WriteString("port");
            w.WriteString(c.Port.Id);
        }
        if (c.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(c.Props.Select(HashProp).ToList());
        }
        w.WriteString("t");
        w.WriteNumber(c.T);
        return w.Digest();
    }

    public static string HashType(Type t)
    {
        var w = new HashWriter();
        w.WriteString("Type");
        if (t.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(t.Attributes.Select(HashAttribute).ToList());
        }
        if (t.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteIdList(t.Authors.Select(a => a.Id).ToList());
        }
        if (t.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteIdList(t.Concepts.Select(c => c.Id).ToList());
        }
        if (t.Connectors?.Count > 0)
        {
            w.WriteString("connectors");
            w.WriteHashList(t.Connectors.Select(HashConnector).ToList());
        }
        if (t.Description != null)
        {
            w.WriteString("description");
            w.WriteString(t.Description);
        }
        if (t.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(t.Folder);
        }
        w.WriteString("id");
        w.WriteString(t.Id);
        if (t.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(t.Icon);
        }
        if (t.Image != null)
        {
            w.WriteString("image");
            w.WriteString(t.Image);
        }
        if (t.IsAbstract.HasValue)
        {
            w.WriteString("isAbstract");
            w.WriteBool(t.IsAbstract.Value);
        }
        if (t.Location != null)
        {
            w.WriteString("location");
            w.WriteString(t.Location.Id);
        }
        if (t.Representations?.Count > 0)
        {
            w.WriteString("representations");
            w.WriteHashList(t.Representations.Select(HashRepresentation).ToList());
        }
        w.WriteString("name");
        w.WriteString(t.Name);
        if (t.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(t.Parent.Id);
        }
        if (t.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(t.Props.Select(HashProp).ToList());
        }
        if (t.Stock.HasValue)
        {
            w.WriteString("stock");
            w.WriteIntNumber(t.Stock.Value);
        }
        if (t.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(t.Unit);
        }
        if (t.Virtual.HasValue)
        {
            w.WriteString("virtual");
            w.WriteBool(t.Virtual.Value);
        }
        return w.Digest();
    }

    public static string HashLayer(Layer l)
    {
        var w = new HashWriter();
        w.WriteString("Layer");
        if (l.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(l.Attributes.Select(HashAttribute).ToList());
        }
        if (l.Color != null)
        {
            w.WriteString("color");
            w.WriteString(l.Color);
        }
        if (l.Description != null)
        {
            w.WriteString("description");
            w.WriteString(l.Description);
        }
        w.WriteString("id");
        w.WriteString(l.Id);
        if (l.IsHidden.HasValue)
        {
            w.WriteString("isHidden");
            w.WriteBool(l.IsHidden.Value);
        }
        if (l.IsLocked.HasValue)
        {
            w.WriteString("isLocked");
            w.WriteBool(l.IsLocked.Value);
        }
        w.WriteString("path");
        w.WriteString(l.Path);
        return w.Digest();
    }

    public static string HashStat(Stat s)
    {
        var w = new HashWriter();
        w.WriteString("Stat");
        w.WriteString("id");
        w.WriteString(s.Id);
        if (s.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(s.Max.Value);
        }
        if (s.MaxExcluded.HasValue)
        {
            w.WriteString("maxExcluded");
            w.WriteBool(s.MaxExcluded.Value);
        }
        if (s.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(s.Min.Value);
        }
        if (s.MinExcluded.HasValue)
        {
            w.WriteString("minExcluded");
            w.WriteBool(s.MinExcluded.Value);
        }
        w.WriteString("quality");
        w.WriteString(s.Quality.Id);
        if (s.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(s.Unit);
        }
        return w.Digest();
    }

    public static string HashGroup(Group g)
    {
        var w = new HashWriter();
        w.WriteString("Group");
        if (g.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(g.Attributes.Select(HashAttribute).ToList());
        }
        if (g.Color != null)
        {
            w.WriteString("color");
            w.WriteString(g.Color);
        }
        if (g.Description != null)
        {
            w.WriteString("description");
            w.WriteString(g.Description);
        }
        w.WriteString("id");
        w.WriteString(g.Id);
        if (g.Name != null)
        {
            w.WriteString("name");
            w.WriteString(g.Name);
        }
        w.WriteString("pieces");
        w.WriteIdList(g.Pieces?.Select(p => p.Id).ToList() ?? new List<string>());
        return w.Digest();
    }

    public static string HashSide(Side s)
    {
        var w = new HashWriter();
        w.WriteString("Side");
        if (s.Connector != null)
        {
            w.WriteString("connector");
            w.WriteString(s.Connector.Id);
        }
        if (s.DesignPiece != null)
        {
            w.WriteString("designPiece");
            w.WriteString(s.DesignPiece.Id);
        }
        w.WriteString("piece");
        w.WriteString(s.Piece.Id);
        return w.Digest();
    }

    public static string HashConnection(Connection c)
    {
        var w = new HashWriter();
        w.WriteString("Connection");
        if (c.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(c.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("parent");
        w.WriteHash(HashSide(c.Parent));
        w.WriteString("child");
        w.WriteHash(HashSide(c.Child));
        if (c.Description != null)
        {
            w.WriteString("description");
            w.WriteString(c.Description);
        }
        w.WriteString("gap");
        w.WriteNumber(c.Gap);
        w.WriteString("id");
        w.WriteString(c.Id);
        w.WriteString("rise");
        w.WriteNumber(c.Rise);
        w.WriteString("rotation");
        w.WriteNumber(c.Rotation);
        w.WriteString("shift");
        w.WriteNumber(c.Shift);
        w.WriteString("tilt");
        w.WriteNumber(c.Tilt);
        w.WriteString("turn");
        w.WriteNumber(c.Turn);
        w.WriteString("u");
        w.WriteNumber(c.U ?? 0);
        w.WriteString("v");
        w.WriteNumber(c.V ?? 0);
        return w.Digest();
    }

    public static string HashPiece(Piece p)
    {
        var w = new HashWriter();
        w.WriteString("Piece");
        if (p.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(p.Attributes.Select(HashAttribute).ToList());
        }
        if (p.Center != null)
        {
            w.WriteString("center");
            w.WriteHash(HashCoordinate(p.Center));
        }
        if (p.Color != null)
        {
            w.WriteString("color");
            w.WriteString(p.Color);
        }
        if (p.Description != null)
        {
            w.WriteString("description");
            w.WriteString(p.Description);
        }
        if (p.Design != null)
        {
            w.WriteString("design");
            w.WriteString(p.Design.Id);
        }
        w.WriteString("id");
        w.WriteString(p.Id);
        if (p.IsHidden.HasValue)
        {
            w.WriteString("isHidden");
            w.WriteBool(p.IsHidden.Value);
        }
        if (p.IsLocked.HasValue)
        {
            w.WriteString("isLocked");
            w.WriteBool(p.IsLocked.Value);
        }
        if (p.MirrorPlane != null)
        {
            w.WriteString("mirrorPlane");
            w.WriteHash(HashPlane(p.MirrorPlane));
        }
        if (p.Name != null)
        {
            w.WriteString("name");
            w.WriteString(p.Name);
        }
        if (p.Plane != null)
        {
            w.WriteString("plane");
            w.WriteHash(HashPlane(p.Plane));
        }
        if (p.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(p.Props.Select(HashProp).ToList());
        }
        if (p.Scale.HasValue)
        {
            w.WriteString("scale");
            w.WriteNumber(p.Scale.Value);
        }
        if (p.Type != null)
        {
            w.WriteString("type");
            w.WriteString(p.Type.Id);
        }
        return w.Digest();
    }

    public static string HashDesign(Design d)
    {
        var w = new HashWriter();
        w.WriteString("Design");
        if (d.ActiveLayer != null)
        {
            w.WriteString("activeLayer");
            w.WriteString(d.ActiveLayer.Id);
        }
        if (d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        if (d.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteIdList(d.Authors.Select(a => a.Id).ToList());
        }
        if (d.CanMirror.HasValue)
        {
            w.WriteString("canMirror");
            w.WriteBool(d.CanMirror.Value);
        }
        if (d.CanScale.HasValue)
        {
            w.WriteString("canScale");
            w.WriteBool(d.CanScale.Value);
        }
        if (d.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteIdList(d.Concepts.Select(c => c.Id).ToList());
        }
        if (d.Connections?.Count > 0)
        {
            w.WriteString("connections");
            w.WriteHashList(d.Connections.Select(HashConnection).ToList());
        }
        if (d.Description != null)
        {
            w.WriteString("description");
            w.WriteString(d.Description);
        }
        if (d.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(d.Folder);
        }
        if (d.Groups?.Count > 0)
        {
            w.WriteString("groups");
            w.WriteHashList(d.Groups.Select(HashGroup).ToList());
        }
        w.WriteString("id");
        w.WriteString(d.Id);
        if (d.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(d.Icon);
        }
        if (d.Image != null)
        {
            w.WriteString("image");
            w.WriteString(d.Image);
        }
        if (d.IsAbstract.HasValue)
        {
            w.WriteString("isAbstract");
            w.WriteBool(d.IsAbstract.Value);
        }
        if (d.Layers?.Count > 0)
        {
            w.WriteString("layers");
            w.WriteHashList(d.Layers.Select(HashLayer).ToList());
        }
        if (d.Location != null)
        {
            w.WriteString("location");
            w.WriteString(d.Location.Id);
        }
        w.WriteString("name");
        w.WriteString(d.Name);
        if (d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Id);
        }
        if (d.Pieces?.Count > 0)
        {
            w.WriteString("pieces");
            w.WriteHashList(d.Pieces.Select(HashPiece).ToList());
        }
        if (d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        if (d.Stats?.Count > 0)
        {
            w.WriteString("stats");
            w.WriteHashList(d.Stats.Select(HashStat).ToList());
        }
        if (d.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(d.Unit);
        }
        return w.Digest();
    }

    public static string HashTypology(Typology t)
    {
        var w = new HashWriter();
        w.WriteString("Typology");
        if (t.Description != null)
        {
            w.WriteString("description");
            w.WriteString(t.Description);
        }
        if (t.Designs?.Count > 0)
        {
            w.WriteString("designs");
            w.WriteHashList(t.Designs.Select(HashDesign).ToList());
        }
        if (t.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(t.Folder);
        }
        if (t.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(t.Icon);
        }
        w.WriteString("id");
        w.WriteString(t.Id);
        w.WriteString("name");
        w.WriteString(t.Name);
        if (t.Types?.Count > 0)
        {
            w.WriteString("types");
            w.WriteHashList(t.Types.Select(HashType).ToList());
        }
        return w.Digest();
    }

    public static string HashKit(Kit k)
    {
        var w = new HashWriter();
        w.WriteString("Kit");
        if (k.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(k.Attributes.Select(HashAttribute).ToList());
        }
        if (k.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteHashList(k.Authors.Select(HashAuthor).ToList());
        }
        if (k.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteHashList(k.Concepts.Select(HashConcept).ToList());
        }
        if (k.Description != null)
        {
            w.WriteString("description");
            w.WriteString(k.Description);
        }
        if (k.Designs?.Count > 0)
        {
            w.WriteString("designs");
            w.WriteHashList(k.Designs.Select(HashDesign).ToList());
        }
        if (k.Files?.Count > 0)
        {
            w.WriteString("files");
            w.WriteHashList(k.Files.Select(HashFile).ToList());
        }
        if (k.Folders?.Count > 0)
        {
            w.WriteString("folders");
            w.WriteHashList(k.Folders.Select(HashFolder).ToList());
        }
        w.WriteString("id");
        w.WriteString(k.Id);
        if (k.Homepage != null)
        {
            w.WriteString("homepage");
            w.WriteString(k.Homepage);
        }
        if (k.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(k.Icon);
        }
        if (k.Image != null)
        {
            w.WriteString("image");
            w.WriteString(k.Image);
        }
        if (k.License != null)
        {
            w.WriteString("license");
            w.WriteString(k.License);
        }
        w.WriteString("name");
        w.WriteString(k.Name);
        if (k.Ports?.Count > 0)
        {
            w.WriteString("ports");
            w.WriteHashList(k.Ports.Select(HashPort).ToList());
        }
        if (k.Preview != null)
        {
            w.WriteString("preview");
            w.WriteString(k.Preview);
        }
        if (k.Qualities?.Count > 0)
        {
            w.WriteString("qualities");
            w.WriteHashList(k.Qualities.Select(HashQuality).ToList());
        }
        if (k.Remote != null)
        {
            w.WriteString("remote");
            w.WriteString(k.Remote);
        }
        if (k.Tags?.Count > 0)
        {
            w.WriteString("tags");
            w.WriteHashList(k.Tags.Select(HashTag).ToList());
        }
        if (k.Types?.Count > 0)
        {
            w.WriteString("types");
            w.WriteHashList(k.Types.Select(HashType).ToList());
        }
        if (k.Version != null && k.Version != "")
        {
            w.WriteString("version");
            w.WriteString(k.Version);
        }
        return w.Digest();
    }

    // #region 🔗Hash Diffs
    // Deterministic SHA-256 Merkle hash functions for all diff types.
    // Fields are ordered alphabetically by JSON field name. Id is excluded from
    // diff hashes. Null markers (field name + WriteBool(false)) indicate explicit
    // null. Absent/unset fields are skipped entirely.

    private static void WriteDiffNullableString(HashWriter w, string key, string? value, bool shouldSerialize)
    {
        if (value != null) { w.WriteString(key); w.WriteString(value); }
        else if (shouldSerialize) { w.WriteString(key); w.WriteBool(false); }
    }

    private static void WriteDiffOptString(HashWriter w, string key, string? value, bool shouldSerialize)
    {
        if (shouldSerialize && value != null) { w.WriteString(key); w.WriteString(value); }
    }

    private static void WriteDiffOptNumber(HashWriter w, string key, double? value, bool shouldSerialize)
    {
        if (shouldSerialize && value.HasValue) { w.WriteString(key); w.WriteNumber(value.Value); }
    }

    private static void WriteDiffOptIntNumber(HashWriter w, string key, int? value, bool shouldSerialize)
    {
        if (shouldSerialize && value.HasValue) { w.WriteString(key); w.WriteIntNumber(value.Value); }
    }

    private static void WriteDiffOptBool(HashWriter w, string key, bool? value, bool shouldSerialize)
    {
        if (shouldSerialize && value.HasValue) { w.WriteString(key); w.WriteBool(value.Value); }
    }

    private static string HashCollectionDiffGeneric<TEntity, TDiff>(
        string tag, string updateTag, string entityKeyName,
        Func<TEntity, string> hashEntityFn,
        Func<TDiff, string> hashDiffFn,
        List<string> removedIds,
        List<(string key, TDiff diff)> updates,
        List<TEntity> added)
    {
        var w = new HashWriter();
        w.WriteString(tag);
        if (added?.Count > 0)
        {
            w.WriteString("added");
            w.WriteHashList(added.Select(hashEntityFn).ToList());
        }
        if (removedIds?.Count > 0)
        {
            w.WriteString("removed");
            w.WriteIdList(removedIds);
        }
        if (updates?.Count > 0)
        {
            w.WriteString("updated");
            var keys = new List<string> { entityKeyName, "diff" };
            keys.Sort(StringComparer.Ordinal);
            var updateHashes = updates.Select(u =>
            {
                var uw = new HashWriter();
                uw.WriteString(updateTag);
                foreach (var k in keys)
                {
                    if (k == "diff")
                    {
                        uw.WriteString("diff");
                        uw.WriteHash(hashDiffFn(u.diff));
                    }
                    else
                    {
                        uw.WriteString(k);
                        uw.WriteString(u.key);
                    }
                }
                return uw.Digest();
            }).ToList();
            w.WriteHashList(updateHashes);
        }
        return w.Digest();
    }

    // #region 🐹Hash Diff Value Types

    public static string HashCoordinateDiff(Coordinate c)
    {
        var w = new HashWriter();
        w.WriteString("CoordinateDiff");
        w.WriteString("u");
        w.WriteNumber(c.U);
        w.WriteString("v");
        w.WriteNumber(c.V);
        return w.Digest();
    }

    public static string HashPointDiff(Point p)
    {
        var w = new HashWriter();
        w.WriteString("PointDiff");
        w.WriteString("x");
        w.WriteNumber(p.X);
        w.WriteString("y");
        w.WriteNumber(p.Y);
        w.WriteString("z");
        w.WriteNumber(p.Z);
        return w.Digest();
    }

    public static string HashVectorDiff(Vector v)
    {
        var w = new HashWriter();
        w.WriteString("VectorDiff");
        w.WriteString("x");
        w.WriteNumber(v.X);
        w.WriteString("y");
        w.WriteNumber(v.Y);
        w.WriteString("z");
        w.WriteNumber(v.Z);
        return w.Digest();
    }

    public static string HashPlaneDiff(Plane p)
    {
        var w = new HashWriter();
        w.WriteString("PlaneDiff");
        w.WriteString("origin");
        w.WriteHash(HashPointDiff(p.Origin));
        w.WriteString("xAxis");
        w.WriteHash(HashVectorDiff(p.XAxis));
        w.WriteString("yAxis");
        w.WriteHash(HashVectorDiff(p.YAxis));
        return w.Digest();
    }

    // #endregion 🐹Hash Diff Value Types

    // #region ⚗️Hash Diff Entities

    public static string HashAttributeDiff(AttributeDiff d)
    {
        var w = new HashWriter();
        w.WriteString("AttributeDiff");
        WriteDiffOptString(w, "definition", d.Definition, d.ShouldSerializeDefinition());
        WriteDiffOptString(w, "key", d.Key, d.ShouldSerializeKey());
        WriteDiffOptString(w, "value", d.Value, d.ShouldSerializeValue());
        return w.Digest();
    }

    public static string HashAttributesDiff(AttributesDiff d)
    {
        return HashCollectionDiffGeneric(
            "AttributesDiff", "AttributeModification", "attribute",
            (Attribute a) => HashAttribute(a),
            (AttributeDiff ad) => HashAttributeDiff(ad),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Attribute.Id, u.Diff!)).ToList() ?? new List<(string, AttributeDiff)>(),
            d.Added ?? new List<Attribute>());
    }

    public static string HashAuthorDiff(AuthorDiff d)
    {
        var w = new HashWriter();
        w.WriteString("AuthorDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        WriteDiffOptString(w, "email", d.Email, d.ShouldSerializeEmail());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashAuthorsDiff(AuthorsDiff d)
    {
        return HashCollectionDiffGeneric(
            "AuthorsDiff", "AuthorModification", "author",
            (Author a) => HashAuthor(a),
            (AuthorDiff ad) => HashAuthorDiff(ad),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Author.Id, u.Diff!)).ToList() ?? new List<(string, AuthorDiff)>(),
            d.Added ?? new List<Author>());
    }

    public static string HashFileDiff(FileDiff d)
    {
        var w = new HashWriter();
        w.WriteString("FileDiff");
        WriteDiffOptString(w, "blob", d.Blob, d.ShouldSerializeBlob());
        if (d.ShouldSerializeFolder() && d.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(d.Folder.Id);
        }
        WriteDiffOptString(w, "hash", d.Hash, d.ShouldSerializeHash());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        WriteDiffOptString(w, "remote", d.Remote, d.ShouldSerializeRemote());
        WriteDiffOptIntNumber(w, "size", d.Size, d.ShouldSerializeSize());
        return w.Digest();
    }

    public static string HashFilesDiff(FilesDiff d)
    {
        return HashCollectionDiffGeneric(
            "FilesDiff", "FileModification", "file",
            (File f) => HashFile(f),
            (FileDiff fd) => HashFileDiff(fd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.File.Id, u.Diff!)).ToList() ?? new List<(string, FileDiff)>(),
            d.Added ?? new List<File>());
    }

    public static string HashFolderDiff(FolderDiff d)
    {
        var w = new HashWriter();
        w.WriteString("FolderDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeParent() && d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Id);
        }
        return w.Digest();
    }

    public static string HashFoldersDiff(FoldersDiff d)
    {
        return HashCollectionDiffGeneric(
            "FoldersDiff", "FolderModification", "folder",
            (Folder f) => HashFolder(f),
            (FolderDiff fd) => HashFolderDiff(fd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Folder.Id, u.Diff!)).ToList() ?? new List<(string, FolderDiff)>(),
            d.Added ?? new List<Folder>());
    }

    public static string HashBenchmarkDiff(BenchmarkDiff d)
    {
        var w = new HashWriter();
        w.WriteString("BenchmarkDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptNumber(w, "max", d.Max, d.ShouldSerializeMax());
        WriteDiffOptBool(w, "maxExcluded", d.MaxExcluded, d.ShouldSerializeMaxExcluded());
        WriteDiffOptNumber(w, "min", d.Min, d.ShouldSerializeMin());
        WriteDiffOptBool(w, "minExcluded", d.MinExcluded, d.ShouldSerializeMinExcluded());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashQualityDiff(QualityDiff d)
    {
        var w = new HashWriter();
        w.WriteString("QualityDiff");
        if (d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        if (d.Benchmarks?.Count > 0)
        {
            w.WriteString("benchmarks");
            w.WriteHashList(d.Benchmarks.Select(HashBenchmark).ToList());
        }
        if (d.Scalable.HasValue)
        {
            w.WriteString("canScale");
            w.WriteBool(d.Scalable.Value);
        }
        if (!string.IsNullOrEmpty(d.Imperial))
        {
            w.WriteString("defaultImperialUnit");
            w.WriteString(d.Imperial);
        }
        if (!string.IsNullOrEmpty(d.SI))
        {
            w.WriteString("defaultSiUnit");
            w.WriteString(d.SI);
        }
        if (d.Default.HasValue)
        {
            w.WriteString("defaultValue");
            w.WriteNumber(d.Default.Value);
        }
        if (d.Description != null)
        {
            w.WriteString("description");
            w.WriteString(d.Description);
        }
        if (!string.IsNullOrEmpty(d.Formula))
        {
            w.WriteString("formula");
            w.WriteString(d.Formula);
        }
        if (d.MaxExcluded.HasValue)
        {
            w.WriteString("isMaxExcluded");
            w.WriteBool(d.MaxExcluded.Value);
        }
        if (d.MinExcluded.HasValue)
        {
            w.WriteString("isMinExcluded");
            w.WriteBool(d.MinExcluded.Value);
        }
        if (!string.IsNullOrEmpty(d.Key))
        {
            w.WriteString("key");
            w.WriteString(d.Key);
        }
        if (d.Kind != QualityKind.General)
        {
            w.WriteString("kind");
            w.WriteIntNumber((int)d.Kind);
        }
        if (d.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(d.Max.Value);
        }
        if (d.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(d.Min.Value);
        }
        if (!string.IsNullOrEmpty(d.Name))
        {
            w.WriteString("name");
            w.WriteString(d.Name);
        }
        if (!string.IsNullOrEmpty(d.Uri))
        {
            w.WriteString("uri");
            w.WriteString(d.Uri);
        }
        return w.Digest();
    }

    public static string HashTagDiff(TagDiff d)
    {
        var w = new HashWriter();
        w.WriteString("TagDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashTagsDiff(TagsDiff d)
    {
        return HashCollectionDiffGeneric(
            "TagsDiff", "TagModification", "tag",
            (Tag t) => HashTag(t),
            (TagDiff td) => HashTagDiff(td),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Tag.Id, u.Diff!)).ToList() ?? new List<(string, TagDiff)>(),
            d.Added ?? new List<Tag>());
    }

    public static string HashConceptDiff(ConceptDiff d)
    {
        var w = new HashWriter();
        w.WriteString("ConceptDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashConceptsDiff(ConceptsDiff d)
    {
        return HashCollectionDiffGeneric(
            "ConceptsDiff", "ConceptModification", "concept",
            (Concept c) => HashConcept(c),
            (ConceptDiff cd) => HashConceptDiff(cd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Concept.Id, u.Diff!)).ToList() ?? new List<(string, ConceptDiff)>(),
            d.Added ?? new List<Concept>());
    }

    public static string HashPortDiff(PortDiff d)
    {
        var w = new HashWriter();
        w.WriteString("PortDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        if (d.ShouldSerializeCompatiblePorts() && d.CompatiblePorts?.Count > 0)
        {
            w.WriteString("compatiblePorts");
            w.WriteIdList(d.CompatiblePorts.Select(cp => cp.Id).ToList());
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashPortsDiff(PortsDiff d)
    {
        return HashCollectionDiffGeneric(
            "PortsDiff", "PortModification", "port",
            (Port p) => HashPort(p),
            (PortDiff pd) => HashPortDiff(pd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Port.Id, u.Diff!)).ToList() ?? new List<(string, PortDiff)>(),
            d.Added ?? new List<Port>());
    }

    public static string HashPropDiff(PropDiff d)
    {
        var w = new HashWriter();
        w.WriteString("PropDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeQuality() && d.Quality != null)
        {
            w.WriteString("quality");
            w.WriteString(d.Quality.Id);
        }
        WriteDiffOptString(w, "unit", d.Unit, d.ShouldSerializeUnit());
        WriteDiffOptString(w, "value", d.Value, d.ShouldSerializeValue());
        return w.Digest();
    }

    public static string HashRepresentationDiff(RepresentationDiff d)
    {
        var w = new HashWriter();
        w.WriteString("RepresentationDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeFile() && d.File != null)
        {
            w.WriteString("file");
            w.WriteString(d.File.Id);
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeTags() && d.Tags?.Count > 0)
        {
            w.WriteString("tags");
            w.WriteIdList(d.Tags.Select(t => t.Id).ToList());
        }
        return w.Digest();
    }

    public static string HashRepresentationsDiff(RepresentationsDiff d)
    {
        return HashCollectionDiffGeneric(
            "RepresentationsDiff", "RepresentationModification", "representation",
            (Representation m) => HashRepresentation(m),
            (RepresentationDiff md) => HashRepresentationDiff(md),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Representation.Id, u.Diff!)).ToList() ?? new List<(string, RepresentationDiff)>(),
            d.Added ?? new List<Representation>());
    }

    public static string HashConnectorDiff(ConnectorDiff d)
    {
        var w = new HashWriter();
        w.WriteString("ConnectorDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeDirection() && d.Direction != null)
        {
            w.WriteString("direction");
            w.WriteHash(HashVectorDiff(d.Direction));
        }
        WriteDiffOptBool(w, "mandatory", d.Mandatory, d.ShouldSerializeMandatory());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePoint() && d.Point != null)
        {
            w.WriteString("point");
            w.WriteHash(HashPointDiff(d.Point));
        }
        if (d.ShouldSerializePort() && d.Port != null)
        {
            w.WriteString("port");
            w.WriteString(d.Port.Id);
        }
        if (d.ShouldSerializeProps() && d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        WriteDiffOptNumber(w, "t", d.T, d.ShouldSerializeT());
        return w.Digest();
    }

    public static string HashConnectorsDiff(ConnectorsDiff d)
    {
        return HashCollectionDiffGeneric(
            "ConnectorsDiff", "ConnectorModification", "connector",
            (Connector c) => HashConnector(c),
            (ConnectorDiff cd) => HashConnectorDiff(cd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Connector.Id, u.Diff!)).ToList() ?? new List<(string, ConnectorDiff)>(),
            d.Added ?? new List<Connector>());
    }

    public static string HashTypeDiff(TypeDiff d)
    {
        var w = new HashWriter();
        w.WriteString("TypeDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeAuthors() && d.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteIdList(d.Authors.Select(a => a.Id).ToList());
        }
        else if (d.ShouldSerializeAuthors())
        {
            w.WriteString("authors");
            w.WriteBool(false);
        }
        if (d.ShouldSerializeConcepts() && d.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteIdList(d.Concepts.Select(c => c.Id).ToList());
        }
        else if (d.ShouldSerializeConcepts())
        {
            w.WriteString("concepts");
            w.WriteBool(false);
        }
        if (d.ShouldSerializeConnectors() && d.Connectors != null)
        {
            w.WriteString("connectors");
            w.WriteHash(HashConnectorsDiff(d.Connectors));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "folder", d.Folder, d.ShouldSerializeFolder());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffNullableString(w, "image", d.Image, d.ShouldSerializeImage());
        WriteDiffOptBool(w, "isAbstract", d.IsAbstract, d.ShouldSerializeIsAbstract());
        if (d.ShouldSerializeLocation() && d.Location != null)
        {
            w.WriteString("location");
            w.WriteString(d.Location.Id);
        }
        else if (d.ShouldSerializeLocation())
        {
            w.WriteString("location");
            w.WriteBool(false);
        }
        if (d.ShouldSerializeRepresentations() && d.Representations != null)
        {
            w.WriteString("representations");
            w.WriteHash(HashRepresentationsDiff(d.Representations));
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeParent() && d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Id);
        }
        else if (d.ShouldSerializeParent())
        {
            w.WriteString("parent");
            w.WriteBool(false);
        }
        WriteDiffOptIntNumber(w, "stock", d.Stock, d.ShouldSerializeStock());
        WriteDiffOptString(w, "unit", d.Unit != "" ? d.Unit : null, d.ShouldSerializeUnit());
        WriteDiffOptBool(w, "virtual", d.Virtual, d.ShouldSerializeVirtual());
        return w.Digest();
    }

    public static string HashTypesDiff(TypesDiff d)
    {
        return HashCollectionDiffGeneric(
            "TypesDiff", "TypeModification", "type",
            (Type t) => HashType(t),
            (TypeDiff td) => HashTypeDiff(td),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Type.Id, u.Diff!)).ToList() ?? new List<(string, TypeDiff)>(),
            d.Added ?? new List<Type>());
    }

    public static string HashTypologyDiff(TypologyDiff d)
    {
        var w = new HashWriter();
        w.WriteString("TypologyDiff");
        if (d.Description != null)
        {
            w.WriteString("description");
            w.WriteString(d.Description);
        }
        if (d.Designs != null)
        {
            w.WriteString("designs");
            w.WriteHash(HashDesignsDiff(d.Designs));
        }
        if (d.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(d.Folder);
        }
        if (d.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(d.Icon);
        }
        if (d.Name != null)
        {
            w.WriteString("name");
            w.WriteString(d.Name);
        }
        if (d.Types != null)
        {
            w.WriteString("types");
            w.WriteHash(HashTypesDiff(d.Types));
        }
        return w.Digest();
    }

    public static string HashTypologiesDiff(TypologiesDiff d)
    {
        return HashCollectionDiffGeneric(
            "TypologiesDiff", "TypologyModification", "typology",
            (Typology t) => HashTypology(t),
            (TypologyDiff td) => HashTypologyDiff(td),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Select(u => (u.Typology.Id, u.Diff)).ToList() ?? new List<(string, TypologyDiff)>(),
            d.Added ?? new List<Typology>());
    }

    public static string HashSideDiff(SideDiff d)
    {
        var w = new HashWriter();
        w.WriteString("SideDiff");
        if (d.ShouldSerializeConnector() && d.Connector != null)
        {
            w.WriteString("connector");
            w.WriteString(d.Connector.Id);
        }
        if (d.ShouldSerializeDesignPiece() && d.DesignPiece != null)
        {
            w.WriteString("designPiece");
            w.WriteString(d.DesignPiece.Id);
        }
        if (d.ShouldSerializePiece() && d.Piece != null)
        {
            w.WriteString("piece");
            w.WriteString(d.Piece.Id);
        }
        return w.Digest();
    }

    public static string HashLayerDiff(LayerDiff d)
    {
        var w = new HashWriter();
        w.WriteString("LayerDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "color", d.Color, d.ShouldSerializeColor());
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptBool(w, "isHidden", d.IsHidden, d.ShouldSerializeIsHidden());
        WriteDiffOptBool(w, "isLocked", d.IsLocked, d.ShouldSerializeIsLocked());
        WriteDiffOptString(w, "path", d.Path, d.ShouldSerializePath());
        return w.Digest();
    }

    public static string HashGroupDiff(GroupDiff d)
    {
        var w = new HashWriter();
        w.WriteString("GroupDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "color", d.Color, d.ShouldSerializeColor());
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePieces() && d.Pieces?.Count > 0)
        {
            w.WriteString("pieces");
            w.WriteIdList(d.Pieces.Select(p => p.Id).ToList());
        }
        return w.Digest();
    }

    public static string HashStatDiff(StatDiff d)
    {
        var w = new HashWriter();
        w.WriteString("StatDiff");
        WriteDiffOptNumber(w, "max", d.Max, d.ShouldSerializeMax());
        WriteDiffOptBool(w, "maxExcluded", d.MaxExcluded, d.ShouldSerializeMaxExcluded());
        WriteDiffOptNumber(w, "min", d.Min, d.ShouldSerializeMin());
        WriteDiffOptBool(w, "minExcluded", d.MinExcluded, d.ShouldSerializeMinExcluded());
        if (d.ShouldSerializeQuality() && d.Quality != null)
        {
            w.WriteString("quality");
            w.WriteString(d.Quality.Id);
        }
        WriteDiffOptString(w, "unit", d.Unit, d.ShouldSerializeUnit());
        return w.Digest();
    }

    public static string HashConnectionDiff(ConnectionDiff d)
    {
        var w = new HashWriter();
        w.WriteString("ConnectionDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeParent() && d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteHash(HashSideDiff(d.Parent));
        }
        if (d.ShouldSerializeChild() && d.Child != null)
        {
            w.WriteString("child");
            w.WriteHash(HashSideDiff(d.Child));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptNumber(w, "gap", d.Gap, d.ShouldSerializeGap());
        WriteDiffOptNumber(w, "rise", d.Rise, d.ShouldSerializeRise());
        WriteDiffOptNumber(w, "rotation", d.Rotation, d.ShouldSerializeRotation());
        WriteDiffOptNumber(w, "shift", d.Shift, d.ShouldSerializeShift());
        WriteDiffOptNumber(w, "tilt", d.Tilt, d.ShouldSerializeTilt());
        WriteDiffOptNumber(w, "turn", d.Turn, d.ShouldSerializeTurn());
        WriteDiffOptNumber(w, "u", d.U, d.ShouldSerializeU());
        WriteDiffOptNumber(w, "v", d.V, d.ShouldSerializeV());
        return w.Digest();
    }

    public static string HashConnectionsDiff(ConnectionsDiff d)
    {
        return HashCollectionDiffGeneric(
            "ConnectionsDiff", "ConnectionModification", "connection",
            (Connection c) => HashConnection(c),
            (ConnectionDiff cd) => HashConnectionDiff(cd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Connection.Id, u.Diff!)).ToList() ?? new List<(string, ConnectionDiff)>(),
            d.Added ?? new List<Connection>());
    }

    public static string HashPieceDiff(PieceDiff d)
    {
        var w = new HashWriter();
        w.WriteString("PieceDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeCenter() && d.Center != null)
        {
            w.WriteString("center");
            w.WriteHash(HashCoordinateDiff(d.Center));
        }
        WriteDiffOptString(w, "color", d.Color, d.ShouldSerializeColor());
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeDesign() && d.Design != null)
        {
            w.WriteString("design");
            w.WriteString(d.Design.Id);
        }
        WriteDiffOptBool(w, "isHidden", d.IsHidden, d.ShouldSerializeIsHidden());
        WriteDiffOptBool(w, "isLocked", d.IsLocked, d.ShouldSerializeIsLocked());
        if (d.ShouldSerializeMirrorPlane() && d.MirrorPlane != null)
        {
            w.WriteString("mirrorPlane");
            w.WriteHash(HashPlaneDiff(d.MirrorPlane));
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePlane() && d.Plane != null)
        {
            w.WriteString("plane");
            w.WriteHash(HashPlaneDiff(d.Plane));
        }
        if (d.ShouldSerializeProps() && d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        WriteDiffOptNumber(w, "scale", d.Scale, d.ShouldSerializeScale());
        if (d.ShouldSerializeType() && d.Type != null)
        {
            w.WriteString("type");
            w.WriteString(d.Type.Id);
        }
        return w.Digest();
    }

    public static string HashPiecesDiff(PiecesDiff d)
    {
        return HashCollectionDiffGeneric(
            "PiecesDiff", "PieceModification", "piece",
            (Piece p) => HashPiece(p),
            (PieceDiff pd) => HashPieceDiff(pd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Piece.Id, u.Diff!)).ToList() ?? new List<(string, PieceDiff)>(),
            d.Added ?? new List<Piece>());
    }

    public static string HashDesignDiff(DesignDiff d)
    {
        var w = new HashWriter();
        w.WriteString("DesignDiff");
        if (d.ShouldSerializeActiveLayer() && d.ActiveLayer != null)
        {
            w.WriteString("activeLayer");
            w.WriteString(d.ActiveLayer.Id);
        }
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeAuthors() && d.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteIdList(d.Authors.Select(a => a.Id).ToList());
        }
        WriteDiffOptBool(w, "canMirror", d.CanMirror, d.ShouldSerializeCanMirror());
        WriteDiffOptBool(w, "canScale", d.CanScale, d.ShouldSerializeCanScale());
        if (d.ShouldSerializeConcepts() && d.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteIdList(d.Concepts.Select(c => c.Id).ToList());
        }
        if (d.ShouldSerializeConnections() && d.Connections != null)
        {
            w.WriteString("connections");
            w.WriteHash(HashConnectionsDiff(d.Connections));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptString(w, "folder", d.Folder, d.ShouldSerializeFolder());
        if (d.ShouldSerializeGroups() && d.Groups?.Count > 0)
        {
            w.WriteString("groups");
            w.WriteHashList(d.Groups.Select(HashGroup).ToList());
        }
        WriteDiffOptString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "image", d.Image, d.ShouldSerializeImage());
        WriteDiffOptBool(w, "isAbstract", d.IsAbstract, d.ShouldSerializeIsAbstract());
        if (d.ShouldSerializeLayers() && d.Layers?.Count > 0)
        {
            w.WriteString("layers");
            w.WriteHashList(d.Layers.Select(HashLayer).ToList());
        }
        if (d.ShouldSerializeLocation() && d.Location != null)
        {
            w.WriteString("location");
            w.WriteString(d.Location.Id);
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeParent() && d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Id);
        }
        if (d.ShouldSerializePieces() && d.Pieces != null)
        {
            w.WriteString("pieces");
            w.WriteHash(HashPiecesDiff(d.Pieces));
        }
        if (d.ShouldSerializeProps() && d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        if (d.ShouldSerializeStats() && d.Stats?.Count > 0)
        {
            w.WriteString("stats");
            w.WriteHashList(d.Stats.Select(HashStat).ToList());
        }
        WriteDiffOptString(w, "unit", d.Unit, d.ShouldSerializeUnit());
        return w.Digest();
    }

    public static string HashDesignsDiff(DesignsDiff d)
    {
        return HashCollectionDiffGeneric(
            "DesignsDiff", "DesignModification", "design",
            (Design ds) => HashDesign(ds),
            (DesignDiff dd) => HashDesignDiff(dd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Design.Id, u.Diff!)).ToList() ?? new List<(string, DesignDiff)>(),
            d.Added ?? new List<Design>());
    }

    public static string HashKitDiff(KitDiff d)
    {
        var w = new HashWriter();
        w.WriteString("KitDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeAuthors() && d.Authors != null)
        {
            w.WriteString("authors");
            w.WriteHash(HashAuthorsDiff(d.Authors));
        }
        if (d.ShouldSerializeConcepts() && d.Concepts != null)
        {
            w.WriteString("concepts");
            w.WriteHash(HashConceptsDiff(d.Concepts));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeFiles() && d.Files != null)
        {
            w.WriteString("files");
            w.WriteHash(HashFilesDiff(d.Files));
        }
        if (d.ShouldSerializeFolders() && d.Folders != null)
        {
            w.WriteString("folders");
            w.WriteHash(HashFoldersDiff(d.Folders));
        }
        WriteDiffNullableString(w, "homepage", d.Homepage, d.ShouldSerializeHomepage());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffNullableString(w, "image", d.Image, d.ShouldSerializeImage());
        WriteDiffNullableString(w, "license", d.License, d.ShouldSerializeLicense());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePorts() && d.Ports != null)
        {
            w.WriteString("ports");
            w.WriteHash(HashPortsDiff(d.Ports));
        }
        WriteDiffNullableString(w, "preview", d.Preview, d.ShouldSerializePreview());
        WriteDiffNullableString(w, "remote", d.Remote, d.ShouldSerializeRemote());
        if (d.ShouldSerializeTags() && d.Tags != null)
        {
            w.WriteString("tags");
            w.WriteHash(HashTagsDiff(d.Tags));
        }
        if (d.ShouldSerializeTypologies() && d.Typologies != null)
        {
            w.WriteString("typologies");
            w.WriteHash(HashTypologiesDiff(d.Typologies));
        }
        WriteDiffOptString(w, "version", d.Version, d.ShouldSerializeVersion());
        return w.Digest();
    }

    public static string HashKitsDiff(KitsDiff d)
    {
        return HashCollectionDiffGeneric(
            "KitsDiff", "KitModification", "kit",
            (Kit k) => HashKit(k),
            (KitDiff kd) => HashKitDiff(kd),
            d.Removed?.Select(r => r.Id).ToList() ?? new List<string>(),
            d.Modified?.Where(u => u.Diff != null).Select(u => (u.Kit.Id, u.Diff!)).ToList() ?? new List<(string, KitDiff)>(),
            d.Added ?? new List<Kit>());
    }

    // #endregion ⚗️Hash Diff Entities

    // #endregion 🔗Hash Diffs

    // #region 🌳Flatten Merkle Hashes
    // Per-piece merkle hashes for plane/center so cached flatten calls can reuse unchanged chains.

    internal static string HashFlatPlaneRoot(string id, Plane? plane)
    {
        var w = new HashWriter();
        if (plane == null)
        {
            w.WriteString("plane.root.identity");
            w.WriteString(id);
            return w.Digest();
        }
        w.WriteString("plane.root");
        w.WriteString(id);
        w.WriteNumber(plane.Origin?.X ?? 0);
        w.WriteNumber(plane.Origin?.Y ?? 0);
        w.WriteNumber(plane.Origin?.Z ?? 0);
        w.WriteNumber(plane.XAxis?.X ?? 0);
        w.WriteNumber(plane.XAxis?.Y ?? 0);
        w.WriteNumber(plane.XAxis?.Z ?? 0);
        w.WriteNumber(plane.YAxis?.X ?? 0);
        w.WriteNumber(plane.YAxis?.Y ?? 0);
        w.WriteNumber(plane.YAxis?.Z ?? 0);
        return w.Digest();
    }

    internal static string HashFlatPlaneChain(string parentHash, Connector parentConnector, Connector childConnector, Connection connection)
    {
        var w = new HashWriter();
        w.WriteString("plane.chain");
        w.WriteHash(parentHash);
        w.WriteNumber(parentConnector.Point?.X ?? 0);
        w.WriteNumber(parentConnector.Point?.Y ?? 0);
        w.WriteNumber(parentConnector.Point?.Z ?? 0);
        w.WriteNumber(parentConnector.Direction?.X ?? 0);
        w.WriteNumber(parentConnector.Direction?.Y ?? 0);
        w.WriteNumber(parentConnector.Direction?.Z ?? 0);
        w.WriteNumber(childConnector.Point?.X ?? 0);
        w.WriteNumber(childConnector.Point?.Y ?? 0);
        w.WriteNumber(childConnector.Point?.Z ?? 0);
        w.WriteNumber(childConnector.Direction?.X ?? 0);
        w.WriteNumber(childConnector.Direction?.Y ?? 0);
        w.WriteNumber(childConnector.Direction?.Z ?? 0);
        w.WriteNumber(connection.Gap);
        w.WriteNumber(connection.Shift);
        w.WriteNumber(connection.Rise);
        w.WriteNumber(connection.Rotation);
        w.WriteNumber(connection.Turn);
        w.WriteNumber(connection.Tilt);
        return w.Digest();
    }

    internal static string HashFlatCenterRoot(string id, Coordinate? center)
    {
        var w = new HashWriter();
        if (center == null)
        {
            w.WriteString("center.root.identity");
            w.WriteString(id);
            return w.Digest();
        }
        w.WriteString("center.root");
        w.WriteString(id);
        w.WriteNumber(center.U);
        w.WriteNumber(center.V);
        return w.Digest();
    }

    internal static string HashFlatCenterChain(string parentHash, Connector parentConnector, Connection connection)
    {
        var w = new HashWriter();
        w.WriteString("center.chain");
        w.WriteHash(parentHash);
        w.WriteNumber(parentConnector.Direction?.Z ?? 0);
        w.WriteNumber(parentConnector.T);
        w.WriteNumber(connection.U ?? 0);
        w.WriteNumber(connection.V ?? 0);
        return w.Digest();
    }

    // #endregion 🌳Flatten Merkle Hashes
}

#endregion 🖥️Hash





#region 🎪Api
// Callers MUST use these methods to communicate with the compose engine.

public class PredictDesignBody
{
    public string? Description { get; set; }
    public Type[]? Types { get; set; }
    public Design? Design { get; set; }
}

public interface IApi
{
    [Get("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<Kit>> GetKit(string encodedKitUri);

    [Put("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> CreateKit(string encodedKitUri, [Body]
        Kit input);

    [Delete("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> DeleteKit(string encodedKitUri);

    [Put("/api/kits/{encodedKitUri}/type/{encodedTypeName}")]
    Task<ApiResponse<bool>> PutType(string encodedKitUri, string encodedTypeName, [Body]
        Type input);

    [Delete("/api/kits/{encodedKitUri}/type/{encodedTypeName}")]
    Task<ApiResponse<bool>> RemoveType(string encodedKitUri, string encodedTypeName);

    [Put("/api/kits/{encodedKitUri}/design/{encodedDesignName}")]
    Task<ApiResponse<bool>> PutDesign(string encodedKitUri, string encodedDesignName,
    [Body]
        Design input);

    [Delete("/api/kits/{encodedKitUri}/design/{encodedDesignName}")]
    Task<ApiResponse<bool>> RemoveDesign(string encodedKitUri, string encodedDesignName);

    [Get("/api/assistant/predictDesign")]
    Task<ApiResponse<Design>> PredictDesign([Body]
        PredictDesignBody body);
}

public static class Api
{
    private static IApi GetApi()
    {
        var httpClient = new HttpClient
        {
            BaseAddress = new Uri(Constants.EngineAddress),
            Timeout = TimeSpan.FromMinutes(3)
        };
        return RestService.For<IApi>(httpClient, new RefitSettings
        {
            ContentSerializer = new NewtonsoftJsonContentSerializer(
                new JsonSerializerSettings
                {
                    ContractResolver = new CamelCasePropertyNamesContractResolver()
                }
            )
        });
    }

    private static string UnsuccessfullResponseToString<T>(ApiResponse<T> response)
    {
        return ComposeJson.Codec.Serialize(new
        {
            StatusCode = response.StatusCode.ToString(),
            Message = response.Error?.Content ?? "null",
            Request = response.RequestMessage?.ToString() ?? "null",
            Headers = response.Headers?.ToString() ?? "null",
        });
    }

    private static void HandleErrors<T>(ApiResponse<T> response)
    {
        if (response.StatusCode == HttpStatusCode.BadRequest) throw new ClientException(response.Error?.Content ?? "Bad Request");
        if (!response.IsSuccessStatusCode) throw new ServerException(UnsuccessfullResponseToString(response));
    }

    public static string EncodeNameAndVariant(string name, string variant = "") => Utility.Encode(name) + "," + Utility.Encode(variant);

    public static string EncodeNameAndVariantAndView(string name, string variant = "", string view = "") => EncodeNameAndVariant(name, variant) + "," + Utility.Encode(view);

    public static Kit? GetKit(string uri)
    {
        var response = GetApi().GetKit(Utility.Encode(uri)).Result;
        if (response.IsSuccessStatusCode)
            return response.Content;
        HandleErrors(response);
        return null;
    }

    public static void CreateKit(string uri, Kit input) => HandleErrors(GetApi().CreateKit(Utility.Encode(uri), input).Result);

    public static void DeleteKit(string uri) => HandleErrors(GetApi().DeleteKit(Utility.Encode(uri)).Result);

    public static void PutType(string kitUrl, Type input) => HandleErrors(GetApi().PutType(Utility.Encode(kitUrl), Utility.Encode(input.Name), input).Result);

    public static void RemoveType(string kitUrl, TypeId id) => HandleErrors(GetApi().RemoveType(Utility.Encode(kitUrl), Utility.Encode(id.Id)).Result);

    public static void PutDesign(string kitUrl, Design input) => HandleErrors(GetApi().PutDesign(Utility.Encode(kitUrl), Utility.Encode(input.Name), input).Result);

    public static void RemoveDesign(string kitUrl, DesignId id) => HandleErrors(GetApi().RemoveDesign(Utility.Encode(kitUrl), Utility.Encode(id.Id)).Result);

    public static Design? PredictDesign(string description, Type[] types, Design design)
    {
        var response = GetApi().PredictDesign(new PredictDesignBody
        { Description = description, Types = types, Design = design }).Result;
        if (response.IsSuccessStatusCode)
            return response.Content;
        HandleErrors(response);
        return null;
    }
}

public class ClientException : Exception
{
    public ClientException(string message) : base(message) { }
}

public class ServerException : Exception
{
    public ServerException(string message) : base(message) { }
}

#endregion 🎪Api






#region 📦Kit Diff Validation

public sealed class KitDiffValidationNote
{
    [JsonProperty("code", NullValueHandling = NullValueHandling.Ignore)]
    public string? Code { get; set; }
    [JsonProperty("message")]
    public string Message { get; set; } = "";
}

public sealed class KitDiffValidationResult
{
    [JsonProperty("ok")]
    public bool Ok { get; set; }
    public List<KitDiffValidationNote> Errors { get; set; } = new();
    public List<KitDiffValidationNote> Warnings { get; set; } = new();
    [JsonProperty("diff", NullValueHandling = NullValueHandling.Ignore)]
    public KitDiff? Diff { get; set; }
}

#endregion 📦Kit Diff Validation


#region 🔍Find Replaceable Types In Designs
// Find Replaceable Types In Designs MUST find all types and designs that can replace selected pieces in a design.
// Specs: For each external connection, get ALL connector ports of the other piece's type, compute compatible port set, candidate must have connector in set for every connection.

public partial class Kit
{
    /// <summary>
    /// 🔍Finds all types and designs whose root type can replace the selected pieces in a design.
    /// </summary>
    /// <remarks>
    /// Specs: Returns (typeIds, designIds). For connected pieces, checks port compatibility per external connection.
    /// For isolated pieces, checks compatible port set from selected types' connectors.
    /// </remarks>
    public static (List<string> TypeIds, List<string> DesignIds) FindReplaceableTypesInDesignsForPiecesInDesign(
        Kit kit, string designId, List<string> pieceIds)
    {
        var design = kit.Designs?.FirstOrDefault(d => d.Id == designId);
        if (design == null) return (new List<string>(), new List<string>());

        var ports = kit.Ports ?? new List<Port>();
        var types = kit.Types ?? new List<Type>();
        var designs = kit.Designs ?? new List<Design>();
        var pieces = design.Pieces ?? new List<Piece>();
        var connections = design.Connections ?? new List<Connection>();

        var portsMap = ports.ToDictionary(p => p.Id);
        var typesMap = types.ToDictionary(t => t.Id);
        var pieceMap = pieces.ToDictionary(p => p.Id);
        var selectedSet = new HashSet<string>(pieceIds);

        bool ArePortsCompatible(string pg1, string pg2)
        {
            if (pg1 == pg2) return true;
            if (!portsMap.TryGetValue(pg1, out var port1) || !portsMap.TryGetValue(pg2, out var port2))
                return false;
            if (port1.CompatiblePorts.Count == 0 && port2.CompatiblePorts.Count == 0)
                return true;
            if (port1.CompatiblePorts.Any(cp => cp.Id == pg2))
                return true;
            if (port2.CompatiblePorts.Any(cp => cp.Id == pg1))
                return true;
            return false;
        }

        HashSet<string> BuildCompatiblePortSet(Type typ)
        {
            var set = new HashSet<string>();
            foreach (var connector in typ.Connectors)
            {
                var portId = connector.Port?.Id;
                if (string.IsNullOrEmpty(portId)) continue;
                set.Add(portId);
                if (portsMap.TryGetValue(portId, out var port))
                {
                    foreach (var ci in port.CompatiblePorts)
                        set.Add(ci.Id);
                    foreach (var otherPort in ports)
                    {
                        if (otherPort.Id == portId) continue;
                        if (otherPort.CompatiblePorts.Any(ci => ci.Id == portId))
                            set.Add(otherPort.Id);
                    }
                }
                foreach (var p in ports)
                {
                    if (ArePortsCompatible(portId, p.Id))
                        set.Add(p.Id);
                }
            }
            return set;
        }

        bool HasConnectorInSet(List<Connector> connectors, HashSet<string> portSet)
        {
            return connectors.Any(c => c.Port?.Id != null && portSet.Contains(c.Port.Id));
        }

        // Find external connections
        var externalConnections = connections.Where(conn =>
        {
            var connectedSelected = selectedSet.Contains(conn.Parent.Piece.Id);
            var connectingSelected = selectedSet.Contains(conn.Child.Piece.Id);
            return connectedSelected != connectingSelected;
        }).ToList();

        var hasConnections = externalConnections.Count > 0;
        var typeIds = new List<string>();
        var designIds = new List<string>();

        if (hasConnections)
        {
            var perConnectionSets = new List<HashSet<string>>();
            foreach (var conn in externalConnections)
            {
                var connectedSelected = selectedSet.Contains(conn.Parent.Piece.Id);
                var otherPieceId = connectedSelected
                    ? conn.Child.Piece.Id
                    : conn.Parent.Piece.Id;
                if (!pieceMap.TryGetValue(otherPieceId, out var otherPiece)) continue;
                var otherTypeId = otherPiece.Type?.Id;
                if (string.IsNullOrEmpty(otherTypeId)) continue;
                if (!typesMap.TryGetValue(otherTypeId, out var otherType)) continue;
                perConnectionSets.Add(BuildCompatiblePortSet(otherType));
            }

            foreach (var typ in types)
            {
                if (typ.Connectors.Count == 0) continue;
                if (perConnectionSets.All(cs => HasConnectorInSet(typ.Connectors, cs)))
                    typeIds.Add(typ.Id);
            }

            foreach (var d in designs)
            {
                var designConnectors = new List<Connector>();
                foreach (var p in d.Pieces ?? new List<Piece>())
                {
                    if (p.Type == null || string.IsNullOrEmpty(p.Type.Id))
                        continue;
                    if (typesMap.TryGetValue(p.Type.Id, out var t))
                        designConnectors.AddRange(t.Connectors);
                }
                if (designConnectors.Count == 0) continue;
                if (perConnectionSets.All(cs => HasConnectorInSet(designConnectors, cs)))
                    designIds.Add(d.Id);
            }
        }
        else
        {
            var selectedPortIds = new HashSet<string>();
            foreach (var pg in selectedSet)
            {
                if (!pieceMap.TryGetValue(pg, out var piece)) continue;
                var typeId = piece.Type?.Id;
                if (string.IsNullOrEmpty(typeId)) continue;
                if (!typesMap.TryGetValue(typeId, out var typ)) continue;
                foreach (var connector in typ.Connectors)
                {
                    if (connector.Port?.Id != null)
                        selectedPortIds.Add(connector.Port.Id);
                }
            }

            if (selectedPortIds.Count == 0)
            {
                foreach (var typ in types)
                {
                    if (typ.Connectors.Count == 0)
                        typeIds.Add(typ.Id);
                }

                foreach (var d in designs)
                {
                    var designConnectors = new List<Connector>();
                    foreach (var p in d.Pieces ?? new List<Piece>())
                    {
                        if (p.Type == null || string.IsNullOrEmpty(p.Type.Id))
                            continue;
                        if (typesMap.TryGetValue(p.Type.Id, out var t))
                            designConnectors.AddRange(t.Connectors);
                    }
                    if (designConnectors.Count == 0)
                        designIds.Add(d.Id);
                }
            }
            else
            {
                var compatibleSet = new HashSet<string>();
                foreach (var pg in selectedPortIds)
                {
                    compatibleSet.Add(pg);
                    foreach (var p in ports)
                    {
                        if (ArePortsCompatible(pg, p.Id))
                            compatibleSet.Add(p.Id);
                    }
                }

                foreach (var typ in types)
                {
                    if (typ.Connectors.Count == 0) continue;
                    if (HasConnectorInSet(typ.Connectors, compatibleSet))
                        typeIds.Add(typ.Id);
                }
                foreach (var d in designs)
                {
                    var designConnectors = new List<Connector>();
                    foreach (var p in d.Pieces ?? new List<Piece>())
                    {
                        if (p.Type == null || string.IsNullOrEmpty(p.Type.Id))
                            continue;
                        if (typesMap.TryGetValue(p.Type.Id, out var t))
                            designConnectors.AddRange(t.Connectors);
                    }
                    if (designConnectors.Count == 0)
                        continue;
                    if (HasConnectorInSet(designConnectors, compatibleSet))
                        designIds.Add(d.Id);
                }
            }
        }

        return (typeIds, designIds);
    }
}

#endregion 🔍Find Replaceable Types In Designs


#region 🌤️Flatten Design
// Callers MUST use FlattenDesign to compute a DesignDiff that assigns world-space planes to all pieces.

/// <summary>🌳Per-piece merkle hash pair used to cache flattenDesign results and skip recomputation when inputs are unchanged.</summary>
public sealed class FlatMerkleHashes
{
    public string PlaneHash { get; set; } = "";
    public string CenterHash { get; set; } = "";
}

/// <summary>🧠FlatMerkleCacheEntry bundles a piece's merkle hashes with its cached plane/center so incremental flatten calls can reuse unchanged values.</summary>
public sealed class FlatMerkleCacheEntry
{
    public string PlaneHash { get; set; } = "";
    public string CenterHash { get; set; } = "";
    public Plane? Plane { get; set; }
    public Coordinate? Center { get; set; }
}

public partial class Kit
{

    public static DesignDiff FlattenDesignDiff(Kit kit, string designId)
    {
        var design = FindDesign(kit, designId);
        if (design.Pieces == null || design.Pieces.Count == 0) return new DesignDiff();

        var typesDict = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Id);

        Type? GetConnectorType(string typeId) => typesDict.TryGetValue(typeId, out var t) ? t : null;

        Connector? GetConnector(Type? type, string? connectorId)
        {
            if (type == null) return null;

            if (string.IsNullOrEmpty(connectorId))
            {
                if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];
                if (!string.IsNullOrEmpty(type.Parent?.Id))
                {
                    var parentType = GetConnectorType(type.Parent.Id);
                    return GetConnector(parentType, connectorId);
                }
                return null;
            }

            if (type.Connectors != null && type.Connectors.Count > 0)
            {
                var connector = type.Connectors.FirstOrDefault(p => p.Id == connectorId);
                if (connector != null) return connector;
            }

            if (!string.IsNullOrEmpty(type.Parent?.Id))
            {
                var parentType = GetConnectorType(type.Parent.Id);
                var connector = GetConnector(parentType, connectorId);
                if (connector != null) return connector;
            }

            if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];

            return null;
        }

        var flatDesignJson = Utility.Serialize(design);
        var flatDesign = Utility.Deserialize<Design>(flatDesignJson);
        if (flatDesign == null) return new DesignDiff();

        if (flatDesign.Pieces == null) flatDesign.Pieces = new List<Piece>();

        var piecePlanes = new Dictionary<string, Plane>();
        var pieceMap = new Dictionary<string, Piece>();
        foreach (var p in flatDesign.Pieces)
        {
            if (!string.IsNullOrEmpty(p.Id)) pieceMap[p.Id] = p;
        }

        var filteredConnections = (flatDesign.Connections ?? new List<Connection>()).Where(connection =>
        {
            var sourceId = connection.Parent.Piece.Id;
            var targetId = connection.Child.Piece.Id;
            return pieceMap.ContainsKey(sourceId) && pieceMap.ContainsKey(targetId);
        }).ToList();

        static (string, string) NormalizeEdgeEndpoints(string a, string b) =>
            string.CompareOrdinal(a, b) <= 0 ? (a, b) : (b, a);

        var connectionByEndpoints = new Dictionary<(string, string), Connection>();
        foreach (var c in filteredConnections)
        {
            var a = c.Parent.Piece.Id;
            var b = c.Child.Piece.Id;
            connectionByEndpoints[NormalizeEdgeEndpoints(a, b)] = c;
        }

        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var p in flatDesign.Pieces) graph.AddVertex(p.Id);
        foreach (var c in filteredConnections) graph.AddEdge(new Edge<string>(c.Parent.Piece.Id, c.Child.Piece.Id));

        var algorithm = new ConnectedComponentsAlgorithm<string, Edge<string>>(graph);
        algorithm.Compute();

        var components = algorithm.Components;
        var componentDict = new Dictionary<int, List<string>>();
        foreach (var kvp in components)
        {
            if (!componentDict.ContainsKey(kvp.Value)) componentDict[kvp.Value] = new List<string>();
            componentDict[kvp.Value].Add(kvp.Key);
        }

        Piece SetAttributes(Piece piece, IEnumerable<(string key, string value)> newAttrs)
        {
            var updatedAttrs = piece.Attributes?.ToList() ?? new List<Attribute>();
            foreach (var newAttr in newAttrs)
            {
                var existingIndex = updatedAttrs.FindIndex(a => a.Key == newAttr.key);
                if (existingIndex >= 0)
                    updatedAttrs[existingIndex].Value = newAttr.value;
                else
                    updatedAttrs.Add(new Attribute { Id = System.Guid.NewGuid().ToString(), Key = newAttr.key, Value = newAttr.value });
            }
            piece.Attributes = updatedAttrs;
            return piece;
        }

        foreach (var component in componentDict.Values)
        {
            var roots = component.Where(nodeId =>
            {
                var piece = pieceMap.TryGetValue(nodeId, out var p) ? p : null;
                return piece?.Plane != null && piece?.Center != null;
            }).ToList();

            var rootNode = roots.Count > 0 ? roots[0] : (component.Count > 0 ? component[0] : null);
            if (string.IsNullOrEmpty(rootNode)) continue;

            var rootPiece = pieceMap[rootNode];
            if (string.IsNullOrEmpty(rootPiece.Id)) continue;

            var updatedRootPiece = SetAttributes(rootPiece, new[]
            {
                ("compose.fixedPieceId", rootPiece.Id),
                ("compose.depth", "0"),
                ("compose.path", rootPiece.Id)
            });
            pieceMap[rootNode] = updatedRootPiece;

            Plane rootPlane = rootPiece.Plane ?? new Plane { XAxis = new Vector { X = 1, Y = 0, Z = 0 }, YAxis = new Vector { X = 0, Y = 1, Z = 0 }, Origin = new Point { X = 0, Y = 0, Z = 0 } };
            piecePlanes[rootPiece.Id] = rootPlane;

            var rootPieceIndex = flatDesign.Pieces.FindIndex(p => p.Id == rootPiece.Id);
            if (rootPieceIndex != -1)
            {
                flatDesign.Pieces[rootPieceIndex].Plane = rootPlane;
                flatDesign.Pieces[rootPieceIndex].Center ??= new Coordinate { U = 0, V = 0 };
            }

            var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(graph);
            var depths = new Dictionary<string, int>();
            depths[rootNode] = 0;

            bfs.TreeEdge += (sender, e) =>
            {
                var parentId = depths.ContainsKey(e.Source) ? e.Source : e.Target;
                var childId = parentId == e.Source ? e.Target : e.Source;
                depths[childId] = depths[parentId] + 1;

                var parentPiece = pieceMap.TryGetValue(parentId, out var pp) ? pp : null;
                var childPiece = pieceMap.TryGetValue(childId, out var cp) ? cp : null;
                if (parentPiece == null || childPiece == null || string.IsNullOrEmpty(parentPiece.Id) || string.IsNullOrEmpty(childPiece.Id)) return;
                if (piecePlanes.ContainsKey(childPiece.Id)) return;
                if (!piecePlanes.TryGetValue(parentPiece.Id, out var parentPlane)) return;

                if (!connectionByEndpoints.TryGetValue(NormalizeEdgeEndpoints(parentId, childId), out var connection)) return;

                var parentSide = connection.Parent.Piece.Id == parentId ? connection.Parent : connection.Child;
                var childSide = connection.Child.Piece.Id == childId ? connection.Child : connection.Parent;

                var parentType = parentPiece.Type != null ? GetConnectorType(parentPiece.Type.Id) : null;
                var childType = childPiece.Type != null ? GetConnectorType(childPiece.Type.Id) : null;

                var parentConnector = GetConnector(parentType, parentSide.Connector?.Id);
                var childConnector = GetConnector(childType, childSide.Connector?.Id);

                if (parentConnector == null || childConnector == null) return;
                if (parentConnector.Point == null || parentConnector.Direction == null || childConnector.Point == null || childConnector.Direction == null) return;

                var childPlane = Design.DefaultComputeChildPlane(
                    parentPlane, parentConnector.Point, parentConnector.Direction,
                    childConnector.Point, childConnector.Direction,
                    connection.Gap, connection.Shift, connection.Rise,
                    connection.Rotation, connection.Turn, connection.Tilt);
                piecePlanes[childPiece.Id] = childPlane;

                var radius = 2.697;
                var verticalVExtra = 1.0;
                var horizontalScale = 3.0633;
                var parentCenter = parentPiece.Center ?? new Coordinate { U = 0, V = 0 };

                double childU, childV;
                if (parentCenter.U == 0 && parentCenter.V == 0)
                {
                    var angle = 2 * Math.PI * parentConnector.T;
                    childU = radius * Math.Sin(angle);
                    childV = radius * Math.Cos(angle);
                }
                else
                {
                    var isVerticalConnection = Math.Abs(parentConnector.Direction?.Z ?? 0) > 0.5;
                    if (isVerticalConnection)
                    {
                        childU = parentCenter.U + (connection.U ?? 0);
                        childV = parentCenter.V + (connection.V ?? 0) + verticalVExtra;
                    }
                    else
                    {
                        childU = parentCenter.U + (connection.U ?? 0) * horizontalScale;
                        childV = parentCenter.V + (connection.V ?? 0) * horizontalScale;
                    }
                }

                var childCenter = new Coordinate { U = Math.Round(childU, 6), V = Math.Round(childV, 6) };
                var fixedPieceId = parentPiece.Attributes?.FirstOrDefault(q => q.Key == "compose.fixedPieceId")?.Value ?? "";
                var parentPath = parentPiece.Attributes?.FirstOrDefault(q => q.Key == "compose.path")?.Value ?? "";

                childPiece.Plane = childPlane;
                childPiece.Center = childCenter;

                var flatChildPiece = SetAttributes(childPiece, new[]
                {
                    ("compose.fixedPieceId", fixedPieceId),
                    ("compose.parentPieceId", parentPiece.Id),
                    ("compose.depth", depths[childId].ToString()),
                    ("compose.path", parentPath + "," + childPiece.Id)
                });
                pieceMap[childId] = flatChildPiece;
            };

            bfs.Compute(rootNode);
        }

        flatDesign.Pieces = flatDesign.Pieces.Select(p => pieceMap.TryGetValue(p.Id ?? "", out var mapped) ? mapped : p).ToList();
        flatDesign.Connections = new List<Connection>();

        static bool FlattenPlanesApproxEqual(Plane? a, Plane? b)
        {
            if (a == null && b == null) return true;
            if (a == null || b == null) return false;
            const double tol = 0.0001;
            bool Pt(Point? p, Point? q) =>
                p != null && q != null
                && Math.Abs(p.X - q.X) < tol && Math.Abs(p.Y - q.Y) < tol && Math.Abs(p.Z - q.Z) < tol;
            bool Vt(Vector? v, Vector? w) =>
                v != null && w != null
                && Math.Abs(v.X - w.X) < tol && Math.Abs(v.Y - w.Y) < tol && Math.Abs(v.Z - w.Z) < tol;
            return Pt(a.Origin, b.Origin) && Vt(a.XAxis, b.XAxis) && Vt(a.YAxis, b.YAxis);
        }

        static bool FlattenCoordinatesApproxEqual(Coordinate? a, Coordinate? b)
        {
            if (a == null && b == null) return true;
            if (a == null || b == null) return false;
            return Math.Abs(a.U - b.U) < 0.0001 && Math.Abs(a.V - b.V) < 0.0001;
        }

        static bool FlattenAttributesListsEqual(List<Attribute>? a, List<Attribute>? b)
        {
            static string? NormAttr(string? value) => string.IsNullOrEmpty(value) ? null : value;
            if (a == null && b == null) return true;
            if (a == null || b == null) return false;
            if (a.Count != b.Count) return false;
            var byId = a.ToDictionary(x => x.Id ?? "", x => x);
            foreach (var bb in b)
            {
                if (bb.Id == null || !byId.TryGetValue(bb.Id, out var aa)) return false;
                if (aa.Key != bb.Key) return false;
                if (NormAttr(aa.Value) != NormAttr(bb.Value)) return false;
                if (NormAttr(aa.Definition) != NormAttr(bb.Definition)) return false;
            }
            return true;
        }

        var updatedPieces = flatDesign.Pieces.Select(flatPiece =>
        {
            var originalPiece = design.Pieces?.FirstOrDefault(p => p.Id == flatPiece.Id);
            if (originalPiece == null) return null;

            var pieceDiff = new PieceDiff();
            bool hasChanges = false;

            if (flatPiece.Plane != null && !FlattenPlanesApproxEqual(flatPiece.Plane, originalPiece.Plane))
            {
                pieceDiff.Plane = flatPiece.Plane;
                hasChanges = true;
            }

            if (flatPiece.Center != null && !FlattenCoordinatesApproxEqual(flatPiece.Center, originalPiece.Center))
            {
                pieceDiff.Center = flatPiece.Center;
                hasChanges = true;
            }

            if (!FlattenAttributesListsEqual(flatPiece.Attributes, originalPiece.Attributes))
            {
                pieceDiff.Attributes = flatPiece.Attributes.ToList();
                hasChanges = true;
            }

            if (!hasChanges) return null;

            return new PieceModification
            {
                Piece = new PieceId { Id = flatPiece.Id },
                Diff = pieceDiff
            };
        }).Where(u => u != null).Cast<PieceModification>().ToList();

        var removedConnections = (design.Connections ?? new List<Connection>())
            .Select(c => new ConnectionId { Id = c.Id })
            .ToList();

        var designDiff = new DesignDiff();
        if (updatedPieces.Count > 0) designDiff.Pieces = new PiecesDiff { Modified = updatedPieces };
        if (removedConnections.Count > 0) designDiff.Connections = new ConnectionsDiff { Removed = removedConnections };

        return designDiff;
    }

    /// <summary>🌤️Canonical flatten report (forward/backward DesignChange).</summary>
    public static ComposeReport<DesignChange> FlattenDesign(Kit kit, string designId)
    {
        var design = kit.Designs?.FirstOrDefault(d => d.Id == designId);
        if (design == null)
            return ComposeReport<DesignChange>.Failure(new List<OperationNote>
            {
                new() { Code = "flatten.design-not-found", Message = $"Design {designId} not found in kit {kit.Name}" }
            });
        if (design.Pieces == null || design.Pieces.Count == 0)
        {
            var emptyChange = new DesignChange { Forward = new DesignDiff(), Backward = new DesignDiff() };
            return ComposeReport<DesignChange>.Success(emptyChange, new List<OperationNote>(), new List<OperationNote>
            {
                new() { Code = "flatten.empty-pieces", Message = "No pieces to flatten; returning empty forward and backward diffs." }
            });
        }
        var before = Entity<Design>.DeepClone(design) ?? design;
        var forward = FlattenDesignDiff(kit, designId);
        var after = Design.ApplyDiff(Entity<Design>.DeepClone(before)!, forward);
        var backward = Design.GetDesignDiff(after, before);
        var change = new DesignChange { Forward = forward, Backward = backward, Before = before, After = after };
        return ComposeReport<DesignChange>.Success(change, new List<OperationNote>(), new List<OperationNote>());
    }

    #region 🌳Flatten Merkle Hashes
    // Per-piece {PlaneHash, CenterHash} merkle hashes so incremental FlattenDesign calls can reuse cached planes/centers when the chain inputs are unchanged.

    public static Dictionary<string, FlatMerkleHashes> ComputeFlatHashes(Kit kit, string designId)
    {
        var design = FindDesign(kit, designId);
        if (design.Pieces == null || design.Pieces.Count == 0) return new Dictionary<string, FlatMerkleHashes>();

        var typesDict = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Id);

        Type? GetConnectorType(string typeId) => typesDict.TryGetValue(typeId, out var t) ? t : null;

        Connector? GetConnector(Type? type, string? connectorId)
        {
            if (type == null) return null;
            if (string.IsNullOrEmpty(connectorId))
            {
                if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];
                if (!string.IsNullOrEmpty(type.Parent?.Id)) return GetConnector(GetConnectorType(type.Parent.Id), connectorId);
                return null;
            }
            if (type.Connectors != null && type.Connectors.Count > 0)
            {
                var connector = type.Connectors.FirstOrDefault(p => p.Id == connectorId);
                if (connector != null) return connector;
            }
            if (!string.IsNullOrEmpty(type.Parent?.Id))
            {
                var connector = GetConnector(GetConnectorType(type.Parent.Id), connectorId);
                if (connector != null) return connector;
            }
            if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];
            return null;
        }

        var pieceMap = new Dictionary<string, Piece>();
        var pieceOrder = new Dictionary<string, int>();
        for (int i = 0; i < design.Pieces.Count; i++)
        {
            var piece = design.Pieces[i];
            if (!string.IsNullOrEmpty(piece.Id))
            {
                pieceMap[piece.Id] = piece;
                pieceOrder[piece.Id] = i;
            }
        }

        var filteredConnections = (design.Connections ?? new List<Connection>())
            .Where(c => pieceMap.ContainsKey(c.Parent.Piece.Id) && pieceMap.ContainsKey(c.Child.Piece.Id))
            .ToList();

        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var piece in design.Pieces)
            if (!string.IsNullOrEmpty(piece.Id)) graph.AddVertex(piece.Id);
        foreach (var c in filteredConnections)
            graph.AddEdge(new Edge<string>(c.Parent.Piece.Id, c.Child.Piece.Id));

        var ccAlg = new ConnectedComponentsAlgorithm<string, Edge<string>>(graph);
        ccAlg.Compute();
        var components = ccAlg.Components;
        var componentDict = new Dictionary<int, List<string>>();
        foreach (var kvp in components)
        {
            if (!componentDict.TryGetValue(kvp.Value, out var list))
            {
                list = new List<string>();
                componentDict[kvp.Value] = list;
            }
            list.Add(kvp.Key);
        }

        var planeHashes = new Dictionary<string, string>();
        var centerHashes = new Dictionary<string, string>();

        foreach (var component in componentDict.Values)
        {
            var ordered = component.OrderBy(g => pieceOrder.TryGetValue(g, out var idx) ? idx : int.MaxValue).ToList();
            string? rootNode = null;
            foreach (var id in ordered)
            {
                if (pieceMap.TryGetValue(id, out var piece) && piece.Plane != null && piece.Center != null)
                {
                    rootNode = id;
                    break;
                }
            }
            if (string.IsNullOrEmpty(rootNode))
                rootNode = component.OrderBy(g => g, StringComparer.Ordinal).FirstOrDefault();
            if (string.IsNullOrEmpty(rootNode)) continue;

            var rootPiece = pieceMap[rootNode];
            planeHashes[rootNode] = Hashing.HashFlatPlaneRoot(rootNode, rootPiece.Plane);
            centerHashes[rootNode] = Hashing.HashFlatCenterRoot(rootNode, rootPiece.Center);

            var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(graph);
            bfs.TreeEdge += (sender, e) =>
            {
                var parentId = planeHashes.ContainsKey(e.Source) ? e.Source : e.Target;
                var childId = parentId == e.Source ? e.Target : e.Source;
                if (planeHashes.ContainsKey(childId)) return;
                if (!planeHashes.TryGetValue(parentId, out var parentPlaneHash)) return;
                if (!centerHashes.TryGetValue(parentId, out var parentCenterHash)) return;

                var parentPiece = pieceMap.TryGetValue(parentId, out var pp) ? pp : null;
                var childPiece = pieceMap.TryGetValue(childId, out var cp) ? cp : null;
                if (parentPiece == null || childPiece == null) return;

                var connection = filteredConnections.FirstOrDefault(c =>
                    (c.Parent.Piece.Id == parentId && c.Child.Piece.Id == childId) ||
                    (c.Child.Piece.Id == parentId && c.Parent.Piece.Id == childId));
                if (connection == null) return;

                var parentSide = connection.Parent.Piece.Id == parentId ? connection.Parent : connection.Child;
                var childSide = connection.Child.Piece.Id == childId ? connection.Child : connection.Parent;

                var parentType = parentPiece.Type != null ? GetConnectorType(parentPiece.Type.Id) : null;
                var childType = childPiece.Type != null ? GetConnectorType(childPiece.Type.Id) : null;
                var parentConnector = GetConnector(parentType, parentSide.Connector?.Id) ?? new Connector();
                var childConnector = GetConnector(childType, childSide.Connector?.Id) ?? new Connector();

                planeHashes[childId] = Hashing.HashFlatPlaneChain(parentPlaneHash, parentConnector, childConnector, connection);
                centerHashes[childId] = Hashing.HashFlatCenterChain(parentCenterHash, parentConnector, connection);
            };
            bfs.Compute(rootNode);
        }

        var result = new Dictionary<string, FlatMerkleHashes>();
        foreach (var id in planeHashes.Keys)
        {
            result[id] = new FlatMerkleHashes
            {
                PlaneHash = planeHashes[id],
                CenterHash = centerHashes.TryGetValue(id, out var ch) ? ch : "",
            };
        }
        return result;
    }

    public static (ComposeReport<DesignChange> report, Dictionary<string, FlatMerkleCacheEntry> cache) FlattenDesignCached(Kit kit, string designId, Dictionary<string, FlatMerkleCacheEntry>? cache = null)
    {
        var newHashes = ComputeFlatHashes(kit, designId);
        var report = FlattenDesign(kit, designId);
        if (!report.Ok || report.Diff == null)
            return (report, new Dictionary<string, FlatMerkleCacheEntry>());
        var diff = report.Diff.Forward;
        var updatedById = new Dictionary<string, PieceDiff>();
        foreach (var entry in diff.Pieces?.Modified ?? new List<PieceModification>())
        {
            if (entry.Piece != null && !string.IsNullOrEmpty(entry.Piece.Id) && entry.Diff != null)
                updatedById[entry.Piece.Id] = entry.Diff;
        }
        var nextCache = new Dictionary<string, FlatMerkleCacheEntry>();
        if (cache != null)
        {
            foreach (var kvp in newHashes)
            {
                var id = kvp.Key;
                var hashes = kvp.Value;
                cache.TryGetValue(id, out var prev);
                updatedById.TryGetValue(id, out var updated);
                if (prev == null || updated == null)
                {
                    if (updated != null)
                    {
                        nextCache[id] = new FlatMerkleCacheEntry
                        {
                            PlaneHash = hashes.PlaneHash,
                            CenterHash = hashes.CenterHash,
                            Plane = updated.Plane,
                            Center = updated.Center,
                        };
                    }
                    continue;
                }
                var reusedPlane = prev.PlaneHash == hashes.PlaneHash ? prev.Plane : updated.Plane;
                var reusedCenter = prev.CenterHash == hashes.CenterHash ? prev.Center : updated.Center;
                nextCache[id] = new FlatMerkleCacheEntry
                {
                    PlaneHash = hashes.PlaneHash,
                    CenterHash = hashes.CenterHash,
                    Plane = reusedPlane,
                    Center = reusedCenter,
                };
            }
        }
        else
        {
            foreach (var kvp in newHashes)
            {
                var id = kvp.Key;
                var hashes = kvp.Value;
                if (!updatedById.TryGetValue(id, out var updated)) continue;
                nextCache[id] = new FlatMerkleCacheEntry
                {
                    PlaneHash = hashes.PlaneHash,
                    CenterHash = hashes.CenterHash,
                    Plane = updated.Plane,
                    Center = updated.Center,
                };
            }
        }
        return (report, nextCache);
    }

    #endregion 🌳Flatten Merkle Hashes

    public static DesignDiff ReplaceClusterWithDesign(Design originalDesign, List<string> clusterPieceIds, Design clusteredDesign, List<Connection> externalConnections)
    {
        var addedPieces = clusteredDesign.Pieces ?? new List<Piece>();
        var addedConnections = clusteredDesign.Connections ?? new List<Connection>();

        var addedClusteredConnections = externalConnections.Select(c =>
        {
            var newConnection = Utility.Deserialize<Connection>(Utility.Serialize(c));
            if (newConnection != null) newConnection.Id = System.Guid.NewGuid().ToString();
            return newConnection;
        }).Where(c => c != null).Cast<Connection>().ToList();

        addedConnections.AddRange(addedClusteredConnections);

        return new DesignDiff
        {
            Pieces = new PiecesDiff
            {
                Removed = clusterPieceIds.Select(id => new PieceId { Id = id }).ToList(),
                Added = addedPieces
            },
            Connections = new ConnectionsDiff
            {
                Removed = (originalDesign.Connections ?? new List<Connection>())
                    .Where(c => clusterPieceIds.Contains(c.Parent.Piece.Id) || clusterPieceIds.Contains(c.Child.Piece.Id))
                    .Select(c => new ConnectionId
                    {
                        Parent = new Side { Piece = new PieceId { Id = c.Parent.Piece.Id } },
                        Child = new Side { Piece = new PieceId { Id = c.Child.Piece.Id } }
                    }).ToList(),
                Added = addedConnections
            }
        };
    }
}

#endregion 🌤️Flatten Design

#region 🔩Kit Representation Export
// Callers MUST use ExportDesignRepresentation to produce a valid 3D file from a design.

public partial class Kit
{

    /// <summary>📺Supported export formats keyed by file extension.</summary>
    public static Dictionary<string, string> ExportRepresentationFormats => new()
    {
        { ".glb", "GL Transmission Format Binary" },
        { ".gltf", "GL Transmission Format" },
        { ".obj", "Wavefront OBJ" },
        { ".stl", "Stereolithography" },
    };

    /// <summary>
    /// Exports the 3D representation of a design to the specified format.
    /// Uses block definitions for types and instances for pieces.
    /// Connection hierarchy is translated into a scene graph; planes become relative transformation matrices.
    /// </summary>
    public static byte[] ExportDesignRepresentation(Kit kit, string designId, string format = ".glb", string[] tags = null, Dictionary<string, object> options = null)
    {
        if (tags == null) tags = Array.Empty<string>();
        if (options == null) options = new Dictionary<string, object>();
        if (!ExportRepresentationFormats.ContainsKey(format))
            throw new ArgumentException($"Unsupported export format: {format}. Supported: {string.Join(", ", ExportRepresentationFormats.Keys)}", nameof(format));

        var design = FindDesign(kit, designId);
        var pieces = design.Pieces ?? new List<Piece>();
        var connections = design.Connections ?? new List<Connection>();
        var types = kit.Types ?? new List<Type>();

        if (pieces.Count == 0)
            return ExportSceneBuilderToFormat(new SceneBuilder("empty"), format);

        var typesDict = new Dictionary<string, Type>();
        foreach (var t in types) typesDict[t.Id] = t;
        var piecesDict = new Dictionary<string, Piece>();
        foreach (var p in pieces) piecesDict[p.Id] = p;

        var adjacency = new Dictionary<string, List<(Connection connection, string neighborId)>>();
        foreach (var p in pieces) adjacency[p.Id] = new List<(Connection, string)>();
        foreach (var conn in connections)
        {
            var connectedId = conn.Parent.Piece.Id;
            var connectingId = conn.Child.Piece.Id;
            if (adjacency.ContainsKey(connectedId))
                adjacency[connectedId].Add((conn, connectingId));
            if (adjacency.ContainsKey(connectingId))
                adjacency[connectingId].Add((conn, connectedId));
        }

        var piecePlanes = new Dictionary<string, Plane>();
        var parentOf = new Dictionary<string, string>();
        var childrenOf = new Dictionary<string, List<string>>();
        foreach (var p in pieces) childrenOf[p.Id] = new List<string>();

        var visited = new HashSet<string>();
        var roots = new List<string>();
        var queue = new Queue<string>();

        Type GetType(string typeId) => typesDict.TryGetValue(typeId, out var t) ? t : null;
        Connector GetConnector(Type type, string connectorId)
        {
            if (type == null) return null;
            if (string.IsNullOrEmpty(connectorId))
                return type.Connectors?.Count > 0 ? type.Connectors[0] : null;
            return type.Connectors?.FirstOrDefault(c => c.Id == connectorId);
        }

        foreach (var p in pieces)
        {
            if (p.Plane != null && p.Center != null)
            {
                piecePlanes[p.Id] = p.Plane;
                visited.Add(p.Id);
                queue.Enqueue(p.Id);
                roots.Add(p.Id);
            }
        }

        if (queue.Count == 0 && pieces.Count > 0)
        {
            var identityPlane = new Plane
            {
                Origin = new Point { X = 0, Y = 0, Z = 0 },
                XAxis = new Vector { X = 1, Y = 0, Z = 0 },
                YAxis = new Vector { X = 0, Y = 1, Z = 0 }
            };
            piecePlanes[pieces[0].Id] = identityPlane;
            visited.Add(pieces[0].Id);
            queue.Enqueue(pieces[0].Id);
            roots.Add(pieces[0].Id);
        }

        while (queue.Count > 0)
        {
            var currentId = queue.Dequeue();
            var currentPlane = piecePlanes[currentId];
            if (!adjacency.TryGetValue(currentId, out var edges)) continue;
            foreach (var edge in edges)
            {
                if (visited.Contains(edge.neighborId)) continue;
                var conn = edge.connection;
                var isParent = conn.Parent.Piece.Id == currentId;
                if (!isParent) continue;

                var childId = edge.neighborId;
                var parentPiece = piecesDict[currentId];
                var childPiece = piecesDict[childId];
                var parentType = parentPiece.Type != null ? GetType(parentPiece.Type.Id) : null;
                var childType = childPiece.Type != null ? GetType(childPiece.Type.Id) : null;
                var parentConnector = GetConnector(parentType, conn.Parent.Connector?.Id);
                var childConnector = GetConnector(childType, conn.Child.Connector?.Id);

                if (parentConnector != null && childConnector != null &&
                    parentConnector.Point != null && parentConnector.Direction != null &&
                    childConnector.Point != null && childConnector.Direction != null)
                {
                    piecePlanes[childId] = Design.DefaultComputeChildPlane(
                        currentPlane, parentConnector.Point, parentConnector.Direction,
                        childConnector.Point, childConnector.Direction,
                        conn.Gap, conn.Shift, conn.Rise,
                        conn.Rotation, conn.Turn, conn.Tilt);
                }
                else
                {
                    piecePlanes[childId] = currentPlane;
                }

                parentOf[childId] = currentId;
                childrenOf[currentId].Add(childId);
                visited.Add(childId);
                queue.Enqueue(childId);
            }
        }

        foreach (var p in pieces)
        {
            if (!visited.Contains(p.Id))
            {
                piecePlanes[p.Id] = new Plane
                {
                    Origin = new Point { X = 0, Y = 0, Z = 0 },
                    XAxis = new Vector { X = 1, Y = 0, Z = 0 },
                    YAxis = new Vector { X = 0, Y = 1, Z = 0 }
                };
                roots.Add(p.Id);
            }
        }

        var typeMeshBuilders = new Dictionary<string, IMeshBuilder<MaterialBuilder>>();
        foreach (var piece in pieces)
        {
            var typeId = piece.Type?.Id;
            if (string.IsNullOrEmpty(typeId) || typeMeshBuilders.ContainsKey(typeId)) continue;
            if (!typesDict.TryGetValue(typeId, out var type)) continue;

            var representation = ExportFindMatchingRepresentation(kit, type, tags);
            if (representation != null)
            {
                var file = kit.Files?.FirstOrDefault(f => f.Id == representation.File.Id);
                if (file?.Blob != null)
                {
                    var fileBytes = ExportBlobToBytes(file.Blob);
                    var ext = System.IO.Path.GetExtension(file.Name).ToLowerInvariant();
                    if (ext == ".glb")
                    {
                        var mb = ExportGlbToMeshBuilder(fileBytes, file.Name);
                        if (mb != null) { typeMeshBuilders[typeId] = mb; continue; }
                    }
                }
            }
            typeMeshBuilders[typeId] = ExportCreateBoxMeshBuilder(type.Name);
        }

        var sceneBuilder = new SceneBuilder(design.Name ?? "design");

        void BuildNodeHierarchy(string pieceId, NodeBuilder parent)
        {
            var piece = piecesDict[pieceId];
            var worldPlane = piecePlanes[pieceId];

            NodeBuilder node;
            if (parent != null)
            {
                node = parent.CreateNode(piece.Name ?? piece.Id);
                var parentWorld = ExportPlaneToMatrix4x4(piecePlanes[parentOf[pieceId]]);
                var childWorld = ExportPlaneToMatrix4x4(worldPlane);
                System.Numerics.Matrix4x4.Invert(parentWorld, out var parentInv);
                node.LocalMatrix = childWorld * parentInv;
            }
            else
            {
                node = new NodeBuilder(piece.Name ?? piece.Id);
                node.LocalMatrix = ExportPlaneToMatrix4x4(worldPlane);
            }

            var meshTypeId = piece.Type?.Id;
            if (!string.IsNullOrEmpty(meshTypeId) && typeMeshBuilders.TryGetValue(meshTypeId, out var meshBuilder))
                sceneBuilder.AddRigidMesh(meshBuilder, node);
            else
                sceneBuilder.AddNode(node);

            if (childrenOf.TryGetValue(pieceId, out var children))
            {
                foreach (var childId in children)
                    BuildNodeHierarchy(childId, node);
            }
        }

        foreach (var rootId in roots)
            BuildNodeHierarchy(rootId, null);

        return ExportSceneBuilderToFormat(sceneBuilder, format);
    }

    #region 🔧Kit Representation Export Helpers

    private static System.Numerics.Matrix4x4 ExportPlaneToMatrix4x4(Plane p)
    {
        var origin = new System.Numerics.Vector3((float)p.Origin.X, (float)p.Origin.Y, (float)p.Origin.Z);
        var x = System.Numerics.Vector3.Normalize(new System.Numerics.Vector3((float)p.XAxis.X, (float)p.XAxis.Y, (float)p.XAxis.Z));
        var yRaw = new System.Numerics.Vector3((float)p.YAxis.X, (float)p.YAxis.Y, (float)p.YAxis.Z);
        var z = System.Numerics.Vector3.Normalize(System.Numerics.Vector3.Cross(x, yRaw));
        var y = System.Numerics.Vector3.Normalize(System.Numerics.Vector3.Cross(z, x));
        return ExportApplyComposeToGltfBasis(new System.Numerics.Matrix4x4(
            x.X, x.Y, x.Z, 0,
            y.X, y.Y, y.Z, 0,
            z.X, z.Y, z.Z, 0,
            origin.X, origin.Y, origin.Z, 1));
    }

    private static System.Numerics.Matrix4x4 ExportApplyComposeToGltfBasis(System.Numerics.Matrix4x4 matrix)
    {
        var basis = new System.Numerics.Matrix4x4(
            1, 0, 0, 0,
            0, 0, -1, 0,
            0, 1, 0, 0,
            0, 0, 0, 1);
        var inverse = new System.Numerics.Matrix4x4(
            1, 0, 0, 0,
            0, 0, 1, 0,
            0, -1, 0, 0,
            0, 0, 0, 1);
        return System.Numerics.Matrix4x4.Multiply(System.Numerics.Matrix4x4.Multiply(inverse, matrix), basis);
    }

    private static byte[] ExportBlobToBytes(string blob)
    {
        var base64 = blob;
        if (blob.StartsWith("data:"))
        {
            var commaIdx = blob.IndexOf(',');
            if (commaIdx >= 0) base64 = blob.Substring(commaIdx + 1);
        }
        return Convert.FromBase64String(base64);
    }

    public static Representation ExportFindMatchingRepresentation(Kit kit, Type type, string[] tags)
    {
        if (type.Representations == null || type.Representations.Count == 0) return null;
        if (tags == null || tags.Length == 0)
        {
            var defaultRepresentation = type.Representations.FirstOrDefault(m => m.Tags == null || m.Tags.Count == 0);
            return defaultRepresentation ?? type.Representations[0];
        }
        var kitTags = kit.Tags ?? new List<Tag>();
        var selectedTagIds = new HashSet<string>();
        foreach (var tagValue in tags)
        {
            var byId = kitTags.FirstOrDefault(t => t.Id == tagValue);
            if (byId != null)
            {
                selectedTagIds.Add(byId.Id);
                continue;
            }
            foreach (var tag in kitTags.Where(t => t.Name == tagValue))
                selectedTagIds.Add(tag.Id);
        }
        Representation bestRepresentation = null;
        double bestScore = -1;
        foreach (var representation in type.Representations)
        {
            var representationTagIds = new HashSet<string>((representation.Tags ?? new List<TagId>()).Select(t => t.Id));
            if (!selectedTagIds.All(representationTagIds.Contains)) continue;
            var intersection = representationTagIds.Intersect(selectedTagIds).Count();
            var union = representationTagIds.Union(selectedTagIds).Count();
            var score = union > 0 ? (double)intersection / union : 0;
            if (score > bestScore)
            {
                bestScore = score;
                bestRepresentation = representation;
            }
        }
        if (bestRepresentation != null) return bestRepresentation;
        return type.Representations[0];
    }

    private static IMeshBuilder<MaterialBuilder> ExportGlbToMeshBuilder(byte[] glbBytes, string name)
    {
        var srcRepresentation = GltfRepresentation.ReadGLB(new MemoryStream(glbBytes));
        var meshBuilder = new MeshBuilder<VertexPositionNormal>(name);

        foreach (var srcMesh in srcRepresentation.LogicalMeshes)
        {
            foreach (var srcPrim in srcMesh.Primitives)
            {
                var posAccessor = srcPrim.GetVertexAccessor("POSITION");
                if (posAccessor == null) continue;
                var positions = posAccessor.AsVector3Array();

                var normAccessor = srcPrim.GetVertexAccessor("NORMAL");
                var normals = normAccessor?.AsVector3Array();

                var matBuilder = new MaterialBuilder(srcPrim.Material?.Name ?? "default")
                    .WithMetallicRoughnessShader();

                if (srcPrim.Material != null)
                {
                    var baseColor = srcPrim.Material.FindChannel("BaseColor");
                    if (baseColor.HasValue)
                        matBuilder.UseChannel(KnownChannel.BaseColor).Parameter = baseColor.Value.Color;
                }

                var prim = meshBuilder.UsePrimitive(matBuilder);

                var idxAccessor = srcPrim.IndexAccessor;
                if (idxAccessor != null)
                {
                    var indices = idxAccessor.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        var i0 = (int)indices[i];
                        var i1 = (int)indices[i + 1];
                        var i2 = (int)indices[i + 2];
                        prim.AddTriangle(
                            new VertexPositionNormal(positions[i0], normals != null ? normals[i0] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i1], normals != null ? normals[i1] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i2], normals != null ? normals[i2] : System.Numerics.Vector3.UnitZ));
                    }
                }
                else
                {
                    for (int i = 0; i + 2 < positions.Count; i += 3)
                    {
                        prim.AddTriangle(
                            new VertexPositionNormal(positions[i], normals != null ? normals[i] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i + 1], normals != null ? normals[i + 1] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i + 2], normals != null ? normals[i + 2] : System.Numerics.Vector3.UnitZ));
                    }
                }
            }
        }

        return meshBuilder;
    }

    private static IMeshBuilder<MaterialBuilder> ExportCreateBoxMeshBuilder(string name)
    {
        var meshBuilder = new MeshBuilder<VertexPositionNormal>(name);
        var material = new MaterialBuilder("default").WithUnlitShader();
        var prim = meshBuilder.UsePrimitive(material);

        const float h = 0.5f;
        var v = new System.Numerics.Vector3[]
        {
            new(-h, -h, -h), new(h, -h, -h), new(h, h, -h), new(-h, h, -h),
            new(-h, -h, h), new(h, -h, h), new(h, h, h), new(-h, h, h)
        };
        var uz = System.Numerics.Vector3.UnitZ;
        var ux = System.Numerics.Vector3.UnitX;
        var uy = System.Numerics.Vector3.UnitY;
        VertexPositionNormal V(System.Numerics.Vector3 pos, System.Numerics.Vector3 nrm) => new(pos, nrm);
        prim.AddTriangle(V(v[4], uz), V(v[5], uz), V(v[6], uz));
        prim.AddTriangle(V(v[4], uz), V(v[6], uz), V(v[7], uz));
        prim.AddTriangle(V(v[1], -uz), V(v[0], -uz), V(v[3], -uz));
        prim.AddTriangle(V(v[1], -uz), V(v[3], -uz), V(v[2], -uz));
        prim.AddTriangle(V(v[3], uy), V(v[7], uy), V(v[6], uy));
        prim.AddTriangle(V(v[3], uy), V(v[6], uy), V(v[2], uy));
        prim.AddTriangle(V(v[0], -uy), V(v[1], -uy), V(v[5], -uy));
        prim.AddTriangle(V(v[0], -uy), V(v[5], -uy), V(v[4], -uy));
        prim.AddTriangle(V(v[1], ux), V(v[2], ux), V(v[6], ux));
        prim.AddTriangle(V(v[1], ux), V(v[6], ux), V(v[5], ux));
        prim.AddTriangle(V(v[0], -ux), V(v[4], -ux), V(v[7], -ux));
        prim.AddTriangle(V(v[0], -ux), V(v[7], -ux), V(v[3], -ux));

        return meshBuilder;
    }

    private static byte[] ExportSceneBuilderToFormat(SceneBuilder sceneBuilder, string format)
    {
        var representationRoot = sceneBuilder.ToGltf2();
        representationRoot.Asset.Generator = "compose";

        switch (format)
        {
            case ".glb":
                {
                    using var ms = new MemoryStream();
                    representationRoot.WriteGLB(ms);
                    return ms.ToArray();
                }
            case ".gltf":
                {
                    var tmpDir = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "compose_gltf_" + System.Guid.NewGuid());
                    System.IO.Directory.CreateDirectory(tmpDir);
                    try
                    {
                        var gltfPath = System.IO.Path.Combine(tmpDir, "representation.gltf");
                        representationRoot.SaveGLTF(gltfPath);
                        var gltfJson = Newtonsoft.Json.Linq.JObject.Parse(System.IO.File.ReadAllText(gltfPath));
                        foreach (var buffer in gltfJson["buffers"] as Newtonsoft.Json.Linq.JArray ?? new Newtonsoft.Json.Linq.JArray())
                        {
                            var uri = buffer["uri"]?.Value<string>();
                            if (string.IsNullOrWhiteSpace(uri) || uri.StartsWith("data:")) continue;
                            var path = System.IO.Path.Combine(tmpDir, uri);
                            if (!System.IO.File.Exists(path)) continue;
                            var bytes = System.IO.File.ReadAllBytes(path);
                            buffer["uri"] = "data:application/octet-stream;base64," + Convert.ToBase64String(bytes);
                        }
                        foreach (var image in gltfJson["images"] as Newtonsoft.Json.Linq.JArray ?? new Newtonsoft.Json.Linq.JArray())
                        {
                            var uri = image["uri"]?.Value<string>();
                            if (string.IsNullOrWhiteSpace(uri) || uri.StartsWith("data:")) continue;
                            var path = System.IO.Path.Combine(tmpDir, uri);
                            if (!System.IO.File.Exists(path)) continue;
                            var mime = image["mimeType"]?.Value<string>() ?? "application/octet-stream";
                            var bytes = System.IO.File.ReadAllBytes(path);
                            image["uri"] = $"data:{mime};base64,{Convert.ToBase64String(bytes)}";
                        }
                        return Encoding.UTF8.GetBytes(gltfJson.ToString(Formatting.None));
                    }
                    finally
                    {
                        System.IO.Directory.Delete(tmpDir, true);
                    }
                }
            case ".obj":
                return ExportRepresentationRootToObj(representationRoot);
            case ".stl":
                return ExportRepresentationRootToStl(representationRoot);
            default:
                throw new ArgumentException($"Unsupported export format: {format}");
        }
    }

    #region ❄️Geometric Insights
    // Key performance indicators for GLB/GLTF representation geometry. Representation MUST be glb/gltf.

    /// <summary>🔷Geometric KPIs for a GLB/GLTF representation in compose coordinate system (compose x=glb x, compose y=-glb x, compose z=glb y).</summary>
    public class GeometricInsights
    {
        public Point? BoundingBoxMin { get; set; }
        public Point? BoundingBoxMax { get; set; }
        public double DimensionX { get; set; }
        public double DimensionY { get; set; }
        public double DimensionZ { get; set; }
        public double CharacteristicLength { get; set; }
        public double FootprintArea { get; set; }
        public double TotalSurfaceArea { get; set; }
        public double EnclosedVolume { get; set; }
        public double SurfaceToVolumeRatio { get; set; }
        public double AspectRatioXy { get; set; }
        public double AspectRatioXz { get; set; }
        public double AspectRatioYz { get; set; }
        public bool IsWatertight { get; set; }
        public Point? Centroid { get; set; }
        public double Slenderness { get; set; }
        public int VertexCount { get; set; }
        public int FaceCount { get; set; }
        public int EulerCharacteristic { get; set; }
    }

    public static GeometricInsights GetGeometricInsightsForRepresentation(object representation)
    {
        GltfRepresentation root;
        if (representation is string path)
        {
            if (!System.IO.File.Exists(path))
                throw new FileNotFoundException("Representation file not found", path);
            var ext = System.IO.Path.GetExtension(path).ToLowerInvariant();
            if (ext != ".glb" && ext != ".gltf")
                throw new ArgumentException("Representation MUST be .glb or .gltf", nameof(representation));
            root = GltfRepresentation.Load(path);
        }
        else if (representation is byte[] bytes)
        {
            using var ms = new MemoryStream(bytes);
            if (bytes.Length >= 4 && Encoding.ASCII.GetString(bytes, 0, 4) == "glTF")
                root = GltfRepresentation.ReadGLB(ms);
            else
            {
                var tmp = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "compose_gltf_" + System.Guid.NewGuid().ToString("N") + ".gltf");
                try
                {
                    System.IO.File.WriteAllBytes(tmp, bytes);
                    root = GltfRepresentation.Load(tmp);
                }
                finally { try { System.IO.File.Delete(tmp); } catch { } }
            }
        }
        else
            throw new ArgumentException("Representation must be string path or byte[]", nameof(representation));

        var out_ = new GeometricInsights();
        double sxMin = double.MaxValue, syMin = double.MaxValue, szMin = double.MaxValue;
        double sxMax = double.MinValue, syMax = double.MinValue, szMax = double.MinValue;
        double sumSx = 0, sumSy = 0, sumSz = 0;
        double totalArea = 0, totalVolume = 0;
        int vertexCount = 0, faceCount = 0;

        foreach (var mesh in root.LogicalMeshes)
        {
            foreach (var prim in mesh.Primitives)
            {
                var posAcc = prim.GetVertexAccessor("POSITION");
                if (posAcc == null) continue;
                var positions = posAcc.AsVector3Array();
                var idxAcc = prim.IndexAccessor;
                int n = positions.Count;
                for (int i = 0; i < n; i++)
                {
                    var p = positions[i];
                    double xg = p.X, yg = p.Y, _zg = p.Z;
                    double sx = xg, sy = -xg, sz = yg;
                    if (sx < sxMin) sxMin = sx; if (sx > sxMax) sxMax = sx;
                    if (sy < syMin) syMin = sy; if (sy > syMax) syMax = sy;
                    if (sz < szMin) szMin = sz; if (sz > szMax) szMax = sz;
                    sumSx += sx; sumSy += sy; sumSz += sz;
                }
                vertexCount += n;
                if (idxAcc != null)
                {
                    var indices = idxAcc.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        int i0 = (int)indices[i], i1 = (int)indices[i + 1], i2 = (int)indices[i + 2];
                        var a = positions[i0]; var b = positions[i1]; var c = positions[i2];
                        var ab = b - a; var ac = c - a;
                        var cross = Vector3.Cross(ab, ac);
                        totalArea += 0.5 * cross.Length();
                        totalVolume += (1.0 / 6.0) * Vector3.Dot(a, Vector3.Cross(b, c));
                        faceCount++;
                    }
                }
                else
                {
                    for (int i = 0; i + 2 < n; i += 3)
                    {
                        var a = positions[i]; var b = positions[i + 1]; var c = positions[i + 2];
                        var ab = b - a; var ac = c - a;
                        totalArea += 0.5 * Vector3.Cross(ab, ac).Length();
                        totalVolume += (1.0 / 6.0) * Vector3.Dot(a, Vector3.Cross(b, c));
                        faceCount++;
                    }
                }
            }
        }

        if (vertexCount == 0) return out_;

        out_.BoundingBoxMin = new Point { X = (float)sxMin, Y = (float)syMin, Z = (float)szMin };
        out_.BoundingBoxMax = new Point { X = (float)sxMax, Y = (float)syMax, Z = (float)szMax };
        out_.DimensionX = sxMax - sxMin;
        out_.DimensionY = syMax - syMin;
        out_.DimensionZ = szMax - szMin;
        out_.CharacteristicLength = Math.Pow(out_.DimensionX * out_.DimensionY * out_.DimensionZ, 1.0 / 3.0);
        out_.FootprintArea = out_.DimensionX * out_.DimensionZ;
        out_.TotalSurfaceArea = totalArea;
        out_.VertexCount = vertexCount;
        out_.FaceCount = faceCount;
        double nV = vertexCount;
        out_.Centroid = new Point { X = (float)(sumSx / nV), Y = (float)(sumSy / nV), Z = (float)(sumSz / nV) };
        totalVolume = Math.Abs(totalVolume);
        out_.EnclosedVolume = totalVolume;
        if (totalVolume > 1e-20 && totalArea > 0)
            out_.SurfaceToVolumeRatio = totalArea / totalVolume;
        if (out_.DimensionY > 1e-10 && out_.DimensionX > 1e-10) out_.AspectRatioXy = out_.DimensionX / out_.DimensionY;
        if (out_.DimensionZ > 1e-10 && out_.DimensionX > 1e-10) out_.AspectRatioXz = out_.DimensionX / out_.DimensionZ;
        if (out_.DimensionZ > 1e-10 && out_.DimensionY > 1e-10) out_.AspectRatioYz = out_.DimensionY / out_.DimensionZ;
        double maxExt = Math.Max(out_.DimensionX, Math.Max(out_.DimensionY, out_.DimensionZ));
        if (maxExt > 1e-10 && totalArea > 0)
            out_.Slenderness = maxExt / Math.Pow(totalArea * maxExt, 1.0 / 3.0);
        out_.EulerCharacteristic = vertexCount - (3 * faceCount) / 2 + faceCount;
        return out_;
    }

    #endregion ❄️Geometric Insights

    private static byte[] ExportRepresentationRootToObj(GltfRepresentation representation)
    {
        var sb = new StringBuilder();
        sb.AppendLine("# Generated by compose");
        int vertexOffset = 1;
        int normalOffset = 1;

        foreach (var node in representation.DefaultScene.VisualChildren)
            ExportNodeToObj(node, System.Numerics.Matrix4x4.Identity, sb, ref vertexOffset, ref normalOffset);

        return Encoding.UTF8.GetBytes(sb.ToString());
    }

    private static void ExportNodeToObj(GltfNode node, System.Numerics.Matrix4x4 parentWorld,
        StringBuilder sb, ref int vertexOffset, ref int normalOffset)
    {
        var worldMatrix = node.LocalMatrix * parentWorld;

        if (node.Mesh != null)
        {
            sb.AppendLine($"g {node.Name ?? "mesh"}");
            foreach (var srcPrim in node.Mesh.Primitives)
            {
                var posAccessor = srcPrim.GetVertexAccessor("POSITION");
                if (posAccessor == null) continue;
                var positions = posAccessor.AsVector3Array();
                var normAccessor = srcPrim.GetVertexAccessor("NORMAL");
                var normals = normAccessor?.AsVector3Array();

                int startVert = vertexOffset;
                int startNorm = normalOffset;

                foreach (var pos in positions)
                {
                    var wp = System.Numerics.Vector3.Transform(pos, worldMatrix);
                    sb.AppendLine(string.Format(System.Globalization.CultureInfo.InvariantCulture,
                        "v {0:G9} {1:G9} {2:G9}", wp.X, wp.Y, wp.Z));
                    vertexOffset++;
                }

                if (normals != null)
                {
                    foreach (var norm in normals)
                    {
                        var wn = System.Numerics.Vector3.Normalize(
                            System.Numerics.Vector3.TransformNormal(norm, worldMatrix));
                        sb.AppendLine(string.Format(System.Globalization.CultureInfo.InvariantCulture,
                            "vn {0:G9} {1:G9} {2:G9}", wn.X, wn.Y, wn.Z));
                        normalOffset++;
                    }
                }

                var idxAccessor = srcPrim.IndexAccessor;
                if (idxAccessor != null)
                {
                    var indices = idxAccessor.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        var i0 = (int)indices[i] + startVert;
                        var i1 = (int)indices[i + 1] + startVert;
                        var i2 = (int)indices[i + 2] + startVert;
                        if (normals != null)
                        {
                            var n0 = (int)indices[i] + startNorm;
                            var n1 = (int)indices[i + 1] + startNorm;
                            var n2 = (int)indices[i + 2] + startNorm;
                            sb.AppendLine($"f {i0}//{n0} {i1}//{n1} {i2}//{n2}");
                        }
                        else
                        {
                            sb.AppendLine($"f {i0} {i1} {i2}");
                        }
                    }
                }
            }
        }

        foreach (var child in node.VisualChildren)
            ExportNodeToObj(child, worldMatrix, sb, ref vertexOffset, ref normalOffset);
    }

    private static byte[] ExportRepresentationRootToStl(GltfRepresentation representation)
    {
        var triangles = new List<(System.Numerics.Vector3 normal, System.Numerics.Vector3 v0, System.Numerics.Vector3 v1, System.Numerics.Vector3 v2)>();

        foreach (var node in representation.DefaultScene.VisualChildren)
            ExportNodeToStlTriangles(node, System.Numerics.Matrix4x4.Identity, triangles);

        using var ms = new MemoryStream();
        using var writer = new BinaryWriter(ms);

        var header = new byte[80];
        Encoding.ASCII.GetBytes("compose STL", 0, 9, header, 0);
        writer.Write(header);
        writer.Write((uint)triangles.Count);

        foreach (var (normal, v0, v1, v2) in triangles)
        {
            writer.Write(normal.X); writer.Write(normal.Y); writer.Write(normal.Z);
            writer.Write(v0.X); writer.Write(v0.Y); writer.Write(v0.Z);
            writer.Write(v1.X); writer.Write(v1.Y); writer.Write(v1.Z);
            writer.Write(v2.X); writer.Write(v2.Y); writer.Write(v2.Z);
            writer.Write((ushort)0);
        }

        return ms.ToArray();
    }

    private static void ExportNodeToStlTriangles(GltfNode node, System.Numerics.Matrix4x4 parentWorld,
        List<(System.Numerics.Vector3, System.Numerics.Vector3, System.Numerics.Vector3, System.Numerics.Vector3)> triangles)
    {
        var worldMatrix = node.LocalMatrix * parentWorld;

        if (node.Mesh != null)
        {
            foreach (var srcPrim in node.Mesh.Primitives)
            {
                var posAccessor = srcPrim.GetVertexAccessor("POSITION");
                if (posAccessor == null) continue;
                var positions = posAccessor.AsVector3Array();

                var idxAccessor = srcPrim.IndexAccessor;
                if (idxAccessor != null)
                {
                    var indices = idxAccessor.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        var p0 = System.Numerics.Vector3.Transform(positions[(int)indices[i]], worldMatrix);
                        var p1 = System.Numerics.Vector3.Transform(positions[(int)indices[i + 1]], worldMatrix);
                        var p2 = System.Numerics.Vector3.Transform(positions[(int)indices[i + 2]], worldMatrix);
                        var normal = System.Numerics.Vector3.Normalize(
                            System.Numerics.Vector3.Cross(p1 - p0, p2 - p0));
                        if (float.IsNaN(normal.X)) normal = System.Numerics.Vector3.UnitZ;
                        triangles.Add((normal, p0, p1, p2));
                    }
                }
            }
        }

        foreach (var child in node.VisualChildren)
            ExportNodeToStlTriangles(child, worldMatrix, triangles);
    }

    #endregion 🔧Kit Representation Export Helpers
}

#endregion 🔩Kit Representation Export






#region 🎪ZipRoundtrip
// Callers MUST use these methods to import and export kits as ZIP archives.

public class KitImportResult
{
    public Kit Kit { get; set; } = new();
    public Dictionary<string, byte[]> Files { get; set; } = new();
}

public static class ZipRoundtrip
{
    private static string BuildFolderPath(Kit kit, string folderId)
    {
        if (kit.Folders == null) return "";
        foreach (var f in kit.Folders)
        {
            if (f.Id == folderId)
            {
                if (f.Parent != null)
                {
                    var parentPath = BuildFolderPath(kit, f.Parent?.Id);
                    if (!string.IsNullOrEmpty(parentPath))
                        return $"{parentPath}/{f.Name}";
                }
                return f.Name;
            }
        }
        return "";
    }

    private static string BuildFilePath(Kit kit, File file)
    {
        if (file.Folder != null && !string.IsNullOrEmpty(file.Folder.Id))
        {
            var folderPath = BuildFolderPath(kit, file.Folder.Id);
            if (!string.IsNullOrEmpty(folderPath))
                return $"{folderPath}/{file.Name}";
        }
        return file.Name;
    }

    public static KitImportResult ImportKit(string zipPath)
    {
        var result = new KitImportResult();
        var tempDir = Path.Combine(Path.GetTempPath(), $"compose-kit-{System.Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);

        try
        {
            ZipFile.ExtractToDirectory(zipPath, tempDir);
            var kitJsonPath = StoreKitIO.ResolveKitJsonPath(tempDir);
            if (kitJsonPath == null)
                throw new FileNotFoundException($"No kit.json under {tempDir}");
            var storeBin = StorePaths.ResolveStoreBinary();
            if (!string.IsNullOrEmpty(storeBin) && System.IO.File.Exists(storeBin))
                result.Kit = StoreKitIO.LoadKitFromFolder(tempDir);
            else
                result.Kit = Utility.DeserializeKit(System.IO.File.ReadAllText(kitJsonPath))!;

            foreach (var file in Directory.GetFiles(tempDir, "*", SearchOption.AllDirectories))
            {
                var relativePath = file.Substring(tempDir.Length).TrimStart(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar).Replace("\\", "/");
                if (relativePath != "kit.json" && !relativePath.StartsWith(".compose/"))
                    result.Files[relativePath] = System.IO.File.ReadAllBytes(file);
            }

            if (result.Kit.Files != null)
            {
                foreach (var kitFile in result.Kit.Files)
                {
                    var filePath = BuildFilePath(result.Kit, kitFile);
                    if (result.Files.TryGetValue(filePath, out var bytes))
                    {
                        kitFile.Blob = $"data:application/octet-stream;base64,{Convert.ToBase64String(bytes)}";
                    }
                }
            }
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }

        return result;
    }

    public static void ExportKit(Kit kit, string zipPath)
    {
        if (System.IO.File.Exists(zipPath)) System.IO.File.Delete(zipPath);
        var tempDir = Path.Combine(Path.GetTempPath(), $"compose-kit-{System.Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);
        try
        {
            FolderKit.Export(kit, tempDir);
            ZipFile.CreateFromDirectory(tempDir, zipPath);
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }
    }

}

#endregion 🎪ZipRoundtrip





#region 📋TransportKit
// Callers MUST use TransportKit to wrap serialized kit payloads for transport.

/// <summary>📋 Wraps static JSON kit payloads for serialization and deserialization.</summary>
public class TransportKit
{
    public string Json { get; }

    public TransportKit(string json)
    {
        Json = json;
    }

    public Kit ToKit() => Utility.DeserializeKit(Json)!;

    public static TransportKit FromKit(Kit kit) => new(Utility.Serialize(kit));

    public static Kit EditTransportKit(Kit kit, KitDiff diff)
    {
        var clone = Utility.DeserializeKit(Utility.Serialize(kit))!;
        KitInPlaceDiff.ApplyKitDiff(clone, diff);
        return clone;
    }
}

#endregion 📋TransportKit





#region 🔄ISyncKit
// Callers MUST implement ISyncKit for synchronized kit workflows.

/// <summary>🔄 Contract for synchronized kit workflows.</summary>
public interface ISyncKit
{
    Kit Kit { get; }
    void Apply(KitDiff diff);
    void ImportTransport(TransportKit transport);
    TransportKit ExportTransport();
    void Close();
}

#endregion 🔄ISyncKit





#region 🧪DevKit
// Callers MUST use DevKit for synchronized JSON file kit workflows.

/// <summary>📝 Synchronized JSON file kit.</summary>
public class DevKit : ISyncKit
{
    private readonly Kit _kit;

    public DevKit(Kit kit)
    {
        _kit = kit;
    }

    public Kit Kit => _kit;

    public void Apply(KitDiff diff)
    {
        KitInPlaceDiff.ApplyKitDiff(_kit, diff);
    }

    public void ImportTransport(TransportKit transport)
    {
        KitState.ReplaceInPlace(_kit, transport.ToKit());
    }

    public TransportKit ExportTransport() => TransportKit.FromKit(_kit);

    public void Close() { }

    public static DevKit FromJson(string json) => new(Utility.DeserializeKit(json)!);

    public static Kit Import(string path) => FileKit.Import(path);

    public static void Export(Kit kit, string path) => FileKit.Export(kit, path);

    public static Kit Edit(string path, KitDiff diff) => FileKit.Edit(path, diff);

    public static Kit ImportDevKit(string path) => Import(path);

    public static void ExportDevKit(Kit kit, string path) => Export(kit, path);

    public static Kit EditDevKit(string path, KitDiff diff) => Edit(path, diff);
}

#endregion 🧪DevKit





#region 🏡LocalKit
// Callers MUST use LocalKit for synchronized local folder kit workflows.

/// <summary>📂 Synchronized local folder kit (materialized via compose-gql).</summary>
public class LocalKit : ISyncKit
{
    private readonly Kit _kit;

    public LocalKit(Kit kit)
    {
        _kit = kit;
    }

    public Kit Kit => _kit;

    public void Apply(KitDiff diff)
    {
        KitInPlaceDiff.ApplyKitDiff(_kit, diff);
    }

    public void ImportTransport(TransportKit transport)
    {
        KitState.ReplaceInPlace(_kit, transport.ToKit());
    }

    public TransportKit ExportTransport() => TransportKit.FromKit(_kit);

    public void Close() { }

    public static KitImportResult Import(string folderPath) => FolderKit.Import(folderPath);

    public static void Export(Kit kit, string folderPath) => FolderKit.Export(kit, folderPath);

    public static Kit Edit(string folderPath, KitDiff diff) => FolderKit.Edit(folderPath, diff);

    public static KitImportResult ImportLocalKit(string folderPath) => Import(folderPath);

    public static void ExportLocalKit(Kit kit, string folderPath) => Export(kit, folderPath);

    public static Kit EditLocalKit(string folderPath, KitDiff diff) => Edit(folderPath, diff);
}

#endregion 🏡LocalKit






#region 📷FileKit
// Callers MUST use FileKit for JSON file kit import, export, and edit operations.

public static class FileKit
{
    public static Kit Import(string path) => StoreKitIO.LoadKitFromFile(path);

    public static void Export(Kit kit, string path) => StoreKitIO.SaveKitToFile(kit, path);

    public static Kit Edit(string path, KitDiff diff)
    {
        var edited = TransportKit.EditTransportKit(Import(path), diff);
        Export(edited, path);
        return edited;
    }
}

#endregion 📷FileKit






#region 🏰FolderKit
// Callers MUST use FolderKit for local folder kit import, export, and edit operations.

public static class FolderKit
{
    private static string BuildFolderPath(Kit kit, string folderId)
    {
        foreach (var folder in kit.Folders ?? new List<Folder>())
        {
            if (folder.Id != folderId) continue;
            if (folder.Parent != null && !string.IsNullOrEmpty(folder.Parent.Id))
            {
                var parentPath = BuildFolderPath(kit, folder.Parent.Id);
                if (!string.IsNullOrEmpty(parentPath)) return $"{parentPath}/{folder.Name}";
            }
            return folder.Name;
        }
        return "";
    }

    private static string BuildFilePath(Kit kit, File file)
    {
        if (file.Folder != null && !string.IsNullOrEmpty(file.Folder.Id))
        {
            var folderPath = BuildFolderPath(kit, file.Folder.Id);
            if (!string.IsNullOrEmpty(folderPath)) return $"{folderPath}/{file.Name}";
        }
        return file.Name;
    }

    private static Dictionary<string, byte[]> CollectKitFiles(Kit kit)
    {
        var files = new Dictionary<string, byte[]>();
        foreach (var file in kit.Files ?? new List<File>())
        {
            if (string.IsNullOrEmpty(file.Blob)) continue;
            var blobData = file.Blob.StartsWith("data:") && file.Blob.Contains(",")
                ? file.Blob.Substring(file.Blob.IndexOf(',') + 1)
                : file.Blob;
            files[BuildFilePath(kit, file)] = Convert.FromBase64String(blobData);
        }
        return files;
    }

    private static void HydrateKitFiles(Kit kit, Dictionary<string, byte[]> files)
    {
        foreach (var file in kit.Files ?? new List<File>())
        {
            var path = BuildFilePath(kit, file);
            if (files.TryGetValue(path, out var bytes))
                file.Blob = $"data:application/octet-stream;base64,{Convert.ToBase64String(bytes)}";
        }
    }

    public static KitImportResult Import(string folderPath)
    {
        var kit = StoreKitIO.LoadKitFromFolder(folderPath);
        var files = new Dictionary<string, byte[]>();
        foreach (var file in Directory.GetFiles(folderPath, "*", SearchOption.AllDirectories))
        {
            var relativePath = file.Substring(folderPath.Length).TrimStart(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar).Replace("\\", "/");
            if (relativePath.StartsWith(".compose/")) continue;
            files[relativePath] = System.IO.File.ReadAllBytes(file);
        }
        HydrateKitFiles(kit, files);
        return new KitImportResult { Kit = kit, Files = files };
    }

    public static void Export(Kit kit, string folderPath)
    {
        Directory.CreateDirectory(folderPath);
        StoreKitIO.SaveKitToFolder(kit, folderPath);
        foreach (var entry in CollectKitFiles(kit))
        {
            var fullPath = Path.Combine(folderPath, entry.Key.Replace("/", Path.DirectorySeparatorChar.ToString()));
            Directory.CreateDirectory(Path.GetDirectoryName(fullPath)!);
            System.IO.File.WriteAllBytes(fullPath, entry.Value);
        }
    }

    public static Kit Edit(string folderPath, KitDiff diff)
    {
        var imported = Import(folderPath);
        var edited = TransportKit.EditTransportKit(imported.Kit, diff);
        Export(edited, folderPath);
        return edited;
    }
}

#endregion 🏰FolderKit






#region 📐ArchiveKit
// Callers MUST use ArchiveKit for ZIP archive import, export, and edit operations.

public class ArchiveKit
{
    public byte[] Data { get; }

    public ArchiveKit(byte[] data)
    {
        Data = data;
    }

    public static KitImportResult Import(string zipPath) => ZipRoundtrip.ImportKit(zipPath);

    public static void Export(Kit kit, string zipPath) => ZipRoundtrip.ExportKit(kit, zipPath);

    public static Kit Edit(string zipPath, KitDiff diff)
    {
        var imported = Import(zipPath);
        var edited = TransportKit.EditTransportKit(imported.Kit, diff);
        Export(edited, zipPath);
        return edited;
    }
}

#endregion 📐ArchiveKit






#region 🎆RemoteKit
// Callers MUST use RemoteKit for HTTP-based JSON and ZIP kit import and in-memory edits.

public class RemoteKit : ISyncKit
{
    private readonly Kit _kit;

    public RemoteKit(Kit kit)
    {
        _kit = kit;
    }

    public Kit Kit => _kit;

    public void Apply(KitDiff diff)
    {
        KitInPlaceDiff.ApplyKitDiff(_kit, diff);
    }

    public void ImportTransport(TransportKit transport)
    {
        KitState.ReplaceInPlace(_kit, transport.ToKit());
    }

    public TransportKit ExportTransport() => TransportKit.FromKit(_kit);

    public void Close() { }

    public static KitImportResult Import(string url)
    {
        using var client = new HttpClient();
        using var response = client.GetAsync(url).GetAwaiter().GetResult();
        response.EnsureSuccessStatusCode();
        var contentType = response.Content.Headers.ContentType?.MediaType?.ToLowerInvariant() ?? "";
        var bytes = response.Content.ReadAsByteArrayAsync().GetAwaiter().GetResult();

        if (url.EndsWith(".zip", StringComparison.OrdinalIgnoreCase) || contentType.Contains("zip") || contentType.Contains("octet-stream") || (bytes.Length >= 4 && bytes[0] == (byte)'P' && bytes[1] == (byte)'K'))
        {
            var tempPath = Path.Combine(Path.GetTempPath(), $"compose-remote-{System.Guid.NewGuid():N}.zip");
            try
            {
                System.IO.File.WriteAllBytes(tempPath, bytes);
                return ArchiveKit.Import(tempPath);
            }
            finally
            {
                if (System.IO.File.Exists(tempPath)) System.IO.File.Delete(tempPath);
            }
        }

        var json = Encoding.UTF8.GetString(bytes);
        return new KitImportResult { Kit = Utility.DeserializeKit(json)! };
    }

    public static Kit Edit(string url, KitDiff diff)
    {
        var imported = Import(url);
        return TransportKit.EditTransportKit(imported.Kit, diff);
    }
}

#endregion 🎆RemoteKit






#region 🔤TemporaryKit
// Callers MUST use TemporaryKit for in-memory kit edits without persistence.

public static class TemporaryKit
{
    public static Kit Edit(Kit kit, KitDiff diff)
    {
        var clone = Utility.DeserializeKit(Utility.Serialize(kit))!;
        KitInPlaceDiff.ApplyKitDiff(clone, diff);
        return clone;
    }

    public static Kit EditTemporaryKit(Kit kit, KitDiff diff) => Edit(kit, diff);
}

#endregion 🔤TemporaryKit






#region 📦KitImporter
// Callers MUST use ImportFromZip for high-level kit import.

public static class KitImporter
{
    public static KitImportResult ImportFromZip(string zipPath)
    {
        return ArchiveKit.Import(zipPath);
    }
}

#endregion 📦KitImporter






#region 🪁KitExporter
// Callers MUST use ExportToZip for high-level kit export.

public static class KitExporter
{
    public static void ExportToZip(Kit kit, string zipPath)
    {
        ArchiveKit.Export(kit, zipPath);
    }
}

#endregion 🪁KitExporter

//#region 🔀ComposeDiff
/// <summary>Kit diff validation, <see cref="AreKitsEqual"/> (normalized JSON), and canonical <see cref="KitDiff"/> JSON comparison.</summary>
public static class ComposeDiff
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
        var kitObj = JObject.Parse(ComposeJson.Codec.SerializeKitDiffValidation(kit));
        var diffObj = JObject.Parse(ComposeJson.Codec.SerializeKitDiffValidation(diff));
        JObject? outDiff = heal ? (JObject)diffObj.DeepClone() : null;
        var refs = RefSets.FromKit(kitObj);
        RunTypologiesIdCollection(ctx, kitObj, diffObj, outDiff, heal, refs);
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
                diffOut = ComposeJson.Codec.DeserializeKitDiffValidation<KitDiff>(outDiff.ToString(Formatting.None));
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
            IdSetFromNestedEntities(kitObj["typologies"] as JArray, "types"),
            IdSetFromNestedEntities(kitObj["typologies"] as JArray, "designs"),
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

    private static HashSet<string> IdSetFromNestedEntities(JArray? typologies, string key)
    {
        var s = new HashSet<string>();
        if (typologies == null) return s;
        foreach (var t in typologies)
            if (t is JObject topo)
                s.UnionWith(IdSetFromEntities(topo[key]));
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

    /// <summary>🏛️Validates (and, when healing, prunes) the top-level typologies collection plus each modified typology's nested types/designs diffs.</summary>
    private static void RunTypologiesIdCollection(KitDiffValidationContext ctx, JObject kitObj, JObject diffObj, JObject? outDiff, bool heal, RefSets refs)
    {
        const string key = "typologies";
        if (diffObj[key] is not JObject part) return;
        var baseArr = kitObj[key] as JArray;
        var baseTypologyBy = new Dictionary<string, JObject>();
        foreach (var topo in ToJObjectList(baseArr))
            if (topo["id"]?.Value<string>() is { } tg && !string.IsNullOrEmpty(tg))
                baseTypologyBy[tg] = topo;

        var fixedObj = ValidateIdCollectionDiff(ctx, key, "typology", baseArr, part, null);

        if (DiffUpdatesArray(part) is JArray updates)
            foreach (var u in updates)
                if (u is JObject um && um["typology"] is JObject tIdObj)
                {
                    var tg = tIdObj["id"]?.Value<string>() ?? "";
                    if (string.IsNullOrEmpty(tg) || !baseTypologyBy.TryGetValue(tg, out var baseTopo)) continue;
                    if (um["diff"] is not JObject dm) continue;
                    var path = $"{key}.typology[{tg}]";

                    var healedTypesObj = dm["types"] is JObject typesDm
                        ? ValidateIdCollectionDiff(ctx, $"{path}.types", "type", baseTopo["types"] as JArray, typesDm, null)
                        : null;
                    var healedDesignsObj = dm["designs"] is JObject designsDm
                        ? ValidateIdCollectionDiff(ctx, $"{path}.designs", "design", baseTopo["designs"] as JArray, designsDm,
                            (c, item, ddm, dp) => ValidateDesignDiffNested(c, kitObj, item, ddm, dp, refs))
                        : null;

                    if (!heal) continue;
                    var healedUm = FindMatchingUpdate(fixedObj, "typology", tg);
                    if (healedUm?["diff"] is not JObject healedDm) continue;
                    if (healedTypesObj != null) healedDm["types"] = healedTypesObj; else healedDm.Remove("types");
                    if (healedDesignsObj != null) healedDm["designs"] = healedDesignsObj; else healedDm.Remove("designs");
                }

        if (!heal || outDiff == null) return;
        if (fixedObj != null && fixedObj.Properties().Any()) outDiff[key] = fixedObj;
        else outDiff.Remove(key);
    }

    private static JObject? FindMatchingUpdate(JObject? collection, string idKey, string id)
    {
        if (collection == null || DiffUpdatesArray(collection) is not JArray arr) return null;
        foreach (var u in arr)
            if (u is JObject um && um[idKey] is JObject idObj && idObj["id"]?.Value<string>() == id)
                return um;
        return null;
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
                        ctx.Push("warnings", "kitdiff.cycle.no-operation-restore", $"{path}: removed and re-added {idKey} {rg} are deeply equal (no effective change)");
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
//#endregion 🔀ComposeDiff


//#region 🔧KitInPlaceDiff
/// <summary>In-place application of a <see cref="KitDiff"/> to a <see cref="Kit"/> (host-side, no persistence).</summary>
public static class KitInPlaceDiff
{
    public static void ApplyKitDiff(Kit kit, KitDiff diff)
    {
        if (diff.ShouldSerializeName()) kit.Name = diff.Name ?? "";
        if (diff.ShouldSerializeVersion()) kit.Version = diff.Version ?? "";
        if (diff.ShouldSerializeDescription()) kit.Description = diff.Description;
        if (diff.ShouldSerializeIcon()) kit.Icon = diff.Icon;
        if (diff.ShouldSerializeImage()) kit.Image = diff.Image;
        if (diff.ShouldSerializePreview()) kit.Preview = diff.Preview;
        if (diff.ShouldSerializeRemote()) kit.Remote = diff.Remote;
        if (diff.ShouldSerializeHomepage()) kit.Homepage = diff.Homepage;
        if (diff.ShouldSerializeLicense()) kit.License = diff.License;
        if (diff.ShouldSerializeCreatedAt()) kit.CreatedAt = diff.CreatedAt ?? kit.CreatedAt;
        if (diff.ShouldSerializeModificationdAt()) kit.ModificationdAt = diff.ModificationdAt ?? kit.ModificationdAt;

        if (diff.Typologies != null)
        {
            kit.EnsureTypologies();
            kit.Typologies ??= new List<Typology>();
            ApplyTypologiesDiff(kit.Typologies, diff.Typologies);
            kit.FlattenFromTypologies();
        }

        if (diff.Tags != null)
        {
            kit.Tags ??= new List<Tag>();
            ApplyTagsDiff(kit.Tags, diff.Tags);
        }

        if (diff.Folders != null)
        {
            kit.Folders ??= new List<Folder>();
            ApplyFoldersDiff(kit.Folders, diff.Folders);
        }

        if (diff.Ports != null)
        {
            kit.Ports ??= new List<Port>();
            ApplyPortsDiff(kit.Ports, diff.Ports);
        }

        if (diff.Concepts != null)
        {
            kit.Concepts ??= new List<Concept>();
            ApplyConceptsDiff(kit.Concepts, diff.Concepts);
        }

        if (diff.Files != null)
        {
            kit.Files ??= new List<File>();
            ApplyFilesDiff(kit.Files, diff.Files);
        }

        if (diff.Authors != null)
        {
            kit.Authors ??= new List<Author>();
            ApplyAuthorsDiff(kit.Authors, diff.Authors);
        }

        if (diff.Attributes != null)
        {
            kit.Attributes ??= new List<Attribute>();
            ApplyAttributesDiff(kit.Attributes, diff.Attributes);
        }
    }

    private static void ApplyTagsDiff(List<Tag> tags, TagsDiff diff)
    {
        if (diff.Removed != null)
            tags.RemoveAll(t => diff.Removed.Any(r => r.Id == t.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var tag = tags.FirstOrDefault(t => t.Id == update.Tag.Id);
                if (tag != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) tag.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) tag.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) tag.Icon = update.Diff.Icon;
                }
            }
        }

        if (diff.Added != null)
            tags.AddRange(diff.Added);
    }

    private static void ApplyFoldersDiff(List<Folder> folders, FoldersDiff diff)
    {
        if (diff.Removed != null)
            folders.RemoveAll(f => diff.Removed.Any(r => r.Id == f.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var folder = folders.FirstOrDefault(f => f.Id == update.Folder.Id);
                if (folder != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) folder.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) folder.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeParent()) folder.Parent = update.Diff.Parent;
                }
            }
        }

        if (diff.Added != null)
            folders.AddRange(diff.Added);
    }

    private static void ApplyPortsDiff(List<Port> ports, PortsDiff diff)
    {
        if (diff.Removed != null)
            ports.RemoveAll(p => diff.Removed.Any(r => r.Id == p.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var port = ports.FirstOrDefault(p => p.Id == update.Port.Id);
                if (port != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) port.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) port.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) port.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeCompatiblePorts()) port.CompatiblePorts = update.Diff.CompatiblePorts;
                }
            }
        }

        if (diff.Added != null)
            ports.AddRange(diff.Added);
    }

    private static void ApplyConceptsDiff(List<Concept> concepts, ConceptsDiff diff)
    {
        if (diff.Removed != null)
            concepts.RemoveAll(c => diff.Removed.Any(r => r.Id == c.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var concept = concepts.FirstOrDefault(c => c.Id == update.Concept.Id);
                if (concept != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) concept.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) concept.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) concept.Icon = update.Diff.Icon;
                }
            }
        }

        if (diff.Added != null)
            concepts.AddRange(diff.Added);
    }

    private static void ApplyFilesDiff(List<File> files, FilesDiff diff)
    {
        if (diff.Removed != null)
            files.RemoveAll(f => diff.Removed.Any(r => r.Id == f.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var file = files.FirstOrDefault(f => f.Id == update.File.Id);
                if (file != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) file.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeRemote()) file.Remote = update.Diff.Remote;
                    if (update.Diff.ShouldSerializeFolder()) file.Folder = update.Diff.Folder;
                }
            }
        }

        if (diff.Added != null)
            files.AddRange(diff.Added);
    }

    private static void ApplyAuthorsDiff(List<Author> authors, AuthorsDiff diff)
    {
        if (diff.Removed != null)
            authors.RemoveAll(a => diff.Removed.Any(r => r.Id == a.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var author = authors.FirstOrDefault(a => a.Id == update.Author.Id);
                if (author != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) author.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeEmail()) author.Email = update.Diff.Email ?? "";
                }
            }
        }

        if (diff.Added != null)
            authors.AddRange(diff.Added);
    }

    private static void ApplyAttributesDiff(List<Attribute> attributes, AttributesDiff diff)
    {
        if (diff.Removed != null)
            attributes.RemoveAll(a => diff.Removed.Any(r => r.Id == a.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var attr = attributes.FirstOrDefault(a => a.Id == update.Attribute.Id);
                if (attr != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeValue()) attr.Value = update.Diff.Value;
                    if (update.Diff.ShouldSerializeDefinition()) attr.Definition = update.Diff.Definition;
                }
            }
        }

        if (diff.Added != null)
            attributes.AddRange(diff.Added);
    }

    private static void ApplyTypologiesDiff(List<Typology> typologies, TypologiesDiff diff)
    {
        if (diff.Removed != null)
            typologies.RemoveAll(t => diff.Removed.Any(r => r.Id == t.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var topo = typologies.FirstOrDefault(t => t.Id == update.Typology.Id);
                if (topo != null && update.Diff != null)
                {
                    if (update.Diff.Name != null) topo.Name = update.Diff.Name;
                    if (update.Diff.Description != null) topo.Description = update.Diff.Description;
                    if (update.Diff.Icon != null) topo.Icon = update.Diff.Icon;
                    if (update.Diff.Folder != null) topo.Folder = update.Diff.Folder;
                    if (update.Diff.Types != null)
                    {
                        topo.Types ??= new List<Type>();
                        ApplyTypesDiff(topo.Types, update.Diff.Types);
                    }
                    if (update.Diff.Designs != null)
                    {
                        topo.Designs ??= new List<Design>();
                        ApplyDesignsDiff(topo.Designs, update.Diff.Designs);
                    }
                }
            }
        }

        if (diff.Added != null)
            typologies.AddRange(diff.Added);
    }

    private static void ApplyTypesDiff(List<Type> types, TypesDiff diff)
    {
        if (diff.Removed != null)
            types.RemoveAll(t => diff.Removed.Any(r => r.Id == t.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var type = types.FirstOrDefault(t => t.Id == update.Type.Id);
                if (type != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) type.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) type.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) type.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeImage()) type.Image = update.Diff.Image;
                    if (update.Diff.ShouldSerializeParent()) type.Parent = update.Diff.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) type.IsAbstract = update.Diff.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) type.Folder = update.Diff.Folder;
                    if (update.Diff.ShouldSerializeStock()) type.Stock = update.Diff.Stock ?? type.Stock;
                    if (update.Diff.ShouldSerializeVirtual()) type.Virtual = update.Diff.Virtual ?? type.Virtual;
                    if (update.Diff.ShouldSerializeUnit()) type.Unit = update.Diff.Unit;
                    if (update.Diff.ShouldSerializeLocation()) type.Location = update.Diff.Location;
                    if (update.Diff.ShouldSerializeAuthors()) type.Authors = update.Diff.Authors?.Select(a => new AuthorId { Id = a.Id }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) type.Concepts = update.Diff.Concepts?.Select(c => new ConceptId { Id = c.Id }).ToList();
                    if (update.Diff.Connectors != null)
                    {
                        type.Connectors ??= new List<Connector>();
                        ApplyConnectorsDiff(type.Connectors, update.Diff.Connectors);
                    }
                    if (update.Diff.Representations != null)
                    {
                        type.Representations ??= new List<Representation>();
                        ApplyRepresentationsDiff(type.Representations, update.Diff.Representations);
                    }
                    if (update.Diff.Attributes != null)
                    {
                        type.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(type.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            types.AddRange(diff.Added);
    }

    private static void ApplyConnectorsDiff(List<Connector> connectors, ConnectorsDiff diff)
    {
        if (diff.Removed != null)
            connectors.RemoveAll(c => diff.Removed.Any(r => r.Id == c.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var connector = connectors.FirstOrDefault(c => c.Id == update.Connector.Id);
                if (connector != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) connector.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) connector.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializePort()) connector.Port = update.Diff.Port;
                    if (update.Diff.ShouldSerializeMandatory()) connector.Mandatory = update.Diff.Mandatory ?? connector.Mandatory;
                    if (update.Diff.ShouldSerializeT()) connector.T = update.Diff.T ?? connector.T;
                    if (update.Diff.ShouldSerializePoint())
                    {
                        var pd = update.Diff.Point;
                        var bp = connector.Point ?? new Point();
                        connector.Point = new Point { X = bp.X + (pd?.X ?? 0), Y = bp.Y + (pd?.Y ?? 0), Z = bp.Z + (pd?.Z ?? 0) };
                    }
                    if (update.Diff.ShouldSerializeDirection())
                    {
                        var dd = update.Diff.Direction;
                        var bd = connector.Direction ?? new Vector();
                        connector.Direction = new Vector { X = bd.X + (dd?.X ?? 0), Y = bd.Y + (dd?.Y ?? 0), Z = bd.Z + (dd?.Z ?? 0) };
                    }
                    if (update.Diff.Attributes != null)
                    {
                        connector.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(connector.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            connectors.AddRange(diff.Added);
    }

    private static void ApplyRepresentationsDiff(List<Representation> representations, RepresentationsDiff diff)
    {
        if (diff.Removed != null)
            representations.RemoveAll(m => diff.Removed.Any(r => r.Id == m.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var representation = representations.FirstOrDefault(m => m.Id == update.Representation.Id);
                if (representation != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) representation.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) representation.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeFile()) representation.File = update.Diff.File;
                    if (update.Diff.ShouldSerializeTags()) representation.Tags = update.Diff.Tags;
                    if (update.Diff.Attributes != null)
                    {
                        representation.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(representation.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            representations.AddRange(diff.Added);
    }

    private static void ApplyDesignsDiff(List<Design> designs, DesignsDiff diff)
    {
        if (diff.Removed != null)
            designs.RemoveAll(d => diff.Removed.Any(r => r.Id == d.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var design = designs.FirstOrDefault(d => d.Id == update.Design.Id);
                if (design != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) design.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) design.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) design.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeImage()) design.Image = update.Diff.Image;
                    if (update.Diff.ShouldSerializeParent()) design.Parent = update.Diff.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) design.IsAbstract = update.Diff.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) design.Folder = update.Diff.Folder;
                    if (update.Diff.ShouldSerializeCanScale()) design.CanScale = update.Diff.CanScale;
                    if (update.Diff.ShouldSerializeCanMirror()) design.CanMirror = update.Diff.CanMirror;
                    if (update.Diff.ShouldSerializeUnit()) design.Unit = update.Diff.Unit;
                    if (update.Diff.ShouldSerializeActiveLayer()) design.ActiveLayer = update.Diff.ActiveLayer;
                    if (update.Diff.ShouldSerializeLocation()) design.Location = update.Diff.Location;
                    if (update.Diff.ShouldSerializeAuthors()) design.Authors = update.Diff.Authors?.Select(a => new AuthorId { Id = a.Id }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) design.Concepts = update.Diff.Concepts?.Select(c => new ConceptId { Id = c.Id }).ToList();
                    if (update.Diff.Pieces != null)
                    {
                        design.Pieces ??= new List<Piece>();
                        ApplyPiecesDiff(design.Pieces, update.Diff.Pieces);
                    }
                    if (update.Diff.Connections != null)
                    {
                        design.Connections ??= new List<Connection>();
                        ApplyConnectionsDiff(design.Connections, update.Diff.Connections);
                    }
                    if (update.Diff.Attributes != null)
                    {
                        design.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(design.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            designs.AddRange(diff.Added);
    }

    private static void ApplyPiecesDiff(List<Piece> pieces, PiecesDiff diff)
    {
        if (diff.Removed != null)
            pieces.RemoveAll(p => diff.Removed.Any(r => r.Id == p.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var piece = pieces.FirstOrDefault(p => p.Id == update.Piece.Id);
                if (piece != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) piece.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) piece.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeType()) piece.Type = update.Diff.Type;
                    if (update.Diff.ShouldSerializeDesign()) piece.Design = update.Diff.Design;
                    if (update.Diff.ShouldSerializePlane()) piece.Plane = update.Diff.Plane;
                    if (update.Diff.ShouldSerializeCenter()) piece.Center = update.Diff.Center;
                    if (update.Diff.ShouldSerializeScale()) piece.Scale = update.Diff.Scale;
                    if (update.Diff.ShouldSerializeMirrorPlane()) piece.MirrorPlane = update.Diff.MirrorPlane;
                    if (update.Diff.ShouldSerializeIsHidden()) piece.IsHidden = update.Diff.IsHidden;
                    if (update.Diff.ShouldSerializeIsLocked()) piece.IsLocked = update.Diff.IsLocked;
                    if (update.Diff.ShouldSerializeColor()) piece.Color = update.Diff.Color;
                    if (update.Diff.Attributes != null)
                    {
                        piece.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(piece.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            pieces.AddRange(diff.Added);
    }

    private static void ApplyConnectionsDiff(List<Connection> connections, ConnectionsDiff diff)
    {
        if (diff.Removed != null)
            connections.RemoveAll(c => diff.Removed.Any(r => r.Id == c.Id));

        if (diff.Modified != null)
        {
            foreach (var update in diff.Modified)
            {
                var connection = connections.FirstOrDefault(c => c.Id == update.Connection.Id);
                if (connection != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeParent() && update.Diff.Parent != null)
                    {
                        var s = connection.Parent ?? new Side();
                        if (update.Diff.Parent.ShouldSerializePiece()) s.Piece = update.Diff.Parent.Piece;
                        if (update.Diff.Parent.ShouldSerializeDesignPiece()) s.DesignPiece = update.Diff.Parent.DesignPiece;
                        if (update.Diff.Parent.ShouldSerializeConnector()) s.Connector = update.Diff.Parent.Connector;
                        connection.Parent = s;
                    }
                    if (update.Diff.ShouldSerializeChild() && update.Diff.Child != null)
                    {
                        var s = connection.Child ?? new Side();
                        if (update.Diff.Child.ShouldSerializePiece()) s.Piece = update.Diff.Child.Piece;
                        if (update.Diff.Child.ShouldSerializeDesignPiece()) s.DesignPiece = update.Diff.Child.DesignPiece;
                        if (update.Diff.Child.ShouldSerializeConnector()) s.Connector = update.Diff.Child.Connector;
                        connection.Child = s;
                    }
                    if (update.Diff.ShouldSerializeDescription()) connection.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeGap()) connection.Gap = connection.Gap + (update.Diff.Gap ?? 0f);
                    if (update.Diff.ShouldSerializeShift()) connection.Shift = connection.Shift + (update.Diff.Shift ?? 0f);
                    if (update.Diff.ShouldSerializeRise()) connection.Rise = connection.Rise + (update.Diff.Rise ?? 0f);
                    if (update.Diff.ShouldSerializeRotation()) connection.Rotation = connection.Rotation + (update.Diff.Rotation ?? 0f);
                    if (update.Diff.ShouldSerializeTurn()) connection.Turn = connection.Turn + (update.Diff.Turn ?? 0f);
                    if (update.Diff.ShouldSerializeTilt()) connection.Tilt = connection.Tilt + (update.Diff.Tilt ?? 0f);
                    if (update.Diff.ShouldSerializeU()) connection.U = (connection.U ?? 0f) + (update.Diff.U ?? 0f);
                    if (update.Diff.ShouldSerializeV()) connection.V = (connection.V ?? 0f) + (update.Diff.V ?? 0f);
                    if (update.Diff.Attributes != null)
                    {
                        connection.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(connection.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            connections.AddRange(diff.Added);
    }
}
//#endregion 🔧KitInPlaceDiff


//#region 📥KitState
/// <summary>Replace all fields of an existing <see cref="Kit"/> instance in-place (used by transport sync without computing a <see cref="KitDiff"/>).</summary>
public static class KitState
{
    public static void ReplaceInPlace(Kit target, Kit source)
    {
        target.Id = source.Id;
        target.Name = source.Name;
        target.Version = source.Version;
        target.Description = source.Description;
        target.Icon = source.Icon;
        target.Image = source.Image;
        target.Concepts = source.Concepts;
        target.Tags = source.Tags;
        target.Remote = source.Remote;
        target.Homepage = source.Homepage;
        target.License = source.License;
        target.Authors = source.Authors;
        target.Pieces = source.Pieces;
        target.Groups = source.Groups;
        target.Connections = source.Connections;
        target.Props = source.Props;
        target.Stats = source.Stats;
        target.Attributes = source.Attributes;
        target.Preview = source.Preview;
        target.Qualities = source.Qualities;
        target.Ports = source.Ports;
        target.Files = source.Files;
        target.Folders = source.Folders;
        target.Types = source.Types;
        target.Designs = source.Designs;
        target.CreatedAt = source.CreatedAt;
        target.ModificationdAt = source.ModificationdAt;
    }
}
//#endregion 📥KitState


//#region 🗄️Store
namespace Store
{
    //#region Events
/// <summary>📡 Command notifications for a {@link StoreSession} (mirrors compose/js {@code EventBus} command kinds).</summary>
public sealed class StoreEventBus
{
    private readonly Dictionary<string, List<Action>> _kindHandlers = new(StringComparer.Ordinal);

    /// <summary>📡 Fires after any kit command mutation succeeds.</summary>
    public event Action? CommandSucceeded;

    public void SubscribeKind(string kind, Action handler)
    {
        if (!_kindHandlers.TryGetValue(kind, out var list))
        {
            list = new List<Action>();
            _kindHandlers[kind] = list;
        }
        if (!list.Contains(handler))
            list.Add(handler);
    }

    public void UnsubscribeKind(string kind, Action handler)
    {
        if (_kindHandlers.TryGetValue(kind, out var list))
            list.Remove(handler);
    }

    internal void PublishKind(string kind)
    {
        if (!_kindHandlers.TryGetValue(kind, out var list)) return;
        foreach (var h in list.ToArray())
            h();
    }

    internal void AfterCommand(string? fieldEventKind = null)
    {
        CommandSucceeded?.Invoke();
        PublishKind("commandSucceeded");
        if (!string.IsNullOrEmpty(fieldEventKind))
            PublishKind(fieldEventKind);
    }
}

/// <summary>📡 Subscribes {@link StoreSession} bus events.</summary>
public static class StoreEventBridge
{
    public static void SubscribeCommandSucceeded(StoreSession session, Action handler) =>
        session.Events.CommandSucceeded += handler;
}
    //#endregion Events

    //#region StoreGraphqlSelection
/// <summary>📬 Kit command selection helpers — mirrors <c>compose/js/graphql-kit-selection.ts</c>.</summary>
internal static class StoreGraphqlSelection
{
    internal static string WithResponse(string kitSelection, string responseSelection)
    {
        var trimmed = kitSelection.Trim();
        var open = trimmed.IndexOf('{');
        if (open < 0) return AppendResponseAfterArgs(trimmed, responseSelection);
        var close = FindMatchingCloseBrace(trimmed, open);
        if (close < 0) return AppendResponseAfterArgs(trimmed, responseSelection);
        var head = trimmed[..open].TrimEnd();
        var inner = trimmed[(open + 1)..close].Trim();
        var tail = trimmed[(close + 1)..].Trim();
        var result = $"{head} {{ {TransformKitSelectionBlock(inner, responseSelection)} }}";
        return tail.Length == 0 ? result : $"{result} {WithResponse(tail, responseSelection)}";
    }

    static int FindMatchingCloseBrace(string s, int openIdx)
    {
        if (openIdx < 0 || openIdx >= s.Length || s[openIdx] != '{') return -1;
        var depth = 0;
        var inString = false;
        var escape = false;
        for (var i = openIdx; i < s.Length; i++)
        {
            var ch = s[i];
            if (inString)
            {
                if (escape) { escape = false; continue; }
                if (ch == '\\') { escape = true; continue; }
                if (ch == '"') inString = false;
                continue;
            }
            if (ch == '"') { inString = true; continue; }
            if (ch == '{') depth++;
            else if (ch == '}')
            {
                depth--;
                if (depth == 0) return i;
            }
        }
        return -1;
    }

    static int LastArgListCloseParen(string s)
    {
        var depth = 0;
        var inString = false;
        var escape = false;
        var last = -1;
        for (var i = 0; i < s.Length; i++)
        {
            var ch = s[i];
            if (inString)
            {
                if (escape) { escape = false; continue; }
                if (ch == '\\') { escape = true; continue; }
                if (ch == '"') inString = false;
                continue;
            }
            if (ch == '"') { inString = true; continue; }
            if (ch == '(') depth++;
            else if (ch == ')')
            {
                depth--;
                if (depth == 0) last = i;
            }
        }
        return last;
    }

    static bool HasTopLevelSelectionBrace(string s)
    {
        var paren = 0;
        var inString = false;
        var escape = false;
        for (var i = 0; i < s.Length; i++)
        {
            var ch = s[i];
            if (inString)
            {
                if (escape) { escape = false; continue; }
                if (ch == '\\') { escape = true; continue; }
                if (ch == '"') inString = false;
                continue;
            }
            if (ch == '"') { inString = true; continue; }
            if (ch == '(') paren++;
            else if (ch == ')') paren--;
            else if (ch == '{' && paren == 0) return true;
        }
        return false;
    }

    static string AppendResponseAfterArgs(string fieldWithArgs, string responseSelection)
    {
        var t = fieldWithArgs.Trim();
        if (t.Contains(responseSelection, StringComparison.Ordinal)) return t;
        var closeParen = LastArgListCloseParen(t);
        if (closeParen < 0) return $"{t} {{ {responseSelection} }}";
        var after = t[(closeParen + 1)..].Trim();
        if (after.StartsWith("{", StringComparison.Ordinal))
        {
            var open = closeParen + 1 + t[(closeParen + 1)..].IndexOf('{');
            var close = FindMatchingCloseBrace(t, open);
            if (close < 0) return $"{t} {{ {responseSelection} }}";
            var head = t[..open].TrimEnd();
            var inner = t[(open + 1)..close].Trim();
            var tail = t[(close + 1)..].Trim();
            return $"{head} {{ {TransformKitSelectionBlock(inner, responseSelection)} }}{(tail.Length == 0 ? "" : " " + tail)}";
        }
        return $"{t[..(closeParen + 1)]} {{ {responseSelection} }}";
    }

    static string TransformKitSelectionBlock(string inner, string responseSelection) =>
        !HasTopLevelSelectionBrace(inner) ? AppendResponseAfterArgs(inner, responseSelection) : WithResponse(inner, responseSelection);
}
    //#endregion StoreGraphqlSelection

    //#region StoreGraphql
//#region 🌐Wire
/// <summary>🌐 GraphQL-over-HTTP wire helpers aligned with <c>compose/js</c> (<c>graphqlWirePostBodyJson</c>, operation-kind guard).</summary>
public static class StoreGraphqlWire
{
    /// <summary>🧵 Canonical POST body: <c>query</c>, <c>variables</c>, <c>operationName</c> always present.</summary>
    public static string PostBodyJson(string query, JObject? variables = null, string? operationName = null) =>
        ComposeJson.Codec.Serialize(new JObject
        {
            ["query"] = query,
            ["variables"] = variables ?? new JObject(),
            ["operationName"] = operationName == null ? JValue.CreateNull() : operationName,
        });

    /// <summary>🛑 Enforces golden-schema split: <c>Query</c> vs <c>Mutation</c> roots only.</summary>
    public static void AssertOperationKind(string document, string kind)
    {
        var rest = document.TrimStart();
        for (; ; )
        {
            if (rest.StartsWith("#"))
            {
                var nl = rest.IndexOf('\n');
                if (nl < 0) throw new IOException($"graphql: expected {kind}, got unknown");
                rest = rest[(nl + 1)..].TrimStart();
                continue;
            }
            break;
        }
        var head = rest.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries).FirstOrDefault() ?? "";
        var operation = head.Split('(')[0];
        if (!string.Equals(operator, kind, StringComparison.OrdinalIgnoreCase))
            throw new IOException($"graphql: expected {kind}, got {head}");
    }

    /// <summary>📬 Unwraps <c>data</c> or throws on GraphQL <c>errors</c>.</summary>
    public static JToken UnwrapData(JToken response)
    {
        if (response is not JObject o) throw new IOException("graphql: response is not an object");
        if (o["errors"] is JArray errs && errs.Count > 0)
        {
            var msg = errs[0]?["message"]?.Value<string>() ?? "GraphQL error";
            throw new IOException("graphql: " + msg);
        }
        var data = o["data"];
        if (data == null || data.Type == JTokenType.Null) throw new IOException("graphql: no data in response");
        return data;
    }
}
//#endregion 🌐Wire

//#region 🌐Documents
/// <summary>🌐 Golden-schema GraphQL documents aligned with <c>compose/js</c> (<c>schema.golden.graphql</c>).</summary>
public static class StoreGraphql
{
    /// <summary>📬 <c>Response</c> selection on command mutation leaves.</summary>
    public const string ResponseSelection =
        "ok errors { kind message requestId } result { ... on IdResult { value } }";

    /// <summary>🧭 Store entry query — mirrors <c>KIT_SESSION_QUERY_ENTRY</c> in compose/js.</summary>
    public const string KitSessionQueryEntry =
        "query KitStoreEntry { session { stores { edges { node { wip { id theKit { id } } } } } } }";

    /// <summary>🧵 GraphQL string literal for variables.</summary>
    public static string GqlString(string s) => ComposeJson.Codec.Serialize(s);

    /// <summary>🧵 GraphQL ID list literal.</summary>
    public static string GqlIdList(IEnumerable<string> ids) =>
        "[" + string.Join(", ", ids.Select(GqlString)) + "]";

    /// <summary>📖 <c>session.stores → wip.theKit.kit</c> read (theKit head).</summary>
    public static string KitSessionWipKitQuery(string kitSelection) =>
        $"query KitSessionWipStore {{ session {{ stores {{ edges {{ cursor node {{ wip {{ theKit {{ kit {{ {kitSelection} }} }} }} }} }} }} }} }}";

    /// <summary>📖 <c>session.stores → node</c> store branch read.</summary>
    public static string SessionStoreNodeQuery(string innerOnStore) =>
        $"query Stores {{ session {{ stores {{ edges {{ cursor node {{ {innerOnStore} }} }} }} }} }}";

    /// <summary>📖 WIP materialization: <c>initialKit</c>, <c>theKit.kit</c>, checkpoint anchors.</summary>
    public static string KitWipMaterializationQuery() =>
        @"query KitMaterialization {
  session {
    stores {
      edges {
        node {
          wip {
            initialKit { name }
            theKit {
              kit {
                name
                designs { edges { node { name } } }
                types { edges { node { name } } }
              }
            }
            checkpoints {
              edges {
                node {
                  initial { name }
                  kit { name }
                }
              }
            }
          }
        }
      }
    }
  }
}";

    public static string SessionStoresCursorsQuery() =>
        "query SessionStoreCursors { session { stores { edges { cursor } } } }";

    public static string SessionStartMutation() =>
        $"mutation SessionStart {{ session {{ start {{ {ResponseSelection} }} }} }}";

    public static string SessionEndMutation() =>
        $"mutation SessionEnd {{ session {{ end {{ {ResponseSelection} }} }} }}";

    public static string SessionStoreStartNewChangeMutation() =>
        $"mutation($storeId: ID!) {{ session {{ store(id: $storeId) {{ theKit {{ startNewChange {{ {ResponseSelection} }} }} }} }} }}";

    /// <summary>✍️ <c>session.store → theKit → unsavedChange → kit</c> scoped mutation.</summary>
    public static (string Query, JObject Variables) ScopedKitMutation(string storeId, string changeId, string kitSelection)
    {
        var inner = WithResponseSelection(kitSelection);
        return (
            $"mutation($storeId: ID!, $changeId: ID!) {{ session {{ store(id: $storeId) {{ theKit {{ unsavedChange(id: $changeId) {{ kit {{ {inner} }} }} }} }} }} }}",
            new JObject { ["storeId"] = storeId, ["changeId"] = changeId });
    }

    /// <summary>✍️ <c>kit.rename</c> on the open unsaved change.</summary>
    public static (string Query, JObject Variables) RenameKitMutation(string storeId, string changeId, string newName) =>
        ScopedKitMutation(storeId, changeId, $"rename(newName: {GqlString(newName)})");

    /// <summary>📬 Appends {@link ResponseSelection} to the innermost kit command field (mirrors compose/js <c>withResponseSelection</c>).</summary>
    public static string WithResponseSelection(string kitSelection) =>
        StoreGraphqlSelection.WithResponse(kitSelection, ResponseSelection);
}
//#endregion 🌐Documents

//#region 🌐JsonPaths
/// <summary>🧩 JSON-path helpers for golden <c>session → stores → wip → theKit → kit</c> reads.</summary>
public static class StoreGraphqlJson
{
    public static string WipPath(int storeIndex = 0) => $"session.stores.edges[{storeIndex}].node.wip";

    public static string? DefaultStoreCursor(JToken data, int storeIndex = 0) =>
        data.SelectToken($"session.stores.edges[{storeIndex}].cursor")?.Value<string>();

    public static JObject? SessionStoreNode(JToken data, string? storeCursor = null, int storeIndex = 0)
    {
        if (!string.IsNullOrEmpty(storeCursor))
        {
            var edges = data.SelectToken("session.stores.edges") as JArray;
            if (edges != null)
            {
                foreach (var e in edges)
                {
                    if (e is not JObject edge) continue;
                    if (edge["cursor"]?.Value<string>() == storeCursor)
                        return edge["node"] as JObject;
                }
            }
            return null;
        }
        return data.SelectToken($"session.stores.edges[{storeIndex}].node") as JObject;
    }

    public static JObject? WipBranch(JToken data, string? storeCursor = null, int storeIndex = 0) =>
        SessionStoreNode(data, storeCursor, storeIndex)?["wip"] as JObject;

    public static JObject? WipTheKitKit(JToken data, string? storeCursor = null, int storeIndex = 0) =>
        WipBranch(data, storeCursor, storeIndex)?["theKit"]?["kit"] as JObject;

    public static JToken? WipTheKitKitScalar(JToken data, string field, string? storeCursor = null, int storeIndex = 0) =>
        WipTheKitKit(data, storeCursor, storeIndex)?[field];

    public static JToken? WipInitialKitScalar(JToken data, string field, int storeIndex = 0) =>
        data.SelectToken($"{WipPath(storeIndex)}.initialKit.{field}");

    public static string? StartNewChangeId(JToken mutationData) =>
        mutationData.SelectToken("session.store.theKit.startNewChange.result.value")?.Value<string>();

    public static string? ResponseResultId(JToken? responseNode) =>
        responseNode?["result"]?["value"]?.Value<string>();

    /// <summary>📬 Throws when <c>Response.ok</c> is false.</summary>
    public static void AssertResponseOk(JToken? responseNode, string label)
    {
        if (responseNode is not JObject o) return;
        if (o["ok"]?.Value<bool>() == false)
        {
            var msg = o["errors"]?["message"]?.Value<string>() ?? "command failed";
            throw new IOException($"graphql: {label}: {msg}");
        }
    }

    /// <summary>📬 Finds the first <c>Response</c> node under <c>unsavedChange.kit</c> (supports aliased design commands).</summary>
    public static JToken? FindKitCommandResponse(JToken mutationData)
    {
        var kit = mutationData.SelectToken("session.store.theKit.unsavedChange.kit");
        return kit == null ? null : FindResponsePayload(kit);
    }

    static JToken? FindResponsePayload(JToken node)
    {
        if (node is JObject o && o.ContainsKey("ok")) return o;
        if (node is not JContainer container) return null;
        foreach (var child in container.Children())
        {
            var hit = FindResponsePayload(child);
            if (hit != null) return hit;
        }
        return null;
    }
}
//#endregion 🌐JsonPaths
    //#endregion StoreGraphql

    //#region StoreClient
/// <summary>🌐 Thin HTTP GraphQL client to <c>compose-gql</c> (<c>POST /install</c>, <c>POST /graphql</c>); same wire as <c>compose/js</c> <see cref="StoreSession.OpenHttp"/>.</summary>
public sealed class StoreClient : IDisposable
{
    private readonly string _binaryPath;
    private readonly HttpClient _http;
    private Process? _process;
    private string? _baseUrl;

    public StoreClient(string? binaryPath = null, string? baseUrl = null)
    {
        _binaryPath = string.IsNullOrWhiteSpace(binaryPath)
            ? StorePaths.ResolveStoreBinary()
            : binaryPath.Trim();
        _http = new HttpClient { Timeout = TimeSpan.FromMinutes(5) };
        if (!string.IsNullOrWhiteSpace(baseUrl))
            _baseUrl = baseUrl.TrimEnd('/');
    }

    public void Start()
    {
        if (_baseUrl != null) return;
        if (_process != null) return;
        if (!System.IO.File.Exists(_binaryPath))
            throw new FileNotFoundException("compose-gql binary not found", _binaryPath);

        var port = AllocateFreeTcpPort();
        var psi = new ProcessStartInfo
        {
            FileName = _binaryPath,
            UseShellExecute = false,
            RedirectStandardInput = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true,
        };
        psi.Environment["COMPOSE_GQL_PORT"] = port.ToString();
        var rl = Environment.GetEnvironmentVariable("RUST_LOG");
        psi.Environment["RUST_LOG"] = string.IsNullOrEmpty(rl) ? "error" : rl!;

        _process = Process.Start(psi) ?? throw new IOException("compose-gql: start failed");
        _baseUrl = ReadReadyBaseUrl(_process, port);
    }

    private static int AllocateFreeTcpPort()
    {
        var listener = new TcpListener(IPAddress.Loopback, 0);
        listener.Start();
        try
        {
            return ((IPEndPoint)listener.LocalEndpoint).Port;
        }
        finally
        {
            listener.Stop();
        }
    }

    private static string ReadReadyBaseUrl(Process process, int fallbackPort)
    {
        var deadline = DateTime.UtcNow.AddSeconds(60);
        while (DateTime.UtcNow < deadline)
        {
            if (process.HasExited)
                throw new IOException("compose-gql exited before ready");
            var line = process.StandardOutput.ReadLine();
            if (line == null)
            {
                Thread.Sleep(10);
                continue;
            }
            if (line.Length == 0) continue;
            try
            {
                var o = ComposeJson.Codec.ParseJsonRoot(line) as JObject;
                if (o["composeGqlReady"]?.Value<bool>() == true)
                {
                    var port = o["port"]?.Value<int?>() ?? fallbackPort;
                    return $"http://127.0.0.1:{port}";
                }
            }
            catch (JsonException)
            {
                /* not the ready line */
            }
        }
        throw new TimeoutException("compose-gql: ready line timeout");
    }

    private string BaseUrl
    {
        get
        {
            Start();
            return _baseUrl ?? throw new InvalidOperationException("compose-gql: no base url");
        }
    }

    /// <summary>📦 <c>POST /install</c> — exactly one install field per <c>compose-gql</c>.</summary>
    internal void Install(JObject body)
    {
        var json = body.ToString(Formatting.None);
        using var content = new StringContent(json, Encoding.UTF8, "application/json");
        var r = _http.PostAsync($"{BaseUrl}/install", content).GetAwaiter().GetResult();
        var t = r.Content.ReadAsStringAsync().GetAwaiter().GetResult();
        if (!r.IsSuccessStatusCode)
            throw new IOException($"compose-gql install {(int)r.StatusCode}: {t}");
        WarmGraphqlSession();
    }

    /// <summary>🧾 Warm-path after install — <c>session.start</c> + store cursor probe (compose/js <c>warmGraphqlRead</c>).</summary>
    internal void WarmGraphqlSession()
    {
        try
        {
            ExecuteMutation(StoreGraphql.SessionStartMutation());
        }
        catch
        {
            /* session may already be started */
        }
        _ = ExecuteQuery(StoreGraphql.SessionStoresCursorsQuery());
    }

    /// <summary>🪢 First <c>Session.stores.edges[].cursor</c> (store command scope id, typically <c>e0</c>).</summary>
    internal string DefaultStoreId()
    {
        var data = ExecuteQuery(StoreGraphql.SessionStoresCursorsQuery());
        var id = StoreGraphqlJson.DefaultStoreCursor(data);
        if (string.IsNullOrEmpty(id))
            throw new IOException("graphql: no session store cursor");
        return id;
    }

    /// <summary>📖 <c>POST /graphql</c> query root (<c>type Query</c>).</summary>
    internal JToken ExecuteQuery(string query, JObject? variables = null, string? operationName = null)
    {
        StoreGraphqlWire.AssertOperationKind(query, "query");
        return StoreGraphqlWire.UnwrapData(PostGraphql(StoreGraphqlWire.PostBodyJson(query, variables, operationName)));
    }

    /// <summary>✍️ <c>POST /graphql</c> mutation root (<c>type Mutation</c>).</summary>
    internal JToken ExecuteMutation(string query, JObject? variables = null, string? operationName = null)
    {
        StoreGraphqlWire.AssertOperationKind(query, "mutation");
        return StoreGraphqlWire.UnwrapData(PostGraphql(StoreGraphqlWire.PostBodyJson(query, variables, operationName)));
    }

    private JToken PostGraphql(string requestJson)
    {
        using var content = new StringContent(requestJson, Encoding.UTF8, "application/json");
        using var req = new HttpRequestMessage(HttpMethod.Post, $"{BaseUrl}/graphql") { Content = content };
        req.Headers.TryAddWithoutValidation("Accept", "application/json");
        var r = _http.SendAsync(req).GetAwaiter().GetResult();
        var t = r.Content.ReadAsStringAsync().GetAwaiter().GetResult();
        if (!r.IsSuccessStatusCode)
            throw new IOException($"graphql http {(int)r.StatusCode}: {t}");
        return JToken.Parse(t);
    }

    public void Dispose()
    {
        try
        {
            if (_baseUrl != null)
            {
                using var _ = _http.PostAsync($"{_baseUrl}/server/shutdown", null).GetAwaiter().GetResult();
            }
        }
        catch { }

        try
        {
            if (_process?.HasExited == false)
                _process.WaitForExit(2000);
        }
        catch { }

        _process?.Dispose();
        _process = null;
        _http.Dispose();
    }
}

/// <summary>🧭 Thin GraphQL session over <see cref="StoreClient" /> (aligned with compose/js <c>Session</c> store surface).</summary>
public sealed class StoreSession : IDisposable
{
    private readonly StoreClient _client;
    private string? _storeId;
    private string? _activeChangeId;
    private WipKit? _kit;

    public StoreSession(StoreClient client)
    {
        _client = client;
        Events = new StoreEventBus();
    }

    /// <summary>📡 Command and field-change bus for this session.</summary>
    public StoreEventBus Events { get; }

    /// <summary>📦 WIP kit under <c>session.stores → wip.theKit.kit</c>.</summary>
    public WipKit Kit => _kit ??= new WipKit(this);

    /// <summary>🌐 Opens against an existing <c>compose-gql</c> base URL (optional install-create first).</summary>
    public static StoreSession OpenHttp(string baseUrl, JObject? installCreateDto = null)
    {
        var c = new StoreClient(baseUrl: baseUrl);
        var session = new StoreSession(c);
        if (installCreateDto != null)
            session.InstallCreate(installCreateDto);
        else
            c.WarmGraphqlSession();
        return session;
    }

    /// <summary>🪢 Store command scope id (<c>Session.stores.edges[].cursor</c>, typically <c>e0</c>).</summary>
    public string StoreId => _storeId ??= _client.DefaultStoreId();

    //#region 🎬 install commands
    /// <summary>📦 <c>POST /install</c> with <c>create.dto</c>.</summary>
    public void InstallCreate(JObject dto) =>
        _client.Install(new JObject { ["create"] = new JObject { ["dto"] = dto } });

    /// <summary>📥 <c>POST /install</c> with normalized initial-kit projection (<see cref="StoreKitIO.KitToInstallProjection"/>).</summary>
    public void InstallProjection(Kit kit) =>
        InstallCreate(StoreKitIO.KitToInstallProjection(kit));

    /// <summary>📦 <c>POST /install</c> with <c>importFile.path</c>.</summary>
    public void InstallImportFile(string path) =>
        _client.Install(new JObject { ["importFile"] = new JObject { ["path"] = Path.GetFullPath(path) } });
    //#endregion 🎬 install commands

    //#region 🎬 session commands
    /// <summary>🎬 <c>session.store.theKit.startNewChange</c>.</summary>
    public string StartNewChange()
    {
        _activeChangeId = MutateStartNewChange();
        Events.AfterCommand();
        return _activeChangeId;
    }

    internal string EnsureChangeId() => _activeChangeId ??= MutateStartNewChange();
    //#endregion 🎬 session commands

    private string MutateStartNewChange()
    {
        var data = _client.ExecuteMutation(
            StoreGraphql.SessionStoreStartNewChangeMutation(),
            new JObject { ["storeId"] = StoreId });
        var node = data.SelectToken("session.store.theKit.startNewChange");
        StoreGraphqlJson.AssertResponseOk(node, "startNewChange");
        var changeId = StoreGraphqlJson.ResponseResultId(node as JObject);
        if (string.IsNullOrEmpty(changeId))
            throw new IOException("graphql: startNewChange returned empty change id");
        return changeId;
    }

    internal JToken ReadSessionEntry() =>
        _client.ExecuteQuery(StoreGraphql.KitSessionQueryEntry);

    internal JToken ReadKitSelection(string selection) =>
        _client.ExecuteQuery(StoreGraphql.KitSessionWipKitQuery(selection));

    internal JObject? ReadKitObject(string selection) =>
        StoreGraphqlJson.WipTheKitKit(ReadKitSelection(selection), StoreId);

    internal JToken ReadMaterialization() =>
        _client.ExecuteQuery(StoreGraphql.KitWipMaterializationQuery());

    internal void RunKitMutation(string changeId, string kitSelection, string? fieldEventKind)
    {
        var (query, variables) = StoreGraphql.ScopedKitMutation(StoreId, changeId, kitSelection);
        var data = _client.ExecuteMutation(query, variables);
        var operation = kitSelection.Trim().Split('(')[0].Trim();
        var node = StoreGraphqlJson.FindKitCommandResponse(data)
            ?? data.SelectToken($"session.store.theKit.unsavedChange.kit.{operation}");
        StoreGraphqlJson.AssertResponseOk(node, operator);
        Events.AfterCommand(fieldEventKind);
    }

    public void Dispose() => _client.Dispose();
}

public static class StorePaths
{
    public static string ResolveStoreBinary()
    {
        var env = Environment.GetEnvironmentVariable("COMPOSE_GQL_BIN");
        if (!string.IsNullOrWhiteSpace(env) && System.IO.File.Exists(env)) return env!.Trim();
        var nextTo = Path.Combine(AppContext.BaseDirectory, "compose-gql.exe");
        if (System.IO.File.Exists(nextTo)) return nextTo;
        var nextToNix = Path.Combine(AppContext.BaseDirectory, "compose-gql");
        if (System.IO.File.Exists(nextToNix)) return nextToNix;
        for (var here = new DirectoryInfo(AppContext.BaseDirectory); here != null; here = here.Parent)
        {
            var win = Path.Combine(here.FullName, "target", "release", "compose-gql.exe");
            if (System.IO.File.Exists(win)) return win;
            var unix = Path.Combine(here.FullName, "target", "release", "compose-gql");
            if (System.IO.File.Exists(unix)) return unix;
        }
        if (System.IO.File.Exists("compose-gql.exe")) return "compose-gql.exe";
        if (System.IO.File.Exists("compose-gql")) return "compose-gql";
        return "compose-gql";
    }
}
    //#endregion StoreClient

    //#region StoreKit
/// <summary>📦 WIP {@code theKit.kit} handle: getters for reads, methods for commands, events for field changes (compose/js {@link Kit}).</summary>
public sealed class WipKit
{
    private const string KitObjectSelection =
        "id name description icon image types { edges { node { id name } } } designs { edges { node { id name } } }";

    private readonly StoreSession _session;

    internal WipKit(StoreSession session) => _session = session;

    //#region 📖 getters
    /// <summary>📖 Golden <c>wip.theKit.kit.id</c>.</summary>
    public string Id => GetScalar("id") ?? throw new IOException("graphql: missing kit.id");

    /// <summary>📖 Golden <c>wip.theKit.kit.name</c>.</summary>
    public string Name => GetScalar("name") ?? "";

    /// <summary>📖 Golden <c>wip.theKit.kit.description</c>.</summary>
    public string Description => GetScalar("description") ?? "";

    /// <summary>📖 Materialized WIP branch (<c>initialKit</c>, <c>theKit.kit</c>, checkpoints).</summary>
    public JToken Materialization => _session.ReadMaterialization();

    /// <summary>📖 Full <c>kit { id name … }</c> object under the store cursor.</summary>
    public JObject Object =>
        _session.ReadKitObject(KitObjectSelection) ?? throw new IOException("graphql: missing wip.theKit.kit");
    //#endregion 📖 getters

    //#region 📡 field-change events
    /// <summary>📡 <c>kit.name</c> changed after a successful command.</summary>
    public event Action<string>? NameChanged;

    /// <summary>📡 <c>kit.description</c> changed after a successful command.</summary>
    public event Action<string>? DescriptionChanged;
    //#endregion 📡 field-change events

    //#region 🎬 commands
    /// <summary>🎬 <c>kit.rename</c> on the open unsaved change.</summary>
    public void Rename(string newName) =>
        RunKitCommand($"rename(newName: {StoreGraphql.GqlString(newName)})", "kitRenamed");

    /// <summary>🎬 <c>kit.changeDescription</c>.</summary>
    public void ChangeDescription(string newDescription) =>
        RunKitCommand($"changeDescription(newDescription: {StoreGraphql.GqlString(newDescription)})", "changedDescription");

    public void CreateTag(string name, string? description = null, string? icon = null, int? order = null) =>
        RunKitCommand(
            $"createTag(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, order: {GqlOpt(order)})");

    public void DeleteTag(string id) =>
        RunKitCommand($"deleteTag(id: {StoreGraphql.GqlString(id)})");

    public void DeleteTags(IEnumerable<string> ids) =>
        RunKitCommand($"deleteTags(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateConcept(string name, string? description = null, string? icon = null, int? order = null) =>
        RunKitCommand(
            $"createConcept(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, order: {GqlOpt(order)})");

    public void DeleteConcept(string id) =>
        RunKitCommand($"deleteConcept(id: {StoreGraphql.GqlString(id)})");

    public void DeleteConcepts(IEnumerable<string> ids) =>
        RunKitCommand($"deleteConcepts(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateQuality(string key, string? value = null, string? unit = null, string? definition = null, string? description = null, string? icon = null) =>
        RunKitCommand(
            $"createQuality(key: {StoreGraphql.GqlString(key)}, value: {GqlOpt(value)}, unit: {GqlOpt(unit)}, definition: {GqlOpt(definition)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)})");

    public void DeleteQuality(string id) =>
        RunKitCommand($"deleteQuality(id: {StoreGraphql.GqlString(id)})");

    public void DeleteQualities(IEnumerable<string> ids) =>
        RunKitCommand($"deleteQualities(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateType(string name, string? description = null, string? icon = null, string? image = null, string? unit = null) =>
        RunKitCommand(
            $"createType(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, image: {GqlOpt(image)}, unit: {GqlOpt(unit)})");

    public void DeleteType(string id) =>
        RunKitCommand($"deleteType(id: {StoreGraphql.GqlString(id)})");

    public void DeleteTypes(IEnumerable<string> ids) =>
        RunKitCommand($"deleteTypes(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateDesign(string name, string? description = null, string? icon = null, string? image = null, string? unit = null) =>
        RunKitCommand(
            $"createDesign(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, image: {GqlOpt(image)}, unit: {GqlOpt(unit)})");

    public void DeleteDesign(string id) =>
        RunKitCommand($"deleteDesign(id: {StoreGraphql.GqlString(id)})");

    public void DeleteDesigns(IEnumerable<string> ids) =>
        RunKitCommand($"deleteDesigns(ids: {StoreGraphql.GqlIdList(ids)})");

    /// <summary>🎬 Ensures {@code startNewChange} and returns the change id for subsequent commands.</summary>
    public string EnsureChangeId() => _session.EnsureChangeId();
    //#endregion 🎬 commands

    private string? GetScalar(string field) =>
        StoreGraphqlJson.WipTheKitKitScalar(_session.ReadKitSelection(field), field, _session.StoreId)?.Value<string>();

    private static string GqlOpt(string? s) => s == null ? "null" : StoreGraphql.GqlString(s);

    private static string GqlOpt(int? n) => n == null ? "null" : n.Value.ToString(System.Globalization.CultureInfo.InvariantCulture);

    private void RunKitCommand(string kitSelection, string? fieldEventKind = null)
    {
        var changeId = EnsureChangeId();
        _session.RunKitMutation(changeId, kitSelection, fieldEventKind);
        if (fieldEventKind == "kitRenamed")
        {
            var n = Name;
            NameChanged?.Invoke(n);
        }
        else if (fieldEventKind == "changedDescription")
        {
            var d = Description;
            DescriptionChanged?.Invoke(d);
        }
    }
}
    //#endregion StoreKit

    //#region StoreKitIO
/// <summary>📦 Load/save kits through <c>compose-gql</c> GraphQL (<c>POST /install</c> + <c>POST /graphql</c>); equality via normalized JSON compare.</summary>
public static class StoreKitIO
{
    public static JObject KitToJObject(Kit kit) => JObject.Parse(Utility.Serialize(kit));

    /// <summary>📥 <c>POST /install</c> projection JSON: design pieces use <c>pose.plane</c> + <c>pose.center</c> (<c>u</c>/<c>v</c>) for compose-gql hydration.</summary>
    public static JObject KitToInstallProjection(Kit kit)
    {
        var dto = KitToJObject(kit);
        if (dto["pieces"] is JArray rootPieces)
        {
            foreach (var piece in rootPieces.OfType<JObject>())
                NormalizePiecePoseForInstall(piece);
        }
        if (dto["designs"] is JArray designs)
        {
            foreach (var design in designs.OfType<JObject>())
            {
                if (design["pieces"] is not JArray pieces) continue;
                foreach (var piece in pieces.OfType<JObject>())
                    NormalizePiecePoseForInstall(piece);
            }
        }
        return dto;
    }

    private static JObject DefaultInstallPlane() => JObject.Parse(
        """{"origin":{"x":0,"y":0,"z":0},"xAxis":{"x":1,"y":0,"z":0},"yAxis":{"x":0,"y":1,"z":0}}""");

    private static JObject DefaultInstallCenter() => JObject.Parse("""{"u":0,"v":0}""");

    private static JObject NormalizeInstallCenter(JToken? center)
    {
        if (center is JObject o)
        {
            var u = o["u"] ?? o["U"];
            var v = o["v"] ?? o["V"];
            if (u != null && v != null)
                return new JObject { ["u"] = u, ["v"] = v };
        }
        return DefaultInstallCenter();
    }

    private static JObject NormalizeInstallPlane(JToken? plane)
    {
        if (plane is JObject o
            && o["origin"] is JObject origin
            && origin["x"] != null
            && (o["xAxis"] is JObject || o["x_axis"] is JObject)
            && (o["yAxis"] is JObject || o["y_axis"] is JObject))
            return (JObject)o.DeepClone();
        return DefaultInstallPlane();
    }

    private static void NormalizePiecePoseForInstall(JObject piece)
    {
        var pose = piece["pose"] as JObject;
        var planeTok = pose?["plane"] ?? piece["plane"];
        var centerTok = pose?["center"] ?? piece["center"];
        var newPose = new JObject();
        if (planeTok != null)
            newPose["plane"] = NormalizeInstallPlane(planeTok);
        else
            newPose["plane"] = DefaultInstallPlane();
        if (centerTok != null)
            newPose["center"] = NormalizeInstallCenter(centerTok);
        piece["pose"] = newPose;
        piece.Remove("plane");
        piece.Remove("center");
    }

    private static Kit SnapshotToKit(JToken tok) =>
        Utility.DeserializeKit(tok.ToString(Formatting.None))!;

    private static bool TryOpenStore(out StoreClient? client)
    {
        var bin = StorePaths.ResolveStoreBinary();
        if (string.IsNullOrEmpty(bin) || !System.IO.File.Exists(bin))
        {
            client = null;
            return false;
        }
        client = new StoreClient(bin);
        return true;
    }

    /// <summary>🧾 After install/import, verifies materialized <c>wip.theKit.kit.name</c> via golden GraphQL when store is up.</summary>
    private static void AssertGraphqlKitNameMatches(StoreSession session, string expectedName)
    {
        var actual = session.Kit.Name;
        if (actual != expectedName)
            throw new IOException($"graphql: materialized kit name {actual} != expected {expectedName}");
    }

    public static string? ResolveKitJsonPath(string folderPath)
    {
        var root = Path.GetFullPath(folderPath);
        var direct = Path.Combine(root, "kit.json");
        if (System.IO.File.Exists(direct)) return direct;
        foreach (var f in Directory.EnumerateFiles(root, "kit.json", SearchOption.AllDirectories))
        {
            if (f.Contains($"{Path.DirectorySeparatorChar}.compose{Path.DirectorySeparatorChar}", StringComparison.Ordinal)
                || f.Contains("/.compose/", StringComparison.Ordinal))
                continue;
            return f;
        }
        return null;
    }

    public static bool KitsEqual(Kit a, Kit b) =>
        JToken.DeepEquals(
            JObject.Parse(Utility.Serialize(a)),
            JObject.Parse(Utility.Serialize(b)));

    public static Kit LoadKitFromZip(string zipPath)
    {
        if (!System.IO.File.Exists(zipPath)) throw new FileNotFoundException(zipPath);
        var tempDir = Path.Combine(Path.GetTempPath(), $"compose-kit-{Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);
        try
        {
            ZipFile.ExtractToDirectory(zipPath, tempDir);
            return LoadKitFromFolder(tempDir);
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }
    }

    public static Kit LoadKitFromFolder(string folderPath)
    {
        if (!Directory.Exists(folderPath) && !System.IO.File.Exists(folderPath)) throw new IOException(folderPath);
        var kitJson = ResolveKitJsonPath(folderPath)
            ?? throw new FileNotFoundException($"No kit.json under {folderPath}");
        var kit = SnapshotToKit(JObject.Parse(System.IO.File.ReadAllText(kitJson)));
        if (TryOpenStore(out var c) && c != null)
        {
            using (c)
            using (var session = new StoreSession(c))
            {
                session.InstallCreate(KitToInstallProjection(kit));
                AssertGraphqlKitNameMatches(session, kit.Name ?? "");
            }
        }
        return kit;
    }

    public static Kit LoadKitFromFile(string filePath)
    {
        if (!System.IO.File.Exists(filePath)) throw new FileNotFoundException(filePath);
        var kit = SnapshotToKit(JObject.Parse(System.IO.File.ReadAllText(filePath)));
        if (TryOpenStore(out var c) && c != null)
        {
            using (c)
            using (var session = new StoreSession(c))
            {
                session.InstallCreate(KitToInstallProjection(kit));
                AssertGraphqlKitNameMatches(session, kit.Name ?? "");
            }
        }
        return kit;
    }

    public static void SaveKitToFile(Kit kit, string filePath)
    {
        if (TryOpenStore(out var c) && c != null)
        {
            using (c)
            using (var session = new StoreSession(c))
            {
                session.InstallCreate(KitToInstallProjection(kit));
                AssertGraphqlKitNameMatches(session, kit.Name ?? "");
            }
        }
        System.IO.File.WriteAllText(Path.GetFullPath(filePath), Utility.Serialize(kit));
    }

    public static void SaveKitToFolder(Kit kit, string folderPath)
    {
        Directory.CreateDirectory(folderPath);
        SaveKitToFile(kit, Path.Combine(folderPath, "kit.json"));
    }

    public static void SaveKitToZip(Kit kit, string zipPath)
    {
        if (System.IO.File.Exists(zipPath)) System.IO.File.Delete(zipPath);
        var tempDir = Path.Combine(Path.GetTempPath(), $"compose-kit-{Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);
        try
        {
            SaveKitToFolder(kit, tempDir);
            ZipFile.CreateFromDirectory(tempDir, zipPath);
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }
    }

    /// <summary>🔁 Apply <paramref name="diff"/> to <paramref name="baseKit"/> or folder snapshot, persist via <see cref="SaveKitToFolder"/>, return updated kit.</summary>
    public static Kit ApplyKitDiffAndSaveToFolder(string folderPath, KitDiff diff, Kit? baseKit = null)
    {
        var kit = Kit.ApplyDiff(baseKit ?? LoadKitFromFolder(folderPath), diff);
        SaveKitToFolder(kit, folderPath);
        return kit;
    }
}
    //#endregion StoreKitIO

}
//#endregion 🗄️Store

}
