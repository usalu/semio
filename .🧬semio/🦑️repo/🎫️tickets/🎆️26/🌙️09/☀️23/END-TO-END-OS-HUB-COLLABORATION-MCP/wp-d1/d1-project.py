"""🔮️ D1: projects the landed source declarations (descriptions, audiences, destructive marks) onto a copy of the committed
descriptors, so the description law, the capability audit and `capabilities_search` can be measured before W2 re-describes.

Honest by construction: only literal `.action_describe/.action_audience/.action_destructive` steps and the SDK window-kit /
framework texts that are in the tree now are projected; frozen patch sets (`d1-frozen.py`) are not. Output root:
`.🧬semio/🌐hub/s12-d1-projected` (gitignored) with `nx.json`, the registry and one patched `🔣️.json` per plugin."""
import json, os, re, shutil, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
import importlib.util
spec = importlib.util.spec_from_file_location("census", "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1/d1-source-census.py")
ROOT = "/Users/ueli/Documents/semio"
OUT = f"{ROOT}/.🧬semio/🌐hub/s12-d1-projected"
REGISTRY = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"
P = f"{ROOT}/✏️s/🔌️plugins"
APP_FILES = {
    "s.wfc.bitmap": "🀄️wfc/🗿️artifacts/🖼️bitmap", "s.wfc.grid2d": "🀄️wfc/🗿️artifacts/🔲️grid2d", "s.wfc.grid3d": "🀄️wfc/🗿️artifacts/🧱️grid3d",
    "s.wfc.wfc2d": "🀄️wfc/🗿️artifacts/◻️2d", "s.wfc.wfc3d": "🀄️wfc/🗿️artifacts/🧊️3d",
    "s.block.block2d": "🧱️block/🗿️artifacts/◻️2d", "s.block.block3d": "🧱️block/🗿️artifacts/🧊️3d", "s.block.block5d": "🧱️block/🗿️artifacts/🖐️5d",
    "s.fem.fem2d": "🏗️fem/🗿️artifacts/◻️2d", "s.fem.fem3d": "🏗️fem/🗿️artifacts/🧊️3d",
    "s.procedural.generation2d": "🌀️procedural/🗿️artifacts/🌀️generation2d", "s.procedural.generation3d": "🌀️procedural/🗿️artifacts/🧊️generation3d",
    "s.puzzle.puzzle2d": "🧩️puzzle/🗿️artifacts/◻️2d", "s.puzzle.puzzle3d": "🧩️puzzle/🗿️artifacts/🧊️3d", "s.puzzle.puzzle5d": "🧩️puzzle/🗿️artifacts/🖐️5d",
    "s.space.home": "🪐️space/🗿️artifacts/🏠️home", "s.space.space": "🪐️space/🗿️artifacts/🪐️space", "s.space.studio": "🪐️space/⚙️engine/🪐️space",
    "s.trinity.jack": "🔱️trinity/🗿️artifacts/🔌️jack", "s.trinity.rewriting": "🔱️trinity/🗿️artifacts/♻️rewriting",
    "s.gis.gismap": "🌍️gis/🗿️artifacts/🗺️gismap", "s.gis.gisterrain": "🌍️gis/🗿️artifacts/🏔️gisterrain",
    "s.process.process3d": "🏭️process/🗿️artifacts/🧊️process3d", "s.sourcing.curation": "🪵️sourcing/🗿️artifacts/🗂️curation",
    "s.playbook.procedural": "📖️playbook/🧩️extensions/🌀️procedural", "s.demonstrator.playground": "🎪️demonstrator/🗿️artifacts/🎪️playground",
}
STR = r'"((?:[^"\\]|\\.)*)"'
DESCRIBE = re.compile(r'\.action_describe\(\s*' + STR + r',\s*(?:[\w:]*::)?LocalizedLabel::native\(\s*' + STR + r',\s*' + STR + r',?\s*\)\s*\)')
AUDIENCE = re.compile(r'\.action_audience\(\s*' + STR + r',\s*[\w:]*CapabilityAudience::(\w+)\s*\)')
DESTRUCTIVE = re.compile(r'\.action_destructive\(\s*' + STR + r'\s*\)')
KIT = {
    "replace-text": ("Replaces the entire text of the document with the given text; the previous text is gone unless the edit is undone.", "Ersetzt den gesamten Text des Dokuments durch den angegebenen Text; der bisherige Text ist fort, sofern die Änderung nicht rückgängig gemacht wird.", True),
    "set-cell": ("Writes the given value into one table cell, addressed by row index and column, replacing what the cell held before.", "Schreibt den angegebenen Wert in eine Tabellenzelle, adressiert über Zeilenindex und Spalte, und ersetzt ihren bisherigen Inhalt.", False),
    "set-node": ("Writes the given value into one node of the document tree, addressed by node id, replacing the node's previous value.", "Schreibt den angegebenen Wert in einen Knoten des Dokumentbaums, adressiert über die Knoten-Id, und ersetzt dessen bisherigen Wert.", False),
}
PLUGIN_COMMANDS = {("procedural", "listFlowExtensions"): ("Lists every flow extension the procedural plugin can load (id, extension, label and version); nothing is changed.", "Listet alle Flow-Erweiterungen auf, die das Prozedural-Plugin laden kann (Id, Erweiterung, Bezeichnung und Version); nichts wird geändert.")}
cache = {}

