# Wave 12 — final end-to-end audit

Date 2026-08-24, head `c3a79bd4ce` (2026-08-24 16:48:29 +0200). Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
Successor to `📓️w11-verification.md`. Every command below was actually run; every quoted line is real
output copied verbatim; exit codes were read from the tool's own exit status, never through a pipe.

---

## 0. Headline — what is still overstated

1. **The `unregistered-mutation-vocabulary` count went 70 → 0 without a single new oracle being
   written.** 66 subsets carried a `🧪️oracle/🦀️component.rs` at w11; 66 carry one now, and
   `git diff --name-status dced3e936e HEAD | grep '^A' | grep '🧪️oracle/🦀️component.rs'` returns
   **0**. All 70 breaches were closed by giving each vocabulary a catalog and a case whose feature
   carries `@no-oracle-…` and no `@oracle-` tag — so the runner dispatches nothing for them. A reader
   who takes "contract: 0 breaches across 0 rules" as "the surface is now measured" has it backwards:
   what grew is the DECLARED surface.
2. **71% of the repository's scenarios now execute in no phase at all, up from 565 of ~1,886 at
   w11.** 164 features
   expand to **4,562** scenarios. **85 of them (3,231 scenarios) carry `@no-oracle-`** and are skipped
   in the oracle phase. w11 measured 23 cases / 565 scenarios. The oracle-reachable population barely
   moved: 1,321 executed at w11, 1,331 tagged today.
3. **The subject phase — every one of those 3,231 scenarios' stated fallback — now runs for stdio and
   does NOT compile for the largest case that depends on it.** `cargo check -p semio-framework-os-kernel
   --lib` is finally **exit 0**, and `subject exhaustive --case mutate-txt-utf-8` really executed
   (`executed=24 passed=24`) — the first subject execution in this ticket. But
   `subject exhaustive --owner 🏛️architect --case mutate-program-1` reports
   `executed=0` and `could not compile semio-s-plugin-architect (lib) due to 2591 previous errors`.
   `mutate-program-1` alone is 533 of the 3,231.
4. **`mutate-svg-1-1/component.feature:55` still declares a failure that does not happen** — the exact
   stale paragraph w11 item 9 and remedy #5 named. And **15 case files still say the Rust subject phase
   is "peer-blocked" by the os-kernel refactor**, which the exit-0 `cargo check` above contradicts.
5. **The TypeScript suite went red once and green once, and that is worth reporting rather than
   re-rolling.** Under concurrent load: `68 pass / 1 fail`, `discovery is idempotent` timing out at
   **5130.40ms** against a 5000ms budget. Clean re-run: `69 pass / 0 fail`. It is our harness, and the
   cause is item 2's growth — that test performs two full repo discoveries and discovery now walks 164
   cases and 157 vocabulary directories where it walked ~99 and ~88. A 2.6% margin is a flake this
   wave created, not one to shrug at.
6. **`create-and-round-trip-bmp` and `create-and-round-trip-tiff` have byte-identical feature
   descriptions** (similarity 1.000, the only such pair in the repo). One is a silent copy of the other
   with the format name swapped. Pre-existing, not this wave — but it is exactly what THE STANDARD
   forbids and nothing has caught it.

### What genuinely improved

* **Both confirmed codec bugs are fixed at the cause, and neither assertion was weakened.**
* **The 42 previously-silent scenarios all assert now**, and the four subset oracle modules that
  lacked a module-level observability test have one, passing.
* **63 of 63 oracle-dispatched `mutate-*` cases assert all three laws in role. Zero vacuous.**
* **`step-ap214-cc2…cc5` are documented as identical by specification, with an ISO 10303-214 §4.3
  citation in each of the four oracle headers AND a machine check that passes.**
* **The os-kernel blocker is cleared** and the Rust subject phase executes for the first time.
* **No assertion was weakened anywhere.** No comparison profile changed, no `ignoreKeys` was added,
  no fixture was removed or swapped, and the shared `⚖️law` module's only change is additive.

---

## 1. The six commands, verbatim

### 1. `bun ./📜️script.ts contract` — exit 0

```
0 high-priority breach(es) across 0 rule(s):


full breach set (including non-blocking priorities): /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/breaches/testing.json
```

