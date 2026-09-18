#!/usr/bin/env python3
"""Parse tsc --noEmit --pretty false captures into a census.

Read-only auditor script. Does not modify source.
"""
import re
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN = TICKET / "🗑️generated"

CAPTURES = {
    "root": GEN / "ts-census-root.txt",
    "hub": GEN / "ts-census-hub.txt",
    "renderer": GEN / "ts-census-renderer.txt",
}

LINE_RE = re.compile(r"^(?P<file>.+?)\((?P<line>\d+),(?P<col>\d+)\): error (?P<code>TS\d+): (?P<msg>.*)$")

AREA_RULES = [
    ("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer", "💻️os/…/📺️renderer"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev", "💻️os/…/🧑‍💻dev"),
    ("🧰️framework/🛍️products/💻️os", "💻️os/other"),
    ("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🦑️repo/🔨️modules/📚️library"),
    ("🧰️framework/🛍️products/🦑️repo", "🦑️repo/other"),
    ("🧰️framework/🛍️products", "🧰️framework/🛍️products/other"),
    ("🧰️framework", "🧰️framework/other"),
    ("🌎️hub", "🌎️hub"),
    ("✏️s", "✏️s"),
    ("storybook-static", "storybook-static (generated, out of scope)"),
    ("temp/", "temp/"),
    ("♻️mit-bestand", "♻️mit-bestand (legacy/archive)"),
]


def classify_area(path: str) -> str:
    for prefix, label in AREA_RULES:
        if path.startswith(prefix) or ("/" + prefix) in path:
            return label
    return "other"


def parse(path: Path):
    if not path.exists():
        return []
    rows = []
    for raw in path.read_text(errors="replace").splitlines():
        m = LINE_RE.match(raw)
        if m:
            rows.append(m.groupdict())
    return rows


def git_last(file_path: str):
    try:
        out = subprocess.run(
            ["git", "log", "-1", "--format=%an|%ad", "--date=iso", "--", file_path],
            cwd=ROOT, capture_output=True, text=True, timeout=10,
        )
        if out.returncode == 0 and out.stdout.strip():
            an, _, ad = out.stdout.strip().partition("|")
            return an, ad
    except Exception:
        pass
    return "", ""


def mtime_recent_minutes(file_path: str):
    p = ROOT / file_path
    try:
        r = subprocess.run(["stat", "-f", "%m", str(p)], capture_output=True, text=True, timeout=5)
        if r.returncode == 0:
            import time
            mtime = int(r.stdout.strip())
            return (time.time() - mtime) / 60.0
    except Exception:
        pass
    return None


def main():
    all_rows = []
    per_capture = {}
    for name, path in CAPTURES.items():
        rows = parse(path)
        per_capture[name] = rows
        for r in rows:
            r["capture"] = name
        all_rows.extend(rows)

    out = []
    out.append("# TypeScript census (raw parse output)\n")
    out.append(f"Total error lines parsed (all captures, union not deduped across overlapping scopes): {len(all_rows)}\n")
    for name, rows in per_capture.items():
        out.append(f"- capture `{name}` ({CAPTURES[name].name}): {len(rows)} errors")
    out.append("")

    # by code
    code_counter = Counter(r["code"] for r in all_rows)
    out.append("## Top 15 error codes (root capture only, full-repo scope)\n")
    root_rows = per_capture["root"]
    root_code_counter = Counter(r["code"] for r in root_rows)
    for code, cnt in root_code_counter.most_common(15):
        out.append(f"- {code}: {cnt}")
    out.append("")

    # by area (root capture)
    out.append("## By top-level area (root capture)\n")
    area_counter = Counter(classify_area(r["file"]) for r in root_rows)
    for area, cnt in area_counter.most_common():
        out.append(f"- {area}: {cnt}")
    out.append("")

    # by file top 40 (root capture)
    out.append("## Top 40 files by error count (root capture)\n")
    file_counter = Counter(r["file"] for r in root_rows)
    for f, cnt in file_counter.most_common(40):
        rec_min = mtime_recent_minutes(f)
        recent_flag = "LIVE-EDIT<90min" if (rec_min is not None and rec_min < 90) else ""
        an, ad = git_last(f)
        out.append(f"- {cnt:4d}  {f}  [{recent_flag}]  last-commit: {an} {ad}")
    out.append("")

    # hub/renderer breakdowns too
    for name in ("hub", "renderer"):
        rows = per_capture[name]
        out.append(f"## Capture `{name}`: by code (top 15)\n")
        cc = Counter(r["code"] for r in rows)
        for code, cnt in cc.most_common(15):
            out.append(f"- {code}: {cnt}")
        out.append(f"\n## Capture `{name}`: top 20 files\n")
        fc = Counter(r["file"] for r in rows)
        for f, cnt in fc.most_common(20):
            out.append(f"- {cnt:4d}  {f}")
        out.append("")

    report = "\n".join(out)
    dest = GEN / "ts-census-parsed.md"
    dest.write_text(report)
    print(report)


if __name__ == "__main__":
    main()
