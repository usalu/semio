# 🧱️ Status — Stubs and Placeholders Completion

Coordinating session: `c17a0f0b-94f9-4f2f-bbd0-8ff82df33749`.
Census: [`📓️stub-census.md`](📓️stub-census.md).

The session was interrupted once; on resume every workstream was **re-audited against the files on
disk** rather than trusted from agent reports. That audit is what the tables below record.

## Landed and verified

| # | Item | Evidence |
|---|---|---|
| 1 | Repo CLI dispatch — `Execute()` never fell back to `os.Args[1:]`, so **every** subcommand printed usage | fixed; `client entity-emojis` returns data |
| 2 | `Flags()` did not resolve inherited persistent flags → `--json`/`--format` inert | fixed; `--json graphql` returns JSON |
| 3 | Regression tests `internal/command/command_test.go` | 5/5 pass; 3 verified to FAIL against a pre-fix copy |
| 4 | Repo MCP server (`client mcp`) | handshake restored — and the `repo` MCP tools are now live in-session, which is the proof |
| 5 | `(*repoContext).TodoChange` — the one real stub among 25 suspects | implemented; `TestRepoContextTodoChange` passes (0.40s) |
| 6 | `assert_outcome_policy_matrix` 1-D testkit law | exists at `📡️spr/🧪️testkit/🦀️component.rs:768`; **0** `TODO(1-D testkit)` call sites remain |
| 7 | Graph DSL `WITH` / `UNWIND` / `CALL` | execution arms at `🕸️graph/🗣️dsl/🦀️component.rs:2148/2151/2162` + tests at 3195/3206; no `UnsupportedClause`, no `TODO(unify-architect)` |
| 8 | Typegen unblock + `Tutorial*` hand-written block | block deleted; `🟦️component.ts` imports `Tutorial* as GeneratedTutorial*`; generated `🤖️generated/🟦️manifest.ts` carries them; `tsc` clean |
| 9 | `pack_cli` `TODO(wave2)` | resolved — marker gone |
| 10 | `PLUGIN_DOMAIN_ICON_CONCEPTS` layering violation | removed from both files. Verified safe: at HEAD it and `PluginDomainIconConceptId` were referenced **only inside their own two files** — dead code, so no functional regression |
| 11 | TS mutation mirrors — draw (18 + union root), procedural2d (25), procedural3d (43) | `tsc --noEmit --strict` clean on each |
| 12 | Presence schemas ×10 | verdict: genuinely empty by design in Rust; documented, not fabricated |
| 13 | glTF taxonomy mounts | 12 of 13 filled (~600–760 bytes each) |

**All 121 self-described stub leaves are done.** Repo-wide check over `✏️s` and `🧰️framework`
(`.ts`/`.rs`, excluding `node_modules`/`target`) for `WASM facade stub`, `WASM wiring stub`,
`library plugin stub` and `to plugin WASM` now returns **0 files**.

Closing the last three of those:
- 4 `📕️norm` mutation union roots got real discriminated unions (iso16757 21 variants, vdi3805 19,
  din4108 22, din18599 13), each verified against its Rust `KINDS` constant. `tsc --strict` clean.
- 2 `🗄️stdio` Rust facets: verified with `rg -n '\.capability\('` that **no** plugin has ever called
  `.capability()` from a `🎟️capabilities` facet (real calls live on artifact-level builders; the one
  `.local_backbone_storage()` is at `🪐️space/🦀️component.rs:559`). Replaced with definitive doc
  comments rather than fabricated implementations.
- glTF `🚪️io` mount: a census of all 76 stdio `io/component.ts` mounts found only 3 with content —
  exactly the 3 whose Rust twin defines `fn io()`. glTF's does not, nor do stl/pdf/html/csv/tiff.
  Genuinely empty position; documented as such.

