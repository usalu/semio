# Native Preference Document Capacity Contract Audit

## Decision

Do **not** lower `OS_SHELL_CONFIG_MAX_BYTES` from 64 KiB to the host service's 16 KiB physical-page size. The 16 KiB figure is an implementation bound for one page, while 64 KiB is the existing WGPU host-storage value policy for the single `semio.os.config` document. Native needs a retained, bounded four-page document service for that one logical value.

The proposed fixed-page fixture and law in `📓️astra-sol-native-preference-fixed-page.md` encode the opposite relation (`config-document-equals-service-page`) and should not be registered as the repair. It would turn an implementation limitation into the public capability and reject browser-valid shell documents.

## Source Evidence

| Contract | Authoritative evidence | Consequence |
| --- | --- | --- |
| React has one complete configuration document | `🧰️framework/🔨️modules/🖥️platform/🟦️.ts:244-305` defines `OsShellConfigSnapshot`, reads the single `semio.os.config` key, and serializes its entire next snapshot in one `StoragePort.set`. | A mutation must preserve the sibling projections in the same JSON value. |
| That document contains growth-capable projections | `…/🖥️platform/🟦️.ts:247-262,332-361` permits record-valued `namedLayouts`, per-app dock layouts, dock UI, and window panes; a named-layout save rewrites the whole per-app array into that document. | React has no 16 KiB document rule. The type and implementation admit a document larger than 16 KiB. |
| The WGPU browser page adopts 64 KiB for one carried value | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts:326-388` defines `WGPU_HOST_STORAGE_VALUE_MAX_BYTES = 64 * 1024`, retains values through that exact size, and drops only values greater than it. | The renderer's browser-side policy is 64 KiB per carried value; the separate boot snapshot has a 128 KiB aggregate bound. |
| The browser door tests that policy | `…/🧪️tests/🗄️wgpu-host-storage-door/🟦️.ts:87-101` refuses a value at `WGPU_HOST_STORAGE_VALUE_MAX_BYTES + 1`; `…:119-135` applies the same per-value and aggregate rules while snapshotting. | The testable WGPU boundary is 64 KiB, not 16 KiB. |
| The Rust WGPU shell explicitly adopts the same 64 KiB document maximum | `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:26499-26501,26777-26783` sets `OS_SHELL_CONFIG_MAX_BYTES = 64 * 1024` and makes `HOST_STORAGE_VALUE_MAX_BYTES` equal to it. | The shell and browser door were deliberately coupled at 64 KiB. |
| The service only implements one physical page | `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs:1657-1698` sets `STORAGE_FIXED_FILE_PAGE_BYTES = 16 * 1024` and rejects any requested maximum above it before opening the file. | This is a physical-page primitive, not authority to shrink the logical document contract. |
| Native currently calls the wrong primitive | `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:26665-26680` selects 64 KiB for `semio.os.config` then calls the one-page read/write functions. `raw_prefs_get` and `raw_prefs_set` route through those functions at `…:26684-26704`. | Every native config read gets `Err` and becomes `None`; every write returns `false` and is discarded, including an empty document. This follows directly from the two bounds; no runtime assumption is needed. |
| 4 KiB is a different flat-field boundary | `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1390,26653-26680,26707-26723` applies `SHELL_CHROME_IO_FIELD_BYTES = 4 * 1024` to standalone top-level storage key/value calls such as the tour and compute-worker fields. | It must remain a one-page 4 KiB lane. It is not a 4 KiB limit on the entries nested inside the config document. |

React's `StoragePort` itself has no numeric document ceiling: `…/🖥️platform/🟦️.ts:238-241,293-305` accepts and writes the serialized string. Thus 64 KiB is the explicit WGPU cross-target policy, while React's browser storage can physically accept any valid document within the browser quota. The audit found no committed production sample whose serialized document is already above 16 KiB; that is not needed to establish the mismatch because the shared schema and WGPU contract admit it.

## Required Service Shape

Keep `storage_worker_read_fixed_file_page` and `storage_worker_write_fixed_file_page` as the 16 KiB primitive. Add a separate host-storage service abstraction for one bounded logical document:

- `STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES = 64 * 1024`.
- `STORAGE_FIXED_FILE_DOCUMENT_MAX_PAGES = 4`, derived from the existing 16 KiB page size.
- `storage_worker_read_fixed_file_document(path, maximum_bytes)` and `storage_worker_write_fixed_file_document(path, bytes, maximum_bytes)`, or equivalent names with these ownership and admission rules.
- The document service owns the page set and retained generation/commit record. It accepts at most four complete pages totalling at most the requested maximum; it rejects a missing, duplicated, trailing, or over-limit page and never returns a partial JSON document.
- Write all pages for an inactive generation before atomically publishing the bounded current-generation record. A failed or refused write leaves the last committed document readable. This is necessary because `OsShellConfig.update` replaces one logical value, and its callers must never observe a mixture of old and new projections.
- Preserve the raw JSON bytes and the one `semio.os.config` key. Page layout is internal to the host service, not a native schema or a second set of preference keys.

Only `native_pref_read_page`/`native_pref_write_page`'s `semio.os.config` branch should switch to that document service. Non-config keys retain the existing 4 KiB page route. The native shell must continue to apply its 64 KiB document admission before calling the service.

## Fail-First Laws

Implement these before the service change.

1. In `🛎️services/🧪️tests/🔬️component-unit/🦀️.rs`, use the real document service and system-file oracle to write and read exactly 65,536 nontrivial bytes. Assert byte equality, four admitted pages, and a re-opened service read. Then attempt 65,537 bytes and assert refusal before publication; the prior 65,536-byte document remains exact.
2. In the same service test, construct a hostile retained page set with a page missing, an extra page, or a total larger than the committed length. Assert that the document read refuses rather than returning a prefix. This pins document integrity rather than merely teaching a caller to loop over page files.
3. In `🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs`, add a native-route law showing that the config key selects the 64 KiB document primitive and every other durable field selects the 4 KiB single-page primitive. Mutating either route in a source copy must fail the law.
4. In `🧪️tests/🗄️wgpu-host-storage-door/🟦️.ts`, add the exact-boundary companion to its existing plus-one case: a 65,536-byte `semio.os.config` value is served and snapshot-carried, while 65,537 bytes is refused/dropped.
5. Drive the service round-trip with a valid `OsShellConfigSnapshot` containing a named-layout row and dock/UI siblings, then perform a preference update and assert all siblings survive. This proves the logical-document rule actually serves the cross-renderer shape.

The ticket fixture `🔬️native-preference-page/🧫️fixtures/🔣️.json` must change its authority from `fixed-file-page` to the bounded document service, use `configDocumentMaxBytes: 65536`, `servicePageBytes: 16384`, `pageCount: 4`, retain `storedFieldMaxBytes: 4096`, and represent exact-document and plus-one-document cases. Its current exact-16-KiB cases only prove the old physical primitive.

## Scope and Confidence

High confidence in the capacity mismatch and the decision: both bounds, the call path, and the browser boundary tests are direct source evidence. Medium confidence in the exact on-disk generation/manifest encoding: current source has no multi-page service, so the retained-generation design is the required service property, not an observed implementation. No source or Cargo changes were made, and no Cargo command was run.

