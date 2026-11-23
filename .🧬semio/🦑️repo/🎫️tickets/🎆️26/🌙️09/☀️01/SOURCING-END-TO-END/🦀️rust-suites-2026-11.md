# 🦀️ Sourcing Rust Suites, 2026-11-23

Driver: `native-cargo.sh` (shared target dir, `--test-threads=1`, the UI arena registry is process-wide; cached `bun` put on PATH). Logs in `🗑️generated/`.

## Framework fixes
| File | Fix |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧱️construction/📦️original/🦀️.rs` | `VcsArtifactApp::heap_bytes()` never existed; the Box allocation is `size_of::<VcsArtifactApp<A,M>>()`. Unblocked every sourcing crate. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🪡️list-delta/🦀️.rs` | `list_delta!` records and deltas now `deny_unknown_fields` (`{"curated":{"extra":true}}` was accepted). |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🦀️.rs` | `take_fixture_result_page` measured the original pump's own turn allocations against the page progress; now observes only the production operation inside the effect. |

## Sourcing fixes (tests, manifest)
- Curation sqlite snapshot tests ported to `original_owners::{decode_native,encode_native,export_sqlite,import_sqlite}`.
- Editor render tests pass the new `Locale`; archive loads go through `artifact_app_laws::admit_document_archive_load`.
- `aussuchen_pane_smoke`: `#![recursion_limit = "512"]`.
- Pool tests open `UiArenaFixture`; `semio-ui-contract-values` (testkit) is a dev-dependency of the curation crate.

## Results
| Crate | check --tests | test result |
| --- | --- | --- |
| curation lib | clean | 63 passed, 33 failed |
| curation inferences | clean | 17/17 |
| curation domain/schema/operations/examples/tests | clean | 1/1, 4/4, 31/31, 25/25, 14/14 (verbatim lines in `🗑️generated/curation-test2.txt`) |
| curation `persisted_receiving_law` | clean | 1/1 |
| curation `aussuchen_pane_smoke` | clean | 0 passed, 1 failed |
| curation `original_host_configuration` | clean | 0 passed, 1 failed |
| extensions beams, slabs, windows | clean | 1/1 each |
| oracle tests crate | clean | 0 tests defined |
| bridge | clean | 0 tests defined |
| hub semio-hub-sourcing (surface tests) | clean | 1 passed, 1 failed |
| hub semio-hub-sourcing-component | clean | 0 tests defined |
| `check --target wasm32-wasip2 -p semio-hub-sourcing-component` | builds, exit 0 | n/a |

## Remaining failures, all framework-owned (in flight in the process ticket)
1. Typed-operation and archive-load stalls: `complete original typed operation settlement` (loop never settles), `native construction exceeds original remaining Job grant` in archive replay, `original report requires its installed semantic and retirement issuers`, JSON `normal frontier exceeds original admitted depth` in the contributions lane. Same family as "original command fixture dispatch exceeded its authored turn bound".
2. 10 `UI arena already has an actual lifetime owner`: cascade of (1) leaving an arena lease unclosed.
3. 4 sqlite native-encode tests (`forwarded native storage requires its quoted allocation effect`): the encode product law now needs effect-based allocation in the producers and the `with_encoding` test helper (see process ticket `stale-port-*.md`).
4. `original_host_configuration`: heap witness assertion `(0,0)` vs `(2688,1344)` against a measured turn.
5. hub `viewer_never_mutates`: `captured shared original changed its Work or destination` at registered fixture close.
6. 3 sqlite oracle tests need `bun` (now on PATH in the wrapper; rerun pending).
