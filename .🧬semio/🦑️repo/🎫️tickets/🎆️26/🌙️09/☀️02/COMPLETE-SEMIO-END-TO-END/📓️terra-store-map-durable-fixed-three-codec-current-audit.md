# Store Map Fixed-Three Durable Decision Codec — Current Audit

Audit date: 2026-09-05. Read-only inspection of the current Store codec, neutral fixture/oracle, source registration, and selected native-law selectors. No build or product edit was performed.

## Result

The new module is a useful **fixed-shape envelope**, but it is not yet a safe durable Map decision admission. It correctly prevents role swapping and the obvious forged child/owner forms, and it uses an acyclic *field selection* for `decision_sha256`. It does **not** yet establish that an unsigned member hash is the unbound form of its bound Store outcome, nor does its `admit_map` query Store-owned state. Treat the codec as pre-integration scaffolding until the first two blockers below are closed.

## Confirmed working structural fence

| Concern | Current evidence | Assessment |
| --- | --- | --- |
| Fixed membership/order | Individual `parent`, `drawing`, `value` fields and the unsigned projection preserve that order at [`durable-group/🦀️.rs:78-97`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:78). | Good: a generic/reorderable member vector is not exposed. |
| Map role/owner fence | [`member_matches`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:173) fixes Map parent dialect, the two stdio URIs, `parent.owner == None`, and child owner `(parent, slot, childId)` at [241-250](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:241). | Good for this V1 envelope; image has no member slot. |
| Acyclic field selection | The unsigned type contains only role/ref/owner/base/recovery-schema/unbound hash at [64-84](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:64); it excludes bound packs, post frontiers, and seals. | Correctly matches the packet's requirement to omit all group-derived edit/prefix/revision facts. This is necessary, but alone insufficient. |
| Per-member early sealing cap | `seal` now rejects a `>162,000` input before hashing it at [192-199](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:192). | A real improvement for an already-owned caller buffer. |
| Outer canonical checks | JSON reprint and pack re-encode checks are at [221-234](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:221). | They reject ordinary JSON aliases/unknowns/suffixes after parse; they are not a bounded pack-container admission (below). |

## Blockers

### P0 — `unbound_outcome_sha256` is an unchecked caller assertion

The decision preimage is acyclic, but each `unbound_outcome_sha256` is only tested for lowercase-hex syntax at [`251-258`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:251). The Rust path never decodes a member recovery outcome, clears its single `MutationMeta.group_id`, canonically encodes the resulting `UnboundOneItemOutcomeV1`, or compares its digest to the field. `seal` accepts arbitrary member `recovery_pack` bytes plus an arbitrary hex commitment at [192-204](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:192).

The neutral fixture makes the mismatch conspicuous: it carries `unboundOutcomeHex` and its hash [107-120](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️fixtures/🧬️.schema.json:107), and Bun verifies those bytes [20-39](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts:20), but the native fixture loader discards `unboundOutcomeHex` entirely [334-347](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:334). Therefore a changed bound recovery outcome can retain an unrelated valid unbound hash and a self-consistent new `decision_sha256`; current `validate` admits it.

This is exactly the gap the decision packet warned against. The correct binding order remains:

1. Store owns three prepared candidates with sole metadata `group_id=None`.
2. Store's three registered outcome codecs emit canonical `UnboundOneItemOutcomeV1` bytes and computes the three hashes.
3. Store derives `decision_sha256`, then performs only `None -> decision_sha256`, recomputes edit digest/prefix/post revision, and emits bound packs.
4. Replay decodes each registered bound pack, reconstructs that unbound projection, checks all three hashes and the decision digest, then reconstructs the private Store seals.

Do not hash a "recovery pack with a field removed": current edit digest and post revision transitively contain `group_id`, as documented in the prior [decision packet](terra-map-durable-group-decision-codec-current-packet.md:82).

### P0 — the public admission is caller-supplied, not repository-owned

`durable_group` is a public Store module [30-31](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:30). Its decision/member fields are all public [24-43](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:24), and public `seal` takes caller-built members [192-204](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:192). `admit_map` similarly trusts two caller-built `ArtifactRef`s and six caller-built frontier fields [130-144](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:130), rather than reading the parent snapshot/child envelopes and Store cursors under their retained owner.

There is currently no consumer outside this module/tests (`rg` finds only its source and its selector registrations), and none of `ArtifactStore`'s retained one-item preparation, group visibility owner, journal port, or replay path calls it. Thus it must not be described as an "admission before journal" yet.

