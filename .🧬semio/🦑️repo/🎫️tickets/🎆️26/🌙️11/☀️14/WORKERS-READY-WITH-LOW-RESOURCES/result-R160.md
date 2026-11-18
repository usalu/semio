# R160: the closed scoped plan wall `/scope/targets must contain at most 4096 item(s)` (g1-m07, 2026-11-17 23:11 UTC) (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R160/` (`land.py` anchored apply/restore with sha check, `red.sh`, `window.sh`, `battery.sh`, `one.sh`, `compare.py`, `tests.list`). Scratch: `scratchpad/R160/`.

## 1. The schema and its siblings

`TargetScope.targets` in `🏗️build/🧭️selection/🎯️scope/🧬️schema/🔣️.json` had `maxItems: 4096`; `ScopedPlan.scope` is a `TargetScope`, so every scoped plan was cut at 4096 scope targets. `scopedPlanOfReplyV1` (`🧭️invocation/🛰️plan/🟦️.ts`, runs in the CLI process after the service reply) validates the plan against it and threw a plain `Error` ("Internal error" at the CLI boundary). `ScopedPlan.projects/tasks/edges` have no `maxItems`; names are `maxLength: 256` (a name bound, not workspace-scaled).

Siblings of the same plan chain with a fixed ceiling below the whole workspace (found by scanning every schema of build and caching for `targets`/`tasks` collections with a bound below owners x tasksPerOwner x 2 = 152768):

| schema (pointer) | before | role |
| --- | --- | --- |
| `selection/scope` `definitions/TargetScope/properties/targets` | 4096 | scope of the closed plan (the wall) |
| `resident` `definitions/PlanRequest/properties/targets` | 4096 | request of the host service (a roster of named tasks) |
| `resident` `definitions/Plan/properties/tasks` | 65536 | the service reply (a whole-workspace plan has 76k to 120k tasks) |
| `selection/closure` `properties/tasks` | 65536 | closure document of a selection |
| `inputs/selected/rust` `properties/tasks` | 65536 | selected Rust inputs |
| `tools/bazel` `definitions/Planner/properties/request/properties/tasks` | 65536 | Bazel planner request |
| `seal/selection` `$defs/ScopeV1/properties/tasks` | 2000 | sealed selection scope |

Not workspace collections, so exempt with EN/DE reasons in the family: tasks and targets a person names on the command line (`🧬️schema/🔣️.json` and `🚪️selection.json`, 4 pointers), the delegation chain of an orchestration context (`Context.targets`, 64), the fixed Nx maintenance images (`CurrentMaintenancePlan.targets`, 67). `HostPolicy.maximumInlinePlanTasks` (4096) is a spill threshold (the reply goes through a spill file above it), not a refusal: left.

## 2. The law (red first, then green)

Family `🏗️build/📏️workspace-scale/🧬️schema/🔣️.json` gets 7 members (`maxItems`, scale `owners*tasksPerOwner`, required 152768 = 4774 x 16 x 2), one `equal` group (one value) and 6 exempts. The scan of `⚡️caching/…/🔏️authorizers/🧪️tests/📈️growth` now also looks at collections named `targets` and `tasks`. The new law `🧭️invocation/🧪️tests/📏️plan-scale` builds the plan of a whole workspace (4774 owners x 16 tasks = 76384 tasks, 76384 scope targets; and a chained variant with 71610 edges) through the real `scopedPlanOfReplyV1`, validates it with Ajv too, and requires a typed bilingual refusal for any other violation.

Red before (run under the lock at 23:20:46 to 23:20:58 UTC with only the test files applied, restored exact, `scratchpad/R160/red-*.txt`): growth 3 laws red (`every declared ceiling ... holds the declared workspace size` lists the seven ceilings 4096/4096/65536/65536/65536/65536/2000 below 152768; `ceilings of the same documents are one value`: `[4096,4096,65536,65536,65536,65536,2000]`; the scan), plan-scale 3 of 3 red (`Closed scoped plan required: /scope/targets must contain at most 4096 item(s) (76384 tasks, 0 edges, 4774 projects, 76384 scope targets)`), `🛰️plan` typed-refusal law red. Green after: see the landing section (growth: the 3 laws of the family are green; its remaining 4 red laws are the baseline ones).

## 3. Numbers: what the 6276 / 4443 are, and is the plan wider than needed

- "scope targets" = `scope.targets` of the scoped plan: the targets the plan was asked for. `scopedPlanOfReplyV1` takes the named targets of the request; for the roster request `*:*` it takes the ROOTS of the plan (tasks no other task of the plan depends on), else all tasks. The old planner expanded `*:*` of `run-many --projects=<3 literals> --targets=build` to EVERY task of the slice (all targets of the 259 projects of the closure): 6276 tasks, 2504 edges (most tasks have no edge: lint, test, docs, ... of each project), 4443 of them roots, hence 4443 scope targets. The plan was 20 times wider than the selection needs (the selected `build` tasks of the closure are at most 259 plus what they need).
- R158 landing 1 (23:00 UTC, `rosterTaskIdsV1`) already made the roster of an owner selection with literal projects and targets exactly projects x targets that exist; the planner then walks only `dependsOn` (a plan is the transitive closure of the selected tasks). The m07 failure at 23:11 UTC shows the SAME counts as m08 because that read had started before 23:00 (the service of that read ran the old planner). Proven by R158's law `🏡️resident/🧪️tests/🎯️roster-scope` (3 pass): 3 selected projects of a slice of 2500 extra projects give 4 tasks.
- Before/after on the smallest real reproduction available: the sealed selected generation of the probes is not on this machine (the origin state root of g1-m07 holds only logs, `slice-memory.json` and replica segments; a read of the 4774-owner workspace was forbidden), so the synthetic law stands in: 6276-task slices of 3 selected projects give 4 tasks / 4 scope targets with the roster rule (R158 law), a 76384-task whole-workspace plan is accepted with the family bound (new law). The real count of the `build` closure of `norm-din4108`, `norm-din4108-rs`, `repo-mcp` is known only after the next worker read; the worker log of g1-m07/m08 after 00:00 UTC will carry it (see the watch section).

