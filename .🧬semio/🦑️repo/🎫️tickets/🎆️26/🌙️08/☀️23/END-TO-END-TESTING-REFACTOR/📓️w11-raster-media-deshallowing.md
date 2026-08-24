# Wave 11 — de-shallowing the raster/media slice

Scope: `📷️png`, `📷️jpg`, `🎞️gif` (87a and 89a), `🖼️bmp`, `🖼️tiff`, `🎥️mp4`, `🎵️mp3`, `📼️avi`, `🔊️wav`.
Every command quoted below was actually run; exit status was read from the tool's own status line,
never through a pipe.

---

## 1. What was shallow, and what it is now

| Case | Before | After |
|---|---|---|
| `mutate-png-1-2` | projection = geometry + sample digest; **15 of 17 kinds could not move it**; 3 arms returned the input re-encoded | whole-document projection (palette, gAMA/cHRM/sRGB/pHYs/bKGD, tIME, text chunks, private chunks, samples); **15 of 17 kinds observable and asserted**; 2 documented unobservable |
| `mutate-bmp-v3` | 4 of 7 kinds `reencode_unchanged`; palette rows targeted index 0, described as unused when it is the most-used entry | indexed oracle (`set_indexed_color`/`get_palette`/`encode_with_palette`); **all 7 kinds observable and asserted**; fixture re-derived with real palette slack |
| `mutate-gif-87a` | `set-pixel-aspect-ratio` accepted and discarded; background index silently dropped; interlace invisible; shared projection hid 5 kinds | subset projection; LSD scalars patched; interlace read off the Image Descriptor; **all 12 kinds observable and asserted** |
| `mutate-gif-89a` | `set-frame-interlace` invisible (decoder erases the flag AND the row permutation) | flag read off the Image Descriptor, rows re-interleaved on encode; laws switched to the shared helpers; **all 21 kinds asserted** |
| `mutate-jpg-jfif-1-01` | 8 of 12 kinds `passthrough`; the 400 000 tolerance also swallowed **geometry** and every JFIF field | 3 of those 8 made real (JFIF header, insert/remove other segment); exact members re-typed as strings; DQT digests added; **7 of 12 observable and asserted**, 5 documented unobservable |
| `mutate-mp4-isobmff` | no observability assertion; the `set-sample-sync` row named a sample that was never a key frame | law asserted; the row now clears the fixture's real second sync sample (0-based 27, `stss` = {1, 28}) |
| `mutate-wav`, `-avi`, `-tiff`, `-mp3` | no observability assertion | `law::mutation_is_observable` asserted with an empty exemption list |

## 2. New law: observability

`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` gained `mutation_is_observable` /
`mutation_is_observable_within`. A `mutate-<kind>` scenario whose projection is bit-identical to the
untouched input now FAILS, unless the kind is named in an exemption list the feature description
also has to justify. This is what caught four of the findings in §4.

## 3. Defects found in the REFERENCE libraries

1. **`png` 0.18.1 — `Info::source_gamma` and `Info::source_chromaticities` are never assigned.**
   Both are declared, both are documented as the members to "prefer … to also get the derived
   replacement from sRGB chunks", both are initialised to `None` in `Info::default`, and
   `decoder/stream.rs`'s `parse_gama`/`parse_chrm` write only `gama_chunk`/`chrm_chunk`. A caller
   who follows the crate's own advice reads `None` for every file that carries a gAMA or cHRM chunk.
   Found by the observability law failing `mutate-set-gamma` and `mutate-set-chromaticities`.
2. **`gif` 0.13.3 — `Encoder::write_screen_desc` hard-codes three Logical Screen Descriptor bytes**
   (`b"GIF89a"`, `0u8 // bg index`, `0u8 // aspect ratio`) with no setter for any of them. The 87a
   oracle patched only the version digit, so `set-background-color-index` and
   `set-pixel-aspect-ratio` were accepted and discarded.
3. **`gif` 0.13.3 — `Decoder` erases the interlace flag AND de-interlaces the rows**
   (`reader/converter.rs:72`), so a round trip through it is invisible to a flag mutation on BOTH
   GIF standards. The 89a module already documented the encoder half of this and still could not see
   the mutation, because it trusted `Frame::interlaced` on read.
