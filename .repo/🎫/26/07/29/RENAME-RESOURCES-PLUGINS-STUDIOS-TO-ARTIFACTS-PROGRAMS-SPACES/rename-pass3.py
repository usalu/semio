#!/usr/bin/env python3
"""Fix over-replacement: program bundle ids stay program_id; only WorkflowDefinition keeps workflow_step_id."""
from __future__ import annotations
import json, os, re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent
SKIP = {".git", ".repo", "node_modules", "target", "dist", ".venv", ".claude", ".vscode-test"}

def iter_files():
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dp = Path(dirpath)
        if set(dp.parts) & SKIP:
            dirnames[:] = []
            continue
        dirnames[:] = [d for d in dirnames if d not in SKIP]
        for name in filenames:
            p = dp / name
            if p.suffix in {".rs", ".ts", ".tsx", ".js", ".json", ".wit"}:
                yield p

def fix_content(text: str) -> str:
    text = text.replace("pub workflow_step_id: String,\n    pub label: String,\n    pub version: String,", "pub program_id: String,\n    pub label: String,\n    pub version: String,")
    text = re.sub(
        r"(struct (?:LoadedPlugin|PluginHotSwapEvent|PluginContributionEntry|PluginRuntimeHandle|WasmPluginHandle|LoadedWasmPlugin|PluginSupervisorEntry)[^{]*\{[^}]*?)workflow_step_id",
        r"\1program_id",
        text,
        flags=re.DOTALL,
    )
    patterns_to_program_id = [
        "merge_os_program_definition(workflow_step_id",
        "os_program_by_id(workflow_step_id",
        "os_app_registration(workflow_step_id",
        "resolve_os_app_definition(workflow_step_id",
        "supervisor_state(&self, workflow_step_id",
        "recovery_ui(&self, workflow_step_id",
        "validate_swap_instances(&self, workflow_step_id",
        "plan_controller_rebindings(&self, workflow_step_id",
        "spawn_app_instance(&mut self, workflow_step_id",
        "host_state(workflow_step_id",
        "spawn_program(&mut self, workflow_step_id",
        "ui_recovery_panel(workflow_step_id",
        "ui_external_slot(\n    program_id",
        "parse_playgrounds_text(text: &str, workflow_step_id",
        "discover_examples_for_playground(root: &Path, crate_path: &str, workflow_step_id",
        '"workflow_step_id": workflow_step_id',
        "workflow_step_id: workflow_step_id.into(),\n        app_id:",
        "workflow_step_id: &str,\n        manifest:",
        "workflow_step_id: &str,\n        app_id:",
        "workflow_step_id: &str,\n        mut view_state",
        "workflow_step_id: &str,\n        plugin:",
        "workflow_step_id: &str,\n        version:",
        "workflow_step_id: &str,\n        contribution:",
        "workflow_step_id: &str,\n        label:",
        "workflow_step_id: &str,\n        quarantined:",
        "workflow_step_id: &str,\n        variant:",
        "workflow_step_id: &str,\n        crate_path:",
        "workflow_step_id: &str,\n        position:",
        "workflow_step_id: &str) -> UiNode",
        "pub workflow_step_id: String,\n    pub manifest:",
        "pub workflow_step_id: String,\n    pub artifact_uri:",
        "pub workflow_step_id: String,\n    pub version:",
        "pub workflow_step_id: String,\n    pub contribution:",
        "pub workflow_step_id: String,\n    pub wasm",
        "pub workflow_step_id: String,\n    pub module",
        "pub workflow_step_id: String,\n    pub state:",
        "pub workflow_step_id: String,\n    pub app_id: String,\n    pub body_key:",
        "programId\": program_id",
    ]
    for p in patterns_to_program_id:
        text = text.replace(p, p.replace("workflow_step_id", "program_id"))
    text = text.replace("UiExternalSlotNode {\n        workflow_step_id:", "UiExternalSlotNode {\n        program_id:")
    text = text.replace("pub struct UiExternalSlotNode {\n    pub workflow_step_id:", "pub struct UiExternalSlotNode {\n    pub program_id:")
    text = text.replace("workflow_step_id: workflow_step_id.into(),\n        app_id: app_id.into(),\n        body_key:", "program_id: program_id.into(),\n        app_id: app_id.into(),\n        body_key:")
    text = text.replace('let args = Some(serde_json::json!({ "programId": program_id }));', 'let args = Some(serde_json::json!({ "programId": program_id }));')
    text = text.replace("pub fn ui_recovery_panel(workflow_step_id: &str", "pub fn ui_recovery_panel(program_id: &str")
    text = text.replace("    pub workflow_step_id: String,\n    pub workflow_step_id: String,", "    pub program_id: String,\n    pub app_id: String,")
    return text

def main():
    touched = []
    for path in iter_files():
        try:
            original = path.read_text(encoding="utf-8")
        except Exception:
            continue
        updated = fix_content(original)
        if updated != original:
            path.write_text(updated, encoding="utf-8")
            touched.append(str(path.relative_to(ROOT)))
    (TICKET / "rename-pass3-report.json").write_text(json.dumps({"touched": len(touched), "files": touched[:100]}, indent=2))
    print(len(touched))

if __name__ == "__main__":
    main()
