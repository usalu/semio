# 🧪️ The evidence gap: 84 → 18, and what it was actually made of

`oracleEvidenceCoverage` — "mutations that have BOTH a qualifying oracle and a fixture to run it
against" — sat at 574/658. None of that gap needed the blocked `semio-s-plugin-stdio` build, which is
why it was worth attacking rather than waiting.

| | before | after |
|---|---|---|
| manifested mutations | 658 | **614** (44 were phantom, see below) |
| oracleEvidenceCoverage | 574/658 (87.23%) | **596/614 (97.07%)** |
| externalOracleCoverage | 644/658 (97.87%) | **600/614 (97.72%)** |
| fixtures | 862 | **884** |

## 50 of the 84 were a defect of mine, not a missing fixture

`fem` reported 25 + 25 mutations with no evidence while its subsets held 34 and 37 fixtures. The cause:
the catalogs each carried **two** `mutationManifests` for one subset, differing only in artifact id —

* `mutationManifests[0]` — `s.fem.fem2d`, which **I added**, naming the real oracle
  (`serde-json-fem2d-carrier-reader`, capability `fem2d-1-mutate-carrier`);
* `mutationManifests[1]` — `s.fem.2d`, pre-existing, requiring `fem2d-1-mutate` with no oracle named.

Both listed the same 25 kinds, so the matrix counted 50 rows twice over, and my fixtures — targeted at
`s.fem.fem2d` — were unreachable from the `s.fem.2d` rows that the taxonomy declaration
(`🪆️subsets/🔣️component.json`) and the pre-existing manifest both use. A repo-wide sweep confirmed
**these two were the only such pairs**; the drift was entirely mine.

Merged onto the canonical `s.fem.2d`/`s.fem.3d`, keeping the richer entries **and carrying both
`oracleRequirements` forward** — dropping `[1]`'s would have deleted a still-owed third-party
requirement and turned a real gap into a fake green. Fixtures retargeted to match.

## 6 mutations were invisible, and making them visible made the number worse

`✳️drawing`'s manifest listed 11 kinds while the subset has 17 — the six leaves renamed earlier
(`rotate-node`, `scale-node`, `group-nodes`, `ungroup-node`, `flatten-node`, `unflatten-node`) were
never registered, so they appeared in neither the numerator nor the denominator. That is the exact
failure this ticket's own `coverage/untested-appears-as-missing` check exists to forbid. Registered —
which correctly **raised** the gap from 34 to 40 before any of it was closed.

## 22 fixtures written by third parties, one bound per mutation

* **`✳️drawing` — 17 kinds, `🏭️generator/🦀️svg-engine` (quick-xml 0.37.5).** SVG is the one carrier
  that natively represents BOTH things this vocabulary edits: an element `transform`
  (rotate/scale/translate) and `<g>` nesting (group/ungroup/flatten/unflatten). Each fixture is an
  authored `(before.svg, after.svg)` pair — the pair IS the expectation, quick-xml serialises it and
  `quick-xml-drawing-svg-reader` parses both halves. Nothing applies one of our mutations, which is
  what keeps it a reader oracle rather than a predicting one. Every pair was checked to actually
  differ (1–6 changed lines each); a pair whose halves are equal proves nothing.
* **`🔣️json@rfc8259/✳️base` — 5 kinds, `🏭️generator/🦀️serde-json-engine` (serde_json,
  `preserve_order`).** The artifact IS JSON, so the carrier is the artifact. `preserve_order` is on
  deliberately: `set-member`/`remove-member` are only observable as member-set changes if member order
  survives the round trip. The engine `assert_ne!`s each pair.

Both engines are standalone `[workspace]` crates depending on one third-party library and nothing
else — they build while `semio-s-plugin-stdio` does not, by construction.

## The harness caught my own manifests

Registering the 17 drawing fixtures took the suite to **118/119**:
`fixture/provenance-failures-are-named` reported all 17 as malformed — `units` must declare length and
angle, and `comparisonProfile` must be a non-empty **string**, not the array I wrote. Corrected in the
generator (so regeneration stays clean) rather than patched in the output. **119/119** after.

## A weakness in the metric, found while closing it — recorded, not exploited

`withoutEvidence` keys on the **subset**:

```ts
.filter(({ manifest, mutation }) => oracled.has(...) || !fixtureSubsets.has(`${manifest.artifact}@${manifest.standard}/${manifest.subset}`))
```

So **one** fixture anywhere in a subset marks every mutation in that subset as evidenced. That is the
"empty denominator" failure this protocol already forbids at the dimension level and then again at the
mutation level, reappearing one level further down at *fixture binding*. It would have let a single
committed file close all 17 drawing kinds at once.

It was not used that way: the drawing and json corpora bind **one fixture per mutation**, 17 and 5
respectively. But the metric does not require that, and `fem` proves the gap is real — `create-region`,
`delete-region` and `replace-region` have no fixture bound to them individually yet count as evidenced
because their subset has others. **The next protocol refinement is a per-mutation binding dimension**;
until it exists, `oracleEvidenceCoverage` should be read as "the subset has been exercised", not "this
mutation has been".

## What is left: 18, and all of it is blocked

`mathematical` 9 + `sequence` 8 (both crates depend on the un-building stdio; their TypeScript packages
are WASM facades built from that same Rust) and `jpg::remove-huffman-table` (no JPEG marker writer).
