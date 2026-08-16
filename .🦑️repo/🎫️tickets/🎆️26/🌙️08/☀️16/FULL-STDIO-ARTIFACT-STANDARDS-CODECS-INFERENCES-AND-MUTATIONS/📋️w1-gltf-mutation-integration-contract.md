# GLTF Mutation Integration Contract

The authoritative inventory contains 222 commands. This count follows the literal named rows; it does not drop a semantic command to retain the earlier erroneous 221 arithmetic.

## Public envelope

The integrated mutation surface is a versioned semantic envelope, not a closed Rust enum with one match arm per command:

- `command_id`: canonical `s.stdio.gltf.mutation.<semantic-slug>.v1`;
- `payload`: canonical JSON bytes validated and decoded only by the owning command leaf;
- `base_revision` and `base_generation`: required optimistic-concurrency coordinates;
- `policy`, budgets, cancellation, and provenance fields from the framework mutation request.

There is no default/no-op command, whole-snapshot replacement, legacy numeric tag, compatibility alias, or generic `Set*` variant.

## Leaf descriptor

Each physical command leaf contributes one immutable descriptor with:

- identity/version and typed facet sources;
- payload decode/encode and validation function pointers;
- direct sparse forward-diff planner;
- inverse reconstruction from the exact base;
- touched-path derivation;
- indexed-reference repair/rejection policy;
- executable Rust and TypeScript identities;
- deterministic law fixtures.

The descriptor registry rejects duplicate IDs and mismatched idempotent registrations. A command is added by adding its leaf and one manifest member; no central payload inspection or behavioral match is changed.

## Root responsibilities

The mutation root may only re-export public contracts and assemble the immutable descriptor registry/manifest. It cannot:

- match on command IDs or variants;
- decode command-specific payload fields;
- calculate diffs/inverses/touched paths;
- repair references;
- clamp, ignore, or convert typed rejection into an empty diff;
- retain the legacy 28-command tags or text/binary switch statements.

Text and binary transports encode the generic canonical envelope and delegate payload bytes to the selected leaf descriptor. GraphQL, JSON Schema, Proto, and grammar assemblies are derived from the same manifest and preserve the exact command-specific payload schemas.

## Diff boundary

The public artifact diff is an ordered generic envelope set whose entries contain canonical command ID, schema version, canonical typed-diff payload bytes, and exact touched paths. The owning leaf descriptor decodes, applies, inverts, validates, and absorbs its typed diff; the root never inspects payload fields. This avoids adding one field or enum arm to a monolithic `GltfDiff` whenever a command is added.

Weak whole-record collection diffs for buffer views, textures, images, samplers, skins, animations, and cameras are removed from the semantic mutation path. Leaf planners directly construct the smallest command-specific field/collection diff; computing a whole candidate snapshot followed by `between(base, candidate)` is not accepted as direct planning. Cross-command absorb keeps a deterministic ordered sequence; same-command absorb delegates to the leaf's typed algebra.

## CQRS laws

Every descriptor must pass acceptance/rejection, direct sparse diff, apply, inverse, absorb, replay, undo/redo, touched-path, serialization, stale-revision, reference-integrity, budget, cancellation, and deterministic remote-ingest laws. Reset/checkpoint remains the only whole-document replacement lane outside semantic history.

## Current integration blockers

- Command shards are still being implemented and must not be mounted until placeholder/no-op and post-hoc `between` scans are zero.
- The current `🔨️modules/🧭️mutation-dispatch` is a closed 28-variant enum with central matches and must be replaced, not wrapped.
- The current monolithic diff schema uses weak whole-record replacements for several nested families and must be removed from semantic command execution in favor of the leaf-owned diff envelope described above.
- Shared framework mutation-outcome work currently causes unrelated compile errors; the combined gate waits for that concurrent owner to stabilize without overwriting their changes.
