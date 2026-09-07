# ⚖️ W12 — mutation semantics made lawful, fixtures re-pinned to the laws

Closes W2c's F1–F4 and F6. The vocabulary now rests on **two laws**, and the 134 committed vectors are
re-derived from them rather than pinning what the code happened to do.

## 1. The two laws

**L1 — canonical key order.** Every keyed collection is held in ascending key order: `streams`,
`gcps`, `calibration.cameras` by `id`, `calibration.rig` by `camera_id`, a stream's `frames` by
`(index, asset_id)`, a GCP's `observations` by `(stream_id, frame_index)`. Every `create-*`/`add-*`
inserts at that position (`mutations::ordered_index`, new region in `🧬️mutations/🦀️.rs`) instead of
appending, so a member removed from anywhere comes back exactly where it was. This is what F2 was
really about: 12 of the 14 unrestorable inverses were position failures.

**L2 — ownership.** A `delete-*` removes the record and what it OWNS (a stream carries its frames, a
GCP its observations — they travel inside the record). Anything that merely NAMES the record blocks
the delete with a new `mutation.referenced` Error. Applied to `delete-stream` (a GCP observation
naming it), `delete-camera-calibration` (a stream binding or rig entry), `delete-asset` (a stream
frame, `results.mesh.textureAssetId`, a `results.geo.*AssetId`). `delete-gcp` keeps its
`mutation.cascade` info because the observations it sweeps are its own.

Consequences: `delete-stream` no longer severs another record's data (its cascade branch is gone);
`delete-asset` now drops the durable leaf it owned and `create-asset` drops the leaf an upsert
overwrites, making the pair symmetric in both lanes; `add-stream-frame`'s `kind` ASSERTS the stream's
media kind (`mutation.invariant`) instead of silently rewriting it — that hidden second effect was
the reason the verb had no inverse; `add-gcp-observation` gains the referential invariant that its
`streamId` must exist, which is what makes `delete-stream`'s guard total.

**Guard order (F4), one for the whole vocabulary:** `target-missing` → invariant → no-op → apply. A
malformed argument is a fault whether or not it happens to equal what is stored. Fixed in the four
params blocks W2c named (`sfm`, `dense`, `mesh`, `motion`) and, for the same reason, in
`change-stream-sync` and `update-camera-calibration`, which had the same inversion.

Every inverse is now a **single step of this same vocabulary** and restores `before` exactly. The
Python reference lost all five of its synthetic `__move-*` / `__restore-stream-kind` steps — the
clearest evidence the vocabulary is now sufficient. The one narrowing left is
`__restore-asset-handle`, which adopts the committed `DefaultHasher` content-address.

## 2. Per-kind status

Law columns are from the Python oracle dry-run and the TypeScript twin (see §4); **cargo: run
pending** — no `🗑️generated/w11-check-*.txt` ever appeared, so the gate never opened and no Rust was
compiled from this lane.

