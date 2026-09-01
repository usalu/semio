# Store Retained Decoder Readiness — Read-Only Audit 65

## Decision and Scope

The smallest next native RED packet is **exactly the two rejected-page wrapper laws**, using real pages, the existing record cursor, and the current public constructors. It does not require poison recovery, Fresh decoder changes, a new registry API, or changes to job/lease-return routing. Author and compile the tests before changing the two page-close branches.

This is source/readiness evidence, not a new native result. No Store, DSL, test-source, launch, or controller file was edited; no compiler or global inventory ran. This Markdown is the only new file. R17 `compile_dsl`/`print_mirror`, retained backbone/detach retirement, command, Interaction, and return-path implementation remain excluded. Existing return APIs were read only to specify valid test setup/teardown.

## Surviving Evidence and Unavailable Inputs

The following exact ticket paths were checked without following links or traversing their children:

- `🧪️store-owned-field-retirement-49`: **ENOENT**. Its reviewed registry, rejected-page, and Fresh-field prototype bytes, neutral fixtures, controller, and in-owner reports are not available at that owner.
- `🧪️os-kernel-fixtures-41/📜️script.ts`, `🧫️run-lw07Qh/🔣️.json`, and `🧫️run-QraYC8`: the `🧪️os-kernel-fixtures-41` ancestor is **ENOENT**.
- `🧪️store-owned-decoder-grammar-48/📜️script.ts`: its owner is **ENOENT**.
- Store `🧪️tests/🧬️owned-field-retirement`: **ENOENT**. This does not establish that a formerly mounted domain owner was deleted.

Nothing was recreated. Historical hashes cannot recover source bytes. The exact registry prototype path previously reviewed was `🧪️store-owned-field-retirement-49/🧪️registry/🦀️.rs`; this audit cannot reread it. The missing full49 packet also prevents a current byte-level check of its complete 20-group/17-law plan. Do not label a newly authored replacement as the unchanged reviewed prototype.

Surviving, fully read sibling reports:

- [Retained-field review48](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-decoder-retained-field-review-48.md): source findings, not native execution.
- [Concrete repair design](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-decoder-concrete-repair-design.md): original exact-authority design and case groups; later quarantine refinements below supersede any suggestion that bookkeeping validity alone permits normal callbacks.
- [Grammar preparation48](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-owned-decoder-grammar-test-preparation-48.md): records historical native RED then **four grammar tests GREEN** after real 4096-byte nonterminal padding and grammar repairs. Its raw runs are now unavailable at the checked paths. The four domain test bodies still exist; no test was rerun here.
- [Retirement preflight49](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️os-kernel-owned-field-retirement-preflight-49.md): **five source selections, zero native process receipts**, not five native passes. It retains historical registry prototype hash `d027384ca94fc69073081e82db034a6a6d88637ec6dbd0f59d522924c67579d5` and rejected-page prototype hash `8238284105cdcf78d63adac57cc28d3a6eae7517e1b9fcacef9ab62fb4a74cf3`, but not their bodies.
- [R17 coordination](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-r17-codec-backbone-coordination.md): separates the runtime Retained work from this unmounted packet.

The surviving [owned-schema-record native owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️owned-schema-record/🦀️.rs) is mounted near Store line19604. Its four laws execute actual record/nested cursors, semantic keys, cancellation, and page retirement. They do **not** execute retained Fresh-field callback unwind or registry poison quarantine. Its first-page whitespace padding is a valid construction reference, not missing49 ownership proof.

## Current Page Contract and Exact Gap

[Owned pages](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:4762) use fixed inline 4096-byte storage, with an actual length per page. A second page is admitted only when the prior page is full4096; only the terminal tail may be shorter. Page close pops the last admitted page. The [token cursor close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:5309) accepts a page count, not a byte count, and releases at most one page per call. [Record close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:5605) forwards that operation.

Consequently the wrappers must preserve the approved conservative **4096-byte grant granule**, even for a short tail. A successful page release reports the actual tail length; this is not authority to free that page on a tail-length-only grant. There is no new logical-tail liveness requirement.

