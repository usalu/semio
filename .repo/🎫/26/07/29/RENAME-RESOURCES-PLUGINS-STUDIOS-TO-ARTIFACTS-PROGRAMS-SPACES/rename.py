#!/usr/bin/env python3
"""Rename resources→artifacts, plugins→programs, studios→spaces across the active codebase."""

from __future__ import annotations

import json
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent
assert (ROOT / "framework").is_dir(), ROOT

SKIP_DIR_NAMES = {
    ".git", ".repo", "node_modules", "target", "dist", "build", ".next",
    "coverage", "__pycache__", ".turbo", "out", ".venv", ".claude",
    ".vscode-test", "Visual Studio Code.app",
}
SKIP_FILE_SUFFIXES = {
    ".png", ".jpg", ".jpeg", ".gif", ".webp", ".ico", ".pdf", ".zip", ".wasm",
    ".bin", ".lock", ".woff", ".woff2", ".ttf", ".otf", ".mp4", ".webm",
    ".glb", ".gltf", ".hdr", ".exr",
}
TEXT_SUFFIXES = {
    ".rs", ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".json", ".md", ".mdx",
    ".toml", ".wit", ".cs", ".go", ".py", ".graphql", ".gql", ".yml", ".yaml",
    ".css", ".scss", ".html", ".svg", ".txt", ".snap", ".wgsl", ".sh", ".ps1",
    ".bat", ".cmd", ".editorconfig", ".gitignore", ".npmrc", ".s", ".dag",
    ".gismap", ".map", ".plan.md",
}

