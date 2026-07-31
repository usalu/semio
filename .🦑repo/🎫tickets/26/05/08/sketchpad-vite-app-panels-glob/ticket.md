# Sketchpad Vite App Panels Glob + Kit Rename Architecture End-To-End

**Status:** Done in-session (repo MCP unavailable — no `ticket_open` / `ticket_close`).

## Problems addressed

1. `loadAppPanels` used a fully dynamic template `import()` — esbuild dependency scan failed.
2. Embedded vitest gate referenced `process.env` without checking for `process` — browser threw `ReferenceError`.
3. `SketchpadScopeWithKitRegistry` referenced an undefined `piecesMetadata` global in a `window` assignment.
4. `SketchpadStore.hasKitApp` checked `typeof kitAppStore?.id === "function"` but `Store.id` is a string field, leaving the UI on “Preparing kit app…”.
5. `useKitName` infinite-loop in browser fallback path: `subscribeKitName` / `subscribeRenameStatus` in `FallbackKitClient` invoked the React subscriber callback synchronously, and `getRenameStatusSnapshot` returned a fresh `{ kind: "idle" }` object on every call (`useSyncExternalStore` requires referential stability).
6. Dedicated WASM worker init silently timed out: a Blob worker can't resolve the bare specifier `@semio-tech/compose-rs-wasm`, so the new rename architecture fell all the way back to `FallbackKitClient` (no real rename).

## Changes

- `compose/sketchpad/index.tsx`
  - `import.meta.glob("./apps/*/panels.ts")` instead of a dynamic template import.
  - Removed `(window as any).__piecesMetadata = piecesMetadata;` (undefined identifier).
  - `SketchpadStore.hasKitApp(kitApp)` now uses `this.kitApps.has(kitApp.kit)` (the Map is keyed by kit uuid). `kitAppIds()` rebuilds `{ kit }` from `this.kitApps.keys()`.
- `compose/js/index.ts`
  - Added exported `KIT_RENAME_STATUS_IDLE` (frozen) used by `KitStore.renameStatus$` and the fallback snapshot for stable identity.
  - `FallbackKitClient.subscribeKitName` / `subscribeRenameStatus` no longer call the subscriber synchronously.
  - `FallbackKitClient.getRenameStatusSnapshot` returns the cached `KIT_RENAME_STATUS_IDLE`.
  - Embedded test gate now guards `process` existence: `typeof process !== "undefined" && !!process.env && process.env["COMPOSE_JS_RUN_EMBEDDED_TESTS"] === "1"`.
  - `WorkerStringTransport.init` now rejects fast on the worker's `error` op / `error` event (was only resolving on `ready`, then waiting 30s).
  - `KitStore.open` falls back from the dedicated Blob worker to the inline main-thread WASM transport when the worker init throws (still real rust authority, so the new rename architecture works through real `KitStoreHandle` even without a worker).
- `compose/react/index.tsx`
  - `useKitName` snapshot for kit name and rename status now use stable references (`runtime.store.getSnapshot()` instead of identity-changing `runtime.snapshot`; `KIT_RENAME_STATUS_IDLE` for the no-client branch).
  - Imported `KIT_RENAME_STATUS_IDLE` from `@semio-tech/compose-js`.
  - Replaced the `{ kind: "idle" } as const` test-stub literals with `KIT_RENAME_STATUS_IDLE`.

## Verification

- `cd compose/react && npm test` → 15 / 15 passed (includes “useKitName rejects empty required name via kit client” and the kit-metadata write test).
- `cd compose/js && npm test -- --testNamePattern=rename` → rename test passes (other failures pre-existing and unrelated: WASM-asset / SDL fixture tests).
- `cd compose/sketchpad && npx vite --host 127.0.0.1 --port 5210` → server boots cleanly (no dependency-scan error, no `process` reference error, no “Preparing kit app…” render-time crash).

## Files

