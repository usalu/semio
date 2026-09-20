# Dock Split Physical Resize

## Runtime evidence

Checkpoint 15 and the focused same-artifact replay delivered a horizontal gutter drag of `+120`
CSS pixels. React moved the separator by `120` pixels and widened the adjacent Top body to `644`
pixels. WGPU widened it only to `524.9332` and moved the separator by about `1.2` pixels. The
journey previously asserted delivery only; the strengthened probe resolves the divider by axis,
span, and neighboring window identities and measures the adjacent rectangles.

## Cause

`dock_from_window_layout` intentionally preserves authored axis weight totals. React layouts use
percentage weights totaling `100`; nested axes can retain another total such as their parent's
`65`. `dock_axis_slots` correctly normalizes any positive total when solving rectangles.

`apply_split_drag_on_node` instead converted the pointer delta to a unit fraction and added it
directly to the authored weight. A `120 / 1000 = 0.12` pointer ratio added `0.12` to a `50` weight,
so the normalized solver moved the separator by about `0.12 / 100 * 1000 = 1.2` pixels. Its fixed
`0.08` minimum had the same scale error for percentage-weighted axes.

## Contract and repair

The shared dock-axis schema and fixture now contain a physical resize vector: a `1000` pixel axis,
weights `[50, 50]`, a `+120` pixel drag, resulting weights `[62, 38]`, and a `+120` pixel separator
movement. The language-neutral suite validates the fixture with Ajv and calls React's exported
`applyAxisResizeDelta` production function as the behavioral oracle.

WGPU now sums the captured origin weights once per drag application, converts the physical ratio
onto that exact weight scale, and scales React's eight-percent floor by the same total. Pair-total
conservation and untouched siblings are unchanged. This supports percentage-authored layouts,
normalized fallback layouts, and nested axes without canonicalizing or rewriting stored layout
weights. The Rust law loads the shared fixture, applies a real `DockState` resize, solves stack
frames before and after, and requires both `[62, 38]` and the physical `120` pixel movement.

## Validation

Focused Bun+Nx:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace --skip-nx-cache -- bun test './🧰️framework/🔨️modules/🖱️ui/🧪️tests/📐️dock-axis-geometry/🟦️.ts'
3 pass, 0 fail, 18 assertions
```

The first oracle run exposed an incorrect test path representation (`[]` instead of React's root
path `""`); after correcting the oracle invocation, all three laws passed. Rust sources parse via
`rustfmt`, and the scoped diff has no whitespace errors. Root owns native and browser validation;
no post-repair runtime movement is claimed here.

## Adjacent reconciliation

The Native31 snapshot caught two compile errors in in-flight tests: the new Dock law called a
nonexistent `Rect::right`, and the General law called `fixture()` after shadowing that function with
its local fixture value. Current source uses `rect.x + rect.w` and the existing local fixture.

UI38 also showed that the Up-flow glyph-band law compared Tree label ink to the top of a centered
small Select. The Select begins `3.2` pixels below its row top, so that reference incorrectly spent
the font-ascent allowance before measuring any ascent. The law now derives the actual row top from
`TreeRowMetrics.row_height` and `control_height_small`; selected-value glyph ownership remains
checked against the Select rectangle. Root's subsequent UI39 receipt passed all **632 / 632** tests.
