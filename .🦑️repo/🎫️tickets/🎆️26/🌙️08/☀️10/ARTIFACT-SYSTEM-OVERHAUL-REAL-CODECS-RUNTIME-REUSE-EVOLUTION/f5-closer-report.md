# F5 Closer Report (C5) — xlsx / pptx / bcf / dwg (ac1018 + ac1024)

Role: C5 closer for the F5 fan-out wave (xlsx ecma-376, pptx ecma-376, bcf 2.1, dwg ac1018+ac1024
— 4 artifacts, 5 standards). **Last per-artifact fan-out wave.** Only agent this wave allowed to
touch `📦️glue.rs` and `📜️script.ts`.

## 1. Inputs read

`f5-xlsx-report.md`, `f5-pptx-report.md`, `f5-bcf-report.md`, `f5-dwg-report.md` (the 4 fan-out
reports) and `f5-verify-report.md` (independent verifier — re-derived every claim from
disk/`cargo test`, not from the self-reports). Cross-checked against `s2-spine-report.md`'s
ownership-boundary resolution and `w0-recon-report.md` §7/§8 for the defects originally flagged
against dwg specifically (ac1018 frozen-shim boundary). Also read every prior wave's closer report
(F1–F4) end to end in `STATUS.md` to pull the full per-standard ledger for the final summary table
below, and grepped every fan-out report repo-wide for "should be pruned"/"stale"/"left in place" to
catch anything missed by a prior closer (see §4).

## 2. `glue_followup` — reviewed, none actioned (no `glue.rs`/`script.ts` mount required)

All 4 fan-out reports flag deferred consolidation, none of which needs a `glue.rs` mount or a new
top-level directory (confirmed directly — S2's Task 1 resolution means all real work lands inside
already-mounted files, and none of the 4 reports request a new directory):

- **xlsx + pptx**: `XlsxOpcDiff`/`PptxOpcDiff` and their 4 sibling diff types each live in the
  artifact's own `🔺️diff/🦀️component.rs` rather than `zip/📦️opc/🦀️component.rs` — the same deferred
  hoist docx's F4 report already flagged (now a 3rd and 4th byte-identical-shape copy). Not
  actioned this wave, same reasoning as F4's closer: the types are written generically enough
  (`NamedTripleDiff<K,D,T>`-based) to lift verbatim once a session actually owns `zip::opc`; forcing
  that consolidation as a closer-scope side task on a live shared tree is out of this role's mandate.
- **bcf**: its own copy of the generic `NamedTripleDiff<K,D,T>` engine (now duplicated in docx, bcf,
  xlsx, and pptx — 4 independent copies) — same hoist target, same "not this wave" reasoning.
- **dwg**: ac1018's `⚙️engine`/`🧐️analyzer`/`🏗️builder`/`🎹️composer`/`🚪️io` subtree still imports the
  canonical (ac1024) types via the top-level `crate::artifacts::dwg::{...}` re-export rather than its
  own `standards::v_ac1018::subsets::any::*` types, while `schema::{snapshot,diff,mutations}` (this
  wave's own scope) was correctly repointed to ac1018's own types. **Decision made by this closer**:
  treat this as the intentional delegation dwg's own report speculates it may be (part (c) of its
  §6) — Decision #5 already establishes ac1018 as a deliberately frozen legacy shim with "no
  structural insight to carry" of its own; having its operational plumbing (build/mutate/analyze)
  delegate wholesale to ac1024's richer types while keeping only the schema-description layer
  (snapshot/diff/mutations — the actual scope of every F-wave) honestly ac1018-shaped is consistent
  with that decision, not a contradiction of it. No code change made. Flagged here, not re-flagged
  as an open defect, so a future targeted pass has this closer's own reasoning to start from instead
  of re-litigating from scratch.

