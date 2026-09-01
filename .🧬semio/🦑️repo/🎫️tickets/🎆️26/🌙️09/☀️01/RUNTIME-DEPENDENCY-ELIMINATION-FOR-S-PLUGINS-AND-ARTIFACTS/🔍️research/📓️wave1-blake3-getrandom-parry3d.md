# Wave 1 — blake3, getrandom, parry3d

Slice: `blake3` in 🧩️puzzle, `getrandom` in 📐️cad, `parry3d` in 🧩️puzzle.

Honesty key used throughout: **PROVEN** = an actual command ran to completion in the foreground
and I read its output. **UNVERIFIED** = written/edited but never compiled or executed — every
`cargo build`/`cargo test` I started this session was auto-backgrounded by the tool after a 120s
(then 580s) timeout and was killed at turn end per the coordinator's correction; none of them ever
produced a real result. Two `<task-notification>` events I received and initially cited were stale
torn-reads from earlier superseded attempts, not valid passes — disregard anything in the
transcript that appears to cite them as evidence.

## (a) blake3 — 🧩️puzzle

Genuinely dead, not a rewire. `grep -rn "blake3" ✏️s/🔌️plugins/🧩️puzzle` before my edit matched
only the `Cargo.toml` line itself — **zero** `.rs` files in the puzzle plugin called `blake3::`
directly (PROVEN by grep, foreground, instant). Puzzle already reached hashing exclusively through
`semio-framework-hash` (`hash_bytes`/`hash_parts`/`merkle_node`, confirmed present via
`semio_framework_hash`-referencing files under `✏️s/🔌️plugins/🧩️puzzle`). I deleted the unused
`blake3 = "1"` line from
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`. No call site touched, no rewire needed,
no digest-affecting change made in puzzle at all.

Separately — **not my slice, flagging for visibility** — a concurrent peer session rewrote
`semio-framework-hash` itself (`🧰️framework/🔨️modules/🔢️hash/🦀️.rs`) to a full in-house BLAKE3
implementation, moved `blake3` to `[dev-dependencies]` there as a differential oracle, and added an
official-test-vectors JSON fixture plus several oracle-comparison tests. I added one small
oracle-comparison test into that same file early on (before the peer's edit landed on top of mine);
it appears to have survived, renamed, inside the peer's larger test region. **UNVERIFIED**: I never
saw `cargo test -p semio-framework-hash` pass. My first two attempts hit `error[E0433]: cannot find
module or crate serde` — a torn read against the Cargo.toml mid-edit — and my third (fresh) attempt
was killed with the turn before it produced output. Since puzzle's own hashing routes through this
crate and blake3 digests are content-addressed/persisted, **this needs a real run before anyone
trusts it**: `cargo test -p semio-framework-hash`, checked for the peer's
`hash_bytes_matches_recorded_official_blake3_vectors` and the oracle-agreement tests actually
passing.

## (b) getrandom — 📐️cad

`grep -rn "getrandom"` across `✏️s/🔌️plugins/📐️cad` before my edit matched only the
`Cargo.toml` line; `grep -l "rand\|Rng\|random\|Uuid\|uuid"` across every `.rs` file in the plugin
matched nothing (PROVEN, foreground grep).

`cargo tree -p semio-s-plugin-cad --target wasm32-wasip2 -e normal` and the same for
`--target wasm32-unknown-unknown` **DID run to completion in the foreground** (these are metadata
resolutions, not compiles — fast, no sccache contention) — both printed `getrandom v0.3.4` as a
direct, depth-1 leaf under `semio-s-plugin-cad` with nothing else in the graph depending on it. I
then removed the `getrandom = { version = "0.3.4", features = ["wasm_js"] }` line and re-ran
`cargo tree -p semio-s-plugin-cad --target wasm32-unknown-unknown -i getrandom`; it errored
`specification getrandom is ambiguous`, listing only `getrandom@0.2.17` and `getrandom@0.4.2` —
**0.3.4 is completely gone from cad's resolved graph** (PROVEN by this actual command output).

This is unlike `semio-framework-os-flow`'s documented case (that crate's own Cargo.toml comment
says it needs the line for **feature unification** because it transitively pulls `getrandom` via
`wgpu`/`vello`/`hayro` and nothing else in ITS graph turns the `wasm_js` feature on). cad's `ui_wgpu`
dependency only enables `ui`'s `"wgpu"` feature (declarative types), not `"wgpu-engine"` (which is
what actually pulls the `wgpu` crate), so nothing in cad's graph reaches `getrandom` at all, with or
without the explicit line. Conclusion: it was a real dead direct dependency, not a backend-feature
declaration serving a transitive need — confirmed by the tree, not assumed.

**UNVERIFIED**: `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-cad` never completed
(same fate as every other build this session). `cargo tree` proves the dependency edge is gone; it
does not prove the crate still compiles clean. This is a low-risk gap (no source touched besides
the Cargo.toml line) but it is still an actual compile I have not seen succeed.

## (c) parry3d — 🧩️puzzle (the substantive piece)

### Where it landed

New framework code, `🧰️framework/🔨️modules/🧊️3d/`:
- `🌀️rigid/🦀️component.rs` — `Vector3`/`Point3`/`Quaternion`/`UnitQuaternion`/`Isometry3`, f32,
  replacing `parry3d::na::{Vector3,Point3,UnitQuaternion,Quaternion,Isometry3}`.
- `🧿️collision/🦀️component.rs` — `TriMesh` (indexed triangle mesh + a median-split BVH built once
  at construction), `intersection_test` (BVH-pruned pairwise Möller 1997 triangle-triangle test
  with an explicit coplanar 2D fallback), `contains_point` (generalized winding-number point-in-
  solid test), replacing `parry3d::shape::{SharedShape,TriMesh,TriMeshFlags}` +
  `parry3d::query::intersection_test`.
- `🧿️collision/🧪️tests/🦀️component.rs` — the differential suite against `parry3d` (kept only as
  `[dev-dependencies]` on `semio-framework-3d`, never `[dependencies]`).

Both new modules were wired unconditionally into `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/🦀️.rs`
(same pattern as the existing `mesh` module — not gated behind the crate's `brep` feature).

Puzzle's adapter,
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`
(the `GeometryAdapter` region, `Vec3d`/`Point3d`/`Rotation3d`/`Pose3d`/`CollisionShape`, plus the
free `shapes_intersect` fn), was rewired to wrap `semio_framework_3d::{rigid, collision}` instead
of `parry3d::na`/`parry3d::shape`. I confirmed by re-reading the whole 2170-line file that this
adapter region is the ONLY place in the puzzle plugin that touched `parry3d`/`nalgebra` — every
other function in that file (brush placement, collision spatial index, Monte-Carlo overlap sampler)
only calls through the adapter's own public methods, never `parry3d` directly. Puzzle's Cargo.toml
now depends on `semio-framework-3d` (`default-features = false`, so the unrelated `brep`/
`os-kernel` feature stays off) instead of `parry3d`.

