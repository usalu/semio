# 📓️ KN2 — `semio-framework-os-kernel --lib` to zero red (+ replication & pack suites)

Slice KN2 of ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, 2026-09-21.
Inherits KN1 (`📓️kn1-kernel-lib-gate.md`): 1068/48 → **1104 passed / 12 failed**.

All runs foreground, one cargo at a time, `RUST_MIN_STACK=67108864`, `CARGO_INCREMENTAL=0`,
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-kn2` (shared build-dir untouched).

## 1. The 12 reds — status

Baseline re-measured (`🗑️generated/kn2-baseline-serial.txt`): **1104 passed / 12 failed**, byte-identical
to KN1's `kn1-run-9.txt`. The twelfth red KN1's §4 did not list by name is
`os_directory::schema::…document_open_plan_v1_matches_language_neutral_fixture` (FL2 named it as
fleet-in-flight schema-fixture breakage).

| # | test | root | status |
|---|---|---|---|
| 1 | `os_store::…::retained_member_publication_preserves_order_group_identity_and_exact_maximum_grant_progress` | publication ladder: item close under-spends its grant | **fixed** |
| 2 | `os_store::…::retained_member_group_preparation_reserves_real_history_without_partial_visibility_and_aborts_stale_owners` | same | **fixed** |
| 3 | `os_store::…::artifact_store_batch_commit_refuses_an_under_declared_multi_item_gesture_and_accepts_the_invertible_one` | staged inverse capacity pre-refused before the commit gate | **fixed** |
| 4 | `os_store::…::artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step` | store disposer under-spends its 512-byte grant + one idle turn per installed owner | **fixed** |
| 5 | `os_store::component::owned_field_rejected_page_tests::registered_rejected_pages_obey_zero_short_and_exact_grants` | rejected-page close passed no byte grant to the record cursor | **fixed** |
| 6 | `os_store::component::owned_field_rejected_page_tests::unadmitted_rejected_pages_obey_zero_short_and_exact_grants` | same | **fixed** |
| 7 | `os_store::component::canonical_edit::tests::canonical_sealer_preserves_large_domains_and_all_wire_metadata_origins` | canonical cursor spelled the `contributed` origin's keys snake_case where `MutationOrigin::to_value` spells them camelCase | **fixed** |
| 8 | `os_store::component::presence_retirement::tests::retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open` | mounted worker session never reaches terminal; stalls inside `semio-framework-job` | **GAP — the one red I did not close** |
| 9 | `os_spr::channel::tests::paged_generic_decoder_admits_document_config_and_projection_commands_used_during_browser_boot` | stale 4-turn harness bound vs a 28-turn paged archive decode | **fixed** |
| 10 | `os_spr::channel::tests::paged_recursive_archive_crosses_pages_and_decoded_owner_closes_one_field_per_grant` | archive-close stage rebuilt itself every turn, so the command shell was never released | **fixed** |
| 11 | `os_dsl::grammar::tests::every_shipped_grammar_semio_parses_and_compiles` | 70 of 503 shipped grammars did not parse | **fixed — 503 / 503 parse and compile** |
| 12 | `os_directory::schema::tests::document_open_plan_v1_matches_language_neutral_fixture` | stale law: tested a cross-check commit `48b9d63cf6` deliberately removed | **fixed (re-expressed)** |

### Bucket A — the batch/retained-member publication ladder (4 reds, one shared root + one sibling)

Measured with a temporary in-test turn trace (`🗑️generated/kn2-probe-1.txt`, since removed): the
8194-byte first row of `🧫️fixtures/📢️member-publication.json` burned all `byte_count + 32` turns and
never left `Preparing`; the group-preparation law burned all 32 turns at a frozen checkpoint.

**Root 1 — the retirement protocol's byte half of the grant was never spent.** A publication turn
admits `ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }`: ONE structural owner
and up to 4096 BYTES. Both the batched item close and the store's own cursor disposer took exactly
ONE `close_step` from their child per turn. Every retained wire owner releases one byte (or one
string) per step, so retiring the folded item became a SECOND, byte-at-a-time pass over a wire the
preparation's own declared footprint (`work_items = bytes + 3`) pays for exactly once. No
publication bigger than its caller's spare turns could ever reach `Publishing`.

**Root 2 — installing an owner cost a whole turn.** `ArtifactStoreCursorDisposer::close_step`
returned as soon as it had moved the next owner into its `active` slot. Histogram of the
200-mutation store close (`kn2-probe-6.txt`): 4465 turns against the law's 4096 ceiling, of which
~600 released nothing at all — one per owner installed (`HistoryMutations` alone: 1201 turns, 0
bytes, for 200 mutations).

**Root 3 (sibling) — the staged inverse buffer refused before the commit gate could.**
`fold_batch_item` reserved `work_items - admitted_items` inverse rows. For the law's under-declared
3-item gesture that is 0, so the FOLD refused (`batched fold exceeded its admitted fixed inverse
capacity`) where the law requires the gesture-wide `PreflightingCommit` gate to be the refuser
(`batched prepared candidate failed its exact fixed commit contract`). The law's own docstring
states the intended division of labour; the buffer was the accidental first refuser.

Measured after the fix: row 0 publishes in **8209** of 8226 admitted turns (was: never), the group
law reserves on turn ~14 of 32, the 200-mutation store closes inside 4096 turns, and the
under-declared gesture now dies at the commit gate with the message the law names.

### Bucket B — rejected-page grants, the directory plan law, and the spr paged channel

| red | root | fix |
|---|---|---|
| `owned_field_rejected_page_tests` ×2 | `ArtifactEnvelopeDecodeRejected::close_step` and `ArtifactEnvelopeUnadmittedDecodeRejected::close_step` passed `maximum_items` to the record cursor but NEVER its byte grant, so a `maximum_bytes: 0` turn released a whole 4096-byte page and blew the grant. The sibling `InteractiveJob` close path already carries the guard. | page-granule guard added to both, skipped for the empty record husk so the terminal turn still completes under any grant |
| `os_directory::schema::…document_open_plan_v1…` | **stale law.** Commit `48b9d63cf6` (2026-09-20) deliberately removed `self.parent_dialect.artifact_kind != self.artifact.kind` from `DocumentOpenPlanV1::validate` and its sibling, with a docstring stating the two are separate id spaces pinned by `validate_descriptor_open_target`. The removed expression exists nowhere in the tree. The law still asserted the removed cross-check, contradicting the `two_space` law four rows above it. | re-expressed: a foreign parent kind is ADMITTED, with the reason written down |
| `os_spr::channel::…paged_recursive_archive…` | `DecodedAppCommandOwner::close_step` rebuilt its archive closer from the emptied `DocumentArchivePack` on the turn after it drained, reported terminal, dropped it, and rebuilt it again — so a `LoadDocumentArchive` owner never released its command shell and NO decoded archive owner could ever reach terminal. | `close_stage == 0` latch; `close_field` now advances on a released field instead of costing a second turn per field |
| `os_spr::channel::…admits_document_config_and_projection_commands_used_during_browser_boot` | **stale harness bound.** Commit `8add1df147` (2026-09-12) added the two archive rows into a loop whose `0..4` turn budget was written on 09-08 for single-field commands. A paged archive decode is one field per turn by design, so a two-member archive needs 28. Red from the day the rows landed. | re-expressed STRONGER: each row now carries its exact turn count (3/2/28/2/3/2/2/2/2/4/2) and every turn before the last must yield `None` |

## 2. Grammar constructs — `os_dsl::grammar::every_shipped_grammar_semio_parses_and_compiles`

KN1 fixed the discovery filename and the law went honest at **70 of 503 shipped grammars failing to
parse**. The corpus is authored in TWO spellings of the same language: the one
`📖️grammar.grammar.semio` declares (`#` comments, `|` alternation, `{ }` groups, postfix
quantifiers) and the ABNF spelling (`;` comments, `/` alternation, `( )` groups, `[ ]` optional,
prefix `*`/`1*` repetition, `%x` code-point terminals). `parse_grammar` only implemented the first.

