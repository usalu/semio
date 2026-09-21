# Flow12 Composed Document Text Round-Trip Audit

## Scope and evidence

This is a read-only audit of the two Flow12 failures reported in
`🗑️generated/astra-runtime/flow-native12-owner-batch/run.log`:

- `flow_document_text_round_trips_store_with_applied_operation`;
- `duplicate_widget_composite_round_trips_through_op_codecs_and_a_real_store_dispatch`.

The receipt is **3/9 passed, 6/9 failed**.  Both target failures stop at
`store/🦀️.rs:22108`, whose required law is exact equality between the parsed
snapshot and the store's current snapshot.  The receipt shows differing
`flow-content-sha256-*` child ids, not a history parse error.  This audit did
not run a test.

## Confirmed cause

The two tests construct a parent-only `ArtifactStore` using
`plain_test_store`:

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs:24-30`;
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs:38-60`.

They then call the generic parent-only text helper.  That helper writes an
envelope with `print_document_text`, parses it with `parse_document_text`, and
requires `parsed.snapshot == live` at
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22099-22109`.

Flow's actual graph is an owned `content` child:

- `FlowContentChild` is `ArtifactChild<SemioFlowSnapshot>` at
  `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:53-56`;
- `FlowWorkingScene` is the child-local widgets/synapses/layout owner at
  `:243-253`;
- `flow_content_child_handle_and_cache` gives that handle its local owner at
  `:294-306`.

`ArtifactChild` intentionally excludes that local owner from durable identity:
its documentation says it is absent from equality, DSL, pack, and JSON identity
at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2893-2898`; its
`ToValue` emits only `childId` and `target` at `:3033-3040`; and `FromValue`
always restores `local_owner: None` at `:3042-3056`.

Consequently `FlowSnapshot::parse_dsl` uses that generic `FromValue` route for
the JSON emitted by `print_dsl`
(`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:75-108`).
During replay, Flow reads the unresolved child through
`flow_working_scene_for_handle`, which returns a default empty scene when no
local owner exists (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:282-291`).
Each mutation diff then rebuilds a content child by digesting that scene; the
shared builder is `diff_replace_content` at
`.../🧬️schema/🔺️diff/📝️text/🦀️.rs:74-81`.

This is sufficient to explain the receipt: the original bare-store history
uses the live local child owner, while the text replay has an unresolved child
and mints a different `flow-content-sha256-*` id.  The observed values in the
receipt are therefore a semantic child-materialization mismatch, not a SHA
algorithm or operation-codec nondeterminism.

## Required boundary

The equality assertion must remain.  Serializing `FlowWorkingScene` into the
parent `FlowSnapshot` would violate the established `ArtifactChild` contract,
duplicate a composed child's durable bytes in its parent, and allow parent
content and child envelope state to diverge.

The appropriate persisted unit is the existing recursive archive closure:

- the production Flow document codec is
  `EditorApp<editor::flow::FlowPlayApp>` at
  `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:375-380`;
- the Flow editor declares the `content` genesis child through
  `flow_genesis_content_pack` at
  `.../✏️editor/🦀️.rs:2357-2364`;
- `PluginApp::document_archive` exports parent pack + SPR plus every owned
  member envelope, with a generation fence, at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31037-31076`;
- `begin_document_archive_load`, `poll_document_archive_load`, and
  `acknowledge_document_archive_load` are the bounded retained load lifecycle
  at `:30940-31035`.

The framework already has an actual closure oracle:
`retained_window_input_recursive_document_archive_round_trips_the_complete_owned_closure` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:1579-1666`.
It admits an archive, drives the bounded load, reads it again, and checks the
complete member closure.

## Minimal fail-first replacement law

Replace only the parent-only *document* round-trip portions of the two Flow
tests with one Flow editor-app archive law.  Preserve the existing operation
text/binary equivalence assertions and the real dispatch assertions.

1. Construct the actual `EditorApp<FlowPlayApp>` path, so the `content` member
   is opened through its registered Flow member factory.
2. Apply the existing move case, then separately create + duplicate the widget
   through the real app dispatch path.
3. Read `PluginApp::document_archive`; require exactly the parent and Flow
   `content` member closure, preserving the content child id and its member
   envelope bytes.
4. Construct a fresh actual Flow editor app.  Call
   `begin_document_archive_load`, poll only through the existing bounded API to
   `Ready`, then acknowledge the terminal owner.
5. Require exact equality of the restored parent snapshot and the source live
   snapshot.  Also require identical `content` child id/target and identical
   owned `content` member envelope bytes or decoded `SemioFlowSnapshot`.
6. Close both app owners through their normal retained close route.

This is fail-first against the current parent-only helper: removing the content
member from the archive must produce a terminal load refusal or a non-equal
closure; it must never silently pass a parent snapshot comparison.  It covers
the source of both reported SHA differences and preserves the strict semantic
assertion instead of weakening it to a projection.

## Out of scope

The remaining four Flow12 failures have different receipts.  In particular,
the two plugin helper failures at `plugin/🦀️.rs:7396` concern release-grant
accounting and have no evidence connecting them to this content-SHA mismatch.