def declarations(folder):
    if folder not in cache:
        text = []
        for base, _, files in os.walk(folder):
            if "🧪️tests" in base or "/target" in base:
                continue
            text.extend(open(os.path.join(base, name)).read() for name in files if name == "🦀️.rs")
        joined = "\n".join(text)
        cache[folder] = (
            {m.group(1): (m.group(2), m.group(3)) for m in DESCRIBE.finditer(joined)},
            {m.group(1): m.group(2).lower() for m in AUDIENCE.finditer(joined)},
            {m.group(1) for m in DESTRUCTIVE.finditer(joined)},
        )
    return cache[folder]

FROZEN = []
if "--with-frozen" in sys.argv:
    import d1_apply
    d1_apply.describe = lambda path, fn, rows, audiences=(), destructive=(), **_: FROZEN.append((f"{ROOT}/{path}", {vid: (en, de) for vid, en, de in rows}, {vid: audience.lower() for vid, audience in audiences}, set(destructive)))
    os.environ["D1_DRY_RUN"] = "1"
    frozen_spec = importlib.util.spec_from_file_location("frozen", "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1/d1-frozen.py")
    frozen_spec.loader.exec_module(importlib.util.module_from_spec(frozen_spec))
    OUT = OUT + "-with-frozen"
STDIO_EXAMPLE = ("Replaces the whole open document with one of the bundled examples for its format, by example id.", "Ersetzt das gesamte offene Dokument durch eines der mitgelieferten Beispiele für sein Format, anhand der Beispiel-Id.")

def with_frozen(folder, found):
    described, audiences, destructive = dict(found[0]), dict(found[1]), set(found[2])
    for path, rows, extra_audiences, extra_destructive in FROZEN:
        if path.startswith(folder + "/") or os.path.dirname(os.path.dirname(path)).startswith(folder):
            described.update(rows)
            audiences.update(extra_audiences)
            destructive |= extra_destructive
    if FROZEN and "🗄️stdio" in folder:
        described.setdefault("setActiveExample", STDIO_EXAMPLE)
    return described, audiences, destructive

def folder_for(plugin_root, plugin, app_id):
    key = app_id.split("@")[0]
    if key in APP_FILES:
        return f"{P}/{APP_FILES[key]}"
    if plugin == "norm":
        variant = key.split(".")[-1]
        return next(f"{P}/📕️norm/🗿️artifacts/{entry}" for entry in os.listdir(f"{P}/📕️norm/🗿️artifacts") if entry.endswith(variant))
    return plugin_root

def label(en, de):
    return {"native": {"de": de, "en": en}, "reuse": {"de": de, "en": en}}

def patch(row, described, audiences, destructive):
    semantics = row.setdefault("semantics", {})
    vid = row["id"]
    if vid in described and not semantics.get("description"):
        semantics["description"] = label(*described[vid])
    if vid in KIT and not semantics.get("description"):
        semantics["description"] = label(KIT[vid][0], KIT[vid][1])
        if KIT[vid][2]:
            destructive = destructive | {vid}
    if vid in audiences:
        semantics["audience"] = audiences[vid]
        if audiences[vid] != "agent":
            row["inPalette"] = False
    if vid in destructive:
        semantics.setdefault("effects", {})["destructive"] = True
        policy = semantics.setdefault("policy", {})
        if policy.get("approval", "never") == "never":
            policy["approval"] = "whenDestructive"

if os.path.exists(OUT):
    shutil.rmtree(OUT)
os.makedirs(os.path.dirname(f"{OUT}/{REGISTRY}"))
shutil.copy(f"{ROOT}/nx.json", f"{OUT}/nx.json")
shutil.copy(f"{ROOT}/{REGISTRY}", f"{OUT}/{REGISTRY}")
touched = 0
for entry in json.load(open(f"{ROOT}/{REGISTRY}")):
    owner = os.path.dirname(os.path.dirname(entry["cratePath"]))
    descriptor = json.load(open(f"{ROOT}/{owner}/🔣️.json"))
    manifest = descriptor["manifest"]
    for app in manifest.get("apps", []):
        folder = folder_for(f"{ROOT}/{owner}", manifest["pluginId"], app["id"])
        described, audiences, destructive = with_frozen(folder, declarations(folder))
        rows = [action for kind in app.get("windowKinds", []) for action in kind.get("actions", [])] + app.get("actions", []) + app.get("commands", []) + [command for mode in app.get("modes", []) for command in mode.get("commands", [])]
        for row in rows:
            before = json.dumps(row, sort_keys=True)
            patch(row, described, audiences, destructive)
            touched += before != json.dumps(row, sort_keys=True)
    for command in manifest.get("commands", []):
        text = PLUGIN_COMMANDS.get((manifest["pluginId"], command["id"]))
        if text and not command.get("semantics", {}).get("description"):
            command.setdefault("semantics", {})["description"] = label(*text)
            touched += 1
    os.makedirs(f"{OUT}/{owner}", exist_ok=True)
    json.dump(descriptor, open(f"{OUT}/{owner}/🔣️.json", "w"), ensure_ascii=False)
print(f"projected {touched} declaration row(s) into {OUT}")