`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` read directly: `[]`, 3 bytes.
**`unregistered-mutation-vocabulary`: 0. Every rule id: 0.**

### 2. `bun ./📜️script.ts oracle exhaustive` — repo-wide, exit 0

```
[test] level=exhaustive cases=164 executed=1331 passed=1331 failed=0 errored=0 parity=0/0 not-exercised=85
```

Preceded by exactly 85 `[test] not-exercised …` lines, each naming its recorded no-oracle decision and
each ending with the same clause: *"its evidence is discharged by the subject phase"*. **`failed=0`:
the `🧊️obj inverse-remove-face` red w11 left standing is gone.** Re-run alone to see the scenario
itself:

```
[test] level=exhaustive cases=1 executed=45 passed=45 failed=0 errored=0 parity=0/0
```
```json
{"case": "mutate-obj-3-0", "scenario": "inverse-remove-face", "status": "passed"}
```

`parity=0/0` is unchanged. No oracle-versus-subject comparison has run for any case in this ticket.

The comparison that matters: w11's stdio-only run was `cases=99 executed=1321`. Today's **repo-wide**
run is `cases=164 executed=1331`. **The wave added 65 cases and ~2,676 scenarios, and the number the
oracle phase executes went up by 10.**

### 3. `bun ./📜️script.ts dependency` — exit 0

```
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/…/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
```

Identical to w11: `entries=232`, `test-oracle=30`, the same three pre-existing `production-debt`
records. **The 56 new non-stdio cases added no third-party dependency at all** — consistent with §2:
none of them registers an oracle.

### 4. `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — exit 1, then exit 0

First run, while command 2 was still running:

```
🧪️index.test.ts:
(fail) 🔍️ discovery and contract > discovery is idempotent [5130.40ms]
  ^ this test timed out after 5000ms.

 68 pass
 1 fail
 1823 expect() calls
```

Clean re-run, nothing else on the machine:

```
 69 pass
 0 fail
 1823 expect() calls
Ran 69 tests across 1 file. [78.58s]
```

**Reported as a real finding, not dismissed as a flake.** The test is
`expect(JSON.stringify(discoverTestCases(repoRoot))).toBe(JSON.stringify(discoverTestCases(repoRoot)))`
— two full repository discoveries — and discovery now walks 164 cases and 157 vocabulary directories
where it walked ~99 and ~88. 5130ms against a 5000ms default budget is a 2.6% margin: this test is now
one concurrent build away from red, and this wave is what put it there. (w11: 1682 `expect()` calls
over the same 69 tests; the suite gained 141 assertions.)

### 5. `cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — exit 0

```
running 369 tests
test result: ok. 367 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 79.79s
```

**w11's one red is gone**, and the test that carried it still exists and still asserts:

```
test artifacts::txt::standards::v_utf_8::subsets::any::component::tests::every_feature_row_inverts_back_to_the_real_document ... ok
```

The 2 ignored are the same one-shot `#[ignore]`d fixture-derivation helpers w11 identified
(`bmp v3`, `tiff 6.0`); neither is a skipped assertion. Test count went 347 → 369.

### 6. `cargo check -p semio-framework-os-kernel --lib`, from the repo root — **exit 0**

```
warning: `semio-framework-os-kernel` (lib) generated 26 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 26 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1.82s
```

**The blocker w10 and w11 both reported is cleared.** The three errors w11 quoted
(`begin_close`/`close_step`/`terminal_is_empty`, `ArtifactEnvelope: Clone`, `JobFault::detail`) are all
gone. This is what made §5's subject probe possible.


## 2. `unregistered-mutation-vocabulary` — 70 → 0, and what that bought

### The count, by rule id

`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` read directly after the run: content is `[]` (3 bytes),
so no non-blocking priority hides behind the high-priority count. **Every rule id is at zero, including
`unregistered-mutation-vocabulary`.**

### The rule was not narrowed

* The rule still lives at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts:1361`
  and still walks the whole repo: `walkDirectories(repoRoot, …)`, `owner = dirname(dirname(rel))`,
  `registry.contributions.some((entry) => entry.owner === owner && entry.mutationCatalogs.length > 0)`.
* `testExcludedPathPrefixes` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` is
  unchanged since before w11: `compose/`, `node_modules/`, `.🧬semio/🦑️repo/🎫️tickets/`,
  `.🧬semio/🦑️repo/⚡️cache/`, `.git/`, `.venv/`, `target/`, `dist/`, `storybook-static/`. No plugin was
  hidden from the sweep.
