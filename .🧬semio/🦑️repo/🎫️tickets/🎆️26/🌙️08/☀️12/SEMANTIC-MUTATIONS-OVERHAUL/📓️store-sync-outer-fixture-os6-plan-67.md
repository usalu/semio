# Store Sync Outer-Fixture OS6 Preparation — 67

## Decision And Scope

Preparation only. No Rust, domain fixture, production source, launch, dependency, Cargo, target, or existing evidence was changed; no native law ran. This report proposes the smallest synchronous-trait join for the exact **84 diagnostic** cohort, not a mutation-direct-leaf migration or a claim of compiler readiness.

The allowed future source footprint is the outer `#[cfg(test)] mod tests` in [Store Sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3620): fourteen `async fn` qualifiers, sixty-five specific synchronous-call `.await` tokens, and one local `use crate::os_store::Backbone;` inside the fresh-channel success test. Keep the async test/helper shells and every actually asynchronous call intact.

The source was read through the complete outer module, including nested `actor_tests`. The excluded `native_actor::retained_turn_fixtures`, its `fixture_runner_handle` join (current4181 / compiler4169), WorkerSubmitError site (compiler2600), production SyncSession/detach, codec Send thunks, backbone/return/retirement implementation, Fresh/registry and Interaction are not proposed writes. No rejected-page test mount is included.

## Compiler Evidence Actually Read

The runtime ticket owns the [master report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-native-r1-compiler-red-2026-08-28.md), [complete rendered diagnostics](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-r1-full-compiler-diagnostics-2026-08-28.md), and [all JSONL records](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-compiler-diagnostics-2026-08-28.jsonl). They were not recreated.

| Input | Bytes | SHA-256 |
| --- | ---: | --- |
| Master report | 9016 | `f01a98c6eae2a9bd21e1adc48184a51627f2d75d8d3c1efe741aa81f51f44eed` |
| Full rendered diagnostics | 174819 | `9739138f2e9e30e3c9bcf10af69009b7e73352bfb9a144a48c40218b0bf29664` |
| Full JSONL | 818114 | `654962ed8040bcc4fb3f693e5c827faca180e2f4a332f3532aa900476140f16e` |

The rendered Markdown was read in five bounded slices through EOF, including all nineteen appended full-type values. All161 JSONL records were parsed; every one of the161 rendered record strings, after ANSI-color removal only, was independently found in the full Markdown (missing=[]). Large initial tool displays truncated; those displays were not credited as complete reads and were replaced by bounded reads/record views.

The actual historical compile was OS-kernel `--lib --features sync,ureq`, with92 source errors and66 warnings; zero of its six selected tests ran. JSONL contains93 level=error records because the abort summary is one additional record. Assigned outer-test classification independently reproduces:

| Assigned group | Diagnostics |
| --- | ---: |
| E0053 signatures | 14 |
| E0277 not-a-future | 58 |
| E0277 unsized cascades | 10 |
| E0599 channel trait visibility | 2 |
| Total | 84 |

The current outer module begins at3620, twelve lines later than that capture. Each of the58 not-a-future diagnostic source lines was checked byte-for-byte against its current line at old+12; all58 matched. Thus this is a confirmed source join, not an inferred bulk replacement. The ten unsized diagnostics belong to the old encode-document expression (four) and two old chained envelope/print expressions (three each); do not “fix” them with boxes, annotations, or changed values.

## Fourteen Exact Signature Changes

Change only `async fn` to `fn` at these current definitions; keep parameters, return/error types, values, serde/DSL attributes, branches and inverse semantics.

