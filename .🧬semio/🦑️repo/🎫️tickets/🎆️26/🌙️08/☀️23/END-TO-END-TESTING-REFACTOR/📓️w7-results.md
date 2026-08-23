# Wave 7 results — exhaustive real-world mutation round-trips

Every case below applies **every mutation kind its subset declares** to a **real-world artifact**,
performs it first with the registered third-party reference implementation, and compares the result
through an independent reader. Each kind is exercised twice — applied, and inverted — plus a
full decode/re-encode identity scenario that forbids byte pass-through.

## Verified totals

Every stdio artifact is covered: **38 mutation subsets**, one per standard-and-subset that declares a
vocabulary. Full sweep, run after the last agent landed:

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio          # exit 0
[test] not-exercised …/💾️binary/🧪️tests/mutate-binary-raw (recorded no-oracle decision …)
[test] not-exercised …/📄txt/🧪️tests/mutate-txt-utf-8 (recorded no-oracle decision …)
[test] level=exhaustive cases=47 executed=764 passed=764 failed=0 errored=0 parity=0/0 not-exercised=2

$ bun ./📜️script.ts contract                                    # exit 0
0 high-priority breach(es) across 0 rule(s)

$ cargo test --features oracles --lib                           # oracle crate
test result: ok. 131 passed; 0 failed; 1 ignored
```

`parity=0/0` is not a pass — it is the honest record that no subject ran. See "Honest limits".

Individual cases were verified with
`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case <case>`.

## Green

| Subset | Case | Scenarios | Reference | Real input |
|---|---|---|---|---|
| **pdf 1.7 any** | `mutate-pdf-1-7` | **37/37** | lopdf 0.44 | the real 6.3 MB, 65-page bachelor thesis |
| obj 3.0 any | `mutate-obj-3-0` | 45/45 | tobj 4 | 16,128-triangle mesh derived once from the real committed `🧊️pattern-sphere.glb` (679 KB) |
| gif 87a any | `mutate-gif-87a` | 25/25 | gif 0.13 | real frames of the 4.4 MB `🖼️dancing.gif` animation, rewritten to genuine 87a form |
| jpg jfif-1.01 any | `mutate-jpg-jfif-1-01` | 25/25 | image 0.25 | real 483 KB, 2275×2560, 500 DPI JFIF scan |
| gif 89a any | `mutate-gif-89a` | 43/43 | gif 0.13 | the real 4.4 MB, 800×800, 54-frame animation |
| png 1.2 any | `mutate-png-1-2` | 35/35 | png 0.18 | real 250 KB, 2334×2560, 8-bit indexed floor plan (233-entry PLTE) |
| tiff 6.0 any | `mutate-tiff-6-0` | 17/17 | image 0.25 | 17 MB two-IFD TIFF from the real 500 DPI scan + real downsampled plan |
| bmp v3 any | `mutate-bmp-v3` | 15/15 | image 0.25 | 6 MB real 8-bit indexed BMP from the real palette floor plan |
| zip 2.0 any | `mutate-zip-2-0` | 15/15 | zip 6 | 1.5 MB archive of 20 real architecture photographs |
| stl ascii any | `mutate-stl-ascii` | 15/15 | stl_io 0.8 | 958-triangle real modelled architecture from a real `.glb` |
| csv rfc4180 any | `mutate-csv-rfc4180` | 13/13 | csv 1 | real survey of 50 European reuse marketplaces, 50×12, CRLF |
| deflate rfc1950 any | `mutate-deflate-rfc1950` | 11/11 | flate2 1 | the repository's own README at compression levels 1 and 9 |
| wav riff-pcm any | `mutate-wav-riff-pcm` | 11/11 | hound 3 | real camera-captured luma data, 12 s PCM |
| pdf 1.4 any | `mutate-pdf-1-4` | 5/5 | lopdf 0.44 | the real 6.3 MB, 65-page bachelor thesis |

## Wave 8 — structured text, office and CAD/BIM (in progress)

| Subset | Case | Scenarios | Reference | Real input |
|---|---|---|---|---|
| svg 1.1 any | `mutate-svg-1-1` | 23/23 | quick-xml 0.42 | the real QR-code logo: 664 groups, 329 rects, 5 namespaces, a 74 KB base64 `xlink:href` |
| mp4 isobmff any | `mutate-mp4-isobmff` | 21/21 | mp4 0.14 | 2.7 MB stream-copied excerpt of the real 16 MB video (real encoded samples) |
| ifc 4 any | `mutate-ifc-4` | 23/23 | ruststep 0.4 (reader only) | the real 2.5 MB IfcOpenShell Nakagin Capsule Tower export, 24,792 entities |
| step ap214 any | `mutate-step-ap214` | 23/23 | ruststep 0.4 (reader only) | a real 78 KB BIM export, relabelled to AP214 — see caveat below |
| xml 1.0 any | `mutate-xml-1-0` | 17/17 | quick-xml 0.42 | `word/document.xml` extracted from the real committed ECMA-376 DOCX |
| dxf r12 any | `mutate-dxf-r12` | 39/39 | dxf 0.6 | real R12 tables from a real AC1015 export; geometry is representative — see caveat |
| html 5 any | `mutate-html-5` | 21/21 | html5ever 0.39 | the real 149 KB TYPO3 presentation page |
| xlsx ecma-376 any | `mutate-xlsx-ecma-376` | 21/21 | calamine 0.36 + rust_xlsxwriter 0.96 | real 2-sheet workbook, 229-entry shared-string table, from the real marketplace survey |
| ply 1.0 any | `mutate-ply-1-0` | 21/21 | ply-rs 0.1.3 | 874 KB real mesh from the committed `🧊️pattern-sphere.glb`: 8,449 vertices, 16,128 faces, 50 edges |
| tsv iana any | `mutate-tsv-iana` | 15/15 | csv 1 (tab delimiter) | the real reuse-marketplace survey, 51 × 12 |
| md commonmark any | `mutate-md-commonmark` | 13/13 | comrak 0.54 | the repository's own 47 KB README |
| json rfc8259 any | `mutate-json-rfc8259` | reworking | json-rust 0.12 | the real 424 KB CAD model |

### Evidence typed per mutation kind, not per case

The XLSX case is the sharpest example of the read-only discipline being applied at the right
granularity. `calamine`'s shared-string table is private API and `rust_xlsxwriter` can only create
SST entries as a byproduct of writing a cell, so neither crate can address the pool by raw index.
Rather than claim uniform differential coverage, 7 of the 10 kinds are typed `@mode-differential`
(a genuine read → mutate → rebuild second producer) and the 3 shared-string kinds `@mode-round-trip`,
with the oracle returning bytes unchanged and the comparison carried by tracked count arithmetic.

A `@mode-differential` tag in this repository now means a second producer genuinely existed for that
specific mutation — not for the format in general.

### Caveat on the DXF fixture — the tables are real, the geometry is not

`temp/simple_bus_shelter-gray_3D.dxf` is a genuine 445 KB AC1015 (R2000) export, but its whole
`ENTITIES` section is a single `3DSOLID` carrying AutoCAD's proprietary text-obfuscated ACIS body.
The `dxf` 0.6 reference crate exposes that only as opaque `custom_data: Vec<String>`, nothing in this
repository can decode it, and `3DSOLID` does not exist in the R12 specification at all — so there is
no real R12-representable geometry in the file to down-convert.

The committed fixture therefore carries forward what genuinely IS real from that export — the
`LAYER`, `STYLE`, `LTYPE` and block-record tables — and substitutes representative 2D geometry for
the entities. Both the derived fixture and the real AC1015 source are committed, so the substitution
is auditable.

This is the weakest real-world claim in the whole effort and should be read as such: the DXF case
exercises 19 mutation kinds against real table structures and stand-in geometry, not against a real
drawing. A genuine R12 export would fix it.

### Caveat on the STEP fixture — it is relabelled, not native

No git-tracked real-world **AP214** file exists in this repository. Every real STEP file present,
including the one this ticket nominated, declares
`FILE_SCHEMA AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF` — AP242. The committed fixture is a
real 78 KB BIM export whose `FILE_SCHEMA` line was changed to `AUTOMOTIVE_DESIGN`; its 1,396-entity
graph is untouched real data and contains no AP242-only PMI or GD&T entities, which is what makes
the relabelling defensible.

It is still a relabelled file, not a native AP214 export, and should not be read as one. Sourcing a
genuine AP214 export would strengthen this case.

### The JSON oracle had to be replaced

`serde_json` was registered first and the purity gate fired. It was made green with a
`productionDebt` record enumerating **423** reachable-from paths, growing that manifest to 86 KB.
That is silencing a gate rather than recording debt, and the gate was correct: the JSON snapshot
declares `impl From<serde_json::Value> for JsonValue` in production code at
`.../🧬️schema/📸️snapshot/🦀️component.rs:46`, so the differential would compare our implementation
against something it already converts from.

The reference is now **json-rust 0.12**, which appears nowhere in production and is genuinely
independent. The debt record is deleted, not relocated.

The general rule this establishes: **an oracle that is already production-reachable yields a test
that can only pass.** Three registrations tripped the check this session — `image` and `png` were
pre-existing debt correctly recorded against a small, named set of files, `serde_json` was an
invalid choice and was replaced.

## The page operations the brief asked for

`RemovePage` existed; `MovePage`, `SetPageContent` and `SetPageRotation` did not. They were added to
`PdfMutation` with real `diff()`/`inverse()` arms rather than degrading to `SetSnapshot` — the
append-only `AppendPageContent` had no counterpart, so its inverse could only restore the whole
document. Verified on the real 65-page thesis by reading the raw projections back out of the results
cache:

- `remove-page` drops pageCount 65 → 64; its inverse restores 65.
- `move-page` relocates page 10's exact media box and text to index 40; its inverse restores the
  original order across all 65 pages.
- `set-page-rotation` sets page 5's `/Rotate` to 90; its inverse restores 0.
- `insert-page` and `set-page-content` carry the exact specified text.

## Defects found in the REFERENCE libraries themselves

The GIF 89a case failed 22 of 43 scenarios on first run. The signal was `inverse-no-mutation`: a
no-op followed by its own inverse must trivially recover the original, so the fault could not be in
any inverse — the decode/re-encode round trip itself was not projection-stable. The `mutate-*`
scenarios passed only because both sides went through the same re-encode and the instability
cancelled; the inverse and identity scenarios compare against the ORIGINAL, where it did not.

Four real defects in reference libraries came out of this effort — two in `gif` 0.13, one in
`ply-rs` 0.1.3, one in `ruststep` 0.4, each reproduced standalone
before being worked around, and each documented in the oracle module rather than hidden by loosening
the projection:

1. `gif::Encoder::new` unconditionally sets the global-colour-table flag and writes a minimum
   two-entry padding table even when the palette is empty, with no way to omit it through the public
   API. The phantom table is stripped from the output when the snapshot declares no GCT.
2. **`ply-rs` 0.1.3 writes the wrong list-length prefix in binary mode.**
   `Writer::__write_binary_element`'s `PropertyType::List` arm emits `element_def.count` — the
   element's total row count — instead of the current row's own list length. Writing the real
   16,128-face fixture as `binary_little_endian` and reading it back with the SAME crate fails
   (`"Couldn't find a list element at index 114"`; 16128 truncates to 0 as a `uchar`). Found by
   reading the crate's source and reproducing it in a standalone probe, then worked around with a
   hand-written binary payload encoder that reuses the crate's correct header writer — every ASCII
   write and every read still goes through `ply-rs` unmodified.
3. **`ruststep` 0.4 never implements the STEP doubled-apostrophe escape.** ISO 10303-21 encodes a
   literal `'` inside a string as `''`, and real IfcOpenShell output uses it — entity #17012 of the
   real Nakagin export carries an embedded-JSON property value that depends on it. The obvious
   text-level fix was tried and *proven wrong* before being discarded: it corrupts legitimately empty
   header fields such as `FILE_NAME`'s `('')` entries. The working fix is a string-delimiter-aware
   single-pass scanner, verified against the real 24,792-entity fixture in a standalone probe.
