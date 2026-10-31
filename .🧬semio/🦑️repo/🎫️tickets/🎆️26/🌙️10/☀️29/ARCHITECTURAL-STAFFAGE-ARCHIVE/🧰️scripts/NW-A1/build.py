# [scratch] NW-A1 assembly helpers: thermal regions, poses, gait clips, asset document
import json, math, os
from lib import *
from quad import *
from gait import *

S_ROOT = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
ANIMALS = S_ROOT + "/🗂️catalog/🐕️animals"


def Lay(id, title, kind, mat, t, k, f, cap, im, src, ref):
    r = t / k * f
    return {"id": id, "title": title, "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": r4(r), "vapourResistanceM2PaW": round(r / (16.5e-3 * im), 2), "heatCapacityJM2K": cap, "source": src, "reference": ref}


def Reg(id, title, kind, material, skin, f, layers, geom, Lc, sky, cap, seg, mshare, src, ref):
    return {"id": id, "title": title, "kind": kind, "model": "body", "material": material, "areaM2": round(skin * f, 4), "characteristicLengthM": Lc, "geometry": geom, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": cap, "body": {"segment": seg, "metabolicShare": mshare, "skinFraction": skin}, "source": src, "reference": ref, "_skin": skin}


def finalize_regions(regs):
    sa = sum(r["_skin"] for r in regs)
    ms = sum(r["body"]["metabolicShare"] for r in regs)
    out = []
    for r in regs:
        r = dict(r)
        r["body"] = dict(r["body"])
        r["body"]["skinFraction"] = round(r["_skin"] / sa, 5)
        r["body"]["metabolicShare"] = round(r["body"]["metabolicShare"] / ms, 5)
        del r["_skin"]
        out.append(r)
    for key in ("skinFraction", "metabolicShare"):
        tot = round(sum(r["body"][key] for r in out), 5)
        out[0]["body"][key] = round(out[0]["body"][key] + (1 - tot), 5)
    return out, sa


def bind(regions, act, breathing, sun, over=None, emitters=None):
    over = over or {}
    regs = {}
    for r in regions:
        rid = r["id"]
        b = {"exposedFraction": 1.0, "sunlitFraction": sun.get(rid, sun.get("*", 0.4))}
        b.update(over.get(rid, {}))
        regs[rid] = b
    return {"activityMet": act, "breathing": breathing, "regions": regs, "emitters": emitters if emitters is not None else {"plume-back": 1.0}}


def pose(id, title, desc, offs, thermal, contacts, root=None, ground="auto", key=False, loads=None):
    p = {"id": id, "title": title, "description": desc, "bones": poses_offsets(offs), "ground": ground, "contacts": contacts, "thermal": thermal, "loads": loads or []}
    if key:
        p["key"] = True
    if root:
        p["root"] = root
    return p


def gait_clip(sk, cid, ctitle, cdesc, ptitle, T_s, N, duty, order, speed, Gh, Gf, base_offs, legs_x, thermal_fn, foot, trunk_fn=None, bob=None, ground="auto", phases=None, tracks=None, names=None):
    """Builds the key poses and the looping planted clip of a symmetric gait. Returns (poses, clip, S)."""
    legs = [Leg(sk, k, foot, order[k], legs_x[k]) for k in ("hn", "fn", "hf", "ff")]
    S = speed * duty * T_s
    keys, poses = [], []
    stem = cid.replace("-cycle", "")
    for k in range(N):
        ph = k / N
        base = dict(base_offs)
        if trunk_fn:
            base.update(trunk_fn(ph))
        offs, res = solve_phase(sk, base, legs, ph + 1e-9, duty, S, Gh, Gf, extra_y=-(bob(ph) if bob else 0.0))
        contacts = ["contact-" + key for key, (st, p) in res.items() if st]
        authored = ground == "authored" or not contacts
        root = {"y": r4(sk.hip[1] + (bob(ph) if bob else 0.0))} if authored else None
        pid = "%s-phase-%d" % (stem, k)
        pt = ptitle(k, N)
        poses.append(pose(pid, pt[0], pt[1], offs, thermal_fn(ph, contacts), contacts, key=True, ground="authored" if authored else "auto", root=root))
        keys.append({"t": r4(ph * T_s), "pose": pid, "ease": "smooth"})
    spans, events = [], []
    for l in legs:
        a, b = (l.td * T_s) % T_s, ((l.td + duty) * T_s) % T_s
        if a < b:
            spans.append({"anchor": l.anchor, "fromS": r4(a), "toS": r4(b)})
        else:
            spans.append({"anchor": l.anchor, "fromS": r4(a), "toS": r4(T_s)})
            if b > 0:
                spans.append({"anchor": l.anchor, "fromS": 0.0, "toS": r4(b)})
        nm = names[l.key]
        events.append({"t": r4(a), "kind": "contact-on", "title": T("%s touches down" % nm[0], "%s setzt auf" % nm[1]), "target": l.anchor})
        events.append({"t": r4(b), "kind": "contact-off", "title": T("%s lifts" % nm[0], "%s hebt ab" % nm[1]), "target": l.anchor})
    events.sort(key=lambda e: e["t"])
    clip = {"id": cid, "title": ctitle, "description": cdesc, "durationS": T_s, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": r4(S / duty), "keys": keys, "contacts": spans, "phases": phases or [], "events": events, "tracks": tracks or []}
    return poses, clip, S
