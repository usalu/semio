"""🚚️ S20 faults overlay → live tree, ONE train (T6 row 12). For every file the overlay changed, created or deleted since its
baseline (`s20-overlay-diff.py`), a three-way merge: BASE = the live file's history version whose sha1 equals the
baseline sha1 (read-only `git log`/`git show`), OURS = the overlay, THEIRS = the live tree now (`git merge-file -p`,
stdout only — no repository state is touched). `plan` writes every merged file under the landing stage and a report;
`apply` copies the staged files into the live tree (L1 runs it inside the train, with its own byte backups) — refused
while any conflict or unresolved base remains. Idempotent: an already-landed file merges to itself.
Usage: python3 s20-overlay-land.py plan|apply"""
from __future__ import annotations

import difflib
import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
OVERLAY = LIVE / ".🧬semio/🌐hub/s14-s20-overlay-faults"
BASELINE = LIVE / ".🧬semio/🌐hub/s14-s20-overlay-faults.baseline.json"
STAGE = LIVE / ".🧬semio/🌐hub/s14-s20-landing"
REPORT = STAGE / "report.json"
DIFF = Path(__file__).resolve().parent / "s20-overlay-diff.py"
HISTORY_DEPTH = 400


def git(*args: str, binary: bool = False):
    result = subprocess.run(["git", "-c", "core.quotePath=false", *args], cwd=LIVE, capture_output=True)
    return result.stdout if binary else result.stdout.decode("utf-8", "replace")


def base_bytes(rel: str, sha: str) -> tuple[bytes | None, bool]:
    """🔎️ The baseline version of `rel` from history (exact sha1 match), else the history version around the clone that
    differs least from the overlay file (`approximate`, flagged in the report for review)."""
    near = git("log", "--format=%H", "--since=2026-09-29 06:45", "--until=2026-09-29 10:53", "--", rel).split()
    for commit in near + [commit for commit in git("log", f"-n{HISTORY_DEPTH}", "--format=%H", "--", rel).split() if commit not in near]:
        blob = git("show", f"{commit}:{rel}", binary=True)
        if hashlib.sha1(blob).hexdigest() == sha:
            return blob, False
    ours = (OVERLAY / rel).read_text(errors="replace").splitlines()
    best: tuple[int, bytes] | None = None
    for commit in near or git("log", "-n40", "--format=%H", "--", rel).split():
        blob = git("show", f"{commit}:{rel}", binary=True)
        distance = sum(1 for line in difflib.unified_diff(blob.decode("utf-8", "replace").splitlines(), ours, lineterm="", n=0) if line[:1] in "+-")
        if best is None or distance < best[0]:
            best = (distance, blob)
    return (best[1], True) if best else (None, False)


def ignored(paths: list[str]) -> set[str]:
    result = subprocess.run(["git", "-c", "core.quotePath=false", "check-ignore", "--stdin", "-z"], cwd=LIVE, input="\0".join(paths).encode(), capture_output=True)
    return {path for path in result.stdout.decode("utf-8", "replace").split("\0") if path}


def plan() -> dict:
    diff = json.loads(subprocess.run([sys.executable, str(DIFF), "diff"], capture_output=True, text=True, check=True).stdout)
    for kind in ("changed", "created", "deleted"):
        skipped = ignored(diff[kind])
        diff[kind] = [path for path in diff[kind] if path not in skipped and not path.startswith(".🧬semio/")]
    baseline = json.loads(BASELINE.read_text())
    if STAGE.exists():
        shutil.rmtree(STAGE)
    STAGE.mkdir(parents=True)
    rows: list[dict] = []
    for rel in diff["changed"]:
        ours, live = OVERLAY / rel, LIVE / rel
        if not live.exists():
            rows.append({"path": rel, "state": "CONFLICT", "why": "deleted in the live tree"})
            continue
        if hashlib.sha1(live.read_bytes()).hexdigest() == baseline[rel]:
            merged, state = ours.read_bytes(), "clean"
        else:
            base, approximate = base_bytes(rel, baseline[rel])
            if base is None:
                rows.append({"path": rel, "state": "CONFLICT", "why": "baseline version not found in history"})
                continue
            base_file = STAGE / ".base" / rel
            base_file.parent.mkdir(parents=True, exist_ok=True)
            base_file.write_bytes(base)
            merge = subprocess.run(["git", "merge-file", "-p", "--diff3", str(live), str(base_file), str(ours)], capture_output=True)
            merged, state = merge.stdout, ("merged-approximate-base" if approximate else "merged") if merge.returncode == 0 else "CONFLICT"
        target = STAGE / "files" / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(merged)
        rows.append({"path": rel, "state": state})
    for rel in diff["created"]:
        live = LIVE / rel
        state = "CONFLICT" if live.exists() and live.read_bytes() != (OVERLAY / rel).read_bytes() else "create"
        target = STAGE / "files" / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes((OVERLAY / rel).read_bytes())
        rows.append({"path": rel, "state": state, **({"why": "exists in the live tree with other content"} if state == "CONFLICT" else {})})
    for rel in diff["deleted"]:
        live = LIVE / rel
        state = "delete" if not live.exists() or hashlib.sha1(live.read_bytes()).hexdigest() == baseline[rel] else "CONFLICT"
        rows.append({"path": rel, "state": state, **({"why": "changed in the live tree"} if state == "CONFLICT" else {})})
    report = {"rows": rows, "conflicts": sum(row["state"] == "CONFLICT" for row in rows)}
    REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=1))
    return report


def apply() -> None:
    report = json.loads(REPORT.read_text())
    if report["conflicts"]:
        sys.exit(f"refused: {report['conflicts']} conflicts in {REPORT}")
    for row in report["rows"]:
        live = LIVE / row["path"]
        if row["state"] == "delete":
            live.unlink(missing_ok=True)
            continue
        live.parent.mkdir(parents=True, exist_ok=True)
        live.write_bytes((STAGE / "files" / row["path"]).read_bytes())
    print(f"applied {len(report['rows'])} files")


if sys.argv[1] == "plan":
    result = plan()
    states: dict[str, int] = {}
    for row in result["rows"]:
        states[row["state"]] = states.get(row["state"], 0) + 1
    print(json.dumps(states), f"report={REPORT}")
    for row in result["rows"]:
        if row["state"] == "CONFLICT":
            print("CONFLICT", row["path"], row.get("why", ""))
else:
    apply()
