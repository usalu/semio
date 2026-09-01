# 🎞️ gif@87a/any — Why the Reader-Oracle Retrofit Stops Here

Companion to `📓️gif-89a-any-reader-oracle-retrofit.md`. That subset was retrofitted successfully.
Its 87a sibling **cannot be**, and this file records exactly why, with the evidence.

## 1. A conformance defect in the committed fixture

`🧫️fixtures/mosaic-strip/mosaic-strip.gif` declares `GIF87a` **and contains a Graphic Control
Extension** (`0x21 0xF9`) — a block introduced by GIF89a and not part of the 87a grammar. The file
is internally self-contradictory: it announces a version whose vocabulary it then exceeds.

Measured:

```
mosaic-strip.gif      sig=GIF87a   GCE=yes
```

This is not what the generator intended. `🏭️generator/🦀️engine/src/main.rs:7-10` states the fixture
exercises "NOTHING GIF87a [lacks] ... that live entirely on the 89a sibling's vocabulary".

### Mechanism

`gif` 0.13.3 hardcodes its signature and emits a GCE for **every** frame, unconditionally:

- `src/encoder.rs:340` — `tmp.write_all(b"GIF89a")?;` — no version parameter exists.
- `src/encoder.rs:178` — `write_frame_header` opens with
  `self.write_extension(ExtensionData::new_control_ext(frame.delay, frame.dispose,
  frame.needs_user_input, frame.transparent))?;`

There is no conditional guarding that call. The doc comment on `write_frame` (`encoder.rs:167`) says
"writes a control extension **if necessary**" — that phrase does not describe the code; the call is
unconditional for every frame regardless of field values.

The generator's author knew about the signature and patched it (`main.rs:20-23`, patching `b"GIF89a"`
plus the background-index and aspect-ratio bytes, "exactly the way `🧪️oracle/🦀️component.rs`'s
`oracle_encode` patches them"). The GCE is the part that had no reason to be suspected: it is
emitted from a helper whose own documentation says it is conditional.

**The signature patch is what makes the defect visible.** Left unpatched the file would have been an
honest GIF89a; patched, it became an invalid GIF87a.

## 2. No available third-party writer covers 87a's surface

A reader oracle needs fixtures written by something other than us. Both candidates were tested
empirically, not assumed.

| Capability needed by 87a's mutation kinds | `gif` 0.13.3 (Rust) | Pillow 11.3.0 (Python) |
|---|---|---|
| `GIF87a` signature, unpatched | ❌ hardcoded `GIF89a` (`encoder.rs:340`) | ✅ native |
| No GCE, single image | ❌ unconditional (`encoder.rs:178`) | ✅ verified absent |
| Background colour index | ❌ no setter (patched) | ✅ `im.info['background']` → byte 11 |
| Pixel aspect ratio | ❌ no setter (patched) | ❌ not exposed (byte 12 stays `0`) |
| Interlace bit | ✅ `frame.interlaced` → `flags \|= 0b0100_0000` | ❌ `interlace=True` leaves packed byte `0x00` |
| Multiple images, no GCE | ❌ GCE per frame | ❌ GCE per frame under `save_all` |

Pillow's `interlace` keyword was tested as both `True` and `1`; the image descriptor's packed field
read `0x00` (bit 6 clear) in both cases. Pillow's `save_all` multiframe output kept the `GIF87a`
signature but introduced GCE blocks — the same defect as §1, from the other direction.

Reader-side, by contrast, `gif` 0.13.3 handles 87a **fully**: `src/reader/decoder.rs:558` maps
`b"GIF87a"` → `Version::V87a`, and `version()` is public at `decoder.rs:501`. A Pillow-written 87a
file projects correctly through this repo's existing reader binary — dimensions, palette and all
pixel indices round-trip exactly.

**So this is a writer-side limit, not a reader gap** — the same category as `tiff::change-byte-order`,
`bmp::change-header-fields` and the `jpg` quantisation/Huffman kinds recorded elsewhere in this
ticket. The judge is available; the fixtures are not.

## 3. Why 87a was not rebuilt on Pillow

