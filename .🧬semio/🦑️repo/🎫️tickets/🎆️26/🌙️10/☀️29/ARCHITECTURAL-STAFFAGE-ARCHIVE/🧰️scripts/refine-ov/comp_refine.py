import sys, math; sys.path.insert(0, ".")
from lib import *
doc = load("components")
def up(a, g):
    ids = {s["id"] for s in g.items}
    els = a["geometry"]["elevation"]
    for s in g.items: upsert(els, s)
def hatch(x0, y0, x1, y1, s, ang=45):
    segs = []; d = x0 - y1
    while d <= x1 - y0 + 1e-9:
        xa = max(x0, d + y0); xb = min(x1, d + y1)
        if xb - xa > 1e-6: segs.append(f"M {xa:.4f} {xa-d:.4f} L {xb:.4f} {xb-d:.4f}")
        d += s
    return path(" ".join(segs))
def zone(a, ids): return {s["id"]: s for s in a["geometry"]["elevation"] if s["id"] in ids}

# ---- balcony
a = asset(doc, "cantilever-balcony"); g = G()
S_, C_, R_ = ("slab-soffit", "concrete"), ("wall-face", "concrete"), ("railing-steel", "steel")
g.add("slab-hatch", "slab", "texture", 2, 4.2, hatch(0.0, -0.1, 1.5, 0.1, 0.1), filled=False, tone=0.3)
g.add("wall-hatch", "slab", "texture", 2, 1.2, hatch(-0.3, -0.1, 0.0, 1.2, 0.1), filled=False, tone=0.3)
g.add("rebar-top", "slab", "detail", 2, 4.3, poly([(-0.25, 0.062), (1.46, 0.062)], False), filled=False, tone=0.2)
g.add("rebar-bottom", "slab", "detail", 2, 4.3, poly([(-0.02, -0.062), (0.9, -0.062)], False), filled=False, tone=0.2)
g.add("rebar-wall-hook", "slab", "detail", 2, 4.3, path("M -0.25 0.062 L -0.25 -0.03"), filled=False, tone=0.2)
for k in range(10):
    g.add(f"rebar-dot-{k}", "slab", "detail", 3, 4.4, ell(0.1 + 0.15 * k, 0.062, 0.007, 0.007), tone=0.1)
g.add("slab-drip", "slab", "detail", 3, 4.4, poly([(1.38, -0.1), (1.38, -0.07)], False), filled=False, tone=0.2)
g.add("railing-base-plate", "railing", "body", 1, 5.2, rect(0.095, -0.07, 0.02, 0.14, 0.003), *R_, tone=0.3)
for k, y in enumerate((-0.05, 0.05)):
    g.add(f"railing-bolt-{k}", "railing", "detail", 3, 5.3, ell(0.105, y, 0.006, 0.006), *R_, tone=0.1)
g.add("railing-handrail", "railing", "body", 1, 5.2, ell(1.19, 0, 0.03, 0.03), *R_, tone=0.35)
g.add("railing-mid-rail", "railing", "detail", 2, 5.2, ell(0.6, 0, 0.02, 0.02), *R_, filled=False, tone=0.3)
up(a, g)

# ---- glulam: raise on its bearing plates, lamination lines, finger joints
a = asset(doc, "glulam-beam"); g = G()
for b in a["rig"]["bones"]:
    if b["id"] == "beam": b["y"] = 0.32
LM = ("beam-sides", "glulam")
ls = "".join(f"M 0.0 {-0.3 + 0.04 * k:.3f} L 8.0 {-0.3 + 0.04 * k:.3f} " for k in range(1, 15))
g.add("lamination-line", "beam", "texture", 2, 4, path(ls.strip()), filled=False, tone=0.3)
fj = ""
for k, (x, row) in enumerate(((1.9, 3), (3.7, 7), (5.3, 5), (6.6, 10), (2.8, 12), (4.4, 1))):
    y0 = -0.3 + 0.04 * row
    fj += f"M {x:.2f} {y0:.3f} L {x+0.03:.2f} {y0+0.01:.3f} L {x:.2f} {y0+0.02:.3f} L {x+0.03:.2f} {y0+0.03:.3f} L {x:.2f} {y0+0.04:.3f} "