**`glue_edits: []`** — confirmed via `git diff --stat` on
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`: still the same pre-existing 621-insertion diff
from earlier waves (S1/S2/F1–F4), untouched by any F5 fan-out agent or by this closer.

## 3. Full-crate gate

`cargo test -p semio-s-plugin-stdio --lib` (no filter, run fresh by this closer, twice — once
before and once after the `📜️script.ts` policy-allowlist edits in §4):

```
test result: ok. 1013 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.55s
```

Identical both times (as expected — the `📜️script.ts` edits are TypeScript-tooling-only, zero Rust
surface). Matches the independent F5 verify report's own number exactly, up from F4's 965/0 exit
state by the 48 net tests these 4 artifacts' own suites add (965 → 1013). Per-artifact filter, also
independently re-run by this closer: `artifacts::xlsx` → 41/0, `artifacts::pptx` → 48/0,
`artifacts::bcf` → 16/0, `artifacts::dwg` → 31/0 (both standards combined). **`full_crate_passed:
1013, full_crate_failed: 0`.**

## 4. Policy shrink — `bun run ./📜️script.ts policy`, scoped to F5's 4 artifacts, then a full 31-standard sweep

Ran `bun run ./📜️script.ts policy` (regenerates `.🦑️repo/⚡️cache/breaches/compose.json`) and
queried the regenerated cache directly via a small Node script (not the CLI's low-priority-filtered
stdout, which shows none of the 4 S-8 rules by default — same discipline every prior closer used).

**Scoped to xlsx/pptx/bcf/dwg, before pruning**: **10 breaches, every one `-stale-` (satisfied but
still allowlisted), zero real (missing)**:

- `POLICY_DIFF_ALGEBRA_ALLOWLIST`: 5 stale (pptx, bcf, xlsx, dwg/ac1018, dwg/ac1024 — every
  standard's diff type now really implements `DiffAlgebra`).
- `POLICY_FIELD_SWEEP_ALLOWLIST`: 5 stale (same 5 — every standard now has a real
  `field_sweep`-named passing test; bcf's own lives in `⚙️engine/🦀️component.rs` per its own
  report's documented organizational choice, still matched by the rule's repo-wide-per-standard
  regex scan).
- `POLICY_GRAMMAR_HONESTY_ALLOWLIST`: 0 breaches, real or stale, for any of the 4 artifacts — every
  fan-out report explicitly left grammar/facet leaves untouched this wave (deferred, same call every
  F1–F4 artifact's own report made), so these entries remain correctly allowlisted-and-still-real,
  not stale.
- `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST`: 0 breaches, real or stale, for any of the 4 — consistent
  with every prior wave's finding that this rule has known false-positive sources (F1's own
  root-cause investigation) and is left untouched by every closer to date.

**Pruned exactly the 10 confirmed-stale entries** — 5 from `POLICY_DIFF_ALGEBRA_ALLOWLIST`, 5 from
`POLICY_FIELD_SWEEP_ALLOWLIST` — from `📜️script.ts`. Both arrays are now **fully empty**
(`new Set<string>([])`), confirmed by direct read after editing.

**Full 31-standard sweep** (per the brief's explicit instruction, since this is the last per-artifact
fan-out wave): re-ran `bun run ./📜️script.ts policy` after pruning and queried the freshly
regenerated breach cache **repo-wide**, not just F5's 4 artifacts, for all 4 S-8 rules:

| Rule | Stale (repo-wide) | Real/missing (repo-wide) |
|---|---|---|
| `diff-algebra` | **0** | **0** |
| `field-sweep-presence` | **0** | **0** |
| `grammar-honesty` | **0** | 21 (all `ifc/2x3` — see below) |
| `facet-mirror-drift` | **0** | 3 (all `ifc/2x3` — see below) |

**`diff-algebra` and `field-sweep-presence` are now zero-breach, repo-wide, for both rules** — both
allowlists are empty `Set`s, meaning every one of this ticket's 31 standards has a real
`impl DiffAlgebra` and a real passing `field_sweep`-named test, with nothing left to allowlist.
This is the concrete confirmation that F5 was genuinely the last wave these two rules needed.

The remaining 24 non-stale breaches (21 grammar-honesty + 3 facet-mirror-drift) are **all
`ifc/🏅️standards/🔖️2x3`** (confirmed by direct inspection of every hit's `id`/`scope` field) — the
standard F4's own closer already flagged as a pre-existing, out-of-roster defect (added by the
separate, now-closed sibling ticket `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`, never assigned to
any F1–F5 agent, not part of this ticket's original 31-standard count). Untouched by F5, correctly
not allowlisted (these are genuine, non-stale breaches — `ifc/2x3`'s snapshot/diff/mutations really
do still directly import `step::engine::part21::Part21Document` verbatim, the exact defect standard
`4` fixed). Not this closer's scope to fix; flagged again here for the orchestrator, same as F4's
closer did.

**Swept every prior wave's own fan-out reports for unaddressed "should be pruned"/"stale"/"left in
place" mentions**, per the brief's instruction: `grep -rniE "should be pruned|left for a future
closer|left in place|not actioned|deliberately left"` across every `f*-report.md` (excluding
closers/verifiers) found the same single hit F4's closer already found and resolved
(`f3b-tiff-report.md`'s facet-mirror-drift claim, independently investigated and deliberately left
in place by `f3b-closer-report.md`) — no new unaddressed mentions from F5's own fan-out reports or
any earlier wave. `policy_shrink_confirmed: true`.

Full-crate `cargo test` re-run clean after the `📜️script.ts` edits (1013/0, unchanged — expected,
TypeScript-tooling-only allowlist edits with zero Rust surface — confirmed in §3).

## 5. `git check-ignore -v` on new directories

None of the 4 fan-out reports created new directories. The only untracked paths under
`🗿️artifacts/{📕️xlsx,🎞️pptx,💬️bcf,🖊️dwg}/**` are: pptx's and xlsx's own `✳️strict`/`✳️transitional`
subset dirs (pre-existing deliverables from the separate, now-closed sibling ticket
`ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`, confirmed untouched by any F5 agent — matches the
identical pattern F4's closer found for docx's own strict/transitional dirs), and one stray
`🔣️component.json` subset-registry scaffold file per standard (xlsx, pptx, bcf/2.1, dwg/ac1018,
dwg/ac1024 — the same pre-existing-scaffold pattern F2's/F3's/F3b's closers already found and
cleared for their own artifacts). Ran `git check-ignore -v` on all 9: every one matches only the
`.gitignore` line-179 **negation** rule (`!**/🔖️*/**`), i.e. explicitly un-ignored/trackable, not
actually gitignored (`git status --porcelain` independently confirms all show as plain `??`
untracked, not silently absent). No `.gitignore` action needed.

## 6. STATUS.md

Appended a new `## F5 (fan-out wave, xlsx/pptx/bcf/dwg) — closed` ownership-ledger section
(matching the style of every prior wave's closer entry) with the final all-31-standards summary
table pulled from this file's own accumulated F1–F5 ledger entries.

## Structured output

```
full_crate_passed: 1013
full_crate_failed: 0
glue_edits: []
policy_shrink_confirmed: true
all_31_standards_complete: true
report_path: .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f5-closer-report.md
```
