# Wave 10 — final verification and dishonesty audit

Date 2026-08-24. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Every command below was actually run
from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` (or where noted) and every quoted line is the
real output, copied verbatim. Exit codes were read from the tool's own exit status, never through a
pipe.

---

## 1. The six commands

### 1. `bun ./📜️script.ts contract` — exit 0

```
0 high-priority breach(es) across 0 rule(s):


full breach set (including non-blocking priorities): /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/breaches/testing.json
```

`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` was read directly: its content is the empty array
`[]`. There are no non-blocking breaches hiding behind the high-priority count.

### 2. `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio` — exit 0

```
[test] level=exhaustive cases=80 executed=1011 passed=1011 failed=0 errored=0 parity=0/0 not-exercised=20
```

Preceded by 20 `[test] not-exercised …` lines (full list in §5). The run took just under 40 minutes,
mostly cargo host builds.

### 3. `bun ./📜️script.ts dependency` — exit 0

```
[dependency] ecosystems=4 entries=229 production-reachable=151 test-oracle=27
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
```

Four ecosystems now: `rust` (22 packages), `python` (`pypdf`, `simplejson`), `javascript`
(`semver`, `clsx`, `class-variance-authority`).

### 4. `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — exit 0

```
 69 pass
 0 fail
 1458 expect() calls
Ran 69 tests across 1 file. [64.28s]
```

### 5. `cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — exit 0

```
test result: ok. 208 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 19.40s
```

The single ignored test is
`artifacts::tiff::standards::v6_0::subsets::any::component::fixture_derivation::derive_real_world_fixture`
— an explicitly `#[ignore]`d one-shot fixture-derivation helper, documented as such in the module.
Not a skipped assertion.

### 6. `cargo check -p semio-framework-os-kernel --lib` from the repo root — exit 101

**6 `error` lines, 5 real compiler errors, and NO — they are not the two named blockers.**

```
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0499]: cannot borrow `self.rejected` as mutable more than once at a time
error: could not compile `semio-framework-job` (lib) due to 5 previous errors; 6 warnings emitted
```

* `ArtifactEnvelope: Clone` **does not appear anywhere in the output** — that blocker is gone.
* All five errors are in `semio-framework-job`, not in `semio-framework-os-kernel`. The kernel is
  never reached; its dependency fails first.