## 4. Typed refusal, raised where the plan is built

`scopedPlanOfReplyV1` and the second producer `buildScopedPlanV1` (`🧭️selection/🎯️scope/🟦️.ts`, plain `Error("Closed scoped plan required")` without detail) now throw `refuseBuildV1("refused","graph.plan-not-closed",{detail,tasks,edges,projects,targets})` (texts EN/DE in `📖️texts/🔣️.json`: names the violated rule and path and the counts, says it is a defect of the tool, not of the command). Law in `🧭️invocation/🧪️tests/🛰️plan` (replaces the 4097-task law that asserted the old cap) and `📏️plan-scale`: `isRepoErrorV1`, both languages rendered, in about 1 ms for a violation. The 800 s of "The graph service holds this request" before the error were the read plus the plan of 6276 tasks (keys of every task); the narrower plan removes most of that; the refusal itself is immediate where it is raised. NOT done: a host-side pre-check before `taskKeysV1` (the family bound is far above any real plan, so no violation can precede the key computation any more except a 257-character name, which cannot occur in the graph).

## 5. Found, not mine

`🏗️build/🕸️graph/🔁retry/🧬️schema/🔣️.json` `$defs/SealPolicy/const maximumOwners=64` (a peer's file, modified 23:22 UTC) makes the existing scan law `a workspace-named ceiling below the owners times the safety factor is a member or exempt` red at baseline (before my change). It needs a family member or an exempt with a reason (it looks like "owners of one retry seal": exempt). Growth baseline also has 3 red laws unrelated (`Ticket-owned test artifacts required`).

## 6. Landing

First try 00:00:00 to 00:03:13 UTC: restored on red, `plan-scale` printed nothing. Cause (proven in scratch afterwards): my own test let Ajv check `uniqueItems` of a 76384-item array, which Ajv compares pairwise; the run did not finish in 180 s (a run without `uniqueItems`: 41 ms for Ajv, 7 s for the whole test). Scratch proof of the landing files before the second try: a bun preload (`scratchpad/R160/stage4.ts`) serves the staged copies of the 7 runtime files to the live test: `plan-scale` 3 pass (7 s) with the staged files, red with the live ones.
Second landing 00:30:41 to 00:31:35 UTC (`landings.md`, "R160 landing 1"): battery of 17 dirs not worse than before, `plan-scale` 3 pass, `🛰️plan` 13 pass, roster-scope 3, host 38, failure 17, client 8, plan-guard 12, texts 3, rust selected 4, selection/scope 5. Growth: 7 pass, 4 fail, all 4 red before (3 `Ticket-owned test artifacts required`, 1 the scan law red only on the peer entry of section 5).

## 7. Watch after the landing

- The retry of g1-m07 that ran at 00:29 UTC (BEFORE my landing, with the R158 roster fix of 23:00) passed the planning stage: its `ready-log-workspace-failed.txt` no longer shows `Closed scoped plan required`; the plan was built and the stage went on to `Planning build tool storage` (`nx:storage-planning`) and failed with the NEW WALL: `Internal error: SDK derivative publication byte ceiling exceeded` (`repository:command failed with exit code 1`). This confirms that the narrow roster plan is within the closed schema even before the cap was raised.
- NEW WALL, nobody on it as far as `landings.md` and the R161 to R165 reports show: `nxSdkDerivativeHeadroomBytesV1` (`⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧮️formula/🟦️.ts`) throws when `derivativeSnapshotCopies*maximumSnapshotBytes + derivativeProgramCopies*maximumOutputBytes + inputBytes + maximumIndexBytes + maximumDerivativePatchBytes + maximumDerivativeReceiptBytes > maximumPublicationBytes`. With the declared policies the sum is 100663296 + 2*33554432 + 16777216 + 33554432 + 65536 = 218169344 bytes PLUS `inputBytes`, the ceiling `maximumPublicationBytes` is 134217728: the sum exceeds the ceiling for every input, even `inputBytes = 0`, so this stage can never pass on any machine (the cause is the policy numbers `📋️policy.json` against the snapshot policy, not a workspace size; also an untyped `Error` -> "Internal error"). Fix direction: derive `maximumPublicationBytes` from the parts (218169344 + maximumInputBytes 16777216 = 234946560, with the factor of the headroom law) or lower the parts; plus a typed refusal. Law: a table law that the sum of the parts at their maxima is <= the ceiling.
- Other origins at 00:32 UTC: m01 and m10 and m06 reading the graph slice (running), m08 in dependency binding, m04 and m05 in `tools`, m03 failed with `Build source authority keeps changing during inference` (the R156 seal path; its old code), m02 `taken 0 of 4455` after 26374 s (the whole-workspace read, not the probes).
