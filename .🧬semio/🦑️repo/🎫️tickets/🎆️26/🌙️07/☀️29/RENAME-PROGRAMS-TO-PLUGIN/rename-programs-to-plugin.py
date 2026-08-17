#!/usr/bin/env python3
"""Rename program → plugin across the active codebase. No legacy program naming for WASM plugins."""

from __future__ import annotations

import json
import os
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent

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

PROTECTED: list[tuple[str, str]] = [
    ("WorkflowDefinition", "__KEEP_WorkflowDefinition__"),
    ("workflow_step_id", "__KEEP_workflow_step_id__"),
    ("eslint-program", "__KEEP_eslint_program__"),
    ("vite-program", "__KEEP_vite_program__"),
    ("roomprogram", "__KEEP_roomprogram__"),
    ("programming", "__KEEP_programming__"),
    ("architectural programming", "__KEEP_architectural_programming__"),
    ("program registers", "__KEEP_program_registers__"),
    ("coda-programming", "__KEEP_coda_programming__"),
    ("compose-blnbo-programming", "__KEEP_compose_blnbo_programming__"),
]

REPLACEMENTS: list[tuple[str, str]] = [
    # OS workflow leftovers (program → workflow, not plugin)
    ("register_os_builtin_program", "register_os_builtin_workflow"),
    ("register_os_program_definition", "register_os_workflow_definition"),
    ("list_os_programs", "list_os_workflows"),
    ("os_program_by_id", "os_workflow_by_id"),
    ("BUILTIN_PROGRAMS", "BUILTIN_WORKFLOWS"),
    ("EXTENSION_PROGRAMS", "EXTENSION_WORKFLOWS"),
    ("OsProgramArtifactMap", "OsPluginArtifactMap"),
    ("osProgramArtifact", "osPluginArtifact"),
    ("isSpaceProgramFilter", "isSpacePluginFilter"),
    ("OsProgramResourceMap", "OsPluginArtifactMap"),
    ("osBaselineProgram", "osBaselinePlugin"),
    # Registry / codegen identifiers (longest first)
    ("ProgramManifestJson", "PluginManifestJson"),
    ("ProgramManifest", "PluginManifest"),
    ("ProgramBundle", "PluginBundle"),
    ("ProgramInstanceId", "PluginInstanceId"),
    ("ProgramRegistryEntry", "PluginRegistryEntry"),
    ("ProgramHostMetadata", "PluginHostMetadata"),
    ("ProgramHostConfig", "PluginHostConfig"),
    ("ProgramHostError", "PluginHostError"),
    ("GenerateProgramRegistryOptions", "GeneratePluginRegistryOptions"),
    ("generateProgramRegistry", "generatePluginRegistry"),
    ("ensureProgramRegistry", "ensurePluginRegistry"),
    ("resolveProgramBuildTargets", "resolvePluginBuildTargets"),
    ("PROGRAM_BUILD_TARGETS", "PLUGIN_BUILD_TARGETS"),
    ("ProgramBuildTarget", "PluginBuildTarget"),
    ("programModuleUrl", "pluginModuleUrl"),
    ("findProgramCargoFiles", "findPluginCargoFiles"),
    ("parseProgramCargo", "parsePluginCargo"),
    ("parse_program_cargo_text", "parse_plugin_cargo_text"),
    ("find_program_cargo_files", "find_plugin_cargo_files"),
    ("generate_program_registry", "generate_plugin_registry"),
    ("emit_programs_json", "emit_plugins_json"),
    ("emit_programs_ts", "emit_plugins_ts"),
    ("emit_programs_rust_hosts", "emit_plugins_rust_hosts"),
    ("isProgramCrate", "isPluginCrate"),
    ("activeProgramManifest", "activePluginManifest"),
    ("active_program_manifest", "active_plugin_manifest"),
    ("install_program_bundle", "install_plugin_bundle"),
    ("WasmProgramRuntime", "WasmPluginRuntime"),
    ("ProgramRegistry", "PluginRegistry"),
    ("ProgramHost", "PluginHost"),
    ("ProgramApp", "PluginApp"),
    ("GuestProgram", "GuestPlugin"),
    ("ProgramWorld", "PluginWorld"),
    ("ProgramError", "PluginError"),
    ("program_exports!", "plugin_exports!"),
    ("ensure_program_initialized", "ensure_plugin_initialized"),
    ("program_runtime", "plugin_runtime"),
    ("__semio_program_bundle", "__semio_plugin_bundle"),
    ("semio-framework-program-host", "semio-framework-plugin-host"),
    ("semio_framework_program_host", "semio_framework_plugin_host"),
    ("semio-framework-program", "semio-framework-plugin"),
    ("semio_framework_program", "semio_framework_plugin"),
    ("@semio-tech/program-registry", "@semio-tech/plugin-registry"),
    ("program-registry", "plugin-registry"),
    ("program-world", "plugin-world"),
    ("program-error", "plugin-error"),
    ("program-manifest-json", "plugin-manifest-json"),
    ("semio:framework::program", "semio:framework::plugin"),
    ("interface program {", "interface plugin {"),
    ("export program;", "export plugin;"),
    ("trait Program:", "trait Plugin:"),
    ("pub trait Program", "pub trait Plugin"),
    ("impl Program for", "impl Plugin for"),
    ("CommandScope::Program", "CommandScope::Plugin"),
    ("Scope::Program", "Scope::Plugin"),
    ("program instance busy", "plugin instance busy"),
    ("program not initialized", "plugin not initialized"),
    ("resolve_program_host_config", "resolve_plugin_host_config"),
    ("ProgramRegistryCommand", "PluginRegistryCommand"),
    ("buildProgram", "buildPlugin"),
    ("filterPlaygroundProgram", "filterPlaygroundPlugin"),
    ("filterProgram", "filterPlugin"),
    ("program-modules", "plugin-modules"),
    ("programs.stories", "plugins.stories"),
    ("os-programs", "os-plugins"),
    ("programs.json", "plugins.json"),
    ("programs.ts", "plugins.ts"),
    ("dev🦀️os-programs", "dev🦀️os-plugins"),
    ("//#region 🔖️ProgramRegistryEntry", "//#region 🔖️PluginRegistryEntry"),
    ("//#endregion 🔖️ProgramRegistryEntry", "//#endregion 🔖️PluginRegistryEntry"),
    ("// #region 🔖️ProgramRegistryCommand", "// #region 🔖️PluginRegistryCommand"),
    ("// #endregion 🔖️ProgramRegistryCommand", "// #endregion 🔖️PluginRegistryCommand"),
    ("programId", "pluginId"),
    ("program_id", "plugin_id"),
    ("framework/program", "framework/plugin"),
    ("_program", "_plugin"),
    ("-program", "-plugin"),
    ("program/", "plugin/"),
    ("import { program }", "import { plugin }"),
    ("await program.", "await plugin."),
    ("program.clearInstanceGuard", "plugin.clearInstanceGuard"),
    ("semio program web worker", "semio plugin web worker"),
    ("semio program jco", "semio plugin jco"),
    ("Rust program OS", "Rust plugin OS"),
    ("Declarative app program SDK", "Declarative app plugin SDK"),
    ("app program SDK", "app plugin SDK"),
    ("WASM programs", "WASM plugins"),
    ("WASM program", "WASM plugin"),
    ("hot-swappable WASM programs", "hot-swappable WASM plugins"),
    ("sandboxed wasmtime component program host", "sandboxed wasmtime component plugin host"),
    ("per-program metadata", "per-plugin metadata"),
    ("one program from", "one plugin from"),
    ("prebuilt program WASM", "prebuilt plugin WASM"),
    ("program WASM", "plugin WASM"),
    ("program runtimes", "plugin runtimes"),
    ("program-boot", "plugin-boot"),
    ("program ABI", "plugin ABI"),
    ("program SDK", "plugin SDK"),
    ("program instance", "plugin instance"),
    ("program crate", "plugin crate"),
    ("program filter", "plugin filter"),
    ("program's", "plugin's"),
    ("the program ", "the plugin "),
    ("a program ", "a plugin "),
    ("os/program/", "os/plugin/"),
    ("os/program/app", "os/plugin/app"),
    ("scoped command (os/program", "scoped command (os/plugin"),
]


