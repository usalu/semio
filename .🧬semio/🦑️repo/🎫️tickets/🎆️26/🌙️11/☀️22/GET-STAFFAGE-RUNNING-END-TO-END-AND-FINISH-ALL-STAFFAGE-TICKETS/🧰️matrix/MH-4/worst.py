"""🎯️ The subjects a gate log names as worst for a hard finding, as one regular expression for `gate.ts --only`: `python3 worst.py <gate log>… [--twins]` (with `--twins` the worst subjects of the twin tier too); the poses are always included."""
import re, sys
subjects = set()
hard = False
for path in [a for a in sys.argv[1:] if not a.startswith("--")]:
    for line in open(path):
        if not line.startswith("    "):
            hard = " HARD " in line or ("--twins" in sys.argv and " twin " in line)
        if not hard: continue
        m = re.search(r" mm @ (.*?)(?::: | :: |: |$)", line.rstrip("\n"))
        if m: subjects.add(m.group(1).strip())
esc = lambda s: re.sub(r"([.^$*+?()\[\]{}|\\/])", r"\\\1", s)
print("^(pose .*|" + "|".join(sorted(esc(s) for s in subjects if not s.startswith("pose "))) + ")$")
