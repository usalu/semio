# Independent Root Primary Import Audit — Repeat 695

## Scope And Checkpoint

Read-only source audit of 71 pure import/reexport primary ranges against immutable B `b84bf526c7ba26f99c4054b47da1c1a4d5064e3d`, H `8f91299152ceab6c19df64c251f5ad23252ce18b`, S `6e8eff657fb70675db7ad30b890f61b7a3e201d7`. The exact incoming text is from immutable B→S range analysis, rather than interpreting old coordinates as incoming coordinates. Sixty-nine ranges introduce use statements; IDs3 and34 remove only imports. No compiler, build, tests, runtime, dependency resolution, source or Git/index mutation was performed.

This is a checkpoint with actionable reconciliation defects sent to root and the domain executor. It is not an all-preserved signoff before fresh repairs are read. Function/body/deletion packets outside these 71 are root's separate audit.

## Concrete Introduced Defects

All six grouped-import losses below are proven by immutable H supplying the grouped names and actual current same-file consumers still calling/naming them. None is inferred from a name in an unrelated owner.

| Primary ID | Actual receiving owner | Lost productive binding and consumer |
|---|---|---|
|67|P5D edit/tools/fill|terminology `puzzle5d_fill_run_reasons`, `puzzle5d_fill_run_stages`, `puzzle5d_fill_run_unit`, `Puzzle5dLabels`; calls59–62 and signature76|
|70|P5D edit/windows/world3d|terminology `Puzzle5dLabels`; signatures87/103/310/333/352/406|
|79|P5D commands/delete-selection|mutations `disconnect_grips`, `remove_part_grip`, `Puzzle5dMutation`; signature12/calls24–25|
|92|P5D precompute/brush|precompute `puzzle3d_kind_catalogs`; call63|
|93|P5D precompute/fill|precompute `puzzle3d_config`, `puzzle3d_object`, `puzzle3d_snapshot`, `puzzle3d_target_volume`, `puzzle5d_authored_kind_catalogs`, `Puzzle5dPlannerBoard`, `Puzzle5dPlannerToolRunJob`; signature27/calls33–35/63–65|
|121|P5D mutations/connect-kind-compatibility/diff|diff `Puzzle5dKindCompatibilityDelta`; insertion13|

Introduced corrupt qualification witnesses (all absent from immutable H): ID33 P2D editor lines30/34/36/37/41 double `snapshot::crate::editor::puzzle2d::snapshot` prefixes; ID73 add-target-volume line5 double `crate::editor::puzzle5d` prefix on FreshIds; ID79 delete-selection line4 `interaction::crate::editor::puzzle5d::Puzzle5dActionCtx`; ID87 settings line13 repeated editor/entities prefixes on PLAY_CONTROLLER_ID. Restore the actual H leaf-owner qualification while preserving incoming binding intent. The current ID69 root Scene import duplicates the existing view Scene binding; ID92 root Document/Fastener/Part imports duplicate their correct entities imports. These were also routed for owner-aware repair.

Actual canonical definition witnesses independently read: P2 pivot/gesture/selection records are in overview/utilities/select; P2 ActionCtx in interaction; P5 Scene in view; P5 ActionCtx in interaction; P5 FreshIds and PLAY_CONTROLLER_ID in entities; P5 Labels and fill terminology functions in terminology; planner board/job/catalog bridge in precompute. Exact definition paths are embedded below where applicable. Delta types emitted by macros need their existing diff declaration/macro context, not an invented new definition.

Absent import statements are not automatically missing behavior. IDs25(json),41(detail),60–63(close macros),68/96(board2d),104/133(play snapshot),112/115/118(mutation type) have no remaining use of that omitted name in their actual current file; their current split methods/children must remain authoritative. Those import-only omissions are not source-loss findings. ID3/34 import deletions likewise do not justify deleting split-owner bindings still needed by current consumers.

## Exact Range Binding Evidence

For each range below, the source is the actual receiver inspected. Incoming binding names are followed by actual current matching use statements, not arbitrary body text matches. Definition witnesses are canonical current declarations within the same artifact/module family, excluding package mirrors and tests. External framework imports retain their existing current qualification and are not certified by unrelated domain declarations. This evidence must be refreshed for the routed repairs before closure.

### domain-primary-25

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`

Incoming bindings: `json`, `Value`.

Current matching binding statements:

```rust
use semio_framework_pack_json::Value;
```

Names not explicitly imported here at initial checkpoint: `json`. See concrete consumed-vs-unused dispositions above.

### domain-primary-26

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📨️engagement-submit/🦀️.rs`

Incoming bindings: `puzzle2d_selection_pivot`, `Puzzle2dActionCtx`, `Puzzle2dSelectionMotion`, `Puzzle2dSelectionRecord`.

Current matching binding statements:

```rust
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_selection_pivot, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};
use crate::editor::puzzle2d::interaction::Puzzle2dActionCtx;
```

Actual definition witnesses:

- `puzzle2d_selection_pivot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle2dSelectionMotion`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dSelectionRecord`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`

### domain-primary-27

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📏️scale-selection/🦀️.rs`

Incoming bindings: `puzzle2d_gesture_phase`, `puzzle2d_selection_pivot`, `Puzzle2dActionCtx`, `Puzzle2dSelectionMotion`, `Puzzle2dSelectionRecord`.

Current matching binding statements:

```rust
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};
use crate::editor::puzzle2d::interaction::Puzzle2dActionCtx;
```

Actual definition witnesses:

- `puzzle2d_gesture_phase`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `puzzle2d_selection_pivot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle2dSelectionMotion`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dSelectionRecord`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`

### domain-primary-28

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🔄️rotate-selection/🦀️.rs`

Incoming bindings: `puzzle2d_gesture_phase`, `puzzle2d_selection_pivot`, `Puzzle2dActionCtx`, `Puzzle2dSelectionMotion`, `Puzzle2dSelectionRecord`.

Current matching binding statements:

```rust
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};
use crate::editor::puzzle2d::interaction::Puzzle2dActionCtx;
```

Actual definition witnesses:

- `puzzle2d_gesture_phase`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `puzzle2d_selection_pivot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle2dSelectionMotion`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dSelectionRecord`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`

### domain-primary-29

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🚀️translate-selection/🦀️.rs`

Incoming bindings: `puzzle2d_gesture_phase`, `Puzzle2dActionCtx`, `Puzzle2dSelectionRecord`.

Current matching binding statements:

```rust
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_gesture_phase, Puzzle2dSelectionRecord};
use crate::editor::puzzle2d::interaction::Puzzle2dActionCtx;
```

Actual definition witnesses:

- `puzzle2d_gesture_phase`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle2dSelectionRecord`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`

### domain-primary-30

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🩹️patch-inspector/🦀️.rs`

Incoming bindings: `Puzzle2dActionCtx`, `Puzzle2dSelectionRecord`.

Current matching binding statements:

```rust
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::Puzzle2dSelectionRecord;
use crate::editor::puzzle2d::interaction::Puzzle2dActionCtx;
```

Actual definition witnesses:

- `Puzzle2dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle2dSelectionRecord`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`

### domain-primary-33

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

Incoming bindings: `puzzle2d_gesture_phase`, `puzzle2d_selection_pivot`, `Puzzle2dSelectionMotion`, `Puzzle2dSelectionRecord`, `SelectToolRequest`.

Current matching binding statements:

```rust
pub use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord, SelectToolRequest};
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::puzzle2d_selection_pivot;
pub use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::Puzzle2dSelectionMotion;
pub use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::Puzzle2dSelectionRecord;
pub use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::SelectToolRequest;
```

Actual definition witnesses:

- `puzzle2d_gesture_phase`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `puzzle2d_selection_pivot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dSelectionMotion`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `Puzzle2dSelectionRecord`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
- `SelectToolRequest`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`

### domain-primary-41

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs`

Incoming bindings: `detail`, `overview`, `selection`.

Current matching binding statements:

```rust
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::modes::edit::windows::selection;
```

Names not explicitly imported here at initial checkpoint: `detail`. See concrete consumed-vs-unused dispositions above.

### domain-primary-59

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🦀️.rs`

Incoming bindings: `ChangeNodeIcon`, `EditTargetRegionLabel`, `Puzzle2dTextIntent`, `Puzzle2dTextDisposition`, `Puzzle2dTextPlan`, `Puzzle2dTextPreparationCursor`, `Puzzle2dTextPreparationStep`, `Puzzle2dSnapshot`.

Current matching binding statements:

```rust
use super::{Puzzle2dTextIntent, Puzzle2dTextDisposition, Puzzle2dTextPlan, Puzzle2dTextPreparationCursor, Puzzle2dTextPreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::{ChangeNodeIcon, EditTargetRegionLabel};
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
```

Actual definition witnesses:

