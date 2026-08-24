# Wave 11 — final re-audit after the de-stubbing wave

Date 2026-08-24. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Successor to `📓️w10-verification.md`.
Every command below was actually run, from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` unless
noted, and every quoted line is real output copied verbatim. Exit codes were read from the tool's own
exit status, never through a pipe.

---

## 0. Headline — what is still not what it appears to be

1. **Three of the six commands are RED, and two of them for the first time.** `oracle exhaustive`
   exits **1** (`passed=1320 failed=1`); `cargo test --features oracles --lib` exits **101**
   (`344 passed; 1 failed; 2 ignored`); `cargo check -p semio-framework-os-kernel --lib` exits 101.
   Both new reds are deliberate, documented, root-caused failures — that is the good news — but w10's
   "exit 0 / passed=1011 failed=0" and "208 passed; 0 failed" lines are stale greens now, and anyone
   quoting them is misreporting the suite.
2. **Coverage is 86/88, not 88/88.** `jpg jfif-1.01/✳️baseline` and `tiff 6.0/✳️baseline` each got a
   real, handcrafted 9-kind mutation vocabulary this wave (440 and 435 lines) and then **nothing
   else**: no `🧪️oracle` directory at all, no manifest, no catalog, no case. They are invisible to
   the contract gate, so nothing will ever notice. Both are staged-but-uncommitted (`git status` `A`),
   i.e. this wave created the gap it left.
3. **Four executed cases still assert NOTHING on the oracle side**, and they are four of the biggest:
   `mutate-pdf-1-7`, `mutate-pdf-1-4`, `mutate-docx-ecma-376`, `mutate-pptx-ecma-376`. Their
   `mutate_oracle` bodies apply, project and return. That is **42 `mutate-<kind>` scenarios** whose
   green means only "`lopdf`/`quick-xml` did not error". Their subset oracle modules also carry no
   module-level observability test, so nothing catches a silently-no-op arm there either.
4. **`parity=0/0` still holds across the whole ticket.** No oracle-versus-subject comparison has ever
   run for any case. `cargo check -p semio-framework-os-kernel --lib` still fails, so the Rust subject
   phase is still uncompilable. The differential claim this platform exists to make remains unmade.
5. **The os-kernel status line in w10 is wrong in the other direction now.** The `ManuallyDrop`
   migration in `semio-framework-job` has LANDED — that crate compiles. The 3 remaining errors are
   inside `semio-framework-os-kernel` itself, in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`,
   and **`ArtifactEnvelope: Clone` is back**. Answer to the question as posed: **no, they are not the
   `semio-framework-job` `ManuallyDrop` migration.**
6. **565 scenarios across 23 cases still execute in no phase.** 19 semio Pattern-B subsets +
   `binary-raw` + `txt-utf-8` + the two DWG cases carry recorded `noOracleDecision`s, so the runner
   never dispatches their oracle role, and their stated fallback ("the subject phase") has never run.
7. **Two documented weakenings, both narrow and both stated in the feature.** `mutate-pdf-1-7` exempts
   `pages.N.contentOperators` for 3 of its 18 kinds; `mutate-jpg-jfif-1-01` declares 5 of its 12 kinds
   unobservable and `mutate-png-1-2` declares 2 of 17. Nothing was hidden, but 42+7 kinds carry less
   evidence than the headline implies.
8. **One identical-catalog group is not documented as such.** `step-ap214-cc2/cc3/cc4/cc5` declare the
   SAME six kinds. The implementations genuinely differ (`MAX_RUNG` 2/3/4/5 over a shared `ladder`
   module), and each manifest explains its own class — but none of the four says "these four catalogs
   are identical, and here is why", the way the IFC4/AP214 pair and the DWG pair both do.
9. **One feature file overstates a failure that no longer happens.** `mutate-svg-1-1`'s description
   still carries "⚠️ OPEN, and left red rather than tuned away: `inverse-remove-element` FAILS on the
   ORACLE side today". It does not: the clean run reports `inverse-remove-element passed` in all three
   SVG cases, because the remedy that paragraph itself prescribed was applied
   (`oracles::apply_mutation_inverse` now parses ONCE and applies the forward step and its inverse to
   the same tree). Stale in the safe direction, but stale.
10. **The runner crashed on the first attempt at command 2** with a `markRunComplete` `ENOENT` on the
   `mutate-png-1-2` work directory after ~35 minutes of work. The clean re-run succeeded. Worth its
   own ticket.

### What genuinely improved since w10

