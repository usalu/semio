# AVI 1.0/any mutate case — reference investigation and decision (not yet implemented)

Subset: `📼️avi` standard `🔖️1.0` subset `✳️any`. Case: `mutate-avi-1-0`.
`AviMutation` confirmed by reading `.../🧬️schema/🧬️mutations/🦀️component.rs`: 13 variants — `NoMutation`,
`SetSnapshot`, `SetMainHeader`, `SetIdx1Present`, `InsertStream`, `RemoveStream`, `SetStreamHeader`,
`SetStreamFormat`, `InsertChunk`, `RemoveChunk`, `SetChunkKeyframe`, `AddUnknownChunk`,
`RemoveUnknownChunk`.

## Verdict

**Credible read+write path exists, but it needs a new dependency I cannot add myself.** Per the
fleet brief's explicit instruction for this case ("Report it and I will add the dependency — do NOT
edit Cargo.toml yourself; stop and report, and I will batch it"), this report stops short of writing
the catalog, oracle dispatcher, or feature file. The real fixture (independent of this decision) is
already derived and committed — see §4.

**Recommendation: add `riff = "2.0.0"` (MIT) to `🧪️oracle/📦️packages/🦀️rust/Cargo.toml`'s
`[dependencies]` + `oracles` feature list**, then let me write an independent AVI 1.0 reader/writer
composed on top of it — the same shape the BCF case already established in this repo (`zip` +
`quick-xml` composed into a format-specific independent codec, because no single crate covers the
whole format credibly). `riff` plays the role `zip`/`quick-xml` play for BCF: a mature, independent,
generic container-layer codec; the AVI-specific `avih`/`strh`/`strf`/`idx1`/`movi` field packing is
then hand-written against the Microsoft AVI 1.0 specification, independent of this repo's own
`engine::{decode_avi, encode_avi}`.

## 1. `cargo search` and source-reading, in the order the brief asked for

### (a) Is there a maintained crate that reads AND writes RIFF/AVI?

`cargo search riff` / `cargo search avi` surfaced four candidates worth reading source for (not just
README):

| Crate | Version | Read | Write | Verdict |
|---|---|---|---|---|
| `avirus` | 0.2.5 | partial | partial | **Disqualified** — confirms the earlier survey, see §2 |
| `rff-format-avi` | 0.1.0 | yes | **no** | **Disqualified** — first-release + write side non-functional |
| `oxideav-avi` | 0.0.9 | yes | yes | **Disqualified on credibility**, see §3 |
| `riff` | 2.0.0 | yes | yes | **Credible — generic RIFF layer only, not AVI-aware** |

`rff-format-avi` 0.1.0's own module doc, read from the downloaded source
(`~/.cargo/registry/src/.../rff-format-avi-0.1.0/src/lib.rs:15-17`), states plainly: *"Status:
**demuxer implemented** ...; the muxer is still scaffolded."* A muxer that doesn't exist cannot
produce a second differential result, and at 0.1.0 it is also a first release — the brief's own
disqualifying combination ("A first-release or long-unmaintained crate ... is worse than none").
Rejected without further reading.

