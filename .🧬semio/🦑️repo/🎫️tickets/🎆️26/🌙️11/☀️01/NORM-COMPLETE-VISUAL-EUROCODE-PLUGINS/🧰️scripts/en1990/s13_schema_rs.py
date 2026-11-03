from spec import *
p = SCH + "/🦀️.rs"; t = open(p, encoding="utf8").read()
if "fatigue_spectra" in t: raise SystemExit("done")
M = "semio_s_artifact_norm_en1990_model::"
t = t.replace("    pub effects: Vec<semio_s_artifact_norm_en1990_model::MemberEffect>,\n}", "    pub effects: Vec<semio_s_artifact_norm_en1990_model::MemberEffect>,\n" + "".join(f"    #[state(artifact)]\n    pub {e['coll']}: Vec<{M}{e['name']}>,\n" for e in ENTITIES.values()) + "}", 1)
t = t.replace("            effects: self.effects.clone(),\n", "            effects: self.effects.clone(),\n" + "".join(f"            {e['coll']}: self.{e['coll']}.clone(),\n" for e in ENTITIES.values()), 1)
t = t.replace("            effects: snapshot.effects,\n", "            effects: snapshot.effects,\n" + "".join(f"            {e['coll']}: snapshot.{e['coll']},\n" for e in ENTITIES.values()), 1)
assert t.count("fatigue_spectra") == 5
open(p, "w", encoding="utf8").write(t)
