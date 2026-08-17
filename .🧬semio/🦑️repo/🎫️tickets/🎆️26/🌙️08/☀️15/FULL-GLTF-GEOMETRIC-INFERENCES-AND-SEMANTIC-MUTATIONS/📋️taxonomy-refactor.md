# GLTF Inference and Mutation Taxonomy Refactor

## Problem

The complete geometry implementation was accumulated in `💡️inferences/📦bounds`, although bounds are only one size indicator. All fourteen independent indicator domains, shared measurement contracts, aggregate inference contracts, scene extraction, computational geometry, and tests consequently had the wrong owner.

The mutation root likewise owns every command payload, planning, validation, reference transport, text/binary codecs, inverse dispatch, and tests. Only `set-snapshot` has the repository-standard `🦠️mutation` / `🔺️diff` / `↩️inverse` triad, which falsely suggests that unrelated semantic commands belong to the general snapshot replacement command.

## Frozen Target

Inference ownership is split into shared `🧾️measure`, aggregate/orchestration `📐️geometry`, and one semantic component per indicator group: `📦️size`, `🧱️area-volume`, `⚪️compactness`, `📏️proportion`, `⚖️mass-distribution`, `🌀️curvature`, `↕️thickness`, `🕳️concavity`, `↔️clearance`, `🔗️adjacency`, `🧭️orientation`, `🪞️symmetry`, `🌊️roughness`, and `🕸️topology`. The obsolete `📦bounds` component is removed rather than retained as an alias.

Every GLTF mutation command receives its own named folder and the standard `🦠️mutation`, `🔺️diff`, and `↩️inverse` leaves. The root mutation component is limited to the closed command union and common application/rejection assembly. Cross-command reference repair is owned by `🧭️planning`; wire codecs remain in `📝️text` and `💾️binary`.

Wire mutation tags 0 through 27, serialized command names, inference schema version 2, canonical text/binary envelopes, and public geometric field names remain invariant. There is no legacy compatibility module.

## Verification

The refactor is complete only when no production path contains `💡️inferences/📦bounds`, every mutation union member has a command triad, module wiring resolves every semantic component, structural audits pass, focused GLTF tests pass, and the broad GLTF Nx gate passes.