| Owner / trait | Methods (current line) | Current authority |
| --- | --- | --- |
| DemoSnapshot / ArtifactDsl | envelope_id3739; parse_dsl3742; print_dsl3750 | [ArtifactDsl4541–4549](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:4541) |
| DemoSnapshot / ArtifactPack | encode_pack_with3758; decode_pack_with3763; record_spec3771 | [ArtifactPack8979–9003](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8979) |
| DemoDiff / MutationDiff | apply3782; absorb3786 | [MutationDiff58–74](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:58) |
| DemoMutation / OpText | parse_op3801; print_op3812 | [OpText947–949](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:947) |
| DemoMutation / OpBinary | encode_op3821; decode_op3832 | [OpBinary961–965](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:961) |
| DemoMutation / Mutation | diff3852; inverse3859 | [Mutation127–128](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:127) |

Actual trait route: OS glue mounts OS SPR; its component reexports OS command; OS command16 reexports `protocol::mutation::*`. Pack's ByteReader is the actual replication codec type through the pack facade, not a fixture replacement.

The current lower Mutation trait105–164 still supplies `DESCRIPTORS=[]` and `UNDECLARED_MUTATION_LEAF` defaults. DemoMutation inherits them. This is a separately visible direct-leaf metadata deficit, **not** acceptance or a request to add fake metadata here. No descriptor, clock, source-provenance or default is added by this84 join.

## Sixty-Five Exact Await Removals

The table lists the58 compiler-diagnosed calls. In chained expressions remove only the named synchronous inner call, not every await on the line.

| Enclosing function | Current lines | Synchronous call |
| --- | --- | --- |
| DemoSnapshot::print_dsl | 3752 | ArtifactDsl::envelope_id |
| DemoSnapshot::encode_pack_with | 3759,3760 | pack_rt::encode_document; envelope_id |
| DemoSnapshot::decode_pack_with | 3765,3766,3768 | envelope_id twice; pack_rt::decode_document |
| DemoMutation::encode_op | 3825 | os_pack::encode_record_body |
| DemoMutation::decode_op | 3833,3843 | ByteReader::new; os_pack::decode_record_body |
| DemoMutation::diff | 3856 | MutationOutcome::new |
| ensure_demo_codec_registered | 3871 | ArtifactCodec::of only; register_document_codec remains awaited |
| sample_operation_envelope | 3890 | mutation_envelope_from_edit |
| receive_materializes_remote_envelope_into_the_edit_timeline | 3897,3901,3902 | create_document_envelope; snapshot; envelope |
| receive_buffers_out_of_order_envelopes_until_dependencies_arrive | 3907,3914,3916,3917 | create_document_envelope; envelope twice; snapshot |
| wire_fixtures_stay_byte_identical_across_rust_and_ts | 4026,4027 | OpBinary::encode_op |
| sample_wire_envelope_for_fixtures | 4105,4106 | OpBinary::encode_op |
| op_envelope_from_stored_edit_round_trips_through_ingest | 4210,4211,4217 | encode_op twice; decode_op |
| actor_tests::demo_envelope | 4234 | create_document_envelope |
| actor_tests::folder_external_edit_delivers_remote_operations | 4317,4318,4338,4339 | encode_op twice; envelope; snapshot |
| actor_tests::two_hosts_converge_through_hub | 4503 | snapshot |
| actor_tests::reconnect_since_catch_up_replays_backlog | 4557,4558 | envelope; snapshot |
| actor_tests::detach_drains_pending_outbound_operations | 4608 | snapshot only; no detach/lifecycle edit |
| actor_tests::replay_fixture | 4711,4730,4753 | create_document_envelope; parse_op; envelope |
| folder_event_log_storage_round_trips_undo_position_through_pack_spr | 4811,4813,4816,4818,4825,4828 | create_document_envelope; snapshot twice; envelope; snapshot twice |
| folder_text_storage_round_trips_dsl_and_appends_ops | 4844,4845,4848,4850,4854 | create_document_envelope; envelope; create_document_envelope; envelope twice |
| folder_text_storage_round_trips_pack | 4880,4881,4882,4885,4887,4891,4907,4908,4916 | create_document_envelope; envelope twice; create_document_envelope; envelope four times; parse_dsl |