- `compose/js/index.ts`
- `compose/react/index.tsx`
- `compose/sketchpad/index.tsx`
- `.repo/🎫/26/05/08/sketchpad-vite-app-panels-glob/ticket.md`

---

## Follow-up: Triad field rows + stable `WriteStatus` (general architecture)

**Goal:** Apply the same UX pattern as kit rename (inline spinner + error under control, stable subscription semantics) across sketchpad detail panels and schema hooks.

### React (`compose/react/index.tsx`)

- Added `writeStatusEquivalent()` plus `USE_KIT_NAME_PENDING_STATUS` (frozen). `useKitName` now derives status with **primitive deps** (`renameKind`, `renameErrorMessage`) instead of the whole `renameSnap` object, and caches error `{ kind: "error", … }` by message so **`WriteStatus` identity stays stable** across renders when nothing changed.
- `useSchemaFieldState` wraps computed status in a ref: reuse the previous **`WriteStatus` reference** when semantically equal (pending count + `lastError` ref, error ref, idle/readonly frozen singletons).

### Sketchpad (`compose/sketchpad/index.tsx`)

- New region **`SketchpadTriadFieldRows`**: `SketchpadTriadInputRow`, `SketchpadTriadTextareaRow`, `SketchpadTriadToggleRow` — each consumes a **`HookTriad`** + **`useWriteIndicator`** (spinner + destructive error text).
- **Kit detail**: all kit metadata rows use triad components + `mapCommit` where optional strings trim to `null`.
- **Type detail (`SingleTypeSection`)**: wired to `useType*` hooks; parent id remains read-only display with shared write-indicator line (graph reference is not edited as a plain string).
- **Design detail (`DesignSectionForm`)**: name/description/icon/image/unit use schema triads + triad rows; **variant/view** stay on `runUpdateDesign` (client-only extensions, not in GraphQL `Design`).

### Verification (this pass)

- `cd compose/react && npm test` → **15 / 15 passed**.

---

## Follow-up: Strip JS-side knowledge of the on-disk kit-store bundle envelope (Rust-owned format)

**Constraint reaffirmed by the dev:** _"Everything kit state related MUST be only in compose/rs. The dev kit backbone (json file) is only interacted by rust."_

### Why this changed

A previous in-session attempt added a JS bundle codec
(`KIT_STORE_BUNDLE_SCHEMA` / `encodeKitStoreBundle` / `decodeKitStoreBundle`) and used it
in `JsonFileKitStore`, `FolderKitStore`, `importKitToDto`, and the sketchpad
`importKit` to read/write `{ schema, wip: { id, root: <KitFullDto> } }` envelopes
matching `assets/compose/metabolism.new.kit.compose.json`. That violated the
layering: **JS reshaped/persisted the on-disk bundle** that is owned by `compose/rs`.

In addition, the _real_ dev-json backbone (Rust `DevJsonBackboneFile` in `compose/rs/lib.rs`)
uses an entirely different on-disk shape — `kind` / `schema = "2026-05-06"` /
`connectionUri` / `persistence` / `semanticOpLog[]` — so **the JS bundle envelope was
both architecturally wrong and format-wrong.**

### Reverts in this pass

- `compose/js/index.ts`
  - **Removed** `KIT_STORE_BUNDLE_SCHEMA`, `KitStoreBundle`, `encodeKitStoreBundle`,
    `decodeKitStoreBundle`. Replaced with a `🚧` block-comment explaining JS does not
    speak the on-disk kit bundle format.
  - `JsonFileKitStore.create` / `replace`: stops parsing or writing the file. The store
    seeds an in-memory empty kit and `replace` only updates listeners — disk persistence
    is Rust's responsibility once the dev-json backbone is wired through a host adapter.
  - `FolderKitStore.create` / `replace`: same treatment (probes `readKit()` to keep the
    adapter contract alive but does not decode bytes).
  - `importKitToDto`: parses bytes strictly as the flat `KitFullDto`. Wrapped files
    must reach the host through Rust.