The smallest clean surface is an opaque public **verified decision**, with private member fields, returned only by a Store coordinator:

```text
ArtifactStore::prepare_owned_map_three(... prepared parent/drawing/value ...)
  -> DurableOwnedMapThreePrepared (Store-owned)
DurableOwnedMapThreePrepared::seal_for_journal()
  -> DurableOwnedMapThreeVerifiedDecision
Store::recover_verified_owned_map_three(decision, actual three stores)
```

The DB journal port receives/verifies bytes through Store and receives no mutable member records; plugins never call `seal` or supply handles/frontiers. That preserves the already-correct DB -> kernel direction and prevents an API compatibility layer.

### P1 — a 480 KiB compressed input can select the default 4 GiB Pack decoder budget

`decode_canonical_pack` checks the *stored* input length then calls `ArtifactPack::decode_pack` with `PackDecodeOptions::default()` [229-233](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:229). Those defaults allow a 16 GiB pack file, 256 MiB segment and 4 GiB total allocation [62-75](../../../../../../../../../🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs:62). The normal encoder selects `CodecId(1)` [2044-2046](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:2044), which is Deflate [381-405](../../../../../../../../../🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs:381). `decode_document` opens the pack then materializes/decompresses document frames before constructing the record [2129-2137](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:2129).

This defeats the claimed pre-allocation `491,520` decision cap: a short compressed hostile body can declare a much larger raw document segment. `PackFile::body_bytes` also concatenates each `doc_frame_count` segment without an aggregate output check [1037-1058](../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:1037).

Use a dedicated decision pack opener before any `body_bytes` call: `max_file_len = max_segment_len = max_total_alloc = 491_520`, exact small symbol/item/depth limits, no chunks, `manifest.doc_frame_count == 1`, `manifest.uncompressed_body_len <= 491_520`, and an exact decision schema/hash/field layout. Then decode the one body with unknown-field rejection and require every byte canonical. The manifest exposes both required fields at [`423-435`](../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:423). A bounded decoder must be the only path used by journal replay; the generic `ArtifactPack::decode_pack` trait remains unsuitable as an authority boundary.

### P1 — the claimed eight-case corpus is mostly labels, not executable behavior

The Bun oracle verifies fixture schema/digest arithmetic and that case IDs/variant strings equal expected literals [41-59](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts:41). The native law only asserts `cases.len() == 8` [361-388](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:361). It does not execute `only-none-to-decision-group-binds`, `mixed-recovery-is-corrupt-not-compensated`, or either recovery outcome byte vector.

The present two native selectors are correctly registered in the Nx target [12-19](../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json:12) and script [151-170](../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:151), but they prove only codec metadata today. The pre-existing report's source-green/native-pending assertion was not rerun for this audit.

## Exact next laws

1. **`bound_outcome_reconstructs_its_unbound_preimage_before_decision_admission`**: produce three real registered Store outcomes, derive the decision, mutate any group-independent edit/snapshot byte in one bound pack while recomputing its raw pack hash, and reject because regenerated unbound hash differs. Assert no Store group reservation/journal call.
2. **`only_store_can_late_bind_none_to_decision_once`**: start actual three retained prepublication owners; reject foreign/non-`None`/two-meta outcomes, bind all exactly once, and prove final edit/prefix/post revisions are Store-recomputed rather than caller accepted.
3. **`durable_decision_rejects_deflate_expansion_before_document_body_allocation`**: a stored `<=491,520` pack declares raw document bytes `491,521` (and a multi-frame aggregate case); the bounded opener rejects before `body_bytes`/outcome decode.
4. **`durable_decision_three_member_capacity_exact_and_plus_one`**: encode three real members at the accepted aggregate limit and one `+1` record. The latter rejects before journal reservation and recovery/outcome copies. Keep the existing per-member `162,001` law as a separate vector.
5. **`replay_mixed_three_frontiers_is_corrupt_and_invisible`**: use actual parent/drawing/value stores and prove all-base or all-recorded-post is the only admissible state; a parent-only or children-only post state creates neither undo compensation nor visibility publication.

## Bounded handoff

The immediate small corrections are: (a) do not expose `seal`/caller `admit_map` as the durable API; (b) add the bounded one-frame Pack preflight; (c) make the neutral and native fixtures run real unbound/bound outcome transforms rather than hash literals. The full member outcome codecs and Store late-binding/recovery remain the next required implementation slice, not an optional hardening pass.