| Current method | Actual behavior and missing law |
| --- | --- |
| [ArtifactEnvelopeDecodeRejected::close_step](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7822) | After field return/reclamation, calls `record.close_step(1)` without checking `maximum_bytes`. Positive items plus 0 or4095 bytes can release a page. |
| [ArtifactEnvelopeUnadmittedDecodeRejected::close_step](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7895) | After field close, calls `record.close_step(maximum_items.min(1))` without checking `maximum_bytes`. Same page-grant gap. |
| [Existing rejection tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:20650) | Exercise bounded ordinary close using4096-byte grants. |
| [Existing identity/rejection test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:20711) | Verifies original record/lease identity, `close_step(0,0)`, generation reuse, and4096-byte close grants. It does not test positive items with zero/short bytes. |

Both wrappers already reject zero items without releasing ownership and verify a delegated field owner's terminal witness before dropping it. The registered wrapper also waits for the exact ticket to be reclaimed. Those existing checks must remain. Reclamation is detached handoff, not proof that the detached field owner has completed its own close.

## Smallest New Native RED Packet

### Proposed New Files and Mount — Not Written

Use a **newly authored** domain owner, not the missing ticket owner:

- Store `🧪️tests/🧬️rejected-page-close/🧬️schema/🔣️.json`
- Store `🧪️tests/🧬️rejected-page-close/🔣️vectors.json`
- Store `🧪️tests/🧬️rejected-page-close/🦀️.rs`
- One `#[cfg(test)]` path mount adjacent to the existing owned-schema-record mount, with module name `owned_field_rejected_page_tests`.

Retain the two already selected exact law names:

1. `registered_rejected_pages_obey_zero_short_and_exact_grants`
2. `unadmitted_rejected_pages_obey_zero_short_and_exact_grants`

The neutral schema should close page-layout, wrapper-kind, grant-sequence, expected step/counter, and terminal fields. Authored expectations must distinguish the field-owner phase from the record-page phase. Validate it independently with Ajv; use the existing test-only JSON parser for semantic JSON expectations where applicable. Neither library nor a JavaScript ownership model proves Rust retention. The two native bodies must consume those same case records and execute the actual wrappers.

### Real Setup and Identity

Construct real `OwnedSchemaDecodePage` values and `OwnedSchemaDecodePages`, then the existing record constructor. Use a full4096 nonterminal page plus a short terminal tail where two pages are needed.

- Registered: [ArtifactEnvelopeDecodeAuthority::try_new](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7474) then its actual [reject](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7601) method; retain exact record storage address, ticket generation, and registry identity. Close the field owner through the actual current lease/returned-owner APIs. Do not modify those APIs or the job return path.
- Unadmitted: [ArtifactEnvelopeUnadmittedDecodeRejected::new](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7885) with the actual record and a genuine counted field owner.
- A test field owner may own a counted token and release it only under its real close grant. Its role is setup/teardown accounting, not a replacement implementation of either wrapper.
- Page values are `Copy` inline buffers. Original page identity means retained slot storage, ordering, page/byte counts, and original contents; do not claim a non-copy heap identity for each page.

### Required Cases for Each Wrapper

| Phase / grant | Desired observation |
| --- | --- |
| Record owns a full page; items0, bytes4096 | No page removed; exact storage/counts/content unchanged. |
| Record owns a full page; items1, bytes0 | No page removed; no byte/item release credited. |
| Record owns a full page; items1, bytes4095 | Same refusal. |
| Record owns a full page; items1, bytes4096 | Exactly one page removed; released bytes4096. |
| Full page plus short tail; items1, bytes0/4095 | Neither page removed, including the short tail. |
| Same pair; items1, bytes4096 | Tail removed first; released bytes equal its actual length, not4096. Next sufficient grant releases the full page. |
| Multiple items and8192+ bytes | At most the actual cursor's one-page step; grant must never be exceeded. Do not require new batch-close behavior. |
| Positive grant after all owners are terminal | Repeated Complete, no additional release or callback. Existing zero-item Pending behavior is not redefined. |

Record observed mismatches before final assertions, then drain through sufficient bounded grants so an expected RED does not abandon live wrappers and turn a useful assertion into a secondary Drop panic. Assert exact owner-token drop/close counts and terminal emptiness after teardown. No `mem::forget`, suppressed Drop, replacement owners, or unbounded cleanup.

### Eventual Minimal Production Footprint — After Actual RED

Only the record-page branches of the two `close_step` implementations above need an initial byte-granule guard before invoking the page cursor. Do not put a blanket byte guard ahead of legitimate shallow field/lease work. Do not alter the cursor's page-count contract, tail granule, field-return implementation, or final terminal checks.

This audit predicts the zero/4095 failures from the actual source. It has not run either law. Mounting and native compilation still require the root's explicit source/compile coordination.

