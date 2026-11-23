# L19 Static migration of the 5D crates and `semio-hub-puzzle`

No cargo build was run: `/home` had 3.5G free (threshold 14G) and other lanes were compiling. Everything below is static (grep against the current framework definitions), so none of it is compile-verified.

## Method
The 2D/3D lane drift families (l1, l1a, l1b, l1c, l1d) were checked one by one against the 5D workspace (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d`) and `🌎️hub/🧩️compositions/🧩️puzzle`:

| Family | 5D | hub |
|---|---|---|
| `cfg_attr(test, serde)` invisible across crates; `testkit` feature and dev-dep edges | already `any(test, feature = "testkit")`, `testkit = []` features and dev-dep edges present | n/a |
| `protocol::Mutation::<T>::diff/inverse` (trait, now `ColdMutation`) | 6 sites in 2 tests: FIXED | none |
| retained-clone grant API to effect API (`advance`, `close_step`, `take`) | production and `every-example` test already on `close_original_effect_step` / `advance(.., effect)`; `close_original_retained_step(cx, demand, \|_\|..)` calls match the current grant-closure signature | one stale site: FIXED |
| `ControlledRetirement::step(grant)` to `step(body, effect)` | none | the one stale site above |
| `command_from_action` 3-arg form, `original_action_command_producer` | already 3-arg, plugin declares producer (5D is the model the 3D lane followed) | n/a |
| `SqliteSnapshotControl` / `original_owners::{with_sqlite_recipient,with_encoding,with_decoding}` | already migrated; all used helpers exist in `semio-framework-test-original-snapshot-owners` | `io_run` signature matches |
| `OriginalFixture::new/drive/fixture_stock/drain_outboxes` (puzzle-jobs `testing`) | signatures match | n/a |
| board-host helper moves, `load_board_snapshot_json`, `generation().0`, `pump_one`, `admit_owned` | no occurrences | none |
| `DslValue::from(&json)` | one site uses `serde_json::Value`; `From<&serde_json::Value> for DslValue` still exists in `🌳️intrinsic`, so valid | none |
| hub `PuzzleApps` vs 5D `ArtifactApps` (`VcsArtifactApp<EditorApp<..>, Puzzle5dMembers>` = `EditorSurfaceApp<..>`) | consistent | consistent |
| `assert_viewer_never_mutates`, `assert_editor_and_viewer_share_dialect`, `fixture_identity`, `receive_to_completion`, `ReceivingStock`, guest-io actor/job methods | n/a | all exist with matching shapes |

## Edits
1. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️host-admission/🦀️.rs`: `protocol::Mutation::<Puzzle5dSnapshot>::{inverse,diff}` to `protocol::ColdMutation::<Puzzle5dSnapshot>` (3 sites; `Mutation` is now metadata only, `diff`/`inverse` live on `ColdMutation`).
2. `.../✏️editor/🧪️tests/🔬️gesture-kinds/🦀️.rs`: same change, 3 sites.
3. `🌎️hub/🧩️compositions/🧩️puzzle/🧪️tests/🧩️guest-io/⚙️driver/🦀️.rs` (`Actor::retire`): `close_original_retained_step(cx, demand, |grant| owner.step(grant)..)` replaced by `close_original_effect_step(cx, demand, |effect| owner.step(body, effect))` with `body = cx.retained_grant().maximum_copy_bytes`, because `ControlledRetirement::step` now takes `(body, effect)` and returns `OriginalRetirementPoll` (same shape the generation3d crates use).

## Unverified until compile
- Everything: no `cargo check --all-targets` has ever run on 5D/hub in this ticket. Static greps cannot see method-level drift (for example other signature changes in `semio-plugin`/`semio-framework-job` made by concurrent agents today: `admit_original_return_origin` now returns `bool`, new `exclude_original_birth`, `retire_failure` on the fixture caller). No 5D/hub caller of those was found.
- `play-snapshot` hand bridges `Mutation<Puzzle5dPlaySnapshot>` / `ColdMutation` impls match the current trait definitions by reading only.
- Test runtime behaviour (budgets, retirement receipts) is untested; 3D needed production fixes found only by running tests.
- Recommended command when disk >= 14G and RAM >= 5G free: `L2_SIZE=4g ./l2-native.sh x 'cd "$P5" && cargo check --all-targets --jobs 2 --offline' > 🗑️generated/l19-5d.log 2>&1`, then the hub `cargo test -p semio-hub-puzzle --lib` and `--test guest-io`.
