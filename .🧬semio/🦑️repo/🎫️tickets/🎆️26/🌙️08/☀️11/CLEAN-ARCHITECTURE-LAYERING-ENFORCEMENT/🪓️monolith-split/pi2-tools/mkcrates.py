#!/usr/bin/env python3
"""🗂️ Builds `crates.json` of pi2 from the frozen Appendix H table (`🗑️generated/pwp0-crates.json`): the WP7 acyclic rank order of the instance family and the extra crate `instance-dispatch` (PLAN 12.6)."""
import json, copy
SPLIT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
c = json.load(open(SPLIT + "/🗑️generated/pwp0-crates.json"))
order = ["tool-run", "time-travel", "instance-stores", "instance-history", "instance-children", "instance-jobs", "instance-docs", "instance-extension", "instance-render", "instance-dispatch", "instance-typed-ops", "time-travel-turns", "tool-run-turns", "instance-commands", "instance-ladders", "instance-app", "builder", "testing-laws", "testing-acceptance"]
d = copy.deepcopy(c["instance-render"])
d.update(key="instance-dispatch", root="🧵️instance/📨️dispatch", members="DispatchCtx: dispatch_emit, reserved emit producers, host events, typed command admission, group history", name="semio-framework-plugin-instance-dispatch", id="plugin-instance-dispatch", project="@semio-tech/plugin-instance-dispatch-rs", emoji="📨️", desc="Instance dispatch of the plugin SDK: emission, reserved producers, host events and typed command admission", **{"class": "A"})
d["dir"] = d["dir"].rsplit("/", 1)[0] + "/📨️dispatch"
d["pkg"] = d["dir"] + "/📦️packages/🦀️rust"
c["instance-dispatch"] = d
base = 41
for i, k in enumerate(order):
    c[k]["rank"] = base + i
nxt = base + len(order)
for k, v in sorted(c.items(), key=lambda kv: kv[1]["rank"]):
    if v["kind"] in ("companion",):
        v["rank"] = nxt
        nxt += 1
r = copy.deepcopy(c["instance-dispatch"])
r.update(key="testing-caller", kind="instance", wp=7, root="🧪️testing/🎟️caller", members="OriginalFixtureCaller", name="semio-framework-plugin-testing-caller", id="plugin-testing-caller", project="@semio-tech/plugin-testing-caller-rs", rank=0.5, emoji="🎟️", desc="fixture caller", features=[], deps=[])
r["class"] = "A"
r["dir"] = r["dir"].rsplit("/🧵️instance/📨️dispatch", 1)[0] + "/🧪️testing/🎟️caller"
r["pkg"] = r["dir"] + "/📦️packages/🦀️rust"
c["testing-caller"] = r
json.dump(c, open(SPLIT + "/pi2-tools/crates.json", "w"), ensure_ascii=False, indent=1)
print(len(c), [(k, c[k]["rank"]) for k in order])
