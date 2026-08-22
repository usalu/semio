# Phase 10 Owned Artifact Test Boundary

## Scope

This packet removes direct Vitest imports from every generated or co-located `✏️s/**/🧪️tests/🟦️test.ts` source. It changes 121 test files across 32 plugin roots and adds the repository-owned `🧪️artifact.ts` test boundary. No manifest, lockfile, allowlist, root script, renderer Rust source, or Puzzle 3D runtime source changed.

The cohort contains 120 artifact-tree tests and one Space engine example test. Together they retain 149 assertions: 138 `toBeGreaterThan`, seven `toBe`, and four `toContain` checks.

## Implementation

`🧪️artifact.ts` is the root test-runner integration boundary. It wraps Vitest's `describe`, `it`, and `expect` functions behind repository-owned function signatures and an `ArtifactExpectation` interface. No Vitest type is exported to artifact test sources.

All 121 co-located tests now import that owned boundary through a relative path. This leaves the external runner coupled only to the root integration boundary while preserving suite registration and assertion semantics.

## Verification

```text
bun x tsc --noEmit --module preserve --moduleResolution bundler --target esnext --types node --skipLibCheck 🧪️artifact.ts <all 121 migrated tests>
```

Exit `0`: the complete migrated cohort resolves and type-checks.

```text
bun x nx run @semio-tech/puzzle-js:test-quick --skip-nx-cache -- --reporter=verbose
```

Exit `0`: nine artifact test files and 15 assertions passed through the owned boundary.

```text
bun ./📜️script.ts verify dependencies
```

Exit `0`: 181 current third-party identities against the 238-entry baseline, 57 removals, and no additions.

```text
bun ./📜️script.ts verify dependencies parity js
```

Expected Phase 10 red exit: 83 manifests, 304 external rows, 142 evidenced rows, 162 unowned rows, and 54 undeclared imports. The immediately preceding checkpoint recorded 159 undeclared imports, so this packet removes 105 parity findings. The remaining 16 migrated Vitest imports were already declared by their owning package and therefore did not contribute findings; all 121 direct source imports are nevertheless gone. Concurrent dependency-removal work changed the external-row and unowned-row totals and is not attributed to this packet.

## Closure State

This bounded packet is green. Phase 10 remains open because the repository-wide zero-external-dependency and zero-parity-finding gates remain red.
