"""🧪 Read-only audit of authored staffage volumes: splice state, solid counts and kinds per scale, cross-scale copies, decisions quality, template clusters, organic box and extrude shares, thermal and structure coverage. Writes only inside the ticket folder."""
import csv
import collections
import difflib
import glob
import json
import os
import re

T = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
GEN = os.path.join(T, "🗑️generated", "audit3")
OUT_MD = os.path.join(T, "audit-authored.md")
OUT_CSV = os.path.join(T, "audit-authored.csv")
OUT_JSON = os.path.join(GEN, "results.json")
ORDER = ["50", "100", "200", "500"]
KINDS = ["box", "extrude", "loft", "revolve", "capsule", "ellipsoid"]
EXCL = {"id", "bone", "material", "role", "tone", "tags", "region", "layer", "member", "part", "anchor",
        "sectionFaces", "slender", "texture", "envelope", "of", "kind", "title", "label", "layers"}
SKIPK = {"count", "rotation", "rotationAxis", "rotationRad", "exponent", "weights", "capStart", "capEnd"}
EN_STOP = {"the", "and", "with", "of", "is", "are", "its", "which", "from", "that", "this", "each", "into", "by",
           "these", "their", "they", "while", "where", "has", "have", "was", "were", "both"}
DE_STOP = {"der", "die", "das", "und", "mit", "von", "ist", "sind", "ein", "eine", "einer", "einem", "den", "dem",
           "des", "auf", "für", "zum", "zur", "wird", "werden", "nicht", "oder", "sich", "aus", "bei", "als", "wie",
           "im", "am", "zu"}
PH = re.compile(r"\b(TODO|DRAFT|placeholder|lorem|ipsum)\b", re.I)
ORGANIC = ("humans", "animals", "vegetation")


def leaves(obj, path=()):
    out = []
    if isinstance(obj, dict):
        for k in sorted(obj):
            out += leaves(obj[k], path + (k,))
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            out += leaves(v, path + (i,))
    else:
        out.append((path, obj))
    return out


def geo(solid):
    return leaves({k: v for k, v in solid.items() if k not in EXCL})


def is_num(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool)


def rnd(v):
    return round(v, 6) if isinstance(v, float) else v


def classify(a, b):
    if a.get("kind") != b.get("kind"):
        return "different"
    la, lb = geo(a), geo(b)
    if [p for p, _ in la] != [p for p, _ in lb]:
        return "different"
    num = diff = 0
    pairs = []
    for (p, va), (_, vb) in zip(la, lb):
        if is_num(va) and is_num(vb):
            num += 1
            if rnd(va) != rnd(vb):
                diff += 1
            if not any(isinstance(x, str) and x in SKIPK for x in p):
                pairs.append((va, vb))
        elif va != vb:
            return "different"
    if diff == 0:
        return "identical"
    k = None
    good = True
    for va, vb in pairs:
        if abs(va) < 1e-9:
            if abs(vb) > 1e-9:
                good = False
                break
            continue
        r = vb / va
        if k is None:
            k = r
        elif abs(r - k) > 2e-3 * abs(k):
            good = False
            break
    if good and k is not None and abs(k - 1) > 2e-3:
        return "scaled"
    if diff <= max(3, 0.1 * num):
        return "near"
    return "different"


def count_n(s):
    rep = s.get("repeat")
    return int(rep["count"]) if isinstance(rep, dict) and "count" in rep else 1


def english_like(text):
    words = re.findall(r"[a-zA-ZäöüÄÖÜß]+", text.lower())
    if len(words) < 5:
        return False
    en = sum(w in EN_STOP for w in words)
    de = sum(w in DE_STOP for w in words)
    return en > de and en / len(words) >= 0.12


def entries_of(dec):
    out = []
    for d in dec:
        if isinstance(d, dict):
            out.append((d.get("en") or "", d.get("de")))
        else:
            out.append((str(d), None))
    return out


def dec_stats(dec):
    ens, des = [], []
    st = {"n": len(dec), "de_missing": 0, "de_same_en": 0, "de_english_like": 0, "placeholder": 0}
    for en, de in entries_of(dec):
        ens.append(len(en))
        if not de:
            st["de_missing"] += 1
        else:
            des.append(len(de))
            if de.strip() == en.strip():
                st["de_same_en"] += 1
            elif english_like(de):
                st["de_english_like"] += 1
        if PH.search(en) or (de and PH.search(de)):
            st["placeholder"] += 1
    st["en_mean_len"] = round(sum(ens) / len(ens), 1) if ens else 0
    st["de_mean_len"] = round(sum(des) / len(des), 1) if des else 0
    return st


