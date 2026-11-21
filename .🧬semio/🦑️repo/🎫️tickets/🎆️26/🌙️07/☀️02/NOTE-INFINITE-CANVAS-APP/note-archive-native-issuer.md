# Note Archive Replay Producer

The browser mounted the Note editor and created an import instance, which completed activation, native open and acknowledgement. Import then failed with `plugin.internal.document-archive-replacement.initializer-failed`. The nested diagnostic named the missing original installed native semantic producer. The failure occurred while initializing the replacement from archive history, before it could publish a candidate store.

The Note editor and viewer previously inherited the default retirement-only document owner catalog. The editor's live one-item publication already uses `ColdMutationApplyFactory<NoteSnapshot, NoteMutation>`, but its separate archive initializer could not find that native replay interface in the catalog. Both surfaces now quote and admit one concrete Note cold semantic producer using the existing shared semantic source helpers. The catalog attaches the one-item publication and replay ports to the same allocation. The current native fold keeps its own reference and custody through the assembled store's existing allocation, cancellation and retirement steps.

## Verification

- `repo tools exec --light bun -- test './✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🧪️tests/archive-issuer.test.ts'` passed both focused tests and all 16 assertions. SQLite independently checked the source and port projection and denial of all four short grant axes in the neutral fixture.
- `git diff --check` through the repository tool passed on the three production files.
- The companion Note Rust law checks the exact source quote against the receipt, one physical producer, and cancellation at three partially initialized catalog frontiers. Its native execution was deferred while attempt 52 held shared compiler capacity.
- The new guest must be staged before checking archive import, history replay and visible restoration in Chromium.

## Changed Files

- [Shared Note owner catalog](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🦀️.rs)
- [Editor declaration](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs)
- [Viewer declaration](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs)
- [Neutral fixture](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔣️.json)
- [Rust owner law](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🧪️tests/🦀️.rs)
- [Focused wiring and SQLite oracle](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🧪️tests/archive-issuer.test.ts)
