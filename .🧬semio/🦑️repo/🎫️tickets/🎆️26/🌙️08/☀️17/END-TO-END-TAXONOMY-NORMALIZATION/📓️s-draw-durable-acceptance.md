# Draw Durable Acceptance Audit

## Decision

**The real Draw move is not ready for approval.** Its eleven-file destination authority and current source preimages are coherent, but no complete fresh plan exists, and the current tests do not yet provide live canonical-only acceptance. The test named “canonical CAD and Draw authority retains the exact language-neutral mapping digests” still reads the old physical files before constructing canonical nodes in memory. A production move would therefore break that test and six shared source-fixture callers.

This is a bounded read-only audit against baseline `9f449b10659b95148c8bcb3f91ce583bf7446973`. No new incoming scan, schema/catalog change, fixture rewrite, production move, generation or apply was performed. Neither actual Compose tree was inspected. The completed [lossless capture](🧾️draw-reference-capture-kmddtz/📝️.md) remains a failed observation, not an application packet.

## Exact Current Scope Authority

The unchanged [language-neutral catalog](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1) binds the historical source root, destination root and all eleven source/destination pairs. Its SHA-256 is `1410a74ccc87561fd4a4b91db7d503614fe21ddce8bc78dee923d8237820f3e0`; the Draw pair digest is `1f28fcc6e28e54001a9df6ce98b1c30b565cd42b824ed2491bb9b5e407b7436b`.

At **2026-08-27T13:58:40.163Z**, a fresh exact-scope check found:

- all eleven source content hashes, byte counts and modes equal to the retained owner shard; **242,790 bytes**, all **0644**;
- exactly **7 directories and 11 files** within the physical source command subtree, with no extra file or symlink;
- all eleven destination leaves absent, with existing destination ancestors checked without following links;
- 37 distinct existing source/catalog/shard ancestors checked as real directories before reading.

The owner shard remains SHA-256 `6a1d8f3efecf9e8c7991a28ba16ce1e2b06f3eb5e505c81523faabfc915097cb`. Its surrounding retained inventory contains 29 entries because it also includes ancestor context. The retained source-tree digest is `25defb08725cc7a5dc60e733444024078ed1b039c607c383c1d3aaa9d8b6a9c4`; this audit compared the eleven physical file preimages and scope membership, not a newly computed full inventory digest.

The current destination contract is **11 files, 9 directories, 20 nodes, maximum 210 UTF-8 path bytes**. Cargo manifests, Nx manifests and permanent routers retain their registered filenames. Three domain implementations use the physical Rust leaf; both package entry implementations move beneath `📚️library`. Two exact Cargo `lib.path` edits are mandatory. Older reports describing unchanged package-glue filenames, 18 destination nodes or a 204-byte maximum predate this contract and must not replace the current golden.

## Present Acceptance Dependency Map

The current [main repository-library tests](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts) contain these live dependencies. Function and test names identify the regions because other owners are adding unrelated tests concurrently.

| Region | Current input | Required disposition with the Draw batch |
| --- | --- | --- |
| `projectionAuthorityNodes` | Reads every mapping's historical `sourcePath` from the live workspace | Keep source simulation on genuine authored fixture input; do not use it for live canonical observation |
| “canonical CAD and Draw authority retains…” | Calls that source reader, then synthesizes canonical nodes and applies two configuration replacements in memory | Separate Draw live canonical observation from CAD's still-source physical observation; require real destination nodes for Draw |
| “artifact-editor-command-projection preserves…” | Reads the old Draw files and enumerates the old physical subtree | Change the live Draw assertion to the canonical subtree and require zero configuration edits; retain equivalent source-grammar negatives on authored inputs |
| `artifactProjectionNormalizationFixture` | Always copies both families' old live mapping files; with references also copies live external Cargo/glue/router inputs | Make the Draw source scenario self-contained before its real tree moves; a canonical-to-old reconstruction or existence fallback is not acceptable |
| Historical package-identity transaction | Uses authored vector bytes, synthetic manifest/source input and catalog coordinates | Already independent of old live Draw reads; preserve the historical coordinate authority unchanged |

The six direct shared-fixture tests are:

1. “plans the exact CAD and Draw authority mappings with structured cross-profile references”;
2. “rejects unowned artifact prose, unmatched selectors, escaped placeholders, and counterfeit owners”;
3. “rolls back and atomically applies CAD and Draw projections to an empty second plan”;
4. “rejects ignored and unplanned residual children in a projected source owner without following links”;
5. “rederives repeatable CAD and Draw source-tree authority before apply”;
6. “exact CAD source scope plans declared external edits and rejects declared stale consumers before mutation”.

The sixth is labelled CAD but still materializes Draw through the shared helper. Merely editing the Draw-labelled tests is insufficient. Also, the external plugin manifest/glue reads remain live consumers and will change with the move: retaining them as source-scenario fixture input would silently change that scenario even after its eleven file reads were fixed.

The [historical package-identity test](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️historical-package-owner-identity/🟦️.test.ts) is the useful counterexample: its scoped Draw transaction constructs test input from the authored vector instead of copying the old workspace tree. It preserves the genuine historical census while an ordinary neighboring JSON reference is edited. This audit read that implementation but did not rerun its transaction.

