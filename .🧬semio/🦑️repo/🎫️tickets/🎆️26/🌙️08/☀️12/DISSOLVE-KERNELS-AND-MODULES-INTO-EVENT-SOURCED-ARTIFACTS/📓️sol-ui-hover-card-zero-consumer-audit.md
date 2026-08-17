# UI Hover Card Zero-Consumer Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Component SHA-256: `58206cb6ee14e1b3bca4ac75a1e8b95b0f2caf1dd1347f78a6f1a0f23a8250c4`, clean.
- Story SHA-256: `4d7e61976fbaadbbe16600edf6d6a2be510679a4ef9fe16f77315e01743a2905`, dirty only from the accepted removal of Card examples/imports.
- Shared React index SHA-256: `6efa99283af7df14639d1f301456690d3d16860156ed8d2bf1087094a2bfc2fc`, dirty only from accepted serialized UI removals.

## Consumer Closure

The active-source closure contains only:

1. The `HoverCard`, `HoverCardTrigger`, and `HoverCardContent` definitions.
2. Their exclusive Storybook story, including the retained `Default` and `AsideNote` examples.
3. The mechanical shared React import/re-export region.
4. An otherwise-unused package-level `@radix-ui/react-hover-card` namespace import in the same mechanical index.

No active production component, direct source import, test, runtime mount, registry, or independent package consumer remains after Card dissolution. The dirty story is accepted owner-local history, not an independent production consumer.

## Decision

Delete the zero-consumer component and its exclusive story. Remove the exact React registrar region and the now-dead package-level Radix namespace import. Do not create a module, alias, wrapper, replacement, or compatibility export. Queue the direct package dependency and lock prune behind the same atomic Bun-regeneration blocker already documented for Accordion.