* Re-measured independently: **157** `🧬️mutations` directories exist (144 under `🧬️schema`, 12 under
  `🚪️io`), **145** distinct catalogs are declared across the repo, and **0** vocabulary directories
  have an owner with no catalog. The gate's verdict checks out.

### What it bought — measured, not taken on trust

| | w11 | now |
|---|---|---|
| features (cases) | 99 stdio + a few | **164** |
| expanded scenarios, repo-wide | ~1,886 | **4,562** |
| features carrying `@oracle-` | — | **79** (1,331 scenarios) |
| features carrying `@no-oracle-` | 23 (565 scenarios) | **85 (3,231 scenarios)** |
| subsets with a `🧪️oracle/🦀️component.rs` | 66 | **66** |
| oracle modules added since w11 | — | **0** |
| non-stdio cases | ~0 | **56 across 32 plugins** |

The 70 breaches were not 70 vocabularies. `📓️w12-forms-note-writer-playbook-architect-vocabularies.md`
records the arithmetic honestly for its own slice: 📋️forms, 🗒️note and ✒️writer each raised TWO
breaches for ONE vocabulary, because the directory name appears under both `🧬️schema/` and `🚪️io/`.
That is the gate's own double-count, disclosed in the report that closed it.

**None of the 56 new non-stdio cases has an oracle.** Every one records a `noOracleDecision`, and every
one says so plainly in its feature — e.g. `mutate-program-1`: *"Because that decision is recorded, the
runner dispatches NO oracle role for this case: every assertion below lives inside the subject handler."*
The declarations are honest. What they add up to is not coverage.

## 3. The two confirmed codec bugs — both fixed at the cause, verified in source

### 🧊️obj `RemoveFace`'s inverse losing `g`/`o` membership — **FIXED**

`…/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` now routes
`RemoveFace` through a new `InverseRestoration` region:

```rust
ObjMutation::RemoveFace { index } => match base.faces.get(*index) {
    Some(v) => restore_face_at(*index, v, base),
    None => vec![ObjMutation::NoMutation],
},
```

```rust
fn restore_face_at(index: usize, face: &ObjFace, base: &ObjSnapshot) -> Vec<ObjMutation> {
    let disturbed = |faces: &[usize]| faces.iter().any(|member| *member >= index);
    let mut undo = vec![ObjMutation::InsertFace { index, face: face.clone() }];
    undo.extend(base.groups.iter().filter(|group| disturbed(&group.faces)).map(|group| ObjMutation::SetGroup { name: group.name.clone(), faces: group.faces.clone() }));
    undo.extend(base.objects.iter().filter(|object| disturbed(&object.faces)).map(|object| ObjMutation::SetObject { name: object.name.clone(), faces: object.faces.clone() }));
    undo
}
```

This is exactly the `[InsertFace, SetGroup, SetObject]` w11 §10 item 3 prescribed, and it goes further
than the report asked: removing face `index` closes the whole face-index space, so EVERY membership
list naming a face at or after `index` is restored, not only the removed face's own. `RemoveGroup` and
`RemoveObject` got the matching `restore_group_at` / `restore_object_at` repairs (a `SetGroup` on a
name the document no longer carries APPENDS, so the tail is lifted off and re-declared in order). The
module header names all three repairs and cites the ticket. **The assertion is untouched** — the
adapter still calls the unexempted law.

### 📄txt's non-injective `(lines, trailing_newline)` — **FIXED, in the oracle AND in production**

The oracle module (`…/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`) gained
`non_canonical_reason`, and every mutation arm is now gated on it:

```rust
if let Some(reason) = non_canonical_reason(&lines, trailing_newline) {
    return Err(format!("{} is not representable on this document — {reason}", spec.str("kind")));
}
```