REPLACEMENTS: list[tuple[str, str]] = [
    # protect names that contain ProgramDefinition but aren't workflow steps
    ("buildSketchpadProgramDefinition", "buildSketchpadPlatformDefinition"),
    # workflow step (was ProgramDefinition) — before plugin→program
    ("ProgramDefinition", "WorkflowDefinition"),
    ("pub program_id: String,", "pub workflow_step_id: String,"),
    ("program_id: program_id.into()", "workflow_step_id: workflow_step_id.into()"),
    ("program_id: &str", "workflow_step_id: &str"),
    ("program_id, label", "workflow_step_id, label"),
    ("find_program(&self, program_id:", "find_workflow(&self, workflow_step_id:"),
    ("register_program(&mut self, program: WorkflowDefinition)", "register_workflow(&mut self, workflow: WorkflowDefinition)"),
    ("pub fn programs(&self) -> Vec<WorkflowDefinition>", "pub fn workflows(&self) -> Vec<WorkflowDefinition>"),
    ("programs: HashMap<String, WorkflowDefinition>", "workflows: HashMap<String, WorkflowDefinition>"),
    ("pub programs: Vec<WorkflowDefinition>", "pub workflows: Vec<WorkflowDefinition>"),
    ("programs: vec![]", "workflows: vec![]"),
    ("programs: Vec::new()", "workflows: Vec::new()"),
    (".programs()", ".workflows()"),
    (".programs ", ".workflows "),
    ("self.program = Some(WorkflowDefinition", "self.workflow = Some(WorkflowDefinition"),
    ("pub program: Option<WorkflowDefinition>", "pub workflow: Option<WorkflowDefinition>"),
    ("program: None", "workflow: None"),
    ("fn program(mut self, program_id:", "fn workflow(mut self, workflow_step_id:"),
    # studio → space (specific identifiers first)
    ("StudioHistoryProjection", "SpaceHistoryProjection"),
    ("StudioHistoryOperation", "SpaceHistoryOperation"),
    ("StudioHistoryDiff", "SpaceHistoryDiff"),
    ("StudioMemberPin", "SpaceMemberPin"),
    ("StudioMember", "SpaceMember"),
    ("StudioHost", "SpaceHost"),
    ("StudioCheckpoint", "SpaceCheckpoint"),
    ("StudioAlternative", "SpaceAlternative"),
    ("StudioConflict", "SpaceConflict"),
    ("StudioRunner", "SpaceRunner"),
    ("StudioRunError", "SpaceRunError"),
    ("StudioBundle", "SpaceBundle"),
    ("commit_studio_checkpoint", "commit_space_checkpoint"),
    ("checkout_studio_checkpoint", "checkout_space_checkpoint"),
    ("create_studio_alternative", "create_space_alternative"),
    ("switch_studio_alternative", "switch_space_alternative"),
    ("OS_STUDIO_BACKBONE_URI_PREFIX", "OS_SPACE_BACKBONE_URI_PREFIX"),
    ("OS_STUDIO_SCHEMA", "OS_SPACE_SCHEMA"),
    ("STUDIO_FOLDER_DOCUMENT_ID", "SPACE_FOLDER_DOCUMENT_ID"),
    ("os.studio.history", "os.space.history"),
    ("s.studio", "s.space"),
    ("studio-history", "space-history"),
    ("studio://", "space://"),
    ("create_ephemeral_os_studio", "create_ephemeral_os_space"),
    ("create_os_studio", "create_os_space"),
    ("create_empty_os_document(\"studio\"", "create_empty_os_document(\"space\""),
    ("create_os_id(\"studio\")", "create_os_id(\"space\")"),
    ("studio-checkpoint", "space-checkpoint"),
    ("studio-alternative", "space-alternative"),
    ("unknown os studio:", "unknown os space:"),
    ("studio plugin missing", "space program missing"),
    ("studio session missing", "space session missing"),
    ("studio_id", "space_id"),
    ("studio_checkpoint", "space_checkpoint"),
    ("studio.canvas.home", "space.canvas.home"),
    ("studio.canvas.back", "space.canvas.back"),
    ("studio-wide", "space-wide"),
    ("studio-level", "space-level"),
    ("studio history", "space history"),
    ("studio undo", "space undo"),
    ("studio edits", "space edits"),
    ("Ephemeral Studio", "Ephemeral Space"),
    ("Catalog Studio", "Catalog Space"),
    ("\"Studio\"", "\"Space\""),
    ("group: \"Studio\"", "group: \"Space\""),

    # OS resource → artifact
    ("ResourceKindSpec", "ArtifactKindSpec"),
    ("resource_kinds", "artifact_kinds"),
    ("resource_kind", "artifact_kind"),
    ("OsResourceDescriptor", "OsArtifactDescriptor"),
    ("OsResourceKindId", "OsArtifactKindId"),
    ("ResourceKindEntry", "ArtifactKindEntry"),
    ("register_resource_descriptor", "register_artifact_descriptor"),
    ("os_resource_descriptor", "os_artifact_descriptor"),
    ("list_os_resources", "list_os_artifacts"),
    ("OsProgramResourceMap", "OsProgramArtifactMap"),
    ("osBaselineResource", "osBaselineArtifact"),
    ("resources.manifest.json", "artifacts.manifest.json"),
    ("file-resource", "file-artifact"),
    ("label: \"Resource\"", "label: \"Artifact\""),
    ("resource catalog", "artifact catalog"),
    ("resource registry", "artifact registry"),
    ("resource map", "artifact map"),
    ("resource_by_app_id", "artifact_by_app_id"),
    ("fallback_resource", "fallback_artifact"),
    ("ResourceKind", "ArtifactKind"),
    ("ResourceId", "ArtifactId"),
    ("cap.resource", "cap.artifact"),
    ("pub resource: ArtifactKind", "pub artifact: ArtifactKind"),
    ("pub resource: ArtifactId", "pub artifact: ArtifactId"),

    # plugin → program
    ("PluginManifestJson", "ProgramManifestJson"),
    ("PluginManifest", "ProgramManifest"),
    ("PluginBundle", "ProgramBundle"),
    ("PluginError", "ProgramError"),
    ("PluginInstanceId", "ProgramInstanceId"),
    ("PluginRegistry", "ProgramRegistry"),
    ("PluginHostMetadata", "ProgramHostMetadata"),
    ("PluginRegistryEntry", "ProgramRegistryEntry"),
    ("PluginHost", "ProgramHost"),
    ("PluginApp", "ProgramApp"),
    ("semio-framework-plugin", "semio-framework-program"),
    ("semio_framework_plugin", "semio_framework_program"),
    ("framework/plugin", "framework/program"),
    ("plugin-registry", "program-registry"),
    ("@semio-tech/plugin-registry", "@semio-tech/program-registry"),
    ("plugin-world", "program-world"),
    ("plugin_exports!", "program_exports!"),
    ("install_plugin_bundle", "install_program_bundle"),
    ("plugin_manifest", "program_manifest"),
    ("active_plugin_manifest", "active_program_manifest"),
    ("activePluginManifest", "activeProgramManifest"),
    ("__semio_plugin_bundle", "__semio_program_bundle"),
    ("plugin instance busy", "program instance busy"),
    ("plugin not initialized", "program not initialized"),
    ("CommandScope::Plugin", "CommandScope::Program"),
    ("Scope::Plugin", "Scope::Program"),
    ("plugin_id", "program_id"),
    ("findPluginCargoFiles", "findProgramCargoFiles"),
    ("parsePluginCargo", "parseProgramCargo"),
    ("isPluginCrate", "isProgramCrate"),
    ("plugins.json", "programs.json"),
    ("plugins.ts", "programs.ts"),
    ("os-plugins", "os-programs"),
    ("plugins.stories", "programs.stories"),
    ("trait Plugin:", "trait Program:"),
    ("pub trait Plugin", "pub trait Program"),
    ("impl Plugin for", "impl Program for"),
    ("GuestPlugin", "GuestProgram"),
    ("interface plugin {", "interface program {"),
    ("plugin-error", "program-error"),
    ("plugin-manifest-json", "program-manifest-json"),
    ("semio:framework::plugin", "semio:framework::program"),
    ("world plugin-world", "world program-world"),
    ("export plugin;", "export program;"),
    ("architect_program", "architect_spine"),
    ("architect-program", "architect-spine"),
    ("architect/program/rs", "architect/spine/rs"),
    ("architect/plugin", "architect/program"),
    ("plugin/", "program/"),
    ("-plugin", "-program"),
    ("_plugin", "_program"),
]

