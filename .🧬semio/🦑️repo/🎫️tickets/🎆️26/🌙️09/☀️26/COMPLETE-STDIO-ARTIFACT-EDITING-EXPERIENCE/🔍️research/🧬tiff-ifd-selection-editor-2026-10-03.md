# TIFF IFD Selection Editor

Date: 2026-10-03

## Decision

TIFF page selection is ephemeral editor configuration, separate from canonical TIFF artifact history. `TiffEditorConfig.selected_ifd` owns the local selection. Its schema-owned `SetSelectedIfd` mutation has text and binary forms, an exact inverse and a Config-only publication contract.

The `select-ifd` action is a retained bounded-first-step job. It validates the requested IFD against the canonical snapshot before publishing one config mutation. The synchronous editor handler refuses that command so the UI cannot bypass the retained job, revision and publication authority.

## End-user surface

The main editor renders native previous/next buttons and a localized page status inside the shared image window body. English labels are `Previous image page`, `Next image page` and `Image page n of m`; German labels are `Vorherige Bildseite`, `Nächste Bildseite` and `Bildseite n von m`. Button labels provide the native accessibility names.

The selected IFD drives both the derived PNG preview and the existing retained region-paint command. Artifact undo changes TIFF bytes and leaves local selection intact. A fresh editor config selects IFD 0. Loading another example emits a selection reset to 0. If a publication removes the selected IFD, rendering clamps safely to IFD 0 without mutating the document.

The shared `ImageWindowKit` gained additive accessory render methods. The existing image and unavailable render APIs retain their prior behavior. TIFF uses the accessory seam so controls are materialized inside the image body rather than grafted onto a completed tree.

## Neutral fixture and independent oracle

`🧬️ifd-selection/🔣️.json` describes two one-pixel RGB pages and selects page two. Its sibling JSON Schema fixes the language-neutral address and expected RGBA output.

The editor law renders that fixture through the production TIFF projection, locates the selected image node, decodes the actual data URI and asks image-rs to reopen the authored PNG. The resulting RGBA sample must match the fixture. Companion laws verify English/German controls, exact action arguments, stale-selection clamp, reopen default, config codecs/inverse and retained Config-only publication.

## Preservation and limits

Selection never rewrites the canonical TIFF snapshot. Preview remains an explicit derivative. Painting continues to use the canonical revision guard and edits only the selected IFD's admitted raw sample storage, preserving every unaddressed page, tag, chunk and padding byte.

This cut does not add editing support for TIFF profiles already refused by the checked projection or paint operation. Multi-page preview is available when each selected IFD is in the admitted projection matrix; a refused selected page remains mounted with the localized unavailable state and the same page controls.

## Validation status

The first three full native runs are genuine red runs. Runs 1 and 2 repaired local path, dependency, operation and UI-value compilation errors. Run 3 compiled and exposed the missing interactive-job classification before TIFF assertions.

Run 4 contains the completed retained classification/factory cut but stopped in concurrent shared plugin compilation before TIFF compiled: stale typed-fault consumers, stale bounded-fault fields and an unhandled new `MediaWireFormat::Intrinsic` variant. Those shared sites were coherent before the next run.

Run 5 is the fresh registered green: Nextest ran 116 tests in one binary; 116 passed, zero failed and zero skipped. It includes component app assembly and all five new page-selection laws. The independent image-rs check decodes the actual PNG data URI produced by the selected-page renderer. The package TypeScript check also passed all four suites.

Receipts:

- `../🗑️generated/tiff-ifd-selection-native-1.log`
- `../🗑️generated/tiff-ifd-selection-native-2.log`
- `../🗑️generated/tiff-ifd-selection-native-3.log`
- `../🗑️generated/tiff-ifd-selection-native-4.log`
- `../🗑️generated/tiff-ifd-selection-native-5.log`
- `../🗑️generated/tiff-ifd-selection-typescript-check.log`

The TIFF page-selection checkpoint is green. The ticket remains open. Root owns whole-Stdio browser acceptance.