g.add("finger-joints", "beam", "texture", 3, 4.1, path(fj.strip()), filled=False, tone=0.3)
for nm, x in (("left", 0.0), ("right", 7.88)):
    g.add(f"bearing-plate-{nm}", "beam", "body", 1, 3, rect(x, -0.32, 0.12, 0.02, 0.002), tone=0.3)
    g.add(f"bearing-plate-{nm}-bolt", "beam", "detail", 3, 3.1, ell(x + 0.06, -0.31, 0.006, 0.006), tone=0.1)
g.add("beam-end-cap-left", "beam", "detail", 2, 3.2, poly([(0.0, 0.3), (0.0, -0.3)], False), *LM, filled=False, tone=0.4)
up(a, g)

# ---- steel column HEB 200 seen on the web face: flange edges, fillets, stiffeners, plates
a = asset(doc, "steel-column"); g = G()
CS_, BP_ = ("column-surface", "steel"), ("base-plate", "steel")
for sg, n in ((1, "a"), (-1, "b")):
    g.add(f"flange-edge-{n}", "column", "detail", 1, 3.1, poly([(0.02, sg * 0.085), (2.98, sg * 0.085)], False), filled=False, tone=0.3)
    g.add(f"fillet-line-{n}", "column", "detail", 2, 3.1, poly([(0.15, sg * 0.0675), (2.85, sg * 0.0675)], False), filled=False, tone=0.3)
    g.add(f"stiffener-{n}", "column", "body", 2, 3.5, poly([(0.02, sg * 0.1), (0.02, sg * 0.15), (0.14, sg * 0.1)]), *BP_, tone=0.3)
    g.add(f"anchor-nut-{n}", "column", "detail", 3, 4.5, rect(0.02, sg * 0.125 - 0.012, 0.018, 0.024, 0.002), *BP_, tone=0.1)
    g.add(f"anchor-bolt-{n}", "column", "detail", 3, 4.4, poly([(0.02, sg * 0.125), (0.075, sg * 0.125)], False), *BP_, filled=False, tone=0.1)
g.add("cap-plate", "column", "body", 1, 4, rect(2.98, -0.12, 0.02, 0.24, 0.002), *BP_, tone=0.3)
g.add("web-hatch", "column", "texture", 2, 3.2, hatch(0.4, -0.0675, 0.9, 0.0675, 0.06), filled=False, tone=0.3)
g.add("section-mark", "column", "detail", 2, 3.3, poly([(1.5, -0.14), (1.5, 0.14)], False), filled=False, tone=0.4)
g.add("splice-line", "column", "detail", 2, 3.3, poly([(1.5, -0.1), (1.5, 0.1)], False), *CS_, filled=False, tone=0.3)
up(a, g)
a["summary"] = a["summary"]

# ---- stair: flush-cut stringer, treads with nosing, flange lines, cleats
a = asset(doc, "stair-flight"); g = G()
th = math.radians(38.853374); c, s_ = math.cos(th), math.sin(th); L = 4.62277
def loc(xw, yw): return (xw * c + yw * s_, -xw * s_ + yw * c)
ST_, TT_ = ("stringer-steel", "steel"), ("tread-top", "timber")
k_ = 0.08 * c / s_
t_ = s_ / c; c0 = -0.143; hv = 0.08 / c
def wl(v, y0, y1): return [loc((y0 - c0 - v) / t_, y0), loc((y1 - c0 - v) / t_, y1)]
g.add("stringer-body", "stringer", "body", 0, 6, poly([loc((0 - c0 - hv) / t_, 0), loc((0 - c0 + hv) / t_, 0), loc((2.86 - c0 + hv) / t_, 2.86), loc((2.86 - c0 - hv) / t_, 2.86)]), *ST_, tone=0.45)
g.add("stringer-web-line", "stringer", "detail", 1, 6.1, poly(wl(0, 0.2, 2.7), False), filled=False, tone=0.3)
for sg, n in ((1, "a"), (-1, "b")):
    g.add(f"stringer-flange-{n}", "stringer", "detail", 2, 6.1, poly(wl(sg * 0.0675 / c, 0.2 if sg > 0 else 0.35, 2.7), False), filled=False, tone=0.3)
