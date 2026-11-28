# 📈️ Status (2026-11-27 11:40) and Long-Term Development-Speed Proposals

## Progress

| Area | State |
| --- | --- |
| Boot, discovery, context, search, conversation, workspace resources, prompts, jobs, typed refusals | ✅ native binary (see [📋️e2e-matrix.md](📋️e2e-matrix.md)) |
| Framework capability routing (`framework.*` via `target.artifactRef`) | ✅ source, exercised once note create works |
| `artifact_create s.note.note` | 🔧 wire grant fixed, interpreter at `opt-level = 3`; agent re-probing a rebuilt binary and profiling remaining hot spots |
| 73 plugins skipped (`label` is now a terminology matrix, descriptors still plain strings) | 🔧 `describe --all` rerunning with the two missing generator dependencies now declared |
| inference payload contracts (gis, cad, glTF) | ⏳ source done, appears with the regenerated descriptors |
| `semio-os-mcp-workspace` long tests | ⏳ dependency fixed, waiting for a quiet host to compile |
| Shell bridge UI, official publish build, full 36-step flow | ⏳ after the two items above |

## Fixed this session (beyond earlier entries in the matrix)

- `🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs`, `🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs`, `🌎️hub/🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs`: seven stray string labels left by the `LocalizedLabel` migration broke `semio-os-mcp-capabilities` and with it every server build.
- `📋️project.json` `workspace:components` now depends on `@semio-tech/flow-plugin:graph-generate` and `@semio-tech/stdio-plugin:composition-prepare`. Without them `describe --all` failed after 20–25 min on `source.unresolved-generated-module` (flow `graph_manifest`) and `source.missing-input` (stdio `🤖️generated/🧩️composition/🦀️.rs`). Three earlier attempts were lost to this.
- `closure.provider-generated-import` refusals now name the importing modules.
- Guest Core wire reserve grant (agent) and interpreter / metering crates at `opt-level = 3` in `.cargo/config.toml`.

## Expected finish

The Rust build is the critical path. A cold `describe --all` takes 20–40 min and a server rebuild 20–60 min on this host. If no new breakage from concurrent refactors appears, the descriptors land around 12:30 and note create plus the full flow around 13:30–14:30. Every concurrent refactor that breaks compilation (three so far today) adds one build cycle.

## Anticipated

- More fallout from the label migration in checked-in fixtures and JSON descriptors that `describe` does not regenerate.
- Note create may still exceed the open wall budget even at `opt-level = 3`; the agent is profiling which guest phase dominates.
- The publish build `@semio-tech/framework-os-mcp-rs:build` again competing with other agents' builds for 16 GB.

## Resource usage

- 10 cores, 16 GB RAM, load ≈ 10, about 60 MB pages free, 54 GiB disk free. The host is saturated by memory, not CPU. A third concurrent Rust build gets killed by the memory warden, so two heavy builds at a time is the real ceiling. More local agents would only add contention.
- Fleet: 15 workers registered, but offloads are mostly declined (`local-slot`), several fail their workspace stage, and some share duplicate host names. It is being built in parallel and is not usable yet, so builds run natively.
- Agents: one Sonnet agent on note create, one on the workspace tests (waiting for memory), main chat coordinating and fixing build breakage directly.

## Long-term proposals for faster development

1. **Declare generator inputs, never discover them late.** A target whose closure contains `include!`/`mod` of a `🤖️generated` path should get its producer as an inferred `dependsOn`. The native-build already knows the owner (`generatedModuleOwnersV1`); move that check into graph inference so the graph schedules the generator rather than refusing after 20 minutes. This removes the failure class fixed by hand here.
2. **Fail-fast preflight before expensive builds.** Run a cheap source closure check (unresolved modules, missing generated inputs, source boundaries, `closure.provider-generated-import`) in seconds before admitting a long build. Today every one of these surfaced only after 20–80 min of planning and compilation.
3. **Atomic schema migrations.** A schema change such as `label: String → LocalizedLabel` should ship with its regeneration in the same change: a `repo` migration-check target that compiles all dependants' tests (`cargo check --tests` on reverse dependencies) and regenerates descriptors before the change counts as done. Half-migrated trees break every other agent's build in the live worktree.
4. **Keep the main branch compiling: per-crate health gate.** A resident `cargo check` daemon over the whole workspace that publishes the currently broken crates and their last editor, so agents see "broken by in-flight edit" in seconds and do not spend a build cycle to discover it.
5. **Optimised dependencies by default in dev.** Hot loops (interpreter, metering, codecs, pack) at `opt-level = 3` through `[profile.dev.package."*"]` with an explicit opt-out list. Debug-only speed made a correct feature look broken (BUDGET_EXCEEDED) for hours.
6. **Prebuilt, content-addressed binaries for tools agents need.** `semio-os-mcp`, `describe` and `native-transpile` published to the shared cache per source digest so an agent fetches instead of rebuilding a 600 MB debug binary; together with a working fleet cache this turns 40 min into seconds.
7. **Memory-aware build scheduling across agents.** One host-wide queue (the capacity service) for heavy Rust builds with explicit RSS reservations, instead of each agent starting builds that the warden later kills. Builds should be suspended and resumed, not cancelled.
8. **Log hygiene.** Build logs of 37 MB are mostly `"stage":"running"` heartbeats. Heartbeats belong to a progress channel, and the final failure should be the last line plus a one-line summary with the refusal key, so a failed run explains itself without searching.
9. **Native MCP probe harness checked in.** `🐍️probe-flow.py` plus the native environment record (`SEMIO_PLUGIN_CATALOG`, component model manifest) should become a `repo` target (`os-mcp:e2e`) with fixtures, so each agent can run the end-to-end flow against a fresh binary with one command.
10. **Smaller crates on the hot path.** The source boundary refusals (for example `semio-framework-job-context` at 262309 of 262144 bytes) show crates growing at their limits; splitting them early keeps incremental rebuilds small.

