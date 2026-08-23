# Wave 7 — mutate-bcf-2-1 (BCF 2.1 / ✳️any)

Subset assignment: `💬️bcf` standard `🔖️2.1` subset `✳️any`. Oracle composes `zip` 6 + `quick-xml`
0.42 over the real BCF-XML 2.1 `markup.xsd`/`visinfo.xsd` shapes — no standalone BCF crate exists
in the Rust ecosystem (only bundled inside larger MPL-licensed IFC toolkits), so this composition
IS the oracle, per the fleet brief's own instruction.

## Finding: the committed example fixture is a placeholder

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/💬️example.bcf`
is **0 bytes** — a placeholder, not a real fixture. Not used by this case. No real BCF export (from
a coordination tool such as Solibri/BIM Collab/Navisworks) exists anywhere in this repository.

## Derived fixture

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🧫️fixtures/wellness-center-coordination-review.bcf` (248 KB,
9 ZIP entries: `bcf.version`, 3 topic folders' `markup.bcf`, 2 `.bcfv` viewpoints, 2 PNG snapshots,
1 `project.bcfp` raw part) was derived once, honestly, by
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/bcf-2-1-mutate/derive_fixture.py`,
from two real sources:

1. **Every IFC GUID and element name** referenced by a viewpoint's component selection/visibility/
   coloring, or quoted in a topic's title/description, was grepped directly out of the real,
   committed 21 MB IFC2X3 export `temp/wellness-center-sama.ifc` (Autodesk Revit 2021 (ENU),
   IFC2X3, exported 2021-11-21). Used: wall `0HG2A49bzDARlPHy2ZDHwJ`, column
   `0PfeWE7Aj7GBHCsLa67379`, door `2JJqxZjqn96xzCFMbZMpfb`, curtain-wall mullion
   `2lrUU8Tqz92AICLQu1TLwD`, storey `0a3v3dJi10mxIqGCVATOEH` ("First floor"), project
   `0a3v3dJi10mxIqGCSrYdxN`. Two further real elements (a slab `0ZRQUHwuv8SOaxY18jH6Mu` and a second
   column `1eu4XPbTzBchVWRGaY34FW`) are reserved for the `insert-topic`/`set-snapshot` mutation
   scenarios so those kinds introduce genuinely new real content rather than repeating the base
   fixture.
2. **Every viewpoint snapshot is real pixel data**: one topic's snapshot is the real, byte-for-byte
   unmodified 244 KB committed floor plan
   `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png`; the other is a
   real 64×64 crop of the same PNG (182 bytes). The mutation scenarios' own binary-payload params
   (`insert-viewpoint`, `insert-topic`'s nested viewpoint, `set-viewpoint-snapshot`) reuse that crop
   or a second real 64×64 crop of the same source at a different offset (561 bytes) — never
   synthetic filler.

Topic/comment/viewpoint GUIDs (the `3b1f6a1e-...` series) are fresh identifiers this derivation
minted, exactly as any real BCF-writing tool mints them for its own review metadata — never IFC
entity identity, so minting them does not compromise the "real GUIDs" claim, which is about the
`IfcGuid`s a real tool would also have copied from the model. Camera positions are ordinary BCF
viewpoint metadata (a notional viewer's position), the same kind of number every real BCF export
carries; they describe a viewing pose, not building geometry, so they carry no separate
"real-world" claim.

**Honesty**: this is a derived package, not a real coordination-tool export. Every reviewer name/
date/text was authored by this derivation (`ueli.saluz@iek.uni-hannover.de`, 2026-08-23), not
recovered from a real review. What is real is the IFC element identity/geometry data and the
snapshot pixels, not the review narrative itself. This is stated in the Feature file's own
description, not only here.

## Files created/modified

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` —
  filled in the stub dispatcher: independent `zip`+`quick-xml` composition (own `BNode` XML tree,
  own `ODoc`/`OTopic`/`OComment`/`OViewpoint`/`OCamera`/`OComponents` model, own archive read/write),
  covering all 14 declared `BcfMutation` kinds forward + inverse, plus `project_bcf_2_1`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  (new) — oracle registration (`zip-quick-xml-bcf-2-1-mutate`, ecosystem `rust`) and the
  `bcf-2-1-any` mutation catalog (14 kinds), `semantic-bcf-v1` comparison profile.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS` (14 kebab-case entries, declaration order) and a new
  `#[cfg(test)] mod kinds_tests` with `kinds_const_matches_enum_variants_in_declaration_order`,
  asserting `KINDS` against `print_bcf_mutation`'s own keyword order. No other change to this file
  (the `BcfMutation` enum itself pre-existed this ticket, confirmed 14 variants).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🧪️tests/mutate-bcf-2-1/component.feature` (new) — 14-row
  `Scenario Outline` for `@id-mutate`/`@id-inverse` (28 scenarios) plus one `@id-identity-round-trip`
  scenario (29 total).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🧪️tests/mutate-bcf-2-1/🦀️component.rs` (new) — adapter:
  oracle handlers calling into the oracle crate; `#[cfg(feature = "sut")] mod subject` driving
  `decode_bcf`/`encode_bcf`/`apply_bcf_mutation` over the real `BcfMutation` vocabulary, with a
  closed-form `inverse_of` (same precedent as `mutate-pdf-1-7`/`mutate-xml-1-0`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🧫️fixtures/wellness-center-coordination-review.bcf` (new,
  committed) — the derived real-content fixture described above.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/bcf-2-1-mutate/derive_fixture.py`
  (ticket-folder scratch) — the exact, re-runnable derivation script.

Not touched: `Cargo.toml`, `📦️lib.rs`, any other artifact, the framework, `.gitignore`,
`project.json`, `launch.json`.

## Verification (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`)

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-bcf-2-1
0 high-priority breach(es) across 0 rule(s):
full breach set (including non-blocking priorities): .../⚡️cache/breaches/testing.json
```

Confirmed the full breach cache (repo-wide, 0 entries after this run) contains zero occurrences of
the substring "bcf" anywhere — no breach names this subset.

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-bcf-2-1
[test] level=exhaustive cases=1 executed=29 passed=29 failed=0 errored=0 parity=0/0
```

29/29 = 14 kinds × (mutate + inverse) + 1 identity-round-trip, re-run twice for stability, both
green. Spot-checked real projections from
`.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-stdio-artifacts-bcf-cedb79-mutate-bcf-2-1-oracle-rust/`:
`mutate-remove-topic` leaves exactly the two other topics plus `project.bcfp`;
`mutate-insert-viewpoint` shows both the original and the newly-inserted viewpoint guid under topic
B; `mutate-set-comment` shows `viewpointRef` correctly cleared to `null` via the tri-state grammar;
`identity-round-trip`'s re-encoded bytes (248,790 bytes) are confirmed NOT byte-identical to the
248,124-byte input.

**Rust SUBJECT phase**: not compiled (expected — concurrent os-kernel refactor, per fleet brief).
The subject module is written in full and `sut`-gated; oracle-only verification above is what the
brief asked for.

## Deviations / notes

- `BcfMutation` already declared 14 variants before this ticket (vocabulary file pre-existed, per
  the brief's own table); this session only added `KINDS`/`kinds_tests` beside it, per fleet-brief
  §1, and wrote the oracle module, catalog, and case from scratch.
- Binary payloads (viewpoint PNG snapshot, raw part bytes) travel through mutation JSON params as
  lowercase hex, reusing the same convention `BcfSnapshot::parse_dsl`/`print_dsl` already use for
  this artifact's own whole-document DSL form — avoids a new base64 dependency this artifact does
  not otherwise need.
- A viewpoint's snapshot projects under `semantic-bcf-v1` as `{size, digest}`, not raw bytes,
  matching the fleet brief's raster precedent for opaque binary payloads.
- No shared family module exists for BCF (unlike document/raster/archive); the `zip`+`quick-xml`
  composition is specific to this one subset's container shape and lives entirely in its own oracle
  file, per the brief's own note that BCF has no shared family helper.
