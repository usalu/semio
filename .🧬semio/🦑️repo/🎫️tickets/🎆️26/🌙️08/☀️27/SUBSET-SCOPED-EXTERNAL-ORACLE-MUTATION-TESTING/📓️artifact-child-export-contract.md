# ArtifactChild Export Contract

## Decision

An exporter may read child-backed semantic content only from a materialized child owner supplied at the artifact boundary. If that owner is absent or has the wrong type, the exporter must return `IoError`. It must never serialize the parent handle as if it were the child content and must never substitute an empty/default scene.

`IoFidelity` describes successful conversions. A serializer that emits every semantic field after materialization remains `Exact`; missing source content is an availability error, not a lossy conversion. A deliberately partial carrier such as mathematical CSV remains `Lossy`, but it still must fail when even its required child-backed projection is unavailable.

## Boundary

`ArtifactChild.local_owner` is deliberately local-only and serialization-skipped. The generic `serializer_entry` currently decodes only the parent pack, so it cannot reconstruct a child scene: neither child envelope bytes nor a child resolver are in its input. Existing functions such as `mathematical_graph_geometry_from_children` are the correct reconstruction functions once the host has resolved the actual child snapshots; they cannot discover those snapshots from `{childId, target}` alone.

Therefore materialization belongs before serializer invocation, at the host/composition boundary that owns child stores. Exporters enforce the precondition and provide a precise failure instead of inventing data.

## Applied Rules

1. Child-backed JSON carriers contain semantic projections, not `ArtifactChild` handles.
2. Exact JSON import reconstructs new child handles with attached local owners.
3. Accessors used by exporters expose absence explicitly (`Option`/`Result`); they do not default.
4. A decoded parent pack without resolved children is rejected.
5. Oracle registration is permitted only after the materialized export and import path has executable tests.

## Registry Consequence

The current binary `serializer_entry` path necessarily strips local owners while decoding the parent pack. Under this contract, that path will correctly return `IoError` for child-backed snapshots until the host resolves and attaches children before typed serialization, or the erased IO contract is extended to carry resolved child inputs. Claiming `Lossy` would not repair that missing input.
