# UI Shell Scope Multiconsumer Retention Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🟦️component.tsx`
- Source SHA-256: `2244253e260f825710bd9a87dde0001acb889e9d3f0dbfbc3db4134dbeeb8734`
- Source state: clean, 233 lines
- React barrel SHA-256 at audit: `0f8def42b5703b2ab00bd31f6e7b242e334ea9f60fdd9a5d35c1a88fdf8fa401`

## Owned Responsibilities

- `SelectionModeStore`
- `ShellScope`: per-shell root and portal references, storage, page ownership, scoped queries, selection mode, and per-shell localization
- `createShellScope`, provider, required and optional context hooks, and storage fallback
- shell activity-root registration and active-root observation
- shell-gated document keydown and inert-root fallback

These responsibilities form one coherent per-shell isolation boundary. The activity and keydown subregions enforce that boundary and are not independently reusable production capabilities.

## Independent Production Consumers

Framework UI terminals:

- `🪟️Window`
- `🖼️Panel`
- `🎨️Canvas`
- `🪵️Tree`
- `🖱️ContextMenu`

Protected OS renderer terminals:

- `ShellHost`
- `ShellHelpers`
- `Board2dHost`
- `NodeGraph`
- `Paint2dHost`
- `Table`
- `TextEditor`
- `TiledMapHost`
- `World3dHost`

The OS renderer package index and framework React barrel are assembly/glue rather than terminal consumers. `TestShellRoot` is test-only and excluded from the production consumer count.

## Disposition

Retain `🐚️ShellScope` at the framework UI owner. Its reverse dependency closure contains multiple independent framework and protected-renderer production components, and the implementation is already at their lowest common semantic owner. No extraction, inlining, deletion, or Terra implementation lease is justified.