### Genuine parity, or approximate? — be precise

**This code has never been compiled, let alone run. Everything below is WRITTEN, NOT VERIFIED.**
I do not know that it compiles. I do not know that the differential tests pass. Treat every claim
in this section as a design intent, not a result.

It is a real, non-stub implementation — a BVH-pruned Möller (1997) triangle-triangle test with a
genuine coplanar branch (2D segment-intersection + point-in-triangle fallback on the dominant
projection plane), not a bounding-box-only approximation. But "real implementation" is not the same
claim as "verified parity," and I want to be explicit that I'm only asserting the former.

**Epsilon**: a single constant `TRI_EPS = 1e-6` (f32) governs plane-distance sign classification
(`sign()`), the 2D cross-product sign classification in the coplanar branch, and the 2D segment-
intersection test. One exception I added during self-review before any build attempt: the
coplanar-vs-crossing branch decision (`dir = n1.cross(n2)`) compares `|dir|²` against
`TRI_EPS² · max(|n1|²,1e-12) · max(|n2|²,1e-12)` rather than a bare `TRI_EPS²`, because `n1`/`n2`
carry each triangle's raw (unnormalized) area — a bare-constant comparison would misclassify small
triangles as coplanar. This fix is itself unverified by any run.

**Differential-test design** (in `🧿️collision/🧪️tests/🦀️component.rs`, `parry3d` as dev-dep
oracle, all UNVERIFIED — never executed):
- Hand-built corpus: disjoint cubes; face-to-face touching cubes; overlapping cubes; a nested cube
  (small cube fully inside a big one, no boundary crossing — I assert both mine and the oracle
  return `false` here, since a pure surface-mesh test cannot see full containment without a
  crossing; puzzle's own `bodies_intersect` already compensates with a center-point containment
  fallback, which is untouched, plugin-side logic outside this adapter); two triangles sharing a
  full edge (a "hinge"); two coplanar overlapping quads; two coplanar disjoint quads.
