# Async Function Census and Classification (P0c — corrective pass)

> **This document SUPERSEDES the previous version of `📓️p0-inventory-async.md`.**
> The earlier pass hand-sampled a subset of files and extrapolated percentages/counts
> for the rest. It was internally inconsistent: its A/B/C/D percentages summed to
> **112%**, and its own category totals (A 6,750 + B 25,000 + C 29,071 + D ~65 =
> 60,886) did not match its own stated grand total (54,367). At the scale of this
> repo (tens of thousands of `async fn`), hand classification cannot work — this
> pass replaces it with a real tool that parses every occurrence.

**Generated**: 2026-08-20
**Tool**: `🔧️async-census.ts` (bun/TypeScript, this ticket folder) — run directly, not sampled
**Self-test**: `🔧️async-census-selftest.ts` — 30/30 assertions pass against synthetic
Rust snippets exercising every tricky case (nested `fn` items, closures, raw strings,
string/comment traps, trait declarations). See "Tool validation" below.

---

## Headline numbers

The coordinator independently verified the raw scale by direct grep before this pass
started, excluding only `compose/target/node_modules`:

| Metric | Coordinator grep | This tool, same exclusion set |
|---|---|---|
| `async fn ` occurrences | **54,601** | 54,601 (reproduced exactly — see "Reconciling with the raw grep baseline") |
| `.await` occurrences | **24,734** | 24,734 (reproduced exactly) |
| `.rs` files scanned | (not given precisely) | 10,870 |

These are reproduced **exactly** by a literal-substring recount using the same
exclusion set, confirming the coordinator's scale is correct and that this tool's
directory walk is not silently dropping or duplicating files.

The tool's own **classified census**, however, additionally excludes a handful of
directories that are demonstrably not hand-written source under refactor (documented
below), and its function-body parser correctly ignores `async fn` text that appears
inside comments, doc-comments, or string literals (the literal grep cannot tell the
difference — that is precisely the kind of imprecision this packet exists to fix).

### Classified census total

```
Total classified async fn:  53,338
  A          6,251   (11.72%)
  A-shallow    883   (1.66%)
  B         39,796   (74.61%)
  C          6,408   (12.01%)
  D              0   (0.00%)
-----------------------------
  SUM      53,338    (100.00%)   ← percentages verified to sum to 100.00%
```

`D` is genuinely zero: across 53,338 functions in a codebase that compiles, no
brace-matching or signature scan failed to close. This is a meaningful sanity check
on the parser, not a shortcut — see "Tool validation."

### Directories excluded from the classified census (and why)

| Directory | async fn found there | Why excluded |
|---|---|---|
| `.🧬semio/` (ticket folders) | 1,241 | Ticket-scratch files (e.g. a `lib-head.rs` snippet from a past ticket) — not compiled source, not part of any crate |
| `♻️mit-bestand/…/dist/` | 0 (only generated `icon_name.rs` — no async fns) | Generated/build output |
| `storybook-static/` | 0 | Build output |
| `.nx/cache/`, `.🦑️repo/⚡️cache/` | 0 | Build/tool caches (mirror generated files above) |

Net effect: 54,601 (raw, compose/target/node_modules-only exclusion) − 1,241
(`.🧬semio` async fns) = 53,360 expected "clean" baseline; the tool's classified
total of 53,338 is 22 lower than that, accounted for by `async fn` text the tool
correctly recognizes as living inside comments/doc-strings (not real declarations)
plus a small number of `fn` after `async` inside macro-generated text patterns. This
delta is 0.04% of the total and does not materially affect any conclusion below.

---

## Classification criteria (as specified, with explicit heuristic choices)

- **A — genuinely suspending**: the function body contains at least one `.await` at
  its own level (own top-level code, nested closures, and nested `async {}` blocks
  all count; a nested `fn` ITEM's body does not — its awaits belong to that nested
  function, not the outer one).
