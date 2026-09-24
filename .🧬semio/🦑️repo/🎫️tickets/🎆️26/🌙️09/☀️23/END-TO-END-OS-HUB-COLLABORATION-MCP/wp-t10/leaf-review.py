"""🧐️ Lists leaves whose code evidence disagrees with their declared severity classes, grouped for hand review."""
import json, collections, sys
r = json.load(open("/Users/ueli/Documents/semio/.tmp-ticket/wp-t10/generated/leaf-evidence.json"))
groups = collections.defaultdict(list)
for p, v in r.items():
    d, e = set(v["declared"]), set(v["evidence"])
    rej_d = bool(d & {"error", "fatal"}); rej_e = "rejected" in e
    if rej_e and not rej_d: groups["rejected-undeclared"].append(p)
    if rej_d and not rej_e: groups["rejected-declared-unseen"].append(p)
    if "no-op" in e and not (d & {"info", "warning"}): groups["noop-undeclared"].append(p)
    if not e: groups["no-evidence"].append(p)
    if "applied" not in e and e: groups["no-applied"].append(p)
for k, ps in groups.items():
    c = collections.Counter("/".join(p.split("/")[:3]) for p in ps)
    print(k, len(ps), dict(c))
if len(sys.argv) > 1:
    for p in groups[sys.argv[1]]: print(r[p]["kind"], r[p]["declared"], r[p]["evidence"], r[p]["via"][:6], p)