* The shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` module now exists: `inverse_restores`,
  `mutation_is_observable`, `round_trip_preserves`, `reparsed_not_copied`, `carrier_is_exact`, each
  with its own unit tests. w10's "32 of 46 exercised cases assert nothing" is now **60 of 86 asserting
  observability, 66 of 86 asserting the inverse law, 66 of 86 asserting identity** in the oracle role.
* Coverage went 67/88 → 86/88 cases: `mp3`, both DWG standards and `semio ✳️drawing` all got catalogs
  and cases; 88/88 subsets now have a `🧬️mutations` directory.
* w10 item 6 closed: `rust_xlsxwriter` and `markup5ever_rcdom` are now registered oracle entries.
* w10 item 7 closed: the i-json adapter header now says json-rust and explicitly "NOT `serde_json`".
* w10 item 8 closed: `IFCLENGTHMEASURE` is gone from the AP214 mutation module.
* w10's exhibit A is fixed. `mutate-pdf-1-7`'s `identity_round_trip_oracle`, quoted in w10 as naming
  the no-byte-pass-through law in a doc comment whose body never checked it, now checks both halves:
  `if bytes == input { return Err("byte pass-through: …") }` and a `first_divergence` on the
  projection. Its `mutate_oracle`, however, is still vacuous — see §4.
* Eleven real failures were found by asserting the laws (`📓️w11-real-failures.md`). Four were fixed
  properly, two are left RED on purpose with root-cause analysis, one was replaced by a STRONGER law,
  and one (`🎨️svg inverse-remove-element`) was fixed but its feature still says it is red.

---

## 1. The six commands, verbatim

### 1. `bun ./📜️script.ts contract` — exit 0

```
0 high-priority breach(es) across 0 rule(s):


full breach set (including non-blocking priorities): /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/breaches/testing.json
```

`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` read directly: content is `[]` (3 bytes). No
non-blocking breaches hide behind the high-priority count.

⚠️ The gate cannot see the two uncovered subsets. `jpg ✳️baseline` and `tiff ✳️baseline` declare no
catalog at all, and "a subset with a mutation vocabulary but no catalog" is not a rule the contract
checks — only "a catalog with no scenario" and "a scenario exercising an undeclared kind" are.

### 2. `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio`

— **exit 1**

```
[test] level=exhaustive cases=99 executed=1321 passed=1320 failed=1 errored=0 parity=0/0 not-exercised=23
```

Preceded by exactly 23 `[test] not-exercised …` lines (the full list is in §6). **`executed` went
1011 → 1321** and `cases` 80 → 99. `parity=0/0` is unchanged: no oracle-versus-subject comparison has
happened for any case in this ticket.

**The one failure, read out of `.🧬semio/🦑️repo/⚡️cache/tests/results/…-mutate-obj-3-0-oracle-rust/📤️results.jsonl`:**

```json
{"testId": "…/🧊️obj::mutate-obj-3-0::inverse-remove-face::rust::oracle", "case": "mutate-obj-3-0",
 "scenario": "inverse-remove-face", "role": "oracle", "status": "failed", "durationMs": 586,
 "diagnostics": [{"severity": "error", "message": "inverse law violated: applying \"remove-face\" and then its own inverse did not restore the original — $.vertexCount is 8577, expected 8576"}]}
```

⚠️ **The first run of this command crashed rather than finishing** — a `markRunComplete` `ENOENT` on
`…/⚡️cache/tests/work/test-s-plugins-stdio-artifacts-png-8af956-mutate-png-1-2-oracle-rust/🏁️done`
after ~35 minutes. It was re-run clean from scratch and the numbers above are from the clean run.
That crash is a runner robustness bug worth its own ticket; a reader who saw only the first run would
have concluded the suite is broken.

### 3. `bun ./📜️script.ts dependency` — exit 0

```
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
```

`test-oracle` went 27 → **30**. The two crates w10 flagged as escaping the purity gate are now
registered and appear in the listing:

```
[dependency] test-oracle rust:markup5ever_rcdom@0.39 (html5ever-html-5-mutate)
[dependency] test-oracle rust:rust_xlsxwriter@0.96 (xlsx-ecma-376-mutate)
```

The three `production-debt` records are unchanged and all pre-existing. **w10 item 6 is closed.**

### 4. `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — exit 0

```
bun test v1.3.14 (0d9b296a)

 69 pass
 0 fail
 1682 expect() calls
Ran 69 tests across 1 file. [91.78s]
```

(w10 measured 1458 `expect()` calls over the same 69 tests; the suite gained assertions, not cases.)

### 5. `cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — **exit 101**

```
test result: FAILED. 344 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 64.11s
```

```
failures:
    artifacts::txt::standards::v_utf_8::subsets::any::component::tests::every_feature_row_inverts_back_to_the_real_document
```

```
thread 'artifacts::txt::standards::v_utf_8::subsets::any::component::tests::every_feature_row_inverts_back_to_the_real_document' panicked at
🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs:531:13:
assertion `left == right` failed: applying set-trailing-newline and then its own inverse did not
restore the real document (24874 bytes back, 24875 in) — see this test's own doc comment for the
`(lines, trailing_newline)` non-injectivity this exposes
```