- `ChangeNodeIcon`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🎨change-node-icon/🦀️.rs`
- `EditTargetRegionLabel`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🖋️edit-target-region-label/🦀️.rs`
- `Puzzle2dTextIntent`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/🦀️.rs`
- `Puzzle2dTextDisposition`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/🦀️.rs`
- `Puzzle2dTextPlan`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/🦀️.rs`
- `Puzzle2dTextPreparationCursor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/🦀️.rs`
- `Puzzle2dTextPreparationStep`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/🦀️.rs`
- `Puzzle2dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### domain-primary-60

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/↩️inverse/🦀️.rs`

Incoming bindings: `Puzzle2dCloseAxis`, `close_child_demand`, `close_demand_methods`, `Puzzle2dFlagIntent`, `Puzzle2dFlagPlan`, `Puzzle2dFlagPreparationCursor`, `Puzzle2dFlagPreparationStep`, `Puzzle2dSnapshot`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dFlagIntent, Puzzle2dFlagPlan, Puzzle2dFlagPreparationCursor, Puzzle2dFlagPreparationStep};
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
```

Actual definition witnesses:

- `Puzzle2dCloseAxis`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎮️prepare/🧰️child/🦀️.rs`
- `Puzzle2dFlagIntent`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dFlagPlan`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dFlagPreparationCursor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dFlagPreparationStep`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `close_child_demand`, `close_demand_methods`. See concrete consumed-vs-unused dispositions above.

### domain-primary-61

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/📸️candidate/🦀️.rs`

Incoming bindings: `Puzzle2dCloseAxis`, `close_child_demand`, `close_demand_methods`, `Puzzle2dFlagIntent`, `Puzzle2dFlagDisposition`, `Puzzle2dFlagPlan`, `Puzzle2dFlagPreparationCursor`, `Puzzle2dFlagPreparationStep`, `Puzzle2dSnapshot`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dFlagIntent, Puzzle2dFlagDisposition, Puzzle2dFlagPlan, Puzzle2dFlagPreparationCursor, Puzzle2dFlagPreparationStep};
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
```

Actual definition witnesses:

- `Puzzle2dCloseAxis`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎮️prepare/🧰️child/🦀️.rs`
- `Puzzle2dFlagIntent`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dFlagDisposition`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dFlagPlan`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dFlagPreparationCursor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dFlagPreparationStep`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️flag/🎮️prepare/🦀️.rs`
- `Puzzle2dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `close_child_demand`, `close_demand_methods`. See concrete consumed-vs-unused dispositions above.

### domain-primary-62

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/↩️inverse/🦀️.rs`

Incoming bindings: `Puzzle2dCloseAxis`, `close_child_demand`, `close_demand_methods`, `MoveNode`, `Puzzle2dMoveNodePlan`, `Puzzle2dMoveNodePreparationCursor`, `Puzzle2dMoveNodePreparationStep`, `Puzzle2dSnapshot`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dMoveNodePlan, Puzzle2dMoveNodePreparationCursor, Puzzle2dMoveNodePreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::MoveNode;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use super::Puzzle2dMoveNodePlan;
use super::Puzzle2dMoveNodePreparationCursor;
use super::Puzzle2dMoveNodePreparationStep;
use super::Puzzle2dSnapshot;
```

Actual definition witnesses:

- `Puzzle2dCloseAxis`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎮️prepare/🧰️child/🦀️.rs`
- `MoveNode`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🦀️.rs`
- `Puzzle2dMoveNodePlan`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/🦀️.rs`
- `Puzzle2dMoveNodePreparationCursor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/🦀️.rs`
- `Puzzle2dMoveNodePreparationStep`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/🦀️.rs`
- `Puzzle2dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `close_child_demand`, `close_demand_methods`. See concrete consumed-vs-unused dispositions above.

### domain-primary-63

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/📸️candidate/🦀️.rs`

Incoming bindings: `Puzzle2dCloseAxis`, `close_child_demand`, `close_demand_methods`, `MoveNode`, `Puzzle2dMoveNodeDisposition`, `Puzzle2dMoveNodePlan`, `Puzzle2dMoveNodePreparationCursor`, `Puzzle2dMoveNodePreparationStep`, `Puzzle2dSnapshot`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dMoveNodeDisposition, Puzzle2dMoveNodePlan, Puzzle2dMoveNodePreparationCursor, Puzzle2dMoveNodePreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::MoveNode;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use super::Puzzle2dMoveNodeDisposition;
use super::Puzzle2dMoveNodePlan;
use super::Puzzle2dMoveNodePreparationCursor;
use super::Puzzle2dMoveNodePreparationStep;
use super::Puzzle2dSnapshot;
```

Actual definition witnesses:

- `Puzzle2dCloseAxis`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎮️prepare/🧰️child/🦀️.rs`
- `MoveNode`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🦀️.rs`
- `Puzzle2dMoveNodeDisposition`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/🦀️.rs`
- `Puzzle2dMoveNodePlan`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/🦀️.rs`
- `Puzzle2dMoveNodePreparationCursor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/🦀️.rs`
- `Puzzle2dMoveNodePreparationStep`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/🦀️.rs`
- `Puzzle2dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `close_child_demand`, `close_demand_methods`. See concrete consumed-vs-unused dispositions above.

### domain-primary-66

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs`

Incoming bindings: `puzzle5d_action`, `puzzle5d_kind_ids`, `puzzle5d_kind_weight_sum`, `Puzzle5dScene`, `PUZZLE5D_PLAY_CONTROLLER_ID`, `PUZZLE5D_SUGGESTION_OFFSET_MAX`, `PUZZLE5D_SUGGESTION_OFFSET_MIN`, `PUZZLE5D_SUGGESTION_OFFSET_STEP`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::{puzzle5d_action, PUZZLE5D_PLAY_CONTROLLER_ID, PUZZLE5D_SUGGESTION_OFFSET_MAX, PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_STEP};
use crate::editor::puzzle5d::catalog::{puzzle5d_kind_ids, puzzle5d_kind_weight_sum};
use crate::editor::puzzle5d::view::Puzzle5dScene;
use both read the identical weights.
pub fn distribution_children(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    let part_ids = puzzle5d_kind_ids(&envelope.document, "parts");
```

Actual definition witnesses:

- `puzzle5d_action`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `puzzle5d_kind_ids`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs`
- `puzzle5d_kind_weight_sum`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs`
- `Puzzle5dScene`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `PUZZLE5D_PLAY_CONTROLLER_ID`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_SUGGESTION_OFFSET_MAX`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_SUGGESTION_OFFSET_MIN`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_SUGGESTION_OFFSET_STEP`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-67

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`

Incoming bindings: `puzzle5d_fill_run_counters`, `puzzle5d_fill_run_reasons`, `puzzle5d_fill_run_stages`, `puzzle5d_fill_run_unit`, `Puzzle5dLabels`, `puzzle5d_action`, `Puzzle5dScene`, `PUZZLE5D_PLAY_CONTROLLER_ID`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::terminology::puzzle5d_fill_run_counters;
use crate::editor::puzzle5d::entities::{puzzle5d_action, PUZZLE5D_PLAY_CONTROLLER_ID};
use crate::editor::puzzle5d::view::Puzzle5dScene;
```

Actual definition witnesses:

- `puzzle5d_fill_run_counters`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `puzzle5d_fill_run_reasons`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `puzzle5d_fill_run_stages`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `puzzle5d_fill_run_unit`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `Puzzle5dLabels`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `puzzle5d_action`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dScene`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `PUZZLE5D_PLAY_CONTROLLER_ID`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `puzzle5d_fill_run_reasons`, `puzzle5d_fill_run_stages`, `puzzle5d_fill_run_unit`, `Puzzle5dLabels`. See concrete consumed-vs-unused dispositions above.

### domain-primary-68

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`

Incoming bindings: `board2d`, `world3d`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::modes::edit::windows::world3d;
```

Names not explicitly imported here at initial checkpoint: `board2d`. See concrete consumed-vs-unused dispositions above.

### domain-primary-69

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`

Incoming bindings: `puzzle5d_action`, `Puzzle5dScene`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::puzzle5d_action;
use crate::editor::puzzle5d::view::Puzzle5dScene;
use crate::editor::puzzle5d::Puzzle5dScene;
```

Actual definition witnesses:

- `puzzle5d_action`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dScene`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`

### domain-primary-70

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️.rs`

Incoming bindings: `options`, `utilities`, `puzzle5d_placement_entity`, `PUZZLE5D_WORLD_MESH_KINDS`, `puzzle5d_localized`, `Puzzle5dLabels`, `part_scale_json`, `puzzle5d_grip_full_id`, `puzzle5d_gumball_active`, `puzzle5d_scene_mode`, `puzzle5d_transform_handle`, `resolve_grip_world_position`, `resolve_part_mesh_url`, `world_grip_direction`, `target_volume_scale_json`, `world_grip_position`, `Puzzle5dDocument`, `Puzzle5dInteractionSnapshot`, `Puzzle5dPart`, `Puzzle5dScene`, `PUZZLE5D_FALLBACK_MESH_KIND`, `PUZZLE5D_GRANULARITY_FASTENER`, `PUZZLE5D_GRANULARITY_GRIP`, `PUZZLE5D_GRANULARITY_PART`, `PUZZLE5D_GRIP_SHOW_ALWAYS`, `PUZZLE5D_INTERACTION_DOMAIN`, `PUZZLE5D_TARGET_VOLUME_COLOR`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::modes::edit::options as mode_options;
use crate::editor::puzzle5d::modes::edit::windows::world3d::options;
use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities;
use crate::editor::puzzle5d::precompute::puzzle5d_placement_entity;
use crate::editor::puzzle5d::precompute::PUZZLE5D_WORLD_MESH_KINDS;
use crate::editor::puzzle5d::terminology::puzzle5d_localized;
use crate::editor::puzzle5d::entities::{part_scale_json, puzzle5d_grip_full_id, resolve_grip_world_position, resolve_part_mesh_url, world_grip_direction, target_volume_scale_json, world_grip_position, Puzzle5dDocument, Puzzle5dPart, PUZZLE5D_FALLBACK_MESH_KIND, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRIP_SHOW_ALWAYS, PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_TARGET_VOLUME_COLOR};
use crate::editor::puzzle5d::view::{puzzle5d_gumball_active, puzzle5d_scene_mode, puzzle5d_transform_handle, Puzzle5dInteractionSnapshot, Puzzle5dScene};
```

Actual definition witnesses:

- `options`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🧊️volume-brush/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs`
- `puzzle5d_placement_entity`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `PUZZLE5D_WORLD_MESH_KINDS`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle5d_localized`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `Puzzle5dLabels`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `part_scale_json`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `puzzle5d_grip_full_id`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `puzzle5d_gumball_active`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `puzzle5d_scene_mode`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `puzzle5d_transform_handle`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `resolve_grip_world_position`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `resolve_part_mesh_url`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `world_grip_direction`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `target_volume_scale_json`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `world_grip_position`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dDocument`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dInteractionSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dScene`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `PUZZLE5D_FALLBACK_MESH_KIND`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRANULARITY_FASTENER`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRANULARITY_GRIP`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRANULARITY_PART`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRIP_SHOW_ALWAYS`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_INTERACTION_DOMAIN`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_TARGET_VOLUME_COLOR`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `Puzzle5dLabels`. See concrete consumed-vs-unused dispositions above.

### domain-primary-71

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️set-grid-spacing/🦀️.rs`

Incoming bindings: `Puzzle5dActionCtx`, `PUZZLE5D_GRID_SPACING_MAX`, `PUZZLE5D_GRID_SPACING_MIN`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRID_SPACING_MAX, PUZZLE5D_GRID_SPACING_MIN};
use crate::editor::puzzle5d::PUZZLE5D_GRID_SPACING_MAX;
use crate::editor::puzzle5d::PUZZLE5D_GRID_SPACING_MIN;
```

Actual definition witnesses:

- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_GRID_SPACING_MAX`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRID_SPACING_MIN`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-72

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✋️set-lod-manual/🦀️.rs`