4. **`image` 0.25.10 — `BmpEncoder` writes `0` for both pixels-per-metre fields and always stores
   rows bottom-up**, and `BmpDecoder::get_palette` always returns 256 entries regardless of
   `biClrUsed` (deliberate zero-padding, `read_palette`'s own comment). None of that is reachable
   through its API.

## 4. Defects found in THIS repository

1. **`encode_png` emitted `tRNS` alongside colour type 6.** §11.3.3 forbids it, and the reference
   decoder rejects the file outright (`ColorWithBadTrns`). Any PNG with transparency re-encoded by
   this repository produced a file no conforming decoder would read. Fixed; the alpha is already
   resolved into `pixels`, so nothing is lost. Regression test:
   `trns_is_not_re_emitted_alongside_the_canonical_rgba_colour_type`.
2. **`encode_bmp_indexed` wrote `snap.colors_used` as `biClrUsed` while writing
   `snap.palette.len()` table entries.** Neither `InsertPaletteEntry` nor `RemovePaletteEntry`
   maintains that field, so any palette insertion or removal produced a header describing a table
   the encoder did not write. Fixed to write the real table length.
3. **`mutate-mp4-isobmff`'s `set-sample-sync` row addressed a sample that was never a key frame.**
   The fixture's own `stss` box lists exactly two sync samples, at 1-based ids 1 and 28. The row
   named 0-based index 2 and set `sync: false` on a flag that was already false. It now names index
   27 — the second real key frame — and the mutation is a real one.
4. **The committed BMP fixture could not express three of its own seven kinds.** All 233 palette
   entries were referenced (index 0 alone covers 5 659 668 of 5 975 040 pixels), so any targeted
   palette edit orphans a colour and `encode_bmp` legitimately refuses it. The feature file claimed
   the opposite in as many words. Re-derived with 240 entries — 233 real plus 7 spare — which is
   the only shape that makes both a targeted edit and an insertion representable.
5. **`semantic-jpg-mutate-v1`'s 400 000 tolerance was swallowing exact claims.** The engine applies
   `tolerance` to every number in the projection, so `width: 2275` and `width: 3` compared EQUAL,
   as did every JFIF header field. Every exact member is now a string. Measured on the fixture: a
   decode/re-encode at q90 moves at most 2 018 pixels between luma buckets, q50 10 014, q5 55 570 —
   all inside the slack, which is why `set-re-encode-quality` is now observed through the DQT
   payload digests instead.
6. **`mutate-png-1-2` / `mutate-bmp-v3` subject halves could not compile.** Both wrote
   `use protocol::Mutation;`, and `protocol` is an `extern crate` ALIAS private to the plugin's own
   `📦️glue.rs` — no such path exists for a dependent. Fixed the way `inverse_svg_basic_mutation`
   already had been: `inverse_png_mutation` / `inverse_bmp_mutation` / `inverse_jpg_mutation` free
   functions in each subset's own vocabulary.
7. **`mutate-jpg-jfif-1-01`'s subject `inverse` asserted nothing.** It applied the mutation, threw
   the result away, and re-encoded a fresh parse of the pristine original — which restores the
   document by construction. Now applies the real `JpgMutation::inverse` on top of the real forward
   result.

## 5. The two `✳️baseline` subsets — vocabularies written, catalogs deliberately not

Both had no `🧬️schema/🧬️mutations` at all. Both now have a handcrafted, checker-derived one:

* `🖼️tiff/…/✳️baseline` → `TiffBaselineMutation`, 9 kinds, one per axis of
  `check_tiff_baseline_conformance` (Compression, PhotometricInterpretation, BitsPerSample, the
  Tile* pair, StripOffsets).
* `📷️jpg/…/✳️baseline` → `JpgBaselineMutation`, 10 kinds, one per axis of
  `check_baseline_conformance` (SOF marker, sample precision, arithmetic conditioning, the
  per-class Huffman table count, the component count and sampling factors). Both insertion kinds
  carry an `index`, because without one the inverse of removing the second of four Huffman tables
  could only append it back and the vocabulary would not be invertible. TIFF needs no such field:
  `apply_tags` re-sorts an IFD's entries by tag number after every application, which is TIFF 6.0
  §2's own requirement, so a removed tag re-inserts where it left.

Neither declares a `mutationCatalogs` entry or an exhaustive case, and the reason is a finding in
its own right: **both encoders normalize every axis these vocabularies address.**

* `encode_tiff` regenerates all of `CORE_STRIP_TAGS` on IFD 0 from the raster it is about to write —
  `BitsPerSample` 8, `Compression` 1, `PhotometricInterpretation` 2, `SamplesPerPixel` 3 — so four
  of the six conformance kinds cannot survive a re-serialization.
* `encode_jpg` writes `[0xFF, 0xC0]`, `precision: 8`, a fixed three-component 4:2:0 `comps` array
  and exactly four `write_dht` calls, and no DAC anywhere. **It can serialize a conforming baseline
  JPEG and no other kind of JPEG at all**, so every one of its ten conformance kinds is model-only.

Declaring catalogs anyway would have produced two cases whose every scenario reports green while the
mutation never reaches a byte — the exact shape this ticket exists to remove. The vocabularies are
still real and used (analyzer, builder and subset validator all read the same axes on a decoded
snapshot, which is where a conformance verdict lives) and each carries `#[test]`s proving every kind
moves exactly the diagnostic its axis reports and inverts cleanly.

## 6. Honest limits

* **`parity=0/0` still.** Every number below is the ORACLE phase. `semio-framework-job` does not
  compile (`ManuallyDrop<Option<RetainedJobPayload>>` migration, 6 errors, another session's
  in-flight work), so `semio-s-plugin-stdio` does not compile, so no subject ran and no differential
  was made. Verified again this session with `cargo check -p semio-framework-job --lib`.
* **Everything written on the production side is unverified by a compiler.** The subject-half
  adapter fixes, the three `inverse_*_mutation` wrappers, the `encode_png` tRNS fix, the
  `encode_bmp` `biClrUsed` fix and both new baseline vocabularies parse (`rustfmt --edition 2021`
  on each) but cannot be type-checked while the job crate is down. Their `#[test]`s cannot run
  either.
* **The BMP identity round trip now asserts EXACT bytes, not "the bytes moved".** An uncompressed
  BMP v3 leaves a writer no freedom (fixed headers, verbatim palette, padded index rows) and the
  fixture is the reference encoder's own output, so byte-exactness is the correct answer and the
  pass-through tripwire would be meaningless. What rules out a `read`/`write` shortcut on the
  subject side is structural: the only channel is `decode_bmp` → DSL text → `parse_dsl` →
  `encode_bmp`.
* **`set-header` and `set-transparency` (PNG) and the five table/restart kinds (JPG) are exempt from
  the observability law.** Each is justified against the encoder's own source in the oracle module
  AND in the feature description. Nothing else is exempt anywhere in this slice.

---

## 7. Verification

Contract phase, per case, from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` — every one of the ten
reports `0 high-priority breach(es) across 0 rule(s)`:

```
mutate-png-1-2  mutate-bmp-v3  mutate-gif-87a  mutate-gif-89a  mutate-jpg-jfif-1-01
mutate-tiff-6-0  mutate-mp4-isobmff  mutate-mp3-mpeg1-layer3  mutate-avi-1-0  mutate-wav-riff-pcm
```

Oracle phase, per case (`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case <case>`), the
runner's own `[test]` lines verbatim:

```
mutate-jpg-jfif-1-01       [test] level=exhaustive cases=1 executed=25 passed=25 failed=0 errored=0 parity=0/0
mutate-png-1-2             [test] level=exhaustive cases=1 executed=35 passed=35 failed=0 errored=0 parity=0/0
mutate-bmp-v3              [test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0
mutate-gif-87a             [test] level=exhaustive cases=1 executed=25 passed=25 failed=0 errored=0 parity=0/0
mutate-gif-89a             [test] level=exhaustive cases=1 executed=43 passed=43 failed=0 errored=0 parity=0/0
mutate-tiff-6-0            [test] level=exhaustive cases=1 executed=17 passed=17 failed=0 errored=0 parity=0/0
mutate-mp4-isobmff         [test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0
mutate-mp3-mpeg1-layer3    [test] level=exhaustive cases=1 executed=11 passed=11 failed=0 errored=0 parity=0/0
mutate-avi-1-0             [test] level=exhaustive cases=1 executed=27 passed=27 failed=0 errored=0 parity=0/0
mutate-wav-riff-pcm        [test] level=exhaustive cases=1 executed=11 passed=11 failed=0 errored=0 parity=0/0
```

Whole-repository contract, after every edit: `0 high-priority breach(es) across 0 rule(s)`, and
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` read directly holds `0` entries — no non-blocking
breach hiding behind the count.

A whole-owner sweep was also run and its per-case result streams scanned: across every stdio case,
the ONLY failure attributable to this wave was `mutate-gif-89a::mutate-remove-app-extension`, which
the observability law caught and which §4 records. The sweep's own final line is not quotable — it
ended on a compile error in `mutate-pdf-1-7-vt`, a case another session was adding while it ran
(`crate::artifacts::…::obj: not accessible`), unrelated to this slice.

### Failures this wave produced, and what each indicted

| failure | verdict |
|---|---|
| `mutate-set-gamma`, `mutate-set-chromaticities` (PNG) | the REFERENCE library: `png` 0.18.1 never assigns `Info::source_gamma`/`source_chromaticities` |
| `inverse-set-background` (PNG) | this ORACLE: bKGD was captured both typed and verbatim, so clearing the typed copy restored nothing |
| `mutate-set-text-chunk` (PNG) | the FIXTURE arrangement: the arrange step seeded the same content the row then wrote |
| `inverse-set-image-interlace` (GIF87a) | the REFERENCE library: `gif` de-interlaces on read and writes the buffer verbatim |
| `mutate-set-jfif-header`, `mutate-set-re-encode-quality` (JPG) | the COMPARISON PROFILE: a 400 000 per-number slack was swallowing every exact claim |
| `identity-round-trip` (BMP) | the LAW chosen: the fixture is the reference encoder's own output, so exact bytes is the right claim |
| `mutate-remove-app-extension` (GIF89a) | the FIXTURE: its only application extension is NETSCAPE2.0, which is the loop-count axis and not an `appExtensions` entry |
| `mutate-set-sample-sync` (MP4) | the ROW: it named a sample the fixture's `stss` never listed, so it set an already-false flag to false |
