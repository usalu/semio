# W6 ratchet — composition policy report

Scope actually implemented (per the corrigendum-narrowed brief): three composition-specific policy
predicates in repo-root `📜️script.ts`. `declare_artifact!` macro collapsing, `MeshExporter`/
`MeshImporter` deletion, `📇️catalog.json` generation are APA's (explicitly out of scope, untouched).
`semio-hub io-leaf routing policy` and `taxonomy.json shapes` were **not implemented** — no clear,
low-risk, well-defined work item for either surfaced from reading the actual code (see "Skipped" below).

## Pre-flight

`git status --porcelain -- 📜️script.ts` and `git diff --stat -- 📜️script.ts` were both empty before
starting — no live concurrent edit to this file, safe to proceed.

## Policies added

All three live in a new `//#region 🔧️PolicyRuleComposition` inserted between
`//#endregion 🔧️PolicyRuleDissolvedKernels` and `//#region 🔖️PolicyExport`, and are aggregated by
`policyCompositionBreaches` (📜️script.ts:12790), wired into the main `policy` export at
📜️script.ts:12861 (`breaches.push(...policyCompositionBreaches(repoRoot));`), directly under the
existing `policyDissolvedKernelsBreaches` call, following the exact same aggregator pattern as
`policySchemaOverhaulPCBreaches`/`policyDissolvedKernelsBreaches`.

### 1. `policyCanonicalArtifactKindBreaches` — 📜️script.ts:12580

Checks every literal `ArtifactKindSpec { id: "…" }` construction site repo-wide against the canonical
grammar `s.<plugin>.<artifact>` (exactly three dot-separated ASCII segments, first literally `s`, the
rest lowercase-kebab) — a direct TS mirror of `is_canonical_artifact_kind`/`ArtifactKindId::parse` in
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs:101-153`.

**Scope decision (the important disambiguation the brief asked for):** only literal string ids inside
`ArtifactKindSpec { id: "…" }` are checked — these are genuine kind-*declaration* sites (the
pre-migration `pub fn artifact_kind() -> ArtifactKindSpec` shape, ~87 of them across the plugin tree).
`#[child(kind = "…")]` values are explicitly OUT of scope: they legitimately carry a 4th subset
segment (`s.stdio.semio.<subset>`, confirmed by grepping the real migrated plugins — e.g.
`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:23`)
because stdio's single `semio` artifact hosts all 18 subsets under one 3-segment kind — these are
*references* to an already-declared kind, not declarations, exactly the "media-port tag" carve-out the
brief named (confirmed the same pattern for `MediaPortSpec.kind_id: Option<String>` references).
Non-literal id expressions (`format!(...)`, a delegating helper fn like norm's
`app_surface::artifact_kind_spec`) are skipped rather than guessed at — 7 such sites exist and are
honestly out of this rule's reach (see Gaps).