Switching the generator to Pillow would remove all three byte patches and fix §1 for single-image
fixtures. It would simultaneously lose `set-image-interlace` (no interlace bit) and
`insert-image` / `remove-image` / `move-image` (multiframe reintroduces GCE) — trading one defect for
three. Rebuilding was therefore rejected on the evidence, not deferred for effort.

Closing the subset honestly needs one of:

1. a third-party GIF87a writer that emits multiple image descriptors with no GCE and an interlace
   bit — none was found in the Rust or Python ecosystems surveyed here; or
2. **real-world** GIF87a files as fixtures (the goal statement admits real-world examples), judged by
   the `gif` decoder that already reads 87a correctly; or
3. handcrafted 87a byte streams — admissible under the goal statement's "handcrafted", and safe
   because the *judge* stays third-party, but a hand-rolled writer is what §1's patching already
   drifted toward and it should be a deliberate decision, not an accident.

Option 2 is the cheapest sound path and preserves the reader-oracle discipline exactly.

## 4. Consequence for the matrix

`gif@87a/any`'s twelve mutation kinds remain without a qualifying oracle and are reported as such —
they are **not** routed around, and no `-uncarried` entry is claimed for kinds a third-party reader
demonstrably *can* witness. The uncovered count is honest.

For contrast its sibling `gif@89a/any` is fully retrofitted: 16 witnessable kinds against
`gif-89a-any-mutate-reader`, 5 recorded `-uncarried`.

## Reproduction

```bash
# §1 — the defect
python3 -c "b=open('mosaic-strip.gif','rb').read(); print(b[:6], b'\x21\xf9' in b)"

# §2 — Pillow writes conformant single-image 87a, but no interlace bit
python3 -c "
from PIL import Image
im=Image.new('P',(8,8)); im.putpalette([255,0,0,0,255,0,0,0,255]+[0]*(256*3-9))
im.putdata([(x+y)%3 for y in range(8) for x in range(8)]); im.save('/tmp/p.gif', interlace=True)
b=open('/tmp/p.gif','rb').read(); i=b.find(b'\x2c')
print('sig',b[:6],'GCE',b'\x21\xf9' in b,'interlace bit',bool(b[i+9]&0x40))"
```

---

## 5. Correction to §2/§3, and a fixture inventory that does not hold together

A scan of every `.gif` under `✏️s` against its declaring standard corrected part of the above and
surfaced further defects. Only `🔖️89a/GIF89a` (35 files) is self-consistent. Everything else:

| File | Declares | Signature | GCE | Verdict |
|---|---|---|---|---|
| `🎞️gif/🧫️fixtures/🖼️dancing-87a.gif` | — | `GIF87a` | none | ✅ **conformant 87a** |
| `🎞️gif/🧫️fixtures/🖼️dancing-87a-large.gif` | — | `GIF87a` | present | ❌ 89a block, "87a" in the name |
| `🔖️87a/…/📚️examples/🎬️demo/🖼️assets/🎞️example.gif` | 87a | *(empty)* | — | ❌ **0 bytes** |
| `🔖️87a/…/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif` | 87a | `GIF89a` | present | ❌ 89a file under the 87a subset |
| `🔖️87a/…/🧫️fixtures/mosaic-strip/mosaic-strip.gif` | 87a | `GIF87a` | present | ❌ §1 defect |

Three separate problems, none of which any current gate catches: a **0-byte** file standing in as an
example asset, an **89a** file sitting under the **87a** subset, and a name (`-large`) asserting a
conformance its bytes contradict.

### The correction

`🖼️dancing-87a.gif` **disproves §2's implied claim that conformant multi-image 87a was unobtainable.**
Projected through the same third-party reader binary used by the 89a oracle:

```
screen 16x16, bgIndex 0, loopCount None, frameCount 3
  [0] 16x16 @(0,0) interlaced=False localPalette=False
  [1] 16x16 @(0,0) interlaced=False localPalette=True
  [2] 16x16 @(0,0) interlaced=False localPalette=True
```

Three image descriptors, per-frame local palettes, **no GCE**, decoding cleanly. So multi-image 87a
is both real and readable here, and `insert-image` / `remove-image` / `move-image` are **not**
blocked on the reader — §2's table describes what the two *writers* can emit, which is a narrower
claim than "these kinds cannot be witnessed".

### What remains genuinely blocked

