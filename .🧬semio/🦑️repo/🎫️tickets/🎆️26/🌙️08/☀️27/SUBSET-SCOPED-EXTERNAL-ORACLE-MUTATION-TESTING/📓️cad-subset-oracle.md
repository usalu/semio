# 📓️ `semio@v1/cad` — external-oracle qualification

Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad`
Capability `semio-v1-cad-mutate` · artifact `s.stdio.semio` · standard `v1` · subset `cad` · 16 kinds.

Everything below was read or executed, not inferred. Where a prior pass's claim turned out wrong it is
corrected in place with the evidence that overturned it.

---

## 1. Carriers — what is real, verified by reading the bodies

Three export leaves exist under
`✳️cad/🚪️io/📤️export/🧵️serializers/🗿️artifacts/`. All three were read in full.

### 1.1 `step` (ap214) — REAL. The earlier "stub" flag was a FALSE POSITIVE, confirmed.

`.../📐️step/🔖️ap214/✳️any/🦀️component.rs` is 124 lines. The naive stub detector matched
`print_dsl`/`parse_dsl` at **lines 114–115**, which are inside `#[cfg(test)] mod tests` opened at
**line 88**:

```rust
 88  #[cfg(test)]
 89  mod tests {
...
114          let text = store::ArtifactDsl::print_dsl(&step);
115          let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse real step text");
116          assert_eq!(reparsed, step, "step's own codec_retention_law must hold on our emitted graph");
```

That is the test PROVING the round trip, not the export path. The real export path is
`SemioCadToStep::serialize` at **lines 71–83**, which builds genuine AP214 entity graphs:

```rust
 76                  CadEntity::Line { a, b } => entities.extend(line_entities(&mut ids, a, b)),
 77                  CadEntity::Circle { center, radius } => entities.extend(circle_entities(&mut ids, center, *radius)),
 78                  _ => {} // no B-rep/solid equivalent in this bridge's scope — documented, dropped.
```

`line_entities` (**lines 38–48**) emits `CARTESIAN_POINT` + `DIRECTION` (normalized `b-a`) +
`VECTOR` (magnitude `|b-a|`) + `LINE`; `circle_entities` (**lines 54–59**) emits `CARTESIAN_POINT` +
`AXIS2_PLACEMENT_3D` (axis/refdirection `Unset` → `$`) + `CIRCLE`. Its own test asserts
`step.entities.len() == 7` for one Line + one Circle + one dropped Text.

**Verdict: real, but Line/Circle ONLY.** Everything else is dropped by the `_ => {}` arm — and
`from.blocks` and `from.layers` are never read at all (**line 74** iterates `from.entities` only).

### 1.2 `dxf` (r12) — REAL, and the broadest carrier.

`.../🖊️dxf/🔖️r12/✳️any/🦀️component.rs`, 156 lines. `dxf_entity_from_cad` (**lines 53–73**) maps
**all nine** `CadEntity` variants. Seven map to typed R12 entities; the two with no native R12 type
get bridge-owned raw group codes — `Ellipse` → `DxfEntity::Other{kind:"ELLIPSE"}` (**lines 21–34**),
`Dimension` → `DxfEntity::Other{kind:"DIMENSION"}` (**lines 36–48**). `serialize` (**lines 96–105**)
also writes `tables.layers` from `from.layers` and `blocks` from `from.blocks` (with `base_point`
and nested block entities). Its own test round-trips through `print_dxf_document`/`parse_dxf_document`
and asserts the raw-retained `ELLIPSE` survives.

**One real gap found by reading, not documented anywhere:** `ellipse_to_other` and
`dimension_to_other` take no `layer` argument and emit no layer group code. `rec.layer` is therefore
**silently dropped for the `Ellipse` and `Dimension` variants only**. A layer mutation on those two
entity shapes is invisible in DXF. This is a genuine carrier limitation, recorded rather than smoothed.

**Second limitation:** `CadEntityRecord.handle` is never written by any arm. DXF entity identity is
positional, so a mutation that adds/removes one of two *identical* entities is not distinguishable.

### 1.3 `dwg` (ac1024) — NOT A WRITER AT ALL. The brief's "real writer" claim is wrong.

