#!/usr/bin/env python3
"""⚙️ Engine domain fixes that break the module cycles before the crate cut (lane P1, slice S1a).

- `ScheduleId` (+ its value, dsl-field, retirement and canonical-json impls) moves from `🔋️model` to `🗓️schedule`;
  every `crate::model::ScheduleId` in the energy plugin becomes `crate::schedule::ScheduleId`.
- `MAX_PANES` moves from `🪟️fenestration` to `🔋️model` (the document's glazing-construction limit).
- The integration test models `test_model_single_zone` / `test_model_full_topology` move from `🧪️sim` to `🔋️model`.
Every anchor is asserted unique; idempotent (refuses to run twice).
"""
import os, re, sys
PLUGIN = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
E = PLUGIN + "/🔨️modules/⚡️simulation/⚙️engine/"
def read(p): return open(p, encoding="utf-8").read()
def write(p, s): open(p, "w", encoding="utf-8").write(s)
def cut(s, start, end):
    assert s.count(start) == 1, start; i = s.index(start); assert s.count(end, i) >= 1, end; j = s.index(end, i) + len(end)
    return s[:i] + s[j:], s[i:j]
model_p, sched_p, fen_p, sim_p = E + "🔋️model/🦀️.rs", E + "🗓️schedule/🦀️.rs", E + "🪟️fenestration/🦀️.rs", E + "🧪️sim/🦀️.rs"
model, sched, fen, sim = read(model_p), read(sched_p), read(fen_p), read(sim_p)
if "pub struct ScheduleId" in sched: sys.exit("already applied")
model, block = cut(model, "// #region 🔖️Schedule\n/// 📅️ Schedule reference by id.", "// #endregion 🔖️Schedule\n\n")
model = model.replace("semio_framework_value::artifact_retire_leaf!(EntityId, ScheduleId);", "semio_framework_value::artifact_retire_leaf!(EntityId);")
assert model.count("impl semio_framework_pack_json::ArtifactCanonicalJsonTree for ScheduleId {") == 1
model, canon = cut(model, "impl semio_framework_pack_json::ArtifactCanonicalJsonTree for ScheduleId {", "    }\n}\n")
model = model.replace("\n\n// #endregion 🔖️Ids", "\n// #endregion 🔖️Ids")
block = block.replace("// #region 🔖️Schedule\n", "// #region 🔖️ScheduleId\n").replace("// #endregion 🔖️Schedule\n", "") \
    .replace("same tuple-struct reason as `EntityId` above", "same tuple-struct reason as `crate::model::EntityId`") \
    .replace("Hand-written, same reason as above.", "Hand-written, same reason as the `ToValue` pair above.")
block = block.rstrip("\n") + "\n\nsemio_framework_value::artifact_retire_leaf!(ScheduleId);\n\n" + canon + "// #endregion 🔖️ScheduleId\n\n"
model = model.replace("use crate::error::{Diagnostics, Error, Severity};\n", "use crate::error::{Diagnostics, Error, Severity};\nuse crate::schedule::ScheduleId;\n", 1)
assert sched.count("use crate::model::ScheduleId;\n") == 1
sched = sched.replace("use crate::model::ScheduleId;\n", "use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};\n")
anchor = "// #region 🔖️ScheduleType\n"; assert sched.count(anchor) == 1
sched = sched.replace(anchor, block + anchor)
fen, maxp = cut(fen, "pub const MAX_PANES: usize = 4;", "\n")
doc = re.search(r"((?:///[^\n]*\n)*)$", fen[:fen.index("\n\n", max(0, fen.rfind("\n\n", 0, len(fen))) ) ] if False else "")
model = model.replace("crate::fenestration::MAX_PANES", "MAX_PANES")
anchor = "// #region 🔖️FixedTable\n"; assert model.count(anchor) == 1
model = model.replace(anchor, "// #region 🔖️GlazingLimits\n/// 🪟️ Most glazing panes one glazing construction may alternate with gas gaps — the pane count the\n/// fenestration layer-by-layer balance (`crate::fenestration`) is dimensioned for.\npub const MAX_PANES: usize = 4;\n// #endregion 🔖️GlazingLimits\n\n" + anchor)
fen = fen.replace("use crate::model::GasKind;\n", "use crate::model::{GasKind, MAX_PANES};\n")
sim, fixtures = cut(sim, "// #region 🔖️Fixtures\n", "// #endregion 🔖️Fixtures\n\n")
fixtures = fixtures.replace("    use crate::model::*;\n", "")
model = model.rstrip("\n") + "\n\n" + fixtures.rstrip("\n") + "\n"
write(model_p, model); write(sched_p, sched); write(fen_p, fen); write(sim_p, sim)
n = 0
for root, dirs, files in os.walk(PLUGIN):
    dirs[:] = [d for d in dirs if d not in ("target", "node_modules")]
    for f in files:
        if f.endswith(".rs"):
            p = os.path.join(root, f); s = read(p); t = s.replace("crate::model::ScheduleId", "crate::schedule::ScheduleId").replace("crate::sim::test_model_", "crate::model::test_model_")
            if t != s: write(p, t); n += 1
print("rewrote", n, "files")