**This is a deliberate, well-argued red, not a regression to hide.** The module's own doc comment
names the defect as living in the SUBSET'S OWN DATA MODEL: `(lines, trailing_newline)` is not an
injective encoding of a body — `(["a"], true)` and `(["a", ""], false)` both render `"a\n"` — the real
fixture ends `"…conversation.\n\n"`, and `set-trailing-newline(false)` therefore loses a blank line the
inverse cannot recover. It states that the production `TxtSnapshot`/`TxtMutation` share the same
decomposition, so the subject has the identical hole, unmeasured because this is a no-oracle case. A
companion test pins the ambiguity itself so it flips red the moment the vocabulary is fixed.

The **2 ignored** tests are one-shot `#[ignore]`d fixture-derivation helpers in the `bmp v3` and
`tiff 6.0` oracle modules (w10 had 1; the bmp one is new). Neither is a skipped assertion.

### 6. `cargo check -p semio-framework-os-kernel --lib`, from the repo root — **exit 101**

**3 errors, and NO — they are not the `semio-framework-job` `ManuallyDrop` migration.** That crate now
compiles. All three errors are in the kernel's own store module.

```
error[E0046]: not all trait items implemented, missing: `begin_close`, `close_step`, `terminal_is_empty`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6750:1
6750 | impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactEnvelopeDecodeAuthority<P, Mutation>

error[E0277]: the trait bound `ArtifactEnvelope<P, Mutation>: Clone` is not satisfied
    --> …/🏪️store/🦀️component.rs:8845:5
8845 |     pub envelope: ArtifactEnvelope<P, Mutation>,

error[E0308]: mismatched types
    --> …/🏪️store/🦀️component.rs:6637:89
6637 |         semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: diagnostic.code.as_bytes().to_vec() })
     |                                                    expected `RetainedJobPayload`, found `Vec<u8>`

error: could not compile `semio-framework-os-kernel` (lib) due to 3 previous errors; 26 warnings emitted
```

Reading: `semio-framework-job` finished its refactor — `InteractiveJob` gained three items and
`JobFault::detail` became `RetainedJobPayload` — and `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
(mtime 02:55 today; the job module's is 13:50 today, i.e. edited while this verification ran) has not
caught up. `ArtifactEnvelope: Clone`, which w10 declared "gone", has reappeared. This is still a
concurrent session's in-flight framework refactor and not this ticket's regression — but the w10
sentence "that blocker is gone" is now false, and so is "the remaining 5 errors are a `ManuallyDrop`
migration in `semio-framework-job`".

---

## 2. Subset coverage — how many of the 88 have their own vocabulary, catalog and case

Counted over the **88** directories matching
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🏅️standards/*/🪆️subsets/✳️*` (same denominator w10 used).

| | w10 | now |
|---|---|---|
| subsets total | 88 | **88** |
| with their own `🧬️schema/🧬️mutations` directory | 71 | **88** |
| whose manifest declares a `mutationCatalogs` entry | 67 | **86** |
| whose catalog is claimed by a `@mutations-…` feature tag | 67 | **86** |
| `mutate-*` cases under `🗿️artifacts/*/🧪️tests/` | 67 | **86** |
| stdio cases in total | 80 | **99** (86 mutate + 13 pre-existing) |

### The two that still do not have a catalog or a case

* **`📷️jpg 🔖️jfif-1.01 ✳️baseline`** — 440-line handcrafted vocabulary, 9 kinds:
  `no-mutation, set-snapshot, set-sof-marker, set-sample-precision, set-arithmetic,
  insert-huffman-table, remove-huffman-table, insert-frame-component, remove-frame-component,
  set-component-sampling`. Genuinely distinct from `✳️any`'s 12 (which are JFIF-header, quant/huffman
  table and pixel kinds). **No `🧪️oracle` directory at all.**
* **`🖼️tiff 🔖️6.0 ✳️baseline`** — 435-line handcrafted vocabulary, 9 kinds:
  `no-mutation, set-snapshot, set-compression, set-photometric-interpretation, set-bits-per-sample,
  insert-tile-tags, remove-tile-tags, set-strip-offsets, remove-strip-offsets`. Genuinely distinct
  from `✳️any`'s 8 byte-order/IFD/tag kinds. **No `🧪️oracle` directory at all.**

Both were added by this wave and are staged-but-uncommitted. The four subsets w10 named
(`mp3 mpeg1-layer3/✳️any`, `dwg ac1018/✳️any`, `dwg ac1024/✳️any`, `semio v1/✳️drawing`) are now all
covered end to end.

---

## 3. Oracle modules — stubs, rejecting dispatchers, identity arms

66 subsets carry a `🧪️oracle/🦀️component.rs`. The other 22: 19 semio Pattern-B subsets and
`json rfc8259/✳️i-json` (recorded `noOracleDecision`s or, for i-json, a **Python** oracle at
`🔣️json/🧪️tests/mutate-json-rfc8259-i-json/🐍️component.py`), plus the two `✳️baseline` subsets above.

### Rejecting stubs — **none**