* All five are the same new shape: `🧰️framework/🔨️modules/🧵️job/🦀️component.rs` now declares
  `rejected: ManuallyDrop<Option<JobPayloadPageSource>>` and `payload:
  `ManuallyDrop<Option<RetainedJobPayload>>` while lines 489, 513, 523, 546 and 650 still assign a
  bare `Option<…>` to them.

  ```
   --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs:523:29
  523 |         self.payload = None;
      |         ------------   ^^^^ expected `ManuallyDrop<Option<RetainedJobPayload>>`, found `Option<_>`
  ```

* `🧰️framework/🔨️modules/🧵️job/🦀️component.rs` has an mtime of **04:41 today**, i.e. it was being
  edited while this verification ran. This is a concurrent session's in-flight refactor, not this
  wave's regression — but it means the claim "only ArtifactEnvelope: Clone and RetainedJobPayload
  remain" is **stale**: the `RetainedJobPayload` half has changed shape into a `ManuallyDrop`
  migration, and the `ArtifactEnvelope` half is gone entirely.

---

## 2. Mutation-directory and case coverage — the real counts

Counted over the **88** subsets under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*`
(the enumeration that produces exactly 88, matching the w10 report's own denominator):

| | count |
|---|---|
| subsets total | **88** |
| subsets with their own `🧬️schema/🧬️mutations` directory | **71** |
| subsets whose oracle manifest declares a `mutationCatalogs` entry AND a feature file tags it | **67** |
| stdio cases named `mutate-*` | **67** |
| stdio cases in total (the runner's `cases=80`) | 80 (67 mutate + 13 pre-existing) |

**17 subsets have no `🧬️mutations` directory at all** — the untaken residue the w10 report already
mapped, unchanged:

`pdf 1.4 ✳️a ✳️x`; `pdf 1.7 ✳️a ✳️e ✳️h ✳️ua ✳️vt ✳️x`; `step ap214 ✳️cc1…✳️cc6`;
`xml 1.0 ✳️valid`; `jpg jfif-1.01 ✳️baseline`; `tiff 6.0 ✳️baseline`.

**4 subsets have a `🧬️mutations` directory but no oracle catalog and no case** — they have a
`🧪️oracle/🦀️component.rs` stub (or not even that) and are invisible to both the contract phase and
the oracle phase:

* `mp3 mpeg1-layer3 / ✳️any`
* `dwg ac1018 / ✳️any`
* `dwg ac1024 / ✳️any`
* `semio v1 / ✳️drawing` (no `🧪️oracle` directory at all — the only semio subset without a case,
  even though `semio-v1-any`'s own catalog lists `drawing` as a routable kind)

So the honest headline is **67/88 subsets covered end to end, not 71/88** — the four above have a
handcrafted vocabulary that nothing exercises or even declares.

---

## 3. Catalog kinds copied from a sibling subset

Every one of the 67 mutation catalogs was compared pairwise.

### Exactly one pair is byte-identical, and it is documented

`ifc-4-any` and `step-ap214-any` declare the **same 11 kinds** in the same order:

```
no-mutation, set-snapshot, set-file-description, set-file-name, set-file-schema,
insert-entity, remove-entity, set-entity-name, set-entity-arg, insert-entity-arg, remove-entity-arg
```

This is the good case, not the bad one. Both manifests state the reason in their oracle `rationale`
— IFC4 and AP214 are both ISO 10303-21 Part-21 clear-text exchange structures, and the vocabulary is
the Part-21 grammar, not either schema. The IFC4 entry names the STEP case explicitly ("the same
finding this wave's STEP AP214 case already made independently"). Their oracle modules are NOT
copies: 1133 diff lines between the two mutation modules, 624 between the two oracle modules.

**One residue worth cleaning:** `📐️step/…/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` lines 423 and 560
build a STEP test vector out of `SV::TypedValue { type_name: "IFCLENGTHMEASURE" }`. `IFCLENGTHMEASURE`
is an IFC type and has no business in an AP214 (`AUTOMOTIVE_DESIGN`) vector. It is inside a test
fixture, so it changes no behaviour, but it is a visible copy-paste tell from the IFC module.

### The six OOXML conformance-class catalogs differ, and the differences are real

```
docx-strict         no-mutation set-snapshot set-main-namespace set-relationship-base       set-conformance-attribute remove-conformance-attribute insert-vml-part remove-vml-part insert-alternate-content remove-alternate-content
docx-transitional   no-mutation set-snapshot set-main-namespace set-relationship-base       set-conformance-attribute remove-conformance-attribute
xlsx-strict         no-mutation set-snapshot set-main-namespace set-relationships-namespace set-conformance-attribute remove-conformance-attribute insert-vml-part remove-vml-part set-worksheet-content-type
xlsx-transitional   no-mutation set-snapshot set-main-namespace set-relationships-namespace set-conformance-attribute remove-conformance-attribute set-worksheet-content-type
pptx-strict         no-mutation set-snapshot set-main-namespace set-drawing-namespace set-relationship-base set-conformance-attribute remove-conformance-attribute insert-vml-part remove-vml-part insert-alternate-content remove-alternate-content
pptx-transitional   no-mutation set-snapshot set-main-namespace set-drawing-namespace set-relationship-base set-conformance-attribute remove-conformance-attribute
```

The suspicious-looking split — xlsx says `set-relationships-namespace`, docx/pptx say
`set-relationship-base` — was checked against the shared engine and the production checkers, and it
is **not** a fabricated difference:

* In `🧪️oracle/📄️document/🦀️component.rs` these are two genuinely different operations:
  `rewrite_namespaces` (line 861) rewrites `xmlns:r` declarations; `rewrite_relationship_bases`
  (line 862) rewrites relationship `Type` base URIs inside `.rels` parts.
* Each subset's vocabulary is derived one-kind-per-axis from that artifact's own pre-existing
  production `check_strict_conformance`, and those functions really do read different axes:
  `xlsx` (`…/✳️strict/🧬️schema/🦀️component.rs:231`) reads `workbook` root `xmlns:r`;
  `docx` (`…/✳️strict/🧬️schema/🦀️component.rs:274`) reads the `officeDocument` relationship base of
  every relationship and never looks at `xmlns:r`.
* Each oracle module's doc comment states the derivation explicitly and lists the axes.

Verdict: documented and defensible. The mild reservation to record is that these vocabularies are
derived from *this repository's conformance checkers* rather than from ISO/IEC 29500 directly, so a
gap in a checker becomes a gap in the vocabulary silently. That is stated nowhere.

**No other pair of catalogs is identical.** All remaining `⊂` relationships found are trivial
(`pdf-1-4-any` has only `no-mutation`/`set-snapshot`, so it is a subset of everything).

---

## 4. Adapters that compare our output against our own output

**None.** This was checked structurally, and the structure holds:

* `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml` does **not** depend on
  `semio-s-plugin-stdio`. The oracle crate physically cannot call production code.
* Every one of the 67 `mutate-*` adapters was scanned for a `semio_s_plugin_stdio::` import appearing
  **before** the `#[cfg(feature = "sut")] mod subject` boundary. **Zero hits.**
* Every subset oracle module was checked for actually reaching its registered third-party package.
  All of them do, directly or through a shared family module: the six OOXML conformance subsets and
  the two SVG profile subsets delegate to `🧪️oracle/📄️document`/`📰markup`, which use `zip` +
  `quick_xml` (`📄️document/🦀️component.rs:287-319`, `📰markup/🦀️component.rs:21-22`).
* `project_json_value`, the independent reader the i-json subject is projected through, uses
  `json-rust` — `serde_json` appears nowhere in the oracle crate. Confirmed by grep.

### But there IS a much weaker thing hiding under `passed=1011`

**32 of the 46 exercised `mutate-*` cases have oracle handlers that assert nothing at all.** Their
`#region 🔖️Oracle` contains no comparison, no `return Err`, no metamorphic check — the handler calls
the reference library, projects the result, and returns it. Since `parity=0/0`, nothing ever consumes
that projection. Such a scenario passes **iff the reference library did not return `Err`**.

Representative, verbatim from `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7/🦀️component.rs`:

```rust
/// 🔒️ The ORACLE side of the no-byte-pass-through law: `lopdf` fully parses the real document and
/// re-serializes it from its own object graph alone …
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![…]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    let projection = project_pdf_1_7(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
```

The doc comment names the no-byte-pass-through law; the body never compares `bytes` to `input`.
`inverse_oracle` in the same file likewise never applies the inverse and checks restoration — it
just applies a precomputed inverse spec and returns.

**14 cases whose oracle region claims the pass-through law in prose but never checks it:**
`mutate-html-5`, `mutate-zip-2-0-iso21320`, `mutate-gif-87a`, `mutate-pptx-ecma-376`,
`mutate-svg-1-1-basic`, `mutate-svg-1-1-tiny`, `mutate-svg-1-1`, `mutate-bcf-2-1`,
`mutate-txt-utf-8`, `mutate-pdf-1-7`, `mutate-docx-ecma-376`, `mutate-xml-1-0`, `mutate-avi-1-0`,
`mutate-dxf-r12`.

**32 exercised cases with zero oracle-side assertions:** ply-1-0, html-5, epw-energyplus, zip-2-0,
gif-87a, pptx-ecma-376, mp4-isobmff, svg-1-1, ifc-2x3(-cobie/-cv20/-sav), ifc-4, bcf-2-1, pdf-1-4,
pdf-1-7, csv-rfc4180, step-ap214, tsv-iana, xlsx-ecma-376, docx-ecma-376, xml-1-0, jpg-jfif-1-01,
png-1-2, avi-1-0, wav-riff-pcm, json-rfc8259, dxf-r12, bmp-v3, tiff-6-0, gltf-2-0, obj-3-0.

**14 exercised cases that DO assert on the oracle side** — and they are, tellingly, mostly the
newest ones: las-1-0, zip-2-0-iso21320, gif-89a, pptx/xlsx/docx `-strict` and `-transitional` (four
assertions each), svg-1-1-basic, svg-1-1-tiny, md-commonmark, deflate-rfc1950, stl-ascii.

The shape the newer cases got right, from `mutate-xlsx-ecma-376-strict/🦀️component.rs`:

```rust
if spec.str("kind") != "no-mutation" && projection == project_package(&base)? {
    return Err(format!("{:?} left the conformance-class projection unchanged — a mutation that is not observable proves nothing", spec.str("kind")));
}
…
if output == input { return Err("byte pass-through: output is bit-identical to the input".to_string()); }
```

This is not "comparing our output against our own output" — it is *not comparing anything*. The
distinction matters, but the consequence for the ledger is the same: a large share of the 1011
greens is evidence that a third-party crate ran without panicking, and nothing more.

---

## 5. Cases reporting green whose scenarios never execute

**20 of the 80 cases contributed 0 to `executed=1011`.** They are listed as `not-exercised` and are
therefore not silently counted as passes — the runner is honest about this, and the w9 report quotes
`executed=0` per case. Recording it here because a reader of `passed=1011 failed=0` will not see it:

```
[test] not-exercised …/💾️binary/🧪️tests/mutate-binary-raw (recorded no-oracle decision raw-buffer-no-format — its evidence is discharged by the subject phase)
[test] not-exercised …/📄txt/🧪️tests/mutate-txt-utf-8 (recorded no-oracle decision txt-utf-8-line-structure — …)
[test] not-exercised …/🧿️semio/🧪️tests/mutate-semio-{animation,any,audio,brep,cad,document,flow,graph,image,kit,mesh,model,object,presentation,table,text,value,video} (recorded no-oracle decision semio-<subset>-mutation-semantics — …)
```

Those 20 cases carry roughly **550 expanded scenarios** (counted from their `Examples` tables) that
have never run in any phase. Their stated discharge is "the subject phase", and
**`parity=0/0` means the subject phase has never run either.** Their evidence today is zero.

The same `parity=0/0` applies to all 80 cases: **no differential comparison between an oracle and
this repository's own implementation has happened for any case in this ticket.** The oracle-phase
green is a claim about the reference libraries, not about our codecs. w7-results.md says this
plainly ("parity=0/0 is not a pass"); it should stay on the front page of any summary.

---

## 6. Production reachability without a recorded productionDebt

**Clean.** `bun ./📜️script.ts dependency` exits 0 with exactly three `production-debt` records
(`png`, `zip`, `image`), all pre-existing and all owner-attributed. 70 oracle manifests were parsed
directly: every one of the 67 oracle entries has `testOnly: true` and a non-empty `rationale`; no
manifest uses `deferredKinds`.

Checked by hand for the new cross-language registrations, because the JS names are classic runtime
libraries:

* `clsx`, `class-variance-authority`, `semver` appear only in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/package.json` — the test
  host's own manifest. Not production-reachable. The UI module correctly records a
  `noOracleDecision` (`repository-owned-utility-groups`) for its own `cn()`.
* `simplejson` and `pypdf` appear in no `requirements`/`pyproject` anywhere.

**One real blind spot in the gate.** Two third-party crates are linked into the oracle host but are
registered in **no** manifest, so the purity probe never examines them:

* `rust_xlsxwriter` 0.96 — the *writer* half of the XLSX differential
* `markup5ever_rcdom` 0.39 — the DOM half of the HTML5 differential

Both are described inside a neighbouring entry's `rationale` prose, which is honest documentation but
is not a registry entry. Verified by grep that neither is in fact production-reachable (only the
oracle crate's `Cargo.toml` and a ticket scratch crate), so this is a coverage gap, not a violation.
It should be closed by registering them, because prose is not checked by anything.

---

## 7. Cross-language oracle support — verified working

Both new hosts were exercised individually, not merely declared:

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-json-rfc8259-i-json   # exit 0
[test] level=exhaustive cases=1 executed=22 passed=22 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --case satisfy-version-requirements                 # exit 0
[test] level=exhaustive cases=1 executed=3 passed=3 failed=0 errored=0 parity=0/0
```

The Python host is genuinely real: the run provisioned
`.🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-cacdfc3fbe53ad5f7f5baff78016644f/` with
`lib/python3.9/site-packages/simplejson-4.1.1.dist-info` present, and
`🐍️component.py` (417 lines) is a real `simplejson`-based oracle with real assertions
(`object_pairs_hook` duplicate detection at line 90, `Decimal`-lexeme §2.2 check at line 217, and an
observability check at line 314: *"left the document unchanged — a mutation that applies to nothing
is a silent no-op, not a pass"*). It is one of the better-evidenced cases in the whole ticket.

**But its Rust adapter's own header is factually wrong.**
`…/🔣️json/🧪️tests/mutate-json-rfc8259-i-json/🦀️component.rs` lines 11-13 say:

> "the subject through `project_json_value` (`serde_json`, in the stdio oracle crate)"

`project_json_value` is implemented over **`json-rust`**; `serde_json` is not a dependency of the
oracle crate at all — grep for `serde_json` under `✏️s/🔌️plugins/🗄️stdio/🧪️oracle` returns nothing.
The comment names precisely the library that was rejected earlier in this ticket for being
production-reachable, so it reads as an admission of the exact defect the case avoided. It is a
documentation bug, but it is the kind that survives into a reviewer's conclusion. Fix the comment.

---

## 8. Things that are not what they appear to be — ranked

1. **`passed=1011` is mostly "no crash", not "correct".** 32 of 46 exercised mutate cases assert
   nothing on the oracle side; 14 of those name the no-byte-pass-through law in a doc comment whose
   body never checks it (`mutate-pdf-1-7`, `mutate-svg-1-1`, `mutate-xml-1-0`, … — full list §4).
2. **`parity=0/0` across the whole ticket.** No oracle-versus-subject comparison has ever run. The
   os-kernel/job breakage keeps the Rust subject phase uncompilable, so the differential claim the
   platform exists to make is still entirely unmade.
3. **20 cases (~550 scenarios) have zero executed evidence in any phase**, and their stated fallback
   ("discharged by the subject phase") is itself unrun. 18 of the 20 are the semio Pattern-B subsets.
4. **The os-kernel status line is stale.** `ArtifactEnvelope: Clone` is fixed and gone. The remaining
   5 errors are a `ManuallyDrop` migration in `semio-framework-job` around `RetainedJobPayload` /
   `JobPayloadPageSource`, actively being edited by another session (mtime 04:41 today). The kernel
   itself is never reached.
5. **Coverage is 67/88, not 71/88.** `mp3 mpeg1-layer3/✳️any`, `dwg ac1018/✳️any`, `dwg ac1024/✳️any`
   and `semio v1/✳️drawing` have handcrafted mutation vocabularies with no catalog, no oracle
   registration and no case — invisible to the contract phase, so nothing will ever notice.
6. **Two reference crates escape the purity gate entirely** — `rust_xlsxwriter`,
   `markup5ever_rcdom` — because they are documented in prose rather than registered. (Verified
   factually clean today.)
7. **The i-json adapter's header misnames its independent reader as `serde_json`.** It is
   `json-rust`. Cosmetic, but it names the one library this ticket rejected on purity grounds.
8. **`IFCLENGTHMEASURE` in the AP214 mutation module's test vector** (lines 423, 560) — a harmless
   but visible copy-paste residue from the IFC module.

### What held up under audit

* No adapter compares our output against our own output. The oracle crate cannot reach production
  code, and no case adapter imports it on the oracle side. Checked structurally across all 67 cases.
* The single identical catalog pair (`ifc-4-any` == `step-ap214-any`) is the documented
  "identical by specification" case the brief asked for, not a fabrication.
* The six OOXML conformance-class catalogs diverge for real, checker-grounded reasons.
* Contract phase: 0 breaches, and the full breach file is genuinely `[]`.
* Dependency purity: 3 recorded debts, all pre-existing; no new oracle is production-reachable.
* The Python and JavaScript oracle hosts both really execute, with a really provisioned venv.
