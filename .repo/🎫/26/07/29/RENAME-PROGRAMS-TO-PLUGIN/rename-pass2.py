#!/usr/bin/env python3
"""Pass 2: scrub remaining legacy program naming in active codebase."""

from __future__ import annotations

import json
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent

SKIP_DIR_NAMES = {".git", ".repo", "node_modules", "target", "dist", "build", ".next", "coverage", "__pycache__", ".turbo", "out", ".venv", ".claude", ".vscode-test", "Visual Studio Code.app"}
TEXT_SUFFIXES = {".rs", ".ts", ".tsx", ".json", ".toml", ".wit", ".md", ".mdx"}

PROTECTED = [
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
    ("architect_program", "__KEEP_architect_program__"),
    ("empty_program()", "__KEEP_empty_program__"),
    ("sample_program()", "__KEEP_sample_program__"),
    ("PROGRAM_TARGETS", "__KEEP_PROGRAM_TARGETS__"),
]

REPLACEMENTS = [
    ("program registry", "plugin registry"),
    ("semio program registry", "semio plugin registry"),
    ("program_manifest", "plugin_manifest"),
    ("program_app_labels", "plugin_app_labels"),
    ("program runtime", "plugin runtime"),
    ("program dispatch", "plugin dispatch"),
    ("program bundle", "plugin bundle"),
    ("program list", "plugin list"),
    ("program id", "plugin id"),
    ("per program", "per plugin"),
    ("whole program", "whole plugin"),
    ("one program", "one plugin"),
    ("the program ", "the plugin "),
    ("a program ", "a plugin "),
    ("Flow program", "Flow plugin"),
    ("Process program", "Process plugin"),
    ("Shooting program", "Shooting program".replace("program", "plugin")),
    ("Procedural program", "Procedural plugin"),
    ("Procedural 2D program", "Procedural 2D plugin"),
    ("Procedural 3D program", "Procedural 3D plugin"),
    ("GIS program", "GIS plugin"),
    ("GIS 2D program", "GIS 2D plugin"),
    ("GIS 3D program", "GIS 3D plugin"),
    ("Draw program", "Draw plugin"),
    ("DAG program", "DAG plugin"),
    ("Sourcing program", "Sourcing plugin"),
    ("Sequence program", "Sequence plugin"),
    ("program trait", "plugin trait"),
    ("program section", "plugin section"),
    ("program bridge", "plugin bridge"),
    ("program bundles", "plugin bundles"),
    ("program contract", "plugin contract"),
    ("program registration", "plugin registration"),
    ("program round-trip", "plugin round-trip"),
    ("program exports", "plugin exports"),
    ("program export", "plugin export"),
    ("program ABI", "plugin ABI"),
    ("program SDK", "plugin SDK"),
    ("program apps", "plugin apps"),
    ("program app", "plugin app"),
    ("program world", "plugin world"),
    ("program host", "plugin host"),
    ("program crates", "plugin crates"),
    ("program crate", "plugin crate"),
    ("program WASM", "plugin WASM"),
    ("program modules", "plugin modules"),
    ("program module", "plugin module"),
    ("program filter", "plugin filter"),
    ("program-driven", "plugin-driven"),
    ("spawn_program", "spawn_plugin"),
    ("parent row per program", "parent row per plugin"),
    ("bare program id", "bare plugin id"),
    ("wasm registry program id", "wasm registry plugin id"),
    ("`gis` program bundle", "`gis` plugin bundle"),
    ("`norm` program bundle", "`norm` plugin bundle"),
    ("contributed program", "contributed plugin"),
    ("any program has", "any plugin has"),
    ("sandboxed program", "sandboxed plugin"),
    ("puzzle's program", "puzzle's plugin"),
    ("semio program ", "semio plugin "),
    ("program registry codegen", "plugin registry codegen"),
    ("single-source program registry", "single-source plugin registry"),
    ("@semio-tech/norm-program", "@semio-tech/norm-plugin"),
    ("@semio-tech/animate-program-rs", "@semio-tech/animate-plugin-rs"),
    ("@semio-tech/fem-program-rs", "@semio-tech/fem-plugin-rs"),
    ("architect/program", "architect/plugin"),
    ("dev🦀os-programs", "dev🦀os-plugins"),
    ("semio program registry generate", "semio plugin registry generate"),
    ("semio program registry check", "semio plugin registry check"),
    ("semio program registry", "semio plugin registry"),
    ("program registry generate", "plugin registry generate"),
    ("program registry check", "plugin registry check"),
]


def protect(text: str) -> str:
    for original, token in PROTECTED:
        text = text.replace(original, token)
    return text


def unprotect(text: str) -> str:
    for original, token in PROTECTED:
        text = text.replace(token, original)
    return text


def iter_files() -> list[Path]:
    out: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dp = Path(dirpath)
        if set(dp.parts) & SKIP_DIR_NAMES or ".repo" in dp.parts:
            dirnames[:] = []
            continue
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        for name in filenames:
            p = dp / name
            if p.suffix in TEXT_SUFFIXES:
                out.append(p)
    return out


def main() -> None:
    touched = []
    for path in iter_files():
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
    report = {"pass": 2, "files_touched": len(touched)}
    (TICKET / "rename-pass2-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
