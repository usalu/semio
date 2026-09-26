#!/usr/bin/env python3
"""🧪️ Tabulates `quick/<name>.txt` captures of `test-quick-all.sh`: exit, seconds, tests run/failed, and whether every
failure is `descriptor_is_fresh` (the committed descriptor predates this tree → W2's describe regenerates it)."""
import re
from pathlib import Path

OUT = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-t12-captures/quick")
print("| package | exit | s | passed | failed | failures |")
print("|---|---:|---:|---:|---:|---|")
counts = {"green": 0, "stale-only": 0, "other": 0}
for capture in sorted(OUT.glob("*.txt")):
    if capture.name in ("progress.txt", "chain.txt"):
        continue
    text = capture.read_text(encoding="utf-8", errors="replace")
    ended = re.search(r"^EXIT=(\d+) SECONDS=(\d+)$", text, re.M)
    if not ended:
        continue
    summaries = re.findall(r"Summary \[[^]]*\] (\d+) tests? run: (\d+) passed(?:, (\d+) failed)?", text)
    passed = sum(int(match[1]) for match in summaries)
    failed = sum(int(match[2] or 0) for match in summaries)
    failures = sorted(set(re.findall(r"^\s*test (\S+) \.\.\. FAILED", text, re.M)))
    compile_error = "error[E" in text or "could not compile" in text
    if ended.group(1) == "0":
        kind = "green"
    elif failures and all(name.endswith("descriptor_is_fresh") for name in failures) and not compile_error:
        kind = "stale-only"
    else:
        kind = "other"
    counts[kind] += 1
    shown = ", ".join(failures) if failures else ("compile error" if compile_error else "-")
    print(f"| {capture.stem} | {ended.group(1)} | {ended.group(2)} | {passed} | {failed} | {shown} ({kind}) |")
print(f"\n{counts}")