`.../🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs` is 41 lines and its `serialize` body is one line:

```rust
 21      async fn serialize(_from: &Self::From) -> Result<Self::Into, store::PackError> {
 22          Err(store::PackError::Schema("semio/cad→dwg: unsupported until every CAD topology value has a defined logical DWG entity mapping".into()))
```

Its own test is named `documents_unsupported_direction_as_a_real_error_not_fabricated_bytes`. So dwg
is not "real, write-only evidence" — it produces **no bytes at all**. It is excluded on stronger
grounds than the proprietary-format argument: there is nothing for a reader to read. (The
proprietary-format argument would independently exclude it anyway, matching the already-correct
declines for the standalone `dwg@ac1018`/`ac1024` subsets.)

---

## 2. Outcome classes come from the code, and they are uniform

`✳️cad/🧬️schema/🧬️mutations/🦀️component.rs` **line 146**: `SemioCadMutation::diff` opens with a single
`protocol::MutationOutcome::new(match self { ... })` covering all sixteen arms. There is **no**
`MutationOutcome::empty`, `::error` or `::fatal` anywhere in this subset's mutation or diff module
(grepped both; zero hits). `apply_semio_cad_mutation` (**lines 111–114**) is `diff(...).apply_to(...)`.

The only nuance is `📄set-snapshot/🔺️diff/🦀️component.rs` **lines 7–9**, which returns
`MutationOutcome::new(SemioCadDiff::default()).warn("mutation.no-op", …)` when `base == snapshot` —
still `new`, i.e. still `applied`, carrying a warning.

**Therefore every one of the 16 kinds has outcome class `applied`, and only `applied`.** A manifest
claiming `rejected` or `no-op` for any cad kind would be describing code that does not exist. Missing
targets do not reject either: `inverse` returns `NoMutation` for an unknown name/handle rather than
erroring.

---

## 3. `ruststep` write capability — MEASURED, and the answer is NO

The brief asked this to be settled by trying to compile against it rather than by guessing. It was.
A standalone spike crate (`🔬️cad-ruststep-spike/`, own `[workspace]`, deps `ruststep = "0.4"`,
`dxf = "0.6"`) was built and run offline against the vendored registry. **The spike compiled and ran
— a standalone crate does not touch the currently-broken `semio-framework-plugin` workspace.**

### 3.1 Reading works, generically, with no EXPRESS schema

`Exchange::from_str` on a hand-written Part-21 file containing exactly the entity vocabulary our
exporter emits produced, verbatim:

```
[DEBUG] header records = 3
[DEBUG] data entities = 7
[DEBUG]   Simple { id: 1, record: Record { name: "CARTESIAN_POINT", parameter: List([String(""), List([Real(1.0), Real(2.0), Real(0.0)])]) } }
[DEBUG]   Simple { id: 2, record: Record { name: "DIRECTION", parameter: List([String(""), List([Real(1.0), Real(0.0), Real(0.0)])]) } }
[DEBUG]   Simple { id: 3, record: Record { name: "VECTOR", parameter: List([String(""), Ref(Entity(2)), Real(5.0)]) } }
[DEBUG]   Simple { id: 4, record: Record { name: "LINE", parameter: List([String(""), Ref(Entity(1)), Ref(Entity(3))]) } }
[DEBUG]   Simple { id: 5, record: Record { name: "CARTESIAN_POINT", parameter: List([String(""), List([Real(2.0), Real(2.0), Real(0.0)])]) } }
[DEBUG]   Simple { id: 6, record: Record { name: "AXIS2_PLACEMENT_3D", parameter: List([String(""), Ref(Entity(5)), NotProvided, NotProvided]) } }
[DEBUG]   Simple { id: 7, record: Record { name: "CIRCLE", parameter: List([String(""), Ref(Entity(6)), Real(1.5)]) } }
```

`#N` references resolve to `Ref(Entity(n))`, `$` to `NotProvided`, nested lists nest. This is a real,
schema-free, structural Part-21 reader and it reads exactly what our exporter emits.

### 3.2 Writing does NOT work. Two independent proofs.

