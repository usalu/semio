#!/usr/bin/env python3
"""🤖️ S19 set `norm-agent` (guest, T6 round 4+): the norm kinds end to end for an agent (G12 p33-3: 12 of 14 norm kinds
`mutate=failed`, measured `s14-g12-logs/battery-p33-3/coverage-hub/coverage-hub-rows.jsonl`).

1. `setActiveExample.exampleId` published as FREE optional text: an agent following the input schema sent nothing, the
   handler read "" as the empty document = the genesis document → `SUCCEEDED head 0→0` on 11 kinds, and no agent could
   discover an example id. Now every editor publishes a REQUIRED closed choice over its own navbar roster
   (`app_surface::example_id_arg(<Editor>::examples())`, en + de labels from the example leaves).
2. An id outside the roster was a silent `Ok(no-op)` (pinned by the surface law) while the framework's own catalogue
   door refuses it (`app.example.unknown`); norm now refuses it by name (`norm.set-active-example-invalid`) —
   `roster_example_snapshot` answers `Result<D, Fault>`, en1990/din18599's text tables refuse their `_` arm too.
3. en1999's binary op protocol (`🧬️mutations/💾️binary/📡️.protocol.semio`, read by its tagged codec) still named the
   pre-hierarchy flat vocabulary (0 of 17 current kinds) → EVERY en1999 wire edit failed `malformed op tag … declares no
   record for 'change-materials'` (the one en1999 attempt that emitted ops). en1990/en1991/en1992/en1996's protocol files
   were stale the same way (28/70/27/52 kinds missing); en1990 now derives its tags from the file
   (`dsl::protocol_record::tag_u8`, as its header and iso16757 already do; wire values unchanged).
4. MCP catalog audits (os-mcp `search::long`, G12 relay): `setField`/`insertItem`/`removeItem`/`applyRemedy` declared no
   description on all 15 editors (60 findings) and `removeItem` was not destructive (15) → en + de descriptions from
   `app_surface::*_description()`, `removeItem` destructive.
Laws: surface roster law round-trips every mutation a roster example raises through `OpBinary` and refuses an unknown id;
`<family>_op_protocol_declares_every_mutation_kind` (records == `KINDS`, tags unique; file read from disk — the
language-agnostic contract, cross-checked by `s19-protocol-drift.py`); action-args law: `exampleId` is a required
select whose option values are exactly the roster ids. usage: s19-norm-agent.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import re
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
ROOT = next((arg[len("--root="):] for arg in sys.argv[1:] if arg.startswith("--root=")), lib.ROOT)

NORM = "✏️s/🔌️plugins/📕️norm"
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"
FAMILIES = {"din4108": ("🧱️din4108", "Din4108"), "din16798": ("🌬️din16798", "Din16798"), "din18599": ("⚡️din18599", "Din18599"), "en1990": ("⚖️en1990", "En1990"), "en1991": ("🏋️en1991", "En1991"), "en1992": ("🏛️en1992", "En1992"), "en1993": ("🔩️en1993", "En1993"), "en1994": ("🧩️en1994", "En1994"), "en1995": ("🪵️en1995", "En1995"), "en1996": ("🪨️en1996", "En1996"), "en1997": ("🌍️en1997", "En1997"), "en1998": ("🫨️en1998", "En1998"), "en1999": ("🪶️en1999", "En1999"), "iso16757": ("📇️iso16757", "Iso16757"), "vdi3805": ("🏭️vdi3805", "Vdi3805")}
TEXT_TABLES = ("en1990", "din18599")
STALE_PROTOCOLS = ("en1990", "en1991", "en1992", "en1996", "en1999")


def family_path(family, rel):
    return f"{NORM}/🗿️artifacts/{FAMILIES[family][0]}/{ANY}/{rel}"


def kinds_of(family):
    text = open(os.path.join(ROOT, family_path(family, "🧬️schema/🧬️mutations/🦀️.rs")), encoding="utf-8").read()
    block = re.search(r"pub const KINDS: &\[&str\] = &\[(.*?)\];", text, re.S)
    return re.findall(r'"([a-z0-9-]+)"', block.group(1))


# ── 1+2 app surface ────────────────────────────────────────────────────────────────────────────────────────────────
SURFACE = f"{NORM}/🖥️app-surface/🦀️.rs"
OLD_EXAMPLES = '''/// 🎨️ Resolves `setActiveExample` through the editor's OWN example roster — the list its navbar picker
/// offers — so every example the picker shows loads, and nothing outside it does. An empty id is the
/// empty document, an id outside the roster is `None` (the no-op a picker's stale selection needs),
/// and an example body that does not parse is a named fault, never a silent fallback.
pub fn roster_example_snapshot<D: store::ArtifactDsl + Default>(examples: Vec<ExampleSource>, example_id: &str) -> Result<Option<D>, Fault> {
    let id = example_id.trim();
    if id.is_empty() {
        return Ok(Some(D::default()));
    }
    let Some(example) = examples.into_iter().find(|example| example.id() == id) else {
        return Ok(None);
    };
    D::parse_dsl(&example.document())
        .map(Some)
        .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.set-active-example-invalid"), format!("example '{id}' does not parse: {error:?}")))
}
'''
NEW_EXAMPLES = '''/// 🎨️ Resolves `setActiveExample` through the editor's OWN example roster — the list its navbar picker
/// offers — so every example the picker shows loads, and nothing outside it does. An empty id is the
/// empty document; an id outside the roster and an example body that does not parse are named faults
/// ([`unknown_example_fault`]), never a silent no-op or fallback.
pub fn roster_example_snapshot<D: store::ArtifactDsl + Default>(examples: Vec<ExampleSource>, example_id: &str) -> Result<D, Fault> {
    let id = example_id.trim();
    if id.is_empty() {
        return Ok(D::default());
    }
    let Some(example) = examples.into_iter().find(|example| example.id() == id) else {
        return Err(unknown_example_fault(id));
    };
    D::parse_dsl(&example.document()).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.set-active-example-invalid"), format!("example '{id}' does not parse: {error:?}")))
}

/// 🚫️ The refusal of an example id outside the editor's roster — the same answer the framework's catalogue door gives
/// (`app.example.unknown`), so an agent that names a stale or invented example learns it instead of a silent success.
pub fn unknown_example_fault(id: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("norm.set-active-example-invalid"), format!("example '{id}' is not in this editor's example roster"))
}

/// 🗂️ `setActiveExample.exampleId` as the closed, required choice of the editor's own roster (value = example id,
/// label = the example leaf's en + de label), so the published input schema names every example an agent can load.
pub fn example_id_arg(examples: Vec<ExampleSource>) -> ActionArgDef {
    ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), examples.iter().map(|example| ActionArgOption::new(example.id(), example.label().clone())).collect()).required()
}

/// 💬️ `setField` as an agent reads it in the capability catalog (every norm editor declares the same value-tree verb).
pub fn set_field_description() -> LocalizedLabel {
    LocalizedLabel::native("Sets one input of the compliance document, addressed by its value-tree path such as `members[0].nEd`, to the given value.", "Setzt eine Eingabe des Nachweisdokuments, adressiert über ihren Pfad im Wertebaum wie `members[0].nEd`, auf den übergebenen Wert.")
}

/// 💬️ `insertItem` as an agent reads it in the capability catalog.
pub fn insert_item_description() -> LocalizedLabel {
    LocalizedLabel::native("Inserts one entry into a list of the compliance document at a value-tree path and index, optionally with the given value.", "Fügt an einem Pfad im Wertebaum und einer Position einen Eintrag in eine Liste des Nachweisdokuments ein, optional mit dem übergebenen Wert.")
}

/// 💬️ `removeItem` as an agent reads it in the capability catalog.
pub fn remove_item_description() -> LocalizedLabel {
    LocalizedLabel::native("Removes the entry at one index from a list of the compliance document at a value-tree path.", "Entfernt den Eintrag an einer Position aus einer Liste des Nachweisdokuments am angegebenen Pfad im Wertebaum.")
}

/// 💬️ `applyRemedy` as an agent reads it in the capability catalog.
pub fn apply_remedy_description() -> LocalizedLabel {
    LocalizedLabel::native("Applies one of the remedies a failed check proposes, changing the document inputs so that the check can pass.", "Wendet eine der Abhilfen an, die ein nicht erfüllter Nachweis vorschlägt, und ändert die Eingaben so, dass der Nachweis erfüllt werden kann.")
}
'''
IMPORT_OLD = "use semio_framework_plugin::{\n    tree_group, "
IMPORT_NEW = "use semio_framework_plugin::{\n    tree_group, ActionArgDef, ActionArgOption, "


def surface(text):
    return lib.chain(lib.replace_once(OLD_EXAMPLES, NEW_EXAMPLES), lib.replace_once(IMPORT_OLD, IMPORT_NEW))(text)


# ── handlers ───────────────────────────────────────────────────────────────────────────────────────────────────────
def roster_handler(text):
    pattern = re.compile(r"    let Some\(snapshot\) = (crate::app_surface::roster_example_snapshot\(<[^>]+>::examples\(\), &payload\.example_id\)\?) else \{\n        return Ok\(Emit::default\(\)\);\n    \};\n")
    if "let snapshot = crate::app_surface::roster_example_snapshot(" in text:
        return text
    assert len(pattern.findall(text)) == 1, "roster handler anchor"
    text = pattern.sub(lambda m: f"    let snapshot = {m.group(1)};\n", text)
    return text.replace("/// 🎨️ Replaces the live document with the named roster example, or clears it when the id is empty.", "/// 🎨️ Replaces the live document with the named roster example, or clears it when the id is empty; an id outside\n/// the roster is refused by name.")


def text_table_handler(text):
    if "crate::app_surface::unknown_example_fault(" in text:
        return text
    old = "        _ => return Ok(Emit::default()),\n"
    assert text.count(old) == 1, "text-table `_` arm"
    return text.replace(old, "        unknown => return Err(crate::app_surface::unknown_example_fault(unknown)),\n")


# ── editors ────────────────────────────────────────────────────────────────────────────────────────────────────────
DESCRIBE = """            .action_describe("setField", crate::app_surface::set_field_description())
            .action_describe("insertItem", crate::app_surface::insert_item_description())
            .action_describe("removeItem", crate::app_surface::remove_item_description())
            .action_describe("applyRemedy", crate::app_surface::apply_remedy_description())
            .action_destructive("removeItem")
