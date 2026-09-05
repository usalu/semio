# Typesetting and Interaction Review

## Scope and decisions

Hand-reviewed the complete `🧰️framework/🔨️modules/🔤️typeset` and `🧰️framework/🔨️modules/🕹️interaction` trees, their source headers, schemas, package source mounts, and task configurations. No nested `AGENTS.md` applies. No file or directory rename is needed.

The typesetting root is accurately identified by 🔤: it exposes a first-party markup-to-SVG and SVG-to-owned-geometry interface around Typst/usvg. Its 📦 package collection and 🦀 Rust package have distinct siblings; literal `Cargo.toml` is reserved, and the existing `📋️project.json` is the exact Nx filename authority. The single Rust source leaf is format-identifying, not a repeated sibling palette.

The 🕹 interaction root owns hover/selection definitions and the TypeScript state machine. Its 🧬 schema directory contains distinct Rust, TypeScript, GraphQL, and JSON leaves. These names distinguish the actual implementation/schema formats and do not collide across sibling files and directories.

## Exact edits

Only two stale Rust documentation references were changed: the interaction root `🦀️.rs` and its `🧬️schema/🦀️.rs` now identify `semio-framework-replication` and `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` as the current runtime owner. Read-only inspection confirmed `next_selection`, `next_hover`, and `PresenceInteraction` there. No API, implementation, assertion, payload, or file identity changed.

## Verification

The current taxonomy audit inspected all 14 physical entries: 7 typesetting entries (6 governed) and 7 interaction entries (7 governed), with zero path-emoji findings and zero unresolved directory roles. Evidence: `🗑️generated/metabolism-glb/typeset-interaction-audit.json`.

The fresh native typesetting command passed through Bun/Nx with the ticket-local Cargo target directory: `bun x nx exec --projects=workspace -- cargo test -p semio-framework-typeset --lib`. All six tests passed after a 10m02s dependency build. They cover actual text compilation, extracted outlines, invalid input, and a hand-authored language-neutral square SVG with exact dimensions/transformed points through the Typst/usvg implementations. No tests were ignored or filtered. Log: `🗑️generated/metabolism-glb/typeset-native.log`.

No independent broad interaction suite was rerun for these documentation-only edits. Naming completion is separate from runtime verification. No Git operation, bulk rewrite, cleanup, or generated-source mutation was used.
