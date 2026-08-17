# P2-FG1 — 📐️step (ap214) + 🏗️ifc (standard 4) — Grammar/Protocol Recipe Wave Report

Two artifacts this wave (Part-21 syntax siblings), per the brief: decide the shared grammar SHAPE
once, apply to both, but write **separate, non-shared files** per artifact (specific-code mandate —
no cross-artifact `use`, since it doesn't resolve at recognize/walk time anyway per the recipe).

## Scope confirmation

Both `📐️step/🏅️standards/🔖️ap214/**` and `🏗️ifc/🏅️standards/🔖️4/**` only. `🏗️ifc/🏅️standards/🔖️2x3/**`
was explicitly out of scope and is untouched — confirmed via `git status` before/after (zero diff
under that path) and by the scoped test run showing all 46 pre-existing `v2x3::` tests still green,
unmodified.

## What existed before this wave

Both artifacts' 6 `.grammar.semio`/`.protocol.semio` facet files (snapshot/mutations/diff × text/
binary) were the **pre-Phase-2 ABNF placeholder** — a single-line `dialect grammar stdio.step.snapshot`
header (2 tokens on one line, immediately rejected by the real dialect parser: `parse_grammar`
requires `dialect`/`grammar`/`extension`/`start` each on their OWN physical line), ABNF bodies using
`%x` hex ranges, `1*`/`*`, `[...]`/`/`-alternation — none of it valid under this dialect's real
`GKind` token alphabet. Worse, the diff/mutations placeholders described a `serde_json`-shaped wire
(`{"mutation":"setEntityArg",...}`) that the REAL hand-rolled `OpText`/`OpBinary`/`DiffCodec` impls
(landed by the F6a op-codec sub-wave, before this ticket's Phase 2 grammar work began) never emitted
— F6a replaced the old `serde_json`-backed stubs with a real `keyword key=value ...` text grammar,
but nobody updated the grammar/protocol description files to match. This wave fixes both problems:
real dialect syntax AND syncs the description to the REAL, currently-shipping wire shape.

Neither artifact had the 6 conformance-law tests, 5-role `LanguageSpec` registration (only a 1-role
`Document`-only registration existed), or real `.dsl.semio`/`.pack.semio` fixtures (`example.step`/
`example.ifc` existed as unrelated stub assets; `🗣️example.dsl.semio` existed but held a fake
`"Hello, stdio.step!"`/`"Hello, stdio.ifc!"` placeholder string, never real `print_dsl` output; no
`🎒️example.pack.semio` existed for either artifact at all).

## Real-syntax classification (per W0 §1a, confirmed by direct read, not assumed)

Both artifacts' snapshot DSL text IS the real ISO 10303-21 Part-21 exchange-file syntax, produced/
consumed by the SAME shared `step::engine::part21::{Lexer, parse_part21, write_part21}` tokenizer
(ifc/4 imports it directly — confirmed at `🏗️ifc/…/📸️snapshot/🦀️component.rs:14`). This means the
snapshot `.grammar.semio` PRODUCTIONS are structurally identical between step and ifc (same real
wire syntax) — written as two separate files (own copy per the specific-code mandate; cross-artifact
`use` doesn't resolve at recognize time anyway), with ifc's file documenting the parallel in a
comment rather than attempting to share it. The DATA MODEL (mutations/diff grammars) differs
genuinely between the two artifacts (step has typed `StepFileDescription`/`StepFileName`/
`StepFileSchema` HEADER structs and a `StepValue` tag scheme with `U[]`/`D[]` bracketed
payload-free variants; ifc stores raw `Vec<IfcValue>` HEADER tuples and its own `IfcValue` tag
scheme with BARE `U`/`D` payload-free variants, per each artifact's own real hand-rolled codec) —
those files are genuinely, not just nominally, separate.

Both artifacts' `OpText`/`OpBinary`/`DiffCodec` were already real and hand-rolled (F6a wave, before
this ticket) — `StepValue`/`IfcValue` are genuine data-carrying enums (`Integer`/`Real`/`String`/
`Enum`/`Reference`/`Aggregate`/`TypedValue`), confirmed via real `cargo check` `#[derive(dsl::DslDiff)]`/
`#[derive(dsl::DslOps)]` attempts (compiler-error citations preserved as doc comments in both
artifacts' `🔺️diff/🦀️component.rs`/`🧬️mutations/🦀️component.rs`, predating this wave) — `DslField`
has no impl for either value enum, so no derive path exists on either side, on either artifact.
`encode_op`/`encode_diff` on both artifacts are `self.print_op()/self.print_diff()).into_bytes()`
**verbatim** — the same "binary = the text bytes verbatim" simplification `GifDiff`/`SvgDiff`/
`WriterDiff` use elsewhere in the repo — confirmed by direct read, not assumed. This wave did **not**
change that Rust-side behavior (out of scope — the recipe's checklist item is to describe the REAL
wire, and text-bytes-verbatim already IS a legitimate, pilot-precedented real wire; there is nothing
to "upgrade" here, `diffcodec_binary_upgraded`/`opbinary_binary_upgraded` are correctly `false` for
both artifacts).

## Deliverables (both artifacts, full 6-file+fixtures+tests+registration checklist)

### Grammar files (real syntax, real dialect header form)

- **Snapshot** (`📸️snapshot/📝️text/📖️component.grammar.semio`, both artifacts): real ISO 10303-21
  exchange-file grammar — `ISO-10303-21;`/`HEADER;`/the three typed `FILE_DESCRIPTION`/`FILE_NAME`/
  `FILE_SCHEMA` records/`ENDSEC;`/`DATA;`/every `#N=TYPE(args);` instance (incl. the spec-legal
  `#N=(TYPE1(...)TYPE2(...));` COMPLEX-instance form)/`ENDSEC;`/`END-ISO-10303-21;`. Header
  directives per the recipe's own STEP-shaped worked example: `comment line none` + `comment block
  "/*" "*/"` (resolves the `#N=...` entity-ref-vs-`#`-comment collision) + `string single doubled`
  (Part-21's `''`-doubling; the `\X\HH\`/`\X2\HHHH...\X0\` escapes stay opaque `TEXT`-token content,
  confirmed by direct read of `StringEscape::Doubled`'s scanner — it only interprets doubled quotes,
  leaving backslash runs untouched). `.T.`/`.UNSPECIFIED.`-shaped enum literals via `DOTENUM`,
  trailing-dot reals (`0.`/`10.`) via plain `FLOAT`, `$`/`*` unset/derived via `Literal` match on the
  promoted-vs-error-token TEXT (confirmed `Literal` matches by token TEXT only, independent of token
  KIND — works whether `$` lexes as the promoted `Dollar` kind or `#` lexes as an `Error` kind under
  `comment line none`, both carry their own char as `text`).
- **Mutations** (`🧬️mutations/📝️text/📖️component.grammar.semio`): the real `keyword key=value ...`
  op-text form, one alternative per `StepMutation`/`IfcMutation` variant, every literal copied
  verbatim from `print_step_mutation`/`print_ifc_mutation`'s real match arms (traced from the
  function, never guessed) — including the genuine per-artifact difference that STEP's arg-index key
  is `"arg-index"` while IFC's is `"index"`.
- **Diff** (`🔺️diff/📝️text/📖️component.grammar.semio`): the real space-separated `name=value ...`
  diff-text form (only changed top-level fields present), every literal copied verbatim from
  `print_step_diff`/`print_ifc_diff`'s real body — the `entities`/`args` id/index-keyed
  collection-triple shape (§1.4's copy-pasteable pattern) and the `[0]`/`[1,value]` tri-state pattern
  for `StepEntityDiff`/`IfcEntityDiff`'s three `Option`-wrapped sub-fields (§1.4's Tri-state pattern
  — a plain `Option<T>` here, never a genuine `Option<Option<T>>`, so no top-level tri-state wrapper
  was needed on `StepDiff`/`IfcDiff` itself).

Both artifacts' `StepValue`/`IfcValue` single-uppercase-letter tag grammars are modeled EXACTLY per
each artifact's own real `enc_value`/`enc_ifc_value` function — including the genuine difference that
STEP's payload-free `Unset`/`Derived` print as `U[]`/`D[]` (bracketed, empty) while IFC's print as
bare `U`/`D` (no brackets at all) — traced from each artifact's own real encoder, not assumed
identical just because the tag letters match.

### Protocol files (text-native, honest "no packed binary layout" description)

All 6 protocol files (both artifacts × snapshot/mutations/diff) use the identical honest shape:
`framing record` + `chain payload utf8` — matching json's own snapshot-protocol precedent exactly
(§2.1). This is not a shortcut: `StepSnapshot`/`IfcSnapshot`'s real `encode_pack_with` wraps a
`SemioEnvelope` around the UTF-8 `write_part21(...)` bytes (no packed binary layout beyond the
envelope); `StepMutation`/`IfcMutation`'s real `encode_op` and `StepDiff`/`IfcDiff`'s real
`encode_diff` are the print-text bytes verbatim, with **zero** leading format/tag/header bytes at all
(a genuine, further-simplified case beyond json's own mutations-protocol example, which DOES have two
real leading `format u8`/`tag u8` fields before its opaque payload — step/ifc's real wire has none,
confirmed by direct read of both `impl protocol::OpBinary`/`impl protocol::DiffCodec` blocks).

### Real fixtures

`🗣️example.dsl.semio` (real `print_dsl(demo_step_snapshot())`/`print_dsl(demo_ifc_snapshot())`
output, mandatory `semio stdio.step.dsl v1`/`semio stdio.ifc.dsl v1` preamble line included) and
`🎒️example.pack.semio` (real `encode_pack(...)` bytes) for both artifacts — generated via a
temporary `#[ignore]`d `debug_print_demo_fixtures` test in each artifact's own `⚙️engine/🦀️component.rs`
that called the REAL Rust encoders directly, run once with `--ignored --nocapture`, output
hand-copied byte-for-byte (verified with a Python diff against the expected string/hex before
committing), then the temporary test deleted — per the recipe's own instruction. `demo_step_snapshot()`/
`demo_ifc_snapshot()` (new `pub fn`s in each artifact's `⚙️engine/🦀️component.rs`, `DocumentHelpers`
region) are real, minimal, genuine AP214/IFC4 documents (typed HEADER + real entities, e.g. IFC's
demo includes an `IFCPROJECT`→`IFCOWNERHISTORY` reference chain) — the single source of truth for
both fixtures AND for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()` (new `pub(crate)`
fns added to each artifact's own `🧬️mutations/🦀️component.rs`/`🔺️diff/🦀️component.rs`, one
representative case per mutation variant / one empty + two `between()`-direction diff cases,
exercising every `StepValue`/`IfcValue` tag including the recursive `Aggregate`/`TypedValue` cases).

### The 6 conformance-law tests (both artifacts)

Added a new `conformance_laws` submodule inside each artifact's existing `⚙️engine/🦀️component.rs`
`#[cfg(test)] mod tests` region (never a framework file) — `committed_facet_files_parse`,
`grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law`,
`protocol_walk_law`, `fixture_honesty_law` — copied shape-for-shape from `binary`'s own P2-P3
`conformance_laws` module (the direct template), substituting each artifact's own demo
snapshot/mutation/diff cases. All 12 (6 × 2 artifacts) pass — see Verification below.

One real authoring bug caught and fixed during this wave (pitfall #4 from the recipe, hit
independently in both the mutations AND diff grammar files for BOTH artifacts on first draft): the
`document`/`step-value` alternation productions were originally wrapped across two physical source
lines for readability — `parse_sequence` stops at the first `Newline` token, so the wrap silently
truncated the production and mis-parsed the continuation line as an invalid new production
(`"expected Ident, found Pipe"`). Fixed by collapsing each back onto one physical line; re-ran
`committed_facet_files_parse` to confirm.

### 5-role `LanguageSpec` registration

Both artifacts' `register_pilot_languages()` (in `⚙️engine/🦀️component.rs`) expanded from the
pre-existing 1-role (`Document` only) registration to the full 5-role scheme —
`stdio.{step,ifc}`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`, `diff`'s `protocol`
slot `None` matching the exemplar's own shape (the role scheme has no dedicated "diff binary" role
even though the diff protocol file is real and conformance-tested — its binary form is exercised
directly by `protocol_walk_law` instead). **10 total registration roles this wave** (5 × 2 artifacts).

### `register_schema_spec`

Intentionally **not called** for either artifact — `StepValue`/`IfcValue` (genuine data-carrying
enums) have no `DslField` impl, so no `fn() -> RecordSpec` exists for either snapshot or diff type on
either artifact (real `cargo check` confirmed, predating this wave, citations preserved in both
diff files' own doc comments). Filed as the already-known `register-schema-spec-needs-recordspec`
mechanism gap (see `mechanism_gaps` below) rather than fabricating an unrelated spec.

### JSON-transfer elimination

Grepped both artifacts' full `🧬️schema/**` tree for `serde_json::to_vec`/`from_slice`/`to_string`/
`from_str`/`Value` inside `ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks — **zero hits, both
artifacts** (both were already clean before this wave, matching the pilots' own experience — neither
was on W0's original violator list).

## Verification (real, all commands actually run)

The shared tree had multiple large concurrent Phase-2 grammar-recipe sessions active the entire
time — confirmed via repeated real `git status`/direct compiler-error-text reads (not assumed):
transient `E0432`/`E0425` compile failures citing `xml`, `md`, `dxf`, `obj` at various points (each
one a sibling FG-wave agent's own in-progress fixture-file edit, e.g. `couldn't read
.../📝️md/.../🎒️example.pack.semio: No such file or directory` — a file mid-creation by another
session), and a same-session sibling report (`p2-fg1-stl-report.md`, found already present in this
ticket folder) confirming this wave genuinely runs multiple artifact-agents in parallel. Per the
documented "Concurrent Cargo Workspace Churn" pattern, polled (`cargo check`, ~25s intervals) rather
than chasing, classifying every transient failure by file path before treating it as noise — every
one named a file outside `🗿️artifacts/{📐️step,🏗️ifc}/**` before proceeding.

1. `cargo check -p semio-s-plugin-stdio --lib --tests` — clean, 0 errors (confirmed after the
   concurrent churn cleared).
2. `cargo test -p semio-s-plugin-stdio --lib "artifacts::step"` → **106 passed, 0 failed, 0 ignored**
   (`p2-fg1-step-scoped-test.txt`), including all 6 new `conformance_laws` tests.
3. `cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc"` → **74 passed, 0 failed, 0 ignored**
   (`p2-fg1-ifc-scoped-test.txt`) — includes both `v4` (my scope, all 6 new `conformance_laws` tests
   pass) and the pre-existing, untouched `v2x3` standard's own 46 tests (still green, confirmed
   unmodified by `git status`).
4. `cargo test -p semio-s-plugin-stdio --lib` (whole crate, fresh, no filter) →
   **1709 passed, 5 failed, 3 ignored** (`p2-fg1-full-crate-test.txt`). All 5 failures are in
   `artifacts::dxf::standards::v_r12` (1), `artifacts::md::standards::v_commonmark` (1), and
   `artifacts::xml::standards::v1_0` (3) — every one a sibling FG-wave agent's own
   still-in-progress grammar-recipe work (confirmed via `git status` showing those 3 artifacts'
   files actively modified this session by other sessions), entirely outside my ownership boundary
   (`🗿️artifacts/{📐️step,🏗️ifc}/**`). Zero failures attributable to step/ifc.
5. `bun run ./📜️script.ts policy` — ran the full repo-wide policy sweep (`p2-fg1-policy-run.txt`);
   grepped the output for the 5 rules this checklist cares about
   (grammar-parseability/protocol-parseability/fixture-honesty/language-registration/
   json-transfer-ban) filtered to `step`/`ifc` — **zero hits for either artifact** under any of
   those 5 rules. The full policy output is dominated by thousands of pre-existing, unrelated
   `os-state-authority`/`budget` breaches across the whole repo (framework + every other plugin),
   none of which this wave's scope touches or could plausibly have introduced.

## Deviations from the recipe

- STEP/IFC's `StepValue`/`IfcValue` grammars needed the framework's M1 dialect features exactly as
  the recipe's own "STEP-shaped worked example" anticipated (§1.3d/e) — `comment line none` +
  `comment block "/*" "*/"` + `string single doubled` + `DOTENUM`/trailing-dot `FLOAT` — this is the
  FIRST wave to actually exercise those features against a real committed grammar file (the recipe
  cites them from M1's own unit tests, not from any prior pilot's real `.grammar.semio`). No
  divergence from the documented syntax was needed; every feature worked exactly as documented on
  first real use.
- STEP's and IFC's `StepValue`/`IfcValue` tag schemes differ in one concrete way (bracketed `U[]`/`D[]`
  vs. bare `U`/`D` for the payload-free variants) — both grammars model this difference precisely per
  each artifact's own real encoder rather than assuming a shared shape; flagged explicitly above so a
  reader does not assume the two artifacts' value grammars are byte-identical (they are structurally
  parallel but not identical).
- No structural deviation from the recipe's own patterns was needed for either artifact's
  mutations/diff facets (§1.4's collection-triple and tri-state shapes both applied directly).

## Mechanism gaps

| gap id | engine area | symptom | blocking |
|---|---|---|---|
| `register-schema-spec-needs-recordspec` | `dsl::registry::register_schema_spec` | `StepValue`/`IfcValue` are genuine data-carrying enums with no `DslField` impl, so no `fn() -> RecordSpec` exists for `StepSnapshot`/`StepDiff`/`IfcSnapshot`/`IfcDiff` at all — already the known, consolidated gap from the recipe's own table (§5), confirmed to apply identically to both step and ifc; no new variant of this gap discovered. | No — documented, non-blocking, matches 4 of the 6 P1-P3 pilots' own experience (json/csv/zip/png). |

No other mechanism gaps were hit. In particular `protocol-prim-ref-recursion` (§5's headline gap) did
**not** apply at the protocol-description layer for either artifact's mutations/diff facet — because
step/ifc's real `OpBinary`/`DiffCodec` have ZERO fixed leading header bytes (no `format`/`tag` fields
to protocol-walk field-by-field before the opaque payload, unlike json's own mutations-protocol
example), the honest description is "the whole payload is opaque UTF-8 text" (`chain payload utf8`)
from byte 0 — there was never a `Prim::Ref` boundary to hit in the first place, so this is not a new
instance of that gap, just a case where it doesn't arise.

## Files touched (real, live, not reverted)

**📐️step (ap214)**:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (+ `demo_mutation_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (+ `demo_diff_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/⚙️engine/🦀️component.rs` (+ `demo_step_snapshot()`, 5-role registration, `conformance_laws` module)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (overwritten with real `print_dsl` output)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real `encode_pack` bytes)

**🏗️ifc (standard 4)**:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten, real dialect)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (+ `demo_mutation_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (+ `demo_diff_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/⚙️engine/🦀️component.rs` (+ `demo_ifc_snapshot()`, 5-role registration, `conformance_laws` module)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (overwritten with real `print_dsl` output)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real `encode_pack` bytes)

**Ticket-folder scratch** (kept per repo rules): `p2-fg1-step-scoped-test.txt`,
`p2-fg1-ifc-scoped-test.txt`, `p2-fg1-full-crate-test.txt`, `p2-fg1-policy-run.txt`, plus this report.

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema`/`store` framework
crates, `🧪️fixture-sweep`'s `STDIO_CONFORMANCE_GRADUATED` table, and `ifc/2x3` were all untouched —
confirmed by `git status` scoped to those paths showing zero diff from this session.

## Summary

| Check | Result |
|---|---|
| Grammar files rewritten (real dialect) | 6 (3 per artifact × 2 artifacts) |
| Protocol files rewritten (real dialect) | 6 (3 per artifact × 2 artifacts) |
| `DiffCodec`/`OpBinary` binary layout upgraded | No — already real hand-rolled "text bytes verbatim" per artifact, predating this wave (F6a); this wave only rewrote the DESCRIPTION files to match |
| Fixtures regenerated (both dsl.semio + pack.semio, both artifacts) | Yes |
| 5-role `LanguageSpec` registration | 10 roles total (5 × 2 artifacts) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::step"` | 106 passed, 0 failed |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc"` | 74 passed, 0 failed (incl. untouched v2x3's 46) |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | 1709 passed, 5 failed (all 5 in dxf/md/xml, other sessions' own in-progress work, 0 in step/ifc) |
| `bun run ./📜️script.ts policy`, step/ifc under the 5 relevant rules | 0 breaches |
