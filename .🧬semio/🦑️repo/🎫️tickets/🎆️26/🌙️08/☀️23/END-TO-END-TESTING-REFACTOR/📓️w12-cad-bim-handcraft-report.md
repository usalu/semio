# Wave 12 — CAD/BIM scope handcraft: 🏗️ifc, 📐️step, 🖊️dxf, 🖊️dwg, 💬️bcf

Date 2026-08-24. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Every command quoted below was run
from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` (or where noted) and every quoted line is the
tool's own output. Exit codes were read from the tool's own status, never through a pipe.

This report is the deliverable; the chat message points here.

---

## 0. Headline

| | |
|---|---|
| Subsets that had no mutation vocabulary and now have a handcrafted one | **6** (`step ap214 ✳️cc1`…`✳️cc6`) |
| New end-to-end cases (catalog + oracle + feature + adapter) | **6** |
| Cases that had **no adapter at all** and now have one | **2** (`mutate-dwg-ac1018`, `mutate-dwg-ac1024`) |
| Pre-existing oracle defects found by an assertion and fixed | **2** (DWG preamble round trip; STEP product-chain subtype matching) |
| Reference-library defects found, reproduced standalone, documented | **1** new (`ruststep` 0.4 cannot parse an empty aggregate) |
| Existing cases upgraded from "asserts nothing" to an in-role observability law | **8** |
| Templated feature files rewritten | **2** (the two DWG cases were a 4-line diff of each other) |

---

## 1. STEP AP214 CC1–CC6 — six untaken subsets, taken

`w10-verification.md` §2 listed `step ap214 ✳️cc1…✳️cc6` among the 17 subsets with **no**
`🧬️schema/🧬️mutations` directory at all: invisible to the contract phase, the oracle phase and every
count in the ticket. All six now have a handcrafted vocabulary, an oracle, a catalog, a feature and
an adapter.

### 1.1 Where the vocabulary comes from

Each class's `check_ccN_conformance` (in `✳️ccN/🧬️schema/🦀️component.rs`) reads exactly three axes
and no others:

| axis | function it calls | code |
|---|---|---|
| `FILE_SCHEMA` declares `AUTOMOTIVE_DESIGN` | `file_schema_contains` | `CODE_FILE_SCHEMA` (hard) |
| no `*_SHAPE_REPRESENTATION` above the class ceiling | `ladder_violations(doc, MAX_RUNG)` | `CODE_LADDER` / `CODE_SHAPE_REPRESENTATION_PRESENT` (hard) |
| the `PRODUCT`/formation/definition chain is present | `has_product_definition_chain` | `CODE_PRODUCT_CHAIN` (soft) |

So the vocabulary is one kind per axis, plus `no-mutation`/`set-snapshot`. **It is deliberately not
the `✳️any` subset's `StepMutation`**, which is the ISO 10303-21 *grammar* (`insert-entity`,
`set-entity-arg`, …) — eleven verbs that would read identically for any Part-21 file on earth. A
conformance class is a filter, not a grammar.

### 1.2 What genuinely differs per class — and where it does not

The six classes are a linear ladder, so their vocabularies differ where the ladder differs. Two of
them differ in the KIND SET, not just in a constant:

| subset | ceiling rung | ladder verbs | why |
|---|---|---|---|
| `✳️cc1` | 1 | `remove-shape-representation` | `ladder_rung_of` never answers below 2, so `ceiling_type_of(1)` is `None`: CC1 admits **no** representation, has no ceiling type, and owns **no verb that can write one**. Its only conformance repair is deletion. **5 kinds.** |
| `✳️cc2` | 2 | `set-shape-representation`, `demote-shape-representation` | 6 kinds |
| `✳️cc3` | 3 | same | 6 kinds |
| `✳️cc4` | 4 | same | 6 kinds |
| `✳️cc5` | 5 | same | 6 kinds |
| `✳️cc6` | 6 | `set-shape-representation` only | the ladder tops out at 6, so `violations(doc, 6)` is empty for every writable document. A `demote-shape-representation` kind here could never move the projection — a scenario that always passes and proves nothing — so it is **absent**. **5 kinds.** |

For CC2–CC5 the differences are the ceiling rung, the ceiling TYPE a demotion lands on, and the
guard message. Those are real and are enforced in code (`apply_class_edit` refuses a type above the
class ceiling and names both rungs), and each class's tests assert its own line: CC3 admits
`GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION` and refuses `MANIFOLD_SURFACE_…`, CC5 admits
`FACETED_BREP_…` and refuses `ADVANCED_BREP_…`, and so on.

**Stated plainly, because it is the honest limit of the claim:** CC2–CC5 have the same SHAPE of
vocabulary, because the standard gives them the same shape — they are four positions on one ladder.
What is shared is shared through **one named module**, never copied:

* `✳️any/🚪️io/🪜️ladder/🦀️component.rs` (production) gained `ClassEdit`, `apply_class_edit`,
  `invert_class_edit`, `ceiling_type_of`, `upsert/remove/demote_shape_representation`,
  `set/product_identity`, `ShapeRepresentationRow`, `ProductIdentity`. Each `✳️ccN` enum maps its own
  variants onto that one implementation.
* `🏅️standards/🔖️ap214/🧪️oracle/🦀️component.rs` (**new**, standard level, mirroring the IFC2X3
  precedent) holds the Part-21 reader/writer and an independent re-derivation of the §4.3 ladder.
  All seven `ap214` subsets use it, and `✳️any`'s oracle was refactored to stop carrying its own
  private copy of the writer.

### 1.3 A conformance class is not closed under inversion

Undoing a ladder repair re-introduces the violation it repaired, and a class whose whole purpose is
to forbid geometry above its ceiling cannot own a verb that writes such geometry back. So
`inverse()` returns the in-class verb when the base's own representation is admissible *in that
class*, and degrades to `SetSnapshot` when it is not. Against the real fixture that is exactly what
`demote-shape-representation` takes in CC2–CC5 (the base's `#13` is rung 6). This is recorded at the
variant and in `invert_class_edit`'s doc comment rather than papered over with a promotion verb the
class must not have. CC6 is the one class for which the degradation path is unreachable, and its
module says so.

