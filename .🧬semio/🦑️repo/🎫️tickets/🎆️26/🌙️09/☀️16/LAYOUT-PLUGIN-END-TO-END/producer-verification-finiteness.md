# Producer Verification Finiteness

Read-only defining Source trace while the sole setup64843 remains active. No Source/provider edit, test, compiler, runtime probe, cache operation, policy change or guard weakening was performed. This report proves structure only, not the live request's elapsed-work bound, current loop index, completion or successful output.

## Finite Completion Plan

Current publication owner is library/caching/storage/enforcement/budget/opening/publication/🟦️.ts. `completeRepositoryDependenciesV1` captures one current ownership/input projection and calls `runRepositoryDependencyProducerCompletionV1` once (:406). The completion function does not call itself or restart the setup dispatcher. A ready bootstrap cut can return from its separate qualification path before producer completion; a resume cut uses the existing-input branch.

`readJavascriptDependencyProducerMaterializationV1` validates a closed authored maximumScopes/maximumFiles/maximumAggregateFiles schema, sorts a finite copied scope list, rejects duplicate scopes and generation prefixes, requires complete contiguous member/patch roles and projects one native input group. It returns a frozen scope list. Completion traversal uses that list or the already captured finite plans/bindings, never a queue enlarged by its own iteration.

- Fresh path: `if(!resume) for(selected of materialization.scopes)` (:382). One fixed installer command per selected scope; status/signal refusal exits rather than looping. Each successful scope pushes one finite result and emits sync-qualified with that result count.
- Resume path: `if(resume) for(row of captured.plans)` (:383). It checks existing installed paths and readiness; it does not invoke the fresh installer loop.
- Ready publication loop (:383) increments index up to captured.plans.length.
- Source transition loop (:386) is plans × each plan.bindings, with no push into either traversal collection. Before each transition it verifies all ready plans; successful accepted bindings update expected observations, not the list of planned transitions.
- Final refresh loop (:387) is one more finite installed-path pass over plans, then funding/input/boundary checks and return. The catch records incomplete DATA and rethrows; it does not restart work.

No accidental unbounded cycle or repeated installer restart is proved by this Source. A live phase may still take substantial time because its finite plan contains many files/edges and the code deliberately revisits it at multiple boundaries.

## Why The Same Verifications Recur

Let S be captured plan count, B total Source bindings, F total immutable input bytes, E declared membership edges and D installed entries. These are structural variables, not measured values of64843.

| Verification | Current call structure and purpose |
| --- | --- |
| Completed immutable inputs | Native receipt reads verify all captured directory anchors and every immutable file descriptor/name/stamp/content hash under unchanged whole-allocation journal/backing. `fence()` explicitly calls the complete group reader. The producer reader wrapper also validates the complete receipt before entering and after its actual pending read drains. This is current Source identity checking across awaited work, not a readiness cache. |
| Funding measurement | Initial unused-token measurement occurs before staging. Poststage measurements authenticate the completed receipt, including a complete immutable-input read, before actual current usage measurement. Measurements occur after staging, after each fresh installer, before Source transitions and at final return. |
| Fresh selected-root read | `read...FreshArtifactInputGroupRoot(receipt,rootIndex)` scopes the exact fresh roster check to the selected root, but `verifyProtectedArtifactInputFilesV1` intentionally still iterates all immutable input files. Thus repeated selected-root validation can reread F, not just that root's files. Defining docstring explicitly preserves all other immutable byte proofs; this is not an accidental infinite loop. |
| Installed paths | Completion fresh/resume pass, pre-Source-ready refresh (:385), and final refresh (:387) each call the complete installed-path reader for each plan. Each reader performs one full bounded scan with context rechecks around it. The current completion intentionally calls the full reader, not the separate incremental ledger API. This is at least three explicit complete per-plan passes on the fresh completion route, without treating wrapper input reads as installed scans. |
| Workspace/provider edges | Initial `read...SourceOwnerExecution`, every `observeExecution()` and every `verifyReady()` creates a current all-selected-scopes provider observation. `verifyReady` is called before Source publication, before every binding transition, and after all binding transitions. It also reads every plan's exact ready receipt. Per-call manifest/provider/physical memoization is local to that observation; it is not a cross-boundary cache. B transitions therefore can cause B repeated E traversals plus B×S ready-receipt reads. Each such read also crosses completed-input wrapper boundaries. |
| Ownership/Source boundary | `boundary()` rechecks current workspace custody, captured Source inputs, complete binding/parent/ancestor observations and the unchanged funding receipt. `fence()` calls boundary after completed inputs and retained-capsule checks. Inside a producer read, context.recheck now checks its actual live physical/control frame; it does not recursively call full receipt validation. |

The repeated Native full input hashes plus repeated all-scope execution/ready checks can multiply work structurally (including B×S read wrappers each surrounding F). That is a concrete repeated-cost property of current Source, not an observed runtime profile or proof that any guard is redundant. No shortcut or weakening is recommended here.

## Inner Traversals Are Bounded Too

The installed-path scanner in dependencies/storage/🟦️.ts:31–43 uses a declared maximumWork, maximumEntries, maximumDepth, maximumPathUnits and per-directory entry cap. It records visited directory dev/ino identities and rejects repeats. Symlinks are classified as leaf entries, not recursively followed. Its descriptor enumeration ends on null; file traversal and rechecks advance under actual control/progress checks. The directory roster is finite under those checks or refuses.

Source-owner candidate observation (:366–378) caps scopes, work, edges, observations, manifest bytes and output/candidate bytes. Provider symlink resolution uses a seen-path Set plus maximumLinkHops. Ancestor searches move strictly through dirname until a stage/root stop. Relations iterate captured model sources × memberships × declared edges; per-call memo tables avoid resolving the same candidate repeatedly within that observation. Two bounded observation replays bracket the context recheck.

Native immutable file verification consumes only the manifest's captured finite files and directories. Byte loops increase offset by a positive read count and refuse zero progress, compare exact length/hash/stamp/name afterward, and consume the authored Native work limit. These are not free-running retries.

The separate already-ready bootstrap qualifier (:394) explicitly calls its whole verify routine twice, returning true after the second stable pass. This is a fixed double verification of ready receipts/installed paths/exact inputs/Source boundaries, not a `while(not-ready)` loop. It must not be confused with proof the live request is using that branch.

## Runtime Interpretation Limit

Repeated installed-path/workspace-edge/native-artifact-input-read progress names can come from the finite repeated phases above, even after a scope has installed successfully. Source alone cannot identify64843's current scope or prove a phase made progress since the last retained owner receipt. Only the setup owner's actual issued progress and terminal handle can do that. No duplicate runtime or speculative optimization was introduced.
