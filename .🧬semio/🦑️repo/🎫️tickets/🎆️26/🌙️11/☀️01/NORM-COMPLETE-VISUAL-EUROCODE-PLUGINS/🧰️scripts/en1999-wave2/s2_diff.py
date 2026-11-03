"""Stage 2: diff lanes (rust, json, ts, graphql, proto) for WeldRun and SectionVertex cloned from the Placements lane, then every Patch block of Placements, WeldRuns and OutlineVertices regenerated with the leaf-kinds generator functions (`diff.py`), so a clean diff of `--check` against the committed files proves the hand-written Placements twins. Usage: python -I s2_diff.py [--check]"""
import copy, json, pathlib, re, sys
HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / "leaf-kinds"))
from spec import *
import diff as D, model as M

CHECK = "--check" in sys.argv
MODEL = "semio_s_artifact_norm_en1999_model"
URL = "https://json.schemas.assets.semio-tech.com/s/norm/en1999/snapshot.json"
EMOJI = {"weld_runs": "🧵️", "outline_vertices": "🔷️"}
OUT = {}


def rd(p):
    return pathlib.Path(p).read_text(encoding="utf8")


def put(p, t):
    OUT[str(p)] = t


def clone(text, ent):
    text = text.replace("Placements", ent.lane).replace("Placement", ent.struct).replace("placements", ent.field)
    return text


def between(t, a, b):
    i = t.index(a)
    return t[i:t.index(b, i)]


def rust(t):
    key = between(t, "/// 🔑️ Key extractor of the foreign `Placement` rows", "\nuse protocol::list_delta::Keyed as _;")
    lane = between(t, "    /// 🩹️ Sparse patch of the `placements` row", "    /// 🔺️ Keyed sparse diff of the En1999 artifact")
    if "WeldRunKey" not in t:
        t = t.replace(key, key + "".join(clone(key, e) for e in ENTS), 1)
        lane = between(t, "    /// 🩹️ Sparse patch of the `placements` row", "    /// 🔺️ Keyed sparse diff of the En1999 artifact")
        t = t.replace(lane, lane + "".join(clone(lane, e) for e in ENTS), 1)
        t = t.replace("    pub placements: Option<En1999PlacementsRows>,\n", "    pub placements: Option<En1999PlacementsRows>,\n" + "".join(f"    #[state(artifact)]\n    pub {e.field}: Option<En1999{e.lane}Rows>,\n" for e in ENTS), 1)
        ab = between(t, "        if let Some(theirs) = other.placements {", "    }\n\n    fn apply(")
        t = t.replace(ab, ab + "".join(clone(ab, e) for e in ENTS), 1)
        ap = between(t, "        if let Some(rows) = &self.placements {", "        *unpublished = next;")
        t = t.replace(ap, ap + "".join(clone(ap, e) for e in ENTS), 1)
        t = t.replace(".placements.as_ref().map_or(true, |rows| rows.is_empty())\n", ".placements.as_ref().map_or(true, |rows| rows.is_empty())" + "".join(f" && self.{e.field}.as_ref().map_or(true, |rows| rows.is_empty())" for e in ENTS) + "\n", 1)
        inv = "            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),\n"
        t = t.replace(inv, inv + "".join(clone(inv, e) for e in ENTS), 1)
    return t


def twin_ts(t):
    if "WeldRuns" in t:
        return t
    ifaces = between(t, "export interface En1999PlacementsPatch {", "export const parseEn1999Diff:")
    t = t.replace(ifaces, ifaces + "".join(clone(ifaces, e) for e in ENTS), 1)
    i = t.index("export const parseEn1999PlacementsInserted:")
    j = t.index("export const parseEn1999PlacementsPatch:")
    j = t.index("\n", j) + 1
    block = t[i:j]
    t = t[:j] + "".join(clone(block, e) for e in ENTS) + t[j:]
    fields = "".join(f"  /** {EMOJI[e.field]} @state artifact */\n  {e.camel}: En1999{e.lane}Rows | null;\n" for e in ENTS)
    t = t.replace("  placements: En1999PlacementsRows | null;\n}", "  placements: En1999PlacementsRows | null;\n" + fields + "}", 1)
    t = t.replace("placements: normWireDefault(normWireNullable(normWireRef(() => parseEn1999PlacementsRows)), () => null) });", "placements: normWireDefault(normWireNullable(normWireRef(() => parseEn1999PlacementsRows)), () => null), " + ", ".join(f"{e.camel}: normWireDefault(normWireNullable(normWireRef(() => parseEn1999{e.lane}Rows)), () => null)" for e in ENTS) + " });", 1)
    t = t.replace("type FatigueDetail, ", "type FatigueDetail, type Placement, type WeldRun, type SectionVertex, parsePlacement, parseWeldRun, parseSectionVertex, ", 1)
    return t


