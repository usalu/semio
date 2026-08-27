# Wave 7 — mp4 / isobmff / any — mutation oracle report

Subset: `🎥️mp4` standard `🔖️isobmff` subset `✳️any`. Reference: `mp4` 0.14.

## Files touched (all inside the `🎥️mp4` artifact — nothing else edited)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS: &[&str]` (10 kebab-case entries) and
  `kinds_const_matches_enum_variants_in_declaration_order` test. Everything else in this file is
  unchanged.
- `.../🧪️oracle/🔣️.json` (new) — oracle registration (`mp4-isobmff-mutate`, package `mp4`
  0.14), mutation catalog (`mp4-isobmff-any`, 10 kinds), and a new `semantic-mp4-mutate-v1`
  comparison profile (this subset's own, not the shared stdio manifest — no shared entry references
  `mp4` at all).
- `.../🧪️oracle/🦀️component.rs` (filled in) — `oracle_apply_mutation` dispatch, `project_mp4_mutation`,
  and a `reference` module that owns a small independent `DecodedMovie`/`DecodedTrack` model built
  on `mp4::Mp4Reader`/`mp4::Mp4Writer`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🧫️fixtures/🎥️bauen-mit-bestand-ausschnitt.mp4` (new, 2.7 MB)
  — derived fixture, see below.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🧪️tests/mutate-mp4-isobmff/component.feature` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🧪️tests/mutate-mp4-isobmff/🦀️component.rs` (new) — oracle
  handlers + `sut`-gated subject module (subject cannot compile this wave, per brief; written anyway).

## Mode: `@mode-differential` — full differential, not the §6 fallback

`mp4` 0.14 genuinely reads AND writes ISO-BMFF: `Mp4Reader::read_header`/`read_sample` parse real
tracks/samples, and `Mp4Writer::write_start`/`add_track`/`write_sample`/`write_end` re-mux a fresh
real file from them. This was verified directly (standalone scratch crate against the real derived
fixture, in the ticket scratchpad, before writing the production module) for all 10 mutation kinds,
including the degenerate real case of `remove-track` leaving zero tracks — `mp4` still muxes and
re-parses that cleanly. So every scenario is `@mode-differential`; the §6 reader-only fallback does
not apply to this subset.

One real API gotcha found and worked around: `mp4::Mp4Sample` does not implement `Clone` (needed for
`insert-track`'s track duplication) — handled with a manual field-by-field `clone_sample` helper.

## Real input and the no-audio bound

`ffprobe` on `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4` (16 MB)
reports exactly one `[STREAM]` block, `codec_type=video`, `codec_name=h264` — confirmed directly by
me, not assumed from an earlier report. There is no audio stream anywhere in this file. That bounds
`InsertTrack`/`RemoveTrack`: the only real track available to insert or remove is the video track
itself, so `insert-track` inserts a real structural duplicate of the real video track (same real
samples), never a fabricated audio track. Recorded in the feature's own description.

## Fixture derivation (real stream copy, no re-encoded pixels)

```
ffmpeg -i "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4" \
  -t 1.5 -c copy -movflags +faststart 🎥️bauen-mit-bestand-ausschnitt.mp4
```

`-c copy` performs a genuine stream copy — every sample byte is a verbatim slice of the original
encoded H.264 bitstream, not a re-encode. Result: 2.7 MB, same 1200x1080 `avc1` stream,
`nal_length_size=4`, 47 real samples (the source has B-frames, so composition-time offsets are
genuinely non-zero — confirmed with `ffprobe`). Committed at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🧫️fixtures/🎥️bauen-mit-bestand-ausschnitt.mp4`, referenced as
`shared://🎥️bauen-mit-bestand-ausschnitt.mp4` — the 16 MB original is never read by any scenario.

## Comparison profile

`semantic-mp4-mutate-v1` (this subset's own, in its own `🧪️oracle/🔣️.json`, not the shared
stdio manifest): `ftyp`, per-track geometry/timescale, an SPS/PPS digest, and every sample's
duration/composition-time-offset/sync-flag/payload digest — arrays order-significant, no tolerance
(the container level is lossless, so this is exact structural identity, not the lossy raster
oracles' bucket/histogram approximation). A digest stands in for a sample's raw payload only because
a single real sample runs tens of kilobytes; embedding raw arrays for 47+ samples would blow up the
JSON projection. Track identifiers are deliberately excluded from the projection: `mp4`'s own writer
renumbers tracks sequentially from 1 on every write regardless of what was read — a writer
convention, not content.

## Verification (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, real output, both exit codes checked)

`bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-mp4-isobmff` (run repeatedly while other
wave-7 sessions were concurrently landing their own features; the exact unclaimed set fluctuated
between runs — 2 breaches on one run, 3 on the next — but every one of them was always some OTHER
subset's catalog, e.g.:

```
3 high-priority breach(es) across 1 rule(s):
  testing/contract  svg-1-1-any      Mutation catalog svg-1-1-any (11 kinds) is claimed by no feature
  testing/contract  step-ap214-any   Mutation catalog step-ap214-any (11 kinds) is claimed by no feature
  testing/contract  xml-1-0-any      Mutation catalog xml-1-0-any (8 kinds) is claimed by no feature
```

Neither `mp4-isobmff-any` nor `mutate-mp4-isobmff` ever appeared in the breach set across any run.
(Before the feature description was fixed to avoid a fenced code block being mis-parsed as an orphan
doc string, MY subset DID produce a `testing/contract` breach at that doc-string line; that is fixed
and gone.)

`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-mp4-isobmff` (exit code 0):

```
[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0
```

21 = 10 `mutate-<kind>` + 10 `inverse-<kind>` + 1 `identity-round-trip`, all green.

## Honest limits

- Rust SUBJECT phase does not compile this wave (concurrent os-kernel refactor, per brief) — written
  and `sut`-gated anyway, not verified to compile; only the oracle phase is proven green here.
- `SetTrackCodec`'s new SPS/PPS in the example table are small arbitrary byte arrays (the mutation's
  own parameter value, not a re-derivation of the real fixture), the same precedent PDF's
  `insert-object`/WAV's `set-data` already use for a mutation's fabricated new value on a real
  document.
- `SetSnapshot`'s oracle/subject reading is "replace `ftyp` AND drop the first track's last sample" —
  a real multi-facet whole-document replace rather than degrading to a single-field alias of
  `SetFtyp`, mirroring PDF 1.7's own `set-snapshot` extension precedent.
