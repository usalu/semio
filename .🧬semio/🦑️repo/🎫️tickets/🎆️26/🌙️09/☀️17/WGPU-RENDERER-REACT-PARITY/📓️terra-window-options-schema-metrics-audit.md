# Window Options Tree Presentation and Measure Authority

## Finding

`WindowMeasuresProjection` now emits a real `Tree` / `TreeSection` / `TreeItem` document, but the retained WGPU tree still has one global geometry: a 24px row and a 160px value column. React's Window Options tree is a neutral compact tree presentation: its rows use `min-h-tiny`, its select control uses `h-small`, and its value column uses the already shared `windowMeasureValueColumnUiSpacing` token (32.5 × 3.2 = 104px).

The clean missing contract is therefore a closed **generic Tree presentation**, not a Window-Options-specific renderer switch and not `StyleSpec.density`.

## Exact schema recommendation

Add the following closed, defaulted contract field:

```rust
pub enum TreePresentation {
    #[default]
    Standard,
    Compact,
}

pub struct TreeProps {
    #[serde(default, skip_serializing_if = "TreePresentation::is_standard")]
    #[value(default, skip_serializing_if = "TreePresentation::is_standard")]
    pub presentation: TreePresentation,
    pub interaction_domain: Option<UiText>,
}
```

`Compact` is neutral: Window Measures is its first producer, but the value names the reusable tree form rather than one Shell feature. It must be present only on the root `Tree`; every descendant inherits the nearest tree root's presentation. A nested `Tree` starts a new presentation scope. Do not put flow or a physical left/right field on `TreeProps`: the already authoritative document `UiFlow` supplies that independently.

The current source snapshot already contains the parallel `ToggleAppearance { Button, Checkbox }` work in the contract and WGPU reconcile path. It is the right binary-control schema, so do not add a second checkbox flag. Its WGPU painting remains a separate final consumer of that field.

### Contract and generated-wire inventory