## Registry Poison Cohort Is Separate and Still Unmounted

The current [registry](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6613) retains its boxed owner structurally while a `with_owner` callback runs under the mutex. If that borrowed callback unwinds, the mutex is poisoned. Current `try_admit` (6701), `with_owner` (6729), and `take_returned_ticket` (6744) map any failed `try_lock` to Contended. There is no private Normal/Close policy or validated poison recovery: subsequent normal and close access remain indistinguishably contended.

The three historical preflight names remain useful acceptance targets, not available test bodies:

- `registered_decoder_unwind_uses_validated_bounded_close`
- `poisoned_registry_quarantines_admission_and_publication`
- `poisoned_registry_rejects_invalid_bookkeeping_without_owner_loss`

A later newly authored cohort must use the actual registry, lease, returned close owner, exact generations, and counted owned tokens. Its essential postconditions are:

- Catch unwind around a borrowed real callback; original owner remains in its exact slot and is later serviced by bounded actual close.
- Poisoned owners remain close-only. Bookkeeping validity alone does not validate the callback-mutated owner or authorize admission/accept/finish/publication.
- Validate real free-list uniqueness/range, live/slot accounting, generations, and return state before close/witness/detach. Invalid bookkeeping retains every owner.
- `return_now` updates `returned[index]` and `returned_mask` separately. A transient pair mismatch is inconclusive and requires stable observation/retry, not an invented corruption verdict.
- Use existing return-to-detach-to-close ownership. Detachment is not nested completion. Do not edit the return path as part of this packet.
- After every quarantined old owner has detached into close-only ownership, the registry must become empty and revalidated; then a fresh admission must actually succeed with a new generation, normal callback execution, and bounded close. Permanent quarantine would fail the law.
- White-box invalid-state tests may restore the exact saved bookkeeping solely for honest teardown. Restoration is not a production recovery success.

Expected later implementation scope is the existing registry state/access family and its private validation/access helper, not a new public registry family. It must be reviewed before mounting. The missing prototype cannot establish current compilation compatibility.

## Fresh Original-Owner Retention: Current Gaps and Next Test Boundary

The current [FreshFieldDecoder](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8775) and [FreshVcsAuthority](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8454) still implement take-before-fallible dispatch. This is not repaired by the grammar4 work or by the wrappers' eventual byte guard.

| Authority / method | Current exact ownership problem |
| --- | --- |
| FreshField `begin_field` (8732) | Reserves the VCS target before `catalog.begin_vcs`. A factory unwind can leave a live reservation with no active authority. Only the reservation is recoverable here; no returned factory owner exists to resurrect. |
| FreshField `accept_field_token` (8775) | Takes `active` before String/Empty/VCS work. VCS accept/publish Err, unwind, or unexpected RecordComplete loses the outer reachable active owner. Terminal publish is accepted without checking exact target publication/reservation and authority-empty witness. |
| String branch | Partial scalar storage is genuinely inline, not a forgotten heap collection. Losing active state is distinct from losing a completed String. Completed schema/id replacement lacks vacancy preflight. Tests must use the real String authority and not fake a String callback that the API does not expose. |
| FreshField `finish_record` (8840) | Takes schema, then id, then VCS. A later missing slot loses earlier values. Required slots and active/reservation state need preflight before the first take. Ordinary completed-registry refusal already returns and restores the exact pending owner; retain that behavior. |
| FreshVcs `accept_token` (8454) | Takes active before snapshot callbacks or any of four actual history-array authorities. Snapshot ownership remains in its separate field, but its phase/reservation is lost on failure. For Edits/Changes/Checkpoints/Alternatives the local active owns the array authority itself. |
| FreshVcs nested Complete (8550) | Sequential snapshot/edits/changes/checkpoints/alternatives takes can consume earlier owners before a missing later slot. Require all-slot preflight before assembly. |
| FreshVcs `close_step` (8580) | After closing the separately stored snapshot, `if let Some(Snapshot {..}) = self.active.take()` also consumes and drops a non-Snapshot active history arm on a failed pattern match. This obstructs genuine cleanup of history-first failure cases. |
| FreshField `close_step` (8875) | Clears active before fallible reservation cancellation; an orphan reservation with no active arm is not discharged. Final close clears both schema/id Strings on one positive-item grant and sets terminal without proving all owners empty. The separate `owners_terminal_empty`/Drop assertions remain, and must not be weakened. |

Next Fresh tests must execute these concrete authorities, not a parallel transactional decoder:

