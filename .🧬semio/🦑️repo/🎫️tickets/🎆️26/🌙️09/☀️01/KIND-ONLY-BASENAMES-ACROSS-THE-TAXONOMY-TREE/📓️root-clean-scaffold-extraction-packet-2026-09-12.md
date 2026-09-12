# Root Cleanup and Scaffolding Extraction Packet

Date: 2026-09-12

## Current Boundary Evidence

After the active inventory/mutation extraction began moving its functions, root repeated the installed TypeScript declaration-symbol walk over the current mandatory root script. This snapshot has 1,076 top-level statements, 2,632 bound local dependency edges, no syntax diagnostics and no multi-declaration strongly connected component. It does not include imported implementation internals and does not prove repository-wide acyclicity.

Source SHA-256: 433824ac57f141f7a5594b967658464d41a7d7b14432df5ccb9f77937e92544a, report provenance only. Exact graph and member lists are in 🗑️generated/coordinator/root-clean-scaffold-declaration-graph.json and root-clean-scaffold-closures.json. Earlier root graph has separate unchanged output and remains a historical snapshot.

| Seeds | Local non-import declarations | Declaration lines |
| --- | ---: | ---: |
| runWorkspaceClean | 50 | 500 |
| CleanScript | 52 | 658 |
| newScaffoldSubsetTree, newScaffoldStandardTree, newScaffoldArtifactTree, newScaffoldMutationTree | 29 | 336 |

The clean command closure includes the workspace-clean closure; these numbers overlap and must not be added as separate payloads. The scaffold closure is smaller than the previous 36/360 snapshot because the active mutation lane now owns shared identity primitives. Confirm live imports and reread exact bodies before editing. Root inspected command composition, scaffold generation and concrete consumers; this packet is not approval of all cleanup behavior.

## Three Independent Concerns

1. Workspace cleanup: actual candidate discovery, protected-source/ticket boundaries, ticket-state and generated-output interpretation, output size limits, platform names, projected removals and owned filesystem execution. Keep domain primitives beneath their consumers. Candidate policy and removal execution are distinct concerns; do not move all 500 lines to a generic helper. Keep the existing CleanProtectionView first-party boundary, protected prefixes, no-follow behavior, marked Cargo target detection, dry-run reporting, and cache-prune delegation.
2. Taxonomy command workflow: CleanScript.runTaxonomy still owns inventory/plan/apply/verify orchestration, despite its option, shard and mutation APIs now being imported. This belongs beside the normalization command contract/workflows, not under workspace removal policy. Preserve operation validation, guarded input paths, single capture, ticket artifacts, progress, cancellation, plan/baseline/digest checks, unresolved decisions, committed-state requirement and fail-on-warning. Command routing should delegate to that actual workflow.
3. Artifact and mutation authoring: scaffold templates, input identity validation, aggregate mutation update and write ownership belong beside the existing authorArtifactScaffold and mutation semantic owners. Preserve no-overwrite behavior, exact generated source/protocol meaning, dry run, no-follow path checks, rollback only of owned inode/content and cancellation. Scaffold placeholders are actual generated templates and must retain their explicit placeholder semantics; do not turn a generated SCAFFOLD into a fake implemented feature.

CleanScript also has coverage-output deletion and test-domain/cache delegation. Move that behavior to its real cleanup composition owner while the root entry remains command registration. Do not create root reexports or compatibility class facades. Existing repoTaxonomy is a three-line wrapper used elsewhere; reuse its actual lower taxonomy reader rather than reverse-importing root.

## Concrete Consumer Closure

Root found these current consumers that must be rebased, including source-as-data and subprocess strings:

- library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts dynamically imports CleanScript and invokes it on a fixture workspace.
- library/🧪️tests/🧹️clean-ticket-runs/🟦️.ts imports and invokes CleanScript.
- library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts parses root source and selects the CleanScript class. After extraction, assert actual workflow calls/options from the real owner plus command delegation, not a dead copy.
- library/🧪️tests/🔬️workspace-contract/🟦️.ts imports newScaffoldMutationTree and embeds a subprocess string importing CleanScript for inventory. Numerous portable mutation-authoring cases include dry runs, occupied leaf/mount, aggregate replacement, cancellation, symlinks and escaped/excluded scopes.
- library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts dynamically imports rootPath to obtain newScaffoldSubsetTree. Preserve its actual native/independent fixture oracle and move the imported path.
- Read the root new-command classes and helpers, including newResolveChildDir, to keep all command behavior out of the mandatory entry after these scaffold APIs move. The four function seeds alone do not cover their parent command routing body.

The library prefix is 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library. Search all current static imports, literal and computed dynamic paths, export lists, native source expectations and launch/cache inputs before declaring closure.

## Execution and Acceptance

Use a portable schema-first fixture and native installed TypeScript/independent filesystem oracles. Verify actual original and extracted behavior against semantic output, not implementation-body SHA. Register focused Bun/Nx tests and applicable launch seed/derived entries using only mandatory 📜️script.ts executable routers. Validate every new semantic owner through all ancestors.

Never invoke workspace-wide cleanup, live taxonomy apply, or live scaffold generation to prove extraction. Exercise removal and authoring only in private ticket-generated fixture workspaces. Inspect existing test path creation first; do not sweep this task's report/control files or historical ticket data. Preserve concurrent root/shared edits and source-classifier improvements.

Run focused direct and registered tests for portable topology, actual cleanup/protection, cancellation, source-as-data command delegation, and scaffold semantic output/native consumption. Keep meaningful current failures explicit; do not alter unrelated app/tool policy or claimed authority to obtain green.

Report 📓️sol-root-clean-scaffold-extraction-2026-09-12.md with exact changed-file list and command/test/runtime limits. Scratch is 🗑️generated/sol-root-clean-scaffold-extraction. Retain authored controls and Markdown; remove only disposable outputs of this lane. No Git mutation, AGENTS edits or ticket/goal lifecycle changes.