Seven additional awaits are currently masked by earlier invalid types, but their synchronous declarations are directly established; removing them is part of completing these same fixture bodies, not seven new compiler observations:

| Current line | Exact call |
| ---: | --- |
| 3834,3838 | reader.read_u8() |
| 3842,3844 | reader.position() |
| 4731 | concrete.encode_op() after the corrected parse_op |
| 4882,4908 | initial_snapshot.print_dsl() after the corrected envelope getter |

ByteReader::new259, position267, read_u8271 are synchronous; read_bytes307–313 produces the existing typed truncation failure. Keep offsets0/1, format1, ordinal0, variant lookup and record-body bytes unchanged.

Other authority anchors: Store pack_rt4585/4590, OS pack39/45, MutationOutcome858, Store ArtifactCodec9124, create_document_envelope9629, envelope13775, snapshot13950, replication causal mutation_envelope_from_edit548.

**Remain async:** register_document_codec9306; ArtifactStore::new13587; dispatch14830; attach_backbone15668; tick15699; print_edit_lines9944; print_document_text10053; print_document_pack10794; parse_document_text11008; parse_document_pack11020. Also retain session receive/new, host/channel-pair/endpoint/storage/wire operations, actual actor wait helpers and codec erased-future calls. Do not apply a global await-removal transform.

## Exact Backbone Join And Ownership Limit

The master wording “removed ChannelBackbone.send” is contradicted by the raw diagnostic and current source. [Backbone16601](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:16601) requires `async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>`; [ChannelBackbone16856](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:16856) implements it. It is not currently an inherent method and the outer test does not import the trait.

Proposed change: one local `use crate::os_store::Backbone;` inside `artifact_mailbox_nested_identifier_bytes_and_backbone_one_pop_preserve_ownership_order` at3699. Preserve exactly:

- Channel pair URI `store-sync-one-pop`.
- First ACK owner `op_ids=["first"]`, then second `["second"]`.
- Both awaited sends and their existing expect messages.
- First remote pop returns only first; second only second; third returns None.
- Existing nested-presence byte-credit comparison.

Current send16857 takes its message by value and uses `Mutex::lock`; closing/poison errors return only VcsError, not the original message. It is **not** evidence of refusal ownership, unwind recovery, or nonblocking send. No current `try_send` declaration was found in the exact Store component; ChannelBackbone's only inherent method here is pair. Do not invent or restore a method, nor replace it with a generic owner factory. Runtime's retained backbone direction is a separate production packet.

The existing remote `try_pop_front`16880 does use try_lock, returns at most one exact queued owner, and leaves the queue unchanged on contention. The proposed fresh/uncontended success test exercises FIFO only; it must not be relabeled as a broader send contract law.

## Preservation And Inverse Proof Recipe

The fixture remains the original i32 snapshot `DemoSnapshot { n }`, dotted DSL identity `demo.demo`, suffix `demo`, schema `demo/v1`, SetN serde variant, and DSL keyword `set-n`. No alternative wire opcode, direct-leaf rename, wrapper snapshot, arithmetic delta, compatibility path, timestamp/default or changed assertion is proposed.

DemoDiff remains `n: Option<i32>`: None preserves the base, Some replaces n; absorb preserves the earlier diff when the later field is None and selects the later Some otherwise. DemoMutation::diff always emits Some(target) with no messages. Its inverse is exactly one SetN carrying the **pre-state n**, including negative and boundary values. As a one-element inverse there is no unary-minus/MIN overflow problem and no local-order ambiguity.

Source preservation proof before/after the future patch:

