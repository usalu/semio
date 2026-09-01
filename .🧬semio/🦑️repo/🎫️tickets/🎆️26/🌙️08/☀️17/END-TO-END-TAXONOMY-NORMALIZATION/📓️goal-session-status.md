# 📊️ Kind-Only Leaves — Session Status

Goal: the monorepo follows the domain-driven multi-implementation tree taxonomy — **folders carry
semantics, files are kind-only leaves** (`🦀️.rs`, `🟦️.ts`, `🔣️.json`), with implementation-neutral
folders for schema, components, fixtures, tests and oracles.

Baseline commit for every measurement below: `bb06c41f73f0122fbed315b7487428b976f99921`.

## 1. Landed

### Mechanism

| item | before | after |
|---|---:|---:|
| planned moves, scope `🧰️framework/🔨️modules/🧬️schema` | 3 | **7** |
| `collision-*` rows | 5 | **0** |
| `semantic-stem-unresolved` rows | 3 | **0** |
| `unresolved` decisions blocking `apply` | 124 | **127** (composition changed, see §2) |
| `rust-path-join-unproven` in the 25 files worked | 128 | **0** |

The engine now plans exactly the goal's shape, including the two new neutral folders:

```
🔣️entity-kinds.json → 🏷️entity-kinds/🔣️.json
🦀️validator.rs      → ✅️validator/🦀️.rs
🦀️component.rs      → 🦀️.rs
🟦️component.ts      → 🟦️.ts
📦️glue.rs           → 🦀️.rs
```

`🏷️entity-kinds` is deliberately language-neutral: the same directory name carries the JSON source
and its generated TypeScript sibling (`🤖️generated/🏷️entity-kinds/🟦️.ts`).

### Defects found and fixed

1. **The pipeline poisoned its own reference closure.** `📊️taxonomy-plan/🔣️.json` and
   `📊️taxonomy-verification/🔣️.json` are written into the ticket, and the closure admits untracked
   files, so the tool's own output returned as unrewritable references — `unresolved` 504 with them
   present vs 124 parked, identical scope and baseline. It could never converge. Fixed structurally
   by reserving and excluding a ticket-root `🗑️temp` child. Note `--plan` is *rejected* outside the
   repository (`taxonomyCliGuardedPath`), so this exclusion is required, not a convenience.
2. **Duplicated implementation in two proc-macro crates.** `🧬️schema/✨️derive` and
   `🗣️dsl/✨️derive` each held their whole implementation twice — owner `🦀️component.rs` and
   `📦️packages/🦀️rust/📦️glue.rs` differing only in rustfmt wrapping. The `collision-*` rows were the
   gate correctly detecting it. De-duplicated to the repo's own glue-as-wiring pattern
   (`🔄️machine/✨️derive`, 33 lines): **4 927 → 2 581 lines, −2 346**. A slice had suppressed the
   gate instead; that suppression was reverted, since `package-implementation` fires at 226 sites
   repo-wide.
3. **`build.rs` had no contract.** Cargo mandates the basename; it is now
   `fixedFilenameContracts.cargo-build-script`, so all 7 stop emitting `semantic-stem-unresolved`.
4. **Frozen historical evidence.** 25 `frozenMarkdownCoordinateEvidenceContracts` generated, plus a
   justified widening of `markdownSourceCoordinateSpans` to admit heading inline-code.

## 2. Per-mutation facet restoration (dev decision, 2026-08-28)

A concurrent session had collapsed each mutation's `🔺️diff/` and `↩️inverse/` directories into the
direct leaf. The dev overruled it. Two things were done:

**Contract** — `mutationOptionalFacetDirs` split into
`mutationBehaviorFacetDirs = ["🦠️mutation","🔺️diff","↩️inverse"]` (required-when-present, never
inlinable) and `mutationOrganizationalFacetDirs = ["🧩️plan","📝️text","💾️binary","🧬️schema"]`,
`_mutationOwnershipComment` rewritten, every consumer updated, gate + test added. The gate first
shipped marker-based and was **blind to 352 leaves** (`🏛️architect` inlined without any `//#region`
marker); it now detects structurally.

