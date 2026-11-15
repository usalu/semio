"""Author 47: bilingual decisions of miscanthus-grass per scale."""
import json
P = "volumes/vegetation/miscanthus-grass.json"
d = json.load(open(P))
T = {
 "50": [
  ("Body: every blade is its own sheet-like loft (flat superelliptic sections, width horizontal and perpendicular to the blade path, thickness from the drawn elevation) whose path and half widths are read from that blade's own elevation and plan outlines; mirror pairs of blades (a, b segments, azimuth fans) are separate pieces for z<0 and z>0. Culms are slender lofts from elevation and front, the four plumes lofts along their bone, the rhizome tuft a cone loft. 73 solids replace the previous 238 fragments.",
   "Körper: jedes Blatt ist ein eigenes blattartiges Loft (flache superelliptische Querschnitte, Breite waagrecht und senkrecht zum Blattverlauf, Dicke aus der gezeichneten Ansicht), dessen Verlauf und Halbbreiten aus dem eigenen Aufriss und Grundriss des Blattes gelesen sind; spiegelbildliche Blattpaare (Abschnitte a und b, Azimutfächer) sind getrennte Teile für z<0 und z>0. Halme sind schlanke Lofts aus Aufriss und Ansicht, die vier Rispen Lofts entlang ihres Knochens, der Wurzelstock ein Kegel-Loft. 73 Körper ersetzen die früheren 238 Bruchstücke."),
  ("Thermal: blades, plumes and culms reference the body pieces of their region (no copies); blades are an exchange area (leaf area) and need coverage only; the exposed area of plumes and culms agrees with areaM2 within 15 %. Transpiration is an emitter solid at the first blade anchor.",
   "Thermik: Blätter, Rispen und Halme verweisen auf die Körper ihrer Region (keine Kopien); die Blätter sind Austauschfläche (Blattfläche) und brauchen nur Abdeckung; die exponierte Fläche von Rispen und Halmen stimmt mit areaM2 auf 15 % überein. Die Transpiration ist ein Emitterkörper am ersten Blattanker."),
  ("Structure: the load path runs from the rhizome mat (cone-shaped core in the tuft) through four culm cores (slender, along each culm's own piece) to the plume rachis; each blade has a thin midrib core along the blade-1 piece of its bone; joints are small balls where bone cores meet, the support is the anchorage plate under the tuft.",
   "Tragwerk: der Lastpfad läuft vom Wurzelfilz (kegelförmiger Kern im Wurzelstock) über vier Halmkerne (schlank, entlang des Halmstücks) zur Rispenspindel; jedes Blatt hat eine dünne Mittelrippe entlang des Blatt-1-Stücks seines Knochens; Gelenke sind kleine Kugeln, wo Knochenkerne zusammentreffen, das Auflager ist die Verankerungsplatte unter dem Wurzelstock."),
 ],
 "100": [
  ("Body: at 1:100 the drawings show blade bundles, not single blades. Each bundle is a hull loft of the drawn front band (z-y or edge-on) with the x extent read where plan and elevation both draw material; culms, plumes and the tuft are lofts as at 1:50 with this scale's own outlines. 49 solids; generalised, not the 1:50 list.",
   "Körper: bei 1:100 zeigen die Zeichnungen Blattbündel statt Einzelblätter. Jedes Bündel ist ein Hüllen-Loft aus dem gezeichneten Frontband (z-y oder von der Kante) mit der x-Ausdehnung dort, wo Grundriss und Aufriss beide Material zeichnen; Halme, Rispen und Wurzelstock sind Lofts wie bei 1:50 mit den Umrissen dieses Maßstabs. 49 Körper; verallgemeinert, nicht die 1:50-Liste."),
  ("Thermal: bundles, plumes and culms reference body pieces by region; blades coverage only (exchange area); plumes and culms within 30 % of areaM2.",
   "Thermik: Bündel, Rispen und Halme verweisen regionsweise auf Körper; Blätter nur Abdeckung (Austauschfläche); Rispen und Halme innerhalb 30 % von areaM2."),
  ("Structure: same load path as 1:50 (rhizome core, culm cores, plume rachis, one midrib per blade bone) with joints; cores follow the centre path of this scale's own pieces.",
   "Tragwerk: derselbe Lastpfad wie bei 1:50 (Wurzelkern, Halmkerne, Rispenspindel, eine Mittelrippe je Blattknochen) mit Gelenken; die Kerne folgen dem Mittelpfad der Stücke dieses Maßstabs."),
 ],
 "200": [
  ("Body: blade bundles as hull lofts of the drawn front bands (as 1:100, fewer), culms as short capsule chains along the drawn culm axis strokes (at the stroke's own pen width), four plume lofts, tuft cone; 47 solids.",
   "Körper: Blattbündel als Hüllen-Lofts der gezeichneten Frontbänder (wie 1:100, weniger), Halme als kurze Kapselketten entlang der gezeichneten Halmachsen (in der Strichbreite), vier Rispen-Lofts, Wurzelkegel; 47 Körper."),
  ("Thermal: references by region; culm chains referenced as culms; only the sum of exposed areas is compared at this scale.",
   "Thermik: Verweise je Region; Halmketten als Halme referenziert; bei diesem Maßstab wird nur die Summe der exponierten Flächen verglichen."),
  ("Structure: members only (no joints at 1:200): rhizome core, culm cores, rachis and one midrib per blade bone, support plate.",
   "Tragwerk: nur Stäbe (keine Gelenke bei 1:200): Wurzelkern, Halmkerne, Spindel und eine Mittelrippe je Blattknochen, Auflagerplatte."),
 ],
 "500": [
  ("Body: the whole clump is one merged mass: a plan-outline skirt (extrusion of the drawn plan outline between 0.06 and 0.4 m), a dome loft from the elevation and front outlines up to 1.15 m and a spire loft for the culm and plume zone above; 3 solids.",
   "Körper: der ganze Horst ist eine verschmolzene Masse: ein Sockel aus dem gezeichneten Grundrissumriss (Extrusion zwischen 0,06 und 0,4 m), ein Kuppel-Loft aus Aufriss- und Frontumriss bis 1,15 m und ein Spitzen-Loft für die Halm- und Rispenzone darüber; 3 Körper."),
  ("Thermal: skirt and dome carry the blades (exchange area, coverage only), the spire carries the plumes, the culms are a stem solid inside; the envelope of the merged spire exposes more than the physical plume area (see author-47.md).",
   "Thermik: Sockel und Kuppel tragen die Blätter (Austauschfläche, nur Abdeckung), die Spitze trägt die Rispen, die Halme sind ein innenliegender Stielkörper; die Hülle der verschmolzenen Spitze legt mehr Fläche frei als die physische Rispenfläche (siehe author-47.md)."),
  ("Structure: members of every bone as cores inside the merged mass, support plate; no joints at 1:500.",
   "Tragwerk: Stäbe jedes Knochens als Kerne in der verschmolzenen Masse, Auflagerplatte; keine Gelenke bei 1:500."),
 ],
}
for sc, rows in T.items():
    d["volumes"][sc]["decisions"] = [{"en": en, "de": de} for en, de in rows]
json.dump(d, open(P, "w"), ensure_ascii=False, separators=(",", ":"))
print("ok")