def should_skip_dir(path: Path) -> bool:
    return bool(set(path.parts) & SKIP_DIR_NAMES)

def iter_text_files() -> list[Path]:
    files: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dp = Path(dirpath)
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        if should_skip_dir(dp):
            continue
        for name in filenames:
            p = dp / name
            if p.suffix in SKIP_FILE_SUFFIXES:
                continue
            if p.suffix in TEXT_SUFFIXES or name in {".gitignore", "Cargo.lock", "bun.lock"}:
                files.append(p)
    return files

def rename_directories() -> list[str]:
    log: list[str] = []
    spine_src = ROOT / "architect" / "program"
    spine_dst = ROOT / "architect" / "spine"
    if spine_src.is_dir() and not spine_dst.exists():
        spine_src.rename(spine_dst)
        log.append("architect/program -> architect/spine")

    plugin_dirs: list[Path] = []
    for dirpath, dirnames, _ in os.walk(ROOT):
        dp = Path(dirpath)
        if should_skip_dir(dp) or ".repo" in dp.parts:
            dirnames[:] = []
            continue
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        if "plugin" in dirnames:
            plugin_dirs.append(dp / "plugin")
    plugin_dirs.sort(key=lambda p: len(p.parts), reverse=True)
    for src in plugin_dirs:
        dst = src.parent / "program"
        if src.is_dir() and not dst.exists():
            src.rename(dst)
            log.append(f"{src.relative_to(ROOT)} -> program")
    return log

def rename_files() -> list[str]:
    log: list[str] = []
    src = ROOT / "s" / "manifest" / "resources.manifest.json"
    dst = ROOT / "s" / "manifest" / "artifacts.manifest.json"
    if src.is_file() and not dst.exists():
        src.rename(dst)
        log.append("s/manifest/resources.manifest.json -> artifacts.manifest.json")
    return log

def main() -> None:
    dir_log = rename_directories()
    file_log = rename_files()
    touched: list[str] = []
    for path in iter_text_files():
        try:
            original = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        updated = original
        for old, new in REPLACEMENTS:
            updated = updated.replace(old, new)
        if updated != original:
            path.write_text(updated, encoding="utf-8")
            touched.append(str(path.relative_to(ROOT)))
    report = {"directories": dir_log, "files_renamed": file_log, "files_touched": len(touched)}
    (TICKET / "rename-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))

if __name__ == "__main__":
    main()
