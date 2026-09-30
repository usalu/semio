"""⚖️ W3-CODES: an apply-time rejection (`MutationApplyError`, always `mutation.apply.<detail>`) is Fatal by the frozen
vocabulary; the per-aggregate `apply_*_mutation` helpers reported it at Error. Rewrites exactly the listed lines
(`<NOW|MEDIA>\t<file>\t<line>` rows from the census) whose lane is selected on the command line. Idempotent."""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
rows, lanes = sys.argv[1], set(sys.argv[2:])
for row in pathlib.Path(rows).read_text(encoding="utf-8").splitlines():
    lane, file, line = row.split("\t")
    if lane not in lanes:
        continue
    path = ROOT / file
    lines = path.read_text(encoding="utf-8").split("\n")
    index = int(line) - 1
    before = lines[index]
    lines[index] = before.replace("MutationOutcome::error(error.code", "MutationOutcome::fatal(error.code", 1)
    if lines[index] != before:
        path.write_text("\n".join(lines), encoding="utf-8")
        print("[w3-codes] fatal apply rejection", file, line)
    elif "MutationOutcome::fatal(error.code" not in before:
        print("[w3-codes] UNMATCHED", file, line)
