# Development speed: clean long-term solutions, 2026-11-27

These recommendations come from about 30 hours of coordinating the norm end-to-end goal on one 16 GiB Mac with 6–9 parallel agents. Each one answers a failure that actually cost hours today. They are ordered by measured time lost.

## 1. Memory is the only real bottleneck, so make it elastic and honest

**Observed:**
- A 5-hour hold-and-wait deadlock with 0 rustc running.
- 5.3 GB reserved but idle.
- A 6 GB request starved for 4.4 h.
- An inner component scheduler waiting forever inside an exact grant.
- CPU load was 3–7 of 10 cores almost all day.

**Solutions:**
- **Measured per-unit peaks as the only sizing source.** Persist the observed peak of every compile unit and test target (keyed by crate, profile and target) in the ledger. Every `--memory` then comes from that record, not from hand-picked numbers like 3,400, 2,600 or 2,663. Agents should never choose a memory figure.
- **Grants that track observed use.** Admit at the forecast, lend back unused reservation after a grace period, and grow on demand through the new growth path. Today's over-reservation (11.9 GB admitted against 7.2 GB observed) halves throughput.
- **Deadlock and starvation handling,** now implemented (`capacity-hold-and-wait-2026-11-27.md`). Next: cargo-level preemption that requeues units rather than whole commands, and broker-supplied `floorMegabytes` so units can start reduced.
- **Fast refusal of the unsatisfiable,** now implemented for the outer gate and the component pass. Extend it to every nested scheduler: no wait loop without a bound and a "needs N" refusal.

## 2. Make the fleet carry the compile load, not only final links

**Observed:**
- Offload was about 30 units per 15 min.
- Most units were declined as "cheap" or "local-slot".
- 7 workers refused for lack of headroom, while g1-m05 had 35 GB free.
- The local Mac was memory-starved the whole time.

**Solutions:**
- **Invert the default when local memory is the bottleneck.** With local spare below the next unit's peak, offload instead of waiting for a local slot. "Cheap" should mean cheap in memory and time against the local queue wait, not CPU seconds alone.
- **Whole-command remote execution** for hermetic `cargo test` / `check` of a workspace: run it on the worker with the most headroom and stream the logs back. Nested-workspace suites (15–75 min each, serialized locally) are the ideal case.
- **Worker hygiene:** heal duplicate host names automatically (6 workers share one) and restart absent wardens. Otherwise the scheduler sees fewer machines than exist.

## 3. Host health as a supervised resource

**Observed:**
- `fseventsd` grew to 15 GB from worktree and `.cache` churn, until an admin restarted it.
- Disk ran down to 13–17 GiB free, and disk pressure cancelled builds.
- `cache gc` freed about 2 GiB, because everything counts as recently used.

**Solutions:**
- **Move high-churn build output out of the FSEvents-watched tree.** Put `.cache` on a separate APFS volume or in an excluded path, so the replica daemon and Spotlight don't watch it. That removes the root cause of `fseventsd` growth.
- **A host-health doctor** (`repo doctor` extension) that detects daemon bloat, low disk and zombie lease holders. It tells the operator the one command that needs admin rights, before agents start.
- **Age-plus-generation cache GC:** keep only the newest N generations per area. "Recently used" by a stale run shouldn't pin 54 GB images.

## 4. Long-lived readers must never block writers

**Observed:** a 20-hour-old dev MCP server held shared leases on component directories, which stalled publications for about an hour each until a deadline killed them.

**Solutions:**
- **Content-addressed generations with an atomic head pointer** for every artifact a long-lived process reads. Done for components (`component-publication-lease-2026-11-27.md`); apply the same pattern to every fixed-path artifact.
- **Lease age alarms:** a shared lease older than its declared bound shows up in `repo fleet status` with the holder's pid and command.

## 5. Coordination primitives for many agents in one shared worktree

**Observed:**
- One agent killed another's queued request believing it was its own orphan.
- Two agents launched duplicate nested-suite runs.
- Leftover `[DEBUG]` `eprintln!` lines from one session broke zero-allocation tests of another.
- Agents repeatedly needed help finding which process belongs to whom.