The failing test is still there and still asserts the same equality — it now asserts the REFUSAL for the
one row the real fixture cannot represent, and asserts that the refusal leaves the document
byte-identical *"so the row cannot pass by quietly doing nothing"*. Two companions were added rather
than removed: `set_trailing_newline_inverts_where_its_result_is_representable` carries the kind's
positive inverse on a document that can hold both answers, and
`the_line_terminator_collision_is_named_and_unreachable` pins the collision itself. The test's own doc
comment says it: *"Nothing here is weakened to fit: the assertion below is the same equality it always
was."*

**And the production side was fixed too**, which w11 §10 item 4 asked for and which a report could
easily have skipped: `…/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` gates every variant on the same
`non_canonical_reason` and raises `CODE_NOT_REPRESENTABLE`, pinned by a test asserting
`assert_eq!(next, base, "a refused mutation must leave the document untouched")`.

## 4. The 42 previously-silent scenarios — all four now assert

| case | scenarios | `mutate_oracle` now | exemptions |
|---|---|---|---|
| `mutate-pdf-1-7` | 18 | `mutation_is_observable_within(&kind, &projection, &project_pdf_1_7(&input)?, UNOBSERVABLE, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?` | `UNOBSERVABLE = ["insert-object"]` — 1 of 18 |
| `mutate-pdf-1-4` | 2 | `mutation_is_observable_within(…, &[], PDF_WRITER_FREEDOM, PDF_TOLERANCE)?` | none |
| `mutate-docx-ecma-376` | 13 | `mutation_is_observable(&spec.str("kind"), &projection, &project_docx_ecma_376(&input)?, &[])?` | none |
| `mutate-pptx-ecma-376` | 9 | `mutation_is_observable(&spec.str("kind"), &projection, &project_pptx_mutation(&input)?, &[])?` | none |

`PDF_WRITER_FREEDOM` is not a new loosening: it is a verbatim mirror of `semantic-pdf-v1`'s own
declared `ignoreKeys` and `tolerance` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️component.json`, and that
manifest is **unchanged since before w11** (`git diff dced3e936e HEAD` on it is empty). The adapter says
why in as many words: *"never stricter, which would invent a failure the comparison itself would
forgive, and never looser, which would let a real one through."*

All four subset oracle modules also gained the module-level test w11 said they lacked, and all four pass:

```
test artifacts::pdf::standards::v1_7::subsets::any::component::tests::every_declared_kind_is_observable_and_its_inverse_restores_the_document ... ok
test artifacts::pdf::standards::v1_4::subsets::any::component::tests::every_declared_kind_is_observable_and_its_inverse_restores_the_document ... ok
test artifacts::docx::standards::v_ecma_376::subsets::any::component::tests::every_declared_kind_is_observable_and_its_inverse_restores_the_document ... ok
test artifacts::pptx::standards::v_ecma_376::subsets::any::component::tests::every_declared_kind_is_observable_and_its_inverse_restores_the_presentation ... ok
```

`mutate-pdf-1-7` also carries `insert_object_is_unobservable_only_because_nothing_can_reference_the_new_object`,
which turns its one exemption into a checked claim rather than an excuse.

### Repo-wide: how many inverse handlers assert, out of how many

Method: for each of the 164 cases, the feature's tag lines were read for `@oracle-`; for each
oracle-dispatched case the adapter was split at the `#[cfg(feature = "sut")] mod subject` boundary, the
registered handler for `inverse-*` / `mutate-*` / `identity-round-trip` was located, and its call graph
expanded to depth 3 above that boundary and searched for a shared-`⚖️law` call or a
comparison-guarded `return Err` / `raise AssertionError`.

| oracle-role handler | asserts | vacuous | case has no such scenario |
|---|---|---|---|
| `inverse-<kind>` | **62** | **0** | 14 |
| `mutate-<kind>` | **62** | **0** | 14 |
| `identity-round-trip` | **62** | **0** | 14 |

Plus `mutate-json-rfc8259-i-json`, whose oracle is the Python host and asserts all three
(`🐍️component.py:312, 330, 404`). **63 of 63 oracle-dispatched mutate cases assert the inverse law in
role — up from 66 of 86 at w11, with the vacuous bucket now empty.** The 14 without such handlers are
the pre-existing non-mutate cases (`create-and-round-trip-*`, `edit-existing-pdf`,
`extract-text-pdf-1-4`, `zlib-round-trip`, `create-and-edit-archive`, `create-minimal-pdf`). The
arithmetic: 79 features carry `@oracle-`; 3 are TypeScript framework cases with a `🟦️component.ts`
adapter and no mutation vocabulary, 14 are the pre-existing non-mutate Rust cases, and the remaining
**62 are the `mutate-*` cases — every one of which asserts** — plus the Python `mutate-json-rfc8259-i-json`.