Every catalog kind has a matching `"kind"` arm in its subset's dispatcher. The only module with no
arms of its own is `🖊️dwg 🔖️ac1018/✳️any`, a 22-line file whose body is
`pub use crate::artifacts::dwg::standards::v_ac1024::subsets::any::*;` — the documented shared-module
case, not a copy (see §5).

### Arms returning the input unchanged — **exactly one, and it is correct**

`📊️csv rfc4180/✳️any` line 94: `"set-has-header" => Ok(input.to_vec())`. Documented three lines
above ("RFC 4180 carries no header/data distinction on the wire, so toggling the convention never
touches a byte; the caller carries the toggled flag into the comparison projection instead"), pinned
by its own `set_has_header_is_a_true_byte_identity` test, and the case adapter is not let off the
observability law for it — `mutate-csv-rfc4180` calls
`mutation_is_observable(kind, projection, base, &[])` with an EMPTY exemption list and projects the
caller-tracked flag, so the kind must still move the projection. This is the honest shape.

### `todo!` / `unimplemented!` / placeholder `Err` — **none**

Across all 86 case adapters and all 66 subset oracle modules: zero `todo!`, zero `unimplemented!`,
zero `Err("not implemented …")` as a dispatcher body. The only "is not implemented" strings are
catch-all guard arms (`kind => return Err(format!("mutation kind {kind:?} is not implemented by the
subject"))` in the las/mp3/stl SUBJECT halves) and one defensive arm in the shared PDF conformance
engine (`🧪️oracle/📄️document/🦀️component.rs:2139`), all of which are unreachable for declared kinds.

---

## 4. Do the handlers assert the laws? — the w10 "32 of 46 vacuous" number, re-measured

Method: for every one of the 86 `mutate-*` adapters, the region **above** the
`#[cfg(feature = "sut")] mod subject` boundary was parsed into functions, each role handler's call
graph expanded up to depth 4, and each expansion checked for a call into the shared `law` module
(`inverse_restores*`, `mutation_is_observable*`, `round_trip_preserves*`, `reparsed_not_copied`,
`carrier_is_exact`) or for a comparison-guarded `return Err`.

| oracle-role handler | asserts | vacuous | no such handler |
|---|---|---|---|
| `inverse-<kind>` | **66** | 18 | 2 |
| `mutate-<kind>` (observability) | **60** | 24 | 2 |
| `identity-round-trip` | **66** | 11 | 9 |

### The inverse law: 66 of 86 assert it — up from 14 of 46

The 20 that do not are **19 semio Pattern-B cases plus `mutate-json-rfc8259-i-json`**, and neither is
a straggler in the sense w10 meant:

* `mutate-json-rfc8259-i-json`'s oracle is the **Python** host. `🐍️component.py:332` raises
  `AssertionError("applying %s and then its own inverse did not restore the document")`, and its
  `identity_round_trip` raises on both byte pass-through and projection drift. The Rust file in that
  case directory is the SUBJECT half only.
* The 19 semio cases record a `noOracleDecision` and carry no `@oracle-` tag, so **the runner never
  dispatches their oracle role at all**. Their `mutate_oracle_for`/`inverse_oracle_for` closures are
  deliberately literal fixture readers (`Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))`),
  and every adapter header says so in as many words. Their laws are asserted inside the `sut`-gated
  subject handlers, which were inspected and do assert (7–19 error/assert sites each; `mutate-semio-any`
  routes its three laws through a `checked(role, kind, tag, raised, predicate)` helper). **That code
  has never executed.**

### The observability law: 60 of 86 — and 4 executed cases still assert nothing

Excluding the 23 no-oracle cases the runner skips, exactly **four dispatched cases have a fully
vacuous `mutate_oracle`**:

| case | mutate scenarios with no oracle-side assertion |
|---|---|
| `mutate-pdf-1-7` | 18 |
| `mutate-docx-ecma-376` | 13 |
| `mutate-pptx-ecma-376` | 9 |
| `mutate-pdf-1-4` | 2 |

Verbatim, from `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7/🦀️component.rs`:

```rust
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pdf_1_7(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
```

`mutate-docx-ecma-376` and `mutate-pptx-ecma-376` are byte-for-byte the same shape. All four of these
subsets ALSO lack a module-level `every_declared_kind_is_observable…` test in their oracle module —
unlike their own conformance-class siblings (`pdf 1.7 ✳️a/e/h/ua/vt/x`, `docx/pptx/xlsx ✳️strict` and
`✳️transitional`), every one of which has one and passes it. **42 scenarios in the exhaustive run pass
iff the reference library declined to error.** These four are the last of the w10 finding.

### The two exemptions from the observability law, both declared

* `mutate-jpg-jfif-1-01`: `const UNOBSERVABLE = ["set-quant-table", "remove-quant-table",
  "set-huffman-table", "remove-huffman-table", "set-restart-interval"]` — 5 of 12 kinds. Stated in the
  feature (line 44: "⚠️ Consequence: … none of the five is observable").
* `mutate-png-1-2`: `const UNOBSERVABLE = ["set-header", "set-transparency"]` — 2 of 17, each with a
  PNG §11.3.3 / IHDR-consistency reason in both the adapter and the feature.

No other case passes a non-empty exemption list. This is exactly what the shared `law` module's
doc comment demands ("in code AND in its feature description"), so it is disclosed rather than hidden
— but it does mean 7 declared kinds pass without their mutation ever being observed.

### The one exemption inside an inverse handler

`mutate-pdf-1-7` strips `pages.N.contentOperators` from BOTH sides for
`remove-page`, `append-page-content` and `set-page-content` before comparing:

```rust
let (expected, actual) = if regenerates_page_content(&kind) { (without_content_operators(&original), without_content_operators(&projection)) } else { (original, projection.clone()) };
```

The reason is documented in the adapter and repeated in the feature (`PdfPage`'s only content field
is `text`, so both producers regenerate a five-operator stream where the real page 8 carries 294),
names the fix and its owner, and applies to exactly one axis for exactly three of eighteen kinds —
every other axis and every other kind stay under the full law. It is a weakening, and it is declared.

---

## 5. Catalogs copied from a sibling

All 86 catalogs were compared pairwise on their `kinds` lists, and all 88 mutation modules plus all
66 oracle modules were compared pairwise by text similarity.

### Identical `kinds` lists — three groups

| group | verdict |
|---|---|
| `ifc-4-any` == `step-ap214-any` (11 kinds) | **Documented.** Both are ISO 10303-21 Part-21 clear-text grammars; the vocabulary is Part-21, not either schema. Unchanged from w10, and the modules still differ by ~1100 lines. |
| `dwg-ac1018-any` == `dwg-ac1024-any` (3 kinds) | **Documented and machine-checked.** `ac1018`'s mutation, schema, snapshot and oracle facets are all one-line `pub use` re-exports of `ac1024`'s, so `DwgMutation` is ONE enum. `every_ac1018_facet_is_a_re_export_of_this_one` in the ac1024 oracle module `include_str!`s all four committed files and fails the moment that stops being true. This is the brief's "share through a named module rather than copying", done exactly. |
| `step-ap214-cc2` == `cc3` == `cc4` == `cc5` (6 kinds) | ⚠️ **Correct but not documented as identical.** Each manifest and each oracle header explains its own class against `✳️any` and cites ISO 10303-214 §4.3, and the implementations are genuinely parameterised (`pub const MAX_RUNG: u8 = 2/3/4/5` over the shared `ladder` module, with `cc1` = 1 and `cc6` = 6 declaring 5 kinds each instead of 6). But no file says "cc2–cc5 declare the same six kinds because every class whose ceiling is strictly inside the ladder reads the same six axes". That sentence is the citation the brief asks for and it is missing. |

`pdf-1-7-vt` ⊃ `pdf-1-7-x`: vt's first 16 kinds are x's 16, in the same order, plus 4 DPart kinds.
**Documented with a code citation, and the citation checks out** — the vt feature and oracle header
both say "`check_vt_conformance`'s first statement is literally `let mut out = check_x_conformance(snapshot);`",
and `📄️pdf/…/✳️vt/🧬️schema/🦀️component.rs:238` is exactly that. ISO 16612-2 is defined on top of
ISO 15930. This is the good case.

No other catalog pair is identical. All remaining `⊂` relations are trivial (`pdf-1-4-any` declares
only `no-mutation` and `set-snapshot`, so it is a subset of everything — see §8).

### Near-identical MODULES that are not catalog copies

Text similarity ≥0.85 was found in exactly two families, and in both the shared logic really does live
in a named family module:

* The six OOXML conformance-class oracle modules (`docx/pptx/xlsx` × `strict/transitional`) are
  0.91–0.96 similar to each other **across different artifacts**. Diffing `pptx ✳️strict` against
  `docx ✳️strict` shows the whole body is a `PROFILE` const (`format`, `main_namespaces`,
  `drawing_namespaces: Some(...)` vs `None`), a `KINDS` list and two one-line delegations into
  `crate::document::ooxml::{apply, project}`. That is parameterisation, not duplication.
* The six PDF 1.7 conformance oracle modules (0.85–0.91) are the same arrangement over the named
  `document::pdf_conformance` engine.

⚠️ What IS templated is one layer up: the **case adapters** for those six OOXML subsets are 0.95
similar to each other and the **feature files** 0.87. The per-subset facts in the prose are real and
different (55 parts vs 7 parts; which axes `check_strict_conformance` reads for that format), but the
structure and most sentences are a template instantiated six times. See §6.

---

## 6. Templated feature wording, and no-op Examples parameters

### No-op parameters — none

Every `Examples` row whose `params` cell is `{}` names a kind that takes no parameters
(`remove-conformance-attribute`, `remove-output-intent`, `remove-lang`, `strip-non-tiny`,
`collapse-page-size`, …). The one that looked odd, `set-struct-tree-root {}` in `mutate-pdf-1-7-ua`,
is fine: that case DOES assert observability, and its subset oracle module carries
`every_declared_kind_is_observable_and_its_inverse_restores_the_document`, which passes — so the kind
demonstrably moves the projection with no parameters. A genuinely no-op row now fails in role in 60
of 86 cases.

### Templated prose — 61 sentences repeated verbatim in more than two cases

The Feature description blocks were split into sentences; **61 sentences longer than 70 characters
appear verbatim in 3 or more cases.** The heaviest:

| repeats | sentence | cases |
|---|---|---|
| 16 | "A handler that merely ran the mutation and returned would report a pass having checked nothing." | all semio |
| 11 | "Every scenario copies the fixture into the case work directory before touching it; the committed file is never written to." | csv, tsv, json, stl, 6× OOXML |
| 10 | "THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because `lopdf` did not error." | all 10 PDF cases |
| 8 | "📌️ Every Examples row below other than `no-mutation` is required to MOVE the semantic projection…" | bcf, dxf, 4× ifc-2x3, ifc-4, step-ap214 |
| 6 | "Unzipping the committed file and reading every declared namespace confirms it is a genuine ISO/IEC 29500-4 Transitional package that declares no strict-family namespace, no VML, no mc:AlternateContent and no conformance attribute — which is exactly what makes it the right input…" | 6× OOXML |

**Verdict: templated, not fabricated.** Every one of these sentences states a discipline or a law that
really is identical across those cases (the fixture-copy rule, which crate is used, what the adapter
asserts). No Feature description in the whole set is a byte-for-byte copy of another, and the
subset-distinguishing paragraphs — the vocabulary derivation, the axis list, the fixture census — are
per-subset and factually specific. The one place worth reading with suspicion is the six OOXML
conformance features, whose non-boilerplate content differs by roughly one axis and a set of part
paths.

### Cases executing zero scenarios while reporting green — none, but 23 report nothing at all

No case is counted as passing without executing. The runner emits an explicit
`[test] not-exercised …` line for every no-oracle case, and the clean run's summary confirms
`not-exercised=23` against `cases=99 executed=1321`. The 23 are the 19 semio Pattern-B cases plus
`mutate-binary-raw`, `mutate-txt-utf-8`, `mutate-dwg-ac1018` and `mutate-dwg-ac1024`, each with the
same suffix:

```
[test] not-exercised …/🧿️semio/🧪️tests/mutate-semio-mesh (recorded no-oracle decision semio-mesh-mutation-semantics — its evidence is discharged by the subject phase)
[test] not-exercised …/🖊️dwg/🧪️tests/mutate-dwg-ac1024 (recorded no-oracle decision dwg-ac1024-proprietary-container — its evidence is discharged by the subject phase)
```

Their expanded scenario counts total **565** (semio alone: 507; txt 24, binary 20, dwg 7+7), counted
directly from the `Examples` tables. **"discharged by the subject phase" is the load-bearing phrase,
and `parity=0/0` means that phase has never run.** Those 565 scenarios have zero evidence today.

---

## 7. Did anything newly fail once a law was asserted — and did anyone weaken an assertion?

`📓️w11-real-failures.md` records eleven failures that appeared the moment the laws were made to
assert. Each was traced to its current disposition in the committed source.

| failure | disposition now | indicts |
|---|---|---|
| `🧊️obj inverse-remove-face` (`vertexCount` 8577, expected 8576) | **Left RED, asserting.** The feature carries a full root-cause paragraph: face 16127 belongs to `g band-2` and `o pattern-sphere`; `RemoveFace`'s inverse is the single `InsertFace`, which carries no membership, so the restored face lands in no band and `tobj` reads a fourth model. | **Our codec.** "`Mutation::inverse` returns `Vec<Self>`, so `RemoveFace`'s inverse is entitled to be `[InsertFace, SetGroup, SetObject]` … yet `…/🧬️mutations/🦀️component.rs` returns the single `InsertFace`." Named as "the one declared kind of the twenty-two whose inverse cannot exist". |
| `🧊️obj inverse-remove-group / remove-object / set-object` | **Fixed** — the feature claims only `remove-face` remains, and the clean run confirms it: `failed=1`. | — |
| `📄️pdf 1.7 inverse-append-page-content / remove-page / set-page-content` | **Assertion narrowed, in the open.** `contentOperators` is dropped from both sides for those three kinds only; every other axis and all fifteen other kinds keep the full law. Documented in the adapter AND the feature, with the fix and its owner named. | **Our snapshot model** (`PdfPage` carries only `text`). The narrowing is a disclosed weakening, not a hidden one. |
| `📜️docx inverse-remove-style` (`styles.1.id` = "Heading1") | **Fixed.** `inverse_oracle` now compares the full projection with `first_divergence` and no exemption of any kind. | — |
| `🔣️json inverse-set-snapshot` (`…vertices[19].position`) | **Fixed.** `inverse_oracle` calls `inverse_restores(kind, projection, project_json_value(&input)?)` with zero tolerance and zero ignore keys. | — |
| `🎨️svg inverse-remove-element` | ⚠️ **FIXED, but the feature still says it is red.** The clean run reports `inverse-remove-element passed` in `mutate-svg-1-1`, `-basic` and `-tiny`. The fix is the exact remedy the feature prescribed: `oracles::apply_mutation_inverse` (`🎨️svg/…/✳️any/🧪️oracle/🦀️component.rs:647`) now does `let base = parse_svg(input)?;` ONCE and applies the forward kind and its inverse to the same `doc` before a single `write_svg`, with no re-serialize in between. The adapter's law is unchanged and unexempted. **The stale "⚠️ OPEN, and left red rather than tuned away" paragraph should be deleted from the feature.** | Was the oracle module's inverse routing; now fixed correctly. |
| `📰xml identity-round-trip` byte pass-through | **The law was REPLACED BY A STRONGER ONE, and the change is argued in the feature.** The old check (`output != input`) was dropped because `shared://📰️ooxml-word-document.xml` is genuinely minified and two minifying writers converging is not evidence of copying. The new probe perturbs the start tags into a byte-different rendering of the SAME document and requires BOTH renderings to re-encode to identical bytes — which a shortcut that hands back its input cannot satisfy. It additionally fails in role if the perturbation is a no-op. | The FIXTURE, correctly identified. This is the only law change in the ticket that makes a scenario harder to pass rather than easier. |

### Newly red outside the oracle phase

`cargo test --features oracles --lib` gained one failure this wave:
`artifacts::txt::…::every_feature_row_inverts_back_to_the_real_document`. Quoted in full in §1.5. It
indicts **our vocabulary** (`SetTrailingNewline { value: false }` has no representable result on a
document whose last line is empty and should be rejected), it is left red on purpose, and a companion
test pins the underlying non-injectivity so it flips the moment the fix lands.

### Was any assertion weakened to make something pass?

Three narrowings exist and **all three are declared in both the code and the feature**: the PDF 1.7
`contentOperators` exemption (3 of 18 kinds, 1 axis), the JPEG `UNOBSERVABLE` list (5 of 12 kinds) and
the PNG `UNOBSERVABLE` list (2 of 17 kinds). No comparison profile gained an `ignoreKeys` entry to
cover a failure — `vertexCount` and `contentOperators` appear in no manifest's `ignoreKeys`. No
fixture was swapped to dodge a failure; the one fixture question raised (XML) was answered by
strengthening the law instead. **No hidden weakening was found.**

---

## 8. Smaller things that are less than they look

* **`pdf-1-4-any` declares two kinds**, `no-mutation` and `set-snapshot`, and `mutate-pdf-1-4`
  expands to 5 scenarios. The catalog is honest — `PdfMutation` in
  `📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` really has exactly two
  variants — but "Apply every typed PDF 1.4 mutation" is a claim over one real kind. The vocabulary,
  not the case, is the thin part.
* **38 of the 66 subset oracle modules carry no module-level observability test.** The adapters carry
  the law for 60 of 86 cases, so this is redundancy rather than a hole — except for the four cases in
  §4 where BOTH are missing.
* **`IFCLENGTHMEASURE` still appears in the AP214 tree**, though no longer in the mutation module
  (w10's finding is closed). It survives at `📐️step/…/✳️any/🚪️io/📐️part21/🦀️component.rs:95,928,932`
  and in the grammar file, as the example of a Part-21 defined-type wrapper. Defensible — the wrapper
  syntax is Part-21, not IFC — but an AP214 (`AUTOMOTIVE_DESIGN`) parser test using an IFC type name
  still reads as a copy-paste tell.
* **The purity argument still holds structurally.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml`
  does not depend on `semio-s-plugin-stdio`; zero of the 86 adapters import `semio_s_plugin_stdio::`
  above the `mod subject` boundary; `serde_json` appears nowhere in the oracle crate's sources. No
  adapter compares our output against our own output.

---

## 9. Answers, in the order asked

1. **86 of 88** subsets have their own handcrafted `🧬️mutations`, a catalog and a case (88/88 have the
   vocabulary). The two without a catalog or a case: **`📷️jpg 🔖️jfif-1.01 ✳️baseline`** and
   **`🖼️tiff 🔖️6.0 ✳️baseline`**.
2. **No oracle module is a rejecting stub.** One arm returns its input unchanged —
   `📊️csv rfc4180/✳️any` `"set-has-header"` — and it is correct by RFC 4180, documented, pinned by its
   own test, and still held to the observability law through a caller-tracked projection flag.
3. **No adapter handler is a placeholder `Err`, `todo!` or `unimplemented!`.** Zero across all 86
   adapters and all 66 oracle modules.
4. **66 of 86 inverse handlers assert the metamorphic law** (w10: 14 of 46). The 20 that do not are
   the 19 semio Pattern-B cases — whose oracle role the runner never dispatches and whose laws live in
   the unrun subject half — and `mutate-json-rfc8259-i-json`, whose oracle is the Python host and does
   assert. **The real stragglers are on the FORWARD side**: `mutate-pdf-1-7`, `mutate-docx-ecma-376`,
   `mutate-pptx-ecma-376` and `mutate-pdf-1-4` still have a fully vacuous `mutate_oracle` — 42
   dispatched scenarios.
5. **Newly failing scenarios — exactly two survive, and both are honest.**
   `🧊️obj mutate-obj-3-0 :: inverse-remove-face` in the oracle phase: *"inverse law violated: applying
   \"remove-face\" and then its own inverse did not restore the original — $.vertexCount is 8577,
   expected 8576"*. **Indicts our codec** — `RemoveFace`'s inverse is a single `InsertFace` that loses
   the `g`/`o` membership, and `Mutation::inverse` returns `Vec<Self>` so a correct
   `[InsertFace, SetGroup, SetObject]` is expressible today.
   `📄txt …::every_feature_row_inverts_back_to_the_real_document` in `cargo test`: *"applying
   set-trailing-newline and then its own inverse did not restore the real document (24874 bytes back,
   24875 in)"*. **Indicts our vocabulary** — `(lines, trailing_newline)` is not injective, and the
   production `TxtSnapshot`/`TxtMutation` carry the identical hole.
   Resolved rather than surviving: `🎨️svg inverse-remove-element` (fixed by the prescribed remedy,
   though its feature still claims it is red — see §7), `📜️docx inverse-remove-style` and
   `🔣️json inverse-set-snapshot` (both now under the full unexempted law), and `📰xml
   identity-round-trip` (the law was replaced with a STRONGER probe, and the change is argued in the
   feature: the fixture is genuinely minified, so `output != input` was the wrong law).
   **No assertion was weakened to make anything pass**; the three narrowings that exist are declared
   in code and in the feature, and no comparison profile gained an `ignoreKeys` entry to cover a
   failure.
6. **Templated wording: yes, 61 sentences repeated verbatim across 3+ cases**, heaviest in the PDF
   family (10), the semio family (16) and the six OOXML conformance cases (6, whose adapters are 0.95
   similar and whose features are 0.87 similar). All state genuinely-shared disciplines; the
   subset-specific paragraphs are real. **No `Examples` params make a mutation a no-op** — the one
   suspicious row (`set-struct-tree-root {}`) is proven observable by its subset's own passing test.
7. **Catalogs copied from a sibling**: three identical groups. `ifc-4-any`/`step-ap214-any` and
   `dwg-ac1018-any`/`dwg-ac1024-any` are both documented, the DWG pair machine-checked.
   **`step-ap214-cc2/cc3/cc4/cc5` are identical and nowhere say so** — the implementations differ by
   `MAX_RUNG`, but the "identical by specification" sentence the brief requires is missing.
   `pdf-1-7-vt ⊃ pdf-1-7-x` is documented with a code citation that was verified against the source.
8. **No case reports green while executing zero scenarios.** 23 cases (≈565 expanded scenarios)
   execute nothing and are reported honestly as `not-exercised`; their stated fallback, the subject
   phase, has never run either.


---

## 10. The concrete list, if this is to be closed out

1. Give `📷️jpg jfif-1.01 ✳️baseline` and `🖼️tiff 6.0 ✳️baseline` an oracle module, a manifest with a
   catalog, a feature and an adapter — or delete their vocabularies. Right now they are 875 lines of
   handcrafted code nothing can see.
2. Add the observability law to `mutate-pdf-1-7`, `mutate-docx-ecma-376`, `mutate-pptx-ecma-376` and
   `mutate-pdf-1-4`'s `mutate_oracle`, and an `every_declared_kind_is_observable…` test to their four
   subset oracle modules. 42 scenarios currently prove only that the reference library ran.
3. Fix `RemoveFace`'s inverse in `🧊️obj/…/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` to
   `[InsertFace, SetGroup, SetObject]`, or make `RemoveFace`/`InsertFace` carry membership.
4. Make `SetTrailingNewline { value: false }` REJECT on a document whose last line is empty, in both
   the `txt utf-8` vocabulary and the production `TxtMutation`.
5. Delete the stale "⚠️ OPEN, and left red" paragraph from `mutate-svg-1-1/component.feature`.
6. Add one sentence to `step-ap214-cc2…cc5` saying the four catalogs are identical and why.
7. Add a contract rule for "subset declares a mutation vocabulary but no catalog" — the gate is
   currently blind to exactly the gap this wave left.
8. File the `markRunComplete` `ENOENT` crash.
9. Unblock the Rust subject phase (`semio-framework-os-kernel`'s store module vs the finished
   `semio-framework-job` refactor). Until that lands, `parity=0/0` and the platform's central claim
   remains unmade.
