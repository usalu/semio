# Right Chip Audit

The clip overview currently places Ansehen in footerLeft. Shared OverviewCard forwards footerRight to WindowChrome footerRightChips. With only a right footer, WindowChrome uses justify-end and wraps its content in one data-window-silhouette-chip element; this is the native right-aligned fused silhouette geometry. Shared OverviewCard stories and component tests already put OpenChip in footerRight.

OverviewCardOpenChip hardcodes a trailing chevron-right after children. Adding a maximize icon as a child without replacing that chevron gives two icons. An optional icon prop retaining chevron-right as its default allows the clip to supply its existing local Icon maximize-2 with no unrelated behavior change.

The overview card itself is a button. Its OpenChip deliberately contains only div/span and icon content: keep this structure so the fullscreen icon shares the card action and does not introduce an invalid nested button. The entire chip remains inside one shared silhouette wrapper.

The expanded VideoPane already puts its restore action inside capRightChips. Adding localized restore text and minimize-2 within the existing button yields one fused right cap with one semantic action. Preserve the fullscreen element id, data-slot, pointer handlers, and active fill so animation hit targets and pressed state remain intact.

Source evidence: shared OverviewCard lines 90–91 and 130–140; shared WindowChrome lines 599–610; clip browser composition VideoPane and Overview. No source files edited by this scout.

## Final Preview Audit

Visually inspected right-chip-overview.png and right-chip-fullscreen.png. All six overview cards show Ansehen followed by exactly one maximize icon in a single bottom-right silhouette chip; the right edge aligns with each card border and no chevron remains. The expanded view shows Wiederherstellen followed by exactly one minimize icon in one upper-right cap, connected cleanly to the window silhouette. No separate icon button outline, duplicate icon, or detached chip is visible. Preview audit passed.
