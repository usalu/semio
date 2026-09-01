# Store Sync Outer-Fixture Reference And Source Join — 67

## Result And Limits

The schema-first neutral reference ran through Bun/Nx **151/151 PASS, exit0** before any Rust edit. Runtime then explicitly approved the exact outer cfg(test) join. The approved source patch is now mounted: fourteen `async ` qualifiers and sixty-five named `.await` tokens removed, plus one fresh-channel-test-local `Backbone` import. Actual post-source bytes match the predicted patch; reversing precisely those changes in memory reproduces the complete original byte image.

No Cargo, rustc, native test, wire generator, full outer-test namespace, production Store/Sync behavior, descriptor/default, actor fixture, launch, or Retained ownership repair ran or changed here. **The historical84 compiler diagnostics have not been rerun and are not claimed resolved by compilation.** No new domain/native test was mounted.

The preparation [plan67](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-sync-outer-fixture-os6-plan-67.md) remains unchanged, SHA `dc5e7d3cae080faf53f9022771342c337fece76d1b6cae74efad994a7eff392f`. Root independently reviewed it in [join-review68](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-sync-fixture-join-review-68.md). Runtime's subsequent bounded source approval is the authority for this patch; this report supersedes preparation-only status, not the original evidence.

## Authored Neutral Laws

The new ticket-owned [schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/🧬️schema/🔣️.json), [vectors](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/🔣️.json), and [controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/📜️script.ts) contain:

| Roster | Count | Authored coverage |
| --- | ---: | --- |
| SetN | 15 | Original values0,1,2,5,6,9,42; equal-value0/5; negative; i32 MIN/MAX; exact pre-state inverse and no messages |
| Diff apply | 6 | None preserves0/MIN/MAX; Some replaces with0/MIN/MAX |
| Ordered absorb | 7 | All four None/Some pairs, later None preservation, MIN/MAX replacement, four-step sequence |
| Stored inverse sequence | 5 | Original0→1→2,0→5→9,0→1→42; equal5; MIN→MAX→MIN→0; authored forward snapshots and reverse undo traces |
| Closed-schema rejection | 22 | Missing/unknown/wrong-type fields, wrong semantic operation, i32 overflow/fraction, wrong case kind |
| Raw JSON rejection | 3 | Duplicate key, trailing comma, comment |
| Wrong-expectation rejection | 4 | Post-state inverse, Some0 treated as None, later None erasing Some, forward undo order |

There are33 semantic cases. Every expected result was authored from the reviewed scalar contract, not copied from running Rust. The four deliberately wrong expected-output cases are schema-valid and must disagree with the independent reference.

The schema is the **closed normalized test-data contract**, not a new assertion that the existing Demo serde derives reject every unknown field or missing Option field. Current Demo types do not declare deny_unknown_fields, and serde permits omission of an Option field. The neutral fixture spells Diff None explicitly as `{"n":null}`; its missing/unknown-field rejection cases validate fixture discipline only. No stricter production serde behavior is claimed or introduced.

The actual retained Demo semantics are replacement, not arithmetic delta: `Some(n)` replaces, `None` preserves; later Some wins absorption; later None leaves the earlier diff intact. SetN produces Some(target) and no messages. Its sole inverse stores the pre-state n. MIN requires no unary negation. For multiple stored operations, inverse groups remain in forward storage order and the undo trace traverses the flat stored list backwards.

## Independent Reference And Actual Execution

The test-only third-party reference uses installed Decimal.js for exact signed scalar arithmetic and affine diff composition. Replacement is independently evaluated as `base + (target - base)`; None/Some are represented as affine identity/constant maps, and absorption composes those maps in order. Each final scalar is checked as an integer within i32 bounds. The authored expected object structure and empty message list remain the fixture contract, not a Decimal.js feature.

Ajv2020 validates the full schema with `strict:true`. jsonc-parser checks every authored JSON tree and rejects the explicit duplicate-key/comment/trailing-comma cases. No runtime dependency was added. This proves numeric/reference and fixture-shape agreement, **not Rust ownership, codec bytes, async behavior, queue behavior, or complete mutation metadata**.

