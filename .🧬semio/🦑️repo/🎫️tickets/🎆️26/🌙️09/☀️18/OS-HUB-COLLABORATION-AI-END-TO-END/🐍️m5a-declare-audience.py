#!/usr/bin/env python3
"""🖱️ M5a audience codemod — inserts one `.action_audience("<id>", …)` chain line immediately after
the builder step that DECLARES `<id>`, in the plugin file that declares it.

Guards (a name-keyed edit without a region guard hit production code on this repo before):
  * the declaring line must be a chain step (first non-space character is `.`);
  * its parentheses must balance on that one line, so the insertion lands between steps;
  * exactly one line in the whole plugin tree may declare the id;
  * nothing is written unless every id passes, and a per-file diffstat is printed.

Usage: 🐍️m5a-declare-audience.py <plugin-dir> input:<id>,<id> agent:<id> destructive:<id>
"""
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[7]
DECLARATION = re.compile(r'^(\s*)\.(action_with|mutation|view_action|shell_action|action)\(')


def declaring_lines(plugin_dir: Path, verb_id: str):
    found = []
    for path in plugin_dir.rglob("*.rs"):
        text = path.read_text(encoding="utf8")
        if f'"{verb_id}"' not in text:
            continue
        for number, line in enumerate(text.splitlines()):
            if f'"{verb_id}"' not in line:
                continue
            if not DECLARATION.match(line):
                continue
            found.append((path, number, line))
    return found


def main() -> int:
    plugin_dir = REPO / sys.argv[1]
    plan = {}
    for argument in sys.argv[2:]:
        audience, _, raw = argument.partition(":")
        for verb_id in filter(None, raw.split(",")):
            plan[verb_id] = {"agent": "Agent", "destructive": "Destructive"}.get(audience, "Input")

    edits = []
    for verb_id, audience in plan.items():
        matches = declaring_lines(plugin_dir, verb_id)
        if len(matches) != 1:
            print(f"REFUSED {verb_id}: {len(matches)} declaring line(s)")
            for path, number, line in matches:
                print(f"   {path.relative_to(REPO)}:{number + 1}  {line.strip()[:160]}")
            return 1
        path, number, line = matches[0]
        if line.count("(") != line.count(")"):
            print(f"REFUSED {verb_id}: unbalanced declaring line {path.relative_to(REPO)}:{number + 1}")
            return 1
        indent = DECLARATION.match(line).group(1)
        awaited = ".await" if line.rstrip().endswith(".await") else ""
        call = f'.action_destructive("{verb_id}")' if audience == "Destructive" else f'.action_audience("{verb_id}", semio_framework_plugin::CapabilityAudience::{audience})'
        edits.append((path, number, f"{indent}{call}{awaited}"))

    by_file = {}
    for path, number, text in edits:
        by_file.setdefault(path, []).append((number, text))
    for path, rows in by_file.items():
        lines = path.read_text(encoding="utf8").splitlines(keepends=True)
        for number, text in sorted(rows, reverse=True):
            lines.insert(number + 1, text + "\n")
        path.write_text("".join(lines), encoding="utf8")
        print(f"+{len(rows)} {path.relative_to(REPO)}")
    print(subprocess.run(["git", "diff", "--stat", "--", *[str(path) for path in by_file]], cwd=REPO, capture_output=True, text=True).stdout.strip())
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