**(a) There is no text writer.** `ast::ser::to_record` returns an in-memory `Record` AST node, not
text, and no AST type implements `Display`. Compiling `println!("{}", record)` fails:

```
error[E0277]: `Record` doesn't implement `std::fmt::Display`
error[E0277]: `DataSection` doesn't implement `std::fmt::Display`
error[E0277]: `Exchange` doesn't implement `std::fmt::Display`
```

A grep of the whole crate for `fmt::Display` impls finds them only on `error::TokenizeFailed` and
`primitive::logical::Logical`. `src/ast/ser.rs` is 327 lines and ends at `Record`.

**(b) Even the AST-level serializer is not round-trip-faithful.** `to_record` on a
`CARTESIAN_POINT`-shaped struct FLATTENS the coordinate list into sibling parameters:

```
[DEBUG] to_record -> Record { name: "CARTESIAN_POINT", parameter: List([String(""), Real(1.0), Real(2.0), Real(0.0)]) }
```

Compare with what its own parser produces for the same entity (`#1` above):
`List([String(""), List([Real(1.0), Real(2.0), Real(0.0)])])`. The nesting the STEP grammar requires
is lost. ruststep's serializer does not agree with ruststep's parser here.

**Conclusion: `ruststep` 0.4.0's lib.rs claim "a crate for reading AND WRITING ASCII encoding of
exchange structure" is, for 0.4.0, aspirational — the write half stops at the AST and never reaches
Part-21 text.** ruststep qualifies as a **read-only** STEP oracle and nothing more. Any STEP fixture
must therefore be honestly classed `handcrafted` (deterministic Part-21 text emitted by our
generator, every byte then accepted and structurally verified by ruststep's own parser before it is
hashed) — never `third-party-generated`, which would be a false provenance claim.

`dxf` 0.6.1 has no such problem: `Drawing::save_file` / `Drawing::load_file` are both real, so DXF
fixtures are genuinely `third-party-generated`.

---

## 4. Per-kind witnessability

Derived from §1: DXF carries layers, blocks (name + base point + nested entities) and all nine entity
shapes; STEP carries top-level `Line`/`Circle` geometry only, with no layer, block or handle concept.

| # | kind | dxf | step | note |
|---|------|-----|------|------|
| 1 | `no-mutation` | ✅ | ✅ | both graphs identical |
| 2 | `set-snapshot` | ✅ | ⚠️ partial | step sees the entity list only, never layers/blocks |
| 3 | `add-layer` | ✅ | ❌ | step export never reads `from.layers` |
| 4 | `remove-layer` | ✅ | ❌ | " |
| 5 | `set-layer` | ✅ | ❌ | " |
| 6 | `add-block` | ✅ | ❌ | step export never reads `from.blocks` |
| 7 | `remove-block` | ✅ | ❌ | " |
| 8 | `set-block-base-point` | ✅ | ❌ | " |
| 9 | `add-entity` | ✅ | ✅ Line/Circle only | |
| 10 | `remove-entity` | ✅ | ✅ Line/Circle only | dxf identity is positional (handle dropped) |
| 11 | `set-entity-layer` | ⚠️ 7 of 9 | ❌ | **not** witnessable on `Ellipse`/`Dimension` — §1.2 |
| 12 | `set-entity-geometry` | ✅ all 9 | ✅ Line/Circle only | |
| 13 | `add-block-entity` | ✅ | ❌ | |
| 14 | `remove-block-entity` | ✅ | ❌ | |
| 15 | `set-block-entity-layer` | ⚠️ 7 of 9 | ❌ | same `Ellipse`/`Dimension` exclusion |
| 16 | `set-block-entity-geometry` | ✅ | ❌ | step drops blocks entirely |

**16 of 16 witnessable by at least one carrier; 0 uncarried.** 5 of 16 are witnessed by BOTH carriers
(`no-mutation`, `set-snapshot`, `add-entity`, `remove-entity`, `set-entity-geometry`) and therefore
carry two `oracleRequirements`; the other 11 carry the dxf one only.

Two documented partial exclusions (`set-entity-layer` / `set-block-entity-layer` on `Ellipse` and
`Dimension`) must make the probe return `status: "unsupported"` rather than an empty `"ok"` — the
empty-ok mistake the mesh pilot paid for.