No standalone, credible, AVI-*semantic* read+write crate exists. `riff` reads and writes RIFF
generically (chunk id/size/data, `LIST` nesting, even-byte padding) but has zero knowledge of AVI's
own structures — no `avih`/`strh`/`strf`/`idx1` types at all. That mirrors exactly why the BCF case
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`)
composed `zip` + `quick-xml` rather than adopting a single BCF-specific crate: *"no standalone BCF
crate exists ... so the reference here composes the already-linked, genuinely independent `zip` and
`quick-xml` crates ... rather than adopting a weaker/unlicensed substitute."* The AVI case is the
same shape, one layer down: RIFF is the generic substrate AVI (and WAV) sit on, same as ZIP+XML is
the generic substrate BCF sits on.

`riff` 2.0.0's credibility, checked past the README:
- Source read in full (`src/lib.rs`, 315 lines + 222-line integration test file). `ChunkContents::write`
  correctly patches chunk/LIST sizes after the fact via `seek`, pads odd-length data with one zero
  byte, and rejects >4 GiB payloads. `Iter::next` correctly accounts for the same padding byte when
  advancing (`self.cur + len + 8 + (len % 2)`). No defect found.
- crates.io: created 2018-07-03, **5 releases**, latest 2.0.0 (2023-06-15). **11,963,651 total
  downloads, 731,768 recent** — this is a real, heavily-used dependency, not an experiment.
- GitHub (`frabert/riff`): 14 stars, 10 forks, MIT. Its own crate description names AVI explicitly:
  *"utilities for reading and writing RIFF formatted files, such as Microsoft Wave, Audio Video
  Interleaved or Downloadable Sounds."*
- Not currently linked in `🧪️oracle/📦️packages/🦀️rust/Cargo.toml` — confirmed by reading the file;
  the `oracles` feature list has no `dep:riff` and there is no `[dependencies.riff]` entry.

### (b) `avirus` — verified, not just re-surveyed

Read `~/.cargo/registry/src/.../avirus-0.2.5/src/{lib,frame,frames}.rs` in full (425 lines total).
Confirmed and sharpened the earlier "obscure and metadata-only" survey with concrete evidence:

- `is_formatted` only checks for `RIFF`/`AVI `/`idx1` markers at expected offsets — no header is ever
  parsed into typed fields (no `avih`, no `strh`, no `strf`).
- `Frames::overwrite` (`frames.rs:141-166`) rebuilds output by copying
  `self.stream[..self.pos_of_movi - 4]` **verbatim** — the entire `hdrl` section, i.e. every header
  this subset's `AviMainHeader`/`AviStreamHeader`/`AviStreamFormat` model, is byte-pass-through, not
  re-derived. That is precisely the pattern the fleet brief's "no-byte-pass-through rule" forbids for
  *our* subject; using a reference that itself does it would make every mutation except frame splicing
  untestable.
- Its only real capability is deleting/reordering `movi` frame records and rebuilding `idx1` —
  built for glitch-art frame manipulation (per its own crate description), not general AVI editing.
  It cannot express `SetMainHeader`, `InsertStream`/`RemoveStream`, `SetStreamHeader`,
  `SetStreamFormat`, `AddUnknownChunk`/`RemoveUnknownChunk`, or `SetIdx1Present` — 8 of 13 kinds have
  no path through this crate's API at all.

Confirmed disqualified, with evidence beyond the README this time.

### (c) Is `hound`'s RIFF handling reusable as the generic layer?

No. Read `~/.cargo/registry/src/.../hound-3.5.1/src/{lib,read,write}.rs`. Its chunk-walking code
(`ReadExt`/`WriteExt` traits, `ChunkKind`) is `pub` only in the Rust-visibility sense inside the
crate's own modules — nothing generic is exported for a caller to drive over an arbitrary RIFF form.
Every public entry point (`WavReader`, `WavWriter`, `WavSpec`) is WAVE-shaped: sample format, channel
count, `fmt`/`data`/`fact` chunk semantics are baked into the same functions that walk chunks at all.
There is no seam to reuse for AVI's `hdrl`/`strl`/`movi`/`idx1` shape. `riff` is the correct
generic-layer substitute (see (a)) — not an extraction from `hound`.

## 2. Why `oxideav-avi` 0.0.9 was surveyed in depth and still rejected

This one deserved real scrutiny: unlike the other three, its source shows an actual AVI-*semantic*
muxer and demuxer (8,595-line demuxer, 4,817-line muxer in
`~/.cargo/registry/src/.../oxideav-avi-0.0.9/src/`), with builder methods matching almost every field
this subset's schema needs (`with_avih_flags`, `with_stream_handler`, `with_stream_flags`,
`with_has_index`, per-stream `with_stream_*` for every `strh` field, `write_packet` with a keyframe
flag) plus 60+ dedicated round-trip regression tests, each named after the specific field it pins
down (`round210_strh_handler.rs`, `round292_avih_streams.rs`, `round285_idx1_rec_lists.rs`, …).

It was rejected on credibility, not capability:
- crates.io: created **2026-04-19**, i.e. 4 months old at the time of this ticket, 7 releases,
  1,510 total / 979 recent downloads.
- GitHub (`OxideAV/oxideav-avi`): **0 stars, 0 forks**, 1 open PR, 0 issues — no external adoption
  signal at all, despite 147 commits and an elaborate fuzzing/benchmarking README.
- It is one crate in an entire from-scratch `OxideAV` media-framework ecosystem that is itself brand
  new and shows the same pattern (every sibling crate similarly unstarred). Its own README volunteers
  that the demuxer **"never fails `open()`"** on malformed input and that certain writer options
  deliberately produce **"internally inconsistent"** files for exercising repair tools — reasonable
  design choices for a fuzzing target, but not the posture of a strict reference implementation whose
  output should be trusted as ground truth for someone else's differential test.
- It would also pull in `oxideav-core` (`[dependencies.oxideav-core] version = "0.1"`) as a second,
  equally unvalidated transitive crate from the same young ecosystem.

This is exactly the case the brief warns about: *"A first-release or long-unmaintained crate put in
the position of judging our implementation is worse than none."* `oxideav-avi` is not literally
first-release or unmaintained by the letter of that rule, but it fails the same underlying test —
zero independent validation — and the depth of its feature surface makes it *more* tempting to trust
than `avirus`'s obvious toy status, which is exactly why it needed the closer look rather than a
README skim.

## 3. Registration this report proposes (for the coordinator to land)

```json
{
  "id": "riff-avi-1-0-mutate",
  "ecosystem": "rust",
  "package": "riff",
  "version": "2.0.0",
  "capabilities": ["avi-1-0-mutate"],
  "comparisonProfiles": ["semantic-avi-v1"],
  "license": "MIT",
  "testOnly": true,
  "homepage": "https://github.com/frabert/riff",
  "rationale": "No standalone AVI crate is credible (avirus is a byte-pass-through frame splicer; rff-format-avi 0.1.0's muxer is unimplemented; oxideav-avi 0.0.9 has zero independent adoption). riff 2.0.0 (11.9M downloads, MIT, unmodified generic RIFF chunk reader/writer) is composed with a hand-written AVI 1.0 hdrl/strl/movi/idx1 codec, independent of this subset's own engine::{decode_avi, encode_avi} — same composition shape as the BCF case's zip+quick-xml."
}
```

Cargo.toml change needed (not made by me):
```toml
[features]
oracles = [..., "dep:riff"]

