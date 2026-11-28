# Long-Term Solutions for Faster Project and Framework Development

## What This Goal Showed

Getting the GIS plugin working end to end has taken more than 30 hours of wall-clock time. Very little of that went into GIS code; the time went here:

| Where the time went | Share (estimate) |
|---|---|
| Graph planning: reads of 20 to 60 min, restarts, starvation, kills | ~45% |
| Host memory contention: warden kills, swap, fseventsd leak, parallel cargo | ~25% |
| Other agents' edits breaking shared tooling: import paths, renamed APIs, half-applied refactors | ~15% |
| Disk and image management: oversized images, EFAULT, emptied node_modules | ~10% |
| Actual GIS fixes | ~5% |

The leverage is therefore almost entirely in the inner loop: plan, build, test and verify. Domain code is a small share of the time.

## 1. Make Planning Incremental, Snapshot-Based and Cheap

- **One resident, always-warm project graph.** A watcher on the replica stream applies each change as an event (event-sourced, consistent with CQRS). Changes are applied incrementally, so no request ever reads 3,600 owners. A plan request then reads a sealed graph revision in milliseconds; `graph_policy=current` already points this way.
- **Snapshot isolation.** A plan binds to a graph revision. Changes after that revision go to the next plan; they never restart the current one.
- **Owner facts as content-addressed memo rows.** Key each row by its input hashes, so only owners whose inputs changed are re-verified, never whole rounds.
- **Static owner descriptions.** Script providers that spawn bun or git per owner should become schema-declared manifests, which are parsed and never executed. This is the schema-first rule applied to the build graph.
- **A planning SLO with a test:** a plan for any single project under 5 s on a 16 GB host. Run it as a benchmark (`bench_run`) in conformance, so regressions fail loudly.

## 2. Treat the Build Graph as Many Small Units

- **Smaller crates and packages with narrow dependency closures.** The Terrain editor split is one example, and the Gismap `tiles` unit (1 h 40 min, 2.2 GB) is the anti-pattern. Set per-unit ceilings for compile time and peak RSS, and enforce them in conformance; the existing carving ceilings are a start.
- **Test targets that do not pull the world.** `gis-js:test` should depend only on its own closure plus prebuilt artifacts.
- **Prebuilt, content-addressed artifacts from the fleet:** compiled crates, generated types and bundles. Agents should normally download and only compile what they changed. Make units portable by removing host-probing and process-spawning build scripts. Most units were declined as `not-portable`, and that is the single biggest fleet blocker.

## 3. Use the Fleet for More Than Execution

- **Offload planning and verification as well as compiles.** A warm planner node serves plans to thin clients.
- **Use host roles.** A 16 GB laptop should edit and run focused tests. Heavy cargo suites, full plans and browser acceptance should go to larger nodes by default, with a local fallback.
- **Sign everything with the machine identity.** This removes the stale-token class of failures, which is already fixed for offload; apply it everywhere.

## 4. Make Host Resources a Single Scheduler, Not Competing Lanes

- **One capacity broker owns CPU, memory and disk.** It replaces agent-side lock files such as `compile.lock*` and exclusive windows; those were hand-made workarounds in this ticket.
- **Admission by measured peak per unit,** using the history in the build cache. The goal is that over-commit never happens, rather than having the warden kill work afterwards.
- **Suspend and resume instead of cancel,** with progress kept, so no hour of compile is lost.
- **Host hygiene as a daemon.** Image gc, disk budget, zombie reaping and FSEvents stream accounting run continuously, not as manual quiet-window work.

## 5. Protect Shared Tooling From Concurrent Edits

About 8 breakages in this goal came from other agents' edits to the repo tool and the framework, in a worktree replicated live.

- **Pin the toolchain the agents run.** `repo` runs from a sealed, verified snapshot of itself, which is promoted only after its own tests pass. Edits to the repo tool must not break every agent's build within one second.
- **Turn on the existing sealed mode.** The `sealed` module already implements the pinned snapshot, but it is off (`defaultMode: live` in `🏛️sealed/🔣️.json`). Turning it on needs three things:
  - a promoter that seals a snapshot after the repo tool's own tests pass;
  - `SEMIO_REPO_SEAL=live` for agents that edit the tool;
  - a team decision, because it changes how every agent's edits go live (see service-generation-churn.md).
- **Use atomic multi-file changes.** A change set is applied all at once, or staged until its generators and tests pass. This prevents half-applied refactors, such as the bilingual-label change and the RetainedOrderedMap rename, from becoming visible to other agents.
- **Keep an ownership and intent registry.** Agents declare which area they are refactoring, and others see it before they touch it. `repo://goals` and the tickets could carry this.

## 6. Faster Feedback for Agents

- **Two-tier verification.**
  - Tier 1 runs in seconds: type check, focused unit tests and conformance for the changed closure.
  - Tier 2 is the full suite and browser acceptance, run by the fleet asynchronously, with results as events the agent subscribes to.
- **Structured, early failure output.** Every target reports a compact JSON verdict with the first failure. Today, agents must tail logs with thousands of repeated progress lines; collapse repeated progress lines in logs.
- **Clear signals for a hang versus progress.** Report throughput (items/s) and an ETA in progress messages; "164 of 3600" without a rate hides a stall. The warden and the stage runner use the same signal to tell slow from stuck.
- **Dry-run and explain tools.** `--dry-run` is now plan-only. Add `repo explain <target>` to show why a target rebuilds or which input changed.

## 7. Agent Workflow Conventions

- **Recipes as skills or MCP tools.** Offload environment, lane locks, flag order and retries as one tool call: today the offload recipe is folklore copied between agents.
- **Shared result caches per ticket.** A test verdict for an input hash is reused by every agent; no agent re-runs a suite that another just ran.
- **Standard lane scripts with cancellation and progress,** instead of ad-hoc bash loops. This also avoids self-matching `pgrep` patterns and bash's incremental script reads.
- **Small, verifiable tickets.** "End to end" goals should be cut into milestones that a single plan can verify.

## 8. Lessons From 2026-11-27 Evening

- **Per-target plans beat union plans.** One target planned in minutes, while the union of four targets needed 6.5 GB and more than 5 h and never finished. Agents should request single targets; the service should coalesce shared reads.
- **Every long-lived handshake needs renewal and a typed retry.** Tokens, generations and silence timeouts each ended multi-hour runs. Design all client-to-service paths so they resume, never fail as fatal on transient states, and always print a verdict with an exit code.
- **The implementation key should follow plan semantics, not bytes.** Hash the parsed module graph of plan code, so comment-only and test edits do not start generations.
- **Disk images need hard caps.** ENOSPC in the sealed wrapper image failed four generators at once. Every image must follow the ≤32 GiB invariant, with gc triggered by watermarks rather than by quiet windows.

## Recommended Order

1. A planning SLO with an incremental, snapshot-isolated resident graph (1, 4). This removes the dominant cost.
2. A pinned, self-tested repo toolchain plus atomic change sets (5). This removes most surprise breakages.
3. Portable build units and fleet artifacts by default (2, 3). Laptops stop compiling the world.
4. A single capacity broker with suspend and resume (4). The lock-file workarounds go away.
5. Two-tier feedback and structured verdicts (6, 7). Agents spend their time on code.
