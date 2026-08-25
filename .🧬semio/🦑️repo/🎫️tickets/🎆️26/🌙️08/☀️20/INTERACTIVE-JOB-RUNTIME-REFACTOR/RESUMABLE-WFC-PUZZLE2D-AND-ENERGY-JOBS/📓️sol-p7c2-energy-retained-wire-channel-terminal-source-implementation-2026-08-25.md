# P7c2 Energy Retained Wire, Channel, and Terminal Source Implementation

Date: 2026-08-25  
Implementer: Sol extra-high P7c2 Energy packet  
Scope: Energy simulation wire/schema/source/fixtures only  
Runtime execution: deliberately deferred by packet instruction

## Outcome

The accepted P7c1 numerical job is preserved and its provisional publication side is replaced with an Energy-owned, schema-first retained binary protocol.

- `SMENERGY`, version `1`, little-endian, fixed 80-byte header.
- Header carries kind, stage, tier, operation, base revision, generation, seed, sequence, observed-item cap, page cap, operation cap, and process-job cap.
- Checkpoints are a fixed 164-byte retained replay capsule. Restore owns the exact input packet plus exact Model+Config until all four fields have been decoded. It reconstructs the numerical authority by driving the same admitted `EnergyJob` one grant at a time, retiring every replay publication page-wise, and accepts installation only when decoded stage/tier/warmup/hour/RNG/weather/aggregate/backing cursors and the retained numerical-graph digest agree.
- Decoded `time_series_count`, `meter_count`, and `history_count` are retained restore-target fields rather than admission-only scalars. Replay readiness and atomic finish both require exact rebuilt lengths and sufficient admitted series/meter/history backing before the graph digest can authorize installation.
- Preview/checkpoint/fault builders write one retained field fragment or page transition per grant. There is no production serde/JSON checkpoint path and no whole owner clone.
- Commit output no longer uses `commit_output: Vec<u8>`, `try_reserve`, or a second payload copy. It writes admitted header, count, UTF-8 character, record, sample, and summary fragments directly into staged retained pages.
- Commit admission is a retained cursor, not a scan: each grant visits one name character, meter record, series header, series sample, summary row, publication slot, in-flight slot, or retained packet.
- Before the writer is mounted or the first output byte is copied, commit admission reserves the exact queue slot and exact item/byte/page totals, then prepares one fixed page source per grant. Encoding consumes only those prepared sources; page/item/byte mismatches become a retained typed output fault.
- Meter and time-series names are emitted one Unicode character fragment per grant; samples and records remain one semantic record per grant.

## Fixed publication authorities

- Preview: one generation-tagged latest-wins slot. An occupied slot moves to a distinct retirement owner and releases exactly one retained page per grant before replacement installation.
- Preview has one canonical 100-byte `SMENERGY` representation. It includes every typed preview field and the live retained facility-electricity accumulator; the packet owns its typed projection, which is decoded from those exact installed bytes. The separate pending/cached typed authorities and `ENERGYP1` encoder no longer exist.
- Checkpoint: fixed four-slot lossless FIFO with saturated-packet identity preserved. Taking the head creates a provenance-bound in-flight lease without moving the head or rotating the tail.
- Commit: fixed four-slot lossless FIFO; terminal only takes the already sealed head packet and never traverses or encodes result state.
- Fault: fixed four-slot lossless FIFO; production numerical/output faults first build a retained fault packet and then expose the terminal fault signal.
- Consumer transfer: generation-gated `take_*_packet` returns the exact lease; retry restores that owner to the same physical head slot, while ACK alone advances the head. Arbitrary matching-packet injection is not an API.
- Checkpoint adoption is not an API. Restore replay keeps checkpoint output inside its original provenance lease, closes it page-wise, and calls the queue ACK only after terminal-empty; an interrupted restore retains the lease with the replay authority, and ordinary lease loss uses the fixed recovery registry without advancing the head.
- Batch `Engine::run`: drives the same `EnergyJob`, consumes notifications, takes the exact Energy packet, and closes one retained page per iteration.

## Exact owner and recovery laws

