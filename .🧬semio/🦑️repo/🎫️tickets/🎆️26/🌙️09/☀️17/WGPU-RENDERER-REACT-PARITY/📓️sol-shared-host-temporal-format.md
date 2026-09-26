# Shared Host Temporal Formatting

## Scope

EventFeed previously owned a time-only page contract while VirtualFileSystem decoded time descriptors and painted raw or renderer-local strings. The native EventFeed trait had no production installer. This slice replaces that split with one schema-first host temporal contract and one accepted-frame cache used by both surfaces.

## Contract

The generic `host-temporal-format.v1` request carries:

- a bounded list of stable ids;
- a tagged source, either an epoch millisecond instant or an ISO string;
- `time`, `date`, `dateTime`, or `relative` presentation;
- a numeric epoch `nowMs`, quantized by the renderer for deterministic relative labels.

The reply carries the host profile (`locale`, `timeZone`, `hourCycle`, `profileRevision`) and exactly one nonempty label per requested id. Rust and TypeScript reject partial, duplicate, oversized, stale-profile, and mismatched replies. Invalid ISO strings remain values in the request and the formatter returns the original ISO text, matching the React VFS behavior.

The old EventFeed-only types, operation, module taxonomy, and source paths were removed. There is no compatibility operation.

## Browser and React

The page host door now answers only `format-temporal-values`. The TypeScript implementation uses `Intl.DateTimeFormat` and `Intl.RelativeTimeFormat`, advances the profile revision only when the resolved host profile changes, and shares its single-value formatter with the actual React EventFeed and VirtualFileSystem components.

The neutral fixture covers two DST zones, UTC, epoch EventFeed values, all three VFS presentation formats, forward and backward relative values, and invalid ISO values. The Vitest oracle computes the same outputs directly with browser Intl and validates the fixture plus request/reply JSON schema through Ajv.

## WGPU Retained State

`HostTemporalPresentation` replaces the EventFeed-specific cache. It owns one request generation and pending/candidate/queued/accepted reply sequence. Shell frame seal, acknowledgement, and discard move generic temporal candidates with the accepted frame. Profile revisions older than the accepted profile are refused.

EventFeed emits epoch sources with `time`. VFS decodes the authored descriptor `format`, collects all visible time cells into one bounded batch per frame, maps authored `datetime` to wire `dateTime`, deduplicates equal cells within a column, and resolves labels through the current scene host. A missing or pending host label paints the authored ISO rather than inventing a renderer timezone.

## Native Host

`run_native` installs `SystemTemporalFormatter` directly. The formatter is behind `NativeHostTemporalFormatter`, reads the system locale/timezone profile, converts epoch and RFC3339 ISO sources through the platform local-time API, formats all four presentation variants, preserves invalid ISO text, and owns profile revisions. No runtime dependency was added.

## Validation Receipts

Passed:

- WGPU temporal schema/Intl/page-door oracle: 4 tests.
- Actual React VFS calendar and relative formatter law: 1 test, 568 filtered.
- Renderer React TypeScript typecheck after the Tree-detail closure.
- `rustfmt --emit stdout` parser pass over the generic contract, contract law, native formatter, Scenes, EventFeed tests, renderer root, and Shell.

The first renderer React typecheck exposed only unfinished Tree-detail propagation in this same ticket: accessibility narrowing and four Interpreter story literals. Those were repaired and the rerun passed.

Coordinated native build `native156` owns the Rust compile and native laws. Exact new or updated filters:

- `shared_fixture_reply_is_exact_and_strict`
- `native_system_formatter_covers_every_generic_format_and_preserves_invalid_iso`
- `temporal_labels_publish_only_with_the_accepted_frame_and_stale_profiles_lose`
- `vfs_visible_time_cells_form_one_generic_host_temporal_batch`
- `vfs_descriptor_cells_decode_the_actual_react_tagged_union`

## Additional Tree Closure Found by the Typecheck

`uiAccessibilityProjectionNodeV1` now narrows a TreeItem before reading `inlineToolbar` and `detail`, and the Interpreter story TreeItem literals explicitly carry both required relations. These are direct completion of the earlier Tree contract packet, not temporal behavior.

## P0 parity corrections

A follow-up audit found three production mismatches. EventFeed had treated epoch `0` as absent, VFS had minute-floored `nowMs` before relative formatting, and the browser completion path addressed only a reusable host id plus a presentation generation. EventFeed now admits every safe integer including epoch zero. Both surfaces forward the exact renderer clock. Each temporal request now captures the `AdmittedSurfaceToken`; completion and refusal mutate state only through `get_token_mut`, so a late reply from a retired instance cannot attach to a remount with the same host id.

The neutral fixture adds epoch zero, an exact 59-second relative threshold, and ISO date-only values. The browser Intl/schema/page-door oracle passed 4/4 after this addition.

The native formatter no longer synthesizes locale strings with hand-written German/English templates. Its owned platform seam delegates macOS labels to CoreFoundation localized date patterns and Foundation `NSRelativeDateTimeFormatter`, records the system locale/time-zone/hour-cycle profile, and retains the existing accepted-frame cache. The shared ISO ingress accepts date-only input in UTC like JavaScript `Date`. POSIX and Windows adapters explicitly refuse service rather than publish a false locale/time-zone profile until an exact shell adapter is present.

Native laws prepared for the coordinated run:

- `temporal_completion_is_fenced_by_the_admitted_surface_token`
- `epoch_zero_is_a_valid_event_feed_time_source`
- `vfs_visible_time_cells_form_one_generic_host_temporal_batch`
- `profile_injected_platform_adapter_owns_every_label_and_revision`
- `native_iso_parser_accepts_date_only_in_the_same_utc_domain_as_react_date`
- `native_system_formatter_covers_every_generic_format_and_preserves_invalid_iso`

The macOS raw system FFI still requires the coordinated native compile/test receipt; no pass claim is made for it here.

Linux and Windows now load the operating system ICU implementation dynamically behind the same owned adapter. The adapter resolves the current locale as BCP 47, the ICU canonical default time-zone identifier, the actual default hour cycle, localized best patterns for the contract's fixed field widths, and ICU numeric relative-time labels. Linux searches the installed `libicui18n` ABI and versioned symbol namespace; Windows loads the OS `icu.dll`. Failure to resolve the complete formatter API refuses the batch rather than publishing partial or fabricated labels. This keeps the feature zero-package and prevents a reduced English fallback.

A focused Nx native run was attempted with the six temporal filters. Compilation stopped before tests on two concurrent renderer errors (`node` value missing and `InputState` type missing). It emitted no temporal adapter diagnostic, but that is not a compile or pass receipt for the temporal packet. The exact run log is temporary generated output under `🗑️generated/sol-temporal-native/run.log` and must be removed when the coordinated verification is captured.
