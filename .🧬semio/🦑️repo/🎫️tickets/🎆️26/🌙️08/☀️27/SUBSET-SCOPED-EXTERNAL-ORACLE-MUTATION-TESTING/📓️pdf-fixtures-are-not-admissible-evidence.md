# 📄️ pdf — 120 mutations blocked by the FIXTURES, not by the oracle registration

`pdf` is the single largest block in the uncovered 227: ten subsets, roughly 120 mutation kinds, all
but one registered `cross-semio-implementation`. The obvious reading is that they simply await the
same reader-oracle retrofit that closed `obj`, `dxf`, `gif@89a`, `svg`, `xml`, `png`, `jpg`, `bmp`,
`tiff` and `gltf`. **They do not.** The blocker is one level deeper and the retrofit pattern does not
reach it.

## What is already there

| Subset | Fixtures | Pipelines | Probes | Oracle kind |
|---|---|---|---|---|
| 1.7/vt | 18 | 0 | 0 | cross-semio-implementation |
| 1.7/a | 14 | 0 | 0 | cross-semio-implementation |
| 1.7/x | 14 | 0 | 0 | cross-semio-implementation |
| 1.7/e | 12 | 0 | 0 | cross-semio-implementation |
| 1.7/base | 1 | 0 | 0 | cross-semio-implementation |
| 1.4/base | 0 | 0 | 0 | **third-party-library** |
| 1.4/a, 1.4/x, 1.7/h, 1.7/ua | 0 | 0 | 0 | cross-semio-implementation |

59 fixtures exist. **Zero probes and zero comparison pipelines exist anywhere under `pdf`** — so
nothing currently judges those 59 fixtures at all. `1.4/base` is registered `third-party-library`
while carrying no fixtures, no probes and no pipeline; that entry discharges nothing and its kind is
flattering.

The fixtures look exactly right at first glance: committed `base.pdf` + `mutated.pdf` pairs, sha256
recorded, `provenance.source: generated`, `generator.engineFamily: lopdf`, `packageVersion: 0.44`,
and lopdf 0.44.0 is genuinely vendored in the cargo registry cache.

## Why they are not admissible

`🏭️generator/🦀️engine/src/main.rs:36`:

```rust
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_7::subsets::vt::{
    oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip,
    project_conformance, KINDS,
};
```

That is **this repository's own crate**. In the generation loop:

- `mutated.pdf` is `oracle_apply_mutation(&base, &forward)` — **our code writes the after state**.
- `expected-projection.json` is `project_conformance(&mutated)` — **our code predicts how it reads**.

lopdf appears only as `use lopdf::{dictionary, Document, Object, Stream}` — used to lay out the seed
document and to serialise. The `Cargo.toml` states the arrangement plainly: it "links the
repository's own already-qualified `semio-s-plugin-stdio-test-oracle` crate's
`document::pdf_conformance` engine (the SAME code the differential test case ... drives)".

So both halves of the comparison descend from one implementation. lopdf is a **codec for our own
answer**, which is the precise definition of a predicting oracle: a shared misreading of the PDF
specification yields two agreeing wrong answers and the test still passes. Registering a lopdf
"reader" beside this and pointing `oracleRequirements` at it would change the label and nothing else —
the *expected state it reads* would still be a state we computed.

**The `cross-semio-implementation` classification on these nine subsets is therefore correct, and was
not an oversight to be undone.**

## What the generator does establish

It is not worthless — it just proves supplemental properties rather than qualifying ones:

- **observability** — `mutated_projection != base_projection`, so every mutation is visible in the
  conformance projection (a *metamorphic* property);
- **inverse restoration** — `oracle_inverse_spec` then `oracle_apply_mutation` returns exactly the
  base projection (an *inverse* property);
- **round trip** — lopdf reparses and re-serialises the whole COS graph; bytes must change while the
  projection must not.

Under Protocol v2 all three are *supplemental*. None discharges the external-oracle requirement.

## What closing pdf actually requires

The `after` bytes must be produced by something that is not us. Concretely, per subset:

1. **Rewrite the generator to apply each mutation through lopdf's own API** — `set-trim-box`,
   `set-dpart-root`, `insert-encryption-dictionary`, `remove-output-intent` and the rest are all COS
   dictionary edits well within lopdf's public surface. Our `oracle_apply_mutation` must not appear.
2. **Add a reader** that projects both files through lopdf and compares — the `import` / `project` /
   `compare` probe triple used by every other retrofitted subset.
3. **Register** the reader oracle, its probes and a comparison pipeline, and repoint each mutation's
   `oracleRequirements`, recording genuinely unreachable kinds as `-uncarried`.

Writer and reader both being lopdf is consistent with the accepted precedent (`gif::Encoder` writes,
`gif::Decoder` judges; likewise png, bmp, tiff). What is *not* acceptable is the current arrangement,
where the writer is ours.

Step 1 is the real cost and it is per-subset, because each subset's mutation vocabulary differs.
This is generator work, not registration work — which is why pdf did not fall to the wave that
closed the other ten artifacts.

## Consequence

`pdf`'s ~120 mutations remain honestly uncovered. No `-uncarried` entry is claimed for them, because
lopdf demonstrably *can* witness these kinds — the gap is on the writer side of the fixture, not the
reader side. Any future status line reporting pdf as retrofitted should be checked against
`🏭️generator/🦀️engine/Cargo.toml`'s dependency list first: if
`semio-s-plugin-stdio-test-oracle` is still there, the fixtures are still ours.

---

## Pinned as a gate

`fixtureWriterProvenanceBreaches(repoRoot, registry)` was added to the shared library beside
`reimplementationOracleBreaches` and `stubSerializerBreaches`, and is exercised by four harness checks
(116 → **120**).

It keys on the disqualifying **symbols** — `oracle_apply_mutation`, `oracle_apply`, `apply_mutation`,
`project_conformance`, `oracle_inverse_spec`, `oracle_round_trip` imported from a `semio_*` crate —
rather than on the presence of any `semio-*` dependency. That distinction is deliberate: pdf's
generators also depend on `semio-repo-test-host`, a JSON helper, and flagging on that would make the
gate indistinguishable from one that fires on the mere existence of a generator. Gates that cannot
tell those apart are the ones people learn to ignore.

Validated in both directions, which is what makes the result meaningful:

| Check | Result |
|---|---|
| `fixture-writer/detects-our-own-engine` | flags exactly **4** generators |
| `fixture-writer/names-pdf` | all four populated pdf 1.7 subsets named |
| `fixture-writer/no-false-positives` | **0** of the 11 reader-oracle subsets flagged |
| `fixture-writer/base-subset-is-clean` | `pdf 1.7/base`, whose generator is clean, is not flagged |

The no-false-positives check is the load-bearing one. Every subset retrofitted this ticket — `obj`,
`dxf`, `gif@89a`, `svg`, `xml`, `png`, `jpg`, `bmp`, `tiff`, `gltf`, `avi` — comes back clean, which is
independent confirmation that their fixtures really are written by third-party libraries and not just
labelled that way. The wave's result survives a check it did not design itself.

Consistent with its two siblings, the gate is surfaced through the ticket harness rather than wired
into `validateAllContracts`; changing where these three surface is a repo-gating decision, not a
finding.
