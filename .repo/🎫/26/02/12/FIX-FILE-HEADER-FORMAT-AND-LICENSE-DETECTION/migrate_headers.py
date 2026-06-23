#!/usr/bin/env python3
"""Migrate file headers to [ID](URI) format with AGPL license text."""

import os
import re
import sys

AGPL_TEXT_LINES = [
    "This program is free software: you can redistribute it and/or modify",
    "it under the terms of the GNU Affero General Public License as",
    "published by the Free Software Foundation, either version 3 of the",
    "License, or (at your option) any later version.",
    "This program is distributed in the hope that it will be useful,",
    "but WITHOUT ANY WARRANTY; without even the implied warranty of",
    "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
    "GNU Affero General Public License for more details.",
    "You should have received a copy of the GNU Affero General Public License",
    "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
]

AFFECTED_FILES = [
    "coda/engine/coda.py",
    "conftest.py",
    "compose/assets/grasshopper/build.py",
    "compose/assets/icons.ts",
    "compose/assets/index.ts",
    "compose/assets/logo/logo.ts",
    "compose/desktop/forge.config.ts",
    "compose/desktop/forge.env.d.ts",
    "compose/desktop/main.ts",
    "compose/desktop/postcss.config.ts",
    "compose/desktop/preload.ts",
    "compose/desktop/renderer.tsx",
    "compose/desktop/tailwind.config.ts",
    "compose/desktop/vite.main.config.ts",
    "compose/desktop/vite.preload.config.ts",
    "compose/desktop/vite.renderer.config.ts",
    "compose/docs/index.tsx",
    "compose/docs/postcss.config.ts",
    "compose/docs/tailwind.config.ts",
    "compose/engine/build.ts",
    "compose/engine/engine.py",
    "compose/engine/engine.test.py",
    "compose/engine/generate-schemas.ts",
    "compose/engine/post-build.ts",
    "compose/engine/sqliteschema.ts",
    "compose/engine/test.ts",
    "compose/gh/Compose.Grasshopper/build.ts",
    "compose/gh/Compose.Grasshopper/build-value-lists.ts",
    "compose/gh/Compose.Grasshopper/Compose.Grasshopper.cs",
    "compose/gh/Compose.Grasshopper.Tests/Tests.cs",
    "compose/gh/Compose.Grasshopper.Tests/Usings.cs",
    "compose/gh/Compose.Grasshopper/yak/build.ts",
    "compose/gh/Compose.Grasshopper/yak/login.ts",
    "compose/gh/Compose.Grasshopper/yak/publish.ts",
    "compose/gh/Compose.Grasshopper/yak/test-push.ts",
    "compose/gh/Compose.Grasshopper/yak/test-search.ts",
    "compose/gh/Compose.Grasshopper/yak/unyank.ts",
    "compose/gh/Compose.Grasshopper/yak/yank.ts",
    "compose/go/kit_sqlite.go",
    "compose/go/compose_benchmark.go",
    "compose/go/compose.go",
    "compose/go/compose_test.go",
    "compose/js/dev.ts",
    "compose/js/eslint.config.ts",
    "compose/js/global.d.ts",
    "compose/js/i18n.ts",
    "compose/js/index.ts",
    "compose/jsonschema/build.ts",
    "compose/js/playwright.config.ts",
    "compose/js/postcss.config.ts",
    "compose/js/compose.benchmark.ts",
    "compose/js/compose.test.ts",
    "compose/js/compose.ts",
    "compose/js/site.tsx",
    "compose/js/sketchpad/apps/index.ts",
    "compose/js/sketchpad/Design.tsx",
    "compose/js/sketchpad/Docs.tsx",
    "compose/js/sketchpad/elements.tsx",
    "compose/js/sketchpad/Feedback.tsx",
    "compose/js/sketchpad/Home.tsx",
    "compose/js/sketchpad/kitSelectionHelper.ts",
    "compose/js/sketchpad/Kit.tsx",
    "compose/js/sketchpad/portColor.ts",
    "compose/js/sketchpad/Quality.tsx",
    "compose/js/sketchpad/shared.ts",
    "compose/js/sketchpad/Sketchpad.tsx",
    "compose/js/sketchpad.test.ts",
    "compose/js/sketchpad/Tutorials.tsx",
    "compose/js/sketchpad/Type.tsx",
    "compose/js/tailwind.config.ts",
    "compose/js/vite.config.ts",
    "compose/js/vite-env.d.ts",
    "compose/net/Compose.Benchmark/Program.cs",
    "compose/net/Compose/build.ts",
    "compose/net/Compose/Compose.cs",
    "compose/net/Compose.Tests/Tests.cs",
    "compose/net/Compose.Tests/Usings.cs",
    "compose/play/index.tsx",
    "compose/play/postcss.config.ts",
    "compose/play/tailwind.config.ts",
    "compose/play/vite.config.ts",
    "compose/py/compose.benchmark.py",
    "compose/py/compose.py",
    "compose/py/compose.test.py",
    "repo/cli/main.go",
    "repo/cli/main_test.go",
    "repo/server/main.go",
    "repo/vscode/codegen.ts",
    "repo/vscode/eslint.config.ts",
    "repo/vscode/extension.test.ts",
    "repo/vscode/extension.ts",
    "repo/vscode/queries.ts",
    "repo/vscode/vite.config.ts",
    "repo/vscode/vite.test.config.ts",
    "compose/sketchpad/index.tsx",
    "compose/sketchpad/postcss.config.ts",
    "compose/sketchpad/tailwind.config.ts",
    "compose/sketchpad/vite.config.ts",
    "vitest.config.ts",
]


