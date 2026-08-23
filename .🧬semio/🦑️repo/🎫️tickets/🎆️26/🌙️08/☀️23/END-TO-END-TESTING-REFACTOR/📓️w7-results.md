# Wave 7 results — exhaustive real-world mutation round-trips

Every case below applies **every mutation kind its subset declares** to a **real-world artifact**,
performs it first with the registered third-party reference implementation, and compares the result
through an independent reader. Each kind is exercised twice — applied, and inverted — plus a
full decode/re-encode identity scenario that forbids byte pass-through.

Verified with `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case <case>` from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

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

Two real defects in the `gif` 0.13 reference library came out of it, each reproduced standalone
before being worked around, and each documented in the oracle module rather than hidden by loosening
the projection:

1. `gif::Encoder::new` unconditionally sets the global-colour-table flag and writes a minimum
   two-entry padding table even when the palette is empty, with no way to omit it through the public
   API. The phantom table is stripped from the output when the snapshot declares no GCT.
2. `gif::Decoder` always de-interlaces on read and resets `interlaced` to `false`, while
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
