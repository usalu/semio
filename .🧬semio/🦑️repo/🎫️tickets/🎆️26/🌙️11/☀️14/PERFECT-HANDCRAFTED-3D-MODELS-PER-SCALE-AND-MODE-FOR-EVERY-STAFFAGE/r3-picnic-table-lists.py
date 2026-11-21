"""🌡 Round 3, furniture/picnic-table: body, thermal and structural list of every scale and the bilingual decisions, from the parts of the kit.

Thermal: every body surface is the surface of its region (top planks, bench planks with their batten, the timber of the A-frames with cleats, stretchers and braces, the galvanised steel of the brackets), one layer each at 1:50. Structure: the load path only: planks as beams on the cleats, cleats and stretchers lapped on the raked legs, braces from the stretchers to the centre cleat, bench planks on their gusset brackets; the bolts that clamp the laps, the screws of the planks and the pivot bolts as joints; a pad under each foot; one mass per bone (the frame's in each A-frame).
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_picnic_table_kit", os.path.join(HERE, "r3-picnic-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)


def body(scale):
    """🧱 The body list of a scale."""
    return [K.solid(p) for p in K.PARTS[scale]()]


def thermal(scale):
    """🌡 The thermal list of a scale: a reference per body solid."""
    out = []
    for p in K.PARTS[scale]():
        region, layer = K.REGION[p["group"]]
        entry = {"id": "th-" + p["id"], "of": p["id"], "region": region}
        if scale == "50":
            entry["layer"] = layer
        out.append(entry)
    return out


def own(p, member, part, **more):
    """🧩 An own structural solid from a part."""
    out = K.solid(p)
    out.pop("texture", None)
    out.update({"member": member, "part": part}, **more)
    return out


def pin(sid, bone, origin, axis, radius, length, member, **more):
    """🔩 A bolt or screw as a joint: a shank of `radius` from `origin` along a world axis."""
    p = K.rev(sid, bone, origin, axis, [[0.0, radius], [length, radius]], 0.25, (None, None, None), **more)
    return own(p, member, "joint")


def joints(scale):
    """🔩 The joints of a scale: pivot bolts of the benches, lap bolts of the A-frames, screws of the top planks into the cleats."""
    f = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    out = []
    z0, z1 = K.GUSSET_Z[0] - 0.008, K.BEAM_Z[1] + 0.016
    r_pivot = max(0.006, f / 2)
    for side, sx in (("r", 1), ("l", -1)):
        for end, sz in (("a", 1), ("b", -1)):
            out.append(pin(f"pivot-{side}-{end}", "frame", (sx * K.PIVOT[0], K.PIVOT[1], sz * z0), (0, 0, sz), r_pivot, max(z1 - z0, 1.6 * f), "bench-" + side))
    if scale in ("50", "100"):
        r = max(0.005, f / 2)
        zs, length = K.LEG_Z[0] - 0.013, K.BEAM_Z[1] - K.LEG_Z[0] + 0.019
        step = (-0.045 * math.cos(K.RAKE), 0.045 * math.sin(K.RAKE), 0.0)
        for side, sx in (("r", 1), ("l", -1)):
            for end, sz in (("a", 1), ("b", -1)):
                rep = {"repeat": (2, (sx * step[0], step[1], 0.0))} if scale == "50" else {}
                y = 0.64 if scale == "50" else 0.66
                out.append(pin(f"lap-cleat-{side}-{end}", "frame", (sx * K.leg_x(y), y, sz * zs), (0, 0, sz), r, length, "frame", **rep))
                if scale == "50":
                    out.append(pin(f"lap-stretcher-{side}-{end}", "frame", (sx * K.leg_x(0.405), 0.405, sz * zs), (0, 0, sz), r, length, "frame"))
        rs = max(0.004, f / 2)
        pitch = K.PLANK_W + K.PLANK_GAP
        x0 = -K.TOP_HALF + K.PLANK_W / 2
        for name, z in (("a", (K.BEAM_Z[0] + K.BEAM_Z[1]) / 2), ("m", 0.0), ("b", -(K.BEAM_Z[0] + K.BEAM_Z[1]) / 2)):
            out.append(pin(f"screw-top-{name}", "top", (x0, K.TOP_Y1 - 0.003, z), (0, -1, 0), rs, 0.085, "top", repeat=(K.PLANK_N, (pitch, 0, 0))))
    else:
        r = f / 2
        for end, sz in (("a", 1), ("b", -1)):
            p = K.part(f"seat-top-{end}", "top", "pz", K.PINE, 0.25, "frame", (None, None, None), profile=K.rect(-r, K.TOP_Y0 - r, r, K.TOP_Y0 + r), span=tuple(sorted((sz * (K.BEAM_Z[1] - 2 * r), sz * K.BEAM_Z[1]))), role="detail")
            out.append(own(p, "top", "joint"))
    return out


ANCHORS = (("foot-left-l", -1, 1), ("foot-left-r", -1, -1), ("foot-right-l", 1, 1), ("foot-right-r", 1, -1))


def supports(scale):
    """🟢 The contact pad under each foot, one per support anchor, as high as the scale's smallest feature."""
    f = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    z = K.LEG_Z if scale != "500" else (K.LEG_Z[0], K.BEAM_Z[1])
    out = []
    for anchor, sx, sz in ANCHORS:
        x0, x1 = sorted((sx * K.FOOT[0], sx * K.FOOT[1]))
        p = K.pz("support-" + anchor, "frame", K.rect(x0, 0.0, x1, f), tuple(sorted((sz * z[0], sz * z[1]))), K.PINE, 0.2, "frame", (None, None, None), role="ground")
        out.append(own(p, "frame", "support", anchor=anchor))
    return out


