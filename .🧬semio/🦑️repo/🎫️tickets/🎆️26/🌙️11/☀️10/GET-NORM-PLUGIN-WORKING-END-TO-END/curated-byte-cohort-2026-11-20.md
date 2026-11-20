# Curated example bytes in the joint component cohort

Read-only observation at approximately 01:32 CET, 20 November. The `wOltbvh` joint Cargo build remains active, using lane `p9171e3`; it has not yet emitted the EN 1990, EN 1995, EN 1997, EN 1998 or DIN V 18599 examples or editor rlibs. Their corresponding output directories exist but contain no `.rlib`. Thus no exact embedded-byte conclusion about those libraries or the final component is available yet. The emitted EN 1995, EN 1997, EN 1998 and DIN V 18599 `io-text` rlibs do not contain the complete current example files as byte substrings; this is not evidence that the component has stale examples. The EN 1990 `io-text` rlib was not yet present.

The relevant current asset modification times, all CET on 20 November, are:

| Family and affected files | Current asset time | Relationship to `wOltbvh` run creation (00:12) |
| --- | --- | --- |
| EN 1990 road-bridge showcase | 00:54:11 | after |
| EN 1997 demo, compliant, noncompliant | 00:49:05 | after |
| EN 1995 four examples | 00:52:35 | after |
| DIN V 18599 five examples | 01:01:09 | after |
| EN 1998 five examples | 00:06:25 | before |

The repaired EN 1997 files use `direct-shear`; DIN V 18599 has explicit zone thermal-mass columns; EN 1995 uses `eq610`; the EN 1990 showcase supplies corrected empty/default fields. The EN 1998 repair predates run creation. Asset timestamps alone establish only that these files changed after or before run creation, not when `include_str!` consumed them. Do not assume a stale or fresh result from those timestamps or from empty output directories. The run's earlier graph/source-admission state may still matter; compare the exact emitted example/editor rlibs against current files and check component receipts after compilation, then rebuild only affected components if bytes differ. EN 1997 is in `norm-eurocode`, EN 1995 in `norm-eurocode-materials`, and DIN V 18599 in `norm-energy`.

The separate scratch EN 1997 proof documented in `component-warm-rebuild-audit.md` verified fresh bytes in its isolated rebuilt library, not in the `wOltbvh` component. No compiler or build was started for this observation.

## Emitted example libraries, 01:38 CET

The five example libraries subsequently appeared in `p9171e3`. A direct search for each complete current source byte sequence in its family rlib found DIN V 18599 5/5, EN 1995 4/4, EN 1997 demo/compliant/noncompliant 3/3, EN 1998 picker examples 4/4, and EN 1990 seven examples 7/7. No selected example in these libraries is stale. EN 1990 `default-office` is fixture-only. The EN 1990 road-bridge, EN 1997 ground and EN 1998 seismic showcase assets are included by their editor crates instead and are pending emission. An exact match establishes the bytes in the rlib, not successful final publication or runtime behavior.

## Emitted editor libraries, 01:42 CET

The EN 1990, EN 1997 and EN 1998 editor rlibs have now appeared. Each contains the complete current bytes of its respective road-bridge, ground and seismic showcase asset. Together with the example-library comparisons above, no stale selected example was found in this compiler cohort. This evidence does not establish that the final components have linked or been published.
