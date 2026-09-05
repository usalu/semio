# Current Frontend: Three User-Visible Completion Slices

## Scope and current inventory

Read-only source snapshot on 2026-09-05. No build, server, or browser was started for this audit.

The generated OS registry declares **59** component targets: 33 plugins in `PLUGIN_BUILD_TARGETS` and 26 extensions in `EXTENSION_TARGETS` ([generated catalog](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts:45)). The shared development module root currently has **38** `🔣️.json` descriptor siblings, leaving **21** registry targets without one. This is a file-inventory fact, not a claim that the other 38 are all runnable: descriptors are only staged when the owner has one ([stager](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:206)).

Do not make a 59-target repair the next goal. The selected Home/Space/GIS vertical only needs the three slices below.

| Surface | Genuine currently implemented seam | Current frontier |
| --- | --- | --- |
| Home | The `s.space.home@1/*#editor` descriptor has the `s-home-main` window; its typed retained command set includes `applyDirectoryEventPage`, `openSpace`, and `manageSpace` ([descriptor](../../../../../../../../✏️s/🔌️plugins/🪐️space/🔣️.json:10), [commands](../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:62)). Shell retains a visible Home owner and applies the Hub's canonical page before ACK ([ShellHost](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1924)). | The in-flight Space output is not yet a coherent descriptor/catalog/module closure. |
| Space collaboration/admin | `manageSpace` emits only `os.directory.open-administration` with the row's space id ([command](../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏛️manage-space/🦀️.rs:25)); Shell maps it to the retained worker operation ([ShellHost](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3720)), and the pane renders Hub-canonical capability flags rather than a cached role ([pane](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx:1)). | The source path is substantive, but it cannot be called a user-visible journey until the Home component from the selected closure executes and the two-user UI path runs. |
| GIS Map editor | The GIS descriptor has a real `s.gis.gismap@1/*#editor`, `gis2d-main`, `gismap`, and `patchPositions` mutation ([descriptor](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🔣️.json:173), [action](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🔣️.json:634)). Native handling diffs accepted position JSON into artifact mutations ([handler](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🦀️.rs:52)). The staged GIS core SHA-256 is `90a7cd5f…`, matching the generated GIS registry row. | Browser target admission verifies bytes but deliberately emits `renderer-unavailable` instead of executing the component ([worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:867)). The GIS *viewer* descriptor is also a placeholder (`componentKind: ""`) ([descriptor](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🔣️.json:2942)); use the editor only for this first slice. |

### Space closure is presently stale

This is a measured current mismatch, not a metadata-only complaint:

| Item | Current value |
| --- | --- |
| `s` generated registry `coreWasmSha256` | `762dad6b1eca109108ff781d0697bdc2114ed8869b692c2cf88cc60ec03209af` ([row](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts:71)) |
| owner + staged Space descriptor declared core SHA | the same `762dad…` ([owner descriptor](../../../../../../../../✏️s/🔌️plugins/🪐️space/🔣️.json:13460)) |
| staged `semio_s_plugin_space_component.core.wasm` SHA | `46dcc8f82539b110bd38b78dc290e5dead16b2631f080ce93a7579bcf4879aa1` |
| timestamps | core: 2026-09-05 19:26:24; owner/staged JSON: 2026-09-04 11:18:00; bridge: 2026-09-02 10:56:47 |

`materializePlugin` is designed to derive and publish owner descriptor bytes only after JCO materialization ([implementation](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:333), [order](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:399)). Its descriptor stager merely copies those owner bytes ([stager](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:206)). Therefore no caller should patch the staged JSON or registry hash by hand. The active Space producer must either finish the materialize/describe stage or fail; then registry generation and staging must agree with the new core before admission. The registry renderer has an exact source-descriptor/registry-hash equality check ([gate](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2598)).

## 1. Home: finish and admit one coherent Space component closure

**User result:** an authenticated user lands on the genuine Home `s-home-main` UI and sees the Hub-projected Space row, rather than a boot error or an unattested physical dev module.

**Smallest implementation boundary:**

1. Complete the `s` producer's existing component materialize/describe path, whose owners are [the Space descriptor](../../../../../../../../✏️s/🔌️plugins/🪐️space/🔣️.json) and [the dev materializer](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:333).
2. Re-render the generated registry from that owner descriptor, then stage that same descriptor/pack/core tuple. `ensurePluginRegistry` currently renders the registry then syncs descriptor siblings ([function](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:507)); do this only after the producer owns a finished descriptor. Do not use a caller module URL or bypass the catalog equality gate.
3. Keep the landing's existing page route and retained ACK protocol; do not create another Home transport. The registered `directory-home-browser-process-check` already owns its source/runtime/process phases ([Hub script](../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:6499)).

