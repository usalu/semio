import sys, json, importlib, copy, re
sys.path.insert(0, ".")
from lib2 import *
import cfg, texts
from texts import T
MODS = {"english-oak": "d_oak", "lombardy-poplar": "d_poplar", "multi-stem-birch": "d_birch", "scots-pine": "d_pine", "date-palm": "d_palm", "weeping-willow": "d_willow", "hornbeam-hedge": "d_hedge", "miscanthus-grass": "d_grass"}

def circle(cx, cy, r):
    k = 0.5523*r
    return f"M{f(cx+r)} {f(cy)}C{f(cx+r)} {f(cy+k)} {f(cx+k)} {f(cy+r)} {f(cx)} {f(cy+r)}C{f(cx-k)} {f(cy+r)} {f(cx-r)} {f(cy+k)} {f(cx-r)} {f(cy)}C{f(cx-r)} {f(cy-k)} {f(cx-k)} {f(cy-r)} {f(cx)} {f(cy-r)}C{f(cx+k)} {f(cy-r)} {f(cx+r)} {f(cy-k)} {f(cx+r)} {f(cy)}Z"

HATCH = lambda sp: {"id": "shade-lines", "title": T("Shade line hatch", "Schattenschraffur"), "pattern": "lines", "angleDeg": 45, "spacingMm": sp, "penMm": 0.1}

def mk_rep(a, k, shapes):
    used = {s.get("hatch") for s in shapes if s.get("hatch")}
    hat = [HATCH(0.6 if k == "100" else 0.8)] if used else []
    lo, hi = VALID[k]
    return {"status": "authored", "denominator": int(k), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[k], "hatches": hat,
            "geometry": {"elevation": shapes}, "decisions": texts.decisions(a, k)}