## Update 2026-11-28 02:00

### Progress

| Area | State |
| --- | --- |
| Discovery, search, context, conversation, workspace resources, prompts, jobs, typed refusals | ✅ native |
| Note create: wire grant | ✅ fixed. The authored Infrastructure grant was 131072 items; one note open turn costs ~49 k items. The grant now funds the largest staged actor, with a law test. |
| Note create: hang | ✅ gone. Every run now answers typed (`BUDGET_EXCEEDED` / `INTERNAL`). |
| Note create: speed | 🔧 cold open needs ≥ 12 guest turns × ~34 s, against a 210 s budget. An agent has landed: budget and cancellation checks once per 1024 interpreter iterations instead of per instruction, O(1) remaining fuel, a typed cause for `Refused: None`, and the drifted first-response fixture. The rebuild and probe are pending. The 290 MB per turn is a ledger figure, not a memmove. See [🔎️guest-wire-reserve-grant.md](🔎️guest-wire-reserve-grant.md). |
| Descriptor regeneration (`describe --all`) | 🔧 attempt 6 of 6 running. Earlier attempts failed on: missing flow/stdio generated inputs (fixed generically), other agents' half-written edits (3×), build-implementation changes mid-run (exit 75), and twice an anonymous SIGTERM to the outer `repository:command`. An agent is tracing that SIGTERM: [🔎️unattributed-sigterm.md](🔎️unattributed-sigterm.md). |
| Workspace long tests, shell bridge UI, official publish, full 36-step flow | ⏳ behind the two items above |

### Fixed since the last update

- The native component build now runs every declared `[package.metadata.semio.preparation]` of its closure, reusing the `repo cargo` preparation. `describe --all` therefore no longer fails after 20+ min on missing `🤖️generated` inputs. Before this change only the playground passed `prepareCargoOwners`. See [🔎️native-build-preparations.md](🔎️native-build-preparations.md). The stopgap `dependsOn` entries are removed once a run succeeds.
- Seven stale string labels in Rust test builders broke every `semio-os-mcp` build after the `LocalizedLabel` migration.
- Operation grants in `host-operation-policy.json` and `host-driver-policy.json` are sized from the staged actor.
- `opt-level = 3` for the interpreter, job, value, actor and host wake/relay/poll crates: throughput went from ~4 M to ~13 M fuel/s.

### Expected finish

Not before the morning (≈ 08:00–10:00):
1. The note rebuild and probe take ~1 h.
2. If the 1024-iteration window is not enough, there is a further interpreter iteration.
3. `describe --all` takes ~1.5 h per attempt and has a ~50 % chance of surviving the churn.
4. After that: the publish build (~1 h), and the full flow and shell UI (~30 min).

### Anticipated

- The interpreter may need deeper work (fused metering, a cheaper dispatch loop), or the note actor's open may itself do avoidable work.
- More anonymous kills of long runs until the SIGTERM sender is attributed and fixed.
- Disk drops ~6 GiB/h with three agents building; currently 31 GiB. Old cargo lanes are pruned below 15 GiB.

### Resources

- Load is 5 on 10 cores, the host is still memory-bound, and a `describe` and an `os-mcp` build run concurrently (the safe maximum).
- Two Sonnet agents (cold-open speed, SIGTERM attribution) plus the coordinator.
- The fleet still declines most offloads, so builds stay local.

### Additional long-term proposals

11. **No anonymous kills.** Every termination the build system sends carries sender, reason key and text, and the victim prints it. Two 80-minute runs were lost without a cause.
12. **Long runs survive implementation churn.** A change to build-implementation sources during a run should apply to the next unit, not restart the whole run (exit 75 class) or kill it.
13. **Interpreter performance budget in CI.** A bench of a real cold actor open with a fixed fuel/s floor, so a slow debug path cannot be mistaken for a broken feature.
14. **Plugin open cost visible per turn.** Ledger totals, fuel and wall per turn emitted as structured progress, so diagnosing "12 turns × 34 s" takes one probe, not a day.
