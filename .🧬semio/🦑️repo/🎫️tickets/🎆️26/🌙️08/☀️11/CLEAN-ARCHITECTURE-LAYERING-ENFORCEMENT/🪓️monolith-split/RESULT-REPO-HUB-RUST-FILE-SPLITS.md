# Result: repo and hub Rust file splits (slice REPO-HUB-RUST-FILE-SPLITS, prefix `rhs`)

Executor: Sonnet, foreground, no sub-agents, no ticket tool, no modifying git command. Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools and originals: `F/rhs-tools/`. Logs: `F/🗑️generated/rhs-*`. Date 2026-10-16.

## 1. Verdict

All six files are split into cohesive `<emoji>️<kebab>/🦀️.rs` children, installed in the repo and verified statically. **Nothing was compiled** (Wave 6: the launcher is refused); the evidence is the scoped full-resolution rustc differential (native, `--test`, `testkit`, `conformance`), a lossless line multiset, a definition scan, a seeded-sensitivity check, rustfmt, path and taxonomy checks, and reader-equivalence checks. The real-check commands are in section 9.

| Target (path tail) | Lines before | Parent after | Children | Largest child |
| --- | ---: | ---: | ---: | ---: |
| `📐️model/📦️packages/🦀️rust/🦀️.rs` | 3,819 | 211 | 8 | 640 (`🪪️entity`) |
| `🎫️tickets/📦️packages/🦀️rust/🦀️.rs` | 2,788 | 121 | 9 | 729 (`🔓️lifecycle`) |
| `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` | 3,450 | 365 | 12 | 483 (`🧺️artifact-cas`) |
| `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` | 3,161 | 376 | 12 | 450 (`🧺️artifact-cas`) |
| `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` | 2,923 | 348 | 10 | 506 (`🎟️capabilities`) |
| `🖥️server/🔨️modules/📡️gateway/🦀️.rs` | 3,306 | 145 | 11 | 839 (`👥️presence`) |

Every child is at or below 839 raw lines (limit: about 1,500). 62 children plus the shared-core file of section 4 are 63 new files.

## 2. The three hub backends: one seam list

`sqlite`, `postgres` and `neo4j` implement the same port, so they share one child structure, assigned by the same rules (`rhs-tools/backend_split.py`):

| Child (`<backend>/<folder>/🦀️.rs`) | Holds |
| --- | --- |
| `🗃️table-schema` (sqlite, postgres) | the table DDL const. Neo4j has none: its constraints already live in the `🧰️rows` crate. |
| `🔌️connection` | the backend struct, `connect`, `seed`, `lock`, the test-only fault hooks and `ArtifactGenesisTestControlV1` |
| `🧱️rows` (sqlite, postgres) | row decoders, select lists, small value helpers. Neo4j's decoders are the existing `🧰️rows` crate. |
| `📽️projections` | `persist_event*`, `project`, `projection_rejection`, `project_verified_checkpoint` |
| `🏭️artifact-creation` | creation claims, facts, recovery, document genesis |
| `🎟️capabilities` | share tokens and invites |
| `👤️accounts` | users, auth sessions, credentials, agent delegations |
| `🏘️spaces` | spaces, members, document descriptors, checkpoint reads |
| `🛡️administration` | admin operation audit, command receipts, checkpoint publication claims |
| `🔄️sync-sessions` | sync session bookkeeping |
| `🧺️artifact-cas` | CAS reservations, sweep candidates, delete fences |
| `📜️event-log` | the append-only log and projection rebuild |

The parent of each backend keeps its docs, the shared `use` block, the `admin_effect_try!` macro (it must precede the `mod` lines, because `macro_rules!` scope is textual), the mounts and the port adapter. Public paths are unchanged (`pub use connection::*`, `#[path]` mounts, same `testkit` mount at the tail).

### Decision: one trait impl cannot be split, so the adapter forwards

`impl HubDirectory for X` is 1,908 / 2,146 / 1,795 lines and Rust allows exactly one such impl per type. So the parent keeps the single `impl HubDirectory for X`, and each of its 91 / 86 / 86 methods is a forwarder, `self.name(args).await`. The original body moved verbatim into an inherent method of the same name, `pub(super) async fn name`, in the section child. Signatures are copied exactly (checked: `rhs-final-delegates.log`, one difference, a trailing comma where rustfmt wrapped a long inherent signature).

