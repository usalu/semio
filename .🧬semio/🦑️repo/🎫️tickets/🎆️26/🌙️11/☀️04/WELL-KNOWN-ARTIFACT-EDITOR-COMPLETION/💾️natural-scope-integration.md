# Actual Natural File Scope Integration

The focused suite invokes the production buildScopeState and production picked-file/opening/durable-admission coordinators. Only external file selection, routing/format declaration and panel projection are controlled ports; create/import/archive/identity/disk/attachment/catalog are observed mocks. No helper-only success is counted as host integration.

Nine tests pass (20.10seconds, single worker): Ajv neutral corpus validation; selected filename, real parent UUID and exact selected surface retention; import then archive write then attachment then deferred durable catalog acknowledgment before focus and opened notice; duplicate ID refusal before write; exact full archive/surface/UUID reopen; already-open focus without disk read or creation; failed load, wrong archive identity, stale host and cancellation cleanup through drain then destroy; genuine deferred native import acknowledgment blocks all persistence and opened notice.

Neutral expected traces now live beside the tests. Canonical broad host config includes this suite. Canvas close fixture updated for required local catalog/status/current/open props; focused rerun separately pending.

Scope: these controlled-port tests prove wiring and acknowledgment ordering, not native compilation, real disk durability or browser reload. The real acceptance harness must still establish those.

## Save acknowledgment gap

Current saveNaturalFile exports natural bytes, downloads them and emits file-saved. It does not itself prove the bound document actor durably persisted a fresh complete archive reflecting that exact edit. Background actor auto-persistence can race the download, so an earlier saved status or a catalog admission acknowledgment is insufficient proof of the current content. The save command must await persistence and fresh archive readback for the exported revision before claiming saved. Draft-backed palette/text edits must first publish their owned edit through the actual editor operation and await its acknowledgment; exporting the prior store cannot satisfy Save.

Browser acceptance must continue comparing baseline, edited, undo, redo, reopen and reload exports, and must preserve the saved-state assertion. Do not remove persistence/reload checks or accept the downloaded file alone. This gap is distinct from the imported-file admission path tested above. Root owns the save flow correction.

Canvas fixture verification completed:4/4pass17.63seconds with current required local catalog/status/open inputs. Root confirmed generic local documents have unscoped runtimeKey equal to document UUID; current-document and persisted badge keys therefore match this route.

## Save exact archive proof

Expanded the production scope integration suite to18tests; all18pass in11.69seconds after root source fences. Save submits localDocumentArchive to the exact active actor and probes externalChanged using the real wire codec. False status does not probe; true status probes but cannot download alone. Older archive bytes keep Save pending. Exact fresh bytes gate native export, download and file-saved notice. Palette drafts write, attach and await catalog acknowledgment before fresh proof and export. Cancel, stale session and archive read failures prevent download and saved notice, retire observers, and release the transfer slot.

Regression initially demonstrated a stale owner could finish native export after its verified owner was retired and still download. Root added post-export ownership plus fresh archive equality fences. New regressions prove no download after owner retirement during export, edits during export, and plugin/worker replacement during the final archive read. These all pass. The harness client ID was corrected to the actual wire-required UUIDv4; the document ID remains UUIDv7.

Boundary: observer status/archive delivery is a controlled external worker port. These tests do not prove the new worker observer-forwarding branch preserves ordinary document dispatch; that remains a separate handler/browser verification requirement.

The worker forwarding gap is now covered by extracted production routeLocalDocumentObserverV1 and an11-test ordinary-handler integration suite; see 📬️document-observer-routing.md. Raw observer archive delivery coexists with actual ordinary readBack/bootstrap and typed storage failures reach both observer and saved-state dispatch.
