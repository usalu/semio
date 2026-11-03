"""EN 1993 profile and driver (Delta variant): python -I en1993.py [--apply]. Stages under 🗑️generated/leaf-kinds-en1993/stage; `--apply` writes the stage and removes the retired kinds."""
import copy, glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import common, leaf as L, model as M, delta as D, wire as W

STAGE93 = TICKET + "/🗑️generated/leaf-kinds-en1993/stage"
art = W.Art("en1993", "🔩️", "🔩️", None, None)
art.anchor_crate = "semio-s-artifact-norm-en1993-mut-general"
art.anchor_dir = "🏗️general"
L.init("en1993")
E = lambda name, emoji, coll, df, en, de, slug=None, item=None: (M.Ent(name, emoji, [(coll, df, "id")], en, de, slug=slug or name), item)
ROWS = [
    E("material", "⚗️", "materials", "SteelMaterial", "Material", "Werkstoff", item="material"),
    E("section", "📐️", "sections", "SteelSection", "Section", "Querschnitt", item="section"),
    E("member", "📏️", "members", "SteelMember", "Member", "Bauteil", item="member"),
    E("load-case", "📚️", "load_cases", "LoadCase", "Load case", "Lastfall", item="loadCase"),
    E("member-action", "🏋️", "member_actions", "MemberAction", "Member action", "Bauteileinwirkung", slug="action", item="memberAction"),
    E("joint", "🔩️", "joints", "SteelJoint", "Joint", "Anschluss", item="joint"),
    E("fatigue-detail", "🔁️", "fatigue_details", "FatigueDetail", "Fatigue detail", "Ermüdungsdetail", slug="fatigue", item="fatigueDetail"),
    E("fire-exposure", "🔥️", "fire_exposures", "FireExposure", "Fire exposure", "Brandbeanspruchung", slug="fire", item="fireExposure"),
    E("cold-formed", "🥶️", "cold_formed_members", "ColdFormedMember", "Cold-formed member", "Kaltprofil", slug="cf", item="coldFormedMember"),
    E("plated", "🧱️", "plated_panels", "PlatedPanel", "Plated panel", "Plattenfeld", slug="plated", item="platedPanel"),
    E("silo", "🛢️", "silo_shells", "SiloShell", "Silo shell", "Silo-Schale", slug="silo", item="siloShell"),
    E("tension", "🪢️", "tension_components", "TensionComponent", "Tension component", "Zugglied", slug="tension", item="tensionComponent"),
    E("bridge", "🌉️", "bridge_fatigue", "BridgeFatigue", "Bridge fatigue", "Brückenermüdung", slug="bridge", item="bridgeFatigueItem"),
    E("tower", "🗼️", "tower_legs", "TowerLeg", "Tower leg", "Turmstiel", slug="tower", item="towerLeg"),
    E("pile", "🪵️", "piles", "SteelPile", "Pile", "Pfahl", item="pile"),
    E("crane", "🏗️", "crane_runways", "CraneRunway", "Crane runway", "Kranbahn", slug="crane", item="craneRunway"),
    E("storey", "🏢️", "storeys", "Storey", "Storey", "Geschoss", item="storey"),
    E("restraint", "🧷️", "restraints", "Restraint", "Restraint", "Halterung", item="restraint"),
    E("built-up", "🪜️", "built_up_members", "BuiltUpMember", "Built-up member", "Zusammengesetzter Stab", slug="built-up", item="builtUpMember"),
    E("weld-group", "🧵️", "weld_groups", "WeldGroup", "Weld group", "Schweißnahtgruppe", slug="weld", item="weldGroup"),
    E("param-section", "🔧️", "parametric_sections", "ParametricSection", "Parametric section", "Parametrischer Querschnitt", slug="param", item="parametricSection"),
    E("hollow-joint", "🕳️", "hollow_joints", "HollowJoint", "Hollow joint", "Hohlprofilknoten", slug="hollow", item="hollowJoint"),
    E("joint-row", "🔩️", "joint_rows", "JointRow", "Joint row", "Schraubenreihe", slug="row", item="jointRow"),
    E("frame-node", "🕸️", "frame_nodes", "FrameNode", "Frame node", "Rahmenknoten", slug="frame-node", item="frameNode"),
    E("frame-bar", "🕸️", "frame_bars", "FrameBar", "Frame bar", "Rahmenstab", slug="frame-bar", item="frameBar"),
    E("placement", "📍️", "placements", "Placement", "Placement", "Platzierung", slug="placement", item="placement"),
    E("joint-plate", "🧩️", "joint_plates", "JointPlate", "Joint plate", "Anschlussblech", slug="joint-plate", item="jointPlate"),
    E("foundation-block", "🧱️", "foundation_blocks", "FoundationBlock", "Foundation block", "Fundamentblock", slug="foundation", item="foundationBlock"),
]
NEW_ROWS = {"Storey", "Restraint", "BuiltUpMember", "WeldGroup", "ParametricSection", "HollowJoint", "JointRow", "FrameNode", "FrameBar", "Placement", "JointPlate", "FoundationBlock"}
ROW_CRATES = [("frame-rows", "🏢️", ["Storey", "Restraint", "BuiltUpMember"]), ("joint-rows", "🧵️", ["WeldGroup", "ParametricSection", "HollowJoint", "JointRow"]), ("frame-model-rows", "🕸️", ["FrameNode", "FrameBar"]), ("placement-rows", "📍️", ["Placement"]), ("plate-rows", "🧩️", ["JointPlate", "FoundationBlock"])]
MAXLEAVES = 9
M.ABBREV += [("transformation", "xform"), ("stiffness", "stiff"), ("resistance", "res"), ("diameter", "dia"), ("distance", "dist"), ("concrete", "conc"), ("compression", "compr"), ("protection", "prot"), ("conductivity", "cond"), ("deflection", "defl"), ("coefficient", "coef"), ("extension", "ext"), ("eccentricity", "ecc"), ("inertia", "inert"), ("moment", "mom"), ("reaction", "react"), ("friction", "fric"), ("direction", "dir"), ("strength", "str"), ("length", "len"), ("column", "col"), ("restraint", "rest"), ("toughness", "tough"), ("intermediate", "mid"), ("transverse", "trans"), ("stiffener", "stiff"), ("effective", "eff"), ("horizontal", "horiz"), ("lateral", "lat"), ("permitted", "perm"), ("amplitude", "amp"), ("specific", "spec"), ("category", "cat"), ("required", "req"), ("opposite", "opp"), ("crippling", "crip"), ("spacing", "space"), ("position", "pos"), ("flange", "fl"), ("interlock", "ilock"), ("factor", "fac"), ("stiffened", "stfd"), ("stiff", "stf"), ("corrugation", "corr"), ("projection", "proj"), ("surfacing", "surf"), ("crossbeam", "xbeam")]
L.TARGET_DE["id"] = "Kennung"
RET_SUFFIX = ("-inputs", "-properties")
SUF = "/✅apply/🔣️.after.json"


