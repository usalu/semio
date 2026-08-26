# Wave 17 — the four defects `📓️w13-final-audit.md` named, fixed at the cause

Date 2026-08-25. Successor to `📓️w13-final-audit.md`, which produced the first full differential
number for this ticket:

```
oracle:  cases=164 executed=1331 passed=1331 failed=0  parity=0/0
parity:  cases=101 executed=3205 passed=3154 failed=27 errored=24 parity=1012/1277
```

This wave takes four of that audit's findings and fixes each one in OUR code. Nothing was weakened:
no `ignoreKeys`, no widened tolerance, no changed `arrays` mode, no swapped or normalised fixture,
no deleted scenario. One projection was NARROWED, deliberately and with the justification written
into the production source (§3).

---

## 0. The tree this ran against

Head `8d9b51f081`, dirty: a concurrent session is mid-refactor on `📄️pdf` (renaming
`PdfSnapshot::page` → `pages`, which is audit remedy #3 — the 65-page thesis). Through that window
`cargo build -p semio-s-plugin-stdio` fails with 19 `E0609: no field 'page'` errors, ALL of them in
`🗿️artifacts/📄️pdf/`. Every file this wave touched type-checks: no error in `🖊️dwg`, `📷️jpg`,
`🖼️tiff`, `🔣️json`, `📜️docx` or `📕️xlsx` appears in that build. Two other sessions are also live
(`🎬️sequence` added four untracked `🧪️*.test.js` files, which is what the `testing/discovery`
`unmanaged-tests` breach in the contract phase is reporting — 4 in `✏️s`, 42 in `🧰️framework`, none
of them this wave's).

Probe crate used for the DWG diagnosis: `w17-dwg-probe/` (standalone `[workspace]`, so it never
touches the contended root `Cargo.toml`). Hand-decoder used to settle the bit layout:
`🐍️w17-dwg-obj-probe.py`.

---

## 1. DWG cannot read its own fixture — 14 scenarios

`mutate-dwg-ac1018` and `mutate-dwg-ac1024` failed ALL 7 scenarios each with
`R2004 entity 0x239 type 77: dwg bitstream underflow` on the real committed 148,638-byte AC1024
file at `🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg`.

### What it actually was

Not a section-header offset and not a mis-sized field: **the R2010 common-entity layout was
transcribed from the R2000 one.** Two version-gated differences, in opposite directions:

* `nolinks` (`B`) is **R13–R2000 only**. The drawing-geometry decoder consumed it anyway — one bit
  too many.
* R2010 adds **three visual-style presence bits** (`has_full_visualstyle`, `has_face_visualstyle`,
  `has_edge_visualstyle`) after `shadow_flags`, each gating a handle in the handle stream. The
  decoder read none of them — three bits too few.

Net two bits, and from there every field of a real AutoCAD entity decodes as noise.

### The evidence, not the theory

The framing was verified first, so it could be ruled out. Object `0x239` sits at offset `0x4cfd` of
the decompressed `AcDb:AcDbObjects` section; `dwg_crc16(0xC0C1, …)` over bytes `[0, 59)` — `MS`
size + `UMC` handle-stream size + the 56-byte payload — equals the two bytes at `[59, 61)` exactly.
So the payload window and the handle-stream boundary this codec already computed were RIGHT, and
`payload_len` really does count from after the `UMC` field.

Then the same bit reader, in Python, on the same 56 bytes, under both hypotheses:

| field | WITH `nolinks` (what the code did) | WITHOUT `nolinks`, WITH 3 visual bits |
|---|---|---|
| colour | 160 | 0 (ByBlock) |
| linetype scale | `-9.25e+61` | `1.0` |
| lineweight | 0 | **29** (`0x1D`, ByLayer — the exact value this repository's own writer emits) |
| LWPOLYLINE flags | 0 | 4 (constant width present) |
| constant width | — | `0.15` |
| vertex count | **185466880** | **2** |
| vertices | — | `(-0.5,-0.5), (0.5,0.5)` |
| ends at bit | underflow | **419 of a 420-bit data stream** — one pad bit, exactly right |

### The fix

`🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs`.

This codec already contained a CORRECT R2010 common-entity reader —
`decode_r2010_entity_common_main` / `decode_r2010_entity_common_handles`, used by the document and
object-identity decode paths, matching `encode_r2010_entity_common_main` bit for bit including the
three visual-style bits. The drawing-geometry path had a second, stale transcription beside it. The
repair is not a patch to the stale one: it is DELETED, and its two entry points now delegate to the
one shared body.

* `decode_r2010_entity_common_main` split into a graphic gate + a new shared
  `decode_r2010_entity_common_fields`, whose doc comment names both version-gated traps.
* `dwg_decode_r2010_entity_common` = skip the preview bitmap (this path has no typed graphic model
  to hold one), then the shared fields.
* `dwg_decode_r2010_entity_handles` = the shared handle walk, which also brings base-RELATIVE handle
  resolution (`read_object_handle`) to the geometry path — the stale one read handle codes 6/8/10/12
  as absolute values, so a layer reference could never have been resolved through the object map.
* The local `struct DwgEntityCommon` and its 11 duplicated call sites are gone;
  `dwg_decode_r2010_entity` now takes the entity's own handle as the resolution base.

**Also fixed, per the audit's §6:** the broken `include_str!` at
`🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:163` — four `../`
resolved back inside `🔖️ac1024/`, it needs five. That `#[cfg(test)]` cross-check
(`kinds_matches_every_variant_and_both_catalogs`) had never compiled. The two `.collect().await`
lines the automated async sweep had left in the same test body were repaired in the same pass; the
tree moved to `8d9b51f081` mid-session and those two turned out to be already fixed there, so
`git diff` on that file shows only the `include_str!` path.

---

## 2. `mutate-jpg-jfif-1-01-baseline :: identity-round-trip`

```
identity law violated: decoding and re-encoding moved the semantic projection —
$.components[0] is "1:2x2", expected "1:1x1"
```

`encode_jpg` hard-coded `hmax = 2, vmax = 2` and a fixed three-component `1:2x2, 2:1x1, 3:1x1`
sampling array. The committed scan is 4:4:4. So a decode/re-encode **resampled the chroma of every
4:4:4 document** and moved a conformance-class axis while doing it — T.81 §B.2.2 makes `H`/`V`
per-component FRAME parameters, and `check_baseline_conformance` reads them as one of its five axes.

Fixed at the cause, in `📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/🦀️component.rs`:

* New `frame_components_of` — the SOF0 component list comes from `snap.frame` when the snapshot
  carries one, and keeps the historical 4:2:0 default only when it does not (a snapshot that was
  never decoded from a real file). Sampling factors outside `1..=4`, or that do not divide the frame
  maximum, are REFUSED (`JpgError::Unsupported`) rather than silently rounded — both make the MCU
  geometry unrepresentable. The quantization-table selector is deliberately NOT carried through:
  this encoder emits exactly two fresh Annex K DQT tables, the same canonicalization its doc comment
  already declares for the tables themselves.
* `encode_jpg` generalized to `hmax`/`vmax` derived from the frame, one plane per component
  box-filtered by `(hmax/h, vmax/v)`, and a T.81 §A.2.3 MCU interleave emitting `h * v` blocks per
  component per MCU. It reduces to the previous behaviour exactly when the frame really is
  `1:2x2, 2:1x1, 3:1x1`. A component already at the frame maximum keeps its buffer by MOVE rather
  than being box-filtered 1:1 into a copy of itself — 46 MB of `f64` per component on this scan.

The case's feature prose said the encoder writes "a fixed three-component 4:2:0 sampling array" and
that "every axis here is normalized away on re-serialization". That is no longer true of the fifth
axis, so the paragraph was rewritten to say so rather than left stale.

---

## 3. `mutate-tiff-6-0-baseline` — two failures, two different causes

### 3a. `mutate-remove-tile-tags` — the adapter read its own feature backwards

```
this row moves its axis in the direction that stays inside the class, so the verdict must not
change, but it went from ["stdio.tiff.baseline.tiled-not-baseline"] to []
```

The feature says an empty `code` column means the row "moves its axis in the direction that stays
INSIDE the class". The adapter implemented that as *the verdict must not change*. Those are not the
same claim, and they come apart in exactly the row that carries the most weight: `remove-tile-tags`
runs from its `setup` state, where `insert-tile-tags` has already put the document OUT of the class,
and putting it back inside is the entire point of the row — the verdict is REQUIRED to change there,
from one code to none.

`mutate-tiff-6-0-baseline/🦀️component.rs` now asserts what the feature says: an empty `code` means
the document certifies CLEAN afterwards. This is **strictly stronger** than what it replaced, not
weaker — for `no-mutation` and `set-strip-offsets` (before = `[]`) the two are identical, and for
`remove-tile-tags` the new law demands an empty verdict where the old one demanded `[tiled]`.

### 3b. `identity-round-trip` — a PROJECTION narrowing, stated on purpose

```
identity law violated … $.stripOffsets is "388", expected "412"
```

This one is genuine writer freedom, and it is recorded here rather than fixed silently.

`encode_tiff_baseline_projection_json` carried the raw `StripOffsets` VALUES. Those are physical
byte positions of the strips inside whatever file the writer lays out. Adobe TIFF 6.0 Part 1 makes
Baseline class membership depend on `StripOffsets` being PRESENT when the IFD is not tiled —
`check_tiff_baseline_conformance` reads exactly that and never looks at a value — and the case's own
feature already states that `encode_tiff` regenerates every `CORE_STRIP_TAGS` entry from the raster
it is about to write. So the projection was asserting the encoder's file layout under the name of a
conformance projection, which is the one thing that surface must not do.

`stripOffsets` now reports `absent` or `present <count>`. The COUNT is retained because it is a real
property of the document — how many strips the pixel data is cut into — and because it is what keeps
`set-strip-offsets` (`{"offsets": [8, 65536]}`, 1 → 2) observable and `remove-strip-offsets`
(`present 1` → `absent`) distinguishable from it. The justification is written into the production
doc comment, not only here.

No tolerance changed, no key was ignored, no comparison profile moved: `ordered-json-v1` still
compares every field of this projection exactly.

---

## 4. Five stdio cases whose subject half had never compiled

One line each, all reported green for the whole ticket.

| case | error | fix |
|---|---|---|
| `mutate-docx-ecma-376-strict` | `E0432` unresolved import `…strict::schema::mutations::vml_markup` | `pub fn vml_markup()` added to the docx ✳️strict vocabulary, mirroring the pptx ✳️strict sibling that already had it |
| `mutate-xlsx-ecma-376-strict` | same, for xlsx | same |
| `create-and-read-jpeg` | `E0433` cannot find `jfif_1_01` in `standards` | the module is `v_jfif_1_01` |
| `create-and-round-trip-stl` | `E0433` cannot find `ascii` in `standards` | the module is `v_ascii` |
| `mutate-json-rfc8259-i-json` | `E0603` crate `protocol` is private | see §5 |

The two `vml_markup()` bodies are byte-identical to the oracle's own `VML_MARKUP` constant
(`🧪️oracle/📄️document/🦀️component.rs:778`) and to pptx's, so the three ✳️strict OOXML cases insert
the same real VML.

---

## 5. `mutate-json-rfc8259-i-json` — 0 of 22

Two independent causes, both fixed.

### 5a. `E0603: crate protocol is private`

The subject adapter reached `Mutation::inverse` through
`use semio_s_plugin_stdio::protocol::Mutation;`. `protocol` is an `extern crate … as protocol;`
alias in `📦️glue.rs`, which is private, so that path never resolved.

The repository already has the right idiom for this and it is used elsewhere: a free function beside
`apply_…`, so an owner-root adapter can drive the vocabulary without naming a trait it has no reason
to link (`inverse_svg_tiny_mutation`,
`🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧬️schema/🧬️mutations/🦀️component.rs:230`). Added
`inverse_json_i_json_mutation` to the ✳️i-json vocabulary with the same shape and the same
rationale; the adapter calls it and its local `protocol_inverse` helper is gone. The `protocol`
alias stays private — no new public surface leaks a kernel type.

### 5b. `adapter has no subject registration` ×22

Not a defect in the case at all: **the coordinator dispatched the PYTHON adapter in the subject
role.**

`runPhases` (`📜️script.ts:553`) ran the subject phase for *every* implementation the case declares
an adapter for. This repository's `🐍️component.py` files are reference HOSTS — every one of them
registers oracle handlers only, and says so in its own docstring ("registering these handlers as
subjects too would make the reference its own subject and manufacture a green self-comparison"). So
the python host errored on all 22 scenarios, and 22 null projections then entered the parity ratio
as a subject that ran and disagreed. That is how a case whose rust half is correct scores `0/22`.

Fixed in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` with a new
`ownerShipsImplementation`: an implementation is dispatched in the SUBJECT role only if this
repository actually ships a package for the case's owner in that language (walking up from the owner
root to `📦️packages/<language>`, with the language directory names read from the taxonomy's own
`testImplementationIds` — no language is named in the code). No owner outside the test domain itself
ships `📦️packages/🐍️python`; the test domain ships all five, so its own
`host-protocol-parity-subject-{go,python,dotnet,rust,typescript}` cases are untouched.

This is deliberately NOT "skip an implementation whose adapter registers no subject handlers". An
owner that DOES ship a package in a language and whose adapter then forgets a subject registration
must still fail, loudly, per scenario — that rule is unchanged.

And it must not go silent: a case every one of whose adapters is a reference host now prints
`[test] no-subject-implementation <caseDir> …`, in the same reporting style as the existing
`[test] not-exercised` line. Exactly one case in the repository is in that state —
`extract-text-pdf-1-4`, whose only adapter is `🐍️component.py` — and it was contributing the other
2 of the audit's 24 python errors.

---

## 6. Files changed

Production:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧬️schema/🧬️mutations/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧬️schema/🧬️mutations/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧬️schema/🧬️mutations/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/🧬️schema/🧬️mutations/🦀️component.rs`

Test cases and platform:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🧪️tests/create-and-read-jpeg/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🧪️tests/mutate-jpg-jfif-1-01-baseline/component.feature`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🧪️tests/create-and-round-trip-stl/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧪️tests/mutate-tiff-6-0-baseline/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧪️tests/mutate-json-rfc8259-i-json/🦀️component.rs`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts`