Incoming bindings: `Puzzle5dActionCtx`, `PUZZLE5D_LOD_SLIDER_MAX`, `PUZZLE5D_LOD_SLIDER_MIN`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::entities::{PUZZLE5D_LOD_SLIDER_MAX, PUZZLE5D_LOD_SLIDER_MIN};
use crate::editor::puzzle5d::PUZZLE5D_LOD_SLIDER_MAX;
use crate::editor::puzzle5d::PUZZLE5D_LOD_SLIDER_MIN;
```

Actual definition witnesses:

- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_LOD_SLIDER_MAX`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_LOD_SLIDER_MIN`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-73

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-target-volume/🦀️.rs`

Incoming bindings: `puzzle5d_value_as_f64_3`, `Puzzle5dActionCtx`, `Puzzle5dFreshIds`, `create_target_volume`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::puzzle5d_value_as_f64_3;
use crate::editor::puzzle5d::crate::editor::puzzle5d::Puzzle5dFreshIds;
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
```

Actual definition witnesses:

- `puzzle5d_value_as_f64_3`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle5dFreshIds`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `create_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🦀️.rs`
- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

### domain-primary-74

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs`

Incoming bindings: `gumball_target_world`, `Puzzle5dActionCtx`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::view::gumball_target_world;
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
```

Actual definition witnesses:

- `gumball_target_world`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`

### domain-primary-75

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-grid-factor/🦀️.rs`

Incoming bindings: `puzzle5d_absolute_or_delta`, `Puzzle5dActionCtx`, `PUZZLE5D_GRID_FACTOR_MAX`, `PUZZLE5D_GRID_FACTOR_MIN`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::{puzzle5d_absolute_or_delta, PUZZLE5D_GRID_FACTOR_MAX, PUZZLE5D_GRID_FACTOR_MIN};
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
```

Actual definition witnesses:

- `puzzle5d_absolute_or_delta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_GRID_FACTOR_MAX`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRID_FACTOR_MIN`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-76

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-voxel-dims/🦀️.rs`

Incoming bindings: `Puzzle5dActionCtx`, `PUZZLE5D_VOXEL_DIM_MAX`, `PUZZLE5D_VOXEL_DIM_MIN`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::entities::{PUZZLE5D_VOXEL_DIM_MAX, PUZZLE5D_VOXEL_DIM_MIN};
use crate::editor::puzzle5d::PUZZLE5D_VOXEL_DIM_MAX;
use crate::editor::puzzle5d::PUZZLE5D_VOXEL_DIM_MIN;
```

Actual definition witnesses:

- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_VOXEL_DIM_MAX`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_VOXEL_DIM_MIN`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-77

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📡️set-proximity-radius/🦀️.rs`

Incoming bindings: `puzzle5d_absolute_or_delta`, `Puzzle5dActionCtx`, `PUZZLE5D_PROXIMITY_RADIUS_MAX`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::{puzzle5d_absolute_or_delta, PUZZLE5D_PROXIMITY_RADIUS_MAX};
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
```

Actual definition witnesses:

- `puzzle5d_absolute_or_delta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_PROXIMITY_RADIUS_MAX`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-78

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-snapshot/🦀️.rs`

Incoming bindings: `Puzzle5dActionCtx`, `Puzzle5dDocument`, `PUZZLE5D_SCHEMA`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::entities::{Puzzle5dDocument, PUZZLE5D_SCHEMA};
use crate::editor::puzzle5d::Puzzle5dDocument;
use crate::editor::puzzle5d::PUZZLE5D_SCHEMA;
```

Actual definition witnesses:

- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle5dDocument`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_SCHEMA`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-79

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs`

Incoming bindings: `puzzle5d_grip_full_id`, `Puzzle5dActionCtx`, `delete_part`, `disconnect_grips`, `remove_part_grip`, `Puzzle5dMutation`, `Puzzle5dFastener`, `Puzzle5dSnapshot`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;
use crate::editor::puzzle5d::interaction::crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::standards::v1::subsets::any::schema::mutations::delete_part;
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener};
```

Actual definition witnesses:

- `puzzle5d_grip_full_id`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `delete_part`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `disconnect_grips`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `remove_part_grip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `disconnect_grips`, `remove_part_grip`, `Puzzle5dMutation`. See concrete consumed-vs-unused dispositions above.

### domain-primary-80

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚧️set-brush-placement-contact-tolerance/🦀️.rs`

Incoming bindings: `puzzle5d_absolute_or_delta`, `Puzzle5dActionCtx`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::puzzle5d_absolute_or_delta;
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
```

Actual definition witnesses:

- `puzzle5d_absolute_or_delta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`

### domain-primary-81

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚩️set-selection-flag/🦀️.rs`

Incoming bindings: `Puzzle5dActionCtx`, `PUZZLE5D_GRANULARITY_PART`, `change_part_2d_hidden`, `change_part_2d_locked`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::entities::PUZZLE5D_GRANULARITY_PART;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_locked;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
```

Actual definition witnesses:

- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_GRANULARITY_PART`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `change_part_2d_hidden`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `change_part_2d_locked`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

### domain-primary-82

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤏️set-grip-show/🦀️.rs`

Incoming bindings: `Puzzle5dActionCtx`, `PUZZLE5D_GRIP_SHOW_ALWAYS`, `PUZZLE5D_GRIP_SHOW_SELECTED`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRIP_SHOW_ALWAYS, PUZZLE5D_GRIP_SHOW_SELECTED};
use crate::editor::puzzle5d::PUZZLE5D_GRIP_SHOW_ALWAYS;
use crate::editor::puzzle5d::PUZZLE5D_GRIP_SHOW_SELECTED;
```

Actual definition witnesses:

- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_GRIP_SHOW_ALWAYS`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRIP_SHOW_SELECTED`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-83

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️set-grip-direction/🦀️.rs`

Incoming bindings: `Puzzle5dActionCtx`, `PUZZLE5D_GRIP_DIRECTION_INWARDS`, `PUZZLE5D_GRIP_DIRECTION_OUTWARDS`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRIP_DIRECTION_INWARDS, PUZZLE5D_GRIP_DIRECTION_OUTWARDS};
use crate::editor::puzzle5d::PUZZLE5D_GRIP_DIRECTION_INWARDS;
use crate::editor::puzzle5d::PUZZLE5D_GRIP_DIRECTION_OUTWARDS;
```

Actual definition witnesses:

- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_GRIP_DIRECTION_INWARDS`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRIP_DIRECTION_OUTWARDS`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-84

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️set-chunk-size/🦀️.rs`

Incoming bindings: `puzzle5d_absolute_or_delta`, `Puzzle5dActionCtx`, `PUZZLE5D_CHUNK_SIZE_MAX`, `PUZZLE5D_CHUNK_SIZE_MIN`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::entities::{puzzle5d_absolute_or_delta, PUZZLE5D_CHUNK_SIZE_MAX, PUZZLE5D_CHUNK_SIZE_MIN};
use crate::editor::puzzle5d::interaction::Puzzle5dActionCtx;
```

Actual definition witnesses:

- `puzzle5d_absolute_or_delta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dActionCtx`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `PUZZLE5D_CHUNK_SIZE_MAX`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_CHUNK_SIZE_MIN`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-87

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️settings/🦀️.rs`

Incoming bindings: `ui_label`, `Puzzle5dScene`, `PUZZLE5D_PLAY_CONTROLLER_ID`, `ActionBinding`, `Buildable`, `BuiltNode`, `Component`, `HasBase`, `HasChildren`, `NumberStepperProps`, `Trigger`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::ui_label;
use crate::editor::puzzle5d::view::Puzzle5dScene;
use crate::editor::puzzle5d::crate::editor::puzzle5d::entities::crate::editor::puzzle5d::PUZZLE5D_PLAY_CONTROLLER_ID;
use semio_framework_plugin::plugin_app_close_prelude::ActionBinding;
use semio_framework_plugin::plugin_app_close_prelude::Buildable;
use semio_framework_plugin::plugin_app_close_prelude::BuiltNode;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::plugin_app_close_prelude::HasBase;
use semio_framework_plugin::plugin_app_close_prelude::HasChildren;
use semio_framework_plugin::plugin_app_close_prelude::NumberStepperProps;
use semio_framework_plugin::plugin_app_close_prelude::Trigger;
```

Actual definition witnesses:

- `ui_label`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle5dScene`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `PUZZLE5D_PLAY_CONTROLLER_ID`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-88

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`

Incoming bindings: `engine_grip_kind`, `part_scale_json`, `puzzle5d_grip_full_id`, `puzzle5d_part_display_label`, `target_volume_flat_rect`, `target_volume_scale_json`, `ui_label`, `Puzzle5dDocument`, `Puzzle5dFastener`, `Puzzle5dGrip`, `Puzzle5dInteractionSnapshot`, `Puzzle5dPart`, `Puzzle5dScene`, `Puzzle5dTargetVolume`, `PUZZLE5D_GRANULARITY_FASTENER`, `PUZZLE5D_GRANULARITY_GRIP`, `PUZZLE5D_GRANULARITY_PART`, `PUZZLE5D_GRANULARITY_TARGET_VOLUME`, `PUZZLE5D_PLAY_CONTROLLER_ID`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::ui_label;
use crate::editor::puzzle5d::catalog::puzzle5d_part_display_label;
use crate::editor::puzzle5d::view::{engine_grip_kind, Puzzle5dInteractionSnapshot, Puzzle5dScene};
use crate::editor::puzzle5d::entities::{part_scale_json, puzzle5d_grip_full_id, target_volume_flat_rect, target_volume_scale_json, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dTargetVolume, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME, PUZZLE5D_PLAY_CONTROLLER_ID};
use crate::editor::puzzle5d::part_scale_json;
use crate::editor::puzzle5d::puzzle5d_grip_full_id;
use crate::editor::puzzle5d::puzzle5d_part_display_label;
use crate::editor::puzzle5d::target_volume_flat_rect;
use crate::editor::puzzle5d::target_volume_scale_json;
use crate::editor::puzzle5d::ui_label;
use crate::editor::puzzle5d::Puzzle5dDocument;
use crate::editor::puzzle5d::Puzzle5dFastener;
use crate::editor::puzzle5d::Puzzle5dGrip;
use crate::editor::puzzle5d::Puzzle5dInteractionSnapshot;
use crate::editor::puzzle5d::Puzzle5dPart;
use crate::editor::puzzle5d::Puzzle5dScene;
use crate::editor::puzzle5d::Puzzle5dTargetVolume;
use crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_FASTENER;
use crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_GRIP;
use crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_PART;
use crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_TARGET_VOLUME;
use crate::editor::puzzle5d::PUZZLE5D_PLAY_CONTROLLER_ID;
```

Actual definition witnesses:

- `engine_grip_kind`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `part_scale_json`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `puzzle5d_grip_full_id`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `puzzle5d_part_display_label`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs`
- `target_volume_flat_rect`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `target_volume_scale_json`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `ui_label`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle5dDocument`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dGrip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dInteractionSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dScene`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `Puzzle5dTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRANULARITY_FASTENER`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRANULARITY_GRIP`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRANULARITY_PART`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_GRANULARITY_TARGET_VOLUME`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE5D_PLAY_CONTROLLER_ID`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-89

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`

Incoming bindings: `puzzle5d_inferred_kind_rows`, `ui_label`, `Puzzle5dScene`, `PUZZLE5D_PLAY_CONTROLLER_ID`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::interaction::ui_label;
use crate::editor::puzzle5d::catalog::{catalog_kind_label, puzzle5d_inferred_kind_rows};
use crate::editor::puzzle5d::view::Puzzle5dScene;
use crate::editor::puzzle5d::entities::PUZZLE5D_PLAY_CONTROLLER_ID;
```

Actual definition witnesses:

- `puzzle5d_inferred_kind_rows`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs`
- `ui_label`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `Puzzle5dScene`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `PUZZLE5D_PLAY_CONTROLLER_ID`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-92

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🖌️brush/🦀️.rs`

Incoming bindings: `puzzle3d_config`, `puzzle3d_kind_catalogs`, `puzzle3d_snapshot`, `puzzle5d_authored_kind_catalogs`, `Puzzle5dPlannerBoard`, `Puzzle5dPlannerToolRunJob`, `Puzzle5dDocument`, `Puzzle5dFastener`, `Puzzle5dPart`, `Puzzle5dPlayApp`, `Puzzle5dPlaySnapshot`, `EditorApp`, `Fault`, `ToolRunJob`, `ToolRunJobPurpose`, `ToolRunJobRequest`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::precompute::puzzle3d_config;
use crate::editor::puzzle5d::entities::{Puzzle5dDocument, Puzzle5dFastener, Puzzle5dPart};
use crate::editor::puzzle5d::precompute::puzzle3d_snapshot;
use crate::editor::puzzle5d::precompute::puzzle5d_authored_kind_catalogs;
use crate::editor::puzzle5d::precompute::Puzzle5dPlannerBoard;
use crate::editor::puzzle5d::precompute::Puzzle5dPlannerToolRunJob;
use crate::editor::puzzle5d::Puzzle5dDocument;
use crate::editor::puzzle5d::Puzzle5dFastener;
use crate::editor::puzzle5d::Puzzle5dPart;
use crate::editor::puzzle5d::Puzzle5dPlayApp;
use crate::Puzzle5dPlaySnapshot;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Fault;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobPurpose;
use semio_framework_plugin::ToolRunJobRequest;
```

Actual definition witnesses:

- `puzzle3d_config`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle3d_kind_catalogs`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle3d_snapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle5d_authored_kind_catalogs`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `Puzzle5dPlannerBoard`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `Puzzle5dPlannerToolRunJob`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `Puzzle5dDocument`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dPlayApp`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔧️play-app-retained-commands-and-tool-proofs/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dPlaySnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎞️play-snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `puzzle3d_kind_catalogs`. See concrete consumed-vs-unused dispositions above.

### domain-primary-93

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🪣️fill/🦀️.rs`

Incoming bindings: `editor_part`, `puzzle3d_config`, `puzzle3d_object`, `puzzle3d_snapshot`, `puzzle3d_target_volume`, `puzzle5d_authored_kind_catalogs`, `Puzzle5dPlannerBoard`, `Puzzle5dPlannerToolRunJob`, `Puzzle5dDocument`, `Puzzle5dPlayApp`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::precompute::editor_part;
use crate::editor::puzzle5d::entities::Puzzle5dDocument;
```

Actual definition witnesses:

- `editor_part`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle3d_config`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle3d_object`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle3d_snapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle3d_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `puzzle5d_authored_kind_catalogs`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `Puzzle5dPlannerBoard`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `Puzzle5dPlannerToolRunJob`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`
- `Puzzle5dDocument`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPlayApp`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔧️play-app-retained-commands-and-tool-proofs/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `puzzle3d_config`, `puzzle3d_object`, `puzzle3d_snapshot`, `puzzle3d_target_volume`, `puzzle5d_authored_kind_catalogs`, `Puzzle5dPlannerBoard`, `Puzzle5dPlannerToolRunJob`, `Puzzle5dPlayApp`. See concrete consumed-vs-unused dispositions above.

### domain-primary-94

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`

