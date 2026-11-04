# Native Shared Borders Result

## Implemented

Within OS renderer engine Dock/Shell wgpu element targets and their Rust tests only:

- Every live Dock geometry consumer (paint, body placement, cap/drop targeting, resize walk and Shell's resize-axis snapshot) uses zero visual sibling separation. Authored weights, chip measurements, silhouette cutouts, body padding, and centered/clipped 20px resize hits remain intact. Explicit separator arithmetic APIs remain available for their explicit geometry tests; production uses zero.
- Added dependency-free exact logical-segment arbitration matching the shared `border-presentation/segments.schema.json` and its four language-neutral cases: endpoint splitting, normalized reversal, deterministic state/owner/id winners, contributor IDs, invalid-input rejection, hover highest.
- Dock collects silhouette edges and paints resolved seams after all stack chrome, preventing sibling order from replacing hover with active/neutral. Nonshared active silhouette edges stay present; rectangular bottom strokes remain inward.
- Shell chrome items now paint hover before active/normal border state. Navbar control clusters, leading/center/trailing panel-tab rows and footer panel-tab rows use semantic frame plans, not primitive inference. Resolved partial parent/child intervals are supported by the planner. Retained emission remains at most one border primitive per opportunity; hit and icon/grip paths remain unchanged. Missing plans retain ordinary per-item outlines.
- Floating panel insets/column gaps, centered navbar chip geometry, menu overlay routing/scissors, and content menu separators are unchanged. No blanket edge suppression, global registry, runtime dependency, or React/CSS edits.

## Validation Actually Run

All shell commands used `/home/clstb/.local/share/semio/launcher/bin/repo`.

1. `repo tools exec --light python -- <ticket>/native-check.py`: **11 Rust tests passed**. Compiles the actual dependency-free Dock resolver and Shell frame planner by canonical path; covers shared JSON expectations in both orders, native frame JSON cases (siblings, partial hierarchy, detached frames, vertical adjacency), plus invalid-input/unit checks. This does **not** compile the full Dock/Shell integration.
2. `repo tools exec --light bun -- test ./<ticket>/native-oracle.test.ts`: **1 test / 9 assertions passed**. Independent lodash interval composition and Ajv schema validation reproduce the native neutral-frame expected counts and strokes.
3. `repo tools exec --light bun -- test ./🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/segments.test.ts`: **9 tests / 142 assertions passed**, including independent lodash oracle, schema checks and mixed fractional overlaps.
4. Pinned nightly `rustc -Zparse-crate-root-only --edition=2021`, invoked through `repo tools exec --light python`: **24 owned Rust source/test files parsed**, no typechecking or compiler output emission.
5. Scoped `repo tools exec --light git -- diff --check`: passed. Final scoped diff and untracked additions inspected. No modifying Git commands used.

Added but **not executed in the full crate graph**: Dock frame/body/resize-center, single-hover-seam and chip-cutout/inward-outline tests; Shell retained one-output/hit-publication seam test. Existing body-spacing assertions were updated to zero visual separator.

## Build / Runtime Limitations

- First requested Cargo invocation timed out after 120s during repo-managed toolchain preparation, before compilation.
- Second `repo cargo --store=shared --configuration=prototype -- test -p semio-framework-os-renderer-wgpu-dock-laws shared_ --no-fail-fast` failed before compilation during repo native foreign-source capture: **`Materialized absolute path exceeds 256 characters`** (`native-build/units/capture`). New source paths were checked and are below 256 characters.
- `repo tools exec --light cargo -- test ... --offline` also failed before compilation: **`Original compiler ingress requires the repository launcher and authentic enforcing store`**. No enforcement bypass or infrastructure edits attempted.
- **Full integration typechecking, Cargo tests and native/browser runtime verification remain unrun.** Re-run focused Dock laws and Shell retained-chrome tests once repository build capture works, then inspect actual native resizing, outlines and popup visibility.
- Arbitrary retained-widget border producers outside Dock/Shell are not enrolled in a shared semantic registry. Separate navbar-vs-dock paint scopes are not globally composed. Current native navbar chips and floating pane controls retain their intentional positive gaps; joining noncoincident edges would change geometry and was not done. This is not evidence of exhaustive cross-widget parity.

## Changed Source Files

Base `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`:

### Dock

Under `🛰️Dock/🎯️targets/🧊️wgpu/`:
- `🦀️.rs`
- `shared-borders.rs` (new)
- `border-paint.rs` (new)
- `🧮️axis-solving-and-stack-rendering/🦀️.rs`
- `🎯️dock-state-render-and-drop/🦀️.rs`
- `🏷️tab-chrome-and-layout-edits/🦀️.rs`
- `📐️layout-conversion-and-drop-zones/🦀️.rs`
- `🔲️window-silhouette-and-stack-geometry/🦀️.rs`

Under `🛰️Dock/🧪️tests/🔬️wgpu-unit/`:
- `🦀️.rs`
- `shared-borders.rs` (new)
- `🪟️window-instances-geometry-and-silhouette/🦀️.rs`
- `🧭️drag-merge-mobile-and-focus/🦀️.rs`

### Shell

Under `🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/`:
- `🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs`
- `🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs`
- `🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs`
- `🍱️view-chrome/🧱️chrome-paint-and-navbar/shared-borders.rs` (new)
- `🍱️view-chrome/🧱️chrome-paint-and-navbar/border-cases.json` (new)
- `🍱️view-chrome/🧱️chrome-paint-and-navbar/border-cases.schema.json` (new)
- `🪀️view-panels/🧭️navbar-footer-and-overlays/🦀️.rs`

Test `🐚️Shell/🧪️tests/🔬️wgpu-retained-chrome-text-laws/🦀️.rs`.

Ticket inputs/reports retained: `native-check.py`, `native-oracle.test.ts`, `native-feasibility.md`, `native-result.md`. The checker generates only `<ticket>/🗑️generated/native`, removed after validation. Ticket lifecycle remains lead-owned.