| kind | inverse restores | guards (codes raised) | guard order | vectors toy/real/refusal/edge |
|---|---|---|---|---|
| `create-stream` | ✓ delete-stream | duplicate-id, invariant | id → invariant | 1/1/2/1 |
| `delete-stream` | ✓ create-stream | target-missing, **referenced** | target → referenced | 1/1/2/0 |
| `change-stream-sync` | ✓ self | target-missing, invariant, no-op | **fixed** | 1/1/1/1 |
| `add-stream-frame` | ✓ remove-stream-frame @ordered | target-missing, **invariant (kind)**, no-op | **fixed** | 1/1/2/1 |
| `remove-stream-frame` | ✓ add-stream-frame | target-missing (id, index) | target | 1/1/1/1 |
| `replace-stream-source` | ✓ self | target-missing | target | 1/1/1/1 |
| `create-asset` | ✓ delete-asset / self | invalid-asset-payload | payload | 1/1/1/1 |
| `delete-asset` | ✓ create-asset (+leaf) | target-missing, **referenced** ×2 | target → referenced | 1/1/3/0 |
| `create-camera-calibration` | ✓ delete-camera-calibration | duplicate-id | id | 1/1/1/0 |
| `update-camera-calibration` | ✓ self | target-missing, invariant, no-op | **fixed** | 1/1/1/1 |
| `delete-camera-calibration` | ✓ create-camera-calibration | target-missing, **referenced** | target → referenced | 1/1/2/0 |
| `create-rig-extrinsic` | ✓ delete-rig-extrinsic | duplicate-id, invariant | id → invariant | 1/1/2/0 |
| `delete-rig-extrinsic` | ✓ create-rig-extrinsic | target-missing | target | 1/1/1/1 |
| `update-rig-extrinsic` | ✓ self | target-missing, invariant, no-op | already correct | 1/1/1/1 |
| `create-gcp` | ✓ delete-gcp | duplicate-id | id | 1/1/1/1 |
| `delete-gcp` | ✓ create-gcp (owns observations) | target-missing (+cascade info) | target | 1/1/1/1 |
| `add-gcp-observation` | ✓ remove-gcp-observation @ordered | target-missing, **invariant (stream)**, no-op | new, correct | 1/1/1/1 |
| `remove-gcp-observation` | ✓ add-gcp-observation | target-missing (id, index) | target | 1/1/1/1 |
| `update-ingest-params` | ✓ self | invariant, no-op | already correct | 1/1/1/1 |
| `update-feature-params` | ✓ self | invariant, no-op | already correct | 1/1/1/1 |
| `update-match-params` | ✓ self | invariant, no-op | already correct | 1/1/1/1 |
| `update-sfm-params` | ✓ self | invariant, no-op | **fixed** | 1/1/0/1 |
| `update-dense-params` | ✓ self | invariant, no-op | **fixed** | 1/1/0/1 |
| `update-mesh-params` | ✓ self | invariant, no-op | **fixed** | 1/1/0/1 |
| `update-motion-params` | ✓ self | invariant, no-op | **fixed** | 1/1/0/1 |
| `update-geo-params` | ✓ self | invariant, no-op | already correct | 1/1/1/1 |
| `replace-job` | ✓ self | no-op | n/a | 1/1/0/1 |
| `replace-sparse` | ✓ self | no-op | n/a | 1/1/0/1 |
| `replace-dense` | ✓ self | no-op | n/a | 1/1/0/1 |
| `replace-mesh-result` | ✓ self | incomplete-mesh, no-op | invariant → no-op | 1/1/1/1 |
| `replace-trajectory` | ✓ self | target-missing (clear absent), no-op | target → no-op | 1/1/1/1 |
| `replace-tracks` | ✓ self | no-op | n/a | 1/1/0/2 |
| `replace-geo-products` | ✓ self | target-missing, no-op | target → no-op | 1/1/1/1 |
| `replace-qc` | ✓ self | target-missing, no-op | target → no-op | 1/1/1/1 |
| `commit-reconstruction` | ✓ (refusal-only, see below) | invalid-reconstruction-{sparse,asset,mesh} | staged-handle order | 0/0/3/0 |

**134 vectors** (was 132), **134 inverse scenarios** — one per vector, no exceptions. 80 applied, 36
rejected, 18 warned no-op. The generator now *asserts* `inverse_restores` for every vector and the
`NO_INVERSE_BODY` template ("this vector's inverse does NOT restore, pinned here") was **deleted** —
no fixture, feature row, module or drift list pins a gap any more.

`commit-reconstruction`'s inverse gained the asset reversal it lacked (`create-asset` of the
overwritten payload, `delete-asset` of a newly minted one, after the result lanes are restored so the
referential guard sees the base references). **Remaining honest gap, documented in the leaf:** the
durable leaves a commit mints for the SPARSE cloud and the MESH are addressed by content id, not by an
`assets` key, and no verb in this vocabulary can drop them. All three committed vectors of this kind
are refusals, so nothing pins that as if it were the law.

**New outcome code `mutation.referenced`.** There is no normative outcome-code registry to extend:
`🧬️schema/**/🔣️.json` types payloads and documents only, and codes live in the Rust leaf, the TS twin
and the Python reference (all three updated). Repo-wide `🎯️outcome` vocabulary (`rejected` + `path`)
is unchanged.

## 3. F1 — the f32 widening is a framework bug (NOT edited here)

- `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:115` — inside `impl_float_codec!`:
  `DslValue::Number(Number::Float(*self as f64))`; line **129** applies it to `f32` as well as `f64`.
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:31` — `Number` has only `Float(f64)`; there is no f32 lane.
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1064` `write_float` / `:1124` `format_f64` then print the
  **f64's** shortest round-trip lexeme, so `0.42f32` reaches the wire as `0.41999998688697815`.

**Verdict: a bug.** `serde_json::serialize_f32` writes the f32's own shortest round-trip lexeme
(`0.42`); that is what every fixture, every `.dsl.semio` carrier and every other plugin's committed
bytes were authored against, and `write_float`'s own test is named
`write_float_matches_serde_json_byte_for_byte` — the f32 path silently exits that contract. Widening
by binary value is not the same as widening by decimal identity.