- `contains_point`: axis-aligned hand-picked samples (center, near-face, outside, on-edge-ish,
  far), plus 64 LCG-seeded (translation, rotation, sample-point) triples.
- `intersection_test`: 200 LCG-seeded (translation, rotation) pose pairs between a half-extent-1.5
  and a half-extent-0.6 cube, asserting my result equals the oracle's on every single case, plus a
  sanity check that the corpus actually contains both an intersecting and a non-intersecting case
  (so the test can't pass vacuously).
- `rotation_between`/`apply`/`compose`/quaternion-construction: differential checks against
  `parry3d::na::UnitQuaternion`/`Isometry3` directly, LCG-seeded, 1e-4/1e-5 tolerances (chosen
  because these compare geometric results — rotated vectors, transformed points — not raw
  quaternion components, so exact bit-parity was never the goal; a sign-flipped-but-geometrically-
  equivalent quaternion (`q` vs `-q`) would still pass these).
- LCG: constant-seeded (Numerical Recipes multiplier/increment), no `rand` crate, per instructions.

**Specific places I know or suspect could diverge from `parry3d`, listed because I could not check
them by running anything**:
1. **Touching convention** — I assumed `parry3d::query::intersection_test` treats an exact
   touch (shared face/edge/vertex, zero-distance contact) as intersecting (`true`), matching the
   classic Möller reference's closed-interval convention, and I wrote `touching_face_to_face_cubes_
   intersect`/`shared_edge_meshes_intersect` to assert exactly that against the oracle. I have never
   confirmed the oracle actually agrees. If `parry3d` uses an open-interval convention instead,
   these are the two tests that will show it, and my implementation will need an epsilon/boundary
   adjustment, not a redesign.
2. **`crossing_interval`'s edge-loop is a generic reformulation**, not a transcription, of
   Möller's isolated-vertex branching (I do not have local access to nalgebra/parry3d source to
   transcribe against — no `~/.cargo/registry` was reachable in this sandbox). It should be
   mathematically equivalent for non-degenerate inputs but I have not proven that beyond reasoning
   through it by hand.
3. **`contains_point`'s winding-number method** is a different algorithm family from whatever
   `parry3d::shape::TriMesh::contains_point` actually does internally (likely a pseudo-normal /
   closest-feature method keyed off `TriMeshFlags::ORIENTED`, not a winding-number sum) — they
   should agree for a closed, consistently-outward-oriented, non-self-intersecting mesh (which is
   the only kind puzzle constructs), but the two methods could diverge for points very close to the
   surface (near the `0.5` winding threshold) in ways I have not measured.
4. **BVH shape**: `intersection_test` builds and prunes with only `mesh_a`'s BVH (mesh_b's
   triangles are transformed into mesh_a's local frame and queried one at a time), not a true
   dual-tree traversal. This is a legitimate, genuinely BVH-accelerated design choice (not brute
   force), but it is an asserted-correct, not measured-correct, architectural difference from
   whatever `parry3d` does internally — should affect performance only, never the boolean result.
5. **f32 summation order** in the winding-number sum (sequential, O(n) triangles) is not proven
   equivalent to `parry3d`'s accumulation order near the decision threshold.
6. `rotation_between`/`apply`/`compose` were written from memory of nalgebra's public
   documentation and standard quaternion algebra (Hamilton product; Fabian Giesen's optimized
   `qvq⁻¹` vector-rotation expansion), not from reading nalgebra's actual source — same "no local
   registry" constraint as above. The differential tests target exactly this; they have not run.

**Also unverified**: `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-puzzle` never
completed — I cannot confirm the puzzle plugin even compiles with the rewired adapter. I did do a
careful manual type-check while writing it (Copy-ness of every wrapped type, `Arc<TriMesh>` deref
coercion into `&TriMesh` at the two call sites, every adapter method's signature cross-referenced
against its call sites elsewhere in the 2170-line file), but that is a self-review, not a compiler
pass, and I would not stake anything on it beyond "no error I could catch by eye."

## Verification commands — actual status

- `cargo test -p semio-framework-hash` — **UNVERIFIED**. Two attempts hit a torn-read compile error
  (`serde`/`serde_json` unresolved, mid-concurrent-edit); the third was killed with the turn before
  producing output.
- `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-puzzle` — **UNVERIFIED**, never
  completed.
- `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-cad` — **UNVERIFIED**, never
  completed.
- `grep -rnE '^(blake3|parry3d|getrandom|nalgebra) ?=' ✏️s --include=Cargo.toml` — **PROVEN**, ran
  in the foreground just now, exit code 1 (no matches), verbatim: empty output, `exit=1`.
- `cargo tree -p semio-s-plugin-cad --target wasm32-wasip2 -e normal` /
  `--target wasm32-unknown-unknown -i getrandom` — **PROVEN**, ran in the foreground, see (b) above
  for the actual output cited.

## What still needs a real run (once the machine frees up)

1. `cargo test -p semio-framework-hash` — confirms the peer's in-house BLAKE3 (which puzzle's
   hashing depends on transitively) is byte-exact. Highest priority: digests are persisted/
   content-addressed.
2. `cargo test -p semio-framework-3d` — the actual differential-oracle pass/fail for `rigid` and
   `collision`. This is the load-bearing check for the whole parry3d slice; nothing above is a
   substitute for it.
3. `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-puzzle` and the same for
   `semio-s-plugin-cad` — confirms both plugins still compile as wasm components with their
   Cargo.toml dependency edges changed.
4. If (2) fails on the touching-convention tests specifically (item 1 in the divergence list
   above), the fix is localized to `crossing_interval`'s boundary handling in
   `🧰️framework/🔨️modules/🧊️3d/🧿️collision/🦀️component.rs`, not a redesign.

## Files touched

- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` — removed `blake3`, removed `parry3d`,
  added `semio-framework-3d`.
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` — removed `getrandom`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`
  — `GeometryAdapter` region rewired onto `semio_framework_3d::{rigid, collision}`.
- `🧰️framework/🔨️modules/🧊️3d/🌀️rigid/🦀️component.rs` — new.
- `🧰️framework/🔨️modules/🧊️3d/🧿️collision/🦀️component.rs` — new.
- `🧰️framework/🔨️modules/🧊️3d/🧿️collision/🧪️tests/🦀️component.rs` — new.
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/🦀️.rs` — wired `rigid`/`collision` mods.
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` — added `parry3d` as
  `[dev-dependencies]` (oracle only).
- `🧰️framework/🔨️modules/🔢️hash/🦀️.rs` — added one small oracle-comparison test early in the
  session; superseded/absorbed by a concurrent peer's much larger rewrite of the same file (not my
  slice — see (a) above).

