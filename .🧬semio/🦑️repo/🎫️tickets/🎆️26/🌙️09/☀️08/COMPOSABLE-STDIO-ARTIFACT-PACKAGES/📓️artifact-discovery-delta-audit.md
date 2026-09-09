# Artifact Discovery Delta Audit

Captured: 2026-09-09T16:30:35+02:00. Read-only manifest and source audit; no Nx, Cargo, or native task was run.


## Inventory delta

The current discovery set remains exactly the retained baseline: 99 Rust artifact owners, 99 unique Cargo package names, and 40 TypeScript declarations. Category counts remain 36 stdio, 15 Norm, 17 multi-artifact, 24 single-artifact, and 7 framework owners. There are no added or removed Rust or TypeScript owners.

The two current artifact-root source paths without a Rust package are the already-documented nested glTF JSON serializer/deserializer pseudo-artifacts. They are not new owners and were excluded from the 99-owner baseline.

## Current boundary result

No owner lacks its Rust manifest or package router; each current Cargo library retains `lib.path = "../../🦀️.rs"`. The package-local implementation scan remains empty. Root-source scanning found no `crate::artifacts`, `crate::registry`, or `semio_s_plugin_` import in any artifact root. All 36 stdio leaves remain selectable and have no direct stdio-parent or full-catalog dependency declaration.

The existing 99-owner gate map still covers every current owner: the current owner set has no member missing from it, and its retained `artifactsMissingAllPlannedOrAcceptedGates` list is empty. This establishes plan/coverage membership only; parent transitive compilation is not promoted to standalone leaf acceptance.

## Exact new dependency delta

One current manifest has changed since the prior static boundary receipt: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml` adds `semio-s-plugin-flow-extension-brep` and `semio-s-plugin-flow-extension-math` under `[dev-dependencies]`, for its explicit `example-geometry` test target. Both resolve to Flow extension packages, not the Flow parent plugin or an artifact catalog, and are unavailable to the artifact's normal production dependency set.

This is therefore not a production parent/catalog backedge or package-implementation leak. It does enlarge Generation3D's direct test closure. The existing multi-artifact gate map marks that owner as planned/partially attempted rather than currently standalone-accepted, so this new test dependency must remain covered by its eventual direct Generation3D native gate; Mathematical parent compilation does not accept it.

Raw comparison: `🗑️generated/artifact-discovery-delta-current.json`. The direct dependency scan is `🗑️generated/artifact-current-parent-dependency-hits.txt`.