def lumps(scale):
    """⚫ The mass of every bone as a ball at its centre of mass; the frame's mass is shown in each A-frame, in the stretcher under the frame's centre."""
    r = {"50": 0.018, "100": 0.02, "200": 0.02, "500": 0.0375}[scale]
    m = K.masses()
    spots = [("mass-top", "top", (0.0, K.TOP_Y1 - (0.02 if scale != "500" else 0.0375), 0.0))]
    zf = (K.BEAM_Z[0] + K.BEAM_Z[1]) / 2 if scale != "500" else (K.LEG_Z[0] + K.BEAM_Z[1]) / 2
    spots += [("mass-frame-a", "frame", (0.0, 0.375, zf)), ("mass-frame-b", "frame", (0.0, 0.375, -zf))]
    by = K.PIVOT[1] + (K.BENCH_Y[0] + K.BENCH_Y[1]) / 2 if scale != "500" else K.PIVOT[1] + K.BENCH_Y[1] - 0.0375
    spots += [("mass-bench-r", "bench-r", (m["bench-r"][1], by, 0.0)), ("mass-bench-l", "bench-l", (m["bench-l"][1], by, 0.0))]
    out = []
    for sid, bone, c in spots:
        out.append({"id": sid, "kind": "ellipsoid", "bone": bone, "material": K.PINE, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [K.r6(v) for v in K.local(bone, *c)]}, "member": bone, "part": "mass"})
    return out


