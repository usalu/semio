"""🧵 EN 1994 v10: add the snapshot leaf `beams[].reinforcementProduct` (String, hot_rolled|cold_worked, default hot_rolled) through schema, twins, diff, Rust model, SQLite and field-meta by text surgery; the mutation kind, fixtures and assets are separate scripts. Idempotent."""
import json, re

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/"
S = ANY + "🧬️schema/"


def sub(path, old, new, count=1):
    text = open(path, encoding="utf-8").read()
    if new in text:
        return
    assert text.count(old) == count, (path, old[:80], text.count(old))
    open(path, "w", encoding="utf-8").write(text.replace(old, new))


def sub_re(path, pattern, repl, flags=0):
    text = open(path, encoding="utf-8").read()
    new, n = re.subn(pattern, repl, text, count=1, flags=flags)
    assert n == 1, (path, pattern)
    if new != text:
        open(path, "w", encoding="utf-8").write(new)


sub(S + "📸️snapshot/🦀️.rs", "    pub end_zone_length_m: f64,\n    #[dsl(table)]\n    pub actions: Vec<CharacteristicAction>,",
    "    pub end_zone_length_m: f64,\n    /// 🕸️ hot_rolled | cold_worked hogging reinforcement (EN 1992-1-2 Table 3.2a strength reduction in fire).\n    pub reinforcement_product: String,\n    #[dsl(table)]\n    pub actions: Vec<CharacteristicAction>,")
sub(S + "📸️snapshot/🦀️.rs", "end_zone_length_m: 0.0, actions,", 'end_zone_length_m: 0.0, reinforcement_product: "hot_rolled".into(), actions,')

for proto in (S + "🛰️.proto", S + "📸️snapshot/🛰️.proto"):
    sub(proto, "  double end_zone_length_m = 27;\n  repeated CharacteristicAction actions = 28;", "  double end_zone_length_m = 27;\n  string reinforcement_product = 28;\n  repeated CharacteristicAction actions = 29;")
for ts in (S + "🟦️.ts", S + "📸️snapshot/🟦️.ts"):
    sub(ts, "  endZoneLengthM: number;\n  actions: CharacteristicAction[];", "  endZoneLengthM: number;\n  reinforcementProduct: string;\n  actions: CharacteristicAction[];")
    sub(ts, "endZoneLengthM: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicAction)", "endZoneLengthM: normWireRequired(normWireNumber), reinforcementProduct: normWireRequired(normWireString), actions: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicAction)")
for gql in (S + "🔗️.graphql", S + "📸️snapshot/🔗️.graphql"):
    sub(gql, "  endZoneLengthM: Float!\n  actions: [CharacteristicAction!]!", "  endZoneLengthM: Float!\n  reinforcementProduct: String!\n  actions: [CharacteristicAction!]!")

D = S + "🔺️diff/"
sub(D + "🛰️.proto", "  optional double end_zone_length_m = 44;\n  optional En1994BeamsActionsRows actions = 45;", "  optional double end_zone_length_m = 44;\n  optional string reinforcement_product = 45;\n  optional En1994BeamsActionsRows actions = 46;")
sub(D + "🟦️.ts", "  endZoneLengthM?: number | null;\n  actions?: En1994BeamsActionsRows | null;", "  endZoneLengthM?: number | null;\n  reinforcementProduct?: string | null;\n  actions?: En1994BeamsActionsRows | null;")
sub(D + "🔗️.graphql", "  endZoneLengthM: JSON\n  actions: En1994BeamsActionsRows", "  endZoneLengthM: JSON\n  reinforcementProduct: JSON\n  actions: En1994BeamsActionsRows")
sub(D + "🏗️beam-rows/🦀️.rs", "end_zone_length_m: f64 }\n", "end_zone_length_m: f64, reinforcement_product: String }\n")

