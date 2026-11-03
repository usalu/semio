# Storybook Visual Repairs

## Delivered

| Reported issue | Underlying repair / browser acceptance |
| --- | --- |
| Badge text cramped | Replaced undefined spacing utilities with real semantic tokens:6.4px horizontal padding,3.2px icon/text gap; contained SVGs. |
| Catalogue final connector continues | Actual Tree renderer terminates the final sibling's incoming guide at its elbow. Catalogue inherits the fix without a story override. |
| Tree has the same tail | Correct sibling/ancestor guide ownership, nested terminal branches, upward flow and virtual-window tails. Real browser expanded Kit/Types/Capsules/Variants/Balcony/Bases and checked final and continuing guide geometry. |
| Label text/input misaligned | Measures intrinsic caption plus explicit control minimum instead of allocated track widths.280/360/640px rows remain inline with matching centers;128px stacks without artificial value-column indentation. Native input-label association and focus work. |
| ActionGroup icons overflow |22.4px shell,20.4px contained items,9.6px icons with positive clearance; removed conflicting minimum height and fixed SVG utility specificity. No clipping mask. |
| Floor grid vanishes below camera | Scene explicitly uses a double-sided grid. Added BelowFloor story. Real WebGL screenshot comparison sees floor-region pixels change when the below-camera grid is toggled off. |
| PresenceBar cramped | Shared24px avatars with positive3.2px peer gaps instead of8px overlap; overflow chip uses the same dimensions. |
| TableAvatar cramped / inconsistent |24px border box with text/image/icon clearance. Shared class composer now keeps font sizing independent from arbitrary selected-state text colors, preserving11.2px/16px initials in all states. |

All sizing uses existing semantic styling authority; the global spacing scale and generated CSS were not hand-edited. Existing control interactions, focus/activity markers and Tree drag/drop remain intact.

## Validation

Final Chromium runs in the local validation environment:

- **Cold dev:240 UI/styling variants, zero failed.**
- **Static build:240 variants built and rendered, zero failed.**
- Five browser tests pass in both environments: complete catalog, Button autodocs, measured geometry, expanded connector endpoints, and real below-floor WebGL pixels.
- Full manager on the refreshed network-bound6010 static server renders successfully, with no page/resource errors and zero WebSockets.
- Reviewed before/after screenshots for Badge, Label, ActionGroup, PresenceBar and TableAvatar. Final browser metrics agree with the table above.

Focused suites:

| Suite | Passed |
| --- | --- |
| Storybook scope/catalog/preparation |28 |
| Explicit subject contract/CSF |19 |
| Tree components, rerun after Label integration |71, including12 new connector checks |
| Label fit/component cases |4, including8 neutral layout vectors |
| Avatar and presence component cases |8 |
| Badge/ActionGroup contract |2; plus3 existing styling-geometry tests in agent validation |
| Class composition |37 feature vectors plus12 independent typography-oracle cases |
| Scene grid |3 |

Test-first failures reproduced missing Badge padding, shell/item mismatch, avatar dimensions/overlap, Label layout fit, connector termination/ancestry, arbitrary color erasing font size, and below-floor grid visibility. Independent references include real browser layout/WebGL, d3-hierarchy, CSS calculation/schema libraries, Three geometry/raycasting and isolated test-only tailwind-merge3.4.0. No production dependency was added.

One first integration run exposed an ambiguous test selector matching a hidden Catalogue item; the selector was corrected to the visible tree caption, then complete dev/static suites passed. No runtime error was filtered or retried into success.

## Inspection and Operations

Updated read-only Storybook is intentionally left running at **http://localhost:6010**, bound to all IPv4 interfaces. Refresh the existing forwarded page. It has no live-reload/file-writing server endpoints. Keep the forward private.

Current PID:215494. Runtime files: `🗑️generated/s`, `🗑️generated/storybook-static-inspect.log`, and `🗑️generated/storybook-inspect.pid`. Those are retained only while the user inspects it; stop the server to remove static outputs, then remove its logs/PID file. Temporary audit/test outputs and before/after captures were removed after review.

Audit runners now use short, separate generated subdirectories rather than deleting the live server's artifacts. Relative Playwright output directories resolve against the workspace root instead of being accidentally nested under the config directory. Current live output contains296 files; maximum absolute path length248, none exceed256.

## Detailed Inputs and Limits

See `spacing-source-diagnosis.md`, `badge-action-spacing.md`, `tree-connectors.md`, `label-alignment.md`, `avatar-spacing.md`, `class-composition.md`, and `scene-grid.md` for exact source/test files and reproduction inputs. Browser contracts live in `.storybook/🧪️tests/🧪️visual-layout/🟦️.ts` and its neutral JSON fixture, registered in the existing browser runner.

Frozen root dependency installation and tracked diff whitespace check pass. Full generated-declaration typechecking, full OS/plugin/WASM builds, every locale/device/theme, and external pod-forwarding control-plane health are not newly certified. MCP ticket lifecycle remains unavailable; no ticket state was fabricated.
