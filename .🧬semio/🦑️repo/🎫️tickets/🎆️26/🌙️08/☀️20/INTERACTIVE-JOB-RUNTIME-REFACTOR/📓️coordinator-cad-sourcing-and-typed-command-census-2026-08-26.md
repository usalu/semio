# CAD, Sourcing, and Typed Command Census

## Scope

- Reconciled the exact owner-local bounded-first-step proof catalogs for CAD and Sourcing with per-action `Migrated` declarations.
- Removed the broad `.interactive_jobs(Migrated)` declarations from both manifests.
- Added exact per-action declarations for all 39 CAD and 14 Sourcing proof-backed routes.
- Removed the two obsolete Procedural3d response commands `flowEvalResolve` and `flowTessellateResolve` from the public typed command surface. The current actor ABI correlates extension completions by `RequestId`; these legacy commands had no live response-action producer.
- Extended `verify interactivity tool-jobs` discovery to enumerate typed `*_command_variants!` surfaces, including constant-backed route IDs, without accepting unresolved copied constants.

## Verification

Before the typed-variant census change, the full tool-job report wrote successfully with:

- command rows: 774
- unique command rows: 772
- accepted bounded rows: 232
- remaining rows: 651
- failures: 99
- self-tests: 430

That run proved only that the CAD and Sourcing declarations were joined to their owner-local classification proofs. It did not prove executable runtime reachability: the production dispatcher rejects `QualifiedToolProof::Bounded` before preparation unless the app also registers an exact app-owned factory and supplies its retained builder.

The expanded census adds two hostile verifier laws:

1. a constant-backed typed-command variant with an exact proof/declaration is accepted;
2. an unresolved typed-command constant is not invented as a route.

The taxonomy edit completed and the fresh expanded census ran. The verifier was then hardened so annotation-only bounded proofs cannot enter `acceptedCommandRows`: an executable route now requires its exact `ArtifactOwnedToolJobFactory`, live `registry.register(...)` call, owner-local `build_tool_job` implementation, migrated disposition, and proof identity. Three hostile laws reject proof-only, unregistered-factory, and missing-builder routes.

Fresh result after that correction:

- command macro rows: 774
- unique command macro rows: 772
- literal/typed production rows: 939
- exact app-owned factory tool registrations discovered: 139
- executable accepted rows: 128
- fail-closed rows: 811
- failures: 89
- verifier self-tests: 449, clean

The census was then corrected to accept any concrete app-owned factory name rather than silently
hard-coding `BoundedFirstStepCommandJobFactory`. A fresh live run on 2026-08-26 now records:

- command macro rows: 774
- unique command macro rows: 772
- literal/typed production rows: 939
- exact app-owned factory tool registrations discovered: 139
- executable accepted rows: 138, including Draw's 6 concrete gesture routes
- fail-closed rows: 805
- failures: 89
- verifier self-tests: 449, clean

The live JSON ledger was regenerated at `📊️all-tool-job-coverage-live-2026-08-26.json`. CAD and Sourcing therefore remain fail-closed despite their classification catalogs; they still require app-owned retained reducer jobs. This is the intended truthful red gate.

## Files

- `📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
