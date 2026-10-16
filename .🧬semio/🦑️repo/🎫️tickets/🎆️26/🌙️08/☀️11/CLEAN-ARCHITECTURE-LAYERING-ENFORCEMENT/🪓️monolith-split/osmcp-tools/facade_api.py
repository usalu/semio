import sys, re, os, collections
sys.path.insert(0, ".")
from layout import *
lay = Layout("plan3")
w = lay.w
REPO = "/Users/ueli/Documents/semio/"
consumers = [
 "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🌉️mcp/🦀️.rs",
 "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🌉️mcp/🧪️tests/🦀️.rs",
 "✏️s/🧑‍💻dev/💡️services/🦀️.rs",
 "🌎️hub/💡️inference/🧪️tests/🔬️client-contract/🦀️.rs",
 "🌎️hub/🔐️auth/🔌️client/🦀️.rs",
 "🌎️hub/🔐️auth/🔌️client/🧪️tests/🔬️unit/🦀️.rs",
 "🌎️hub/🧪️tests/🔬️bin-unit/🪶️count-lease/🦀️.rs",
 "🌎️hub/🔐️auth/🤖️agent/🦀️.rs",
 "🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🦀️.rs",
 "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs",
]
text = {}
for c in consumers:
    p = REPO + c
    if os.path.exists(p): text[c] = open(p, encoding="utf-8").read()
    else: print("missing", c)
mods = ["inference", "agent_credential", "workspace", "catalog", "protocol"]
# public names per old module (top-level facet + submodules by prefix)
pub = collections.defaultdict(dict)
for a in w.atoms:
    if id(a) in w.mount_atoms or id(a) not in lay.crate_of: continue
    if lay.flags[id(a)]: continue
    if not a.item.vis.startswith("pub") or a.item.vis.replace(" ", "") in ("pub(crate)", "pub(super)"): continue
    mp = a.file.modpath
    top = mp[1] if len(mp) > 1 else "root"
    pub[top].setdefault(a.name, []).append(a)
used = collections.defaultdict(set)
for c, t in text.items():
    ids = set(re.findall(r'[A-Za-z_]\w*', t))
    for top in list(pub) :
        for nm in pub[top]:
            if nm in ids: used[top].add(nm)
for top in sorted(used): print(top, len(used[top]), sorted(used[top])[:80])