The other side of that number: those 63 cases are 63 of **164**. The remaining 101 assert nothing that
has run.

## 5. "Discharged by the subject phase" — measured for the first time

w11 could only say `parity=0/0` and that the Rust subject phase was blocked. It is no longer blocked
for `🗄️stdio`, so this audit ran the probe w11 could not.

**`bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-txt-utf-8` — exit 0**

```
[test] level=exhaustive cases=1 executed=24 passed=24 failed=0 errored=0 parity=0/0
```

That is the first subject execution anywhere in this ticket, and it is real evidence: 24 scenarios of a
no-oracle case discharged exactly as its feature promised.

**`bun ./📜️script.ts subject exhaustive --owner 🏛️architect --case mutate-program-1` — exit 1**

```
[test] not-exercised …/mutate-program-1 (recorded no-oracle decision architect-program-mutation-semantics — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
[test] …/mutate-program-1: rust subject host exited 101 without emitting results
error: could not compile `semio-s-plugin-architect` (lib) due to 2591 previous errors; 13 warnings emitted
```

**`bun ./📜️script.ts subject exhaustive --owner 📕️norm --case mutate-en1990-1` — exit 1**

```
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
error: could not compile `semio-s-plugin-norm` (lib) due to 6082 previous errors; 69 warnings emitted
```

Which plugin libraries actually build (`cargo check -p <plugin> --lib`, error lines counted):

| plugin | errors |
|---|---|
| `semio-s-plugin-stdio` | **0** |
| `semio-s-plugin-norm` | 6082 |
| `semio-s-plugin-architect` | 2588 |
| `semio-s-plugin-block` | 1516 |
| `semio-s-plugin-cad` | 9 |
| `semio-s-plugin-gis` / `-puzzle` / `-forms` / `-dag` | 2 each |

