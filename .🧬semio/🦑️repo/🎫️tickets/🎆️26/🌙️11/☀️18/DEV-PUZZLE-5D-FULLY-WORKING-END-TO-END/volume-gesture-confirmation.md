# Target volume gumball

The renderer sends the volume ID, transform mode, before pose and after pose on every release. The 5D integration test omitted both the mode and before pose, so the gesture parser returned no motion. Its assertions now exercise translation, rotation and scale through the same three requests the world renderer sends, then check that a locked volume stays in place. The half-turn quaternion uses a tolerance for the cosine of π/2.

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️target-volumes/🦀️.rs`
