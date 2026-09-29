"""🚚️ S20 faults overlay ⇄ live tree (T6 row 12). OURS = every file the overlay changed, created or deleted since its
baseline (`s20-overlay-diff.py`); BASE = the live bytes the overlay was last based on (`BASES/<rel>`, written by `rebase`;
before the first rebase: the history version whose sha1 equals the baseline sha1, read-only `git log`/`git show`);
THEIRS = the live tree now. Every merge is `git merge-file -p --diff3` (stdout only — no repository state is touched).

- `plan`: every OURS file merged onto live → landing stage + report (L1's `apply` input).
- `apply`: copies the staged files into the live tree (inside the train) — refused while any conflict remains.
- `rebase [--write]`: moves the overlay onto the live tree: every non-OURS file := live, every OURS file := its merge;
  conflicts are written with diff3 markers under `REBASE/conflicts/<rel>`; a hand resolution goes to `REBASE/resolved/<rel>`
  (+ the live sha1 it answers in `REBASE/resolved.json`). `--write` (refused while a conflict is unresolved) backs up the
  overlay's OURS bytes + the old baseline, writes the overlay, the new baseline (= live at the rebase) and `BASES`.
Idempotent: an already-landed / already-rebased file merges to itself.
- `baseline`: records the overlay's current sources as its baseline (a new set's start; refuses to overwrite one).
Sets: `S20_SET=<name>` selects `s14-s20-overlay-faults-<name>` (pass 2 = `p2`); unset = the pass-1 overlay.
Usage: [S20_SET=p2] python3 s20-overlay-land.py plan|apply|rebase [--write]|baseline"""
from __future__ import annotations

import difflib
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
HUB = LIVE / ".🧬semio/🌐hub"
SET = os.environ.get("S20_SET", "")
SUFFIX = f"-{SET}" if SET else ""
OVERLAY = HUB / f"s14-s20-overlay-faults{SUFFIX}"
BASELINE = HUB / f"s14-s20-overlay-faults{SUFFIX}.baseline.json"
BASES = HUB / f"s14-s20-overlay-faults{SUFFIX}.bases"
STAGE = HUB / f"s14-s20-landing{SUFFIX}"
REPORT = STAGE / "report.json"
REBASE = HUB / f"s14-s20-rebase{SUFFIX}"
RESOLVED = REBASE / "resolved.json"
SKIP = {"node_modules", "target", ".nx", "dist", "🗑️generated", ".git", "__pycache__", ".venv", ".pytest_cache"}
TOP_SKIP = {".🧬semio"}
HISTORY_DEPTH = 400
TOKEN = re.compile(r"\s+|\w+|[^\w\s]")
NEWLINE = "\ue000"


def git(*args: str, binary: bool = False):
    result = subprocess.run(["git", "-c", "core.quotePath=false", *args], cwd=LIVE, capture_output=True)
    return result.stdout if binary else result.stdout.decode("utf-8", "replace")


def sha(data: bytes) -> str:
    return hashlib.sha1(data).hexdigest()


def walk(root: Path) -> dict[str, str]:
    """🚶️ sha1 of every source file under `root` (same skip set as the baseline, `.🧬semio` excluded)."""
    out: dict[str, str] = {}
    for current, dirs, names in os.walk(root):
        rel_dir = Path(current).relative_to(root)
        dirs[:] = [d for d in dirs if d not in SKIP and not d.startswith("target-") and not (rel_dir == Path(".") and d in TOP_SKIP)]
        for name in names:
            path = Path(current) / name
            try:
                out[str(path.relative_to(root))] = sha(path.read_bytes())
            except OSError:
                continue
    return out


def ignored(paths: list[str]) -> set[str]:
    result = subprocess.run(["git", "-c", "core.quotePath=false", "check-ignore", "--stdin", "-z"], cwd=LIVE, input="\0".join(paths).encode(), capture_output=True)
    return {path for path in result.stdout.decode("utf-8", "replace").split("\0") if path}


def ours(overlay: dict[str, str], baseline: dict[str, str], tracked_only: bool) -> dict[str, list[str]]:
    """🧾️ The overlay's own change set against its baseline (`.🧬semio` never; gitignored files only when not `tracked_only`)."""
    diff = {
        "changed": sorted(p for p in overlay if p in baseline and overlay[p] != baseline[p]),
        "created": sorted(p for p in overlay if p not in baseline),
        "deleted": sorted(p for p in baseline if p not in overlay and not p.startswith(".🧬semio/")),
    }
    for kind in diff:
        skipped = ignored(diff[kind]) if tracked_only else set()
        diff[kind] = [p for p in diff[kind] if p not in skipped and not p.startswith(".🧬semio/")]
    return diff


