# Tree Window Row Extent

## Contract

Virtual Tree windows now carry a required closed-row extent token:

- `standard` — 24 px
- `compactText` — 14.4 px
- `compactSmallControl` — 16 px
- `compactControl` — 22.4 px

The language-neutral fixture and JSON Schema are:

- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-window-row-extent/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🌳️tree-window-row-extent/🔣️.json`

A direct virtual row header and both absent-row spacer bands use the declared extent. Expanded descendant height remains measured separately. Finite unwindowed compact trees remain variable-height.

## Fail-first evidence

The real React Tree component oracle was run before production support:

- receipt: `🗑️generated/astra-runtime/react-tree-window-extent-red/run.log`
- result: 1 selected failure, 32 skipped
- actual failure: `standard-text` expected a declared extent DOM token but received `null`

## Implementation

The required field is propagated through the Rust UI contract, typed/copy/compare/retirement metadata, generated schema metadata, WGPU mirror and reconcile bridge, Shell projection, and plugin producer. Standard producers name `Standard` explicitly.

React resolves each declared extent from the canonical styling metrics, stamps it on the window container, uses it for leading and trailing spacers, fixes materialised row headers to the same extent, and lets viewport math use each container's own extent.

## Verification status

- All touched core Rust files parse under `rustfmt --edition 2021 --emit stdout`.
- Root Native148 and UI32 were launched after the Rust coherence boundary.
- The same real React component filter is green: 1/1 passed, 32 skipped; Vitest 3.83 s and Nx 5.4 s.