| Seam | Required change |
| --- | --- |
| `🧬️contract/🧩️component/🦀️.rs:436` | Define `TreePresentation`, append defaulted `presentation` to `TreeProps`, and provide `is_standard`. |
| `🧬️contract/🧾️typed/🦀️.rs:28` | Add `presentation: TreePresentation` to the `TreeProps` field roster. This drives typed copy, compare, patch, and retirement paths. |
| `🧬️contract/🏗️builder/🦀️.rs:1405-1445` | Store `Standard` in `TreeBuilder`, add `.presentation(...)`, and construct the field. |
| `🧬️contract/🧬️schema/🦀️.rs:825` | Add `TreePresentation` metadata and add `presentation` to the TypeScript `TreeProps` declaration. Regenerate `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`; do not hand-edit it. Export the generated enum from `🛂️manifest/🟦️.ts` with the other contract types. |
| direct constructors | Update the contract component unit, runtime reconcile unit, Shell legacy tree conversion (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:5116`), and Plugin builder-contract literals. The compiler will enumerate the small remaining list. |

The same release must complete the in-flight `ToggleAppearance` path: its contract root is `component/🦀️.rs:322-346`, typed roster entry is `typed/🦀️.rs:20`, builder is `builder/🦀️.rs:1203-1245`, WGPU transfer is `reconcile/🦀️.rs:744-755`, and shared accessibility role selection is `accessibility/🦀️.rs:78`. `Checkbox` must paint and hit-test as a checkbox while preserving the existing `checked` and focus rules; `Button` retains role `switch`.

## Per-tree retained metrics and flow

Do not use `UiNodeRecord.style.density`: WGPU declares the retained `density` field in `🧩️component/🦀️.rs:48`, but no target layout, paint, or input reader consumes it. Setting it would produce a silent non-scoped style marker.

Instead add one resolver next to `TreeRowMetrics` in `🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:128-153`:

```rust
TreeRowMetrics::for_presentation(theme, presentation)
```

`Standard` returns the current metrics unchanged. `Compact` must use:

* the existing 104px `windowMeasureValueColumnUiSpacing` value-column token;
* a dedicated shared `compactTreeRowUiSpacing` token calibrated to the React compact-tree law (the neutral fixture bounds a row at 20px); and
* existing compact control-height tokens where the concrete control has one, rather than changing global `Theme::control_height`.

There is no existing shared 20px compact-tree row token. React currently says `min-h-tiny` for the row, `h-small` for selects, and the neutral Window Measures fixture asserts `maximumRowHeight: 20`. A new shared compact-tree row token is therefore warranted; hard-coding `20.0` in Shell or changing the standard tree's `treeRowUiSpacing` is not. The existing `windowMeasureValueColumnUiSpacing` is already a shared 104px authority and needs no duplicate token.

`MountedLayoutJob` owns a single `row_metrics` at `📌️mounted_layout/🦀️.rs:382,448`, and its tree branches use it at `:554-555,587`. Replace that global use only for tree work with the nearest owning tree's presentation. `owning_tree_spec` already performs the bounded ancestor lookup used by input (`📥️input/🦀️.rs:693-700`); use the same bounded maximum-depth rule instead of cloning metrics into every child. A nested tree resolves to itself and overrides its outer tree.

The resolver must feed every geometry reader in the same accepted frame:

* mounted tree height, section/item row layout, and inline-control placement in `📌️mounted_layout/🦀️.rs:554-588`;
* retained tree paint and recursive child paint in `🖌️paint/🦀️.rs:491-499,1319`; the current paint recomputes standard metrics at `:499` and legacy test paint at `:2760,2799`;
* the published hit registry, whose caller currently forces standard metrics in `⚙️engine/🦀️.rs:630-636`, and its tree bands/control registrations in `📥️input/🦀️.rs:724-847`.

The existing value-rect helper at `🧮️layout/🦀️.rs:241-255` hard-codes a physical right column. Change it to accept `FlowInline`; `UiFlow::for_anchor` already makes `TopEnd` `Rtl` at `🧬️contract/📐️layout/🦀️.rs:346-371`. Apply that same inline direction to mounted layout, paint, and hit rectangles. `block_reversed` only controls vertical order; it cannot mirror the control column. A checkbox receives the compact value-column allocation, but its painted and hit square is intrinsic (bounded by its compact control height) inside that column; Select/Slider/Number controls may fill the 104px value column.

This avoids a document-wide theme mode, arbitrary physical reversal, and a per-row metric clone.

## Required fail-first laws

1. Extend the existing neutral `🌳️window-measures-tree-parity` fixture and its React test at `🧪️tests/🎚️window-measure-controls/🟦️.tsx:20-37`. Keep React as the third-party reference: checkbox role, 104px column, compact-row bound, a closed group, and no global-tree metric regression.
2. Add a native accepted-document law through the actual Shell projection and ACK: a `Compact` root with a Checkbox, Select, Slider, and closed group must publish 104px value geometry, never exceed the compact row bound, announce checkbox/checked, and retain distinct `.row` and control keys.
3. Repeat the native geometry/input assertions under `TopEnd`: the label and value column must reverse together, the Checkbox hit rectangle must be inside its presented column, and press dispatch must target the same control. Repeat with an adjacent `Standard` tree to prove the scope does not leak.
4. Add contract codec/generator coverage for default elision (`Standard` absent) and `compact` emission/round-trip. Do the analogous `button` default-elision and `checkbox` emission for the already introduced toggle field.

## Locale and intensity: producer, not projection

For the observed Puzzle 3D Window Options data, the WGPU projection is not a translation or numeric-default authority:

* `Shell/🎯️targets/🧊️wgpu/🦀️.rs:4392-4536` converts the already authored `WindowMeasure` rows. Sliders copy their supplied numeric value and labels are passed through; it has no German/English branch and no sun fallback.
* Puzzle 3D's actual per-window publisher is `✏️s/🔌️plugins/🧩️puzzle/.../editor/🦀️.rs:8311-8318`. It resolves `puzzle3d_labels(view_state)` and calls `main::window_measures` with the per-window runtime.
* The label authority is `.../editor/🗣️terminology/🦀️.rs:131-134`, which resolves from `ViewModel.locale` and `ViewModel.terminology`. `main/🦀️.rs:68-80` passes `labels.is_de()` into the Sun measure at `:76`; the shared `world3d_sun_measures` then emits the German or English literal labels at `🔌️plugin/🦀️.rs:40600+`.
* `WorldSunConfig::default()` has intensity `0.85` at `🔌️plugin/🦀️.rs:40572-40583`. Puzzle 3D places that default into its runtime at `.../editor/🎚️config/🦀️.rs:255`; its sun measure takes `&runtime.sun` at `.../options/☀️sun/🦀️.rs:14-15`.

Consequently, English labels in a German Window Options screen, or `1` where the initial configuration should be `.85`, must be diagnosed at the `ViewModel`/guest publication boundary or actual persisted runtime value. The renderer should only be tested for exact preservation. A real-app law should construct the same Puzzle 3D window view with `Locale::De` and `WorldSunConfig::default()`, publish and ACK it through Shell, then assert the presented German measure labels and `.85` slider value. This separates a guest/host locale or persistence defect from a renderer projection defect without inventing renderer-side localization.

## Confidence and limits

High confidence on the contract, retained geometry, and producer ownership findings: each is sourced from current code. The physical screenshot establishes a visible mismatch, but source inspection alone cannot say whether its live `1` came from persisted state or a locale propagation fault; the proposed end-to-end law determines that.