def base_bytes(rel: str, baseline_sha: str) -> tuple[bytes | None, bool]:
    """🔎️ BASE of `rel`: the blob the last rebase stored, else the history version with the baseline sha1, else the history
    version around the clone that differs least from the overlay file (`approximate`, flagged in the report)."""
    stored = BASES / rel
    if stored.exists():
        return stored.read_bytes(), False
    near = git("log", "--format=%H", "--since=2026-09-29 06:45", "--until=2026-09-29 10:53", "--", rel).split()
    for commit in near + [commit for commit in git("log", f"-n{HISTORY_DEPTH}", "--format=%H", "--", rel).split() if commit not in near]:
        blob = git("show", f"{commit}:{rel}", binary=True)
        if sha(blob) == baseline_sha:
            return blob, False
    mine = (OVERLAY / rel).read_text(errors="replace").splitlines()
    best: tuple[int, bytes] | None = None
    for commit in near or git("log", "-n40", "--format=%H", "--", rel).split():
        blob = git("show", f"{commit}:{rel}", binary=True)
        distance = sum(1 for line in difflib.unified_diff(blob.decode("utf-8", "replace").splitlines(), mine, lineterm="", n=0) if line[:1] in "+-")
        if best is None or distance < best[0]:
            best = (distance, blob)
    return (best[1], True) if best else (None, False)


def merge(rel: str, baseline_sha: str, live_bytes: bytes, scratch: Path) -> tuple[str, bytes, str]:
    """🔀️ OURS of `rel` merged onto `live_bytes` → (state, bytes, why)."""
    mine = (OVERLAY / rel).read_bytes()
    if sha(live_bytes) == baseline_sha:
        return "clean", mine, ""
    base, approximate = base_bytes(rel, baseline_sha)
    if base is None:
        return "CONFLICT", b"", "baseline version not found"
    trio = [scratch / "live", scratch / "base", scratch / "ours"]
    for path, data in zip(trio, (live_bytes, base, mine)):
        path.write_bytes(data)
    result = subprocess.run(["git", "merge-file", "-p", "--diff3", "-L", "live", "-L", "base", "-L", "overlay", *map(str, trio)], capture_output=True)
    if result.returncode == 0:
        return ("merged-approximate-base" if approximate else "merged"), result.stdout, ""
    tokens = token_merge(live_bytes, base, mine, scratch)
    if tokens is not None:
        return ("merged-tokens-approximate-base" if approximate else "merged-tokens"), tokens, ""
    return "CONFLICT", result.stdout, f"{result.returncode} conflict hunks"


def token_merge(live: bytes, base: bytes, mine: bytes, scratch: Path) -> bytes | None:
    """🧩️ The same three-way merge over word/punctuation/whitespace tokens (one token per line): changes that touch
    neighbouring LINES but not the same TOKENS merge; `None` while a token-level conflict remains."""
    trio = [scratch / "live.tokens", scratch / "base.tokens", scratch / "ours.tokens"]
    for path, data in zip(trio, (live, base, mine)):
        text = data.decode("utf-8")
        path.write_text("\n".join(token.replace("\n", NEWLINE) for token in TOKEN.findall(text)) + "\n")
    result = subprocess.run(["git", "merge-file", "-p", *map(str, trio)], capture_output=True)
    if result.returncode != 0:
        return None
    return "".join(result.stdout.decode("utf-8")[:-1].split("\n")).replace(NEWLINE, "\n").encode()


def plan() -> dict:
    baseline = json.loads(BASELINE.read_text())
    diff = ours(walk(OVERLAY), baseline, tracked_only=True)
    if STAGE.exists():
        shutil.rmtree(STAGE)
    (STAGE / ".scratch").mkdir(parents=True)
    rows: list[dict] = []
    for rel in diff["changed"]:
        live = LIVE / rel
        if not live.exists():
            rows.append({"path": rel, "state": "CONFLICT", "why": "deleted in the live tree"})
            continue
        state, merged, why = merge(rel, baseline[rel], live.read_bytes(), STAGE / ".scratch")
        target = STAGE / ("conflicts" if state == "CONFLICT" else "files") / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(merged)
        rows.append({"path": rel, "state": state, **({"why": why} if why else {})})
    for rel in diff["created"]:
        live = LIVE / rel
        state = "CONFLICT" if live.exists() and live.read_bytes() != (OVERLAY / rel).read_bytes() else "create"
        target = STAGE / "files" / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes((OVERLAY / rel).read_bytes())
        rows.append({"path": rel, "state": state, **({"why": "exists in the live tree with other content"} if state == "CONFLICT" else {})})
    for rel in diff["deleted"]:
        live = LIVE / rel
        state = "delete" if not live.exists() or sha(live.read_bytes()) == baseline[rel] else "CONFLICT"
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