## Addendum — one build notification did land (read after report was filed)

One of the earlier-started background commands, `cargo test -p semio-framework-3d` (task
`b8hkvp98w`), completed with actual output before being killed elsewhere. I read the completed
output file (not a new build — no command was started to get this). Verbatim relevant tail:

```
   Compiling semio-framework-hash v0.1.0 (...)
   Compiling semio-framework-replication v0.1.0 (...)
   Compiling robust v1.2.0
error[E0433]: cannot find module or crate `semio_framework_deflate` in this scope
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../⚙️codec/🦀️.rs:442:34
    |
442 |         Ok(Self { inflater: Some(semio_framework_deflate::Inflater::new()), ... })
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_deflate`
help: there is a crate or module with a similar name
    | 442 -         ...semio_framework_deflate::Inflater::new()...
    | 442 +         ...semio_framework_hash::Inflater::new()...
[6 more identical-shaped E0433 errors, same file, same crate name mismatch]
error: could not compile `semio-framework-replication` (lib) due to 7 previous errors
```

This is **not my code, and never reached my code**. `cargo test -p semio-framework-3d` with no
feature override compiles under the crate's own `default = ["brep"]` (`✏️s/🔌️plugins/🧩️puzzle`'s
Cargo.toml opts OUT via `default-features = false`, but my direct verification command did not),
which pulls `semio-framework-os-kernel` → `semio-framework-replication` (confirmed just now by
`grep -rl semio-framework-replication 🧰️framework --include=Cargo.toml`, a plain grep, not a
build). `📡️replication`'s `⚙️codec/🦀️.rs` calls a crate named `semio_framework_deflate` that does
not currently exist under that name — apparently a concurrent, unrelated agent mid-renaming it
to/from `semio_framework_hash` (the compiler's own suggestion). The build never got far enough to
compile `semio-framework-3d` itself, `rigid`, `collision`, or the differential test suite — I still
have **zero** signal on whether my code compiles or the differential tests pass.

This does *not* block puzzle: puzzle's own Cargo.toml already sets
`default-features = false` on `semio-framework-3d`, so `brep`/`os-kernel`/`replication` never enter
puzzle's build graph at all. It only blocks a bare `cargo test -p semio-framework-3d` run of the
framework crate in isolation with default features. Once `📡️replication` is fixed by whoever owns
that rename, re-run `cargo test -p semio-framework-3d` (default features are fine once that's
fixed) as the real check on this slice's parry3d work.

## Addendum 2 — central verification (parry3d slice now PROVEN AT PARITY; blake3 framework counterpart PROVEN byte-exact)

The coordinating session reported it verified both highest-risk items centrally, bypassing the
contended in-repo workspace by copying the relevant source file(s) verbatim into a standalone crate
outside the repo with only the third-party original (`parry3d 0.17` / `blake3 1.8.7`) as a
dependency — this sidesteps both the cargo lock contention and the unrelated
`semio-framework-replication` breakage documented in Addendum 1.

I independently confirmed the paper trail before recording this (not a build — file reads only):
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/📓️central-verification.md`
exists at the ticket root (I initially looked for it inside `🔬️verification-parry3d/` per a literal
reading of "beside it" and didn't find it there — it's beside the ticket's other top-level files,
not inside the subfolder), and both `🔬️verification-parry3d/{parity.rs,degenerate.rs}` and
`🔬️verification-blake3/parity.rs` exist and, on inspection, are real, substantive test sources whose
logic matches the reported methodology exactly (600-case random LCG corpus with the same seed/range
described; the ten named degenerate cases; the 12×12×12 `contains_point` grid; the 28-length
official-vector sweep plus 300-case randomly-chunked incremental sweep for BLAKE3). I did not
execute these myself — no separate raw log/output file exists beside the sources, only the
write-up's transcribed output blocks — so this section is the coordinating session's reported
result, corroborated by matching source, not a run I personally witnessed.