for i in range(12):
    xw = 0.3 * i; yw = 0.241766 * (i + 1)
    pts = [loc(xw - 0.03, yw), loc(xw + 0.3, yw), loc(xw + 0.3, yw - 0.04), loc(xw - 0.03, yw - 0.04)]
    g.add(f"tread-{i}", "stringer", "body", 1, 4.5, poly(pts), *TT_, tone=0.7)
    g.add(f"tread-{i}-grain", "stringer", "texture", 3, 4.6, poly([loc(xw, yw - 0.02), loc(xw + 0.27, yw - 0.02)], False), *TT_, filled=False, tone=0.3)
    ax, ay = loc(xw + 0.015, yw - 0.04)
up(a, g)
for sid in ("tread-profile",):
    for sh in a["geometry"]["elevation"]:
        if sh["id"] == sid: sh["lod"] = 1

# ---- sliding door: carriage, rollers, mullions, handle
a = asset(doc, "sliding-door"); g = G()
LF, LG, TS = ("leaf-frame", "steel"), ("leaf-glass", "glass"), ("track-steel", "steel")
g.add("carriage-body", "leaf", "body", 1, 6, rect(-0.06, -0.18, 0.06, 0.36, 0.008), *LF, tone=0.3)
for k, y in enumerate((-0.12, 0.12)):
    g.add(f"carriage-wheel-{k}", "leaf", "detail", 2, 6.1, ell(-0.03, y, 0.025, 0.025), *LF, tone=0.15)
g.add("track-lip", "track", "detail", 1, 2.5, rect(0.0, -0.075, 3.0, 0.025), *TS, tone=0.3)
g.add("track-hatch", "track", "texture", 2, 2.6, hatch(0.0, -0.05, 3.0, 0.05, 0.1), filled=False, tone=0.3)
g.add("leaf-bead", "leaf", "detail", 2, 5.1, rect(0.05, -0.45, 1.9, 0.9, 0.0), *LF, filled=False, tone=0.3)
g.add("leaf-mullion", "leaf", "detail", 1, 5.2, poly([(1.0, -0.45), (1.0, 0.45)], False), *LF, filled=False, tone=0.3)
g.add("leaf-reflection", "leaf", "texture", 3, 5.3, poly([(0.2, -0.3), (0.7, -0.1)], False), *LG, filled=False, tone=0.3)
g.add("leaf-bottom-seal", "leaf", "detail", 2, 5.1, rect(1.95, -0.5, 0.05, 1.0), *LF, filled=False, tone=0.3)
g.add("jamb-west-hatch", "track", "texture", 2, 1.2, hatch(-0.05, -2.1, 0.05, -0.1, 0.1), filled=False, tone=0.3)
g.add("jamb-east-hatch", "track", "texture", 2, 1.2, hatch(2.95, -2.1, 3.05, -0.1, 0.1), filled=False, tone=0.3)
up(a, g)

# ---- tilt window: lift the sill so it stands on the ground; frame lines; double glazing
a = asset(doc, "tilt-window"); g = G()
for b in a["rig"]["bones"]:
    if b["id"] == "sill": b["y"] = 0.05
FT, PG = ("frame-timber", "timber"), ("pane-glass", "glass")
g.add("sash-glazing-bead", "sash", "detail", 1, 4.2, rect(0.04, -0.03, 1.12, 0.06, 0.0), *FT, filled=False, tone=0.3)
g.add("sash-pane-inner", "sash", "glazing", 2, 4.1, rect(0.05, -0.0, 1.1, 0.004), *PG, filled=False, tone=0.2)
g.add("sash-spacer-top", "sash", "detail", 3, 4.3, rect(1.14, -0.012, 0.01, 0.024), *FT, tone=0.2)
g.add("sash-spacer-bottom", "sash", "detail", 3, 4.3, rect(0.05, -0.012, 0.01, 0.024), *FT, tone=0.2)
g.add("sash-handle", "sash", "body", 2, 5.5, rect(0.62, 0.045, 0.1, 0.03, 0.004), *FT, tone=0.25)
g.add("sash-seal", "sash", "detail", 2, 4.4, poly([(0.0, 0.040), (1.2, 0.040)], False), *FT, filled=False, tone=0.3)
g.add("sash-grain", "sash", "texture", 2, 4.5, path("M 0.1 0.03 C 0.4 0.034 0.7 0.026 1.1 0.03"), *FT, filled=False, tone=0.3)
g.add("sill-grain", "sill", "texture", 2, 1.5, path("M 0.02 0.01 C 0.1 0.02 0.2 0.0 0.28 0.01"), *FT, filled=False, tone=0.3)
g.add("sill-drip", "sill", "detail", 2, 1.5, poly([(0.28, -0.05), (0.28, -0.02)], False), *FT, filled=False, tone=0.3)
up(a, g)

