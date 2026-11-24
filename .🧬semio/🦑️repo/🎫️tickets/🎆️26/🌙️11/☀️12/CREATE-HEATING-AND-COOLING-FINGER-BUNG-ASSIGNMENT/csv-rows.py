"""🧾 Prints the numbered TeX rows of the CSV template labels for the assignment listing."""
import sys
sys.stdout.reconfigure(encoding="utf-8")
rows = open(sys.argv[1], encoding="utf-8-sig", newline="").read().split("\r\n")
assert rows[-1] == "" and len(rows) == 61
swaps = [("_", r"\_"), ("^", r"\textasciicircum{}"), ("ρ", r"$\rho$"), ("ΔΘ", r"$\Delta\Theta$"), ("'", r"\textquotesingle{}"), ("…", r"\ldots{}")]
for n, row in enumerate(rows[:-1], 1):
    label, value = row.split(";"); assert value == ""
    for a, b in swaps: label = label.replace(a, b)
    print("%d&%s\\\\" % (n, label.rstrip(" ")))
