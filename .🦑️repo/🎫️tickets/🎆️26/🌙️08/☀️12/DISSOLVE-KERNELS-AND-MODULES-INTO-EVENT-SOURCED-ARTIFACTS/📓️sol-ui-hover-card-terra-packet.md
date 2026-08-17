# Terra Packet UI-HoverCard-01: Zero-Consumer Dissolution

## Preconditions

- Read root/applicable `AGENTS.md` and the zero-consumer audit.
- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- Use `apply_patch` only and no modifying Git.
- Rehash Terra-owned paths and require:
  - component `58206cb6ee14e1b3bca4ac75a1e8b95b0f2caf1dd1347f78a6f1a0f23a8250c4`
  - story `4d7e61976fbaadbbe16600edf6d6a2be510679a4ef9fe16f77315e01743a2905`
- Verify current shared React index hash announced by the coordinator; never edit it.

## Terra Writable Paths

1. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️HoverCard/🟦️component.tsx`
2. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️HoverCard/🧪️story.tsx`
3. Unique acceptance record `📓️terra-ui-hover-card-zero-active-consumer-dissolution-acceptance.md`.

## Change and Handshake

Delete the component and exclusive story. After confirming both absent and the directory without authored files, report a source checkpoint and wait. The coordinator alone will remove the shared index's complete HoverCard region and its package-level Radix namespace import, then provide a new hash.

Do not touch package manifests, bun.lock, generated census output, Storybook configuration, other UI leaves, or protected renderer paths.

## Validation

After registrar signal, run active-scope stale scans for all three exported identifiers, direct paths/imports, and JSX consumers; classify exclusions. Run scoped ordinary/cached diff checks and registered UI React lint, typecheck, test-quick, and build targets once without repairing unrelated failures. Record exact results.
