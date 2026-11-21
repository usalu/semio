# Native Flow Directional Channel Interaction

## Contract

Flow pick handles now add a `channel` value with raw `widgetId`, raw `port`, and `direction` (`in` or `out`). Generic pick consumers still receive `domain`, `id`, `generality`, and an optional label. The DAG checks the physical handle’s node and role for both direct handles and row picks.

Flow WASM accepts a third optional `direction` parameter for `setHoverChannel` on both positional and object calls. The ABI attributes generate a literal union in the browser declaration. Rust refuses any other direction string before applying hover. The two-argument Rust helper still supports channel-row hover for name-only callers. Directed hover can focus a physical connector at other connection-picking zoom tiers.

Typed selection resolves each direction explicitly, keeps node and edge selections, and clears only handle selection for an empty list. The renderer loader exposes the existing typed channel read and write operations. Typed channel resolution also compares the physical owning widget after consulting the generic handle key, so a shared separator cannot resolve another widget.

## Verification

- The Flow ABI oracle passes with Bun and Ajv against 109 operations and eight events. The test’s expected-fixture path was corrected to point to the existing fixture.
- Ajv validates the browser-type fixture, and the TypeScript parser validates the generated positional and object `setHoverChannel` declarations.
- The native regression uses an authored Variable with input and output both named `vector`. It exercises Flow snapshot resync, Detail row picks, Micro connector picks, a Normal connector pick, hover, channel selection round trip, and clearing channels while retaining node selection. The independent serde_json oracle builds and compares the entire handle row from the language-neutral fixture.
- The compact native witness passed all five cases against the same fixture. Run it from the workspace root with `repo tools exec --light python -- <ticket>/directional-flow-source.py`. It compiles the checked-in FlowHost methods and DAG lookup, row picking, hover, selection, and type declarations with pinned rustc and the real pack JSON implementation. The witness uses a small synthetic engine and geometry, compares complete pick rows with Python’s independent JSON decoder, and verifies physical hover direction, retention of node and edge selection, and rejection of a shared raw delimiter collision.
- The full native FlowHost Cargo test remains unverified. The relevant managed Cargo closure could not be admitted, and the remaining caches contain pack JSON and its dependencies without compatible FlowHost and DAG artifacts.
