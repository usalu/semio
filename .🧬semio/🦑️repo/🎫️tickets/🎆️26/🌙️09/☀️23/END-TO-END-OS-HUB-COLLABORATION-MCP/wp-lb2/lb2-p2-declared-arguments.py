#!/usr/bin/env python3
"""🔑️ LB2 prepared patch p2 — the bridge law proves every declared verb reads only the arguments it declares.

LB item 4: T12's census (`wp-lb/undeclared-args-census.py`, text match on `args` inside an arm) flagged 519 candidates in
27 plugins and over-counts (note: 25 flagged, 11 real). The sound predicate is behavioural: a verb's command must be a
function of its DECLARED arguments only — `effective_action_args` drops an undeclared staged key whenever the verb declares
any argument, so a bridge that reads one silently runs on its default in every shell and agent lane. The SDK's
`artifact_app_laws::assert_declared_actions_bridge_to_commands` (21 plugins call it) now also runs
`declared_verbs_reading_undeclared_arguments`: base = the staged declared defaults; probe = each key of
`UNDECLARED_ARGUMENT_VOCABULARY` ∪ every key the app declares anywhere, added under five value shapes (text, integer,
boolean, list, object); a bridge answering differently (other encoded command, refusal ↔ command) reads that key. The
framework-owned verb list moves into one const shared by both laws (its rationale leaves the loop body for a docstring).
Every registered app is covered, not only the 15 apps with a hand-written bridge-law call: `ArtifactCodecTableV1` (the
per-app table of type-erased answers every `PluginBuilder`/`declare_artifact` registration already records) gains
`command_bridge` (`app_command_bridge::<A>`: `command_from_action` resolved on its first poll + `OpBinary` encoding), and
`assert_every_registered_app_reads_only_declared_arguments(plugin)` probes every app a plugin registers through it — one
call per plugin (`wp-lb2/lb2-p2-plugin-calls.py`).

Usage: lb2-p2-declared-arguments.py [--dry-run | --write] [--root <repo-or-overlay root>]"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"

OLD = '''        /// 🧪️ Every declared app action must bridge through `command_from_action` and round-trip `command_id`.
        pub async fn assert_declared_actions_bridge_to_commands<A: ArtifactApp + Default>(manifest: fn() -> App) {
            use semio_framework::{effective_action_args, DslValue};
            let definition = manifest().definition;
            let _app = A::default();
            let skip = [
                "undo",
                "redo",
                "commitCheckpoint",
                "createAlternative",
                "switchAlternative",
                "checkoutCheckpoint",
                "copy",
                "cut",
                "paste",
                "revertToCommand",
                "setHistoryCommandFilter",
                "noteShellCommand",
                "recordTutorial",
                "startIntroduction",
                "startTutorial",
                "setActiveUtility",
                "setActiveTool",
                "interactionSelect",
                "interactionHover",
                "clearSelection",
                "selectAll",
                "setSelectionMode",
                "setInteractionGranularity",
            ];
            for action in definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)) {
                if skip.contains(&action.id.as_str()) || crate::is_tool_run_action_id(&action.id) {
                    continue;
                }
                // 🧱️ Window-KIT catalog rows are framework-owned surface verbs, not app actions: the shared
                // `window_kind_definition` scaffold below mints them (and stamps them `Migrated`) for the
                // editable variants of `TextWindowKit`/`TableWindowKit`/`TreeWindowKit`, so every app that
                // composes one of those kits inherits an id it never authored a command row for. They belong
                // in this skip list for exactly the same reason the framework-reserved verbs above do.
                if matches!(action.id.as_str(), "replace-text" | "set-cell" | "set-node") {
                    continue;
                }
                let empty_args = DslValue::Object(Vec::new());
                let staged = effective_action_args(&action.args, &empty_args, None);
                let command = A::command_from_action(&action.id, Some(&staged)).await.unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
                assert_eq!(A::command_id(&command).await, action.id.as_str(), "command_id mismatch for action {}", action.id);
            }
        }
'''

NEW = '''        /// 🧱️ Verbs the framework owns rather than the app, which the app-level bridge laws skip: the reserved
        /// history, clipboard, tutorial, utility and selection verbs, plus the window-KIT catalog rows the shared
        /// `window_kind_definition` scaffold mints (and stamps `Migrated`) for the editable variants of
        /// `TextWindowKit`/`TableWindowKit`/`TreeWindowKit` — every app composing one of those kits inherits an id it
        /// never authored a command row for.
        const FRAMEWORK_OWNED_VERBS: &[&str] = &[
            "undo",
            "redo",
            "commitCheckpoint",
            "createAlternative",
            "switchAlternative",
            "checkoutCheckpoint",
            "copy",
            "cut",
            "paste",
            "revertToCommand",
            "setHistoryCommandFilter",
            "noteShellCommand",
            "recordTutorial",
            "startIntroduction",
            "startTutorial",
            "setActiveUtility",
            "setActiveTool",
            "interactionSelect",
            "interactionHover",
            "clearSelection",
            "selectAll",
            "setSelectionMode",
            "setInteractionGranularity",
            "replace-text",
            "set-cell",
            "set-node",
        ];

        /// 🧪️ Every declared app action must bridge through `command_from_action`, round-trip `command_id`, and read
        /// only the arguments it declares ([`declared_verbs_reading_undeclared_arguments`]).
        pub async fn assert_declared_actions_bridge_to_commands<A: ArtifactApp + Default>(manifest: fn() -> App) {
            use semio_framework::{effective_action_args, DslValue};
            let definition = manifest().definition;
            let _app = A::default();
            for action in definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)) {
                if FRAMEWORK_OWNED_VERBS.contains(&action.id.as_str()) || crate::is_tool_run_action_id(&action.id) {
                    continue;
                }
                let empty_args = DslValue::Object(Vec::new());
                let staged = effective_action_args(&action.args, &empty_args, None);
                let command = A::command_from_action(&action.id, Some(&staged)).await.unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
                assert_eq!(A::command_id(&command).await, action.id.as_str(), "command_id mismatch for action {}", action.id);
            }
            let reads = declared_verbs_reading_undeclared_arguments::<A>(&definition);
            assert!(reads.is_empty(), "{} undeclared argument read(s) in {}:\\n{}", reads.len(), definition.id, reads.iter().map(ToString::to_string).collect::<Vec<_>>().join("\\n"));
        }

        /// 🔑️ One argument a declared verb's bridge reads without declaring it — see [`undeclared_argument_reads`].
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UndeclaredArgumentRead {
            pub verb: String,
            pub argument: String,
        }

        impl std::fmt::Display for UndeclaredArgumentRead {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{} reads undeclared argument `{}`", self.verb, self.argument)
            }
        }

        /// 🔑️ Argument keys the framework's kits, panels, dialogs and shells bind or seed onto an action
        /// (selections, pointers, grid cells, coordinates, values, texts, ids of the row a control sits on) — the
        /// probe vocabulary [`undeclared_argument_reads`] adds to every key the app itself declares.
        pub const UNDECLARED_ARGUMENT_VOCABULARY: &[&str] = &[
            "value", "values", "text", "label", "name", "title", "description", "kind", "key", "id", "ids", "index", "indices", "path", "pathChunks",
            "from", "to", "source", "target", "targets", "row", "column", "x", "y", "z", "position", "offset", "count", "delta", "mode", "color",
            "width", "height", "size", "url", "query", "selection", "entity", "flag", "domainId", "merge", "method", "exampleId", "nodeId", "blockId",
            "elementId", "layerId", "spaceId", "artifactId", "checkpointId", "entrySeq", "enabled", "visible", "locked",
        ];

        /// 🔑️ Every declared verb of `definition` — window-claimed and app-level, the [`FRAMEWORK_OWNED_VERBS`] and
        /// tool-run verbs excepted — whose `bridge` reads an argument it does not declare. Base: the staged declared
        /// defaults (`effective_action_args`, exactly what a shell or agent dispatches — an undeclared staged key is
        /// dropped there whenever the verb declares any argument). Probe: each key of
        /// [`UNDECLARED_ARGUMENT_VOCABULARY`] ∪ every argument key any action of `definition` declares, which the verb
        /// itself does not declare, added under five value shapes (text, integer, boolean, list, object) — a bridge
        /// that reads the key under any shape answers differently (another encoded command, or a refusal where the
        /// base bridged, or the reverse). A bridge whose two staged answers differ is not a pure function of its
        /// arguments and is reported as argument `*nondeterministic*`. Soundness: every reported read is observed,
        /// never inferred from source text; completeness is relative to the vocabulary.
        pub fn undeclared_argument_reads(definition: &semio_framework::AppDefinition, bridge: super::AppCommandBridge) -> Vec<UndeclaredArgumentRead> {
            use semio_framework::{effective_action_args, DslValue};
            let declared_anywhere = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).flat_map(|action| action.args.iter().map(|argument| argument.id.as_str()));
            let vocabulary = UNDECLARED_ARGUMENT_VOCABULARY.iter().copied().chain(declared_anywhere).collect::<std::collections::BTreeSet<&str>>();
            let shapes = [serde_json::json!("undeclared-probe"), serde_json::json!(7), serde_json::json!(true), serde_json::json!(["undeclared-probe"]), serde_json::json!({ "id": "undeclared-probe" })].map(|shape| DslValue::from(&shape));
            let mut probed = std::collections::BTreeSet::new();
            let mut reads = Vec::new();
            for action in definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(definition, window)).chain(definition.actions.iter()) {
                if !probed.insert(action.id.as_str()) || FRAMEWORK_OWNED_VERBS.contains(&action.id.as_str()) || crate::is_tool_run_action_id(&action.id) {
                    continue;
                }
                let staged = effective_action_args(&action.args, &DslValue::Object(Vec::new()), None);
                let base = bridge(&action.id, Some(&staged));
                if bridge(&action.id, Some(&staged)) != base {
                    reads.push(UndeclaredArgumentRead { verb: action.id.clone(), argument: "*nondeterministic*".into() });
                    continue;
                }
                for key in vocabulary.iter().filter(|key| !action.args.iter().any(|argument| argument.id == **key)) {
                    for shape in &shapes {
                        let DslValue::Object(mut entries) = staged.clone() else { break };
                        entries.push(((*key).to_owned(), shape.clone()));
                        if bridge(&action.id, Some(&DslValue::Object(entries))) != base {
                            reads.push(UndeclaredArgumentRead { verb: action.id.clone(), argument: (*key).to_owned() });
                            break;
                        }
                    }
                }
            }
            reads
        }

        /// 🔑️ [`undeclared_argument_reads`] of the app type `A`.
        pub fn declared_verbs_reading_undeclared_arguments<A: ArtifactApp>(definition: &semio_framework::AppDefinition) -> Vec<UndeclaredArgumentRead> {
            undeclared_argument_reads(definition, super::app_command_bridge::<A>)
        }

        /// 🔑️ Every app `plugin` registers — editors and viewers of every declared subset, and every `PluginBuilder`
        /// app — reads only the arguments its verbs declare ([`undeclared_argument_reads`] through the app's
        /// `ArtifactCodecTableV1::command_bridge`), so one call per plugin covers its whole fleet. Answers the
        /// number of apps probed; a caller pins it, so a plugin that stops registering apps cannot turn the law vacuous.
        pub fn assert_every_registered_app_reads_only_declared_arguments<PA: PluginApp>(plugin: fn() -> Result<super::Plugin<PA>, super::PluginAssemblyError>) -> usize {
            let plugin = plugin().unwrap_or_else(|error| panic!("the plugin assembles: {} {}", error.code, error.message));
            let mut findings = Vec::new();
            let mut probed = 0;
            for definition in &plugin.manifest.apps {
                let Some(codec) = plugin.app_codec(&definition.id) else { continue };
                probed += 1;
                findings.extend(undeclared_argument_reads(definition, codec.command_bridge).into_iter().map(|read| format!("{}: {read}", definition.id)));
            }
            assert!(findings.is_empty(), "{} undeclared argument read(s) over {probed} app(s):\\n{}", findings.len(), findings.join("\\n"));
            probed
        }
'''

TABLE_OLD = """        // 🚫️async: E4 fn-pointer slot
        pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactPackFiles>,
    }
