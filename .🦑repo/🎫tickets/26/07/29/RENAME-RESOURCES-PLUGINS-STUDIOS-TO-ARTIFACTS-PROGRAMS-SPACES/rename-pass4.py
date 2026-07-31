#!/usr/bin/env python3
"""Pass 4: fix broken rename artifacts and finish plugin/studio/resource terminology."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
SKIP_DIRS = {".git", "node_modules", "target", ".vscode-test", ".repo"}

REPLACEMENTS: list[tuple[str, str]] = [
    ("PluginWorld", "ProgramWorld"),
    ("LoadedPlugin", "LoadedProgram"),
    ("PluginHotSwapEvent", "ProgramHotSwapEvent"),
    ("PluginContributionEntry", "ProgramContributionEntry"),
    ("PluginSupervisorState", "ProgramSupervisorState"),
    ("PluginBridgeBackend", "ProgramBridgeBackend"),
    ("PluginBridgeEntry", "ProgramBridgeEntry"),
    ("plugin_bridge", "program_bridge"),
    ("OsStudioCatalogEntry", "OsSpaceCatalogEntry"),
    ("list_os_studio_catalog_entries", "list_os_space_catalog_entries"),
    ("seed_os_studio_catalog_if_empty", "seed_os_space_catalog_if_empty"),
    ("load_os_studio_document", "load_os_space_document"),
    ("import_os_studio_from_json", "import_os_space_from_json"),
    ("import_os_studio_from_pack", "import_os_space_from_pack"),
    ("export_os_studio_pack", "export_os_space_pack"),
    ("export_os_studio_dsl", "export_os_space_dsl"),
    ("delete_os_studio", "delete_os_space"),
    ("sync_os_studio_document", "sync_os_space_document"),
    ("sync_os_studio_document_helper", "sync_os_space_document_helper"),
    ("os_studio_backbone_uri", "os_space_backbone_uri"),
    ("os_studio_catalog_entry_from_document", "os_space_catalog_entry_from_document"),
    ("track_os_studio_backbone_uri", "track_os_space_backbone_uri"),
    ("untrack_os_studio_backbone_uri", "untrack_os_space_backbone_uri"),
    ("list_all_studio_catalog_entries", "list_all_space_catalog_entries"),
    ("open_file_studio_backbone", "open_file_space_backbone"),
    ("open_folder_studio_backbone", "open_folder_space_backbone"),
    ("register_program(", "register_workflow("),
    ("plugin://", "program://"),
    ("WasmPluginRuntime", "WasmProgramRuntime"),
    ("is_studio_mode", "is_space_mode"),
    ("build_studio_", "build_space_"),
    ("StudioPanel", "SpacePanel"),
    ("StudioProgram", "SpaceProgram"),
]

OS_CORE_FIXES: list[tuple[str, str]] = [
    (
        "pub fn load_program(&mut self, plugin: LoadedProgram)",
        "pub fn load_program(&mut self, program: LoadedProgram)",
    ),
    (
        "pub fn hot_swap_program(&mut self, plugin: LoadedProgram)",
        "pub fn hot_swap_program(&mut self, program: LoadedProgram)",
    ),
    (
        "fn validate_swap_apps(&self, plugin: &LoadedProgram)",
        "fn validate_swap_apps(&self, program: &LoadedProgram)",
    ),
    (
        "fn validate_swap_instances(&self, program_id: &str, plugin: &LoadedProgram)",
        "fn validate_swap_instances(&self, program_id: &str, program: &LoadedProgram)",
    ),
    (
        "fn validate_swap_app_retention(&self, plugin: &LoadedProgram, previous: Option<&LoadedProgram>)",
        "fn validate_swap_app_retention(&self, program: &LoadedProgram, previous: Option<&LoadedProgram>)",
    ),
    (
        "fn validate_swap_window_kinds(&self, plugin: &LoadedProgram)",
        "fn validate_swap_window_kinds(&self, program: &LoadedProgram)",
    ),
    (
        "fn plan_controller_rebindings(&self, program_id: &str, plugin: &LoadedProgram)",
        "fn plan_controller_rebindings(&self, program_id: &str, program: &LoadedProgram)",
    ),
    (
        "fn validate_program_manifest(plugin: &LoadedProgram)",
        "fn validate_program_manifest(program: &LoadedProgram)",
    ),
    (
        "OsProjection { workflows: Vec::new()",
        "OsProjection { programs: Vec::new()",
    ),
    (
        "//#region 🔖ProgramRegistry\n    #[derive(Clone, Debug, Default)]\n    pub struct ProgramRegistry {\n        instances: HashMap<String, OsAppInstance>,\n    }\n\n    impl ProgramRegistry {",
        "//#region 🔖WorkflowInstanceRegistry\n    #[derive(Clone, Debug, Default)]\n    pub struct WorkflowInstanceRegistry {\n        instances: HashMap<String, OsAppInstance>,\n    }\n\n    impl WorkflowInstanceRegistry {",
    ),
    ("//#endregion 🔖ProgramRegistry\n\n    //#region 🔖MediaExport", "//#endregion 🔖WorkflowInstanceRegistry\n\n    //#region 🔖MediaExport"),
    (
        "OsWorkflowNodeGraphPayload, OsMediaPort, ProgramRegistry, OS_MEDIA_FLOW_MODULE_ID",
        "OsWorkflowNodeGraphPayload, OsMediaPort, WorkflowInstanceRegistry, OS_MEDIA_FLOW_MODULE_ID",
    ),
    ("plugins: HashMap<String, LoadedProgram>", "programs: HashMap<String, LoadedProgram>"),
    ("self.plugins.", "self.programs."),
    ("&plugin.manifest", "&program.manifest"),
    ("validate_program_manifest(&plugin)", "validate_program_manifest(&program)"),
    ("validate_swap_apps(&plugin)", "validate_swap_apps(&program)"),
    ("validate_swap_instances(&program_id, &plugin)", "validate_swap_instances(&program_id, &program)"),
    ("validate_swap_app_retention(&plugin,", "validate_swap_app_retention(&program,"),
    ("validate_swap_window_kinds(&plugin)", "validate_swap_window_kinds(&program)"),
    ("plan_controller_rebindings(&program_id, &plugin)", "plan_controller_rebindings(&program_id, &program)"),
    ("for app in &plugin.manifest", "for app in &program.manifest"),
    ("for program in &plugin.manifest.workflows", "for workflow in &program.manifest.workflows"),
    ("register_workflow(program.clone())", "register_workflow(workflow.clone())"),
    (
        "for contribution in &plugin.manifest.contributions",
        "for contribution in &program.manifest.contributions",
    ),
    (
        "self.previous_program.as_ref().map(|plugin| program.manifest.version.clone())",
        "self.previous_program.as_ref().map(|previous| previous.manifest.version.clone())",
    ),
    ("host.plugins.insert", "host.programs.insert"),
    ("self.plugins.get", "self.programs.get"),
    ("for program in self.plugins.values()", "for loaded in self.programs.values()"),
    (
        "for contribution in &program.manifest.contributions {\n                    entries.push(ProgramContributionEntry { program_id: program.program_id.clone()",
        "for contribution in &loaded.manifest.contributions {\n                    entries.push(ProgramContributionEntry { program_id: loaded.program_id.clone()",
    ),
    ("let next_app_ids: HashSet<String> = program.manifest.apps", "let next_app_ids: HashSet<String> = program.manifest.apps"),
    (
        "if previous.manifest.version == program.manifest.version && previous.manifest.apps.len() > program.manifest.apps.len()",
        "if previous.manifest.version == program.manifest.version && previous.manifest.apps.len() > program.manifest.apps.len()",
    ),
]


def should_skip(path: Path) -> bool:
    return any(part in SKIP_DIRS for part in path.parts)


def iter_files() -> list[Path]:
    files: list[Path] = []
    for path in ROOT.rglob("*"):
        if not path.is_file() or should_skip(path):
            continue
        if path.suffix in {
            ".rs",
            ".ts",
            ".tsx",
            ".js",
            ".json",
            ".md",
            ".wit",
            ".toml",
            ".go",
        }:
            files.append(path)
    return files


def apply_replacements(text: str, replacements: list[tuple[str, str]]) -> str:
    for old, new in replacements:
        text = text.replace(old, new)
    return text


def main() -> None:
    touched: list[str] = []
    for path in iter_files():
        rel = path.relative_to(ROOT).as_posix()
        if rel.startswith(".repo/"):
            continue
        original = path.read_text(encoding="utf-8", errors="surrogateescape")
        updated = apply_replacements(original, REPLACEMENTS)
        if rel == "framework/product/os/core/rs/lib.rs":
            updated = apply_replacements(updated, OS_CORE_FIXES)
            updated = re.sub(
                r"let next_app_ids: HashSet<String> = program\.manifest\.apps\.iter\(\)\.map\(\|app\| app\.id\.clone\(\)\)\.collect\(\);\n            let previous_app_ids: HashSet<String> = self\.programs\.get\(program_id\)",
                "let next_app_ids: HashSet<String> = program.manifest.apps.iter().map(|app| app.id.clone()).collect();\n            let previous_app_ids: HashSet<String> = self.programs.get(program_id)",
                updated,
            )
            updated = updated.replace(
                "let apps_by_id: HashMap<&str, &AppDefinition> = program.manifest.apps.iter().map(|app| (app.id.as_str(), app)).collect();",
                "let apps_by_id: HashMap<&str, &AppDefinition> = program.manifest.apps.iter().map(|app| (app.id.as_str(), app)).collect();",
            )
        if updated != original:
            path.write_text(updated, encoding="utf-8")
            touched.append(rel)
    report = {"pass": 4, "files_touched": len(touched), "files": touched}
    out = Path(__file__).with_name("rename-pass4-report.json")
    out.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