**Restoration** — 20 plugins, seven workers:

| | before | after |
|---|---:|---:|
| mutation leaves with inlined behaviour | 1 167 | **0** |
| correctly faceted | 49 | **1 216** |

All restored facet leaves are kind-only `🦀️.rs`. Four workers first wrote them as
`🦀️component.rs` (404 files) and were corrected. The 340 named facet leaves that remain are in
mutations that were *never* inlined — pre-existing legacy stems awaiting the engine's own rename,
not a regression.

`🦠️mutation` needs no restoration: sampling against the pinned commit showed all 1 164 deletions
were legacy central-only nestings with no direct leaf, i.e. valid cutovers.

## 3. Remaining blockers (127)

| class | n | note |
|---|---:|---|
| `unsupported-path-syntax` in closed-ticket reports / `.cursor/plans` | 74 | frozen-evidence residue; unmarked prose spans the scanner excludes by design |
| `rust-path-join` | 50 | multi-target join loops — see §7. NOT churn, and NOT stale literals; my earlier note saying so was wrong |
| `frozen-coordinate-evidence-unowned` | 4 | pre-existing, unrelated |
| `rust-path-join-unproven` | 3 | tail |

`apply` refuses while ANY unresolved decision exists repo-wide, regardless of `--scope`, so all of
these gate the 43 449 renames.

## 4. Not attempted, recorded

- **TypeScript surfaces deleted.** ~3 255 `🟦️component.ts` facet files were removed and, unlike
  Rust, were *not* inlined anywhere (0 of 516 TS mutation leaves). `🏛️architect`'s mutation
  directories now hold no TypeScript at all, while mutation descriptors declare
  `requiredLanguageSurfaces` including `typescript`. A separate ~3 000-file decision.
- **Semantic-collection completeness.** `generate taxonomy census` reports 12 362 problems
  (`manifest-child-missing` 4 756, `member-component-leaf-missing` 3 533,
  `collection-manifest-missing` 1 884, …) — the other half of "folders carry semantics".
- **`♻️mit-bestand`** waits on a peer's gitlink repository-boundary contract; all runs here are
  `--scope`d around it.

## 5. Build verification status

`cargo check` could not be completed for the plugin crates: `semio-s-plugin-stdio` carries **65
pre-existing E0046/E0425/E0599 errors** with ~2 900 lines of uncommitted changes from another
session mid-flight, and it transitively blocks every plugin. Four workers hit this independently
and quoted the real output. Evidence in hand is therefore structural: `rustfmt --check` clean across
every touched file, brace balance, 0 residual inlined leaves, delegate calls and glue mounts
verified pairwise. **The restoration is structurally verified, not build-verified** — that must be
re-run once `🗄️stdio` settles.

## 6. Convergence verified (coordinator, independent of the slice's own report)

Stale pre-fix artifacts (`📊️taxonomy-plan`, `📊️taxonomy-verification`, `📓️taxonomy-plan`,
`📓️taxonomy-verification`) were still sitting at the ticket root from runs made before the writer
change; two were tracked. Removed, then two consecutive plan runs with **no `--plan` override and no
manual parking**, run 1's artifact deliberately left in place for run 2:

```
run 1  moves=7 roots=0 relocations=0 symlinks=0 removals=0 edits=201 regenerations=3 unresolved=127
run 2  moves=7 roots=0 relocations=0 symlinks=0 removals=0 edits=201 regenerations=3 unresolved=127
```

Identical. Artifacts now land at `🗑️temp/📊️taxonomy-plan/🔣️.json` and
`🗑️temp/📓️taxonomy-plan/📝️.md`, structurally outside the reference closure
(`TICKET_GENERATED_OUTPUT_DIRECTORY`, `explicitTicketRows`, `taxonomyCliArtifactPath`).

