# 🔀️ "Our encoder omits the field" does not block ORACLE coverage

This ticket twice concluded that ~26 kinds were blocked because *our own encoder discards the field
before any reader could witness it*, and filed them as export-correctness work waiting on a peer's
refactor. **That reasoning conflated two different things**, and separating them closed 8 kinds
immediately with no exporter change at all.

## The conflation

A mutation kind needs two independent things:

| | What it needs | Which dimension it belongs to |
|---|---|---|
| **Fixture** | a `before`/`after` pair, written by a third party | `externalOracleCoverage` |
| **Subject** | our own code able to PRODUCE that after-state | `runtimeMutationCoverage` |

Our encoder's inability to emit a field blocks the **second**. It says nothing about whether a
third-party library can write the pair and another can judge it. Those are separate dimensions in this
repository's own matrix, and they were being reported as one.

## What separating them yielded

| Subset | Kinds | Writer | Reader | Why the first reader could not |
|---|---|---|---|---|
| `png@1.2` | 3 | Pillow `PngInfo.add` | Pillow `ChunkStream` (CRC-checked) | `png` 0.18 has no `tIME` field and skips unrecognised ancillary chunks |
| `jpg@jfif-1.01` | 2 | Pillow (`quality`, `dpi=`) | Pillow (`im.quantization`, `im.info['jfif_*']`) | `image`-rs decodes to PIXELS; quantisation tables and the JFIF APP0 segment are consumed on the way |
| `obj@3.0` | 3 | handcrafted OBJ text | three `OBJLoader` | `tobj` is a MESH reader and discards `mtllib`, `usemtl` and smoothing statements |

**+8, none of which needed `semio-s-plugin-stdio` to compile.**

## Every claim was measured, and the measurement set the scope

No kind was registered because it looked plausible. Each candidate was run through the proposed reader
one at a time, and the ones that did not move its projection were left `-uncarried`:

* `obj` — `set-mtllib`, `set-usemtl`, `set-smoothing-groups` move it. The six vertex/texcoord/normal
  kinds do **not** (an unreferenced element is dropped by three's loader too) and
  `set-unknown-statements` does not (OBJLoader skips an unrecognised line with a warning).
* `jpg` — `replace-quant-table` and `change-jfif-header` move it. `change-restart-interval` does
  **not** (Pillow does not read the DRI segment back), and the Huffman accessors return empty and are
  deprecated for removal in Pillow 12.
* `tiff` — checked and still blocked, but the reason is now sharper than "encoder discards it":
  `tiff` 0.11's decoder **does** expose `byte_order()` publicly (`decoder/mod.rs:977`), so the READER
  can witness it. No available writer emits big-endian — the crate's encoder rejects a non-native order
  with `ByteOrderMismatch`, Pillow always writes `II` regardless of the prefix hint, and no
  big-endian TIFF exists anywhere in the repository (12 scanned). Writer-blocked, not reader-blocked.

The rejected kinds are recorded in each oracle's own `qualification.criteria`, so a later reader can
see which were tested and rejected rather than never considered.

## What this means for the rest

The remaining 41 should each be re-asked as **two** questions, not one:

1. Can any third-party library WRITE a before/after pair for this kind?
2. Can any third-party library READ the difference?

Only when both answers are no is the kind genuinely uncoverable. "Our encoder can't do it" answers
neither. The subject-side gap remains real and still shows up honestly — in
`runtimeMutationCoverage`, which is where it belongs.

---

## Round 2 — `gif@89a` +4, and a stronger arrangement than the others

Coverage **621/658 (94.38%)**. The same two questions closed four more.

`gif-89a-any-mutate-reader` uses the crate's high-level `Decoder`, which models a decoded ANIMATION:
comment (`0xFE`) and application (`0xFF`) extension blocks are consumed on the way to frames and never
surfaced. Four kinds were `-uncarried` against it.

The crate's **documented low-level entry point** does surface them — `StreamingDecoder` emits
`Decoded::SubBlockFinished(AnyExtension)` and `last_ext()` returns `(label, payload, is_end)`. Payloads
are accumulated across sub-blocks, so an extension longer than 255 bytes reads whole rather than as
fragments.

The crate cannot **write** them: `ExtensionData` has only `Control` and `Repetitions`. So Pillow writes
— `comment=` emits the `0xFE` block, `loop=` on a multi-frame save emits the `0xFF` NETSCAPE2.0 block.

**Writer and reader are two different third-party implementations here**, which is stronger than the
same-library arrangement (`gif` writes / `gif` judges, Pillow writes / Pillow judges) used elsewhere in
this ticket. Measured: a plain Pillow GIF reads 0 extensions, a commented one reads a single `0xFE`,
and a looped multi-frame save reads a single `0xFF` — presence, absence and label all discriminate.

## Running total for the reframing

| Subset | Kinds | Writer | Reader |
|---|---|---|---|
| `png@1.2` | 3 | Pillow `PngInfo.add` | Pillow `ChunkStream` |
| `jpg@jfif-1.01` | 2 | Pillow `quality` / `dpi=` | Pillow `quantization` / `info['jfif_*']` |
| `obj@3.0` | 3 | handcrafted OBJ text | three `OBJLoader` |
| `gif@89a` | 4 | Pillow `comment=` / `loop=` | `gif` `StreamingDecoder` |
| | **12** | | |

**Twelve kinds, none of which needed `semio-s-plugin-stdio` to compile.** Every one was measured
against the proposed reader before being claimed, and every kind that did not move its projection was
left `-uncarried` and recorded in the oracle's own qualification criteria.

---

## Round 3 — `pdf` +8, and `pdf` is now complete

Coverage **629/658 (95.59%)**. The last eight pdf kinds — `insert-encryption-dictionary` and
`remove-encryption-dictionary` across `vt`, `a`, `e` and `x` — are closed, so **all 104 pdf kinds now
carry a qualifying oracle**.

Both halves of the old `-uncarried` reason were measured, and both were about **lopdf**, not about PDF:

* **Writing** — lopdf 0.44's writer takes its encryption path whenever the trailer carries `/Encrypt`
  and then demands the encryption state a genuine decryption would have recorded, so a synthetic
  dictionary fails on its own output (`object ID 8 0 not found`). Recorded earlier in this ticket.
* **Reading** — handed a *genuinely* encrypted PDF, lopdf **decrypts transparently** with the empty
  user password and then reports `is_encrypted() == false`. So even a real encrypted fixture would have
  projected as unencrypted. This half had never been tested; the earlier note assumed the writer
  limitation settled the question.

`pypdf` 6.14 does both: `PdfWriter.encrypt` emits a real standard security handler and
`PdfReader.is_encrypted` reports it.

**The determinism check mattered here more than anywhere else.** Encryption schemes commonly randomise
a key, which would make every regeneration produce different bytes and fail this repository's own
`fixtureReproducibilityCoverage` gate. pypdf's encrypted output hashed identically across three runs,
checked before anything was built on it.

The reader is shared by all four conformance subsets — the question is identical across them, and the
framework already lets a probe's `command` point at another subset's script (brep's manifold probe
points at step's).

## Running total

| Subset | Kinds | Writer | Reader |
|---|---|---|---|
| `png@1.2` | 3 | Pillow `PngInfo.add` | Pillow `ChunkStream` |
| `jpg@jfif-1.01` | 2 | Pillow `quality` / `dpi=` | Pillow `quantization` / `jfif_*` |
| `obj@3.0` | 3 | handcrafted OBJ text | three `OBJLoader` |
| `gif@89a` | 4 | Pillow `comment=` / `loop=` | `gif` `StreamingDecoder` |
| `pdf@1.7` ×4 | 8 | pypdf `encrypt` | pypdf `is_encrypted` |
| | **20** | | |

Twenty kinds, none of which needed `semio-s-plugin-stdio` to compile. In every case the recorded reason
described the *first reader tried*, and generalised it to all readers.

---

## Round 4 — `gif@87a` interlace and `bmp` header: +2, and a measurement of mine that was wrong

Coverage **631/658 (95.90%)**.

### `set-image-interlace` — my own earlier measurement was the artifact

This ticket recorded, twice, that no writer sets the GIF image-descriptor's interlace bit while keeping
an 87a signature — and cited a test of Pillow's `interlace` keyword "as both `True` and `1`" showing
the packed byte stayed `0x00`.

That test used a 4×3 and an 8×8 image. Reading `GifImagePlugin.get_interlace`:

```python
def get_interlace(im):
    interlace = im.encoderinfo.get("interlace", 1)
    # workaround for @PIL153
    if min(im.size) < 16:
        interlace = 0
    return interlace
```

Pillow defaults to **interlaced**, and only forces it off below 16 pixels. Both of my test images were
under the threshold, so the keyword genuinely did nothing — **for those images**. At 16×16 the bit is
written by default and cleared by `interlace=False`, both under a `GIF87a` signature.

The kind is now carried, its fixture pair uses a 16×16 canvas where every other kind uses 4×3, and the
oracle's rationale records why — so the next person to read "no writer can do this" in this repository
sees that the claim was once made here and was wrong.

### `bmp::change-header-fields`

`image`-rs decodes a BMP to pixels; the BITMAPINFOHEADER's resolution fields are consumed on the way.
Pillow writes them (`dpi=` → `biXPelsPerMeter`/`biYPelsPerMeter`, measured 3780 → 11811) and reads them
back. The projection unpacks **all ten** header fields, not only the two this fixture moves, so it does
not silently narrow to what happened to be tested.

## Running total for the reframing

| Subset | Kinds | Writer | Reader |
|---|---|---|---|
| `png@1.2` | 3 | Pillow `PngInfo.add` | Pillow `ChunkStream` |
| `jpg@jfif-1.01` | 2 | Pillow `quality` / `dpi=` | Pillow `quantization` / `jfif_*` |
| `obj@3.0` | 3 | handcrafted OBJ text | three `OBJLoader` |
| `gif@89a` | 4 | Pillow `comment=` / `loop=` | `gif` `StreamingDecoder` |
| `pdf@1.7` ×4 | 8 | pypdf `encrypt` | pypdf `is_encrypted` |
| `gif@87a` | 1 | Pillow `interlace=` at ≥16px | `gif` `Decoder` |
| `bmp@v3` | 1 | Pillow `dpi=` | Pillow BITMAPINFOHEADER unpack |
| | **22** | | |

**Twenty-two kinds, none needing `semio-s-plugin-stdio` to compile.** Every recorded blocker that fell
described the first reader tried — or, once, a measurement taken under a condition that suppressed the
very behaviour being tested.

---

## Round 5 — `obj` +6, and the failure mode this ticket keeps repeating

Coverage **637/658 (96.81%)**.

Six `obj` kinds — `insert`/`remove` × `vertex`/`texcoord`/`normal` — were recorded uncarried because
"an element no face references is dropped by this loader too". That measurement was real: I appended
an unreferenced `v 2 2 2` and the projection did not move.

**But appending is one instance of the kind, not the kind.** OBJ face indices are ABSOLUTE. Inserting
or removing at the FRONT of a `v`/`vt`/`vn` list changes what every subsequent index resolves to, so
the mutation reaches the resolved geometry and a mesh loader sees it plainly. All six move the
projection when the edit is at the front.

The projection was widened to include resolved positions, normals and uvs, and all nine obj document
kinds now validate 9/9 both ways.

### The limit that remains, stated rather than hidden

An insertion **past the last referenced element** still would not move this projection. These fixtures
exercise the front, where the kind is observable through a mesh loader. A document-preserving OBJ
parser would cover both cases; none was available offline — npm, PyPI and the vendored cargo registry
were all checked. That limit is written into the oracle's rationale and its qualification criteria, so
it is visible to whoever reads the registration next.

`set-unknown-statements` stays `-uncarried`: OBJLoader skips an unrecognised line with a warning.

## The failure mode, named

Three times now a kind was recorded unwitnessable on a measurement that was **true of the instance
tested and false of the kind**:

| Kind | The too-narrow measurement | What it missed |
|---|---|---|
| `gif@87a::set-image-interlace` | Pillow's `interlace` keyword did nothing | tested at 4×3 and 8×8; `get_interlace` forces 0 below 16px |
| `pdf::*-encryption-dictionary` | lopdf cannot WRITE a synthetic `/Encrypt` | the reading half was never tested — lopdf decrypts and reports `false` |
| `obj::insert-vertex` and five siblings | an appended unreferenced vertex is invisible | a FRONT insertion shifts every absolute face index |

The shape is always the same: one instance is tried, it does not move the projection, and the
conclusion is written as a property of the library or the format. **The guard is to vary the instance
before recording a negative** — a different size, a different position in the list, the other half of
the read/write pair.

---

## Round 6 — `tiff::change-byte-order`, a negative this ticket recorded TWICE

Coverage **638/658 (96.96%)**.

Both earlier measurements were real and neither was wrong:

* `tiff` 0.11's encoder rejects a non-native order with `UsageError::ByteOrderMismatch`.
* Pillow's `im.save(..., format='TIFF')` emits `II` regardless of any prefix hint.

What neither covered is that **Pillow's IFD serialiser is a separate entry point**.
`ImageFileDirectory_v2(ifh=b'MM\x00\x2a...')` encodes every tag in the requested endianness. The
library does the field layout; the generator lays out only the 8-byte header and concatenates the
strip. Nothing hand-encodes a TIFF field.

One implementation detail worth recording, because it cost two wrong attempts: PIL's `tobytes()`
**relocates `StripOffsets` itself**, adding where the strip lands. Setting it to a computed value
yields double-counting (122 became 244, and the file read as truncated); setting it to `0` and letting
PIL place it produces a file both PIL and the `tiff` crate accept.

### The check that makes this fixture mean something

`change-byte-order` must change the order **and nothing else**. A pair that also moved the image would
pass a bare order comparison while proving much less. So the projection carries the pixel checksum
beside the order, and the generator **refuses a pair whose checksums differ**. Measured: both files
decode to checksum 2464, and both load in Pillow as 8×8 mode `L` — so the fixture is a valid TIFF by a
second reader too.

## The failure mode, fourth instance

| Kind | The too-narrow measurement | What it missed |
|---|---|---|
| `gif@87a::set-image-interlace` | Pillow's `interlace` did nothing | tested below the 16px threshold that suppresses it |
| `pdf::*-encryption-dictionary` | lopdf cannot WRITE a synthetic `/Encrypt` | the read half was never tested |
| `obj::insert-vertex` ×6 | an appended unreferenced vertex is invisible | a FRONT insertion shifts every absolute index |
| `tiff::change-byte-order` | the encoder rejects non-native; `im.save` emits `II` | the library's IFD serialiser is a different entry point |

Four negatives, one shape: **an entry point was tested, not a capability.** The guard that keeps
working is to ask "what else in this library could do it, and what other instance of this mutation
would look different" before writing the negative down.

---

## Round 7 — `obj::set-unknown-statements`, and `obj` is complete

Coverage **639/658 (97.11%)**. All ten `obj` document kinds now carry a qualifying oracle.

The recorded reason was "OBJLoader skips an unrecognised line with a warning" — true, and it had been
tested with `zz custom 42`. What it missed is a **definitional** point rather than a library one:

> `unknown_statements` is defined by what THIS SUBSET'S CODEC models, not by what three does.

The codec models `v`/`vt`/`vn`/`f`/`g`/`o`/`mtllib`/`usemtl`/`s`. **`l` (polyline) and `p` (point) fall
outside it** — so they land in `unknown_statements` — and three's `OBJLoader` parses both into `Line`
and `Points` objects. The fixture uses `l`, and the projection moves.

Measured alongside the instances that do **not** work, so the claim stays no wider than the evidence:

| Statement | Witnessed? |
|---|---|
| `l 1 2 3` | ✅ parsed into a Line |
| `p 1 2` | ✅ parsed into Points |
| `# a comment line` | ❌ skipped |
| `zz custom 42` | ❌ skipped with a warning |

The kind is carried for statements three models and not for the rest, and the oracle's rationale says
so — a fixture exercising a comment would not be evidence.

## The failure mode, fifth instance — and a new variety of it

| Kind | The too-narrow measurement | What it missed |
|---|---|---|
| `gif@87a::set-image-interlace` | Pillow's `interlace` did nothing | tested below the 16px threshold that suppresses it |
| `pdf::*-encryption-dictionary` ×8 | lopdf cannot WRITE `/Encrypt` | the read half was never tested |
| `obj::insert-vertex` ×6 | an appended unreferenced vertex is invisible | a FRONT insertion shifts every absolute index |
| `tiff::change-byte-order` | encoder rejects non-native; `im.save` emits `II` | the IFD serialiser is a separate entry point |
| `obj::set-unknown-statements` | `zz custom 42` is skipped | "unknown" is defined by OUR codec, and `l`/`p` are unknown to us and known to three |

The first four were about testing an entry point instead of a capability. The fifth is different and
worth naming separately: **the mutation's own vocabulary was read as if it were the reader's**. When a
kind is named after a gap in our model ("unknown", "unsupported", "other"), its instances are whatever
our model omits — which may be perfectly ordinary to the reader judging it.

---

## Round 8 — `jpg::remove-quant-table`, and the second instance of misreading the KIND

Coverage **640/658 (97.26%)**.

The recorded reason was: *"`remove-quant-table` has no writer at all: a JPEG without a quantisation
table is not a decodable JPEG."* That statement is true. It was also the wrong question.

**The kind removes a table from the TABLE LIST.** A JPEG may legally have all its components share
**one** table instead of two, and Pillow's `qtables=` writes exactly that: `[STD, STD]` produces two
tables, `[STD]` produces one — at the same mode and the same size. Measured: `['0','1'] → ['0']`, mode
`RGB` on both sides. Only the table count moves.

## Two distinct failure modes, now four and two

**(a) An entry point was tested, not a capability** — four instances:

| Kind | Tested | Missed |
|---|---|---|
| `gif@87a::set-image-interlace` | Pillow's `interlace` keyword | the 16px threshold that suppresses it |
| `pdf::*-encryption-dictionary` ×8 | lopdf cannot write `/Encrypt` | the read half |
| `obj::insert-vertex` ×6 | an appended unreferenced vertex | a front insertion shifting absolute indices |
| `tiff::change-byte-order` | the encoder and `im.save` | the IFD serialiser |

**(b) The KIND was misread, and the negative followed from the misreading** — two instances:

| Kind | Read as | Actually |
|---|---|---|
| `obj::set-unknown-statements` | "a statement no reader parses" | a statement **our codec** does not model — `l` and `p` are unknown to us and ordinary to three |
| `jpg::remove-quant-table` | "a JPEG with no quantisation tables" | one fewer entry in the **table list** — two components sharing one table is legal |

(b) is the more dangerous of the two, because the reasoning looks sound in isolation and the library is
never the thing at fault. The guard is to re-read what the mutation's own schema says it changes before
concluding that nothing can observe it.

---

## Round 9 — the inventory itself was the blind spot: `jpg` +2, `gif` +2

Coverage **644/658 (97.87%)**. `gif` is complete; `jpg` has one kind left.

This ticket had enumerated **1876 vendored cargo crates, all of `node_modules`, and the Python
environment**, and concluded that four kinds had no reader anywhere. Every individual finding in that
survey was correct, and two of them were verified against *two* library versions.

**The survey was scoped to LIBRARIES and never to installed command-line tools** — even though
Protocol v2 lists `third-party-cli` as a qualifying oracle kind, in the same sentence as
`third-party-library`. A single `command -v` sweep of the PATH found:

| Tool | Closes |
|---|---|
| `djpeg -v -v` / `jpegtran` (libjpeg-turbo 3.2.0) | `jpg::change-restart-interval`, `jpg::replace-huffman-table` |
| `giftext` / `gifbuild` (giflib 6.1) | `gif@87a::set-pixel-aspect-ratio`, `gif@89a::set-pixel-aspect-ratio` |

### What each provides

**libjpeg-turbo.** `djpeg -v -v` prints every marker it walks — quantisation tables with values,
Huffman tables with code-length rows, Start-of-Frame, and `Define Restart Interval N`. The writer is
`jpegtran` from the same toolchain and it is a **lossless** transcoder, so `-restart` and `-optimize`
re-emit the identical image; the generator asserts the Start-of-Frame line is unchanged rather than
trusting that. The tool's version banner is filtered from the projection so it cannot vary by build
date.

**giflib.** `giftext` prints the logical screen descriptor including `Aspect = N` — the byte both
vendored `gif` crate versions write as a hardcoded zero and neither parses. `gifbuild -d` dumps a text
description carrying `pixel aspect byte N` and `gifbuild` writes a GIF back from it, byte-deterministic
across runs, preserving the `GIF87a` signature on the 87a side rather than upgrading it. The only
authored step is one line of that description — fixture authoring, which the goal statement admits —
with giflib doing every byte of the encoding and a different giflib tool doing the judging. Every other
descriptor field is asserted identical across the pair. `giftext` echoes the input PATH, so that line
is excluded: a fixture must not project differently because of where it lives.

## The third failure mode

Two were already named: **(a)** an entry point was tested, not a capability; **(b)** the KIND was
misread. This is a third and it sits above both:

> **(c) The INVENTORY was scoped, and the scope was never stated.**

The library sweep was genuinely exhaustive *within libraries*. Nothing in the write-up said "libraries",
so the conclusion read as "nothing available can do this" when it meant "no library can". Guards (a)
and (b) both operate inside a candidate set; neither can question the set itself.

The guard is to say **what was searched** whenever recording that nothing was found — and, for this
protocol specifically, to search every oracle KIND the protocol names, not the one that happens to be
most familiar.