---

## 5. Tolerance decision

`cad` records 2D entity geometry EXACTLY — points, radii, angles as `f64`, never tessellated. There is
no legitimate "differently tessellated but equally valid" reading of a `LINE` or a `CIRCLE` the way
there is for a solid. So the gate is **near-exact** (mesh-shaped, not brep-shaped): coordinate and
radius equality to `1e-12` absolute, counts exactly equal. Nothing here needs a tessellation tolerance.

---

## 6. What was built

All five artefacts exist and were exercised. §6.1 and §6.2 below record the two library decisions that
shaped them; §9 records what the harness says.

* `✳️cad/🧪️oracle/🔣️.json` — 2 new `third-party-library` oracles alongside the untouched
  `semio-cad-python-independent`, 6 probes, 1 comparison profile, 1 tolerance profile, 2 comparison
  pipelines, a 16-mutation manifest and 21 fixture manifests over 44 hashed files.
* `✳️cad/🔬️probes/📜️script.ts` + `✳️cad/🔬️probes/🦀️oracle-probe/` — the TypeScript entry point and the
  standalone crate behind it.
* `✳️cad/🏭️generator/📜️script.ts` + `🧪️dxf-entities/` + `🧪️step-line-circle/`.
* `✳️cad/🧫️fixtures/<recipe>/` — 21 recipes.

### 6.1 brepjs was tested against a cad-shaped STEP file and REFUSES it

Run with the already-qualified cc6 probe suite
(`…📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🔬️probes/📜️script.ts step-import`), not with a
hand-rolled harness, so the result is attributable to the input rather than to the caller.

**Control — a committed full-B-Rep fixture from the sibling `brep` corpus:**

```
--input ✏️s/…/✳️brep/🧫️fixtures/topology-remove-delete-shell-redundant-shell-small/operand-a.step
{"status":"ok","measurements":{"bothImport":true,"imported":1,"inputs":1,
 "perInput":[{"input":"…/operand-a.step","ok":true}]},"durationMs":312}
```

**Subject — a STEP file in exactly the shape `SemioCadToStep` emits** (three header records; a data
section of bare `CARTESIAN_POINT`/`DIRECTION`/`VECTOR`/`LINE` and
`CARTESIAN_POINT`/`AXIS2_PLACEMENT_3D`/`CIRCLE`; no `PRODUCT`, no
`SHAPE_DEFINITION_REPRESENTATION`, no context entities):

```
{"status":"ok","measurements":{"bothImport":false,"imported":0,"inputs":1,
 "perInput":[{"input":"…/cadstep.step","ok":false}]},"durationMs":193}
```

The underlying OCCT failure is `shape.IsNull()` in brepjs's `importSTEP$1` —
`"stream reader could not parse the input data"`.

The two sides together are what make this conclusive rather than merely negative:

* The file is **syntactically valid Part-21** — ruststep parses it completely and resolves all seven
  entities, every `#N` reference and both `$` placeholders (§3.1). So this is not a malformed file.
* OCCT nevertheless recovers **zero shapes**, because its STEP reader transfers shapes through
  product / shape-representation structure, and our cad export emits none. There is nothing for it
  to transfer.

**Registering brepjs against `semio-v1-cad-mutate` would therefore produce a probe that reports
`imported: 0` on every single cad fixture — a green capability standing on evidence that was never
read.** It is declined on measured grounds.

### 6.2 Consequence for the TypeScript-only redirect

`node_modules` was enumerated: the only STEP/CAD-capable JS packages vendored are `brepjs` (declined
above), `three` and `manifold-3d` (mesh, not applicable). **There is no vendored JavaScript DXF
parser and no vendored JavaScript Part-21 reader.** A TypeScript-only oracle for this subset is
therefore not reachable without adding new runtime JS dependencies.

The two libraries that DO fit are both already approved in `🔒️dependencies.json` as `test-oracle`,
and both are Rust:

| library | version | role | already registered for |
|---|---|---|---|
| `ruststep` | 0.4 | structural Part-21 reader (read-only, §3) | 12 oracle ids incl. `ruststep-step-ap214-*`, `ruststep-ifc-*` |
| `dxf` | 0.6 | ASCII DXF reader **and** writer | `dxf-crate-r12-mutate` |

