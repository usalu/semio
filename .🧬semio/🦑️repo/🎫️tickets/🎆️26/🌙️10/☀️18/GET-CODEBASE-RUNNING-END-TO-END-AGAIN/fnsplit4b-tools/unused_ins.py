"""🔎 Lists the in-parameters of each planned phase that occur only as a shadowing binding or twice or less: usage unused_ins.py <spec.json> (after rsfn plan -v on stdin)."""
import json, re, subprocess, sys
spec = json.load(open(sys.argv[1], encoding="utf8"))
out = subprocess.run([sys.executable, __file__.rsplit("/", 1)[0] + "/rsfn.py", "plan", sys.argv[1], "-v"], capture_output=True, text=True).stdout.split("\n")
lines = open(spec["file"], encoding="utf8").read().split("\n")
cur = None
for l in out:
    m = re.match(r"phase (\S+)\s+stmts\s+(\d+)-(\d+)", l)
    if m:
        cur = (m.group(1), int(m.group(2)), int(m.group(3))); continue
    m = re.match(r"\s+ins (\[.*\])", l)
    if m and cur:
        names = eval(m.group(1))
        body = lines[cur[1] - 1: cur[2]]
        for n in names:
            uses = []
            for i, t in enumerate(body):
                for mm in re.finditer(r"(?<![\w.])%s\b" % re.escape(n), t):
                    uses.append((cur[1] + i, t.strip()[:110]))
            real = [u for u in uses if not re.search(r"\blet\s+(mut\s+)?%s\b|\|\s*%s\s*\||for\s+%s\s+in|Some\(%s\)|\(%s,|, %s\)" % ((re.escape(n),) * 6), u[1])]
            if len(real) == 0 or (len(uses) <= 2 and len(real) < len(uses)):
                print("%-34s %-10s uses=%d real=%d %s" % (cur[0], n, len(uses), len(real), [u[0] for u in uses][:4]))
