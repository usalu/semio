# Wave 16 — four `🧿️semio` subsets converted to cross-language differential oracles

Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Scope: the `🧿️semio` subsets `✳️drawing`, `✳️mesh`,
`✳️image` and `✳️presentation`. Successor to `📓️w13-cross-language-recipe.md`, whose recipe this wave
follows and extends.

**All four recorded `noOracleDecision`s are gone. Each is replaced by a registered oracle that is a
second producer in another language, running against a real, complex artifact derived once from real
committed content.** Three of the four converted clean; the fourth found a codec defect and is left
red on purpose.

---

## 0. Headline

| case | second producer | language | real artifact | scenarios | parity |
|---|---|---|---|---|---|
| `mutate-semio-image` | **Pillow 11.3 / littleCMS** + an independent carrier implementation | python | 3 native frames of the committed `🖼️color-animated-text.gif` | 40 | **40/40, exit 0** |
| `mutate-semio-presentation` | independent implementation | python | the real committed `🎞️semio-talk.pptx` — 1 master, 11 layouts, 7 slides, 98 shapes | 46 | **44/46, exit 1** — 2 red by design, see §4 |
| `mutate-semio-mesh` | **three.js r185** + an independent carrier implementation | typescript | the committed Metabolism `🧊️base.glb` — 271 meshes, 459 primitives | 52 | **51/52, exit 1** — 1 red by design, see §4b |
| `mutate-semio-drawing` | independent implementation | python | the committed `mouse.svg` + `qr-code.svg` — 3 layers, 1 006 nodes, depth 5 | 52 | **51/52, exit 1** — 1 red by design, see §4c |

`bun ./📜️script.ts dependency` stayed at `entries=232 production-reachable=151 test-oracle=30`,
exit 0 — every oracle entry uses the `package: ""` device, so nothing was added to
`🔒️dependencies.json`.

---

## 1. Which second producer, and why it is genuinely independent

### `✳️image` — Pillow is a real third party and it speaks the payload

The old decision rejected the raster crates on the ground that a decoder would first have to be handed
a file OUR encoder produced. **That objection is answered by reversing the direction.** Pillow reads a
real third-party-authored GIF and PRODUCES the samples; it never sees a byte our encoder wrote.
Concretely:

* every RGBA8 sample under test was decoded out of `🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️color-animated-text.gif`
  by Pillow;
* the profile `set-icc` attaches is a real 588-byte sRGB profile littleCMS emitted through `ImageCms`;
* every scenario that produces frames projects a `raster` report — per frame, whether an `RGBA` image
  of the declared geometry can be reconstructed at all, and then its mode, size, per-band extrema and
  distinct-colour count. The oracle gets those from Pillow; the Rust subject computes the same four
  facts by hand from the same planes. A projection matches only when a raster library that has never
  seen this repository and this repository's own codec agree about the actual samples.

The carrier and the thirteen verbs are semio's own, so that half is a second IMPLEMENTATION
(`🧪️tests/mutate-semio-image/🐍️component.py`, 677 lines), written from the committed snapshot DSL
grammar, the pack protocol AND its Kaitai mirror — which for this subset describes the trailing chain
completely, so nothing had to be reverse-engineered from bytes — the mutation grammar, the mutation
JSON schema and the committed per-kind vectors. Pinned before use: it reproduces the committed
`✳️any/📚️examples/🖼️swatch` artifact **byte for byte in both encodings** and reaches all twelve
committed after-snapshots.

### `✳️mesh` — three.js is a real third party and it speaks the geometry

A `SemioPrimitive` is exactly what a `THREE.BufferGeometry` models. Every scenario builds a real
`BufferGeometry` — positions in a `Float64Array`, so nothing is quantised on the way in — and three.js
states the attribute counts, the bounding box `Box3` computes from the position attribute, and the
flat vertex stream `toNonIndexed()` produces by walking the index buffer. The Rust subject computes
the same by hand. A permuted index buffer or a lost vertex shows up in the expanded stream.

The carrier and the seventeen verbs are a second IMPLEMENTATION in **TypeScript**
(`🧪️tests/mutate-semio-mesh/🟦️component.ts`, 969 lines) — a third language in this family, after the
Rust subject and the Python oracles. Pinned before use: it reproduces the committed `🧊️cube` artifact
**byte for byte in both encodings** and reaches all seventeen committed after-snapshots.