# ---- balustrade: handrail tube on the glass, laminate lines, clamp bolts
a = asset(doc, "glass-balustrade"); g = G()
PGl, CL, HR = ("panel-glass", "glass"), ("clamp-steel", "steel"), ("handrail-steel", "steel")
g.add("glass-panel", "panel", "glazing", 0, 3, rect(0.0, -0.01075, 1.08, 0.0215), *PGl, tone=0.8)
g.add("handrail", "panel", "body", 0, 5, ell(1.079, 0, 0.0212, 0.0212), *HR, tone=0.35)
g.add("handrail-cap", "panel", "detail", 2, 5.1, path("M 1.06 -0.02 C 1.07 -0.012 1.07 0.012 1.06 0.02"), *HR, filled=False, tone=0.2)
g.add("laminate-line-a", "panel", "detail", 2, 3.2, poly([(0.1, -0.00425), (1.06, -0.00425)], False), *PGl, filled=False, tone=0.3)
g.add("laminate-line-b", "panel", "detail", 2, 3.2, poly([(0.1, 0.00425), (1.06, 0.00425)], False), *PGl, filled=False, tone=0.3)
for k, y in enumerate((-0.11, 0.11)):
    g.add(f"clamp-bolt-{k}", "panel", "detail", 3, 4.5, ell(0.05, y, 0.007, 0.007), *CL, tone=0.1)
g.add("clamp-gasket", "panel", "detail", 2, 4.4, rect(0.0, -0.02, 0.1, 0.04, 0.0), *CL, filled=False, tone=0.3)
g.add("clamp-hatch", "panel", "texture", 2, 4.5, hatch(0.0, -0.15, 0.1, -0.025, 0.04), filled=False, tone=0.3)
g.add("clamp-hatch-b", "panel", "texture", 2, 4.5, hatch(0.0, 0.025, 0.1, 0.15, 0.04), filled=False, tone=0.3)
up(a, g)

# ---- tied canopy: bilateral tube strut (roller along the rod), connection plates, wall hatch
a = asset(doc, "tied-canopy"); g = G()
CBs, RS, WF = ("canopy-steel", "steel"), ("rod-steel", "steel"), ("wall-face", "concrete")
ang = math.degrees(math.atan2(2.88, -4.0))
for b in a['rig']['bones']:
    if b['id'] == 'canopy': b['y'] = 0.12
for an in a["anchors"]:
    if an["id"] == "tie-rod":
        an.update(kind="support", support="roller", normalDeg=round(ang, 6), title=T("Tube strut connection (bilateral, along the rod)", "Anschluss des Rohrstabs (beidseitig, entlang des Stabs)"))
        an.pop("frictionCoefficient", None)
a["structure"]["supports"] = ["wall-pin", "tie-rod"]
a["structure"]["assumptions"] = [T("Canopy pinned at the wall; the tie is a circular hollow section strut hinged at both ends and modelled as a bilateral roller acting along the rod direction (144.04 deg from the beam point to the wall above), so it carries tension under snow and compression under net wind uplift.", "Vordach am Wandgelenk; der Rohrstab ist ein beidseitig gelenkig angeschlossenes Kreishohlprofil und als beidseitige Rolle in Stabrichtung (144,04 Grad vom Trägerpunkt zur Wand darüber) modelliert; er trägt Zug unter Schnee und Druck unter Netto-Windsog."),
  T("Buckling of the strut under uplift compression is not checked; its axial stiffness is not measured.", "Knicken des Stabs unter Druck bei Windsog ist nicht nachgewiesen; seine Dehnsteifigkeit ist nicht gemessen.")]