Executed once before source mutation:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/📜️script.ts" reference
```

Actual receipt: [run-dwolJg JSON](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/🧫️run-dwolJg/🔣️receipt.json), SHA `fe4c445daea8442d6ba66eee0fa2546cf6abf17c57b7b22cd232c43a7f7ad336`,182678bytes, success=true, failure=null,151/151. The same complete receipt is retained in [run Markdown](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/🧫️run-dwolJg/📓️receipt.md) and [unique ticket sibling](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-sync-outer-fixture-reference-67-dwolJg.md). The151 checks include62 semantic/rejection checks and89 schema, unique-ID, exact-token, before-image and capture checks; they are not151 native laws.

Ten inputs were captured before and after the reference run with stable identities/hashes. The controller finds the workspace by a .🧬semio ancestor, refuses any-case Compose path components before probing, checks the complete ancestry without following symlinks, uses O_NOFOLLOW/fstat/endpoint checks, and checks exact UTF-8 byte preservation. The receipt records O_NOFOLLOW availability rather than assuming it. Exact input bytes are retained under input01–input10 in the exclusive run.

This controller is deliberately a **pre-write reference and exact-before-image gate**. It retains the original Sync hash/token anchors. After this authorized source change, it must not be relabeled a post-cutover source acceptance gate or silently rewritten to accept a new preimage. It was not rerun after mutation. The post-source evidence below is an independently executed exact-byte proof, not a rewritten baseline.

## Exact Source Patch

Only [Sync outer cfg(test)](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3620) was changed. The complete [applied patch](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/🔀️source-join.patch) and [post-source proof](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/🔣️source-proof.json) retain all80 ordered edit records, byte locations, before/after identities and endpoint captures.

| Source endpoint | Bytes | SHA-256 |
| --- | ---: | --- |
| Before; identical to retained input03 | 268230 | `37012443ee787d1a05e7826e8c3a8ac35ea0be6d43eba6918dcd7076c15e8d93` |
| After | 267795 | `62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6` |
| In-memory inverse of actual after | 268230 | `37012443ee787d1a05e7826e8c3a8ac35ea0be6d43eba6918dcd7076c15e8d93` |

The79 approved deletions total474bytes. The local import plus LF is39bytes. Net change is−435bytes; no other byte was changed. The original standalone MutationOutcome await line retains its indentation and LF deliberately: this packet did not authorize formatting. There are77 changed original lines and one inserted line; the two double-await lines remove both explicitly reviewed tokens. The retained patch text has a final LF; its SHA is `40cebde7a0320b930c183a6bf9ef45fe6579728a354af391c7b68cec7da781c0` (31887bytes). The applied patch string before that evidence-only final LF had SHA `779a650f0e5db30c024ae894b880deaddb36dc04f67423b89f229ded9d404692`.

Fourteen definitions, names and all bodies preserved:

- DemoSnapshot / ArtifactDsl: envelope_id, parse_dsl, print_dsl.
- DemoSnapshot / ArtifactPack: encode_pack_with, decode_pack_with, record_spec.
- DemoDiff / MutationDiff: apply, absorb.
- DemoMutation / OpText: parse_op, print_op.
- DemoMutation / OpBinary: encode_op, decode_op.
- DemoMutation / Mutation: diff, inverse.

The complete65-call roster is the vectors' preservation.tokens and original plan table. No global await transform ran. register_document_codec, ArtifactStore::new, dispatch, session receive/new, host/endpoint/storage operations, print/parse document operations and all other genuine async sites retain their original tokens. All original literals, expectations, assertions, serde/DSL attributes, values, wire ordinal/format logic and inverse expressions are otherwise byte-identical.

The single added `use crate::os_store::Backbone;` is local to `artifact_mailbox_nested_identifier_bytes_and_backbone_one_pop_preserve_ownership_order`. It exposes the actually implemented existing trait method; it does not reintroduce an API. Both original awaited sends, ACK owners["first"]/["second"], and three one-pop assertions remain exact. Current send still takes ownership by value and may return an error without that message; this fresh-success test is not refusal, unwind, nonblocking or retained-send proof.

## Preserved Excluded Boundaries

The actual source proof constructed the expected after image from the reviewed old byte spans and compared it to the filesystem readback, then reversed those exact replacements and compared every byte to [retained input03](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-outer-fixture-67/🧫️run-dwolJg/📄️input-03/🦀️component.rs). No source was restored. This is stronger than a token-count or substring-presence check, but still not a compiler result.

All slices below use LF-split rows joined with LF, no added trailing LF:

| Preserved slice | Old → new lines | SHA-256 |
| --- | --- | --- |
| Production and inner-test prefix | 1–3619 unchanged | `953cddbdb2c572de4e846b578f6ab74c53791f8a945d3b0b6f9aea174b7b504d` |
| Retained fixture_runner_handle definition | 2538–2543 unchanged | `26ad55d9a6b128faa74daa3ef0b0717a3daa504b13da59e359067bd7df4c43e2` |
| Entire reserved heartbeat caller test | 4175–4195 →4176–4196 | `768897dacc0023500820de20ee58a7370b38eca3cde22d053a3c6172184c242d` |
| Reserved caller exact line | 4181→4182 | `a8a58e96c1c0c072e180847c700f11bfb53c29864af7d9abebe2abc466cbb340` |
| ACK owners and one-pop assertions | 3719–3724→3720–3725 | `c6080b28f1129273b0da75c60d70a811694bf81093c1ef58586b34b889b68794` |

The whole native_actor::retained_turn_fixtures region is inside the unchanged prefix. No WorkerSubmitError, return/detach/backbone, R17 codec, Fresh, registry or Interaction source was touched. Post-patch endpoint capture found the other nine reference inputs unchanged, including Store, both mutation/codec authorities and the original schema/vectors/controller/report.

## Store Drift Attribution Now Known

Plan67 correctly recorded the then-unattributed whole-Store drift as historical observation; that text remains unchanged. Root subsequently identified and proved its cause in [rejected-page mount68](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-rejected-page-close-mount-68.md), SHA `965afcdfd49a88610c5cc3753b6f1ea42d57cd45f88f84eac52c1065249a967c`: its separately approved114-byte cfg(test) child include changed Store1541032bytes/`ed1d6b93b36a07f3c2aa914350c97f993613bc1a779e3b019a8f1329c7e19a37` to1541146bytes/`7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4`. Root's inverse proof restored the former hash in memory. That mount was not authored here. This new reference and source-proof capture both bind the later7c71 endpoint.

## Proposed Native Consumer Footprint — Not Mounted

The existing original fixture tests remain the native behavioral authority. Root/runtime own the next serialized compile and selected-law execution. The current source join adds no new test API or metadata.

If root later authorizes direct neutral consumption, the proposed small owner is `Store/🔄️sync/🧪️tests/🧬️demo-sync-trait-laws/{🧬️schema/🔣️.json,🔣️.json,🦀️.rs}`, mounted only inside the outer tests after DemoMutation's implementation so it can use those actual private fixture types. This is a proposal, not a created directory or automatic expansion of the84-diagnostic patch.

Proposed named laws:

- `demo_scalar_diff_and_inverse_match_neutral_vectors`: deserialize the same authored valid cases, execute real DemoMutation::diff/inverse and DemoDiff::apply, and compare complete snapshots/diff/messages/inverse vectors.
- `demo_absorb_and_stored_inverse_match_neutral_vectors`: execute real DemoDiff::absorb and the stored inverse list in reverse order; compare every authored intermediate snapshot including None and MIN/MAX.
- `demo_fixture_text_binary_round_trip_keeps_wire_identity`: call real ArtifactDsl/ArtifactPack/OpText/OpBinary surfaces, preserving `demo.demo`, `demo`, `demo/v1`, and SetN/set-n spelling and complete round trips. Frozen protocol-byte comparison requires the separately reviewed wire-fixture path; a self-roundtrip is not byte compatibility proof.

The existing `folder_event_log_storage_round_trips_undo_position_through_pack_spr` retains its0→1→2→undo1→reload1→redo2 assertions. It was not executed here. The missing actor fixture owner and the test that rewrites20 wire outputs/removes two stale names remain the explicit blockers documented in plan67; do not run the full outer namespace or silently modify those tests under this approval.

## Artifact Release

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| Neutral schema | 10608 | `c50a58b562c68e44d39ddef8d58384d5858b83bbea83abb274862c3a7954160c` |
| Neutral vectors / exact token roster | 47491 | `5d20baa9e199fd70cef4a74eb89bebb3335f7e8e75ab337d93319d237ff1ea1f` |
| Reference controller | 16871 | `f3f54cb2480cec1f008a495e6665803c8a47cae7a41cfd54b21925dcf80ebeed` |
| Actual reference JSON | 182678 | `fe4c445daea8442d6ba66eee0fa2546cf6abf17c57b7b22cd232c43a7f7ad336` |
| Actual post-source proof | 80057 | `a74e58b2b464b32de1b4dd2885a82da6a40b19c98a201345e4c0225e808051f9` |

The controller and neutral assets are frozen at their actual passing baseline. Source is released at62f319… for root review. No cleanup, source restoration, Git mutation or evidence recreation occurred. Mandatory base Mutation metadata defaults remain a separate known deficit; this narrow source join does not establish full direct-leaf completion.
