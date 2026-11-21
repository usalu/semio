# OS web border verification

The document close guard and staged action dialog register their outer edges with the shared border renderer. A flush action row shares that frame. The staged dialog retains native modal behavior, and long content scrolls inside the frame. The close guard still focuses the safe action and supports Escape. In stacked notices the text cell fills the available room, aligning the action with the stack edge even when notices have different intrinsic widths.

## Checks

- The Chrome harness reproduced gaps of 7.5–8.36 CSS pixels on the close guard, 1.5–1.66 CSS pixels on the staged dialog, and 1 CSS pixel on the tablet stall-only notice stack. The corresponding probes returned zero pairs after the fixes.
- Chrome returned zero pairs for the normal and failed close guard, normal and pending staged action dialog, German staged error dialog, combined install-and-stall notices, and stall-only notices at 1440×900, 768×1024, and 375×812.
- The saved-state and notice suites passed all 9 tests. The native staged dialog border test passed.
- At 768×1024 the 72-view OS harness showed no native chrome borders or shared SVG strokes thicker than 1 CSS pixel. The SVG check sampled 1,382 painted paths. Static 2-pixel candidates belong to canvas graphics for selection, navigation previews, or presence.
- Chrome verified that a staged dialog with oversized content remains a native modal. Its inner body scrolled while the action row stayed flush with the fixed dialog bottom.

The shared Table renderer now owns the Task Manager's horizontal boundaries. Its sticky action cells host a flush shared row for the vertical sides and button dividers. That removed the remaining 3-pixel gap between the previous row and the next action strip. Chrome returned zero pairs at rest and on Suspend hover at 1440×900, 768×1024, and 375×812. The 28 Task Manager tests passed.

The shared full-width panel tab row extends its last tab into leftover space smaller than one chip. At 768×1024 the Settings row's last tab previously stopped 4.891 pixels before the frame. Chrome measured zero gap at 768 and 820, preserved an intentional 79.297-pixel opening at 1440, and preserved 41 pixels of scrolling in a constrained 300-pixel row. This removed the short same-row pair from the Settings probe. The probe still reported the two separate floating-panel and canvas edges 38.59 pixels apart. The shared PanelTabBar test suite passed all 12 cases.

## Demonstrator at 375×812

Chrome's narrow fine-pointer viewport uses the scrolling grid. At scrollTop 0 its second-row card caps paint at y748, and the fixed partner footer paints at y784. The probe reports four 36-pixel pairs between these overlapping layers. At scrollTop 64, 256, and 517 the pairs disappear. The footer box stays at y783 while the second-row card box moves from y716 to y652, y460, and y199. The footer contains three labelled logo links in both fine-pointer and touch mode. The fine-pointer probe returned zero pairs on opening the Generator. The app layout and footer source are unchanged.

Chrome's mobile touch emulation uses the swipe layout. The introduction, overview, opened Generator, and returned overview each had zero pairs after the compositor settled.