ROW_ORDER = ["Storey", "Restraint", "BuiltUpMember", "WeldGroup", "ParametricSection", "HollowJoint", "JointRow", "FrameNode", "FrameBar", "Placement", "JointPlate", "FoundationBlock"]


def def_kebab(df):
    return re.sub(r"(?<!^)([A-Z])", lambda m: "-" + m.group(1), df).lower()


def kind_limit():
    return 255 - len((art.fix + "/").encode()[len(SEMIO) + 1:]) - 7 - len(SUF.encode()) - 1


def rule_path(l):
    return camel(l.ent.chain[0][0]) + "[]." + ".".join(camel(p) for p in l.path)


def chunks(ls):
    """🧩 Splits an entity's leaves into ceil(n/MAXLEAVES) near-equal groups, cutting at first-word family boundaries where one lies close to the even cut."""
    n = len(ls)
    k = -(-n // MAXLEAVES)
    if k == 1:
        return [ls]
    word = [l.path[0].split("_")[0] for l in ls]
    bounds = {i for i in range(1, n) if word[i] != word[i - 1]}
    cuts, prev = [], 0
    for i in range(1, k):
        t = round(n * i / k)
        ok = [b for b in bounds if prev < b and b - prev <= MAXLEAVES and n - b <= MAXLEAVES * (k - i) and abs(b - t) <= 3]
        c = min(ok, key=lambda b: abs(b - t)) if ok else t
        cuts.append(c)
        prev = c
    edges = [0] + cuts + [n]
    return [ls[edges[i]: edges[i + 1]] for i in range(k)]


def group_names(ent, cs):
    names, used = [], set()
    skip = set(ent.name.split("-"))
    taken = lambda n: n in used or os.path.isdir(art.mut + "/" + emoji_dir(ent.emoji, n))
    for i, c in enumerate(cs):
        words = [w for w in c[0].field.split("_") if w not in skip] or c[0].field.split("_")
        cands = ([ent.name] if not i else []) + ["%s-%s" % (ent.name, "-".join(words[:k])) for k in range(1, len(words) + 1)]
        n = next(x for x in cands if not taken(x))
        used.add(n)
        names.append(n)
    return names


def diff_names(diff_rs):
    patches = dict((m.group(2), m.group(1)) for m in re.finditer(r"pub (En1993\w+Patch) of [\w:]*::(\w+) \{", diff_rs))
    deltas = dict((m.group(2), m.group(1)) for m in re.finditer(r"pub (En1993\w+Delta) \{[^}]*patch: (\w+),", diff_rs))
    fields = {}
    for m in re.finditer(r"pub (En1993\w+Patch) of [\w:]*::(\w+) \{ set \{(.*?)\} \}", diff_rs, re.S):
        fields[m.group(1)] = re.findall(r"(?:^|,)\s*(\w+):", m.group(3))
    return patches, deltas, fields


def synth(defs, name, rid, doc_members):
    row = {}
    for k, p in defs[name]["properties"].items():
        ref = p.get("$ref", "").split("/")[-1]
        t = p.get("type")
        if ref:
            row[k] = (defs[ref]["enum"][0] if "enum" in defs[ref] else synth(defs, ref, rid, doc_members))
        elif t == "string":
            row[k] = rid if k == "id" else (p["enum"][0] if "enum" in p else (doc_members[0] if k.endswith("Id") and doc_members else ""))
        elif t == "number":
            row[k] = 1.0
        elif t == "integer":
            row[k] = 1
        elif t == "boolean":
            row[k] = False
        else:
            row[k] = []
    return row


def new_value(leaf, old):
    if leaf.ty == "f64":
        return round(old * 2.0, 9) if old else 1.5
    if leaf.ty in ("u32", "u8"):
        return old + 1
    if leaf.ty == "bool":
        return not old
    return old + "-alt" if old else "alt"


def get(row, l):
    cur = row
    for p in l.path:
        cur = cur[camel(p)]
    return cur


def put(row, l, v):
    cur = row
    for p in l.path[:-1]:
        cur = cur[camel(p)]
    cur[camel(l.path[-1])] = v


def main():
    F = Files(STAGE93)
    common.STAGE = STAGE93
    M.LIMIT[0] = kind_limit()
    A = art.any
    schema_path = A + "/🧬️schema/📸️snapshot/🔣️.json"
    rs_path = A + "/🧬️schema/📸️snapshot/🦀️.rs"
    snap_rs = rd(rs_path)
    patches, deltas, patch_fields = diff_names(rd(A + "/🧬️schema/🔺️diff/🦀️.rs"))
    ents, items, pending = [], {}, []
    for ent, item in ROWS:
        df = ent.chain[0][1]
        if df not in patches or ("pub %s: Vec<%s>" % (ent.chain[0][0], df)) not in snap_rs:
            pending.append(df)
            continue
        ents.append(ent)
        items[df] = item
    leaves, defs, skipped = D.read(schema_path, rs_path, ents)
    by_def = {e.chain[0][1]: e for e in ents}
    kinds_now = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M)
    retired = [k for k in kinds_now if k.startswith("update-")]
    keep = [k for k in kinds_now if k not in retired]
    existing = set(keep)
    if os.environ.get("EN1993_REGEN"):
        rowset = {v + "-" + def_kebab(d) for v in ("insert", "remove") for d in NEW_ROWS}
        existing = {k for k in keep if k == "change-annex" or (k.startswith(("insert-", "remove-")) and k not in rowset)}
    new_leaves = [l for l in leaves if l.kind not in existing]
    kinds_seen = {}
    for l in new_leaves:
        assert l.kind not in kinds_seen, "kind collision: " + l.kind
        kinds_seen[l.kind] = l
    groups, order, crate_dirs = {}, [], {}
    for e in ents:
        ls = [l for l in new_leaves if l.ent is e]
        if not ls:
            continue
        cs = chunks(ls)
        for n, c in zip(group_names(e, cs), cs):
            groups[n] = c
            order.append(n)
    names = W.create_crates(art, F, groups, art.mut + "/" + art.anchor_dir)
    row_kinds = []
    for df in ROW_ORDER:
        if df in by_def:
            row_kinds += ["insert-" + def_kebab(df), "remove-" + def_kebab(df)]
    row_kinds = [k for k in row_kinds if k not in existing]
    kinds_all = keep + row_kinds + [l.kind for l in new_leaves]
    tags = {k: i for i, k in enumerate(kinds_all)}
    stats = dict(leaves=len(leaves), new=len(new_leaves), crates=len(names), rows=len(row_kinds), retired=len(retired), pending=pending, skipped=skipped)
    import types
    return types.SimpleNamespace(**{k: v for k, v in locals().items() if k not in ("t", "a")})
