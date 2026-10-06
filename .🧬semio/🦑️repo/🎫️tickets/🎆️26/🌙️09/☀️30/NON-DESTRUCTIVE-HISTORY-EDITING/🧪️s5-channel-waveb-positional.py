#!/usr/bin/env python3
"""🎯️ S5-CHANNEL wave B: the positional description arguments the driver never saw.

`🧪️s4-bump-waveb.py` finds its candidates by the word `description`; a file that only passes the argument positionally
(`begin_apply_batch(…, None, …)`, `.preflight(&mutation, None, lane)`, `DurableOwnedMapMemberAdmissionV1::new(…, Some(..))`)
never contains it. This script lists every such call with the driver's own call table (`--scan`, read-only, whole tree minus
the driver's write set and the hot files) and removes the argument in an EXPLICIT file list (`--apply --files-from <list>`):
the computed set must equal the list, originals are added to the driver's backup + manifest so `--restore` covers them.
Usage: python3 🧪️s5-channel-waveb-positional.py --scan <waveb-files.txt> [--list <out>]
       python3 🧪️s5-channel-waveb-positional.py --apply --files-from <list>
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TICKET = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("waveb", TICKET / "🧪️s4-bump-waveb.py")
waveb = importlib.util.module_from_spec(spec)
spec.loader.exec_module(waveb)


def value_after(flag: str) -> str:
    if flag not in sys.argv or sys.argv.index(flag) + 1 >= len(sys.argv):
        raise SystemExit(f"[DEBUG] {flag} needs a path")
    return sys.argv[sys.argv.index(flag) + 1]


def swept(path: str) -> tuple[str, str, list[tuple[int, str]]]:
    source = (ROOT / path).read_text()
    result, log = waveb.sweep(source, waveb.collect_calls)
    return source, result, log


def scan() -> None:
    written = {line for line in pathlib.Path(value_after("--scan")).read_text().splitlines() if line}
    if len(written) < 100:
        raise SystemExit("[DEBUG] the driver's file list is too short: refusing")
    names = sorted({name.lstrip(".") for name in waveb.CALLS})
    listed = subprocess.run(["git", "-c", "core.quotepath=off", "grep", "-l", "--untracked", "-E", "|".join(names), "--", "*.rs", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True, check=True).stdout
    found: list[str] = []
    for path in listed.splitlines():
        if path in written or path in waveb.HOT or not (ROOT / path).is_file():
            continue
        _, _, log = swept(path)
        for line, what in log:
            print(f"{path}:{line}: {what}")
        if log:
            found.append(path)
    if "--list" in sys.argv:
        pathlib.Path(value_after("--list")).write_text("".join(f"{path}\n" for path in sorted(found)))
    print(f"[DEBUG] {len(found)} files hold a positional description argument")


def apply() -> None:
    expected = [line for line in pathlib.Path(value_after("--files-from")).read_text().splitlines() if line]
    if not expected:
        raise SystemExit("[DEBUG] empty file list: refusing")
    results: dict[str, tuple[str, str]] = {}
    for path in expected:
        if not (ROOT / path).is_file():
            raise SystemExit(f"[DEBUG] missing file, nothing written: {path}")
        source, result, log = swept(path)
        if result == source:
            raise SystemExit(f"[DEBUG] listed but unchanged, nothing written: {path}")
        for line, what in log:
            print(f"{path}:{line}: {what}")
        results[path] = (source, result)
    manifest = json.loads(waveb.MANIFEST.read_text()) if waveb.MANIFEST.is_file() else {}
    digest = lambda text: hashlib.sha256(text.encode()).hexdigest()
    for path, (source, result) in results.items():
        if path in manifest:
            raise SystemExit(f"[DEBUG] already in the driver's manifest, nothing written: {path}")
    for path, (source, result) in results.items():
        backup = waveb.BACKUP / path
        backup.parent.mkdir(parents=True, exist_ok=True)
        backup.write_text(source)
        manifest[path] = {"before": digest(source), "after": digest(result)}
    waveb.MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=1))
    for path, (source, result) in results.items():
        (ROOT / path).write_text(result)
    print(f"[DEBUG] {len(results)} files written")


if __name__ == "__main__":
    if "--scan" in sys.argv:
        scan()
    elif "--apply" in sys.argv:
        apply()
    else:
        raise SystemExit("usage: --scan <waveb-files.txt> [--list <out>] | --apply --files-from <list>")
