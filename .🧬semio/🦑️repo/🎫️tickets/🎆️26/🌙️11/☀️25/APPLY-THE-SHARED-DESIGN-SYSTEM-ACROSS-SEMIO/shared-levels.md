# Shared owner and overlay behavior

The base, window, pane, panel, dialog and menu roles come from the shared styling contract. A host publishes both the level and its fill. This distinction matters in two cases. A transparent descendant can keep using the existing paint, even through another layout wrapper. A floating surface can still paint its own glass when it sits over another menu and both surfaces are at the top of the level scale.

| Relationship | Behavior in the maintained renderers |
| --- | --- |
| Ordinary content inside an owner | Text, tables, form rows, inputs, empty states and loading bodies leave the owner visible. React publishes the owner's level to nested components. Terminal cells redraw the owner's opaque color. WebGPU omits the redundant draw. |
| Independently positioned editor content | A graph node or framed ink block that must occlude a connector or grid uses the opaque color of the containing host. Authored strokes, images, selection and domain colors retain their meaning. |
| In-content chip | The chip uses the next level, capped at menu. Nested content inherits its glass scope. Web sources apply the same rule to tab strips, badges, annotations and floating editor tools. Print applies it to title and number chips. |
| Window and menus | The window owns its content and the shared frame owns its perimeter. A dropdown or context menu owns menu glass. A modal owns dialog glass and its backdrop is a separate veil. |
| Print and motion | The physical print margin is base, and the framed text area is window. Camera frames and slide content use the presentation window. A terminal or opaque print target can reproduce a surface role without backdrop blur. |

The shared React suite passed 1,739 tests across 85 files. The OS DOM suite passed 124 across eight files. The ink editor test mounts under a window, checks that the host is transparent and its positioned blocks cover the grid, and opens a real text editor to check its pane glass. The full terminal unit suite passed 101 cases. The native WebGPU overlay and panel packages also exercise the production document consumer at compilation. The engine-specific regression results are in [wgpu-engine-levels.md](wgpu-engine-levels.md).

Native Animate compares the complete painted picture before reusing a frame. The comparison includes the resolved surface color, camera transform, paths and strokes, raster size and frame rate. Consecutive identical pictures share one render, and the partial movie also carries the run length in its cache key. The regression changes a centered square from red to blue without changing its outline. [Native encoder and color checks](animate-native-encoder.md)