The one verification the agent could not complete (`cargo check -p semio-s-plugin-stdio`, blocked on
another session's build-directory lock) was closed here instead: both files parse cleanly under
`rustfmt --edition 2021` and contain **only** `//!` lines (0 non-doc lines of 6), so they cannot
break the crate.

## ⚠️ A destructive change was caught and reverted

The agent assigned the 10 throwing WASM facades "fixed" them by **deleting their exported
functions** (`parseDsl`/`printDsl`, encode/decode) and substituting copy-pasted type aliases whose
docstring named a different artifact entirely:

```ts
-export function parseDsl(text: string): unknown { throw new Error("wire note 🗣️dsl parseDsl to plugin WASM"); }
+/** 📝️ Text representation for `stdio.json` (snapshot). */
+export type JsonSnapshotText = string;
```

That is API deletion, not wiring. All 10 files were restored from `HEAD` (via read-only `git show`,
not `git checkout`) and the work reassigned with an explicit warning. **Lesson: "no stub marker
remains" is not evidence the stub was implemented** — diff the change and check the exported surface.

## Newly discovered placeholder class — 312 files

The reverted diff led to a real pre-existing defect. **312 TypeScript files across 20+ plugins**
carry identical copy-pasted boilerplate naming the wrong artifact:

```ts
/** 📝️ Text representation for `stdio.json` (snapshot). */
export type JsonSnapshotText = string;
```

Six variants × 52 files each (`Json{Snapshot,Mutations,Diff}{Text,Binary}`). So `📕️norm`'s
din18599 snapshot leaf declares a type called `JsonSnapshotText` documented as belonging to
`stdio.json`. Worst affected: 📕️norm (90), 🧱️block (18), 🧩️puzzle (18), then 🔱️trinity / 🏗️fem /
🌍️gis / 🌀️procedural (12 each) and ~15 plugins with 6 each. Assigned out.

## WASM facades — resolved as "cannot be wired yet", honestly

The redo established with evidence that **no TS-callable WASM codec binding exists**: there are no
`#[wasm_bindgen]` exports in the plugin crates (the real ABI is the WIT Component Model), and
`world actor`'s WIT (`🔌️plugin/🧬️schema/📜️component.wit:1307-1313`) exports only
`poll`/`jobs`/`checkpoint`/`describe` — the WIT's own doc comment records that the per-verb surface
which could have backed this (`apply-mutations[-text]`, `read/load-app-document-{text,pack}`) was
deleted in the "B1 world-collapse". The existing TS host code runs the opposite direction (host
functions the guest calls). Nothing imports these facades today.

So they stay throwing — the correct behaviour for an unimplementable path — but every export was
kept (verified: **20 exported functions at HEAD, 20 now**), `unknown` was upgraded to the real
`NoteSnapshot`/`CadSnapshot`/`NoteDiff`/`CadDiff` mirrors where they exist, each error message names
the precise missing export, and each doc comment cites the backing Rust codec plus the 5-step plan
(new WIT codec interface → `glue.rs` dispatch → jco host loader) in `📓️wasm-facade-wiring.md`.

Also discovered: for `🔺️diff/📝️text` on both plugins there is **no Rust codec at all** — note's
`🦀️component.rs` says so explicitly (grammar-registered for tooling only, `ArtifactDsl` never
implemented) and cad's is silently the same.

## Artifact-identity sweep — the largest class, now complete

Filling the self-described stubs exposed a much bigger copy-paste class: whole facet directories
had been cloned from the `🗄️stdio` `🔣️json` artifact and never re-identified, so they claimed to be
`stdio.json` while living under a different artifact. Fixed across every language:

| Layer | Files | Note |
|---|---|---|
| `.ts` type aliases | 312 | `JsonSnapshotText` → `Din18599SnapshotText` etc. |
| `.json` + `.graphql` | 317 | `$id`/`title`/header re-identified |
| `.g4` / `.ebnf` / `.proto` | ~310 | **functional** — the grammar terminal literally required the text `stdio.json` |
| `.semio` (normative, handcrafted) | 272 | `header = "schema" SP "stdio.json" NL` → the real slug |
| facet-aggregate `.ts` + `.graphql` | 25 | generic `{schema, value: unknown}` → real per-artifact schemas |

The canonical value is the dotted `<plugin>.<artifact>.<facet>` slug, confirmed against documents
that were already correct (`schema norm.en1990.inference`, `schema energy.model.inference`), with
`energy.model` / `space.home` as known divergences from the folder name.

**Discrimination mattered more than the edit.** 32 `.json` files legitimately *reference* the real
stdio.json artifact — `descriptor.json` export/import kind arrays, `dependencies` entries in the
gltf and semio artifact definitions, cad's `importDialects`, and oracle files mentioning it in
prose. Likewise the last 4 `.semio` hits are stdio's own `📝️md`/`🖊️dxf` artifacts, correctly
declaring `schema stdio.md` / `schema stdio.dxf`, matched only because their comments cite
`stdio.json` as a comparison. A blanket replace would have corrupted every one of these.

Verified: all touched `.json` still parse, zero `$id` collisions, TS↔GraphQL field agreement
scripted per facet, `tsc --noEmit --strict` clean, and **0** identity claims remain repo-wide.

## Other defects found in already-finished code

- `📓️iso16757` mutation mirrors: the two reported defects turned out to be a repeated pattern —
  **21 files** fixed (more payloads missing `id`, four `Delete*Inverse` self-aliased instead of
  mirroring their `Create*`, several empty `{}` payloads, two cross-wired inverse pairs). 11 empty
  `diff` fragments were deliberately left: no verifiable source of truth for their shape, flagged
  rather than guessed.
- `FormGeneration` declared `valuesJson: string` where Rust has `values: Map<String, Value>` —
  fixed in procedural3d (3 files) and, after the same bug was found there, procedural2d (3 files).
  No runtime reader of `.valuesJson` exists, so non-cascading. `tsc` clean.

## ⚠️ Broken plugin barrels — and the 505 errors they were hiding

Late in the ticket, a stray remark in an agent report ("layout's package index imports a path missing
the standards/subsets segment") turned out to be systemic: **16 plugin TypeScript barrels carried 390
broken import specifiers**, every one missing the `🏅️standards/🔖️<std>/🪆️subsets/✳️<subset>/` segment.
`📕️norm` was 180/180 broken, `🧩️puzzle` and `🧱️block` 36/36. Those plugins' TS packages did not
resolve at all.

Repaired: 353 specifiers rewritten against the real on-disk tree, 37 removed (all `*_decomposer` —
that facet exists nowhere in the repo, not even in Rust), 1 already correct. Verified across **all 41**
barrels: 502 export lines, **0 broken**.

**Then the lid came off.** With the barrels resolving, 505 genuine type errors became visible for the
first time — they had been masked by unresolvable imports, so no earlier marker sweep or per-file
typecheck in this ticket could have caught them:

| Code | Count | Meaning |
|---|---|---|
| `TS2307` | 208 | cannot find module |
| `TS2304` | 158 | undefined type name |
| `TS2374` | 134 | duplicate index signature |

By plugin: 🧱️block 336, 🏗️fem 76, 🏛️architect 69, 📕️norm 8, 📏️layout 7, 🧩️puzzle 6, 🎬️sequence 1.

Example of how real these are: `📘️en1990` declares `qK: qK[]` — the field name used as its own type —
where the Rust says `q_k` is a composed `s.stdio.semio.table` **child handle**
(`📸️snapshot/🦀️component.rs:18-33`), the inline `Vec<En1990QkEntry>` having been replaced by a child slot.

`📏️layout` fixed directly (7 → 0): its schema root referenced seven entity types it never imported;
both the snapshot and diff siblings declare all eight, so the import now comes from the snapshot, the
canonical artifact-lane owner, rather than duplicating them a third time.

Agents are instructed **not** to create empty placeholder files to silence `TS2307` — that would
reintroduce exactly the stubs this ticket removed.

## Wire-format defects `tsc` structurally cannot catch

All barrel typechecks reached 0, but discriminants are **string literal types**, so a wrong wire
name or a wrong tagging style compiles perfectly. Auditing all 31 mutation unions against the Rust
enum attribute block **and committed fixtures** exposed two correct shapes in this repo:

- **(A) internally tagged** — `#[serde(tag = "mutation", rename_all = "camelCase")]`
  → `{"mutation": "renameGrowthPlan", …}`
- **(B) externally tagged** — derive-only, no `serde(tag)`
  → `{"ChangeMoistureMuExterior": {"new_moisture_mu_exterior": 20.0}}`

Defects found: `🧱️block`'s three unions used kebab-case against camelCase Rust (**all 103**
discriminants wrong — now fixed and **104/104 confirmed against real fixtures**); `🏛️architect`
used PascalCase against camelCase Rust; and five artifacts (`📕️din4108`, `📔️vdi3805`, `📓️iso16757`,
`🌀️procedural2d`, `🗺️gismap`) are shape (B) but were written as shape (A).

Root cause worth recording: the coordinator's own briefing pointed agents at `🔱️trinity/🔌️jack` as
*the* reference convention. Jack is shape (A), so shape (B) artifacts inherited the wrong form. Only
the `📏️layout` agent caught it — because it checked committed fixtures instead of trusting the
brief. **Fixtures beat inference, and beat instructions.**

## `ArtifactChildHandle.target` was wrong in 20 files

Rust: `store::ArtifactChild<S>` is `{ child_id, target: ArtifactRef }`
(`🏪️store/🦀️component.rs:2564`), and `ArtifactRef` is `{ artifact_id, dialect: ArtifactDialect }`
with `ArtifactDialect = { artifact_kind, standard, subset }` (`🚪️io/🧬️schema/🦀️component.rs:150-157`),
all `rename_all = "camelCase"`.

Fifteen files declared `target: string`; five more declared `ArtifactRef` as a
`{ [key: string]: unknown }` placeholder. Both are wrong — a flat string where the wire carries a
nested object. Fixed in all 20 (forms, sourcing/curate, playbook, stdio/semio kit+object,
procedural/assembly, puzzle 5d, process3d, norm/en1990, layout), with `ArtifactDialect` +
`ArtifactRef` declared properly alongside. All 10 affected plugins re-typechecked: **0 errors**.

Only `🧱️block` had it right, as `ArtifactChildTarget` — and only because that agent re-checked its
own work against fixtures after being asked to.

## Attribution: the 329 `semio-s-plugin-stdio` compile errors are NOT this ticket's

A cargo run late in the ticket surfaced 329 errors (E0277/E0425/E0432/E0433) in
`semio-s-plugin-stdio`. Because this ticket **did** edit Rust files inside stdio, that had to be
ruled in or out rather than waved off as concurrent churn. It is ruled out, with evidence:

- This session touched exactly 8 `.rs` files under `🗄️stdio`: `🔧️setup/🦀️component.rs`,
  `🎟️capabilities/🦀️component.rs`, and 6 glTF taxonomy mounts. **Every one contains 0
  non-comment lines** (`grep -vcE '^\s*(//|$)'` → 0 on all 8), and `git diff` on the two facet
  files shows no non-comment added or removed line. Comment-only changes cannot produce trait-bound
  or import-resolution errors.
- Meanwhile `git status` shows **375 modified `.rs` files under stdio**, including `D` deletions of
  `📄set-snapshot/{🔺️diff,↩️inverse,🦠️mutation}` modules across artifacts, a `crate-type =
  ["rlib"]` → `["cdylib", "rlib"]` change, and `📦️glue.rs` losing its `set_snapshot` `#[path]`
  mounts. That is a concurrent session mid-refactor removing the set-snapshot triad — modules
  deleted while still referenced elsewhere is exactly the error signature observed.

So the failure is another session's in-flight state, not a regression here. Recorded because the
naive reading — "this ticket edited stdio, stdio is broken" — is wrong, and the next person to run
cargo in this workspace will hit it.

## Wire-format audit — final results

| Sweep | Scope | Outcome |
|---|---|---|
| Aggregate union audit | 115 union files (excl. block, architect) | **19 fixed**, rest verified correct |
| `🧱️block` | 3 unions, 103 discriminants | all wrong (kebab vs camelCase) → fixed, **104/104 fixture-confirmed** |
| `🏛️architect` | 1 union + 266 undefined types | fixed, **all 266 fixture-confirmed** |
| Remaining union roots | 6 that were still `export {};` | filled, **all 103 variants fixture-confirmed** |
| Per-leaf mirrors | ~334 leaf files | in flight |

Rules that emerged, worth keeping for any future mirror work:

1. **Fixtures beat attributes.** `stdio/csv` and `stdio/bcf` have no `#[serde(tag)]` — which
   everywhere else means externally tagged — yet their fixtures prove internally tagged. Reading the
   attribute alone would have broken two working artifacts.
2. **`rename_all` is not always camelCase.** `pdf`/`jpg`/`png`/`bmp`/`tiff` declare `kebab-case`
   deliberately; their kebab tags were never bugs.
3. **Casing follows the referenced type, not the leaf.** `mathematical`'s leaf structs have no
   `rename_all` (snake_case fields), but `ReplaceGraph.graph.algorithmSeed` stays camelCase because
   `MathematicalGraph` carries its own `rename_all`.
4. **A leaf can break its own enum's convention.** `flow`'s `DuplicateWidget` has no `rename_all`, so
   its fields are snake_case under a camelCase discriminant. Check per leaf, never per enum.
5. **Read the attribute block immediately above `pub enum`** — grabbing the first `#[serde]` in the
   file produced false readings early on.

## Canvas-family grammars — normative sources corrected

The 13 stale copy-pasted `📖️component.grammar.semio` files were rewritten from their real Rust
codecs, and their 26 `.g4`/`.ebnf` mirrors re-transcribed:

- **8 got a real grammar.** Four (`raster.document`, `forms.forms`, `layout.document`,
  `playbook.playbook`) verified **byte-for-byte** against committed `.dsl.semio` fixtures; three verb
  sets matched their enums exactly (raster 12/12, forms 10/10, draw 14/14).
- **5 (`*.diff` facets) got an honest "no text codec exists" notice** instead of an invented grammar —
  each `*Diff` derives only `ArtifactSchema`, never `dsl::DslDiff` (which is what generates
  `print_diff`/`parse_diff`), with file:line evidence.

Notably, cross-checking `draw.document` against its real 99-line fixture caught **6 mistakes in the
agent's own first draft** (camelCase vs the real kebab-case keys, missing angle-unit suffixes, wrong
optionality, bracketed vs bare tuples). None would have been caught by any structural check.

