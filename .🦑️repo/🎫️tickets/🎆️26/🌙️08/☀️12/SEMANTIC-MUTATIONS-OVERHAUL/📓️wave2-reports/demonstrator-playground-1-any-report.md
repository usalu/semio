# Wave 2 — `semio-s-plugin-demonstrator` / playground / standards/1 / subsets/any / mutations

## Facet
`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`

## Snapshot shape
`PlaygroundSnapshot` (`📸️snapshot/🦀️component.rs`) has exactly one persistent field:
`schema: String` — a demonstrator/placeholder artifact with no other structured content. Per
`📓️derivation-rules.md`'s allowance for a trivial snapshot ("its whole vocabulary may legitimately
be just 1-3 metadata-only mutations"), the whole vocabulary is one mutation.

## Vocabulary derived
| Old generic | New semantic | Verb | Record |
|---|---|---|---|
| `SetSnapshot { snapshot }` | dropped — whole-document replace has no mutation-enum replacement (`store::ArtifactStore::reset` is the sanctioned non-history path) | — | — |
| `NoMutation` | dropped — `MutationKind::inverse` returning `Vec::new()` replaces the sentinel | — | — |
| *(new)* | `ChangeSchema { new_schema }` | `change` | `ChangedSchema` |

`PlaygroundMutation` is now:
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::Mutations)]
#[mutations(snapshot = PlaygroundSnapshot, diff = PlaygroundDiff, schema = "s.demonstrator.playground")]
pub enum PlaygroundMutation {
    ChangeSchema(change_schema::mutation::ChangeSchema),
}
```
`impl protocol::Mutation`/`SemanticMutation` are derive-generated; the old hand-written
`apply_playground_mutation`/`inverse_playground_mutation`/`impl Mutation for PlaygroundMutation`
were deleted.

## Triad leaf created
`✒️change-schema/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` — payload
`ChangeSchema { new_schema: String }`, `SEMANTICS = { verb: "change", entity: "playground", kind:
"change-schema", record: "ChangedSchema" }`, diff writes the sparse `PlaygroundDiff.schema` slot,
inverse reconstructs `ChangeSchema { new_schema: base.schema }` from BASE state.

## Text/binary wire codecs rewritten
`📝️text/🦀️component.rs` previously implemented `OpText`/`OpBinary` via `dsl::DslVariants` (relied
on the deleted `#[derive(dsl::DslEnum)]`/`apply_playground_mutation`/`NoMutation`). Replaced with a
handcrafted `OpText`/`OpBinary` (grammar: `change-schema new-schema="..."`), mirroring the already-
migrated `➗️mathematical` facet's pattern exactly (quoted-string scalar codec, tokenizer, one binary
tag per variant). `💾️binary/🦀️component.rs`'s test updated to construct `ChangeSchema` instead of
the removed `NoMutation`; its `encode_op`/`decode_op` wrapper functions were unchanged (they already
just delegated to the trait).