**Solutions:**
- **Owner-tagged processes:** `repo` stamps every spawned process tree with agent id, session and ticket, and `repo capacity status` shows them. Add a `repo kill --mine` that refuses foreign trees.
- **Work claims:** a ticket-scoped claim on "workspace X suite run" or "component Y publication". A second agent asking for the same claim attaches to the running one instead of duplicating it.
- **Debug instrumentation as a first-class, scoped feature:** a `semio-debug` facility enabled only by an env var or feature flag, compiled out by default, and with zero allocation when disabled. Plus a CI-style lint in `repo test` that fails on raw `[DEBUG]` lines older than their ticket. Today's `dbg_add` global mutex and `eprintln!` lines are exactly what this would have prevented.

## 6. Faster feedback loops for agents

**Observed:**
- A single crate test run took 25–75 min, mostly queueing.
- A whole-workspace test stopped at the first failing target and hid the rest.
- MCP output truncated the failures.

**Solutions:**
- **`repo test --affected --no-fail-fast` by default,** writing a structured JSONL of per-test results to the ticket's `🗑️generated/`. Agents then read structured failures, not truncated logs.
- **Test-target granularity in the graph:** run only the targets whose inputs changed, and reuse cached passing results across agents. Today every agent recompiled the same crates in separate lanes.
- **Shared build lanes per configuration:** compiled dependencies go into one CAS and are reused by every agent, instead of a fresh lane per invocation. "It compiles in a fresh lane" was the reason for 15–75 min per nested workspace.

## 7. Guardrails that keep the codebase compiling under concurrency

**Observed:**
- Breakage from concurrent refactors: typed enums replacing strings in tests, `open_folder` returning `Result`, `job-context` over its source boundary, a missing `serde_json` dev-dependency, `Cargo.lock` drift past the offline registry.
- Some of these broke unrelated crates for hours.

**Solutions:**
- **A continuous background `check` of the whole graph** (cheap `--lib --tests` check, memory-capped) on the archivist or a fleet worker. It publishes a red/green map and the first breaking edit, so agents find out within minutes, not hours later.
- **Lock policy:** forbid `Cargo.lock` versions absent from the offline registry, enforced by a fast `repo dependencies verify` before every cargo call. Ship a registry sync job that adds newly wanted versions.
- **Source-boundary early warning** at 85 % of the limit, so carves are planned rather than forced in an emergency.

## 8. Ticket and agent workflow

- **One coordinator dashboard file per ticket,** machine-written: lanes, owners, current step, last rc. Agents and the coordinator then stop re-deriving state from logs.
- **Bound agent context:** hand over to a fresh agent with a brief at about 500k tokens. Today's job-context agent degraded at about 940k, and the fresh one solved the remaining issues in about 2 h.
- **Agent briefs reference reports, not prose:** every lane writes its findings to a `.md` in the ticket. New agents start from that file, which proved effective today.

## Priority

The first three alone would likely have cut today's wall time by more than half:
1. Measured sizing plus elastic grants (§1).
2. The fleet carrying compile load (§2).
3. `.cache` off the watched tree (§3).

Next: owner-tagged processes and work claims (§5), and the continuous background check (§7).

## Addendum, 2026-11-28: lessons from the second day

9. **Schema-versioned, forward-compatible ledgers.** Twice today a live process wrote grant rows in a format that newer readers rejected, and admission stopped host-wide: first a new field, then stats moved to a sidecar mid-run. The rules:
   - Every on-disk record carries an explicit format version.
   - Writers bump it on any shape change.
   - Readers count unknown-version rows conservatively by their declared size, as the newest gate now does with "older format … size declared by the row", and never block admission. Unknown means "count it", not "stop".
   - Code that changes a ledger shape must be hot-swappable. The warden already hands over to a successor when its source changes; give the gate and its writers the same.
10. **Atomic framework changes need a global compile signal.** An API change such as a typed enum, a `Result` return or an owner-API signature broke about 10 nested workspaces, each found only after a 30–120 min run. A continuous, memory-capped `check --tests` of every workspace on the fleet, run per change, would surface all of them within minutes. A static prescan agent found 97 call sites in 5 minutes; that is the cheap version of this.
11. **Size sources limits before they bite.** Crates hit hard limits (`maxActiveLines` 4,000, unit memory 1,536 MB) only at build time, hours in. Lint them on edit, with early warnings at 85 %, so the carve is planned.
12. **Agent memory footprint.** Agent tooling (`bun` MCP and repo processes) held about 4 GB of the 16 GB host. That is the largest single "other application" consumer. Share one long-lived repo daemon per host instead of one per agent, and lazy-load its modules.