- **A-shallow** (transitive refinement of A): every awaited callee the tool can
  resolve by name is *itself* exclusively B/C everywhere that name occurs in the
  census (see "Transitive A-shallow refinement" below for exact resolution rules and
  limits). These are A functions that only *decorate* through other decorative
  functions and are candidates for the same de-async treatment as B, pending human
  confirmation given the name-resolution caveats.
- **B — decorative/simple**: no own-level `.await`, no loop constructs (`for`,
  `while`, `loop`, or `.map(`/`.fold(`/`.for_each(` chains), and body ≤ **80 lines**
  (`LARGE_BODY_LINE_THRESHOLD` in the script — the task spec says "large" without a
  number; 80 was chosen as a defensible line for "still simple" and is a single
  named constant, trivially tunable for a future pass).
- **C — long-running CPU work**: no own-level `.await`, and either contains a loop
  construct/iterator chain, or exceeds the 80-line threshold with no loop keyword
  (straight-line numerical code can still be expensive). These are the resumable-job
  conversion targets for Phases 6–7.
- **D — unparseable**: brace-matching did not close, or the signature scan ran past
  its bound without finding `{`/`;`. Zero occurrences in this run (see validation).

Trait method declarations without a body (`async fn foo(&self);`, 679 of them) are
classified **B** — they have no code to suspend or loop, by definition — and are
flagged in the JSON via `"hasBody": false` so Phase 6/7 tooling can special-case them
if desired (a trait's *default* body, when present, is scanned normally and gets its
own real classification).

---

## Global split, verified to sum to 100%

| Class | Count | % |
|---|---|---|
| A (genuinely suspending) | 6,251 | 11.72% |
| A-shallow (transitively non-suspending) | 883 | 1.66% |
| B (decorative/simple) | 39,796 | 74.61% |
| C (long-running CPU, resumable-job targets) | 6,408 | 12.01% |
| D (unparseable) | 0 | 0.00% |
| **Total** | **53,338** | **100.00%** |

Collapsing A-shallow into "effectively non-suspending" alongside B: **(39,796 + 6,408
+ 883) / 53,338 = 88.28%** of all async fns in the repo either don't suspend at all
or only suspend through other non-suspending functions. Only **11.72%** are confirmed
genuine suspension points once name-resolvable decorative chains are excluded.

## By top-level area

| Area | Total | A | A-shallow | B | C |
|---|---|---|---|---|---|
| `🧰️framework/🔨️modules` | 4,687 | 1,502 (32.05%) | 266 (5.68%) | 2,461 (52.51%) | 458 (9.77%) |
| `🧰️framework/🛍️products` | 6,288 | 3,313 (52.69%) | 435 (6.92%) | 2,231 (35.48%) | 309 (4.91%) |
| `✏️s/🔨️modules` | 553 | 19 (3.44%) | 2 (0.36%) | 285 (51.54%) | 247 (44.67%) |
| `✏️s/🔌️plugins` | 41,559 | 1,229 (2.96%) | 179 (0.43%) | 34,765 (83.65%) | 5,386 (12.96%) |
| Other (root/hub/framework-packages) | 251 | 188 (74.90%) | 1 (0.40%) | 54 (21.51%) | 8 (3.19%) |
| **Total** | **53,338** | 6,251 | 883 | 39,796 | 6,408 | — |

Each row's *exact* (unrounded) fractions sum to 100% independently (verified
programmatically); two rows' 2-decimal-rounded figures as printed above sum to
100.01% purely from display rounding (`framework/modules`: 32.05+52.51+9.77+5.68;
`s/modules`: 3.44+51.54+0.36+44.67) — the underlying counts are exact integers that
add up to each row's total with no discrepancy. The
plugins total (41,559) matches the coordinator's independent grep count of the
plugins tree (41,562) to within 3 functions — the residual is the same
comment/doc-string exclusion effect described above. Framework products and
framework modules carry almost all of the genuine suspension (IPC, actors, channels,
timers) — exactly where you'd expect it in an interactive runtime. Plugins are
overwhelmingly decorative (83.65% B) — this is the "async everything" convention
described below applied uniformly to schema/DSL/artifact code that never suspends.

