# Normalized command map

This schema keeps the collaborative/store-facing commands and drops purely visual UI commands.

## Included as store/backbone commands

### Session and collaboration
- `START_SESSION`
- `HEARTBEAT_SESSION`
- `END_SESSION`
- `RECONNECT_SESSION`
- `SET_SESSION_SELECTION`
- `BEGIN_TRANSACTION`
- `FINALIZE_TRANSACTION`
- `ABORT_TRANSACTION`
- `UNDO_TRANSACTION`
- `REDO_TRANSACTION`
- `UNDO_HISTORY`
- `REDO_HISTORY`
- `VOTE_CANDIDATE`
- `RESOLVE_CONFLICT`

### Kit entity CRUD
- `CREATE_AUTHOR` ← `semio.kit.createAuthor`
- `UPDATE_AUTHOR` ← `semio.kit.updateAuthor`
- `DELETE_AUTHOR` ← `semio.kit.deleteAuthor`
- `CREATE_TYPE` ← `semio.kit.createType`
- `UPDATE_TYPE` ← `semio.kit.updateType`
- `DELETE_TYPE` ← `semio.kit.deleteType`
- `CREATE_DESIGN` ← `semio.kit.createDesign`
- `UPDATE_DESIGN` ← `semio.kit.updateDesign`
- `DELETE_DESIGN` ← `semio.kit.deleteDesign`
- `CREATE_QUALITY` ← `semio.kit.createQuality`
- `UPDATE_QUALITY` ← `semio.kit.updateQuality`
- `DELETE_QUALITY` ← `semio.kit.deleteQuality`
- `CREATE_PORT` ← `semio.kit.createPort`
- `UPDATE_PORT` ← `semio.kit.updatePort`
- `DELETE_PORT` ← `semio.kit.deletePort`
- `CREATE_TAG` ← `semio.kit.createTag`
- `UPDATE_TAG` ← `semio.kit.updateTag`
- `DELETE_TAG` ← `semio.kit.deleteTag`
- `CREATE_CONCEPT` ← `semio.kit.createConcept`
- `UPDATE_CONCEPT` ← `semio.kit.updateConcept`
- `DELETE_CONCEPT` ← `semio.kit.deleteConcept`
- `CREATE_FILE` ← `semio.kit.addFile`
- `UPDATE_FILE` ← `semio.kit.updateFile`
- `DELETE_FILE` ← `semio.kit.removeFile`
- `CREATE_FOLDER` ← `semio.kit.createFolder`
- `UPDATE_FOLDER` ← `semio.kit.updateFolder`
- `DELETE_FOLDER` ← `semio.kit.deleteFolder`
- `MOVE_ARTIFACT_TO_FOLDER` ← `semio.kit.moveToFolder`

### Design / piece / connection edits
- `CREATE_PIECE` ← `semio.kit.addPiece`, `semio.designApp.addPiece`
- `CREATE_PIECES` ← `semio.kit.addPieces`, `semio.designApp.addPieces`
- `UPDATE_PIECE` ← `semio.designApp.updatePiece`
- `UPDATE_PIECES` ← `semio.designApp.updatePieces`
- `DELETE_PIECE` ← `semio.kit.removePiece`, `semio.designApp.removePiece`
- `DELETE_PIECES` ← `semio.kit.removePieces`, `semio.designApp.removePieces`
- `CREATE_CONNECTION` ← `semio.kit.addConnection`, `semio.designApp.addConnection`
- `CREATE_CONNECTIONS` ← `semio.kit.addConnections`, `semio.designApp.addConnections`
- `UPDATE_CONNECTION` ← `semio.designApp.updateConnection`
- `UPDATE_CONNECTIONS` ← `semio.designApp.updateConnections`, `semio.designApp.dragUpdate`
- `DELETE_CONNECTION` ← `semio.kit.removeConnection`, `semio.designApp.removeConnection`
- `DELETE_CONNECTIONS` ← `semio.kit.removeConnections`, `semio.designApp.removeConnections`

### Higher-level semio design operations
- `DELETE_SELECTION` ← `semio.designApp.deleteSelected`
- `FIX_PIECES` ← `semio.designApp.fixPieces`, `fixPieceInDesign`, `fixPiecesInDesign`
- `CLUSTER_PIECES` ← `semio.designApp.clusterPieces`, `createClusteredDesign`, `replaceClusterWithDesign`
- `EXPAND_DESIGN_REFERENCE` ← `semio.designApp.expandDesign`, `expandDesignPieces`
- `DRAG_PIECES` ← `semio.designApp.dragUpdate`, `dragPiecesInDesign`
- `MOVE_PIECES` ← `movePiecesInDesign`
- `PASTE_DESIGN_SELECTION` ← `semio.designApp.pasteSelection`
- `FLATTEN_DESIGN` ← `flattenDesign`
- `CHANGE_PIECE_TYPE`
- `CHANGE_PIECES_TYPE`
- `CREATE_FIXED_PIECE`
- `CREATE_CONNECTED_PIECE`
- `CREATE_HANGING_PIECES`

### Kit import/export
- `IMPORT_KIT` ← `semio.kit.import`
- `RESET_KIT` ← `semio.kit.reset`
- `EXPORT_KIT` ← `semio.kit.export`

## Explicitly excluded from the backbone/store schema

These were found in `sketchpad.tsx` but are UI-local and should stay client-side instead of entering authoritative backbone history:
- `semio.kitApp.*`
- `semio.sketchpad.*`
- hover/focus/fullscreen/theme/filter/table/navigation commands
- docs/tutorial/recording commands
- quality formula editor UI commands

## Main fixes applied

- Removed broken references and incomplete types from the old SDL.
- Normalized `release` vs `version` to `release`.
- Replaced wrapper-style ID objects with direct object references on outputs and `...Id` fields on inputs.
- Split piece ownership from design reference:
  - `Piece.design` = parent design pointer
  - `Piece.designReference` = referenced/nested design pointer
- Fixed folder references that were incorrectly typed as `String`.
- Replaced ad-hoc/incomplete interaction types with a consistent candidate/change/conflict/session/transaction model.
- Added authoritative backbone, linear interaction history, session timeout, candidate agreement, validation, conflict resolution, transaction stack, finalized history, and subscriptions.
- Kept direct raw diff application out of the public mutation surface.
