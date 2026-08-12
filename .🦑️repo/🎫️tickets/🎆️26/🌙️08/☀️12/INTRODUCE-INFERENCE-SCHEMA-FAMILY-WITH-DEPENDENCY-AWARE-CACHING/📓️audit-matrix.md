# P0 audit matrix — the 72 existing `💡️inferences/` families

Coordinator-owned consolidation. Sources: `📓️p0-a1-stdio-norm-audit.md`, `📓️p0-a2-plugins-audit.md`,
`📓️p0-a3-wiring-audit.md`, `📓️p0-a4-laws-and-spine.md`, plus **coordinator-run direct verification**
(the authoritative column — see "Retracted findings" below).

## Headline: the earlier fan-out session's work is GOOD

All 72 families have, verified directly by the coordinator:
- **5/5 family-root leaves** (`🦀️`/`🟦️`/`🔗️`/`🔣️`/`🛰️component.*`) — 72/72
- **8/8 `📝️text/` leaves** — 71/72 (one gap, below)
- **6/6 `💾️binary/` leaves** — 72/72
- **exactly 1 slug dir**, each with both `🦀️component.rs` and `🟦️component.ts` — 72/72
- a real derivation in the slug leaf reading the artifact's own `XSnapshot` — 71/72

## Retracted findings (Haiku sub-audits were wrong — do NOT dispatch repairs from them)

Two of the four P0 explorer reports produced findings that do not survive verification. They are
recorded here so nobody re-derives them and mobilises 20+ pointless repair agents.

| Reported | Verdict | Why |
|---|---|---|
| "38 binary `📡️component.protocol.semio` files contain `foo` placeholder text" (A2) | **FALSE POSITIVE** | The grep matched the token **`footer`**, a real field in the protocol grammar. Control: the same grep flags 63 `📸️snapshot` protocol files, a family nobody claims is dishonest. Grammar honesty is clean. |
| "21 of 38 families / 34 of 34 families lack `impl InferredField<`" (A2, A1) | **NOT A GAP** | A pure-fn leaf (`XOutline::compute(&snapshot)` / `compute_x_topology(&snapshot)`) is an explicitly sanctioned shape — the approved plan names architect's `🧭topology` as the "whole-snapshot pure-fn leaf" exemplar alongside puzzle3d's `InferredField` merkle-chain exemplar, and the P3 script.ts policy is specified as "impl **/ pure-fn** presence". Verified real on `🖨️raster/🧭topology` and `📕️norm/📘️en1990/🧾outline`. |
| "each family has three slug dirs (`📝️text`, `💾️binary`, `🧾outline`)" (A1) | **CATEGORY ERROR** | `📝️text/` and `💾️binary/` are *representation* dirs, not slug dirs. Every family has exactly ONE slug dir. A1's 102 "instances" are 34 real slug dirs plus 68 misclassified representation dirs. |
| "families fully clean: 0 of 34" (A1) | **INVALID** | Follows entirely from the two errors above. |

Lesson for later waves: Haiku structural audits over this tree need a coordinator-run
cross-check before any repair is dispatched. Presence-greps in particular need control runs
against a family nobody disputes.

## Confirmed real gaps (the whole list)

1. **`🧩️puzzle/🖐️5d` `🎛flat-position/🦀️component.rs` is a 752-byte re-export shim** — 0 references
   to any `Snapshot` type, 0 compute fn, 0 tests. Its entire body is:
   `pub use …flatten::{FlattenPlane, FlattenPose};` + `pub use …flatten::{flatten_snapshot, flatten_snapshot_inplace};`
   It re-exports `flatten_snapshot_inplace` — **the exact anti-pattern P1 exists to delete**. This is
   the only family violating "no empty inference families". Fixed as part of the P1 5d slimming,
   not as separate repair work.
2. **`🏗️fem/◻2d` `📝️text/` is missing `🛰️component.proto`** (7 of 8 leaves). Its `🧊️3d` sibling has
   all 8 — that sibling is the template. Only structural leaf gap in the repo.
3. **`🏗️fem/◻2d` and `🏗️fem/🧊️3d` slug leaves (`📦bounds`) carry 0 `#[test]`** — the only two slug
   leaves in the 72 with no tests at all.
4. **`💠️lowpoly/💠️lowpoly` `📦bounds`** — has snapshot reads and 2 tests but no `fn compute`/
   `InferredField` under either sanctioned name; needs a read to confirm the entry point is honest.
5. **5 stdio test failures owned by this ticket** (csv/html/json/pdf `inference_default_law`, md
   `collects_headings_and_counts_words_and_blocks`) — diagnosis in `📓️p0-d1-stdio-failure-diagnosis.md`.

## ⚠️ THE REAL FINDING — only 8 of 72 families actually use the caching mechanism

Verified by the coordinator (`grep -rE "impl[^;]*InferredField"` per family):

