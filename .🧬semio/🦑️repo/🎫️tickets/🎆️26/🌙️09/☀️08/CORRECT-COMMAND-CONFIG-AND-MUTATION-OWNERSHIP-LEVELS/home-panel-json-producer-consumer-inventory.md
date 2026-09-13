# Home Host Panel JSON Producer and Consumer Inventory

## Decision

`ViewModel.panelJson` is host/session state. It owns the active host panel tab together with `spawnedApps` and `activeSpawnedId`; none of those values belongs in Home app config or an exact Home `WindowConfig`.

The carriage must use one strict JSON text codec in both browser and native hosts. The field is explicitly named `panelJson`, the native implementation already writes JSON with `serde_json`, and JSON lets the shared schema validate the carried value directly. The browser Pack/base64 codec is therefore the facet that must change. No fallback decoder is admissible: accepting both would hide split host state and retain an unbounded legacy path.

The neutral contract introduced under `Shell/🧬️schema/📌️panel-state` retains exactly:

- nonempty `activePanelTab`, which names one selected host panel leaf;
- `spawnedApps`, bounded by the existing 64-window-instance capacity;
- optional `activeSpawnedId`, which must name one entry in `spawnedApps`;
- spawned identifiers bounded by the existing 256-character identifier capacity;
- the complete encoded JSON bounded by the existing 65,536-character `panelJson` carriage capacity.

Configured catalogue/default panel identifiers and the stored selection must be nonempty manifest leaves. An absent configured catalogue panel is an error and cannot be converted to an empty identifier.

There is no legitimate empty-selection transition in either host. Browser panel visibility changes only `SET_PANEL_VISIBLE`; tab actions are emitted only when `tabId` exists and resolves to a leaf (`ShellHost:9692-9699,9740-9771`). Native panel close/toggle paths change only `left_panel_open` or `right_panel_open` and retain `active_left_tab` / `active_right_tab` (`wgpu:6609-6645,7524-7530`). Accordingly the schema uses a required nonempty string instead of an empty sentinel or optional selection.

## Authoritative typed codecs

