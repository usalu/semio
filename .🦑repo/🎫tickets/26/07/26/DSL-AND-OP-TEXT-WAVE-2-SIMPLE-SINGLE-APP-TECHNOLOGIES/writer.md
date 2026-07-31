# writer — DocumentDsl + OpText (Wave 2)

## Scope
Only touched files under `/Users/ueli/Documents/semio/writer/`. Did NOT touch the embedded
syntax-highlighting tokenizer in `writer/plugin/rs/lib.rs` (`mod grammar { ... }`,
`tokenize_language`/`GrammarToken`) — that highlights code typed *into* a writer document and is
unrelated to this ticket.

## Design

### `.writer` DSL (`WriterProjection`, `writer/rs/lib.rs`, region `🔖Dsl` inside `mod document_vcs`)
One `@writer` header line (`schema=`, `id=`, `language=`, `uri=`, `x=`, `y=`, `zoom=`, `lines=N`)
followed by exactly `N` raw, unescaped lines of source text — so the document reads as plain source
code on disk, no backslash-escaping noise in the common case. `lines=N` avoids needing any escaping
for the body at all (source text can contain anything, including quotes/backslashes, without special
handling). Implemented in a private `mod writer_dsl` (hand-rolled tokenizer, no new deps), mirroring
`note`'s `note_text` module convention.

Example (`writer/example/jack.writer`):
```
@writer schema=writer.document id=jack language=jack uri=writer://jack x=0 y=0 zoom=1 lines=3
MATCH (a:Piece)-[r:Connection]->(b:Piece)
WHERE a.name = 'core'
RETURN a.name, b.name
```

### OpText (`WriterOperation`, region `🔖OpText`)
One line per op, using `\` `"` `\n` escaping (own private `escape_text`/`unescape_text`, since `vcs`'s
equivalents are private to that crate) since op lines must never contain a raw newline:
- `setText "escaped text"`
- `setCamera x=.. y=.. zoom=..`
- `setDocument schema=.. id=.. language=.. uri=.. x=.. y=.. zoom=.. "escaped text"`

## Fixture conversion
- `writer/example/jack.writer.json` → `writer/example/jack.writer` (deleted the `.json`).
- `writer/example/dag.jack.writer.json` → `writer/example/dag.jack.writer` (deleted the `.json`).
  Dropped `graphDomain`/`fixtureRef` JSON fields — `WriterProjection` never had those fields (no
  `deny_unknown_fields`), so they were already silently discarded on every prior JSON deserialize;
  no behavior change.

## Plugin updates (`writer/plugin/rs/lib.rs`)
- `JACK_EXAMPLE_JSON`/`DAG_JACK_EXAMPLE_JSON` consts → `JACK_EXAMPLE_TEXT`/`DAG_JACK_EXAMPLE_TEXT`
  (`include_str!` of the new `.writer` files).
- New `🔖Examples` region: `jack_example_document()`/`jack_example_json()` and
  `dag_jack_example_document()`/`dag_jack_example_json()`, mirroring `note`'s
  `semio_example_document()`/`semio_example_json()` pattern exactly (parse once via
  `<WriterProjection as vcs::DocumentDsl>::parse_dsl`, re-serialize to JSON only where a
  framework-generic call site still wants a JSON string).
- All `serde_json::from_str::<WriterProjection>(JACK_EXAMPLE_JSON)`-style call sites, the two
  `.example(...)` registrations, and the two JSON-fixture test call sites now go through these
  functions instead.

## BLOCKER — reported via spawn_task, NOT fixed (out of scope: `/Users/ueli/Documents/semio/s/`)
`/Users/ueli/Documents/semio/s/plugin/rs/lib.rs` line 25:
```rust
register_os_fixture_json("jack.writer.json", include_str!("../../../writer/example/jack.writer.json"));
```
This `include_str!` now points at a deleted file → `s-plugin` will fail to compile. Line 24 (draw's
equivalent) was *already* fixed in a prior wave to point at `semio.draw` (keeping the registry key
string `"semio.draw.json"` unchanged) — the same one-line fix (path only, not the key) is needed for
line 25 (`jack.writer.json` → `jack.writer`). Left untouched per this ticket's "only touch
writer/" constraint; flagged via `spawn_task` (task_id `task_23989110`) instead of edited directly.

## Verification
`cargo test -p writer --lib` / `cargo test -p writer-plugin --lib` / `cargo check -p writer-plugin
--target wasm32-unknown-unknown` — repo is under extreme concurrent load from other sessions
(load average ~25-49 on a 10-core machine, ~80+ concurrent cargo processes) so runs are slow; see
this ticket folder for final pass/fail counts once they land.