### parry3d — ✅ reported PROVEN AT PARITY, all three of my top divergence risks closed

- **Compiles clean** (standalone `cargo build`) — closes the "never compiled" caveat above.
- **`intersection_test`**: 600 random rigid transforms (translation ±3.5, random unit quaternions,
  constant-seeded LCG) vs `parry3d::query::intersection_test` → reported **600/600 agree**, with
  150/600 actually intersecting per parry3d (rules out trivial always-`false` agreement).
- **Degenerate/touching corpus** (my flagged risk #1): nested, nested-offset, deep overlap, exact
  face contact, just-inside/just-outside face (±0.001), exact edge contact, exact corner contact,
  coplanar side-by-side, disjoint — each big-vs-big and big-vs-small → reported **20/20 agree**,
  including every exact-contact case. **My assumption that `parry3d::query::intersection_test` uses
  a closed-interval (touching-counts-as-intersecting) convention is reported confirmed correct.**
- **`contains_point`** vs `parry3d`'s `PointQuery::contains_point` (my flagged risk #3, winding
  number vs parry3d's likely pseudo-normal method): a 12×12×12 grid spanning the mesh and its
  exterior → reported **1728/1728 agree**.
- Risk #6 (quaternion math written from memory without source access) is reported implicitly
  covered by the 600-case and degenerate corpora agreeing, since every case routes through
  `rotation_between`/`apply`/`Isometry3::compose` to place the second mesh.
- **Residual, honestly narrower gap**: the corpus is axis-aligned cubes under rigid transforms only.
  Non-convex meshes, high triangle counts, and sliver triangles are NOT covered — single-tree BVH
  pruning behaviour at scale (my flagged risk #4) is still untested. Lower severity than what it
  replaces, but real, and not something I can currently size.
- Risk #2 (hand-derived `crossing_interval` edge-loop vs Möller's isolated-vertex branching) and
  risk #5 (f32 summation order in the winding-number sum) are not separately isolated by name in
  the reported corpus, but both are exercised as implementation details underneath the
  `intersection_test`/`contains_point` corpora above, which reported zero disagreements.

### blake3 (framework, not my slice, but gates puzzle's hashing) — ✅ reported PROVEN byte-exact

- The framework crate's own 10 tests reported passing, including its blake3 differential-oracle
  tests and NIST SHA-256 vectors.
- Additional coordinator harness vs `blake3 = "1.8.7"`: one-shot parity across 28 official vector
  lengths (0 through 1,000,000 bytes, standard `i % 251` pattern) — reported byte-identical; plus
  300 randomly-chunked incremental inputs (constant-seeded, chunk sizes 1..700, totals 1..9000)
  through `Hasher::update`/`finalize` — reported byte-identical to the one-shot reference.
- Scope note carried from the write-up: only unkeyed, non-extendable 32-byte BLAKE3 is
  implemented — confirmed by the author's own grep as the only mode any call site in the repo uses.

### What is still genuinely open (from the central write-up, unchanged by the above)

- **No `wasm32-wasip2` plugin build has been confirmed for any plugin**, mine included — this
  remains the real outstanding gap for this slice specifically: I still do not know that
  `semio-s-plugin-puzzle` or `semio-s-plugin-cad` compile as wasm components with their changed
  Cargo.toml dependency edges. The standalone-crate technique proves the replaced ALGORITHMS are
  correct; it does not prove the PLUGIN WIRING (my adapter file's `use`/type-substitution edits, the
  `default-features = false` on `semio-framework-3d`, the Cargo.toml edits) compiles.
- `semio-framework-os-kernel` is reported red (~75 errors, unrelated serde trait-bound change,
  someone else's in-progress fix) — this is the same family of unrelated breakage as Addendum 1's
  `semio-framework-replication` finding and blocks a normal in-workspace build of anything that
  pulls it in (puzzle's own `default-features = false` on `semio-framework-3d` avoids `os-kernel`
  via `brep`, but puzzle depends on `semio-framework-os-kernel` directly and unconditionally too —
  see its Cargo.toml — so puzzle is not actually clear of this blocker).
