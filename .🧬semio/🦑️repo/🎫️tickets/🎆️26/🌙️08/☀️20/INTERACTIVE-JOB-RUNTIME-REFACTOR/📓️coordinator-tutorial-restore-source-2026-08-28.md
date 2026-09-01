# Tutorial Capture And Restore Source Snapshot

Read-only coordinator follow-through, 2026-08-28. No production edit or native build occurred. This narrows the historical `renderer-tutorial-interaction-contract-seam` report without marking its whole end-to-end work complete.

## Current Source Evidence

The Plugin now contains concrete `begin_local_interaction_query` and `publish_local_interaction_query_reply` implementations (near23912/23949 at this read), plus publication and dispatch callers (30442/30846). The OS TypeScript module has `AppChannel.readLocalInteractionPages`. These are actual source-present query/producer pieces, so a blanket statement that capture has no producer is obsolete.

The renderer ShellHelpers tutorial bridge still captures `session.viewState.selectionJson` into its snapshot and applies it through `SET_SESSION`, including the old selection delta member. Those exact lines remain among the reported strict tutorial diagnostics. Merely supplying an empty selection map, casting the old members, or treating ephemeral remote presence as local restore would not implement the intended capture/seek/restore behavior.

The inspected Plugin names do not expose a `LocalInteractionRestore`/`restore_local_interaction` route. This bounded name scan is not proof that no equivalent canonical mutation exists elsewhere. The Mutation task is independently examining the real command/leaf/retained-publication vocabulary and will return its source-backed report before an implementation assignment. No alternate ABI, cold restore fallback, ordinary selection approximation or structural restore authority is approved by this read.

## Required Acceptance Boundary

Capture must preserve the exact original activation/lifetime and document/config/topology basis through asynchronous pages. Seek/restore needs a typed retained mutation/command route with exact local-only domains and anchors, not state copied only into renderer viewState. Neutral and runtime laws must cover removed and nonbroadcast domains, empty selection versus absent change, comma-containing IDs, stale topology/base revision, cancellation and original-owner publication. Authored cold composition tests do not certify the live query/restore join.

## Selected Source Hashes

This is a read-time selected snapshot, not pre/post execution or a transitive closure.

```text
c2b2dfd9aaf33df31bdeea69496d02103f4ebc237a6e82527a961fe0b1017d83  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts
2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs
0962a01fc34439decc09d0322485dc8a403b7cabef6dfe88114505db4806de1f  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx

```

