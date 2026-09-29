#!/usr/bin/env python3
"""📚️ LB2 p18 (stdio): an example that no longer parses is a typed refusal, never an empty document.

HANDED OVER to slice EX1 (coordinator, 2026-09-29 20:0x): EX1 owns every example-loader fallback with one SDK resolver
`semio_framework_plugin::example_snapshot::<S>(&examples, id)` and the framework codes `app.example.unknown` /
`app.example.unreadable`. This script is EX1's input; its stdio-local helper and `stdio.example.*` codes are superseded.

62 stdio editors load their curated example with `parse_dsl(PRIMARY_TEXT).unwrap_or_default()` and open `Snapshot::default()`
for every other id. A stale asset (xml's old 5-field wire, 14c) therefore opened an EMPTY document silently, and the empty id
the shell sends for "the app's own default document" opened `Default` instead of the editor's genesis document (a 0x0 png, an
invalid `valid`-subset xml; p11 made `initial_snapshot()` the fixed point). AGENTS.md: no fallbacks.

Fix (one rule, in the stdio contract, beside `load_example_effect`):
- `semio_s_artifact_stdio_contract::example_snapshot(example_id, (ID, PRIMARY_TEXT), genesis)`: the empty id opens `genesis`
  (the editor's own `ArtifactEditor::initial_snapshot`); the subset's example id opens its asset parsed by the snapshot's DSL,
  an unreadable asset is refused `stdio.example.unreadable`; every other id is refused `stdio.example.unknown`.
- every `<editor>_example_snapshot` answers `Result<Snapshot, Fault>` through it; `setActiveExample` propagates the refusal (`?`).
- law (`🧪️tests/✏️editor-catalog`, every shipped editor): every example it publishes parses as its snapshot and is a pack
  fixed point — the stale-asset detector the silent fallback hid.
- the 11 editor unit-test files that call a loader `expect` the demo; the 9 that pinned the old fallback pin genesis and the
  refusal code instead.

usage: python3 lb2-p18-example-refusal.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p18/<root-hash>/`.
"""
import hashlib, os, re, shutil, subprocess, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p18/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
CONTRACT = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs"
problems = []


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


HELPER = '''
/// 📚️ The document one named example of an editor opens. The empty id — what the shell sends for "the app's own default
/// document" — opens `genesis` (the editor's `ArtifactEditor::initial_snapshot`); the subset's example id opens its curated
/// asset, parsed by the snapshot's own DSL. An asset that no longer parses is refused as `stdio.example.unreadable` and
/// every other id as `stdio.example.unknown`: the editor never opens an empty document in place of the one it was asked for.
pub fn example_snapshot<S: kernel::ArtifactDsl>(example_id: &str, example: (&str, &str), genesis: fn() -> S) -> Result<S, semio_framework_plugin::Fault> {
    let (id, text) = example;
    if example_id.is_empty() {
        return Ok(genesis());
    }
    if example_id != id {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.example.unknown"), format!("example {example_id:?} is not published by this editor, which publishes {id:?}")));
    }
    S::parse_dsl(text).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.example.unreadable"), format!("example {id:?} no longer parses as its document: {error}")))
}
'''


def contract(text):
    anchor = "    semio_framework_plugin::Effect::LoadDocument { pack, spr }\n}\n"
    if "pub fn example_snapshot<" in text:
        problems.append(f"{CONTRACT}: example_snapshot already declared")
        return text
    return once(text, anchor, anchor + HELPER, "contract: example_snapshot")


LOADER = re.compile(
    r"(?P<doc>(?:[ \t]*///[^\n]*\n)*)(?P<tag>[ \t]*// 🚫️async:[^\n]*\n)?"
    r"fn (?P<name>\w+_example_snapshot)\(example_id: &str\) -> (?P<snap>\w+) \{\s*"
    r"if example_id == (?P<mod>[\w:]+)::ID \{\s*<(?P=snap) as store::ArtifactDsl>::parse_dsl\((?P=mod)::PRIMARY_TEXT\)\.unwrap_or_default\(\)\s*"
    r"\} else \{\s*(?P=snap)::default\(\)\s*\}\s*\}\n"
)
DOC = (
    "/// 📚️ The document a named example opens ([`semio_s_artifact_stdio_contract::example_snapshot`]): the empty id opens this\n"
    "/// editor's genesis document, the subset's one example its curated asset; an unknown id or an unreadable asset is refused.\n"
)


def loader_files():
    found = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "_example_snapshot(example_id: &str) -> ", ART], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
    return sorted(path for path in found if path)


