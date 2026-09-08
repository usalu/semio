# Host UI Preferences Integration

## Ownership

- `semio-framework-os-config` owns the `UiPreferences` snapshot, its value types, schema parser, mutation vocabulary, schema id `os.config.ui-preferences`, and mutation fold.
- The OS shell Rust and TypeScript faces consume and reexport those types. They no longer define the preference snapshot independently.
- React and native WGPU hosts persist canonical `UiPreferencesConfigMutation` events and rebuild the current projection by replaying the event stream. Widget drafts remain ephemeral shell state.
- OS commands such as `os.setLocale`, `os.setAppearance`, `os.setLayout`, and `os.setDriver` append canonical config events. Shell state is updated from the resulting projection and shared-store subscribers propagate it to every mounted shell.

## Persistence and propagation

The local OS config preference entry stores a versioned event stream:

```json
{
  "version": 1,
  "events": [
    { "mutation": "setLocale", "locale": "de" }
  ]
}
```

Reload reads the event stream and folds it through the canonical OS mutation reducer. There is no legacy snapshot read or compatibility route. Subscribers compare the physical stream and replay it for shells using the same scoped OS store; a different scope remains isolated.

## Surface context

- React `surface-visible` events carry a concrete surface id, the authored body key, and a packed `ViewModel`.
- Windows use `windowViewContext`, preserving their exact instance id and per-window active utility. Multiple instances may share one body key without sharing retained state.
- Panels use `panelViewContext`, which clears window id, window kind, and active utility while preserving locale, terminology, mode, tool, maps, and other global context.
- React and native WGPU skip targets without an authored body key instead of deriving the body key from a surface id.
- Context-menu requests carry the live full `ViewModel` and an explicit target window where applicable.
- Browser document actors receive a `browser-actor-view-state` request immediately after the matching `open` request and on locale, terminology, window, tool, mode, or utility changes. The base window list uses `id == windowKindId`, matching the worker's verified base surface without a fallback.

## Generated bridge

The WGPU TypeScript bridge retains documents by concrete surface and passes `surfaceId`, `bodyKey`, and `viewState` through its direct render APIs. The generated `🎞️frame-worker.js` was refreshed through its Nx generator after the bridge change.

## Verification

- Focused React host run: `NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test -- long '../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx' '../../../../🎚️UiPreferences/🟦️.ts' '--testNamePattern=sends the same current locale|binds two instances|replays the language-agnostic' --silent=false --reporter=verbose`.
  - `3 passed`, including two-surface context-menu locale/terminology, distinct concrete render surfaces with packed per-window utility context, and language-neutral event replay with cross-shell propagation, reload, isolation, and Ajv schema validation.
  - Console evidence: `[DEBUG] two context-menu surfaces received the current shared OS locale and terminology`.
  - Console evidence: `[DEBUG] concrete render surfaces received packed locale, terminology, and per-window utility context`.
  - Console evidence: `[DEBUG] canonical OS UI preference events replayed, cross-shell propagated, isolated, and schema-validated`.
- `NX_DAEMON=false bun nx run @semio-tech/framework-renderer-wgpu:check-frame-worker` passed after regeneration and reported `🎞️frame-worker.js is fresh`.
- The shell schema census recognizes the canonical OS config parser reexports and reached its Rust typegen test; that test was stopped while waiting on the shared Cargo build-directory lock held by other active workspace checks.
- The coordinator's focused browser-worker run passed with console evidence for live preference rerender, unchanged artifact config/frontier, and stale opening refusal: `live-preference-refresh=1`, `artifact-config-unchanged=1`, `stale-opening-rejected=1`.
- The first broad React long run was blocked during import by two concurrently deleted unrelated fixture files. The isolated host selection above excludes those suites and passes.
- A native WGPU preference test attempt was blocked before compilation by another concurrently deleted workspace member manifest under `🗣️dsl/🧹️fixture-sweep`; the coordinator owns the current backend compile session.