**Families with a real `InferredField` impl (8):** `🌍️gis/🏔️gisterrain`, `🌍️gis/🗺️gismap`,
`💠️lowpoly`, `📐️cad`, `🧩️puzzle/🧊️3d`, `🧱️block/◻2d`, `🧱️block/🖐️5d`, `🧱️block/🧊️3d`.

**The other 64 use a pure-fn leaf** (`XOutline::compute(&snapshot)` / `compute_x_topology(&snapshot)`)
— a plain whole-snapshot fold with no `DepHash`, no `dep_input`, no cache participation, no
incrementality. They are honest derived-value facets. They are not dependency-aware caches.

This ticket's thesis is "`💡️inferences` **with real dependency-aware merkle caching**". On the
current disk state that thesis holds for 8 subsets out of 72.

**Two governing documents conflict on whether this is acceptable, and the conflict must be resolved
before P3 writes the policy cluster:**

| Source | Says |
|---|---|
| `📌️important.md` rule 13 | "every `💡️inferences/` ships ≥1 slug dir with a **real `InferredField`** (honest `dep_input` covering everything `compute` reads)" — pure-fn leaves are **breaches** |
| Approved plan, P3 §1 + Reuse §  | script.ts policy checks "**impl/pure-fn** presence"; names architect `🧭topology` (a pure-fn leaf) as a **sanctioned exemplar** alongside puzzle3d — pure-fn leaves are **legal** |

Coordinator's recommendation (for the parent/user to confirm, not to decide unilaterally):
**pure-fn leaves stay legal**, and the P3 policy demands `InferredField` only where the derivation is
per-entity and DAG-shaped (a parent/child relation in the snapshot). Rationale: a merkle dep-chain
over a 6-field flat compliance record like `📕️norm/📘️en1990` costs more than the fold it caches, and
forcing 64 artificial `InferredField` impls would be ceremony, not correctness. The counter-argument
— that a family with no cache is not an "inference" and the ticket under-delivers 64/72 — is real and
should be surfaced, not buried. **Escalated to the parent session; not actioned either way.**

## Law-test naming: the laws exist, under different names

`inference_cache_transparency_law` and `inference_incrementality_law` appear **zero times** in the
entire repo — including in the canonical puzzle3d pilot and the framework spine. A4 reported this as
"0 of 72, CRITICAL GAP". That framing is wrong: the *behaviour* is proven, under descriptive names.

- Framework `💡️inference` module: `disabled_cache_matches_pure_recompute`,
  `cold_and_warm_cache_match_pure_recompute`, `tiny_budget_eviction_storm_still_matches_pure_recompute`
  (= cache transparency); `changing_a_leaf_weight_only_recomputes_that_leaf_and_its_descendants`,
  `changing_the_root_weight_recomputes_the_entire_subtree`, `identical_snapshot_recompute_is_all_cache_hits`
  (= incrementality); plus schema-version salting and `DepHash` laws.
- puzzle3d `🎛flat-position`: `changing_a_leaf_own_vortex_does_not_recompute_ancestors`,
  `changing_the_root_position_recomputes_the_whole_chain`,
  `changing_an_attraction_center_param_never_touches_the_plane_chain`, `disabled_cache_matches_pure_recompute`.

All 72 families do carry `inference_determinism_law` + `inference_default_law` (134 files).

So the decision is a naming/policy one, not a missing-coverage one: either P3's policy checks for the
law *names* (⇒ rename the existing descriptive tests, ~10 files) or for the law *behaviours* (⇒ nothing
to do). Note that cache-transparency and incrementality are **only meaningful on the 8 `InferredField`
families** — demanding them on the 64 pure-fn families would produce 64 vacuous tests, which is exactly
the anti-pattern A4 flagged elsewhere.

## A4's "vacuous tests" finding — partially valid, worth acting on

A4 flagged 6 families (`✒️writer`, `🗄️stdio/svg`, `🗄️stdio/md`, `🔱️trinity/jack`, `🧱️block/3d`,
`🪵️sourcing/curate`) whose determinism law asserts `infer(default) == infer(default)` over an empty
snapshot — always true, proves nothing. This one is credible and matches the shape of `📕️norm/📘️en1990`'s
`outline_is_deterministic` (also over `::default()`). Not verified family-by-family by the coordinator
yet. Low-cost fix (build a non-empty fixture, assert real values); fold into P1's small-fixes lane.

## RULING (from the parent session) — the 8-of-72 question is CLOSED

**Pure-fn leaves are legal repo-wide. `InferredField` is required only where the derivation is
genuinely per-entity and DAG-shaped.** This was not a new decision but the resolution of a
contradiction between two governing documents: `📌️important.md` rule 13 overstated the requirement and
**has been corrected by the parent** (re-read it before writing P3 policy); the approved plan's fan-out
checklist already said "real `InferredField`/**pure-fn** leaf", and the original design doc explicitly
prescribes "🧭topology … or the closest honest derived stat" as a whole-snapshot pure function for
workflow/dag artifacts. Architect's topology leaf is that doc's own sanctioned shape.