The workspace breakage does not block this: a standalone crate carrying its own `[workspace]` and
depending only on crates.io never compiles `semio-framework-plugin`. This was proven, not assumed —
the §3 spike built and ran offline while the workspace was broken. It is the same pattern
`…✳️cc6/🏭️bridge/Cargo.toml` already uses, and for the same stated reason.

So the correct structure keeps the house TypeScript entry-point contract and puts the third-party
readers behind it: `🔬️probes/📜️script.ts` in TypeScript, marshalling via `spawnSync("cargo", …)` to a
standalone crate that links `ruststep` and `dxf`. The probe READS both artifacts and emits
measurements; it never computes what a mutation ought to produce, so it does not fall foul of the
`reimplementation-registered-as-third-party` gate.

### 6.3 Why the probe is a Rust crate behind a TypeScript entry point

The house contract is a `🔬️probes/📜️script.ts` that emits a `ProbeReport` on stdout, and that is what
this subset has. What sits behind it is Rust, because §6.2 leaves no alternative: no vendored JS
package can read either carrier. The crate carries its own `[workspace]` and links nothing from this
repository, so it builds while `semio-s-plugin-stdio` does not — the same pattern, and the same
stated reason, as `…✳️cc6/🏭️bridge/Cargo.toml`. Its `Cargo.lock` is committed, pinning both oracles.

It READS; it never predicts. Neither library is ever asked what a mutation ought to produce — only
what a file contains — so this does not fall foul of the `reimplementation-registered-as-third-party`
gate that five other owners tripped.

---

## 7. Two design faults found by building it, both by measurement

### 7.1 The DXF corpus was not reproducible, and `fixture reproduce` would have failed all sixteen

Generating the corpus twice and diffing showed every DXF file differing in one place:

```
336c336
< 2461280.937766203657
---
> 2461280.937893518712
```

That is `$TDCREATE`/`$TDUPDATE` — a Julian day number whose fractional part is the moment of the run.
The `dxf` crate fills all four time variables from `Local::now()`/`Utc::now()` in `Header::set_defaults`.

Fixed by `pin_wall_clock`, which pins them to J2000.0 **through the library's own parser**: the drawing
is serialised, the four values are rewritten in that intermediate text, and the result is handed back
to `Drawing::load` — which accepts it, proving the edit is still valid DXF — so the committed bytes are
ones the `dxf` crate wrote from a state it parsed. Two runs a second apart are now byte-identical
across all 44 files.

### 7.2 One `unsupported` rule was serving two different questions, and it rejected a known-GOOD pair

The first probe had a single comparison command carrying the rule "identical readings on a real
mutation means the carrier did not encode it → `unsupported`". Validating the gate in the ACCEPT
direction — which is the only reason that direction is mandatory — produced:

```
-- ACCEPT: correct after-state against itself
  status = unsupported     readingsEqual = True     maxAbsoluteDelta = 0.000000e+00
```

The rule is right for the WITNESS question (before vs after: did the carrier record the change?) and
wrong for the AGREEMENT question (expected vs actual: do they match?), where identical is exactly what
success looks like. Split into `dxf-witness`/`step-witness` and `dxf-compare`/`step-compare`. A gate
only ever tested on bad input would have shipped this.

### 7.3 A third finding: the DXF oracle reaches seven of nine entity shapes, not nine

The first corpus was built with all nine `CadEntity` shapes and came back three short — `ellipse`,
`polyline` and `dimension` missing. `ELLIPSE`, `LWPOLYLINE` and typed `DIMENSION` are R13+ entities and
the `dxf` crate will not write them into an R12 document, which is this subset's dialect.

