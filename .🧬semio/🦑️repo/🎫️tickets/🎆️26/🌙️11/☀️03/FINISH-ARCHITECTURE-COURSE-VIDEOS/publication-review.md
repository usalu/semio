# Publication Readiness Review

The WindowChrome compositor sets root pending before scheduling and clears it only after a complete commit. Its motion map is reconciled against live CSS transitions. The transition-only hypothesis was disproved by an exact Ventilation_01 restore packet with no live CSS transitions and persistent root pending.

Coordinator code review identified that a thrown commit could leave pending behind; the UI owner checked the actual packet instead of treating that hypothesis as proven. Trace evidence showed repeated complete commits followed by pointerover/pointerout rescheduling on the body while hit targets changed beneath the stationary real cursor. The pending flag therefore reflects actual repeated work.

The owner is checking whether capture should park its real mouse outside the viewport while the authored cursor is rendered separately, and whether compositor deduplication should use effective registered hover owners rather than descendant event targets. Legitimate hover-owner changes must remain observable. Strict readiness criteria remain in force. Actual full focus and restore packets must pass before publication retry.


The effective-owner dedup regression passed, but the full restore still failed. The actual failing restore frame30 contains the authored cursor at650,342; moving it offscreen would discard intended interactions and is rejected. Events alternate between the footer and an adjacent DIV with different owner memberships. The next retained trace samples their bounds, transforms and animation play states/times. It must distinguish moving geometry from noninteractive overlay hit-target feedback; no hypothesis is accepted without trace evidence.


The trace measured a2px footer position alternation with fixed neighboring bounds. All keyframes were paused at identical time3023.2558ms, ruling out advancing document animation time. The exact source is OverviewCard applying hover:-translate-y-0.5 to its entire clickable button; hover moves its own hit box, which triggers pointerout and reverses the lift. The owner is moving visual lift to an inner paint wrapper while retaining stable button geometry, focus, keyboard semantics and accessible content. Boundary-hover and full authored focus/restore regressions are required before publication retry.
