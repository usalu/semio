# 🔎 Live Round 6: Focus Sweep (Keyboard Modality)

This is the elements story site after web Round 5. The page was in keyboard modality, so `:focus-visible` matched and the focus checks ran for the first time. Earlier rounds silently skipped focus because the modality was pointer. The run covered 126 stories and 354 targets and found **86 failing checks**.

The issues fall into these classes:

1. **Browser default focus outline (`1px auto`).** There is no box and no suppression on these:
   - Icons/IconAnimations buttons (30)
   - HistoryTable/Selectable `div[role=button]` rows (6)
   - Diagram `div[role=group]` nodes (Diagram/LargeSkeleton 12, Diagram/Skeleton 5, Skeletons/Diagram 5)
   - Layout/Mobile "Panel" button
   - Tabs `[role=tabpanel]` with a `1px solid` red outline
2. **Focus leaks to ancestors and leaves sides open** on panel tabs: PanelTop*/PanelBottom*/PanelNestedTabs, Layout/Mobile navbar buttons and treeitem, Tabs/Default trigger, MenuItem/StateMatrix "Ready". Hover on the same controls is clean.
3. **Table rows (`tr`) under focus.** The left and right sides are open over the row height.
4. **Layout/Default** "Fullscreen" and "Properties" under focus: the right side is open over the full height.
5. **Hover regressions:**
   - IconSelector combobox trigger: top, left and right open, and the state leaks 3 ancestor segments.
   - PresenceBar `div[role=listitem]`: top, bottom and left open, and the state leaks 3.
   - IconSelector "Import file…": no stated outline.
6. **Void element.** Compose/Rows `input#r3-row-c` hover gets no stated outline.
