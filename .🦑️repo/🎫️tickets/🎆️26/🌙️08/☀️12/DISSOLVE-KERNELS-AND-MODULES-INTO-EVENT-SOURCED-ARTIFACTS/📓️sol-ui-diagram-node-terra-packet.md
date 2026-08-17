# Terra Packet UI-DiagramNode-01: Zero-Consumer Dissolution

## Preconditions

- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Read root and applicable nested `AGENTS.md`; use `apply_patch` only and no modifying Git.
- Rehash every writable path and verify its relevant source region is stable before editing.
- Required definition hashes:
  - component: `3f0fd02b9a2236f72a631e783dca9ebd1e63f261635a12d1cae7306b139106f4`
  - exclusive story: `aafd1ffbf1730ac5e7a1133daef362b144b9d6f077c0f074165feaba8378a85c`
- Shared React index baseline for the audit: `01005e76dbc844cbaa2e9c8b2e6b7727bfd3d575f7ef887e62c3f1ce249c4a52`. The coordinator will provide its serialized current hash; Terra must never edit it.

## Terra Writable Closure

1. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️DiagramNode/🟦️component.tsx`
2. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️DiagramNode/🧪️story.tsx`
3. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️story.tsx`
4. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️story.tsx`
5. Unique acceptance record `📓️terra-ui-diagram-node-zero-active-consumer-dissolution-acceptance.md`.

## Required Source Change

- Delete the DiagramNode component and exclusive story.
- Remove only DiagramNode imports and DiagramNode-specific visual story blocks from the Canvas and Diagram stories.
- Preserve their `DiagramSkeleton` and every unrelated story.
- Do not touch the OS renderer's unrelated `WorkflowDiagramNode`.
- Do not add a wrapper, alias, replacement, compatibility export, or module.

After the source checkpoint, wait. The coordinator alone removes the exact `DiagramNode` region from the shared React index and supplies its new SHA-256.

## Validation

After registrar signal, prove the component directory is empty and active-source stale scans for `DiagramNode`, `PlaceholderDiagramNode`, `DiagramNodeProps`, direct source paths, imports, and JSX consumers are clean except for semantically unrelated local identifiers explicitly classified in the acceptance record. Run scoped ordinary/cached diff checks, then:

1. `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`
2. `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`
3. `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`
4. `bun nx run @semio-tech/ui-react:build --skip-nx-cache`

Record exact results and do not repair unrelated broad UI drift.