def rebase(write: bool) -> dict:
    baseline = json.loads(BASELINE.read_text())
    overlay, live = walk(OVERLAY), walk(LIVE)
    mine = ours(overlay, baseline, tracked_only=False)
    own = set(mine["changed"]) | set(mine["created"]) | set(mine["deleted"])
    resolved = json.loads(RESOLVED.read_text()) if RESOLVED.exists() else {}
    scratch = REBASE / ".scratch"
    if (REBASE / "conflicts").exists():
        shutil.rmtree(REBASE / "conflicts")
    scratch.mkdir(parents=True, exist_ok=True)
    rows: list[dict] = []
    writes: dict[str, bytes | None] = {}
    bases: dict[str, bytes] = {}
    for rel in mine["changed"]:
        if rel not in live:
            if resolved.get(rel) == "deleted":
                writes[rel] = None
                rows.append({"path": rel, "state": "resolved-deleted"})
            else:
                rows.append({"path": rel, "state": "CONFLICT", "why": "deleted in the live tree"})
            continue
        theirs = (LIVE / rel).read_bytes()
        if rel in resolved and resolved[rel] == sha(theirs):
            state, merged, why = "resolved", (REBASE / "resolved" / rel).read_bytes(), ""
        else:
            state, merged, why = merge(rel, baseline[rel], theirs, scratch)
        if state == "CONFLICT":
            target = REBASE / "conflicts" / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(merged)
        else:
            writes[rel] = merged
        bases[rel] = theirs
        rows.append({"path": rel, "state": state, "live": sha(theirs), **({"why": why} if why else {})})
    for rel in mine["created"]:
        if rel in live and live[rel] != overlay[rel]:
            rows.append({"path": rel, "state": "CONFLICT", "why": "created in the live tree with other content"})
    for rel in mine["deleted"]:
        if rel in live and live[rel] != baseline[rel]:
            rows.append({"path": rel, "state": "CONFLICT", "why": "changed in the live tree"})
    sync: dict[str, list[str]] = {"copy": [], "create": [], "delete": []}
    for rel in sorted(set(live) | set(overlay)):
        if rel in own or rel.startswith(".🧬semio/"):
            continue
        if rel not in live:
            if rel in baseline:
                writes[rel] = None
                sync["delete"].append(rel)
        elif overlay.get(rel) != live[rel]:
            writes[rel] = (LIVE / rel).read_bytes()
            sync["create" if rel not in overlay else "copy"].append(rel)
    states: dict[str, int] = {}
    for row in rows:
        states[row["state"]] = states.get(row["state"], 0) + 1
    report = {"states": states, "sync": {kind: len(paths) for kind, paths in sync.items()}, "conflicts": states.get("CONFLICT", 0), "rows": rows, "syncPaths": sync}
    (REBASE / "report.json").write_text(json.dumps(report, ensure_ascii=False, indent=1))
    if not write:
        return report
    if report["conflicts"]:
        sys.exit(f"refused: {report['conflicts']} unresolved conflicts ({REBASE / 'conflicts'})")
    stamp = time.strftime("%m%d-%H%M%S")
    backup = REBASE / f"backup-{stamp}"
    for rel in own:
        if (OVERLAY / rel).exists():
            target = backup / "ours" / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((OVERLAY / rel).read_bytes())
    shutil.copy2(BASELINE, backup / "baseline.json")
    if BASES.exists():
        shutil.copytree(BASES, backup / "bases")
    for rel, data in writes.items():
        path = OVERLAY / rel
        if data is None:
            path.unlink(missing_ok=True)
            continue
        if not path.exists() or path.read_bytes() != data:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
    for rel, data in bases.items():
        path = BASES / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    moved = {rel: digest for rel, digest in baseline.items() if rel.startswith(".🧬semio/")}
    moved.update({rel: sha(bases[rel]) if rel in bases else digest for rel, digest in live.items()})
    BASELINE.write_text(json.dumps(moved))
    report["backup"] = str(backup)
    (REBASE / "report.json").write_text(json.dumps(report, ensure_ascii=False, indent=1))
    return report


if __name__ == "__main__":
    command = sys.argv[1]
    if command == "plan":
        result = plan()
        counts: dict[str, int] = {}
        for row in result["rows"]:
            counts[row["state"]] = counts.get(row["state"], 0) + 1
        print(json.dumps(counts), f"report={REPORT}")
        for row in result["rows"]:
            if row["state"] == "CONFLICT":
                print("CONFLICT", row["path"], row.get("why", ""))
    elif command == "apply":
        apply()
    elif command == "rebase":
        result = rebase("--write" in sys.argv[2:])
        print(json.dumps(result["states"]), json.dumps(result["sync"]), result.get("backup", "dry-run"), f"report={REBASE / 'report.json'}")
        for row in result["rows"]:
            if row["state"] == "CONFLICT":
                print("CONFLICT", row["path"], row.get("why", ""))
    elif command == "baseline":
        if BASELINE.exists():
            sys.exit(f"refused: {BASELINE} exists")
        snapshot = walk(OVERLAY)
        BASELINE.write_text(json.dumps(snapshot))
        print(f"baseline files={len(snapshot)} → {BASELINE}")
    else:
        sys.exit(__doc__)