- Real String Pending→Fault/cancellation state retention; completed schema/id pointer/content retention when a later required slot is absent. Clearly label public wire sequences versus white-box slot-absence probes.
- VCS accept and publish callbacks returning Err or unwinding **while the original object is still borrowed**, with exact active owner identity, reservation, callback count, and later bounded close.
- Actual FreshVcs snapshot callback paths, all four active history kinds, and terminal assembly missing each required slot. Edits exposes an injected entry-decoder seam; the metadata history variants must use their actual repository authorities, not pretend they have arbitrary callbacks.
- False terminal/RecordComplete results cannot discard an owner or bypass the terminal witness.
- Close must retain and service the same history owner even while the separate snapshot owner is being retired.

These later source joins are limited to FreshField `begin_field`, `accept_field_token`, `finish_record`, `close_step`; FreshVcs `accept_token`, `publish_reserved`, `close_step`, and narrowly needed existing close helpers. They are **not** approved writes from this audit. Any test setup that requires excluded return/publication implementation changes must be split out and coordinated.

The current retirement-factory and publication-target traits take values by value. If a callee consumes a value and unwinds after losing it, a caller cannot reconstruct the original owner. No test or report may credit by-value callback resurrection, hidden clone fallback, or weaker Drop behavior. Borrowed callback unwind is testable; consumed-factory unwind is a concrete contract boundary, not missing test cleverness.

## Capture Receipt

Read time endpoint: `2026-08-28T01:28:10.787Z`. Fixed input reads used case-insensitive lexical Compose exclusion, complete no-symlink ancestry checks, O_NOFOLLOW/fstat checks, and before/after endpoint identity. No returned candidate paths, directory census, or nested repository probes were used.

Store first and final fingerprints are identical:

- SHA256 `7450f9d6837055d0766a55c5fc98aae22d068ac813acda09c1385a1df48d4c9c`
- Bytes1540921; device16777230; inode122164261.
- mtimeNs/ctimeNs `1787878873572631738`.

Exact slices are UTF-8 encodings of inclusive source lines joined by LF, without an added trailing LF. Every listed slice hash matched its first read at the endpoint:

| Lines | SHA256 |
| --- | --- |
| 4757–4940 pages | `779cb13b463ceba41994dfbbcb267568d8b94b0710bd31c258989f12f28d03ee` |
| 6613–6849 registry/lease | `3f3e8dbdd56405812372d883b4d64b7c98b60deb806a676a545816666736156d` |
| 7800–7944 wrappers | `facee6862391d381b79e38ec588cc8c258d25714860b49d245ca6f8a2019abdb` |
| 8234–8452 FreshVcs helpers | `b0b718e4cc4b3bda210fe9a50740a12cd03b1280081f6fef048b47ee8e606997` |
| 8453–8620 FreshVcs methods | `906e5c0cddb7152daad2f1c3aa2786c05b56459d05e996dd90bd8400d504dd22` |
| 8620–8955 FreshField | `ab8eabf4c14a103c589987521701eea57cccc58a8bc1b67d218d6585c37c2d58` |

Surviving fixed-input first/final hashes also match:

| Input | SHA256 |
| --- | --- |
| retained-field review48 | `c7cc9c60e2c3ba01c1c8c5cc462fa4065cd39eaf1b40895cf79266e0e9f70dd6` |
| concrete repair design | `2179d22c9cf5736c13acaac655c1b5c2fe725b902e67336ea8f2e1f020e72e43` |
| grammar preparation48 | `d24675deadbe4c3d0f90223a72720995e6a959eb9fd68924a198f17b06f42356` |
| retirement preflight49 | `1df14c5794b7b4346d949e3924b1b9415072cccc03c997c4ef9a781c9454efc5` |
| R17 coordination | `1fcac5ad7aac290ba8798fe9adcd7110e54b66ac79c11a8fac3984b047a0fe56` |
| current grammar native source | `c9e5ab882e662e74470a5604de972e3828e5037f2a3a5177ba79d0825f380047` |
| current grammar vectors | `5372c7cf90be588f5394ce0d326e4fed0e3b5697cbe77c410418811b07ba2d50` |
| current grammar schema | `49c3150ef1661630178e0b71855ee5a66fa2dd6631276d4580c12d1bfc6046d1` |

No whole-Store drift was observed during this audit. Future R17 changes may legitimately alter the whole file; coordinate against the exact method slices rather than restoring this historical whole-file hash.