Against the original measurement — `unresolved=504` with artifacts present vs `124` parked — the
pipeline now converges on its own. This was the precondition for ever reaching a clean gate.

## 7. Correction: the `rust-path-join` class was misdiagnosed twice

I first recorded these 50 rows as churn-coupled stale literals, and the slice working them reached
the same conclusion. Both were wrong. The emitter is unambiguous — `🧹️normalization/🟦️.ts:4298`:

```ts
else if (targets?.length !== 1) unsupportedReason = "Rust manifest-relative path lacks complete physical source authority";
```

The rule is **exactly one physical target per join reference**. The surviving rows are inner loops
of this shape, e.g. `🔱️trinity/🔌️jack/…/🧬️mutations/🦀️component.rs:77`:

```rust
for surface in ["🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
    let surface_source = std::fs::read_to_string(owner.join(surface)).expect("direct language surface");
```

One `owner.join(surface)` ranging over five literals ⇒ five targets ⇒ flagged. Round 1 unrolled the
OUTER tuple loops; these inner `for surface in [...]` loops are the remainder.

The literals themselves are CORRECT and must not be "fixed": verified on disk, `🗑️delete-node/📝️text/`
and `🗑️delete-node/💾️binary/` really do contain `🦀️component.rs` — nested facet leaves still on the
legacy stem, awaiting the engine's own rename pass. The defect is loop SHAPE, not values. Rewriting
them to `🦀️.rs` would break the tests.

Lesson worth keeping: a diagnosis that explains the symptom (stale-looking literals next to renamed
files) is not the same as reading the rule that produced it. Two rounds were spent before anyone
opened the emitter.

## 8. `rust-path-join` — three wrong diagnoses, and what the evidence actually supports

This class was misdiagnosed three times. Recording all three so the next person does not repeat them.

| # | diagnosis | by | verdict |
|---|---|---|---|
| 1 | churn-coupled stale literals naming `🦀️component.rs` in already-renamed dirs | coordinator | **wrong** — literals verified against disk; `📝️text/🦀️component.rs` and `💾️binary/🦀️component.rs` genuinely exist under those names |
| 2 | multi-target inner `for surface in [...]` loops (`targets.length != 1`) | coordinator | **wrong** — all 13 files unrolled to one literal join per block; count did not move |
| 3 | TOCTOU race against a concurrently-rewriting session | slice | **not supported** — see below |

### Why the race explanation does not hold

Four isolated plan runs, same scope and baseline, spread across the session:

```
r1 unresolved=127   r2 unresolved=127   r4 unresolved=127   r5 unresolved=127
```

`rust-path-join` was exactly **50** in every one, across the same 13 files, at the same line
numbers. A TOCTOU race does not produce byte-identical results four times. The slice observed 74
rows on one run against 50 on mine and concluded race; the more likely reading is that its run
overlapped the facet restoration, which was actively rewriting those crates at the time, while all
four of mine ran after it quiesced.

A second hypothesis of mine — that untracked restored facet files break the crate's proof chain —
is also disproven, and inverted: the flagged plugins have **0 untracked** files
(🌿️vcs, 🎬️sequence, 🔱️trinity, ✒️writer), while the unflagged ones carry hundreds
(🧱️block 208, 🏛️architect 532).

### What the evidence does support

- Deterministic and reproducible, not a race.
- The population is precise: the 13 `🧬️schema/🧬️mutations/🦀️component.rs` mutation-ROOT files that
  carry the big self-verification test, all in plugins that were never inlined.
- The slice's instrumentation is the one solid clue: execution returns from
  `rustFiniteManifestTargets` via an **early hash/mtime/stat guard over the crate's module-graph
  proof chain**, before ever reaching the per-candidate loop. `targets` is therefore `undefined`,
  and line 4298 (`targets?.length !== 1`) fires on `undefined` exactly as it would on a genuine
  multi-target — the two failure modes are indistinguishable in the message.