def norm(text, toks):
    t = text.lower()
    t = re.sub(r"\d+(?:[.,]\d+)?", "#", t)
    for tok in toks:
        t = re.sub(r"\b%s\b" % re.escape(tok), "<self>", t)
    return re.sub(r"\s+", " ", t).strip()


def grams(text):
    w = re.findall(r"\w+", text)
    return {" ".join(w[i:i + 3]) for i in range(max(0, len(w) - 2))}


def load_assets():
    recs = []
    for f in sorted(glob.glob(os.path.join(CAT, "*", "*", "*.json"))):
        doc = json.load(open(f, encoding="utf-8"))
        a = doc["asset"]
        cat = os.path.basename(os.path.dirname(os.path.dirname(f)))
        slug = os.path.basename(os.path.dirname(f))
        recs.append({"key": f"{cat}/{slug}", "cat": cat, "slug": slug, "a": a,
                     "spliced": "volumes" in a, "old_volume": "volume" in a})
    return recs


def analyse_scale(a, v):
    body, th, st, dec = v["body"], v["thermal"], v["structure"], v["decisions"]
    body_ids = {x["id"]: x for x in body}
    r = {"decisions": dec_stats(dec), "body": {"n": sum(count_n(x) for x in body)}}
    kc = collections.Counter()
    for x in body:
        kc[x.get("kind", "?")] += count_n(x)
    r["body"]["kinds"] = {k: kc[k] for k in KINDS}
    r["body"]["unknown_kind"] = sum(v_ for k, v_ in kc.items() if k not in KINDS)
    for name, lst in (("thermal", th), ("structure", st)):
        real = [x for x in lst if "of" not in x]
        refs = [x for x in lst if "of" in x]
        r[name] = {"real": sum(count_n(x) for x in real), "ref": sum(count_n(x) for x in refs),
                   "refs_unresolved": sum(count_n(x) for x in refs if x["of"] not in body_ids)}
    allsol = body + th + st
    r["flags"] = {k: sum(count_n(x) for x in allsol if x.get(k)) for k in ("slender", "texture", "envelope")}
    r["flags"]["sectionFaces"] = sum(count_n(x) for x in allsol if x.get("sectionFaces"))
    r["flags"]["layer"] = sum(count_n(x) for x in allsol if x.get("layer"))
    r["structure_parts"] = {p: sum(count_n(x) for x in st if x.get("part") == p)
                            for p in ("member", "joint", "support", "mass")}
    return r


def analyse_asset(rec):
    a = rec["a"]
    out = {"key": rec["key"], "cat": rec["cat"], "slug": rec["slug"], "spliced": rec["spliced"],
           "old_volume": rec["old_volume"], "scales": {}, "pairs": [], "thermal_gaps": [], "struct_gaps": []}
    if not rec["spliced"]:
        return out
    authored = [s for s in ORDER if a["volumes"][s]["status"] == "authored"]
    out["status"] = {s: a["volumes"][s]["status"] for s in ORDER}
    for s in authored:
        out["scales"][s] = analyse_scale(a, a["volumes"][s])
    for fine, coarse in zip(authored, authored[1:]):
        fb = {x["id"]: x for x in a["volumes"][fine]["body"]}
        cb = a["volumes"][coarse]["body"]
        cnt = collections.Counter()
        for x in cb:
            if x["id"] not in fb:
                cnt["new"] += 1
            else:
                cnt["shared"] += 1
                cnt[classify(x, fb[x["id"]])] += 1
        cids = {x["id"] for x in cb}
        out["pairs"].append({"fine": fine, "coarse": coarse, "coarse_n": len(cb),
                             "shared": cnt["shared"], "identical": cnt["identical"],
                             "scaled": cnt["scaled"], "near": cnt["near"],
                             "different": cnt["different"], "new": cnt["new"],
                             "subset": cids <= set(fb) and len(cb) > 0})
    tot = sum(p["coarse_n"] for p in out["pairs"])
    cop = sum(p["identical"] + p["scaled"] + p["near"] for p in out["pairs"])
    out["copy_share"] = round(cop / tot, 3) if tot else None
    out["subset_pairs"] = sum(p["subset"] for p in out["pairs"])
    regions = a["thermal"]["regions"]
    supports = a["structure"].get("supports", [])
    for s in authored:
        v = a["volumes"][s]
        covered = collections.Counter(x.get("region") for x in v["thermal"])
        ids_reg = {r["id"] for r in regions}
        missing = [r["id"] for r in regions if covered[r["id"]] == 0]
        unknown = sorted({x.get("region") for x in v["thermal"] if x.get("region") not in ids_reg})
        out["scales"][s]["thermal_regions"] = len(regions)
        out["scales"][s]["thermal_missing"] = missing
        out["scales"][s]["thermal_unknown_region"] = unknown
        if s == "50":
            ml = []
            for r in regions:
                if len(r.get("layers") or []) >= 2:
                    if not any(x.get("region") == r["id"] and x.get("layer") for x in v["thermal"]):
                        ml.append(r["id"])
            out["scales"][s]["multilayer_no_layer"] = ml
        has_sup = any(x.get("part") == "support" for x in v["structure"])
        out["scales"][s]["support_missing"] = bool(supports) and not has_sup
        out["scales"][s]["members_lt2"] = out["scales"][s]["structure_parts"]["member"] < 2
    return out


