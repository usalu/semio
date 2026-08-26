# Wave 23 — IfcOpenShell as a real second PRODUCER for IFC, and the honest answer for STEP

> **The bar:** *every single mutation exercised on a REAL-WORLD COMPLEX artifact, with a SECOND
> INDEPENDENT IMPLEMENTATION producing the same result.*

Date 2026-08-26. Successor to `📓️w14-final-audit.md`, whose §3.4 and remedy #4 this wave answers.
Working files, probes and raw outputs: `w23-ifc-differential/`. Every `[test]` line below is copied
verbatim from the tool's own stdout and every exit code was read from the tool's own exit status,
never through a pipe.

---

## 0. What changed, in one paragraph

`w14-final-audit.md` §3.4 found that the twelve `🏗️ifc` + `📐️step` cases declare **zero**
`@mode-differential` scenarios because `ruststep` has no writer, that **IfcOpenShell 0.8.4.post1 is
installed on this machine and reads AND writes IFC**, and that the only thing standing between the
IFC cases and a real differential was a governance rule forbidding an edit to the shared stdio
oracle manifest. That rule was lifted for this wave. IfcOpenShell is now a registered, dispatched,
running second PRODUCER for both `🏗️ifc` subsets: **23 new `@mode-differential` scenarios across two
new cases**, every one of them exercising a mutation on a real committed artifact (2 496 437 bytes /
24 792 entities, and 193 915 bytes / 3 464 entities), with the mutation applied and the whole
exchange structure re-serialized by IfcOpenShell itself. **Nothing was retyped, weakened, deleted or
substituted:** all 34 existing `ruststep`-backed scenarios in `mutate-ifc-4` and `mutate-ifc-2x3`
stand exactly as they were.

**And parity IS measured — `23/23`.** The `w14-final-audit.md` blocker (`unresolved import
component::component_persistent_local`) is **gone**: the generated Rust subject hosts compile, both
cases ran both phases, and every one of the 23 comparisons agreed. Better than agreed: with the
comparison profile's tolerance and `ignoreKeys` switched OFF entirely, the two projections are still
**value-for-value identical** on all 24 792 and 3 464 instances. See §6.

---

## 1. Why the cases were ADDED rather than the existing ones RETYPED

The brief asked for the existing `mutate-ifc-4` / `mutate-ifc-2x3` scenarios to be retyped
`@mode-differential`. That was measured and rejected, because it would have **cost evidence**, which
the brief forbids more strongly than it asks for the retyping.

A case has exactly one oracle: `oracleDecision` (`🧪️test/📜️script.ts:545-557`) maps the feature's
single `@oracle-<id>` tag to that entry's ecosystem and dispatches one adapter in the oracle role.
Retyping in place therefore means *replacing* `ruststep` with IfcOpenShell for the whole case. But
this subset's `IfcMutation` vocabulary is Part-21 RECORD-level and IfcOpenShell is EXPRESS-SCHEMA-
bound, so for four of the eleven IFC4 kinds it is not merely unable to PRODUCE the result — it
cannot READ it either, and fails silently:

| kind | measured against the real fixture |
|---|---|
| `set-entity-name` | creating `RENAMED_PROXY` raises `Entity with name 'RENAMED_PROXY' not found in schema 'IFC4'`; a file carrying it reads back through `ifcopenshell.file.from_string` as **16 975 of 24 792** entities, **no error raised** |
| `insert-entity-arg` | a tenth positional argument on the nine-attribute `IfcBuildingElementProxy` raises `IndexError` on assignment; a hand-written file carrying one reads back with the extra argument **silently dropped** |
| `remove-entity-arg` | arity cannot be reduced through the schema-bound API; assigning `None` writes `$` and keeps nine arguments — a different mutation |
| `remove-entity` | `ifcopenshell.file.remove`'s own docstring: *"the reference to the deleted will be removed from the aggregate"*. Confirmed: `#16976` disappears from `#16991`'s member aggregate. `IfcMutation::RemoveEntity` deliberately does not cascade |

`ruststep` reads all four results whole, because it parses the Part-21 GRAMMAR and compiles no
EXPRESS schema at all. **Swapping a schema-agnostic reader that reads the file whole for a
schema-bound one that silently truncates it is a weakening**, and eight scenarios (4 kinds × mutate +
inverse) would have paid for it. So `mutate-ifc-4` and `mutate-ifc-2x3` are untouched and the
differential claim lives in two new cases beside them. Repo-wide nothing is lost and 23 differential
scenarios are gained.