4. `gif::Decoder` always de-interlaces on read and resets `interlaced` to `false`, while
   `gif::Encoder` writes the buffer verbatim and only flips the flag bit. A mutation that flips
   `interlaced` must therefore reorder the buffer into GIF's four-pass storage order itself, or the
   flag and the data disagree.

The resolution explicitly rejected was the easy one: comparing a re-encoded original against a
re-encoded original would have turned all 43 scenarios green while comparing the implementation with
itself — the exact failure mode this platform exists to prevent.

## Subject-side defects the oracles already expose

The oracle phase alone — before any differential comparison can run — has already found two real
gaps in this repository's own encoders. Both are recorded rather than worked around, and both will
fail their differential scenario the moment the subject phase compiles. That is the tests doing
their job.

- **`encode_tiff` is single-IFD only.** Its own `EncodeScopeNote` documents that it silently drops
  every IFD beyond the first. The real fixture is deliberately two-IFD, so `mutate-insert-ifd` and
  `mutate-remove-ifd` will legitimately fail against the subject. Pre-existing, not introduced here.
- **The shared XML codec's `xml_escape_attr` does not re-escape tab, newline or carriage return**
  when re-emitting an attribute value it decoded from a `&#10;`-style character reference, while
  `quick-xml`'s writer does. Found by the SVG case against a real 7,301-character `xlink:href`.