### `✳️presentation` and `✳️drawing` — no third party, so a second implementation

Both earlier decisions surveyed the obvious candidates and declined them, and **both surveys still
stand on their merits**: `python-pptx` cannot create masters or layouts at all and reaching a
`SemioPresentationSnapshot` from pptx bytes needs our own importer; `usvg`/`resvg` have no counterpart
for an anonymous recursive `DrawNode` tree addressed by a structural `NodePath`, and `lyon`/`kurbo`
could adjudicate at most `replace-path`. What has changed is that a second producer no longer has to
be a third-party library. Both are independent Python implementations (1 022 and 1 190 lines) written from the committed
grammar and protocol documents, pinned by reproducing their subsets' committed example artifacts byte
for byte in both encodings and by reaching every committed after-snapshot.

**Neither imports, links, wraps nor transliterates the Rust.** The one place where prose was not
enough is the pack frame: for `✳️mesh`, `✳️presentation` and `✳️drawing` the protocol document declares
the collections an opaque `payload` chain by its own admission. The Kaitai mirrors name what is inside
it (`✳️mesh` in full, `✳️presentation` down to "real-tag-byte-encoded … `DocBlock` … embedded as a
length-prefixed UTF-8 blob"); the exact field ORDER was derived from the committed example bytes
against their readable DSL twin, and each derivation is pinned by re-encoding that committed file byte
for byte, which a misreading could not do. `✳️image` needed no derivation at all.

---

## 2. Which real artifact, and its provenance

Every one was derived ONCE, by an INDEPENDENT reader, from real committed content, and written out
through the case's own independent implementation — so the Rust subject then has to reproduce the
other implementation's bytes. `asset://` cannot leave the artifact root, which is why each derived
document is committed as a case fixture rather than borrowed in place.

| case | source | reader | result |
|---|---|---|---|
| image | `🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️color-animated-text.gif` — 194×84, GIF89a, 16 frames at 130 ms, NETSCAPE2.0, palette-indexed | Pillow | first 3 frames at NATIVE resolution, no resampling and no cropping; `colorspace: indexed`, `bitDepth: 8` because that is what the source is; five metadata entries stating real facts about the file, one German with an en dash and a multiplication sign. **391 703 B DSL / 195 860 B pack** against the committed swatch's 217 / 110 |
| presentation | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx` — a real 2020 conference talk | an independent `zipfile` + `xml.etree` OOXML reader, never our pptx bridge | 1 master, **11 layouts, 7 slides, 98 shapes**, 3 embedded PNG parts, German throughout, real EMU geometry with pptx inheritance (slide→layout→master by `idx` then `type`) resolved rather than zeroed. **183 293 B DSL / 97 849 B pack** against the committed deck's 826 / 516 |
| mesh | `🗿️artifacts/🧊️gltf/…/🌱️metabolism/🖼️assets/🧊️base.glb` — a real architectural model | a hand-written GLB walker (header, JSON chunk, BIN chunk, accessors by component type), no glTF library and not our gltf bridge | **271 meshes, 459 primitives, 1 544 vertices, 2 184 indices, two draw modes, 2 PBR materials**. One addition, stated as one: the GLB embeds no image, so one texture carries the real committed `🖼️marker-left.png`'s own 666 bytes. Ids are derived from the source's own indices because every mesh, primitive and material in that GLB is anonymous. **188 746 B DSL / 106 590 B pack** against the committed cube's 340 / 388 |
| drawing | `🗿️artifacts/🎨️svg/🧫️fixtures/mouse.svg` + `…/qr-code.svg` | an independent `xml.etree` reader plus a path-data scanner written from SVG 1.1 §8.3, never our svg bridge | **3 layers, 1 006 nodes nested 5 deep, 1 728 path segments, 4 styles**, one layer whose `display:none` is its real `visible: false`, one style carrying the QR background's real `opacity:0.5`, and a real 5 476-byte embedded image. Relative commands resolved to absolute, `H`/`V` to `lineTo`, `S`/`T` per §8.3.6. **56 205 B DSL / 85 791 B pack** against the committed sketch's 394 / 533 |

The one resolution in the drawing derivation that is not data — `currentColor` becoming black, CSS's
initial value for `color` — is stated in the feature, in the derivation script and here.

## 3. The case shape

Identical across all four, following `📓️w13-cross-language-recipe.md` §4:

* `mutate-<kind>` and `inverse-<kind>` for EVERY declared kind, against the REAL derived artifact,
  `@mode-differential`;
* `spec-vector-<kind>` for every kind that owns a committed handcrafted `(before, mutation, after)`
  vector — **added, never substituted**: every piece of evidence these cases rested on before the
  oracle existed is still exercised, now by both implementations;
* `identity-round-trip`, `@mode-round-trip`, asserting byte-exact re-emission of both encodings on
  each side plus a cross-language digest comparison.

`inverse-` projects `{mutated, restored}` rather than only `restored`, so the differential cannot go
vacuous. Each side still states its own law in role, so a red scenario is readable without re-running.

---

## 4. 🔴 The finding: `set-snapshot` cannot change a slide's `id`

`mutate-set-snapshot` and `inverse-set-snapshot` in `mutate-semio-presentation` are **RED, and they
are left red.**

The payload replaces the deck with one whose slides are the same seven in REVERSE order. The
independent implementation returns the reversed deck. The subject returns the reversed layouts, shapes
and notes **but the original seven slide ids, still at their original indices** — so slide 0 ends up
carrying slide 23's content under slide 1's identifier.

Confirmed at the cause, in production code and not in either adapter:

```rust
// ✳️presentation/🧬️schema/🔺️diff/🦀️component.rs
pub struct SlideDiff {
    pub layout_id: Option<Option<String>>,
    pub shapes: Option<SlideShapesDiff>,
    pub notes: Option<DocBlocksDiff>,
}
```

There is no `id` slot. `set-snapshot`'s semantics are `SemioPresentationDiff::between`, and slides are
an INDEX-keyed collection, so an index-keyed slide diff has nowhere to carry a new identifier.

**Why it was invisible until now:** the committed specification vector's replacement snapshot reuses
the same single slide id (`before` and `after` both `["slide1"]`), so it cannot discriminate. It takes
a real deck with seven distinct slide ids to see it. `spec-vector-set-snapshot` stays GREEN, which
localises the failure precisely to the reordering case.

**Nothing was tuned away**: no `ignoreKeys`, no relaxed comparison profile, no substituted payload.
Swapping the payload for one that keeps the ids would be exactly the dodge the standard forbids. A
whole-document replacement that keeps the old identity strings while taking the new content is a
defect, not a convention, and the fix — giving `SlideDiff` an `id` slot — is a production change in the
diff facet with its own mirrors and fixtures, which belongs in its own ticket rather than being
smuggled into a test conversion.

### 🔴 4b. The second finding: the `.dsl.semio` TEXT carrier is not byte-stable across implementations

`identity-round-trip` in `mutate-semio-mesh` is **RED, and it is left red.**

All 51 other scenarios agree — every `mutate-`, every `inverse-` and every `spec-vector-` — so the two
implementations agree completely about the model and about what each of the seventeen verbs does to
it, three.js's geometry report included. What they do not agree on is ONE CHARACTER of one coordinate,
at byte 4 732 of a 188 746-byte document:

| | spelling of the same `f64` |
|---|---|
| the committed artifact, written by the TypeScript implementation | `20.170669555664062` |
| what the Rust printer emits for it | `20.170669555664063` |

Verified rather than assumed: both decimals parse to the identical bit pattern `00000000b12b3440`;
neither fifteen nor sixteen significant digits round-trips, so both are seventeen-digit shortest
forms; and Python's `repr` and V8's `String()` both give `…062`, which makes Rust's `f64` `Display`
the outlier on the last digit's tie-break.

This is a finding about the FORMAT, not about either codec's semantics. The `number` production is
`INT | FLOAT` and names no canonical spelling, so "re-printing the document reproduces the file byte
for byte" — the property `law::carrier_is_exact` asserts across this whole family — turns out to be a
property of one language's float printer rather than of `.dsl.semio`. The pack twin has no such
problem and passes: it moves the `f64` bit pattern, not a decimal.

**Nothing was tuned away.** Teaching the TypeScript writer Rust's tie-break would hide precisely the
ambiguity this case just found; the fix belongs in the grammar, which needs to name a canonical
`FLOAT` spelling. Until it does, `.dsl.semio` is a single-implementation text format wherever a
coordinate needs seventeen digits — which no committed example before this wave ever did, because
every one of them was hand-written with short decimals.

### 🔴 4c. The third finding: `Unflatten`'s computed inverse cannot restore an arbitrary replaced node

`inverse-unflatten` in `mutate-semio-drawing` is **RED, and it is left red.**

The payload replaces the mouse layer's real `clipPath` group with a different one. The independent
implementation undoes it by putting the captured node back and restores the drawing exactly. The
subject's own inverse law fails, in its own words:

```
inverse-unflatten: undoing the mutation did not restore the drawing
     got: … digest=03571f4fbf608b4e7a40e4a647057dbc
expected: … digest=fae6a8d20aa5d6fc228b4c0a546a10ae
```

`Unflatten`'s inverse is `Flatten`, and flattening the REPLACEMENT does not bring the replaced node
back. The production vocabulary already knows this — the demo-variant list in
`✳️drawing/🧬️schema/🧬️mutations/🦀️component.rs:149` carries the comment *"`original` is a genuine
no-op restore (identical to the fixture's own node at this path, which has no nested groups) —
`flatten(original) == original` here, so the `unflatten` ↔ `flatten` inverse pair's own law holds
against the shared fixture"*. **That caveat is now a measured failure rather than a code comment**: for
any `Unflatten` payload the grammar admits but that arrangement does not cover, the verb neither
refuses the input nor captures the node it overwrites.

### 4d. One finding that went the other way — the REFERENCE was wrong, and it was fixed

The same first parity run turned `mutate-flatten` and `inverse-flatten` red with 1 322 and 1 320
differences. Diagnosis reversed the usual direction: **the independent implementation had read
`flatten` too broadly.** It dissolved every descendant group into leaves; the subject left the branch
untouched. Production says why, in a test rather than in prose —
`flatten_refuses_a_non_identity_descendant_group` asserts `m.inverse(&base).is_empty()` and
`m.diff(&base).diff().apply(&base) == base`, so `flatten` REFUSES the whole mutation when any
descendant group carries a transform other than the identity, because dissolving one would silently
drop the transform its children are drawn under. The committed vector's own slug says it too —
`flattens-an-IDENTITY-nested-group-into-its-leaves` — and the reference had read past it.

The reference was corrected to the specified refusal, and the case's `flatten` payload was retargeted
from the QR foreground (whose 329 descendant groups all carry `matrix(0.35,…)`, so `flatten` there is
a refusal that exercises nothing) to the mouse layer root, whose only descendant group is the identity
`clipPath` group — the one branch of this real drawing `flatten` can actually dissolve. That is not a
weakened assertion: it moves the scenario onto the input the verb is defined for, and the refusal case
is now stated in the feature as the reason.

### A second, smaller finding

`✳️drawing`'s committed JSON-schema mirror
(`✳️drawing/🧬️schema/📸️snapshot/🔣️component.json`) spells `PathSegment::ArcTo`'s fields `xRotation` and
`largeArc`. The real serde shape is `x_rotation` and `large_arc`: `#[serde(rename_all = "camelCase")]`
on an enum renames VARIANTS, not struct-variant FIELDS (that is `rename_all_fields`, which this enum
does not carry). Verified with a standalone `serde_json` probe rather than assumed. The mirror is
descriptive and says so, but it disagrees with the wire form an implementer would be reading it for.

---

## 5. Verification — real output

Every command was run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`; every exit code is the
tool's own, never a pipe's.

### contract — exit 1, and no breach names any of the four cases

```
$ bun ./📜️script.ts contract --owner 🗄️stdio                                   # exit 1
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery
  testing/discovery  🧰️framework  42 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s          4 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
```

`⚡️cache/breaches/testing.json` read directly: 2 records, both `unmanaged-tests`, scoped `🧰️framework`
and `✏️s` — the concurrent sessions' stray `.test.ts`/`.test.js` files that `📓️w13`'s trap 5 already
records. **Zero breaches name `mutate-semio-drawing`, `mutate-semio-mesh`, `mutate-semio-image` or
`mutate-semio-presentation`**, and the `testing/contract`, `testing/oracle`, `testing/fixture` and
`testing/taxonomy` families are all at zero.

### oracle — every case executes green in the oracle role

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-image         # exit 0
[test] level=exhaustive cases=1 executed=40 passed=40 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-presentation  # exit 0
[test] level=exhaustive cases=1 executed=46 passed=46 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-mesh          # exit 0
[test] level=exhaustive cases=1 executed=52 passed=52 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-drawing       # exit 0
[test] level=exhaustive cases=1 executed=52 passed=52 failed=0 errored=0 parity=0/0
```

Before this wave all four were `executed=0 … not-exercised=1`.

### parity — the number that matters

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-image --implementation rust
[test] level=exhaustive cases=1 executed=80 passed=80 failed=0 errored=0 parity=40/40      # exit 0

$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-presentation --implementation rust
[test] level=exhaustive cases=1 executed=92 passed=92 failed=0 errored=0 parity=44/46      # exit 1
[test] parity failed: …::mutate-set-snapshot::rust::subject (6 differences)
[test] parity failed: …::inverse-set-snapshot::rust::subject (6 differences)

$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-mesh --implementation rust
[test] level=exhaustive cases=1 executed=104 passed=103 failed=1 errored=0 parity=51/52    # exit 1
[test] parity failed: …::identity-round-trip::rust::subject (1 differences)
```

The one red line is §4b's finding, and the subject side names it exactly:
`exact-bytes law violated: the re-encoded output was required to reproduce the input, yet 188746
byte(s) out differ from 188746 byte(s) in (first at byte 4732)`.

The two red lines are §4's finding: exactly six differences, one per slide whose id did not move.

### `mutate-semio-drawing` parity — landed

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-drawing --implementation rust
[test] level=exhaustive cases=1 executed=104 passed=103 failed=1 errored=0 parity=51/52    # exit 1
[test] parity failed: …::mutate-semio-drawing::inverse-unflatten::rust::subject (1 differences)
```

The one red line is §4c's finding. §4d's correction is confirmed by this run: `mutate-flatten` and
`inverse-flatten`, which the FIRST parity run reported red with 1 322 and 1 320 differences because
the reference had read `flatten` too broadly, are both green now that the reference implements the
specified refusal and the payload addresses the one branch of this drawing the verb is defined for.

Its Rust subject host had to be built outside the runner first: the runner's own 900 000 ms cargo
budget was exhausted by target-directory contention from the concurrent sessions
(`[budget] cargo run … exceeded 900000ms — killed`), not by any compilation failure. `cargo build
--quiet --features sut` in that host directory exits 0 in 11 min 55 s with warnings only, after which
the parity run above completes normally. Recorded because the budget kill is a machine-load artefact
that reads like a case failure.

### Re-verification of all four against the tree as it stands

The four numbers above were produced at different points of a long session, and two of them predated
later edits (the `flatten` correction, and a projection change in `✳️presentation`). Every one was
therefore re-run against the current working tree, and every one reproduces:

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-image          # exit 0
[test] level=exhaustive cases=1 executed=40 passed=40 failed=0 errored=0 parity=0/0
$ … --case mutate-semio-presentation                                                     # exit 0
[test] level=exhaustive cases=1 executed=46 passed=46 failed=0 errored=0 parity=0/0
$ … --case mutate-semio-mesh                                                             # exit 0
[test] level=exhaustive cases=1 executed=52 passed=52 failed=0 errored=0 parity=0/0
$ … --case mutate-semio-drawing                                                          # exit 0
[test] level=exhaustive cases=1 executed=52 passed=52 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case … --implementation rust
image         executed=80  passed=80  failed=0 errored=0 parity=40/40                    # exit 0
presentation  executed=92  passed=92  failed=0 errored=0 parity=44/46                    # exit 1
mesh          executed=104 passed=103 failed=1 errored=0 parity=51/52                    # exit 1
drawing       executed=104 passed=103 failed=1 errored=0 parity=51/52                    # exit 1
```

`contract --owner 🗄️stdio` re-run at the same time: exit 1, the same **2** breaches, both
`unmanaged-tests` under `testing/discovery`, scoped `🧰️framework` (42 vs 35) and `✏️s` (4 vs 1).
`⚡️cache/breaches/testing.json` read directly — 2 records, and a substring search over the whole
record set for `mutate-semio-drawing`, `mutate-semio-mesh`, `mutate-semio-image` and
`mutate-semio-presentation` returns nothing for all four.

The three red scenarios are the three findings in §4, §4b and §4c, unchanged and still left red.

### dependency — unchanged

```
$ bun ./📜️script.ts dependency                                                 # exit 0
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
```

---

## 6. Traps hit that `📓️w13` does not record

1. **`Buffer.prototype.slice` is a VIEW, not a copy.** A TypeScript adapter that reads
   `ctx.fixtureBytes(...)` gets a Node `Buffer`; taking a `DataView` off `slice(a, b).buffer` reads
   from the shared pool's start, not from the slice. `parsePack` copies once with `Uint8Array.from`
   before doing anything. This produced a spectacular error message quoting the whole pack frame.
2. **`serde_json` prints an `f32` with the shortest decimal that round-trips as an `f32`.**
   `0.1f32` serialises as `0.1`, while the widened double prints `0.10000000149011612`. Because
   `ordered-json-v1` compares parsed NUMBERS, an oracle in a language with only doubles must route
   every single-precision leaf through a shortest-`f32` printer before projecting it — `Math.fround`
   plus a precision search in TypeScript, `struct.pack('<f', …)` plus the same search in Python. This
   bites `SemioRgba` (mesh, drawing) and `DrawStyle::opacity`. It also bites the CARRIER: a Python or
   TypeScript model that stores the widened double cannot survive its own pack frame's `f32` field.
3. **`#[serde(rename_all = "camelCase")]` on an enum does not rename struct-variant fields.** Already
   noted for `✳️image` in wave 12; §4's second finding is the same rule catching a committed mirror
   document out.
4. **A case's Rust adapter can carry a structural JSON decoder that covers less than the union.**
   `mutate-semio-presentation`'s decoder handled three of document's eight `DocBlock` kinds and
   panicked on `list` the moment the real payloads exercised the rest. Extended to all eight.
5. **`parity` still runs the oracle adapter in the SUBJECT role** — `📓️w13` trap 1, unchanged. All
   four runs use `--implementation rust`. A framework fix is still needed and was not made here.

---

## 7. Files

Per case: `component.feature` (rewritten), the new oracle adapter, `🦀️component.rs` (rewritten,
subject-only), the subset's `🧪️oracle/🔣️component.json` (`noOracleDecisions` removed, `oracles[]`
added) and a `🧫️fixtures/` directory holding the derived artifact and the per-kind payloads.