1. Capture this exact outer source and the14 definition qualifiers/65 await token locations. Reject any mismatch before write.
2. Compare token streams after removing only those approved async/await tokens and the single trait import: all remaining tokens and literal strings must be identical. Do not use a substring marker gate as behavioral proof.
3. Verify DemoSnapshot/DemoDiff/DemoMutation declarations, DSL/serde attributes, all original expected assertions, raw wire constants, and diff/inverse bodies are unchanged apart from the approved qualifiers/MutationOutcome await. Retain the actual before image and complete diff inside the ticket.
4. Bind the production prefix and excluded caller slice independently, allowing separately announced runtime drift without claiming it is ours. Future compiler capture must use a newly coherent source boundary.

Future native proof must call the actual fixture implementations and current Store APIs: for base0 SetN1 then SetN2, retain stored pre-state inverse values0 then1 and prove undo/reload restores1 and redo restores2. Existing `folder_event_log_storage_round_trips_undo_position_through_pack_spr` already asserts this save/load/undo-position scenario; keep it. For scalar contract checks also use equal-value, negative, i32::MIN/MAX and Diff None/Some sequences. Execute any returned inverse list in the Store's required reversal order, not a new forward-fold law. No Store inverse code is changed; single-element Demo inverses remain the existing behavior.

## Schema-First Regression And Independent Oracle Route

No new controller or fixture was authored or executed here. The proposal below is test-only and requires the next approved source packet; it adds no runtime library/API.

The existing twenty committed wire fixtures are the strongest surviving language-neutral bytes for this join. Each currently has an exact canonical counterpart under [replication wire fixtures](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🧫️fixtures/wire), and all20 pairs were read no-follow and compared equal (actual file comparison, **not** Rust/TS codec execution). Preserve both sets unchanged.

For the narrow scalar/inverse proof, author a closed test schema first using the actual i32 range and explicit expected snapshot/diff/inverse/message fields; include original scenario values0,1,2,5,6,9,42 and negative/boundary cases. The fixture is test-owned data, not a parallel mutation implementation. Use existing Ajv2020 strict validation plus jsonc-parser duplicate/error checks. Use installed Decimal.js as an independent exact numeric oracle for replacement via `base + (target - base)`, signed bounds, pre-state inverse values and ordered absorption; compare authored expectations before comparing Rust results. Decimal arithmetic/schema validation supplies numeric/data evidence only, never Rust ownership, async, queue or byte-codec proof. Local module resolution confirmed Ajv2020, jsonc-parser and Decimal.js available; binary-parser/protobufjs-minimal were absent, so do not silently add dependencies or label ordinary JS byte code “third-party”.

Wire regression must use the actual existing codec paths: DemoSnapshot ArtifactDsl/ArtifactPack and DemoMutation OpText/OpBinary, then actual protocol client/server frame encoders/decoders. Compare complete output bytes to the unchanged committed inputs and decoded values to independently authored schema-valid expectations. The existing TypeScript twin is another owned implementation, not itself a third-party oracle. The third-party numeric/reference check covers exact scalar outputs; do not claim it validates the complete wire protocol.

A source-only controller may validate the exact qualifier/callsite preservation roster and source hashes, but no source count substitutes for compilation. If implemented, it must be a ticket-owned `📜️script.ts`, executed via Bun/Nx and separately registered through the launch owner. No command was registered or run by this preparation. Root/runtime retain ownership of the next serialized OS-kernel `--lib --features sync,ureq` compile and explicitly selected native laws.

### Current Regression Blockers, Not Silent Success

