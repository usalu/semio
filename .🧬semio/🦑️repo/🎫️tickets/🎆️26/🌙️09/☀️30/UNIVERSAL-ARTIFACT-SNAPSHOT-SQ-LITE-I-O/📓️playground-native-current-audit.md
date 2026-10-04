# Playground Native Current Audit

Read-only source audit on 2026-10-02. Paths were validated with `rg --files`. No Cargo or other test command was run; fresh runtime green remains unverified here. Repo MCP was unavailable to this audit; no ticket lifecycle claim is made.

## Mounted Implementation

The actual owning `PlaygroundSnapshot` has exactly one `schema: String` field. Its snapshot module mounts `🪶️sqlite/🦀️.rs`, and `ArtifactPack::sqlite_snapshot_codec` returns the typed owner codec. Root `declaration()` uses `document_codec_bare::<PlaygroundSnapshot, PlaygroundMutation>` with the owned dialect. The erased codec records `TypeId::of::<PlaygroundSnapshot>()`; the owning Rust law checks that identity.

The authored SQL declares only `playground_document(id INTEGER PRIMARY KEY, schema TEXT NOT NULL)`. Projection uses a literal text cell. Reconstruction checks the authored database schema and aggregate limits before copying, requires one row with a positive identity matching the first integer cell, and copies only its text field. Empty, Unicode, NUL, and JSON-looking strings undergo no JSON carrier interpretation. Any positive surrogate singleton identity is accepted.

`admit` checks cancellation, authored schema bytes, and the required singleton row limit before native decode/encode copies and before project/reconstruct work. Projection charges eight bytes for identity plus literal text bytes before allocation. Reconstruction first checks full database value bytes and retains scalar accounting; controlled reconstruction adds the string allocation to that identity budget. Native input/output use one cumulative native controller across record specification, physical parser/emitter, typed binding, and envelope work. All four phases use controlled helpers; long text project/reconstruct copies have interior checkpoints. The owner explicitly retires its sole String by dropping it, which is safe for this nonrecursive shape.

The semantic validator strictly requires artifact `s.demonstrator.playground`, standard `1`, subset `*`, and checks projected text against the snapshot. It runs after decode/project or reconstruct, so a wrong dialect is rejected before successful output but does not itself provide an early pre-copy rejection.

## Owning Evidence And Gaps

Five owning Rust laws now cover capability, actual declaration routes in both directions, ordinary native literal preservation, independent Bun SQL queryability, and erased native binary/text SQLite roundtrips. The shared fixture includes empty, Unicode/NUL, and JSON-looking strings. The Bun corpus independently uses `bun:sqlite` and Ajv, verifies renumbered/edited real SQLite bytes, malformed row ownership and storage class, row/schema/value admission, and interior project/reconstruct cancellation.

Actionable validation gap: the freshly mounted Rust owner's five laws do not exercise negative dialects, externally renumbered positive identity, malformed/multiple/missing rows, schema/row admission before native copying, exact cumulative byte boundaries, or interior cancellation for the actual owner in each of DecodeNative, ProjectSnapshot, ReconstructSnapshot, EncodeNative. Bun coverage proves TypeScript project/reconstruct behavior, while generic controller tests prove shared helpers; neither establishes these Rust owner frontiers directly. Add owner-level tests for those laws before claiming full native acceptance. No definite production defect was found in the inspected provider.