text = open(D + "🟦️.ts", encoding="utf-8").read()
m = re.search(r"endZoneLengthM: normWireOptional\(normWireNullable\(normWireNumber\)\), (actions: normWireOptional\(normWireNullable\(normWireRef\(\(\) => parseEn1994BeamsActionsRows)", text)
assert m or "reinforcementProduct: normWireOptional" in text.split("parseEn1994BeamsPatch", 1)[1][:3000]
if m:
    open(D + "🟦️.ts", "w", encoding="utf-8").write(text.replace(m.group(0), "endZoneLengthM: normWireOptional(normWireNullable(normWireNumber)), reinforcementProduct: normWireOptional(normWireNullable(normWireString)), " + m.group(1), 1))

for schema in (S + "🔣️.json", S + "📸️snapshot/🔣️.json"):
    d = json.load(open(schema, encoding="utf-8"))
    beam = d["$defs"]["CompositeBeam"]
    if "reinforcementProduct" in beam["properties"]:
        continue
    beam["required"].insert(beam["required"].index("actions"), "reinforcementProduct")
    prop = json.loads(json.dumps(d["$defs"]["CompositeColumn"]["properties"]["reinforcementProduct"]))
    prop["x-semio-ui"]["description"] = {"en": "hot_rolled or cold_worked hogging reinforcement (EN 1992-1-2 Table 3.2a: the strength reduction k_s,θ of the bars in fire).", "de": "hot_rolled (warmgewalzt) oder cold_worked (kaltverformt) nach EN 1992-1-2 Tabelle 3.2a: Festigkeitsabminderung k_s,θ der Stützbewehrung im Brandfall."}
    props, out = beam["properties"], {}
    for key, value in props.items():
        if key == "actions":
            out["reinforcementProduct"] = prop
        out[key] = value
    beam["properties"] = out
    open(schema, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")

path = D + "🔣️.json"
text = open(path, encoding="utf-8").read()
if '"En1994BeamsPatch"' in text and "CompositeBeam/properties/reinforcementProduct" not in text:
    head, rest = text.split('"En1994BeamsPatch":', 1)
    assert '"endZoneLengthM","actions"]' in rest
    rest = rest.replace('"endZoneLengthM","actions"]', '"endZoneLengthM","reinforcementProduct","actions"]', 1)
    anchor = '"endZoneLengthM":{"anyOf":[{"$ref":"https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/CompositeBeam/properties/endZoneLengthM"},{"type":"null"}]}'
    assert rest.count(anchor) == 1
    rest = rest.replace(anchor, anchor + ',"reinforcementProduct":{"anyOf":[{"$ref":"https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/CompositeBeam/properties/reinforcementProduct"},{"type":"null"}]}', 1)
    open(path, "w", encoding="utf-8").write(head + '"En1994BeamsPatch":' + rest)

sub(ANY + "✏️editor/🏷️field-meta/🦀️.rs", '    ("beams[].actions",', '    ("beams[].reinforcementProduct", mc("Reinforcement product", "Bewehrungsprodukt", REBAR_PRODUCT)),\n    ("beams[].actions",')

SQ = ANY + "🚪️io/🪶️sqlite/📸️snapshot/"
sub(SQ + "🗄️.sql", "end_zone_length_m REAL,span_m_ieee754_bits", "end_zone_length_m REAL,reinforcement_product TEXT NOT NULL,span_m_ieee754_bits")
sub(SQ + "🦀️.rs", "Cell::Real(b.end_zone_length_m)", "Cell::Real(b.end_zone_length_m),Cell::Text(&b.reinforcement_product)")
sub(SQ + "🦀️.rs", "end_zone_length_m:b.real(26)?,", "end_zone_length_m:b.real(26)?,reinforcement_product:reconstruct_text(control,b.text(27)?)?,")
sub(SQ + "🟦️.ts", "b.endZoneLengthM],BEAM", "b.endZoneLengthM,b.reinforcementProduct],BEAM")
sub(SQ + "🟦️.ts", "endZoneLengthM:readBinary64(b,26,BEAM),", "endZoneLengthM:readBinary64(b,26,BEAM),reinforcementProduct:artifactSqliteText(b,27),")
print("ok")