Why same-name forwarding resolves correctly (proved with a toy crate under `rustc --emit=metadata -o /dev/null`, no `unconditional_recursion` lint): inside the backend module family the inherent `pub(super)` method wins and is the body; outside it (the dispatch crate, laws crates, tests) the inherent method is inaccessible and the trait method is picked, which forwards. Costs: about 22 KB and 360 active lines more per backend (sqlite lib closure 3,106 to 3,471 active lines, still under the 4,000 limit), and every method signature exists twice.

Rejected alternatives: splitting the port into sub-traits (changes the public API of `semio-hub-directory` and every one of the ~30 callers that import `HubDirectory`; callers on concrete types would need each sub-trait in scope), and keeping the impl whole (1.2x to 1.4x over the limit). **Follow-up for the owner of the port:** the clean long-term shape is capability traits for the port with `HubDirectory` as the aggregate; it needs a compile loop.

## 3. Repo files (flat, region-based)

Children are whole `//#region` groups with their markers (all region and endregion counts are identical before and after).

- **model** (`📦️packages/🦀️rust`): `📋️allowed-values` (allowed values and slugs; the `include_str!` gained one `../`), `🏷️kinds`, `🌐️repo-aggregates`, `🎫️work-aggregates`, `🖥️codebase-aggregate`, `🌳️tree-aggregate`, and the two inline modules `pub mod ansi` and `pub mod entity` as files `🎨️ansi` and `🪪️entity` (wrapper removed, body dedented 4 spaces, module doc `///` became `//!`). The parent keeps header, imports, encoding helpers, errors and the `wire_types!` round trip. One doc sentence in the parent now says the tables are embedded by `📋️allowed-values`.
- **tickets**: `🪪️ids`, `📄️codec`, `🗄️store`, `💾️documents`, `📐️file-scope`, `🐙️issue-sync`, `⏰️ports`, `🔓️lifecycle` (lifecycle plus real purge), `🔎️search`; the `🧪️tests` mount stays in the parent.
- **gateway**: `❌️error`, `🧩️instance`, `📄️document-port`, `📣️fanout`, `👥️presence`, `🛂️edge`, `🧠️state`, `🗂️static-apps`, `🔁️streams`, `🛰️presence-stream`, `🖥️server`. `📄️document-port` is mounted with `#[macro_use]`, because the `#[dyn_enum]` traits expand `macro_export` macros there and the module is mounted `#[macro_use]` at the crate root.

Visibility rule (all six): a private item, field or inherent method that another file of the family (or a descendant test file) names becomes `pub(super)`, which is exactly the scope the private item had in the unsplit module (none of the originals used `pub(super)`). Applied by `rhs-tools/widen.py` (name-based superset, idempotent: `rhs-final-widen.log` shows 0 pending in all six). Totals: sqlite 38, postgres 51, neo4j 5, model 3, tickets 9, gateway 83. Only four signatures exceeded `max_width` after the keyword and were re-wrapped by rustfmt.

## 4. Deduplicated backend-neutral code (listed, as Wave 6 requires)

New shared file `🌎️hub/📇️directory/🛟️backend-support/🦀️.rs`, mounted from the contract crate root and re-exported there. Each item existed three times (sqlite, postgres, neo4j rows) and now exists once:

| Item | Removed from |
| --- | --- |
| `backend(err)` (driver error to `DirectoryError::Backend`) | sqlite, postgres, neo4j rows |
| `enum AdminEffectPreflightFailure`, its `From` impl for `AdminEffectCommitV1<T>`, `admin_effect_preflight` | sqlite, postgres, neo4j rows |
| `actor_kind_to_str`, `actor_kind_from_str` | sqlite, postgres, neo4j rows |
| `now_ms` (the contract already exported the identical function) | sqlite, postgres, neo4j rows |

