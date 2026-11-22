"""🔩️ Scissor lift, second round pass 1: every part joined to what carries it (pivot bearer on the chassis, full-width roller axle, rail posts in their sockets, gate hinges and latch, control box bracket, coarse-scale track, lugs and stub axles) with the hand-chosen numbers below. Idempotent."""
import json
P = "volumes/machinery/scissor-lift.json"
doc = json.load(open(P))
V = doc["volumes"]
GEO = ("kind", "size", "transform", "repeat", "cornerRadius", "profile", "axis", "origin", "plane", "from", "to", "holes", "vertices", "faces", "tags")

def find(lst, i):
    return next((x for x in lst if x["id"] == i), None)

def put(lst, after, new):
    lst[:] = [x for x in lst if x["id"] != new["id"]]
    k = next((n for n, x in enumerate(lst) if x["id"] == after), len(lst) - 1)
    lst.insert(k + 1, new)

def solid(scale, sibling, i, geo, region, member=None, bone=None):
    s = V[scale]
    base = {k: v for k, v in find(s["body"], sibling).items() if k not in GEO}
    base.update({"id": i, **geo})
    if bone: base["bone"] = bone
    put(s["body"], sibling, base)
    put(s["thermal"], "th-" + sibling, {"id": "th-" + i, "of": i, "region": region})
    if member: link(scale, i, member)

def link(scale, i, member, after=None):
    put(V[scale]["structure"], after or "member-roller-slide", {"id": "member-" + i, "of": i, "member": member, "part": "member"})

def box(size, at, repeat=None):
    g = {"kind": "box", "size": size, "transform": {"translation": at}}
    if repeat: g["repeat"] = {"count": 2, "translation": repeat}
    return g

def rod(r, length, origin, repeat=None):
    g = {"kind": "revolve", "profile": [[0, 0], [0, r], [length, r], [length, 0]], "axis": [0, 0, 1], "origin": origin}
    if repeat: g["repeat"] = {"count": 2, "translation": repeat}
    return g

for scale, r in (("50", 0.025), ("100", 0.025), ("200", 0.025), ("500", 0.0375)):
    t = find(V[scale]["body"], "tube-b1-root")
    t["origin"] = [0, 0, -0.215]
    t["profile"] = [[0, 0], [0, r], [0.43, r], [0.43, 0]]

for scale in ("50", "100"):
    for i in ("post-rear", "post-mid", "post-front"):
        p = find(V[scale]["body"], i)
        p["size"][1] = 1.21
        p["transform"]["translation"][1] = 0.605
    for x in V[scale]["structure"]:
        if x["id"].startswith("member-joist-"):
            x["size"][1] = 0.05
            x["transform"]["translation"][1] = 0.075

solid("50", "pothole-guard", "lug-bearer", box([0.09, 0.08, 0.142], [-0.7275, 0.36, 0.271], [0, 0, -0.542]), "chassis", "chassis")
solid("50", "gate-stile", "gate-hinge", box([0.025, 0.05, 0.015], [-0.07, 0.6, 0.33], [0, -0.3, 0]), "rails", "platform")
solid("50", "gate-hinge", "gate-latch", box([0.03, 0.025, 0.015], [-0.07, 0.6, -0.33]), "rails", "platform")
solid("50", "control-box", "control-mount", box([0.16, 0.11, 0.01], [-0.235, 1.155, -0.265]), "rails")
link("50", "cylinder-eye", "lift-cylinder", "member-cylinder")
link("50", "cylinder-gland", "lift-cylinder", "member-cylinder")
link("50", "rod-eye", "lift-rod", "member-rod")

solid("100", "tray-left", "lug-bearer", box([0.09, 0.08, 0.145], [-0.7275, 0.36, 0.2725], [0, 0, -0.545]), "chassis", "chassis")
solid("100", "ladder-stile", "ladder-bracket", box([0.015, 0.16, 0.03], [-0.8625, 0.28, 0.195], [0, 0, -0.39]), "chassis")
solid("100", "control-box", "control-mount", box([0.16, 0.11, 0.015], [-0.235, 1.155, -0.2625]), "rails")
link("100", "track", "chassis")
link("100", "base-lug", "chassis")
c = find(V["100"]["structure"], "member-cylinder")
V["100"]["structure"][V["100"]["structure"].index(c)] = {"id": "member-cylinder", "of": "cylinder-barrel", "member": "lift-cylinder", "part": "member"}

find(V["200"]["body"], "roller-slide")["profile"] = [[0, 0], [0, 0.03], [0.04, 0.03], [0.04, 0]]
solid("200", "tray", "track", box([0.91, 0.03, 0.04], [0.3433, 0.415, 0.19], [0, 0, -0.38]), "chassis", "chassis")
solid("200", "track", "base-lug", box([0.09, 0.1, 0.03], [-0.7275, 0.45, 0.335], [0, 0, -0.67]), "chassis", "chassis")
solid("200", "base-lug", "lug-bearer", box([0.09, 0.08, 0.15], [-0.7275, 0.36, 0.275], [0, 0, -0.55]), "chassis", "chassis")
solid("200", "control-box", "control-mount", box([0.16, 0.11, 0.03], [-0.235, 1.155, -0.275]), "rails")

solid("500", "tray", "lug-bearer", box([0.09, 0.095, 0.145], [-0.7275, 0.3675, 0.2725], [0, 0, -0.545]), "chassis", "chassis")
solid("500", "tray", "axle-f", rod(0.0375, 0.135, [0.66, 0.1525, 0.165], [0, 0, -0.465]), "chassis")
solid("500", "axle-f", "axle-r", rod(0.0375, 0.135, [-0.66, 0.1525, 0.165], [0, 0, -0.465]), "chassis")

for scale in ("200", "500"):
    b = find(V[scale]["structure"], "member-deck-beam")
    b["size"][1] = 0.14
    b["transform"]["translation"][1] = 0.03

json.dump(doc, open(P, "w"), ensure_ascii=False, indent=1)
print("written", {k: [len(V[k][l]) for l in ("body", "thermal", "structure")] for k in V})