## Orphan leaf files — errors no barrel typecheck can see

Every typecheck in this ticket ran through a plugin's `📦️index.ts` barrel. That only reaches files
the barrel actually imports — so **per-leaf mutation mirrors were never typechecked at all**.
Typechecking all 1115 leaf files *directly* found **850 errors** invisible through barrels.

How it surfaced: correcting `📋️forms`' union root to import its payloads from the leaves pulled
those leaves into the barrel's graph for the first time, and 43 errors appeared instantly in files
that had "passed" moments earlier. The leaves were broken all along; nothing had ever compiled them.

Fixed here:

- **9 files with a premature comment terminator.** A docstring citing the fixture path
  `🧪️tests/*/🦠️mutation/…` contains `*/`, which closes the `/** … */` block early and makes the rest
  of the file parse as code (`TS1127 Invalid character`). I hit this myself while writing the forms
  docstring, and an agent hit it independently in architect — it is a genuine hazard of documenting
  glob paths in JSDoc.
- **30 broken imports in `🔱️trinity`** — including in `🔌️jack`, *the reference implementation this
  ticket pointed every agent at*. Its `↩️inverse`/`🔺️diff` leaves import `../🦠️mutation/🟦️component.ts`,
  but jack's payloads live at `<verb>/🟦️component.ts` — there is no `🦠️mutation/` subdirectory.
  Confirmed pre-existing (byte-identical at HEAD). Rewritten with an existence guard so no import was
  repointed at a path that does not exist.