## 2026-09-05 Addendum — Canonical JSON Prepared-Outcome Carrier

This is a read-only review of the current JSON-carrier revision. It corrects two now-stale observations above: the decision/member fields and operational helpers are now `pub(crate)`, and both the decision and unbound-outcome Pack openers now use explicit one-frame, no-chunk, full-verification limits. No build was run.

### Result

The replacement of a `Shape::Value` carrier with `next_clock_canonical_json` and `edit_without_group_canonical_json` **does preserve exact signed/unsigned 64-bit integers** for the Store-owned unbound outcome. It is a material correction to the earlier dynamic-value loss. It is not, by itself, a durable-outcome authority: the sealed decision still accepts a separately supplied member recovery byte string and separately supplied base/post frontiers. Therefore it cannot yet claim that the persisted recovery body, its hash, or its frontiers came from the authenticated prepared Store outcome.

| Question | Current evidence | Assessment |
| --- | --- | --- |
| Exact numeric representation | The JSON implementation carries `Number::UInt(u64)`, `Int(i64)`, and `Float(f64)` separately, writes the first two as exact decimal, and parses them back before an overflow falls back to float at [`pack/json:70-111,628-693,1041-1052`](../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:70). `u64::MAX` and `2^53+1` already have value-codec laws at [`value/codec:518-529`](../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:518). | Good for integers in `[i64::MIN, u64::MAX]`; a JSON number outside that interval is intentionally a float, not an arbitrary-precision integer. |
| Typed Edit survives the carrier | `from_prepared` emits the actual `Edit<Mutation>` through that JSON writer, then `verify_inverse` parses the JSON back to the same typed `Edit<Mutation>` and requires byte-for-byte canonical reprint at [`durable-group:233-263,279-317`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:233). The underlying `Edit` codec preserves its typed generic forwards/inverse and `MutationMeta` fields at [`replication/mutation:1531-1618`](../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1531). | Good: `2`, `2.0`, `2^53+1`, `u64::MAX`, and negative integers do not silently collapse while the JSON is round-tripped. The canonical reprint also rejects aliases such as a float spelling for an integer field. |
| Store-authenticated source | The unbound constructor takes an `ArtifactStoreOneItemPrepared`, requires its private authority/seal to validate, serializes its exact edit/post snapshot, re-inverts the result, and compares typed edit/snapshot outputs at [`durable-group:218-264`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:218). The Store seal itself checks pointer identity, immutable authority, exact meta, actor, sequence, and one-forward shape at [`store:13121-13182`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13121). | Good, but only for `DurableStorePreparedOutcomeV1.pack` and its SHA. |
| Outer Pack preflight | Decision and unbound outcome require exact envelope/schema, one document frame, no chunks, bounded raw length, Full verification, and canonical re-encode before returning at [`durable-group:175-214,484-520`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:175). | The prior default-decoder/deflate-expansion blocker is closed for these two opener paths. |
| JSON depth/control/allocation | The Pack layer limits file/segment/raw/body allocation before record decode. The subsequent JSON parser has its own fixed depth ceiling of 128 and rejects literal control bytes at [`pack/json:33-67,822-902`](../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:33). It has no carrier-specific item or allocation budget, and escaped controls are permitted as normal JSON characters. | Finite at 162,000 bytes per unbound Pack, but not a fully budgeted JSON admission. Do not describe it as enforcing the Pack's depth-32/item budget *before* JSON-tree allocation. |

### P0 — decision member recovery bytes and frontier are not derived from the verified outcome

`seal` receives three full `DurableOwnedGroupMemberV1` records in addition to the verified `outcomes`. For each pair it checks only recovery-schema equality, the unbound pack's self-hash/canonical decode, and then copies `outcome.sha256` into `member.unbound_outcome_sha256` ([`durable-group:431-455`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:431)). It does **not**:

- derive `member.recovery_pack` from an unbound or late-bound typed Store outcome;
- decode/validate that recovery pack in `seal` or `validate` (the latter only checks its SHA at [537-543](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:537));
- compare `member.expected_generation` or `expected_revision` to the verified unbound authority's generation/base revision; or
- derive/recompute `post_generation` and `post_revision` after the sole `None -> decision_sha256` late bind.