- **`decode_avi` rejects real ffmpeg output.** It requires a 64-byte `strh`, but ffmpeg writes the
  classic 56-byte `AVISTREAMHEADER` with `rcFrame` omitted — so production cannot read the real
  fixture at all. A synthetic 64-byte fixture would never have shown this.
- **`AviSnapshot` has no slot for nested `vprp`/`JUNK` chunks** inside `hdrl`/`strl`; both codecs
  silently drop about 4.4 KB of the real file's data on decode. A schema-completeness gap rather
  than a coding bug, and again only visible against a real recording.
- **`encode_bmp` always emits 24-bit BI_RGB and discards the palette** regardless of what the
  snapshot holds. Palette mutations therefore re-encode pixel-identical content on both sides — a
  faithful agreement about a lossy encoder, documented in the oracle module, not a fabricated pass.

## Findings the wave produced

**PDF 1.4 is an unfinished stub, not a reduced profile.** Its `decode_pdf` hardcodes
`width=612 / height=792` for every input and never builds an object graph. Against the real thesis it
silently discards 64 of 65 pages and the true A4 geometry (595.276 × 841.89) — no error, the document
is simply thrown away. The 2-kind catalog is therefore honest about what the subset *is*, and the
subset itself is what needs work. Detail in `📓️w7-pdf-1-4-mutate-report.md`.