**One-line fix** (framework owner's call): split `f32` out of the macro and widen through its own
shortest decimal instead of its bits —

```rust
impl ToValue for f32 {
    fn to_value(&self) -> DslValue {
        DslValue::Number(Number::Float(self.to_string().parse::<f64>().unwrap_or(*self as f64)))
    }
}
```

`f32::to_string()` is the shortest-round-trip f32 lexeme, so `0.42f32` becomes `0.42f64` and prints
`0.42`; `FromValue`'s `as f32` still round-trips. (A `Number::F32` lane would be the deeper fix.)

**Fixtures are now printer-agnostic**, so they are correct under either behaviour and the framework
decision cannot invalidate them. `🐍️w2c-bases.py`'s `_quantize` no longer merely f32-round-trips: it
snaps to the most precise binary fraction whose **shortest f32 lexeme is also its shortest f64
lexeme** (the scale adapts to magnitude), and `f32_lexeme_agrees` asserts that property for every
number in both base scenes and in every one of the 536 emitted before/after/mutation/diff documents.
Values that no f32 can hold are exempt — they cannot be on an f32 lane.

## 4. Validation actually run

```
python3 🐍️w2c-generate.py --apply  → 134 vectors, 134 inverse scenarios, 812 files
python3 🐍️schema-validate.py       → 503 documents, 0 violations; 134 payloads, 0 violations
python3 🐍️fixture-audit.py         → 406 uris, 0 missing, 406 resolved, 0 unexpanded
python3 🐍️mount-check.py           → 0 dangling
python3 🐍️python-oracle-dryrun.py  → 271 planned, 271 registered, 271 passed, 0 failed
bun nx run @semio-tech/remodel-js:test --skip-nx-cache → 4 files, 1353 passed (was 1333)
bunx tsc -p tsconfig.w5-remodel-stories.json → zero diagnostics under .storybook/stories/remodel
bunx tsc --noEmit --strict fixture.ts (standalone)  → exit 0
rustfmt --edition 2021 --emit stdout (169 changed/generated Rust leaves) → 0 parse failures
```

**cargo: NOT run — the gate opened RED, on someone else's breakage.** `🗑️generated/w11-check-2.txt`
landed at 22:47 and is not green: `error: could not compile semio-framework-plugin-host (lib) due to
5 previous errors`. All five are one peer's in-flight kernel migration, none of them remodel's —
`TurnResult` gained a `cold_pair_ingress` field and a method gained a 5th argument, and the host's
`🧵️shard/🦀️.rs:1910`, `⏳️runtime/🦀️.rs:283,380` and `🖥️host/…/🦀️.rs:2105,2136` have not caught up
(`E0063` ×3, `E0061` ×2). `w11-check-1.txt` has exactly the same single failure. A peer is already on
it: `cargo check -p semio-framework-plugin-host --lib` (pid 33084) has been running 13 minutes.

I deliberately did NOT start a build anyway. The box is at load 39 with **seven** concurrent peer
cargo invocations (energy ×3, wgpu ×2, raster, the workspace `--keep-going`), and
`/System/Volumes/Data` swung from 48 GiB free at 22:40 down to 32 GiB (97%) just after the gate
opened — the same exhaustion that killed `central-check-2.txt` at 22:35 with `failed to write …
full.rmeta: No space left on device (os error 28)`. Seeding a lane-private target dir (4.4 GiB) plus
an eighth rustc would likely fail and would starve the peer who is unblocking the host. By 22:50 the
disk had recovered to 76 GiB but the concurrent cargo count had risen from 7 to **16**, so the
decision stands rather than being merely deferred.

**To run it once the host is fixed:** `cargo test -p semio-s-plugin-remodel --lib` — and note that the
remodel plugin reaches the host only OFF-wasm, so `cargo check -p semio-s-plugin-remodel --lib
--target wasm32-wasip2` type-checks this lane's Rust even while the host is broken.

What WAS proved about the Rust without rustc: all 35 changed leaves and all 134 regenerated case
modules parse (`rustfmt --edition 2021 --emit stdout`, 0 failures over 169 files) — syntax only, not
type-checking. The Rust changes are asserted structurally (mount-check) and semantically (the Python
reference and the TypeScript twin each replay all 134 vectors and all 134 inverses byte for byte
against exactly these laws), not by rustc.

## 5. Base-scene corrections these laws forced

- Both scenes are now in canonical key order (`canonical`/`assert_canonical`): the toy scene's GCPs
  were `gcp-ridge, gcp-corner` and the survey's `north, south, roof, unsurveyed` — neither sorted,
  which alone broke `delete-gcp`'s inverse.
- toy gains `asset-spare` (referenced by nothing — the only asset an applied `delete-asset` can
  target) and `asset-dsm`, the raster `results.geo.dsmAssetId` NAMED but the document never carried:
  exactly the dangling reference L2 exists to prevent.
- `orbit-survey` gains the `orbit-spare` stream (no GCP observes it — the only stream an applied
  `delete-stream` can target) and the `orbit-orphan` calibration-target raster.
- Vectors moved by the new guards: toy `delete-stream` → `stream-a`, toy `delete-asset` →
  `asset-spare`, realworld `delete-stream` → `orbit-spare`, realworld `delete-asset` →
  `orbit-orphan`, realworld `add-stream-frame` `kind` → `video`. Three former "documents the gap"
  edge vectors became genuine refusals (`delete-camera-calibration-referenced`,
  `delete-asset-referenced-frames`, `delete-asset-geo-product`), and two refusals are new
  (`delete-stream-referenced`, `add-stream-frame-kind`).

**L1's forward half is really exercised, not just its inverse half.** Three committed vectors insert a
member into the MIDDLE of its collection rather than at the end — `create-stream-unbound`
(`orbit-handheld` at position 0 of 4), `create-gcp-realworld` (position 1 of 5) and
`create-gcp-unobserved` (position 0 of 5) — so an implementation that appended would fail on the
committed `after`, not merely on the inverse.

## 6. Files changed

Rust (mine), under `…/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`: `🦀️.rs` (new `🔖️CanonicalOrder`
region); `🔺️diff/🦀️.rs` of `🌱create-stream`, `🪓delete-stream`, `⏱️change-stream-sync`,
`➕add-stream-frame`, `🧷create-asset`, `🗞️delete-asset`, `🔭create-camera-calibration`,
`🛠️update-camera-calibration`, `🚫delete-camera-calibration`, `⛓️create-rig-extrinsic`,
`🧿create-gcp`, `🔎add-gcp-observation`, `🧮update-sfm-params`, `🌁update-dense-params`,
`🕸️update-mesh-params`, `🏎️update-motion-params`; `↩️inverse/🦀️.rs` of `➕add-stream-frame`,
`🔎add-gcp-observation`, `🏁commit-reconstruction`, `🧷create-asset`, `🗞️delete-asset` and docstring
corrections on `🪓delete-stream`, `🚫delete-camera-calibration`, `🚮delete-gcp`,
`✂️delete-rig-extrinsic`, `➖remove-stream-frame`, `🚷remove-gcp-observation`; `🪓delete-stream/🦀️.rs`
(unused import).

Other repo files: `🧬️schema/🧬️mutations/🟦️.ts` (TS twin — same laws), `🧪️tests/📸️mutate-remodeling-1/`
`🐍️.py` (reference — same laws, `__move-*` steps removed) + `🥒️.feature` + `🦀️.rs` (regenerated
registrations), 134 case directories under `🧬️mutations/*/🧪️tests/**` (812 files),
`📦️packages/🦀️rust/🦀️.rs` (mounts only), `🔮️oracle/🔣️.json` (`fixtureCoverage.vectors` 132 → 134), `🧫️fixtures/🏁️commit-reconstruction/**`,
`.storybook/stories/remodel/fixture.ts` (F6 — regenerated from the corrected toy scene, plus
`RemodelDurableArtifact` / `RemodelWatertightReport` story types and the `durableArtifacts` /
`geo.origin*` / `dense.confidence,classification` / `mesh.watertight` lanes it was missing).

Ticket generators (kept): `🐍️w2c-bases.py`, `🐍️w2c-diff.py`, `🐍️w2c-vectors.py`, `🐍️w2c-generate.py`.

## 7. Handover — three editor call sites the new guards reach (W11's files, NOT touched)

1. **`✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs:67` `replace_document_operations` will now fail
   mid-sequence.** It deletes gcps → streams → cameras (a correct dependency order, by luck) but never
   clears `calibration.rig`, so every `delete_camera_calibration` of a camera the rig names is now
   refused with `mutation.referenced`. Fix: emit `delete_rig_extrinsic(entry.camera_id)` for each
   `current.calibration.rig` entry BEFORE the camera deletes, and `create_rig_extrinsic` for each
   `next.calibration.rig` entry AFTER the camera creates (the command does not transplant the rig at
   all today — a pre-existing gap).
2. **`✏️editor/🎮️commands/🖼️import-frame-payload/🦀️.rs:129`** adds an `ImageSequence` frame to whatever
   stream `batch_stream_id` resolves to. If a video import created that stream as `Video`, the append
   used to silently rewrite the stream's kind and now refuses with `mutation.invariant`. Both import
   paths create their stream with the kind they later assert, so the only affected case is mixing a
   still-image drop into a video-derived stream — which should refuse.
3. `✏️editor/🦀️.rs:1103` and `📼️import-video-frame-payload/🦀️.rs:205` are consistent and unaffected.