The 2-error plugins are all blocked by ONE upstream error, `E0425: cannot find type Arc in this scope`
at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️gpu.rs:39`. The bulk failures are the
repo-wide async-convention refactor arriving in production plugin code — `async fn encode_op(&self) ->
Result<Vec<u8>, protocol::ProtocolError>` against a now-synchronous trait, in `👁️viewer/`, `✏️editor/`
and `🧬️schema/🧬️mutations/📝️text/` files, **not in any test adapter**. `📕️norm`'s mutation files were
last touched 2026-08-19, before this ticket's audit window, so this is not a regression this wave
introduced — but it does mean the fallback was already unavailable when the 56 cases were written
against it.

### The split that matters

| no-oracle cases | cases | scenarios | subject phase today |
|---|---|---|---|
| `🗄️stdio` (19 semio + txt + binary + 2 DWG + 2 `✳️baseline`) | 25 | 605 | **runs** (proven on txt) |
| every other plugin | 56 | **2,614** | **does not compile** |
| framework-owned | 4 | 12 | not probed |
| **total** | **85** | **3,231** | |

**2,614 scenarios, 57% of the repository, currently have no evidence in any phase and no phase that
could produce any.** That is the single number this audit exists to surface.

## 6. Did anything newly fail, and did anybody weaken an assertion?

### Weakening — a positive finding, checked four ways

* **Comparison profiles**: `git diff dced3e936e HEAD -- "*🔣️component.json" | grep -E "^[+-].*(ignoreKeys|tolerance)"`
  returns **nothing**. No profile in the repo gained an `ignoreKeys` entry or a looser tolerance.
  `semantic-pdf-v1` still declares the same 11 keys and 0.0001; `semantic-mesh-v1` still 1e-05.
* **Fixtures**: `git diff dced3e936e HEAD -- "*component.feature" | grep -E "^-.*(asset://|shared://|local://)"`
  returns **nothing**. Not one existing case's fixture URI was removed or changed. Every `+` line is a
  new case's own fixture.
* **The shared law module**: the only change to `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs`
  since w11 is the ADDITION of `feature_rows` (an `Examples`-table reader, so a subset's unit test runs
  the same rows the case runs) plus its own test. No existing helper's comparison changed.
* **Exemption lists**: the complete set of non-empty observability exemptions in the repo is
  `mutate-jpg-jfif-1-01` (5 of 12), `mutate-png-1-2` (2 of 17), `mutate-pdf-1-7` (`insert-object`, 1 of
  18, NEW this wave and pinned by its own passing test), `mutate-draw-1` (`duplicate-layer`),
  `mutate-remodel-1` (`commit-reconstruction`), `mutate-raster-1` (2 layer-asset kinds), plus five
  `GUARD_VECTORS` lists in `✒️writer`, `🗒️note`, `📋️forms`, `🏛️architect` (6 of 266) and `📖️playbook`.
  Every one is stated in its feature. The `mutate-pdf-1-7` inverse handler's single-axis
  `regenerates_page_content` carve-out (3 of 18 kinds) is unchanged from w11.

**No assertion was weakened, no comparison profile loosened, no `ignoreKeys` added, and no fixture
swapped to dodge a failure.**

### The identity law's byte half — where it is opted out of

The no-byte-pass-through tripwire (`reparsed_not_copied`, or an inline `bytes == input` guard) is
asserted by **55 oracle-dispatched and 13 no-oracle cases**. Its documented mirror `carrier_is_exact`
— "reproducing the input exactly is the CORRECT answer" — is asserted by **8 oracle-dispatched and 42
no-oracle cases**. All 8 oracle-side ones state which of the law's three admissible reasons applies;
`mutate-bmp-v3` is the clearest (*"an uncompressed BMP v3 leaves a writer no freedom at all … the
committed fixture was AUTHORED by this same reference encoder … `law::reparsed_not_copied` would be
exactly backwards here"*), `mutate-zip-2-0` names the fixture's own `1980-01-01` timestamps and
version-20/Unix headers as the writer's defaults. These are documented byte-preserving carriers, which
THE STANDARD allows.

**One family asserts neither half**: the 19 `mutate-semio-*` cases. Their subject `identity` handlers
decode both the `.dsl.semio` and `.pack.semio` envelopes, require the two to agree, re-encode through
both and require model equality — but make no byte claim in either direction and say nothing about why.
Every other `.dsl.semio` carrier in the repo does make one: `mutate-dag-1` asserts the exact-bytes law
inline with a full paragraph (*"`.dag.dsl.semio` is a fixed-layout record grammar and the committed
example is this codec's own output, so the re-printed text was required to reproduce it"*), and the 42
`carrier_is_exact` users name their reason. The semio family is the gap, and it is 507 of the 605
stdio no-oracle scenarios.

### Newly failing scenarios — none in the oracle phase

`failed=0` across all 1,331 executed scenarios. Nothing that ran at w11 regressed, and the two reds w11
left standing are both resolved at the cause (§3). The two failures this audit found are both OUTSIDE
the oracle phase and both attributable:

| failure | message | attribution |
|---|---|---|
| `bun test 🧪️index.test.ts :: discovery is idempotent` | *"this test timed out after 5000ms"* at 5130.40ms | **Our harness.** Discovery cost grew with the case count this wave added. Passes in isolation; the margin is 2.6%. |
| `subject exhaustive` on `mutate-program-1` / `mutate-en1990-1` | *"rust subject host exited 101 without emitting results"* → `could not compile semio-s-plugin-architect (lib) due to 2591 previous errors` / `semio-s-plugin-norm (lib) due to 6082 previous errors` | **Neither our codec nor the reference library nor the fixture** — production plugin code (`👁️viewer/`, `✏️editor/`, `🧬️schema/🧬️mutations/📝️text/`) still declaring `async fn` against traits that are now synchronous, plus one upstream `E0425` in `semio-framework-ui`. `📕️norm`'s files were last committed 2026-08-19, before this ticket's audit window. |

Neither is a reference-library defect and neither is a fixture defect.

## 7. `step-ap214-cc2/cc3/cc4/cc5` — now documented with a citation, and machine-checked

w11 item 8 is closed. Each of the four oracle module headers carries the missing sentence, e.g.
`✳️cc2/🧪️oracle/🦀️component.rs:19`:

> 🧬️ **This catalog declares exactly the same six kinds as `✳️cc3`, `✳️cc4`, `✳️cc5`, and that is a
> CONSEQUENCE rather than a copy.** All four ceilings sit STRICTLY INSIDE the ISO 10303-214 §4.3
> ladder, so all four admit a representation to write AND have at least one rung above them to demote
> from, and all four read the same three axes … because §4.3 varies only the ceiling. `✳️cc1` sits
> below the ladder and `✳️cc6` on top of it, and each declares five kinds instead.

It also names what distinguishes each module (`MAX_RUNG = 2`) and points at the shared `ladder` module
rather than a per-class re-implementation — and the claim is machine-checked, not just asserted:

```
test artifacts::step::standards::v_ap214::reference::component::tests::the_four_interior_classes_share_one_vocabulary_because_their_ceilings_share_one_place ... ok
```

The complete set of identical `kinds` lists repo-wide, re-measured across all **145** catalogs, is
still exactly three groups, and all three are now documented:

| group | verdict |
|---|---|
| `step-ap214-cc2/cc3/cc4/cc5` (6 kinds) | **Documented + machine-checked. Closed this wave.** |
| `ifc-4-any` == `step-ap214-any` (11 kinds) | Documented — both are ISO 10303-21 Part-21 clear-text grammars. |
| `dwg-ac1018-any` == `dwg-ac1024-any` (3 kinds) | Documented + machine-checked; `ac1018` is a one-line `pub use` of `ac1024`, one enum, not a copy. |

## 8. Stubs, placeholders, `todo!`, and templated prose

* **`todo!` / `unimplemented!`**: **zero** across every `🧪️oracle` and `🧪️tests` tree in `✏️s/🔌️plugins`.
* **Placeholder `Err` dispatcher bodies**: none. The only "is not implemented" strings are catch-all
  guard arms unreachable for declared kinds (`🎵️mp3`'s subject half, the shared PDF conformance engine
  at `🧪️oracle/📄️document/🦀️component.rs:2139`).
* **Rejecting stub oracles**: none. All 66 oracle modules cover every kind their catalog declares; the
  one apparent exception, `🖊️dwg ac1018`, is the documented 22-line `pub use` re-export.
* **Arms returning the input unchanged**: still exactly one, `📊️csv rfc4180 "set-has-header"`,
  documented by RFC 4180, pinned by `set_has_header_is_a_true_byte_identity`, and still held to the
  observability law through a caller-tracked flag.

### Templated prose — one genuine silent copy, and it is not in the new work

Feature descriptions were compared pairwise across all 164 cases. **Exactly one pair scores 1.000:**

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🧪️tests/create-and-round-trip-bmp/component.feature`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧪️tests/create-and-round-trip-tiff/component.feature`

The two files are byte-identical apart from `BMP` vs `TIFF` in the `Feature:` line and the three tag
values — same description, same two scenarios, same `{ "width": 4, "height": 4 }` and
`{ "width": 8, "height": 4 }` payloads. TIFF and BMP are not the same format by any specification, and
neither file says they are. **This is precisely the silent copy of a sibling THE STANDARD forbids, and
it is invisible to every gate.** It is pre-existing, not this wave's work — which is the point: 65 new
cases were reviewed and this was not.

Everything else is parameterisation with real per-subset content:

| family | max pairwise similarity | verdict |
|---|---|---|
| `mutate-pdf-1-4-a` / `-x` | 0.947 | conformance-class siblings over one engine; each names its own axes |
| `step-ap214-cc2…cc5` | 0.917 | now explicitly documented as identical by specification (§7) |
| the 6 OOXML conformance cases | **< 0.80** | improved since w11, which measured 0.87 |
| `📕️norm` (15 new features) | 0.682 | genuinely differentiated |
| `🧩️puzzle` (3) / `🧱️block` (3) / `🌀️procedural` (3) / `🌍️gis` (2) / `🏗️fem` (2) / `🔱️trinity` (2) | 0.66 / 0.49 / 0.48 / 0.39 / 0.25 / 0.18 | genuinely differentiated |

### Stale prose that overstates or understates the truth

1. **`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1/component.feature:55`** still reads
   *"⚠️ OPEN, and left red rather than tuned away: `inverse-remove-element` FAILS on the ORACLE side
   today"*. It does not. w11 item 9 and remedy #5 named this exact paragraph; it was not deleted.
2. **15 case files still say the Rust subject phase is blocked**, e.g. `mutate-zip-2-0/🦀️component.rs:10`
   — *"whose subject phase is peer-blocked right now (concurrent os-kernel refactor)"*. `cargo check -p
   semio-framework-os-kernel --lib` is exit 0 and `mutate-txt-utf-8`'s subject phase ran green. The
   files are `mutate-zip-2-0`, `mutate-gif-89a`, `mutate-pptx-ecma-376`, `mutate-txt-utf-8`,
   `mutate-docx-ecma-376`, `mutate-dxf-r12`, `mutate-tiff-6-0` and 8 `mutate-semio-*`.

## 9. Cases reporting green while executing zero scenarios

**In the oracle phase: none.** Every one of the 85 unexercised cases gets its own explicit
`[test] not-exercised …` line naming its decision, and the summary carries `not-exercised=85` beside
`cases=164 executed=1331`. Nothing is counted as passing without executing.

**In the subject phase: the summary line alone would say otherwise.** For `mutate-program-1`:

```
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
```

`failed=0` on a case whose host could not be built. The runner does not hide it — it prints
`rust subject host exited 101 without emitting results`, then `no result stream at …/📤️results.jsonl`,
and exits 1 — but the one line a dashboard would scrape reads clean. Worth a runner change: a case
whose host failed to build should be `errored`, not silently absent from every counter.

### Scenarios executing in NO phase at all

**3,231 of 4,562 (71%)**, across 85 of 164 cases. Split by whether the stated fallback could even run
today:

| | cases | scenarios | subject phase |
|---|---|---|---|
| `🗄️stdio` no-oracle | 25 | 605 | **available** — `semio-s-plugin-stdio` builds, proven on `mutate-txt-utf-8` |
| every other plugin | 56 | **2,614** | **unavailable** — the plugin lib does not compile |
| framework-owned | 4 | 12 | not probed |

w11 measured 565 scenarios in 23 cases. The figure grew 5.7×.

## 10. The concrete list, if this is to be closed out

1. **Make the 56 non-stdio plugins compile, or the 2,614 scenarios they carry are decoration.**
   Four of them (`gis`, `puzzle`, `forms`, `dag`) are blocked by ONE upstream error —
   `E0425: cannot find type Arc` at `🧰️framework/🔨️modules/🖱️ui/…/🎯️targets/🧊️wgpu/🦀️gpu.rs:39`. That
   one fix is the cheapest coverage in the ticket. The rest need the async-convention migration to
   reach `👁️viewer/`, `✏️editor/` and `🧬️schema/🧬️mutations/📝️text/`.
2. **Run the subject phase repo-wide and record a real `parity=` number.** It is no longer blocked for
   stdio. Until it runs, the differential claim this platform exists to make is still unmade.
3. Delete the stale "⚠️ OPEN, and left red" paragraph from `mutate-svg-1-1/component.feature:55`
   (w11 remedy #5, still open), and the "peer-blocked" sentence from the 15 adapters that carry it.
4. Give `create-and-round-trip-bmp` and `create-and-round-trip-tiff` genuinely distinct features, or
   merge them into one shared case that says so. Byte-identical prose for two unrelated formats is the
   silent copy the standard forbids.
5. Give the 19 `mutate-semio-*` `identity-round-trip` handlers a byte claim in one direction or the
   other, the way `mutate-dag-1` and the 42 `carrier_is_exact` users do. 507 scenarios currently assert
   the semantic half only, with no stated reason.
6. Raise the `discovery is idempotent` timeout, or make discovery cheaper. A 2.6% margin under load is
   a flake waiting to be blamed on something else.
7. Make the runner report a case whose subject host failed to build as `errored=1`, not as
   `executed=0 failed=0`.
8. Add a contract rule for "a case declares `@no-oracle-` and its owning plugin does not compile" —
   the gate currently reads a recorded no-oracle decision as satisfied evidence regardless of whether
   the phase it defers to exists.