**Acceptance, after the implementation:**

- Native/process: `directory-home-browser-process-check process` must reach the real Hub page → `applyDirectoryEventPage` terminal receipt → ACK sequence, with the selected Space core/descriptor hashes equal at admission.
- Browser: retain `directory-home-bootstrap-check` ([React script](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts:71)) and add one real-page assertion that `s-home-main` displays the admitted row. It must fail on either old descriptor/core mismatch; a static file-exists assertion is insufficient.

## 2. Space: Home row → canonical administration pane for an author

**User result:** from that displayed Home row, its author opens `manageSpace`, views canonical members, creates one spectator invitation, and gets the refreshed page only after the exact receipt; a spectator is shown no author controls and cannot disclose an invite token.

**Why it is next:** the end-to-end production semantics are already sharply bounded rather than speculative. The worker retains exactly one administration operation, clears page/receipt/capability before a denial/cancel notification, and does not retry unknown mutations ([worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2113)). The pane and Hub have schemas/fixtures. This is the smallest Space collaboration surface because it needs neither arbitrary plugin activation nor the Map durable triple.

**Exact seams to reuse:**

- Home row's author-only `manageSpace` affordance: [Home window](../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:71).
- `manageSpace` → `os.directory.open-administration`: [command](../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏛️manage-space/🦀️.rs:25).
- Shell's effect routing and retained operation: [ShellHost](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3720), [worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2186).
- UI authority fence: [SpaceAdministration pane](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx:1).

**Acceptance, after slice 1:**

- Native: reuse the four selected `space-administration-check native` laws: author window/receipt, spectator denial, removed-member empty denial, and noncanonical/foreign cursor rejection ([selector list](../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:7150)).
- Browser: reuse `directory-invite-capability-check` (pane + worker) and `scoped-presence-check` (two identical document ids in distinct Space scopes) ([React script](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts:128), [scoped selector](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts:175)). Add one process-browser traversal from a rendered Home row into the pane. Assert one author-only invite result, no invite token in DOM/state after close, and a second Space with the same document id remains untouched.

## 3. Map: execute a verified GIS editor, then persist one `patchPositions`

**User result:** an authorized author opens one `s.gis.gismap` editor, sees `gis2d-main`, changes one feature through `patchPositions`, and sees that mutation after a fresh authenticated reopen. A second Space sharing the document id does not receive the Map state or roster.

**Smallest missing boundary:** activate the exact verified execution target, not the GIS guest command or the raw `/🔌️plugin-modules` directory. The worker already fetches the Hub-selected component and descriptor under a private lease, checks them against the plan/manifest, and only then exchanges the plan receipt for the socket grant ([authority sequence](../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:808)). It then intentionally stops at `renderer-unavailable` ([worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:867)).

The implementation therefore needs one executor handoff from the live private lease to the renderer-owned GIS session. It must retain the lease and exact scope through close/reconnect, start only the Hub-selected JCO component closure, and never let a binding supply a module URL/path. It must not solve Map's broader fixed-three durable group: `patchPositions` is declared artifact-only ([publication contract](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:282)), so it is the correct first render-and-persist proof.

**Acceptance:**

- Native: preserve `execution-target-lease-check --native` as the manifest/byte/scope fence ([selector](../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:3772)); extend its selected GIS positive to assert executor startup only after the existing lease has been minted. The present GIS corpus expressly asserts no renderer claim ([corpus](../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:3744)), so it cannot be reported as execution until that assertion is replaced by an actual target-start receipt.
- Browser: replace—not supplement—the current `browser GIS viewer exposes localized renderer-unavailable` expectation ([worker test](../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4855)) with a private lease → visible `gis2d-main` → `patchPositions` → close/reopen scenario. Include hostile component/descriptor bytes, stale plan, cancellation before execution, and same-document-id/different-space isolation. None may create a renderer session or leak an execution-target receipt to the plugin view state.

## Priority and nonclaims

1. Complete Space's descriptor/core/catalog closure. It removes the immediate boot blocker for both Home and Space without touching the 21 missing descriptors.
2. Exercise the already-bounded Home-to-Space administration path against the real closure.
3. Add the verified GIS executor handoff and a single artifact-only persisted map edit.

This audit does **not** claim that Space59241 has finished, that any current browser tab rendered Home/Space/Map, that all 38 staged descriptor modules are activatable, or that the GIS viewer placeholder is usable. It also does not claim that `patchPositions` establishes fixed-three Map group durability.
