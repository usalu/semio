A self-contained miniframework for building playgrounds (one app, one window kind, one fixture, selection, filter, workbench, details).

- [core](./core/core.ts) — React-neutral runtime + one-app shell (`PlaygroundController`, `ProductRuntime`, declarative `UiNode` bodies, registries).
- [renderer/react](./renderer/react/index.tsx) — Shell renderer (`PlaygroundView`) and `bootPlayground`; puzzle chrome stays on `./puzzle/*` subpath exports.