The new cases carry no `@mutations-<catalog>` tag, so they make no exhaustiveness claim they cannot
keep (`mutationCoverageBreaches` returns early when `feature.mutationCatalog === null`); the
exhaustive claim stays where it is discharged, next door.

---

## 2. The two new cases

### 2.1 `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-4` — 15 scenarios

| | |
|---|---|
| second producer | **IfcOpenShell 0.8.4.post1**, `ifcopenshell.open` (its own C++ Part-21 parser + IFC4 EXPRESS schema) → mutation through its own typed API → `ifcopenshell.file.to_string` (its own writer) |
| fixture | `shared://🏗️nakagin-capsule-tower.ifc` — **2 496 437 bytes, 24 792 entity instances**, `FILE_SCHEMA(('IFC4'))`, a real IfcOpenShell export of Kisho Kurokawa's Nakagin Capsule Tower. Unchanged, uncopied, unsubstituted; each scenario copies it into the work directory first |
| kinds claimed | 7 of 11: `no-mutation`, `set-snapshot`, `set-file-description`, `set-file-name`, `set-file-schema`, `insert-entity`, `set-entity-arg` |
| scenarios | 7 `differential-<kind>` + 7 `differential-inverse-<kind>` + 1 `differential-identity-round-trip`, **all `@mode-differential`** |
| subject | this repository's own `IfcSnapshot` codec (`parse_part21` → `apply_ifc_mutation` → `write_part21`), projected through `project_ifc_4_any` |

### 2.2 `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-2x3` — 8 scenarios

| | |
|---|---|
| second producer | the same, against the IFC2X3 schema |
| fixture | `shared://🏗️wellness-center-sama-street-level.ifc` — **193 915 bytes, 3 464 entity instances**, `FILE_SCHEMA(('IFC2X3'))`, a real forward-reference-closed slice of a real 21 MB EDM StepFileFactory export |
| kinds claimed | 4 of 5 forward (`no-mutation`, `set-snapshot`, `upsert-instance`, `set-header`), 3 of 5 inverse |
| scenarios | 4 + 3 + 1 `differential-identity-round-trip`, **all `@mode-differential`** |
| subject | `decode_ifc2x3` → `apply_ifc2x3_mutation` → `encode_ifc2x3`, projected through `project_ifc_2x3_any` |

### 2.3 Proof the producer is independent

Mechanical census over both `🐍️component.py` adapters:

* **Complete import list:** `json`, `os`, `re`, `ifcopenshell`, and `from semio_repo_test import
  Adapter, Context, Outcome` (the host's own facade). **No semio production module.**
* **`subprocess`, `os.system`, `popen`, `ctypes`, `cffi`, `dlopen`, `importlib`, `cargo`, `wasm`,
  `semio_s_plugin`: zero hits outside the docstrings that state their absence.** No shell-out to any
  binary of ours.
* The projection reads **bytes IfcOpenShell actually wrote**, through a from-scratch ISO 10303-21
  reader written from clause 6 (`§6.4.2` control directives, `§6.2` doubled apostrophe) and clause 8
  (`§8.2.2`/`§8.2.3` header attribute order) — never an in-memory Python object graph that never
  survived serialization, and never the production `step::engine::part21` codec the test is evidence
  about.

---

## 3. Oracle-phase results, verbatim

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case differential-ifc-4      # exit 0
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case differential-ifc-2x3    # exit 1→0
[test] level=exhaustive cases=1 executed=8 passed=8 failed=0 errored=0 parity=0/0
```

`passed=N` on its own is worth nothing — that is `w14-final-audit.md` §0's whole point. What the run
actually produced, read back out of the result stream (`w23-ifc-differential/oracle-projections.txt`):
**15 and 8 genuinely different 2.5 MB / 188 KB IFC documents**, each carrying its mutation and
nothing else.

```
differential-no-mutation             entities=24792  schema=['IFC4']    fileName.name="/dev/null"           probeArg[2]="b"              #90001=no   rawBytes=2496437
differential-set-snapshot            entities=24792  schema=['IFC4X3']  fileName.name="/dev/null"           probeArg[2]="b"              #90001=no   rawBytes=2496439
differential-set-file-name           entities=24792  schema=['IFC4']    fileName.name="wave-7-mutated.ifc"  probeArg[2]="b"              #90001=no   rawBytes=2496409
differential-insert-entity           entities=24793  schema=['IFC4']    fileName.name="/dev/null"           probeArg[2]="b"              #90001=yes  rawBytes=2496484
differential-set-entity-arg          entities=24792  schema=['IFC4']    fileName.name="/dev/null"           probeArg[2]="origin-marker"  #90001=no   rawBytes=2496449
differential-inverse-insert-entity   entities=24792  schema=['IFC4']    fileName.name="/dev/null"           probeArg[2]="b"              #90001=no   rawBytes=2496437
…
differential-upsert-instance         entities=3464   schema=['IFC2X3']  fileName.name="0001"                probeArg[2]="WAVE8-RENAMED-COLUMN"       rawBytes=188261
differential-set-header              entities=3464   schema=['IFC2X3']  fileName.name="wellness-center-…"   probeArg[2]="UC-Universal Columns-…"     rawBytes=188323
```

Every inverse row returns to the pristine values (`entities=24792`, `"/dev/null"`, `"b"`,
`#90001=no`, 2 496 437 bytes), which is the inverse law being discharged by the reference
implementation in role, before any subject exists.

### 3.1 The assertions are load-bearing — negative controls

`w23-ifc-differential/negative_controls.py`, run under the runner's own cache-local interpreter,
**exit 0**, every probe forcing an assertion to fire:

```
PASS  ifc4 observability law on a no-op set-entity-arg           raised: 'set-entity-arg' left IfcOpenShell's semantic projection … unchanged
PASS  ifc4 observability law                                     accepted the real row
PASS  ifc4 divergence finder on one deep argument                $.entities[16975].args[2].v: 'b' against 'origin-marker'
PASS  ifc4 inverse law on a wrong inverse                        raised: $.entities[16975].args[2].v: 'b' against 'NOT-THE-ORIGINAL'
PASS  ifc4 remove-entity cascade guard on referenced #16976      raised: #16976 is referenced by 7 other instance(s) …
PASS  ifc4 unproducible kind set-entity-name                     raised ValueError: mutation kind 'set-entity-name' has no IfcOpenShell producer …
PASS  ifc4 string decoder on \Q / \X\ZZ / unterminated \X2\ / ISO 8859-2 \S\   all raised
PASS  ifc4 string decoder  '\\'→'\'  '\X\41'→'A'  '\X2\4E2D\X0\'→'中'  '\S\A'→'Á'
PASS  ifc2x3 observability law on a no-op set-header             raised
PASS  ifc2x3 remove-instance cascade guard on referenced #270549 raised: #270549 is referenced by 8 other instance(s) …
ALL NEGATIVE CONTROLS PASSED
```

The third line is the one that matters most: **one changed argument, 16 975 entities deep in a
24 792-entity graph, is found.** The comparison is not looking at a corner of the document.

---

## 4. Two findings the second producer surfaced

Neither is ours, and neither was tuned away.

### 4.1 IfcOpenShell 0.8.4.post1 silently discards the whole DATA section on an unknown FILE_SCHEMA identifier

ISO 10303-21 §8.2.4 makes `schema_identifiers` a LIST. IfcOpenShell **writes** a two-identifier
`FILE_SCHEMA(('IFC2X3','IFC2X3-WAVE8-SNAPSHOT-MARKER'));` correctly. It then cannot read its own
output back — and does not say so:

```
ifcopenshell.open(<that file>)         → returns WITHOUT raising; header intact, data section EMPTY
    .to_string()                       → 332 bytes: `DATA;` `ENDSEC;` and none of the 3 464 instances
ifcopenshell.file.from_string(<same>)  → RuntimeError: No schema loaded
```

IfcOpenShell's two entry points disagree about whether this is an error at all. This surfaced as a
real red scenario in the first 2x3 run (`differential-inverse-set-snapshot`,
`$.entityCount: 3464 against 0`) and was reproduced standalone before being acted on.

**What was done about it.** Not a widened tolerance and not a changed fixture. Two things:
1. Every read in both oracles now goes through `open_model`, which compares IfcOpenShell's
   materialized instance count against the count the document text itself declares and **refuses a
   truncated model**. The class of silent loss can no longer pass anywhere in this oracle.
2. `set-snapshot` keeps its forward differential row and has **no inverse row**, because IfcOpenShell
   genuinely cannot produce the second half of that chain. `inverse-set-snapshot` keeps its
   `ruststep`-backed scenario in `mutate-ifc-2x3`, unchanged. The reason is written into the feature
   file, the adapter and the registration.

### 4.2 The Nakagin fixture is a fixed point of IfcOpenShell's own writer

`ifcopenshell.open(🏗️nakagin-capsule-tower.ifc).to_string()` is **bit-identical to the input** —
2 496 437 bytes in, 2 496 437 identical bytes out — because that file *is* an IfcOpenShell 0.8.4
export. A "must not be bit-identical" tripwire on the oracle side would therefore fail a correct
implementation, so the IFC4 round trip asserts instead that IfcOpenShell materialized as many typed
instances as its own written bytes carry — two independent counts of one model, which a byte copy
could not report. The 2x3 fixture, written by EDM StepFileFactory, is **not** a fixed point
(193 915 → 188 288 bytes, projections identical instance for instance), so there the byte tripwire is
real and is asserted. Both facts are recorded in the feature files rather than papered over.

---

## 5. The projection cross-check — why the parity number is about the CODECS

The subject projects through `project_ifc_4_any` / `project_ifc_2x3_any`, the `ruststep`-backed
projectors in the stdio oracle crate; the oracle projects through its own from-scratch Python reader.
If those two disagreed, the parity number in §6 would be measuring the projectors, not the codecs.
`w23-ifc-differential/ruststep-crosscheck` (a ticket-local standalone `[workspace]`
crate that links the oracle crate with `features = ["oracles"]`) dumps the Rust projection of any
document; `w23-ifc-differential/crosscheck.py` diffs it against the Python one under
`semantic-ifc-v1`'s own tolerance and ignore list:

```
== differential-ifc-4 (🏗️nakagin-capsule-tower.ifc, 2496437 bytes)
   no-mutation            bytes=2496437   ruststep entities=24792  ifcopenshell entities=24792  AGREE
   set-snapshot           bytes=2496439   ruststep entities=24792  ifcopenshell entities=24792  AGREE
   set-file-description   bytes=2496418   ruststep entities=24792  ifcopenshell entities=24792  AGREE
   set-file-name          bytes=2496409   ruststep entities=24792  ifcopenshell entities=24792  AGREE
   set-file-schema        bytes=2496439   ruststep entities=24792  ifcopenshell entities=24792  AGREE
   insert-entity          bytes=2496484   ruststep entities=24793  ifcopenshell entities=24793  AGREE
   set-entity-arg         bytes=2496449   ruststep entities=24792  ifcopenshell entities=24792  AGREE
== differential-ifc-2x3 (🏗️wellness-center-sama-street-level.ifc, 193915 bytes)
   no-mutation            bytes=188288    ruststep entities=3464   ifcopenshell entities=3464   AGREE
   set-snapshot           bytes=188319    ruststep entities=3464   ifcopenshell entities=3464   AGREE
   upsert-instance        bytes=188261    ruststep entities=3464   ifcopenshell entities=3464   AGREE
   set-header             bytes=188323    ruststep entities=3464   ifcopenshell entities=3464   AGREE

PROJECTIONS AGREE ON EVERY DOCUMENT
```

Negative control on the comparator itself, so the eleven `AGREE`s are not vacuous:

```
negative control (ruststep(set-entity-arg) vs python(no-mutation)): $.entities[16975].args[2].v: 'origin-marker' against 'b'
positive control (same document):                                    None
```

**Reading:** across 24 792 and 3 464 instances, IfcOpenShell's serialization and `ruststep`'s reading
of it project identically under the profile the comparison uses. So §6's 23/23 is not two
projectors agreeing with themselves — the projectors were shown to agree first, which leaves the two
CODECS as the only thing the differential is comparing.

---

## 6. Parity — measured, not deferred

`w14-final-audit.md` §5.3 recorded that no generated Rust subject host in the repository linked, so
`parity` was `0/0` everywhere. **That blocker is gone.** Measured directly rather than assumed:

```
$ cargo check --features sut --manifest-path .../hosts/…-differential-ifc-2x3-subject-rust/Cargo.toml
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5m 18s          # exit 0
```

No `E0432`, no `component_persistent_local`. The peer session's `💻️os/🔨️modules/🔌️plugin` refactor
has landed the macro since the audit. So the headline command runs:

```
$ SEMIO_TEST_BUDGET_MS=3000000 bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case differential-ifc-2x3
[test] level=exhaustive cases=1 executed=16 passed=16 failed=0 errored=0 parity=8/8      # exit 0
```

```
$ SEMIO_TEST_BUDGET_MS=5400000 bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case differential-ifc-4
[test] level=exhaustive cases=1 executed=30 passed=30 failed=0 errored=0 parity=15/15
```

**23 of 23.** Every scenario of both cases had both producers emit a projection and every one
agreed: IfcOpenShell's own C++ Part-21 writer and this repository's `IfcSnapshot` /
`Ifc2x3Snapshot` codecs produce documents that project identically across **24 792** and **3 464**
instances — for `no-mutation`, `set-snapshot`, `set-file-description`, `set-file-name`,
`set-file-schema`, `insert-entity`, `set-entity-arg`, `upsert-instance`, `set-header`, ten inverses
and two identity round trips.

### 6.0 The profile's tolerance and ignoreKeys absorb NOTHING

A parity number that leans on a loose profile is not evidence. Re-diffed with `tolerance` and
`ignoreKeys` switched off entirely — exact value equality, every key compared including
`timestamp`, `preprocessorVersion`, `originatingSystem` and `authorization`
(`w23-ifc-differential/parity-exact-diff.txt`):

```
== differential-ifc-4
   differential-no-mutation            entities=24792  IDENTICAL (exact, no tolerance, no ignoreKeys)
   differential-insert-entity          entities=24793  IDENTICAL (exact, no tolerance, no ignoreKeys)
   differential-set-entity-arg         entities=24792  IDENTICAL (exact, no tolerance, no ignoreKeys)
   … all 15 IDENTICAL
== differential-ifc-2x3
   … all 8 IDENTICAL
```

**All 23 are exactly identical.** `semantic-ifc-v1` is doing no work here at all; the agreement is
value-for-value. And the two producers are genuinely producing different bytes — every scenario's
`rawHash` differs between the roles (e.g. `differential-no-mutation`: oracle `4d1d9862…`, subject
`2f5f42f9…`), which is exactly right: two conformant Part-21 writers, one semantics.

### 6.1 The first parity run was 7/8, and the red one was a real finding in OUR adapter

```
[test] level=exhaustive cases=1 executed=16 passed=15 failed=1 errored=0 parity=7/8
[test] parity failed: …::differential-ifc-2x3::differential-inverse-no-mutation::rust::subject (1 differences)
```

Subject diagnostic: `byte pass-through: output is bit-identical to the input`. The cause is not a
codec defect — it is **this repository's Part-21 writer being idempotent**, which is what a correct
writer is: re-encoding a document it already wrote reproduces it exactly. The no-pass-through
tripwire belongs on the cycle whose input is the foreign committed fixture, where identical bytes
really would mean the document was copied instead of decoded. On the *second* cycle of an inverse
pair the input is already our own normal form, so the tripwire was asserting something false.

Fixed by scoping it — `apply_and_encode(input, spec, refuse_identity)`, `true` for the cycle that
reads the committed artifact and for the round trip, `false` for the second cycle — with the
measurement written into the docstring. **Nothing was relaxed:** the property is still asserted
against the real artifact, on every forward scenario and on the round trip. The sibling
`mutate-ifc-2x3` never hits this only because its inverse handler short-circuits `no-mutation` to
`input.clone()` and does not run the codec at all for that row; this case runs both cycles for every
kind, including the trivial one, which is why it found it.

After the fix: `executed=16 passed=16 failed=0 errored=0 parity=8/8`, exit 0.

### 6.2 Caveats a reader still needs

* `oracle exhaustive` prints `parity=0/0` **by construction** — it runs only the reference side. The
  numbers in §3 are oracle-phase numbers and are not parity.
* The default per-case budget is not enough here. `parity exhaustive` at the stock budget died with
  `spawnSync cargo ETIMEDOUT` before reaching a single scenario, because the 900 s is spent compiling
  the host from scratch — `w14-final-audit.md` §5.4 and remedy #6, still open. Both runs above set
  `SEMIO_TEST_BUDGET_MS` explicitly. **A reader running the stock command on a cold checkout will get
  no summary line at all, not a zero.**
* During this wave the shared results cache was swept out from under a running phase twice
  (`differential-ifc-4` reported `executed=15 passed=1 errored=14`, every error a
  `FileNotFoundError` on its own output directory; the `differential-ifc-2x3` results directory
  vanished between the run and the read). A concurrent session's `clean test --stale` reaches the
  same per-case paths. That is `w14-final-audit.md` §7's "`⚡️cache/tests` is shared, not per-run"
  again, at a new symptom: **a sweep during a run reads as fourteen failing scenarios.** Both runs
  were repeated and were green.

## 7. STEP AP214 — checked, not inherited, and the answer is still no

The brief asked whether ifcopenshell's Part-21 layer, or another installed library, can be a real
writer for AP214. Four candidates were checked against the real committed fixture
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp`
(79 984 bytes, `FILE_SCHEMA(('AUTOMOTIVE_DESIGN'))`).

| candidate | verdict | evidence |
|---|---|---|
| `ifcopenshell` core | **no** | `ifcopenshell.open` and `ifcopenshell.file.from_string` both raise `SchemaError: Unsupported schema: AUTOMOTIVE_DESIGN`. Its Part-21 layer is schema-GATED, not schemaless |
| `ifcopenshell.simple_spf` | **no**, twice over | Its file API is `by_id, by_type, header, mvd, schema, schema_identifier, schema_version` — **read-only; there is no `write`/`to_string`/`save` anywhere in the package**, and its entity type exposes no public member at all. It also crashes on this fixture: `AttributeError: 'tuple' object has no attribute 'children'` inside its own `parser/transformer.py:90`. (It parses the IFC4 fixture fine, so the invocation is correct) |
| every other installed Python distribution | **no** | Complete site-packages inventory: `PIL`, `dateutil`, `fitz`/`pymupdf`, `ifcopenshell`, `isodate`, `lark`, `mpmath`, `numpy`, `pypdf`, `yaml`, `shapely`, `sympy`, `toml`, `typing_extensions`. None reads or writes STEP |
| `brepjs` / `brepjs-opencascade` (npm, present) | **no**, twice over | It *does* export `importSTEP`/`exportSTEP` (OpenCASCADE WASM). But (i) `🔒️dependencies.json` classifies both as `production-runtime`, `productionReachable: true`, reachable from `🌐️spatial-kernel`, `📐️cad` and five more manifests — registering either as an oracle is a `testing/dependency: oracle-in-production` breach by this repository's own rule; and (ii) it is a GEOMETRY KERNEL, not a Part-21 record editor: `STEPControl_Writer` regenerates the entity graph from OCC topology, so it can neither express nor observe `set-entity-arg` / `insert-entity-arg` / `remove-entity-arg` / `set-entity-name` / `set-product-identity` / `demote-shape-representation`, and it cannot preserve the id-keyed entity graph the AP214 comparison profile compares |
| `ruststep` (already registered) | **no** | `ast::ser::to_record` only builds an in-memory `Record` from an already-typed struct; no `Display`/`fmt::Formatter` impl on `Exchange`/`DataSection`/`Record`/`Parameter` anywhere in the crate. Confirmed again, not inherited |

**Conclusion: no available implementation can serve as a second PRODUCER for ISO 10303-21 AP214.**
The seven `📐️step` cases are therefore left exactly as they are — honestly typed `@mode-property` /
`@mode-round-trip` with `ruststep` as the independent READER. That is a measurement now, not an
assumption carried forward. The previous wave's reason ("`ruststep` has no writer") was true but
incomplete; the complete reason is that the only two STEP writers reachable from this machine are a
schema-gated IFC library and a production-reachable geometry kernel, and neither speaks the
record-level vocabulary these cases mutate.

---

## 8. Nothing was weakened — checked eight ways

1. **No comparison-profile knob changed anywhere.** `git diff -- '*🔣️component.json'` filtered to
   `^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"` returns **nothing**.
2. **The three manifest edits are strictly additive.** `git diff --numstat`: `6/0`, `12/0`, `12/0` —
   **zero deleted lines** in all three.
3. **No existing feature file was touched.** `mutate-ifc-4`, `mutate-ifc-2x3` and the ten other
   `🏗️ifc`/`📐️step` cases are byte-identical to what this wave found. (`mutate-ifc-4`'s feature does
   carry one unstaged one-line change — a peer session removing a stale `parity=1/23` from prose at
   05:51, before this wave's edits; no scenario or assertion changed.)
4. **No scenario was deleted and no assertion removed.** Everything here is new files.
5. **No fixture was added, deleted, swapped or normalised.** Both cases read the artifacts already
   committed, at their real committed sizes.
6. **The migration ratchet was not touched.** `🔒️migration.json` unchanged.
7. **The dependency gate is green and one blind spot is now closed.** `bun ./📜️script.ts dependency`
   → **exit 0**, `ecosystems=4 entries=233 production-reachable=151 test-oracle=31`, with
   `[dependency] test-oracle python:ifcopenshell@0.8.4.post1 (ifcopenshell-ifc-2x3-any-differential,ifcopenshell-ifc-4-any-differential)`.
   Unlike Pillow and three.js (`w14-final-audit.md` §3.3), this new third-party reference **is**
   declared, pinned and classified. The runner rebuilt its interpreter for the new package set and
   verified the pin: `signature: "ifcopenshell==0.8.4.post1 pypdf==6.14.2 simplejson==4.1.1"`.
8. **The contract gate reports nothing against these cases.** Final run:
   `bun ./📜️script.ts contract` → exit 1 with **2** breaches, both `testing/discovery`
   `unmanaged-tests` — the pre-existing overage `w14-final-audit.md` §1.1 attributes to other
   sessions, unchanged in count. Both `differential-ifc-*` cases pass `case-slug`, `no-adapter`,
   `missing-capability`, `missing-comparison`, `unknown-oracle`, `oracle-capability-mismatch`,
   `oracle-profile-mismatch`, `differential-without-evidence`, `missing-fixture`, `orphan-fixture`,
   `unknown-case-child` and `unknown-adapter-filename`. `bun ./📜️script.ts discover` → **166** test
   cases, 164 + these two. (An intermediate run also showed 2 `testing/fixture` orphans under
   `🧿️semio/🧪️tests/mutate-semio-kit/🧫️fixtures/`, created at 06:37 today by a live peer session
   upgrading that case's artifact; they had resolved by the final run.)

   The sibling cases `mutate-ifc-4` and `mutate-ifc-2x3` were **not re-executed** — their files are
   byte-identical to what this wave found (§8.3), and re-running them means a cold Rust oracle-host
   compile each. That is stated rather than implied.

**No measured ratio was written into any source file.** `grep -rn "parity=" ` over every file this
wave created returns nothing. The byte counts and instance counts that do appear in the docstrings
are properties of committed fixtures and of a pinned library version — what the case ASSERTS and what
bounds it — not what it last scored.

---

## 9. Files

Created:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-4/component.feature`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-4/🐍️component.py`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-4/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-2x3/component.feature`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-2x3/🐍️component.py`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-2x3/🦀️component.rs`

Modified (additive only):

* `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️component.json` — one `oracleHostPackages` entry, `ifcopenshell 0.8.4.post1`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` — oracle `ifcopenshell-ifc-4-any-differential`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` — oracle `ifcopenshell-ifc-2x3-any-differential`

Ticket working files (`w23-ifc-differential/`): `probe_part21.py`, `negative_controls.py`,
`crosscheck.py`, `ruststep-crosscheck/` (standalone `[workspace]` crate), `oracle-projections.txt`.

---

## 10. What is still open

1. **Repo-wide parity.** These two cases are `15/15` and `8/8`; the other 162 cases were not run
   here. The subject host now links, so the number `w14-final-audit.md` reported as `0/0` is
   measurable again — but only with `SEMIO_TEST_BUDGET_MS` raised, because the stock 900 s per-case
   budget is consumed by the cold compile (§6.2).
2. **The four IFC4 kinds and one IFC2X3 kind with no second producer.** They are bounded by
   IfcOpenShell's schema binding, not by effort. A Part-21-level third-party writer would close them;
   none exists in any ecosystem reachable from this machine.
3. **AP214 has no second producer at all** (§7), and that is now measured rather than assumed.
4. **The shared test cache is still not per-run** — this wave lost two runs to a concurrent sweep
   (§6). `w14-final-audit.md` remedy #7.
