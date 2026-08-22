# Phase 10 Owned Story Schema

## Scope

This packet removes the UI element-story cohort's direct Storybook type dependency. It changes the shared UI story schema and 39 co-located UI element stories only. No manifest, lockfile, allowlist, generated fixture, renderer Rust source, or dependency-gate severity changed.

## Implementation

`framework/modules/ui/story.ts` now owns the component-fixture `Meta` and `StoryObj` schema used by the UI element stories. Its types infer component properties, story arguments, render properties, and the asynchronous browser play context without exporting any external type.

All 39 UI element stories now import the owned schema through their co-located `../../🧪️story` path instead of importing `@storybook/react-vite`. The external Storybook renderer remains an exporter/consumer at the root integration boundary; domain stories no longer require its types.

## Verification

```text
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache
```

Exit `0`.

```text
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
```

Exit `0`.

```text
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --reporter=verbose
```

Exit `0`: 1 test file and 533 tests passed.

```text
bun ./📜️script.ts verify dependencies
```

Exit `0`: 185 current third-party identities against the 238-entry baseline, 53 removals, and no additions.

```text
bun ./📜️script.ts verify dependencies parity js --format json
```

Expected Phase 10 red exit: 83 manifests, 308 external rows, 142 evidenced rows, 166 unowned rows, and 159 undeclared imports. This cohort removed exactly 39 undeclared imports from the preceding 198-count checkpoint without suppressing a finding.

## Closure State

This packet is green. Phase 10 remains open because the repository-wide zero-external-dependency and zero-parity-finding gates remain red.
