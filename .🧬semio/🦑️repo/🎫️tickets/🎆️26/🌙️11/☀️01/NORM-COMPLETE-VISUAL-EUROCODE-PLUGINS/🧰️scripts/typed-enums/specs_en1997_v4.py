import sys; sys.path.insert(0, ".")
from addspec import add
add("en1997", [
("En1997Snapshot.design_situation,FoundationLoadCase.design_situation", "SituationChoice", "⚖️ DIN 1054 design situation BS-P, BS-T or BS-A of a geotechnical check.", "Persistent",
 [("Persistent", "Persistent (BS-P)", "Ständig (BS-P)", ["bsp", "persistent"]), ("Transient", "Transient (BS-T)", "Vorübergehend (BS-T)", ["bst", "transient"]), ("Accidental", "Accidental (BS-A)", "Außergewöhnlich (BS-A)", ["bsa", "accidental"])]),
("En1997Snapshot.design_approach", "ApproachChoice", "🧭 Design approach of EN 1997-1 2.4.7.3.4 (DA1 combinations, DA2, DA2*, DA3, GEO-3).", "Da1Str",
 [("Da1Str", "DA1 Combination 1 (STR)", "DA1 Kombination 1 (STR)", ["da1str"]), ("Da1Geo", "DA1 Combination 2 (GEO)", "DA1 Kombination 2 (GEO)", ["da1geo"]), ("Da2", "DA2 (A1 + M1 + R2)", "DA2 (A1 + M1 + R2)", ["da2"]), ("Da2Star", "DA2* (effects factored)", "DA2* (Beanspruchungen faktorisiert)", ["da2star"]), ("Da3", "DA3 (A1/A2 + M2 + R3)", "DA3 (A1/A2 + M2 + R3)", ["da3"]), ("Geo3", "GEO-3 / GZ 1C (A2 + M2 + R3)", "GEO-3 / GZ 1C (A2 + M2 + R3)", ["geo3"])]),
("SoilLayer.soil_type", "SoilType", "🪨 Soil class of a layer (GK1 presumptive bearing table).", "Sand",
 [("Sand", "Sand", "Sand"), ("Gravel", "Gravel", "Kies"), ("Clay", "Clay", "Ton"), ("Silt", "Silt", "Schluff"), ("Fill", "Fill", "Auffüllung")]),
("Pile.pile_type", "PileType", "🏗 Pile installation type (bored, driven or CFA).", "Bored",
 [("Bored", "Bored pile", "Bohrpfahl"), ("Driven", "Driven pile", "Rammpfahl"), ("Cfa", "CFA pile", "Schneckenortbetonpfahl (CFA)")]),
("RetainingWall.earth_pressure_mode", "EarthPressureMode", "🧱 Earth pressure regime behind a retaining wall (DIN 4085).", "Active",
 [("Active", "Active (Rankine/Coulomb)", "Aktiv (Rankine/Coulomb)"), ("AtRest", "At rest (Jáky K₀)", "Ruhe (Jáky K₀)"), ("IncreasedActive", "Increased active (DIN 4085)", "Erhöht aktiv (DIN 4085)")]),
("RetainingWall.wall_movement", "WallMovement", "↔️ Wall movement regime of a retaining wall (DIN 4085).", "Free",
 [("Free", "Free (cantilever, sufficient displacement)", "Frei (Kragarm, ausreichende Verschiebung)"), ("Rigid", "Rigid (little displacement)", "Starr (geringe Verschiebung)"), ("Propped", "Propped / supported", "Gestützt")]),
("Anchor.tendon", "TendonType", "🔗 Tendon type of a ground anchor.", "Strand",
 [("Strand", "Strand tendon", "Litzenanker"), ("Bar", "Bar tendon", "Stabanker")]),
("EmbeddedWall.kind", "EmbeddedWallKind", "🧱 Support condition of an embedded wall.", "FreeEarth",
 [("FreeEarth", "Free earth support", "Freie Auflagerung"), ("Cantilever", "Cantilever (fixed in the ground)", "Einspannung im Boden")]),
("LabTest.kind", "LabTestKind", "🧪 Laboratory test kind.", "Oedometer",
 [("Oedometer", "Oedometer test", "Ödometerversuch"), ("Triaxial", "Triaxial test", "Triaxialversuch"), ("DirectShear", "Direct shear test", "Rahmenscherversuch")]),
])
import json
p="specs/en1997.json"; d=json.load(open(p,encoding="utf-8")); d["wire"]="camel"; json.dump(d,open(p,"w",encoding="utf-8"),ensure_ascii=False,indent=1)