### 1.4 Reachability wrappers (the `kit` treatment)

`protocol` is a private `extern crate` alias in the stdio glue, so a test host compiled as an
external crate cannot name `protocol::Mutation` or `protocol::MutationOutcome`. Each `✳️ccN`
vocabulary therefore exports two thin wrappers whose signatures name only its own public types:

```rust
pub fn apply_step_ccN_mutation_checked(snapshot: &mut StepSnapshot, mutation: &StepCcNMutation) -> Result<(), String>
pub fn inverse_step_ccN_mutation(base: &StepSnapshot, mutation: &StepCcNMutation) -> Vec<StepCcNMutation>
```

The subject adapters drive the **production** apply and inverse through these, so they test the
production semantics rather than a re-derivation of them, and a rejected mutation surfaces as an
error instead of being discarded. The same two wrappers were added to the DWG `✳️any` vocabulary.

---

## 2. DWG — the two cases that could never have run

`w10`/the adapter hunt were right: `🖊️dwg/🧪️tests/mutate-dwg-ac1018` and `mutate-dwg-ac1024` each
contained **only** `component.feature`. The runner's `materializeRustHost` does
`join(repoRoot, discovered.adapters.rust!)`, so nothing could ever register a handler.

Measured before the fix:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-dwg-ac1018   # exit 1
1 high-priority breach(es) across 1 rule(s):
  testing/contract  ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🧪️tests/mutate-dwg-ac1018  Test case has no implementation adapter