def protect(text: str) -> str:
    for original, token in PROTECTED:
        text = text.replace(original, token)
    return text


def unprotect(text: str) -> str:
    for original, token in PROTECTED:
        text = text.replace(token, original)
    return text


def should_skip_dir(path: Path) -> bool:
    return bool(set(path.parts) & SKIP_DIR_NAMES)


def iter_text_files() -> list[Path]:
    files: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dp = Path(dirpath)
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        if should_skip_dir(dp) or ".repo" in dp.parts:
            continue
        for name in filenames:
            p = dp / name
            if p.suffix in SKIP_FILE_SUFFIXES:
                continue
            if p.suffix in TEXT_SUFFIXES or name in {".gitignore", "Cargo.lock", "bun.lock"}:
                files.append(p)
    return files


def rename_files() -> list[str]:
    log: list[str] = []
    pairs = [
        (ROOT / "framework/program/registry/generated/programs.json", ROOT / "framework/plugin/registry/generated/plugins.json"),
        (ROOT / "framework/program/registry/generated/programs.ts", ROOT / "framework/plugin/registry/generated/plugins.ts"),
        (ROOT / ".storybook/stories/framework/os/programs.stories.tsx", ROOT / ".storybook/stories/framework/os/plugins.stories.tsx"),
        (ROOT / ".storybook/os-programs.spec.ts", ROOT / ".storybook/os-plugins.spec.ts"),
    ]
    for src, dst in pairs:
        if src.is_file():
            dst.parent.mkdir(parents=True, exist_ok=True)
            if dst.exists():
                dst.unlink()
            src.rename(dst)
            log.append(f"{src.relative_to(ROOT)} -> {dst.relative_to(ROOT)}")
    return log


def rename_directories() -> list[str]:
    log: list[str] = []
    program_dirs: list[Path] = []
    for dirpath, dirnames, _ in os.walk(ROOT):
        dp = Path(dirpath)
        if should_skip_dir(dp) or ".repo" in dp.parts:
            dirnames[:] = []
            continue
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        if "program" in dirnames:
            program_dirs.append(dp / "program")
    program_dirs.sort(key=lambda p: len(p.parts), reverse=True)
    for src in program_dirs:
        dst = src.parent / "plugin"
        if src.is_dir() and not dst.exists():
            src.rename(dst)
            log.append(f"{src.relative_to(ROOT)} -> plugin")
    return log


def apply_text_replacements() -> list[str]:
    touched: list[str] = []
    for path in iter_text_files():
        try:
            original = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        updated = protect(original)
        for old, new in REPLACEMENTS:
            updated = updated.replace(old, new)
        updated = unprotect(updated)
        if updated != original:
            path.write_text(updated, encoding="utf-8")
            touched.append(str(path.relative_to(ROOT)))
    return touched


def main() -> None:
    file_log = rename_files()
    dir_log = rename_directories()
    touched = apply_text_replacements()
    report = {
        "directories": dir_log,
        "files_renamed": file_log,
        "files_touched": len(touched),
        "files": touched[:200],
    }
    (TICKET / "rename-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps({k: v for k, v in report.items() if k != "files"}, indent=2))
    print(f"files_touched sample: {len(touched)} total")


if __name__ == "__main__":
    main()
