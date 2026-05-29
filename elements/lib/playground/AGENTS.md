A self-contained miniframework for building playgrounds (one app, one window kind, one fixture, selection, filter, workbench, details).

- [core](./core.ts) — React-neutral runtime (`ProductRuntime`, declarative `UiNode` bodies, registries).
- [react](./react/index.tsx) — Playground shell renderer (`PlaygroundView`); depends only on `@elements/ui`.
