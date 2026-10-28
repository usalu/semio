# Staged doc batches for the build guide (docs3-g)

Guide: `🧰️framework/🛍️products/🦑️repo/📓️build-guide.md`, base sha256 `38412f96781d2985b7d09a375ced44a5318be8cea76592a588294d45f79b2a4c`.

Each batch: `python3 -I staged/<name>/stage.py check|apply|revert [--root DIR]` (idempotent, anchored, refuses on drift with exit 2, on a missing code prerequisite with exit 3, writes nothing then). Land the doc batch WITH its code batch and revert both together.

| # | doc batch | land | code batch | edits | adds |
|---|---|---|---|---|---|
| 1 | `bench2` | any time (measurement only) | `none (measurement only, the host-total patch is already live)` | 2 | Replaces the 'Not met' status line with the first honest baseline table. Land any time. |
| 2 | `mirror` | any time (tool exists) | `none (ticket tooling mirror-tools/, exists)` | 2 | Names the shared mirror tool in the staging rule. Land any time. |
| 3 | `guard-go` | any time (guard-g edits are live) | `none (guard-g edits are live, outside the library)` | 4 | The Go test sandbox compares roots by identity and scratch leaves the checkout. Land any time. |
| 4 | `quietfc` | G1, right after the quietfc code batch (needs its text: exit 3 before) | `quietfc-g-tools (apply.sh)` | 8 | Needs the code batch applied first: its own guide edit adds the three bullets (quiet follow-up, envelopes, declared machine) with the word 'staged'; this batch turns that word into 'live', closes the 4b status, and adds the waiting row. |
| 5 | `journal-fix` | G2, with verify2 `journal-fix` | `verify2-g-tools/staged/journal-fix` | 4 | The change-journal service survives the live tree. Land before `vouch`. |
| 6 | `vouch` | G2, with verify2 `vouch` (after journal-fix) | `verify2-g-tools/staged/vouch` | 2 | The change journal narrows the dependency verification. Land after `journal-fix`. |
| 7 | `probes` | G2, with toolsetup `probes` (or `probes-after-conv`) | `toolsetup-g-tools/staged/probes (or probes-after-conv when conv-g landed first)` | 4 | The planner skip finally hits. The twin code batch `probes-after-conv` needs the same doc batch. |
| 8 | `plan-once` | G3, with graphlat `plan-once` | `graphlat-g-tools/staged/plan-once` | 2 | A scoped graph refresh plans once. |
| 9 | `dashlock` | G3, with dashlock | `dashlock-g-tools/staged/dashlock` | 4 | A running dashboard never fails or blocks a build; old executable digests are reclaimed when unheld. |
| 10 | `purge` | G3, with purge-g `purge` (P0, land early) | `purge-g-tools/staged/purge` | 4 | The one purge law of ticket output (P0 data-loss guard). Go and Rust (`repo ticket purge-artifacts`) are already live; the TypeScript `repo clean` lands with the batch. |
| 11 | `fleet` | G4, with obs-g `fleet` (same text as obs-g's apply_guide.py: either may land first) | `obs-g-tools/staged/fleet` | 6 | `repo fleet status` and the MCP tool `fleet_status`. Rows 1-4 are byte-identical to obs-g's own `apply_guide.py`, so either may land first. |
| 12 | `census2` | G6, with census2 (xor testmem2) | `census2-g-tools/staged (supersedes testmem2-g-tools/staged: land exactly one)` | 4 | Per-file memory ceilings for every owned `bun test` process. The code batch's own bullet is in the test-runner paragraph; this batch adds the message row and the budget-table row. |
| 13 | `fdleak-guard` | G6, with fdleak `guard` (after census2) | `fdleak-g-tools/staged/guard (after census2)` | 6 | The `bun test ./` rule, the open-files ceiling and the orphan guard. The code batch carries its own bullet in the test-runner paragraph; this batch adds the ENFILE and message rows and sharpens the single-file rule. |
| 14 | `jobsrv` | G6, with jobsrv | `jobsrv-g-tools/staged` | 4 | Cargo's job-token pool is handed on to build scripts and the compiler wrapper. |
| 15 | `nxfix-core` | G6 INSTEAD of the guide file of nxfix `core` (core replaces the whole guide and refuses once any other guide edit landed) | `nxfix-g-tools/staged/core (its manifest entry for the guide)` | 2 | nxfix-g's `core` replaces the WHOLE guide file (base sha256 must equal the live file) and refuses after any other guide edit. Land this batch instead and remove the guide entry from core's manifest, or land core before every other doc batch. The two blocks are byte-identical to core's, so either order ends in the same guide. |
| 16 | `nxlock` | G6, with nxlock (after nxfix core) | `nxlock-g-tools/staged/nxlock (after nxfix-g core)` | 4 | Nx daemon start order and the inert lease classes. |
| 17 | `oracle2` | G4, with oracle2 | `oracle2-g-tools/staged` | 2 | The read oracle proves huge keys, waits for the unified log, and uses no local time stamp. |
| 18 | `setup` | G4, with setup | `setup-g-tools/staged` | 2 | Zero-touch native prerequisites: preflight names them, `repo tools setup all` builds the Node native core and the compiler helpers. |
| 19 | `xplat3` | G4, with xplat3 (creates the Windows section) | `xplat3-g-tools/staged (apply.py)` | 2 | Windows notes and the platform fixes of the Windows audit. Creates the section 'Windows, Linux and macOS'. |
| 20 | `platforms` | G4, with xplat4 `platforms` | `xplat4-g-tools/staged/platforms` | 2 | Explicit platform verdicts for all 220 suites; Windows rows in CI block. |
| 21 | `freespace` | G4, with xplat4 `freespace` (after xplat3) | `xplat4-g-tools/staged/freespace (after xplat3)` | 2 | One free-space question that answers 'unknown'. |
| 22 | `conv` | G4, with conv (after oracle2) | `conv-g-tools/staged (after oracle2-g)` | 2 | Policy numbers in schema JSON, bilingual refusals, audit rules. |
