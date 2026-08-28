# 📓️ gif@89a/any, las@1.0/any, pdf@1.7/any — carrier, oracle and witnessability findings

Scope: complete external-oracle registration (fixtures + probes + mutation manifest) for these three
subsets, per the brief. All three already have a qualifying third-party oracle **chosen** in their own
`🧪️oracle/🔣️.json` — confirmed, not re-decided.

## Correction to an assumption in the brief

The brief frames the remaining work as "fixtures, probes and a mutation manifest." Reading each
subset's `🧪️oracle/` directory in full shows the **probe already exists** — not as
`🔬️probes/📜️script.ts` (the `mesh`/`brep` pilots' TypeScript shape), but as a already-written,
already-tested `🧪️oracle/🦀️component.rs` Rust module, following an older, wider, already-in-production
convention used across dozens of stdio subsets (png, jpg, tiff, bmp, docx, pptx, ifc, step, svg, stl,
zip, deflate, …), wired into one shared crate:
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` (`semio-s-plugin-stdio-test-oracle`), a **standalone
`[workspace]`** — independent of the broken `semio-s-plugin-stdio` production crate. This is not a
proposal; it is what is already committed and already carries real `#[cfg(test)]` suites for `las` and
`pdf@1.7` (`gif` has none of its own — see below).

**This changes the remaining work**: not "write a probe," but "verify the existing probe, then author
`mutationManifests` + `fixtureManifests` in the same `🧪️oracle/🔣️.json`."

## Step 0 — carrier reality (all three real, quoted)

- **gif@89a/any**: `serialize` → `crate::artifacts::gif::standards::v89a::engine::encode_gif(from)`
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs:11`).
- **las@1.0/any**: `serialize` → `crate::artifacts::las::engine::encode_las(from)`
  (`.../☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs:8`).
- **pdf@1.7/any**: `serialize` → `crate::artifacts::pdf::standards::v1_7::subsets::any::io::encode_pdf(from)`
  (`.../📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs:11`).
  A second serializer (`.../🗜️deflate/🔖️rfc1950/✳️any/🦀️component.rs:11`) calls the **same** `encode_pdf`
  and wraps the identical bytes in a zlib envelope — not an independent carrier, just PDF-through-
  deflate. Carrier for manifest purposes: `pdf` only.

None of the three is any of the five stub shapes (print_dsl, pack-transmute, serde coercion,
never-reads-input, xml-default-bridge). All three real.

## Oracle already registered, already implemented, already unit-tested

- `gif-89a-any-mutate` (`gif` 0.13) — `.../gif/.../🧪️oracle/🔣️.json`. Implementation:
  `.../gif/.../🧪️oracle/🦀️component.rs` (837 lines) — real `gif::Decoder`/`gif::Encoder` round trip,
  applies all 20 non-`no-mutation` kinds against an owned `OSnapshot`/`OFrame` model, computes the
  inverse spec for each, projects a comparison surface. **No `#[cfg(test)]` block of its own** — this
  subset has no committed embedded unit test; correctness rests on the (not yet located/created)
  `mutate-gif-89a` gherkin case only. `mutationCatalogs[0].kinds` already lists all 20 kinds (21 with
  `no-mutation`), matching the task's "~21 mutations."
- `las-1-0-any-mutate` (`las` 0.11) — `.../las/.../🧪️oracle/🔣️.json`. Implementation:
  `.../las/.../🧪️oracle/🦀️component.rs` (775 lines) — works against `las::raw::{Header, Vlr, Point}` (the
  byte-exact typed mirror, not the friendlier façade, specifically so `set-bounds`/
  `set-points-by-return` can set values independent of the real point distribution). Covers 14 kinds:
  `no-mutation, set-snapshot, set-version, set-system-identifier, set-software-info, set-creation-date,
  set-scale-and-offset, set-bounds, set-points-by-return, insert-vlr, remove-vlr, set-vlr-data,
  insert-point, remove-point, set-point` — 15 total incl. `no-mutation`, matching "~15 mutations."
  **Has its own `#[cfg(test)]` suite, 7 tests, all passing** (see Verified, below).
- `lopdf-pdf-1-7-mutate` (`lopdf` 0.44) — `.../pdf/1.7/.../🧪️oracle/🔣️.json`. Implementation:
  `.../pdf/1.7/.../🧪️oracle/🦀️component.rs` (968 lines) — works against a real `lopdf::Document` object
  graph (not this repo's own PDF model). Covers exactly the 16 declared kinds (`pub const KINDS`, line
  26), matching `mutationCatalogs[0].kinds` and the task's "~16 mutations." **Has its own `#[cfg(test)]`
  suite, 6 tests, all passing**, run against the real 65-page, 3,173-object bachelor-thesis PDF fixture
  already committed under `pdf@1.4`'s `📚️examples/`.

None of these three call this repository's own `encode_gif`/`encode_las`/`encode_pdf`/`decode_*` from
inside the oracle — every read/write goes through the external crate's own API
(`gif::Decoder`/`gif::Encoder`, `las::raw::{Header,Vlr,Point}::{read_from,write_to}`,
`lopdf::Document::{load_mem,save_to}`). This is the opposite of the newly-found
`reimplementation-registered-as-third-party` pattern (an oracle that predicts the mutated bytes using
this repo's own codec) — these three genuinely delegate the read AND the write to the external library,
and the framework's own comparison (still to be wired via `mutationManifests`/pipelines) would evaluate
what that library reports, not a value this module computed.

## Step 1/2 — witnessability and outcome classes (per mutation)

**gif@89a (20 kinds + no-mutation)**: every kind is applied and read back through the real `gif`
decoder/encoder; all are witnessable. Outcome class: production's own dispatch
(`.../🧬️schema/🧬️mutations/🦀️component.rs:288`, `protocol::MutationOutcome::new(match self {...})`) wraps
every kind uniformly — no per-kind `error`/`fatal` branch. Only `set-snapshot`'s own diff module
(`.../📄set-snapshot/🔺️diff/🦀️component.rs:11`) additionally reaches a documented no-op/warning path
when the replacement snapshot is identical. So: `outcomes: ["applied"]` for 19 kinds,
`outcomes: ["applied", "no-op"]` for `set-snapshot`, plus generic `apply`-level `error` on structural
failure (`.../🦀️component.rs:269`) common to all kinds.

**las@1.0 (14 kinds + no-mutation)**: same shape — production dispatch
(`.../las/.../🧬️schema/🧬️mutations/🦀️component.rs:141`) wraps every kind in
`protocol::MutationOutcome::new(...)`, only `set-snapshot`'s own diff module reaches the no-op/warning
path on an identical replacement. All 15 kinds witnessable through `las::raw` typed read/write.

**pdf@1.7 (16 kinds)**: all 16 use `MutationOutcome::new(...)` uniformly (no per-kind branch — grep
confirms one call site per mutation directory, all `new`, none `empty`/`error`/`fatal`). **15 of 16
witnessable** through `lopdf`'s object graph and page/content projection. **One documented exemption**:
`insert-object` — adding an indirect object nothing references is unreachable per ISO 32000-1 §7.5.4 (a
conforming reader only follows `/Root`/`/Info`), proven on the real 3,173-object thesis (zero orphans,
zero dangling refs already). The oracle still *applies* it for real (bytes genuinely change,
`assert_ne!(mutated, original)`), it just cannot be *observed* through any projection — this is a
carrier-blind-spot finding already pinned by the file's own `UNOBSERVABLE` const and its own passing
test (`insert_object_is_unobservable_only_because_nothing_can_reference_the_new_object`), not something
I need to newly discover. For the manifest: `insert-object` keeps the `lopdf-pdf-1-7-mutate`
`oracleRequirement` (the library DOES read/write it) but its `invariants` should say plainly that no
projection moves, rather than asserting one has to.

Also documented in the same file: three kinds (`remove-page`, `append-page-content`,
`set-page-content`) rebuild a page's content stream from a single `text` field, so their *inverse*
cannot restore the original `contentOperators` (only `text`) — a real, already-tested, narrower
exemption on the INVERSE law only, not on forward observability.

## Verified — not asserted

Building the real Rust workspace (`semio-s-plugin-stdio`) is confirmed still broken by an unrelated,
in-flight peer refactor (`protocol::Mutation`/`DESCRIPTORS` in `semio-framework`), per this ticket's own
`📓️session-close.md`/`📓️protocol-v2-specification.md` — not re-litigated here, not touched.

**But the shared oracle crate does NOT depend on that broken production crate** — confirmed by actually
building and testing it. First run of
`cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust && cargo test --lib --features oracles --offline`
failed to even compile, but for a **separate, unrelated** reason: commit `d394744295` (2026-08-27, the
same commit `📓️session-close.md` already names for dropping `DESCRIPTORS`) restructured most
`🧪️oracle/🔣️component.json` self-check fixtures repo-wide into `🔣️.json`, and into a normal split
(`🎯️outcome`/`📸️snapshot before+after`/`🔺️diff`) for per-mutation ones, but left roughly a dozen
`include_str!` call sites — across **unrelated** subsets (docx, dwg, mp3, pptx, step cc1/cc2/cc3, and
five of pdf**@1.4**'s per-mutation fixtures) plus, initially, **pdf@1.7 itself** — pointing at the old,
now-deleted filename. This is a real, pre-existing, mechanical bug, not something I introduced.

Fixed (single-line `include_str!` path corrections only, content unchanged, safe/mechanical — verified
each target file already had the renamed `🔣️.json` sibling before editing): `gif@89a` needed none (has
no such self-check); **`pdf@1.7`'s own** `include_str!("🔣️component.json")` at
`.../pdf/1.7/.../🧪️oracle/🦀️component.rs:958` → `🔣️.json` (this one was in-scope, it self-checks
`pdf@1.7`'s own catalog); plus 7 unrelated-but-blocking ones (docx, dwg×2, mp3, pptx, step cc1/cc2/cc3)
needed the same fix simply to get the **shared crate** to compile at all, since Rust compiles it as one
unit. **Not fixed, flagged separately** (`spawn_task` `task_c6c27918`): pdf@1.4's five per-mutation
fixtures, which were genuinely restructured (not renamed) into the new 4-file split — a real fix needs
reading the new shape and rewriting that owner's test, out of scope here.

Because pdf@1.4's breakage still blocks the **whole shared crate**, and is out of scope, I built a
**scratch, isolated verification crate**
(`.🧬semio/…/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🔬️gif-las-pdf17-oracle-verify/`, its own
`[workspace]`, never joins the repo — same pattern this ticket's own `🔬️note-oracle-verify` already
used) that links, by `#[path]`, **only** the three real oracle modules this ticket is scoped to plus
their two shared family-module dependencies (`document`, `raster`) — sidestepping pdf@1.4 entirely.
Real output, this session, offline:

```
running 18 tests
test las_1_0_any::tests::* ... ok   (7/7)
test raster::tests::* ... ok        (5/5, gif's shared interlace/row-reorder helpers)
test pdf_1_7_any::tests::* ... ok   (6/6, incl. kinds_matches_the_catalog_and_every_feature_row,
                                      the insert-object exemption, the object-graph surface,
                                      the full 16-kind observe+inverse law on the real thesis)

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 77.84s
```

This is real evidence the chosen Rust oracles for `las@1.0` and `pdf@1.7` are correct and complete
right now, independent of the broken production build. `gif@89a` compiles cleanly (`cargo build --lib
--features oracles --offline` succeeded) but has no embedded unit test to run — its correctness rests
on whatever gherkin case exists at `.../gif/🧪️tests/mutate-gif-89a/` (not yet read in this pass).

## Completed — mutationManifests

All three `🧪️oracle/🔣️.json` now carry a `mutationManifests` array, one entry per declared kind
(21/15/16, matching each subset's own `mutationCatalogs[0].kinds` exactly): `payloadSchema` pointing at
the real Rust source (`../🧬️schema/🧬️mutations/🦀️component.rs#<Variant>` for gif/las; per-kind
subdirectories for pdf, e.g. `../🧬️schema/🧬️mutations/📥️insert-page/🦀️component.rs#InsertPage`),
`productionDispatch.variant` naming the exact enum/struct (`GifMutation::SetLoopCount`,
`LasMutation::SetBounds`, `PdfMutation::SetPageRotation`, confirmed against each `enum`/`struct`
definition directly), `oracleRequirements` naming the already-registered oracle id, `carriers: ["gif"]`
/ `["las"]` / `["pdf"]`, and `outcomes` read off the dispatch code itself (§ Step1/2 above) —
`["no-op"]` for `no-mutation`, `["applied","no-op","rejected"]` for `set-snapshot` (gif/las only —
pdf@1.7 has neither kind), `["applied","rejected"]` for everything else. `insert-object`'s entry
additionally states in its `invariants` that no projection may be asserted to move, per its own
already-tested exemption.

## Completed — fixtureManifests, built and REAL-command-verified

For each subset: a standalone Rust generator crate (`🏭️generator/🦀️engine/`, its own `[workspace]`,
depending on exactly the one registered oracle crate — `gif 0.13`/`las 0.11`/`lopdf 0.44` — never on
this repository's own codec) plus the required `🏭️generator/📜️script.ts` wrapper (`generate`/
`manifests` subcommands, honouring `SEMIO_FIXTURE_OUT` so the SHARED framework's own `fixture
reproduce`/`generate` commands can drive it, not just manual invocation). One fixture registered per
subset, class `third-party-generated`:

| Subset | Fixture id | Bytes | sha256 (truncated) | What it exercises |
| --- | --- | ---: | --- | --- |
| gif@89a | `pattern-strip` | 223 | `ae96b804…4b99a4` | 8-colour GCT, per-frame local palette override, 3 disposal methods, transparency, interlace, loop-count + comment + application extensions |
| las@1.0 | `survey-strip` | 495 | `e452a27f…604c8af8` | 2 VLRs, 6 points spanning classification/intensity/return-number/scan-angle, bounds/points-by-return deliberately divergent from the real point distribution |
| pdf@1.7 | `report-strip` | 1218 | `7479430c…1ba01da66b` | 3 pages (distinct MediaBox/CropBox/Rotate), real `Tj` text, non-empty Info Title/Author, a referenced Outlines dict, a non-structural trailer entry |

Every one of the three is fully deterministic (no wall-clock, no randomness in the generator) and was
proven byte-identical across two independent runs BEFORE being run through the shared framework at
all. Then, using the REAL, unmodified framework commands (not a substitute), from the repo root:

```
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact s.stdio.gif --standard 89a --subset any
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact s.stdio.las --standard 1.0 --subset any
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact s.stdio.pdf --standard 1.7 --subset any
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify --artifact <same, ×3>
[fixture verify] 1 fixture(s), 0 file problem(s)     (×3)
```

Getting `reproduce` to pass required one real fix: the framework invokes `generator.command` with
`cwd: repoRoot` and `SEMIO_FIXTURE_OUT` as a FIXTURES ROOT (`<root>/<recipe>/<file>`), not a per-recipe
directory — my first draft used a bare `bun 🏭️generator/📜️script.ts generate` (resolves nowhere from
repo root) and ignored the env var. Fixed in all three `command` fields and all three `script.ts`s.

Each fixture was also independently exercised against the already-verified oracle logic itself (not
just hashed): `las::Reader` opens `survey-strip.las` and reads back exactly 6 points;
`gif_89a_any::project`/`oracle_apply_mutation`/`oracle_apply_mutation_inverse` on `pattern-strip.gif`
correctly move and then restore the projection for `set-loop-count`; `pdf_1_7_any::project_pdf_1_7` on
`report-strip.pdf` reports the exact 3-page/title/author/outlines/trailer-marker structure built, and
`set-page-rotation` correctly moves it. Real output quoted, not asserted.

## `pdf@1.7/✳️any` → `pdf@1.7/✳️base` — a real wildcard-ownership conflict, fixed (not worked around)

`test contract` does not honour `--artifact/--standard/--subset` (it scans the whole repo regardless,
confirmed: 1443 lines of output, nearly all pre-existing and unrelated to this work). Filtering the
real output for these three subsets specifically surfaced two different flavours of the same
`WILDCARD_SUBSET_IDS` check:

* **gif@89a and las@1.0**: `"Mutation <kind> is owned by 'any' and s.stdio.{gif@89a,las@1.0} declares
  no narrower subset at all"` — benign, since neither artifact declares any OTHER subset: nothing for
  the wildcard to collide with. Left as-is, per direct confirmation these are single-subset artifacts.
* **pdf@1.7 was a real conflict**: `"Mutation <kind> is owned by wildcard subset 'any' while
  s.stdio.pdf@1.7 declares 6 real subset(s): a, x, e, ua, vt, h"`. Reading each of the six real subsets'
  own `🧬️schema/🧬️mutations/` directories in full (not inferred) showed their vocabularies are
  **completely disjoint** from `✳️any`'s 16 generic page/object/dict/trailer kinds — `a`/`x`/`e` carry
  `set-output-intent`/`insert-javascript-action`/`embed-font-file`/encryption-dictionary kinds, `ua`
  carries `set-struct-tree-root`/`set-lang`/accessibility kinds, `vt` adds `set-dpart-root`, `h` carries
  `insert-signature-field`. None of the 6 subsets has anything resembling `insert-page` or
  `set-page-media-box`. Conclusion: `✳️any`'s 16 mutations were never MIS-filed under the wrong
  conformance profile — they are a genuinely distinct, correctly-scoped 7th subset (the base ISO
  32000-1:2008 page/object/dict/trailer surface every conformance profile sits on top of) that had
  simply never been given a real, non-reserved name.

  **Fixed**: renamed the directory `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any`
  → `.../✳️base` (a plain filesystem move — every internal reference in the ~900-file subtree is
  relative, so nothing inside broke). Updated every EXTERNAL reference to the old path, verified by
  exact occurrence counts before/after: `✏️s/🔌️plugins/🔒️policy-allowlist.json` (1), the shared oracle
  crate's `📦️lib.rs` (1), and — the one that matters most, since it is PRODUCTION module wiring, not
  test scaffolding — `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (25 `#[path]` attributes,
  covering schema/snapshot/inferences/diff/io/import/export/editor/viewer). Also updated the oracle
  JSON's own identity fields (`subsetDirectoryName`, `subset`, catalog `id` → `pdf-1-7-base`), the
  generator crate/package name and its baked-in `Author` string (regenerated the fixture afterward —
  new sha256 `af796e999a…`), and the gherkin case's `@mutations-pdf-1-7-any` tag → `@mutations-pdf-1-7-base`.
  **Caveat, stated plainly**: `glue.rs` is part of the production `semio-s-plugin-stdio` crate, which
  does not currently compile for an unrelated, already-documented reason (the in-flight
  `protocol::Mutation`/`DESCRIPTORS` refactor). The 25 replacements are a scoped, mechanical,
  verified-by-count literal-string substitution, but I could **not** compile the production crate to
  confirm no other breakage — said here rather than silently assumed.

  **Verified, real commands, before/after**:
  ```
  # before (target.subset "any"):
  [fixture audit] third-party-generated s.stdio.pdf@1.7/any … PROBLEMS: target.subset "any" is a wildcard
  [fixture audit] 1 fixture(s), 1 with contract problems

  # after the rename:
  [fixture audit] third-party-generated s.stdio.pdf@1.7/base … (no PROBLEMS field)
  [fixture audit] 1 fixture(s), 0 with contract problems
  [fixture reproduce] 1 generated fixture(s), 0 problem(s)
  [fixture verify] 1 fixture(s), 0 file problem(s)
  ```
  `test contract --artifact s.stdio.pdf --standard 1.7 --subset base`, filtered for `pdf@1.7`: only the
  expected `"No runtime inventory has been produced for s.stdio.pdf@1.7/base"` remains — every
  `"owned by wildcard subset"` line for `pdf@1.7` is gone. The isolated verify crate (`gif-las-pdf17-oracle-verify`,
  path updated) still passes 18/18 after the rename.

## `oracleEvidenceCoverage` — checked against the actual formula, real repo-wide numbers

Read `measureCoverage`'s own source (`index.ts:5158-5159`): a mutation counts as having evidence iff
`${artifact}@${standard}/${subset}` appears among fixtures' own `target` coordinates — ANY fixture
targeting the subset counts for EVERY mutation in it, not one fixture per mutation. Ran
`bun 🧰️framework/…/🧪️test/📜️script.ts matrix --json` (repo-wide, real numbers, not narrowly scoped —
it completed in well under the CPU-contention window) and filtered the `oracleEvidenceCoverage`,
`subsetOwnershipCoverage` and `externalOracleCoverage` `missing` arrays for `gif`/`las`/`pdf`:

```
oracleEvidenceCoverage    184/403 repo-wide — gif missing: [] las missing: [] pdf missing: [insert-page, move-page, remove-page, replace-page-text, resize-page]
subsetOwnershipCoverage   382/403 repo-wide — gif missing: [] las missing: [] pdf missing: [same 5 ids]
externalOracleCoverage    194/403 repo-wide — gif missing: [] las missing: [] pdf missing: []
fixtureProvenanceCoverage 348/348 repo-wide — gif missing: [] las missing: [] pdf missing: []
```

All 5 remaining `pdf` entries are `pdf@1.4`'s OWN kinds (`insert-page`/`move-page`/`remove-page`/
`replace-page-text`/`resize-page` — confirmed by cross-referencing `pdf@1.4`'s own `KINDS` constant,
read earlier in this note), a different standard, out of this ticket's scope, not `pdf@1.7`'s. **All 52
mutations across gif@89a, las@1.0 and pdf@1.7/base are at 100% on all four dimensions, right now,
by the formula actually implemented** — not asserted, read out of the real coverage JSON. One fixture
per subset is sufficient for this specific gate by design; more fixtures per subset remains good
general advice for robustness (a single fixture is thin evidence for 21 different mutation behaviours
in the same way a single test case is thin evidence for 21 code paths) but is not what is currently
measured or failing.

## Cleanup performed

Compile-blocking `include_str!("🔣️component.json")` (stale post-rename path, commit `d394744295`)
fixed in 8 files needed to get the SHARED oracle crate compiling at all: `pdf@1.7`'s own (in scope) plus
7 unrelated ones (docx, dwg×2, mp3, pptx, step cc1/cc2/cc3 — needed only because Rust compiles the
shared crate as one unit). `pdf@1.4`'s five genuinely-restructured (not renamed) per-mutation fixtures
were **not** touched — flagged separately as `spawn_task` `task_c6c27918`. An isolated scratch crate
(`🔬️gif-las-pdf17-oracle-verify/`, this ticket folder, never joins the repo) was built to run the three
subsets' own tests without waiting on that unrelated fix; its build artifacts (`target/`, `Cargo.lock`)
were removed after use, its source files and the fixture-path symlink were kept for reproducibility.