Incoming bindings: `board2d`, `world3d as world3d_window`, `app_with_registry`, `close_app`, `dispatch`, `meta`, `projection_of`, `render_body`, `window_view`, `Puzzle5dApp`, `capsule_dream_example_document`, `concrete_forest_example_document`, `nakagin_example_document`, `Puzzle5dPlaySnapshot`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::modes::edit::windows::world3d as world3d_window;
use crate::editor::puzzle5d::testkit::{app_with_registry, close_app, dispatch, meta, projection_of, render_body, window_view, Puzzle5dTestApp};
use crate::editor::puzzle5d::entities::unit_tests::context::close_app;
use crate::editor::puzzle5d::unit_tests::context::dispatch;
use crate::editor::puzzle5d::unit_tests::context::meta;
use crate::editor::puzzle5d::unit_tests::context::projection_of;
use crate::editor::puzzle5d::unit_tests::context::render_body;
use crate::editor::puzzle5d::unit_tests::context::window_view;
use crate::editor::puzzle5d::unit_tests::context::Puzzle5dApp;
use crate::editor::puzzle5d::capsule_dream_example_document;
use crate::editor::puzzle5d::concrete_forest_example_document;
use crate::editor::puzzle5d::nakagin_example_document;
use crate::Puzzle5dPlaySnapshot;
```

Actual definition witnesses:

- `Puzzle5dApp`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌉️bridge/🦀️.rs`
- `capsule_dream_example_document`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `concrete_forest_example_document`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `nakagin_example_document`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPlaySnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎞️play-snapshot/🦀️.rs`

### domain-primary-96

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs`

Incoming bindings: `Puzzle5dCamera2d`, `Puzzle5dCamera3d`, `Puzzle5dRuntime`, `Puzzle5dSelectableKinds`, `board2d`, `world3d`.

Current matching binding statements:

```rust
use crate::editor::puzzle5d::config::Puzzle5dCamera2d;
use crate::editor::puzzle5d::config::Puzzle5dCamera3d;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::config::Puzzle5dSelectableKinds;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
```

Actual definition witnesses:

- `Puzzle5dCamera2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `Puzzle5dCamera3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `Puzzle5dRuntime`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `Puzzle5dSelectableKinds`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `board2d`. See concrete consumed-vs-unused dispositions above.

### domain-primary-97

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/◻️2d/🦀️.rs`

Incoming bindings: `Puzzle5dFastener`, `Puzzle5dPart`, `Puzzle5dSnapshot`.

Current matching binding statements:

```rust
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener, Puzzle5dPart};
use crate::Puzzle5dPart;
use crate::Puzzle5dSnapshot;
```

Actual definition witnesses:

- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### domain-primary-98

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️3d/🦀️.rs`

Incoming bindings: `Puzzle5dPart`, `Puzzle5dScale`, `Puzzle5dSnapshot`, `MeshView`, `MeshWindowKit`, `WindowKit`, `world3d_mesh_id_from_url`, `world3d_meshes_json_from_urls`, `world3d_selection_json`, `WindowKindDefinition`.

Current matching binding statements:

```rust
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart;
use crate::Puzzle5dScale;
use semio_framework_plugin::app::MeshView;
use semio_framework_plugin::app::MeshWindowKit;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::world3d_mesh_id_from_url;
use semio_framework_plugin::world3d_meshes_json_from_urls;
use semio_framework_plugin::world3d_selection_json;
use semio_framework_plugin::WindowKindDefinition;
```

Actual definition witnesses:

- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dScale`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### domain-primary-99

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📐️geometry/🦀️.rs`

Incoming bindings: `Puzzle5dFastener`, `Puzzle5dGrip`, `Puzzle5dPart`, `Puzzle5dPartAnchor`, `Puzzle5dSnapshot`, `HashMap`, `HashSet`, `VecDeque`.

Current matching binding statements:

```rust
use the sketchpad bridge hands it raw JSON, whereas every Rust caller already holds
//! the schema. Everything downstream of that boundary — the tolerance, the diagram constants, the
//! BFS root order, the degenerate-direction branches, the matrix order — is React's.

use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dFastener;
use crate::Puzzle5dGrip;
use crate::Puzzle5dPart;
use crate::Puzzle5dPartAnchor;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
```

Actual definition witnesses:

- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dGrip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dPartAnchor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### domain-primary-100

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs`

Incoming bindings: `Puzzle5dFastener`, `Puzzle5dKindCompatibility`, `Puzzle5dMeta`, `Puzzle5dPart`, `Puzzle5dTargetVolume`, `PUZZLE_5D_SCHEMA`.

Current matching binding statements:

```rust
use crate::{PUZZLE_5D_SCHEMA};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dTargetVolume};
use crate::Puzzle5dKindCompatibility;
use crate::Puzzle5dMeta;
use crate::Puzzle5dPart;
use crate::Puzzle5dTargetVolume;
use crate::PUZZLE_5D_SCHEMA;
```

Actual definition witnesses:

- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dKindCompatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dMeta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE_5D_SCHEMA`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs`

### domain-primary-101

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs`

Incoming bindings: `Puzzle5dPlaySnapshot`.

Current matching binding statements:

```rust
use crate::Puzzle5dPlaySnapshot;
```

Actual definition witnesses:

- `Puzzle5dPlaySnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎞️play-snapshot/🦀️.rs`

### domain-primary-102

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`

Incoming bindings: `PUZZLE_5D_SCHEMA`, `Puzzle5dPart`, `ArtifactCommand`, `create_document_envelope`.

Current matching binding statements:

```rust
use crate::PUZZLE_5D_SCHEMA};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dPart;
use store::os_store::store_schema::ArtifactCommand;
use store::os_store::document_materialize::create_document_envelope;
```

Actual definition witnesses:

- `PUZZLE_5D_SCHEMA`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

### domain-primary-103

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs`

Incoming bindings: `Puzzle5dFastener`, `Puzzle5dKindCompatibility`, `Puzzle5dMeta`, `Puzzle5dPart`, `Puzzle5dTargetVolume`, `PUZZLE_5D_SCHEMA`.

Current matching binding statements:

```rust
use crate::{PUZZLE_5D_SCHEMA};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dTargetVolume};
use crate::Puzzle5dKindCompatibility;
use crate::Puzzle5dMeta;
use crate::Puzzle5dPart;
use crate::Puzzle5dTargetVolume;
use crate::PUZZLE_5D_SCHEMA;
```

Actual definition witnesses:

- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dKindCompatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dMeta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE_5D_SCHEMA`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs`

### domain-primary-104

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs`

Incoming bindings: `Puzzle5dPlaySnapshot`.

Current matching binding statements:

```rust

```

Actual definition witnesses:

- `Puzzle5dPlaySnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎞️play-snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `Puzzle5dPlaySnapshot`. See concrete consumed-vs-unused dispositions above.

### domain-primary-105

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✂️transfer/🦀️.rs`

Incoming bindings: `Puzzle5dFastener`, `Puzzle5dPart`, `Puzzle5dSnapshot`, `HashMap`, `HashSet`.

Current matching binding statements:

```rust
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dFastener;
use crate::Puzzle5dPart;
use std::collections::HashMap;
use std::collections::HashSet;
```

Actual definition witnesses:

- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### domain-primary-106

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs`

Incoming bindings: `Puzzle5dFastener`, `Puzzle5dGrip`, `Puzzle5dPart`, `Puzzle5dPartAnchor`, `Puzzle5dScale`, `Puzzle5dSnapshot`.

Current matching binding statements:

```rust
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dPartAnchor, Puzzle5dScale};
use crate::Puzzle5dGrip;
use crate::Puzzle5dPart;
use crate::Puzzle5dPartAnchor;
use crate::Puzzle5dScale;
use crate::Puzzle5dSnapshot;
```

Actual definition witnesses:

- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dGrip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dPartAnchor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dScale`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### domain-primary-107

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Incoming bindings: `Puzzle5dFastener`, `Puzzle5dKindCompatibility`, `Puzzle5dMeta`, `Puzzle5dPart`, `Puzzle5dTargetVolume`, `PUZZLE_5D_SCHEMA`.

Current matching binding statements:

```rust
use crate::{PUZZLE_5D_SCHEMA};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dTargetVolume};
use crate::Puzzle5dKindCompatibility;
use crate::Puzzle5dMeta;
use crate::Puzzle5dPart;
use crate::Puzzle5dTargetVolume;
use crate::PUZZLE_5D_SCHEMA;
```

Actual definition witnesses:

- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dKindCompatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dMeta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `PUZZLE_5D_SCHEMA`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs`

