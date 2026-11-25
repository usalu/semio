"""🧾️ The proof lines of the closing runs of the takeover: `python3 proof-lines.py <GEN> <id>…` prints per asset the lines of $GEN/proof/<id>.light.out (gate, scan, silhouette, validate, sweep, check CLI), <id>.suites.out (six suites) and <id>.matrix.out (cells, failing checks by scale and view) as a markdown list, and a one-line verdict."""
import os, re, sys
gen = sys.argv[1]
for a in sys.argv[2:]:
    lines = []
    for part in ("light", "suites", "matrix"):
        path = os.path.join(gen, "proof", f"{a}.{part}.out")
        if not os.path.exists(path):
            lines.append(f"({part}: not run)")
            continue
        for line in open(path, encoding="utf8").read().splitlines():
            if re.fullmatch(r"[0-9a-f]{32}", line.strip()) or not line.strip() or "band-m" in line:
                continue
            lines.append(line.rstrip()[:260])
    text = "\n".join(lines)
    fail = re.search(r"(\d+) fail", text)
    clean = "gate exit 0" in text and "scan exit 0" in text and "silhouette: OK" in text and "validate: 0 issues" in text and "check: ✓" in text and fail is not None and fail.group(1) == "0" and "exit 1" not in text and "not run" not in text
    print(f"### `{a}`: {'clean, all proofs' if clean else 'OPEN'}\n")
    for line in lines:
        print(f"  - `{line.strip()}`")
    print()