### Recommended next step

Instrument the early guards themselves (not the candidate loop) on one file and identify which
guard returns and for which member of the proof chain. Do not restructure the Rust again: two
rewrites have already been spent on hypotheses that the measurement did not support. Note also that
the diagnostic text conflates "no targets resolved" with "several targets resolved"; splitting those
two messages would have saved all three misdiagnoses and is worth doing regardless.

### This does not gate on its own

Even at zero, `apply` stays blocked: 73 `unsupported-path-syntax` rows in closed-ticket prose remain,
and the frozen-evidence slice determined most are unfreezable by design — admitting them would mean
trusting a heuristic scanner as exact evidence, which defeats the contract. That is a decision about
the evidence mechanism, not more grinding.

## 9. Why no rename has ever been applied: historical prose gates every scope

`clean taxonomy apply` refuses while `plan.unresolved` is non-empty. I probed the smallest, most
isolated modules in the repository to find any scope that could apply today. Real output, isolated
`--plan` paths:

| scope | moves | edits | regenerations | unresolved |
|---|---:|---:|---:|---:|
| `🧰️framework/🔨️modules/🔢️number` | 2 | 8 | 1 | **28** |
| `🧰️framework/🔨️modules/⏱️trace` | 3 | 16 | 1 | **88** |
| `🧰️framework/🔨️modules/🧬️schema` | 7 | 201 | 3 | **127** |

For `🔢️number`, the smallest in the repo:

```
unresolved: 28   all reference-syntax-unsupported
by kind:  unsupported-path-syntax 26, rust-path-join-unproven 2
by area:  historical/ticket 28      ← 100%, zero production rows
```

The two moves it wants are `📦️glue.rs → 🦀️.rs` and `🦀️component.rs → 🦀️.rs`. **Two renames, blocked
by 28 bare path tokens in prose inside closed tickets' reports.**

This is the root explanation for the whole programme's stall. It is not a tail case: every scope,
however small and isolated, is gated by closed-ticket text mentioning its files. There is no scope
that applies today, which is why 43 449 planned renames have never been executed.

It also shows why per-coordinate freezing cannot finish. Closing this by freezing byte-coordinates
would mean freezing essentially every historical document in the repository, forever, and
re-freezing them after every future rename — the wall reappears at each wave.

The clean generalization, now in flight: **a closed ticket's own documents are historical evidence,
not live references.** Rewriting them falsifies the record; leaving them in the closure blocks every
rename they mention. The existing `frozen*EvidenceContracts` already encode "neutralize, do not
rewrite" — this extends that from per-coordinate to per-document over a tightly bounded population
(`📓️` ticket reports and `.cursor/plans/*.plan.md`), with production source, schemas, manifests and
configs untouched.

## 10. The historical-evidence contract, and the one gate that remains

### `unresolved` reaches zero for the first time

Extending the frozen-evidence principle from per-coordinate to per-document
(`historicalDocumentEvidencePopulations`, enforced at the single choke point
`repositoryReferenceCandidatePaths`) cleared the class that gated every scope:

| scope | before | after |
|---|---:|---:|
| `🧰️framework/🔨️modules/🔢️number` | 28 | **0** |
| `🧰️framework/🔨️modules/🧬️schema` | 127 | 53 (only the unrelated `rust-path-join` class) |
| repo-wide `unsupported-path-syntax` | 73 | 1 |

Three populations, all bounded by document KIND rather than ticket lifecycle — a `📓️` report
describes what was true when written whether or not its ticket has closed, and the engine's only two
fates for a reference are *rewrite* or *block*, both wrong for a narrative record:

- `ticket-report` — `^📓️.+\.md$` under a ticket root
- `ticket-workspace` — direct children of a ticket root (evidence snapshots, scratch scripts, `📌️` notes)
- `cursor-plan-snapshot` — `.cursor/plans/*.plan.md`