a["provenance"]["notes"] = T("The tie is a tube strut hinged at both ends: a bilateral roller along the rod, so uplift is carried in compression. Beam spacing 1.0 m sets the line loads. The wall stub is drawn for context.", "Der Zugstab ist ein beidseitig gelenkig angeschlossenes Rohr: eine beidseitige Rolle entlang des Stabs, sodass Windsog auf Druck abgetragen wird. Trägerabstand 1,0 m legt die Linienlasten fest. Das Wandstück dient der Darstellung.")
a["summary"] = T("A 5.5 m steel canopy beam pinned at the wall and held by a tube strut hinged at both ends rising to the wall above; it carries snow in tension and wind uplift in compression.", "Ein 5,5 m langer Stahlträger des Vordachs, an der Wand gelenkig gelagert und durch einen beidseitig gelenkig angeschlossenen Rohrstab zur Wand darüber gehalten; er trägt Schnee auf Zug und Windsog auf Druck.")
for p in a["poses"]:
    p["contacts"] = []
    if p["id"] == "self-weight": p["description"] = T("Canopy under its own weight, tube strut in tension.", "Vordach unter Eigengewicht, Rohrstab auf Zug.")
    if p["id"] == "wind-uplift": p["description"] = T("Wind uplift of 0.5 kN/m2 over 1.0 m spacing; the tube strut carries the net uplift in compression.", "Windsog von 0,5 kN/m² bei 1,0 m Abstand; der Rohrstab trägt den Netto-Sog auf Druck.")
for cl in a["clips"]:
    cl["contacts"] = []
    cl["events"] = [e for e in cl["events"] if e["kind"] not in ("contact-on", "contact-off")]
    if cl["id"] == "loading-ramp": cl["description"] = T("Snow rises smoothly from zero to its full value; the tube strut is in tension.", "Schnee steigt gleichmäßig von null auf den vollen Wert; der Rohrstab ist auf Zug beansprucht.")
    if cl["id"] == "wind-gust": cl["description"] = T("An uplift gust rises and falls; the strut changes from tension to compression when net uplift exceeds the self-weight.", "Eine Sogböe steigt und fällt; der Stab wechselt von Zug zu Druck, sobald der Netto-Sog das Eigengewicht übersteigt.")
    if cl["id"] == "wind-gust":
        cl["events"].append({"t": 2.0, "kind": "motion", "title": T("Strut switches to compression", "Stab wechselt auf Druck")})
        cl["events"].sort(key=lambda e: e["t"])
g.add("strut-tube", "canopy", "body", 0, 5, cap(4.0, 0, 0.0, 2.88, 0.03, 0.03), *RS, tone=0.45)
g.add("strut-axis", "canopy", "detail", 2, 5.1, poly([(3.9, 0.072), (0.1, 2.808)], False), *RS, filled=False, tone=0.3)
g.add("strut-clevis-beam", "canopy", "body", 1, 5.2, rect(3.94, -0.12, 0.12, 0.24, 0.01), *RS, tone=0.3)
g.add("strut-clevis-wall", "canopy", "body", 1, 5.2, rect(-0.04, 2.76, 0.08, 0.24, 0.01), *RS, tone=0.3)
g.add("strut-pin-beam", "canopy", "detail", 3, 5.3, ell(4.0, 0, 0.012, 0.012), *RS, tone=0.1)
g.add("strut-pin-wall", "canopy", "detail", 3, 5.3, ell(0.0, 2.88, 0.012, 0.012), *RS, tone=0.1)
g.add("beam-flange-bottom", "canopy", "detail", 1, 4, poly([(0.0, -0.065), (5.5, -0.065)], False), *CBs, filled=False, tone=0.3)
g.add("beam-glass", "canopy", "body", 1, 3.5, rect(0.1, 0.08, 5.3, 0.02, 0.0), tone=0.8)
g.add("beam-wall-plate", "canopy", "body", 1, 3.6, rect(-0.02, -0.12, 0.02, 0.24, 0.0), *CBs, tone=0.3)
g.add("wall-hatch", "canopy", "texture", 2, 1.2, hatch(-0.5, -0.12, 0.0, 3.02, 0.15), filled=False, tone=0.3)
a["geometry"]["elevation"] = [s for s in a["geometry"]["elevation"] if s["id"] != "tie-rod-line"]
for sh in a["geometry"]["elevation"]:
    if sh["id"] == "wall-stub": sh.update(x=-0.5, y=-0.12, w=0.5, h=3.14)
a["dimensions"]["heightM"] = 3.1
up(a, g)
save("components", doc)
