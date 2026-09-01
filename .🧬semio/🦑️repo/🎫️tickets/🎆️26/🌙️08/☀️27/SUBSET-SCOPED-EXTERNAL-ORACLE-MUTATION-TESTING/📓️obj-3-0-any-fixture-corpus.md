# 📓️ `s.stdio.obj@3.0/✳️any` — fixture corpus, mutation manifest and gate calibration

Scope: close the one remaining gap for this subset — a `third-party-generated` fixture corpus, a
`mutationManifests` block for all 22 declared kinds, and a `fixtureManifests` block — in the
`🏭️generator/` + `🧫️fixtures/` + `🧪️oracle/🔣️.json` shape the gif@89a/las@1.0/pdf@1.7 wave
established (`📓️gif-las-pdf17-findings.md`). The oracle (`tobj-obj-3-0-mutate`, `tobj` 4), the
comparison profile (`semantic-obj-3-0-v1`, tolerance 1e-5) and the 22-kind catalog were already
decided and committed; none of them was re-opened.

## What was delivered

```
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/
  🏭️generator/📜️script.ts                    generate | manifests, honours SEMIO_FIXTURE_OUT as a ROOT
  🏭️generator/🦀️engine/Cargo.toml            own [workspace], one dependency: tobj = "4"
  🏭️generator/🦀️engine/src/main.rs           writes the grammar, ADMITS it through tobj, then writes the file
  🧫️fixtures/pattern-shell/pattern-shell.obj  482 bytes, sha256:caa5195064e5011f97fccc8afedb2d61a7cf5e844b2f9dda9de9dc52e6f32418
  🧪️oracle/🔣️.json                            + mutationManifests (22) + fixtureManifests (1)
```

