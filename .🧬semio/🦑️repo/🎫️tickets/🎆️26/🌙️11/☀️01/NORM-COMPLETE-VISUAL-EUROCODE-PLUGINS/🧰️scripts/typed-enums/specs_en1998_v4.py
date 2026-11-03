import sys; sys.path.insert(0, ".")
from addspec import add
add("en1998", [
("En1998Site.en_ground_type", "EnGroundType", "🪨 EN 1998-1 ground type (Table 3.1) of the EN annex site.", "A",
 [(l, f"Ground type {l}", f"Baugrundtyp {l}", [l.lower(), l]) for l in "ABCDE"]),
("En1998Site.en_spectrum_type", "EnSpectrumType", "📈 EN 1998-1 3.2.2.2 elastic response spectrum type.", "Type1",
 [("Type1", "Type 1 spectrum", "Spektrum Typ 1", ["type1", ""]), ("Type2", "Type 2 spectrum", "Spektrum Typ 2", ["type2"])]),
("En1998Site.importance_class", "ImportanceClass", "🏛 Importance class I to IV of the building (EN 1998-1 4.2.5).", "II",
 [("I", "Class I", "Bedeutungskategorie I", ["i"]), ("II", "Class II", "Bedeutungskategorie II", ["ii"]), ("III", "Class III", "Bedeutungskategorie III", ["iii"]), ("IV", "Class IV", "Bedeutungskategorie IV", ["iv"])]),
("En1998System.system_type", "SystemType", "🏗 Structural type of a lateral-force-resisting system (EN 1998-1 5.1.2, 6.3.1, 7.2.1).", "Frame",
 [("Frame", "Moment frame", "Rahmen", ["frame", ""]), ("Wall", "Wall system", "Wandsystem"), ("Dual", "Dual system", "Dualsystem"), ("Masonry", "Masonry", "Mauerwerk")]),
("En1998System.material,En1998Member.material", "StructuralMaterial", "🧱 Structural material of a system or member.", "Rc",
 [("Rc", "Reinforced concrete", "Stahlbeton", ["rc", ""]), ("Steel", "Steel", "Stahl"), ("Composite", "Composite", "Verbund"), ("Timber", "Timber", "Holz"), ("Masonry", "Masonry", "Mauerwerk")]),
("En1998System.ductility_class", "DuctilityClass", "📐 Ductility class DCL, DCM or DCH (EN 1998-1 5.2.1).", "Dcm",
 [("Dcl", "DCL (low)", "DCL (niedrig)"), ("Dcm", "DCM (medium)", "DCM (mittel)"), ("Dch", "DCH (high)", "DCH (hoch)")]),
("En1998Assessment.knowledge_level", "KnowledgeLevel", "🔍 Knowledge level KL1 to KL3 of an existing building (EN 1998-3 3.3).", "Kl1",
 [("Kl1", "KL1 (limited knowledge)", "KL1 (begrenzte Kenntnis)"), ("Kl2", "KL2 (normal knowledge)", "KL2 (normale Kenntnis)"), ("Kl3", "KL3 (full knowledge)", "KL3 (vollständige Kenntnis)")]),
("En1998Diaphragm.material", "DiaphragmMaterial", "🧱 Material of a floor diaphragm.", "Concrete",
 [("Concrete", "Concrete", "Beton"), ("Timber", "Timber", "Holz"), ("SteelDeck", "Steel deck", "Stahlprofiltafel", ["steel-deck", "steel", "steeldeck"])]),
("En1998Bearing.device", "BearingDevice", "⚙ Bearing device type of a bridge bearing.", "Fixed",
 [("Fixed", "Fixed", "Fest"), ("Elastomeric", "Elastomeric", "Elastomer"), ("Sliding", "Sliding", "Gleitend"), ("Isolator", "Isolator", "Isolator")]),
("En1998VariableAction.category,En1998Silo.content_category,En1998Tank.content_category", "ImposedCategory", "📦 Imposed-load or content category of an EN 1990 Table A1.1 action (psi_2 row).", "B",
 [("A", "Residential (A)", "Wohnen (A)", ["a", "residential"]), ("B", "Office (B)", "Büro (B)", ["b", "office", ""]), ("C", "Congregation (C)", "Versammlung (C)", ["c", "congregation"]), ("D", "Retail (D)", "Verkauf (D)", ["d", "retail"]), ("E", "Storage (E)", "Lager (E)", ["e", "storage"]), ("F", "Traffic (F)", "Verkehr (F)", ["f", "traffic"]), ("G", "Vehicles (G)", "Fahrzeuge (G)", ["g", "vehicles"]), ("H", "Roofs (H)", "Dächer (H)", ["h", "roofs"]), ("Snow", "Snow", "Schnee"), ("Wind", "Wind", "Wind")]),
])