The old ticket-scoped `🧪️cad-draw-scoped-consumers.test.ts` and its fixture directory are currently absent; their report remains historical evidence, not a runnable current gate. No missing input was restored or replaced during this audit.

## Canonical-Only Transition Design

The production [projection validator](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts) already has an explicit destination-layout mode. It uses the forward owner vector and historical mapping identity, but obtains physical nodes and Cargo configuration values from the destination. The [normalizer](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts) already calls that mode for registered canonical roots and rejects unadmitted physical children. Keeping a historical source-coordinate field in a frozen contract is not a requirement to retain or read the source tree.

The durable split should be explicit:

- **Authored source simulation:** a schema-first, language-neutral eleven-member Draw bundle plus its minimal external Cargo, Rust, Nx and TypeScript consumers. Preserve the exact fixed-file union, owner tuple, configurable entry and structured-reference forms. These are authored test inputs, not compiler outputs or a serialized copy of the generated plan. The source scenario must run unchanged when no old production Draw path exists.
- **Live canonical observation:** directly enumerate the destination command subtree with no-follow checks and compare all eleven members, nine directories, twenty nodes, tuple, pair digest and path budget. Read both real destination Cargo manifests and require canonical `lib.path`, valid package ownership and no pending authority edit. Require the old source subtree to be absent. Missing canonical input must fail even if an old source copy is present; never select a source/canonical fallback at runtime.
- **Independent validation:** retain filesystem-set parity with the existing third-party filesystem oracle; validate JSON/TOML and path spans using the existing Ajv, JSON/TypeScript parser and `@iarna/toml` test dependencies. Run the real Rust packages through their registered Nx targets to prove relocated module paths and proc-macro semantics, not just string replacement. Keep these dependencies test-only.

CAD's real source phase must remain explicit until its own reviewed move. A Draw-only batch must not switch the whole shared family to destination reads, replace historical catalog coordinates, or erase CAD source tests. The eleven-file historical mapping golden remains byte-identical as evidence.

## Required Same-Batch Gates

1. Implement and red/green-test the durable input/live observation split before the final plan freeze; cover the six shared fixture callers and the existing source negatives. Handcraft the input rather than reconstructing old files from canonical outputs.
2. Obtain a complete fresh unfiltered Draw plan after the registry language/opaque-view corrections. Review all moves, authored edits, generator previews, removals and unresolved findings, with exact source and consumer preimages. The previous 43 reference-progress records are not 43 approved edits.
3. Check the two moved Cargo entries, their rebased Rust module attributes, package-local dependencies, both permanent router imports, Nx `cwd`/input/schema paths, root Cargo members, dependency-policy user coordinates, root policy source collection and external Draw Cargo/Rust consumer. This list is a minimum semantic checklist, not a filter on the incoming closure.
4. Rehearse the exact reviewed bundle in an isolated fixture: rollback, retry, committed canonical state, preserved unrelated bytes/modes, no source residue, unchanged frozen evidence and a wholly empty destination replan. New incoming references, consumer drift, collisions, symlink ancestors and stale post-state references must still fail closed.
5. In the same approved production batch, install canonical live acceptance alongside the actual moves/reference edits. The final source/consumer preimages must include any test changes that the plan considers incoming consumers; edits after plan freeze require rederivation.
6. Run the relevant registered uncached repository-library gates plus `@semio-tech/draw-fsm:test` and `@semio-tech/draw-fsm-macros:test`, and the Draw plugin's registered acceptance for its relocated command module. Verify canonical Cargo/Nx discovery and required generator/check outputs. Use explicit long test level for genuinely cold Cargo work; do not relax the fundamental or transaction budgets.
7. Finish with a fresh canonical-scope plan containing zero operations and zero unresolved findings, independent final byte/mode/member checks and durable transaction completion. Only then dispose of completed generated output/log artifacts; retain authored fixtures, scripts and Markdown reports.

## Checks Actually Run

The bounded no-follow source/shard comparison described above completed successfully. The two existing physical-authority tests were then run through the actual uncached Nx route:

```text
NX_DAEMON=false bun nx run @semio-tech/repo-lib:test --skip-nx-cache --test-name-pattern='canonical CAD and Draw authority retains|artifact-editor-command-projection preserves'
```

Result: **2 passed, 0 failed, 23 assertions, 1.79 seconds**; 296 tests were filtered out. These two tests prove current source authority and synthetic canonical validation, not post-move physical acceptance, a real Rust compile, the six fixture lifecycles, or a completed plan. No broader test pass is claimed.

The inspected main test file at the concluding read was 406,894 bytes, SHA-256 `fb26634094e4b78e540ed4db0e2992d0ca3a00168a08e36e2c6bcfdceb074413`; its unrelated regions changed concurrently. The historical package-identity test was 13,742 bytes, SHA-256 `ae97b516c14293e1dcee99d86595e86122c48bb4575712e057ac0c173ff69b02`. No source/test/schema/catalog file was edited by this audit.
