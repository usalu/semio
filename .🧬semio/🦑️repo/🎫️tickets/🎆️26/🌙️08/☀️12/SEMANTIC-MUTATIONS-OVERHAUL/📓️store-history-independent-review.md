# Store Space-History Independent Review

## Scope

This review reads the six direct Store history leaves, their descriptors, aggregate root,
`SpaceHistoryDiff`, and Store's existing codec/inverse integration. It creates only the
ticket-local neutral law matrix and public kernel client. It makes no Store production edit,
does not run Cargo, and does not read or materialize `compose`.

The six direct leaves are exactly `CommitSpaceCheckpoint`, `CreateSpaceAlternative`,
`SwitchSpaceAlternative`, `RemoveSpaceCheckpoint`, `RemoveSpaceAlternative`, and
`RestoreActiveSpaceAlternative`. The aggregate mounts and reexports each exact owner.
The frozen expected descriptor matrix distinguishes the two plan inverses from the four
explicit-mutation leaves: `CreateSpaceAlternative` and `RemoveSpaceAlternative` are `plan`;
all six are `apply-only`, require `rust`, `json-schema`, `text`, and `binary`, use the physical
`🧬️schema/🔣️.json` payload schema, and leave the generic-codec selectors null. The static matrix is retained at
`🧪️store-history-independent-review/🛂️laws.schema.json` and
`🧪️store-history-independent-review/🧫️fixtures/🔣️laws.json`.
The latest static replay is green in
`🧪️store-history-independent-review/🧪️metadata-per-leaf-green.log`: it validates both the
neutral law schema and the repository descriptor schema, then parses every descriptor-referenced
payload schema at its exact physical leaf-relative path.

## Existing Integration

`SpaceHistoryMutation` is a serde tagged aggregate (`operation` and `payload`, camel case,
unknown fields denied). Store owns generic text through serde JSON and binary through the
actual DslValue/pack codec; no copied codec or synthetic aggregate was used. The inverse
executor reverses every leaf's returned vector before applying it. Therefore leaf inverse
vectors must be returned in stored order, not execution order.

The existing Store test `space_history_op_round_trips` exercises commit, create, switch,
remove-checkpoint, and restore-active. It omits `RemoveSpaceAlternative`, which is precisely
the uncovered inverse boundary below. The independent inverse client covers named sole or last
removal cases only; ordered removal/restoration for an earlier position is retained as an open
semantic inventory item and is not implied by this scoped regression.

## Intermediate Ordered-Inverse Finding

The ticket client calls only the public current-kernel API: `From` conversions, `MutationLeaf`
metadata/provenance selectors, `SemanticMutation::kinds`, `Mutation::diff`/
`MutationDiff::apply`, `Mutation::inverse`, `OpText`, and `OpBinary`. Its inverse executor
matches Store's order: it reverses each returned inverse vector before applying it.

Against the first new current-contract kernel artifact, client compilation and listing passed;
metadata/From/all-six text-and-binary round trips passed. The inverse law then failed only for
removing an inactive alternative. Starting with active `alt-1` and removing `alt-2`, the current
inverse creates `alt-2`; creation activates it, so the restored snapshot has active `alt-2`
instead of `alt-1`.

The retained real-client evidence is
`🧪️store-history-independent-review/🧫️run-lIcuoz/`. Its compile and list have status zero;
the runtime has status 101 solely from this exact failed law, and its source/kernel hashes are
stable. That artifact is intentionally marked intermediate: Store corrections after its build
require a final rebuild and replay before any readiness claim.

The direct source implication is that `RemoveSpaceAlternative::inverse` needs both the
recreation and a restoration of the prior active alternative, arranged so reverse execution
creates first and restores active state second. This intermediate finding was reproduced rather
than inferred; its designated owner has since applied a source correction, which requires a final
kernel rebuild and client replay before it can be considered closed.

## Final Scoped Current-Contract Replay

After the root-released, source-stable current-contract kernel build, the independent client
compiled, listed, and ran successfully. The retained final run is
`🧪️store-history-independent-review/🧫️run-Pcrs7G/`: all three child processes have status zero,
no signal, and no spawn error; the client binary SHA-256 is
`7de991e778b0f656cda2d1d94f39dd66f6a33d41884c217bccdd0ffe4ad3bf0b`; source and both genuine
kernel artifacts remained byte-stable before and after execution. Its two public tests cover:

- all six `From` conversions; text and binary serde/pack round trips; required-nullable and
  unknown envelope/payload rejection; each leaf provenance; all six physical payload-schema
  paths; and the six-entry `Mutation::DESCRIPTORS` roster/order;
- each active aggregate variant's `descriptor()` equality with its corresponding leaf descriptor
  (the complete fourteen-field value), including the two `plan` values, `apply-only`, null generic
  selectors, and the four required language surfaces;
- the named last-or-sole inverse cases and nullable restore case through the genuine public
  `Mutation`, `MutationDiff`, and inverse APIs.

This closes only the required-metadata adoption and named public inverse/codec regression. It is
not a finding that every historic SpaceHistory behavior, native codec ownership, or global
language parity is complete.

## Additional Risks

- Removing the currently active alternative leaves `active_alternative_id` pointing at a removed
  id because `SpaceHistoryDiff::apply` retains the active field unless the diff explicitly sets
  it. No `SpaceHistorySnapshot` invariant check currently rejects that dangling state. This is a
  open semantic inventory item separate from the ordered-removal restoration correction.
- Removing a checkpoint does not reject or repair alternatives whose `checkpoint_ids` still
  reference it. Whether that relation is intentionally soft is not stated by the mutation
  contract, so this remains an open audit inventory item rather than a claimed defect.
- The metadata correction is reflected by the latest static gate. This does not claim native-codec
  ownership or global language-surface parity; the public runtime replay remains pending the
  root-released final current-contract kernel artifact.

## Reproduction

The current intermediate command is below; it does not use an old-contract target. A final replay
must use the root-released rebuilt artifact directory.

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-history-independent-review/📜️script.ts runtime
```