[dependencies]
riff = { version = "2.0.0", optional = true }
```

Once linked, the remaining wave-7 work for this case (catalog `mutationCatalogs`/`KINDS` const +
conformance test, `oracle_apply_mutation` dispatch over all 13 kinds, the `mutate-avi-1-0` feature
file with `Scenario Outline`, and adapter registration) follows the same shape already used across
the other 30 green cases and is not blocked on anything else.

## 4. Real fixture — derived and committed now (independent of the decision above)

The committed `📼️example.avi` (732 bytes) is a stub. Derived a real one from this repository's only
real video, the same source the MP4 and WAV wave-7 cases used
(`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4`, 16 MB, real
footage, confirmed video-only — no audio stream — by a peer via `ffprobe`, reconfirmed here).

```
ffmpeg -y -i "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4" \
  -t 3 -vf "scale=480:-2" -r 15 -c:v mjpeg -q:v 6 -pix_fmt yuvj420p \
  📼️bauen-mit-bestand-mjpeg.avi
```

3 real seconds, downscaled 480×432 (same 10:9 aspect as the source), re-encoded to Motion-JPEG (the
canonical AVI 1.0 video codec — `strf.compression = "MJPG"`, matching this subset's own existing
handcrafted test fixture in `🧬️mutations/🦀️component.rs`). Result: **749,824 bytes** (766,968 with
FS metadata), committed at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧫️fixtures/📼️bauen-mit-bestand-mjpeg.avi`, referenced as
`shared://📼️bauen-mit-bestand-mjpeg.avi`.

