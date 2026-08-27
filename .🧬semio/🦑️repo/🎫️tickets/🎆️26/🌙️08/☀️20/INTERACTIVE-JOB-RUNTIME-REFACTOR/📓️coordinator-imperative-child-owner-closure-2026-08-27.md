# Imperative Child Owner Closure

## Scope

Removed the Imperative artifact's two process-global maps for flow programs and text seeds.

## Exact changes

- `ImperativeFlowWorkingData` and `ImperativeTextWorkingData` belong to their exact typed child handles.
- `imperative_flow_child_with_owner` and `imperative_text_child_with_owner` replace the former mint-and-cache APIs without compatibility aliases.
- `materialize_imperative_flow` attaches decoded/test programs to one mutable flow child.
- All production, engine, diff, fixture, leaf-test, and external mutation-vector call sites now use exact ownership.
- Read paths resolve only the addressed child and fail soft for wire-only handles awaiting host materialization.
- A language-neutral fixture and serde_json oracle cover both owners, exact wire identity, and absence of owner payload after wire reconstruction.

## Validation

- Bun fixture parse and exact object comparison: green.
- Removed-symbol scan across the plugin for both statics, cache APIs, and old mint-and-cache names: green.
- Every materialization site has a mutable snapshot/handle.
- `git diff --check -- '✏️s/🔌️plugins/📜️imperative'`: green.
- Rust compiler/rustfmt validation remains queued behind Flow's exclusive compiler lease; no compiler-green claim is made here.
