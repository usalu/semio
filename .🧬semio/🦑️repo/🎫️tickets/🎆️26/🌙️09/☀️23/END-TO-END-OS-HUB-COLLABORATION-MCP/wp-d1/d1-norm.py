"""📕️ D1 codemod: agent-facing en/de descriptions for the four verbs of each of the 15 norm compliance editors."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

STANDARDS = [
    ("⚖️en1990", "en1990", "EN 1990 (Eurocode 0, basis of structural design)", "EN 1990 (Eurocode 0, Grundlagen der Tragwerksplanung)"),
    ("🏋️en1991", "en1991", "EN 1991 (Eurocode 1, actions on structures)", "EN 1991 (Eurocode 1, Einwirkungen auf Tragwerke)"),
    ("🏛️en1992", "en1992", "EN 1992 (Eurocode 2, concrete structures)", "EN 1992 (Eurocode 2, Betonbau)"),
    ("🔩️en1993", "en1993", "EN 1993 (Eurocode 3, steel structures)", "EN 1993 (Eurocode 3, Stahlbau)"),
    ("🧩️en1994", "en1994", "EN 1994 (Eurocode 4, composite steel and concrete structures)", "EN 1994 (Eurocode 4, Verbundbau aus Stahl und Beton)"),
    ("🪵️en1995", "en1995", "EN 1995 (Eurocode 5, timber structures)", "EN 1995 (Eurocode 5, Holzbau)"),
    ("🪨️en1996", "en1996", "EN 1996 (Eurocode 6, masonry structures)", "EN 1996 (Eurocode 6, Mauerwerksbau)"),
    ("🌍️en1997", "en1997", "EN 1997 (Eurocode 7, geotechnical design)", "EN 1997 (Eurocode 7, Geotechnik)"),
    ("🫨️en1998", "en1998", "EN 1998 (Eurocode 8, earthquake resistance)", "EN 1998 (Eurocode 8, Erdbebenauslegung)"),
    ("🪶️en1999", "en1999", "EN 1999 (Eurocode 9, aluminium structures)", "EN 1999 (Eurocode 9, Aluminiumbau)"),
    ("🧱️din4108", "din4108", "DIN 4108 (thermal insulation and moisture protection)", "DIN 4108 (Wärmeschutz und Feuchteschutz)"),
    ("⚡️din18599", "din18599", "DIN V 18599 (energy performance of buildings)", "DIN V 18599 (energetische Bewertung von Gebäuden)"),
    ("🌬️din16798", "din16798", "DIN EN 16798 (indoor environment and ventilation)", "DIN EN 16798 (Innenraumklima und Lüftung)"),
    ("📇️iso16757", "iso16757", "ISO 16757 (product data for building services catalogues)", "ISO 16757 (Produktdaten für Kataloge der Gebäudetechnik)"),
    ("🏭️vdi3805", "vdi3805", "VDI 3805 (product data exchange in building services)", "VDI 3805 (Produktdatenaustausch in der Gebäudetechnik)"),
]

for folder, variant, en, de in STANDARDS:
    describe(f"✏️s/🔌️plugins/📕️norm/🗿️artifacts/{folder}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", f"create_{variant}_app", [
        ("setSnapshot", f"Replaces the whole {en} compliance document with the supplied document JSON; the previous inputs are discarded.", f"Ersetzt das gesamte Nachweisdokument nach {de} durch das übergebene Dokument-JSON; die bisherigen Eingaben werden verworfen."),
        ("evaluate", f"Recomputes every {en} check from the document's inputs and refreshes the results window; the document is not changed.", f"Berechnet alle Nachweise nach {de} aus den Eingaben des Dokuments neu und aktualisiert das Ergebnisfenster; das Dokument ändert sich nicht."),
        ("setSelectedCheckIndex", "Points the inspection panel at one computed check by its index in the results list; only the view changes.", "Richtet das Inspektionspanel anhand seines Index in der Ergebnisliste auf einen berechneten Nachweis aus; nur die Ansicht ändert sich."),
        ("setActiveExample", f"Loads one of the bundled {en} examples into the open compliance document, replacing its inputs, by example id.", f"Lädt eines der mitgelieferten Beispiele nach {de} in das offene Nachweisdokument und ersetzt dessen Eingaben, anhand der Beispiel-Id."),
    ])
