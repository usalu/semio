# Terra Packet UI-Breadcrumb-01: Zero-Consumer Dissolution

## Preconditions

- Read root/UI AGENTS and `📓️luna-ui-breadcrumb-zero-consumer-audit.md`.
- Apply patches only; no modifying Git commands.
- Require definition SHA-256 `d04e5bc47ca1495a6f20f01dc556ff42979ec9be3da7d2fd5aad0dac2e546828` and accepted-dirty story SHA-256 `45ad6a6112a6f5de152f75b0114ec15641a41661c0796c483b8d93265b81a154`.
- Shared React barrel is coordinator-owned; never edit it. Expected hash will be supplied at dispatch.

## Terra Writable Closure

1. Delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🍞️Breadcrumb/🟦️component.tsx`.
2. Delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🍞️Breadcrumb/🧪️story.tsx`.
3. In `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🎨️ui.css`, remove only selectors whose exact `data-slot` is `breadcrumb`, `breadcrumb-link`, `breadcrumb-item`, or `breadcrumb-separator-control`; preserve every shared selector and declaration used by other components.
4. Write unique acceptance `📓️terra-ui-breadcrumb-zero-active-consumer-dissolution-acceptance.md`.

Stop after source/CSS checkpoint. Coordinator will remove the barrel import/export and the two Breadcrumb-only test cases, then signal final scans/gates. Do not edit Storybook central files, package manifests/locks, generated census, plugins, or protected renderer.