| Facet | Reader | Writer | Current wire format | Finding |
| --- | --- | --- | --- | --- |
| Native WGPU host | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3143-3145` | `:3147-3149` | plain JSON through `serde_json` | Decode errors become `None`; encode errors become an empty string. Neither path distinguishes absent state from invalid state, and `SpacePanelState` at `:707-722` does not reject unknown fields. |
| Browser host helper | `🛠️ShellHelpers/📌️panel/🟦️.ts:33-40` | `:28-30` | Pack bytes encoded as base64 | The reader casts decoded data to `SpacePanelState` without validating shape or semantic links and catches every failure as `null`. The writer cannot be inspected against a JSON schema on the wire. |
| Neutral host-panel schema and oracle | `🐚️Shell/🧬️schema/📌️panel-state/🔣️.json` and `…/🧪️tests/🔬️unit/🟦️.ts` | same | strict JSON | Ajv and an independent validator agree on the closed shape and bounds; the independent validator additionally proves ID uniqueness and active-ID membership. |

The two live codecs are incompatible: native JSON cannot be read by the browser Pack reader, and Pack/base64 cannot be read by native `serde_json`. This is a present cross-facet defect, not a compatibility requirement. The coordinated implementation must replace every producer and consumer at once.

## Native producers

| Producer | Source | Current behavior | Required behavior |
| --- | --- | --- | --- |
| Host boot | WGPU Shell `:3294-3318` | Builds the first configured catalogue tab with `unwrap_or_default`, encodes it, and stores it in the new host session. | Reject a missing/empty configured catalogue leaf before creating the session; encode a validated state. |
| Host app-session replacement | `:5866-5885` | Reuses a supplied view or constructs the same default panel with `unwrap_or_default`. | Validate a supplied panel; build a default only from a valid configured leaf. |
| Spawned app creation | `:5997-6013` | Parses or silently replaces with a default, appends without capacity or duplicate validation, sets focus, and encodes. It creates the guest instance before knowing that panel publication can succeed. | Validate current panel first, reject over-capacity/duplicate states, and publish only after the exact spawned record and focus validate. Creation failure must not be hidden by a default panel. |
| Guest host effect `setPanel` | `:5666-5680` | Copies arbitrary `operation.panel.to_string()` into `panel_json`. | A claimed host panel effect must decode and validate the complete panel before mutation. Invalid claimed effects must return an error; they cannot fall through into guest dispatch. |
| Tutorial snapshot application | `:9924-9930` | Copies opaque snapshot `panel_json` into the live session. | Validate present snapshot panel state before the session mutation; preserve explicit absence according to the tutorial contract. |

Native `panel_json: None` on spawned guest view states at `:3663-3673` and ordinary nonhost sessions at `:5917` are deliberate non-producers. They keep host/session panel state out of guest windows.

## Native consumers and pass-throughs

| Consumer | Source | Ownership use |
| --- | --- | --- |
| Active spawned render selection | WGPU Shell `:3656-3673` | Resolves `activeSpawnedId` in `spawnedApps`, then mounts that exact plugin/app/instance window. A dangling ID currently renders no spawned guest silently. |
| Host back command | `:6656-6661` | Tests whether the host session has an active spawned ID and dispatches `closeFocusedInstance`. |
| Tutorial capture | `:9898-9921` | Copies the host session panel carriage into `TutorialUiSnapshot`. |
| Directory Home session retention | `:5866-5870` | Copies the whole active `ViewModel`, including panel state, when the exact Home instance owns the session. This is an opaque lifecycle pass-through, not another panel owner. |

## Browser producers

| Producer | Source | Current behavior | Required behavior |
| --- | --- | --- | --- |
| Host boot | `🏛️ShellHost/🟦️.tsx:3428-3445` | Calls `buildSpacePanelState([], [])`; its helper hardcodes `s-play-catalogue` and still writes an obsolete empty `programs` array. | Resolve the configured host catalogue leaf, reject an invalid default, and emit only the three canonical fields. |
| Plugin unload and reload | `:3635-3661` and `:3720-3741` | Filters spawned entries and clears focus when it named a removed entry, then rewrites the carriage. | Preserve this transition through the validated codec; reject malformed source state rather than treating it as absent. |
| Central host panel commit | `updateSpacePanel`, `:5055-5071` | Writes the helper output into the session. | Make this the typed validation/publication seam for every host-owned panel transition. |
| Host-role session default | `:5123-5138` | Emits another hardcoded default only in host mode. | Use the same configured-default constructor as boot. |
| Spawn/open flow | `:5174-5193`, `:5590-5650`, `:5824-5840` | Uses parse-or-default, searches/upserts spawned entries, changes focus, and closes focus. Several paths can turn corrupt state into a new empty roster. | Invalid current panel must reject the claimed host action. Preserve `spawnedApps` and `activeSpawnedId` except for the exact open/close transition. Enforce ID and capacity bounds before guest dispatch/publication. |
| Guest host effect `setPanel` | `:5312-5328` | Copies `effect.setPanel.panelJson` directly into the next view state. | Decode and validate before accepting the host effect. An invalid claimed effect is an error and cannot be offered to the guest as an unhandled action. |
| Active panel tab action | `:6378-6394` | Rebuilds the panel with a requested tab ID and commits it; current helper also re-carries `programs`. | Qualify the host/session action, require a declared nonempty panel leaf, change only `activePanelTab`, and preserve both spawned fields byte-for-value after canonical re-encoding. |
| Spawned-window close UI | `:10234-10252` | Removes the exact entry and focuses the first survivor. | Keep this exact transition, validated and bounded, with explicit no-active state when the roster becomes empty. |
| Tutorial snapshot/change application | `🛠️ShellHelpers/🟦️.tsx:2554-2594,2634` | Copies opaque `panelJson` from tutorial state into `SET_SESSION`. | Validate at the same canonical codec boundary before the session mutation. |

## Browser consumers and pass-throughs

| Consumer | Source | Ownership use |
| --- | --- | --- |
| Shell render model | `🏛️ShellHost/🟦️.tsx:4008-4016` | Parses once by raw `panelJson`, derives the spawned roster and exact active entry. |
| Effect/dialog source liveness | `:2610` and related source checks | Supplies spawned entries when qualifying whether a captured host effect still owns the current source. |
| Open, close, toolbar, and panel UI | `:5045`, `:5183-5193`, `:5824-5853`, `:6106`, `:6386-6394`, `:8438`, `:8968`, `:9942-9943`, `:10172-10252`, `:10431` | Reads the host/session roster, active entry, and panel tab to render and route exact actions. |
| View change reporting | `:1018` | Treats a changed raw carriage as a `panelState` view change. This is event carriage, not an independent panel owner. |
| Plugin runtime WIT host-effect decode | `🔌️PluginRuntime/🟦️.tsx:989` | Reads `setPanel.panelJson` from the plugin effect payload. The shell must validate the value before session publication. |
| WGPU TypeScript plugin bridge | `renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:616` | Deletes `panelJson` from the slim guest contribution view. This correctly prevents the host panel from becoming guest contribution state. |

## Generic carriage and capacity owners

The framework manifest owns only the opaque, bounded carrier:

- Rust `ViewModel.panel_json` appears in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4388`; TypeScript mirrors it at `🟦️.ts:804`.
- The view-context schema caps `panelJson` at 65,536 Unicode characters in `🪟️view-context/🧬️schema/🔣️.json:30`.
- Rust constants define identifier capacity 256, long-string capacity 65,536, and window-instance capacity 64 at `🦀️.rs:4465-4471`; TypeScript mirrors the long-string bound at `🟦️.ts:878-887`.
- The resolved-host-context Rust and TypeScript tests pin the carrier bound to the schema. They do not validate host-panel content.
- Host effects and tutorial snapshots carry the string opaquely. Their generic types must stay content-neutral; validation belongs at the Shell host/session owner before mutation.