Two negative checks, both derived from the schema rather than hardcoded, keep it honest: never exempt
a path matching any `fixedFilenameContracts` pattern, and never exempt anything inside a
ticket-embedded package boundary (an ancestor owning a `scope.kind === "package-root"` manifest).
Verified against two real ticket-root Cargo packages — their sources stayed live.
`📌️important.md` was confirmed emptiness-only (`planTicketImportantRemovals` /
`projectTicketImportantFiles` read `entry.size` and the manifest, never the file's content).

### The remaining gate: generator input enumeration traverses into the submodule

With `unresolved=0` and a plan digest, `apply` still dies:

```
error: Generator input crosses an index-owned repository boundary: ♻️mit-bestand (♻️mit-bestand/recherche)
  at assertTransactionRepositoryPath (🧹️normalization/🟦️.ts:10296)
```

Measured from the plan artifact: the single regeneration is `plugin-registry`, whose input set is
**40 232 paths spanning the whole repository** (`✏️s` 33 338, `♻️mit-bestand` 5 157,
`🧰️framework` 1 691, …). Because those inputs are effectively repo-wide, *every* plan for *every*
scope carries this regeneration — confirmed across four independent probes, `regenerations >= 1`
each time. And 5 157 of them recurse INTO the submodule
(`♻️mit-bestand/recherche/_archive/dropped_knots/…`).

So apply is blocked repo-wide by exactly one thing: **generator input enumeration does not treat a
gitlink as terminal.** The designed contract already says it should — `retained-gitlink-boundary`,
`authoredSource: false`, `traversal: "terminal"`, zero nested traversals — it is simply not
implemented on the generator-input path. No escape-hatch flag; the general contract is the fix.

## 11. First renames committed — and the loop that produced them

### Applied to disk, transaction `state = committed`

```
🧰️framework/🔨️modules/🔢️number/🦀️component.rs                → 🦀️.rs
🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust/📦️glue.rs  → 📦️packages/🦀️rust/🦀️.rs
🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust/Cargo.toml  → path = "🦀️.rs"   (engine rewrite)
```

plus the `🖍️draw` editor-command projection (11 moves) that had to land first:
`…/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/…`
→ `…/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/…`, leaves kind-only.

Three independent gates had to fall, in this order, for any rename to execute at all:

1. **Self-poisoning closure** (§3.2) — artifacts re-entered as references; 504 vs 124 on identical input.
2. **Historical prose** (§9) — every scope, however small, was 100% blocked by closed-ticket text.
3. **Submodule traversal** — `plugin-registry` enumerates 40 232 inputs repo-wide, 5 157 recursing
   into `♻️mit-bestand/recherche`, so `assertTransactionRepositoryPath` refused every transaction.

### An engine gap found while applying

The plan's own `referenceEdits` did NOT cover relative `../..` arithmetic inside moved
`Cargo.toml` / `📋️project.json` / `📜️script.ts`. When `🏅️standards/🔖️1/🪆️subsets/✳️any` collapses to
`🪆️1-any`, directory depth drops by exactly 3 and every relative dependency path in a moved file is
wrong by 3 levels. It needed hand-correction here and **will recur on every profile-collapsing
projection** — worth fixing in the engine before the next one.

### Coordination lesson

An apply run failed with
`Taxonomy v7 discovery contract validation failed … fileKindResolutionRules["physical-model-3d-3dm"]`
purely because it raced a concurrent edit to `🔣️taxonomy.json`. **Never run `apply` while the SSOT is
being edited** — the transaction correctly rolled back, but the failure said nothing about the plan.

### Scaling picture

| scope | moves | unresolved |
|---|---:|---:|
| `🔢️number` | 2 | 0 → **applied** |
| `📐️geometry` | 3 | 0 (apply pending, blocked only by the SSOT race above) |
| `🔄️machine` | 5 | 52 |
| `🖥️platform` | 2 | 51 |
| `🧰️framework/🔨️modules` (all) | 953 | 2 631 |

`--scope` takes ONE path; a comma-separated list silently matches nothing (`moves=0 unresolved=0`),
so each scope must be planned separately.

At full-module scale the blockers are now dominated by the vocabulary gap — exactly the
"implementation-neutral folders" half of the goal:

```
semantic-stem-unresolved                     1495
reference-syntax-unsupported                  397
semantic-stem-ambiguous                       167
package-implementation-destination-unresolved 117
directory-kind-unresolved                     111
```

Every registration converts a blocked file into a neutral folder plus a kind-only leaf, so `moves`
RISING as these resolve is the goal being met, not a regression.

## 12. Applied so far, and the ~50-row floor

### Committed to disk by the engine

| scope | moves | note |
|---|---:|---|
| `🔢️number` | 2 | first ever |
| `🖍️draw` editor-command projection | 11 | unblocked everything else |
| `📐️geometry` | 3 | `⚙️engine/🦀️.rs`, `🎲️random/🦀️.rs`, `📦️packages/🦀️rust/🦀️.rs` |
| `🔢️hash` | 2 | directory renamed first, see below |

`#⃣hash` could not be planned at all — `directory-kind-unresolved`, because `#⃣` fails the taxonomy's
own emoji-validity gate (it begins with ASCII `#`). The registered kind is `{"emoji":"🔢️","slugPattern":"^hash$"}`,
so the directory itself was wrong, not the vocabulary. Renamed `#⃣hash` → `🔢️hash` by hand with all
17 references (16 `Cargo.toml` + `🔒️dependencies.json`), after which the engine planned and applied
both leaves cleanly.

### The floor that blocks everything else

Sweeping plan+apply across nine framework modules shows a constant floor unrelated to each module:

| module | moves | unresolved |
|---|---:|---:|
| `📐️geometry` | 3 | **0** → applied |
| `🔢️hash` | 2 | **0** → applied |
| `🔺️mesh-engine` | 1 | 2 |
| `🔺️mesh` | 1 | 50 |
| `🖥️platform` | 2 | 51 |
| `🧮️action-argument-resolution` | 1 | 51 |
| `🔄️machine` | 5 | 52 |
| `⏳️async` | 10 | 52 |
| `🎯️action-bus` | 5 | 84 |

`apply` refuses on any non-empty `unresolved`, so a module with ONE legitimate move is blocked by ~50
rows it does not own. `🔺️mesh-engine`'s two rows are both:

```
frozen-coordinate-evidence-unowned
  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json
```

i.e. ONE fixture file. ~80 of the 125 `frozen-coordinate-evidence-unowned` rows come from it, and a
two-coordinate wildcard patch for it is designed and tested (`🧪️frozen-coordinate-wildcard-coverage`,
5/5) awaiting application. **Clearing that one file is what turns a one-module-per-15-minutes crawl
into bulk applying.**

### `rust-path-join`, finally root-caused (fourth attempt)

After three wrong diagnoses (§8), instrumenting the guards inside `rustFiniteManifestTargets` found
it: `//!` doc comments — which CLAUDE.md *mandates* — trip a raw-text `/[#!]/` scan. Fixed with
`rustCodeOnlyText()` reusing the existing Rust tokenizer. The 155-row family's count did not move
because those same 16 files also call `plugin_exports!`, a separate and documented follow-up.

### Structural risk for bulk applies

The `plugin-registry` generator's input set spans the whole repository (40 232 paths), so ANY
concurrent edit anywhere invalidates a plan between plan and apply
(`Regeneration input preimage changed`). With a peer session and several workers editing
continuously, plans go stale within minutes. Mitigated by running plan and apply back-to-back with
retries (`📜️goal-planapply.sh`), but narrowing that generator's input scope is the real fix before
any large-scale apply.

## 13. Applied inventory (verified on disk) and the guard-chain lesson

### Committed by the engine, zero legacy stems remaining in these owners

| scope | moves |
|---|---:|
| `🔢️number` | 2 |
| `🖍️draw` editor-command projection | 11 |
| `📐️geometry` | 3 |
| `🔢️hash` | 2 (+ directory renamed `#⃣hash` → `🔢️hash`) |
| `🔺️mesh-engine` | 1 |
| `🧮️math` | 2 |
| `🔄️machine` | 5 |
| `🔺️mesh` | 1 |
| `🧮️action-argument-resolution` | 1 |
| **total** | **28** |

Verified: 17 kind-only leaves across the eight framework modules, and **zero** residual
`🦀️component.rs` / `📦️glue.rs` / `🟦️component.ts` in any of them.

### The guard chain — ten layers, one recurring mistake

Every blocker that stopped this programme turned out to be a guard checking a PROXY for the property
it claimed to enforce:

| # | guard | proxy it checked | property it meant |
|---|---|---|---|
| 1 | reference closure | "file is tracked/untracked" | "file is a live reference" |
| 2 | closed-ticket prose | "path token appears in text" | "reference must be rewritten" |
| 3 | generator inputs | "path is under the repo root" | "path is authored source" |
| 4 | frozen coordinates | one fixture's exact bytes | "evidence is pinned" |
| 5 | `rustFiniteManifestTargets` | raw-text `/[#!]/` scan | "text can contribute module structure" |
| 6 | trust scan | `#[cfg]` present | "module structure is conditional" |
| 7 | authority proof | glob import present | "file participates in module graph" |
| 8 | macro trust | macro invoked | "macro can emit `mod`" |
| 9 | `targets?.length !== 1` | one message for two states | "no targets" vs "many targets" |
| 10 | `inspectRustJoinArgumentSpans` | `.join("literal")` textually | "this is a filesystem path join" |

Three of these were misdiagnosed before being read properly (§8). The method that finally worked
every time: **instrument the actual guard and read the code path before proposing a fix.**

Two fixes were rejected on evidence rather than implemented — a worker refused my proposed
receiver-rooting discriminator for (10) as unsound (it would have silently stopped detecting genuine
non-manifest-rooted `Path::join` calls repo-wide) and found the real cause instead: `impl Trait for
Type` was being misparsed as a `for`-loop, aborting the remaining file scan. Another disproved its
own hypothesis about ticket-embedded packages by finding a live generator script with identical
shape. Those negative results were worth more than the rows they did not clear.

### Diagnostic defect worth fixing regardless

`🧹️normalization/🟦️.ts:4298` fires `targets?.length !== 1` identically for `undefined` (reference
never reached the finite map) and for a genuine multi-target. That conflation directly caused three
wrong diagnoses. Splitting it into two messages is cheap and prevents recurrence.

### Known-remaining, precisely located

- `🖼️assets` — **1 089 moves, 5 unresolved**. 4 are a generated wgpu bundle whose regeneration is
  gated by `WGPU generation requires canonical package inputs without old source leaves`
  (`📜️script.ts:820`) — the wgpu package projection must be applied first, and that scope
  (`📺️renderer`) carries 179 blockers of its own. The earlier "playwright-core break" diagnosis was
  wrong. 1 is a ticket python file correctly blocked by an embedded package (hypothesis tested and
  disproven — the blocking behaviour is right).
- `🕸️graph` — 1 row, deliberately unproven: a `Vec<String>::join` pushed through an opaque function
  call, which the locked `unknown-pushed-type` test requires to stay unproven.
- `🖱️ui` (809 moves / 563 rows) and `🎭️actor` (55 / 97) remain the large untouched scopes.
- `impl Trait for Type` misparse (fix landed) likely affected other files repo-wide — not quantified.