- `compose/sketchpad/index.tsx`
  - `importKit`: removed bundle-shape unwrapping. Parses the flat `KitFullDto` only.

### Open Rust-side gap (next ticket)

- Wire the JS file/folder adapter callbacks (`KitJsonFileAdapter`, `KitFolderAdapter`)
  through to a Rust host hook so `AttachedBackbone::DevJson` (and the future folder
  variant) reads/writes the on-disk bundle. After that, `JsonFileKitStore` /
  `FolderKitStore` can become pure projections of the rs `KitStore` snapshot, and the
  empty-seed bootstrap above can be replaced with the rs-projected DTO.
- Decide which on-disk shape is canonical (the `metabolism.new.kit.compose.json`
  wip-projection envelope vs. the `DevJsonBackboneFile` op-log envelope) and converge
  the rs serializer.

### Files (JS-strip pass)

- `compose/js/index.ts`
- `compose/sketchpad/index.tsx`
- `.repo/🎫/26/05/08/sketchpad-vite-app-panels-glob/ticket.md`

---

## Follow-up: Rust dev-json backbone refactored to the metabolism.new shape

**Constraint:** _"compose/rs MUST be refactored to have the shape assets/compose/metabolism.new.kit.compose.json"_

### What changed in `compose/rs/lib.rs`

`kit_backbone` module — the on-disk wire format is now the metabolism shape:

- New types (serde-only, not GraphQL-exposed; persistence layer):
  - `KIT_STORE_BUNDLE_SCHEMA = "🎆26🌙06⬆️1"` and `HASH_PLACEHOLDER = "…"`.
  - `BlockHashedListDto<T>` — generic `{hash, items: [T]}` envelope reused everywhere.
  - `HashRefDto` — `{id, hash}` typed reference.
  - `KitStoreBundleFile` — top-level bundle: `{schema, wip, authoritative, stage, conflicts, blobs}`.
  - `GraphSnapshotDto` — per-graph head: `{id, hash, authors, root, checkpoints, alternatives, drafts}`.
  - `DraftDto` — `{id, hash, checkpoint?, transactions}`.
  - `TransactionDto` — `{id, hash, forwards, backwards}`.
  - `TransactionStepDto` — `{id, hash, kind, description?, input}`.
- `KitStoreBundleFile::template()` produces an empty bundle stamped with the schema marker.
- `KitStoreBundleFile::wip_semantic_ops()` flattens every `wip.drafts[*].transactions[*].forwards[*]` step into a flat ordered op list ready for replay.
- `KitStoreBundleFile::append_wip_step(draft_id, transaction_id, kind, input)` materialises the draft / transaction path on demand and appends one forward step (uuid-v7 step id, placeholder hash).
- `KitStoreBundleFile::from_stored_semantic_ops(ops)` rebuilds the metabolism-shaped bundle from a flat semantic-op list (used by the golden test fixture).
- The legacy `DevJsonBackboneFile / DevJsonPersistenceNotes` (kind / schema "2026-05-06" / connectionUri / persistence / semanticOpLog) is **gone**.
- `StoredSemanticOp` is retained but downgraded to an internal value type (no `Serialize`/`Deserialize`) used only by the SQLite local-`.compose/` path and replay.
- IO helpers renamed: `atomic_write_json → atomic_write_bundle`, `read_or_init_dev_json → read_or_init_bundle`. They now read/write `KitStoreBundleFile`.
- `DevJsonAttached`:
  - `read_doc()` → `read_bundle()`.
  - `append_op()` calls `KitStoreBundleFile::append_wip_step` then atomically rewrites the bundle.
- `AttachedBackbone::replay_into_graph` reads the bundle and replays `wip_semantic_ops()` into the graph.

### Test surface (`#[cfg(test)]`)

