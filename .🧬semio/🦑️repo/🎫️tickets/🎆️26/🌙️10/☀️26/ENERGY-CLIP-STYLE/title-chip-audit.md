# Title Chip Audit

The opened video control should be one native button containing the window name followed by the minimize icon, inside WindowChrome.titleChips. Remove capRightChips so WindowChrome does not paint a separate top-right controls chip. Keep overview Ansehen chips unchanged.

Evidence: framework WindowChrome renders titleChips inside the painted title cap at lines 513–516 and only renders a separate controls cap when capRightChips, enlarge, or close exists at lines 519–521. windowChromeTitleChipClass inherits modeDockTabClassName and provides transparent, borderless, medium-height geometry. Native OverviewCard chip composition uses inline-flex, items-center, gap-single, and px-single.

Retain the :fullscreen native button ID for the existing measured pointer target. Give the combined button an accessible label including the localized title and restore action, hide the decorative icon from assistive technology, and preserve native button keyboard semantics. Apply pointer pressed state to that button. No nested buttons are needed.

The base modeDockTabClassName limits width to 12rem. Preserve or consciously override this when accommodating longer titles; truncate a text span while keeping the icon shrink-0 so the action remains visible. Shared windowPaneChromeToggleClass adds inset focus-visible styling if a native existing toggle presentation is desired.

## Final Preview Visual Audit

Reviewed title-chip-overview.png, title-chip-opened.png, and title-chip-control.png. All six visible overview maximize icons are embedded immediately after their window names in the upper-left title chips. Every lower-right Ansehen chip is text-only. The opened Physikalische Grundlagen window places its minimize icon inside its name chip; no separate upper-right cap appears. The pressed preview shows the pointer on that embedded icon, with a coherent compact title cap and no clipping or misplaced outline. The reported 22.4 px native cap height is visually proportionate and consistent with the surrounding application chrome. Exact height and native icon-only hitbox behavior require DOM measurement/interaction evidence; still images alone do not establish those values. Visual review passes.
