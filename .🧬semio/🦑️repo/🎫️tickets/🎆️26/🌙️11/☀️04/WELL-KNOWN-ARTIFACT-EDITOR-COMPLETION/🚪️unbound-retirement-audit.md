# Unbound Draft Retirement Audit

This is a current-source audit and neutral regression proposal, not an executed failing test claim. Existing broad host verification is252/252pass; those tests do not cover this unbound case.

## Confirmed draft loss

ensureSpawnedPlugin in spawned-session/switch-plugin-app creates an app and adds a panel entry without attaching a document actor. The new Save method later attaches it through keepProgramDocument. Before first Save, edits live only inside that app.

Canvas final-window admission in canvas-footer/canvas lines120–129 gathers only matching openDocumentSessionsRef entries. An unbound palette editor yields an empty list, so no guard runs; the callback records shell.windowClose, destroys the app and removes its panel. The existing async UI close admission cannot help because the host callback itself returns true. Main-session windows also skip the spawned-owner branch; closing one is not proven to destroy the primary app here, so it must not be conflated with the confirmed spawned case.

## Other current bypasses

- plugin-lifecycle/reload-and-extension-callbacks reloadPlugin destroys active and spawned instances, drops panel entries, swaps modules and disposes the predecessor without program/document close admission. Candidate acquisition before retirement is good but does not preserve unsaved content. It does not drain ordinary document attachments at those direct destruction sites.
- The same module uninstallPlugin directly destroys spawned instances, removes the plugin and disposes it. It refuses the primary/current session plugin, but a host session containing a spawned editor from another plugin still permits this path.
- spawned-session/update-space-panel retireSessionInstance directly closes matching attachments, drains and destroys. Ordinary switchToPluginApp passes this as gate retire after publishing a successor, with no saved-content admission. Quiescing pending work and sealing input do not establish persistence.
- A switch carrying documentSource uses prepareDocumentSurface to capture/restore the complete archive and transfer attachment. That is a content-preserving handoff; do not demand a discard guard merely because its predecessor is retired. It still needs existing identity/ownership fences.
- Human-change re-establishment calls retireSessionInstance after replacing the old session. It is another retirement entry point requiring an explicit content-preservation policy; avoid silently treating an empty attachment list as saved.

## Clean shared seam

Introduce retirement admission for a captured program owner: exact plugin handle, canonical app, instance and ownership/generation predicate. Attached documents use current saved/readback authority. An unbound documented editor is unknown or unsaved, never implicitly saved. Its flush calls the same keepProgramDocument persistence coordinator already used by first Save, with the captured target and explicit cancellation; it must not call focused saveNaturalFile, change focus, or trigger a download. After attachment and durable catalog readback, verify the same current owner before accepting. Keep or any stale/failed admission leaves instance, panel, attachment and plugin handle intact.

Separate admission from destructive commit. Plugin reload/uninstall must collect affected owners and finish all admissions before closing any attachment or destroying any instance; a loop of current requestCloseDocument would partially retire earlier owners before a later Keep. Commit then drains and destroys only captured owners that still match. Canvas, plugin lifecycle and ordinary app-switch should share this seam.

Neutral scenarios are in 🚪️unbound-retirement-scenarios.json. Convert them into actual useCanvas/usePluginReloadCallbacks/switch gate regressions before implementation. Include focus movement, asynchronous first-save persistence failure/cancel, replacement generation, multi-editor Keep atomicity, and explicit discard.