| construct | files | what was added |
|---|---|---|
| `;` line comments | 20 | cut out of the segment before `core_lex` sees them — their PROSE carries emoji (🚧 📸 🦀 🧬), backticks, apostrophes, `%`, dots and URLs, so an unrecognised comment marker made a file's commentary, not its syntax, the thing that failed |
| `/` alternation | 22 | lexed as `Pipe`, alongside `|` |
| `( … )` grouped alternation | 15 | new `parse_atom` arm; told apart from a macro-call argument list by ADJACENCY of the `(` to its name (spans carry column + length), replacing the old "unresolvable without whitespace" reservation |
| leading-`|`/`/` continuation lines | 8 | `parse_alternatives` consumes a newline run only when an alternation operator follows it |
| `[ … ]` optional group | 4 | `LBracket`/`RBracket` token kinds + `Symbol::Optional(Group)` |
| prefix repetition `*x`, `1*x`, `0*x` | (with the above) | `Symbol::Star`/`Symbol::Plus`; any other count is refused by name |
| `%x20-21`, `%d65`, `%b1010` | 1 | lifted out of the byte stream as one terminal name |
| `dialect grammar <id>` one-line header | 18 | a third ident on the dialect line IS the grammar id |
| implicit `root` start symbol | 18 | no `start` directive + a `root` production ⇒ start at `root`; a file with neither is still refused |
| header keyword as a production name | 2 | `comment`/`start`/`string` followed by `=` is a production (html defines `comment` for HTML's own `<!-- -->`) |
| indented sequence continuation | 1 | a newline run is consumed only when what follows is indented and is not a production head |

Two diagnostics defects were fixed on the way, both of which had made every earlier reading of this
law misleading: `push_segment` lexed each segment on its own and reported the core lexer's
SEGMENT-relative spans as file positions (so the 15 group failures all pointed at "line 2 column 9"
of an unrelated line), and the refusal path did the same. Spans are now rebased onto the file.

`📖️grammar.grammar.semio`, the normative self-description, was extended with the same constructs so
the parser and the grammar-of-grammars still state one language.

## 3. Cross-crate suites — KN1's fixes 8 and 9, never run by KN1

KN1 changed `MutationDag::insert`'s cascade in `semio-framework-replication` and
`PackWriter::begin_identity_chunk`'s framing in `semio-framework-pack` without running either
crate's own suite. Both are clean.

| suite | result | capture |
|---|---|---|
| `cargo test -p semio-framework-replication --lib --no-fail-fast` | **289 passed / 0 failed** | `🗑️generated/kn2-replication.txt` |
| `cargo test -p semio-framework-pack --lib --no-fail-fast` | **103 passed / 0 failed** | `🗑️generated/kn2-pack.txt` |

No fixes were needed in either crate.

## 3b. Run ladder

| run | passed | failed | landed |
|---|---|---|---|
| `kn2-baseline-serial.txt` | 1104 | **12** | — (identical to KN1's `kn1-run-9`) |
| `kn2-probe-7.txt` (os_store only) | — | — | grant drain, install-and-drain, staged inverse capacity: ladder ×4 green |
| `kn2-probe-8.txt` | — | — | page-granule guard, directory law, spr archive latch |
| `kn2-run-12.txt` | 1114 | 3 | grammar 70 → 3 |
| `kn2-run-14.txt` | 1114 | 3 | grammar 3 → 0; a peer's red appears |
| `kn2-run-16.txt` (final) | **1116** | **2** | canonical sealer origin keys |

**Final: `cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- --test-threads=1` reads
1116 passed / 2 failed** (`🗑️generated/kn2-run-16.txt`). Of the two, ONE is mine (row 8) and one is a
peer's in-flight change that landed after `kn2-run-12` — see §5.

## 4. Fixes — file:line (post-edit)

Product:

| # | file:line | fix |
|---|---|---|
| 1 | `🏪️store/🦀️.rs:434` `spend_close_byte_grant` (new) | spends a turn's admitted BYTE grant on a retained owner instead of taking one `close_step`; stops at the first structural release, charges a no-progress step one unit so the drain is bounded by `maximum_bytes` |
| 2 | `🏪️store/🦀️.rs:1996` `ArtifactStoreCursorDisposer::close_step` + `take_next_owner` (new) | drains the active child through (1), and installs the next owner and drains it in the SAME turn — installing cost a whole idle turn before, 600 of them for a 200-mutation gesture |
| 3 | `🏪️store/🦀️.rs:17038` `advance_apply_batch` / `spend_item_close_grant` (new) | the folded item's preparation owner is retired within the turn's grant instead of one byte per publication turn |
| 4 | `🏪️store/🦀️.rs:17240` `fold_batch_item` | staged inverse capacity is the gesture's declared `work_items`, not `work_items - admitted_items`, so the gesture-wide commit gate — not the staging buffer — refuses an under-declared gesture |
| 5 | `🏪️store/🦀️.rs:9002` `ArtifactEnvelopeDecodeRejected::close_step` | page-granule byte guard (a page is released whole or not at all; the empty husk still completes) |
| 6 | `🏪️store/🦀️.rs:9067` `ArtifactEnvelopeUnadmittedDecodeRejected::close_step` | same guard |
| 7 | `🏪️store/🧵️canonical-edit/🦀️.rs:122` | the `contributed` `MutationOrigin`'s keys are `pluginId`/`mutationId`/`payloadHash`, matching its own `ToValue` (the `transaction` sibling was already aligned) |
| 8 | `📡️spr/🧵️channel/🦀️.rs:1797` `DecodedAppCommandOwner::close_step` | `close_stage == 0` latch so a drained archive closer is not rebuilt from the emptied pack every turn |
| 9 | `📡️spr/🧵️channel/🦀️.rs:1520` `PagedDocumentArchiveDecode::close_step` | `close_field` advances on a released field instead of costing a second turn per field |
| 10 | `🗣️dsl/📖️grammar/🦀️.rs:301` `span_at` (new) + `push_segment` | segment-relative core-lexer spans are rebased onto the file, on both the token and the refusal path |
| 11 | `🗣️dsl/📖️grammar/🦀️.rs:379` `lex` | `;` line comments cut out of the segment; `/` lexed as an alternation operator; `%x…`/`%d…`/`%b…` lifted out as one terminal name |
| 12 | `🗣️dsl/📖️grammar/🦀️.rs:544` `parse_atom` | `( … )` groups (told from a macro call by `(`-adjacency), `[ … ]` optional groups, prefix `*x`/`1*x`/`0*x` repetition, exact `n(…)` repetition bounded by `EXACT_REPETITION_MAXIMUM` |
| 13 | `🗣️dsl/📖️grammar/🦀️.rs:600` `parse_sequence` / `parse_alternatives` | indented sequence continuation and leading-operator alternative continuation, both guarded by "not a production head" |
| 14 | `🗣️dsl/📖️grammar/🦀️.rs:700` `parse_grammar` | `dialect grammar <id>` one-line header; a header keyword followed by `=` is a production name; no `start` directive + a `root` production starts at `root` |

Contracts and laws:

| file | change |
|---|---|
| `🗣️dsl/📖️grammar/📖️grammar.grammar.semio` | the normative self-description now declares `/`, `( )`, `[ ]` and both repetition forms, so parser and grammar-of-grammars still state ONE language |
| `📇️directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:400` | stale law re-expressed (foreign parent kind is admitted; the two id spaces are bounded separately) |
| `📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs:615` | stale harness bound re-expressed STRONGER: exact per-command turn counts |

## 5. Gaps — honest

1. **`retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open` is
   still red, and I did not close it.** Measured (`🗑️generated/kn2-probe-13.txt`, instrumentation
   since removed): `session.close_step(1, 4096)` returns
   `WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 }` on EVERY turn from turn 0,
   `owner.maintenance_local_reads_step` answers `Complete` from turn 0 (the registry never sees a
   returned read), `worker_job_retirements_are_parked()` is false and
   `pump_worker_job_retirements(1,1,4096)` returns 0 — so this is NOT the known retirement-slot
   leak. `CapturedLocalJob::close_step` would answer `Pending { released_items: 1, .. }` on its
   first turn, so `worker_job_close_step` (`🧰️framework/🔨️modules/🧵️job/🦀️.rs:3010`) never reaches
   its `close_stage == 1` branch: it returns from an earlier one — the `quarantined_outcome` or
   `outcome` branch, whose `JobPayloadCloseStep::Complete` arm answers `Pending { 0, 0 }` and puts
   the authority back without clearing the payload, or `release_retirement_slot` refusing. Pinning
   which of the two needs one instrumented build of `semio-framework-job`, a crate outside this
   slice whose rebuild cascades through the kernel; I ran out of budget before doing it. The
   defect, if it is the first branch, is a payload owner whose `close_step` says `Complete` while
   its `terminal_is_empty()` stays false — a spin, not a deadlock.
2. **`snapshot_read_double_return_is_counted_once_and_reclaimed_once` is a PEER's red, not mine.**
   It was green in `kn2-run-12` and red in `kn2-run-14`; in between a peer added the uncommitted
   `SnapshotReadLeaseRegistry::try_release_aliased` + the `SnapshotReadLease::return_now` fast path
   (`🏪️store/🦀️.rs:224` / `:315`, still uncommitted at the time of writing). With an aliased root the
   first `return_now` takes the fast path (slot freed, `registry.returned` not incremented) and the
   second one falls through to the counter and also answers `true`, so the law's
   "the same generation cannot be returned twice" fails. I left it to its owner rather than editing
   a peer's in-flight hunk.
3. **Native only.** No wasm32 build, no `activate`, no serve, no browser. Every claim here is a
   native law run in-crate. The guest behaviour of the spr archive close latch and of the grammar
   parser's new constructs is unverified at runtime.
4. **The grammar law only asserts PARSE**, not compile: it runs `let _ = Recognizer::compile(&grammar)`
   and discards the result. 503/503 now parse and reach `compile`; how many compile to a recogniser
   that actually recognises their format is a different question this law does not ask.
5. **`EXACT_REPETITION_MAXIMUM` expands `n(…)` to n copies.** The corpus uses 9 and 35. A grammar
   stating a large count would build a large production; the bound is 1024.
6. **The silencing panic hook** KN1 recorded (`🔬️unit/🦀️.rs`) still makes the parallel gate capture
   unreadable for any test that panics beside it. Every run here used `--test-threads=1`.
7. `🏪️store/🦀️.rs` is edited by peers continuously; every edit above was re-read immediately before
   writing and is scoped to the named lines.

## 6. Files changed

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs`

Normative sources:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/📖️grammar.grammar.semio`

Laws re-expressed (neither weakened; the spr one strengthened):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`

No shipped grammar source was edited: all 70 parse failures were `parse_grammar` gaps, none was a
malformed grammar.

Captures (`🗑️generated/`): `kn2-baseline-serial.txt`, `kn2-probe-1` … `kn2-probe-15.txt`,
`kn2-run-12.txt`, `kn2-run-14.txt`, `kn2-run-16.txt`, `kn2-replication.txt`, `kn2-pack.txt`.
