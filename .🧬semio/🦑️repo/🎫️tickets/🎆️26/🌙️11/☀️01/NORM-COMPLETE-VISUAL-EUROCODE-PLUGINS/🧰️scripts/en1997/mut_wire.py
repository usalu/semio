import sys, os, re, json
sys.path.insert(0, ".")
from spec import *
from rows import *

NK = newkinds()
def crate(r): return f"semio_s_artifact_norm_en1997_mut_{r.group}"

# group mounts
for g in ("ground", "foundations"):
    gdir = {"ground": "🪨️ground", "foundations": "🏗️foundations"}[g]
    p = f"{MUT}/{gdir}/🦀️.rs"
    s = open(p, encoding="utf8").read()
    add = "".join(f'#[path = "../{dirname(r, v)}/🦀️.rs"]\npub mod {mod(r, v)};\n' for r, v in NK if r.group == g)
    if add.split("\n")[1] in s: continue
    s = s.replace("//#endregion 🪢️Mounts", add + "//#endregion 🪢️Mounts", 1)
    open(p, "w", encoding="utf8").write(s)

# aggregate rs
p = MUT + "/🦀️.rs"
s = open(p, encoding="utf8").read()
if "InsertWall" not in s:
    uses = "".join(f"use {crate(r)}::{mod(r, v)};\n" for r, v in NK)
    s = s.replace("//#endregion 🔖️Leaves", uses + "//#endregion 🔖️Leaves", 1)
    vs = "".join(f"    {struct(r, v)}({mod(r, v)}::{struct(r, v)}),\n" for r, v in NK)
    s = s.replace("    RemovePile(remove_pile::RemovePile),\n}", "    RemovePile(remove_pile::RemovePile),\n" + vs + "}", 1)
    ks = "".join(f'    "{kind(r, v)}",\n' for r, v in NK)
    s = s.replace('    "remove-pile",\n];', '    "remove-pile",\n' + ks + "];", 1)
    open(p, "w", encoding="utf8").write(s)

# aggregate json
p = MUT + "/🔣️.json"
d = json.load(open(p, encoding="utf8"))
refs = [x["$ref"] for x in d["oneOf"]]
for r, v in NK:
    u = f"https://json.schemas.assets.semio-tech.com/s/norm/en1997/mutation/{kind(r, v)}/schema.json"
    if u not in refs: d["oneOf"].append({"$ref": u})