"""
TABLE_NEW = """        // 🚫️async: E4 fn-pointer slot
        pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactPackFiles>,
        /// 🔌️ The app's command bridge, type-erased — see [`AppCommandBridge`].
        pub command_bridge: AppCommandBridge,
    }

    /// 🔌️ One registered app's `command_from_action` as a plain function of its type: the bridge resolved on its first
    /// poll (parsing arguments never suspends) and encoded with the command's own `OpBinary`, a refusal as its fault
    /// code. What `artifact_app_laws::assert_every_registered_app_reads_only_declared_arguments` probes for every app a
    /// plugin registers, without naming its type.
    pub type AppCommandBridge = fn(&str, Option<&semio_framework::DslValue>) -> Result<Vec<u8>, String>;

    /// 🔌️ The [`AppCommandBridge`] of `A`.
    pub fn app_command_bridge<A: ArtifactApp>(action: &str, args: Option<&semio_framework::DslValue>) -> Result<Vec<u8>, String> {
        let command = semio_framework::resolve_ready(A::command_from_action(action, args)).map_err(|fault| fault.code.0.clone())?;
        ::protocol::OpBinary::encode_op(&command).map_err(|error| error.to_string())
    }
"""
CTOR_OLD = """        ArtifactCodecTableV1 { pack_schema_hash: pack_schema_hash::<A>, genesis: genesis::<A>, print_mirror: print_mirror::<A>, apply_ops: apply_ops::<A>, replay_envelopes: replay_envelopes::<A> }
"""
CTOR_NEW = """        ArtifactCodecTableV1 { pack_schema_hash: pack_schema_hash::<A>, genesis: genesis::<A>, print_mirror: print_mirror::<A>, apply_ops: apply_ops::<A>, replay_envelopes: replay_envelopes::<A>, command_bridge: app_command_bridge::<A> }
"""

problems = []
path = ROOT / SDK
current = path.read_text(encoding="utf-8")
if "pub async fn declared_verbs_reading_undeclared_arguments" in current:
    print("already applied")
    sys.exit(0)
for label, old in [("bridge law block", OLD), ("codec table struct tail", TABLE_OLD), ("codec table constructor", CTOR_OLD)]:
    count = current.count(old)
    if count != 1:
        problems.append(f"{SDK}: expected 1× the {label}, found {count}")
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={0 if problems else 1} hunks=3 problems={len(problems)} write={WRITE}")
if WRITE and not problems:
    path.write_text(current.replace(OLD, NEW, 1).replace(TABLE_OLD, TABLE_NEW, 1).replace(CTOR_OLD, CTOR_NEW, 1), encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)
