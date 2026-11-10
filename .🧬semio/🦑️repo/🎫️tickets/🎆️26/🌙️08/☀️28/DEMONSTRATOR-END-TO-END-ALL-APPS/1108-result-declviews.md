# Lane declviews result (semio-plugin-decl-views, 237 errors in W-main-182)

## State (step 1: all red files edited, verify W-declviews-1 wasm)
- tool-run: `ToolRunTraceKeys` holds `SharedOriginalStorage<AtomicU64>` (manual Default via `from_cold`); `ToolRunView` holds `SharedOriginalStorage<BTreeSet<u64>>` / `Option<SharedOriginalStorage<Vec<u8>>>` (manual PartialEq); `OriginalToolRunJobPortRetirement` = inherent `close_step(body,effect)` + `native_retirement_reply!`, `RetireOwned` via `retirement_seed`.
- operation-contexts: `AppOperationContext` derives `RetireOwned`.
- empty-lanes: derive(RetireOwned) uninhabited enums fixed in the derive (`value/✨️derive/🧬️retained-clone`: wildcard arm in every cursor match when there are no variants).
- presence / transient / draft-none retirement: bespoke Arc factories removed; `SharedValueRetirementFactory` / `OwnedValueRetirementFactory` (same as ownership `bounded_transient_*`); presence disposer drives `PresenceStoreRetirement::close_step(&mut effect.nested(child))`, Pending -> zero progress.
- child read / command / custody / lookup / lifted read: cursors effect-shaped (`advance`/`close_step`/`take_authority`/`admit_qualified_source` take `&mut RetainedAllocationEffect`), `work` fields and `last_turn_work` removed on read cursor, command cursor, authority, custody, lifted read; `crate::child_view::pay_work` charges grant-shaped children (lender.read, ChildEditorSourceCursor) to the effect.

## Interface changes (callers outside this crate)
- `ChildReadRegistration::qualify` / `admit_original_source`, `ChildEditorRegistration::qualify_original`, `ChildEditorRead::admit_original_source`, `OriginalChildEditorCommandCursor::take_authority`: last argument is the effect, not a grant (take_authority returns `Result`, no `RetainedWorkOutcome`).
- `ChildReadCursor` / command cursor / authority / `ChildReadSourceCustody` / `ChildContentReadRetirement`: `close_step(body,effect)`, no `last_turn_work`.
- `ToolRunView.provisional_entities` / `.payload` and `ToolRunTraceKeys` are `SharedOriginalStorage`; tool-run entry (`entities`, `payload`, `trace_keys`) and decl-app `ContextToolRunSources` must hold the same type.
- `bounded_presence_root_retirement_factory<P: RetireOwned + Sync>`; `NoPresenceRetirementFactory` / `NoTransientRetirementFactory` removed.

## Debts
- cfg(test) code in `lifted/👁️read` (`shared_root_laws`), `command/🧪️tests.rs`, `📖️vector/🧪️tests.rs` still uses grant shapes.
