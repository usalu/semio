"""shared bird thermal helpers (plumage layers, regions) for mallard-like and gull assets"""
from nw import *


def plumage_layers(prefix, R, th, cap, ref="de-vries-van-eerden-1995"):
    vr = lambda r: round(r / (0.0165 * 0.6), 2)
    return [layer(prefix + "-down", ("Down layer with enclosed air", "Daunenschicht mit eingeschlossener Luft"), "feather", "plumage", round(th*0.55, 4), round(R*0.6, 3), vr(R*0.6), round(cap*0.4), "authored-estimate", ref),
            layer(prefix + "-contour", ("Contour feather layer", "Konturfederschicht"), "feather", "plumage", round(th*0.45, 4), round(R*0.4, 3), vr(R*0.4), round(cap*0.6), "authored-estimate", "scholander-1950")]


def bird_regions(ar, sh, mats, wing_mat="wing-feathers", refs=("walsberg-king-1978", "scholander-1950", "kahl-1963")):
    skin = sum(ar.values())
    tot = sum(sh.values())
    sh = {k: v/tot for k, v in sh.items()}
    sf = lambda k: round(ar[k]/skin, 5)
    P = lambda k, R, th, cap: plumage_layers(k, R, th, cap)
    regs = [
        region("body-plumage", ("Body plumage (back, breast, belly)", "Körpergefieder (Rücken, Brust, Bauch)"), "feather", "body", "plumage", ar["body"], 0.22, "cylinder", 0.5, P("body", mats["body"][0], mats["body"][1], 900), 14000, ("trunk", round(sh["body"], 4), sf("body")), "authored-estimate", refs[0]),
        region("neck-plumage", ("Neck plumage", "Halsgefieder"), "feather", "body", "plumage", ar["neck"], 0.06, "cylinder", 0.5, P("neck", mats["neck"][0], mats["neck"][1], 600), 10000, ("head", round(sh["neck"], 4), sf("neck")), "authored-estimate", refs[1]),
        region("head-plumage", ("Head plumage", "Kopfgefieder"), "feather", "body", "plumage", ar["head"], 0.05, "sphere", 0.5, P("head", mats["head"][0], mats["head"][1], 400), 9000, ("head", round(sh["head"], 4), sf("head")), "authored-estimate", refs[1]),
        region("bill", ("Bill (keratin sheath)", "Schnabel (Hornscheide)"), "skin", "body", "bill-keratin", ar["bill"], 0.03, "cylinder", 0.5, [], 2500, ("head", round(sh["bill"], 4), sf("bill")), "authored-estimate", refs[2]),
        region("wing-plumage", ("Wings (both)", "Flügel (beide)"), "feather", "body", wing_mat, ar["wings"], 0.25, "vertical-plate", 0.5, P("wing", mats["wing"][0], mats["wing"][1], 700), 6000, ("wing", round(sh["wings"], 4), sf("wings")), "authored-estimate", refs[1]),
        region("tail-plumage", ("Tail feathers", "Schwanzfedern"), "feather", "body", "plumage", ar["tail"], 0.1, "vertical-plate", 0.5, P("tail", mats["tail"][0], mats["tail"][1], 400), 3500, ("tail", round(sh["tail"], 4), sf("tail")), "authored-estimate", refs[1]),
        region("legs-skin", ("Bare shank and tarsus (both legs)", "Unbefiederter Unterschenkel und Lauf (beide Beine)"), "skin", "body", "scaled-skin", ar["legs"], 0.015, "cylinder", 0.3, [], 3000, ("leg", round(sh["legs"], 4), sf("legs")), "authored-estimate", refs[2]),
        region("web-feet", ("Webbed feet (both)", "Schwimmfüße (beide)"), "skin", "body", "scaled-skin", ar["feet"], 0.07, "horizontal-plate-down", 0.0, [], 2500, ("foot", round(sh["feet"], 4), sf("feet")), "authored-estimate", refs[2]),
    ]
    s1 = sum(r["body"]["metabolicShare"] for r in regs)
    regs[0]["body"]["metabolicShare"] = round(regs[0]["body"]["metabolicShare"] + (1 - s1), 4)
    s2 = sum(r["body"]["skinFraction"] for r in regs)
    regs[0]["body"]["skinFraction"] = round(regs[0]["body"]["skinFraction"] + (1 - s2), 5)
    return regs, skin


def pattern(id, title, rate, vt, i, p1, e, p2, ref):
    return {"id": id, "title": tx(*title), "rateBpm": rate, "tidalVolumeL": vt, "inhaleFraction": i, "pauseAfterInhaleFraction": p1,
            "exhaleFraction": e, "pauseAfterExhaleFraction": p2, "route": "nose", "source": "authored-estimate", "reference": ref}


def chain_from_world(asset, parent_deg, ids, worlds):
    return offsets_from_world(asset, parent_deg, ids, worlds)
