from pathlib import Path
p = Path(__file__).resolve().parent / "verify-check-experiments.py"
lines = p.read_text(encoding="utf-8").split("\n")
out = []
for line in lines:
    if line.startswith("printed_literal = "):
        out.append("diff_70_30 = -278.9 - (-122.3)")
    elif line.startswith('verdict(abs(printed_literal'):
        out.append('verdict(abs(diff_70_30 - (-156.6)) < .06 and abs(0.5 * diff_70_30 - (-78.3)) < .06, "difference (three rows): -278.9 - (-122.3) = -156.6 and 1/2 x (-156.6) = -78.3", f"{diff_70_30:.2f} and {0.5 * diff_70_30:.2f}")')
    else:
        out.append(line)
p.write_text("\n".join(out), encoding="utf-8")
