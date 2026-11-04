# Foundation Review

Independent static review found two issues:
- Oversized ContextMenu keyboard focus was invisible because focus used preventScroll and never revealed active rows. Lead added bounded fusion-scroller scroll deltas plus a regression test.
- Forced-colors DOM seams retained custom hover colors. Lead added matching system Highlight rules for hovered cells, preceding-cell seams and frames.

Additional lead integration correction: runtime appends overlay SVG as a group child, so CSS last-child no longer identifies the final cell. Shared-cell selectors now identify the final participating cell rather than the final DOM node. Lead added structural CSS assertions.

Loading/waiting original pseudo rings are now suppressed only when every outer side has replacement SVG ownership, avoiding duplicate effect seams.

Reviewer inspected resolver/schema/tests/class recipes/CSS/menu code; no other foundation findings. Runtime compositor review tracked separately in compositor-review.md.
