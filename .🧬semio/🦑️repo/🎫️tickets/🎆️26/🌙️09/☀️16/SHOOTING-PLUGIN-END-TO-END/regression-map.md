# Native Regression Entry Map

Source-only mapping verified from Cargo manifests, library entries, mirror mounts and cfg(test) modules. The raw native-cargo.log was read once; its tail contained capacity waiting, not a completed Rust result. No test/build started by this audit.

| Regression | Cargo package (--lib) | Expected fully qualified test name |
| --- | --- | --- |
| Sun producer | semio-s-artifact-shooting-shooting-tests | mirror::standards::v1::subsets::any::schema::tests::icon_render_request_respects_sun_enabled_fixture |
| Transform producer | semio-s-artifact-shooting-shooting-tests | mirror::standards::v1::subsets::any::schema::tests::icon_render_request_forwards_asset_transform_fixture |
| Captured camera | semio-s-artifact-shooting-shooting-shooting | mirror::editor::shooting::component::unit_tests::new_shot_keeps_captured_camera_after_viewport_navigation |
| Retained camera/selection presence | semio-s-artifact-shooting-shooting-shooting | mirror::editor::shooting::component::unit_tests::retained_camera_and_selection_publish_presence_and_preserve_each_other |
| Saved camera/add-shot presence | semio-s-artifact-shooting-shooting-shooting | mirror::editor::shooting::component::unit_tests::saved_camera_load_and_new_shot_publish_the_same_ephemeral_presence |
| Preview/PNG instance, sample points and transformed fit bounds | semio-renderer-wgpu-icon-host | icon_render::icon_render_tests::icon_subject_transform_matches_neutral_fixture |
| SVG projection and fit versus baked world geometry | semio-renderer-wgpu-icon-host | icon_render::icon_export::svg::tests::svg_export_applies_subject_transform_before_projection |

## Owning Entries

Artifact workspace: ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml. Producer tests manifest relative to artifact root: 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml. Its lib path is 🦀️.rs, loading 🪞️mirror/🦀️.rs and the real schema unit file under schema::tests.

Camera/presence manifest relative to artifact root: 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml. Its lib path is 🦀️.rs; mirror mounts the real editor under editor::shooting::component, which mounts unit_tests under cfg(test).

Host manifest: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/💠️icon-host/📦️packages/🦀️rust/Cargo.toml. Its lib path ../../🦀️.rs mounts the real Scenes source as icon_render. That source mounts icon_render_tests and icon_export; the latter mounts svg, whose cfg(test) tests module owns the SVG regression.

## Concrete Acceptance Gap

The corrected three-package shooting command covers the first five tests, provided the actual runner prints their passing names. Root-only testing does not enable dependency cfg(test). Those three selectors do not execute the two host tests. A shell-doc-open cargo check compiles dependencies but does not run their cfg(test). Native owner was notified to explicitly select semio-renderer-wgpu-icon-host in the eventual owned host test run. No Rust execution success is asserted here.

The preview/PNG test checks shared world-scene instance forwarding, point transforms and eight-corner bounds math without GPU rendering. SVG compares actual vector output for fit=false and fit=true against baked world geometry. Actual PNG pixels remain separate runtime acceptance.

Cross-reference typescript-verification.md: normal registered Bun ingress passed 30 tests, 154 expectations, eight files in 239ms, including Three/AJV/XState oracles. This does not substitute for Rust execution.