A same-line `//` guard (`policyMatchIsCommentedOut`, 📜️script.ts:12561) skips matches that are
doc-comment prose quoting the pattern as an example — this exact false-positive class is called out in
this ticket's own `📌️important.md` ("strip comments, check the target") and was empirically hit once
during development (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs:160` quotes
`` `#[child(kind = "s.stdio.semio")]` `` inside a `///` doc comment).

**Priority: `medium`** — deliberately, so this known pre-migration debt (renaming existing ids is
explicitly "a later wave" per the io/component.rs doc comment, and belongs to APA's eventual
registration-macro pass) does not gate the build. This is a ratchet against the count growing further,
not a mandate to fix all 86 now.

### 2. `policyChildSlotKindDagBreaches` — 📜️script.ts:12642

Builds a directed graph over every `#[child(kind = "…")]` composition-slot declaration under
`✏️s/🔌️plugins/**/🧬️schema/**` and runs DFS cycle detection, so no artifact may (transitively) compose
itself as a child (design doc: `CompositionGraph{Owns: forest}`).

**How `#[child(kind = "…")]` is actually spelled** (grepped 5 stdio + 62 plugin real occurrences before
writing the detector): always `#[child(kind = "s.<plugin>.<artifact>[.<subset>]")]`, generated-derive
attribute consumed by `#[derive(ArtifactSchema)]`'s `parse_child_kind`
(🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs:146). `policyChildSlotOwner` (📜️script.ts:12616)
derives a node identity per schema file from its own path (`<plugin>/<artifact>[/<subset>]`) plus the
kind string(s) another file's `#[child(kind=…)]` would use to target it — this correctly resolved 97 of
97 real `#[child(kind=…)]` occurrences in the live repo with zero unresolved targets (verified with a
throwaway script before writing the real detector).

Same `//` same-line comment guard applied (two doc comments in `🧿️semio/🦀️component.rs` quote this
exact attribute with a real quoted string and would otherwise have produced a false self-loop breach —
caught and fixed during development, before landing in script.ts).

**Priority: `high`** — a real cycle would be a genuine structural bug; current count is 0, so this
costs nothing today and gates any future regression.

### 3. `policyDissolvedKindRedefinitionBreaches` — 📜️script.ts:12754

Seeded-allowlist ratchet, exactly as instructed. Bans any plugin outside `🗄️stdio` from declaring a
`struct`/`enum` with the exact name of one of the 18 frozen stdio `🧿️semio` subset snapshot types
(`SemioAnimationSnapshot` … `SemioVideoSnapshot`, `POLICY_DISSOLVED_KIND_CANONICAL_TYPES`,
📜️script.ts:12724 — confirmed by grepping the real `📸️snapshot/🦀️component.rs` files under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️*`). `POLICY_DISSOLVED_KIND_REDEFINITION_ALLOWLIST`
(📜️script.ts:12719) is seeded **empty**, matching the brief exactly.

**Deliberately narrow, on purpose**: exact type-name collision only, not a fuzzy shape-similarity
heuristic. Verified before writing: grepping the design doc's own named legacy duplicates (`CadEdge`/
`CadWire`/`CadFace`/`CadShell`/`CadSolid` in cad, `KitCatalog`) confirms they're already gone as
*persisted* types — the only remaining `Cad{Edge,Wire,Face,Shell,Solid}` hit is a `pub(crate)`,
explicitly-documented-ephemeral import intermediate
(`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️component.rs`,
doc comment cites this exact ticket's corrigendum) — correctly NOT flagged by a name-exact check. Also
checked `LocalizedText` as a candidate ban target and deliberately did NOT add it: one instance remains
outside stdio (`✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs:110`) with an in-file doc comment
explaining it's a deliberate architecture decision (a small leaf value nested by the dozens as an
ordinary field, not a top-level composable content slot) — adding it to the ban list would have been a
false positive against a reasoned, already-documented exception, exactly the trap the brief warned
against ("if it finds a large number of hits… narrow the heuristic").

**Priority: `high`** — current count is 0 (the migration wave's job, done), so this also costs nothing
today and gates the exact regression class the ticket exists to prevent.

## Verification

### Before (baseline, captured before any edit)

```
$ git status --porcelain -- 📜️script.ts && git diff --stat -- 📜️script.ts
(both empty)

$ bun ./📜️script.ts policy
23888 high-priority breach(es) across 30 rule(s)
  (top: 22274 handcrafted-grammar/spec-distinctness, 336 os-state-authority/item-scope-global,
   276 artifact-schema/facet-completeness, 255 taxonomy/dead-example-leaf, …)
exit 1

total breach records (all priorities, from .🦑️repo/⚡️cache/breaches/compose.json): 29601
```

Matches DKM's pre-briefing baseline closely (23,792 → 23,888; 22,274 spec-distinctness identical) —
small drift from live concurrent sessions in the ~90 minutes between DKM's measurement and mine,
exactly as `📌️important.md` warned to expect ("re-measure, don't gate on a stale baseline").

### After (same commands, same repo, after the edit)

```
$ bun ./📜️script.ts policy
23888 high-priority breach(es) across 30 rule(s)
  (identical top-30 list and identical per-kind counts to the before run — byte-for-byte same 30 kinds)
exit 1

total breach records (all priorities): 29687   (= 29601 + 86)
```

**Per-new-policy breach counts** (from `.🦑️repo/⚡️cache/breaches/compose.json` after the edit):

| kind | count | priority |
|---|---|---|
| `composition/canonical-artifact-kind` | 86 | medium |
| `composition/child-slot-kind-dag` | 0 | high |
| `composition/dissolved-kind-redefinition` | 0 | high |

**Confirmed**:
- (a) runs without crashing, both before and after.
- (b) new policies fire a small, fully-explained, non-inflated set: 86 medium-priority (expected,
  pre-migration legacy `artifact_kind()` ids, real and individually spot-checked, see samples below) +
  0 + 0 high-priority. Zero surprise/inflated dumps.
- (c) **no existing policy's breach count changed** — the before/after per-kind tallies for all 30
  pre-existing high-priority kinds are identical, and the all-priorities total moved by exactly +86
  (all attributable to the one new medium kind). `29601 + 86 = 29687` ✓.
- **0 new high-priority breaches** — matches the design doc's own W6 exit criterion verbatim ("Exit: 0
  new high-priority breaches; verify gate no worse than W0 baseline").

Sample of the 86 `composition/canonical-artifact-kind` hits (spot-checked, all real, none false
positive): `"3d.mesh"`-style legacy grammar across `writer` (`text.document`), `mathematical`
(`computation.mathematical`), `cad` (`3d.cad`), every stdio format artifact (`stdio.json`,
`stdio.png`, …), `puzzle`/`sourcing` (`kit.catalog`, the exact duplicate the design doc names),
`architect` (`data.🏛️program`, an emoji id). All are pre-existing `artifact_kind()` bodies untouched by
this migration wave — this ticket's registration-macro pass is APA's, ceded per the corrigendum.

### Typecheck

```
$ bunx tsc --noEmit -p tsconfig.json 2>&1 | grep -i "script.ts"
(no output — no type errors attributed to 📜️script.ts)
```

### Repo-lib TS tests (optional per brief; extended, not created new)

Found and extended the existing policy test file
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` (already
tests `policyEmojiPrefixBreaches`/`policyWindowCompletenessBreaches`/`policyModeCompletenessBreaches`
individually against `mkdtempSync` fixture dirs — same pattern followed). Added one new
`describe("composition policy …")` region (`//#region 🧪️CompositionPolicy`) with 3 tests, one per new
exported policy function (`policyCanonicalArtifactKindBreaches`, `policyChildSlotKindDagBreaches`,
`policyDissolvedKindRedefinitionBreaches` — all three made `export`ed to allow direct unit testing,
matching how `policyEmojiPrefixBreaches` etc. are already exported).

```
$ bun test 🧪️index.test.ts -t "composition policy"
3 pass, 163 filtered out, 0 fail, 12 expect() calls
```

```
$ bun test 🧪️index.test.ts   (full file, unfiltered)
138 pass, 28 fail, 900 expect() calls
```

**The 28 failures are pre-existing / concurrent-churn, not caused by this change.** None of them are in
the new composition-policy region (that region's 3 tests are all in the 138 passing). Every failing
test name is in an area this change never touched: `bundle-script`, `dependency-boundary`,
`ui scrollbar styling`, `micro-commit`, `playground static sites`, `package boundary guards`, `commit`,
`command budgets`, `resolveCargoPackageName`, `loadTaxonomy`, `validateTaxonomy`, `discoverPackages`,
`computeWorkspaces` — several of these assert against "the real repo root" (`discoverPackages`,
`computeWorkspaces`, `loadTaxonomy`) and fail on drift like a missing
`✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/package.json` semio-role marker and a missing
`🧰️framework/🔨️modules/🧮️math/➕️algebra/📦️packages/🦀️rust` path — exactly the shape of the live,
in-flight `math`→`geometry`/`graph` crate extraction and other concurrent-session churn this ticket's
own `📌️important.md` describes as active tonight. This change's only edits to this test file are one
import-list addition and one fully self-contained new `describe` block using isolated `mkdtempSync`
temp dirs — it cannot have caused failures in tests that walk the real repo root for unrelated subjects.
Not investigated further or "fixed" here — out of this ticket's scope and exactly the trap
`📌️important.md` warns against ("never 'fix' someone else's file").

## Skipped from the original plan text (deliberately)

- **`declare_artifact!` macro collapsing / `MeshExporter`/`MeshImporter` deletion / `📇️catalog.json`
  generation** — explicitly CEDED to ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE per the design doc's own
  corrigendum. Not touched.
- **`semio-hub io-leaf routing policy`** — no concrete, low-risk predicate found. Grepped for an
  io-leaf/hub-routing convention to validate and found nothing narrow enough to lint without
  guessing at a rule that doesn't yet exist in code; the composition/link-resolution machinery this
  would police (`resolve-artifact-link`, `LinkResolver`) is still framework-side and W1-owned/in-flux
  per `📓️status.md`. Skipped per the brief's own instruction not to force scope that isn't
  well-defined.
- **`taxonomy.json shapes`** — `📓️status.md` explicitly reassigns this: "`🔣️taxonomy.json`… **#2553
  next**, then UCAS W6" and later "APA done; queue UCAS-W6 → SMO → IIF → DKM" for `📜️script.ts` — but
  no taxonomy.json shape change specific to composition (child/link slot vocabulary) was evident in the
  current taxonomy schema during this session, and `📌️important.md`'s own DKM pre-briefing flags the
  one known taxonomy-adjacent gap (`policyTaxonomyDirsBreaches`'s dead restrictive walk at
  📜️script.ts:4076+) as explicitly "not urgent" and informational-only, not part of this task. Left
  untouched.

## Honest gaps

1. **`policyCanonicalArtifactKindBreaches` cannot see non-literal id expressions.** 7 sites use
   `format!(...)` or a delegating helper (`norm`'s `app_surface::artifact_kind_id(variant)` →
   `format!("computation.norm.{variant}")`, `block`'s `KIT_CATALOG_ARTIFACT_ID.into()`, `animate`'s
   `PRESENT_DOCUMENT_SCHEMA.into()`) — these are also non-canonical in practice but the rule
   deliberately does not attempt to resolve arbitrary Rust expressions statically, to avoid
   guessing/false claims. Documented as a known limitation, not silently swallowed.
2. **The DAG detector only sees `✏️s/🔌️plugins/**/🧬️schema/**`.** If a future composition slot is
   declared somewhere outside that path shape (unlikely given the taxonomy, but not physically
   impossible), it would silently not participate in the graph. Matches the same file-shape assumption
   every other taxonomy-aware policy in this file makes.
3. **The dissolved-kind ban is intentionally exact-name-only.** A plugin could still reintroduce
   duplicated *shape* under a different name (e.g. `struct ProbeMeshData` with the same fields as
   `SemioMeshSnapshot`) and this rule would not catch it — accepted per the brief's explicit
   instruction to keep this heuristic narrow rather than risk a large false-positive dump.

## Files touched

- `/Users/ueli/Documents/semio/📜️script.ts` — added `//#region 🔧️PolicyRuleComposition` (3 policy
  functions + `policyCompositionBreaches` aggregator + 2 small helpers + 1 const array + 1 allowlist
  Set), and one line wiring it into the main `policy` export.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
  — added 3 imports and one new `//#region 🧪️CompositionPolicy` test block (3 tests).