The panel schema is therefore bounded by both the outer carriage and the real session instance capacity. It does not broaden either generic framework limit.

## Existing tests that must move atomically

`renderer/🧑‍🎨engine/🧪️tests/📌️view-state-carriage/🟦️.ts:39-58` currently asserts the 65,536-character outer bound and a 64-entry spawned roster, but also asserts that encoded browser panel state has an obsolete empty `programs` array. The codec cut must update this test to the canonical JSON shape and retain its capacity assertions.

The engine contract contains host action and effect fixtures, while generated WGPU frame-worker output contains copies of the browser runtime. Generated output is not a source owner and must be regenerated only by its registered repository route after source tests pass.

## Qualified action law required before production edits

A host-panel action is claimed only when its controller/source resolves to the current host session and its verb is one of the bounded panel transitions. For a valid claimed action:

1. decode the one current canonical state;
2. validate configured identifiers and semantic links;
3. apply the exact transition;
4. validate and encode the complete next state;
5. publish it once while preserving unrelated fields.

For an invalid claimed action, stop with an error before any guest dispatch. The absence or non-runnability of a guest handler must be present in the fixture so the law proves both valid and invalid claimed panel actions are host-owned independently of guest availability. An unclaimed action may follow the ordinary guest route.

The browser and native laws must each cover: valid tab selection with unchanged spawned state, invalid/empty configured catalogue rejection, corrupt carried state rejection, over-capacity spawn rejection, duplicate/dangling active ID rejection, and missing guest dispatch. Native host production edits and Home duplicate cleanup remain gated on the Architect native ownership suite and this qualified-route law.

## Source status

This inventory and the neutral schema/fixture/oracle are test-first work only. No native Shell, browser ShellHost, panel-helper production codec, Home config, or generated frame-worker source was changed in this preparation step.
