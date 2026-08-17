# Terra Packet: Virtual File System Table Import

## Objective

Correct VirtualFileSystem's runtime `Table` import from a Storybook metadata file to the authored Table component. This establishes the intended independent production consumer and removes a direct-generated/example boundary violation.

## Lease and Baseline

- Writable source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📁️VirtualFileSystem/🟦️component.tsx` only.
- Unique ticket acceptance Markdown.
- Baseline SHA-256: `3c1ce5cfc96b49967d1f9a1050fea59f0d91385e7198bc7b9b1857aabd9c7540`.
- Table component SHA-256: `d8de6cc8375fd4856e5cd8f5a45a01ee7665a0c52cb945e986eb4c346de2ccb3`.

Change only the `Table` value import from `../🦴️Skeletons/🧪️story.tsx` to `../📊️Table/🟦️component.tsx`. Preserve existing Table type imports and all runtime behavior. Do not edit stories, Table, barrels, products, generated output, manifests, or locks.

## Gates

- VirtualFileSystem has zero production import from any story/test file.
- Table value and types resolve from the authored component.
- Scoped ordinary/cached diff-check pass.
- Run UI lint, typecheck, and test-quick once through Nx with `--skip-nx-cache`; record exact results and final SHA without unrelated repairs.
