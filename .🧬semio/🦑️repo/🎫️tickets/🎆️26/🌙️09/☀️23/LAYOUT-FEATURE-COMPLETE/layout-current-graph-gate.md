# Current Layout Project Query

## Live Blocker

The launch-entry-point reproduction reached the exclusive `repository-graph-facts:/Users/ueli/Documents/semio:shard:60` queue. The CLI had obtained host capacity and captured the command source before the selected graph query. Cancelling that inspection produced exit 130. [Layout launch entry point](layout-launch-entry-point.md) and [original command log](🗑️generated/layout-project-current.log).

The runtime graph query gives the literal project an owner selection. The default slice mode sends that selection into `baseline:true` selected inference. This path disregards the current policy and calls `withBuildGraphStateV1` with the selected scope. Graph custody calls `withRepositoryBuildGraphSelectionV1`, which derives the facts shard from the scope and acquires it exclusively. Recomputing the selected scope from the current canonical owner registry produced scope `508c27860250c85056002c9675d3825edd9ff0bf0f36ff5c77ce7ea1c8943cd1` and shard 60 of 64, matching the live log. Task-store maintenance also precedes the query. [Runtime query](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts:83) [Graph load](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts:281) [Graph custody](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🛑️custody/🟦️.ts:24)

## Generation Switch

`SEMIO_BUILD_GRAPH_SLICE_READ=generation` selects the generation-backed loader. It can serve a local generation over its shared reader. It can also enter the writer on a missing generation, a fleet-ahead response or missing closure data. The existing whole-workspace current path can restore a fleet generation. This switch does not guarantee that a named current query stays in its reader.

## Candidate

The patch routes explicit current project inspection with no retained or bound graph to a sealed-only projection of the named project's generation closure. Host admission, implementation custody and command diagnostics still apply. The query uses the existing exact closure and machine output paths, skips the live owner selector and task-store maintenance and produces a typed refusal when the sealed generation or the required closure data is absent. This path stops before fleet lookup, repair, builder admission and generation pin recording. Ordinary selected execution keeps its existing graph provenance. [Candidate patch](🗑️generated/layout-current-graph.patch).

## Generation Pin Check

The implementation pin stores source authority in an asynchronous context. Clearing that authority does not alter the graph generation pin. The top-level front door creates its own Original and does not issue a generation pin. A generation pin must be present in the environment and name a different live ancestor. The ordinary non-orchestrating task caller strips that environment variable. The candidate also checks the bound-pin reader before early dispatch, so a valid inherited pin follows the existing route even when the retained-graph flag is absent.

## Qualified Owner and Target Aliases

The OS dev manifest declares `@semio-tech/framework-os-dev` as the canonical project. The resolver passes this positional through for `show` and applies development-target resolution only to `run`. The live selector used to recognize directory and name expressions before the output step indexed the raw positional as a project name. The candidate retains the canonical project query. The `dev-layout-wgpu-prototype` and `serve-layout-wgpu-prototype` inference rules both depend on `activate-layout-wgpu-prototype`. The selected-runtime fixture expects the server to use port 6179 and the Layout WGPU composition. These source checks did not start a build or server.

## Isolated Checks

The current-generation fixture checks the captured runtime branch with in-memory ports and compares sealed project output with the independent Python graph model. The qualified-project regression exercises cold refusal, sealed stale output and refusal when required sealed closure input data disappears. It checks that the original pointer bytes hold, with no selected inference, graph facts hold, fleet operation, builder, closure repair or provenance update. The regression failed against the unchanged command with `Current project query requested selected graph facts`. All seven tests in the current-generation suite passed against the staged code. The existing selected-task generation case passed against the candidate loader, including exact task admission and refresh of only its dirty closure. All five edited files parsed successfully in memory. A read-only patch applicability check also passed against the shared worktree. Those isolated checks ran before the coordinated application.

## Regression Receipts

| Source exercised | Exit | Result |
| --- | --- | --- |
| Original command and graph loader | 1 | The focused test reached the selected graph facts route and refused before the generation reader. |
| Staged command and graph loader | 0 | Seven current-generation tests passed with 65 assertions. |
| Staged loader with the existing selected-task case | 0 | The selected-task case passed with 14 assertions. |
| Candidate patch against the shared worktree | 0 | Read-only patch applicability check passed. |

The isolated programs used the ticket test and preload under the generated directory. The original control ran the focused test with `LAYOUT_CURRENT_GRAPH_BASELINE=1`, which loaded the unchanged runtime source and disabled the candidate generation-loader overlay.

## Post-Application Checks

Run the current project query in a top-level shell once the runtime owner has applied the candidate. Live mode loads the edited door and captures the edited command kernel without promoting the shared sealed implementation. The graph query still reads the sealed graph.

```sh
SEMIO_REPO_SEAL=live repo show project @semio-tech/framework-os-dev --graph-policy=current --json
```

Expect JSON with the exact project name and the targets present in the newest usable sealed graph. If the graph or required closure data is absent, expect a typed refusal. In either case the query should report no graph-facts writer queue and no generation refresh. An older sealed graph can omit recent targets.

Run the production test sources without the ticket preload after applying. The selected-task filter keeps this check scoped to the neutral fixture.

```sh
repo tools exec bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🔒️current-never-builds/🟦️.ts'
repo tools exec bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts' --test-name-pattern 'a selected scope is the closure'
```

The staged patch passed the read-only applicability check before application. A reverse applicability check can confirm that the coordinated trial contains the patch if the touched ranges have not moved.

```sh
repo tools exec git -- apply --check --reverse './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/🗑️generated/layout-current-graph.patch'
```

## Shared-Source Refresh

The loader gained a shared-reader retry and a remote closure refetch before builder admission. Those peer changes left named current dispatch on the selected facts writer path. The refreshed candidate retains the peer retry for callers who can already return a repair request. The sealed-only named query refuses a missing closure inside its shared attempt and never returns a repair request to that retry. The candidate's added and removed lines match the original staged patch. Only context in the patch changed.

The refreshed patch applied against the latest bytes with exact context and passed the read-only Git applicability check. All five proposed files parsed. A focused execution of the refreshed loader passed four cases. Cold, missing-closure and warm named sealed-only requests each entered only the shared reader. The unflagged comparison performed a shared read, refetched the missing closure, and completed a second shared read.

The full Bun rerun stayed in host capacity admission for 151 seconds under memory and swap pressure and was cancelled before it started. The independent four-case loader execution above validated the context that changed.

## Coordinated Live Verification

After the coordinated application, the five live source files matched the tested refreshed candidate byte for byte. The focused test and the CLI probe use the applied source.

The focused Bun test passed on the applied source without a preload overlay. It exited 0 with one test and 16 assertions. [Live test receipt](🗑️generated/layout-current-graph-live-test.log).

The real CLI probe exited 1 after tool capacity and live command source admission. It reported that this machine held no usable sealed whole-workspace generation, and produced zero stdout bytes. Its diagnostics show no exclusive graph-facts or graph-state custody and no graph refresh or inference. The typed cold refusal establishes the live queue fix. It leaves the warm Layout target listing unverified on this host. [Live CLI receipt](🗑️generated/layout-project-current-live.log).