- Schema/version/kind/identity/cap/trailing rejection returns the exact retained packet and exact Model+Config owner.
- Numerical MAX+1 rejection occurs before restore registry mount and retains the original Model allocation identity.
- `EnergyRestoreJob` uses a separate fixed, generation-qualified 64-slot recovery registry. Direct Drop and panic unwind move the exact incomplete packet/Model/Config/parser cursor into the registry; same-generation recovery is single-owner.
- Restore checks operation/generation before every decode or replay step and again immediately before atomic install. `finish` transfers the replayed authority itself; it never mounts a fresh incomplete job.
- Lossless consumer leases use a separate fixed 64-slot recovery registry. Lease Drop/panic publishes the exact in-flight packet under its token; the owning queue can only recover it to its unchanged head slot.
- `EnergyJob` close now includes encoder, ready packet, restore input, preview/current retirement, every lossless queue, output writer/payload, and all P7c1 numerical owners. Each grant releases at most one page, owner, or control.
- Cancel and deadline checks still precede fuel consumption, wire allocation, field mutation, and page mutation. Stale generations cannot take any packet slot.

## Live source laws

Energy-local Rust tests now cover:

1. fixed checkpoint size/page count;
2. numerical MAX/MAX+1 exact restore owner and retry;
3. Drop recovery of an incomplete decoder and stale-generation rejection;
4. fixed lossless FIFO saturation identity and order, same-head retry, and exact Drop/panic recovery;
5. cancel/deadline/stale generation before wire mutation;
6. live magic/version/kind/identity/item/page/operation/process-job/trailing mutations through the production restore admission path;
7. restore stale-step and stale-install rejection without authority mutation or loss;
8. individual under-cap series, meter, and history count mismatches remaining unready rather than being discarded;
9. canonical typed/wire preview equality and a substantive live facility-electricity total;
10. restored commit byte parity across fuel 1 and 4;
11. retained commit census, up-front page-source reservation, exact page/item/byte verification, and existing P7c1 one/two/four/default deterministic chronology.

Schema/law fixture:

- `✏️s/🔌️plugins/🔋️energy/🪨️tests/p7c2-energy-retained-wire-laws.json`

Hostile mutation fixture:

- `✏️s/🔌️plugins/🔋️energy/🪨️tests/p7c2-energy-retained-wire-mutations.json`

## Source-only verification

Executed after the final edit:

```text
rustfmt --edition 2021 <Energy sim, precompute wire-signature, kernel wire-signature source>
rustfmt --edition 2021 --check <same Energy source>
git diff --check -- <Energy plugin>
jq empty <P7c2 schema fixture> <P7c2 mutation fixture>
```

All completed with no output/failure.

Static forbidden census over the Energy sim source returned zero production occurrences of:

```text
from_checkpoint
encode_state
commit_output
output_payload_cursor
ENERGYOUT1
preflight_commit_wire
values().try_fold
adopt_checkpoint_packet
ENERGYP1
encode_preview
pending_preview
last_preview
facility_electricity_kwh: 0.0
serde_json::to_*
serde_json::from_slice
payload_from_bytes
write_slice_page
HashMap
HashSet
extract_if
```

The final census found eight live P7c2 Rust laws and thirty-five explicit hostile source mutations. The added faithful mutations cover lossy fresh restore, every retained decoded count, stale replay/install guards, queue head rotation, matching-packet injection, unacknowledged adoption, lease loss, zero/split/legacy preview authority, whole commit scans, late reserve, and unreserved pages.

The remaining `serde_json::from_str` occurrence is the accepted P7c1 test-only third-party differential parser; it is not a checkpoint, restore, preview, output, terminal, or production path.

No Cargo, Nx, Wasm, browser, or runtime/build command was run, per packet boundary. No Puzzle2d, shared script, actor, renderer, or P7c3-mounted UI source was touched.

## P7c2 boundary

This packet owns the retained Energy wire, Energy publication channels, consumer transfer/ACK/retry, terminal prepared-commit handoff, decoder admission, and recovery. It does not claim P7c3 actor/session/UI mounting or runtime execution evidence.
