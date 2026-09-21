# Native135 And Paired Browser Journey

The WGPU goal remains active. Native135 ran all 1,362 tests: **1,343 passed, 19 failed, none skipped** in 35.457 seconds. The preceding census had 33 failures. Full output and the exact remaining failures are in `🗑️generated/astra-runtime/renderer-native135-full/`.

## Verified changes

The tutorial resolver now distinguishes a public surface ID from its generated host identity. Clipboard focus and Ink cancellation use accepted retained documents. Palette search, modal absorption, dense Actions scrolling, ToolRun controls, and the presenter watchdog are green in this census. File preference persistence waits for the actual fixture writer to become terminal before reading JSON. Display journal item/byte refusal and ready-peer publication now reach their intended boundaries after destination geometry is set before painting; both pass. Context-menu identity checks first reject candidate-only geometry and then resolve the acknowledged window.

Two retained cleanup cases still need investigation: Map replacement retains the old gesture cache even after a following accepted frame, and one Ink close case retains its focused ID. The happy Display transfer now reaches a later World owner admission failure. The new Canvas replacement test also exposes a missing Canvas branch in its helper. These are recorded as failures, not waived.

## Physical browser evidence

The current WGPU tab 4 and React tab 5 both measure 1600×1000 and use completed Puzzle component digest `1db04e39cf7c07e23ba875a17ea6e44d76b6de72669999fb170aa32ca87d9d1f`. The browser comparison uses physical WGPU clicks and React controls, then saves screenshots, DOM snapshots, and console entries. It does not read app internals or storage. Evidence is `🗑️generated/astra-runtime/checkpoint19-iab/journey-physical.json` and adjacent `journey-*.jpg` files.

Artifact, Catalogue, Inspection, Tool runs, Chat, Settings, General, Appearance, language, Driver, Actions, Search, Utilities, Projection, Window Options and window focus have been exercised. Dark appearance and German language commit on both renderers; seven Driver axes appear and retire. App setting values agree (0.005, 0.75, 256, 10). No new WGPU console error was observed in these steps. This is partial physical coverage, not a passed 57-step journal or Dock8 gate.

Confirmed differences remain:

- General and Actions tree rows lose intrinsic accessibility labels; explicit Input names are also missing in App settings.
- Appearance and language popup text overlaps underlying rows. Actions rows are blank, and pane backgrounds expose the World beneath them.
- WGPU Actions and Search appear together; the paired React controls expose their respective panes.
- German selection leaves some WGPU Settings tabs, window cap controls and Window Options fields in English. Labels and accessibility roles differ elsewhere too.
- Projection exposes all 15 entries, but WGPU accessibility names are generated IDs instead of the displayed captions.
- Window Options spacing reports 10.5 through accessibility while its displayed caption says 10.0; this needs separate range/value analysis.

The fleet is repairing intrinsic tree labels and auditing paint ordering while accepted-input fixture repairs continue. Window close/reopen, split dragging, camera gestures, keyboard journeys and the broader surface matrix remain to be finished.


## Window Close Runtime Failure

At **2026-09-21 17:42:41.417 UTC**, closing the surviving Perspective window quarantined WGPU:

```
presentation stalled: phase=Render engine=0 upload=1 gpu-cursor=None upload-progress=(0, 0, 0) input-wait=InteractionCheckout
```

The earlier Top focus/unfocus transitions produced one/two windows correctly. Closing the focused Top initially left its prior pixels; a later Display click showed Perspective. React reached an empty dock after its two close actions. The subsequent confirmed physical click on Perspective Close at (402,45) produced the terminal fault. No later WGPU journey steps are accepted. The live receipt and screenshot are `🗑️generated/astra-runtime/checkpoint19-iab/window-close-interaction-checkout-fault.{json,jpg}`. Sol is assigned the presenter close handback; Terra is auditing the exact ownership path. This supersedes the earlier error-free partial-journey statement for the terminal window-close step.
