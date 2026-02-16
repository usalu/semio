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
    "coda/py/coda.py",
    "conftest.py",
    "semio/assets/grasshopper/build.py",
    "semio/assets/icons.ts",
    "semio/assets/index.ts",
    "semio/assets/logo/logo.ts",
    "semio/desktop/forge.config.ts",
    "semio/desktop/forge.env.d.ts",
    "semio/desktop/main.ts",
    "semio/desktop/postcss.config.ts",
    "semio/desktop/preload.ts",
    "semio/desktop/renderer.tsx",
    "semio/desktop/tailwind.config.ts",
    "semio/desktop/vite.main.config.ts",
    "semio/desktop/vite.preload.config.ts",
    "semio/desktop/vite.renderer.config.ts",
    "semio/docs/index.tsx",
    "semio/docs/postcss.config.ts",
    "semio/docs/tailwind.config.ts",
    "semio/engine/build.ts",
    "semio/engine/engine.py",
    "semio/engine/engine.test.py",
    "semio/engine/generate-schemas.ts",
    "semio/engine/post-build.ts",
    "semio/engine/sqliteschema.ts",
    "semio/engine/test.ts",
    "semio/gh/Semio.Grasshopper/build.ts",
    "semio/gh/Semio.Grasshopper/build-value-lists.ts",
    "semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs",
    "semio/gh/Semio.Grasshopper.Tests/Tests.cs",
    "semio/gh/Semio.Grasshopper.Tests/Usings.cs",
    "semio/gh/Semio.Grasshopper/yak/build.ts",
    "semio/gh/Semio.Grasshopper/yak/login.ts",
    "semio/gh/Semio.Grasshopper/yak/publish.ts",
    "semio/gh/Semio.Grasshopper/yak/test-push.ts",
    "semio/gh/Semio.Grasshopper/yak/test-search.ts",
    "semio/gh/Semio.Grasshopper/yak/unyank.ts",
    "semio/gh/Semio.Grasshopper/yak/yank.ts",
    "semio/go/kit_sqlite.go",
    "semio/go/semio_benchmark.go",
    "semio/go/semio.go",
    "semio/go/semio_test.go",
    "semio/js/dev.ts",
    "semio/js/eslint.config.ts",
    "semio/js/global.d.ts",
    "semio/js/i18n.ts",
    "semio/js/index.ts",
    "semio/jsonschema/build.ts",
    "semio/js/playwright.config.ts",
    "semio/js/postcss.config.ts",
    "semio/js/semio.benchmark.ts",
    "semio/js/semio.test.ts",
    "semio/js/semio.ts",
    "semio/js/site.tsx",
    "semio/js/sketchpad/apps/index.ts",
    "semio/js/sketchpad/Design.tsx",
    "semio/js/sketchpad/Docs.tsx",
    "semio/js/sketchpad/elements.tsx",
    "semio/js/sketchpad/Feedback.tsx",
    "semio/js/sketchpad/Home.tsx",
    "semio/js/sketchpad/kitSelectionHelper.ts",
    "semio/js/sketchpad/Kit.tsx",
    "semio/js/sketchpad/portColor.ts",
    "semio/js/sketchpad/Quality.tsx",
    "semio/js/sketchpad/shared.ts",
    "semio/js/sketchpad/Sketchpad.tsx",
    "semio/js/sketchpad.test.ts",
    "semio/js/sketchpad/Tutorials.tsx",
    "semio/js/sketchpad/Type.tsx",
    "semio/js/tailwind.config.ts",
    "semio/js/vite.config.ts",
    "semio/js/vite-env.d.ts",
    "semio/net/Semio.Benchmark/Program.cs",
    "semio/net/Semio/build.ts",
    "semio/net/Semio/Semio.cs",
    "semio/net/Semio.Tests/Tests.cs",
    "semio/net/Semio.Tests/Usings.cs",
    "semio/play/index.tsx",
    "semio/play/postcss.config.ts",
    "semio/play/tailwind.config.ts",
    "semio/play/vite.config.ts",
    "semio/py/semio.benchmark.py",
    "semio/py/semio.py",
    "semio/py/semio.test.py",
    "semio-repo/cli/main.go",
    "semio-repo/cli/main_test.go",
    "semio-repo/server/main.go",
    "semio-repo/vscode/codegen.ts",
    "semio-repo/vscode/eslint.config.ts",
    "semio-repo/vscode/extension.test.ts",
    "semio-repo/vscode/extension.ts",
    "semio-repo/vscode/queries.ts",
    "semio-repo/vscode/vite.config.ts",
    "semio-repo/vscode/vite.test.config.ts",
    "semio/sketchpad/index.tsx",
    "semio/sketchpad/postcss.config.ts",
    "semio/sketchpad/tailwind.config.ts",
    "semio/sketchpad/vite.config.ts",
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
        content_after_prefix = stripped[len(cp) :].strip() if stripped.startswith(cp) else ""
        if not content_after_prefix:
            continue
        has_ext = any(
            ext in content_after_prefix
            for ext in [".ts", ".tsx", ".go", ".cs", ".py", ".sh", ".sql", ".graphql", ".d.ts"]
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
    return f"semiorepo://file/{filepath}"


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
        if any("[" in line and "semiorepo://" in line for line in header_lines):
            print(f"  SKIP (already [ID](URI) format): {filepath}")
            return False
        print(f"  SKIP (no bare ID found): {filepath}")
        return False
    uri = make_uri(filepath)
    new_id_line = f"{cp} [{bare_id}]({uri})"
    has_license = any("GNU Affero General Public License" in line or "AGPL" in line or "gnu.org/licenses" in line for line in header_lines)
    contributor_pattern = re.compile(r"\d{4}.*<[\w.@-]+>")
    contributor_lines = []
    contributor_last_idx = -1
    for i, line in enumerate(header_lines):
        if contributor_pattern.search(line):
            contributor_lines.append(line)
            contributor_last_idx = i
    summary_lines = []
    specs_lines = []
    for i, line in enumerate(header_lines):
        stripped = line.strip()
        if stripped == "" or stripped == cp:
            continue
        if stripped.startswith(cp + " #region") or stripped.startswith(cp + " #endregion"):
            continue
        if stripped == header_start_marker.strip() or stripped == header_end_marker.strip():
            continue
        if "#region" in stripped or "#endregion" in stripped or "region " in stripped.lower() and "endregion" not in stripped.lower():
            continue
        content_text = stripped[len(cp):].strip() if stripped.startswith(cp) else stripped
        if not content_text:
            continue
        if "[" in content_text and "semiorepo://" in content_text:
            continue
        if contributor_pattern.search(line):
            continue
        if any(m in content_text for m in ["GNU Affero", "AGPL", "gnu.org/licenses", "free software", "redistribute", "General Public License", "WARRANTY", "PURPOSE", "Foundation", "received"]):
            continue
        has_ext = any(ext in content_text for ext in [".ts", ".tsx", ".go", ".cs", ".py", ".sh", ".sql"])
        has_emoji = any(ch in content_text for ch in "\U0001f4bb\U0001f9ea\U0001f4dc\U0001f4c3\u2699\U0001f4be\u2696")
        if has_ext and has_emoji and "[" not in content_text:
            continue
        if "MUST" in content_text or "SHALL" in content_text or "SHOULD" in content_text:
            specs_lines.append(line)
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
    if specs_lines:
        for sl in specs_lines:
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
    root_dir = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", ".."))
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