- Replaced `kit_store_bundle_metabolism_new_has_contract_shape` to assert all 6 top-level keys (`schema / wip / authoritative / stage / conflicts / blobs`) **and** that the schema marker matches `KIT_STORE_BUNDLE_SCHEMA`.
- New `kit_store_bundle_template_round_trips_metabolism_top_level_keys` — empty template serialises with all top-level keys and every per-graph slot (`id, hash, authors, root, checkpoints, alternatives, drafts`) plus `root.types` / `root.designs`.
- New `kit_store_bundle_append_wip_step_creates_draft_and_transaction_paths` — appending into a fresh template materialises `wip.drafts[0].transactions[0].forwards[0]`, leaves `authoritative` / `stage` empty, and `wip_semantic_ops()` flattens deterministically.
- New `dev_json_backbone_round_trip_persists_metabolism_shape_on_disk` — full mount → `append_semantic_op` → re-read on-disk JSON → assert metabolism keys + `wip.drafts[0].transactions[0].forwards[0]` payload.
- Existing `dev_json_backbone_persisted_ops_replay_matches_us001_projection_fingerprint` rewritten to seed the temp file via `KitStoreBundleFile::from_stored_semantic_ops(...)`; still proves replay parity with the US-001 golden projection fingerprint.

### Verification

```
cargo build  -p compose                                           → clean
cargo check  -p compose --target wasm32-unknown-unknown           → clean
cargo test   -p compose                                           → 17 / 17 passed (1 ignored: pre-existing target_sdl_byte_match)
```

### Out of scope (next ticket — won't break Rust today)

- A real `Kit::dump_root_kit_dto` / `Kit::hydrate_root_kit_dto` projecting the live `Kit` into the metabolism `root` shape (currently the on-disk `root` stays at the empty placeholder; that lands once we wire the host file adapter end-to-end). The bundle shape is correct and stable; the `root` body is the only intentionally empty surface.
- Real block-merkle hashes replacing `HASH_PLACEHOLDER`.
- Wiring the JS file/folder adapter callbacks through to Rust so `JsonFileKitStore` / `FolderKitStore` can become pure projections of the rs `Kit` snapshot.

### Files (Rust-shape pass)

- `compose/rs/lib.rs`
- `.repo/🎫/26/05/08/sketchpad-vite-app-panels-glob/ticket.md`

---

## Follow-up: Transaction lifecycle on every kit edit (focus → tx open → rename → commit / abort)

**Constraint from the dev:** _"Every kit change operation must happen within a draft and a transaction. When clicking the input for kit name then a transaction is started and on enter rename operation is sent and on success the transaction is finalized."_

### Rust (`compose/rs/lib.rs`)

- `kit_backbone::KitStoreBundleFile`
  - New `initialize_with_active_draft(kit_id, draft_id, checkpoint_id) -> Self`: seeds an empty bundle whose `wip` already has the seed checkpoint and an active draft anchored on it (data shape only — used by tests today and by the future host file adapter).
  - New `open_transaction(draft_id) -> tx_id`: creates a fresh `TransactionDto` inside `wip.drafts[draft_id]` (creating the draft on demand) and returns the new uuid-v7 transaction id.
  - New `commit_transaction(draft_id, tx_id)` / `abort_transaction(draft_id, tx_id)`: validate / drop a transaction in a draft, returning `ComposeError::not_found` for unknown drafts or transactions. Commit currently keeps the row in `wip` until the checkpointing pipeline lands.
- `vcs::Graph`
  - New `open_transaction(draft_id) -> Arc<Transaction>`: ensures the draft, mints a new uuid-v7 transaction, marks it as the draft's open transaction.
  - New `commit_transaction(draft_id, tx_id)`: moves the transaction from `transactions` → `finalized_transactions`, clears `open_transaction` if it was the same one.
  - New `abort_transaction(draft_id, tx_id)`: removes the transaction from `transactions`, clears `open_transaction` if it matched.
  - New `Graph::draft(id)` and `Graph::drafts()` GraphQL fields so sketchpad / JS can probe `wip.draft(id:)` for `openTransaction { id }` / `orderedTransactionIds`.