Verified real and structurally rich, not synthetic minimal:
- `file`: `RIFF (little-endian) data, AVI, 480 x 432, ~15 fps, video: Motion JPEG`.
- `ffprobe`: genuine `mjpeg` stream, `480x432`, `15 fps`, `45` frames, `3.000000s` duration.
- Raw hex confirms real MS-shaped `RIFF`/`AVI `/`LIST hdrl`/`avih`/`LIST strl`/`strh`/`strf` and,
  crucially for this subset's mutation vocabulary, a real **`JUNK`** top-level chunk (exercises
  `AddUnknownChunk`/`RemoveUnknownChunk`/`unknown_chunks`) and a real **`idx1`** chunk (exercises
  `SetIdx1Present`). Single video stream (exercises `InsertStream`/`RemoveStream` against a
  real base of 1, matching the subset's own `base_snapshot()` shape in the mutation unit tests).

## 5. What is NOT done yet, and why

Per the brief's explicit instruction for this exact fork ("stop and report, and I will batch it"),
this report initially did not touch `🧪️oracle/🔣️component.json`, `🧪️oracle/🦀️component.rs`, or the
`🗿️artifacts/📼️avi/🧪️tests/mutate-avi-1-0/` case — all of them depend on `riff` actually compiling
into the oracle crate, which requires the `Cargo.toml`/`oracles`-feature edit above. Only the fixture
(§4), which needs no new dependency, was completed at that point.

## 6. `riff` linked by the coordinator — case finished

The coordinator added `riff = "2.0"` to `🧪️oracle/📦️packages/🦀️rust/Cargo.toml`'s `[dependencies]`
and `oracles` feature. With it linked, the rest of the case was completed:

- **Oracle module** (`.../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`): an
  independent `ODoc`/`OMainHeader`/`OStreamHeader`/`OStreamFormat`/`OChunk`/`ORiffChunk` model, its
  own `decode`/`encode` walking RIFF chunk framing entirely through `riff::Chunk`/
  `riff::ChunkContents` (never this repository's hand-rolled chunk walker), fresh `avih`/`strh`/
  `strf` byte-layout parse/write functions against the format's own public spec, all 13 kinds'
  `apply_kind`/`inverse_spec`, and a `project` function mirroring exactly the fields
  `AviSnapshot` has slots for. `oracle_apply_mutation`, `oracle_apply_mutation_inverse`,
  `project_avi_1_0` are the crate-level entry points, `oracles`-feature-gated per the repo's
  standard shape. 6 plain `#[test]`s (not `#[semio_framework_async_macros::async_test]`) cover
  encode/decode round trip, a non-AVI RIFF form rejection, apply+inverse for two kinds, an
  unrecognised-kind error, and projection shape.
- **Catalog** (`.../🧪️oracle/🔣️component.json`): registers `riff-avi-1-0-mutate` under
  `"ecosystem": "rust"`, `capabilities: ["avi-1-0-mutate"]`, with a `rationale` that names exactly
  what `riff` provides (the generic RIFF/LIST/chunk framing) versus what the module provides itself
  (AVI 1.0's own `avih`/`strh`/`strf`/`idx1` semantics) — composition honesty, not overstated
  coverage. `mutationCatalogs.kinds` lists all 13 kebab-case kinds; `comparisonProfiles` declares
  `semantic-avi-v1` with `"arrays": "ordered"` (stream/chunk position is semantic identity in AVI,
  unlike BCF's guid-keyed sets).
- **`KINDS` + conformance test**: added beside `AviMutation` in
  `.../🧬️schema/🧬️mutations/🦀️component.rs` (13 kebab-case entries in declaration order) plus
  `kinds_const_matches_enum_variants_in_declaration_order`, matching each variant to its `KINDS`
  entry by direct pattern match (AVI's `OpText` emits camelCase JSON, not this file's kebab-case
  convention, so BCF's `print_op`-keyword-split approach doesn't apply here — matched directly
  instead).
- **Case** (`.../📼️avi/🧪️tests/mutate-avi-1-0/`): `component.feature` with all 13 kinds ×
  mutate/inverse (`@mode-differential` / `@mode-property`, one shared Examples table per BCF
  precedent) plus the `@mode-round-trip` identity scenario, and `🦀️component.rs` registering oracle
  handlers for all 27 scenario ids plus `sut`-gated subject handlers that decode through production
  `decode_avi`, apply through `apply_avi_mutation`, and re-encode through `encode_avi`, projecting
  both sides through the SAME `project_avi_1_0`. `set-idx1-present` and the `*-unknown-chunk` kinds
  act on the fixture's real `idx1` (45 real keyframe entries) and real top-level `JUNK`/`LIST INFO`
  chunks, per the coordinator's instruction, not on stand-ins.

### Two honest findings surfaced by using the real fixture, recorded in the oracle module's doc comment

1. **The fixture's `strh` is 56 bytes, not 64.** `ffmpeg`'s own AVI-1.0 muxer writes the classic
   `AVISTREAMHEADER` with `rcFrame` simply omitted — real, common, spec-legal producer behaviour.
   This oracle's `parse_strh` now tolerates 56+ bytes (missing trailing `rcFrame` fields default to
   zero, the tolerance every real-world AVI reader needs). Production's `decode_avi` requires
   exactly 64 bytes and returns `Err("avi: strh shorter than 64 bytes")` on this real file — a
   genuine pre-existing subject-side gap, not introduced here, in the same spirit as wave 7's
   TIFF/BMP findings. It will surface as a real failure the moment the subject phase compiles;
   not worked around.
2. **Nested `hdrl`/`strl` auxiliaries have no schema slot at all.** The real fixture's `strl` also
   carries a genuine `vprp` (video properties) chunk and a 4120-byte `JUNK`, and its `hdrl` a
   further 260-byte `JUNK` — none of which `AviSnapshot` models. Both this oracle and production
   silently drop ~4.4 KB of real nested data on decode; a schema-completeness gap, not a bug either
   side introduces, and the projection never claims fields neither side actually has.

### Verification (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, real output)

```
$ cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust && cargo test --features oracles --lib avi
running 6 tests
test ...oracles::tests::decode_recognises_a_non_avi_riff_form ... ok
test ...oracles::tests::unrecognised_kind_is_an_error_not_a_silent_no_op ... ok
test ...oracles::tests::encode_decode_round_trips_a_synthetic_document ... ok
test ...oracles::tests::apply_mutation_sets_idx1_present_and_inverse_restores_it ... ok
test ...oracles::tests::apply_mutation_removes_and_inverse_restores_a_chunk ... ok
test ...oracles::tests::project_reports_stream_and_chunk_shape ... ok
test result: ok. 6 passed; 0 failed

$ cargo test --features oracles --lib   # full target, unfiltered
test result: FAILED. 130 passed; 1 failed   # the pre-existing json member-order failure only — not avi

$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-avi-1-0
0 high-priority breach(es) across 0 rule(s):

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-avi-1-0
[test] level=exhaustive cases=1 executed=27 passed=27 failed=0 errored=0 parity=0/0
```

27/27 = 13 kinds × (mutate + inverse) + 1 identity-round-trip. No breach in the full breach cache
(`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`) names `avi` at any priority. A `cargo check --lib`
in `semio-s-plugin-stdio` was attempted to sanity-check the subject module and failed at the
workspace-manifest level in an entirely unrelated plugin (`✒️writer`, a `js-sys` workspace-dependency
inheritance error) — confirming the coordinator's "Rust SUBJECT phase cannot compile right now" note
and that it is unrelated to this case.

### Files touched in this session

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧫️fixtures/📼️bauen-mit-bestand-mjpeg.avi` (new, real fixture)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` (filled in)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (`KINDS` + conformance test added)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧪️tests/mutate-avi-1-0/component.feature` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧪️tests/mutate-avi-1-0/🦀️component.rs` (new)

Not touched: `Cargo.toml`, `📦️lib.rs`, any file outside the `📼️avi` artifact, the ticket itself (not
closed/reopened — coordinator owns it).
