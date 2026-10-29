# Browser Probe Review

2026-10-28. Read-only review of launch-layout.py, browser-journey.json and both browser probes, checked against current owner test selectors and action routes. No browser run performed by this reviewer.

## Actionable Findings

1. **Undo can falsely pass.** The add-frame-then-undo step requires only `after.frameRows === baseline.frameRows`. It records but never asserts the second action submission, creation count or successful Undo handling. A failed second addFrame followed by a no-op undo leaves the baseline unchanged and satisfies the contract. Require successful second submission and evidence that exactly one frame was added before undo, then the exact added stable ID disappears. Avoid introducing a history-affecting panel toggle merely to obtain the pre-undo observation; use an already-open pane, nonmutating mirror/store witness or instrument the existing history operation receipt.

2. **Selection can falsely pass.** The final check requires only a successful click and any selected tree row. Previously selected pages or frames satisfy selected.length even if frame-1 never becomes selected. Record selection before the click and assert the clicked stable row itself has aria-selected=true afterward, ideally after selecting a distinct prior target. Current selector also hardcodes the combined English text `frame-1 Page 1`, so it fails under German or alternate demo labels. Prefer stable element identity and the addressed Artifact tree.

3. **Fault coverage has gaps.** Assertions inspect only contract-listed step faults. artifact-panel-baseline faults are never asserted, nor is the per-surface `hosts[].fault`/phase data that state() already records. Error console lines that do not match the narrow faultLines regex also pass. Require successful mounted surface phases and null surface faults at boot and after each settled action; assert all recorded step faults including baseline. Keep any known benign-message exception narrowly scoped and documented.

4. **The interaction journey remains narrower than end-to-end readiness.** No redo, document reopening/reload persistence, export command or downloaded SVG/PNG/PDF content is tested. Those remain required external validation before claiming the full ticket plan complete. Current JSON fixture describes expected booleans only, rather than independent artifact/state witnesses for these flows.

## Launch and Portability

- launch-layout.py computes the correct workspace from ticket.parents[6], invokes repo by argument array, and pins the current generated catalog. No shell quoting or platform-specific path bug was found in that driver. The driver still assumes repo and provisioned Bun/Playwright Chromium are available through repository tool setup.
- The interaction probe now uses platform-specific undo modifier and portable Chromium WebGPU flags. The separate console-dump probe still contains --use-angle=metal, which is macOS-specific; update that probe too before describing all ticket browser probes as portable.
- Action selectors match current owner tests (`action.<verb>` and suffix `.action.<verb>.execute`). Existing owner tests scope them to `[data-slot=window-action-pane]`; this probe uses page-global first matches, which may select another window when multiple panes exist. Scope lookup to the selected Blueprint window.
- submitAction currently receives empty args for both actions, so its unsupported select.fill and generic input selectors do not affect this journey. They must be fixed and checked before extending it to export parameters.

The findings concern validation strength rather than proof of a current product regression. The reviewer made no source changes.

## Source-Based Stable Selectors

- React readiness is published on document.documentElement by ShellHost/canvas-footer/footer-items: beaconId is pluginFilter or unknown; for the layout pluginFilter route the expected value is layout. The inner ShellHost ready attribute is an empty marker and is distinct from the document root beacon. Existing root comparison to layout is correct for this URL.
- PanelTabBar renders both id={tab.id} and data-tab-id={tab.id} on its tab control. Use `[data-tab-id=\"framework.panel.artifact\"]:visible` (or exact id attribute) instead of Artifact label text. Shell chrome placeholders are aria-hidden when the floating panel owns the bar; selecting visible controls avoids the placeholder/duplicate host. Source: ui/elements/PanelTabBar lines 426-439 and react/panel-chrome-tab-bar.
- Layout Artifact rows use frame.id() as the TreeItem ID; the UI Tree/leaf item renders id={id} and aria-selected on the same role=treeitem element. Use `[role=\"treeitem\"][id=\"frame-1\"]` for this fixture, then assert that exact row has aria-selected=true. Page rows use stable layout-document.page.<page-id> identifiers; count by that prefix instead of English Page/Seite text. Frames may be counted by fixture frame- ID prefix or by the resolved artifact frame ids. Source: layout editor/panels/artifact lines 199-214, ui/Tree/leaf item lines 208-214.
- Action row IDs remain action.addFrame/action.addPage and execute controls end with .action.<verb>.execute. Scope these to the Blueprint window and its data-slot=window-action-pane.
