# Media Output Credit Interface Review

2026-10-28. Read-only current media root/producer/construction implementations and bounded Rust caller inventory in framework plugin, Layout plugin and Hub compositions. No source edits, test, compiler or graph execution.

## Authoritative Implementation

The extracted `📜️declaration/🎞️media/📤️producer/🦀️.rs` Option-credit implementation is the coherent current owner. The earlier root required-credit duplicate caused E0592 and should not coexist. During this review the root duplicate was removed by its writer; the producer remains mounted from the media root.

Compared with the obsolete body, producer preserves exact original source custody and handles jointly synchronized queue+optional credit admission. It acquires the actual queue gate and, when credit exists, the same nonblocking credit writer used by direct credit(). It checks original_source_parent, retained grant items/copy/depth, cancellation/yield, sealed/empty/chunk extent, checked total ceiling, checked outstanding byte increment, admitted fixed queue slot/allocation and credit ceiling/exceeded status. It consumes actual caller retained/fuel accounting and returns retained progress with charged refusals. The exact existing Vec allocation moves only after validation; failed push restores source custody. The credit value is committed only after the queue accepts the donor.

The obsolete body did not share the credit writer gate and reset bytes_remaining to total bytes rather than incrementing outstanding bytes, which is wrong after draining some prior chunks. It also treated missing donor as success rather than the current producer invariant refusal. These make the extracted tested implementation preferable on concrete semantics, not just signature preference.

## Why None Still Has Original Budget Authority

None omits an additional host media-operation byte-credit ledger; it does not omit the initiating caller budget or the queue byte ceiling. Ordinary Layout tool exports construct a LayoutExportJob with no media credit and transfer their already funded output vectors through the same original StepContext into a fixed-slot queue whose maximum is MAX_LAYOUT_EXPORT_OUTPUT_BYTES. Retained export construction obtains that queue through original scoped/admitted output construction; package commit reserves the caller items/header-copy work and passes the reduced actual retained loan into push_original. Vector generation/copy and transfer are separately charged by the output work/buffer owners. No fabricated credit, body or control is introduced.

True host media exports use the same core but the media factory installs input.output_credit through with_media_output_credit. Framework native snapshot media likewise explicitly passes Some(&outputs.0). Thus the extra byte ledger remains present on the actual media-export route, while ordinary direct download output remains bounded and original-funded without that ledger.

This reasoning is scoped to inspected production construction routes. A new caller must retain genuine original source context, funded vector ownership and declared queue construction; None alone is not evidence that an arbitrary export is properly budgeted.

## Current Callers and Migration

- Framework plugin media snapshot: push_original(&mut self.page, Some(&outputs.0), cx).
- Layout package commit: push_original(&mut output.publishing, self.media_output_credit.as_ref(), same), where the actual original retained loan reserves the parent commit work.
- No Hub composition Rust caller was found in the bounded inventory.
- Producer tests intentionally cover optional credit and missing original caller refusal.

The writer performed exactly two guarded repairs: removed the obsolete root duplicate and changed the framework snapshot argument from &outputs.0 to Some(&outputs.0). Layout already supplied as_ref(). The reviewed current callers now use the Option signature; no further migration is needed for these inspected production callers. Do not force a fake credit into ordinary tool exports merely to satisfy the obsolete duplicate signature.

The old public credit_original helper still uses atomic credit mutation without the writer gate, but no production caller of that helper remains in this inspected packet after removal of the root duplicate. Any future use concurrent with producer commit would need to honor the common writer authority; direct credit() already does. This is an interface observation rather than evidence of a current export-path failure.

## Validation Limits

Source review establishes the owner/caller/budget relationship. It does not claim successful compilation or media export execution; the focused compiler and actual SVG/PNG/PDF/browser witnesses remain coordinator-owned. Concrete authoritative body and call sites were sent to Rust before choosing the duplicate removal.
