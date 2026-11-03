"""📋 Writes a rows.py spec: for the given fixtures (uri list) every check id of the given prefixes whose utilisation is outside the near-limit band 0.93..1.07. Usage: spec_from_oracle.py <enXXXX> <out.tsv> <prefix,prefix> <fixture-uri>..."""
import glob, json, subprocess, sys
code, out, prefixes, fixtures = sys.argv[1], sys.argv[2], sys.argv[3].split(","), sys.argv[4:]
sub = glob.glob(f"/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*{code}/🏅️standards/🔖️1/🪆️subsets/✳️any")[0]
lines = []
for uri in fixtures:
    path = sub + "/🧪️tests/🧫️fixtures/" + uri.removeprefix("shared://")
    res = subprocess.run(["nice", "-n", "19", sys.executable, "-I", f"{sub}/🔮️oracles/🚦️check-outcome/🐍️.py", path], capture_output=True, text=True, check=True)
    for c in json.loads(res.stdout)["checks"]:
        u = c["utilization"]
        if any(c["id"].startswith(p) for p in prefixes) and not (0.93 <= u <= 1.07) and u < 1e5:
            lines.append(f"{uri}\t{c['id']}")
open(out, "w", encoding="utf8").write("\n".join(lines) + "\n")
print(len(lines), "lines")
