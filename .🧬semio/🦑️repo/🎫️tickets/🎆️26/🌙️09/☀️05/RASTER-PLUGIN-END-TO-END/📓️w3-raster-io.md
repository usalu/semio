# W3 — `✏️s/🔌️plugins/🖨️raster` io layer: made real, reconciled, and cross-language tested

Scope: `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/**`, `🗿️artifacts/🖨️raster/🦀️.rs`'s
`artifact_kind()`, and `📦️packages/🟦️typescript/{package.json,📜️script.ts}`.
All paths relative to `/Users/ueli/Documents/semio`.

This is the SECOND W3 run. The first was killed by the account's Opus session limit at ~01:30 mid-edit
(the compositor region); the machine then rebooted. Everything it had written was auto-committed in
`5e03e56997`. This run verified that work against `3a6a9d6bfc` (the ticket start commit), found the
compositor insert had in fact landed complete, and then finished the parts that were still missing:
the cross-language TypeScript twins + shared fixture oracle, the Rust parity tests, the honest
`derived_composition` pdf branch, and the TypeScript package rewrite.

---

## 1. Format-by-format: before → after

"Before" is the leaf body at `3a6a9d6bfc` (`git show 3a6a9d6bfc:<leaf>`), "after" is the current tree.

### Export (`🚪️io/📤️export/🧵️serializers/🗿️artifacts/<fmt>/…/🦀️.rs`)

