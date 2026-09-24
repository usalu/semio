#!/usr/bin/env python3
"""✋️ U5 one-off: the label edits a literal swap cannot express.

1. `stdio.semio` base envelope `apply-*` leaves labelled themselves with their subset's bare name ("cad"/"cad",
   "graph"/"Graph") — the history showed "Graph" for every graph edit. They now delegate to the wrapped subset
   mutation's own `SemanticMutation::label`, which is already localized.
2. Count-bearing labels used an English `(s)` / German `(s)`/`(e)` hack; they now pick singular or plural per count.

Every edit is an exact, unique substring replacement; a missing or ambiguous anchor aborts before any write.
Usage: python3 u5-manual-label-edits.py <repoRoot> [--apply]
"""
import os
import sys

ROOT = sys.argv[1]
APPLY = "--apply" in sys.argv
BASE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations"
DELEGATE = "protocol::SemanticMutation::label(&self.mutation)"
APPLY_LEAVES = {
    "🌐apply-graph": ("graph", "Graph"), "🎞️apply-animation": ("animation", "Animation"), "🎬apply-video": ("video", "Video"),
    "🏛️apply-model": ("model", "Modell"), "📃apply-document": ("document", "Dokument"), "📐apply-cad": ("cad", "cad"),
    "📦apply-object": ("object", "Objekt"), "📽️apply-presentation": ("presentation", "Präsentation"), "🔀apply-flow": ("flow", "Fluss"),
    "🔊apply-audio": ("audio", "Audio"), "🔢apply-value": ("value", "Wert"), "🔤apply-text": ("text", "Text"), "🕸️apply-mesh": ("mesh", "Netz"),
    "🖊️apply-drawing": ("drawing", "Zeichnung"), "🖼️apply-image": ("image", "Bild"), "🗂️apply-table": ("table", "Tabelle"),
    "🧰apply-kit": ("kit", "Kit"), "🧱apply-brep": ("brep", "brep"),
}


def plural(count: str, en_verb: str, en_one: str, en_many: str, de_one: str, de_many: str, de_verb: str, suffix_en: str = "", suffix_de: str = "", args: str = "") -> tuple[str, str]:
    tail = f", {args}" if args else ""
    one = f'1 => protocol::LocalizedLabel::native(&format!("{en_verb} 1 {en_one}{suffix_en}"{tail}), &format!("1 {de_one}{suffix_de} {de_verb}"{tail}))' if args else f'1 => protocol::LocalizedLabel::native("{en_verb} 1 {en_one}{suffix_en}", "1 {de_one}{suffix_de} {de_verb}")'
    many = f'count => protocol::LocalizedLabel::native(&format!("{en_verb} {{count}} {en_many}{suffix_en}"{tail}), &format!("{{count}} {de_many}{suffix_de} {de_verb}"{tail}))'
    return one, many


def plural_block(count: str, one: str, many: str) -> str:
    return f"match {count} {{\n            {one},\n            {many},\n        }}"


EDITS: list[tuple[str, str, str]] = []
for leaf, (en, de) in APPLY_LEAVES.items():
    EDITS.append((f"{BASE}/{leaf}/🦀️.rs", f'protocol::LocalizedLabel::native("{en}", "{de}")', DELEGATE))

PLURALS = [
    ("✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widgets/🦀️.rs",
     'protocol::LocalizedLabel::native(&format!("Move {} widget(s)", self.entries.len()), &format!("{} Widget(s) verschieben", self.entries.len()))',
     "self.entries.len()", ("Move", "widget", "widgets", "Widget", "Widgets", "verschieben")),
    ("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚move-objects/🦀️.rs",
     'protocol::LocalizedLabel::native(&format!("Move {} object(s)", self.placements.len()), &format!("{} Objekt(s) verschieben", self.placements.len()))',
     "self.placements.len()", ("Move", "object", "objects", "Objekt", "Objekte", "verschieben")),
]


def read(path: str) -> str:
    return open(os.path.join(ROOT, path), encoding="utf-8").read()


def discover_plural(path: str, en_word: str, de_word: str) -> tuple[str, str]:
    text = read(path)
    start = text.index("protocol::LocalizedLabel::native(&format!(")
    end = text.index("\n", start)
    return text[start:end].rstrip(), text


for rel, en_verb, en_one, en_many, de_one, de_many, de_verb in [
    ("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️scale-objects/🦀️.rs", "Scale", "object", "objects", "Objekt", "Objekte", "skalieren"),
    ("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-objects/🦀️.rs", "Rotate", "object", "objects", "Objekt", "Objekte", "drehen"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🦀️.rs", "Drag", "node", "nodes", "Knoten", "Knoten", "ziehen"),
    ("✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↕️scale-assets/🦀️.rs", "Scale", "asset", "assets", "Asset", "Assets", "skalieren"),
    ("✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-assets/🦀️.rs", "Rotate", "asset", "assets", "Asset", "Assets", "drehen"),
    ("✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️drag-assets/🦀️.rs", "Drag", "asset", "assets", "Asset", "Assets", "ziehen"),
]:
    line, text = discover_plural(rel, en_one, de_one)
    count = line.split(f'{en_one}(s)", ', 1)[1].split(")", 1)[0] + ")"
    PLURALS.append((rel, line, count, (en_verb, en_one, en_many, de_one, de_many, de_verb)))

for rel, anchor, count, (en_verb, en_one, en_many, de_one, de_many, de_verb) in PLURALS:
    one, many = plural(count, en_verb, en_one, en_many, de_one, de_many, de_verb)
    EDITS.append((rel, anchor, plural_block(count, one, many)))

GROUP = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🧷group-nodes/🦀️.rs"
EDITS.append((
    GROUP,
    'protocol::LocalizedLabel::native(&format!("Group {} node(s) in layer #{}", self.indices.len(), self.parent.layer), &format!("Gruppe {} Knoten(s) in Ebene #{}", self.indices.len(), self.parent.layer))',
    plural_block(
        "self.indices.len()",
        '1 => protocol::LocalizedLabel::native(&format!("Group 1 node in layer #{}", self.parent.layer), &format!("1 Knoten in Ebene #{} gruppieren", self.parent.layer))',
        'count => protocol::LocalizedLabel::native(&format!("Group {count} nodes in layer #{}", self.parent.layer), &format!("{count} Knoten in Ebene #{} gruppieren", self.parent.layer))',
    ),
))
SLOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🦀️.rs"
EDITS.append((SLOT, '                count => format!("Set {count} slot child reference(s)"),', '                1 => "Set 1 slot child reference".to_string(),\n                count => format!("Set {count} slot child references"),'))
EDITS.append((SLOT, '                count => format!("{count} Slot-Kindverweis(e) deklarieren"),', '                1 => "1 Slot-Kindverweis deklarieren".to_string(),\n                count => format!("{count} Slot-Kindverweise deklarieren"),'))

pending: dict[str, str] = {}
errors = []
for rel, anchor, replacement in EDITS:
    text = pending.get(rel) or read(rel)
    hits = text.count(anchor)
    if hits != 1:
        errors.append(f"{rel}: anchor found {hits}× — {anchor[:90]}")
        continue
    pending[rel] = text.replace(anchor, replacement)
if errors:
    print("\n".join(errors))
    raise SystemExit(1)
for rel, text in pending.items():
    if APPLY:
        open(os.path.join(ROOT, rel), "w", encoding="utf-8").write(text)
    print(("wrote " if APPLY else "would write ") + rel)