Effects: the removed lines are the only lines missing from the line multiset (28 lines in sqlite and in postgres, 1 changed import line in neo4j). Public-path changes (greenfield, no alias): `semio_hub_directory_{sqlite,postgres}::now_ms` and `semio_hub_directory_neo4j_rows::{now_ms, backend, admin_effect_preflight, actor_kind_to_str}` are gone. Callers re-pointed at once: the neo4j main import list, and the six laws files `🪶️sqlite/🌱️creation-v1/🧪️tests/🔬️standalone`, `🪶️sqlite/🧪️tests/🔬️unit`, the same two for `🐘️postgres` and `🌐️neo4j` (all now take `now_ms` from `semio_hub_directory_contract`; both laws crates already depend on it). The neo4j rows crate imports the three names it still uses.

**Left duplicated, with reasons:** `artifact_creation_phase` (postgres and neo4j rows; its owner is `semio-hub-artifact-creation`, outside this slice, and the contract cannot depend on that crate), `ArtifactGenesisTestControlV1` and its `Default` (postgres, neo4j; `cfg(test)` types cannot cross a crate), `admin_effect_rollback` and the three `admin_effect_try!` macros (driver-specific, sync versus async), `DIRECTORY_COMMAND_RECEIPT_SELECT` (SQL text shared by two dialects only), the dialect `SCHEMA` consts. Each sits in the same-named child of every backend, so repeats stay next to each other.

## 5. Source-text readers updated

| Reader | Change |
| --- | --- |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | new `readDirectoryBackendSource(repoRoot, backend)`: root head (imports, macro), then every section child in mount order via `listModuleTree`, `pub(super) ` stripped, adapter impl cut off, for neo4j plus the rows crate. `readNeo4jDirectorySource` forwards to it; 12 `readFileSync` sites replaced (5 sqlite, 5 postgres, `const backend` in the command-authority gate, the postgres/neo4j loop). Gates affected: share-issuance-atomicity, retained-short-admin, directory-event-page-v1, directory-command-receipt, invite-redemption-transaction, space-administration. |
| `🌎️hub/🧪️tests/🗂️artifact-root/🦀️.rs` and `🧪️tests/🔬️bin-unit/📇️directory/🔬️unit/🦀️.rs` | new `semio_hub_testing::hub_module_tree` (root plus `#[path]` children, `🧪️tests` mounts skipped); the two source-count laws (postgres, neo4j) call it instead of `hub_source` |
| `🖥️server/🧪️tests/🔬️wire/🟦️.ts` | `readModuleTreeSource` of the gateway tree (import added) |
| `🖥️server/🧪️tests/🔬️wire/🦀️.rs` | `GATEWAY` is now `GATEWAY_SERVER`, an `include_str!` of `🖥️server/🦀️.rs` (where `base_router` lives) |
| descriptor-digest fixture `🔣️.json` (`rustReceivers`) | added `🌐️neo4j/📽️projections/🦀️.rs`, the child that now holds the checkpoint digest receivers |
| three `@see` comments in `🎓️teaching/🛂️proctor/🔨️modules/{❓️queries,👥️presence,🧩️instance}/🦀️.rs` | point at the gateway children |

Reader equivalence (evidence, since the gates cannot run, section 8): of 5,288 string literals in `📜️script.ts`, the 245 code-like ones that occur in the backend sources have identical occurrence counts in the old monolith text and the new reader text except the adapter marker and the testkit mount path, which the reader drops by design (`rhs-final-gate-literals.log`). The method slices the gates cut (`issue_share_token_as*`, `authenticate_share_binding..socket_share_binding`, `revoke_invite_as..list_invites`, the share schema) are identical for all three backends; the event-page order predicate is identical (`rhs-final-gate-slices.log`; 4 listed differences are last-in-child methods that no gate slices). The Rust counts of the two laws are equal (postgres 2, 3, 5; neo4j 5, 5). The gateway route and error-kind extraction gives the same 15 routes and 10 kinds old versus new and matches the fixture (`rhs-tools/gateway_reader.ts`). `bun build --no-bundle` parses `📜️script.ts` and the wire test.

## 6. Taxonomy

