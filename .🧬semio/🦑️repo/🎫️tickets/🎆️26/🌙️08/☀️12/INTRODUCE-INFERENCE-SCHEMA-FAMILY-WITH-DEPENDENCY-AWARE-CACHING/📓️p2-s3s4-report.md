# P2/S3+S4 report — stdio media (4) + container (3) inference fan-out

Executor: P2/S3+S4. Scope:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎵️mp3/🏅️standards/🔖️mpeg1-layer3,🔊️wav/🏅️standards/🔖️riff-pcm,🎥️mp4/🏅️standards/🔖️isobmff,📼️avi/🏅️standards/🔖️1.0,🎒️zip/🏅️standards/🔖️2.0,🗜️deflate/🏅️standards/🔖️rfc1950,💾️binary/🏅️standards/🔖️raw}/🪆️subsets/✳️any` + `📦️glue.rs`.

## Pre-flight

- Read `📌️important.md` and `📓️p2-s1a-report.md` (closest sibling template — 8-subset semio fan-out) before starting.
- Read the actual template files S1a produced (`🧿️semio/✳️audio/🧬️schema/💡️inferences/**`) and a container exemplar already on disk (`🔋️energy/🔋️model/…/💡️inferences/🗃entries/🦀️component.rs`, `🪐️space/🏠️home/…/💡️inferences/🆔digest/🦀️component.rs`) to derive the exact repo-wide leaf shapes (family-root 5 files, `📝️text/` 8, `💾️binary/` 6, ≥1 slug dir, `DefaultHasher`-based content digest convention).
- Read `🎒️zip`'s `✳️iso21320` subset directly: its `🧬️schema/` and `🚪️io/` each hold only `🦀️component.rs` + `🟦️component.ts` (a validation-gated dialect STAMP reusing `✳️any`'s `ZipSnapshot` verbatim) — confirmed delegating stamp per the ticket's own test, **skipped**, no inference files created there.
- Read every owned subset's own `📸️snapshot/🦀️component.rs` fresh before authoring (mp3's typed `Mp3FrameHeader`/ID3 tags, wav's `WavFmt`/`WavData` enum, mp4's ISO-BMFF `Mp4Track`/`Mp4Sample` (`stts`-flattened), avi's `avih` `AviMainHeader`/per-stream `AviStream`, zip's `ZipEntry` list, deflate's typed RFC1950 CMF/FLG fields, binary's single opaque `bytes: Vec<u8>`) so every derivation below reads real fields, not a mechanical name-match against an older plan.

## Per-subset: what changed

1. **mp3 (`🎵️mp3/🔖️mpeg1-layer3`) → `⏱duration`** (`Mp3Duration{durationSeconds,frameCount,channelCount}`). Real per-frame fold: for every `Mp3FrameHeader`, looks up MPEG Layer I/II/III's real samples-per-frame constant (384/1152/1152-or-576, ISO/IEC 11172-3 §2.4.2.3) and the real sample rate via the engine's own `sample_rate_hz(version_id, index)` table (bumped `fn` → `pub(crate) fn` in `⚙️engine/🦀️component.rs` so the derivation reuses the real table instead of re-declaring it — mirrors S1a's `subset_tag` visibility bump). `channelCount` reads the first frame's `channel_mode` (`3`=mono⇒1, else⇒2; `0` when no frames — honest unknown, not a fabricated stereo guess). Reserved layer/version/sample-rate-index values contribute `0` duration but are still counted in `frameCount`, matching the engine's own reserved-index treatment.
2. **wav (`🔊️wav/🔖️riff-pcm`) → `⏱duration`** (`WavDuration{durationSeconds,frameCount,bitsPerSample}`). Real per-variant sample count off `WavData` (`Pcm16`/`Pcm8`/`Float32` element count; the honest `Raw` fallback divides byte length by `fmt.blockAlign`, RIFF's own "bytes per interleaved frame" quantity), divided by `fmt.channels` (floored to 1). **Trap #1 caught and fixed one level deeper than usual**: `WavFmt::default()` is a real 44.1kHz/16-bit PCM normal form (not zeroed), so a `#[derive(Default)]` on the OUTPUT struct `WavDuration` would have disagreed with the honest compute and broken `inference_default_law` even though `WavSnapshot`'s own `Default` is hand-rolled correctly — hand-rolled `impl Default for WavDuration { compute_wav_duration(&WavSnapshot::default()) }` instead.
3. **mp4 (`🎥️mp4/🔖️isobmff`) → `⏱duration`** (`Mp4Duration{durationSeconds,trackCount,sampleCount}`). Real `stts`-flattened per-sample `duration`/`timescale` fold per track; container duration = MAX across tracks (bounded by the slowest-ending track), same convention S1a's own semio-animation clip-duration facet established. `0.0` for a `timescale` of `0` (honest degenerate case).
4. **avi (`📼️avi/🔖️1.0`) → `⏱duration`** (`AviDuration{durationSeconds,streamCount,totalFrames}`). Real `avih` MainAVIHeader read: `durationSeconds = totalFrames * microSecPerFrame / 1_000_000` (RIFF/AVI 1.0's own container-level duration relationship — unlike mp4, AVI 1.0 defines duration at the container level, not per-stream). `streamCount = streams.len()`.
5. **zip (`🎒️zip/🔖️2.0`) → `🗃entries`** (`ZipEntries{entryCount,totalUncompressedSize,contentDigest}`, matching the ticket's own container-facet guidance verbatim). Real fold over `entries`: `entryCount = entries.len()`, `totalUncompressedSize` sums every entry's real decompressed `data.len()` (`ZipEntry::data` is always the decompressed payload per the snapshot's own doc comment), `contentDigest` folds every `(name,data)` pair through `std::collections::hash_map::DefaultHasher` — same std-only convention `🔋️model/🗃entries` and `🏠️home/🆔digest` already established. **Trap #3 (io_registry shadowing) addressed explicitly**: `crate::artifacts::zip::io_registry::entries()` exists as a sibling module at the artifact root — my leaf never imports `zip::io_registry` and names its own function `compute_zip_entries` (not bare `entries()`), so there is no unqualified-call collision risk.
6. **deflate (`🗜️deflate/🔖️rfc1950`) → `🪟window`** (`DeflateWindow{windowSize,compressionLevelHint,hasPresetDictionary,payloadSize,contentDigest}`) — **deliberately NOT** a `🗃entries` census. RFC1950 wraps exactly one deflate-compressed stream, not a multi-entry container; forcing zip's shape onto it would fabricate structure it doesn't have. Instead derives real RFC1950 zlib HEADER semantics zip has no equivalent of: `windowSize = 2^(windowBits+8)` (RFC1950 §2.2's own CINFO formula, valid for `windowBits 0..=7`; `8..=15` is spec-reserved and honestly reported as `0`, not fabricated), `compressionLevelHint` echoes the real FLG.FLEVEL-decoded enum, `hasPresetDictionary` reads FDICT via `dictId.is_some()`. Hand-rolled `Default` (same non-zero-snapshot-default trap as wav: `DeflateSnapshot::default()` is `window_bits: 7`, not `0`).
7. **binary (`💾️binary/🔖️raw`) → `📏extent`** (`BinaryExtent{byteLength,isEmpty,contentDigest}`) — **deliberately NOT** an `entries` shape either, per the ticket's own explicit call-out. `BinarySnapshot` is a single opaque `bytes: Vec<u8>` with no header/chunk/entry structure of any kind (confirmed by reading the snapshot fresh — one field). This facet reports exactly what an opaque byte blob honestly supports: real length, real emptiness, real content digest — nothing fabricated.

**Leaf shape ruling applied**: all 7 are pure-fn leaves (`compute_<x>_<slug>(&snapshot) -> Value`), per the coordinator's P2 ruling — none are genuinely per-entity/DAG-shaped with incremental payoff (every derivation is an O(n) fold or O(1) header read over a flat `Vec`/scalar; a merkle dep-chain would cost more than the fold it caches).

**Grammar honesty**: `📝️text/`+`💾️binary/` generic scaffold leaves are the repo-wide declaration-only shape (ID-substituted only — inference values are computed, never authored via DSL text/binary, so there is no encode/decode pair to make format-specific, matching every existing inference family's own text/binary leaves). The DERIVATION content — the actual slug `🦀️component.rs`/`🟦️component.ts` — is genuinely different per subset (mp3 walks MPEG frame headers with real bitrate/sample-rate tables, wav reads RIFF `fmt`, mp4 folds ISO-BMFF `stts` tables, avi reads `avih`, zip census's ZIP entries, deflate reads RFC1950 CMF/FLG header bits, binary reports honest byte extent) — mp3/wav/mp4/avi do **not** end up byte-identical despite sharing the `⏱duration` slug name (same convention S1a's own audio/animation duration facets already established: same name, different real per-format logic).

## Files created (147 total: 7 × 21)

Per subset `<S>` in `{🎵️mp3,🔊️wav,🎥️mp4,📼️avi,🎒️zip,🗜️deflate,💾️binary}` (standards `🔖️mpeg1-layer3,🔖️riff-pcm,🔖️isobmff,🔖️1.0,🔖️2.0,🔖️rfc1950,🔖️raw`), under `.../🪆️subsets/✳️any/🧬️schema/💡️inferences/`:
- `🦀️component.rs`, `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto` (family root)
- `📝️text/{🅰️component.g4, 📖️component.grammar.semio, 🔗️component.graphql, 🔣️component.json, 🔤️component.ebnf, 🛰️component.proto, 🟦️component.ts, 🦀️component.rs}`
- `💾️binary/{🌶️component.spicy, 📡️component.protocol.semio, 🔠️component.abnf, 🟦️component.ts, 🥋️component.ksy, 🦀️component.rs}`
- 1 slug dir (`⏱duration/` ×4 for mp3/wav/mp4/avi, `🗃entries/` for zip, `🪟window/` for deflate, `📏extent/` for binary), each with `🦀️component.rs` + `🟦️component.ts`

Verified via `find … -type f | wc -l` = 21 files, 1 slug dir, per subset (all 7 confirmed).

## Files edited

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `sample_rate_hz` visibility `fn` → `pub(crate) fn` (needed by the sibling `💡️inferences/⏱duration/` module; docstring explains why).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎵️mp3,🔊️wav,🎥️mp4,📼️avi}/…/✳️any/🚪️io/🦀️component.rs` — each `register()` gained a `register_artifact_inferences();` call + a new `pub fn register_artifact_inferences()` calling `::schema::register_artifact_inference_descriptor(...)`, sibling to `register_artifact_schema_descriptor`, matching the mp3/wav/mp4/avi pattern exactly (all four register their schema descriptor from `🚪️io/🦀️component.rs`, not `⚙️engine`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎒️zip,🗜️deflate,💾️binary}/…/✳️any/⚙️engine/🦀️component.rs` — each `register()` gained a `register_artifact_inferences();` call + a new `pub fn register_artifact_inferences()`, sibling to `register_artifact_schema()` (all three register their schema descriptor from `⚙️engine/🦀️component.rs`, matching each artifact's own established pattern rather than a one-size-fits-all assumption).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — added 7 `pub mod inferences { ... }` mount blocks (one per owned subset), mirroring the existing `🧬️mutations`/other siblings' mount shape exactly (family-root `mod component; pub use component::*;` + `pub mod text;` + `pub mod binary;` + one `#[path="."]` submodule per slug: `duration` ×4, `entries` for zip, `window` for deflate, `extent` for binary).

## Concurrent-churn self-corrections during authoring (own mistakes, caught and fixed — recorded per the "verify before amending" precedent)

- **Repeated `🏅️standards`→`🏅️标准`/`🏅️标标` path-typo, same class S1a's own report flagged**: during `Write` calls for `mp3/📝️text/🟦️component.ts`, `📼️avi/…/🔣️component.json`, `🎒️zip/…/🦀️component.rs`, and `🗜️deflate/…/🔣️component.json`, my own composed path substituted the Chinese characters `标准` for `standards`, each time writing into a bogus sibling tree instead of the real one. Caught immediately each time via `ls <artifact>/ | grep 🏅` (two entries instead of one) and cleaned up with a scoped `rm -rf` on the bogus tree only. A final repo-wide sweep (`find … -iname "*标准*" -o -iname "*标标*"`) after all 7 subsets confirms **zero** stray trees remain.
- **A real corruption inside `📦️glue.rs` itself, caught post-edit, not pre-edit**: while inserting the `wav` and `binary` `inferences` mount blocks, the SAME typo (`standards`→`标标`/`标准`) landed inside two `new_string` replacement texts that were supposed to reproduce EXISTING unrelated lines verbatim (the wav `📝️text`/`🔺️diff` mount paths, the binary `inferences/💾️binary` mount path) — this one **did** land in the shared file since the `Edit` tool matched successfully (the corrupted text was still valid, uniquely-matching replacement content, not a match failure). Caught by a mandatory post-edit `grep -n "标" 📦️glue.rs` sweep after every single glue.rs edit (adopted as standard practice for the rest of this wave, per S1a's own precedent) and fixed with a follow-up scoped `Edit` retyping only the corrupted line correctly. **Zero** `标` occurrences remain in `📦️glue.rs` after the final sweep.
- **A second, more serious class of self-inflicted glue.rs bug: block duplication**, discovered only via a post-mount structural audit (`grep -c` per artifact of the family-root `💡️inferences/🦀️component.rs` mount line — expected `1`, found `2` for six of seven artifacts: wav, mp4, avi, zip, deflate — mp3 and binary were clean). Each duplicate was a byte-identical second copy of the same `pub mod inferences { ... }` block landing between that artifact's `diff` and `mutations` mounts (in addition to the correct copy between `snapshot` and `diff`) — root cause not fully diagnosed (my own `Edit` calls were `replace_all: false` against artifact-uniquely-anchored `old_string`s, which should make a silent double-apply impossible; the mechanism is not confirmed, but the fix is verified). Each duplicate was found and removed with a targeted `sed -i '' '<start>,<end>d'` after visually confirming the exact line range via `Read`, followed by a full-file brace-balance check (`open: 1629 close: 1629 balanced: True`) and a final `grep -c` sweep confirming exactly `1` family-root `inferences` mount per artifact across all 7. **This is the single most important verification step of this wave** — a naive "file was written, `grep -n` found no typo" check would have shipped six duplicate module trees into a shared file six other sessions are also editing.

## Verification

Static checks (before the cargo gate, all passing):
- Repo-wide sweep for the path-typo tree (`find "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts" -iname "*标准*" -o -iname "*标标*"`) — **0 matches**.
- `📦️glue.rs`: `grep -n "标"` — **0 matches**; Python brace-balance check — **1629 open / 1629 close, balanced**; per-artifact `grep -c` of the family-root inference mount line — **1629/1 for all 7 of mp3/wav/mp4/avi/zip/deflate/binary** (no duplicates, no misses).
- All 7 subsets: `find … -type f | wc -l` = 21 files, 1 slug dir each — confirmed.
- All 7 `register_artifact_inferences` call sites + fn definitions spot-checked with a final `grep -n` sweep — clean, no corruption, correctly wired (mp3/wav/mp4/avi from `🚪️io/🦀️component.rs`, zip/deflate/binary from `⚙️engine/🦀️component.rs`).

Gate command:
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="<ticket>/🎯️target" cargo check -p semio-s-plugin-stdio --all-targets
```
Raw output: `scratch-p2s3s4-gate.txt` (8926 lines).

## Gate result

```
    Checking semio-s-plugin-stdio v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust)
error[E0433]: cannot find `inferences` in `schema`
   --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././././../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:160:120
    |
160 | ...ndards::v_ap214::subsets::any::schema::inferences::step_artifact_inference_descriptor());
    |                                           ^^^^^^^^^^ could not find `inferences` in `schema`

...
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error; 601 warnings emitted
...
error: could not compile `semio-s-plugin-stdio` (lib test) due to 1 previous error; 737 warnings emitted
```

**Exactly one real compile error in the entire crate (both `lib` and `lib test` targets fail on the same single error, since it sits in a module every target compiles).** Verified by exhaustive grep of the raw output: `grep -c "^error"` = 3 total lines (the one `error[E0433]` + the two downstream "could not compile ... due to 1 previous error" summary lines it causes — not 3 independent errors). The failing file, `🗿️artifacts/📐️step/…/⚙️engine/🦀️component.rs:160`, is **not** in this wave's 7 owned subsets.

**Zero errors and zero warnings trace to any of the 7 owned subsets.** Verified two ways:
1. `grep -B1 "^error" scratch-p2s3s4-gate.txt` — the only `-->` line is the `📐️step` one above.
2. `grep -n "💡️inferences" scratch-p2s3s4-gate.txt | grep -- "-->"` (every warning whose location falls inside ANY artifact's `💡️inferences/` tree, repo-wide) — 7 hits total, belonging to `🔣️json`, `☁️ply`, `📄️pdf`, `🌐️html`, `🧿️semio/✳️brep`, `🧿️semio/✳️drawing` ×2 — **none of mp3/wav/mp4/avi/zip/deflate/binary appear at all.**

## Concurrent-churn observations (external, not fixed)

- **The sole gate blocker is external, textbook "not ours"**: `error[E0433]: cannot find 'inferences' in 'schema'` at `🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:160`. `📐️step` is not one of this wave's 7 owned subsets (mine are the 4 media + 3 container formats listed above). A peer session's in-flight inference work on `step` added a `register_artifact_inferences()` call referencing `schema::inferences::step_artifact_inference_descriptor()` without a matching `step` `inferences` module mounted in `📦️glue.rs` (or one that was mounted and has since been reverted/is mid-edit). Per the gate's own instruction ("if not yours, that is external churn: report with real output and stop — do not loop retrying"), **not touched, not retried**.
- **Self-inflicted issues** (typo trees + the glue.rs corruption + the glue.rs block-duplication bug) are documented above under "Concurrent-churn self-corrections during authoring" — all caught and fixed before the gate ran, none are reflected in the gate output above.

## Pass/fail

**Authored, structurally verified, gate blocked on documented external churn (`📐️step`, not ours).** All 7 subsets: files authored (21/21 each), glue.rs mounted (1 clean mount each, duplicates caught and removed), registration wired (`register_artifact_inferences()` called + defined in each subset's own established registration site), law tests written (`inference_determinism_law` + `inference_default_law` at the family root of all 7, plus ≥3 substantive hand-built-fixture tests per subset). The crate does not produce a clean `cargo check --all-targets` right now, but the one error and all warnings are independently verified to originate outside this wave's scope.