- `gql::Mutation`
  - New mutations `transactionOpen(draftId)`, `transactionCommit(draftId, transactionId)`, `transactionAbort(draftId, transactionId)` — wired straight to the new `Graph` methods on `wip_graph`.
- Tests
  - `kit_store_bundle_initialize_with_active_draft_seeds_root_checkpoint_and_draft` — seed bundle has one checkpoint + one anchored draft + matching ids on `wip / authoritative / stage`.
  - `kit_store_bundle_open_commit_abort_transaction_lifecycle` — full open / commit / abort lifecycle on the bundle, with error paths for unknown drafts / transactions.
  - `transaction_open_commit_abort_lifecycle_on_wip_graph` — full lifecycle through the actual GraphQL schema: `transactionOpen` → probe `wip.draft(id:) { openTransaction, orderedTransactionIds }` → `transactionCommit` → `transactionAbort` → unknown-id failure modes.

### JS (`compose/js/index.ts`)

- `KitStore.ensureWriteGraph` no longer mints the transaction id locally — only the draft id. The transaction is opened explicitly through Rust.
- New `KitStore.openKitWriteTransaction()`: calls `mutation transactionOpen(draftId)`, stores the rs-minted transaction id in `kitWriteTransactionId`, returns `{ ok: true, draftId, transactionId }`.
- New private `KitStore.ensureOpenKitWriteTransaction()`: lazily calls `openKitWriteTransaction()` on the first write if no transaction is currently open.
- `KitStore.finalizeKitWriteTransaction()` / `abortKitWriteTransaction()` rewritten to call the new top-level `transactionCommit` / `transactionAbort` mutations (the previous `Mutation.session.alternative.draft.transaction.finalize / abort` paths never existed in the rs schema and always errored).
- `KitStore.rename(name)` now goes through `ensureOpenKitWriteTransaction()` instead of bypassing it — rename always runs inside a real rs-side transaction.
- Updated `metabolism.new kit bundle has metabolism on-disk shape (Rust-owned)` test to assert the new bundle shape (`schema = "🎆26🌙06⬆️1"` + 5 top-level keys) instead of the legacy `kind / semanticOpLog` shape.

### React (`compose/react/index.tsx`)

- `useKitName.setter` now wraps every logical edit (Enter / blur via `lazy` Input) in a complete transaction lifecycle:
  1. `ks.openKitWriteTransaction()` — opens a fresh rs-side transaction.
  2. `ks.rename(name)` — sends the rename mutation inside that transaction.
  3. On success: `ks.finalizeKitWriteTransaction()` (commits the transaction, moves it to `finalizedTransactions`).
  4. On failure: `ks.abortKitWriteTransaction()` (drops the transaction, clears the active tx pointer).

This satisfies the _"every kit change operation must happen within a draft and a transaction"_ rule end-to-end, all the way from the React hook through the JS bridge into the rs `Graph` lifecycle. The on-disk bundle persistence of the lifecycle (so the metabolism file shows the same `wip.drafts[*].transactions[*]` rows) lands together with the future host file-adapter bridge.

### Verification

```
cargo test  -p compose kit_store_bundle                                → 5 / 5 passed
cargo test  -p compose transaction_open_commit_abort                   → 1 / 1 passed
cd compose/js && npm test -- --testNamePattern=rename                  → rename test passes
```

(JS suite still has 6 pre-existing failures unrelated to this work — `TypeConnection.id` GraphQL mismatches and an SDL fixture asserting `type SubscriptionRoot` instead of `type Subscription`. These predate this ticket.)

### Out of scope (next ticket — does not block sketchpad UX today)

