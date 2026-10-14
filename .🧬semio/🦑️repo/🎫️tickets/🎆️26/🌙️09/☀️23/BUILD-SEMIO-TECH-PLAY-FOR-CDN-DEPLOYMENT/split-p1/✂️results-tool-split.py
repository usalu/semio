#!/usr/bin/env python3
"""✂️ Ends the editor-edit <-> editor-results cycle (lane P1, S2h): the run-map read filtered by the simulation tool id
(`surface_energy_from_run`) moves beside that tool in the edit mode crate, so editor-results (the payload codec and the
colour ramp) names no edit-mode item; the window kind/body-key gate moves to the simulation window's own laws.
"""
import importlib.util, os, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
ED = f"{L.ANY}/✏️editor"; EDIT = f"{ED}/🎭️modes/✏️edit"; TOOL = f"{EDIT}/🛠️tools/⚡️simulation"; WIN = f"{EDIT}/🪟️windows/⚡️simulation"
def edit(path, *pairs):
    t = L.rd(path)
    for a, b in pairs:
        if t.count(a) != 1: sys.exit(f"anchor count {t.count(a)} in {path}: {a[:80]!r}")
        t = t.replace(a, b)
    L.wr(path, t)
FN = ('/// 🔓️ The live energy simulation run\'s per-surface map, if there is one. Filters on\n'
      '/// `tool_id == energySimulation` so another plugin\'s run in the same document never colours this one.\n'
      'pub fn surface_energy_from_run(run: Option<&ToolRunView>) -> Option<SurfaceEnergyMap> {\n'
      '    let run = run.filter(|run| run.tool_id == tools::simulation::TOOL_ID)?;\n'
      '    decode_run_payload(run.payload.as_deref()?)\n}\n')
edit(f"{ED}/📊️results/🦀️.rs", (FN, ""), ("use crate::editor_edit::tools;\n", ""), ("use semio_framework_plugin::ToolRunView;\n", ""))
edit(f"{ED}/📊️results/🧪️tests/🔬️unit/🦀️.rs",
     ('    assert_eq!(surface_energy_from_run(None), None, "no run at all");\n', ""),
     ('\n#[test]\nfn the_simulation_window_kind_and_body_key_equal_the_run_session_literals() {\n    assert_eq!(crate::session::ENERGY_SIMULATION_WINDOW_KIND_ID, crate::editor_edit::windows::simulation::WINDOW_KIND_ID);\n    assert_eq!(crate::session::ENERGY_SIMULATION_WINDOW_KIND_ID, crate::editor_edit::windows::simulation::BODY_KEY, "the simulation window\'s body key IS its kind id");\n}\n', ""))
tool_fn = FN.replace("tools::simulation::TOOL_ID", "TOOL_ID").replace("Option<&ToolRunView>", "Option<&semio_framework_plugin::ToolRunView>").replace("-> Option<SurfaceEnergyMap>", "-> Option<crate::editor_results::SurfaceEnergyMap>").replace("    decode_run_payload(", "    crate::editor_results::decode_run_payload(")
edit(f"{TOOL}/🦀️.rs", ("//#region 🧪️Tests\n", "//#region 🔓️RunMap\n" + tool_fn + "//#endregion 🔓️RunMap\n\n//#region 🧪️Tests\n"),
     ("(`crate::energy_simulation_session::EnergySimulationRunJob`)", "(`semio_s_artifact_energy_model_session::EnergySimulationRunJob`)"))
t = L.rd(f"{TOOL}/🧪️tests/🔬️unit/🦀️.rs").rstrip("\n")
L.wr(f"{TOOL}/🧪️tests/🔬️unit/🦀️.rs", t + '\n\n#[test]\nfn no_run_publishes_no_surface_map() {\n    assert_eq!(surface_energy_from_run(None), None, "no run at all");\n}\n')
t = L.rd(f"{WIN}/🧪️tests/🔬️unit/🦀️.rs").rstrip("\n")
L.wr(f"{WIN}/🧪️tests/🔬️unit/🦀️.rs", t + '\n\n#[test]\nfn the_window_kind_and_body_key_equal_the_run_session_literal() {\n    assert_eq!(crate::session::ENERGY_SIMULATION_WINDOW_KIND_ID, crate::windows::simulation::WINDOW_KIND_ID);\n    assert_eq!(crate::session::ENERGY_SIMULATION_WINDOW_KIND_ID, crate::windows::simulation::BODY_KEY, "the simulation window\'s body key IS its kind id");\n}\n')
edit(f"{ED}/🦀️.rs", ("crate::editor_results::surface_energy_from_run(", "crate::editor_edit::tools::simulation::surface_energy_from_run("))
edit(f"{L.ANY}/🧵️simulation-session/🦀️.rs", ("/// `semio_s_artifact_energy_model_editor_results` unit laws assert the simulation literal still equals the window's own\n/// kind id and body key.\n",
                                            "/// `semio_s_artifact_energy_model_editor_edit` simulation window laws assert the simulation literal still equals the\n/// window's own kind id and body key.\n"))
print("ok")