def structure(scale):
    """🏗 The structural list of a scale."""
    out = []
    for p in K.PARTS[scale]():
        if p["kind"] == "rev":
            continue
        sid = p["id"]
        if sid.startswith("plate"):
            out.append({"id": "j-" + sid, "of": sid, "member": "bench-" + ("l" if "-l" in sid else "r"), "part": "joint"})
        else:
            out.append({"id": "m-" + sid, "of": sid, "member": p["bone"], "part": "member"})
    return out + joints(scale) + supports(scale) + lumps(scale)


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": [
        T("Body, authored for 7.5 mm: five 132 x 40 mm top planks with 5 mm eased edges and 10 mm drainage gaps on two end cleats and an 89 x 45 mm centre cleat; two A-frames, each of two 95 x 45 mm legs raked 60 degrees (level cuts at ground and top, eased toe) and, lapped on their outer faces, a 95 x 45 mm cleat with bevelled ends and a 95 x 45 mm stretcher; two 70 x 45 mm braces from the stretchers up to the centre cleat; the domed heads and nuts of the M10 carriage bolts in every lap and nut and washer of the M12 pivot bolts. Each bench: two 145 x 40 mm eased planks 1.50 m long with a centre batten, hanging between the frames on two folded 8 mm gusset brackets (web beside the plank ends, flange under the planks) that turn on the pivot bolts; an 8 mm bearing plate on each inner leg face with the open stow hole; pivot bolt heads and spring plungers on the gussets.",
          "Körper, für 7,5 mm gebaut: fünf Tischbohlen 132 x 40 mm mit 5 mm gebrochenen Kanten und 10-mm-Wasserfugen auf zwei Endriegeln und einem Mittelriegel 89 x 45 mm; zwei A-Böcke, je aus zwei um 60 Grad geneigten Beinen 95 x 45 mm (waagerechte Schnitte am Boden und oben, gebrochene Zehe) und, außen aufgeblattet, einem Riegel 95 x 45 mm mit gefasten Enden und einer Zange 95 x 45 mm; zwei Streben 70 x 45 mm von den Zangen hinauf zum Mittelriegel; Rundköpfe und Muttern der M10-Schlossschrauben in jeder Überblattung sowie Mutter und Scheibe der M12-Drehbolzen. Jede Bank: zwei gefaste Bohlen 145 x 40 mm, 1,50 m lang, mit Mittelleiste, zwischen den Böcken an zwei gekanteten 8-mm-Knotenblechkonsolen (Steg neben den Bohlenenden, Flansch unter den Bohlen), die um die Drehbolzen schwenken; eine 8-mm-Lagerplatte auf jeder Beininnenseite mit dem offenen Rastloch der Parkstellung; Drehbolzenköpfe und Federrastbolzen auf den Knotenblechen."),
        T("Thermal: every region is the surface of its body solids with its one layer (40 mm planks, 45 mm frame timber, 8 mm steel); no hidden regions, no emitters. Structure: planks, cleats, legs, stretchers, braces, bench planks, battens and gusset brackets are the members themselves; joints are the pivot bolts through gusset, bearing plate, leg and stretcher, the bearing plates, the lap bolts and one screw per plank and cleat; a pad under each of the four feet; masses of top and benches at their centres, the frame's mass in each stretcher.",
          "Thermik: jede Region ist die Oberfläche ihrer Körper mit ihrer einen Schicht (40-mm-Bohlen, 45-mm-Gestellholz, 8-mm-Stahl); keine verdeckten Regionen, keine Emitter. Tragwerk: Bohlen, Riegel, Beine, Zangen, Streben, Bankbohlen, Leisten und Knotenblechkonsolen sind die Stäbe selbst; Gelenke sind die Drehbolzen durch Knotenblech, Lagerplatte, Bein und Zange, die Lagerplatten, die Blattbolzen und je eine Schraube pro Bohle und Riegel; ein Auflager unter jedem der vier Füße; Massen von Platte und Bänken in ihren Schwerpunkten, die Gestellmasse in jeder Zange."),
    ],
    "100": [
        T("Body, authored for 15 mm: square-edged planks with their gaps, centre cleat, legs without the eased toe, cleats with bevels, plain stretchers, braces; bench planks square with the batten; each bracket as a 15 mm web against the leg that takes up the bearing plate, its flange drawn at the 15 mm floor; bolts, nuts and plungers are left out.",
          "Körper, für 15 mm gebaut: scharfkantige Bohlen mit ihren Fugen, Mittelriegel, Beine ohne gebrochene Zehe, Riegel mit Fasen, glatte Zangen, Streben; Bankbohlen scharfkantig mit Leiste; jede Konsole als 15-mm-Steg am Bein, der die Lagerplatte aufnimmt, ihr Flansch im 15-mm-Mindestmaß; Schrauben, Muttern und Rastbolzen entfallen."),
        T("Thermal: references to the merged body solids, one surface per region. Structure: the same load path with one 15 mm lap bolt per cleat joint, 15 mm pivot bolts and plank screws, 15 mm foot pads.",
          "Thermik: Verweise auf die zusammengefassten Körper, eine Oberfläche je Region. Tragwerk: derselbe Lastpfad mit einem 15-mm-Blattbolzen je Riegelanschluss, 15-mm-Drehbolzen und Bohlenschrauben, 15-mm-Auflagern."),
    ],
    "200": [
        T("Body, authored for 30 mm: the top as one 40 mm slab (gaps merged) on the 89 x 45 mm centre cleat that takes up both braces, each bench as one 300 x 40 mm board, legs, square cleats, stretchers and braces member by member at 45 mm, each bracket as a 30 mm block that fills the gap between board end and leg.",
          "Körper, für 30 mm gebaut: die Platte als eine 40-mm-Scheibe (Fugen aufgegangen) auf dem Mittelriegel 89 x 45 mm, der beide Streben aufnimmt, jede Bank als ein Brett 300 x 40 mm, Beine, kantige Riegel, Zangen und Streben Stab für Stab mit 45 mm, jede Konsole als 30-mm-Block, der die Lücke zwischen Brettende und Bein füllt."),
        T("Thermal: one surface per region on the merged solids. Structure: slab on cleats, cleats and stretchers on legs, braces, boards on bracket blocks; pivot bolts 30 mm, the seat of the slab on each cleat as a joint block, 30 mm foot pads.",
          "Thermik: eine Oberfläche je Region auf den zusammengefassten Körpern. Tragwerk: Scheibe auf Riegeln, Riegel und Zangen an Beinen, Streben, Bretter auf Konsolblöcken; Drehbolzen 30 mm, das Auflager der Scheibe auf jedem Riegel als Gelenkblock, 30-mm-Fußauflager."),
    ],
    "500": [
        T("Body, authored for 75 mm: the top with its cleats as one 75 mm slab, each A-frame as one 90 mm plate of both legs and the stretcher (legs and stretcher keep their 95 mm), each bench as a 75 mm board with its top at seat height, each bracket as a 75 mm block under the board end; braces (45 mm) are left out.",
          "Körper, für 75 mm gebaut: die Platte mit ihren Riegeln als eine 75-mm-Scheibe, jeder A-Bock als eine 90-mm-Scheibe aus beiden Beinen und der Zange (Beine und Zange behalten ihre 95 mm), jede Bank als 75-mm-Brett mit Oberkante auf Sitzhöhe, jede Konsole als 75-mm-Block unter dem Brettende; die Streben (45 mm) entfallen."),
        T("Thermal: one surface per region. Structure: slab on two A-frames, boards on bracket blocks, 75 mm pivot pins, seat blocks of the slab and foot pads; the model is minimal at this scale.",
          "Thermik: eine Oberfläche je Region. Tragwerk: Scheibe auf zwei A-Böcken, Bretter auf Konsolblöcken, 75-mm-Drehbolzen, Auflagerblöcke der Scheibe und Fußauflager; das Modell ist in diesem Maßstab minimal."),
    ],
}


def decisions(scale):
    """📝 The decisions of a scale."""
    return DECISIONS[scale]


if __name__ == "__main__":
    for s in ("50", "100", "200", "500"):
        print(s, len(body(s)), len(thermal(s)), len(structure(s)))
