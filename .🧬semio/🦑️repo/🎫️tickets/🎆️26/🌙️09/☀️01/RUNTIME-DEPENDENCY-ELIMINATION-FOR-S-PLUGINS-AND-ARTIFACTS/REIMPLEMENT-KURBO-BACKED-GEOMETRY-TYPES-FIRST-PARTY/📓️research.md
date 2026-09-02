# First-Party Geometry Inventory and Differential Verification

## Scope

This child ticket completes and audits the `semio-framework-geometry` removal of production
`kurbo`. It follows the constraints and oracle pattern in:

- `../🔍️research/📓️kurbo-peniko-first-party.md`
- `../🔍️research/📓️wave-text-and-path.md`

The public geometry vocabulary is now made from ordinary first-party fields. `kurbo = "0.13.1"`
exists only in `semio-framework-geometry`'s `[dev-dependencies]`, where it is the differential
oracle.

## Repository-Wide Consumer Inventory

`cargo metadata --no-deps --format-version 1` reports these 17 direct dependents of
`semio-framework-geometry`:

- `semio-framework-3d`
- `semio-framework-graph`
- `semio-framework-math`
- `semio-framework-os-infinite`
- `semio-framework-raster`
- `semio-framework-typeset`
- `semio-framework-ui`
- `semio-framework-ui-render`
- `semio-framework-ui-scene`
- `semio-s-plugin-animate`
- `semio-s-plugin-energy`
- `semio-s-plugin-mathematical`
- `semio-s-plugin-procedural`
- `semio-s-plugin-puzzle`
- `semio-s-plugin-remodel`
- `semio-s-plugin-stdio`
- `semio-s-plugin-trinity`

A full Rust-source scan, rather than a canvas-only scan, established the facade surface that must
remain available:

| Type | Repository-used surface retained first-party |
|---|---|
| `Point` | public `x`/`y`, `ZERO`, `new`, `distance`, `x`, `y`, `Point + Vec2`, `Point - Vec2`, `Point - Point`, conversion to `(f64, f64)` |
| `Vec2` | public `x`/`y`, `ZERO`, `new`, `hypot`, `dot`, `x`, `y`, tuple construction, add/sub and assign variants, scalar mul/div and mul-assign, negation, conversion to `(f64, f64)` |
| `Affine` | `IDENTITY`, `new`, `translate`, `scale`, `rotate`, `as_coeffs`, affine composition, point transformation |
| `Rect` | `new`, `from_points`, `inflate`, `x0`, `y0`, `x1`, `y1`, `width`, `height` |
| `RoundedRectRadii` | `new` |
| `RoundedRect` | `new` and tolerance-sensitive path generation through `ShapeRef`/`append_shape_to_path` |
| `Circle` | `new` and tolerance-sensitive path generation through `ShapeRef`/`append_shape_to_path` |
| `Line` | `new` and exact path generation through `ShapeRef`/`append_shape_to_path` |
| `Arc` | `new`, `eval`, and tolerance-sensitive path generation through `ShapeRef`/`append_shape_to_path` |
| `CubicBez` | public control-point fields, `new`, `eval`, `p0`/`p1`/`p2`/`p3`, and exact path generation through `ShapeRef`/`append_shape_to_path` |
| `BezPath`/`PathEl` | `new`, `move_to`, `line_to`, `quad_to`, `curve_to`, `close_path`, `push`, `elements`, `bounding_box`, `is_empty`, `path_segments`, `apply_affine` |

The scan found no pre-existing public or consumer call to `Affine::invert`, `decompose`, or
`determinant`; those operations were not part of the facade contract and were not invented for this
ticket. Constructor usage alone confirms the scope is repository-wide: the lexical scan found 777
`Point::new`, 520 `Rect::new`, 106 `Affine::IDENTITY`, 99 `Vec2::new`, 63 `BezPath::new`, 24
`Circle::new`, and 17 `CubicBez::new` references. Ambiguous standard-library names such as
`Arc::new` were deliberately excluded from these counts.

## Implementation Audit