def get_comment_prefix(filepath):
    ext = os.path.splitext(filepath)[1]
    if ext in (".py",):
        return "#"
    if ext in (".cs",):
        return "//"
    if ext in (".sql",):
        return "--"
    if ext in (".graphql",):
        return "#"
    if ext in (".sh",):
        return "#"
    return "//"


def get_header_region_start(filepath):
    ext = os.path.splitext(filepath)[1]
    if ext in (".py",):
        return "# region Header"
    if ext in (".cs",):
        return "#region \U0001f516Header"
    return "// #region \U0001f516Header"


def get_header_region_end(filepath):
    ext = os.path.splitext(filepath)[1]
    if ext in (".py",):
        return "# endregion Header"
    if ext in (".cs",):
        return "#endregion \U0001f516Header"
    return "// #endregion \U0001f516Header"


def find_bare_id(lines, cp):
    emoji_pattern = re.compile(
        r"^"
        + re.escape(cp)
        + r"\s+([\U0001f4bb\U0001f9ea\U0001f4dc\U0001f4c3\u2699\ufe0f\U0001f4be\u2696\U0001f4dc\U0001f4d1\U0001f4dd\U0001f4dc])"
    )
    for i, line in enumerate(lines):
        stripped = line.strip()
        if not stripped:
            continue
        content_after_prefix = (
            stripped[len(cp) :].strip() if stripped.startswith(cp) else ""
        )
        if not content_after_prefix:
            continue
        has_ext = any(
            ext in content_after_prefix
            for ext in [
                ".ts",
                ".tsx",
                ".go",
                ".cs",
                ".py",
                ".sh",
                ".sql",
                ".graphql",
                ".d.ts",
            ]
        )
        has_emoji = any(
            ch in content_after_prefix
            for ch in "\U0001f4bb\U0001f9ea\U0001f4dc\U0001f4c3\u2699\U0001f4be\u2696"
        )
        if has_ext or has_emoji:
            if "[" not in content_after_prefix:
                return i, content_after_prefix
    return -1, None


def make_uri(filepath):
    return f"composerepo://file/{filepath}"


def make_license_block(cp):
    lines = []
    for line in AGPL_TEXT_LINES:
        if line == "":
            lines.append(cp)
        else:
            lines.append(f"{cp} {line}")
    return lines


