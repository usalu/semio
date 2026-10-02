# Hub Actor Lifetime Retained Fixture Owner

Read-only review, 2026-10-01; no source edits or tests. An exact retained shared fixture exists. Rebind the Hub test input to its actual lower owner; no fixture reconstruction, new schema, generated replacement or weakened assertions is needed.

The input owner is `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧫️fixtures/🔣️.json` with adjacent `🧬️schema/🔣️.json`, Rust/TS lifetime implementations and owner tests. From `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` the exact source-relative literal is `../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧫️fixtures/🔣️.json`. Existing line 76 instead targets nonexistent Hub `🎭️actor/🚪️lifetime/🧪️fixture`, using an obsolete singular fixture directory.

## Byte shape and retained assertions

The retained version-1 JSON has nine lifecycle wire vectors and four turn-result vectors. Its close row is exactly `{"kind":"close","lifetime":{"activationGeneration":"1","instanceId":7,"guestLifetime":"13"},"requestSequence":9}`, hex `0201070d09`. Hub lines 76–83 select that close row, remove only the wire discriminator kind, compare the complete remaining structure against first-party request JSON, and check serde Event roundtrip. Preserve all those operations and assertions.

The matching real helper remains in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:2592–2599`: ActorInstanceCloseRequest with generation1, instance7, guest lifetime13 and sequence9 wrapped in Event::InstanceClose. Its live `🧪️tests/🔬️unit/🦀️.rs:74–83` contains the same law and already includes the retained neutral fixture via the correct source-relative binding. The live source mounts these tests at shard source line2602 onward.

Other retained fixture semantics must stay byte-for-byte: reopen changes guest lifetime13→14 under activation41 and numeric instance7, rejects stale request/receipt; completion requires all exact owners (app/reactor/nativeUi/hostUi/ingress/publication/transport) retired rather than accepted/idle/map removal; native cases retain same-ID replacement, overflow/live-owner, quarantine terminal witness, contention, repeat close and surface-clear scope checks. Lease receipts join exact accepted+retired ownership and reject wrong worker/activation/request/close generation; published-close progression tracks metadata/credit/handback owner transitions. This missing-input test itself validates request JSON and serde Event structure, not all these state/window transitions. Do not claim that a rebind alone proves the full lifecycle protocol.

## Existing independent and runtime evidence route

Neutral owner Rust law `🚪️lifetime/🧪️tests/🔬️unit/🦀️.rs:34` compares all nine encoded wires against committed independent LEB128 hex vectors, decodes them, rejects every truncated prefix and trailing byte, and compares serde receipt/ack structure. Retained max-width row covers u64/u32 and JS-safe request-sequence limits. The neighboring fault-publication TS law validates its separate fault fixture/schema and independent elapsed comparison; that fault fixture is not the missing shared lifecycle input and should not be substituted.

Actual native shard law belongs to semio-framework-os-kernel, project `@semio-tech/framework-os-kernel`; use its registered test route with the exact filter `instance_close_event_matches_the_shared_first_party_fixture_and_serde_structure`, and establish the listed test exists before counting runtime proof. Neutral actor package separately owns its lifecycle codec law. Hub package project os-hub routes `test` through `📦️packages/🦀️rust/📜️script.ts:2604–2612` to semio-hub, with optional all-features. Current authored Rust search found no module mount for the Hub `🔬️dist-assets-bll2-7qi-unit` copy: it is inventoried by the all-authored source gate but is not thereby executed by semio-hub tests. Do not report Hub package tests as runtime execution of that copy without adding/confirming genuine composition ownership. The existing mounted shard law is the preserved runtime proof of the matching original behavior.

## Read-only history evidence

Current retained lifecycle fixture is tracked in history; latest path commit inspected was `989582baab2` (June4). Hub test copy latest path commit inspected was `40a2736e661`; `git show` of that commit retains the same stale Hub include literal and assertions. No git operation changed state. This supports binding the existing neutral owner, not recreating a lost Hub-only asset. No compiler/test command was run in this audit.
