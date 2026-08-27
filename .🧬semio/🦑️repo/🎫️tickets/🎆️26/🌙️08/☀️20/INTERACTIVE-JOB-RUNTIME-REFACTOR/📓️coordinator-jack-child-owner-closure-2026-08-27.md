# Jack Child Owner Closure

## Scope

Removed Trinity/Jack's process-global node/edge scene map.

## Exact changes

- `JackWorkingScene` is attached to one exact `JackContentChild`.
- `jack_content_child_with_owner` replaces the old mint-and-cache API without a compatibility alias.
- `materialize_jack_content` attaches decoded/test content to one mutable child.
- All snapshot, diff, text-codec, mutation-test, and constructor call sites use exact ownership.
- Read paths resolve only the addressed child and fail soft for wire-only handles awaiting host materialization.
- A language-neutral fixture and serde_json oracle cover owner presence, exact wire identity, and owner absence after wire reconstruction.

## Validation

- Bun fixture parse and exact booleans: green.
- Removed-symbol scan in the artifact and plugin for the global/static cache APIs: green.
- `git diff --check -- '✏️s/🔌️plugins/🔱️trinity'`: green.
- Rust compiler/rustfmt validation remains queued behind Flow's exclusive compiler lease; no compiler-green claim is made here.