def editor_of(text, path):
    editors = re.findall(r"impl ArtifactEditor for (\w+) \{", text)
    if len(editors) != 1:
        problems.append(f"{path}: {len(editors)} ArtifactEditor impls")
        return None
    return editors[0]


def loader(text, path, names):
    editor = editor_of(text, path)
    found = list(LOADER.finditer(text))
    if len(found) != 1 or editor is None:
        problems.append(f"{path}: {len(found)} loader fns")
        return text
    match = found[0]
    names[path] = (match["name"], match["snap"], editor)
    replacement = (
        DOC
        + (match["tag"] or "")
        + f"fn {match['name']}(example_id: &str) -> Result<{match['snap']}, Fault> {{\n"
        + f"    semio_s_artifact_stdio_contract::example_snapshot(example_id, ({match['mod']}::ID, {match['mod']}::PRIMARY_TEXT), <{editor} as ArtifactEditor>::initial_snapshot)\n"
        + "}\n"
    )
    text = text[: match.start()] + replacement + text[match.end() :]
    call = f"load_example_effect(&{match['name']}(example_id), "
    if text.count(call) < 1:
        problems.append(f"{path}: no setActiveExample call of {match['name']}")
    return text.replace(call, f"load_example_effect(&{match['name']}(example_id)?, ")


def unit_test(text, path, name, snap, editor):
    text = text.replace(
        f'    assert_eq!({name}(""), {snap}::default());\n',
        f'    assert_eq!({name}("").expect("the empty id opens the genesis document"), <{editor} as ArtifactEditor>::initial_snapshot());\n',
    )
    text = text.replace(
        f'    assert_eq!({name}("no-such-example"), {snap}::default());\n',
        f'    assert_eq!({name}("no-such-example").expect_err("an unknown example id is refused").code.0, "stdio.example.unknown");\n',
    )
    if not re.search(rf"{name}\((crate::[\w:]+::ID)\)", text):
        problems.append(f"{path}: no demo call of {name}")
    text = re.sub(rf"{name}\((crate::[\w:]+::ID)\)", lambda call: f'{name}({call[1]}).expect("the published example opens")', text)
    raw = [call for call in re.finditer(rf"{name}\([^)]*\)", text) if not text[call.end() :].startswith((".expect(", ".expect_err("))]
    if raw:
        problems.append(f"{path}: {len(raw)} {name} calls neither expect nor refuse")
    return text


CATALOG = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/✏️editor-catalog/🦀️.rs"


def catalog(text):
    anchor = "    let registry = AppActionRegistry::from_definition(&definition);\n    let mut app = EditorSurfaceApp::<E>::with_registry("
    law = (
        "    for example in E::examples() {\n"
        "        let document = <E::Snapshot as semio_framework_os_kernel::ArtifactDsl>::parse_dsl(example.document()).unwrap_or_else(|error| panic!(\"{} publishes example {} that no longer parses: {error}\", definition.id, example.id()));\n"
        "        let reopened = E::Snapshot::decode_pack(&document.encode_pack()).unwrap_or_else(|error| panic!(\"{} example {} does not reopen: {error:?}\", definition.id, example.id()));\n"
        "        assert_eq!(snapshot_json(&reopened), snapshot_json(&document), \"{} example {} is a codec fixed point\", definition.id, example.id());\n"
        "    }\n"
    )
    return once(text, anchor, law + anchor, "catalog: every published example opens")


def plan():
    edits = {CONTRACT: contract, CATALOG: catalog}
    names = {}
    loaders = loader_files()
    if len(loaders) != 62:
        problems.append(f"{len(loaders)} stdio example loaders, expected 62")
    for path in loaders:
        edits[path] = lambda text, path=path: loader(text, path, names)
        loader(open(os.path.join(TREE, path), encoding="utf-8").read(), path, names)
    tests = 0
    for path, (name, snap, editor) in sorted(names.items()):
        test = path.replace("/🦀️.rs", "/🧪️tests/🔬️unit/🦀️.rs")
        if os.path.isfile(os.path.join(TREE, test)) and name + "(" in open(os.path.join(TREE, test), encoding="utf-8").read():
            tests += 1
            edits[test] = lambda text, test=test, name=name, snap=snap, editor=editor: unit_test(text, test, name, snap, editor)
    if tests != 11:
        problems.append(f"{tests} editor unit-test files call a loader, expected 11")
    return edits


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    if mode == "--revert":
        for root, _, files in os.walk(BACKUP):
            for file in files:
                source = os.path.join(root, file)
                path = os.path.relpath(source, BACKUP)
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    edits = plan()
    problems.clear()
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