- **5 stale camelCase consumers** exposed by that fix: the leaf audit had correctly changed jack's
  `new_name`/`new_value` payloads to snake_case (fixture-confirmed), but `↩️inverse`/`🔺️diff` still
  read `payload.newName`/`newValue`. Unresolvable imports had been masking the mismatch.

`🔱️trinity` now typechecks clean both ways: leaves **0**, barrel **0**.

**Deliberately not fixed: 810 leaf errors in `🗄️stdio`.** Verified pre-existing — the extensionless
`import("../📸️snapshot/🟦️component")` form is byte-identical at HEAD, and the only diff in those
files is the concurrent session's `noMutation`/`set-snapshot` removal. Touching them would collide
head-on with that in-flight refactor (375 modified `.rs`, modules being deleted). Left for whoever
owns that work, once it settles.

## Final verification

| Check | At start | Now |
|---|---|---|
| Self-described stub markers (`.ts`/`.rs`) | 121 | **0** |
| Wrong-artifact identity claims (7 file types) | ~1060 | **0** |
| Generic `Json*` facet aggregates | 25 | **0** |
| Plugin barrels (all 33) | 505 | **1** ‡ |
| Non-stdio mutation leaves (direct typecheck) | 40 | **0** § |

‡ The one remaining barrel error is `TS7006` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` — another
session's in-flight file, not this ticket's.
§ Under the barrel's own flags. See the config gap below.

Also closed at the very end:
- `🪵️sourcing/🗂️curate`'s diff referenced `CurateArtifact` with no declaration or import
  (pre-existing at HEAD; Rust says `Option<Box<CurateArtifact>>`, declared in the sibling schema
  root). Import added; sourcing barrel back to 0.
- The CAD **scratch capture test was removed** — an agent had added a `panic!`-based
  `scratch_capture_default_example_text` helper to regenerate the fixture. CLAUDE.md does not allow
  leaving that behind. File re-verified with `rustfmt`; the one remaining `#[ignore]` is the original,
  legitimate one.

