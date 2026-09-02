# 🔩️ Framework seams closed this session

The recurring pattern of this ticket: a plugin "still has serde" because a FRAMEWORK type signature
or trait bound forces it. The plugin is never the real blocker. Seams closed today:

## 1. `try_serialize` — 🔌️plugin/🦀️.rs:13091
`fn try_serialize<T: serde::Serialize>` → `<T: protocol::ToValue>`, body now
`protocol::json::to_json_string` instead of `serde_json::to_writer`. Dead
`TypedOperationResultPageWriter` + its `impl std::io::Write` removed (grep confirms 0 occurrences).
Same Fault code, capacity checked before copy — no truncation/panic change.
⚠️ Subtlety handled correctly: one caller passed a 4-tuple. Tuples encode as flat ARRAYS while a
`#[derive(ToValue)]` struct encodes as an OBJECT; a hand-written `ToValue` preserved the array. A
silent object-for-array swap would have changed the wire format with nothing failing loudly.

## 2. `InferredField<P>` — 💡️inference/🦀️.rs:83
`Key`/`Value` re-bound from `serde::Serialize + DeserializeOwned` to `ToValue + FromValue`;
`encode`/`decode` rebuilt on `crate::os_pack::json`. This bound alone was pinning FOUR 🗄️stdio files
(✅validation-report, 🔗connectivity, 🎲entropy, 📊moments) to unconditional serde; all four are now free.
Reviewed and accepted:
- `encode_map`/`decode_map` were hand-rolled as `[[key,value],…]` because the codec's `BTreeMap` impl
  only covers `K = String`. Both fns are file-local (`fn`, not `pub`) and the cache shows no
  persistence, so the representation change has no external contract.
- `decode_map` panics via `.expect`/`panic!`. NOT a regression: the original `decode` was
  `serde_json::from_slice(bytes).expect("cached inference bytes must decode …")`. Semantics preserved.

## 3. Still open in 🔌️plugin/🦀️.rs (~79 refs, agent dispatched)
- **9 types with DUAL derives** — `serde::Serialize, ToValue, serde::Deserialize, FromValue` on one
  type. Dual-derive is its own failure mode: it looks converted, passes any "has ToValue?" check, and
  keeps serde linked.
- A SECOND `serde_json::to_writer` site the scoped agent never had in view.
- `serde_json::to_vec` / `to_string` / `json!` and two `serde_json::Value` bridges.

## 4. glTF — named explicitly by the goal (agent dispatched)
🧊️gltf/…/🚪️io/🦀️.rs parses the real wire format at RUNTIME via `serde_json::from_str` into
`GltfDocument`, plus hand-written `impl serde::Serialize/Deserialize` newtype codecs. A prior agent
classified this as "genuine runtime codec need" — correct about the need, wrong about the conclusion:
the need is exactly why it must be met by the first-party codec. glTF is oracle-only per the goal.
Correctness bar set for that work: `.gltf`/`.glb` bytes must not change; accessor indices/offsets are
INTEGERS and must not render as `1.0`; `skip_serializing_if` must survive (absent ≠ null in glTF).

## 5. ✅️ `MeshData` — 🔺️mesh-engine is now production-serde-FREE (fully verified)
- Hand-written `impl pack::value::FromValue for MeshData` — deliberately NOT derived: the derive
  hardcodes `::semio_framework_os_kernel::…` paths, which would invert this leaf crate's layering.
- Reads exactly the camelCase shape the existing `ToValue` already emits, so encode output is
  unchanged. Indices/counts decode via `u32`/`u8` (UInt), positions/normals via `f32` (Float) —
  never crossed. A mesh index silently becoming a float would corrupt geometry rather than fail loudly.
- `Serialize`/`Deserialize` downgraded to `#[cfg_attr(test, …)]` and **`serde` moved to
  `[dev-dependencies]`** — it survives only as a differential oracle.
- VERIFIED: `cargo check -p semio-framework-mesh-engine` → 0 errors;
  `cargo test -p semio-framework-mesh-engine` → **35 passed / 0 failed** (29 pre-existing + 6 new,
  incl. a UInt-vs-Float fidelity guard and a serde_json differential oracle);
  downstream `cargo check -p semio-framework` → 0 errors.
- Unblocks (not yet edited): procedural3d ✏️editor/🦀️.rs `mesh_data_for_preview_handle` :1095,
  `pending_preview_tessellate_handles` :1125, `export_mesh_from_document` :1325.

### ✏️ Correction to my own brief
I told this agent that two 📐️cad sites (`✏️editor/🎭️modes/✏️edit`, `👁️viewer/…/📐️shape`) were
blocked by the missing `FromValue`. They are ENCODE-direction and were never blocked; their
in-file comments claiming "MeshData has no ToValue" are stale — `ToValue` predates this session.
I propagated those stale comments from two agents' reports without checking the direction of the
call. Verify the direction of a conversion before citing it as a blocker.
