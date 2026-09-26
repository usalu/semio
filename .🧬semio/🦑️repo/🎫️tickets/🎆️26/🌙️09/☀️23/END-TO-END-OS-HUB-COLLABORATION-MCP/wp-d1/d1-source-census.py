"""🧾️ D1 source-level census: every agent-published verb the committed descriptors list without a description must now carry
an `action_describe`, an audience declaration or a framework/SDK description in the source that builds its app.

Heuristic by construction (the law is the Rust/AJV census over regenerated descriptors); this is the pre-describe proof."""
import json, os, re, sys, collections
ROOT = "/Users/ueli/Documents/semio"
rows = json.load(open(f"{ROOT}/.tmp-ticket/wp-d1/generated/census-before.json"))
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
SDK_DESCRIBED = {"set-cell", "set-node", "replace-text"}
FROZEN = {"gis", "stdio", "vcs"}
cache = {}

def source_text(folder):
    if folder not in cache:
        chunks = []
        for base, _, files in os.walk(folder):
            if "🧪️tests" in base or "/target" in base:
                continue
            for name in files:
                if name == "🦀️.rs":
                    chunks.append(open(os.path.join(base, name)).read())
        cache[folder] = "\n".join(chunks)
    return cache[folder]

def app_folder(plugin, app):
    key = (app or "").split("@")[0]
    if key in APP_FILES:
        return f"{P}/{APP_FILES[key]}"
    if plugin == "norm":
        return f"{P}/📕️norm/🗿️artifacts"
    for entry in os.listdir(P):
        if entry.endswith(plugin) or re.sub(r"^[^a-z]+", "", entry) == plugin:
            return f"{P}/{entry}"
    return None

missing = collections.defaultdict(list)
covered = collections.Counter()
for row in rows:
    if row["audience"] != "agent" or (row["desc_en"] and row["desc_de"]):
        continue
    plugin, app = row["plugin"], row["app"]
    verb = row["id"].rsplit(".", 1)[-1] if row["shape"] == "action" else row["id"].split(".")[-1]
    if verb in SDK_DESCRIBED:
        covered["sdk"] += 1
        continue
    folder = app_folder(plugin, app)
    text = source_text(folder) if folder else ""
    quoted = [f'"{verb}"'] + [const for const in re.findall(r"const (\w+): &str = \"" + re.escape(verb) + r"\"", text)]
    described = any(re.search(r"\.action_(describe|audience)\(\s*" + re.escape(token), text) for token in quoted)
    described = described or (verb == "listFlowExtensions" and "set_active_example_description" not in text and ".describe(LocalizedLabel::native(\"Lists every flow extension" in source_text(f"{P}/🌀️procedural"))
    if described:
        covered["frozen" if plugin in FROZEN else "source"] += 1
    else:
        missing["frozen" if plugin in FROZEN else plugin].append(f"{app} {verb}")
print("covered", dict(covered))
for plugin, entries in sorted(missing.items()):
    print(f"MISSING {plugin} {len(entries)}: {entries[:12]}")