### domain-primary-108

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`

Incoming bindings: `Puzzle5dCompatSpecificity`, `Puzzle5dFastener`, `Puzzle5dGrip`, `Puzzle5dKindCatalogsExtra`, `Puzzle5dKindCompatibility`, `Puzzle5dMeta`, `Puzzle5dPart`, `Puzzle5dPart2d`, `Puzzle5dPart3d`, `Puzzle5dPartAnchor`, `Puzzle5dGrip2d`, `Puzzle5dGrip3d`, `Puzzle5dScale`, `Puzzle5dTargetVolume`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dCompatSpecificity, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dScale, Puzzle5dTargetVolume};
use crate::standards::v1::subsets::any::schema::kind_catalogs::{Puzzle5dKindCatalogsExtra};
```

Actual definition witnesses:

- `Puzzle5dCompatSpecificity`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dGrip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dKindCatalogsExtra`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️kind-catalogs/🦀️.rs`
- `Puzzle5dKindCompatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dMeta`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dPart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `Puzzle5dPart2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPart3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dPartAnchor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dGrip2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dGrip3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dScale`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-109

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`

Incoming bindings: `Puzzle5dKindCatalogsExtra`, `Puzzle5dSnapshot`, `Puzzle5dTargetVolume`.

Current matching binding statements:

```rust
use crate::Puzzle5dSnapshot;
use crate::standards::v1::subsets::any::schema::kind_catalogs::{Puzzle5dKindCatalogsExtra};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dTargetVolume};
use crate::Puzzle5dSnapshot;
use crate::Puzzle5dTargetVolume;
```

Actual definition witnesses:

- `Puzzle5dKindCatalogsExtra`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️kind-catalogs/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `Puzzle5dTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`

### domain-primary-110

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-selection2d/↩️inverse/🦀️.rs`

Incoming bindings: `move_part_2d`, `puzzle5d_selection`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::move_part_2d;
use crate::standards::v1::subsets::any::schema::mutations::move_part_2d;
use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
```

Actual definition witnesses:

- `move_part_2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `puzzle5d_selection`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

### domain-primary-111

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-selection2d/🦀️.rs`

Incoming bindings: `puzzle5d_selection_items`, `puzzle5d_selection_number`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_number};
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection_number;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
```

Actual definition witnesses:

- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

### domain-primary-112

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/↩️inverse/🦀️.rs`

Incoming bindings: `puzzle5d_selection`, `quat_from_axis_angle`, `quat_mul`, `rotate_part_3d`, `rotate_target_volume`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection;
use crate::standards::v1::subsets::any::schema::mutations::{quat_from_axis_angle, quat_mul, rotate_part_3d, rotate_target_volume};
use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection};
```

Actual definition witnesses:

- `puzzle5d_selection`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `rotate_part_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `rotate_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `Puzzle5dMutation`. See concrete consumed-vs-unused dispositions above.

### domain-primary-113

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/🔺️diff/🦀️.rs`

Incoming bindings: `puzzle5d_selection`, `puzzle5d_selection_outcome`, `quat_from_axis_angle`, `quat_mul`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::{quat_from_axis_angle, quat_mul};
use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection, puzzle5d_selection_outcome};
```

Actual definition witnesses:

- `puzzle5d_selection`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `puzzle5d_selection_outcome`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`

### domain-primary-114

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/🦀️.rs`

Incoming bindings: `puzzle5d_selection_items`, `puzzle5d_selection_number`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_number};
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection_number;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
```

Actual definition witnesses:

- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

### domain-primary-115

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/↩️inverse/🦀️.rs`

Incoming bindings: `puzzle5d_scaled`, `puzzle5d_selection`, `scale_part_3d`, `scale_target_volume`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_scaled;
use crate::standards::v1::subsets::any::schema::mutations::{scale_part_3d, scale_target_volume};
use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection};
```

Actual definition witnesses:

- `puzzle5d_scaled`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️support/🦀️.rs`
- `puzzle5d_selection`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `scale_part_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `scale_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🦀️.rs`
- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `Puzzle5dMutation`. See concrete consumed-vs-unused dispositions above.

### domain-primary-116

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/🔺️diff/🦀️.rs`

Incoming bindings: `puzzle5d_scaled`, `puzzle5d_selection`, `puzzle5d_selection_outcome`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_scaled;
use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection, puzzle5d_selection_outcome};
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection;
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection_outcome;
```

Actual definition witnesses:

- `puzzle5d_scaled`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️support/🦀️.rs`
- `puzzle5d_selection`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `puzzle5d_selection_outcome`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`

### domain-primary-117

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/🦀️.rs`

Incoming bindings: `puzzle5d_selection_items`, `puzzle5d_selection_triple`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_triple};
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection_triple;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
```

Actual definition witnesses:

- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

### domain-primary-118

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/↩️inverse/🦀️.rs`

Incoming bindings: `move_part_2d`, `move_part_3d`, `move_target_volume`, `puzzle5d_selection`, `Puzzle5dMutation`, `PUZZLE5D_FLAT_TO_WORLD`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::move_part_2d;
use crate::standards::v1::subsets::any::schema::mutations::{move_part_3d, move_target_volume, PUZZLE5D_FLAT_TO_WORLD};
use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection};
```

Actual definition witnesses:

- `move_part_2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `move_part_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `move_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `puzzle5d_selection`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `PUZZLE5D_FLAT_TO_WORLD`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️support/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `Puzzle5dMutation`. See concrete consumed-vs-unused dispositions above.

### domain-primary-119

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/🔺️diff/🦀️.rs`

Incoming bindings: `puzzle5d_selection`, `puzzle5d_selection_outcome`, `PUZZLE5D_FLAT_TO_WORLD`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;
use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection, puzzle5d_selection_outcome};
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection_outcome;
use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;
```

Actual definition witnesses:

- `puzzle5d_selection`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `puzzle5d_selection_outcome`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `PUZZLE5D_FLAT_TO_WORLD`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️support/🦀️.rs`

### domain-primary-120

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/🦀️.rs`

Incoming bindings: `puzzle5d_selection_items`, `puzzle5d_selection_triple`, `Puzzle5dMutation`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_triple};
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection_triple;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
```

Actual definition witnesses:

- `Puzzle5dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

### domain-primary-121

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🔺️diff/🦀️.rs`

Incoming bindings: `Puzzle5dDiff`, `Puzzle5dKindCompatibilityDelta`, `Puzzle5dKindCompatibility`, `Puzzle5dSnapshot`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dKindCompatibility};
```

Actual definition witnesses:

