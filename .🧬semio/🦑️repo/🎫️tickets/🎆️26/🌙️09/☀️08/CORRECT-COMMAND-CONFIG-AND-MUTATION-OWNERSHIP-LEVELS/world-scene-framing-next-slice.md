# World Scene Framing Boundary

## Verified Source Behavior

Root inspected the current `World3dHost` after the neutral projection audit. The host reads a raw `cameraJson` string and uses its spelling to make lifecycle decisions. Near line4928, `includes('"position"')` selects between the parsed pose and instance-derived initial fit. Near line875, raw string inequality triggers reattachment unless the new parsed pose approximately equals the last dispatched gesture. Near line5986, presence of a projection spec enables repeated content fitting until a gesture takes local ownership. These are three different policies and must not become accidental consequences of introducing a typed active projection value.

The current projection pane handler near line5960 updates only local pending state. Its own source explains that the granular `setProjection {field,value}` command cannot transport its full active spec. Thus a pane change is not yet proof of persisted exact-window preference ownership. The current `WorldProjectionContentFrame` changes the renderer camera directly and calls the local adoption callback; it does not itself dispatch a window mutation. This is correct for automatic framing, but it must remain separate from user-triggered projection selection.

This is source evidence only. No new renderer test or behavior correction is claimed in this report.

## Required Typed Migration

The common scene must transport the neutral orbit and active projection independently. A configured field of view must be carried by its active projection mode. The editable full preset bank remains owned by the exact window and is not duplicated into renderer-owned state.

Framing belongs to the scene/renderer presentation boundary. Replace raw JSON spelling and active-spec presence with an explicit, closed framing policy only where needed to represent actual behavior: preserve a supplied pose; fit newly available content; or keep fitting expanding content until user interaction. The final schema should use only behavior observed in the production consumers. Do not add an implicit-perspective wrapper, compatibility parser, arbitrary generic options record, or persistence of local pending animation state.

The migration must define whether a changed scene orbit represents external replacement or a delayed echo of a local gesture. Use typed numeric equality and explicit owner/revision context where present. Equal typed values with reordered source fields must not reseed controls. A typed projection becoming present must not, by itself, restart content fitting.

## Acceptance Cases Before Switching Consumers

- A saved exact-window orbit and configured FOV survive initial render, document content replacement, and reopen without a projection-default override.
- A scene that requests fitting frames the first nonempty content and, when explicitly requested, growing bounds until the first gesture. Automatic fit emits no persisted user command.
- Two same-kind windows keep separate orbit, full projection preferences, pending framing state, and delayed-echo handling.
- A user projection selection changes the full preference bank and any required orbit atomically through the exact window owner. Inactive projection presets survive selection and recall. Derived active specs are renderer input, not a second persisted bank.
- A delayed self-echo and a content-only rebuild preserve the current live pose; an actual external pose replacement reseeds once.
- Perspective and orthographic renderer classes consume the same neutral projection contract in React and WGPU. Configured helper FOV currently being overwritten by the React default50 is recorded as a bug correction when fixed, not described as preserved runtime behavior.

The neutral range review confirmed that the editable controls own their slider bounds, while existing active helpers accept finite numeric values beyond those bounds. The neutral active transport therefore uses finite values without copying preference slider limits; new outside-control cases must prove perspective130, curvilinear200 and larger shift/depth values survive codecs. This establishes transport admission, not the usable domain of every projection matrix. The subsequent renderer/math boundary must explicitly handle unusable finite angles and retain the existing curvilinear effective-capture behavior only where its actual tests establish it. No invalid value should silently become a default lens. Authored Shooting cameras, icon cameras, renderer engine objects, and tutorial driver state retain their distinct owners.
