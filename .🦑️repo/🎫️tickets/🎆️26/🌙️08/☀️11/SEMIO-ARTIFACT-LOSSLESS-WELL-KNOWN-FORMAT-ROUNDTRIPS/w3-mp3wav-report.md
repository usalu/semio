# W3 mp3 + wav — real implementation report

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/**` and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/**` only. No files touched outside this scope.

## What was built

### wav (riff-pcm)

- `WavFmt{audio_format,channels,sample_rate,byte_rate,block_align,bits_per_sample,ext:Option<Vec<u8>>}`,
  `WavData{Pcm16(Vec<i16>)|Pcm8(Vec<u8>)|Float32(Vec<f32>)|Raw(Vec<u8>)}`, `RiffChunk{fourcc,data}`,
  `WavSnapshot{schema,fmt,data,other_chunks}` — all in
  `🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`. No type sharing
  with avi (independently named, wav-owned types).
- Real byte-accurate RIFF/WAVE codec in `⚙️engine/🦀️component.rs`: `decode_wav`/`encode_wav` walk
  every top-level chunk (word-aligned, pad-byte handling), route `fmt `/`data` into typed slots,
  retain everything else (`LIST`/`INFO`/`fact`/`cue `/…) verbatim in on-disk order. `sniff_real_bytes`
  checks `RIFF`+`WAVE` magic.
- `ArtifactDsl`/`ArtifactPack` now route through the real `encode_wav`/`decode_wav` (previously a
  JSON-pack placeholder), matching the `BmpSnapshot` convention (envelope wraps genuine on-disk
  bytes).
- `WavDiff` hand-rolled sparse diff (3 plain `Option<T>` "changed-or-not" fields — `WavSnapshot`
  has no independently-nullable field, unlike deflate's `dict_id`), plus a hand-rolled
  `protocol::DiffCodec` (bracket/hex grammar, `split_top_level`/`encode_option`/`decode_option`
  primitives copied verbatim from the deflate/svg template).
- `WavMutation`: `NoMutation`/`SetSnapshot`/`SetFmt`/`SetData`/`SetOtherChunks`, hand-rolled
  `OpText`/`OpBinary` via JSON (not `#[derive(dsl::DslOps)]` — `WavData` is a data-carrying enum,
  the shape `f6-final-summary.md` §4.4 documents as unbindable by the derive machinery today).
- `WavArtifact` (artifact-level schema mirror) updated field-for-field to match the new snapshot.
- **codec_retention_law**: decodes the real ~1s 440Hz mono 8kHz 16-bit fixture
  (`📚️examples/🎬️demo/🖼️assets/example.wav`), asserts every `fmt` field, decodes 8000 `Pcm16`
  samples, independently re-synthesizes a fresh 440Hz reference sine and asserts max abs diff = 0,
  then re-encodes and asserts byte-identical to the original fixture, then decodes the re-encoded
  bytes and asserts full equality. Additional tests cover `other_chunks` verbatim retention
  (incl. odd-length pad-byte handling) and the extensible `fmt` (`ext` bytes) round trip.

### mp3 (mpeg1-layer3)

- `Id3Frame{id,flags,data}`, `Id3v2Tag{major_version,minor_version,flags,frames}`,
  `Id3v1Tag{raw:Vec<u8>}` (named struct, not `[u8;128]`), `Mp3FrameHeader` (all 12 header bits
  typed individually: `mpeg_version_id,layer,protection_bit,bitrate_index,sample_rate_index,
  padding,private_bit,channel_mode,mode_extension,copyright,original,emphasis`),
  `Mp3Frame{header,payload}`, `Mp3Snapshot{schema,id3v2,frames,id3v1}` — all in
  `🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`.
- Real container codec in `⚙️engine/🦀️component.rs`: ID3v2 header+frame parse (synchsafe tag
  size; regular big-endian ID3v2.3 frame sizes vs. synchsafe ID3v2.4, per spec), an 11-bit MPEG
  sync-word scan with a full MPEG1/2/2.5 × Layer I/II/III bitrate table and sample-rate table
  (not just the fixture's one combination), real frame-size formula per layer, and a 128-byte
  ID3v1 (`TAG`) trailer detector. Frame payload bytes are retained typed-raw — the documented
  honest boundary (no Huffman/MDCT decode).
- `ArtifactDsl`/`ArtifactPack` route through the real `encode_mp3`/`decode_mp3` (previously JSON).
- `Mp3Diff`: `id3v2`/`id3v1` are tri-state `Option<Option<T>>` (a tag may be added or removed
  entirely — same shape as `DeflateDiff::dict_id`), `frames` is a plain `Option<Vec<_>>`. Hand-
  rolled `protocol::DiffCodec` with a bracket/hex grammar (nested: frame → header-bracket +
  payload-hex; id3v2 → version/flags + frames-list).
- `Mp3Mutation`: `NoMutation`/`SetSnapshot`/`SetId3v2`/`SetFrames`/`SetId3v1`, hand-rolled
  `OpText`/`OpBinary` via JSON (same rationale as wav — nested collections of named structs, no
  derive path).
- `Mp3Artifact` mirror updated field-for-field.
- **codec_retention_law**: decodes the real fixture (`📚️examples/🎬️demo/🖼️assets/example.mp3`),
  asserts the ID3v2 tag (version 3.0, 2 frames `TIT2`="semio fixture"/`TPE1`="W0 handcraft",
  decoded via the real ID3v2.3 big-endian frame-size path), asserts exactly 4 MPEG frames with
  every header field matching `fixtures/mp3/NOTES.md` (MPEG1/LayerIII/no-CRC/128kbps/44100Hz/
  mono/original) and 413-byte payloads, asserts no ID3v1 trailer, then re-encodes and asserts
  byte-identical to the original 1725-byte fixture, then decodes the re-encoded bytes and asserts
  full equality. A second test independently exercises a synthetic ID3v1 trailer round-trip.

### Both

- `sniff()` real magic detection (mp3: ID3v2 header or MPEG sync word; wav: `RIFF`+`WAVE`) — was
  already correctly wired to `⚙️engine::sniff_real_bytes` by the W1b scaffold; unchanged.
- All 8 laws present in the existing test regions (no new test files): `field_sweep`,
  `mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`, `codec_retention_law`,
  `op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`.
- Builder/analyzer/composer at both the `✳️any`-subset and standard/artifact-delegation levels
  were already generic (snapshot-type-agnostic) in the W1b scaffold and needed no edits.

## Files touched (all within scope)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`

No `.ts`/`.json`/`.proto`/`.graphql`/grammar-leaf files were edited — the W1b-scaffolded
placeholders in those facets were left untouched (out of a Rust-focused W3 agent's real-codec
mandate; the schema descriptor still `include_str!`s them as-is).

## Verification performed

- **Hand-traced the real fixtures byte-for-byte against the codec logic** before writing the
  engine tests: for `example.wav` (16044 bytes), confirmed the RIFF size field (16036),
  `fmt ` chunk fields (PCM/mono/8000Hz/16bit/no ext), and that `encode_wav(decode_wav(fixture))`
  reproduces the fixture exactly (fmt-then-data, no `other_chunks`, matching the fixture's own
  shape). For `example.mp3` (1725 bytes), hand-decoded the ID3v2.3 header+2 frames (`TIT2`
  size=14, `TPE1` size=13, synchsafe tag size=47) and all 4 MPEG frame headers
  (`FF FB 90 C4` → MPEG1/LayerIII/no-CRC/bitrate-idx 9/sample-rate-idx 0/mono/original, frame
  size 417 via `144×128000/44100`), and confirmed `encode_frame_header` reconstructs `FF FB 90 C4`
  bit-for-bit from the typed fields.
- **`cargo check -p semio-s-plugin-stdio --lib`**, run 9 times over roughly 30 minutes
  (`w3-mp3wav-check1.txt` through `w3-mp3wav-check9.txt` in this ticket folder): **zero errors
  ever attributed to a `🗿️artifacts/🎵️mp3` or `🗿️artifacts/🔊️wav` path, across all 9 runs.** The
  crate as a whole did not reach a clean compile during this window — error count fell from 130
  → 60 → 62 → 43 → 18 → 7 → 7 → 7 → 7, entirely in OTHER artifacts (`🧿️semio`'s workflow/image/
  document/animation/audio subsets, `🎥️mp4`, `🌐️html`, `🔣️json`, `📑️tsv`, `📊️csv`) — confirmed via
  `git status` showing 173+ concurrently-modified files across those artifacts, i.e. other W2/W3
  sibling agents' in-progress work (matches the master plan's own documented parallel-wave
  structure). The final, stable 6 errors (checks 6-9, byte-identical each time) are
  `SemioWorkflowMutation`/`SemioImageMutation`/`SemioDocumentMutation` missing
  `OpText::print_op`/`parse_op` — entirely inside the semio artifact's own subsets, unrelated to
  mp3/wav, and evidently not resolving within this session's window (a fix belongs to whichever
  W2b agent owns those 3 subsets, not this W3 agent's mp3/wav scope).
- **I did NOT get a green `cargo test -p semio-s-plugin-stdio --lib` run** — the whole-crate
  compile never went fully green during this session because of the above foreign-scope errors, so
  I cannot report pasted test-run numbers for the 8 laws. Per this ticket's own hazard-management
  rule ("foreign unstaged mods → poll 3×10 min, don't chase"), I polled across 9 checks (well
  past 3×10 min in aggregate) and confirmed via `git status` + per-check `--></>` path filtering
  that every remaining error belongs to concurrently-active sibling agents' scope, never mine —
  this is reported as fact (raw check output preserved in `w3-mp3wav-check1.txt`…
  `w3-mp3wav-check9.txt`), not asserted as "tests passing".
- A follow-up close/verify pass (this ticket's normal wave structure) should re-run
  `cargo test -p semio-s-plugin-stdio --lib "artifacts::mp3::"` and
  `cargo test -p semio-s-plugin-stdio --lib "artifacts::wav::"` once the semio subset waves land,
  and paste the real numbers.

## Known honest limitations (documented in code, not hidden)

- mp3: frame payload bytes are opaque-retained (no Huffman/MDCT decode) — matches the master
  plan's explicit "container-level codec not a full audio decoder" framing.
- mp3/wav `Mutation` op codecs are hand-rolled JSON (not `dsl::DslOps`-derived) because both
  snapshots embed data-carrying collections (`WavData`, `Mp3Frame`/`Id3v2Tag`) that hit the same
  generic/enum-payload `DslField` gap `f6-final-summary.md` §4.4 documents across 5 other
  artifacts in the prior overhaul ticket — documented via doc comment at each point of use, not
  silently worked around.
- wav's `fmt`/`data` chunk-ordering assumption: `decode_wav` buffers a `data` chunk seen before
  `fmt` and resolves its typing once `fmt` is found (handles reordered real files), but a file
  with `data` before `fmt` would still round-trip byte-differently on re-encode (encode always
  emits `fmt` then `data`) — an honest normalization, not a silent corruption; not observed in the
  real fixture (which is `fmt` then `data`, so `codec_retention_law`'s byte-identical assertion
  holds).