def twin_gql(t):
    if "WeldRuns" in t:
        return t
    blk = between(t, "type En1999PlacementsPatch {", "type En1999Diff {")
    t = t.replace(blk, blk + "".join(clone(blk, e) for e in ENTS), 1)
    t = t.replace("  placements: En1999PlacementsRows @state(class: ARTIFACT)\n", "  placements: En1999PlacementsRows @state(class: ARTIFACT)\n" + "".join(f"  {e.camel}: En1999{e.lane}Rows @state(class: ARTIFACT)\n" for e in ENTS), 1)
    return t


def twin_proto(t):
    if "WeldRuns" in t:
        return t
    blk = between(t, "message En1999PlacementsPatch {", "message En1999Diff {")
    t = t.replace(blk, blk + "".join(clone(blk, e) for e in ENTS), 1)
    t = t.replace("  optional En1999PlacementsRows placements = 9;\n", "  optional En1999PlacementsRows placements = 9;\n" + "".join(f"  optional En1999{e.lane}Rows {e.field} = {10 + n};\n" for n, e in enumerate(ENTS)), 1)
    return t


def twin_json(text):
    d = json.loads(text)
    defs = d["$defs"]
    if "En1999WeldRunsRows" not in defs:
        for e in ENTS:
            for suffix in ("Inserted", "Modified", "Moved", "Removed", "Rows", "Patch"):
                defs[f"En1999{e.lane}{suffix}"] = json.loads(clone(json.dumps(defs[f"En1999Placements{suffix}"], ensure_ascii=False), e))
            d["properties"][e.camel] = json.loads(clone(json.dumps(d["properties"]["placements"], ensure_ascii=False), e))
    return d


def regenerate(rust_t, ts_t, gql_t, proto_t, jd):
    ents = [M.Ent("placement", "📍️", [("placements", "Placement", "placement_id")], "Placement", "Platzierung")]
    for e in ENTS:
        ents.append(M.Ent(e.struct.lower(), e.emoji, [(e.field, e.struct, e.field + "_id")], e.id_en, e.id_de, slug=e.struct.lower()))
    leaves, _ = M.read_entities(str(SNAP / "🔣️.json"), ents, set())
    spec = D.specs("En1999", ents, leaves, MODEL)
    rust_t = D.patch_rust(rust_t, "En1999", spec, MODEL, "MembersActions", None)
    defs = jd["$defs"]
    for s in spec.values():
        defs[s.full] = D.json_patch(s, URL, "En1999")
    ts_t = D.patch_ts(ts_t, "En1999", spec, "MembersActions")
    gql_t = D.patch_blocks(gql_t, "En1999", spec, "MembersActions", "gql")
    proto_t = D.patch_blocks(proto_t, "En1999", spec, "MembersActions", "proto")
    return rust_t, ts_t, gql_t, proto_t, jd


def main():
    paths = {"rs": DIFF / "🦀️.rs", "ts": DIFF / "🟦️.ts", "gql": DIFF / "🔗️.graphql", "proto": DIFF / "🛰️.proto", "json": DIFF / "🔣️.json"}
    before = {k: rd(p) for k, p in paths.items()}
    if CHECK:
        placements_only = [M.Ent("placement", "📍️", [("placements", "Placement", "placement_id")], "Placement", "Platzierung")]
        leaves, _ = M.read_entities(str(SNAP / "🔣️.json"), placements_only, set())
        spec = D.specs("En1999", placements_only, leaves, MODEL)
        bad = 0
        for name, got, want in (
            ("rust", D.patch_rust(before["rs"], "En1999", spec, MODEL, "MembersActions", None), before["rs"]),
            ("ts", D.patch_ts(before["ts"], "En1999", spec, "MembersActions"), before["ts"]),
            ("graphql", D.patch_blocks(before["gql"], "En1999", spec, "MembersActions", "gql"), before["gql"]),
            ("proto", D.patch_blocks(before["proto"], "En1999", spec, "MembersActions", "proto"), before["proto"]),
        ):
            same = got == want
            bad += not same
            print("placements patch", name, "identical" if same else "DIFFERS")
            if not same:
                import difflib
                for line in list(difflib.unified_diff(want.splitlines(), got.splitlines(), "committed", "generated", lineterm="", n=0))[:40]:
                    print("   ", line)
        jd0 = json.loads(before["json"])
        gen = D.json_patch(spec["Placements"], URL, "En1999")
        same = gen == jd0["$defs"]["En1999PlacementsPatch"]
        bad += not same
        print("placements patch json", "identical" if same else "DIFFERS")
        sys.exit(1 if bad else 0)
    rs_t, ts_t, gql_t, proto_t = rust(before["rs"]), twin_ts(before["ts"]), twin_gql(before["gql"]), twin_proto(before["proto"])
    jd = twin_json(before["json"])
    rs_t, ts_t, gql_t, proto_t, jd = regenerate(rs_t, ts_t, gql_t, proto_t, jd)
    for k, t in (("rs", rs_t), ("ts", ts_t), ("gql", gql_t), ("proto", proto_t), ("json", json.dumps(jd, indent=2, ensure_ascii=False) + "\n")):
        paths[k].write_text(t, encoding="utf8")
    print("stage 2 done")


main()
