"""📈️ W13: per worker and slice-worker pid (one read attempt) from the poll file argv[1]: first and last sample, the highest `taken` in the slice report and in the CLI line, the fleet counters at the end, the longest report silence, the last phase line."""
import json, re, sys, time, collections
attempts = collections.OrderedDict()
for line in open(sys.argv[1]):
    r = json.loads(line)
    if r["pid"] is None: continue
    key = (r["w"], r["pid"])
    a = attempts.setdefault(key, {"first": r["t"], "last": r["t"], "taken": None, "cli": None, "age": 0, "fleet": {}, "phase": "", "up": 0})
    a["last"] = r["t"]; a["age"] = max(a["age"], r["reportAge"] or 0); a["up"] = max(a["up"], r["up"] or 0)
    if r["taken"] is not None: a["taken"] = max(a["taken"] or 0, r["taken"])
    m = re.search(r"taken (\d+) of (\d+)", r["tail"] or "")
    if m: a["cli"] = max(a["cli"] or 0, int(m.group(1)))
    if r["fleet"]: a["fleet"] = r["fleet"]
    p = re.search(r"\(\d+ s\): (.{0,70})", r["tail"] or "")
    if p: a["phase"] = p.group(1)
fmt = lambda t: time.strftime("%H:%M", time.localtime(t))
for (w, pid), a in attempts.items():
    print(w[3:], pid, fmt(a["first"]) + "-" + fmt(a["last"]), "up<=", a["up"], "taken", a["taken"], "cli", a["cli"], "silence<=", a["age"], a["fleet"] or "", "|", a["phase"])
