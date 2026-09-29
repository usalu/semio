# Presented WGPU Media Slots

## Contract and Presentation

The current source already contained the canonical seven-field resource validator and updated ready fixture. This slice adds missing, extra, invalid instance/document/generation, revision mismatch and controller length vectors. Rust and TypeScript independently validate the same fixture; the existing Ajv 2020 oracle validates schema admission. Revision equality is a semantic contract law beyond standard JSON Schema, identified separately by `contractValid` in the one revision-mismatch vector.

The native accessibility projection gives the exact reserved media extension the localized painted fallback as its status label and removes interactivity. Native snapshots carry no media slots and initiate no exports. No native decoder boundary exists.

The shell now records retained body rectangles even when the retained extension has no hit targets. Candidate body rectangles and media owner identity are sealed into `PresentedInputGeometry`, promoted with its existing GPU acceptance witness, and cleared for every new chrome walk. Per-surface owner snapshots join the rendered session plugin, instance and controller with a separate trusted document identity registry. Active and spawned sessions are captured independently; a missing/unavailable identity retains the localized fallback. Media resources must match controller, app instance and parent document identity.

A strict presented-tree reader refuses unacknowledged surfaces. The renderer collector walks accepted mounted layout only, composes each rectangle with the accepted shell body, clips to that body and every accepted CLIPS_CHILDREN ancestor, applies the shared scroll-aware child origin, follows disclosure and presence visibility, and emits document node identity and stable paint order. The fixed 64-character first-party SHA-256 token binds window, node id/key, trusted owner/document lease and the canonical seven-field resource authority (including revision and generation). UI tree/layout/surface epochs are excluded so seeking, selection, rerenders and concealment do not re-export the same owned playback resource. A neutral exact identity/hash vector is checked against Node crypto in the test-only oracle. Hashing avoids JSON escaping making a valid bounded window/key exceed the token budget. There are independent caps of 32 slots and 65,536 JSON bytes; overflowing a publication yields an empty slot list.

The immutable list travels through `AppPresentCursor`, `AppPresentStep::Complete`, `RenderSnapshot`, `BrowserRedrawOutcome` and `BrowserTickOutput.mediaSlots`. Presentation faults publish no browser slots. Candidate lists are never emitted to the page. The list persists through pending ticks. The canonical required `occluded` field preserves accepted slot identity and playback through temporary concealment. Accepted shell menus/dialogs/dropdowns/tour/drag/sync cards/tooltips and accepted retained overlays/tooltips mark replacement controls occluded. Later window occluders use the accepted outer chrome rectangle when it contains their body; later painted sibling nodes also mark earlier media occluded. Scrolled-off media stays published with zero clip extent and `occluded: true`. The DOM host hides pointer/control/ARIA surfaces while preserving the runner; removal occurs when retained ownership disappears. Geometry schema permits zero bounds only when occluded.

## Validation

Focused Rust and third-party schema tests have been added. Execution is coordinated through the root agent's renderer Cargo gate; no passing result is claimed yet. Shared geometry and owner vectors cover body composition, partial clipping, retained hidden total clipping and forged controller/instance/document identities. Rust also covers nested scroll clipping, slot count and aggregate descriptor-byte admission, and generation-scoped tokens. The TypeScript oracle checks canonical descriptor schema admission and refuses unknown fields and overflow count.

## Follow-Up Checks

Confirm native and wasm compilation, run the new tests, and verify end-to-end browser overlay playback with the sibling media transport lane. Confirm native and wasm identity query compilation against a mounted runtime resource and the newly recorded spawned session.

## Spawned-App Authority Audit

`SpawnedAppEntry` stores workflow plugin, instance and app identity but no document identity. `spawn_plugin` creates the app without a document load. The refresh renders its active app into `spawned_ui`; `plan_dock_windows` names the painted retained surface literally `spawned`, while its current dock reconcile receives the active session controller. That routing defect was communicated to the windows lane, which owns rendered spawned-session retention and controller/action routing. The collector now joins that session with its independent document identity and captures the correct owner for the literal surface and concrete workflow window ids.

The existing `ProgramBridgeEntry::read_app_document_archive` provides an independent root pack but copies the complete bounded archive (up to 4 MiB) and offers no standalone root-id response. Polling it on the presentation path would violate the intended bounded read-only descriptor seam. The implemented version-20 protocol adds command tag 41 `ReadDocumentIdentity` and frame tag 31 `DocumentIdentity`. The response contains only the actual app instance and optional bounded document envelope id; `VcsArtifactApp` returns `store.envelope().id`, while apps without document ownership return null. Query responses require one exact correlated receipt and matching instance. JSON and wire replies are byte bounded, document ids admit at most 512 Unicode scalars, and instance ids fit u32. The shell queries on UI refresh rather than every presentation step, retires registry entries outside current sessions/workflow apps, and seals ownership into the accepted geometry witness.

The neutral document identity schema and three exact wire vectors cover mounted, unavailable and Unicode/u64-max cases. Rust covers direct and paged command decoding, frame bytes, invalid presence, overflow, empty/oversized document ids and trailing bytes. The sibling TypeScript lane reports these vectors passing against its codec plus independent Ajv and @webassemblyjs LEB128 oracles, with real AppChannelClient receipt/owner refusals. Root-native and wasm gates remain the integration authority; initial failures included an in-flight protocol source snapshot and a corrected test-only `UiDocumentTree` import. No passing Rust/native/wasm outcome is claimed here yet.


## Concealment Revision

Parent review identified that omission during a popup would cancel/revoke/re-export the retained playback resource and pause audio, unlike React. The collector and schema now retain occluded descriptors. Four neutral overlap vectors include foreground header chrome, separate windows, touching edges and later siblings. The sibling media lane owns DOM concealment and tests retention through occlusion.


## Identity Refresh and Build Repair

Every active/spawned UI refresh performs the scalar identity query after app render, including an earlier null result. The bounded registry now clears candidate authority on a new request and admits only the current request receipt for the exact plugin/instance. Retirement removes the entry; stale responses cannot resurrect it. The neutral transition fixture covers null → document A → document B, an outdated receipt, a foreign instance and a reply after retirement. Rust and the Ajv oracle consume the same transitions. Token regressions cover different UI lease/tree generations and geometry with stable owned resource identity, and invalidate on document/resource revision/generation changes.

The second Wasm build hid its rendered error from Trunk output. Its compiler fingerprint revealed E0433: the existing first-party hash dependency was scoped to native only. It has been moved to the non-WASI renderer dependency section shared by native and browser. The exact cached compiler diagnostic was saved under the ticket generated output directory; the next root gate will validate the repair.


## Executed Identity Law

The root agent executed `document_identity_wire_matches_the_language_neutral_v20_fixture`: **1 passed, 1,183 filtered**. This confirms the Rust direct/paged scalar wire vectors and negative cases through the native OS kernel runner. The sibling media gate passed **25/25** focused tests, including the three presented slot oracle cases. The latest broader renderer native/Wasm source gates remain coordinated by the root agent.