def asset_clusters(recs):
    items = []
    for rec in recs:
        if not rec["spliced"]:
            continue
        a = rec["a"]
        toks = [t for t in rec["slug"].split("-") if len(t) >= 3]
        for s in ORDER:
            vv = a["volumes"][s]
            if vv["status"] != "authored":
                continue
            for i, (en, _) in enumerate(entries_of(vv["decisions"])):
                n = norm(en, toks)
                items.append({"key": rec["key"], "s": s, "i": i, "raw": en, "n": n, "g": grams(n)})
    parent = list(range(len(items)))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            x, y = items[i], items[j]
            if x["key"] == y["key"]:
                continue
            u = x["g"] | y["g"]
            if not u or len(x["g"] & y["g"]) / len(u) < 0.5:
                continue
            if x["n"] == y["n"] or difflib.SequenceMatcher(None, x["n"], y["n"], autojunk=False).ratio() >= 0.9:
                parent[find(i)] = find(j)
    groups = collections.defaultdict(list)
    for idx, it in enumerate(items):
        groups[find(idx)].append(it)
    clusters = []
    for g in groups.values():
        keys = sorted({it["key"] for it in g})
        if len(keys) < 2:
            continue
        clusters.append({"assets": len(keys), "entries": len(g), "scales": sorted({it["s"] for it in g}),
                         "exact": len({it["n"] for it in g}) == 1, "members": sorted({f"{it['key']}@{it['s']}" for it in g}),
                         "sample": g[0]["raw"][:200]})
    clusters.sort(key=lambda c: (-c["assets"], -c["entries"]))
    per_item = {}
    for idx, it in enumerate(items):
        per_item[(it["key"], it["s"], it["i"])] = find(idx)
    multi = {r for r in set(per_item.values()) if sum(1 for it in groups[r] if True) and len({x["key"] for x in groups[r]}) >= 2}
    return clusters, per_item, multi, items, groups


