# 🎯️ Verify the RIGHT crate — `#[path]` mounts make os-kernel a false baseline

Framework modules under 💻️os/🔨️modules/ are NOT all part of `semio-framework-os-kernel`. Each is
`#[path]`-mounted into a specific crate. Checking the wrong one returns exit 0 no matter what you
broke. Confirmed ownership:

| file | crate to verify with |
|---|---|
| 🏪️store/🦀️.rs, 🌿️vcs/🦀️.rs, 💡️inference/🦀️.rs | `semio-framework-os-kernel` |
| 🔌️plugin/🦀️.rs | **`semio-framework-plugin`** (os-kernel mounts it 0 times) |
| 📖️playbook/🦀️.rs | **`semio-framework-os-flow`** (mounted at 🌊️flow/📦️packages/🦀️rust/🦀️.rs:35) |

## Two false baselines I actually issued this session
1. Briefed the playbook agent to verify with `-p semio-framework-os-kernel`. That crate does not
   compile playbook at all — the check would have read green regardless of the edit. The agent
   caught it and told me; credit to it, not me.
2. Claimed twice that "os-kernel exit 0 validates the `try_serialize` seam". It does not —
   `🔌️plugin/🦀️.rs` is in `semio-framework-plugin`. The try_serialize conversion and the 20
   dual-derive conversions in that file remain UNVERIFIED.

## What os-kernel exit 0 DOES cover
🏪️store (ArtifactCursor, ArtifactBackboneRef), 🌿️vcs, 💡️inference `InferredField`. Nothing else.

## How to check ownership before trusting a verification target
    grep -rn 'path = "' <crate>/📦️packages/🦀️rust/🦀️.rs | grep <module-emoji>
Zero matches means that crate does not compile the file, and its exit code says nothing about it.

## ✅️ Verified against the CORRECT crates (isolated target dir, peers' lock untouched)
    semio-framework-plugin = 0 errors   ← the seam I had wrongly called unverified
    semio-framework-os-flow = 5 errors  ← playbook, down from 18

`semio-framework-plugin = 0` confirms, against the crate that actually compiles the file:
- `try_serialize<T: protocol::ToValue>` via `protocol::json::to_json_string`, dead
  `TypedOperationResultPageWriter` removed, capacity/Fault semantics preserved;
- the Download-lane 4-tuple's hand-written `ToValue` (tuples encode as flat ARRAYS, a derived struct
  as an OBJECT — this preserved the wire format);
- 20 dual-derive types stripped of their serde half;
- the SECOND `serde_json::to_writer` site (`admit_command_json_with_proof`).

Remaining 5 in os-flow: four `serde_json::Value: ToValue/FromValue` bounds at 📖️playbook/🦀️.rs
:229/:235/:979 and 📖️playbook/🧬️generation/🦀️.rs:97 — these are the FormGeneration CONSUMER wave
(the `values` field changed shape; a streaming JSON walker still assumes the old `serde_json::Map`).
Plus one unrelated `E0502` borrow error in 🌿️vcs/🦀️.rs:2771.

## 🔁️ Operational rule earned the hard way
Peer sessions repeatedly reclaim the shared build-directory lock (observed 2h15m, then four more
workspace checks minutes later). ANY verification plan that depends on the default target dir will
keep dying. Use the isolated dir FIRST, not as a fallback:
    export CARGO_TARGET_DIR=<scratchpad>/isolated-target
    export RUSTC_WRAPPER=""
It does not disturb peers, and warm it costs seconds-to-minutes per crate.