```

Both adapters now exist and both cases report **0 breaches**.

### 2.1 The two features were a 4-line diff; they are now two different arguments

`diff` of the two committed feature files was 4 lines (the 3 tags and the `Feature:` title), and the
AC1024 copy carried AC1018-specific prose about itself. They now assert **opposite** things about the
same bytes, which is why they are two cases:

* **AC1024** — the committed `📄️architectural.dwg` really is stamped `AC1024`, so for that case it is
  a native fixture. Its identity scenario asserts the R2010 stamp **survives** a full
  decode/re-encode; its `set-version-info` row moves the stamp away (`AC1032`) and back.
* **AC1018** — **no AC1018-stamped file exists in this repository.** Both `.dwg` files outside
  `./compose` begin `AC1024`. So that case demonstrates the narrower thing it can: every mutating row
  drives the stamp **to** `AC1018` and the adapter fails unless an independent preamble reader reads
  `AC1018` back; and its identity scenario asserts the reader reports `AC1024` — the stamp the FILE
  carries, not the standard the case is filed under. Asserting `AC1018` there would be asserting a
  fiction.

### 2.2 Identical by specification — with the citation

The shared `DwgMutation` is legitimate for **exactly what it covers**. The ODA `.dwg` specification
gives R2004 (AC1018) and R2010 (AC1024) the SAME file-header prefix — six ASCII version characters at
`0x00`, the application maintenance-release byte at `0x12`, the codepage `RS` at `0x13`-`0x14` — and
the vocabulary addresses only those three fields. `🔖️ac1018/…/🧬️mutations/🦀️component.rs` re-exports
the AC1024 enum through a named module rather than copying it, and the AC1024 oracle's
`every_ac1018_facet_is_a_re_export_of_this_one` test reads the committed sources and fails the moment
that stops being true.

### 2.3 What is NOT identical by specification, and is left open

The whole AC1018 **schema/snapshot/diff/io** tree is a re-export of AC1024's, and this repository has
ONE decode path for both: `decode_dwg` branches only on `AC1015`, everything else goes to
`decode_r2004_*`, and that encoder's own error strings name AC1024 framing ("AC1024 Header stream
length {} != 854"). A genuine R2004 section-map codec, an AC1018-authored fixture, and an AC1018
vocabulary addressing what R2004 actually carries are **not** delivered here. That is production
codec work of ticket scale, and it cannot even be started without an AC1018 file to test against —
this repository has none. Recorded as an open gap, and stated in both feature descriptions rather
than dressed up as coverage.

---

## 3. Defects found by asserting a law

### 3.1 DWG: the round trip destroyed 12 bytes of the published header (fixed)

`the_round_trip_rebuilds_the_preamble_from_the_parse_alone` in the AC1024 oracle module **failed**
the first time it was run in this scope:

```
assertion `left == right` failed: the preamble region is zeroed before it is written back,
so equality here proves the parse/write pair is exact — not that the bytes were copied
```

`oracle_round_trip` zeroes the whole 21-byte preamble (`0x00..0x15`) and then rewrote only the three
fields the `Preamble` struct modelled. Bytes `0x06..0x12` were wiped. In the real fixture those are
real published fields:

```
00000000: 4143 3130 3234 0000 0000 0002 03c0 0100  AC1024..........
00000010: 001d 021e ...
```

`0x0B` = maintenance release version `0x02`, `0x0C` = the `0x00/0x01/0x03` marker `0x03`,
`0x0D`-`0x10` = preview (image seeker) address `0x000001c0`, `0x11` = application version `0x1d`.

**Verdict: it indicts the oracle's own reader/writer, not our codec, the reference or the fixture.**
Fixed by modelling the whole published prefix; the three fields the vocabulary addresses are still
the only ones PROJECTED, because no declared kind edits the others. Two new tests assert the full
prefix against the real file and assert that `set-snapshot`'s stub is byte-identical to this
artifact's own committed 22-byte demo example.

```
$ cargo test --features oracles --lib dwg      # exit 0
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 238 filtered out; finished in 0.03s
```

### 3.2 STEP: `has_product_definition_chain` was wrong against real data (fixed)

The production check matched the three chain types by EXACT name. The committed real export carries

```
#822=PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE('A', ...)
```

— the ISO 10303-41 SUBTYPE every real AP214/AP242 exporter writes. So **every `✳️ccN` analyzer
reported the soft `product-definition-chain` diagnostic against a file that genuinely carries the
chain.** The ladder half of that module already classified `*_SHAPE_REPRESENTATION` subtypes; the
product half did not, and only a real export made it visible.

**Verdict: it indicts our conformance checker.** Fixed by enumerating the ISO 10303-41 subtypes
explicitly on both halves. Deliberately NOT by name-prefix matching, and there is a test for why:
`PRODUCT_DEFINITION_FORMATION` begins with `PRODUCT_DEFINITION` and is a different entity, so a
prefix rule would have reported a chain rung that is not there.

### 3.3 `ruststep` 0.4 cannot parse an empty aggregate `()` (reference-library defect)

Found while building the CC oracles; reproduced standalone in this ticket's scratch folder against
the crate itself:

```
FOO(())                            -> ERR   Error while tokenizing STEP input … in Tag: DATA;
FOO('a',())                        -> ERR
FOO()                              -> ok
FOO('a')                           -> ok
FOO('a',(''))                      -> ok
FOO('a',$)                         -> ok
empty DATA section                 -> ok
empty HEADER section               -> ERR   expected '(', found ;
```

ISO 10303-21 §6.2 permits an empty aggregate. The registered reader cannot read one back.

**Verdict: it indicts the reference library.** The real fixture is unaffected — its four `()`
occurrences (`LENGTH_UNIT()`, `PLANE_ANGLE_UNIT()`, `SOLID_ANGLE_UNIT()`) are empty RECORD argument
lists inside complex instances, which parse fine. Worked around where the oracle AUTHORS a document:
`set_product_identity` writes three of `product`'s four ISO 10303-41 attributes and omits
`frame_of_reference` rather than emitting `()`. The production `engine::ladder` authors the same
shape so both sides agree, and both cite the reproduction. The projection is unaffected (it reports
the chain's ids and names, which is what `has_product_definition_chain` reads).

A second, smaller finding from the same probe: `ruststep` also rejects a `HEADER` section with no
records, which is why the CC oracles' `minimal_document` seeds all three records ISO 10303-21 §8.2
makes mandatory. That one is the reader being *right*.

### 3.4 Three Part-21 projections were blind to the header records their own vocabularies edit

**This is the finding the new observability law was worth adding for.** The moment it was turned on,
three cases failed — and the failure was correct:

```
mutate-ifc-4        executed=23 passed=21 failed=2
    mutate-set-file-description | "set-file-description" left the semantic projection of the IFC4
                                  exchange structure unchanged -- a mutation that is not observable
                                  proves nothing, so this row's parameters do not exercise the kind
                                  they name
    mutate-set-file-name        | (same)