```
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/
  🧪️tests/mutate-semio-image/{component.feature, 🐍️component.py, 🦀️component.rs, 🧫️fixtures/}
  🧪️tests/mutate-semio-presentation/{component.feature, 🐍️component.py, 🦀️component.rs, 🧫️fixtures/}
  🧪️tests/mutate-semio-mesh/{component.feature, 🟦️component.ts, 🦀️component.rs, 🧫️fixtures/}
  🧪️tests/mutate-semio-drawing/{component.feature, 🐍️component.py, 🦀️component.rs, 🧫️fixtures/}
  🏅️standards/🔖️v1/🪆️subsets/✳️{image,presentation,mesh,drawing}/🧪️oracle/🔣️component.json
```

Derivation scripts and scratch drivers (ticket folder,
`w16-semio-drawing-mesh-image-presentation/`): `🐍️load.py`, `🐍️derive-image-artifact.py`,
`🐍️derive-presentation-artifact.py`, `🟦️derive-mesh-artifact.ts`, `🐍️derive-drawing-artifact.py`,
`🟦️smoke-mesh.ts`.

**Nothing else was edited**: no framework file, no shared stdio manifest, no `Cargo.toml`, no
`🔒️dependencies.json`, no comparison profile, no `ignoreKeys`, and no existing fixture was removed or
swapped.
