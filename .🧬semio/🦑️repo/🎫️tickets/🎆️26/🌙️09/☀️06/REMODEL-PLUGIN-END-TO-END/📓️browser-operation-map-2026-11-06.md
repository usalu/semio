# Browser Operation Map — 2026-11-06

Source-only; no selector or action observed live. ShellHost command-bar scope exposes playground.navbar.fixture when session and focused app have matching registered examples, exampleId unlocked and not host landing app. Editor with nonempty examples qualifies; no general hostMode exclusion. Mode buttons playground.navbar.modes.<mode ID> require session with multiple modes; no hostMode restriction. Live focus, catalog, lock and layout still require observation.

Remodel media panel remodeling.media declares remodeling-media.summary with stream and asset counts and remodeling-media.stream.<stream ID> with stream name, kind, frame count and sync. Compare before and after import, rejection or append, but counts alone do not prove durable bytes.

Artifact panel declares nested framework.panel.document.reconstruction and framework.panel.document.run; reconstruction rows remodeling-pipeline.run and remodeling-pipeline.result display state and localized result. Reserved actions are toolRunStart, Pause, Resume, Step, Abort and Finalize; use published chords and observe actual state. Complete means ready to finalize, Finalized means committed as one undoable change, Aborted means nothing committed. Existing generic undo helper demonstrates document change, not necessarily one whole reconstruction.

Manifest declares in-palette exportQcReport. The action pane uses action.<id> for an immediate action; staged execute ID is childElementId of framework.window, actual window instance ID, action, action ID, execute. Never assume instance equals kind. QC handler does nothing without report; otherwise emits remodeling-qc-report.json application/json. Download requires actual observation.

Editor export_media implements mesh:out by GLB encoding resolved durable mesh. No public UI route choosing mesh:out was established; ShellHost natural Save calls artifact:native, not mesh:out. Do not infer a mesh download from the native port or generic Save.