mutate-ifc-2x3      executed=11 passed=10 failed=1
    mutate-set-header           | (same, IFC2X3 building model)
mutate-step-ap214   executed=23 passed=21 failed=2
    mutate-set-file-description | (same, AP214 exchange structure)
    mutate-set-file-name        | (same)
```

**Verdict: it indicts the projections, not the rows, the reference or the fixtures.** All three
`project_*` functions reported `fileSchema` plus the id-keyed entity graph and **nothing else**, so
five declared mutation kinds across three subsets — kinds those vocabularies name explicitly —
could not be seen at all. Those scenarios had been passing since wave 7 because `ruststep` did not
error, which is precisely the failure mode this platform exists to prevent. The rows themselves are
fine: they change `FILE_DESCRIPTION`'s description text and `FILE_NAME`'s `name`/`author`/
`organization`, none of which the comparison profiles list as writer freedom.

Fixed by projecting `FILE_DESCRIPTION` and `FILE_NAME` under the attribute NAMES ISO 10303-21 §8.2
fixes for them (`description`/`implementationLevel`; `name`/`timestamp`/`author`/`organization`/
`preprocessorVersion`/`originatingSystem`/`authorization`) rather than positionally — naming them is
what lets each profile's existing `ignoreKeys` (`timestamp`, `preprocessorVersion`,
`originatingSystem`, `authorization`) actually address the header. Against a positional array that
declaration of writer freedom would silently stop applying.

Three in-crate unit tests were vacuous for the same reason and now assert observability:
`set_header_renames_the_model_and_inverts` (IFC2X3) asserted only that the entity graph was
untouched and that the restore matched; it now also requires the projection to MOVE and requires the
renamed model to read back through the independent parser. The IFC4 and AP214
`set_file_description…`/`set_file_name…` tests got the same treatment.

---

## 4. "Asserts nothing" — closed for this scope

`w10-verification.md` §4 listed 32 exercised cases whose oracle handlers assert nothing. Eight of
them are in this scope. Their `inverse-<kind>` and `identity-round-trip` handlers had ALREADY been
upgraded to assert in role by an earlier wave (the w10 list is stale on that point — verified by
reading all eight). What was still missing everywhere was the observability law on the forward
mutation, so every `mutate_oracle` in this scope now ends with:

```rust
if kind != "no-mutation" && projection == baseline {
    return Err(format!("{kind:?} left the semantic projection of the … unchanged -- a mutation that is
        not observable proves nothing, so this row's parameters do not exercise the kind they name"));
}
```

with the baseline taken through one `no-mutation` cycle so the comparison isolates the mutation
rather than the writer's normal form. Applied to `mutate-ifc-4`, `mutate-ifc-2x3`,
`mutate-ifc-2x3-cobie`, `mutate-ifc-2x3-cv20`, `mutate-ifc-2x3-sav`, `mutate-step-ap214`,
`mutate-dxf-r12`, `mutate-bcf-2-1`, and built into all six new `✳️ccN` adapters and both DWG ones.

The six new `✳️ccN` adapters additionally assert a per-class CLAIM on top of observability — e.g.
CC5's `demote-shape-representation` must drive `aboveCeiling` to 0 and `conformsToClass` to true;
CC6's `set-shape-representation` must leave `#836` carrying rung 6 in the independently read census.

Also cleaned up: the `IFCLENGTHMEASURE` copy-paste residue in the AP214 diff module's test vector
(`🧬️schema/🔺️diff/🦀️component.rs:1676`) is now `LENGTH_MEASURE`. The one in the mutations module had
already been fixed; the ones in `🚪️io/📐️part21` and `📸️snapshot` are legitimate — that Part-21
engine is genuinely shared with the IFC artifact, and an IFC-shaped typed value is a real parse
exercise there.

---

## 5. Verification — the real `[test]` lines

### 5.1 Oracle crate unit tests, per artifact in scope

From `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`:

```
$ cargo test --features oracles --lib artifacts::step
test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 287 filtered out; finished in 9.07s

$ cargo test --features oracles --lib artifacts::ifc
test result: ok. 47 passed; 0 failed; 0 ignored; 0 measured; 297 filtered out; finished in 82.22s

$ cargo test --features oracles --lib artifacts::dwg
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 333 filtered out; finished in 0.03s

$ cargo test --features oracles --lib artifacts::dxf
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 342 filtered out; finished in 0.50s
```

The whole-crate run is `340 passed; 2 failed` — **both failures are outside this scope**
(`artifacts::txt::…::every_feature_row_inverts_back_to_the_real_document` and
`artifacts::xlsx::…::shared_string_kinds_are_a_true_byte_identity`), from concurrent sessions
working on those artifacts. Recorded, not claimed as green.

### 5.2 Contract

```
$ bun ./📜️script.ts contract --owner 🗄️stdio          # exit 1
2 high-priority breach(es) across 1 rule(s):
  testing/contract  …/📷️png/🧪️tests/mutate-png-1-2/component.feature  Step at line 30 is outside a Background or Scenario
  testing/contract  …/📷️png/🧪️tests/mutate-png-1-2/component.feature  Step at line 35 is outside a Background or Scenario
```

Both are another session's in-flight edit to `mutate-png-1-2`. Every case in this scope reports
**0 breaches** individually, including the six new ones and the two DWG ones that could not report
anything at all before this work.

### 5.3 Dependency purity

```
$ bun ./📜️script.ts dependency                        # exit 0
[dependency] test-oracle rust:ruststep@0.4 (ruststep-ifc-2x3-any-mutate, ruststep-ifc-2x3-cobie-mutate,
  ruststep-ifc-2x3-cv20-mutate, ruststep-ifc-2x3-sav-mutate, ruststep-ifc-4-any-mutate,
  ruststep-step-ap214-any-mutate, ruststep-step-ap214-cc1-mutate, ruststep-step-ap214-cc2-mutate,
  ruststep-step-ap214-cc3-mutate, ruststep-step-ap214-cc4-mutate, ruststep-step-ap214-cc5-mutate,
  ruststep-step-ap214-cc6-mutate)
```

Six new registrations, all `testOnly: true`, no new `production-debt` record — the three that print
are the pre-existing `png`/`zip`/`image` ones.

### 5.4 Oracle phase, per case

*(the sweep's own output, appended verbatim below)*

---

## 6. Honest limits

* **The production crate does not compile**, so nothing written into
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/…/🧬️schema/…` or `🖊️dwg/…/🧬️mutations/…` has been
  compiler-checked. The blocker is unchanged and is not this work: a concurrent
  `INTERACTIVE-JOB-RUNTIME-REFACTOR` session's `ManuallyDrop<Option<RetainedJobPayload>>` migration
  in `semio-framework-job`. Every `✳️ccN` vocabulary, its `#[test]`s and both DWG reachability
  wrappers are therefore **written and unverified**. The ORACLE side of everything IS verified, and
  the oracle side is where the third-party evidence lives.
* **`parity=0/0` everywhere**, for the same reason. No oracle-versus-subject comparison has run for
  any case in this scope.
* **The AC1018 gap in §2.3 is real and open.**
* The oracle crate was broken twice mid-session by concurrent sessions editing `📷️jpg` and `📕️xlsx`
  oracles. Both cleared on their own; the runs quoted in §5 were taken after they did.