The checked-in neutral fixture is an executable minimal witness: each `recoveryPackHex` is literal ASCII such as `bound-parent-create-region-v1`, not an ArtifactPack, while a different genuine `unboundOutcomePackHex` is used only to calculate the unsigned hash ([`fixture:26-37`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️fixtures/🔣️.json:26)). The Rust `decision()` feeds those arbitrary recovery bytes and genuine outcomes into `seal` ([`durable-group:699-721`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:699)), and the current decision law expects admission. Thus the carrier proves an *unbound* SHA exists, but the record it would persist is not recoverable as a typed Store result.

A second minimal vector is fully source-local: start from the existing `outcomes()`, change `parent.expected_generation` and `post_generation` together to preserve the `+1` check, replace its expected/post revisions with two unequal 32-byte values, then call `seal`. The unbound parent authority remains at its original base, but [`member_matches`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:412) and `seal` admit the contradictory claimed frontier. This is an authority gap even though all structures are now crate-private: the future Store coordinator is the first `pub(crate)` caller and would be able to make this mistake.

The correct Store-owned API remains: consume the three retained `ArtifactStoreOneItemPrepared` owners; derive their unbound JSON Packs and hashes; derive `decision_sha256`; late-bind only `group_id`; recompute each bound edit digest/prefix/post frontier through Store; and have that coordinator construct the three bound recovery packs and decision members. `seal` should receive that one private aggregate, not prebuilt `member.recovery_pack`/frontier facts. A replay path must decode each bound typed pack, remove only its verified group id to regenerate the stored unbound Pack hash, and recompute the final frontier before visibility staging. This addendum makes no durable-publication claim.

### P1 — remaining JSON-admission and corpus limits

The Pack bound is real: `decode_document` materializes its document body before type construction ([`pack/value:2124-2140`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:2124)), while this carrier limits an unbound one-frame raw body and its Pack allocation to 162,000 bytes before that call. However, `verify_inverse` then invokes the whole-document JSON parser for the clock and edit ([`durable-group:279-287`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:279)). That parser allocates a JSON tree without a supplied item/allocation limit. Its fixed 128-level guard is useful but is not the Pack's 32-level policy. This is bounded by input bytes, so it is not a decompression-size bypass; it is a resource-accounting omission worth closing before the record is exposed to hostile replay bytes.

Likewise, raw controls are rejected by JSON grammar, but `\u0000` becomes an ordinary string character. `valid_identity` bans such characters only for decision identities ([`durable-group:388-398`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:388)); the one-item authority checks a nonempty/256-byte `Edit.id` but not controls ([`store:13121-13135`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13121)). If history/edit identity forbids controls, enforce that semantic rule before serializing a prepared outcome and after typed recovery parsing; otherwise document escaped controls as deliberately accepted content.

The four registered native laws include a typed round trip, canonical-order mutation, deflate expansion, and top-level capacity ([`os/rust script:166-188`](../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:166)), but their prepared fixture uses small ordinals and string mutations ([`durable-group:655-696`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:655)). It does not exercise the regression it is meant to prevent.

### Exact next laws

1. **`durable_unbound_outcome_preserves_uint_int_and_float_kinds_at_numeric_boundaries`** — use one actual sealed `ArtifactStoreOneItemPrepared<DslValue, DslValue>` with `2`, `2.0`, `9_007_199_254_740_993`, `u64::MAX`, and `i64::MIN` in the typed edit and all three HLC `u64` fields. Assert the canonical JSON spellings and `verify_inverse` recover the same `Number` variants, not merely `as_f64` equality.
2. **`durable_group_seal_derives_all_member_bases_and_recovery_bytes_from_prepared_store_owners`** — current literal `recoveryPackHex` fixture must reject. The accepted row must use the future Store-generated bound outcome, prove its stripped unbound JSON Pack hashes to the member commitment, and prove the base/frontier values are Store-derived.
3. **`durable_unbound_json_rejects_depth_item_and_escaped_control_policy_before_typed_edit_admission`** — make the chosen policy executable: depth 129, an item-heavy but byte-valid edit JSON, raw control, escaped NUL in `Edit.id`, and escaped NUL in a permitted mutation payload. Reject/accept each before Store staging with an explicit retained allocation budget.

### Handoff

The JSON carrier is the correct lossless representation for current 64-bit Store values. Keep it, keep the bounded Pack opener, and move member construction/recovery binding into the Store-owned fixed-three coordinator. Do not qualify the decision envelope as a durable Map publication until the P0 source relationship (prepared outcome → unbound commitment → Store late bind → bound recovery Pack/frontier) is present.
