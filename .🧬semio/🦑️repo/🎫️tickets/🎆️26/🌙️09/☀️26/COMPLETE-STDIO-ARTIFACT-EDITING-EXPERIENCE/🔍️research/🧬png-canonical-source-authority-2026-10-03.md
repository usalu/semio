# PNG Canonical Source Authority

Date: 2026-10-03

## Authority decision

`PngSnapshot` now persists only the schema identifier and the complete PNG source byte sequence. Source bytes are required at the schema and Rust value boundaries. The previous parallel semantic snapshot fields are no longer part of the mounted snapshot or mutation aggregate. Checked chunk addresses, IHDR/profile information, typed ancillary values, exact decoded samples and RGBA8 preview pixels are projections.

A no-op import/export returns the same byte sequence. This retains source bit depth, packed palette indices, duplicate palette entries, 16-bit low bytes, filter choices, Adam7 layout, multiple IDAT boundaries, private chunks, CRCs and chunk order. The SQLite representation has one singleton document row and stores the source as one intrinsic BLOB, so it cannot become another semantic authority.

## Checked projections

The source parser checks the PNG signature, chunk framing, CRCs, IHDR profile combinations and the complete chunk chain. The layout exposes ordered, byte-addressed chunks. The semantic projection reads the admitted PNG 1.2 profiles, including 1/2/4/8/16-bit samples, indexed transparency, Adam7 and typed ancillary chunks. `png_preview` is an explicit derived RGBA8 PNG and never replaces source authority.

The editor and viewer request the bounded derivative. Invalid or unsupported preview work renders the shared localized `ImageWindowKit::render_unavailable` state while leaving the artifact mounted and its exact source available.

## Mutations and editing

The mounted aggregate contains exact `SetSnapshot` and `PatchSnapshot` operations plus two stable addressed semantic operations:

- `ChangeGamma` revision-checks the immutable byte root and inserts, replaces or removes the gAMA chunk while retaining every unrelated pre-existing chunk payload and order.
- `PatchPixels` revision-checks the byte root and paints a bounded region only for non-interlaced 8-bit RGBA sources, where the edit has an exact profile-preserving interpretation. Other profiles are refused without changing their bytes.

Every operation has an exact inverse. The pixel-region command uses the shared retained raster planner, publishes one mutation after completion, exposes progress, supports cancellation and refuses equal-length stale roots. The registered command laws retain 128-patch admission and DCI 4K bounded-row planning.

This cut deliberately does not claim general sample editing for packed, indexed, 16-bit or Adam7 sources. Those profiles remain exactly readable, previewable and saveable. A future natural command must edit their native sample representation or perform an explicit reversible profile conversion.

## Neutral fixtures and independent authorities

Five hand-authored source fixtures cover 1-bit grayscale, 2-bit indexed data with duplicate palette values and indexed transparency/background, 16-bit grayscale precision, Adam7 RGBA and a multi-IDAT RGBA source with a private chunk. The native laws prove exact no-op source preservation, checked projections, metadata edit preservation, refusal without source damage and exact undo.

Independent `pngjs` decoding verifies preview pixels and profile metadata in the registered native suite. The source SQLite suite independently reopens the neutral corpus with `pngjs`, checks exact BLOB bytes, exercises Latin-1, zTXt and iTXt source retention, and uses Node zlib for window and dictionary corpus laws. The existing TypeScript region fixture also compares the two current region edits with `pngjs` bitblt results.

The older standalone feature-oracle route did not reach the PNG crate because the repository-wide oracle contract rejected 3,816 unrelated catalog entries first. Its failed receipt is retained as evidence. The independent pngjs/zlib laws above run inside the registered PNG targets and are green.

## Direct consumers

Semio image import derives its resolved RGBA8 frame and source profile from the checked PNG projection. Semio image and drawing export explicitly author a new PNG from their semantic frame, then mount the returned exact bytes. Mesh export tests use the same explicit conversion. These are declared format conversions; they do not pretend to preserve a source PNG that does not exist on the Semio side.

## Remaining boundary

The source tree still contains physically present, unmounted files from the former broad semantic mutation vocabulary. They are not variants of `PngMutation`, are not registered editor actions and are ignored by the exact-source fixture laws after source bytes became required. They were left untouched because the shared checkout shows concurrent staged work in those paths. Removing or redesigning those files needs an ownership-confirmed cleanup rather than deleting another executor's staged changes.

The full Stdio wasm/browser integration remains root-owned. This report claims the PNG package and direct source boundaries validated below, not the root browser receipt.

## Terra audit structural repair

The follow-up audit identified two source-boundary defects. Missing gAMA used the generic ancillary insertion point immediately before IDAT, which placed it after PLTE in an indexed PNG. The parser also checked framing and CRCs without enforcing the complete PNG chunk grammar before exposing projections.

The mounted repair gives gAMA a controlled authoring path that inserts before the first PLTE or IDAT, rejects the forbidden zero value, retains in-place replacement and exact deletion, and validates the completed byte sequence. The checked projection boundary now validates IHDR/IEND uniqueness and position, consecutive IDAT, modeled singleton chunks, chunk type bits, and the placement/cardinality rules for PLTE, tRNS, bKGD, gAMA, cHRM, sRGB and pHYs against the active color profile. It also refuses trailing bytes after IEND, invalid compressed-text methods, non-ASCII iTXt language tags and invalid UTF-8 rather than projecting lossy strings.

The language-neutral audit fixture now declares the indexed gamma edit, rejected zero, expected chunk order, modeled singleton set and explicit malformed structure/profile cases. Native laws build each case from that fixture, execute the real mutation and inverse, compare every unrelated source chunk byte-for-byte, and reopen the edited indexed PNG with `pngjs`. The shipped demo PNG, DSL and pack payloads were reordered to `IHDR,gAMA,cHRM,sRGB,PLTE,...` while preserving every chunk byte.

Fresh validation is recorded in `../report/validation.md`: native 44/44, SQLite/source 8/8 with 50 assertions, TypeScript pngjs region laws 2/2, and the TypeScript package check. The prior 42/42 receipt remains an accurate earlier checkpoint and is not relabeled as the current run.