- `actor_tests::fixtures_replay_matches_expected_events`4693 uses `CARGO_MANIFEST_DIR/🧫️fixtures`. Its actual package path is [OS Rust package](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust); exact child `🧫️fixtures` is ENOENT. The Sync owner currently contains only component.rs. The production loader3202 returns empty on missing directory and the native test explicitly rejects empty. No missing actor fixture was recreated, and no other root was inferred. Resolve its true authored owner separately before claiming actor-fixture replay.
- `wire_fixtures_stay_byte_identical_across_rust_and_ts`3997 currently creates its destination, removes two stale filenames, and overwrites20 outputs at `CARGO_MANIFEST_DIR/../fixtures/wire`. It tests self roundtrip after write, not frozen-byte comparison. Do not run the entire outer-test namespace under this task's no-output/no-cleanup constraint. A future approved test-only conversion to read/compare the existing twenty bytes, preserving all values/assertions, is a distinct write footprint from the minimal84 join. It must be explicitly reviewed before execution; no generator run is proposed now.
- Store/Sync production ownership defects, R17 Send/backbone work and removed-fixture metadata acceptance remain outside this compile-join packet. No catch/panic/drop workaround is allowed to make those tests appear green.

## Source Boundary Receipt

All listed source inputs were captured with case-insensitive lexical Compose exclusion, full-ancestry no-follow checks, O_NOFOLLOW open, fstat-before/after and endpoint identity checks. The table records the initial/read-phase bytes. Eight of nine source inputs remained identical at release, including the entire Sync source. During Markdown preparation, whole Store changed from `ed1d6b93…` /1541032 bytes to `7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4` /1541146 bytes. No cause is attributed and no source was restored. All ten already-read Store API/import slices (1–85,4538–4610,8974–9095,9118–9230,9300–9320,9618–9655,13670–13785,13940–13970,16585–16660,16800–17035) were separately re-read and remained byte-identical; the precise trait/backbone plan is still grounded in those unchanged declarations. This is not a globally stable Store capture or a native release. Exact ancestor AGENTS read: root, products and OS; Store and Sync AGENTS were absent.

| Source | Bytes | SHA-256 |
| --- | ---: | --- |
| [🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs) | 268230 | `37012443ee787d1a05e7826e8c3a8ac35ea0be6d43eba6918dcd7076c15e8d93` |
| [🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs) | 1541032 | `ed1d6b93b36a07f3c2aa914350c97f993613bc1a779e3b019a8f1329c7e19a37` |
| [🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs) | 13640 | `663e0aa9d06e6aae7b9620f718793b7cb3ebd122510fec7183e0d1ba2c6cb9f3` |
| [🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs) | 21844 | `7af80688ad062f113f9506d9374b8b10edad164480db77f8b28380ef2538ce9b` |
| [🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️component.rs) | 5656 | `fbcbf7d9fa0f8f7e148f0a66631808a13e914b74c47677cecea70b8fe5062547` |
| [🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs) | 91352 | `fae229ba917547d57a466a10daefc8a415bcf85d74ea481f0fbf3e1e605e89d3` |
| [🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs) | 57347 | `e5f2f9ce74cc305bcbc23c0d99ab70cc2af54cf299a561f7910d56a7dbbd8385` |
| [🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs) | 31170 | `5af159efa43a760a6e533cc1ba71a2dd4774dc66e742319ab05bafbc0fd944e5` |
| [🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs) | 58962 | `97a2716ac3d8acd67e3e53c7bbde0af0699b33205d7806cccdb1a62ff386c22a` |

Current Sync slices, defined by UTF-8 text split on LF and the stated lines joined with LF (no invented final newline):

| Slice | SHA-256 |
| --- | --- |
| lines3620–EOF, outer module including final newline from the final empty split row | `d6a30b536701c87c2438c85d6dc0aa96dbb9cc8c6d3a3da62c44b8f5435d6dd7` |
| lines3727–3862, fixture declarations/impls | `e9330d8a3f0d6034de97a3380c57ce55de4228a48369b978a3b96b62747aacdf` |
| lines1–3619, production/inner-test prefix | `953cddbdb2c572de4e846b578f6ab74c53791f8a945d3b0b6f9aea174b7b504d` |
| excluded caller line4181, without LF | `a8a58e96c1c0c072e180847c700f11bfb53c29864af7d9abebe2abc466cbb340` |

