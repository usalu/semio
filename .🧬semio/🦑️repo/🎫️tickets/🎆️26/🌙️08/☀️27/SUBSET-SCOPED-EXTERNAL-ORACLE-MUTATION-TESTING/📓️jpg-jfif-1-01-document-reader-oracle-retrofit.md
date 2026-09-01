# 📓️ JPG (jfif-1.01/document) reader-oracle retrofit

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document` only. Mirrors the
avi `riff-avi-1-0-mutate-reader` reference exactly (five-artefact shape). The existing
`image-jpeg-jfif-1-01-mutate` (`cross-semio-implementation`) oracle and its `🧪️oracle/🦀️component.rs` were
**not** touched — read only, as a spec reference for what each mutation's byte-level effect is.

## What was built

1. `🏭️generator/🦀️jpeg-jfif-codec/` — standalone Cargo crate, own `[workspace]`, one dependency:
   `image = { version = "0.25.10", default-features = false, features = ["jpeg"] }`, resolved fully
   offline against the local cargo registry cache (confirmed exact patch version `0.25.10` /
   `zune-jpeg 0.5.15` via `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.lock`). Two
   subcommands: `build <recipe-id> <out-dir>` and `project <path>`. `cargo test --offline`: **5/5
   pass**. `cargo build --offline` / `cargo run --offline`: clean, no warnings.
2. `🏭️generator/📜️script.ts` — `generate [--only <id>]` / `manifests [--only <id>]`, mirrors avi's CLI
   shape verbatim.
3. `🔬️probes/📜️script.ts` — `jpg-import` / `jpg-project` / `jpg-compare`, marshal-only, no JPEG
   semantics computed locally.
4. `🧫️fixtures/<recipe>/{before,after}.jpg` — **10/10 recipes**, one per declared mutation kind, all
   `applied` outcome (this catalog declares no `rejected` outcomes), generated via
   `bun 🏭️generator/📜️script.ts generate`.
5. `🧪️oracle/🔣️.json` edited in place: new oracle entry `image-jpeg-jfif-1-01-mutate-reader`
   (`kind: third-party-library`), new `probes` (3) and `comparisonPipelines` (1) arrays, each of the
   10 mutations' `oracleRequirements` patched per the witnessable/uncarried split below, and 10
   `fixtureManifests` entries appended. The pre-existing `image-jpeg-jfif-1-01-mutate` oracle object
   is byte-for-byte unchanged.

## The witnessable / uncarried split — 4 / 6

Investigated for real against the vendored crate source
(`~/.cargo/registry/src/…/image-0.25.10` and `zune-jpeg-0.5.15`), the same rigor the sibling PNG
session used (`📓️png-reader-witnessability.md`), not assumed:

`image::codecs::jpeg::JpegDecoder`'s public `ImageDecoder` impl (`src/codecs/jpeg/decoder.rs`) exposes
only `dimensions`, `color_type`, `icc_profile`, `exif_metadata`, `xmp_metadata`, `iptc_metadata`,
`orientation`, and the decoded raster via `read_image`. Nothing else is public.

- **No DQT/DHT/DRI accessor exists anywhere** in `image` or `zune-jpeg`'s public API —
  `grep -n "pub fn" zune-jpeg-0.5.15/src/decoder.rs` lists nine methods total, none of them
  quantization tables, Huffman tables, or the restart interval.
- `zune_jpeg::decoder::ImageInfo` *declares* `pub x_density: u16` / `pub y_density: u16`, documented
  "Found in the APP(0) marker" — but the setters `set_x`/`set_y` that would populate them from a real
  APP0 segment are **never called anywhere in the crate**
  (`grep -rn "set_x(\|set_y("` finds only the dead `pub(crate) fn` definitions, zero call sites). They
  always read back their `#[derive(Default)]` zero. `ImageInfo.pixel_density` is a different,
  misleadingly-named field — set from the SOF sample-precision byte
  (`headers.rs::parse_start_of_frame`, `img.info.set_density(dt_precision)`), not from the APP0
  density-unit byte. There is no working JFIF-header read path in this crate version. Verified by
  building `change-jfif-header-applied`'s recipe and diffing the raw bytes (`cmp` reports byte 14
  onward differ — a real 300×300 DPI / Inches density write via the encoder's own
  `set_pixel_density`) while `project`'s own JSON shows no field capturing it at all.
