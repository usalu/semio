# Batch vs Interactive Dispatch — Lowpoly's 28 BatchOnlyPendingRewrite Commands

## Verdict: (c) Hard-dead in the running app

A live user triggering `extrude` (or any of the other 27 `BatchOnlyPendingRewrite` commands) gets a
`Fault` before any Lowpoly handler runs. The document is never mutated. There is no separate "batch"
runtime path in the shipped app that reaches `app_commands!`-generated dispatch for these ids — "batch"
in `LOWPOLY_BATCH_ONLY_TOOL_IDS` names an *offline/fixture* execution mode (tests, migration tooling),
not a live UI-reachable route. Confidence: high — the gate is a single hard-coded classification check
hit by every UI entry point, with no bypass found in Rust or TS.

## Evidence chain

1. **Both UI entry points require `Migrated`.** `dispatch_action` and `dispatch_command` — the only two
   ways a UI action/command reaches an app — each call
   `validate_ui_dispatch_classification(owner, id, definition.semantics.execution.interactive_job)`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:21750,21797`, plus the typed-command
   admission helpers at `:18906,18937`). The function itself:
   ```
   fn validate_ui_dispatch_classification(owner: &str, id: &str, classification: InteractiveJobClassification) -> Result<(), Fault> {
       if classification == InteractiveJobClassification::Migrated { Ok(()) }
       else { Err(Fault::new(..., "interactive-job.not-ui-safe", format!("UI dispatch rejected {owner}:{id} with interactive-job classification {classification:?}"))) }
   }
   ```
   (`🦀️component.rs:11806-11812`). There are exactly 4 call sites, all requiring `Migrated`; none allow
   `BatchOnlyPendingRewrite` through.

2. **The classification comes straight from the live app manifest**, not a separate doc. Lowpoly's own
   editor declares it per command: `.action_interactive_job("extrude", InteractiveJobClassification::BatchOnlyPendingRewrite)`
   (`✏️s/…/💠️lowpoly/…/✏️editor/🦀️component.rs:1759`, and 27 sibling lines 1757-1802). This is read back via
   `self.registry.get(verb)` inside `dispatch_action`/`dispatch_command`, so the same 28 ids that are
   BatchOnly in `🧪️interactive-job/🔣️component.json` are BatchOnly in the runtime manifest — confirmed
   identical partition (19 Migrated / 28 BatchOnly).

3. **Even if a request got past the classification gate**, `build_tool_job` (Lowpoly's factory for the
   app-owned/interactive-job path) explicitly declines batch-only ids: `let Some(disposition) =
   lowpoly_command_disposition(&request.tool_id) else { return Ok(None) }` (`✏️editor/🦀️component.rs:1438`),
   and `lowpoly_command_disposition` returns `None` for every id in `LOWPOLY_BATCH_ONLY_TOOL_IDS`
   (`:414-421`, asserted at `:1885`). The framework caller turns that `Ok(None)` into a hard error:
   `.ok_or_else(|| Fault::new(..., "interactive-job.missing-owned-builder", ...))`
   (`🦀️component.rs:22417-22420`) — a second, independent dead end.

4. **No client-side (TS/React/wgpu shell) awareness of the classification exists at all.** Grepping the
   framework's TS for `BatchOnlyPendingRewrite` / `not-ui-safe` returns nothing. The UI does not hide or
   grey out these 28 commands specially — it would simply dispatch and receive the Fault back from Rust.
   (The `🟦️interactive-job.ts` port and `interactive-job-registry.ts` found via the "interactive-job"
   grep are an unrelated subsystem — a diagram-layout worker-job scheduler, not the tool-command
   dispatch path.)

5. **`lowpoly_retained_reduce`'s `_ => Err("lowpoly-batch-only-command-entered-retained-reducer")`
   fallthrough is unreachable from the UI** given (1)-(3); it functions as a defense-in-depth assertion,
   not the live rejection point.

## Concurrent ticket: INTERACTIVE-JOB-RUNTIME-REFACTOR (2026/08/20) — OWNS this exact migration, high collision risk

- This ticket's `📓️sol-lowpoly-reactive-cohort-2026-08-27.md` and
  `📓️coordinator-lowpoly-retained-publication-scout-2026-08-27.md` are the **origin** of the current
  19/28 partition — this fleet moved Lowpoly from "47 BatchOnly" to "19 Migrated / 28 BatchOnly" and
  explicitly states: *"The four source-blocked rows are only this first cohort. Other Lowpoly BatchOnly
  mesh and paint commands... remain part of the all-app exit gate."* — i.e. it intends to keep migrating
  Lowpoly's remaining 28 rows.
- Root cause named for why they're stuck: Artifact/Config preparation factories do whole-result
  prepare/scan instead of paged/bounded work, and Transient publication lacks an exact
  preparation/root-retirement owner pair — matches the manifest's stated blocker ("Reducer lacks a
  bounded operation-owned cursor or exact Store publication authority").
- **Repo-wide, this ticket is very actively live right now**: its `📌️status.md` (2026-08-28, same day as
  today's session) logs dozens of fleet checkpoints across many plugins/subsystems, and its
  `PHASE-1-5-DE-ASYC-REPAIR-SWEEP` subfolder (note: misspelled "ASYC" not "ASYNC") has an in-progress
  git-staged file (`mesh-deasync-span-journal.json`) that did not resolve to an existing path on disk at
  read time — consistent with another session actively writing/renaming inside it at this moment.
- **Recommendation**: touching Lowpoly's retained reducer/preparation factories (`✏️editor/🦀️component.rs`)
  right now has a real chance of colliding with this fleet's in-flight work on the same 28-row cohort.
  Coordinate via the ticket rather than editing independently, or scope any change tightly and diff
  against the ticket's latest before committing.

## Per-plugin Migrated vs BatchOnlyPendingRewrite census

Counted directly from `.action_interactive_job("id", InteractiveJobClassification::X)` call sites in
every plugin's `🦀️component.rs` (both `InteractiveJobClassification::` and
`semio_framework_plugin::InteractiveJobClassification::` forms). Totals differ slightly (700 vs 708)
from the concurrent ticket's own live `nx run workspace:verify-interactivity --args=tool-jobs` census
(**350 admitted / 315 BatchOnly / 2 forbidden / 270 remaining(unclassified) of 771 unique rows**,
`📓️coordinator-current-interactivity-census-2026-08-28.md`) because that tool also counts
non-`action_interactive_job` registration forms (host-configuration routes, aliases, framework-owned
rows); the per-plugin table below is Rust-source ground truth for the `action_interactive_job` rows only.

| Plugin | Migrated | BatchOnly | Total | % Migrated |
|---|---:|---:|---:|---:|
| puzzle | 13 | 104 | 117 | 11% |
| space | 20 | 36 | 56 | 36% |
| procedural | 30 | 19 | 50 | 60% |
| **lowpoly** | **19** | **28** | **47** | **40%** |
| norm | 0 | 45 | 45 | 0% |
| cad | 24 | 16 | 40 | 60% |
| flow | 21 | 16 | 37 | 57% |
| fem | 5 | 32 | 37 | 14% |
| note | 9 | 27 | 36 | 25% |
| process | 11 | 22 | 33 | 33% |
| layout | 13 | 7 | 20 | 65% |
| writer | 18 | 0 | 18 | 100% |
| animate | 4 | 14 | 18 | 22% |
| sequence | 17 | 0 | 17 | 100% |
| gis | 17 | 0 | 17 | 100% |
| block | 7 | 9 | 16 | 44% |
| sourcing | 6 | 8 (+1 forbidden) | 15 | 40% |
| dag | 2 | 11 | 13 | 15% |
| imperative | 1 | 10 | 11 | 9% |
| reasoning | 2 | 8 | 10 | 20% |
| vcs | 10 | 0 | 10 | 100% |
| trinity | 8 | 1 | 9 | 89% |
| playbook | 2 | 7 | 9 | 22% |
| mathematical | 7 | 0 | 7 | 100% |
| draw | 6 | 0 | 6 | 100% |
| remodel | 2 | 0 | 2 | 100% |
| shooting | 2 | 0 | 2 | 100% |
| demonstrator | 1 | 0 | 1 | 100% |
| forms | 1 | 0 | 1 | 100% |
| **TOTAL** | **278** | **420 (+2 forbidden)** | **700** | **40%** |

**Lowpoly is exactly average** (40% migrated, matching the repo-wide average of 40%). It is not
conspicuously behind — several larger plugins (puzzle 11%, norm 0%, fem 14%, dag 15%) are further
behind; several small/simple plugins are at 100%.