| format | before | after | honest? |
|---|---|---|---|
| `🪟️bmp@v3` | `print_dsl(snapshot).into_bytes()` — the raster DSL text under a `.bmp` extension | **REAL** — `raster_composite_image` flattens the layer stack to one RGBA8 canvas → stdio's registered `s.stdio.semio/v1/image` → `s.stdio.bmp` serializer → stdio's own `encode_bmp` (24bpp `BI_RGB`) | real; alpha dropped by the FORMAT (documented in the leaf) |
| `📷️png@1.2` | `print_dsl` text under `.png` | **REAL** — same composite → stdio's `SemioImageToPng` → stdio's `encode_png` | real, lossless both ways (canonical RGBA8, colour type 6) |
| `🎞️gif@87a` | `print_dsl` text under `.gif` | **REAL** — composite → stdio's `SemioImageToGif` **@89a** (its own exact 1:1 quantizer) → `io::gif87a::from_89a` version remap → stdio's own `87a` `encode_gif` | real; 89a-only frame/doc state has no 87a home and is dropped ON PURPOSE (that is what GIF87a *is*) |
| `📸️jpg@jfif-1.01` | `print_dsl` text under `.jpg` | **REAL** — composite → stdio's `SemioImageToJpg` → stdio's `encode_jpg` | real; JPEG's own lossiness + no alpha |
| `🖼️tiff@6.0` | `print_dsl` text under `.tiff` | **REAL** — composite → stdio's `SemioImageToTiff` → stdio's `encode_tiff` (IFD 0, RGBA8 strips) | real |
| `🎨️svg@1.1` | `print_dsl` text under `.svg` | **REAL** — `drawing_snapshot_from_raster` builds a real `SemioDrawingSnapshot` (one `DrawNode::Image` per visible pixel layer, carrying that layer's own asset bytes and transform) → stdio's `s.stdio.semio/v1/drawing` → `s.stdio.svg` serializer → `write_svg_xml` (bare `<svg>…</svg>`, never the `.semio` envelope) | real |
| `📖️pdf@1.4` | `print_dsl` text under `.pdf` | **HONESTLY DECLINED** — typed `Err(RASTER_PDF_EXPORT_UNSUPPORTED)` | this repo's `PdfSnapshot` models a page as `{width, height, text}` — no image XObject, no path-painting operator. Nothing a pixel composite could become. |
| `🖊️dwg@ac1018` | `print_dsl` text under `.dwg` | **HONESTLY DECLINED** — typed `Err(RASTER_DWG_EXPORT_UNSUPPORTED)` | a raster document contributes only `DrawNode::Image` nodes, and stdio's own semio/drawing→dwg serializer documents that it drops those; the DWG would be an empty drawing. |
| `🔣️json@rfc8259` | real (`JsonSnapshot::from_value` + `write_json_pretty`) | unchanged, still real | real, exact |

### Import (`🚪️io/📥️import/🧩️deserializers/🗿️artifacts/<fmt>/…/🦀️.rs`)

| format | before | after | honest? |
|---|---|---|---|
| `🪟️bmp@v3` | `let _ = bytes; Ok(empty_raster_snapshot())` — **silent total data loss** | **REAL** — stdio's `decode_bmp` → stdio's `s.stdio.bmp` → `semio/v1/image` deserializer → one `Pixel` layer with a materialized asset child | real |
| `📷️png@1.2` | `let _ = bytes;` empty snapshot | **REAL** — same shape via `decode_png` | real, lossless |
| `🎞️gif@87a` | `let _ = bytes;` empty snapshot | **REAL** — stdio's `87a` `decode_gif` → `io::gif87a::to_89a` → stdio's `s.stdio.gif@89a` → `semio/image` | real; only the FIRST image of a multi-image GIF reaches the document (a raster document has no frame concept) |
| `📸️jpg@jfif-1.01` | `let _ = bytes;` empty snapshot | **REAL** — `decode_jpg` → hub | real; alpha forced opaque by the codec |
| `🖼️tiff@6.0` | `let _ = bytes;` empty snapshot | **REAL** — `decode_tiff` → hub | real |
| `🎨️svg@1.1` | `let _ = bytes;` empty snapshot | **REAL, host-tiered** — `semio_framework_os::rasterize_svg_to_png_base64` (the framework's own usvg/resvg interface, the only vector rasterizer in the repo), then canonicalized through the real png↔semio/image codec | real natively; inside a `wasm32-wasip2` guest the renderer's own "requires the native semio-framework-os host" error is propagated VERBATIM instead of substituting a blank canvas |
| `📖️pdf@1.4` | `let _ = bytes;` empty snapshot | **HONESTLY DECLINED** — typed `Err(RASTER_PDF_IMPORT_UNSUPPORTED)` | `decode_pdf` yields `{width, height, text}` per page and no pixels |
| `🖊️dwg@ac1018` | real (`dwg_from_bytes` → `raster_document_json_from_dwg`) | unchanged, still real (now via the typed `SemioDrawingSnapshot` bridge rather than hand-formatted SVG strings) | real |
| `🔣️json@rfc8259` | real (`parse_json_text` + `FromValue`) | unchanged, still real | real, exact |

**No leaf prints this artifact's own DSL text under a foreign extension any more.** That is asserted
by two Rust tests: `png_export_writes_a_real_png_signature` (the 8-byte PNG signature) and
`bmp_export_writes_real_bytes_that_import_reads_back` (`BM` magic + a real round trip).

### No external runtime library anywhere on this path (CLAUDE.md hard rule)

Every real hop above bottoms out in an in-repo codec, never a crate:

- `encode_png`/`decode_png` (`🗄️stdio/🗿️artifacts/📷️png/…/🚪️io/🦀️.rs:454`) compress through
  `crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::zlib_compress` — stdio's own
  `🗜️deflate` artifact. The brief's "reuse an existing in-repo deflate, don't duplicate" is satisfied
  by *not writing a PNG encoder here at all*.
- `encode_bmp`/`decode_bmp`, `encode_gif`/`decode_gif`, `encode_jpg`/`decode_jpg`,
  `encode_tiff`/`decode_tiff` are all hand-written stdio artifacts.
- `semio-s-plugin-stdio`'s own `[dependencies]` are in-repo paths only (plus the interim, documented
  `serde`/`serde_json` conversion debt that stdio's manifest already carries).
- Stdio's `🧪️oracle/🖼️raster` external crates were NOT reached at runtime by anything in this work —
  raster's runtime path never names them.

### Why the two refusing hops are still REGISTERED

Same rule `🧱️block`'s W3 recorded: an unregistered `(from, into)` pair yields a bare "no route" at
the router; a registered one that answers with the leaf's own sentence tells the caller *why* the
conversion cannot exist. `io_registry::entries()` therefore keeps all 9 export composers, and
`derived_composition::reads()` keeps all 9 import dialects.

---

## 2. `artifact_kind()` reconciliation (task item 2)

Landed in the first run and verified here.
`🗿️artifacts/🖨️raster/🦀️.rs:651-666` no longer re-lists formats; it READS them from the io module
(🗒️note's convention, `🗒️note/🗿️artifacts/🗒️note/🦀️.rs:101-102`):

```rust
export_stdio_kinds: …::io::export_stdio_kinds().to_vec(),
import_stdio_kinds: …::io::import_stdio_kinds().to_vec(),
```

and the two io functions (`🚪️io/🦀️.rs:9-18`) list only hops that genuinely encode/decode:

- `import_stdio_kinds` = `bmp, dwg, gif, jpg, json, png, svg, tiff` (pdf absent — no decoder)
- `export_stdio_kinds` = `bmp, gif, jpg, json, png, svg, tiff` (pdf **and** dwg absent — no encoder)

This matters because `negotiate_wire_format`
(`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`) picks a workflow wire straight out of these two lists:
anything advertised here without a real codec becomes a workflow edge that always fails at run time.
Before the ticket the three surfaces disagreed three ways (io listed 9, the spec listed 2, the
composer registry registered 9). `export_formats`/`import_formats` stay `vec![]` — the pre-stdio
format-enum peer that 🗒️note, 🧱️block and every stdio artifact also leave empty.
The invariant is now a test: `advertised_stdio_kinds_exclude_every_declined_hop` (`🚪️io/🦀️.rs:812`)
asserts both lists AND their equality with `artifact_kind()`'s own fields.

---

## 3. Registration channel (task item 3) — **NOT migrated, with reason**

The brief asked to put raster on `IoDeclaration.entries` / subset-root `io: io::io()` like
🗒️note/🧱️block. That is **not a leaf-level change** and it was deliberately not attempted in this run.

Findings:

- Raster has **no** `🏅️standards/🔖️1/🦀️.rs` and **no** `🪆️subsets/✳️any/🦀️.rs` at all — the two files
  that would carry `standard() -> StandardDeclaration` and `subset() -> SubsetDeclaration`. Every
  plugin already on the new channel (note, draw, dag, writer, sequence, reasoning, forms,
  mathematical, vcs, sourcing, animate, trinity, block, stdio…) has both.
- Raster is on the OLD channel end to end: `✏️s/🔌️plugins/🖨️raster/🦀️.rs:47`
  `.artifact(crate::artifacts::raster::declaration()…)`, and
  `🗿️artifacts/🖨️raster/🦀️.rs:724-732` `ArtifactDeclaration::builder(definition()?)…
  .composers(…io::io_registry::entries())`.
- **That channel is NOT dead here.** Unlike block (whose `io_registry::entries()` had zero callers
  repo-wide, which is why block's W3 deleted it), raster's `io_registry::entries()` is the argument
  of the live `.composers(…)` call above. Deleting it would unregister every raster hop. So there is
  no dead channel to drop.
- Migrating would mean, in one step: two new declaration files, a `pub fn artifact() ->
  ArtifactDeclaration<crate::RasterApps>`, 18 leaves rewritten as typed
  `Serializer<RasterSnapshot>`/`Deserializer<RasterSnapshot>` impls, new module mounts in
  `📦️packages/🦀️rust/🦀️.rs`, and swapping `.artifact(...)` → `.declare_artifact(...)` in
  `✏️s/🔌️plugins/🖨️raster/🦀️.rs` — a file W2 edited DURING this run (it added
  `editor_with_examples` + `examples()` at 04:5x). Landing that unverified, concurrently, on top of
  W1's and W2's in-flight edits, would have been reckless.

**Recommended follow-up ticket** (everything needed is now in place for it): the 18 leaves are
already pure `snapshot ↔ bytes` functions with real bodies, so each becomes a two-line typed impl;
the only genuinely new artefacts are the two declaration files and the plugin-root swap.

---

## 4. `require_empty_output_shell` — **kept, it is load-bearing**

The brief asked to remove it "if nothing legitimate needs it". It is needed twice over:

- `🧬️schema/📸️snapshot/🦀️.rs:47` defines it; `:261`, `:496`, `:536`, `:546` call it on the DSL-print
  and pack paths, and `🧬️schema/🧬️mutations/💾️binary/🦀️.rs:4913-4914` tests it.
- The repo verify gate `verify interactivity tool-jobs` asserts its exact text
  (`📜️script.ts:4103`, `:4116`, `:4117`, `:8098`).

Only its use *inside the eight binary export leaves* is gone, because those leaves now serialize
real, populated documents — which is precisely what that guard forbids.

### ⚠️ Pre-existing repo-gate breakage this run did NOT touch (needs a decision)

`📜️script.ts:4009-4011` counts, over a concatenation that includes the eight raster export leaves:

```
rasterMountedOutputGuards  === 8   // occurrences of `snapshot.require_empty_output_shell().map_err(str::to_owned)?;`
rasterMountedOutputCallers === 8   // occurrences of `Ok(<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())`
```

Two independent problems:

1. **Already stale before this ticket.** Three of the eight paths in `rasterMountedOutputSerializers`
   (`📜️script.ts:10240-10250`) do not exist on disk — the script says `🖼️bmp`, `🌳️pdf`,
   `📷️jpg/🔖️jfif-1.01/✳️any`; the tree has `🪟️bmp`, `📖️pdf`, `📸️jpg/🔖️jfif-1.01/♾️any`.
   `policyReadFileSafe` returns `""` for those, so the counter could only ever reach **5**, not 8 —
   the gate at `📜️script.ts:10339` was failing before `3a6a9d6bfc`.
2. **The law itself now contradicts the ticket.** It requires every mounted raster exporter to be a
   `print_dsl`-under-a-foreign-extension writer that refuses populated snapshots. Real exporters must
   do the opposite. The gate's expectation, not the exporters, is what has to change.

Deliberately left alone by W3: `📜️script.ts` is the shared, heavily-contended repo root script, the
fix also has to move the matching self-tests (`📜️script.ts:8130-8134`, `:8274-:8277`), and the
correct new law touches W1's tool-job/publication-authority territory.

**Status: picked up and closed by W5 — see `📓️w5-raster-export-gate.md`,** which rewrites the law for
real exporters (and confirms independently that source #3 of the gate's concatenation,
`…/✳️any/🧬️schema/🦀️component.rs`, does not exist either). Nothing further is owed from W3 here.

---

## 5. Changes made in THIS run (file:line)

**Rust**

- `…/✳️any/🚪️io/🦀️.rs:757` — `BMP_PARITY_FIXTURES`, the two `include_str!`-ed shared oracles.
- `…/✳️any/🚪️io/🦀️.rs:759-791` — `parity_fixture` (JSON read via stdio's own `parse_json_text`/
  `JsonValue`, no new dependency), `hex_of`, `bytes_of`, `parity_document`.
- `…/✳️any/🚪️io/🦀️.rs:793-801` — `bmp_export_matches_the_typescript_parity_fixture`.
- `…/✳️any/🚪️io/🦀️.rs:804-813` — `bmp_import_matches_the_typescript_parity_fixture`.
- `…/✳️any/🚪️io/🦀️.rs:928-934` — `derived_composition::compose`'s `DEP_PDF` branch now returns the
  leaf's own `RASTER_PDF_IMPORT_UNSUPPORTED` sentence instead of falling through to the generic
  "no source in a known read dialect" (the pdf leaf can never succeed, so swallowing its `Err`
  answered a pdf source with the wrong reason).

**TypeScript (new twins, not re-exports)**

- `…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️.ts` — was `export {};`. Now
  `RasterCanvas`, `bmpRowBytes`, `rasterCanvasToBmpV3` (a real 24bpp `BI_RGB` bottom-up BMP v3
  writer restating stdio's `encode_bmp` direct path + `SemioImageToBmp`'s pinned header fields),
  `bmpBytesToHex`.
- `…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️.ts` — was `export {};`. Now
  `bmpV3ToRasterCanvas` (a real reader restating stdio's `decode_bmp` 24bpp branch, refusing every
  other depth/compression with the reason) and `bmpHexToBytes`.
- `…/🚪️io/🧪️tests/🟦️.ts` — NEW, 7 bun tests.
- `…/🚪️io/🧪️tests/🧫️fixtures/🪟️solid-3x2.json` — NEW shared oracle (aligned scanline).
- `…/🚪️io/🧪️tests/🧫️fixtures/🌈️gradient-5x3.json` — NEW shared oracle (width 5 ⇒ a 15-byte scanline
  padded to 16, so the row-padding rule is exercised too).
- The other 16 per-format TS leaves stay `export {};` — repo convention
  (`📓️explore-raster-history-and-prior-tickets.md` §3), and there is no second implementation to
  write for a hop that goes through stdio's PNG/GIF/JPEG/TIFF codecs.

**TypeScript package (task item 4)**

- `📦️packages/🟦️typescript/package.json` — was a verbatim `cad-js` copy: a CAD/brepjs/R3F
  description, three `scripts` all pointing at `@semio-tech/cad-js`, and nine `dependencies`
  (`s-3d-js`, `ui-react`, `infinite-world-r3f`, four `cad-js-module-*`, …) that this package does not
  import. Rewritten honestly: raster description, one `test` script pointing at
  `@semio-tech/raster-js`, `"dependencies": {}` (mirrors `🧱️block`'s own package.json).
- `📦️packages/🟦️typescript/📜️script.ts` — removed `console.log("[DEBUG] raster ts ok")`; the case
  list is now a named `CASES` const and includes the new io parity test file.

## 6. Cross-language parity design

`🚪️io/🧪️tests/🧫️fixtures/*.json` is ONE file asserted from BOTH languages:

- **TypeScript** (`bun test …/🚪️io/🧪️tests/🟦️.ts`): `rasterCanvasToBmpV3(fixture.rgba8)` must hex-equal
  `fixture.bmpHex`; `bmpV3ToRasterCanvas(fixture.bmpHex)` must equal `fixture.rgba8`; writer ∘ reader
  must be the identity; and a mutated bit-depth byte must be REFUSED with its reason, not guessed at.
- **Rust** (`🚪️io/🦀️.rs`): `bmp_export_matches_the_typescript_parity_fixture` runs the fixture rgba8
  through `raster_document_from_semio_image` → the real bmp export leaf and asserts the same
  `bmpHex`; `bmp_import_matches_the_typescript_parity_fixture` runs `bmpHex` through the real bmp
  import leaf → `raster_composite_image` and asserts the same `rgba8`.

So the Rust path (composite → stdio `SemioImageToBmp` → stdio `encode_bmp`) and an independent
hand-written second implementation are pinned to identical bytes; a drift fails in both languages.

### Third-party oracle (CLAUDE.md: validate our implementation against a third-party one)

Two implementations that agree with each other could still agree on something that is not a BMP. A
THIRD, fully independent implementation — Apple's ImageIO, driven by macOS `sips(1)` — was asked:

```
$ python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🐍️w3-bmp-third-party-oracle.py"
ok 🪟️solid-3x2.json: Apple ImageIO reads 3x2 and every RGB triple matches the fixture
ok 🌈️gradient-5x3.json: Apple ImageIO reads 5x3 and every RGB triple matches the fixture

$ file solid.bmp grad.bmp
solid.bmp: PC bitmap, Windows 3.x format, 3 x 2 x 24, image size 24, cbSize 78, bits offset 54
grad.bmp:  PC bitmap, Windows 3.x format, 5 x 3 x 24, image size 48, cbSize 102, bits offset 54
```

The script (`🐍️w3-bmp-third-party-oracle.py`, kept in this ticket folder) writes each fixture's
`bmpHex` out as a `.bmp`, has `sips` transcode it to PNG, decodes that PNG with nothing but the
Python standard library, and compares every RGB triple — including row order — against the fixture's
own `rgba8`. It is deliberately NOT wired into `bun test`: `sips` is macOS-only and the bun suite must
stay cross-platform.

## 7. Verbatim output

### `bun test` (TypeScript twins)

```
$ bun test "$(pwd)/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts"
bun test v1.3.14 (0d9b296a)

 7 pass
 0 fail
 15 expect() calls
Ran 7 tests across 1 file. [49.00ms]
```

And the whole rewritten package script (the two example suites plus the new io suite):

```
$ cd "✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript" && bun ./📜️script.ts test
bun test v1.3.14 (0d9b296a)

 9 pass
 0 fail
 17 expect() calls
Ran 9 tests across 3 files. [159.00ms]
```

### `cargo check -p semio-s-plugin-raster --lib`

Run 1 (`CARGO_TARGET_DIR=target-s-e2e RUSTC_WRAPPER="" cargo check -p semio-s-plugin-raster --lib
--message-format short`), tail:

```
✏️s/…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:7:22: warning: unused import: `ToValue`
✏️s/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:7:11: warning: unused import: `FromValue`
warning: `semio-s-plugin-raster` (lib) generated 83 warnings
error: could not compile `semio-s-plugin-raster` (lib) due to 29 previous errors; 83 warnings emitted
```

**Zero of the 29 errors are in `🚪️io/`.** They are, verbatim, the drift slice the coordinator had
already assigned to W4 plus two in W2's/W1's regions: 15× `Label: From<LabelText|&str|
plugin::Label>` in `✏️editor/📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue,🎭️masks}`, 2× E0046
`DESCRIPTORS`/`descriptor` on `✏️editor/{🎚️config,👥️presence}`, 2× `render` must return
`Result<ComponentTree>` (`👁️viewer/🦀️.rs:73`, `✏️editor/🦀️.rs:821`), 4× `ActionDescriptor` vs
`Result<(ActionId, Option<UiValue>)>` in the brush/eraser option windows, 3× `UiNode` vs
`Result<BuiltNode, PluginAssemblyError>` in the viewer windows/panels, 1× `no field base on
BuiltNode`, 2× in `🧬️schema/🧬️mutations/💾️binary/🦀️.rs` (E0061 arity at `:3988`, `?` `Fault`→`String`
at `:3480`), 1× E0428 duplicate `set_active_example` mount in `📦️packages/🦀️rust/🦀️.rs:599`.
(The full list is the coordinator's `🗑️generated/check-native-1-errors.txt`.)

The **83 warnings** matter as evidence: the crate was fully expanded and type-checked, so "no io
error" is a real result and not an early abort.

The two `unused import` warnings above WERE in this region and are fixed (`use dsl::FromValue;` in
the json import leaf, `use dsl::ToValue;` in the json export leaf).

Run 2, after those two fixes and after W4 landed most of the drift slice, filtered to this region:

```
$ … cargo check -p semio-s-plugin-raster --lib --message-format short 2>&1 \
    | grep -E "🚪️io|^error: could not compile|warning: .semio-s-plugin-raster"
warning: `semio-s-plugin-raster` (lib) generated 82 warnings
error: could not compile `semio-s-plugin-raster` (lib) due to 4 previous errors; 84 warnings emitted
```

29 → 4 errors, and **no `🚪️io` line at all** — not one error, not one warning. The 4 remaining are
outside this region (W4's drift slice, still in flight).

### ⚠️ The two new Rust parity tests are WRITTEN BUT NOT YET EXECUTED

`cargo check --lib` does not expand `#[cfg(test)]`, and `cargo test`/`--tests` cannot get past the
lib errors above, which are in other agents' regions. So `bmp_export_matches_the_typescript_parity_
fixture` and `bmp_import_matches_the_typescript_parity_fixture` have **not been run**, and this
report does not claim they pass. What IS verified independently of the Rust compiler:

- the fixtures are genuine BMP v3 files with exactly those pixels (third-party oracle, §6);
- the TypeScript twin reproduces them byte-for-byte (7/7 bun tests);
- the byte shape they assert is derived line-by-line from the two Rust functions actually on the
  path — `SemioImageToBmp::serialize` (`…/🧿️semio/…/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs:29-49`,
  which pins `BottomUp`/`planes 1`/`24bpp`/`BI_RGB`/`x,y_pixels_per_meter` from metadata, defaulted
  to 0) and `encode_bmp_direct` (`…/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs:481-521`).

**Run this the moment the crate compiles:**

```
CARGO_TARGET_DIR=target-s-e2e RUSTC_WRAPPER="" cargo test -p semio-s-plugin-raster --lib parity_fixture
```

## 8. Exact file list touched by W3 (both runs)

Created in this run:

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures/🪟️solid-3x2.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures/🌈️gradient-5x3.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️w3-raster-io.md` (this file)

Updated in this run:

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️.ts`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️.ts`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs` (unused-import warning)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs` (unused-import warning)
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🐍️w3-bmp-third-party-oracle.py` (new, kept as a ticket input file)

Updated by the FIRST W3 run (auto-committed in `5e03e56997`, verified complete here):

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` (+498)
- all 16 per-format Rust leaves under `🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/{🎞️gif/🔖️87a,🎨️svg/🔖️1.1,📖️pdf/🔖️1.4,📷️png/🔖️1.2,📸️jpg/🔖️jfif-1.01,🖊️dwg/🔖️ac1018,🖼️tiff/🔖️6.0,🪟️bmp/🔖️v3}/…/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs` (`artifact_kind()` reconciliation, +16)
- `.🧬semio/🦑️repo/🎫️tickets/…/RASTER-PLUGIN-END-TO-END/🐍️w3-io-leaves.py` (the generator for the 16 structurally identical leaves; kept)

Not touched (other agents' regions): `✏️editor/**` (W1/W2), `🔏️publication-authority/**` (W1),
`📚️examples/**`, `🎬️set-active-example` (W2), `🔮️oracle/🔣️.json` (W2),
`📦️packages/🦀️rust/🦀️.rs` (W2), `✏️s/🔌️plugins/🖨️raster/🦀️.rs` (W2), `📜️script.ts` (see §4).