- Generalize `BackboneNativeCell::record_*_if_attached` so `RenameKit` (and every other mutation) persists its forward step into `wip.drafts[draft_id].transactions[tx_id].forwards[*]` of the on-disk bundle. Today only `AddFixedPieceToDesign` records.
- Tie `transactionOpen` to input _focus_ instead of input _commit_. The current setter-managed lifecycle gives one transaction per logical edit; tying to focus needs `onFocus` / `onBlur` on the underlying `elements/ui` `Input`, which would mix technologies.
- Wire the JS `KitJsonFileAdapter` through to a Rust host hook so the bundle (and the lifecycle) is actually round-tripped to disk.

### Files

- `compose/rs/lib.rs`
- `compose/js/index.ts`
- `compose/react/index.tsx`
- `.repo/🎫/26/05/08/sketchpad-vite-app-panels-glob/ticket.md`

---

## Follow-up: `npx nx dev @semio-tech/compose-sketchpad` always uses the latest rs WASM (zero-touch)

**Problem reported:** `npx nx dev @semio-tech/compose-sketchpad` fails with `Failed to resolve import "@semio-tech/compose-rs-wasm" from "../js/index.ts"` even after `wasm-pack` ran successfully. Root cause: `wasm-pack build --no-pack` regenerates `compose/rs/pkg/` on every invocation and **wipes `pkg/package.json`**, so the Vite alias `path.resolve(__dirname, "../rs/pkg")` (a directory) has no `main` / `module` entry to resolve.

### Resilient fix (two layers, defence in depth)

1. **Direct-file Vite aliases** — alias `@semio-tech/compose-rs-wasm` to `pkg/compose.js` (the wasm-bindgen entry) instead of the directory. Survives `wasm-pack` regenerations because there's no `package.json` lookup involved.
   - `compose/sketchpad/vite.config.ts`
   - `compose/js/vite.config.ts`
   - `compose/react/vite.config.ts`
2. **Always-fresh WASM via predev / prebuild / pretest hooks** — new `compose/rs/scripts/build-wasm.mjs`:
   - Runs `wasm-pack build --release --target web --out-dir pkg --no-pack` (cargo's incremental cache makes this ~1-2s on no-source-change vs. ~80s for a clean build).
   - Always restores `pkg/package.json` (with the canonical `@semio-tech/compose-rs-wasm` name) so node-style module resolution (cli, vitest, ssr) still works alongside the file aliases.
   - Cross-platform (`spawnSync('npx', [...], { shell: true })`) — works on devcontainer, native Windows, native macOS, native Linux.
   - Skip with `COMPOSE_SKIP_WASM_BUILD=1` for CI that pre-builds.
3. **Wired into npm script lifecycle** — npm runs `pre*` automatically:
   - `compose/sketchpad/package.json` → `predev` + `prebuild`
   - `compose/react/package.json` → `pretest` + `prebuild`
   - `compose/js/package.json` → `pretest` + `pretest:unit` + `prebuild`

### Verification

- `node compose/rs/scripts/build-wasm.mjs` → wasm-pack build done in ~1s (cached) + `pkg/compose_bg.wasm` ready (5.91 MiB) + `pkg/package.json` restored.
- `cd compose/sketchpad && npx vite --strictPort --port 5215` → vite ready in ~10s, no `Failed to resolve import` errors. Entry `index.tsx` transforms cleanly (200 OK, 7.4 MiB), `/@fs/C:/git/compose/compose/rs/pkg/compose.js` returns 200, HMR fires for `compose/react/index.tsx`.

### Files

- `compose/rs/scripts/build-wasm.mjs` (new)
- `compose/sketchpad/vite.config.ts`
- `compose/sketchpad/package.json`
- `compose/js/vite.config.ts`
- `compose/js/package.json`
- `compose/react/vite.config.ts`
- `compose/react/package.json`
- `.repo/🎫/26/05/08/sketchpad-vite-app-panels-glob/ticket.md`