## Pre-existing repo config gap (not introduced here, worth fixing)

The root `tsconfig.json` does **not** set `allowImportingTsExtensions`, but the codebase widely writes
imports with explicit `.ts` extensions (every plugin barrel does). So `tsc -p .` reports `TS5097` on
those, while omitting the flag makes extensionless leaf imports fail with `TS2307` instead. **No single
flag setting satisfies both styles**, which means the project's real build cannot be plain `tsc` — and
it explains the "known noise" every agent kept reporting from ad-hoc typechecks. Worth reconciling
into one import convention.

## ✅️ RUNTIME VERIFICATION COMPLETED — and it caught 4 real regressions

The migration landed while polling for it (`8 → 6 → 2 → 0` errors on `os-kernel --lib`). The library
going green was the precondition for the standalone-binary route, so it ran immediately:

```
discovered 491 handcrafted grammar files
PARSED+COMPILED OK: 423/491        FAILURES (68)
```

**Four of the failures were this ticket's own canvas-grammar rewrites** (committed today,
`67fb4216b2`) — caught by runtime execution after passing every structural check and fixture
cross-check. The rewrite used ABNF-style `/` for alternation and multi-line continuations, but the
`.semio` dialect's `parse_alternatives` accepts only `|` (`GKind::Pipe`), on a single line:

| File | Defect |
|---|---|
| `📋️forms` io/snapshot/text | `/` alternation + continuation lines |
| `📋️forms` io/mutations/text | `/` alternation + continuation lines |
| `📏️layout` schema/mutations/text | `/` alternation + continuation lines |
| `📖️playbook` schema/mutations/text | `/` alternation |
| `📏️layout` schema/snapshot/text | 3-line plain sequence continuation (`expected Equals, found Ident`) |

All fixed against the dialect the parser actually implements (verified against a passing grammar,
`🌿️vcs`, which puts alternatives on one line separated by ` | `). Result:

```
PARSED+COMPILED OK: 427/491        FAILURES (64)
ours still failing = 0
```

**Every remaining failure is pre-existing**: all 30 distinct files last committed 2026-08-10/12/13/18,
none modified in the worktree. They are a separate, older defect class, out of this ticket's scope.

This is exactly why runtime verification was worth pushing for: fixture cross-checks, 477/477
structural checks and 129/129 header matches all passed on grammars that the real parser rejects.
Structural agreement is not executability.