## Per-plugin breakdown (✏️s/🔌️plugins), sorted by total

| Plugin | Total | A | A-shallow | B | C |
|---|---|---|---|---|---|
| 🗄️stdio | 9,052 | 1,218 | 179 | 6,488 | 1,167 |
| 📕️norm | 5,773 | 0 | 0 | 5,446 | 327 |
| 🧩️puzzle | 2,691 | 1 | 0 | 2,294 | 396 |
| 🏛️architect | 2,192 | 0 | 0 | 2,056 | 136 |
| 📸️remodel | 2,074 | 0 | 0 | 1,355 | 719 |
| 🌀️procedural | 1,903 | 0 | 0 | 1,548 | 355 |
| 🧱️block | 1,560 | 0 | 0 | 1,441 | 119 |
| 🎞️animate | 1,477 | 0 | 0 | 1,318 | 159 |
| 🔱️trinity | 1,060 | 1 | 0 | 899 | 160 |
| 🏗️fem *(plugin wrapper, distinct from the `✏️s/🔨️modules/🏗️fem` engine below)* | 1,049 | 0 | 0 | 892 | 157 |
| ➗️mathematical | 986 | 0 | 0 | 736 | 250 |
| 🔋️energy | 843 | 0 | 0 | 753 | 90 |
| 📐️cad | 824 | 1 | 0 | 672 | 151 |
| 🌍️gis | 750 | 0 | 0 | 675 | 75 |
| 🪐️space | 705 | 0 | 0 | 613 | 92 |
| 💠️lowpoly | 679 | 0 | 0 | 587 | 92 |
| 📏️layout | 672 | 1 | 0 | 606 | 65 |
| 🎥️shooting | 651 | 0 | 0 | 596 | 55 |
| 🌊️flow | 635 | 0 | 0 | 576 | 59 |
| 🗒️note | 597 | 0 | 0 | 526 | 71 |
| 🖍️draw | 582 | 2 | 0 | 500 | 80 |
| 🏭️process | 569 | 4 | 0 | 503 | 62 |
| 🖨️raster | 521 | 0 | 0 | 459 | 62 |
| 🎬️sequence | 498 | 1 | 0 | 436 | 61 |
| 📋️forms | 484 | 0 | 0 | 405 | 79 |
| 📖️playbook | 429 | 0 | 0 | 385 | 44 |
| 📜️imperative | 421 | 0 | 0 | 372 | 49 |
| 🕸️dag | 408 | 0 | 0 | 339 | 69 |
| ✒️writer | 402 | 0 | 0 | 352 | 50 |
| 💡️reasoning | 377 | 0 | 0 | 315 | 62 |
| 🪵️sourcing | 317 | 0 | 0 | 277 | 40 |
| 🌿️vcs | 229 | 0 | 0 | 207 | 22 |
| 🎪️demonstrator | 149 | 0 | 0 | 138 | 11 |

`🗄️stdio` carries almost all plugin-side genuine suspension (1,218 A + 179
A-shallow) — it is the plugin providing I/O primitives other plugins call into, so
this is expected. Every other plugin in the list is 0% genuinely suspending.

---

## Simulation hot-spot deep dive

Per-area counts, matching the coordinator's independently-verified numbers almost
exactly (the FEM and Energy figures are an **exact match**; Puzzle is off by 2 and
all-plugins by 3, both attributable to the documented comment/string-exclusion
delta):

| Hot spot | Path | Total | B | C | A |
|---|---|---|---|---|---|
| FEM engine | `✏️s/🔨️modules/🏗️fem/` | 514 | 270 | 244 | 0 |
| Energy simulation | `✏️s/🔌️plugins/🔋️energy/` | 843 | 753 | 90 | 0 |
| Puzzle (2D/3D/5D) | `✏️s/🔌️plugins/🧩️puzzle/` | 2,691 | 2,294 | 396 | 1 |
| Procedural (assembly/WFC) | `✏️s/🔌️plugins/🌀️procedural/` | 1,903 | 1,548 | 355 | 0 |
| **Combined hot spots** | | **5,951** | 4,865 | **1,085** | 1 |