**The `💃️dancing` example is a GIF89a filed under the 87a example directory.** The existing example
code already works around it by decoding with the 89a codec. A genuine 87a fixture was derived from
its real frames rather than moving the mis-filed asset.

**Two defects in the wave-0 gate, both found by executors and fixed:**
- The unclaimed-catalog check answered a repository-wide question over the caller's narrowed
  selection, so `--case X` — which is what every generated Nx target runs — reported every other
  catalog as unclaimed. It now derives the claimed set from a full discovery.
- The oracle-purity scan derived test-ownership from parsed manifests, so a contribution directory
  became production source whenever its JSON was absent or mid-write, and an owner adding an oracle
  saw its own reference libraries reported as a production dependency. Ownership now follows the
  taxonomy's contribution directory name, which is what actually defines it.

**Per-entry production debt.** The `image` crate was already production-reachable from
`✏️s/🔌️plugins/🎞️animate/…/🎥️video/🦀️component.rs` and
`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` before it was ever an oracle. Debt is
recorded per registry entry, so each new per-subset registration of the same package carries its own
`productionDebt` record. The finding stays visible rather than being exempted.

## The largest finding: `serde_json` is a production runtime dependency

Registering `serde_json` as the JSON oracle did not create a problem — it made an existing one
visible to a gate that had no reason to look before. Measured directly:

- **94** production `Cargo.toml` files declare `serde_json`.
- **1,503** production `.rs` files reference `serde_json::`.
- The public API leaks the external type outright:
  `impl From<serde_json::Value> for JsonValue` at
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:46`.

This sits against two standing rules: runtime dependencies on external libraries are forbidden, and
exported API must not directly or indirectly require a type from outside this codebase — which a
`From<serde_json::Value>` impl on a public snapshot type does by definition.

It is recorded as `productionDebt` on the JSON oracle entry with the full reachable-from list and a
remediation plan, not silenced. Fixing it is a repository-wide change well outside this ticket, and
it is the user's call whether to open one. The mechanical part — the public-API leak in that single
snapshot file — is small and separable from the 1,503-file usage question.

## A case that executed nothing reported success

Found by the 💾️binary agent. `oracleDecision` returns no implementation for a feature carrying a
recorded `@no-oracle-` decision — correctly, since there is no oracle to run — so the oracle phase
executed zero scenarios for that case. The run then printed
`executed=0 passed=0 failed=0` and exited 0, which is indistinguishable from a case that passed.

That is the same failure this platform exists to prevent, one level up: absence of evidence reading
as evidence. The contract phase already refuses a feature with no scenarios; nothing refused a RUN
that exercised none.

Fixed in the runner: any selected case that produced no result is now reported explicitly, with the
reason, and the summary line carries a `not-exercised=N` count.

```
[test] not-exercised …/💾️binary/🧪️tests/mutate-binary-raw (recorded no-oracle decision
       raw-buffer-no-format — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
```

It reports rather than fails, because an empty oracle phase for a no-oracle case is legitimate. What
is not legitimate is staying silent about it.

**Consequence for this effort's totals:** the no-oracle cases (💾️binary, 📄txt) contribute **zero
executed scenarios today**. Their evidence is written and waiting on the subject phase, which the
os-kernel refactor still blocks. They must not be counted as green.

## The in-crate unit tests were never compiling

Agents added `#[cfg(test)]` unit tests beside their oracle dispatchers, and several reported them
passing — but those runs were in standalone scratch crates. In the real oracle crate the whole test
target failed to build, so **none of them had ever run**.

The cause: seven agents copied `#[semio_framework_async_macros::async_test]` from surrounding
production code into a standalone crate that does not depend on that macro, and the oracle code is
synchronous anyway. Replacing it with plain `#[test]` (and dropping the `async`/`.await` that came
with it) makes the target build.

With the tests actually running: **83 pass, 11 fail.** The failures are real and were invisible until
now. The first triaged one is instructive — the JSON oracle asserts its projection is insensitive to
object member order, but the projection preserves order; what actually delivers order-insensitivity
is the `ordered-json-v1` comparison profile at compare time. The scenario run passes legitimately
because of the profile; the unit test's stronger claim about the projection is simply false.

Two lessons worth keeping:
- A test that cannot compile is indistinguishable from a test that passes, unless something forces
  the target to build. The oracle crate's test target now needs to stay green.
- "Verified in a scratch crate" is not verification of the committed code.

The `KINDS`-versus-enum conformance tests live in the artifact's own production crate, which is
blocked by the os-kernel refactor, so they still cannot run. In their place the catalogs were audited
directly from this ticket folder: **all 30 catalogs match their `KINDS` const exactly and are
complete against their enum's variant count.** That audit is coordinator tooling and deliberately
not part of the framework, which must never parse implementation source.

## Honest limits

- The Rust **subject** phase does not compile this wave: a concurrent session is mid-refactor in the
  os-kernel (`📡️spr/🧵️channel` carries a `semio_framework::` cycle). Every case's subject half is
  written and `sut`-gated, so it compiles into the subject role the moment that lands, and the oracle
  phase is unaffected. No case claims subject or parity results.
- **The `inverse-<kind>` scenarios are weaker than they look while the subject is blocked.** The
  expected value of `apply(inverse(m), apply(m, base))` is `base` by the law itself, so the oracle's
  answer for those scenarios is the original document's projection. That is correct, but it means the
  oracle side asserts the law's expected value rather than independently performing an inverse. Their
  real force arrives with the subject phase, which is where the actual inverse implementation runs.
  The `mutate-<kind>` scenarios do not have this property: there the reference library genuinely
  performs the mutation on the real artifact.
- Several formats had no real-world file in the repository. Those fixtures were **derived once** from
  real committed assets through the reference libraries, never synthesised; each feature description
  records the exact source and derivation, and the derivation scripts are in this ticket folder.
