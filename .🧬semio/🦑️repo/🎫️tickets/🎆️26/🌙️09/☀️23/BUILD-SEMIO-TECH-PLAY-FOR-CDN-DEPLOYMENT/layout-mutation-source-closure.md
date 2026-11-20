# Layout mutation release source closure

The release preflight measured the Layout mutations library above the 262,144-byte Rust source limit. The live tree already contained a separate retained-fields library with its member, dependency, and lock entries in the dedicated Layout Cargo workspace. The production source reader still measured the mutation library at 25 files, 281,993 bytes, and 2,281 active lines.

The retained-fields library now owns story splice diff publication and exact inverse preparation. It accepts the existing borrowed mutation view and typed payload port. Both the original story output and concrete inverse remain available through their existing Layout mutation paths. The patch changes the type boundary and leaves the story schema, phase transitions, and budgets intact. It uses the existing Layout dependency and lock entries.

The production Rust source reader, with browser target configuration, measured the mutations library at 23 files, 258,274 bytes, and 2,047 active lines. It measured the retained-fields library at 15 files, 175,598 bytes, and 2,000 active lines. Both satisfy the release limits of 32 files, 262,144 bytes, and 4,000 active lines. The dedicated workspace members and lockfile edge resolved on the live tree. All three existing TypeScript story fixture cases passed before and after the change. The Rust behavioral law and full release are pending separate execution.

Rust files changed, relative to the Layout mutations directory

- `🦀️.rs`
- `🧳️retained/🏗️fields/🦀️.rs`
- `🧳️retained/📖️story/🦀️.rs`
- `🧳️retained/📖️story/🔺️output.rs`
- `🧳️retained/📖️story/↩️inverse.rs`

## Release 61559 review

The production Rust scanner with `wasm32` browser configuration measured the live mutations library at 23 files, 258,274 bytes, and 2,047 active lines. It measured the retained-fields library at 15 files, 177,593 bytes, and 2,031 active lines. Both satisfy all three limits. The scoped preflight reported no over-budget libraries. The dedicated Layout workspace and lockfile resolve the parent-to-fields dependency.

Static review traced the borrowed story view, both typed payload implementations, the concrete inverse alias, and retirement bounds through the relocated modules. All three shared story fixtures passed against the TypeScript implementation and fast-json-patch oracle. The Rust law remains registered under `layout-mutation-laws`. Compiler and Rust law validation are pending the release run.