`polyline` was recovered by switching to the R12 `POLYLINE` + `VERTEX` form. `Ellipse` and `Dimension`
were not: our exporter smuggles them through as bridge-owned RAW GROUP CODES (`DxfEntity::Other`),
which is not standard R12 and which the crate does not recover as typed entities. Combined with the
§1.2 finding that neither `ellipse_to_other` nor `dimension_to_other` emits a layer group code at all,
`set-entity-layer` and `set-block-entity-layer` are witnessable for seven shapes and not for two. Both
therefore carry a third `oracleRequirement` naming `semio-v1-cad-mutate-uncarried`, a capability no
oracle provides, so the residue reports honestly as un-oracled instead of being absorbed into the
passing majority. This corrects §4's table, which had followed the earlier research pass in reading
"our exporter maps all nine" as "the oracle can witness all nine".

---

## 8. Registered oracles, probes and gate

| oracle | kind | package | engine family | kinds | fixture class |
|---|---|---|---|---|---|
| `dxf-crate-cad-r12-read` | `third-party-library` | `dxf` 0.6.1 (MIT) | `dxf-rs` | all 16 | `third-party-generated` |
| `ruststep-cad-line-circle-read` | `third-party-library` | `ruststep` 0.4.0 (Apache-2.0) | `ruststep` | 5 | `handcrafted` |
| `semio-cad-python-independent` | `cross-semio-implementation` | — | none | supplemental | — |

Two DIFFERENT engine families: `ruststep` is a nom-based structural Part-21 reader, `dxf-rs` a
group-code reader. They share no kernel, so on the five kinds both witness, each checks the other.

Probes: `dxf-read`, `dxf-witness`, `dxf-compare`, `step-read`, `step-witness`, `step-compare`.

**Witness sweep — all 21 recipes, real output.** Every mutating recipe reports
`carrierWitnessed: true` with a non-zero difference count; both `no-mutation` recipes report 0
differences, as they must. Sample:

```
dxf   set-block-base-point-door        set-block-base-point  ok  witnessed=True  diffs=2
dxf   set-entity-layer-...             set-entity-layer      ok  witnessed=True  diffs=3
step  step-set-entity-geometry-...     set-entity-geometry   ok  witnessed=True  diffs=1
```

**`unsupported` fires, and is not an empty `ok`.** `step-witness` handed a `set-layer` mutation on a
pair STEP cannot distinguish:

```
"status": "unsupported",  "carrierWitnessed": false,  "readingsEqual": true,
"differenceCount": 0,     "maxAbsoluteDelta": 0.0
```

**GATE, validated in BOTH directions on BOTH carriers.** Known-good is the correct after-state
(circle radius 1.5 → 2.25); known-bad is radius 2.30 with centre x 2.0 → 2.1, deliberately small
enough that a loose tolerance would wave it through.

| carrier | direction | `agree` | `maxAbsoluteDelta` | vs tolerance |
|---|---|---|---|---|
| dxf | ACCEPT | `true` | `0.000000e+00` | 1e-12 |
| dxf | REJECT | `false` | `1.000000e-01` | 1e-12 |
| step | ACCEPT | `true` | `0.000000e+00` | 1e-12 |
| step | REJECT | `false` | `1.000000e-01` | 1e-12 |

The rejection names its counterexample rather than just failing:

```
entity[1].geom[0] differs by 1.000000e-1: 2 vs 2.1
entity[1].geom[2] differs by 5.000000e-2: 2.25 vs 2.3
```

Roughly eleven orders of magnitude of margin between a correct answer and a wrong one.

---

## 9. Harness results — real output

```
[fixture verify]    21 fixture(s), 0 file problem(s)
[fixture audit]     21 fixture(s), 0 with contract problems
[fixture reproduce] 16 generated fixture(s), 0 problem(s)
```

`fixture reproduce` re-runs each recorded generator command into a scratch root and diffs the bytes
against the committed hashes; it only considers `third-party-generated` fixtures, which is why it
reports 16 and not 21. The 5 `handcrafted` STEP bundles are hash-verified by `fixture verify` instead.
Determinism was additionally checked directly: two full generations one second apart are byte-identical
across all 44 files.

`matrix --subset cad` — all 16 cad mutations now carry a qualifying external oracle. Per-dimension,
filtered to what cad itself is missing:

