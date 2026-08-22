# P10g Plain Vitest Configurations

## Scope

This packet removes eight undeclared `vitest/config` imports from package-owned Vitest configuration files. Vitest accepts the exported configuration object directly, so each file now exports its existing object without importing or invoking `defineConfig`. No test behavior, manifest row, allowlist, or gate severity changed.

## Files

- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🧪️vitest.config.ts`
- `🧰️framework/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/◻2d/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️vitest.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️vitest.config.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️vitest.config.ts`

## Verification

The following Nx quick-test targets passed with the plain configuration objects:

- `@semio-tech/puzzle-5d-react:test-quick`: 1/1 test passed.
- `@semio-tech/framework:test-quick`: 87/87 tests passed.
- `@semio-tech/s-2d-js:test-quick`: 4/4 tests passed.
- `@semio-tech/framework-actor:test-quick`: 46/46 tests passed.
- `@semio-tech/ui-styling-tokens:test-quick`: command passed; the existing include pattern selected no test files.
- `@semio-tech/infinite-canvas-react-renderer:test-quick`: 1/1 test passed.
- `@semio-tech/plugin-window-kits:test-quick`: 8/8 tests passed.

The coordinator target remains blocked before Vitest starts by its pre-existing missing internal repo-library script import; this packet neither introduced nor changes that infrastructure blocker.

The canonical ownership-aware parity command was rerun:

```text
bun ./📜️script.ts verify dependencies parity js --format json
```

It reported the expected Phase 10 red exit with 83 manifests, 304 external rows, 142 evidenced rows, 162 unowned rows, and 46 undeclared imports. The immediately preceding checkpoint contained 54 undeclared imports, so this packet removes exactly eight genuine findings without suppressing or allowlisting them.

## Status

This bounded packet is green. Phase 10 remains open because the repository-wide zero-external-dependency and zero-parity-finding gates remain red.
