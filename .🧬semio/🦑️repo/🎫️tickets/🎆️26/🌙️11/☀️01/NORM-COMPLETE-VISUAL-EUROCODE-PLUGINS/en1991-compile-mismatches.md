# EN 1991 compile mismatches

`bridge_lanes` is on `En1991Snapshot` (`🧬️schema/📸️snapshot/🦀️.rs:258`) and on `En1991Artifact` (`🧬️schema/🦀️.rs:191`) and `En1991Diff` (`🧬️schema/🔺️diff/🦀️.rs:1046`). `BridgeLane` is `id`, `width`, `offset`, `lm1_lane` (`📸️snapshot/🦀️.rs:701-708`). Every `BridgeLane { ... }` literal (scene unit test `:946`, compliance-report `:873`, `:1048`, `:1385`, live scene `:89`, sqlite decode `:265`) passes those four fields. The stale run-14 parse error was a missing comma before `bridge_lanes` in the sqlite `Self` literal; that comma is present at `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:335`.

No E0063 missing-field mismatch. The 114 snake_case fields of `En1991Snapshot`, `En1991Artifact`, and `En1991Diff` are the same set. Full literals that name every field match that set: `example` (`📸️snapshot/🦀️.rs:356`, `annex` is shorthand), `to_snapshot` / `from_snapshot` (`🧬️schema/🦀️.rs:257` and `:376`), sqlite `Self` (`sqlite/📸️snapshot/🦀️.rs:331`, collection names are shorthand), `En1991Diff::apply` (`🔺️diff/🦀️.rs:1384`). Other snapshot literals use `..`.

## Will not compile (wrong value type, not a missing name)

`En1991Diff` defines the last four collections as `Option<…Delta>`, and `inverse` passes `vec![]`.

| Struct | Field | Definition | Use that will not compile |
| --- | --- | --- | --- |
| `En1991Diff` | `fatigue_models` | `🧬️schema/🔺️diff/🦀️.rs:1098` | `🧬️schema/🔺️diff/🦀️.rs:1732` (`inverse`, `vec![]`) |
| `En1991Diff` | `support_collisions` | `🧬️schema/🔺️diff/🦀️.rs:1100` | `🧬️schema/🔺️diff/🦀️.rs:1733` |
| `En1991Diff` | `local_fires` | `🧬️schema/🔺️diff/🦀️.rs:1102` | `🧬️schema/🔺️diff/🦀️.rs:1734` |
| `En1991Diff` | `snow_guards` | `🧬️schema/🔺️diff/🦀️.rs:1104` | `🧬️schema/🔺️diff/🦀️.rs:1735` |

`apply` at `🔺️diff/🦀️.rs:1495-1498` writes the same `vec![]` into `En1991Snapshot`, where those fields are `Vec<…>`, so that literal type-checks and drops the four collections.

## Separate parse error

`En1991Outline::compute` closes the sum at `rotating_machines` (`🧾outline/🦀️.rs:153`) and then has stray `+ snapshot.dust_explosions.len()) as u32;` lines through `:160`. That is not a missing struct field.

## Files compared

`🧬️schema/📸️snapshot/🦀️.rs`, `🧬️schema/🦀️.rs`, `🧬️schema/🔺️diff/🦀️.rs`, `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`, `🧬️schema/💡️inferences/🧾outline/🦀️.rs`, `📚️examples/🧬️subjects/🦀️.rs`, `🎬️scene/🧪️tests/🔬️unit/🦀️.rs`, `🎬️scene/🧪️tests/🔬️live/🦀️.rs`, `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`. Paths are under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/`.