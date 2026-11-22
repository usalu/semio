# PDF editor presence (ephemeral shared)

## Mechanism reused
Framework presence (`PresencePeer`, typed interaction channel, `PresenceTyping`, `EphemeralEmit.presence`, bounded peer table of 64, hub TTL 4 s). Selection (domain `objects`) and typing previews already travel on the typed channels and are not repeated.

## New crate `semio-s-artifact-stdio-pdf-presence-lane`
Path: `7.1.7/subsets/base/editor/👥️presence/📦️packages/🦀️rust` (workspace member added).
- `PdfPresence` lane (page, pointer, visible page rectangle, caret; per mille of page) with sparse diff, `SetPresence` mutation, DSL/pack/binary codecs. Same shape as the view-lane transient.
- `PeerBoard` fold: `Heard`, `Left`, `Disconnected`, `Reconnected`, `Tick`. Stale heartbeats ignored, away after 4 s, forgotten after 30 s, offline freezes peers as away (no UI freeze), after reconnect peers stay away until heard again, dropped after 10 s grace.
- `PeerOverlay` projection: pointers, viewports, carets, selections on the local page, per-page badges for the thumbnail strip, roster; `describe` in en and de; `peer_layers` scene layers (Okabe-Ito palette, labels as text, away peers dimmed).
- `PresenceLane` trait (NoPresence follows nothing) so view and presence work serve all editors; actions `presence-pointer`, `presence-pointer-leave`, `presence-caret`, `presence-caret-clear`.

## Wiring
- View lane (`view-lane/🧵️work`): `ViewCommandWork` also publishes page + viewport; new `PresenceCommandWork`; page body adds peer layers, thumbnail rows append names ("here: Ada").
- Page crate: `peer_overlay_layers`, `object_page_of`, presence action definitions; thumbnails `render_thumbnail_strip_marked`.
- All 7 1.7 editors (base, a, e, h, vt, ua, x): `Presence = PdfPresence`, `presence_lane_stores!`, roster ids.

## Tests
Fixture `🧪️tests/🔬️fixtures/🧮️two-collaborators.json` (events in, overlays out, language-agnostic); unit tests for fold, bound, en/de, layers, intents, mutation inverse laws.

## Follow-ups (second pass)
- Framework (domain-neutral): new default `render_with_peers(.., peers: &PresencePeersView<Presence>, ..)` on `ArtifactApp`, `ArtifactEditor`, `ArtifactViewer` and the `EditorApp`/`ViewerApp` wrappers; the render surface now calls it with the live peer root. The default forwards to `render_with_request_context`, so every existing render is unchanged. `PresencePeersView` is re-exported from `semio_plugin`.
- PDF: all 7 1.7 editors and the three 1.4 editors (base, a, x) override it, so page, pointer, caret, viewport and selection overlays and thumbnail captions use typed presence (`typed_samples`, `render_page_body_peers`, 1.4 `render_body_peers`). Presence action ids are in `PAGE_ACTIONS` (now 90), `edit_from_action`, the rosters and `transient_tools`; 1.4 canvas routes them.
- Two-user test: `📜️pdf-hub-presence.ts` (run with `PDF_HUB_SCRIPT=📜️pdf-hub-presence.ts` through `📜️pdf-hub-runtime.ts`, which starts `os-hub:dev-collaboration`). User A selects page 2; user B sees "here: ..." on that thumbnail; after A leaves it disappears.

## Gaps
- Nothing here is compiled or run yet: every cargo, repo-run and generator job was queued behind host memory limits when I stopped. Expect compile errors to fix.
- Viewers (1.7 and 1.4) still use `NoPresence`: they have no view lane to publish from and their render does not share the page body.
- Presence schema leaves (json/ts/graphql/proto/rs) still show `NoPresence` until the surface-schema generator finishes (started in the background; output in the scratchpad `schema-gen.txt`).
- Caret is drawn at the object's left edge (no text metrics); the third-party cross-check test is not added.