- `exif`/`xmp`/`iptc` **are** real, populated reads: `zune-jpeg-0.5.15/src/headers.rs::parse_app1`
  recognizes the literal prefixes `b"Exif\x00\x00"` and `b"http://ns.adobe.com/xap/1.0/\0"`;
  `parse_app13` recognizes `b"Photoshop 3.0\0"`.
- No generic JPEG segment/marker crate (e.g. `img-parts`) is vendored in the local registry cache —
  `find ~/.cargo/registry/src -iname "*jfif*" -o -iname "*img-parts*"` returns nothing — so composing
  a second, independent crate for JFIF-header visibility (option 1 in the brief) was not available
  offline, and hand-writing a segment scanner in this repo's own code would not be a *third-party*
  reader. `change-jfif-header` is therefore registered uncarried rather than improvised.

**Witnessable (4)** — `oracleRequirements[0].oracle = "image-jpeg-jfif-1-01-mutate-reader"` added,
capability left at `jpg-jfif-1-01-mutate`:

| kind | recipe shape | what the reader actually measured |
| --- | --- | --- |
| `insert-other-segment` | before: no XMP; after: real APP1 XMP spliced in (the one segment `image`'s public `xmp_metadata()` can see) | `xmp.present false → true`, `xmp.size`/`xmp.digest` populated |
| `remove-other-segment` | inverse of the above | same fields, `true → false` |
| `replace-pixels` | gradient/checkerboard raster → uniform mid-grey fill | `raster.digest` changes (`fnv1a64:ad423320774c52b5` → `fnv1a64:5ef8bb0c81d85725`) |
| `change-re-encode-quality` | same textured raster, quality 90 → quality 20 | `raster.digest` changes (`fnv1a64:ad423320774c52b5` → `fnv1a64:429b04e5967df071`) |

**Uncarried (6)** — `oracleRequirements[0].capability` renamed to `jpg-jfif-1-01-mutate-uncarried`,
`oracle` field dropped, `qualifyingKind` left as-is (exact shape copied from
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧪️oracle/🔣️.json`'s
`semio-v1-cad-mutate-uncarried` entries):

| kind | reason |
| --- | --- |
| `replace-quant-table`, `remove-quant-table`, `replace-huffman-table`, `remove-huffman-table`, `change-restart-interval` | **Carrier-uncarried, doubly confirmed.** This subset's own production encoder (`../🚪️io/🦀️component.rs::encode_jpg`) regenerates fresh Annex K DQT/DHT tables scaled by `re_encode_quality` on every write and never emits a DRI marker — independently confirmed by reading that file (`grep -n "re_encode_quality\|0xDB\|0xC4\|0xDD"`), matching what the reclassified oracle's own `🧪️oracle/🦀️component.rs` module docstring already documents. No encoder in this repository can carry these five into the bytes at all, so a reader has nothing to witness regardless of capability. Recipes are hand-authored `before == after` byte-identical — the literal truth, not a shortcut (asserted by a codec unit test). |
| `change-jfif-header` | **Reader-uncarried only.** `image`'s own encoder *can* write a real density difference (`set_pixel_density`, a genuine public API — used to build this kind's own fixture), so the bytes genuinely differ; this reader has no decode-side way to read it back (see investigation above). |

## Fixture verify / reproduce — real numbers

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture verify --artifact s.stdio.jpg --standard jfif-1.01 --subset document
[fixture verify] 10 fixture(s), 0 file problem(s)
```

`fixture reproduce`, run **per mutation kind** (10 separate invocations, never a single whole-corpus
double-run — reproducibility.md's own lesson about order-dependent state):

```
change-jfif-header       → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
replace-quant-table       → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
remove-quant-table         → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
replace-huffman-table      → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
remove-huffman-table       → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
change-restart-interval    → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
insert-other-segment       → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
remove-other-segment       → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
replace-pixels             → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
change-re-encode-quality   → [fixture reproduce] 1 generated fixture(s), 0 problem(s)
```

**10/10 fixture verify pass, 10/10 fixture reproduce pass (0 problems each).**

## Gate qualification — both directions, real numbers

`jpg-compare` on `insert-other-segment-applied/before.jpg` against itself (byte-identical pair):

```json
{"equal": true, "diffCount": 0, "diffs": []}
```

`jpg-compare` on the same recipe's `before.jpg` vs `after.jpg` (one real, deliberately-spliced APP1 XMP
segment):

```json
{"equal": false, "diffCount": 3, "diffs": [
  "$.xmp.present: false ≠ true",
  "$.xmp.size: undefined ≠ 223",
  "$.xmp.digest: undefined ≠ \"fnv1a64:2b59c96b89b2e2da\""
]}
```

Gate **accepts** the known-good pair and **rejects** the known-bad pair, naming the exact field paths.

`jpg-import` and `jpg-project` were also run against real committed fixtures (see qualification
evidence embedded in `🧪️oracle/🔣️.json`'s `probes` entries) — both `qualified`.

## `productionReachable` — corrected, with evidence

The new oracle entry is registered `productionReachable: true` (not copied blindly from the sibling —
independently verified). `image` is a real, non-optional, non-dev dependency of at least five
production crates (`✏️s/🔌️plugins/🎞️animate`, `✏️s/🔌️plugins/🖍️draw`,
`🧰️framework/🔨️modules/🗺️surface`, `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite`, the os wgpu
renderer target), and genuinely reached from real (non-test) code paths — confirmed by reading the two
paths the sibling `image-jpeg-jfif-1-01-mutate` entry's own `productionDebt.reachableFrom` names
directly:

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/…/⚙️engine/🎥️video/🦀️component.rs`, `writer` module —
  `use image::{ImageBuffer, Rgba};`, a real (non-`#[cfg(test)]`) partial-movie-writer path.
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs`, `upload_tile` — a real (non-test)
  method calling `image::load_from_memory`.

The repository-wide `🔒️dependencies.json` registry entry for `image` currently still reads
`"productionReachable": false` (line ~2481) despite these `users` entries being non-optional/non-dev in
their own Cargo.tomls — this is stale, independently re-confirmed from an unrelated peer session's
finding for the PNG retrofit. Not fixed here (shared file, out of this ticket's scope); recorded as a
`productionDebt` block on the new oracle entry, mirroring the sibling entry's own shape.

## `contract` / `matrix` — real output, not fabricated

`bun 🧰️framework/…/🧪️test/📜️script.ts contract --owner 🗄️stdio` (repository-wide by nature; `--owner`
narrows what it reports but many breach categories are still repo-global) exits **1**, but every
jpg-specific line is either pre-existing (unrelated to this session) or an already-accepted repo-wide
pattern, confirmed by comparison:

- `s.stdio.jpg@jfif-1.01/document: No runtime inventory has been produced` — expected: the stdio plugin
  does not compile (documented, out of scope).
- Six `mutation <X> requires a third-party-library for capability jpg-jfif-1-01-mutate-uncarried, and
  none is registered` lines, one per uncarried kind. **This is the established, accepted shape of the
  `-uncarried` convention itself**, not a defect: the CAD subset's own pre-existing
  `semio-v1-cad-mutate-uncarried` entries (`set-entity-layer`, `set-block-entity-layer`) produce the
  *identical* breach message today, unrelated to this session.
- `image-jpeg-jfif-1-01-mutate-reader is registered as a qualifying third-party oracle, but this owner
  predicts mutation output in its own Rust` — a heuristic false positive shared by **every single
  `-reader` oracle already registered in this repository**, checked directly: `riff-avi-1-0-mutate-reader`
  (the exact reference this ticket mirrors), `png-png-1-2-mutate-reader`, `image-bmp-3-mutate-reader`,
  `three-gltf-2-0-mutate-reader`, `tobj-obj-3-0-mutate-reader`, `jszip-bcf-2-1-mutate-reader`,
  `jszip-docx-ecma-376-mutate-reader`, `quick-xml-1-0-mutate-reader`,
  `quick-xml-svg-1-1-mutate-reader`, `gif-89a-any-mutate-reader` **all** trigger this exact message
  (full list in `🗑️temp/jpg-jfif-1-01-document/jpg-contract-breaches-excerpt.txt`). The checker flags
  any owner directory that contains a mutation-predicting `component.rs` *anywhere*, regardless of
  which specific oracle entry is being evaluated — it cannot distinguish the reader entry (which calls
  none of that code) from the cross-semio entry (which is that code). Not something this ticket's scope
  can fix without touching the checker itself.
- Two unrelated pre-existing breaches on lines about `@mutations-jpg-jfif-1-01-any` /
  `image-jpeg-jfif-1-01-mutate does not declare capability jpg-jfif-1-01-mutate` — these concern the
  `✳️any`/`✳️baseline` JPG subsets and an already-registered `.feature` file, not `✳️document`, and
  predate this session (the cross-semio oracle's own capability list and an unrelated subset's mutation
  catalog, neither edited here).
- The remainder (mesh `manifold3d-three`, discovery baseline overages, `png-png-1-2-mutate-reader`
  production-debt note) belong to other subsets/sessions entirely.

`bun 🧰️framework/…/🧪️test/📜️script.ts matrix --artifact s.stdio.jpg --standard jfif-1.01 --subset
document` ran (repo-wide report, selectors narrow individual rows rather than the whole report). Real,
relevant lines:

- "Which mutations have no external oracle?" lists exactly the six uncarried kinds —
  `s.stdio.jpg::change-jfif-header, change-restart-interval, remove-huffman-table, remove-quant-table,
  replace-huffman-table, replace-quant-table` — and **not** the four witnessable ones, confirming the
  registration is read correctly.
- "Which fixtures are not reproducible?" → `none`. "Which fixtures lack provenance?" → `none`.
- `s.stdio.jpg@jfif-1.01/document` appears under "no runtime inventory" (stdio doesn't compile) and
  "no real-world fixture" (a separate, pre-existing coverage axis about a photographic corpus fixture,
  unrelated to this ticket's third-party-generated fixtures).

Neither command was fabricated as passing; both genuinely fail for reasons explained above, none of
which are defects introduced by this retrofit.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🏭️generator/🦀️jpeg-jfif-codec/{Cargo.toml,src/main.rs}` — new
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🏭️generator/📜️script.ts` — new
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🔬️probes/📜️script.ts` — new
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧫️fixtures/**` — new (10 recipes × 2 files)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧪️oracle/🔣️.json` — edited (new oracle + probes + comparisonPipelines entries added, 10 `oracleRequirements` patched, 10 `fixtureManifests` appended; existing `image-jpeg-jfif-1-01-mutate` object untouched)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🗑️temp/jpg-jfif-1-01-document/{patch_oracle.py,jpg-fixture-manifests.json,jpg-contract-breaches-excerpt.txt}` — new scratch/evidence

## Not done / left honest

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🧪️tests/mutate-jpg-jfif-1-01/🥒️.feature` was **not** edited
  to consume the new `jpg-jfif-1-01-image-compare-v1` pipeline. Retargeting the existing
  `semantic-jpg-mutate-v1` profile's (currently absent) `pipeline` field would have silently redirected
  that scenario's live comparison mechanism away from its current tolerance-based JSON diff — outside
  this ticket's declared five-artefact scope, and risking exactly the kind of side effect the "leave
  `image-jpeg-jfif-1-01-mutate` untouched" instruction was guarding against. The new pipeline is fully
  registered and independently qualified, ready for whoever wires a scenario to it.
- The repo-wide `🔒️dependencies.json` `image` entry's stale `productionReachable: false` is not
  corrected (shared file, explicitly out of scope) — only this oracle's own field.
