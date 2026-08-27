# Graph Slider Accessible Names And Identity

## Delivered Scope

The shared DAG slider overlay now publishes the exact `DagNodeSpec.name` as `label`. The React overlay validates named numeric rows, passes that caption to the actual shared Slider accessibility contract, and gives each control a stable DOM root ID derived from the JSON-encoded pair of graph scope and widget ID. Both production consumers supply their window/controller/surface scope. Encoding the pair together avoids delimiter collisions and preserves Unicode identities.

No English fallback label was added. Captions are already resolved/user-authored document text; the fixture covers English `Radius` and German `Außenradius (m)`. Empty, whitespace, non-string, malformed-array, duplicate-ID, and invalid numeric rows do not produce unnamed controls. The existing Slider owns keyboard interactions, focus, disabled state, min/max/step, and pointer gestures unchanged.

Six files changed: shared NodeGraph renderer, existing renderer test suite, existing repository-owned DOM test adapter (rerender/keyUp forwarding), native DAG producer/test, strict schema, and language-neutral fixture. No scripts or runtime dependencies added.

## Test Evidence

Schema and three DOM tests were written before implementation. The canonical renderer quick test produced RED: three new tests failed (unnamed roles and invalid labels accepted), while two existing track/zoom tests passed. The first small-budget attempt expired during module import at 15 seconds. A subsequent 30-second run also expired under concurrent builds; these are not passing runs.

The first post-fix run exposed missing `rerender` and `keyUp` methods in the repository-owned test adapter. They were forwarded through its existing Testing Library boundary. Final canonical command:

`NX_DAEMON=false bun x nx run '@semio-tech/framework-renderer-react:test-long' --skip-nx-cache --args='🧪️index.test.ts --run -t "graph slider"'`

Result: **5 passed, 347 skipped, 352 total discovered**. Third-party `dom-accessibility-api.computeAccessibleName` and Testing Library role/name queries verify actual rendered DOM names. Tests cover both language captions, multiple graph scopes using the same widget ID, stable identity after rerender/value change, ArrowRight/ArrowLeft/Home/End exact values, focus, and disabled inertness. Existing track-only and zoom presentation tests remain green. Targeted diff check passed.

Native source adds `dag_host_slider_overlay_preserves_language_neutral_field_labels` using the same fixture and strengthens `dag_host_slider_overlay_state_json_includes_slider_track`. They are pending the coordinator's Rust lease; no native execution is claimed here. Fresh-WASM browser verification remains with the coordinator.

## Exact Remaining Flow Producer Defect

Meaningful Generator captions are not yet proven end-to-end. The framework FlowHost hexagonal fixture in `🌊️flow/🖥️host/🦀️component.rs` around line 4453 contains `label: Column Height`, `Profile Radius`, and `Side Count`. However, framework `🌊️flow/📄️artifact/🦀️component.rs` defines `Widget::InputSlider` without a label member (around line 192), and `widget_display_meta` hardcodes `Slider` (around line 395). Deserialization therefore discards the original label before DAG publication.

The shared overlay faithfully preserves the DAG name but cannot reconstruct discarded labels. Fixing that requires retaining typed Flow label metadata through its schema, constructors, copier, and canonical visitor; that source is concurrently owned by the Flow executor. This was reported to the coordinator rather than replacing real captions with a generic localized fallback or claiming full Generator label closure.
