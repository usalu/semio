#!/usr/bin/env python3
"""Idempotent surgical splice of the `plan-linework` and `diagnostics` fields into the shared inference aggregate (label i-plan-diagnostics)."""
import re
path = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"
s = open(path, encoding="utf8").read()
if "compute_plan_linework" in s:
    print("already spliced")
    raise SystemExit(0)
s = s.replace("\n//#region 🔖️Inference", "\nuse super::diagnostics::{compute_diagnostics, Diagnostic};\nuse super::plan_linework::{compute_plan_linework, PlanLinework};\n\n//#region 🔖️Inference", 1)
start = s.index("pub struct ModelInference {")
end = s.index("\n}\n", start)
s = s[:end] + "\n    #[derived]\n    pub plan_linework: BTreeMap<String, PlanLinework>,\n    #[derived]\n    pub diagnostics: Vec<Diagnostic>," + s[end:]
s = s.replace("Ok(Self { ", "Ok(Self { plan_linework: compute_plan_linework(snapshot), diagnostics: compute_diagnostics(snapshot), ", 1)
marker = "        ]\n    }\n}\n//#endregion 🔖️Inference"
assert marker in s
rows = '            protocol::InferenceFieldSpec { id: "s.bim.model.inference.plan-linework", reads: super::plan_linework::READS },\n            protocol::InferenceFieldSpec { id: "s.bim.model.inference.diagnostics", reads: super::diagnostics::READS },\n'
s = s.replace(marker, rows + marker, 1)
open(path, "w", encoding="utf8").write(s)
print("spliced")
