# 🔬️ Central verification — the two highest-risk replacements

Both items were reported by their authoring agents as **WRITTEN BUT UNVERIFIED** (the build machine
was saturated: ~77 concurrent rustc processes across twelve live interactive developer sessions plus
the agent fleet, so no in-workspace run completed). Both have now been verified by the coordinating
session using a **standalone crate outside the repo workspace** — the source file copied verbatim,
the third-party original as the only dependency, which bypasses the contended workspace lock
entirely. Test sources are preserved beside this file in `🔬️verification-blake3/` and
`🔬️verification-parry3d/`.

## 1. First-party BLAKE3 — ✅ PROVEN byte-exact

Source: `🧰️framework/🔨️modules/🔢️hash/🦀️.rs`. Stakes: digests are content-addressed and persisted
via `mint_edit_id` / `mint_change_id` / `mint_mutation_id` and `🎒️pack` container hashing — a
one-bit divergence silently invalidates stored data.

- The crate's own 10 tests pass, including its blake3 differential oracle tests and the NIST
  SHA-256 vectors.
- Additional coordinator harness vs `blake3 = "1.8.7"`:
  - **one-shot: 28 official vector lengths** (0,1,2,3,4,5,6,7,8,63,64,65,127,128,129,1023,1024,
    1025,2048,2049,3072,3073,4096,4097,5120,10240,102400,1000000) using the standard `i % 251`
    input pattern — all byte-identical.
  - **incremental: 300 randomly-chunked inputs** (constant-seeded xorshift, chunk sizes 1..700,
    totals 1..9000) — `Hasher::update`/`finalize` byte-identical to one-shot reference.

```
one-shot parity OK across 28 official vector lengths (max 1000000)
incremental parity OK across 300 randomly-chunked inputs
test result: ok. 2 passed; 0 failed
```

Scope note: only the unkeyed, non-extendable 32-byte mode is implemented, which the author
confirmed by grep is the only mode any call site uses. Keyed / derive-key / XOF are absent by
design, not by omission.

## 2. parry3d replacement (`🧊️3d::{rigid, collision}`) — ✅ PROVEN at parity

Its author reported it had **never compiled** and listed six divergence risks: touching-convention
assumption, hand-derived edge-loop vs Möller's isolated-vertex branching, winding-number vs
parry3d's pseudo-normal containment, single-tree BVH pruning, f32 summation order, and quaternion
math written without source access.

- **It compiles** (standalone `cargo build`, clean).
- **`intersection_test` vs `parry3d::query::intersection_test`**: 600 random rigid transforms
  (translations ±3.5, random unit quaternions, constant-seeded LCG) → **600/600 agree**, with 150
  reported intersecting and 450 not, so agreement is not the trivial always-false case.
- **Degenerate configurations** — directly targeting the touching-convention risk: nested,
  nested-offset, deep overlap, exact face contact, just-inside/just-outside face, exact edge
  contact, exact corner contact, coplanar side-by-side, disjoint; each in big-vs-big and
  big-vs-small → **20/20 agree**, including every exact-contact case.
- **`contains_point` vs `parry3d`'s `PointQuery::contains_point`** — targeting the winding-number
  vs pseudo-normal risk: a 12×12×12 grid spanning the mesh and its exterior → **1728/1728 agree**.

```
agree=600 disagree=0; parry reported intersecting in 150/600 cases
degenerate disagreements: 0
contains_point: 1728 points, 0 disagreements
test result: ok. 3 passed; 0 failed
```

Residual scope: the corpus is axis-aligned cubes under rigid transforms. Non-convex, high-triangle
and sliver-triangle meshes are not covered, so BVH-pruning behaviour at scale remains untested.
That is a narrower and much lower-severity gap than the one it replaces.

## Still open

- **No `wasm32-wasip2` plugin build has been confirmed by anyone, for any plugin.** This is the
  most important remaining gap; it is blocked behind the `semio-framework-os-kernel` repair.
- `semio-framework-os-kernel` is red (~75 errors) from the serde trait-bound change — expected
  fallout from the correct root-cause fix, being repaired by the pilot.
- First-party DEFLATE (`🧰️framework/🔨️modules/🗜️deflate/`) parity against `miniz_oxide` is
  written but unverified; it can be verified with exactly the standalone technique used above.
