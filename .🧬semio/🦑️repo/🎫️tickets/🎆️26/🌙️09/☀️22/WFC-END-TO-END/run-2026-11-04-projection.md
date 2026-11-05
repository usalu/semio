# Bitmap Retained Snapshot And Diff Projection

Implemented the assigned snapshot/diff slice, coordinated with the aggregate planner owner. Existing positional diff wire shapes are unchanged.

## Owned Snapshot

All owned snapshot records derive `RetainedClone`. `BitmapSnapshot` explicitly uses the existing `OwnedFieldProjection`, including its paid birth, binding, fragment, output transfer and controlled close frontiers. Added the projection crate as an explicit dependency of the snapshot package and artifact root which mount this source.

## Staged Diff

`BitmapDiff` implements `RetainedDiff` through the typed `BitmapApply` plan in the diff directory. It never calls the cold diff implementation. Preflight retains an immutable post clone through metered field preparation, separately copies a changed schema field, and stages all buffers before changing the post.

Input resizing and writes copy one pixel per admitted turn. Palette validation and assembly process individual rows. Pins use positional removal/move validation, after-slot insertion/movement, a survivor merge, final uniqueness indexing and modifications. Replacement performs no `Vec::remove` or `Vec::insert`, so a complete pin batch is linear in the bitmap extent and row counts. Its coordinate index is bounded by the authored output extent.

Every changed dynamic array first quotes and admits its backing allocation, then initializes/copies elements through individual paid turns. Dirty-field masks skip rebuilding unchanged pixels, palette and pins; empty and seed-only diffs transfer those owners from the paid candidate clone without allocating redundant arrays or dense indexes. Output-only changes still validate retained pin bounds without allocating a pin rebuild. Original input extents are bounded before platform-sized multiplication, including a malformed original combined with a valid resize request. Old buffers remain owned until controlled retirement. All snapshot/schema source aliases close before preflight reports Ready, allowing the exact projected post issuer to hand back mutable authority. Authorized commit exchanges one complete staged snapshot; refusal and cancellation leave the original post unchanged. Staged/displaced owners and aliases close through `RetainedCloneClose` and the field preparations.

## Portable Laws And Independent Oracle

Added `schema/diff/tests/retained.json` (the actual directories carry their repository emoji names) with five cases: complete replacement, exact prior order restoration, duplicate target refusal, late invalid region refusal and ordered region writes. Native tests exercise central preflight/commit, grant denial, unchanged original post, projection transfer/closure and cancellation at each staged frontier. Native output pixels are compared with the JSON expectations through third-party `serde_json`.

The retained ticket input `bitmap-retained-oracle.py` independently applies the region operations through Pillow, validates target uniqueness, and compares the same fixture verdicts and pixel outputs.

Added four allocation-budget cases for no change, seed-only, pixel-only and palette-only edits, plus a malformed-input extent case. Native allocator-observer tests compare every preflight allocation/release receipt with actual heap events and compare total retained capacity with the independently observed owned-field clone baseline. The portable additional backing budgets are zero for empty/seed-only edits, two bytes for the two-pixel-only edit and 32 bytes for the two-color-only edit. These tests are authored, not executed. Added the existing test allocator as a development dependency in both mounting packages.

## Verification Receipts

- `repo tools exec --light python -- <ticket>/bitmap-retained-oracle.py`: passed, Pillow matched all nine semantic cases.
- Bounded parser check through `repo tools exec --light rustup -- run nightly-2026-07-20 rustfmt --check --edition 2021 --config disable_all_formatting=true,skip_children=true` over snapshot, retained diff plan and retained native tests: passed.
- Scoped `repo tools exec --light git -- diff --check`: passed.
- Rust type checking and native runtime laws have **not executed**. Existing repo target admission is blocked before compiler entry by the host capacity conditions documented in the ticket. No redundant target was queued, and parser/oracle checks are not represented as native test success.

## Files

- Bitmap snapshot source and its package Cargo manifest.
- Bitmap diff source, sibling retained apply plan, portable retained JSON and native laws.
- Bitmap artifact root Cargo manifest.
- This report and retained Pillow oracle input.

No generated output files were persisted by this slice. Ticket lifecycle remains with the coordinating parent.

Allocator guard: scoped inspection found no existing global_allocator declaration in bitmap artifact sources. Tests import the shared ObservedAllocator dependency rather than defining another allocator. Link-time confirmation remains part of the blocked native gate.
