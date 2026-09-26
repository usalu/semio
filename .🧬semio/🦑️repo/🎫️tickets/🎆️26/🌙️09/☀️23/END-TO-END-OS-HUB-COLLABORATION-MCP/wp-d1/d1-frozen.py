"""🧊️ D1 frozen patch set (session-12 rule 1: stdio, gis, vcs are hub-native codec crates, frozen until W2's `--packages all`).

Dry run by default (`D1_DRY_RUN=1` semantics); `--apply` writes. Lands in the post-publish landing window, compile-atomic:
`cargo check -p semio-s-plugin-gis -p semio-s-plugin-vcs -p semio-s-plugin-stdio` right after applying."""
import os, re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
if "--apply" not in sys.argv:
    os.environ["D1_DRY_RUN"] = "1"
from d1_apply import describe, ROOT

G = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts"
describe(f"{G}/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_gis2d_app", [
    ("setActiveExample", "Replaces the whole map with one of the plugin's bundled map examples, by example id.", "Ersetzt die gesamte Karte durch eines der mitgelieferten Kartenbeispiele, anhand der Beispiel-Id."),
    ("patchPositions", "Replaces the coordinates of the map's features from a JSON position list, writing only the positions that change.", "Ersetzt die Koordinaten der Kartenobjekte aus einer JSON-Positionsliste und schreibt nur die sich ändernden Positionen."),
    ("patchRoutes", "Sets one named field (such as name or colour) on several routes at once.", "Setzt ein benanntes Feld (etwa Name oder Farbe) auf mehreren Routen gleichzeitig."),
    ("patchRoute", "Sets one named field of one route, such as its name, colour or waypoints.", "Setzt ein benanntes Feld einer Route, etwa Name, Farbe oder Wegpunkte."),
    ("addFeature", "Adds a new point, line or area feature with a label at longitude and latitude to a collection of the map.", "Fügt einer Sammlung der Karte ein neues Punkt-, Linien- oder Flächenobjekt mit Bezeichnung an Länge und Breite hinzu."),
    ("moveFeature", "Moves one map feature of a collection to new longitude and latitude.", "Verschiebt ein Kartenobjekt einer Sammlung an neue Länge und Breite."),
    ("renameFeature", "Renames one map feature of a collection.", "Benennt ein Kartenobjekt einer Sammlung um."),
    ("deleteFeature", "Removes one feature by id from a collection of the map.", "Entfernt ein Objekt anhand seiner Id aus einer Sammlung der Karte."),
    ("openSource", "Opens the selected feature's source URL through the host.", "Öffnet die Quell-URL des ausgewählten Objekts über den Host."),
    ("proposeBoundsRegion", "Asks the inference service to propose a bounds region for human review; it never writes the map itself.", "Bittet den Inferenzdienst, eine Begrenzungsregion zur menschlichen Prüfung vorzuschlagen; die Karte selbst wird nie geschrieben."),
], [(verb, "Chrome") for verb in ["toggleLayerVisibility", "fitWorld", "setCamera", "setRenderMode", "setVectorStyle", "setLodMode", "focusFeature", "setLayerStrokeScale"]])

describe(f"{G}/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_gis3d_app", [
    ("setActiveExample", "Replaces the whole terrain with one of the plugin's bundled terrain examples, by example id.", "Ersetzt das gesamte Gelände durch eines der mitgelieferten Geländebeispiele, anhand der Beispiel-Id."),
    ("setExaggeration", "Sets the vertical exaggeration factor the terrain surface is drawn with.", "Legt den Überhöhungsfaktor fest, mit dem die Geländeoberfläche gezeichnet wird."),
], [("setCamera", "Chrome")])

describe("✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_vcs_app", [
    ("incrementCounter", "Adds one to the demo document's counter.", "Erhöht den Zähler des Demodokuments um eins."),
    ("patchSnapshot", "Sets one field of the demo document (title, counter, status or notes) from a text value.", "Setzt ein Feld des Demodokuments (Titel, Zähler, Status oder Notizen) aus einem Textwert."),
    ("textEdit", "Reads the given text as the demo document's projection and writes the title, counter, status and notes that differ; consecutive typing merges.", "Liest den angegebenen Text als Projektion des Demodokuments und schreibt abweichenden Titel, Zähler, Status und Notizen; fortlaufendes Tippen wird zusammengefasst."),
    ("edit", "Reads the given text as the demo document's projection and writes every field that differs as one edit.", "Liest den angegebenen Text als Projektion des Demodokuments und schreibt jedes abweichende Feld als eine Änderung."),
    ("setActiveExample", "Replaces the whole demo document with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte Demodokument durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."),
], [("noMutation", "Chrome")])

contract = f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs"
text = open(contract).read()
anchor = "pub fn set_active_example_args("
assert text.count(anchor) == 1
function = '''/// 💬️ The agent-facing description of every stdio editor's `setActiveExample` — one sentence for all formats,
/// beside the argument form every stdio editor already shares.
pub fn set_active_example_description() -> semio_framework_plugin::LocalizedLabel {
    semio_framework_plugin::LocalizedLabel::native(
        "Replaces the whole open document with one of the bundled examples for its format, by example id.",
        "Ersetzt das gesamte offene Dokument durch eines der mitgelieferten Beispiele für sein Format, anhand der Beispiel-Id.",
    )
}

'''
doc_start = text.rfind("\n\n", 0, text.find(anchor)) + 2
sites = 0
editors = []
for folder, _, files in os.walk(f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"):
    if "🧪️tests" in folder or "🦀️.rs" not in files:
        continue
    path = os.path.join(folder, "🦀️.rs")
    source = open(path).read()
    pattern = re.compile(r"^(\s*)\.action_destructive\(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID\)\n", re.M)
    if not pattern.search(source) or "set_active_example_description()" in source:
        continue
    patched, count = pattern.subn(lambda match: match.group(0) + f"{match.group(1)}.action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())\n", source)
    sites += count
    editors.append((path, patched))
print(f"stdio: contract fn + {sites} editor site(s) in {len(editors)} file(s)")
if "--apply" in sys.argv:
    open(contract, "w").write(text[:doc_start] + function + text[doc_start:])
    for path, patched in editors:
        open(path, "w").write(patched)
