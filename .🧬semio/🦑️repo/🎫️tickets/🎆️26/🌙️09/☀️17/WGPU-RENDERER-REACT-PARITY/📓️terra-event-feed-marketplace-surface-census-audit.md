# Event Feed, Marketplace, and Surface Census Audit

Read-only source audit on 2026-09-26. This is an implementation packet, not runtime acceptance: no browser journey, native run, Cargo invocation, or Nx target ran.

## Event Feed: a real temporal-presentation mismatch

React is the current behavioural authority. `EventFeedHost/🟦️.tsx:30-35` formats each timestamp through:

```ts
new Date(timestampMs).toLocaleTimeString([], {
  hour: "2-digit", minute: "2-digit", second: "2-digit",
})
```

That is the page host's actual locale, time zone, hour cycle, Unicode punctuation, and daylight-saving rule. It is neither a fixed `en`/`de` format nor UTC.

WGPU instead calls `event_feed_time_of_day_utc` in `Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4491-4497`, reduces the epoch modulo one UTC day, and paints fixed `HH:MM:SS` at `:4594-4597`. The existing WGPU unit test at `Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs:36-41` intentionally asserts this UTC-wrap behaviour. It proves the currently divergent rule; it is not a parity test.

### Existing authority and ownership

There is an established **UI language** authority, but not a temporal-format authority. Shell locale resolution is lock, stored UI preference, then page `navigator.language` (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:29463-29467`). The page/worker boot path forwards that through `semioWgpuSetHostLocale` (`frame-worker/🟦️.ts:623-627`). It eventually resolves the product's `en` or `de` Chrome strings. It does not transport a time zone, resolved hour cycle, or a general BCP-47 temporal locale.

A scoped source inventory found no renderer `Intl.DateTimeFormat`, `toLocaleTimeString`, `timeZone`, or native temporal-format port other than Event Feed itself. The print product has a separate UTC-only formatting oracle, which explicitly excludes local zones; it is not a UI-host authority.

The appropriate browser boundary already exists. `WgpuHostIoRequest` and `createWgpuPageHostIo` are page-owned, typed host I/O (`🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts:27-40,500-524`); the Worker reaches the same page code and wasm Rust calls the one `semioWgpuHostIo` door (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:29806-29820`). This is the right browser owner because a Worker does not own the page's presentation settings.

There is no matching native temporal adapter in the audited renderer. A system-UTC fallback would therefore be an invented policy and would preserve the bug.

### Minimal neutral contract

Define the schema and fixtures under the domain-neutral UI contract before adding a target implementation. Keep raw `timestampMs` in the Event Feed domain record. Temporal text is an ephemeral host presentation projection, never persisted event data.

```text
EventFeedTimeFormatRequestV1 {
  timestampsMs: integer[]                  // bounded batch, epoch instants
}

HostTemporalProfileV1 {
  locale: string                           // host-resolved BCP-47 tag
  timeZone: string                         // host-resolved IANA identifier
  hourCycle: "h11" | "h12" | "h23" | "h24"
  revision: integer                        // changes whenever the host profile changes
}

EventFeedTimeFormatReplyV1 {
  profile: HostTemporalProfileV1
  labels: [{ timestampMs: integer, text: string }]
}
```

The formatter port's defined operation is: produce `text` with the host's equivalent of React's exact `toLocaleTimeString([], { hour, minute, second })` rule. The core may use `{ timestampMs, profile.revision }` as an ephemeral cache key, but it does not parse, synthesize, or translate `text`.

For browser WGPU, the page implementation should call the same built-in `Intl` path as React and return the observed profile with the labels. Refactor the React helper to call that shared page formatter so the real reference and the bridge share one format definition. For native WGPU, each application host implements the same port through its platform formatting facility and returns the typed reply. The Rust renderer consumes only the reply; it gains no operating-system or third-party runtime dependency.

Do not overload `semioWgpuSetHostLocale`: selected UI language and host temporal profile are distinct contracts. Do not make `locale`, `timeZone`, or `hourCycle` input to a home-grown Rust clock formatter; that would require locale and zone data the renderer does not own and would still fail Unicode/hour-cycle parity.

### Accepted-frame and stale-result law

The Shell holds a bounded presentation cache. A response may update a candidate only when its profile revision and the requested timestamp set still match. Its labels become visible with the same accepted frame that paints the Event Feed rows. A late result from the preceding profile is discarded. Before a matching reply exists, the WGPU row must suppress the timestamp rather than invent UTC text.

This makes page/Worker latency explicit and remains correct if the OS language, hour-cycle preference, or time zone changes between request and reply. It also preserves React's present failure behaviour: an invalid `Date` produces no label.

### Fail-first fixture, oracle, and tests

Add a language-neutral `event-feed-time-format.v1` fixture with at least these cases:

* one instant before and one after a daylight-saving boundary for a 12-hour zone such as `en-US` / `America/New_York`;
* the same instants for a 24-hour zone such as `de-DE` / `Europe/Berlin`;
* two profile revisions for one instant, so an old reply is provably rejected; and
* an invalid/out-of-range timestamp whose label is absent.

Use the running browser's `Date#toLocaleTimeString` with React's current options as the browser oracle. Compare exact strings, including spaces and punctuation; do not write hand-authored `08:04:05 AM` expectations, because ICU output is host data. The native contract test injects the fixture reply and proves the core paints its `text` verbatim; each native adapter adds a platform test against its standard formatter. That separates renderer correctness from intentional OS locale-data variation.

Required implementation tests:

1. A schema/fixture reader on both sides validates limits, exact timestamp association, profile fields, and rejection of duplicate or missing labels.
2. A page-host I/O test checks a bounded batch uses the actual browser formatter and reports the matching resolved profile.
3. React and browser-WGPU receive one fixture reply and render byte-identical time text.
4. A worker response for revision `r` arriving after `r + 1` cannot alter the accepted Event Feed frame.
5. A missing/refused formatter reply paints no synthetic UTC `HH:MM:SS` label.
6. Native host adapters use the same schema fixture and their standard OS formatter; no extra Rust runtime dependency is admitted.

## Marketplace: the Store door exists, the renderer bridge does not

The earlier Marketplace report correctly described the missing Rust ledger and dynamic admission. Current source adds a valuable first half that the earlier report did not have: page host I/O now exposes bounded Store operations:

* `extension-store-list`, `extension-store-install-url`, `extension-store-install-file`, and `extension-store-uninstall` are typed at `🚪️host-io/🟦️.ts:27-40`;
* its page handler validates the Store record identity and caps list size at `:212-250`; and
* `ExtensionStore` itself is the durable local package authority, with `installFromBytes`, `installFromUrl`, `uninstall`, and `listInstalled` at `🔌️plugin/🏪️store/📥️installation/🟦️.ts:92-98,328-351`. Its schema fixes `extensionId`, `directoryName`, `version`, `label`, `extends`, `moduleUrl`, `packageHash`, and `installedAt`.

The WGPU Rust side does not consume any `extension-store-*` request. `ShellState::build_marketplace_ui` still explicitly says extensions have no ledger and paints only resident plugins plus the generated static activation catalogue (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:8733-8738`). The action router still only implements `installPlugin`, `reloadPlugin`, and `uninstallPlugin` (`:10951-10967`); `uninstall_plugin` merely drops a resident shell entry (`:9510-9516`).

The existing lazy loader cannot close that gap. Rust calls `install_js_plugin(plugin_id)` with only a static ID (`ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:1473-1480`), and the frame Worker exposes `semioWgpuInstallPlugin(pluginId)` (`frame-worker/🟦️.ts:960`). It has no Store-returned `moduleUrl`, package hash, or extension identity admission. The direct page boot also has no lazy-install door.

### Required bridge boundary

Keep three concerns separate:

1. **Local durable package inventory:** the Store's installed-record schema and host-I/O operations remain its owner.
2. **Ephemeral/application extension projection:** WGPU Shell owns a typed bounded projection of installed records plus `enabled` and load status. It must not fabricate an extension from the static plugin catalogue. Enablement is distinct from Store installation; React currently expresses it through its session ledger/space operation.
3. **Dynamic program admission:** add a separate browser bridge accepting the Store record's `{ extensionId, moduleUrl, packageHash, version, extends }`, then verify the returned program manifest against all of those identity fields before admitting it. It is a different operation from static `semioWgpuInstallPlugin` and must be installed in both the Worker and direct-page variants.

The Marketplace tree should then group Store-backed extension rows by their real source/host, expose URL and file install rows, and route install, uninstall, and enable commands through those typed operations. A Store package remains visible as failed/unavailable if dynamic admission fails. Uninstall retains the row and contribution until actor/instance retirement and Store deletion both succeed; refusal leaves the previous projection intact.

Required tests are one cross-language Store-record fixture; host-I/O list/install/file-cancel/uninstall tests; Worker and direct-page dynamic admission tests; manifest-identity rejection; enabled/disabled contribution publication; and terminal uninstall ordering. The existing Store installation-identity test is the appropriate package oracle. The existing WGPU Marketplace test validates only the static plugin lane and cannot establish this bridge.

## Current complete-surface census

The `SurfaceKind` census remains fifteen variants. The September census correctly found all fifteen React producers, WGPU payload fields, and paint routes, then listed Canvas2d and InkCanvas behaviour gaps. Those three reported gaps are no longer current source gaps:

| Earlier gap | Current source confirmation |
| --- | --- |
| Canvas modifiers and terminal cancellation | `cancel_canvas_pointer_gesture_for` and `canvas_pointer_button_into` (`Scenes/🦀️.rs:1913,2073`) now own the path; the current `wgpu-canvas2d` tests exercise cancellation and independent pointer ownership. |
| Canvas catalogue DnD and double-click | `canvas_catalogue_drag_over_into` and `canvas_catalogue_drop_into` are present (`Scenes/🦀️.rs:2217,2304`), with current Canvas tests checking `canvasDragLeave` and `canvasDrop`. |
| Ink text/table editing and clipboard | `ink_double_click_edit`, `ink_edit_key_into`, and `ink_clipboard_paste_into` exist (`Scenes/🦀️.rs:6282,6806,6996`), backed by Ink editing/clipboard tests. |

The accepted-frame retained pointer map is also current: Shell swaps the retained hit/window/scene maps together only after acknowledgement (`Shell/🦀️.rs:14071-14093`). The immediate and retained Select routes are already covered separately, so no Select work is included in this packet.

Therefore this audit found no remaining **source-only missing paint or input producer** among the fifteen surface variants. Event Feed temporal formatting is the remaining surface-related mismatch. This does not claim visual or runtime acceptance; it records that the earlier Canvas2d/InkCanvas source findings are obsolete and should not be scheduled again.

## Ownership and next slices

1. **Temporal presentation:** schema/fixture first; page host-I/O formatter and React helper; accepted-frame cache and WGPU painter; then native adapter. This is the smallest high-value user-visible parity slice.
2. **Marketplace extension bridge:** consume the existing Store host-I/O door, add typed Shell projection and dynamic record admission, then replace the static-only Marketplace projection. It is larger but already has its package and browser-door halves.
3. **Do not reopen Select, Canvas2d, InkCanvas, Tree/Dock, or Chrome accessibility work from this audit.** Their current source routes are either covered above or owned elsewhere.