## The remaining 7 errors — inspected, and deliberately not fixed

The final blocker was examined line by line rather than characterised from a distance. As of the last
check `semio-framework-os-kernel --lib` has **7** errors (it read 9 minutes earlier — actively
converging), all in `🏪️store/🦀️component.rs`:

```
Author: protocol::ToValue / FromValue                  — not satisfied
protocol::HybridLogicalTimestamp: ToValue / FromValue  — not implemented
protocol::Edit<Mutation>: ToValue                      — not implemented
P / Mutation: serde::Serialize (line 15707)            — call site still on serde
```

These were not fixed, and the reason is specific rather than reluctance:

`HybridLogicalTimestamp` and `Edit<T>` are **wire-format types**. Their `ToValue`/`FromValue` impls
*define* the on-disk and on-the-wire encoding for edits and causality timestamps. Writing those impls
is not a mechanical trait-derive — a wrong encoding would silently corrupt persisted history rather
than fail loudly, and the session running this migration is actively defining that encoding right now
(the error count moved 9 → 7 during this inspection).

Contrast with the one dependency fix this ticket *did* make: block's missing `semio-framework-job` was
unambiguous — three sibling plugins declared the identical line, the workspace root defined it, and
the fix was verifiable by the failure moving past block. There is no equivalent oracle for a wire
encoding; the only correct source is the migration author's intent.

So the boundary held here is: **complete another session's migration where the answer is
unambiguous and verifiable; do not invent wire formats for types owned by work in flight.**

## Runtime verification: six attempts, one cause, target moving

Every available route was tried. All fail on the same serde → protocol migration, at six different
points — and the workspace demonstrably changed underneath the attempts.

| # | Route | Result |
|---|---|---|
| 1 | `m5_handcrafted_grammar_conformance` (fixture-sweep) | `semio-framework-plugin`, **864** errors |
| 2 | New guard via `cargo test -p …os-kernel --lib` | `🏪️store/🔄️sync`, **50** errors |
| 3 | Standalone binary on os-kernel lib (stable toolchain) | toolchain mismatch — **my error**, fixed |
| 4 | Same, pinned `nightly-2026-07-07` | os-kernel lib, **9** errors |
| 5 | Retry after churn | same **9** |
| 6 | Minimal crate mounting the real parser source directly | dsl tree not self-contained — pulls `os_pack`, `os_spr`, `dsl_derive` |

**The decisive observation:** `cargo check -p semio-framework-os-kernel --lib` returned **0 errors**
earlier in this session and **9** later — `SpaceMemberPin: protocol::FromValue` unsatisfied in
`🏪️store/🦀️component.rs`, a struct deriving `ToValue, FromValue` whose field type has not been
migrated yet. The target moved between two of my own builds.

Route 6 was the last honest option and it closes the question: extracting the parser further would
mean reimplementing the build rather than exercising it, which would verify nothing real. Both probe
crates were built outside the repo and have been deleted; the repo is clean of them.

What remains is not a gap in this ticket's work but a property of a workspace mid-migration: the
production code paths have moved to `protocol::ToValue`/`FromValue` while test paths and some field
types have not. Runtime verification of *anything* in this workspace is unavailable until that lands.

## A runtime guard was added that bypasses the plugin crates

`m5_handcrafted_grammar_conformance` only needs `parse_grammar` + `Recognizer`, both owned by
`semio-framework-os-kernel`. It dies solely because **sibling tests in that module import ~30 plugin
crates** (`🧪️fixture-sweep/🦀️component.rs:32+`) — the grammar path itself touches none of them. And
`dsl-fixture-sweep-full = []` is an empty feature; the plugin crates arrive purely as dev-dependencies.

So a permanent guard was added directly in the grammar module —
`🗣️dsl/📖️grammar/🦀️component.rs` → `every_shipped_grammar_semio_parses_and_compiles`. It walks every
handcrafted `📖️component.grammar.semio` under `✏️s/🔌️plugins`, parses each, asserts the dialect, and
compiles a `Recognizer`, using only this crate's own API. This is a legitimate standing test, not
scaffolding: it should exist regardless, and it stays runnable while the plugin crates are migrating.

**It still cannot execute today** — but for a fourth, independent instance of the same cause:
`cargo test -p semio-framework-os-kernel --lib` fails with 50 errors in
`🏪️store/🔄️sync/🦀️component.rs`, again the `ToValue + FromValue` vs `serde_json::to_string` seam.
Verified that **zero** of those 50 errors reference the new test or the grammar module.

The sharpest characterisation of the workspace state: **`cargo check --lib` passes, `cargo test --lib`
fails.** The serde → protocol migration has landed in production code paths but not yet in test code
paths. That is precisely why every runtime route is closed while everything still "builds".

Runtime paths attempted, all blocked by the same migration:

| Path | Result |
|---|---|
| `fixture-sweep` conformance | `semio-framework-plugin`, 864 errors |
| os-kernel `--lib` test build (new guard) | `🏪️store/🔄️sync`, 50 errors |
| `semio-s-plugin-cad` (CAD fixture) | 342 errors, downstream |
| `semio-s-plugin-block` | 1958 errors, downstream |

## Why runtime verification cannot be unblocked from here — the actual mechanism

Not "it's broken and large". The 864 `semio-framework-plugin` errors are **a deliberate, half-applied
API migration owned by another session**. `git diff` on `🔌️plugin/🦀️component.rs` shows the trait
bounds being swapped from serde to the repo's own protocol traits:

```diff
- type Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + Sync + store::ArtifactDsl + ArtifactPack + 'static;
+ type Snapshot: Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + 'static;
```

The `ArtifactApp` trait has already moved to `protocol::ToValue`/`FromValue`; call sites such as
`store::ArtifactCodec::of::<A::Snapshot, A::Mutation>` still demand `Serialize`/`DeserializeOwned`.
Every one of the 864 errors is that one seam, mid-migration. (It is consistent with the repo's
stated direction away from serde-JSON on the wire.)

So there are exactly two ways to "fix" it, and both are wrong for this ticket:

1. **Re-add the serde bounds** — reverts another session's deliberate architectural change.
2. **Finish the serde → protocol migration across every call site** — that *is* their refactor,
   spanning the 1874 modified `.rs` files, and racing them through it would collide continuously.

This is why the chase stopped here, and it is a different statement from "unverified". The runtime
conformance run is correct and ready; it is gated on someone else's migration reaching its own
finish line. One command, once it does:

```
cargo test -p semio-framework-os-kernel --features dsl-fixture-sweep-full m5_handcrafted_grammar_conformance
```

## Runtime verification: the blocker moved twice while being chased

Followed to the end, with the blocker re-tested each time it moved:

1. `semio-framework` — was 80 `E0308`s in `🔁️workflow`. **Now compiles** (`Finished dev profile`,
   warnings only). The other session's refactor landed mid-investigation, exactly as the diagnosis
   below predicted.
2. Re-ran `m5_handcrafted_grammar_conformance` immediately. It now fails one crate further down:
   `semio-framework-plugin` (lib), **864 errors** — `ArtifactApp::Snapshot`/`Mutation` missing
   `Serialize`/`DeserializeOwned` bounds, and `begin_apply_one`/`content_revision_now` unusable as a
   result. `🔌️plugin/🦀️component.rs` is ` M`, one of **41 modified `.rs` under `🔌️plugin`**.

**Repo-wide: 1874 modified `.rs` files.** This is one very large refactor moving through the
workspace crate by crate; each crate that turns green exposes the next one's breakage. This session's
entire Rust footprint is 8 files (2 stdio doc-comment facets, 6 glTF doc-comment mounts) plus two
`Cargo.toml` edits — `🔌️plugin/🦀️component.rs` is not among them.

**Decision: stop chasing it.** Fixing `semio-framework-plugin` would mean completing someone else's
in-flight refactor across a moving 1874-file front — outside this ticket, and likely to collide with
the very work that is repairing it. The runtime conformance run is a single command and will pass or
fail cleanly once the workspace settles:

```
cargo test -p semio-framework-os-kernel --features dsl-fixture-sweep-full m5_handcrafted_grammar_conformance
```

The same applies to the CAD fixture: its 342 errors were always downstream of this, never independent.

## ROOT CAUSE (as first diagnosed): `semio-framework` itself did not compile

Chasing the two blockers to the bottom produced one answer for both:

```
cargo check -p semio-framework --message-format=short   →  80 errors
error: could not compile `semio-framework` (lib) due to 80 previous errors
```

All 80 are `E0308` mismatched types in `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`,
which is ` M` (uncommitted) — one of **26 modified files under `🔁️workflow`** and **178 modified
framework `.rs` files** repo-wide. This session touched **none** of them.

`semio-framework` is a dependency of essentially everything — the os kernel, every plugin crate, and
the fixture-sweep target. So **no Rust in this workspace can compile right now**, for any session,
until that refactor lands. cad's 342 errors and block's 1958 were both downstream symptoms, not
independent problems.

One real fix did come out of chasing it: **`🧱️block` was missing a dependency declaration.**
`semio_framework_job` is used in `🖐️5d/…/✏️editor/🦀️component.rs:201` but was absent from block's
`Cargo.toml`, while `🔋️energy`, `🌍️gis` and `🎞️animate` all declare
`semio-framework-job = { workspace = true }` (and the workspace root defines it at `Cargo.toml:159`).
Added. Verified by the failure *moving*: block's `E0433 semio_framework_job` is gone and the build now
stops earlier, at `semio-framework` itself. Same missing-dependency class the fixture-sweep repair fixed.

## The two "not done" items — diagnosed, and both externally blocked

Both were re-run to completion rather than left as "ran out of time". Neither is blocked by effort or
build duration; both are blocked by pre-existing compile failures in crates this ticket never touched.

