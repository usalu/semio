---
technology: animate
emoji: 🎬
---

# Animate

Manim-class Rust animation compiler technology. Imperative `Scene` timelines over an `Sobject` scene graph, with headless video (`animate/video`) and static present (`animate/present`) engines consuming `animate/core`.

## Naming (Manim → Semio)

| Manim | Semio |
| --- | --- |
| Mobject / VMobject | Sobject / VSobject |
| Group / VGroup | Group / VGroup |
| Scene | Scene |
| Animation | Animation |
| `.animate` | `.animate()` |
| Tex / MathTex | Text / MathText (Typst) |

## Core (`animate/core/rs`)

- **Sobject** — scene-graph trait with style, transforms, hierarchy, updaters, `save_state` / `generate_target`
- **Animation** — leaf tweens and composites (`AnimationGroup`, `Succession`, `LaggedStart`, `LaggedStartMap`) with recursive parent-alpha → child-alpha mapping
- **Scene** — `construct` / `play` / `wait` / `add` / `remove` imperative timeline
- **Rate functions** — full easing catalog in `rate.rs`
- **Geometry** — 2D shape catalog as `VSobject`
- **Text** — Typst-backed `Text` / `MathText`
- **Camera** — `Camera`, `MovingCamera`, `ThreeDCamera`, `ZoomedCamera`
- **Hash** — content-addressed animation fingerprints via `framework/hash`

## Conventions

- Use `mathematical_geometry` types (`Point`, `Vec2`, `Affine`, `BezPath`, …)
- Docstrings start with a unique emoji; no comments inside definitions
- Regions in `lib.rs` or `src/` modules; unit tests in each module
- `bun ./script.ts test` via nx `@semio-tech/animate-core-rs`

## Stack

- Rust crate `animate`
- kurbo / mathematical_geometry for vector paths
- typst / typst-svg for math labels
- framework/hash for Merkle animation hashing
