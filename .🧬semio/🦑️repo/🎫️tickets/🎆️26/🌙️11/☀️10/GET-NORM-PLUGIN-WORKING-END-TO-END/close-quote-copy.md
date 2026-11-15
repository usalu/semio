# Close-quote copy bound (systemic)

## Root cause
`PagedList::release_empty_page` denies unless `grant.maximum_copy_bytes >= next_release_copy_byte_demand()`. Close quotes (`RetirementDemand`) that declared only `release_bytes` and `depth` produced a grant equal to the quote with `copy_bytes: 0`, so the release was denied, the caller saw no progress and stalled, reported phantom progress or looped.

## Fix at the shared place
- `PagedList::next_release_demand()` (`🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`) quotes copy, release bytes and depth of the next empty-page release in one value. New quotes use it, so no caller can declare a partial release quote.
- `UiFixedList::next_release_copy_byte_demand()` forwards the copy demand for the ui fixed list.
- Every existing close quote that put `next_release_allocation_bytes()` into `RetirementDemand { release_bytes ... }` without a copy field now carries `copy_bytes: <same list>.next_release_copy_byte_demand()` (22 sites, scripted rewrite, same error mapping as the release field).

## Sites changed
Framework: tool-run tick receive (native, progress: 3 quotes), value dictionary equal (`🟰️equal`), replication dictionary decode (source ranges), ui runtime resources and loans, ui contract values original (frames), pack record storage (`record_storage_retirement_demands`: symbol-table physical phase now quotes the symbol copy), ui flex tree (new `next_close_copy_byte_demand`) and mounted-layout collect-publish-and-close (trait copy quote delegates to flex when no leased list is open).
Products (os): flow host read compare, store replay derived, store original-outbox, store export, inference plan, spr history deflate, plugin declaration job lifted, publication mounted payload, window config registry.
Plugins (✏️s): layout drawing and export output, energy prepared bands, draw validation, fem sparse numerical protocol.
Not changed on purpose: owners that already quote copy through a separate `next_close_copy_byte_demand` (scratch, clamp, accumulate), and mounted-layout lists / ui original builder, which release through `release_empty_page_leased` (no copy grant exists there).
Open candidate: reactor receiving (`🔌️plugin/⚛️reactor/🔄️turn/📥️receiving`) checks release bytes and depth from the ambient step grant before `release_empty_page`; it has no quote, so a step grant without copy would still stall (not observed).

## Verification (logs in `🗑️generated/close-quote-copy-*.log`)
- `check --keep-going`: framework workspace, 9 crates (value-paged-list, pack-record-retained, tool-run-tick-wire, value-dictionary, replication, ui-wgpu-mounted-layout, semio-ui, ui-runtime, ui-contract-values): 0 errors. Root workspace, 9 crates (flow-host-read, store-replay-operation, store-backbone, store-text-io, inference, history, plugin-decl-tool-contract, publication-mounted, window-config-registry): 0 errors. The `✏️s` workspace artifact crates (layout, energy, draw, fem-sparse) were not compiled (no build slot).
- `test --lib`: semio-framework-value-paged-list 28/28 pass. semio-framework-tool-run-tick-wire 1 pass, 2 fail: both `cursor.admit(..).unwrap()` is `None` at admission, before any close (decoder admission, not this bug).
- Test targets that do not compile for unrelated drift: paged-list `paid-edit-faults` (RetainedWorkPoll vs RetainedWorkOutcome), pack-record-retained lib test (`NativeDecodeWorkRecipient` retired), ui-contract-values test crate exceeds its source byte boundary (22890 vs 21999 bytes remaining).
- Plugin job crates: see below.
- `test --lib -p semio-plugin-job-receiving -p semio-plugin-job -p semio-plugin-instance-initialization`: first run of six plugin crates was cancelled by its 3500 s timeout while queued/compiling (no results). A narrowed rerun was still running when this report was written (`🗑️generated/close-quote-copy-test5.log`); results not yet known. Not run: reactor-jobs, typed-ops, surface-fixture, laws crates.
