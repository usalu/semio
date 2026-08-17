# UI Chrome Umbrella Split Audit

## Snapshot

- Definition: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️Chrome/🟦️component.tsx`, SHA-256 `ef751d89dc56a32d1d9012213c3806726099eaa7bbdcadc4e5e337c51d64fba0`, clean.
- No co-located Chrome story exists.
- Shared React barrel SHA-256: `e82f73a9fd61e5d140d69f7df7498fa1afcd2217fde523fb6f64c9e130844e81`.

## Distinct Responsibilities

1. Window silhouette geometry schema, normalization, outline/path/clip/region generation, and hit testing.
2. `ChromeControlHint`, an accessible native hint wrapper.
3. `LoadingRow`, a skeleton presentation used in production only by `Skeletons`.
4. Window-content dead-line measurement, overflow detection, CSS variables, and scroll hook.

These responsibilities differ in inputs, effects, invariants, and consumers. `Chrome` is an umbrella rather than a semantic component.

## Consumer Evidence and Disposition

- Window silhouette geometry is the specific geometry facet used by the public WindowChrome implementation and its contract tests. Move it to a specific `WindowSilhouette` component; preserve package exports without treating tests as module consumers.
- `ChromeControlHint` has active consumers in Toggle, PanelTabBar, and DragHandle. Move it to a specific UI element.
- `LoadingRow` has one active production consumer, Skeletons. Inline it privately into Skeletons and delete its standalone package/story surface.
- Window-content dead-line behavior is shared by Scrollable, Window, and the package Chrome-aware scroll surface, with downstream OS terminals. Move it to a specific shared UI presentation component.
- Delete the `Chrome` identity once every responsibility is moved.