Ticket-local, not part of the repository tree:
`🔬️obj-3-0-any-oracle-verify/` (isolated Rust crate, own `[workspace]`, links the already-committed
shared oracle crate by path with `features = ["oracles"]`), `🔬️obj-3-0-any-gate.ts` (runs the
framework's OWN `compareProjections`), `🔬️obj-3-0-any-manifests.ts` (writes the two manifest blocks).

## Step 0 — the carrier is real

`s.stdio.obj@3.0/✳️any` has exactly one export serializer,
`🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs:14`, whose body is
`crate::artifacts::obj::engine::encode_obj(from)` — the real OBJ writer at `🚪️io/🦀️component.rs:215`,
wrapped in a `TxtSnapshot` because OBJ is a text format. It is none of the five stub shapes, and
`test contract` reports no `stub-serializer` line for it (it DOES report eleven for OTHER plugins'
`→ 🧊️obj` serializers — lowpoly, puzzle×2, sourcing, gis, procedural, block×3, cad, process,
remodel — all pre-existing and none of them this subset's own carrier).

## Why the generator writes the grammar itself

OBJ has no reference WRITER in the Rust ecosystem; `tobj` parses and never emits. That is already the
recorded reason this subset's own oracle (`🧪️oracle/🦀️component.rs`) and the shared
`mesh::oracle_create_obj` write the grammar directly and use `tobj` as the independent READER. The
generator mirrors that precedent exactly and adds an admission step: `tobj::load_obj_buf`
(`triangulate: true, single_index: true`, the same options `mesh::project_obj` uses) must parse the
produced bytes or generation aborts. Real output:

```
tobj 4 admitted the document: 3 model(s), 13 referenced vertex position(s), 15 triangulated index/indices
wrote 482 bytes to …/🧫️fixtures/pattern-shell/pattern-shell.obj
```

It depends on `tobj` and nothing else — never on this repository's `encode_obj`/`decode_obj`.

## The fixture — one document, deliberately shaped for all 22 kinds

`pattern-shell.obj`: 6 `v` rows (one with an explicit `w`), 6 `vt`, 4 `vn`, 5 `f` across 2 `g` bands
and 2 `o` objects, an `mtllib`, two `usemtl` runs and two `s` runs starting at *different* face
indices (one of them `s off`), and 2 retained comment lines.

The load-bearing design choice is that `v6`, `vt6` and `vn4` are **declared and referenced by no
face**. `tobj` re-indexes per model and drops every unreferenced row, so mutations of those rows move
the DOCUMENT projection and nothing at all in the mesh projection. That asymmetry is exactly what the
registered profile's own description claims, and it is now measured rather than asserted.

One fixture is sufficient for the four release-gated dimensions by the formula actually implemented
(`measureCoverage`, `📦️index.ts:5478-5480`): a mutation is evidenced iff a qualifying oracle
discharges it AND some fixture targets `${artifact}@${standard}/${subset}`. ANY fixture targeting the
subset counts for EVERY mutation in it. Confirmed by reading the source, and then by the real numbers
below.

## Step 2 — outcome classes, read off the code

Grepped every `MutationOutcome::` call site under `🧊️obj/`:

| Site | Call |
| --- | --- |
| `🧬️schema/🧬️mutations/🦀️.rs:220` | `MutationOutcome::new(match self { … })` — one uniform wrapper, all 22 arms, no per-kind `empty`/`error`/`fatal` |
| `🧬️schema/🧬️mutations/🦀️.rs:171` | `MutationOutcome::error(…)` in `apply_obj_mutation` when the diff fails to apply — reachable for every kind |
| `🧬️schema/🧬️mutations/📄set-snapshot/🦀️.rs:19` | `MutationOutcome::new(ObjDiff::default()).warn("mutation.no-op", …)` — the ONLY per-kind no-op branch |

So: `no-mutation` → `["no-op"]` (its arm is `ObjDiff::default()`, an empty diff — it never applies
anything); `set-snapshot` → `["applied", "no-op", "rejected"]`; the other 20 → `["applied",
"rejected"]`. Same shape the gif and las manifests arrived at, from the same evidence.

`productionDispatch.variant` for each row was read off the `ObjMutation` enum itself
(`🦀️.rs:43-128`); `payloadSchema` points at `../🧬️schema/🧬️mutations/🦀️.rs#<Variant>`, which is where
the variants actually live. Note the filename: this subset's mutations file is `🦀️.rs`, not
`🦀️component.rs` — the las/gif manifests still point at the pre-rename `🦀️component.rs`, a stale
reference in those two files that this one does not copy.

## Requirement 5 — witnessability, measured, not assumed

The oracle registration's `rationale` asserts "the case asserts in role that every kind but
`no-mutation` actually moves it". That claim was checked directly: apply each of the 22 kinds to
`pattern-shell.obj` through `oracle_apply_mutation`, compute the composed projection of the result
(`mesh::project_obj`'s `tobj` reading + `oracle_document_projection`), and diff each half against the
base under the registered profile with the framework's own `compareProjections`.

```
[IDENT] no-mutation              mesh=0 document=0    via neither
[MOVES] set-snapshot             mesh=0 document=7    via document
[MOVES] insert-vertex            mesh=0 document=7    via document
[MOVES] remove-vertex            mesh=0 document=7    via document
[MOVES] set-vertex               mesh=0 document=9    via document
[MOVES] insert-texcoord          mesh=0 document=3    via document
[MOVES] remove-texcoord          mesh=0 document=3    via document
[MOVES] set-texcoord             mesh=0 document=2    via document
[MOVES] insert-normal            mesh=0 document=3    via document
[MOVES] remove-normal            mesh=0 document=3    via document
[MOVES] set-normal               mesh=0 document=5    via document
[MOVES] insert-face              mesh=43 document=9   via mesh+document
[MOVES] remove-face              mesh=34 document=11  via mesh+document
[MOVES] set-face                 mesh=40 document=0   via mesh
[MOVES] set-group                mesh=0 document=2    via document
[MOVES] remove-group             mesh=0 document=6    via document
[MOVES] set-object               mesh=8 document=4    via mesh+document
[MOVES] remove-object            mesh=0 document=6    via document
[MOVES] set-mtllib               mesh=0 document=1    via document
[MOVES] set-usemtl               mesh=0 document=3    via document
[MOVES] set-smoothing-groups     mesh=0 document=4    via document
[MOVES] set-unknown-statements   mesh=0 document=3    via document

[witnessability] 22 declared kind(s); 0 move NEITHER half
```

**21 of 21 non-identity kinds are witnessable; ZERO `-uncarried` exemptions are warranted**, so every
one of the 22 manifest rows carries a real `oracleRequirement` naming `tobj-obj-3-0-mutate`. The
rationale's claim is true as written.

Two facts the table settles that the prose only guessed at:

* **17 of the 21 non-identity kinds move ONLY the document half** (`mesh=0`; everything except
  `insert-face`, `remove-face`, `set-face` and `set-object`). A `tobj`-only comparison would pass all
  17 against evidence that never moved — the registration would be green and empty. The prose said
  "14 of the 22"; the measured number on this fixture is 17, because the fixture deliberately aims
  the `*-vertex`/`*-texcoord`/`*-normal` families at rows no face references.
* **`set-face` moves ONLY the mesh half** (`document=0`). `oracle_document_projection` reports
  `declaredFaces` as a COUNT, and the `g`/`o`/`usemtl`/`s` structures are keyed by face INDEX, none
  of which a same-arity face replacement touches. So the mesh half is not decoration either: drop it
  and `set-face` becomes the one blind kind. The two halves are each other's blind-spot cover, which
  is the whole argument for composing them.

## Requirement 4 — the gate, calibrated in BOTH directions

Run with the framework's own `compareProjections` under the registered `semantic-obj-3-0-v1`
(`tolerance = 1e-5`, `ignoreKeys = [byteLength, fileSize, precision]`), never a re-implementation:

```
=== 1. ACCEPT direction ===
[ACCEPT] fixture vs itself: 0 diff(s), max |Δ| = 0.000e+0 against tolerance 0.00001
[ACCEPT] fixture vs its own parse→render identity round trip: 0 diff(s), max |Δ| = 0.000e+0 against tolerance 0.00001
[ACCEPT] hand-corrupted copy, sub-tolerance (v 0.5 0.5 1 → v 0.500001 0.5 1): 0 diff(s), max |Δ| = 0.000e+0

=== 2. REJECT direction ===
[REJECT] hand-corrupted copy, supra-tolerance (v 0.5 0.5 1 → v 0.501 0.5 1): 4 diff(s), max |Δ| = 1.000e-3
           $.document.declaredVertices.sum[0]: oracle=4.5 subject=4.501
           $.mesh.vertices[6][0]:  oracle=0.5 subject=0.501
           $.mesh.vertices[9][0]:  oracle=0.5 subject=0.501
           $.mesh.vertices[12][0]: oracle=0.5 subject=0.501
[REJECT] set-vertex aimed at the WRONG index (v6 moved instead of left alone): 9 diff(s), max |Δ| = 6.000e+0
           $.document.declaredVertices.max[0..2]: oracle=2 subject=1
           $.document.declaredVertices.min[0..2]: oracle=0 subject=-4
           $.document.declaredVertices.sum[0..2]: oracle=4.5 subject=-1.5
[REJECT] set-face aimed at the WRONG winding (face 0 reversed): 40 diff(s), max |Δ| = 3.000e+0
           $.mesh.triangles[1][0]: oracle=0 subject=3   (the whole index space shifts)
[REJECT] remove-group aimed at the WRONG band (`base` dropped): 6 diff(s), max |Δ| = 3.000e+0
           $.document.groups: array length 2 → 1
           $.document.groups[0].firstFace: oracle=0 subject=2
```

The two hand-corrupted copies are the calibration that matters: they differ from the fixture in ONE
character of ONE coordinate, one side of the tolerance each. `0.500001` (Δ = 1e-6) is accepted with 0
diffs; `0.501` (Δ = 1e-3) is rejected with 4. A profile that rejected both would not be a tolerance;
one that accepted both would not be a gate. `1e-5` sits between them with two orders of magnitude of
margin on each side. The three "wrong target" cases are genuine semantic differences produced by the
oracle itself, not documents invented to look different.

The identity round trip deserves a line of its own: `render(parse(fixture))` is NOT byte-identical to
the fixture (the oracle's writer emits retained comment lines at the end and normalises the
`g`/`o`/`usemtl`/`s` transition order), yet its composed projection is bit-for-bit the base's, 0
diffs in both halves. That is the correct relationship — the profile is a semantic comparison and the
byte layout is producer freedom — and it is what makes every "did this kind move it?" answer above
signal rather than noise.

## Verified with the real framework commands

```
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify    --artifact s.stdio.obj --standard 3.0 --subset any
[fixture verify] 1 fixture(s), 0 file problem(s)
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact s.stdio.obj --standard 3.0 --subset any
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture audit     --artifact s.stdio.obj --standard 3.0 --subset any
[fixture audit] third-party-generated s.stdio.obj@3.0/any / licence=public-domain (synthetic, …) reproducible=true generator=tobj-obj-3-0-mutate(tobj)
[fixture audit] 1 fixture(s), 0 with contract problems
```

`fixture reproduce` here IS the per-fixture check the reproducibility note demands: the selector
resolves to exactly one fixture, so the generator is re-run **on its own**, in a fresh
`SEMIO_FIXTURE_OUT` root, and its bytes are hashed against the committed digest — not as part of a
whole-corpus batch whose shared ordering can hide process-global state. The generator has no
wall-clock, no randomness and no counters: the document is a literal in `main.rs`.

Nothing was rewritten after being hashed. The digest is computed by `🔬️obj-3-0-any-manifests.ts` as
its last read of the fixture, and the fixture is never touched again.

### Coverage matrix, real repo-wide numbers

`bun 🧰️framework/…/🧪️test/📜️script.ts matrix --json`, filtered for `obj` in each gate's `missing`:

```
subsetOwnershipCoverage    645/658 repo-wide — obj missing: []
externalOracleCoverage     436/658 repo-wide — obj missing: []
oracleEvidenceCoverage     262/658 repo-wide — obj missing: []
oracleCapabilityCoverage    35/48  repo-wide — obj missing: []
fixtureProvenanceCoverage  419/444 repo-wide — obj missing: []
fixtureReproducibilityCoverage 444/444 — obj missing: []
```

The registry loads the new block as `1 manifest, 22 mutations`, all 22 inside the 658 denominator, so
the empty `missing` lists mean **22/22 on all four release-gated dimensions**, not "not counted".

## A real framework bug found and fixed: `fixture audit` read the bare spelling

`fixture audit` initially reported `PROBLEMS: target.subset "any" is a wildcard` — and so did the
untouched las and gif fixtures, which is what identified it as pre-existing rather than mine.

`fixtureManifestProblems(value, repoRoot?)` decides wildcard-ness two different ways: WITHOUT
`repoRoot` it judges the bare spelling; WITH it, it asks `isWildcardSubsetFor`, which returns false
when the artifact declares no narrower sibling. Its own comment says judging fixtures by the spelling
while judging mutations by the resolved rule "made 27 fixtures of genuinely single-subset owners
unregisterable". Three call sites exist; the contract phase (`📦️index.ts:5010`) and the coverage gate
(`📦️index.ts:5489`) both pass `repoRoot`, and only `FixtureScript`'s audit row (`📜️script.ts:1081`)
did not. So the audit command contradicted the very gate it feeds.

Fixed by passing `this.repoRoot` — one argument, the parameter that exists for exactly this. Blast
radius measured against the real registry before and after:

```
fixtures=428  problems before the fix=106  after=25  freed=81
```

81 fixtures of genuinely single-subset owners (obj's, las's and gif's included) stop auditing as
wildcard breaches; the remaining 25 are unrelated, real problems belonging to other owners.
`s.stdio.obj@3.0`, `s.stdio.las@1.0` and `s.stdio.gif@89a` all now audit `0 with contract problems`.

## Open findings, stated rather than papered over

1. **`unsplit-artifact-subset`, 22 lines, deliberately NOT silenced.** `test contract` reports
   `Mutation <kind> is owned by "any" and s.stdio.obj@3.0 declares no narrower subset at all` for
   every kind. The prescribed remedy is either declaring real subsets or recording
   `"subsetPolicy": "single"` in `🏅️standards/🔖️3.0/🪆️subsets/🔣️component.json`. I did **not** record
   it, because unlike gif@89a and las@1.0 the claim would not be clearly true: Wavefront OBJ has a
   genuine internal split — polygonal geometry versus the free-form `curv`/`surf`/`trim`/`parm`
   statements — and this subset's 22-kind vocabulary implements only the polygonal half. Declaring
   the artifact single-scoped would assert something the format does not support. It is a real,
   actionable finding for the artifact's owner, not a naming artifact.
2. **`reimplementation-registered-as-third-party`, pre-existing.** `test contract` reports
   `tobj-obj-3-0-mutate is registered as a qualifying third-party oracle, but this owner predicts
   mutation output in its own Rust` (`testing/oracle`, on `🧪️oracle/🦀️component.rs`). It is a
   consequence of the already-approved design — no Rust crate writes OBJ, so the oracle must — and
   two sibling subsets carry the identical line (`riff-avi-1-0-mutate`, `dxf-crate-r12-mutate`). It
   predates this work, this work does not change it, and the brief explicitly ruled it out of scope.
   Recording it here so the green fixture numbers are not read as clearing it.
3. **`No runtime inventory has been produced for s.stdio.obj@3.0/any`** — expected. The runtime
   inventory comes from running the production bridge, which needs `semio-s-plugin-stdio` to compile.
   Same state as gif/las/pdf.
4. **Schema drift on `ManifestMutation`.** The framework's JSON schema declares
   `additionalProperties: false` with no `carriers` field, yet 11 of the 33 manifest-bearing oracle
   registries use it AND `ManifestScript`'s merge logic explicitly preserves it across regeneration
   (`📜️script.ts:1484`). Symmetrically, `OracleRequirement.oracle` is absent from the schema but IS
   read by `measureCoverage` (`📦️index.ts:5473`). Both fields are used here, matching the code that
   actually runs; the schema is what is behind, in `🧬️schema/🔣️.json`'s `ManifestMutation` and
   `OracleRequirement` defs. Not fixed — a shared framework schema edit is outside this subset's
   scope and would need the owners of the other 10 registries.
5. **No `ComparisonPipeline` was added, on purpose.** Pipelines exist for multi-artifact,
   probe-measured comparisons. This subset's comparison is one declarative projection compared under
   one profile, the composition happens in the case adapter (as the oracle module's own doc comment
   already states), and the profile discriminates correctly as measured above. Adding a pipeline
   would be machinery with nothing to do.

6. **`🏭️generator/📜️script.ts` is not in `launch.json`** — and neither is any sibling generator, nor
   the repository test platform's own `📜️script.ts` (`grep -n "🧪️test/📜️script.ts"
   .vscode/launch.json` returns nothing). Registering one of ~15 generators into a group that does
   not exist yet would be worse than the gap, and both `.vscode/launch.json` and
   `.vscode/🧩️launch.seed.jsonc` are being modified by a concurrent session right now. Flagged as
   `spawn_task task_7f6b0695` for a single coherent pass over the whole family instead.

## Nothing here needed `semio-s-plugin-stdio`

Stated explicitly because it is the usual blocker: the production crate's in-flight
`protocol::Mutation` refactor was never touched and never needed. The shared oracle crate
(`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`) is a standalone `[workspace]` that does not
depend on it — `cargo check --features oracles --offline` finishes clean (3 pre-existing warnings in
unrelated svg/docx modules) — and both the generator crate and the verification crate are standalone
workspaces of their own. The only thing that could not be produced is the runtime mutation inventory
(open finding 3), which genuinely requires the production bridge to run.