open(p, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")

# aggregate graphql
p = MUT + "/🔗️.graphql"
s = open(p, encoding="utf8").read()
if "InsertWall" not in s:
    GQ = {"s": "String", "n": "Float", "u": "Int"}
    def inp(e):
        l = [f"input {e.struct}Input {{", "  id: String!"] + [f"  {f.camel}: {GQ[f.kind]}!" for f in e.fields]
        if e.nested: l.append(f"  {camel(e.nested[0])}: [{e.nested[2].struct}Input!]!")
        l.append("}")
        return "\n".join(l) + "\n"
    inputs = ""
    for e in ENTITIES:
        if e.nested: inputs += inp(e.nested[2])
        inputs += inp(e)
    inputs += """input RetainingWallInput {
  id: String!
  height: Float!
  embedment: Float!
  baseWidth: Float!
  stemThickness: Float!
  backfillPhiDeg: Float!
  backfillGamma: Float!
  wallFrictionDeg: Float!
  earthPressureMode: String!
  wallMovement: String!
  ocr: Float!
  concreteGamma: Float!
  surcharge: Float!
  verticalPermanent: Float!
  horizontalPermanent: Float!
}
input SlopeInput {
  id: String!
  angleDeg: Float!
  height: Float!
  length: Float!
  governingLayerId: String!
}
input UpliftCaseInput {
  id: String!
  permanentStabilizing: Float!
  permanentDestabilizing: Float!
  variableDestabilizing: Float!
  porePressure: Float!
  totalStress: Float!
}
"""
    s = s.replace("union En1997Mutation =", inputs + "union En1997Mutation =", 1)
    s = s.replace("  | RemovePile\n", "  | RemovePile\n" + "".join(f"  | {struct(r, v)}\n" for r, v in NK), 1)
    types = ""
    for r, v in NK:
        f = {"insert": f"index: Int! {camel(r.item)}: {r.struct}Input!", "remove": "index: Int!", "update": f"{camel(r.item)}: {r.struct}Input!"}[v]
        types += f"type {struct(r, v)} {{ mutation: String! {f} }}\n"
    s = s.rstrip("\n") + "\n" + types
    open(p, "w", encoding="utf8").write(s)

# aggregate ts
p = MUT + "/🟦️.ts"
s = open(p, encoding="utf8").read()
if "InsertWall" not in s:
    imps = "".join(f'import {{ type {struct(r, v)}, parse{struct(r, v)} }} from "./{dirname(r, v)}/🧬️schema/🟦️.ts";\n' for r, v in NK)
    s = s.replace("\nexport type En1997Mutation =", imps + "\nexport type En1997Mutation =", 1)
    s = s.replace("  | RemovePile;", "  | RemovePile\n" + "\n".join(f"  | {struct(r, v)}" for r, v in NK) + ";", 1)
    ent = "".join(f"  {struct(r, v)[0].lower() + struct(r, v)[1:]}: parse{struct(r, v)},\n" for r, v in NK)
    s = s.replace("  removePile: parseRemovePile,\n", "  removePile: parseRemovePile,\n" + ent, 1)
    open(p, "w", encoding="utf8").write(s)

# binary protocol
p = f"{S}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
s = open(p, encoding="utf8").read()
if "insert-wall" not in s:
    t = 20
    for r, v in NK:
        s = s.rstrip("\n") + f"\nrecord {kind(r, v)} tag={t}\nfield payload bytes\n"
        t += 1
    open(p, "w", encoding="utf8").write(s)

# text + binary rs codecs
p = f"{S}/🚪️io/📝️text/🧬️mutations/🦀️.rs"
s = open(p, encoding="utf8").read()
if "InsertWall" not in s:
    s = s.replace("use semio_s_artifact_norm_en1997_model::{Pile, SoilLayer, SpreadFoundation};", "use semio_s_artifact_norm_en1997_model::{Anchor, EmbeddedWall, LabTest, LayerWater, Pile, PileGroup, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase};")
    # imports from group crates
    s = s.replace("use semio_s_artifact_norm_en1997_mut_foundations::{change_footing_embedment, change_footing_width, change_pile_count, change_pile_length, change_wall_base_width, insert_footing, insert_pile, remove_footing, remove_pile};",
        "use semio_s_artifact_norm_en1997_mut_foundations::{change_footing_embedment, change_footing_width, change_pile_count, change_pile_length, change_wall_base_width, insert_footing, insert_pile, remove_footing, remove_pile, " + ", ".join(mod(r, v) for r, v in NK if r.group == "foundations") + "};")
    s = s.replace("use semio_s_artifact_norm_en1997_mut_ground::{change_layer_oedometric_modulus, change_layer_phi_prime, change_slope_angle, insert_layer, remove_layer};",
        "use semio_s_artifact_norm_en1997_mut_ground::{change_layer_oedometric_modulus, change_layer_phi_prime, change_slope_angle, insert_layer, remove_layer, " + ", ".join(mod(r, v) for r, v in NK if r.group == "ground") + "};")
    def dsl_var(r, v):
        st = struct(r, v)
        if v == "insert": return f"    {st} {{\n        index: usize,\n        #[dsl(block)]\n        {r.item}: {r.struct},\n    }},\n"
        if v == "remove": return f"    {st} {{ index: usize }},\n"
        return f"    {st} {{\n        #[dsl(block)]\n        {r.item}: {r.struct},\n    }},\n"
    s = s.replace("    RemovePile { index: usize },\n}", "    RemovePile { index: usize },\n" + "".join(dsl_var(r, v) for r, v in NK) + "}", 1)
    def to_dsl(r, v):
        st = struct(r, v)
        if v == "insert": return f"        En1997Mutation::{st}(payload) => En1997MutationDsl::{st} {{ index: payload.index, {r.item}: payload.{r.item}.clone() }},\n"
        if v == "remove": return f"        En1997Mutation::{st}(payload) => En1997MutationDsl::{st} {{ index: payload.index }},\n"
        return f"        En1997Mutation::{st}(payload) => En1997MutationDsl::{st} {{ {r.item}: payload.{r.item}.clone() }},\n"
    s = s.replace("        En1997Mutation::RemovePile(payload) => En1997MutationDsl::RemovePile { index: payload.index },\n", "        En1997Mutation::RemovePile(payload) => En1997MutationDsl::RemovePile { index: payload.index },\n" + "".join(to_dsl(r, v) for r, v in NK), 1)
    def from_dsl(r, v):
        st, m = struct(r, v), mod(r, v)
        if v == "insert": return f"        En1997MutationDsl::{st} {{ index, {r.item} }} => En1997Mutation::{st}({m}::{st} {{ index, {r.item} }}),\n"
        if v == "remove": return f"        En1997MutationDsl::{st} {{ index }} => En1997Mutation::{st}({m}::{st} {{ index }}),\n"
        return f"        En1997MutationDsl::{st} {{ {r.item} }} => En1997Mutation::{st}({m}::{st} {{ {r.item} }}),\n"
    s = s.replace("        En1997MutationDsl::RemovePile { index } => En1997Mutation::RemovePile(remove_pile::RemovePile { index }),\n", "        En1997MutationDsl::RemovePile { index } => En1997Mutation::RemovePile(remove_pile::RemovePile { index }),\n" + "".join(from_dsl(r, v) for r, v in NK), 1)
    open(p, "w", encoding="utf8").write(s)

# binary rs
p = f"{S}/🚪️io/💾️binary/🧬️mutations/🦀️.rs"
s = open(p, encoding="utf8").read()
if "insert_wall" not in s:
    add = "".join(f"use semio_s_artifact_norm_en1997_mut_{r.group}::{mod(r, v)};\n" for r, v in NK)
    s = s.replace("use semio_s_artifact_norm_en1997_mut_foundations::remove_pile;\n", "use semio_s_artifact_norm_en1997_mut_foundations::remove_pile;\n" + add, 1)
    open(p, "w", encoding="utf8").write(s)
print("wired", len(NK))
