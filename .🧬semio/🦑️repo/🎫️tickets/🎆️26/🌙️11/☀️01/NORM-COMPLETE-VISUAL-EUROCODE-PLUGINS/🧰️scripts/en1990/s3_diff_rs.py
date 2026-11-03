from spec import *
p = SCH + "/🔺️diff/🦀️.rs"
t = open(p, encoding="utf8").read()
if "FatigueSpectrumDelta" in t: raise SystemExit("already applied")
M = "semio_s_artifact_norm_en1990_model::"
OLD = [("Permanent", "PermanentAction"), ("Variable", "VariableAction"), ("Accidental", "AccidentalAction"), ("Seismic", "SeismicAction"), ("Member", "Member"), ("BridgeSls", "BridgeSls")]
# 1. key markers
markers = ""
for short, row in OLD + [(e["name"], e["name"]) for e in ENTITIES.values()]:
    markers += f"/// 🔑️ Key extractor of the foreign `{row}` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct {short}Key;\n\nimpl protocol::list_delta::KeyOf<{M}{row}> for {short}Key {{\n    type Key = String;\n    fn key_of(row: &{M}{row}) -> String {{\n        row.id.clone()\n    }}\n}}\n\n"
t = t.replace("//#region 🔖️Rows\n", "//#region 🔖️Rows\n" + markers, 1)
for short, row in OLD:
    old = f"row: {M}{row}, patch: En1990{short}Patch, key: id }}"
    assert t.count(old) == 1, short
    t = t.replace(old, f"row: {M}{row}, patch: En1990{short}Patch, list: Vec<{M}{row}>, key: String = by {short}Key }}")
t = t.replace("set { category: String, qk: f64 } }", "set { category: String, qk: f64, group: String } }")
# 3. new rows
new = ""
for e in ENTITIES.values():
    n = e["name"]; fs = [f for f in e["fields"] if f["snake"] != "id"]
    sets = ", ".join(f"{f['snake']}: {f['ty']}" for f in fs)
    new += f"""
protocol::row_patch! {{
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `{n}`.
    pub En1990{n}Patch of {M}{n} {{ set {{ {sets} }} }}
}}

protocol::list_delta! {{
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `{n}` list.
    pub En1990{n}Delta {{ removal: En1990{n}Removal, insertion: En1990{n}Insertion, relocation: En1990{n}Relocation, modification: En1990{n}Modification, row: {M}{n}, patch: En1990{n}Patch, list: Vec<{M}{n}>, key: String = by {n}Key }}
}}
"""
t = t.replace("//#endregion 🔖️Rows", new.lstrip("\n").join(["", ""]) + "//#endregion 🔖️Rows", 1) if False else t.replace("//#endregion 🔖️Rows", new[1:] + "//#endregion 🔖️Rows", 1)
# 4. Diff struct
t = t.replace("    #[state(artifact)]\n    pub effects: Vec<En1990EffectEdit>,\n}", "    #[state(artifact)]\n    pub effects: Vec<En1990EffectEdit>,\n" + "".join(f"    #[state(artifact)]\n    pub {e['coll']}: En1990{e['name']}Delta,\n" for e in ENTITIES.values()) + "}")
# 5. algebra
t = t.replace("        for edit in other.effects {\n            push_row(&mut self.effects, edit);\n        }\n", "        for edit in other.effects {\n            push_row(&mut self.effects, edit);\n        }\n" + "".join(f"        self.{e['coll']}.absorb(other.{e['coll']});\n" for e in ENTITIES.values()))
t = t.replace("        apply_rows::<En1990EffectEdit>(&mut next.effects, &self.effects).map_err(|error| error.under([\"effects\"]))?;\n", "        apply_rows::<En1990EffectEdit>(&mut next.effects, &self.effects).map_err(|error| error.under([\"effects\"]))?;\n" + "".join(f"        next.{e['coll']} = self.{e['coll']}.commit_onto(&base.{e['coll']}, capability).map_err(|error| error.under([\"{e['coll_camel']}\"]))?;\n" for e in ENTITIES.values()))
t = t.replace("            && self.effects.is_empty()\n", "            && self.effects.is_empty()\n" + "".join(f"            && self.{e['coll']}.is_empty()\n" for e in ENTITIES.values()))
t = t.replace("            effects: invert_rows::<En1990EffectEdit>(&base.effects, &self.effects),\n", "            effects: invert_rows::<En1990EffectEdit>(&base.effects, &self.effects),\n" + "".join(f"            {e['coll']}: self.{e['coll']}.inverse(&base.{e['coll']}),\n" for e in ENTITIES.values()))
for k in ("pub fatigue_spectra: En1990FatigueSpectrumDelta", "self.test_series.absorb", "&& self.reliability_analyses.is_empty()", "reliability_analyses: self.reliability_analyses.inverse", "next.fatigue_spectra = "):
    assert k in t, k
open(p, "w", encoding="utf8").write(t)
print("diff rs ok")
