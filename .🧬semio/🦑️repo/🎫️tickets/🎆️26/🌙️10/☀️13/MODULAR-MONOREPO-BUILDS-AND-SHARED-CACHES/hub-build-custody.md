# Hub Compiler Admission and Artifact Custody

The scoped Hub `EditorComponentCheckScript` now uses the narrow first-party native lease facade and canonical `<repository>/.cache/framework/leases`. It derives one finite admission from the existing ScriptInvocation, preserving its original signal, deadline, progress and continuation. It no longer creates a polling AbortController or renews a per-package deadline. The actual Cargo preparation/command runs through `runRepositoryCommand` with that original invocation. The stale storage import was corrected to the current upper native orchestration owner.

Each component receives a fresh invocation-owned target directory. This avoids another same-profile cohort compiler replacing the target artifact. The existing bounded, checked and fsynced `copyCatalogArtifact` captures its linked component into a new `wx` file while admission remains held. Only then does the lease release. Optimization reads the private captured file, and final cleanup removes the invocation-owned scratch target and raw capture. No shared profile/target cache is deleted.

## Qualification

An initial test harness attempt had a wrong fixture directory and is not semantic red evidence. After correcting that harness, `hub-capture-red2.log` captured two actual focused failures: source capture ordering followed release, and the actual consumer supplied the foreign namespace rather than canonical lease storage. These original defects were reproduced in the current defining consumer before final repair.

`hub-capture-green5.log` passed via actual Bun + Nx in 1.0 seconds: three laws, 93 assertions, zero failures. The schema-first language-neutral fixture is checked by strict Ajv2020. SQLite independently records capture before release and observes held custody during every actual copy. The actual Hub class, bounded copying helper and release transformer run via Bun and independent esbuild with neutral child transport. A separate Node filesystem copy/hash/transform oracle matches captured bytes and digest. Replacing the linked target artifact at lease release does not alter transformed captured input. Original cancellation and capture refusal both release admission and prevent transformation. All scratch fixture roots are removed by their owner.

A third law bundles the complete current defining Hub router with esbuild, resolving actual first-party storage and narrow lease imports. It also parses both real launch sources with their JSON/JSONC grammars and verifies exactly one registered acceptance entry.

No real Cargo or JCO child ran during this scope. This is source/consumer/custody qualification with neutral transport; complete Hub component compiler runtime qualification remains outstanding. The wider target-maintenance identity and shared cohort protocol belong to the parent and were not edited. Ending observed disk headroom was 3.2 GiB; no broad compile was started.

## Exact Touched Paths

- `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`: canonical namespace, original control, private target, capture before release and current storage import.
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧬️schema/🔣️.json`: closed neutral custody corpus schema.
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧪️tests/🧫️fixtures/🔣️.json`: canonical namespace, original/replacement bytes, phase order and denial cases.
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧪️tests/🟦️.ts`: actual consumer, independent SQLite/NodeFS/esbuild and defining-router bundle laws.
- Ticket `headroom/📜️script.ts` and `headroom/project.json`: canonical `hub-capture` source gate; generated fixture outputs stay in the ticket.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: Hub Compiler Artifact Custody Source Acceptance, existing gate group, order 900.05748025.
- This report.

No runtime external dependency, compatibility adapter, copied producer, modifying Git command, worktree or AGENTS edit was introduced.