def migrate_file(root_dir, filepath):
    abs_path = os.path.join(root_dir, filepath)
    if not os.path.exists(abs_path):
        print(f"  SKIP (not found): {filepath}")
        return False
    with open(abs_path, "r", encoding="utf-8") as f:
        content = f.read()
    lines = content.split("\n")
    cp = get_comment_prefix(filepath)
    header_start_marker = get_header_region_start(filepath)
    header_end_marker = get_header_region_end(filepath)
    header_start_idx = -1
    header_end_idx = -1
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == header_start_marker or stripped == header_start_marker.strip():
            header_start_idx = i
        if stripped == header_end_marker or stripped == header_end_marker.strip():
            header_end_idx = i
            break
    if header_start_idx == -1 or header_end_idx == -1:
        print(f"  SKIP (no header region): {filepath}")
        return False
    header_lines = lines[header_start_idx : header_end_idx + 1]
    bare_id_idx, bare_id = find_bare_id(header_lines, cp)
    if bare_id_idx == -1:
        if any("[" in line and "composerepo://" in line for line in header_lines):
            print(f"  SKIP (already [ID](URI) format): {filepath}")
            return False
        print(f"  SKIP (no bare ID found): {filepath}")
        return False
    uri = make_uri(filepath)
    new_id_line = f"{cp} [{bare_id}]({uri})"
    has_license = any(
        "GNU Affero General Public License" in line
        or "AGPL" in line
        or "gnu.org/licenses" in line
        for line in header_lines
    )
    contributor_pattern = re.compile(r"\d{4}.*<[\w.@-]+>")
    contributor_lines = []
    contributor_last_idx = -1
    for i, line in enumerate(header_lines):
        if contributor_pattern.search(line):
            contributor_lines.append(line)
            contributor_last_idx = i
    summary_lines = []
    requirements_lines = []
    for i, line in enumerate(header_lines):
        stripped = line.strip()
        if stripped == "" or stripped == cp:
            continue
        if stripped.startswith(cp + " #region") or stripped.startswith(
            cp + " #endregion"
        ):
            continue
        if (
            stripped == header_start_marker.strip()
            or stripped == header_end_marker.strip()
        ):
            continue
        if (
            "#region" in stripped
            or "#endregion" in stripped
            or "region " in stripped.lower()
            and "endregion" not in stripped.lower()
        ):
            continue
        content_text = (
            stripped[len(cp) :].strip() if stripped.startswith(cp) else stripped
        )
        if not content_text:
            continue
        if "[" in content_text and "composerepo://" in content_text:
            continue
        if contributor_pattern.search(line):
            continue
        if any(
            m in content_text
            for m in [
                "GNU Affero",
                "AGPL",
                "gnu.org/licenses",
                "free software",
                "redistribute",
                "General Public License",
                "WARRANTY",
                "PURPOSE",
                "Foundation",
                "received",
            ]
        ):
            continue
        has_ext = any(
            ext in content_text
            for ext in [".ts", ".tsx", ".go", ".cs", ".py", ".sh", ".sql"]
        )
        has_emoji = any(
            ch in content_text
            for ch in "\U0001f4bb\U0001f9ea\U0001f4dc\U0001f4c3\u2699\U0001f4be\u2696"
        )
        if has_ext and has_emoji and "[" not in content_text:
            continue
        if (
            "MUST" in content_text
            or "SHALL" in content_text
            or "SHOULD" in content_text
        ):
            requirements_lines.append(line)
        else:
            summary_lines.append(line)
    new_header = []
    new_header.append(lines[header_start_idx])
    new_header.append("")
    new_header.append(new_id_line)
    new_header.append("")
    for cl in contributor_lines:
        new_header.append(cl)
    new_header.append("")
    for ll in make_license_block(cp):
        new_header.append(ll)
    new_header.append("")
    if summary_lines:
        for sl in summary_lines:
            new_header.append(sl)
        new_header.append("")
    if requirements_lines:
        for sl in requirements_lines:
            new_header.append(sl)
        new_header.append("")
    new_header.append(lines[header_end_idx])
    new_lines = lines[:header_start_idx] + new_header + lines[header_end_idx + 1 :]
    new_content = "\n".join(new_lines)
    with open(abs_path, "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"  FIXED: {filepath}")
    return True


def main():
    root_dir = os.path.abspath(
        os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            "..",
            "..",
            "..",
            "..",
            "..",
            "..",
        )
    )
    if not os.path.exists(os.path.join(root_dir, "AGENTS.md")):
        root_dir = "/workspaces/semio"
    print(f"Root dir: {root_dir}")
    fixed = 0
    skipped = 0
    for filepath in AFFECTED_FILES:
        result = migrate_file(root_dir, filepath)
        if result:
            fixed += 1
        else:
            skipped += 1
    print(f"\nDone. Fixed: {fixed}, Skipped: {skipped}")


if __name__ == "__main__":
    main()
