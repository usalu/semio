#!/usr/bin/env python3
"""Resolve stash-pop conflicts across the semio workspace."""
from __future__ import annotations

import pathlib
import re
import sys

ROOT = pathlib.Path(r"C:\git\semio")
MARKER_UP = "<<<<<<< Updated upstream"
MARKER_MID = "======="
MARKER_DOWN = ">>>>>>> Stashed changes"


def split_conflicts(text: str) -> list[tuple[str, str, str]]:
    conflicts: list[tuple[str, str, str]] = []
    while MARKER_UP in text:
        start = text.index(MARKER_UP)
        mid = text.index(MARKER_MID, start)
        end = text.index(MARKER_DOWN, mid)
        upstream = text[start + len(MARKER_UP) + 1 : mid]
        stashed = text[mid + len(MARKER_MID) + 1 : end]
        conflicts.append((upstream, stashed, text[start : end + len(MARKER_DOWN) + 1]))
    return conflicts


def resolve_union(upstream: str, stashed: str) -> str:
    up = upstream.strip("\n")
    st = stashed.strip("\n")
    if not up:
        return stashed
    if not st:
        return upstream
    if up == st:
        return upstream
    return upstream.rstrip("\n") + "\n" + stashed.lstrip("\n")


def resolve_plugin_script(upstream: str, stashed: str) -> str:
    merged = upstream
    if '"preview-generated":GraphPreviewScript' in stashed and '"preview-generated":GraphPreviewScript' not in merged:
        merged = merged.replace(
            '"graph-generate":GraphGenerateScript,"graph-wire-check"',
            '"graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript,"graph-wire-check"',
        )
        merged = merged.replace(
            '"graph-generate":GraphGenerateScript, verify',
            '"graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript, verify',
        )
    return merged


def resolve_file(path: pathlib.Path, text: str) -> str:
    rel = path.relative_to(ROOT).as_posix()
    conflicts = split_conflicts(text)
    if not conflicts:
        return text

    for upstream, stashed, block in conflicts:
        if rel.endswith("📜️script.ts") and "runArtifactRustPackageMain" in block:
            replacement = resolve_plugin_script(upstream, stashed)
        elif rel == ".vscode/launch.json" or rel == ".vscode/🧩️launch.seed.jsonc":
            replacement = resolve_union(upstream, stashed)
        elif rel == "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts":
            if "artifact-io-ownership" in upstream and "cargo-library-search-path" in stashed:
                replacement = upstream + "\n    " + stashed.strip()
            elif "owner-cmd-policy" in upstream and "owner-command-policy" in stashed:
                replacement = (
                    '      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");\n'
                    '      resolveTestLevel([], "quick");\n'
                    '      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts");\n'
                )
            else:
                replacement = resolve_union(upstream, stashed)
        elif rel == "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json":
            if "test-rust-runtime-path-direction" in upstream and "test-cargo-library-search-path" in stashed:
                upstream_block = upstream + stashed.split('"test-cargo-library-search-path"')[0]
                stashed_tail = stashed[stashed.index('"test-cargo-library-search-path"') :]
                stashed_entry = (
                    '    "test-cargo-library-search-path": {\n'
                    '      "executor": "nx:run-commands",\n'
                    '      "cache": false,\n'
                    '      "dependsOn": [],\n'
                    '      "outputs": [],\n'
                    '      "options": {\n'
                    '        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript",\n'
                    '        "command": "bun ./📜️script.ts test cargo-library-search-path"\n'
                    "      }\n"
                    "    },\n"
                )
                replacement = upstream.rstrip() + "\n    },\n" + stashed_entry
            else:
                replacement = resolve_union(upstream, stashed)
        elif rel == "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🟦️.ts":
            replacement = (
                'import { writePrintGalleryEvidence } from "../../🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/🟦️.ts";\n'
                'import { verifyPrintKindPaintFixtures, verifyPrintGalleryCarrier, verifyPrintApiTitleRendering, verifyPrintMacroStagingNative, verifyPrintPipelineLong, verifyPrintPipelineQuick, verifyPrintVisualizationBuild } from "./🧪️tests/🖨️pipeline/🟦️.ts";\n'
            )
        elif rel == "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts":
            replacement = (
                'import { declaredProjectTargets, generateLaunchJson, LAUNCH_OUTPUT_REL_PATH, reconcileRepositoryLaunchSeed } from "../🚀️launch/🟦️.ts";\n'
                'import { MODULE_BRIDGE_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, parseModuleDirectories, type ModuleDirectory } from "../📦️deployment/🟦️.ts";\n'
            )
        elif rel == "🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust/Cargo.toml":
            replacement = stashed
        elif rel == "🧰️framework/🔨️modules/🎒️pack/🌱️value/🫳️preflight/🦀️.rs":
            replacement = upstream
        elif rel == "🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🫳️preflight/🦀️.rs":
            replacement = upstream
        elif rel == "🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🦀️.rs":
            replacement = stashed
        elif rel == "🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🦀️.rs":
            if "semio_framework_value_derive" in upstream and "semio_framework_value::{" in stashed:
                replacement = stashed
            elif "domain diagnostics" in upstream and "empty authored chart" in stashed:
                replacement = stashed
            else:
                replacement = stashed if ", " in stashed and ", " not in upstream else upstream
        elif rel.startswith("🌎️hub/"):
            replacement = upstream
        elif rel == "Cargo.lock":
            replacement = upstream
        elif rel.endswith(".rs") or rel.endswith(".ts"):
            replacement = stashed if ", " in stashed else upstream
        elif rel.endswith(".json") or rel.endswith(".jsonc"):
            replacement = resolve_union(upstream, stashed)
        else:
            replacement = resolve_union(upstream, stashed)
        text = text.replace(block, replacement, 1)
    return text


def main() -> int:
    unresolved: list[str] = []
    resolved_count = 0
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if ".git" in path.parts:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if MARKER_UP not in text:
            continue
        new_text = resolve_file(path, text)
        if MARKER_UP in new_text:
            unresolved.append(str(path.relative_to(ROOT)))
            continue
        path.write_text(new_text, encoding="utf-8", newline="")
        resolved_count += 1
        print(f"resolved {path.relative_to(ROOT)}")
    print(f"resolved {resolved_count} files")
    if unresolved:
        print("still conflicted:")
        for item in unresolved:
            print(f"  {item}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
