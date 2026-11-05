# Host Interface Review

Read-only review of current production scope, canvas, catalog and authoritative contracts; no broad typecheck run.

- AppDefinition v2 and AppIo v1 in schema/projection/rows-a-h require io:AppIo and artifactSchema:string. New direct schema access is correct. Stale comments saying I/O is not populated are contradicted by current descriptors. Descriptor results are in 🧬️descriptor-schema-audit.md: IDML absent and Semio text schema mismatched; other36 exact owners match an authored DOCUMENT_SCHEMA.
- PluginRuntime/handle-contract requires readAppDocumentIdentity with AppDocumentIdentity snake_case parent_document_id. Full archive readers, loaders and native export are optional and new scope feature-checks them before use. Loading progress and optional AbortSignal parameters match current calls.
- ActiveSession.viewState is the imported PluginViewState/ViewModel type. spawnedProgramViewStateV1 retains its generic input type while dropping host window selectors and choosing the receiver mode. Both new attachment paths supply the exact owning plugin, AppDefinition, instance and that view state.
- BackboneWorkerRequest send supports documentId, optional spaceId and optional clientInstanceId. New Save messages use captured entry documentId/client and optional entry.scope.spaceId, matching existing ordinary document sends. Client IDs are required UUIDv4 by the decoder; document UUIDv7 remains valid.
- recordLocalCatalogRef now returns Promise<LocalCatalog|null>. ReturnType-based directory and shell-core propagation retains it. All current replay admit/retire and new keep/unlist call sites await it. Optimistic projection is not used as acknowledgment.
- localDocumentOwnersRef has optional status and archive callbacks. Extracted observer helper uses optional calls; root catalog handler optional archive call was flagged separately and root owns its correction.
- Canvas passes current local catalog, actual syncStatusByDocumentId, current runtime key and Promise open callback to LocalFilesV1. Generic local documents are unscoped, so current runtime keys equal document UUIDs; root confirmed this route.

## Path bounds

The new SHA256 basename is64ASCII characters and has no extension. Native build_folder_endpoint therefore selects EventLog, whose authoritative file is target/.semio/events.semio (20additional characters). The reserve64 rule bounds this new local folder lane below256characters and avoids Windows reserved/invalid document-ID filename characters. UUID/schema-based Pack mirror paths are a distinct extension-bearing binding path and are not selected by the new target. With the current prefix and reserve, a trimmed dataDir longer than108characters is refused; actual VITE_S_DATA_DIR remains a runtime/startup check. Metadata text has an independent256UTF8-byte cap. This source review does not prove every browser server storage sidecar path; real browser save/reload remains required.

No additional concrete strict interface mismatch found in the new scope/canvas/catalog beyond the descriptor mismatch and optional catalog archive callback already reported.