A committed fixture needs a **before and an after**. The reader judges both; neither may be predicted
by us. `🖼️dancing-87a.gif` supplies credible *before* bytes, but every *after* state still requires
something able to write conformant 87a:

- **`set-image-interlace`** — no surveyed writer sets the descriptor's interlace bit while keeping an
  87a signature. Genuinely blocked on a writer.
- **single-image kinds** (`set-screen-size`, `set-global-color-table`, `set-background-color-index`,
  `set-image-geometry`, `set-image-pixels`, `set-snapshot`, `no-mutation`) — Pillow writes these
  cleanly (§2) and they are **available now**; this is the tractable slice.
- **multi-image kinds** (`insert-image`, `remove-image`, `move-image`) — before-bytes exist; after-bytes
  need either byte-level excision of an image block (handcrafting, admissible but a deliberate choice)
  or a GCE-free multi-image writer.
- **`set-pixel-aspect-ratio`** — unreachable in both writers; the one kind that is legitimately
  `-uncarried`, matching its 89a sibling.

So §3's "rebuilding was rejected" stands only for a *wholesale* switch to Pillow. A **partial**
retrofit covering the seven single-image kinds is sound and unblocked, leaving interlace and the
three multi-image kinds recorded as writer-blocked. That is the recommended next step for this
subset, and it was scoped but not executed here.

### Independent of the oracle work

The 0-byte `🎞️example.gif` and the 89a-under-87a `🖼️dancing.gif` are defects in the subset's example
assets regardless of any oracle decision, and are worth fixing on their own.

---

## 6. RESOLVED — the corpus was rebuilt, and §1's defect is gone

§3 said rebuilding on Pillow "would simultaneously lose `set-image-interlace` and the three
multi-image kinds — trading one defect for three". §5 corrected part of that. This section records the
rest: **the multi-image objection was wrong**, and the subset is now retrofitted.

### What §3 missed

Pillow's `save_all` path reintroduces a GCE — that much was measured and is true. But a multi-image
GIF87a does not have to be written in one `save_all` call. It can be **assembled** from Pillow's own
single-image output:

* GIF's block grammar is fixed: signature + logical screen descriptor + global colour table, then a
  sequence of image blocks, then the trailer.
* Each Pillow-written single-image 87a contributes exactly one image block, LZW data included.
* Concatenating those blocks under one container header yields a conformant multi-image 87a.

Nothing is hand-encoded — every byte of image data is Pillow's. The one real hazard is that Pillow
computes a palette per image, so blocks from files with different global palettes would silently
re-attribute every frame's colours. The generator **asserts the source headers are byte-identical**
before assembling, and the fixtures are built so every frame uses all four palette entries.

Verified before it was relied on: `f_ab`'s two frames read back with indices exactly equal to those of
`f_a` and `f_b` read separately.

### Result

| | |
|---|---|
| Kinds carried by `gif-87a-any-mutate-reader` | **10** |
| Kinds `-uncarried` | **2** |
| Gate directions correct | **20/20** |
| Committed files verified `GIF87a` with no GCE | **20/20** |

The two that remain are the two §2's table already identified as writer-blocked, and neither is a
reader gap:

* **`set-pixel-aspect-ratio`** — no surveyed writer emits that byte; uncarried in the 89a sibling too.
* **`set-image-interlace`** — Pillow's `interlace` keyword leaves the descriptor's packed byte at
  `0x00` (tested as both `True` and `1`), and `gif` 0.13, which does set the bit, cannot write an 87a
  signature.

### §1's defect is fixed, and cannot return unnoticed

`mosaic-strip.gif` is gone along with the generator that patched its signature. The reader's projection
now leads with the **declared version**, taken from `gif`'s own header parser through the crate's
documented `streaming_decoder` entry point rather than from us reading six bytes — and the gate
compares it like any other field. A file that declares 87a while carrying 89a constructs would now
fail on the version, on the GCE, or on both.

The reader was checked against an 89a file and correctly reported `GIF89a`, so that field
discriminates rather than echoing an assumption.

### Still outstanding from §5, independent of the oracle work

The 0-byte `🎬️demo/🖼️assets/🎞️example.gif` and the 89a-under-87a `💃️dancing/🖼️assets/🖼️dancing.gif`
are example assets, not fixtures, and remain as recorded.