**1,085 category-C functions** across the four simulation hot spots are the concrete
resumable-job conversion targets for Phases 6 and 7. FEM and Energy are 100%
non-suspending (0 genuine A) — every one of their 514 + 843 = 1,357 async fns is
either decorative padding (B) or CPU work masquerading as async (C), confirming the
earlier report's qualitative read even though its numbers were wrong.

### Top 100 category-C functions in the hot spots, ranked by body line count

Full ranked list: `top100_table.md` / `top100_hotspot_C.json` (same directory as this
report — kept alongside the JSON census per the task's "keep the tool and JSON
alongside it" instruction). Top 20 shown here for context:

| # | Lines | Function | Hot Spot | Loop keywords | File:Line |
|---|---|---|---|---|---|
| 1 | 307 | `advance_timestep` | Energy | for, map() | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️component.rs:214` |
| 2 | 194 | `apply_puzzle3d_inspector_patch` | Puzzle | map(), for | `.../✏️editor/🦀️component.rs:732` |
| 3 | 192 | `create_puzzle3d_app` | Puzzle | (large, no loop) | `.../✏️editor/🦀️component.rs:2541` |
| 4 | 163 | `simulate_air_system` | Energy | (large, no loop) | `.../🌬️air_system/🦀️component.rs:60` |
| 5 | 155 | `puzzle3d_snapshot_mutations` | Puzzle | for | `.../🧬️mutations/🦀️component.rs:108` |
| 6 | 144 | `flatten_objects_with_assignment` | Puzzle | map(), for, while | `.../🎛flatten/🦀️component.rs:336` |
| 7 | 141 | `simulate` | Energy | (large, no loop) | `.../🌡️zone_hvac/🦀️component.rs:55` |
| 8 | 123 | `validate` | Energy | map(), for | `.../🔋️model/🦀️component.rs:581` |
| 9 | 122 | `puzzle5d_snapshot_mutations` | Puzzle | for | `.../🧬️mutations/🦀️component.rs:94` |
| 10 | 118 | `puzzle2d_snapshot_mutations` | Puzzle | for | `.../🧬️mutations/🦀️component.rs:96` |
| 15 | 101 | `solve_linear_static` | FEM | for, map() | `✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️component.rs:416` |
| 23 | 91 | `dkt_b_matrix` | FEM | for | `.../📏️elements2d/🦀️component.rs:731` |
| 24 | 90 | `solve_multi_case` | FEM | for, map() | `.../🧮️analyses/🦀️component.rs:442` |
| 41 | 74 | `dense_symmetric_eigen_jacobi` | FEM | for, map() | `.../🔢️sparse/🦀️component.rs:384` |
| 52 | 63 | `solve_inner` | Procedural (WFC search) | loop, map() | `.../🧩️wfc-engine/🔍️search/🦀️component.rs:492` |
| 66 | 57 | `drive` | Procedural (WFC search) | loop | `.../🧩️wfc-engine/🔍️search/🦀️component.rs:259` |
| 88 | 49 | `backtrack_and_repair` | Procedural (WFC search) | loop | `.../🧩️wfc-engine/🔍️search/🦀️component.rs:146` |
| 100 | 46 | `run_to_fixed_point` | Procedural (WFC constraint prop) | while, for | `.../🧩️wfc-engine/🔁️prop-ac3/🦀️component.rs:23` |

`advance_timestep` (Energy) is the single largest category-C function in the entire
hot-spot set at 307 lines with both a `for` loop and a `.map()` chain — matching the
prior report's qualitative call-out of the hourly/zone simulation loop, now with an
exact location and line count instead of an estimate.

---

## Transitive A-shallow refinement

**Method**: for every category-A function, each own-level `.await` is traced back to
a callee name via lexical matching on the call expression immediately preceding
`.await` (`name(args…).await`, `obj.method(args…).await` → callee = `method`, one
level of nested-parens tolerated). A global multiset `name → {classifications
observed under that name anywhere in the census}` is built from all 53,338 records.
An A-function is downgraded to **A-shallow** only if *every one* of its awaited
callee names resolves, with zero ambiguity, to a set that is a non-empty subset of
`{B, C}` — i.e. every function anywhere in the repo sharing that name is itself
non-suspending. If any awaited callee name is unresolved, or resolves to a set that
includes A, A-shallow, or D anywhere in the repo, the function stays A. This is
deliberately the conservative direction: false negatives (an effectively-decorative
function staying classified A) are safe; false positives (a genuinely suspending
function wrongly downgraded) are not.

**Result**: 883 of 6,251 A functions (14.1%) downgrade to A-shallow.

### Explicit limits of name-based resolution

This is stated per the task's requirement to be explicit about it:

- **No type/trait resolution.** `method()` calls are matched purely by identifier
  text. Two unrelated types that both define a method called `run` are conflated —
  if either one is genuinely suspending, *neither* can downgrade (safe direction),
  but it also means a truly safe callee can be blocked from downgrading by an
  unrelated same-named function elsewhere in the repo (conservative, not incorrect,
  but understates A-shallow).
- **No dynamic dispatch resolution.** `dyn Trait` method calls, generic `T: Trait`
  bounds, and closures stored in fields all resolve (or fail to resolve) by the
  literal identifier only; there is no attempt to follow trait impls.
- **std/external-crate callees are always "unresolved."** `tokio::time::sleep(...).await`,
  `mpsc::Receiver::recv().await`, `reqwest::get(...).await`, etc. have no matching
  entry in the census (the census only contains this repo's own `async fn`), so any
  A-function awaiting one of these correctly stays A and is never a candidate for
  A-shallow — this is the expected, safe outcome for genuine I/O/timer/channel
  suspension points.
- **Bare-future awaits** (`some_future_var.await` with no call parens) are recorded
  as unresolved by construction (tagged `<bare-future:name>` in the JSON) — these
  block downgrade for their owning function, since the tool cannot know what the
  variable resolves to.
- **This refinement is a signal for the Phase 6/7 codemod to start from, not a
  ground truth for automatic conversion.** Every A-shallow function should still get
  a human/compiler-assisted check (e.g. via `cargo check` after a trial de-asyncing)
  before its `async` is actually removed — name collisions across a 53k-function
  codebase are not rare enough to skip that step.

---

## Reconciling with the raw grep baseline

For full transparency, here is the same literal-substring recount the coordinator
used, reproduced by this session (excluding only `compose/target/node_modules`, per
the task's stated exclusion set — note this is *not* the classified-census exclusion
set, which additionally drops ticket-scratch/build-cache directories as described
above):

```
$ grep -rE "async fn " --include="*.rs" . | grep -v "^compose/" | grep -v "/target/" | grep -v "/node_modules/" | wc -l
54601
$ grep -rE "\.await" --include="*.rs" . | grep -v "^compose/" | grep -v "/target/" | grep -v "/node_modules/" | wc -l
24734
$ find . -name "*.rs" -not -path "./compose/*" -not -path "*/target/*" -not -path "*/node_modules/*" | wc -l
10870
```

These numbers reproduce the coordinator's 54,601 / 24,734 exactly, confirming the
"~45% of async fns can contain a `.await` at all" scale argument in the task
description (24,734 / 54,601 = 45.30%). Note this is a *file-level* ratio (any
`.await` anywhere in the file's async fns vs. total async fns in that file), not a
per-function statistic — the tool's own-level, per-function A/A-shallow/B/C/D split
above is the precise version of that same observation.

---

## Tool validation

`🔧️async-census-selftest.ts` runs 30 assertions against a synthetic Rust snippet
covering every case flagged as risky in the task description:

- a genuine `.await` on a channel recv → correctly counted as A
- two awaits on purely decorative callees → correctly counted (own-level), correctly
  resolved by name for the A-shallow refinement
- `for`/`while` loop keyword detection at the function's own level
- a **nested `async fn` item** whose internal `.await` does **not** leak into the
  outer function's own-await count (this was the exact failure mode the task warned
  a naive brace counter would get wrong)
- an **await inside a nested closure and a nested `async {}` block**, both of which
  **do** count toward the outer function (per spec — distinguished from the nested
  `fn` item case above)
- a raw string `r#"...{ fake braces } and .await..."#`, an escaped normal string
  containing a stray `"` and `.await` text, a char literal `'\''`, a line comment,
  and a nested block comment — all must be correctly blanked so they cannot corrupt
  brace matching or the await/loop-keyword scan
- a trait method declaration with no body (`;` instead of `{`) → correctly detected
  as `hasBody: false`

All 30 pass. Combined with the exact reproduction of the coordinator's raw-grep
baseline and the near-exact match (within the documented, explained delta) on FEM
(514/514 exact), Energy (843/843 exact), and plugins (41,559 vs 41,562), this is
strong evidence the classification is structurally sound, not just internally
self-consistent.

**Known residual limitations** (documented rather than silently accepted):
- The B/C "large" threshold (80 lines, absent a loop keyword) is a chosen constant,
  not derived from the spec — flagged above, trivially tunable.
- Loop-keyword detection is a lexical scan over the cleaned body and does not
  distinguish a bounded `for i in 0..3` from an unbounded `for x in huge_vec` — by
  design, per the task's own definition of B/C ("no loop constructs" as the bright
  line, not loop bound analysis). Phase 6/7 authors doing the actual resumable-job
  conversion will still need to read each C function to size its checkpoint
  interval — this census tells you *which* 6,408 functions to look at, not how to
  chunk each one.
