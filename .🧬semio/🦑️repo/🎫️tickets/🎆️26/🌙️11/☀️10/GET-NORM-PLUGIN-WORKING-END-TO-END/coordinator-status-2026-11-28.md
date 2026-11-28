# Coordinator status, 2026-11-28 01:35 CET

## Goal checklist

| Item | State |
|---|---|
| EN 1990 physical replay | **Green** (run 37). Follow-ups: 3 of the 4 plugin checks pass; sourcing-curation and the `test-support` cold-replay tests are queued. |
| EN 1995 | **Done** (peak 925 MiB, tests green). |
| 5 signed components | **None published on the content-addressed layout yet.** Eurocode published on the old layout and must be republished. The chain order is Energy, Eurocode, Structural, Materials, Base. Energy attempt 2 is running at 4,400 MB. |
| 15-route sweep | Runs after the 5 components. |
| Norm Rust suites | Hub norm package green (15). Plugin workspace: 4 targets fixed, awaiting their rerun. inspector-forms: root cause fixed, run in progress. panels-inspection is next. |
| Nested workspaces | Running one at a time. Compile errors fixed statically in en1995, din4108, en1998, en1996, en1994 (carve) and en1991/92/93/90 (owner API). None green yet. |

## Fixed in this round, all with tests

- **Hub norm:** `semio-hub-norm` carved into 8 app crates, each 1.24–1.38 GiB, under the 1,536 MB budget.
  - The final link uses the lean profile (no LTO).
  - The forecast learns from units cancelled at their ceiling.
  - 14 app manifests had missing dependencies.
- **Capacity:**
  - The scheduler resolves hold-and-wait deadlocks, refuses unreachable requests fast, and ages requests.
  - Growth requests run before new work.
  - The component pass refuses or grows instead of waiting forever inside an exact grant, and no longer double-counts the 4,096 MB floor.
- **Warden:**
  - It picks the right victim.
  - It resumes suspended work with hysteresis.
  - Overdue granted trees are resumed.
- **Broker:**
  - It admits units that fit the grant when nothing runs.
  - An oversize head no longer blocks smaller units.
  - SIGSTOPped units are not declared lost.
- **Components:** content-addressed generations, so a long-lived reader never blocks publication. Reader ctime was also excluded from the fingerprint.
- **Deadline:** the publication deadline is renewable (6 h maximum, supervised by idle progress).
- **Framework:**
  - The JSON native-input receive phase forwards `maximum_items`.
  - The test fixture restores the turn ceiling with `scoped_maximum`.
  - The job-receiving lifecycle and test callers.
  - `repo run` uses declared scopes.

## Blocking now

Every grant is memory-bound on one 16 GiB Mac. "Other applications" (agent tooling bun processes, Claude, git, compressed memory) hold 8.5–9.7 GB, so at most about 5.8 GB is grantable. Each publication step and each nested suite takes 30–120 min, run serially.

## Expected finish

Rough estimate, 8–14 h:
- **Components:** Energy, then 4 more at about 40–90 min each, then the sweep.
- **Nested suites:** reruns of about 15 workspaces at 30–120 min each run in parallel with the components, one at a time.

The estimate holds only if no further framework breakage arrives from concurrent sessions, which has happened about every 2 hours so far.