def main():
    os.makedirs(GEN, exist_ok=True)
    recs = load_assets()
    assert len(recs) == 152, len(recs)
    res = [analyse_asset(r) for r in recs]
    clusters, per_item, multi, items, groups = asset_clusters(recs)
    for r in res:
        for s, sc in r["scales"].items():
            sc["template_entries"] = sum(1 for i in range(sc["decisions"]["n"])
                                         if per_item.get((r["key"], s, i)) in multi)
    json.dump({"assets": res, "clusters": clusters}, open(OUT_JSON, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    write_csv(res)
    write_md(res, clusters)
    print("assets", len(res), "spliced", sum(r["spliced"] for r in res), "clusters", len(clusters))


def write_csv(res):
    cols = ["category", "slug", "spliced", "old_volume", "scale", "status", "decisions", "body_n"] + \
           [f"body_{k}" for k in KINDS] + ["body_unknown_kind", "thermal_real", "thermal_ref", "thermal_refs_unresolved",
           "structure_real", "structure_ref", "structure_refs_unresolved", "member", "joint", "support", "mass",
           "slender", "texture", "envelope", "sectionFaces", "layer", "pair_fine", "pair_coarse_n", "pair_shared",
           "pair_identical", "pair_scaled", "pair_near", "pair_different", "pair_new", "pair_subset",
           "dec_en_mean_len", "dec_de_mean_len", "dec_de_missing", "dec_de_same_as_en", "dec_de_english_like",
           "placeholder", "template_entries", "thermal_regions", "thermal_missing", "thermal_unknown_region",
           "multilayer_no_layer", "support_missing", "members_lt2"]
    with open(OUT_CSV, "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh)
        w.writerow(cols)
        for r in res:
            if not r["spliced"]:
                w.writerow([r["cat"], r["slug"], 0, int(r["old_volume"])] + [""] * (len(cols) - 4))
                continue
            pairs = {p["coarse"]: p for p in r["pairs"]}
            for s in ORDER:
                if s not in r["scales"]:
                    if r["status"][s] == "unavailable":
                        w.writerow([r["cat"], r["slug"], 1, 0, s, "unavailable"] + [""] * (len(cols) - 6))
                    continue
                sc = r["scales"][s]
                p = pairs.get(s)
                d = sc["decisions"]
                row = [r["cat"], r["slug"], 1, 0, s, "authored", d["n"], sc["body"]["n"]]
                row += [sc["body"]["kinds"][k] for k in KINDS] + [sc["body"]["unknown_kind"]]
                row += [sc["thermal"]["real"], sc["thermal"]["ref"], sc["thermal"]["refs_unresolved"],
                        sc["structure"]["real"], sc["structure"]["ref"], sc["structure"]["refs_unresolved"]]
                row += [sc["structure_parts"][x] for x in ("member", "joint", "support", "mass")]
                row += [sc["flags"][x] for x in ("slender", "texture", "envelope", "sectionFaces", "layer")]
                row += ([p["fine"], p["coarse_n"], p["shared"], p["identical"], p["scaled"], p["near"],
                         p["different"], p["new"], int(p["subset"])] if p else [""] * 9)
                row += [d["en_mean_len"], d["de_mean_len"], d["de_missing"], d["de_same_en"], d["de_english_like"],
                        d["placeholder"], sc["template_entries"], sc["thermal_regions"],
                        ";".join(sc["thermal_missing"]), ";".join(map(str, sc["thermal_unknown_region"])),
                        ";".join(sc.get("multilayer_no_layer", [])), int(sc["support_missing"]), int(sc["members_lt2"])]
                w.writerow(row)


def pct(x, n):
    return f"{100 * x / n:.0f} %" if n else "n/a"


def write_md(res, clusters):
    L = []
    sp = [r for r in res if r["spliced"]]
    unsp = [r for r in res if not r["spliced"]]
    L.append("# Audit: authored staffage volumes (per asset, per scale)")
    L.append("")
    L.append("Read-only measurement by `audit3-measure.py` (stdlib only). Raw data: `🗑️generated/audit3/results.json`; one row per asset and scale: `audit-authored.csv`. Counts are expanded by `repeat.count` unless stated. Shares are by solid count, not volume.")
    L.append("")
    L.append("## 1. Splice state")
    L.append("")
    L.append(f"- Assets: {len(res)}. Spliced (`volumes`, no old `volume`): {len(sp)}. Not spliced (old `volume` still present, no `volumes`): {len(unsp)}. Both keys present: {sum(1 for r in res if r['spliced'] and r['old_volume'])}.")
    L.append(f"- Not spliced: {', '.join(r['key'] for r in unsp)}.")
    L.append("")
    L.append("## 2. Per-scale solid measures")
    L.append("")
    L.append("Totals over spliced assets, then mean per asset with authored scale. Kind shares are counts of body solids.")
    L.append("")
    L.append("| scale | authored | unavailable | body | thermal real | thermal ref | struct real | struct ref | box % | extrude % | loft % | revolve % | capsule % | ellipsoid % | slender | texture | envelope | sectionFaces | layer |")
    L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for s in ORDER:
        scs = [r["scales"][s] for r in sp if s in r["scales"]]
        un = sum(1 for r in sp if r["status"][s] == "unavailable")
        if not scs:
            continue
        bn = sum(x["body"]["n"] for x in scs)
        kk = {k: sum(x["body"]["kinds"][k] for x in scs) for k in KINDS}
        fl = {k: sum(x["flags"][k] for x in scs) for k in ("slender", "texture", "envelope", "sectionFaces", "layer")}
        L.append(f"| 1:{s} | {len(scs)} | {un} | {bn} | {sum(x['thermal']['real'] for x in scs)} | {sum(x['thermal']['ref'] for x in scs)} | "
                 f"{sum(x['structure']['real'] for x in scs)} | {sum(x['structure']['ref'] for x in scs)} | " +
                 " | ".join(pct(kk[k], bn) for k in KINDS) + " | " +
                 " | ".join(str(fl[k]) for k in ("slender", "texture", "envelope", "sectionFaces", "layer")) + " |")
    L.append("")
    refs_bad = [(r["key"], s) for r in sp for s, sc in r["scales"].items()
                if sc["thermal"]["refs_unresolved"] or sc["structure"]["refs_unresolved"]]
    L.append(f"Unresolved `of` references (sanity): {len(refs_bad)} asset-scales {refs_bad[:10]}.")
    L.append("")
    L.append("## 3. Scale independence")
    L.append("")
    L.append("Per adjacent authored pair (finer, coarser): coarse body solids, ids shared with the finer scale, and of the shared ones identical / scaled-copy / near-copy (few numbers changed) / different. `subset` = every coarse body id exists at the finer scale.")
    L.append("")
    L.append("| asset | pair | coarse body | shared ids | identical | scaled | near | different | new | subset |")
    L.append("|---|---|---|---|---|---|---|---|---|---|")
    for r in sp:
        for p in r["pairs"]:
            L.append(f"| {r['key']} | {p['fine']}→{p['coarse']} | {p['coarse_n']} | {p['shared']} | {p['identical']} | {p['scaled']} | {p['near']} | {p['different']} | {p['new']} | {'yes' if p['subset'] else ''} |")
    L.append("")
    ranked = sorted([r for r in sp if r["copy_share"] is not None], key=lambda r: (-r["copy_share"], -r["subset_pairs"], r["key"]))
    flagged = [(r["key"], p) for r in sp for p in r["pairs"]
               if p["subset"] and p["coarse_n"] and p["identical"] + p["scaled"] + p["near"] == p["coarse_n"]]
    L.append("### 3b. Coarse scales that are pure copies of the next finer scale")
    L.append("")
    L.append("Every coarse body id exists at the finer scale and every solid is identical, scaled or near (thickness or radius numbers only).")
    L.append("")
    for k, p in flagged:
        L.append(f"- {k} {p['fine']}→{p['coarse']}: {p['coarse_n']} solids, {p['identical']} identical, {p['scaled']} scaled, {p['near']} near")
    L.append(f"- Total: {len(flagged)} of {sum(len(r['pairs']) for r in sp)} adjacent pairs.")
    L.append("")
    L.append("### 3a. Top 25 most likely not modelled anew")
    L.append("")
    L.append("Ranking: share of coarse-scale body solids (1:100 to 1:500) that are identical, scaled or near copies of the same-id solid at the next finer scale; ties by subset pairs.")
    L.append("")
    L.append("| rank | asset | copy share | subset pairs | pairs (fine→coarse: identical+scaled+near / coarse body) |")
    L.append("|---|---|---|---|---|")
    for i, r in enumerate(ranked[:25], 1):
        det = "; ".join(f"{p['fine']}→{p['coarse']}: {p['identical']}+{p['scaled']}+{p['near']} / {p['coarse_n']}" for p in r["pairs"])
        L.append(f"| {i} | {r['key']} | {100 * r['copy_share']:.0f} % | {r['subset_pairs']} | {det} |")
    L.append("")
    zero = sum(1 for r in ranked if r["copy_share"] == 0)
    L.append(f"Assets with zero copies at every adjacent pair: {zero} of {len(ranked)}.")
    L.append("")
    L.append("## 4. Decisions quality")
    L.append("")
    L.append("| scale | entries | mean en chars | mean de chars | de missing | de same as en | de English-like | placeholder hits |")
    L.append("|---|---|---|---|---|---|---|---|")
    for s in ORDER:
        ds = [r["scales"][s]["decisions"] for r in sp if s in r["scales"]]
        if not ds:
            continue
        n = sum(d["n"] for d in ds)
        L.append(f"| 1:{s} | {n} | {sum(d['en_mean_len'] * d['n'] for d in ds) / n:.0f} | {sum(d['de_mean_len'] * d['n'] for d in ds) / n:.0f} | "
                 f"{sum(d['de_missing'] for d in ds)} | {sum(d['de_same_en'] for d in ds)} | {sum(d['de_english_like'] for d in ds)} | {sum(d['placeholder'] for d in ds)} |")
    L.append("")
    L.append("Template clusters: decision texts normalised (digits to `#`, the asset's own slug words to `<self>`, lower case), linked when word-3-gram Jaccard ≥ 0.5 and difflib ratio ≥ 0.9 across different assets (single linkage). Every cluster of 2+ assets is listed. `exact` = identical after normalisation.")
    L.append("")
    L.append(f"Clusters: {len(clusters)}; entries in them: {sum(c['entries'] for c in clusters)}; assets touched: {len({m.split('@')[0] for c in clusters for m in c['members']})}.")
    L.append("")
    for i, c in enumerate(clusters, 1):
        L.append(f"- C{i:02d}: {c['assets']} assets, {c['entries']} entries, scales {','.join(c['scales'])}, {'exact' if c['exact'] else 'near'}. Members: {', '.join(c['members'])}. Sample: “{c['sample']}”")
    L.append("")
    L.append("## 5. Organic subjects: box and extrude shares at 1:50")
    L.append("")
    L.append("Box share (count) above 20 % and extrude share above 40 % in humans, animals and vegetation.")
    L.append("")
    for r in sp:
        if not any(k in r["cat"] for k in ORGANIC) or "50" not in r["scales"]:
            continue
        b = r["scales"]["50"]["body"]
        n = b["n"] or 1
        bs = b["kinds"]["box"] / n
        es = b["kinds"]["extrude"] / n
        if bs > 0.2 or es > 0.4:
            L.append(f"- {r['key']}: box {100 * bs:.0f} % ({b['kinds']['box']}/{b['n']}), extrude {100 * es:.0f} % ({b['kinds']['extrude']}/{b['n']})" + (" BOX" if bs > 0.2 else "") + (" EXTRUDE" if es > 0.4 else ""))
    L.append("")
    L.append("## 6. Thermal coverage")
    L.append("")
    for r in sp:
        for s, sc in r["scales"].items():
            if sc["thermal_missing"]:
                L.append(f"- {r['key']} 1:{s}: regions without solid: {', '.join(sc['thermal_missing'])}")
            if sc["thermal_unknown_region"]:
                L.append(f"- {r['key']} 1:{s}: solids name unknown regions: {', '.join(str(x or '(no region key)') for x in sc['thermal_unknown_region'])}")
            if sc.get("multilayer_no_layer"):
                L.append(f"- {r['key']} 1:{s}: multi-layer regions without a layer solid: {', '.join(sc['multilayer_no_layer'])}")
    L.append("")
    L.append("## 7. Structure sanity")
    L.append("")
    for r in sp:
        for s, sc in r["scales"].items():
            if sc["support_missing"]:
                L.append(f"- {r['key']} 1:{s}: supports declared, no `part: support` solid")
            if sc["members_lt2"]:
                L.append(f"- {r['key']} 1:{s}: fewer than 2 member solids ({sc['structure_parts']['member']})")
    L.append("")
    L.append("## 8. Per category")
    L.append("")
    L.append("| category | spliced / total | mean copy share | assets with copies | template entries | placeholder hits |")
    L.append("|---|---|---|---|---|---|")
    cats = sorted({r["cat"] for r in res})
    for c in cats:
        rs = [r for r in res if r["cat"] == c]
        ss = [r for r in rs if r["spliced"]]
        cs = [r["copy_share"] for r in ss if r["copy_share"] is not None]
        te = sum(sc["template_entries"] for r in ss for sc in r["scales"].values())
        ph = sum(sc["decisions"]["placeholder"] for r in ss for sc in r["scales"].values())
        L.append(f"| {c} | {len(ss)} / {len(rs)} | {100 * sum(cs) / len(cs):.0f} % | {sum(1 for x in cs if x > 0)} | {te} | {ph} |" if cs else f"| {c} | {len(ss)} / {len(rs)} | n/a | 0 | {te} | {ph} |")
    L.append("")
    L.append("CATEGORY-LINES")
    L.append("")
    L.append("## 9. What the script cannot judge")
    L.append("")
    L.append("- Whether a copy at a coarser scale is wrong: identical solids can be right where the 2D drawing is identical; the script only sees geometry parameters.")
    L.append("- Silhouette, IoU, Hausdorff, bounds, volume, area and watertightness: needs the kernel tessellation and the 2D shapes (`volumes-check.ts`), not run here.")
    L.append("- Whether a decision is true for the object, whether it names what was merged, or whether a German text reads well. Only presence, length, English-stopword heuristics and text similarity are measured.")
    L.append("- Physical plausibility of supports, load paths, sectionFaces, areaM2 and area shares (C1, C16, C28).")
    L.append("- Box and extrude shares are counts; a few large boxes can dominate volume while many small boxes do not.")
    L.append("- Solid overlap, burial and whether thermal solids lie inside the body; these need geometry, not parameters.")
    L.append("")
    open(OUT_MD, "w", encoding="utf-8").write("\n".join(L) + "\n")


if __name__ == "__main__":
    main()