No registration edit was needed. `crate-module-children` (kebab pattern, owners `hub`, `capabilities`, `editor`), `schema`, `presence` and `members-of-modules` already admit every new name. The tx2 simulator (`🗑️generated/tx2-tools/sim.ts`, live taxonomy, 107,586 directories) resolves all 63 new directories (62 children plus `🛟️backend-support`), 0 ambiguous, 0 unresolved among them. All folder names are `<emoji>️<kebab>` with U+FE0F, each emoji is already a folder prefix elsewhere, unique among siblings; each new file starts with a `//!` docstring whose emoji is unique within its crate (`rhs-final-layout.log`). Backend DDL child is `🗃️table-schema`, not `🧬️schema`, to avoid the taxonomy's `schema` kind.

## 7. Verification actually run

| Check | Command or method | Result |
| --- | --- | --- |
| Full-resolution differential | `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=short -W unused-imports [--test] [--cfg 'feature="testkit"'\|'feature="conformance"']` from the real lib root (`rhs-tools/run_backend_oracle.py`, sandbox copy outside the repo, original monolith restored in the baseline, oracle patches applied to both). Log `rhs-final-oracle.log`. | model 712 vs 712, tickets 88 vs 88 (test 89 vs 89), gateway 68 vs 68 (test 354 vs 354, also conformance): **identical**. sqlite, postgres, neo4j (native, test, testkit, test+testkit): identical plus exactly one `E0432 unresolved import semio_hub_directory_contract` for the new shared import, the expected unresolved-external line. Contract: identical plus one `E0433 directory` for the new file's `use directory::...` (the same external crate the sibling files import). Rows: identical plus the same contract import line. No new E0412, E0425, E0422, E0423, E0603, E0432 (local) in any configuration. |
| Seeded sensitivity | one `pub(super)` removed per family; `rhs-final-seeded.log` | caught: sqlite `E0425 user_row` x4, model `E0425 go_to_lower`, tickets `E0425 normalize_separators` x23, gateway `E0425 wrap_relay` (test configuration); a harmless added item (negative control, neo4j) produced no difference |
| Losslessness | `rhs-tools/verify_installed.py`, multiset of stripped non-blank lines with `pub(super) ` undone; `rhs-final-lossless.log` | model, tickets, gateway, neo4j: 0 missing except the removed wrapper lines of the two inline modules, the include path line, the edited doc line and the neo4j import list. sqlite, postgres: 28 missing lines each, all of them the section 4 items. Extras are only docstrings, `use super::*;`, mounts, impl wrappers and braces, forwarders, and the one import. Region markers equal in all six. |
| Definition scan | same script, top-level items and impl members | lost: the 7 section 4 items per SQL backend, nothing else; gained: the inherent forwarding targets (120 / 116 / 108), mounts |
| Forwarders | `rhs-tools/verify_delegates.py` | every forwarder has exactly one same-named inherent method with an identical signature and the same argument order; no duplicate inherent names; no duplicate top-level names across children |
| Paths | `verify_layout.py` | all `#[path]` and `include_str!` targets exist; longest new path 165 bytes with the repo prefix (limit 240) |
| rustfmt | `rustfmt --check` hunk counts with the repo config, base versus split: sqlite 7 / 6, postgres 5 / 5, neo4j 5 / 4, model 10 / 10, tickets 5 / 2; `--emit stdout` parse of all 62 children, the shared file, the parents and every other touched `.rs` | no new hunks; parse failures 0 |
| Census | `F/dm2-tools/closure.ts --summary` (`rhs-census.log`) | all under 32 files and 262,144 bytes in lib configuration; active lines: sqlite 3,471 (was 3,106), postgres 3,771 (3,417), neo4j 3,273 (2,890), rows 555 (601), contract 1,712 (1,653), model 3,874, tickets 2,849, server lib 3,448. The server test closures are 353,837 bytes and 6,915 lines, over the byte limit; the gateway changed that closure by +2.8 KB only, so the overrun is not caused here. |
| `[DEBUG]` | grep over all touched files | none |

## 8. Not verified, and why