"""


def editor(type_):
    def transform(text):
        new = f"crate::app_surface::example_id_arg(<{type_}PlayApp as ArtifactEditor>::examples())"
        text = lib.replace_once('semio_framework_plugin::ActionArgDef::text("exampleId", LocalizedLabel::native("Example", "Beispiel"))', new)(text)
        if DESCRIBE in text:
            return text
        lines = [line for line in text.splitlines(keepends=True) if line.startswith('            .action_describe("setActiveExample", ')]
        assert len(lines) == 1, f"setActiveExample describe line x{len(lines)}"
        return text.replace(lines[0], lines[0] + DESCRIBE)
    return transform


# ── protocols ──────────────────────────────────────────────────────────────────────────────────────────────────────
def en1990_tags():
    text = open(os.path.join(ROOT, family_path("en1990", "🧬️schema/🧬️mutations/📝️text/🦀️.rs")), encoding="utf-8").read()
    consts = dict(re.findall(r"const (TAG_[A-Z0-9_]+): u8 = (\d+);", text))
    if not consts:
        consts = {name: None for name in re.findall(r"const (TAG_[A-Z0-9_]+): u8 = dsl::protocol_record::tag_u8", text)}
    encode = re.search(r"fn encode_op\(&self\)(.*?)\n        \};", text, re.S).group(1)
    arms = re.findall(r"En1990Mutation::([A-Za-z0-9]+)\(_\) => (TAG_[A-Z0-9_]+)", encode)
    return [(re.sub(r"(?<!^)(?=[A-Z])", "-", variant).lower(), const, consts[const]) for variant, const in arms]


def protocol(family):
    def transform(text):
        head, sep, _ = text.partition("\nrecord ")
        assert sep, "protocol has no record section"
        kinds = kinds_of(family)
        if family == "en1990":
            tags = en1990_tags()
            assert sorted(kind for kind, _, _ in tags) == sorted(kinds), "en1990 encode arms != KINDS"
            current = dict(re.findall(r"^record ([a-z0-9-]+) tag=(\d+)$", text, re.M))
            pairs = sorted(((kind, int(value) if value is not None else int(current[kind])) for kind, _, value in tags), key=lambda pair: pair[1])
        else:
            pairs = list(zip(kinds, range(len(kinds))))
        return head + "\n" + "".join(f"record {kind} tag={tag}\nfield payload bytes\n" for kind, tag in pairs)
    return transform


def en1990_codec(text):
    if "const WIRE_PROTOCOL: &str" in text:
        return text
    by_const = {const: kind for kind, const, _ in en1990_tags()}
    first = "const TAG_CHANGE_ANNEX: u8 = 0;\n"
    assert text.count(first) == 1, "en1990 first TAG const"
    text = text.replace(first, 'const WIRE_PROTOCOL: &str = include_str!("../💾️binary/📡️.protocol.semio");\n' + first)
    for const, kind in by_const.items():
        old = re.compile(rf"const {const}: u8 = \d+;")
        assert len(old.findall(text)) == 1, f"en1990 {const}"
        text = old.sub(f'const {const}: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "{kind}");', text)
    return text


# ── laws ───────────────────────────────────────────────────────────────────────────────────────────────────────────
SURFACE_TESTS = f"{NORM}/🧪️tests/🔬️surface/🦀️.rs"
UNKNOWN_OLD = '''                .expect("unknown example is a no-op")
            };
            assert!(unknown.artifact_mutations.is_empty(), "{} unknown example must be a no-op", stringify!($name));
'''
UNKNOWN_NEW = '''                .err()
                .expect("an example id outside the roster is refused")
            };
            assert_eq!(unknown.code.0, "norm.set-active-example-invalid", "{} refuses an unknown example by name", stringify!($name));
'''
ROSTER_OLD = '''                assert!(
                    !emit.artifact_mutations.is_empty() || emit.description.as_deref() == Some("setSnapshot"),
                    "{}: example '{}' loaded nothing",
                    stringify!($name),
                    example.id()
                );
            }
'''
ROSTER_NEW = '''                assert!(
                    !emit.artifact_mutations.is_empty() || emit.description.as_deref() == Some("setSnapshot"),
                    "{}: example '{}' loaded nothing",
                    stringify!($name),
                    example.id()
                );
                assert_ops_round_trip(&emit.artifact_mutations).unwrap_or_else(|fault| panic!("{}: example '{}' raises an op the wire cannot carry: {fault}", stringify!($name), example.id()));
            }
'''
HELPERS_ANCHOR = "/// 📚️ The `pub const ID` of every `📚️examples/*/🦀️.rs` example module of one norm family.\n"
HELPERS = '''/// 📡️ Every op a roster example raises crosses the wire: `OpBinary` encodes it and decodes it back equal (en1999's
/// stale protocol failed `declares no record for 'change-materials'` on the hub, the first op of every example load).
fn assert_ops_round_trip<M: semio_framework_os_kernel::OpBinary + PartialEq + std::fmt::Debug>(ops: &[M]) -> Result<(), String> {
    for op in ops {
        let bytes = op.encode_op().map_err(|error| format!("{op:?} does not encode: {error}"))?;
        let decoded = M::decode_op(&bytes).map_err(|error| format!("{op:?} does not decode: {error}"))?;
        if &decoded != op {
            return Err(format!("{op:?} decodes to {decoded:?}"));
        }
    }
    Ok(())
}

/// 📡️ LAW: a family's binary op protocol (`🧬️mutations/💾️binary/📡️.protocol.semio`, the language-agnostic wire contract)
/// declares exactly one record per mutation kind of `KINDS`, each at its own tag — a kind without a record cannot
/// cross the wire, a record without a kind is a vocabulary the codec no longer speaks.
macro_rules! op_protocol_case {
    ($name:ident, $kinds:expr, $family_dir:literal) => {
        #[test]
        fn $name() {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts").join($family_dir).join("🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio");
            let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let records: Vec<(String, u64)> = text
                .lines()
                .filter_map(|line| line.strip_prefix("record "))
                .map(|rest| {
                    let (kind, tag) = rest.split_once(" tag=").unwrap_or_else(|| panic!("{}: malformed record line {rest:?}", stringify!($name)));
                    (kind.to_string(), tag.trim().parse().unwrap_or_else(|_| panic!("{}: malformed tag {rest:?}", stringify!($name))))
                })
                .collect();
            let mut declared: Vec<&str> = records.iter().map(|(kind, _)| kind.as_str()).collect();
            declared.sort_unstable();
            let mut kinds: Vec<&str> = $kinds.to_vec();
            kinds.sort_unstable();
            assert_eq!(declared, kinds, "{}: the op protocol must declare exactly the mutation kinds", stringify!($name));
            let mut tags: Vec<u64> = records.iter().map(|(_, tag)| *tag).collect();
            tags.sort_unstable();
            tags.dedup();
            assert_eq!(tags.len(), records.len(), "{}: every record carries its own tag", stringify!($name));
        }
    };
}

'''
OP_CASES_ANCHOR = "//#endregion 🗂️RosterExamples\n"


def surface_tests(text):
    text = lib.replace_once(UNKNOWN_OLD, UNKNOWN_NEW)(text)
    text = lib.replace_once(ROSTER_OLD, ROSTER_NEW)(text)
    if "macro_rules! op_protocol_case" not in text:
        assert text.count(HELPERS_ANCHOR) == 1, "surface helpers anchor"
        text = text.replace(HELPERS_ANCHOR, HELPERS + HELPERS_ANCHOR)
    cases = "".join(f'op_protocol_case!({family}_op_protocol_declares_every_mutation_kind, semio_s_artifact_norm_{family}::artifact_schema::mutations::KINDS, "{FAMILIES[family][0]}");\n' for family in FAMILIES)
    if cases not in text:
        assert text.count(OP_CASES_ANCHOR) == 1, "roster region end"
        text = text.replace(OP_CASES_ANCHOR, cases + OP_CASES_ANCHOR)
    return text


ARGS_TESTS = f"{NORM}/🧪️tests/🎯️action-args/🦀️.rs"
ARGS_IMPORT_OLD = "use semio_framework_plugin::{ActionArgDef, AppDefinition, ArgSchema};\n"
ARGS_IMPORT_NEW = "use semio_framework_plugin::{ActionArgDef, AppDefinition, ArgSchema, ArtifactEditor, ExampleSource};\n"
ARGS_HELPER_ANCHOR = "fn assert_norm_value_tree_arguments(definition: AppDefinition) {\n"
ARGS_HELPER = '''/// 🗂️ `setActiveExample.exampleId` is the required closed choice of the editor's own roster — an agent reads every
/// loadable example id off the published input schema (measured over MCP: free optional text, sent empty → the
/// genesis document again, `head 0→0` on 11 kinds).
fn assert_example_argument(definition: &AppDefinition, roster: Vec<ExampleSource>) {
    let declared = argument(definition, "setActiveExample", "exampleId");
    let ArgSchema::String { options, .. } = &declared.schema else { panic!("{} setActiveExample.exampleId must be a choice", definition.id) };
    assert!(declared.required, "{} setActiveExample.exampleId must be required", definition.id);
    let offered: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
    let rostered: Vec<&str> = roster.iter().map(ExampleSource::id).collect();
    assert!(!rostered.is_empty(), "{} offers no example", definition.id);
    assert_eq!(offered, rostered, "{} setActiveExample.exampleId must offer exactly the roster", definition.id);
}

'''


def args_tests(text):
    text = lib.replace_once(ARGS_IMPORT_OLD, ARGS_IMPORT_NEW)(text)
    if "fn assert_example_argument(" not in text:
        assert text.count(ARGS_HELPER_ANCHOR) == 1, "action-args helper anchor"
        text = text.replace(ARGS_HELPER_ANCHOR, ARGS_HELPER + ARGS_HELPER_ANCHOR)
    for family, (_, type_) in FAMILIES.items():
        old = f"    assert_norm_value_tree_arguments(semio_s_artifact_norm_{family}::editor::{family}::create_{family}_app());\n"
        new = f"    let definition = semio_s_artifact_norm_{family}::editor::{family}::create_{family}_app();\n    assert_example_argument(&definition, <semio_s_artifact_norm_{family}::editor::{family}::{type_}PlayApp as ArtifactEditor>::examples());\n    assert_norm_value_tree_arguments(definition);\n"
        text = lib.replace_once(old, new)(text)
    return text


EDITS = [(SURFACE, surface)]
for family, (directory, type_) in FAMILIES.items():
    handler = family_path(family, "✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs")
    EDITS.append((handler, text_table_handler if family in TEXT_TABLES else roster_handler))
    EDITS.append((family_path(family, "✏️editor/🦀️.rs"), editor(type_)))
for family in STALE_PROTOCOLS:
    EDITS.append((family_path(family, "🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio"), protocol(family)))
EDITS.append((family_path("en1990", "🧬️schema/🧬️mutations/📝️text/🦀️.rs"), en1990_codec))
EDITS.append((SURFACE_TESTS, surface_tests))
EDITS.append((ARGS_TESTS, args_tests))
lib.run("norm-agent", EDITS, sys.argv[1:])