Binding rationale, to be cited in the P3 policy cluster's doc comments: *a merkle dep-chain over a
6-field flat record costs more than the fold it caches.*

**P3 policy consequence:** slug-dir impl presence must accept EITHER `impl … InferredField<` OR a plain
`pub fn` reading the snapshot. Do not gate on `InferredField` universally.

**Law naming — also ruled:** check law BEHAVIOURS, not literal names. Do not rename the ~10 correctly
named descriptive tests. Do **not** manufacture `cache_transparency`/`incrementality` laws on the 64
pure-fn families — they have no cache, and a vacuous test is worse than no test. Pure-fn families owe
only `inference_determinism_law` + `inference_default_law` (both already present on all 72). Optional
polish: canonical-named alias tests on just the 8 `InferredField` families for policy grep-ability.

## A3 (wiring audit) — VERIFIED, and its headline is FALSE

| A3 claim | Verdict | Evidence |
|---|---|---|
| "55 of 72 families have NO glue mount — their code never compiles" | **FALSE** | Every one of the 33 plugins shows an exact **4:1** ratio of `💡️inferences` references in its `📦️glue.rs` to families on disk (writer 1→4, norm 15→60, stdio 19→76, block 3→12, …). The 4 are family-root + `📝️text` + `💾️binary` + slug submodule. |
| broken `#[path]` strings | **NONE** | All **288** `#[path = "…💡️inferences…"]` strings across every plugin glue.rs resolve to a real file on disk. Checked exhaustively, zero misses. |
| mount shape deviates from the mutations pattern | **FALSE** | Verified on `🖨️raster/📦️glue.rs:59-73` — `#[path="."] pub mod inferences { mod component; pub use component::*; pub mod text; pub mod binary; #[path="."] pub mod topology { mod component; pub use component::*; } }`. Exactly the mutations shape. |
| "TS index export VERDICT: REQUIRED, per `POLICY_TS_FACADE_CONSTITUTIONAL_FACETS`" | **MISREADING** | That constant (`📜️script.ts:2433`) is the set of facets whose `🟦️component.ts` **stubs are ACCEPTED without an allowlist** — stub *tolerance*, the opposite of an export requirement. It is consumed only by `policyIsConstitutionalTsFacadePath` (:3312), which tests the **parent dir of a `🟦️component.ts`**, and never looks at `📦️index.ts` at all. |

That makes **four of four** P0 explorer audits wrong on their headline finding. Every substantive P0
conclusion in this document is coordinator-verified; none rests on an explorer report.

## 🚨 NEW FINDING — the plugin TS facades are 91% dead, repo-wide

While settling the TS-export question:

```
repo-wide 📦️index.ts export paths:  517 MISS / 50 OK
```

Of 567 `export … from "…"` paths across the 33 plugin `📦️packages/🟦️typescript/📦️index.ts` files,
**517 point at files that do not exist.** `🖨️raster` is representative — all 12 of its exports are dead:

```
MISS ../../🗿️artifacts/🖨️raster/🧬️schema/📸️snapshot/🟦️component.ts
MISS ../../🗿️artifacts/🖨️raster/🧬️schema/🧬️mutations/🟦️component.ts
…
```

The cause is structural: they use **pre-standards paths** (`🗿️artifacts/<a>/🧬️schema/…`) while the real
tree has been migrated to `🗿️artifacts/<a>/🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/🧬️schema/…`. The approved
plan already knew of one instance ("repair stale pre-standards paths while there, mandatory for
🌀️procedural") — it is in fact near-total.

**There is no `📜️script.ts` policy on `📦️index.ts`** — the only two mentions in the whole file are an
unrelated import (:51) and a raw-spawn exemption (:3802). Nothing enforces these facades, which is
precisely why 517 dead paths went unnoticed.

### Verdict on W-C step 5: **CANCELLED — do not add inference exports to `📦️index.ts`**

Adding 3 inference exports per plugin would contribute ~99 more dead paths to a facade that is already
91% dead, enforced by nothing, and broken for reasons wholly outside this ticket. Repairing all 517 is
someone else's migration debt (the `🏅️standards`/`🪆️subsets` restructure — APA/UCAS territory), far
outside our scope, and would collide head-on with APA's live plugin migration.

**Action:** report the 517 to the peer sessions as an unowned repo-wide breakage; do not touch it.
Removes step 5 from the per-subset P2 checklist — a real scope reduction across every remaining subset.

## Not-yet-consolidated

`📓️p0-a3-wiring-audit.md` (glue mounts / registration / descriptor fns / TS-index-export verdict) and
`📓️p0-a4-laws-and-spine.md` (law-test matrix, framework spine, derive dual-copy diff) were still
running when this matrix was first written. Both need the same coordinator cross-check treatment
before any repair is dispatched from them — especially A3's `#[path]` resolution check, which is the
single highest-value item in the whole P0 wave and the one most likely to be right.