```
externalOracleCoverage          48.14%  194/403    cad-missing=0
oracleEvidenceCoverage          45.66%  184/403    cad-missing=0
subsetOwnershipCoverage         90.82%  366/403    cad-missing=0
fixtureProvenanceCoverage       99.71%  347/348    cad-missing=0
fixtureReproducibilityCoverage 100.00%  348/348    cad-missing=0
runtimeMutationCoverage          0.00%    0/20     cad-missing=1  (no runtime inventory)
oracleCapabilityCoverage        60.71%   17/28     cad-missing=1  (semio-v1-cad-mutate-uncarried)
```

⚠️ **Do not read the repo-wide movement as this subset's contribution.** `externalOracleCoverage` went
from 82/403→194/403 against a 28.67% (82/286) baseline, but the denominator grew by 117 while this
subset added 16; peer sessions landed the rest concurrently. The attributable claim is the narrow one:
`cad-missing=0`, i.e. 16 of 16 cad mutations oracled where the baseline had none registered at all.

Each cad row resolves to a real oracle rather than a placeholder:

```
mutation=no-mutation  fixture=no-mutation-identity  fixtureClass=third-party-generated
oracle=dxf-crate-cad-r12-read  oracleKind=third-party-library  oracleEngineFamily=dxf-rs
comparisonProfile=semantic-cad-entity-v1  status=missing ("no execution produced a result")
```

`status: missing` on all 16 is the RUN phase, not the registration: executing a case needs the subject
side, and `semio-s-plugin-stdio` does not currently compile. The oracle side is complete and
independently exercisable today via the probes above.

### 9.1 Contract — three cad-scoped findings, and one is mine

`contract` is repo-wide (`--subset` is not honoured by it) and reports 1434 high-priority breaches
against a 1280 baseline. Of those, eight name `✳️cad`:

1. **`duplicate-mutation-owner` ×3 — MY REGRESSION, and a framework key defect underneath it.**
   `no-mutation`, `set-snapshot` and `remove-block` are now "owned by 2 manifests: ✳️cad, ✳️document".
   The ownership key is built at `📦️index.ts:4329` as `${artifact}@${standard}::${mutation.id}` — the
   SUBSET is not in it, although `coordinate` is computed on the line immediately above and used in
   the owner string. Two subsets of one artifact cannot both own a same-named mutation, yet every
   subset legitimately has `no-mutation` and `set-snapshot`. This is pre-existing rather than novel —
   `s.stdio.semio@v1::move-vertex` already collides between `brep` and `mesh` at baseline — but I
   added three more instances. The fix is one line, `const key = \`${coordinate}::${mutation.id}\``,
   in shared framework code; I have NOT made it, because peers are working live in that file and it
   is outside this subset's scope. It needs an owner.
2. **`semio-v1-cad-mutate-uncarried` ×2 — by design.** This is the `sequence@1/any` convention the
   brief prescribes: a capability no oracle provides, so the Ellipse/Dimension layer residue reports
   as un-oracled. 167 such breaches already exist across `sequence`, `fem2d` and `fem3d`; mine are 2
   more of the same kind.
3. **`runtime-inventory-missing` ×1 and two pre-existing findings.** The runtime inventory needs the
   production bridge. Separately, contract independently confirms §1.3 from the other direction:
   `The dwg serializer never reads its input`.

---

## 10. Honest summary of what is and is not covered

* **16 of 16** kinds carry a qualifying `third-party-library` oracle requirement.
* **5 of 16** (`no-mutation`, `set-snapshot`, `add-entity`, `remove-entity`, `set-entity-geometry`)
  are witnessed by TWO oracles on two different engine families.
* **2 of 16** (`set-entity-layer`, `set-block-entity-layer`) carry an additional uncarried requirement
  for the `Ellipse` and `Dimension` shapes, which r12 cannot express typed.
* **0 of 16** are fully uncarried.
* `set-snapshot`'s STEP leg is partial by construction (entities only, never layers or blocks) and is
  registered alongside dxf rather than instead of it, for exactly that reason.
* Fixtures: 21 bundles, 44 files. 16 `third-party-generated` (built and written by the `dxf` crate),
  5 `handcrafted` (our Part-21 text, verified by ruststep, which cannot write). All 44 byte-reproducible.
* The subject side is not yet wired: `status: missing` on every row until `semio-s-plugin-stdio`
  compiles and a production bridge can emit a runtime inventory.
