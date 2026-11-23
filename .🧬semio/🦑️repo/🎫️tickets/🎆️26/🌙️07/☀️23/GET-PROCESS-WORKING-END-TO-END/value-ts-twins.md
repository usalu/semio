# Value TS twins: 52 failures down to the Cargo-dependent and off-limits remainder

Scope: all `🌱️value/**/🧪️tests` TS twins (`🟦️.ts`, `*.test.ts`). Run with the repo bun, `SEMIO_TEST_ARTIFACT_DIR` set outside the repo tree and `RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin`.

## Real Rust fixes (checked with `stale-cargo.sh chk check -p ...`, all 10 touched crates compile)
- `🛬️decode/🦀️.rs` `charge`: a forwarded allocation refusal was rebuilt with `ValueError::new(.., format!("{refusal:?}"))`, allocating in a refusal path. It now returns the owned `ValueError` or a borrowed literal for a retained handle (`⚠️refusal` twin law: no `ValueError::new(` in decode or encode).
- `reborrow(grant, effect.maximum_work_units())` replaced by `effect.nested(grant)` (lends exactly what is left; no refusal after spent work) in 15 files: job fault publication, value clone-source (2 files), retirement root, factory (+owned), controlled, selection, result, mutex, arc-custody sealed, decode owned value, decode receiver protocol (2 macro sites), os plugin shard event, window epoch, icon paint cache. Fixes `work-nested-sources.test.ts`.
- `🧳️owned/🧬️protocol/♻️backing`: a `RetainedWorkPoll::Pending{..}=>{}` arm now returns `reply.pending()` (same behaviour via `finish`, no silent accept). Fixes `source-hazards.test.ts`.

## Schema or fixture drift (twin/schema updated)
- draft-07 `$schema` to 2020-12 for the Ajv2020 twins: shared-utf8 (+original-lease), original-vector, result, canonical, factory construction.
- original-vector schema: added `actualCloseWork`. scalar schema: added `closeWork`, `advanceWork`; payload width is a grant (16) over an 8-byte atom. list reservation schema: added `metadataReuse`.
- bounded-ord: `maximumTurnWork` is 3 (primitive + transfer + counter work).
- uncontrolled-owners: fixture lists the current test fixtures plus `SourceRoots` (job source roots, closed by their own close step).
- ownership registry (`🧪️testing/🎯️ownership`): `value-sort` left to its own package, `value-paged-append-work` added, publication/receiving/scratch hashes and token counts refreshed, `libraryTestFiles` 17, `individualTests` 19.
- sealed-shared-receiver and `🌳️value/📖️source`: the fixture and schema files had been swapped (the schema sat in the fixture file). The twins now read data and a 2020-12 schema from the right files.

## Source-text laws re-pointed to the current owner
leaf (slice ends at `pub fn leaf<`), controlled (engine field/scalar/string cursors, shared custody contract, `close_demands`/`next_close_*` trait defaults, derive uses `close_demands(self,0)?.depth`), queue (`size_of_val(owner)`, `self.next_demand(body)?`), mutex (`ControlledRetirement::retain`), factory owned/construction/arc-custody, sealed alias return progress, decode ledger (`Option<NativeAllocationLedger>` and identity moved to recipient), decode ownership mount (`semio-framework-value-native-owned`), bulk-vec (`advance_captured_into`, `demands(body,maximum_work_units)`), list reservation (metadata-only page copy quote, `reserve_effect`), derive close demand, default (`retained_clone_grant_admits(effect.grant()`), ordered-map close path, snapshot owned retirement, neutral-owner `project_owned` path, `🌳️value/📦️storage` SQL placeholder count.

## Remaining (full run: 625 pass, 5 fail before the last revert; 4 now after reverting the original-lease schema to draft-07, which `🛬️decode/🧪️tests` still reads with plain Ajv)
- `🏷️type/🧪️tests/🚮️absence` fails for the same Cargo compile errors as neutral-owner.
- `🧳️owned/🧩️record/🔎️schema/🎟️work` flatten oracle: off-limits (draft 2020-12 vs plain Ajv).
- `🧪️tests/🧩️neutral-owner` "actual value derives build and execute with product trees absent": needs a hermetic Cargo build and fails in real Rust, not in the twin. The `semio-framework-value` integration test targets `value-paged-record`, `value-utf8`, `value-paged-append-work`, `value-publication`, `value-paged-receiving` do not compile (stale API: `RetainedWorkPoll` vs `RetainedWorkOutcome` fields, `PagedUtf8*`, `PagedTextReceiver::advance/begin_close`, missing `semio_framework_value_clone_source_testing`). Needs a Rust test migration, not done here.
- Off-limits, untouched: `🛬️decode/🧳️owned/📦️storage/*`, `✨️derive/🛬️receive/*` (`🎟️frontier` twin `derived receiving pays selected getter and alias before publication`), any `🧩️record/🔎️schema`.