The independently compared wire roster has JSON-row SHA `75984a117a862a8edb5b970e1e193e1f92a4393cdb33665e2d1ed1a1b2afea89` (sorted rows with name, byte length, SHA and canonical equal-byte tuple). Complete per-file facts:

| Filename, same at both observed roots | Bytes | SHA-256 |
| --- | ---: | --- |
| 📦️client-bye.bin | 2 | `40d88127d4d31a3891f41598eeed41174e5bc89b1eb9bbd66a8cbfc09956a3fd` |
| 📦️client-commands.bin | 60 | `9901383b326230e9fe3d5551a8526b84ef5e812802da2dc3a380c8dce30c5092` |
| 📦️client-credit-grant.bin | 3 | `1871658dc7416ec87cfac880aa61d4dd53347e5099631598bc2acd7b67219645` |
| 📦️client-frontier-advertise.bin | 47 | `13ef64663c99fe3660fa467ae5737b8b0c545b4bbd2d1aa264824563adca127e` |
| 📦️client-hello.bin | 63 | `73d2dd55469957f794b56a0f8da2ec5638c9411b3706c6a8ecc3fc5d0cbeb073` |
| 📦️client-presence.bin | 327 | `830cd63ee6df77c66eac748f97a136f451d992861b56499d103d4de0c6f1666e` |
| 📦️client-preview-publish.bin | 14 | `083c075f4895aeb7c369398e39eb68633ea154feff23d89047df9637c0da1199` |
| 📦️server-ack-accepted.bin | 53 | `0f88100f521896d6b8ab1388640cd3ab666a8663301d6ec548de0d8bdf0d66e8` |
| 📦️server-ack-rejected.bin | 66 | `75382cdcd7becd56db1769d1952249e9ec6598e6ac9655bf1e65267920bbcf41` |
| 📦️server-ack-transformed.bin | 114 | `195d413f9e4002e5ecbfe23a89e93443e8f50e43788c3f44bff979cfb7553ed9` |
| 📦️server-commands.bin | 112 | `a8f760d5e6e925e2959e9451ee296348b29e814ee0f8bf11a4f7ac14d80d6a67` |
| 📦️server-credit-grant.bin | 3 | `13dc1ba04cc8354c29e11a9860f9b678656389753c562d4b96f65a7da07dbe2b` |
| 📦️server-error.bin | 21 | `5f7b89557d9129965ef600d11b01e9c9349ee4de666f70f331cd1acf52023141` |
| 📦️server-presence.bin | 339 | `a099f35f27ae3b95024aa422adbc3b42ce731c2cdb99bd9ffa6eb932e71f3940` |
| 📦️server-preview.bin | 21 | `bee8e2738578240bb037d867a90e04bbf263ff6bce709ec9b44a8f64736dce40` |
| 📦️server-session.bin | 11 | `968dc23d161bc15b7987ffae4de2f500ab5fb37778560e247d017c50caf1b699` |
| 📦️server-snapshot-chunk.bin | 8 | `eae72f264ea6ee1fb34f25f4a57b5614c1afa6cf8ee8f75cbb87f49a47394e0f` |
| 📦️server-snapshot-done.bin | 3 | `95a52fbc37d8806e535830ee084bc1a566a53686be5c3b63a371f18db9fe7062` |
| 📦️server-welcome-snapshot-inline.bin | 104 | `187a8ff4ebce79920fdbf3363f848d87e4053c12f0ad0b93a6607e37a7a6c1ec` |
| 📦️server-welcome-tail.bin | 67 | `fc398c4c8c6be9ab5460476bc284e9571c2b31cef06efeb6a71e43bc61bc21b6` |

## Handoff

Only this new Markdown was written. No schemas or tests are mounted, no native/source gate was run, and no original evidence was deleted. The requested next approval is the exact14-signature/65-await/local-trait-import source join after runtime receives the pre-write boundary; metadata adoption, missing actor fixture ownership and wire-generator conversion are explicitly not implied by that approval.
