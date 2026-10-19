# Domain TOML Source Audit — Repeat 695

Source-only parsing using the Python standard library; no repository executable, Cargo, build, test, or dependency resolver was run.

Parsed 83 Cargo source manifests/lock files.

## Parse Errors

- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📦️packages/🦀️rust/Cargo.toml`: Cannot declare ('features',) twice (at line 42, column 10)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/📦️packages/🦀️rust/Cargo.toml`: Cannot overwrite a value (at line 39, column 122)

## Duplicate Locked Package Identities

- `🎓️teaching/Cargo.lock`: `('semio-framework-dsl', '0.1.0', None)`
- `🎓️teaching/Cargo.lock`: `('semio-framework-dsl-record', '0.1.0', None)`

## After Source Merge Repairs

Consolidated the Energy sim feature tables, deduplicated an identical Puzzle mutation dependency, and removed two identical duplicate Teaching lock package blocks. Reparsed 85 source manifests/locks: 0 parse errors and 0 duplicate locked package identities. No resolver was executed.
