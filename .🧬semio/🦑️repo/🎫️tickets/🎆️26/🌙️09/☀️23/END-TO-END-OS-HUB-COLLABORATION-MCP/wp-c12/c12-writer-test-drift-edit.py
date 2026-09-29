"""🧪️ C12 14c (LW1 run `c12-nextest-1`, rule 22 test-only): the laws and fixtures c12-splice and a raster peer change left behind.
  * writer command surface: `every_command()` + the wire-keyword law gain the `textSplice` row (19 ≠ 20 rows);
  * writer migration fixture: the `textSplice` row of the factory join (`🎬️writer-migration`);
  * `WriterEditorSelection.splice` is a required field (the host's applied-splice echo): the window-state-ownership,
    partial-construction and set-editor-selection transient fixtures carry `"splice": 0`;
  * STEP 8 law: every edit a store holds — folded from the hub AND authored locally — is named after its own operation's
    mutation id (measured 17/17 on the live tree, H13's "local = entry id + #0" premise does not hold for dispatch), so the
    mixed shape is asserted by authorship (hub actors vs this replica), and the naming claim covers all 17 edits;
  * ui-scene typed-scene corpus: the `paint-2d` case carries the four fields `Paint2dScene` gained (raster brush colour,
    hardness, paint target, mask value; values of the wire-format law). `mask_value` is the corpus's first integer field:
    the neutral packet encoder writes every plain JSON number as the pack's f64 and the strict pack decoder admits no f64
    for a `u32` (run `drift-proof-1`), so the corpus names the wire integer explicitly — `{"$u64": 255}`, the same way
    `{"$some": …}` names an option tag — and the encoder writes it as the pack's u64.
Idempotent; `--dry-run`."""
import json
import re
import sys

W = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/"
MAIN = W + "✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/"
UNIT = W + "✏️editor/🧪️tests/🔬️unit/🦀️.rs"
MIGRATION = W + "✏️editor/🧫️fixtures/🎬️writer-migration/🔣️.json"
OWNERSHIP = MAIN + "🎚️config/🧫️fixtures/🔬️window-state-ownership/🔣️.json"
PARTIAL = MAIN + "🫧️transient/🧫️fixtures/🧩️partial-construction/🔣️.json"
QUINTET = MAIN + "🫧️transient/🧫️fixtures/📐️set-editor-selection/✅️set-editor-selection-applied/"
QUINTET_FILES = [QUINTET + "📸️snapshot/➡️after/🔣️.json", QUINTET + "🔺️diff/🔣️.json", QUINTET + "🦠️mutation/🔣️.json"]
SEED_LAW = W + "🚪️io/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs"
TYPED_SCENE = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/🧾️typed-scene/🔣️.json"
PACK_LAW = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️pack-unit/🦀️.rs"

SPLICE_ROW = "        WriterCommand::TextSplice(text_splice::TextSplice { start: 5, deleted: String::new(), insert: \"!\".into(), before: \"hello\".into(), after: String::new(), seq: 1, anchor: 6, caret: 6 }),\n"
TEXT_HUNKS = [
    (UNIT,
     "        WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }),\n    ]\n}\n",
     "        WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }),\n" + SPLICE_ROW + "    ]\n}\n"),
    (UNIT,
     "        (\"engagement-submit\", WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some(\"x\".into()) })),\n    ];\n",
     "        (\"engagement-submit\", WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some(\"x\".into()) })),\n"
     "        (\"text-splice\", WriterCommand::TextSplice(text_splice::TextSplice { start: 0, deleted: String::new(), insert: \"x\".into(), before: String::new(), after: String::new(), seq: 1, anchor: 1, caret: 1 })),\n    ];\n"),
    (SEED_LAW,
     "    let (remote, local) = folded.envelope().vcs.edits.iter().fold((0, 0), |(remote, local), edit| if edit.mutation_meta.first().and_then(|meta| meta.mutation_id.as_ref()).is_some_and(|id| id.0 == edit.id) { (remote + 1, local) } else { (remote, local + 1) });\n"
     "    assert_eq!((remote, local), (15, 2), \"STEP 8's mixed shape: every folded operation is an edit named after its own mutation id, the replica's own edits are local (entry id + operation id)\");\n",
     "    let hub_actors: std::collections::BTreeSet<&str> = tail.iter().map(|value| value[\"actor\"].as_str().expect(\"hub tail actor\")).collect();\n"
     "    let edits = &folded.envelope().vcs.edits;\n"
     "    let from_hub = edits.iter().filter(|edit| edit.actor.as_deref().is_some_and(|actor| hub_actors.contains(actor))).count();\n"
     "    assert_eq!((from_hub, edits.len() - from_hub), (15, 2), \"STEP 8's mixed shape: the hub's 15 operations beside this replica's own 2 edits\");\n"
     "    assert!(edits.iter().all(|edit| edit.mutation_meta.first().and_then(|meta| meta.mutation_id.as_ref()).is_some_and(|id| id.0 == edit.id)), \"every edit — folded or authored here — is named after its own operation, the shape SeedHistory must seed once\");\n"),
    (PACK_LAW,
     "                if let Some(value) = values.get(\"$some\") {\n                    bytes.push(TAG_SOME);\n                    packet(value, bytes);\n                } else {\n",
     "                if let Some(value) = values.get(\"$some\") {\n                    bytes.push(TAG_SOME);\n                    packet(value, bytes);\n                } else if let Some(value) = values.get(\"$u64\") {\n                    bytes.extend(to_bytes(&value.as_u64().unwrap()).unwrap());\n                } else {\n"),
]


