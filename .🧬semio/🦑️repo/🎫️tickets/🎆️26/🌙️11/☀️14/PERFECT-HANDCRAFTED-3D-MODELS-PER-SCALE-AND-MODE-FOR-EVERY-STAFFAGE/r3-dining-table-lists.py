"""🌡 Round 3, furniture/dining-table: the thermal and the structural list of every scale and the bilingual decisions.

Thermal: every body surface is the surface of its region (top halves, leaf, aprons with their blocks, legs, steel slides), one layer each; at 1:500 the slides, absent from the body, are a hidden bar of their own. Structure: the load path only: the top plates as the bending members of each half and of the leaf, the aprons as the frame that ties the legs, the legs as the columns, the runners and centre bars of the slide; tenons, blocks, slide shoes and dowels as the joints; felt glides as the supports; one mass per bone at its centre.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_dining_table_" + name, os.path.join(HERE, f"r3-dining-table-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")

REGIONS = (("top-", "table-top", "top-plank"), ("slab-", "table-top", "top-plank"), ("leaf", "leaf-top", "leaf-plank"), ("apron-", "apron", "apron-board"), ("block-", "apron", "apron-board"), ("frame-", "apron", "apron-board"), ("leg-", "legs", "leg-solid"), ("runner-", "slide-rails", "rail-steel"), ("centre-bar", "slide-rails", "rail-steel"), ("slide", "slide-rails", "rail-steel"))
ANCHORS = (("leg-left-l", "top-left", 1, 1), ("leg-left-r", "top-left", 1, -1), ("leg-right-l", "top-right", -1, 1), ("leg-right-r", "top-right", -1, -1))


def region(sid):
    """🏷 The thermal region and layer of a body solid."""
    for prefix, name, layer in REGIONS:
        if sid.startswith(prefix):
            return name, layer
    raise KeyError(sid)


def thermal(scale, body):
    """🌡 The thermal list of a scale."""
    out = []
    for solid in body:
        name, layer = region(solid["id"])
        entry = {"id": "th-" + solid["id"], "of": solid["id"], "region": name}
        if scale == "50":
            entry["layer"] = layer
        out.append(entry)
    if scale == "500":
        bar = K.box("th-slide", "frame", K.STEEL, (K.CENTRE_X0, K.CENTRE_X1), (K.FRAME500_Y0, K.SLAB_Y0), (0.25, 0.325))
        out.append(dict(K.repeat(bar, 2, dw=-0.575), region="slide-rails"))
    return out


def of(solid, member, part="member"):
    """🔗 A structural reference to a body solid that is the member itself."""
    return {"id": ("m-" if part == "member" else "j-") + solid, "of": solid, "member": member, "part": part}


def own(solid, member, part):
    """🧩 An own structural solid."""
    solid.update({"member": member, "part": part})
    return solid


def tenons(prefix, bone, hand, member, size):
    """🪚 The mortise-and-tenon joints of the aprons in the legs of a half (long aprons both sides, end apron both ends)."""
    shoulder = K.LEG_X + K.LEG_TOP
    y = (K.APRON_Y0 + 0.01, K.APRON_Y1 - 0.01)
    side = K.box(prefix + "-side", bone, K.OAK, K.span(shoulder - size, shoulder, hand), y, (K.APRON_Z1 - K.APRON_T, K.APRON_Z1), tone=0.3, role="detail")
    K.repeat(side, 2, dw=-(2 * K.APRON_Z1 - K.APRON_T))
    reach = K.LEG_Z - K.LEG_TOP
    end = K.box(prefix + "-end", bone, K.OAK, K.span(K.END_X0, K.END_X0 + K.APRON_T, hand), y, (reach, reach + size), tone=0.3, role="detail")
    K.repeat(end, 2, dw=-(2 * reach + size))
    return [own(side, member, "joint"), own(end, member, "joint")]


def joint_dowels(radius, length):
    """📍 The alignment dowels in the joint edges: the left half carries three (z 0, +-0.30 m), the right half two (z +-0.15 m); closed they enter the other half, extended the holes in the two ends of the leaf."""
    out = []
    for name, bone, start, count in (("left", "top-left", 0.3, 3), ("right", "top-right", 0.15, 2)):
        d = K.turned("dowel-" + name, bone, K.OAK, [(0, 0), (0, radius), (length, radius), (length, 0)], "u", (K.HALF - length / 2, K.CREASE_Y, start), tone=0.3, role="detail")
        out.append(own(K.repeat(d, count, dw=-0.3), bone, "joint"))
    return out


def glides(radius, height):
    """🟢 One felt glide under each leg as the support of its anchor."""
    out = []
    for anchor, bone, hand, side in ANCHORS:
        g = K.turned("support-" + anchor, bone, K.OAK, [(0, 0), (0, radius), (height, radius), (height, 0)], "v", (K.wx(K.LEG_X, hand), 0.0, side * K.LEG_Z), tone=0.2, role="ground")
        out.append(own(g, bone, "support") | {"anchor": anchor})
    return out


def lumps(r):
    """⚫ The mass of every moving bone at its centre: each half, the leaf, the slide frame."""
    m = K.masses()
    kg, cx, cy = m["half"]
    out = []
    for name, bone, hand in K.HALVES:
        out.append(own(K.ellipsoid("mass-" + name, bone, K.OAK, (K.wx(cx, hand), cy, 0.0), (r, r, r), tone=0.3, role="detail"), bone, "mass"))
    out.append(own(K.ellipsoid("mass-leaf", "leaf", K.OAK, (0.8, K.GAP_Y0 + K.LEAF_T / 2, 0.0), (max(r, 0.04), min(r, 0.012) if r < 0.03 else r, max(r, 0.04)), tone=0.3, role="detail"), "leaf", "mass"))
    out.append(own(K.ellipsoid("mass-frame", "frame", K.STEEL, (0.8, K.FRAME_Y, 0.0), (r, r, r), tone=0.3, role="detail"), "frame", "mass"))
    return out


def structure(scale, body):
    """🏗 The structural list of a scale."""
    out = []
    for solid in body:
        sid = solid["id"]
        if sid.startswith(("top-", "apron-", "leg-", "runner-", "slab-", "frame-")):
            bone = solid["bone"]
            out.append(of(sid, bone))
        elif sid.startswith("block-"):
            out.append(of(sid, solid["bone"], "joint"))
        elif sid.startswith("leaf"):
            out.append(of(sid, "leaf"))
        elif sid.startswith(("centre-bar", "slide")):
            out.append(of(sid, "frame"))
    if scale == "50":
        for name, bone, hand in K.HALVES:
            out += tenons("tenon-" + name, bone, hand, bone, K.TENON)
            cage = K.box("shoe-" + name, bone, K.STEEL, K.span(0.69, K.RUN_X1, hand), (0.667, 0.688), (K.CENTRE_Z0, K.CENTRE_Z1), tone=0.25, role="detail")
            out.append(own(K.repeat(cage, 2, dw=-(K.CENTRE_Z0 + K.CENTRE_Z1)), bone, "joint"))
        out += joint_dowels(0.005, 0.03)
        out += glides(0.018, 0.0075)
        out += lumps(0.02)
    elif scale == "100":
        for name, bone, hand in K.HALVES:
            out += tenons("tenon-" + name, bone, hand, bone, 0.02)
            link = K.box("slide-link-" + name, bone, K.STEEL, K.span(0.65, 0.75, hand), (0.667, 0.687), (0.357, 0.375), tone=0.25, role="detail")
            out.append(own(K.repeat(link, 2, dw=-0.732), bone, "joint"))
        out += joint_dowels(0.0075, 0.04)
        out += glides(0.0225, 0.015)
        out += lumps(0.025)
    elif scale == "200":
        for name, bone, hand in K.HALVES:
            corner = K.box("corner-" + name, bone, K.OAK, K.span(K.LEG_X - 0.03, K.LEG_X + 0.03, hand), (K.APRON_Y0, K.APRON_Y1), (K.LEG_Z - 0.03, K.LEG_Z + 0.03), tone=0.3, role="detail")
            out.append(own(K.repeat(corner, 2, dw=-2 * K.LEG_Z), bone, "joint"))
            link = K.box("slide-link-" + name, bone, K.STEEL, K.span(0.6, 0.75, hand), (0.66, K.GAP_Y0), (0.34, 0.37), tone=0.25, role="detail")
            out.append(own(K.repeat(link, 2, dw=-0.71), bone, "joint"))
        out += glides(0.03, 0.03)
        out += lumps(0.03)
    else:
        bar = K.box("member-slide", "frame", K.STEEL, (K.CENTRE_X0, K.CENTRE_X1), (K.FRAME500_Y0, K.SLAB_Y0), (0.25, 0.325))
        out.append(own(K.repeat(bar, 2, dw=-0.575), "frame", "member"))
        out += glides(0.0375, 0.075)
        out += lumps(0.0375)
    return out


DECISIONS = {
    "50": [
        ("Body, authored for 7.5 mm: two 0.80 x 0.90 m solid oak top halves, 30 mm thick with a 15 mm underside bevel on the three free edges and a square joint edge; a 30 mm shadow gap under the top in which the 0.60 x 0.90 m leaf (same bevel on its long edges) is stored and in which four oak blocks per half join top and aprons; 25 x 80 mm aprons, the long ones running to the joint, tenoned 20 mm into the legs; four legs, 70 mm square under the aprons and tapered on all four faces to 45 mm at the floor; per long apron a 25 x 12 mm steel tube of 2 mm sheet, and in the tubes the two stationary 21 x 8 mm centre bars of the telescopic slide (tube-in-tube). Screws, felt glides, table locks and dowels are omitted from the body (below 7.5 mm or hidden).",
         "Körper, für 7,5 mm gebaut: zwei Plattenhälften 0,80 x 0,90 m aus massiver Eiche, 30 mm stark, mit 15 mm Unterfase an den drei freien Kanten und gerader Stoßkante; eine 30-mm-Schattenfuge unter der Platte, in der die Einlegeplatte 0,60 x 0,90 m (gleiche Fase an den Längskanten) liegt und in der je Hälfte vier Eichenklötze Platte und Zargen verbinden; Zargen 25 x 80 mm, die langen bis zum Stoß, 20 mm in die Beine gezapft; vier Beine, unter der Zarge 70 mm im Quadrat und allseitig auf 45 mm am Boden verjüngt; je Längszarge ein Stahlrohr 25 x 12 mm aus 2 mm Blech und in den Rohren die zwei feststehenden Mittelstäbe 21 x 8 mm des Teleskopauszugs (Rohr in Rohr). Schrauben, Filzgleiter, Tischverbinder und Dübel fehlen im Körper (unter 7,5 mm oder verdeckt)."),
        ("Thermal: every region is the surface of its body solids with its one layer (top plank, leaf plank, apron board with the blocks, leg, steel rail); no hidden regions, no emitters. Structure: per half the top plate, the aprons, the legs and the runners are the members, the tenons, the gap blocks and the slide shoes at the inner ends of the tubes are the joints; the leaf plate is the member of the leaf; the joint edges carry the alignment dowels (three in the left half, two in the right half) that enter the other half when closed and the leaf when extended; the centre bars are the frame member; four felt glides are the supports; each half, the leaf and the slide frame carry their mass at their centre.",
         "Thermik: jede Region ist die Oberfläche ihrer Körper mit ihrer einen Schicht (Plattenbohle, Einlegebohle, Zargenbrett mit den Klötzen, Bein, Stahlschiene); keine verdeckten Regionen, keine Emitter. Tragwerk: je Hälfte sind Platte, Zargen, Beine und Laufschienen die Stäbe, Zapfen, Fugenklötze und die Gleitschuhe an den inneren Rohrenden die Gelenke; die Einlegeplatte ist der Stab der Einlage; die Stoßkanten tragen die Passdübel (drei in der linken, zwei in der rechten Hälfte), die geschlossen in die andere Hälfte und ausgezogen in die Einlage greifen; die Mittelstäbe sind der Rahmenstab; vier Filzgleiter sind die Auflager; jede Hälfte, die Einlage und der Auszugsrahmen tragen ihre Masse im Schwerpunkt."),
    ],
    "100": [
        ("Body, authored anew for 15 mm: the top halves and the leaf as square-edged 30 mm plates (the 15 mm bevel is one feature and merges into the edge), the shadow gap with the stored leaf, the 25 x 80 mm aprons, the tapered legs, and per side one 18 x 20 mm steel slide in which runner and centre bar are merged; the gap blocks are omitted.",
         "Körper, für 15 mm neu gebaut: Plattenhälften und Einlegeplatte als 30-mm-Platten mit gerader Kante (die 15-mm-Fase ist ein Merkmal und geht in der Kante auf), die Schattenfuge mit der verstauten Einlage, die Zargen 25 x 80 mm, die verjüngten Beine und je Seite ein Stahlauszug 18 x 20 mm, in dem Laufschiene und Mittelstab verschmolzen sind; die Fugenklötze entfallen."),
        ("Thermal: one surface per region; the slide is the steel region. Structure: plates, aprons, legs, leaf and slides as members; tenons, one 15 mm slide link per half and side, dowels of the joint edges; four glides; masses at the centres.",
         "Thermik: eine Oberfläche je Region; der Auszug ist die Stahlregion. Tragwerk: Platten, Zargen, Beine, Einlage und Auszüge als Stäbe; Zapfen, je Hälfte und Seite ein 15-mm-Auszugsglied, Dübel der Stoßkanten; vier Gleiter; Massen in den Schwerpunkten."),
    ],
    "200": [
        ("Body, authored anew for 30 mm: square 30 mm plates for the halves and the leaf, the shadow gap of one feature, 30 x 80 mm aprons, straight 60 mm legs (the 25 mm taper is below the feature) and one 30 mm slide bar per side.",
         "Körper, für 30 mm neu gebaut: gerade 30-mm-Platten für die Hälften und die Einlage, die Schattenfuge von einem Merkmal, Zargen 30 x 80 mm, gerade 60-mm-Beine (die 25-mm-Verjüngung liegt unter dem Merkmal) und je Seite ein 30-mm-Auszugsstab."),
        ("Thermal: one surface per region. Structure: plates, aprons, legs, leaf and slides as members; one 60 mm frame corner per leg and one slide link per half and side as the joints; glides; masses.",
         "Thermik: eine Oberfläche je Region. Tragwerk: Platten, Zargen, Beine, Einlage und Auszüge als Stäbe; je Bein eine 60-mm-Rahmenecke und je Hälfte und Seite ein Auszugsglied als Gelenke; Gleiter; Massen."),
    ],
    "500": [
        ("Body, authored anew for 75 mm: per half one 75 mm slab for top and shadow gap and below it a U frame of the three aprons (75 mm walls), two 75 mm square legs; the leaf is a 75 mm block, 0.78 m wide (the 60 mm it lacks per side is below the feature), that lies inside the slabs and the apron frame while stored and fills the gap flush when the table is extended. The model is minimal: nothing smaller can be left out without losing the table.",
         "Körper, für 75 mm neu gebaut: je Hälfte eine 75-mm-Platte für Platte und Schattenfuge und darunter ein U-Rahmen aus den drei Zargen (75-mm-Wände), zwei 75-mm-Vierkantbeine; die Einlage ist ein 75-mm-Block, 0,78 m breit (die fehlenden 60 mm je Seite liegen unter dem Merkmal), der verstaut in Platten und Zargenrahmen liegt und ausgezogen die Lücke bündig füllt. Das Modell ist minimal: kleiner geht es nicht, ohne den Tisch zu verlieren."),
        ("Thermal: slabs, frames, legs and leaf are their regions' surfaces; the slides, absent from the body, are one hidden 75 mm bar per side inside the frame. Structure: slabs, frames, legs, leaf and the two slide bars as members; glides of the leg section; masses at the centres.",
         "Thermik: Platten, Rahmen, Beine und Einlage sind die Oberflächen ihrer Regionen; die Auszüge, im Körper nicht vorhanden, sind je Seite ein verdeckter 75-mm-Stab im Rahmen. Tragwerk: Platten, Rahmen, Beine, Einlage und die zwei Auszugsstäbe als Stäbe; Gleiter im Beinquerschnitt; Massen in den Schwerpunkten."),
    ],
}


def decisions(scale):
    """💬 The bilingual decisions of a scale."""
    return [{"en": en, "de": de} for en, de in DECISIONS[scale]]