- `Puzzle5dDiff`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `Puzzle5dKindCompatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `Puzzle5dKindCompatibilityDelta`. See concrete consumed-vs-unused dispositions above.

### domain-primary-122

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

Incoming bindings: `add_part_grip`, `AddPartGrip`, `change_description`, `ChangeDescription`, `change_domain`, `ChangeDomain`, `change_fastener_kind`, `ChangeFastenerKind`, `change_part_2d_hidden`, `ChangePart2dHidden`, `change_part_2d_icon`, `ChangePart2dIcon`, `change_part_2d_locked`, `ChangePart2dLocked`, `change_part_3d_mesh`, `ChangePart3dMesh`, `change_part_anchor`, `ChangePartAnchor`, `change_target_volume_hidden`, `ChangeTargetVolumeHidden`, `change_target_volume_locked`, `ChangeTargetVolumeLocked`, `change_part_kind`, `ChangePartKind`, `connect_grips`, `ConnectGrips`, `connect_kind_compatibility`, `ConnectKindCompatibility`, `create_part`, `CreatePart`, `create_target_volume`, `CreateTargetVolume`, `delete_part`, `DeletePart`, `delete_target_volume`, `DeleteTargetVolume`, `disconnect_grips`, `DisconnectGrips`, `disconnect_kind_compatibility`, `DisconnectKindCompatibility`, `drag_selection_2d`, `DragSelection2d`, `drag_selection_3d`, `DragSelection3d`, `edit_part_2d_text`, `EditPart2dText`, `edit_part_3d_label`, `EditPart3dLabel`, `move_part_2d`, `MovePart2d`, `move_part_3d`, `MovePart3d`, `move_target_volume`, `MoveTargetVolume`, `remove_part_grip`, `RemovePartGrip`, `rename_puzzle5d`, `RenamePuzzle5d`, `replace_fastener_geometry`, `ReplaceFastenerGeometry`, `replace_kind_catalogs`, `ReplaceKindCatalogs`, `replace_part_2d_geometry`, `ReplacePart2dGeometry`, `replace_part_grip`, `ReplacePartGrip`, `rotate_part_3d`, `RotatePart3d`, `rotate_selection_3d`, `RotateSelection3d`, `rotate_target_volume`, `RotateTargetVolume`, `scale_part_3d`, `ScalePart3d`, `scale_selection_3d`, `ScaleSelection3d`, `scale_target_volume`, `ScaleTargetVolume`.

Current matching binding statements:

```rust
pub use crate::standards::v1::subsets::any::schema::mutations::add_part_grip::add_part_grip;
pub use super::add_part_grip::AddPartGrip;
pub use crate::standards::v1::subsets::any::schema::mutations::change_description::use super::change_description::ChangeDescription;
pub use crate::standards::v1::subsets::any::schema::mutations::change_domain::use super::change_domain::ChangeDomain;
pub use crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::use super::change_fastener_kind::ChangeFastenerKind;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::use super::change_part_2d_hidden::ChangePart2dHidden;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::use super::change_part_2d_icon::ChangePart2dIcon;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::use super::change_part_2d_locked::ChangePart2dLocked;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::use super::change_part_3d_mesh::ChangePart3dMesh;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_anchor::use super::change_part_anchor::ChangePartAnchor;
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::use super::change_target_volume_hidden::ChangeTargetVolumeHidden;
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::use super::change_target_volume_locked::ChangeTargetVolumeLocked;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_kind::use super::change_part_kind::ChangePartKind;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_grips::use super::connect_grips::ConnectGrips;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::connect_kind_compatibility;
pub use super::connect_kind_compatibility::ConnectKindCompatibility;
pub use crate::standards::v1::subsets::any::schema::mutations::create_part::create_part;
pub use super::create_part::CreatePart;
pub use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::create_target_volume;
pub use super::create_target_volume::CreateTargetVolume;
pub use crate::standards::v1::subsets::any::schema::mutations::delete_part::use super::delete_part::DeletePart;
pub use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::use super::delete_target_volume::DeleteTargetVolume;
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::use super::disconnect_grips::DisconnectGrips;
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::use super::disconnect_kind_compatibility::DisconnectKindCompatibility;
pub use crate::standards::v1::subsets::any::schema::mutations::drag_selection_2d::use super::drag_selection_2d::DragSelection2d;
pub use crate::standards::v1::subsets::any::schema::mutations::drag_selection_3d::use super::drag_selection_3d::DragSelection3d;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::use super::edit_part_2d_text::EditPart2dText;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::use super::edit_part_3d_label::EditPart3dLabel;
pub use crate::standards::v1::subsets::any::schema::mutations::move_part_2d::use super::move_part_2d::MovePart2d;
pub use crate::standards::v1::subsets::any::schema::mutations::move_part_3d::use super::move_part_3d::MovePart3d;
pub use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::use super::move_target_volume::MoveTargetVolume;
pub use crate::standards::v1::subsets::any::schema::mutations::remove_part_grip::use super::remove_part_grip::RemovePartGrip;
pub use crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::use super::rename_puzzle5d::RenamePuzzle5d;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::use super::replace_fastener_geometry::ReplaceFastenerGeometry;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::replace_kind_catalogs;
pub use super::replace_kind_catalogs::ReplaceKindCatalogs;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::use super::replace_part_2d_geometry::ReplacePart2dGeometry;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_part_grip::replace_part_grip;
pub use super::replace_part_grip::ReplacePartGrip;
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::use super::rotate_part_3d::RotatePart3d;
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::use super::rotate_selection_3d::RotateSelection3d;
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::use super::rotate_target_volume::RotateTargetVolume;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::use super::scale_part_3d::ScalePart3d;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_selection_3d::use super::scale_selection_3d::ScaleSelection3d;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::ScaleTargetVolume;
pub use super::scale_target_volume::ScaleTargetVolume;
```

Actual definition witnesses:

- `add_part_grip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🦀️.rs`
- `AddPartGrip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🦀️.rs`
- `change_description`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangeDescription`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/🦀️.rs`
- `change_domain`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🦀️.rs`
- `ChangeDomain`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🦀️.rs`
- `change_fastener_kind`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangeFastenerKind`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🦀️.rs`
- `change_part_2d_hidden`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangePart2dHidden`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🦀️.rs`
- `change_part_2d_icon`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangePart2dIcon`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🦀️.rs`
- `change_part_2d_locked`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangePart2dLocked`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🦀️.rs`
- `change_part_3d_mesh`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangePart3dMesh`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🦀️.rs`
- `change_part_anchor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangePartAnchor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🦀️.rs`
- `change_target_volume_hidden`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangeTargetVolumeHidden`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🦀️.rs`
- `change_target_volume_locked`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangeTargetVolumeLocked`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🦀️.rs`
- `change_part_kind`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-part-kind/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ChangePartKind`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-part-kind/🦀️.rs`
- `connect_grips`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ConnectGrips`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/🦀️.rs`
- `connect_kind_compatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦀️.rs`
- `ConnectKindCompatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦀️.rs`
- `create_part`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🦀️.rs`
- `CreatePart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🦀️.rs`
- `create_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🦀️.rs`
- `CreateTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🦀️.rs`
- `delete_part`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `DeletePart`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/🦀️.rs`
- `delete_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪦️delete-target-volume/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `DeleteTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🦀️.rs`
- `disconnect_grips`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `DisconnectGrips`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🦀️.rs`
- `disconnect_kind_compatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `DisconnectKindCompatibility`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦀️.rs`
- `drag_selection_2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-selection2d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `DragSelection2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-selection2d/🦀️.rs`
- `drag_selection_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `DragSelection3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/🦀️.rs`
- `edit_part_2d_text`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `EditPart2dText`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🦀️.rs`
- `edit_part_3d_label`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `EditPart3dLabel`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🦀️.rs`
- `move_part_2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `MovePart2d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🦀️.rs`
- `move_part_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `MovePart3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🦀️.rs`
- `move_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `MoveTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🦀️.rs`
- `remove_part_grip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `RemovePartGrip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🦀️.rs`
- `rename_puzzle5d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-puzzle5d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `RenamePuzzle5d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-puzzle5d/🦀️.rs`
- `replace_fastener_geometry`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ReplaceFastenerGeometry`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🦀️.rs`
- `replace_kind_catalogs`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦀️.rs`
- `ReplaceKindCatalogs`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦀️.rs`
- `replace_part_2d_geometry`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ReplacePart2dGeometry`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🦀️.rs`
- `replace_part_grip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🦀️.rs`
- `ReplacePartGrip`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🦀️.rs`
- `rotate_part_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `RotatePart3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🦀️.rs`
- `rotate_selection_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `RotateSelection3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/🦀️.rs`
- `rotate_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `RotateTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🦀️.rs`
- `scale_part_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ScalePart3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🦀️.rs`
- `scale_selection_3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `ScaleSelection3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/🦀️.rs`
- `scale_target_volume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🦀️.rs`
- `ScaleTargetVolume`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🦀️.rs`

### domain-primary-123

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/🔺️diff/🦀️.rs`

Incoming bindings: `Puzzle5dDiff`, `Puzzle5dFastenersDelta`, `Puzzle5dFastener`, `Puzzle5dSnapshot`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dFastenersDelta;
use crate::Puzzle5dSnapshot;
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener};
```

Actual definition witnesses:

- `Puzzle5dDiff`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `Puzzle5dFastener`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- `Puzzle5dSnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### domain-primary-129

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs`

Incoming bindings: `NoPresence as Puzzle3dPresence`, `NoPresenceMutation as Puzzle3dPresenceMutation`, `NoPresenceRetirementFactory as Puzzle3dPresenceRetirementFactory`, `no_presence_store_disposer as puzzle3d_presence_store_disposer`.

Current matching binding statements:

```rust
pub use semio_framework_plugin::NoPresence as Puzzle3dPresence;
pub use semio_framework_plugin::NoPresenceMutation as Puzzle3dPresenceMutation;
pub use semio_framework_plugin::NoPresenceRetirementFactory as Puzzle3dPresenceRetirementFactory;
pub use semio_framework_plugin::no_presence_store_disposer as puzzle3d_presence_store_disposer;
```

### domain-primary-132

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs`

Incoming bindings: `inverse_puzzle3d_mutation`, `Puzzle3dMutation`, `Puzzle3dPlaySnapshot`.

Current matching binding statements:

```rust
use crate::standards::v1::subsets::any::schema::mutations::inverse_puzzle3d_mutation;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dPlaySnapshot;
use crate::Puzzle3dPlaySnapshot;
```

Actual definition witnesses:

- `inverse_puzzle3d_mutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `Puzzle3dMutation`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `Puzzle3dPlaySnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📸️snapshot/🌱️play-snapshot-authority-and-codecs/🦀️.rs`

### domain-primary-133

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`

Incoming bindings: `Puzzle3dPlaySnapshot`.

Current matching binding statements:

```rust