def add_splice(node):
    if isinstance(node, dict):
        if set(node) == {"start", "end"} and all(isinstance(node[key], int) for key in node):
            node["splice"] = 0
        for value in node.values():
            add_splice(value)
    elif isinstance(node, list):
        for value in node:
            add_splice(value)


dry = "--dry-run" in sys.argv
writes, plan, problems = {}, [], []
for path, old, new in TEXT_HUNKS:
    text = writes.get(path) or open(path, encoding="utf-8").read()
    if new in text:
        plan.append(f"present {path.rsplit('/', 3)[-3]}")
        continue
    if text.count(old) != 1:
        problems.append((path.rsplit("/", 3)[-3], text.count(old)))
        continue
    writes[path] = text.replace(old, new)
    plan.append(f"hunk    {path.rsplit('/', 3)[-3]}")

migration = open(MIGRATION, encoding="utf-8").read()
data = json.loads(migration)
if json.dumps(data, indent=2, ensure_ascii=False) + "\n" != migration:
    problems.append(("migration-format", 0))
elif not any(row["action"] == "textSplice" for row in data["migrated"]):
    data["migrated"].append({"action": "textSplice", "variants": ["text-splice"], "publication": ["Artifact"]})
    writes[MIGRATION] = json.dumps(data, indent=2, ensure_ascii=False) + "\n"
    plan.append("fixture migration + textSplice")

for path in [PARTIAL, *QUINTET_FILES]:
    text = open(path, encoding="utf-8").read()
    value = json.loads(text)
    before = json.dumps(value, sort_keys=True)
    add_splice(value)
    if json.dumps(value, sort_keys=True) == before:
        plan.append(f"present {path.rsplit('/', 2)[-2]}")
        continue
    indent = 2 if json.dumps(json.loads(text), indent=2, ensure_ascii=False) + "\n" == text else None
    if indent is None:
        problems.append((path, "format"))
        continue
    writes[path] = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
    plan.append(f"fixture {path.rsplit('/', 2)[-2]} + splice")

ownership = open(OWNERSHIP, encoding="utf-8").read()
if '"splice": 0' not in ownership:
    patched, count = re.subn(r'\{ "start": (\d+), "end": (\d+) \}', r'{ "start": \1, "end": \2, "splice": 0 }', ownership)
    if count != 2:
        problems.append(("ownership-selections", count))
    else:
        writes[OWNERSHIP] = patched
        plan.append("fixture window-state-ownership + splice ×2")

scene = open(TYPED_SCENE, encoding="utf-8").read()
old_paint = '"brushSize":10,"brushOpacity":1,"viewMode":"composite"'
first_paint = '"brushSize":10,"brushOpacity":1,"brushColor":"#2878dc","brushHardness":1,"paintTarget":"pixels","maskValue":255,"viewMode":"composite"'
new_paint = '"brushSize":10,"brushOpacity":1,"brushColor":"#2878dc","brushHardness":1,"paintTarget":"pixels","maskValue":{"$u64":255},"viewMode":"composite"'
if first_paint in scene:
    old_paint = first_paint
if new_paint not in scene:
    if scene.count(old_paint) != 1:
        problems.append(("typed-scene paint-2d", scene.count(old_paint)))
    else:
        writes[TYPED_SCENE] = scene.replace(old_paint, new_paint)
        plan.append("fixture typed-scene paint-2d + 4 fields")

print("\n".join(plan))
print(f"{len(problems)} problems {problems}")
if problems:
    sys.exit(1)
if not dry:
    for path, text in writes.items():
        open(path, "w", encoding="utf-8").write(text)
    print(f"written {len(writes)} files")