- **No cargo or rustc build of any crate.** Field and method privacy, trait-in-scope resolution, orphan/coherence, `private_interfaces` and macro hygiene are checked by name only (the oracle stops before type check because the external crates are unresolved). The forwarder design rests on method-probing rules checked with a toy crate, not on the real crates.
- **The `os-hub` gates could not run.** `bun ./📜️script.ts share-issuance-atomicity-check` fails at import: `Export named 'startLocalSessionBroker' not found` in `🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts` (`📜️script.ts:93`; a peer renamed the function to `withLocalSessionBrokerV1`). Foreign error, left alone (`rhs-gate-share-issuance.log`).
- The descriptor-digest tree-sitter test, the wire TS test and all Rust laws were not run.
- `git grep` printed `error: daemon terminated` (fsmonitor) in this session; searches were repeated with `core.fsmonitor=false`.

## 9. Real-check commands for the compile queue

Run one launcher call at a time under the fleet compile mutex. Prefix: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --`

1. Root workspace (model, tickets, server and their dependents): `check -p semio-framework-repo-model -p semio-framework-repo-tickets -p semio-framework-server -p semio-framework-repo-cli -p semio-framework-repo-cli-verbs -p semio-framework-repo-cli-mcp-verb -p semio-framework-repo-graphql-fs -p semio-framework-repo-dashboard-repo-domain -p semio-framework-repo-dashboard-inventory -p teaching-proctor --lib --keep-going --message-format=short`, then the same with `--tests`, then `test -p semio-framework-repo-model -p semio-framework-repo-tickets -p semio-framework-server --lib` and `-p semio-framework-server --test wire --test closed_ports`.
2. Hub workspace lib: `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory-contract -p semio-hub-directory-neo4j-rows -p semio-hub-directory-sqlite -p semio-hub-directory-postgres -p semio-hub-directory-neo4j -p semio-hub-testing -p semio-hub-directory-dispatch -p semio-hub-directory-service --lib --keep-going --message-format=short`, plus the feature lane `--features semio-hub-directory-postgres/testkit,semio-hub-directory-neo4j/testkit`.
3. Hub tests: `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory-laws -p semio-hub-directory-backend-laws -p semio-hub-directory-route-laws --tests --keep-going --message-format=short`. Expected review points: pub(super) gaps, `private_interfaces` warnings, unused imports in the three parents.
4. After the `startLocalSessionBroker` import defect is fixed, from `🌎️hub/📦️packages/🦀️rust`: `bun ./📜️script.ts share-issuance-atomicity-check`, `retained-short-admin-check`, `directory-event-page-v1-check`, `directory-command-receipt-check`, `invite-redemption-transaction-check`, `space-administration-check`.

## 10. Files

Created (62 children + 1 shared file): under `🌎️hub/📇️directory/{🪶️sqlite,🐘️postgres,🌐️neo4j}/` the folders in section 2, `🌎️hub/📇️directory/🛟️backend-support/🦀️.rs`, under `📐️model/📦️packages/🦀️rust/` and `🎫️tickets/📦️packages/🦀️rust/` and `🖥️server/🔨️modules/📡️gateway/` the folders in section 3.

Edited: the six target parents; `🌎️hub/📇️directory/📜️contract/🦀️.rs` (mount and re-export); `🌎️hub/📇️directory/🌐️neo4j/🧰️rows/🦀️.rs` (7 items out, one import in); six laws files named in section 4; `🌎️hub/📦️packages/🦀️rust/📜️script.ts`; `🌎️hub/🧪️tests/🗂️artifact-root/🦀️.rs`; `🌎️hub/🧪️tests/🔬️bin-unit/📇️directory/🔬️unit/🦀️.rs`; `🖥️server/🧪️tests/🔬️wire/🟦️.ts` and `🦀️.rs`; the descriptor-digest `🔣️.json`; three proctor `🦀️.rs` comments. Peers' unrelated edits in `📜️script.ts` and elsewhere were left intact.

Tools (`F/rhs-tools/`): `orig/` holds the eight pre-edit files with `SHA256` (rollback = copy back the parents, remove the new folders); `carve.py`, `backend_split.py`, `region_split.py`, `stage.py`, `install.py` (hash-guarded), `widen.py`, `fmt_widened.py`, `sandbox.py`, `run_backend_oracle.py`, `seeded.py`, `verify_installed.py`, `verify_delegates.py`, `verify_layout.py`, `widen_dry.py`, `gate_literals.ts`, `gate_slices.ts`, `gateway_reader.ts`, `new/backend-support.rs`. Sandboxes and stages live in the session scratchpad outside the repo.