def symbols(R, A, a):
    rx, ry, tr = cfg.PLAN[a]; root = A["rig"]["root"]
    sh = []
    if a == "hornbeam-hedge":
        sh.append(shape(R, "outline", root, "body", poly([(-rx, -ry), (rx, -ry), (rx, ry), (-rx, ry)]), filled=False, tone=0.5, z=1))
        sh.append(shape(R, "axis", root, "detail", "M-1.4 0L1.4 0", filled=False, tone=0.3, z=2))
    else:
        n = 20; pts = [((0.93*rx)*math.cos(2*math.pi*i/n), (0.93*ry)*math.sin(2*math.pi*i/n)) for i in range(n)]
        crown = fit_to(lobes(pts, 0.3), (-rx, rx, -ry, ry))
        sh.append(shape(R, "crown-extent", root, "body", crown, filled=False, tone=0.5, z=1))
        if a == "multi-stem-birch":
            for i, x in enumerate((-0.35, 0.0, 0.35)):
                sh.append(shape(R, f"stem-{i+1}", root, "body", circle(x, 0.08 if x == 0 else -0.05, tr), filled=True, tone=0.3, z=2))
        else:
            sh.append(shape(R, "stem", root, "body", circle(0, 0, tr), filled=True, tone=0.3, z=2))
    H, Wd = cfg.DIMS[a]; base = cfg.CROWN_BASE[a]
    el = []
    if a == "hornbeam-hedge":
        el.append(shape(R, "outline", root, "body", poly([(-Wd/2, 0), (Wd/2, 0), (Wd/2, H), (-Wd/2, H)]), filled=False, tone=0.5, z=1))
    else:
        cyy = (H+base)/2; ryy = (H-base)/2
        n = 20; pts = [((0.93*Wd/2)*math.cos(2*math.pi*i/n), cyy + (0.93*ryy)*math.sin(2*math.pi*i/n)) for i in range(n)]
        el.append(shape(R, "crown-extent", root, "body", fit_to(lobes(pts, 0.3), (-Wd/2, Wd/2, base, H)), filled=False, tone=0.5, z=1))
        if base > 0.4: el.append(shape(R, "stem", root, "body", poly([(-tr, 0), (tr, 0), (tr*0.8, base+0.3), (-tr*0.8, base+0.3)]), filled=False, tone=0.3, z=2))
    d = 2*tr*2
    note = T(f"The outline is the true plan extent of the crown in model metres ({2*rx:.2f} m by {2*ry:.2f} m, from the asset dimensions) and the inner mark the true stem section; at 1:500 the stem measures {d:.1f} mm on paper. The symbol carries dimension, not species detail.",
             f"Der Umriss ist die wahre Kronenausdehnung im Grundriss in Modellmetern ({2*rx:.2f} m mal {2*ry:.2f} m, aus den Asset-Maßen) und die Innenmarke der wahre Stammquerschnitt; bei 1:500 misst der Stamm {d:.1f} mm auf dem Papier. Das Symbol trägt Maß, keine Artdetails.")
    if a == "hornbeam-hedge": note = T(f"The rectangle is the true plan footprint of the hedge ({2*rx:.2f} m by {2*ry:.2f} m) with its axis line; the symbol carries dimension, not clipping detail.", f"Das Rechteck ist der wahre Grundrissfußabdruck der Hecke ({2*rx:.2f} m mal {2*ry:.2f} m) mit Achslinie; das Symbol trägt Maß, keine Schnittdetails.")
    return [{"id": "plan-extent", "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
             "description": T("A conventional plan sign of the plant for site and landscape plans: outline of the true crown extent and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen der Pflanze für Lage- und Freiraumpläne: Umriss der wahren Kronenausdehnung und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100],
             "pens": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [],
             "geometry": {"elevation": el, "plan": sh}, "dimensionalNote": note}]

def anchors_for(R, shapes, region, suffix, titleen, titlede):
    byb = {}
    for s in shapes:
        if s["role"] == "body" and (s.get("region") == region or region in (s.get("regions") or [])):
            byb.setdefault(s["bone"], []).append(s)
    pts = []
    for b, ss in byb.items():
        X = []; Y = []
        for s in ss:
            for x, y in shape_pts(R, s): X.append(x); Y.append(y)
        pts.append((b, (min(X)+max(X))/2, (min(Y)+max(Y))/2, (max(X)-min(X))/2, (max(Y)-min(Y))/2))
    if len(pts) > 12: pts = pts[::max(1, len(pts)//12)][:12]
    out = []
    if len(pts) >= 5:
        for b, cx, cy, hw, hh in pts: out.append((b, cx, cy + 0.2*hh))
    else:
        per = -(-5 // len(pts))
        for b, cx, cy, hw, hh in pts:
            for j in range(per):
                u = 0 if per == 1 else -0.55 + 1.1*j/(per-1)
                out.append((b, cx + u*hw, cy + 0.15*hh))
    res = []
    for i, (b, x, y) in enumerate(out):
        lx, ly = R.to_local(b, x, y)
        res.append({"id": f"tp-{suffix}-{i+1}", "bone": b, "x": round(lx, 3), "y": round(ly, 3), "kind": "emitter",
                    "title": T(f"{titleen}, anchor {i+1}", f"{titlede}, Anker {i+1}")})
    return res

def build(a):
    doc = load_orig(a); A = doc["asset"]; R = Rig(A)
    res = importlib.import_module(MODS[a]).design(R, A)
    A.pop("$comment", None)
    shapes50 = []
    for s in A["geometry"]["elevation"]:
        s = {k: v for k, v in s.items() if k != "lod"}
        shapes50.append(s)
    if a == "hornbeam-hedge":
        shapes50 += [s for s in res["100"] if s["id"].startswith("core-")]
    shade = {"leaf-shade", "needles-shade", "leaf-core"}
    for s in shapes50:
        if s["role"] == "body" and s.get("region") in shade: s["hatch"] = "shade-lines"
    A.pop("geometry")
    A["representations"] = {k: mk_rep(a, k, res[k] if k != "50" else shapes50) for k in ("500", "200", "100", "50")}
    A["symbols"] = symbols(R, A, a)
    h, w = cfg.DIMS[a]; A["dimensions"]["heightM"] = h; A["dimensions"]["widthM"] = w
    A["presentation"]["defaultScale"] = cfg.DEFAULT_SCALE[a]
    # thermal regions
    th = A["thermal"]; old_regions = th["regions"]; new = []
    for r in old_regions:
        g, svf, hc, lai, photo, ref, _ = cfg.REG[a][r["id"]]
        n = {"id": r["id"], "title": r["title"], "kind": r["kind"], "model": r["model"], "material": r["material"], "areaM2": r["areaM2"],
             "characteristicLengthM": r["characteristicLengthM"], "geometry": g, "skyViewFactor": svf, "layers": [], "heatCapacityJM2K": hc}
        if r["model"] == "leaf":
            n["leaf"] = {"stomatalConductanceMaxMS": r["stomatalConductanceMS"], "leafAreaIndex": round(r["areaM2"]/cfg.PROJ[a], 2), "photosynthesis": photo}
        n["source"] = "authored-estimate"; n["reference"] = ref
        new.append(n)
    if a == "hornbeam-hedge" and not any(x["id"] == "leaf-core" for x in new): raise SystemExit("core missing")
    th["regions"] = new
    old_em = th["emitters"]
    base_anchor = [x for x in A["anchors"] if x["kind"] != "emitter"]
    ems = []; anc = []
    for suf, reg, dia, ten, tde in cfg.EMIT[a]:
        ans = anchors_for(R, shapes50, reg, suf, ten, tde)
        anc += ans
        ems.append({"id": f"transpiration-{suf}", "kind": "transpiration", "title": T(f"{ten} (distributed over the crown)", f"{tde} (über die Krone verteilt)"),
                    "anchor": ans[0]["id"], "anchors": [x["id"] for x in ans], "directionDeg": round(90 - R.angle(ans[0]["bone"]), 1), "diameterM": dia, "region": reg,
                    "source": "authored-estimate", "reference": "monteith-unsworth-2013"})
    A["anchors"] = base_anchor + anc
    th["emitters"] = ems; th["operating"] = []
    th["environment"]["diffuseFraction"] = 0.1 if a == "date-palm" else 0.15
    gmax = {r["id"]: r["leaf"]["stomatalConductanceMaxMS"] for r in new if "leaf" in r}
    th["assumptions"] = [
        T("Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection. The maximum stomatal conductance is an authored estimate for the species type, guided by Jones (2014); an earlier citation of published conductance maxima could not be verified and was removed. Photosynthesis type: " + ("C4 (Miscanthus)." if a == "miscanthus-grass" else "C3.") + " Aperture and night closure are the engine's business, not authored here.",
          "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für den Artentyp, angelehnt an Jones (2014); eine frühere Angabe veröffentlichter Leitfähigkeitsmaxima ließ sich nicht prüfen und wurde entfernt. Photosynthesetyp: " + ("C4 (Miscanthus)." if a == "miscanthus-grass" else "C3.") + " Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
        T("Bark, stems and similar regions are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1 to 3 cm (a quarter of the stem diameter for thin members), an authored estimate guided by the Wood Handbook. No surface temperature is authored.",
          "Rinde, Stämme und ähnliche Regionen sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1 bis 3 cm (ein Viertel des Durchmessers bei dünnen Gliedern), eine Autorenschätzung angelehnt an das Wood Handbook. Es werden keine Oberflächentemperaturen vorgegeben."),
        T("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the crown; the direction is the vertical measured in the frame of the first anchor's bone, since the release is diffuse. Sun and shade foliage are separate regions where the crown has an inner shaded mass; otherwise the sunlit fraction is a crown mean authored per pose.",
          "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker über die Krone; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist. Sonnen- und Schattenlaub sind getrennte Regionen, wo die Krone eine innere Schattenmasse hat; sonst ist der besonnte Anteil ein je Pose vorgegebener Kronenmittelwert.")]
    A["provenance"]["references"] = [r for r in A["provenance"]["references"] if r["id"] != "koerner-1979"]
    # poses
    em_v1 = None
    for p in A["poses"]:
        old = p["thermal"]["emitters"]; v = list(old.values())[0] if old else 0
        idx = {"calm": 0, "windy": 1, "storm": 2}.get(p["id"], 3)
        regs = {}
        for r in new:
            sun = cfg.REG[a][r["id"]][6][idx]
            b = {"exposedFraction": 1.0, "sunlitFraction": sun}
            wet = cfg.WET["leaf" if r["model"] == "leaf" else "bark"][idx]
            if wet: b["wettedness"] = wet
            regs[r["id"]] = b
        p["thermal"] = {"regions": regs, "emitters": {e["id"]: v for e in ems}}
    # clips
    for c in A["clips"]:
        old = c["tracks"]
        if c["id"] == "wind-sway":
            ks = old[0]["keys"]
            c["tracks"] = [{"target": f"emitter:{e['id']}", "ease": "smooth", "keys": ks} for e in ems]
        elif c["id"] == "diurnal-transpiration":
            etr = [t for t in old if t["target"].startswith("emitter:")]
            curve = [t for t in old if t["target"].endswith(":exposedFraction")][0]["keys"]
            tracks = [{"target": f"emitter:{e['id']}", "ease": "smooth", "keys": etr[i % len(etr)]["keys"]} for i, e in enumerate(ems)]
            for r in new:
                base = cfg.REG[a][r["id"]][6][0]
                tracks.append({"target": f"region:{r['id']}:sunlitFraction", "ease": "smooth", "keys": [{"t": k["t"], "v": round(base*max(0.0, (k["v"]-0.3)/0.7), 3)} for k in curve]})
            c["tracks"] = tracks
            c["description"] = T("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; leaf and bark temperatures are not authored but follow from the thermal solver.",
                                  "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem Thermiklöser.".replace("Thermiklöser", "thermischen Löser"))
    # checks
    txt = json.dumps(A, ensure_ascii=False)
    rg = {r["id"] for r in new}
    for k in ("500", "200", "100", "50"):
        shp = A["representations"][k]["geometry"]["elevation"]
        got = set()
        for s in shp:
            if s.get("region"): got.add(s["region"])
            got |= set(s.get("regions") or [])
        ids = [s["id"] for s in shp]
        bb = bbox_shapes(R, shp)
        ok = lambda v, n: abs(v-n) <= max(0.02*n, 0.015)
        print(a, k, "missing", sorted(rg-got), "dups", len(ids)-len(set(ids)), "h", round(bb[3]-bb[2], 2), "w", round(bb[1]-bb[0], 2), "dimsOK", ok(bb[3]-bb[2], h) and ok(bb[1]-bb[0], w), "shapes", len(shp))
    for m in re.finditer(r"[Kk](oe|ö)rner", txt): print("KOERNER", a, txt[max(0, m.start()-60):m.start()+60])
    json.dump({"$schema": doc["$schema"], "version": doc["version"], "asset": A}, open(f"{CAT}/{a}/🔣️.json", "w"), ensure_ascii=False, indent=2)
    open(f"{CAT}/{a}/🔣️.json", "a").write("\n")

if __name__ == "__main__":
    for a in (sys.argv[1:] or MODS): build(a)