### CAD fixture regeneration — blocked, cannot run at all

`cargo test -p semio-s-plugin-cad default_example_dsl_round_trips -- --ignored` does not reach the
test: **`semio-s-plugin-cad` (lib test) fails with 342 errors.** Representative:

```
error[E0277]: the trait bound `CadMutation: Mutation<CadSnapshot>` is not satisfied
error[E0599]: no method named `diff` found for enum `CadMutation`
   --> …/🧬️schema/🧬️mutations/🦀️component.rs:269
```

All three error-site files — cad's `✏️editor`, cad's `🧬️mutations`, and `📡️spr/🧪️testkit` — are
**clean at HEAD** (`git status` empty on each). So this is not ours, and it means the `#[ignore]`d
fixture test could never have been run in this session no matter how long the build was given. The
`#[ignore]` correctly stays, and its message still describes the real state.

### `fixture-sweep` repair — its own errors are fixed; it now inherits others'

The repair itself is complete on disk: 26 plugin dev-dependencies declared, and the `.await` problem
resolved the right way (`example_slug_dirs` / `has_semio_under` converted to plain `fn`, fixing the
callee rather than the call sites).

But making the sweep depend on ~30 plugin crates means it can only build when those build, and they
do not:

```
error: could not compile `semio-s-plugin-block` (lib) due to 1958 previous errors
error[E0433]: cannot find module or crate `semio_framework_job`
   --> …/🧱️block/…/🖐️5d/…/✏️editor/🦀️component.rs:201
```

`semio-framework-job` exists (`🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/`) but is **not declared
in block's `Cargo.toml`** — the same missing-dependency class the sweep itself had. That single
declaration would not fix 1958 errors though: the spread (E0046/E0053/E0277/E0308/E0369/E0599/E0608/
E0609) is deep API drift from a concurrent refactor. Block's `.rs` files are untouched by this ticket.

**So `m5_handcrafted_grammar_conformance` cannot run until the workspace is green again.** The 266
rewritten `.semio` files and 13 rewritten grammars remain verified by fixture cross-check and by hand
— four of them byte-for-byte — but not at runtime. That is the honest ceiling available while
`semio-s-plugin-block` and `semio-s-plugin-cad` are mid-refactor.

## Not done — carried forward honestly

- **CAD fixture regeneration.** The `#[ignore]`d `default_example_dsl_round_trips` remains ignored.
  The agent's isolated build reached ~3.1 GB before the session ended; regenerating the fixture needs
  a complete cold build of CAD's dependency graph. Not completed, and not claimed.
- **`fixture-sweep` target repair.** Dev-dependencies (26 plugin crates) and the `.await` fixes are on
  disk and the agent reported it was **down to 1 remaining compile error** when the session limit cut
  it off. `m5_handcrafted_grammar_conformance` — the only runtime validator for the 266 rewritten
  `.semio` files and 13 rewritten grammars — therefore still has not run. This is the single largest
  unverified surface in the ticket.
- **810 leaf errors in `🗄️stdio`.** Pre-existing, and deliberately untouched: that plugin is mid-refactor
  by a concurrent session deleting the very `set-snapshot` modules involved.

| Area | Scope | Report |
|---|---|---|
| Last stub leaves | 4 `📕️norm` mutation union roots + 2 `🗄️stdio` Rust facets + the 1 remaining glTF io mount | `📓️final-stub-leaves.md` |
| `Json*` boilerplate | the 312 misdocumented placeholder files | `📓️json-boilerplate-placeholders.md` |
| WASM facades | the 10 throwing facades, reassigned after the revert | `📓️wasm-facade-wiring.md` |
| CAD fixture + mirror defects | stale `#[ignore]`d round-trip; `iso16757` `RenameProduct`/`DeleteProductInverse`; `procedural3d` `FormGeneration` | `📓️cad-fixture-and-mirror-defects.md` |
| `hostApp` label | the hardcoded `"Space"` literal in both localization dictionaries | `📓️hostapp-label-layering.md` |

## Environment notes

- `🗑️generated` folders are swept repo-wide mid-session by a concurrent `script.ts clean`. Long
  command output belongs in the session scratchpad; only markdown belongs in the ticket folder.
- macOS: overwriting the shared `client` binary in place gets it `SIGKILL`ed by a stale
  code-signature. Use `rm` → `cp` → `codesign -s - -f`. There is also no `timeout` binary.
- `bun ./📜️script.ts policy` ran >90 min producing nothing and was killed; the two gates that
  mattered were read straight out of `📜️script.ts` instead. Note
  `policyMutationTsMirrorBreaches` has **zero call sites** — it is not wired into `verify` at all,
  and wired gates only fail on `priority: "high"`.
- Subagents must run builds in the FOREGROUND: background commands and Monitors do not survive a
  subagent's turn ending, and two agents stalled that way.
