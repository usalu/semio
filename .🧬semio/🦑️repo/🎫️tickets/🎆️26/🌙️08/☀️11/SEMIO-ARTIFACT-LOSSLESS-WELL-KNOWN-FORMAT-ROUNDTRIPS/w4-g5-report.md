# W4 (G5) Report — video↔mp4/avi + audio↔mp3/wav + animation↔gltf/mp4/gif

Agent: W4 group G5, one of the parallel W4 io-leaf agents. Scope: the `s.stdio.semio/v1/video`,
`.../v1/audio`, `.../v1/animation` subsets' bidirectional io bridges — video↔mp4, video↔avi,
audio↔mp3, audio↔wav, animation↔gltf, animation↔mp4, animation↔gif (7 pairs; the group prompt's
line "five format bridges" undercounts its own enumerated list of 7 — implemented the full
enumerated list, matching the master plan's G5 row exactly).

## What was built

14 new leaf files (deserializer + serializer per pair), all under the three subsets' own `🚪️io`
trees (zero edits to any FORMAT artifact's own tree — mp4/avi/mp3/wav/gltf/gif were only ever read):

- `✳️video/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs` — `SemioVideoFromMp4`
- `✳️video/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs` — `SemioVideoToMp4`
- `✳️video/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️component.rs` — `SemioVideoFromAvi`
- `✳️video/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️component.rs` — `SemioVideoToAvi`
- `✳️audio/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️component.rs` — `SemioAudioFromMp3`
- `✳️audio/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️component.rs` — `SemioAudioToMp3`
- `✳️audio/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️component.rs` — `SemioAudioFromWav`
- `✳️audio/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️component.rs` — `SemioAudioToWav`
- `✳️animation/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs` — `SemioAnimationFromGltf`
- `✳️animation/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs` — `SemioAnimationToGltf`
- `✳️animation/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs` — `SemioAnimationFromMp4`
- `✳️animation/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs` — `SemioAnimationToMp4`
- `✳️animation/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️component.rs` — `SemioAnimationFromGif`
- `✳️animation/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️component.rs` — `SemioAnimationToGif`

All under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`. Each is a real
`ArtifactDeserializer`/`ArtifactSerializer` trait impl doing genuine Snapshot-to-Snapshot field
mapping — no byte-level re-parsing of any format's own on-disk bytes (gltf's own
`engine::decode_accessor` is reused for real accessor decode; every other format's own
`ArtifactPack` codec is invoked transparently by the generic `deserializer_entry_of`/
`serializer_entry_of` erasure in the SDK, never re-implemented here).

**Existing files edited** (directed by the master plan / ticket brief, not new-file scope):
- `✳️video/🚪️io/🦀️component.rs`, `✳️audio/🚪️io/🦀️component.rs`, `✳️animation/🚪️io/🦀️component.rs` —
  each was a doc-comment-only stub (W1b scaffold); replaced with real `#[path=...] pub mod ...;`
  mount declarations for this group's leaf files, resolved RELATIVE TO EACH FACET FILE'S OWN
  DIRECTORY (the same mechanism `📦️glue.rs` itself uses one level up) — a deliberate choice to avoid
  touching `📦️glue.rs` (a closer-only hot file per the master plan's hazard management) entirely.
  Confirmed working end-to-end across 9 full-crate `cargo check`/`cargo test` runs this session.
- `✳️video/🎹️composer/🦀️component.rs`, `✳️audio/🎹️composer/🦀️component.rs`,
  `✳️animation/🎹️composer/🦀️component.rs` — each extended (not replaced) the existing `register()`
  with a `bridge_entries()` fn (`vec![deserializer_entry_of::<...>(), serializer_entry_of::<...>(),
  ...]`, one pair per format) + `register_composer_entries(bridge_entries())`, giving all 4 IoKeys
  per pair via the two-row convention the master plan describes.

No `📦️glue.rs`/`📇️catalog.json`/`📜️script.ts`/`🔣️taxonomy.json`/`⚙️engine/🧰️triples`/`🧮️geometry`
edits (none needed or permitted).

## Documented real-world impedance mismatches (per pair, never silently fabricated)

- **video↔mp4** (direct metadata reshape, per the master plan's framing): one `Mp4Track` per
  `SemioVideoStream` (video-handler tracks only, matching `Mp4Track`'s own decode scope). `pts` is
  derived from `dts (running duration sum) + cts_offset`; on export `cts_offset` is always `0`
  (`SemioVideoSample` has no decode/presentation timestamp pair) and per-sample `duration` is
  derived from the delta to the next sample's `pts`. AVC `sps`/`pps`/`nal_length_size` collapse to
  a plain codec-name string on import and are never reconstructed on export (never captured in the
  first place — an honest, real gap, not a bug). Round-trip test is FROM the semio side
  (`video → mp4 → video` is a clean fixpoint); `mp4 → video → mp4` is documented-lossy.
- **video↔avi**: `vids`/`auds` streams map to `Video`/`Audio` kind; any other `fccType` (`txts`,
  `mids`) is honestly dropped (out of `video`'s 3-way kind vocabulary). `pts` is synthesized as
  `chunk_index * strh.scale` (AVI has no per-sample timestamp of its own). `AviMainHeader` is
  synthesized from the first stream on export (AVI has exactly one global header; `video` has
  per-stream dimensions — a real cardinality mismatch, documented). `Subtitle` kind folds to
  `"auds"` on export (AVI has no dedicated subtitle `fccType`).
- **audio↔mp3** (the ticket's explicit, unavoidable asymmetry): mp3's own honest boundary never
  decodes Huffman/MDCT payloads, so `mp3→audio` produces REAL metadata (`sample_rate` via the
  genuine MPEG Table 3.B.2 lookup, keyed by `mpeg_version_id`/`sample_rate_index`; channel COUNT
  from `channel_mode`; real ID3v2 text-frame/ID3v1-presence tags) with every channel's `samples`
  left empty — never fabricated PCM. `audio→mp3` (the reverse) returns a typed `Err` for any
  snapshot with real sample content: there is no MP3 ENCODER anywhere in this repo and building one
  is out of scope ("zero codec reimplementation") — documented as the honest mirror of the forward
  direction's own opaque-payload boundary, not a bug. (An all-empty-samples snapshot round-trips to
  an empty container without erroring, since nothing needs inventing there.)
- **audio↔wav** (the LOSSLESS pair, per the ticket brief): `WavData::{Pcm16,Pcm8,Float32}`
  de-interleave exactly into real f32 `channels[i].samples` (exact PCM↔float conversions — the
  divisors 32768/128 are powers of two, so the conversion is exact, not merely close). `WavData::Raw`
  (24-bit/ADPCM/extensible) yields the correct channel COUNT with empty samples, never fabricated.
  Export ALWAYS writes `WavData::Float32` regardless of the snapshot's `format` label (the only
  lossless encoding for real f32 data) — `audio → wav → audio` is a byte-exact fixpoint for
  `sample_rate`/`channels`; `tags` have no wav-subset counterpart (`other_chunks` stays raw/
  undecoded in wav's own snapshot) and are intentionally dropped, documented.
- **animation↔gltf** (the most direct pairing, per the master plan): reuses gltf's own
  `engine::decode_accessor` for real `sampler.input`/`sampler.output` resolution.
  `channel.target.node` (an index) becomes `AnimTarget.node` (a name) via the node's own `name`,
  else a synthesized `"node#<index>"`; undone on export by building one synthetic node per distinct
  referenced name. `CubicSpline` samplers' in/out tangent thirds are stripped on import (kept only
  the real value third — `AnimKeyframe` has no tangent slot) and `CubicSpline` DOWNGRADES to
  `Linear` on export (no tangent data to re-encode; picking a tangent-free mode instead of
  fabricating zero tangents is the honest choice). `AnimTargetProperty::Custom` channels are
  dropped on export (gltf 2.0's core `GltfAnimationPath` enum has no equivalent — that's a
  `KHR_animation_pointer`-extension concept this bridge doesn't model). Round-trip test is FROM the
  semio side for Linear/Step Translation/Rotation (a clean fixpoint); test fixture values are chosen
  exactly f32-representable (glTF accessors are real IEEE-754 single precision — an f64 value not
  exactly representable in f32 would not byte-for-byte survive the accessor round trip, a genuine
  precision boundary of glTF itself, isolated from by the test's value choice, documented in-line).
- **animation↔mp4** (MINIMAL, per the master plan's explicit instruction — mp4 has no
  animation/timeline concept at all): one `AnimTimeline`/channel per track, targeting node
  `"track-<id>"` with `property: Custom{name:"mp4SampleIndex"}`, `interpolation: Step`, `t` = the
  same real `dts+cts_offset` pts derivation `video↔mp4` uses, converted to seconds via `timescale`.
  Export uses ONLY the first timeline's first channel's keyframes (a genuine multi-timeline vs.
  single-track-timing cardinality mismatch, documented) to build ONE synthetic track with real
  derived sample durations, empty `data`, `width=height=0`, fixed `codec: Other{fourcc:"anim "}` —
  a structurally-valid container capturing ONLY real timing, never a fabricated playable video.
- **animation↔gif** (real but approximate, per the master plan): ONE timeline/channel targeting
  synthetic node `"gif-frame"` with `property: Custom{name:"frameIndex"}`, `interpolation: Step`,
  `t` = real cumulative `delay_cs` in seconds. Export (same first-timeline/first-channel
  cardinality-reduction choice as mp4) derives real `delay_cs` from consecutive keyframe `t` deltas
  (1-centisecond floor, since GIF's `0` commonly means "as fast as possible" to real decoders, not
  "instant"); every produced frame has `width=height=0`, empty `indices` — no pixel/palette data is
  ever fabricated, matching the master plan's own framing of what `animation` can and cannot carry.

## Round-trip tests (fixture-backed, per pair)

Each new leaf file has its own first test region (`#[cfg(test)] mod tests`, new files per the
recipe's own carve-out — no new test FILES were created elsewhere). Every lossy pair's primary test
round-trips FROM the semio side (`semio → format → semio` fixpoint, proving everything the subset
can represent survives, per the exit checklist's own framing); `audio↔wav` additionally proves
byte-exact sample fidelity; `audio↔mp3` proves the documented asymmetry directly (forward-decode
correctness test + a dedicated test asserting the reverse errors on real content, plus a
no-fabrication test for the all-empty case). Each file also carries 1-3 focused honest-boundary
tests (non-`vids`/`auds` AVI stream drop, mono vs stereo mp3 channel count, `Raw` wav fallback
channel count with no fabricated samples, `CubicSpline` tangent-stripping and downgrade,
`Custom`-property drop, fourcc truncation, empty-input edge cases).

## Verification

`cargo check -p semio-s-plugin-stdio --lib` — run 9 times over this session (raw output of the final
0-error run saved as `w4-g5-cargo-check-zero-errors.txt`; an earlier still-foreign-blocked run saved
as `w4-g5-cargo-check-final.txt` for the session's own timeline). **Zero errors and zero
non-cosmetic warnings anywhere under `✳️video/**`,
`✳️audio/**`, `✳️animation/**`** in every run from the point this group's files were complete —
confirmed by grepping every run's full output for `✳️video|✳️audio|✳️animation` next to an `error[`
line; only the same pre-existing, repo-wide `hidden lifetime parameters in types are deprecated`
lint on `fn compose(sources: &[ComposeSource])` (identical shape on every other subset's composer,
not introduced by this wave) appears.

Three genuine issues in this group's OWN files were found and fixed during this session's own
repeated polling (all self-caught via careful re-reading, not by chasing a copy-pasted foreign
error) — see the Verification-outcome paragraph below for how they were confirmed fixed:
1. `✳️video/.../🎥️mp4/.../📤️export/.../🦀️component.rs` — a leftover `let _ = Mp4Box::default();`
   (dead code from an earlier import cleanup) referenced a type whose import had already been
   removed. Deleted the dead line.
2. `✳️animation/.../🧊️gltf/.../📥️import/.../🦀️component.rs`'s own test module called
   `GltfDocBuilder::empty()` (a trait-provided method from `ArtifactBuilder`) without that trait in
   scope. Added `use semio_framework_plugin::ArtifactBuilder;` to the test module.
3. Not a compile error, a real logic bug: the animation↔gltf round-trip test fixture originally
   used `0.7071` for a quaternion component, which is not exactly representable in `f32` — since
   gltf accessor storage is genuinely single-precision, an `f64→f32→f64` round trip would not have
   byte-matched, making the test spuriously fail despite the MAPPING being correct. Replaced with
   exactly-f32-representable values (`0.5`/`0.25`, `0.75`/`0.125` for the two affected fixtures) and
   documented the precision fact in-line so a future reader doesn't mistake it for a mapping defect.

**The crate went green during this session's own polling window.** After ~50 minutes and
9 `cargo check`/`cargo test` polls (error set/count shifting as other concurrent W4/W5 sessions —
`cad`/`drawing`/`image`/`brep`/`workflow`, none this group's scope — landed their own fixes:
18 → 12 → 6 → 4 → 3 → 3 → 0 on `cargo check`), a full `cargo check -p semio-s-plugin-stdio --lib`
finally reported **0 errors** (`w4-g5-cargo-check-zero-errors.txt`). Immediately re-ran the real
target command:

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio" 2>&1 | tail -60
```

**`test result: FAILED. 426 passed; 1 failed; 0 ignored; 0 measured; 1217 filtered out`**
(`w4-g5-cargo-test-final-green.txt`, reproduced twice). The ONE failure is entirely foreign:

```
FAILED: artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::component::tests::real_byte_round_trip_through_pdf_codec
  assertion `left == right` failed
    left: "hellosemio"
   right: "hello\nsemio"
```

— `✳️drawing`'s own pdf serializer leaf (G4's scope; `git status` confirms it as an independent
uncommitted diff this session never touched), not `video`/`audio`/`animation`.

**Every test in this group's own scope passes, 100%.** Grepped the full run for
`subsets::(video|audio|animation)::` — **95 tests, 0 failures**: all 33 of this wave's new io-leaf
round-trip/honest-boundary tests (9 video, 11 audio, 13 animation — listed by name below) plus every
pre-existing video/audio/animation subset test (snapshot/diff/mutation laws, composer/validator
tests) from W2b.

```
video::io::  9/9 ok   (mp4_deserializer×2, mp4_serializer×3, avi_deserializer×2, avi_serializer×2)
audio::io:: 11/11 ok  (mp3_deserializer×3, mp3_serializer×3, wav_deserializer×2, wav_serializer×3)
animation::io:: 13/13 ok (gltf_deserializer×2, gltf_serializer×4, mp4_deserializer×1, mp4_serializer×2, gif_deserializer×2, gif_serializer×3)
```

Two genuine bugs in this group's OWN files (found via this session's own careful re-reading before
the crate went green, not by chasing a foreign error) were fixed prior to this final green run:
1. `✳️video/.../🎥️mp4/.../📤️export/.../🦀️component.rs` — a leftover `let _ = Mp4Box::default();`
   (dead code from an earlier import cleanup) referenced a type whose import had already been
   removed. Deleted the dead line.
2. `✳️animation/.../🧊️gltf/.../📥️import/.../🦀️component.rs`'s own test module called
   `GltfDocBuilder::empty()` (a trait-provided method from `ArtifactBuilder`) without that trait in
   scope. Added `use semio_framework_plugin::ArtifactBuilder;` to the test module.

Also self-caught (a real logic bug, not a compile error): the animation↔gltf round-trip test
fixture originally used `0.7071` for a quaternion component, not exactly representable in `f32` —
since gltf accessor storage is genuinely single-precision, an `f64→f32→f64` round trip would not
have byte-matched, making the test spuriously fail despite the MAPPING being correct. Replaced with
exactly-f32-representable values (`0.5`/`0.25`, `0.75`/`0.125`) and documented the precision fact
in-line.

Raw proof in this ticket folder: `w4-g5-cargo-check-zero-errors.txt` (0-error check),
`w4-g5-cargo-test-final-green.txt` (426 passed/1 failed run, reproduced identically twice),
`w4-g5-cargo-check-final.txt`/`w4-g5-cargo-test-attempt.txt` (earlier still-blocked attempts, kept
for the session's own foreign-blockage timeline).

**Final note**: a THIRD, later `cargo test` re-run for this exit checklist's own literal paste
(`w4-g5-final-exit-checklist-test.txt`) hit a transient, unrelated, genuinely different foreign
failure — `error: couldn't read .../🎒️zip/.../🎒️example.pack.semio: No such file or directory` (a
`zip` artifact fixture, confirmed missing on disk at the time via direct `ls`, not
`video`/`audio`/`animation`, evidently another concurrent session mid-rewrite of that file) — rerun
twice, same transient error both times. This is unrelated to and does not retract the clean,
reproduced-twice 426/1 result above (captured moments earlier, before this fixture went missing);
recorded here per the same never-silently-fixed convention, not chased.

## Files changed (created/edited) this wave

Created (14, listed above under "What was built").
Edited: `✳️video/🚪️io/🦀️component.rs`, `✳️video/🎹️composer/🦀️component.rs`,
`✳️audio/🚪️io/🦀️component.rs`, `✳️audio/🎹️composer/🦀️component.rs`,
`✳️animation/🚪️io/🦀️component.rs`, `✳️animation/🎹️composer/🦀️component.rs`.

## Open item for the orchestrator / W4 closer

One foreign failure remains repo-wide as of this report: `✳️drawing`'s own pdf export leaf test
(`real_byte_round_trip_through_pdf_codec`, `"hellosemio"` vs `"hello\nsemio"` — looks like a missing
newline in that leaf's own text-join logic). Entirely G4's scope, not touched here — flagged for
that group's own closer. Nothing outstanding for `video`/`audio`/`animation`.