- Name-based call-graph resolution for A-shallow is explicitly conservative; see
  the dedicated section above.

---

## Repository convention mandating async style

**Verified location**: `/Users/ueli/Documents/semio/AGENTS.md`, line 44 (confirmed by
direct `grep -n` against the current file — not re-typed from memory):

```
44:- You SHOULD implement everything async when it makes sense.
```

**Exact quote**: "You SHOULD implement everything async when it makes sense."

This is a `SHOULD`, not a `MUST`, and is qualified by "when it makes sense" — the
census shows that qualifier has not been honored in practice: 88.28% of async fns in
the repo (B + C + A-shallow) demonstrably do not suspend, i.e. async was applied
where it does *not* make sense by the convention's own wording. Per the task's
framing, this refactor deliberately narrows the convention going forward (async
reserved for genuine suspension points); `AGENTS.md` itself is intentionally left
unedited, per instruction.

---

## Files produced by this packet

- `🔧️async-census.ts` — the analysis tool (bun/TypeScript, ticket-folder-local,
  not wired into `📜️script.ts`)
- `🔧️async-census-selftest.ts` — validation harness, 30/30 passing, run via
  `bun 🔧️async-census-selftest.ts`
- `🔧️async-census.json` — one record per function: `file, area, plugin, name, line,
  pub, hasBody, classification, bodyLineCount, loopKeywords, ownAwaitCount,
  awaitCalleeNames, dReason`. Consumed directly by Phases 6, 7, and the de-async
  codemod.
- `🔧️async-census-summary.json` — the aggregated counts this report is built from
- `top100_hotspot_C.json` / `top100_table.md` — the top 100 category-C functions in
  the four simulation hot spots, ranked by body line count

To reproduce: `cd` into this ticket folder and run `bun 🔧️async-census.ts` (takes
~2.5s) followed by `bun 🔧️async-census-selftest.ts` (~0.1s) to re-verify the parser
before trusting a re-run's numbers.