```

Actual definition witnesses:

- `Puzzle3dPlaySnapshot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📸️snapshot/🌱️play-snapshot-authority-and-codecs/🦀️.rs`

Names not explicitly imported here at initial checkpoint: `Puzzle3dPlaySnapshot`. See concrete consumed-vs-unused dispositions above.

### domain-primary-3

Source: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`. Incoming change removes a group of imports only. Retained H split scopes may require those bindings; do not apply aggregate import deletion blindly. No new executable body is supplied by this range.

### domain-primary-34

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`. Incoming change removes a group of imports only. Retained H split scopes may require those bindings; do not apply aggregate import deletion blindly. No new executable body is supplied by this range.

## Independent Duplicate-Binding Checkpoint

A second lexical audit counted only top-level `use` statements, comparing current local binding multiplicity with each immutable H and S. The following bindings each occurred twice current versus once in H and once in S at this checkpoint. This establishes introduced duplicated imports rather than inherited authored S duplicates. Exact same-name imports are reviewed in their actual top-level scope; bodies are not changed. Repairs may supersede this snapshot.

| ID | Actual receiver | New duplicated local bindings |
|---|---|---|
|domain-primary-62|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/↩️inverse/🦀️.rs`|`Puzzle2dMoveNodePlan`, `Puzzle2dMoveNodePreparationCursor`, `Puzzle2dMoveNodePreparationStep`, `Puzzle2dSnapshot`|
|domain-primary-63|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/📸️candidate/🦀️.rs`|`Puzzle2dMoveNodeDisposition`, `Puzzle2dMoveNodePlan`, `Puzzle2dMoveNodePreparationCursor`, `Puzzle2dMoveNodePreparationStep`, `Puzzle2dSnapshot`|
|domain-primary-68|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`|`Puzzle5dScene`|
|domain-primary-71|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️set-grid-spacing/🦀️.rs`|`PUZZLE5D_GRID_SPACING_MAX`, `PUZZLE5D_GRID_SPACING_MIN`|
|domain-primary-72|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✋️set-lod-manual/🦀️.rs`|`PUZZLE5D_LOD_SLIDER_MAX`, `PUZZLE5D_LOD_SLIDER_MIN`|
|domain-primary-74|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs`|`Puzzle5dActionCtx`|
|domain-primary-76|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-voxel-dims/🦀️.rs`|`PUZZLE5D_VOXEL_DIM_MAX`, `PUZZLE5D_VOXEL_DIM_MIN`|
|domain-primary-78|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-snapshot/🦀️.rs`|`Puzzle5dDocument`, `PUZZLE5D_SCHEMA`|
|domain-primary-80|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚧️set-brush-placement-contact-tolerance/🦀️.rs`|`Puzzle5dActionCtx`|
|domain-primary-82|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤏️set-grip-show/🦀️.rs`|`PUZZLE5D_GRIP_SHOW_ALWAYS`, `PUZZLE5D_GRIP_SHOW_SELECTED`|
|domain-primary-83|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️set-grip-direction/🦀️.rs`|`PUZZLE5D_GRIP_DIRECTION_INWARDS`, `PUZZLE5D_GRIP_DIRECTION_OUTWARDS`|
|domain-primary-88|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`|`ui_label`, `puzzle5d_part_display_label`, `Puzzle5dInteractionSnapshot`, `Puzzle5dScene`, `part_scale_json`, `puzzle5d_grip_full_id`, `target_volume_flat_rect`, `target_volume_scale_json`, `Puzzle5dDocument`, `Puzzle5dFastener`, `Puzzle5dGrip`, `Puzzle5dPart`, `Puzzle5dTargetVolume`, `PUZZLE5D_GRANULARITY_FASTENER`, `PUZZLE5D_GRANULARITY_GRIP`, `PUZZLE5D_GRANULARITY_PART`, `PUZZLE5D_GRANULARITY_TARGET_VOLUME`, `PUZZLE5D_PLAY_CONTROLLER_ID`|
|domain-primary-92|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🖌️brush/🦀️.rs`|`Puzzle5dDocument`, `Puzzle5dFastener`, `Puzzle5dPart`|
|domain-primary-94|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`|`close_app`, `dispatch`, `meta`, `projection_of`, `render_body`, `window_view`|
|domain-primary-97|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/◻️2d/🦀️.rs`|`Puzzle5dSnapshot`, `Puzzle5dPart`|
|domain-primary-100|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs`|`PUZZLE_5D_SCHEMA`, `Puzzle5dKindCompatibility`, `Puzzle5dMeta`, `Puzzle5dPart`, `Puzzle5dTargetVolume`|
|domain-primary-103|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs`|`PUZZLE_5D_SCHEMA`, `Puzzle5dKindCompatibility`, `Puzzle5dMeta`, `Puzzle5dPart`, `Puzzle5dTargetVolume`|
|domain-primary-106|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs`|`Puzzle5dSnapshot`, `Puzzle5dGrip`, `Puzzle5dPart`, `Puzzle5dPartAnchor`, `Puzzle5dScale`|
|domain-primary-107|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`|`PUZZLE_5D_SCHEMA`, `Puzzle5dKindCompatibility`, `Puzzle5dMeta`, `Puzzle5dPart`, `Puzzle5dTargetVolume`|
|domain-primary-109|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`|`Puzzle5dSnapshot`, `Puzzle5dTargetVolume`|
|domain-primary-110|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-selection2d/↩️inverse/🦀️.rs`|`move_part_2d`|
|domain-primary-111|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-selection2d/🦀️.rs`|`puzzle5d_selection_number`|
|domain-primary-112|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/↩️inverse/🦀️.rs`|`puzzle5d_selection`|
|domain-primary-114|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-selection3d/🦀️.rs`|`puzzle5d_selection_number`|
|domain-primary-116|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/🔺️diff/🦀️.rs`|`puzzle5d_selection`, `puzzle5d_selection_outcome`|
|domain-primary-117|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-selection3d/🦀️.rs`|`puzzle5d_selection_triple`|
|domain-primary-119|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/🔺️diff/🦀️.rs`|`PUZZLE5D_FLAT_TO_WORLD`, `puzzle5d_selection_outcome`|
|domain-primary-120|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️drag-selection3d/🦀️.rs`|`puzzle5d_selection_triple`|

Qualification nuance: the current P5D editor facade explicitly reexports entities, view, catalog and interaction. A root import through those exports is not itself a missing-definition defect. The defect is a duplicated same-name binding when the current H leaf import already supplies it, or the concrete repeated `::crate::` qualification documented above.
## Fresh Recheck After Repairs

The earlier checkpoint findings above were repaired. Independently reread all 69 nonempty import packets across 68 actual files. A lexical brace-scope check, masking comments/literals and including nested module use statements, found zero duplicate same-scope import bindings; current import lines contain no repeated `::crate::` qualification. All previously missing consumed bindings in67/70/79/92/93/121 occur in actual receiver imports exactly once. Every recorded domain import operation after-hash matches fresh current bytes. IDs100/103 nested codec duplicates were additionally caught and repaired before this recheck.

ID129 current whole Rust owner independently equals immutable S after newline normalization, preserving exactly the four framework NoPresence aliases; root-primary-body-preservation-repeat-695.md owns the accompanying presence-body/schema disposition. The current diff declaration at schema/diff line227 emits Puzzle5dKindCompatibilityDelta through protocol::list_delta!, so121 uses the actual emitted diff owner rather than an invented foreign type.

Deletion-only3 retains H imports required by actual split test owners; its context/body merge is separately root-owned. Deletion-only34 retains InteractionView/json/Value once because current H bodies consume them, while unused EphemeralEmit is omitted. This closes the concrete import findings; it does not sign off unrelated root body/context packets or whole-merge behavior.

### Durable Final Per-ID Dispositions

| ID | Final binding disposition |
|---|---|
|domain-primary-25|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-26|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-27|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-28|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-29|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-30|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-33|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-41|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-59|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-60|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-61|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-62|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-63|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-66|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-67|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-68|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-69|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-70|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-71|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-72|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-73|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-74|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-75|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-76|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-77|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-78|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-79|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-80|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-81|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-82|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-83|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-84|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-87|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-88|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-89|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-92|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-93|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-94|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-96|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-97|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-98|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-99|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-100|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-101|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-102|nested current H import brace restoration|
|domain-primary-103|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-104|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-105|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-106|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-107|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-108|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-109|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-110|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-111|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-112|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-113|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-114|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-115|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-116|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-117|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-118|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-119|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-120|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-121|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-122|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-123|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-129|root presence whole-owner reconciliation; no concurrent import write|
|domain-primary-132|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-133|current defining-owner bindings reconciled; unused incoming imports omitted|
|domain-primary-3|S removes obsolete monolithic imports. Current H crate::context and plugin imports preserved and deduplicated by coordinator with actual mounted-context preservation; no duplicate local incoming one-field context remains.|
|domain-primary-34|S import deletion composed with H body: InteractionView/json/Value remain consumed and imported once; removed EphemeralEmit has no remaining unqualified use.|