The first-party representation and algorithms live in
`🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs`:

- `Point`, `Vec2`, `Affine`, `Rect`, `RoundedRectRadii`, `RoundedRect`, `Circle`, `Line`, `Arc`,
  `CubicBez`, and `BezPath` contain only first-party scalar/value fields.
- affine multiplication preserves `kurbo`'s `[a, b, c, d, e, f]` layout and composition order.
- `CubicBez::eval` uses the same polynomial ordering as `kurbo` and intentionally extrapolates for
  `t` outside `[0, 1]`.
- `BezPath::bounding_box` uses analytic derivative roots for tight quadratic/cubic extrema. Empty
  and move-only paths return the oracle's zero rectangle.
- `RoundedRect::new` normalizes an inverted rectangle, takes absolute radii, and clamps each radius
  to half the shorter side, matching `kurbo`.
- `Circle` preserves `kurbo`'s four-segment minimum-error circle construction and switches to its
  sixth-root adaptive segment formula for tighter tolerances.
- `Arc` preserves `kurbo`'s sixth-root adaptive segment count, signed sweeps, rotated ellipses,
  tangent-arm construction, and zero-sweep `MoveTo` behavior.
- `RoundedRect` preserves `kurbo`'s exact path-element order, including zero-radius cubic elements.
- renderer-specific conversions now live only at the host renderer boundaries in `raster` and
  `os-infinite/canvas`; the first-party geometry API no longer exposes `to_kurbo` escape hatches.

The integration test
`🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/tests/first_party_geometry.rs`
exercises the public API without enabling the crate's unrelated `random` unit-test module. It
contains deterministic, language-independent arithmetic fixtures and a `kurbo` differential
oracle for:

- `Point`/`Vec2`/`Rect` arithmetic across 64 generated cases;
- affine composition, coefficients, and point transforms across 64 generated cases;
- exact `Rect`/`Line`/`CubicBez` path elements;
- `CubicBez::eval` inside and outside `[0, 1]`;
- element-for-element `Circle`, `Arc`, and `RoundedRect` paths across multiple tolerances and edge
  cases;
- `Arc::eval`;
- tight cubic `BezPath` bounds across 32 generated paths;
- affine path transformation and empty/move-only path bounds.

## Verification

Passed:

```text
cargo check -p semio-framework-geometry --lib
cargo check -p semio-framework-geometry --lib --target wasm32-wasip2
cargo test -p semio-framework-geometry --test first_party_geometry
  5 passed; 0 failed
bun nx run @semio-tech/framework-raster:test-quick
  3 passed; 0 failed
cargo check --lib -p semio-framework-3d -p semio-framework-graph \
  -p semio-framework-math -p semio-framework-ui-scene \
  -p semio-framework-os-infinite -p semio-framework-raster
```

The direct-consumer build finished successfully after compiling the native renderer conversion
boundaries. It emitted existing warnings in unrelated crates but no geometry migration error.

Dependency proof:

```text
cargo tree -p semio-framework-geometry --target wasm32-wasip2 --edges normal --depth 1
semio-framework-geometry v0.1.0 (...)
└── serde v1.0.228
```

For each of `semio-s-plugin-puzzle`, `semio-s-plugin-flow`, `semio-s-plugin-trinity`, and
`semio-s-plugin-animate`:

```text
cargo tree -p <plugin> --target wasm32-wasip2 -i kurbo@0.13.1
warning: nothing to print.
```

The full `semio-s-plugin-puzzle` WASI check compiled geometry and the shared WASI dependency path,
then failed inside the puzzle crate with 201 unrelated schema/value conversion errors (missing
`ToValue`/`FromValue`/`Serialize` implementations and mismatched `DslValue`/`serde_json::Value`).
No reported error involved geometry or `kurbo`.

The crate's broad unit-test target remains independently blocked by 49 pre-existing erroneous
`.await` expressions in `🎲️random/🦀️.rs`, where synchronous return values are awaited. The
new integration target avoids that unrelated `cfg(test)` compilation failure while testing the
actual public library artifact and the `kurbo` oracle.