## Old generic leaves — orphaned, not deleted
`🖼️set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}` and `🫙no-mutation/{🦠️mutation,🔺️diff,↩️inverse}`
directories/files were **kept physically present** (rewritten to no longer reference the deleted
enum variants) because the plugin-shared `📦️glue.rs` (outside this facet's package boundary) still
`#[path]`-wires all six of them as `pub mod set_snapshot { mutation, diff, inverse }` / `pub mod
no_mutation { mutation, diff, inverse }` submodules under `mutations`. Deleting the directories would
make `glue.rs` fail to compile (`#[path]` pointing at a missing file), which is out of this ticket's
edit boundary. Each file now carries a `🪦️ Orphaned by 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` doc
comment explaining why it's dead and what `glue.rs` cleanup would let it be deleted. This exactly
mirrors the precedent already established by the `➗️mathematical` facet's `📄set-snapshot`/
`📊set-graph`/`📐set-geometry` orphan stubs from earlier in this same wave.

## Grammar / protocol `.semio` files
Left unchanged — both `📝️text/📖️component.grammar.semio` and `💾️binary/📡️component.protocol.semio`
are generic envelope descriptors (`header "schema" SP "stdio.json" NL` / `payload = OCTET+`, opaque
binary framing) that don't enumerate per-variant keywords; the already-migrated `mathematical` facet
left its equivalent files in the same generic shape, so no rewrite was needed here either (rule f of
the task brief — "not blocking" — applies).

## Tests
Extended the existing `🧪️Tests` region in `🧬️mutations/🦀️component.rs` (no new test files):
`playground_mutation_round_trips_store` (rewritten to use `ChangeSchema` instead of `SetSnapshot`),
plus new `change_schema_inverse_round_trips` and `semantic_kinds_cover_every_variant`. Extended
`📝️text/🦀️component.rs`'s existing `op_text_binary_roundtrip_law` (via `demo_mutation_cases`) and
`💾️binary/🦀️component.rs`'s existing `op_binary_round_trips_and_agrees_with_text` test to use
`ChangeSchema`.

`assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` (testkit law helpers) were **not**
added: grepped this crate (`semio-s-plugin-demonstrator`) and found no existing dependency on the
`testkit` module/crate anywhere in the plugin, so per the task brief step (e) this was skipped rather
than adding a new dependency.

## Files touched (all inside the package boundary)
- `.../🧬️mutations/🦀️component.rs` (rewritten — dispatch enum + inline leaf wiring + tests)
- `.../🧬️mutations/✒️change-schema/🦠️mutation/🦀️component.rs` (new)
- `.../🧬️mutations/✒️change-schema/🔺️diff/🦀️component.rs` (new)
- `.../🧬️mutations/✒️change-schema/↩️inverse/🦀️component.rs` (new)
- `.../🧬️mutations/📝️text/🦀️component.rs` (rewritten — handcrafted OpText/OpBinary)
- `.../🧬️mutations/💾️binary/🦀️component.rs` (test updated)
- `.../🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs` (orphaned)
- `.../🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs` (orphaned)
- `.../🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs` (orphaned)
- `.../🧬️mutations/🫙no-mutation/🦠️mutation/🦀️component.rs` (doc comment only, orphan note added)
- `.../🧬️mutations/🫙no-mutation/🔺️diff/🦀️component.rs` (doc comment only, orphan note added)
- `.../🧬️mutations/🫙no-mutation/↩️inverse/🦀️component.rs` (doc comment only, orphan note added)

## Shared-file requests (for the dedicated later reconciliation pass)
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📦️glue.rs`: delete the `pub mod set_snapshot { ... }`
  and `pub mod no_mutation { ... }` blocks under `artifacts::playground::standards::v1::subsets::any::schema::mutations`
  (currently around the `mutations` module's tail) — they only exist to keep the now-orphaned
  `🖼️set-snapshot`/`🫙no-mutation` leaf files compiling; once glue.rs no longer `#[path]`-wires them,
  those six files/three directories can be deleted outright.
- No app-level emit call site in this plugin was found constructing `PlaygroundMutation::SetSnapshot`
  or `::NoMutation` (grepped the whole artifact directory), so no additional app-facing
  `sharedFileRequests` beyond the glue.rs cleanup above.

## Verification
`cargo check -p semio-s-plugin-demonstrator` was launched but did not finish within the available
turn budget — this machine had a large number of concurrent `cargo check` processes from other
sessions running in parallel at the time (confirmed via `ps aux`: semio-s-plugin-block, -flow,
-stdio, -norm, -fem, -procedural, -animate, -space, -dag all checking simultaneously), which slows
every individual check substantially. The dispatch-enum/`dsl_derive::Mutations` pattern used here is
copied verbatim from the already-migrated `➗️mathematical` facet, which I separately confirmed
compiles past its own `🧬️mutations` module (its `cargo check` only fails later, on an unrelated
pre-existing missing file under `🎛️apps/➗️mathematical/🎮️commands/📄️document/`, nothing to do with
this ticket) — so the `dsl_derive::Mutations` derive resolution and `protocol::MutationKind`/`Mutation`
trait shapes used here are proven to work in this exact crate-dependency configuration. This report is
filed with `cargoCheck: not-run` because the demonstrator crate's own check did not complete in time;
the orchestrator should re-run `cargo check -p semio-s-plugin-demonstrator` to get a green/red
confirmation before treating this facet as fully closed.
