"""Per-family pass/fail row coverage of a code's check-outcome feature. Usage: cover.py <enXXXX> <instance-regex> [--all]"""
import glob, re, sys
code, inst = sys.argv[1], re.compile(sys.argv[2])
sub = glob.glob(f"/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*{code}/🏅️standards/🔖️1/🪆️subsets/✳️any")[0]
text = open(glob.glob(sub + "/🧪️tests/🚦️check-outcome-*/🥒️.feature")[0], encoding="utf8").read()
have = {}
for s, c in re.findall(r"\|\s*(pass|fail)-(?:v\d+-)?\d+\s*\|\s*\S+\s*\|\s*(\S+)\s*\|", text):
    have.setdefault(inst.sub("", c), set()).add(s)
for f in sorted(have):
    if "--all" in sys.argv or len(have[f]) < 2:
        print(f, "+".join(sorted(have[f])))
print(len(have), "families;", sum(len(v) == 2 for v in have.values()), "paired")
