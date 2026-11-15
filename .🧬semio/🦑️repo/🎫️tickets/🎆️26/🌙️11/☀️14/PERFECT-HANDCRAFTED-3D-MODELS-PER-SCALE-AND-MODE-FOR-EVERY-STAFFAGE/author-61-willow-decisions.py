"""Author 61: bilingual `decisions` of the weeping willow per scale (counts read from the source)."""
import json
F = "volumes/vegetation/weeping-willow.json"
d = json.load(open(F))
V = d["volumes"]
def n(s, l): return len(V[s][l])
import re
def nb(s, prefix): return len([x for x in V[s]["body"] if re.match(r"^lobe-c\d", x["id"])])
T = {
"50": (
 f"WILLOW 1:50: {n('50','body')} body solids. Root flare, trunk and the six limbs are lofts along the drawn elevation outlines (limbs as 3D tubes whose z comes from the front outline, kept inside the plan footprint); every one of the {nb('50','lobe-c')} drawn curtain sheets is one spindle loft whose section is the oriented superellipse fitted to its own plan footprint, scaled over the height by its own front width profile; the five caps are the same kind of loft; the plan-only crown fill is a stack of exact plan frusta. Two burls sit on the trunk. The elevation drawing showed only the sheets of two azimuths per ring, so the 32 missing sheet outlines were added to it.",
 f"WILLOW 1:50: {n('50','body')} Body-Solids. Wurzelanlauf, Stamm und die sechs Aeste sind Lofts entlang der gezeichneten Aufrissumrisse (Aeste als 3D-Roehren, deren z aus dem Vorderansichts-Umriss kommt und im Grundriss-Fussabdruck bleibt); jede der {nb('50','lobe-c')} gezeichneten Vorhangbahnen ist ein spindelfoermiger Loft, dessen Querschnitt die an den eigenen Grundriss angepasste, orientierte Superellipse ist und ueber die Hoehe mit dem eigenen Breitenverlauf der Vorderansicht skaliert wird; die fuenf Kappen sind ebensolche Lofts; die reine Grundriss-Kronenfuellung ist ein Stapel exakter Grundriss-Kegelstuempfe. Zwei Wuelste sitzen am Stamm. Der Aufriss zeigte nur die Bahnen zweier Azimute je Ring, daher wurden die 32 fehlenden Bahnumrisse ergaenzt.",
 "Thermal: every curtain, cap and fill solid is referenced by `leaf-curtain` (exchange area, `areaM2` untouched); trunk, flare and burls reference `trunk-bark`, the limb tubes `limb-bark`; the ground face of the root flare is a `sectionFaces` contact face. Ten transpiration patches stand at the emitter anchors.",
 "Thermik: jede Vorhang-, Kappen- und Fuellungs-Solid wird von `leaf-curtain` referenziert (Austauschflaeche, `areaM2` unveraendert); Stamm, Anlauf und Wuelste referenzieren `trunk-bark`, die Asttuben `limb-bark`; die Bodenflaeche des Wurzelanlaufs ist eine `sectionFaces`-Kontaktflaeche. Zehn Transpirationsfelder stehen an den Emitter-Ankern.",
 "Structure: member cores follow the centre line of each limb tube (60 % of its section) and the axis of each curtain's own spindle (the bone section), joints at the child starts, one fixed support plate under the root flare; everything lies inside the body silhouette.",
 "Struktur: Tragkerne folgen der Mittellinie jeder Asttube (60 % ihres Querschnitts) und der Achse der eigenen Spindel jedes Vorhangs (Knochenquerschnitt), Gelenke an den Anfaengen der Kindknochen, eine feste Auflageplatte unter dem Wurzelanlauf; alles liegt innerhalb der Koerpersilhouette."),
"100": (
 f"WILLOW 1:100: {n('100','body')} body solids, modelled again from the 1:100 drawings: one bole loft, one limb tube per side (limbs a to c merged as drawn), the {nb('100','lobe-c')} curtain sheets and five caps as spindle lofts of their own plan footprints, and the two plan crown fills as frusta stacks. The 20 missing elevation sheet outlines were added to the drawing.",
 f"WILLOW 1:100: {n('100','body')} Body-Solids, neu aus den Zeichnungen 1:100: ein Stammloft, eine Asttube je Seite (Aeste a bis c wie gezeichnet vereint), die {nb('100','lobe-c')} Vorhangbahnen und fuenf Kappen als Spindel-Lofts der eigenen Grundrisse und die beiden Grundriss-Kronenfuellungen als Kegelstumpf-Stapel. Die 20 fehlenden Aufriss-Bahnumrisse wurden der Zeichnung ergaenzt.",
 "Thermal: references as at 1:50 (curtains and caps to `leaf-curtain`, bole to `trunk-bark`, merged limbs to `limb-bark`); the ground face of the bole is a contact face.",
 "Thermik: Referenzen wie bei 1:50 (Vorhaenge und Kappen auf `leaf-curtain`, Stamm auf `trunk-bark`, vereinte Aeste auf `limb-bark`); die Bodenflaeche des Stamms ist eine Kontaktflaeche.",
 "Structure: cores along the limb tubes and the curtain axes, joints kept for the trunk and the limbs a and b (the joints of the outer limbs are merged into the curtains' joints), support plate under the bole.",
 "Struktur: Kerne entlang der Asttuben und der Vorhangachsen, Gelenke fuer Stamm und Aeste a und b (die Gelenke der Aussenaeste sind in die Vorhanggelenke aufgegangen), Auflageplatte unter dem Stamm."),
"200": (
 f"WILLOW 1:200: {n('200','body')} body solids from the 1:200 drawings: bole, one tube per side, 16 sheet spindles, four caps and four crown-fill frusta stacks; the outline is generalised to the sheets that the scale still draws. The 8 missing elevation sheet outlines were added.",
 f"WILLOW 1:200: {n('200','body')} Body-Solids aus den Zeichnungen 1:200: Stamm, eine Tube je Seite, 16 Bahn-Spindeln, vier Kappen und vier Kronenfuellungs-Stapel; der Umriss ist auf die Bahnen verallgemeinert, die der Massstab noch zeichnet. Die 8 fehlenden Aufriss-Bahnumrisse wurden ergaenzt.",
 "Thermal: references only; the contact face of the bole is marked; regions are covered, only the total area is compared at this scale.",
 "Thermik: nur Referenzen; die Kontaktflaeche des Stamms ist markiert; die Regionen sind abgedeckt, verglichen wird bei diesem Massstab nur die Gesamtflaeche.",
 "Structure: cores along the limb tube and the curtain axes, one support plate; joints are merged into the cores at this scale.",
 "Struktur: Kerne entlang der Asttube und der Vorhangachsen, eine Auflageplatte; Gelenke sind bei diesem Massstab in die Kerne aufgegangen."),
"500": (
 f"WILLOW 1:500: {n('500','body')} body solids: the bole and a crown of stacked prisms cut from the one drawn crown plan outline by the elevation and front intervals at each height (the scalloped crown is one mass here, not 44 sheets).",
 f"WILLOW 1:500: {n('500','body')} Body-Solids: der Stamm und eine Krone aus gestapelten Prismen, die aus dem einen gezeichneten Kronen-Grundriss durch die Aufriss- und Vorderansichts-Intervalle je Hoehe geschnitten sind (die gewellte Krone ist hier eine Masse, nicht 44 Bahnen).",
 "Thermal: the crown prisms are `leaf-curtain`; the buried limbs are two real thin tubes inside the crown (the limbs are not drawn at this scale), the bole is `trunk-bark`.",
 "Thermik: die Kronenprismen sind `leaf-curtain`; die verdeckten Aeste sind zwei echte duenne Roehren in der Krone (die Aeste sind in diesem Massstab nicht gezeichnet), der Stamm ist `trunk-bark`.",
 "Structure: the load path is kept as thin cores (limb tubes, curtain axes) inside the crown and a support plate under the bole.",
 "Struktur: der Lastpfad bleibt als duenne Kerne (Asttuben, Vorhangachsen) in der Krone und eine Auflageplatte unter dem Stamm erhalten."),
}
for s, v in T.items():
    V[s]["decisions"] = [{"en": v[0], "de": v[1]}, {"en": v[2], "de": v[3]}, {"en": v[4], "de": v[5]}]
json.dump(d, open(F, "w"), indent=1, ensure_ascii=False)
open(F, "a").write("\n")
print({s: len(V[s]["decisions"]) for s in V})
