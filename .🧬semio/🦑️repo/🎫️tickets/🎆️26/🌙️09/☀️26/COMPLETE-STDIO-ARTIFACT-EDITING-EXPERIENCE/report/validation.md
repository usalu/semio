# BMP Checkpoint Validation

Date: 2026-10-03

## Registered native suite

Command:

```text
bun nx run @semio-tech/stdio-bmp-rs:test
```

Result: passed. Nextest ran 42 tests in one binary; 42 passed, 0 failed, 0 skipped. The registered pre-native twins also passed 14 source-hex checks and four paint-region checks. `component-app-assembly` is registered in the BMP package router and was active for this run.

Receipt: `../🗑️generated/bmp-canonical-native-9.log`

This proves the byte-authoritative profile laws, exact no-op source preservation, exact mutation/history outputs, stale/cancel behavior, SQLite carrier laws, editor/component assembly, first-party PNG pixel projection and localized mounted unavailable state covered by those tests.

## Independent image-rs oracle

Command:

```text
bun nx run @semio-tech/stdio-bmp-rs:test-oracle
```

Result: passed. Four tests passed, zero failed: every admitted canonical profile reopened, exact direct/indexed paint results reopened, the neutral opaque RGBA8→Direct RGB24 fixture reopened as `[10,20,30,255]`, and the supported 8-bit raw indexed plane preserved duplicate-index selection.

Receipt: `../🗑️generated/bmp-image-rs-oracle-3.log`

The preceding `bmp-image-rs-oracle-2.log` is a genuine failed run. It exposed an image-rs 0.25.10 panic for raw 1/4-bit indexed reads: the decoder allocates a packed row buffer and then copies `width` unpacked bytes from it. The final oracle limits raw-index inspection to its supported 8-bit path. First-party exact-byte laws retain 1/4/8-bit index-identity coverage.

## Neutral alpha policy

Fixture: `../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/rgba8-direct-rgb24.json`

The native suite checks alpha 255 against the exact hand-authored `rgba8-opaque-direct-rgb24.bmp` output and checks alpha 0 and 128 as refusals. The independent oracle reopens the accepted bytes.

## Full Stdio consumer/browser status

The BMP Semio import/export leaves and BMP viewer/editor were source-coherent at this checkpoint. Root owns the full Stdio wasm/browser run after the source freeze; preview30 is the authoritative integration receipt. No preview30 pass is claimed in this report until root records it.

## Accepted limits

This validation accepts the current canonical BMP checkpoint. It does not prove the remaining natural header, DPI, palette and profile-conversion controls, and it does not validate the deferred PNG authority migration.

# PNG Canonical Source Checkpoint Validation

Date: 2026-10-03

## Registered native suite

Command:

```text
NX_FORCE_REUSE_CACHED_GRAPH=true CARGO_TARGET_DIR=<ticket>/🗑️generated/png-native-target SEMIO_TEST_BUDGET_MS=3600000 bun nx run @semio-tech/stdio-png-rs:test --skip-nx-cache
```

Result: passed. Nextest ran 44 tests in one binary; 44 passed, 0 failed and 0 skipped. The registered target enables `component-app-assembly`.

Receipt: `../🗑️generated/png-structure-native-6.log`.

The suite proves exact source import/no-op export for five neutral fixtures, checked packed/indexed/16-bit/Adam7 projections, structural and profile-aware chunk validation, chunk addressing, legal indexed gAMA insertion, nonzero gAMA enforcement, exact inverses, unrelated-chunk preservation, revision guards, stable-profile paint refusal, derived preview/source separation, pngjs preview and edited-gamma agreement, editor/viewer assembly, history edit acceptance, DCI 4K bounded planning, 128-patch admission, cancellation without publication, stale-root refusal and native SQLite ownership/preflight laws.

`png-canonical-native-10.log` remains the genuine earlier 42/42 canonical-source checkpoint. The audit repair added two neutral laws, so the current complete count is 44. `png-structure-native-3.log` first exposed the shipped demo's invalid gAMA/PLTE order. `png-structure-native-4.log` and `png-structure-native-5.log` then exposed the DSL envelope and fixture-print newline mismatches introduced while repairing the three demo forms. Those fixture defects were corrected before the fresh green receipt.

`png-canonical-native-9.log` is the genuine preceding failed run. It exposed a missing required-byte invariant in legacy fixture decoding and a DCI law that expected a four-byte patch even though the shared planner correctly owns one bounded complete row. Both laws were corrected before the green receipt.

## SQLite source and independent oracle suite

Command:

```text
NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/stdio-png-rs:test-snapshot-sqlite-source --skip-nx-cache
```

Result: passed. Bun ran 8 tests with 50 assertions; 8 passed and 0 failed.

Receipt: `../🗑️generated/png-structure-sqlite-source.log`.

The suite proves the one-table singleton BLOB representation, byte-exact reconstruction, byte-domain rejection, malformed singleton rejection and bounded cancellation in both directions. Independent pngjs laws reopen every neutral audit profile before and after SQLite. Node zlib independently checks window/distance and dictionary identity. Latin-1, zTXt and iTXt chunk payloads remain exact.

`png-sqlite-source-1.log` is the genuine preceding failed run. It showed that the test corpus still imported deleted semantic snapshot guards and asserted the removed sixteen-table authority. The final source laws test the mounted exact-byte representation.

## TypeScript editor fixture and check

Commands:

```text
NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/stdio-png:test --skip-nx-cache
NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/stdio-png:check --skip-nx-cache
```

Results: the region suite passed 2/2 pngjs bitblt comparisons, and the package check completed successfully.

Receipts: `../🗑️generated/png-structure-ts-test.log` and `../🗑️generated/png-structure-ts-check.log`.

## Standalone feature-oracle limitation

The attempted repository feature-oracle target did not reach PNG compilation. Its global oracle contract stopped first on 3,816 unrelated catalog breaches across other artifacts. No pass is claimed for that route.

Receipt: `../🗑️generated/png-canonical-oracle-1.log`.

The registered native and source targets still execute third-party pngjs and Node zlib authorities directly and are green.

## Full Stdio consumer/browser status

The PNG package and direct Semio conversion source are coherent at this checkpoint. Root owns the full Stdio wasm/browser integration run. No browser pass is claimed here until root records its receipt.

# TIFF Tiled 8-Bit Validation

Date: 2026-10-03

## Focused red and green sequence

The focused native sequence first exposed invalid mutation-descriptor enum values and corrected them against the repository schema. The final receipt `../🗑️generated/tiff-tiled-core-green-6.log` ran nine selected laws and passed 9/9 with 102 unrelated laws skipped. It covers the neutral 17×17 four-tile fixture, exact uncompressed and PackBits projections, image-rs save/reopen checks, WhiteIsZero and unassociated-alpha samples, revision/inverse/cancel behavior, bilingual action metadata, and real viewer assembly.

## Full registered native suite

Command:

```text
NX_FORCE_REUSE_CACHED_GRAPH=true CARGO_TARGET_DIR=<ticket>/🗑️generated/tiff-tiled-native-target SEMIO_TEST_BUDGET_MS=3600000 bun nx run @semio-tech/stdio-tiff-rs:test-quick --skip-nx-cache
```

The first full receipt, `../🗑️generated/tiff-tiled-full-native-1.log`, is a genuine red run: 93 laws passed before the history input resolver rejected the new payload because its `ifdIndex` property lacked localized UI metadata. Every payload property then received explicit English/German labels, groups and deterministic order.

The final receipt, `../🗑️generated/tiff-tiled-full-native-2.log`, passed 111/111 Nextest laws with zero skipped. The target includes `component-app-assembly`; this validates production codec/schema/editor/viewer compilation, action input resolution, retained command wiring, cancellation, exact inverse, the language-neutral fixture and the independent image-rs oracle laws.

## TypeScript and schema surface

Command:

```text
NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/stdio-tiff:check --skip-nx-cache
```

Receipt `../🗑️generated/tiff-tiled-typescript-check-2.log` passed all four package suites after final JSON Schema and text/binary public facet alignment. The earlier `tiff-tiled-typescript-check.log` also passed before the final Kaitai/ABNF identity cleanup and is retained as an intermediate receipt.

## Integration boundary

No browser or whole-Hub pass is claimed by this TIFF lane. Root owns the full Stdio wasm/browser gate. The checkpoint does not claim page-selection UI or editing for refused profiles.

# TIFF IFD Selection Validation

Date: 2026-10-03

## Current full native attempt

Command:

```text
NX_FORCE_REUSE_CACHED_GRAPH=true CARGO_TARGET_DIR=<ticket>/🗑️generated/tiff-ifd-selection-target SEMIO_TEST_BUDGET_MS=3600000 bun nx run @semio-tech/stdio-tiff-rs:test --skip-nx-cache
```

Receipt `../🗑️generated/tiff-ifd-selection-native-4.log` compiled through the current retained selection source but stopped before TIFF compilation on seven concurrent shared-plugin diagnostics. The diagnostic set covered typed operation fault return-shape drift, bounded fault field drift and the new `MediaWireFormat::Intrinsic` variant. Current source already contains the typed/bounded fault repairs; active shared owners are settling the natural-media match.

No page-selection test pass is claimed from this run. The preceding `tiff-ifd-selection-native-1.log`, `-2.log` and `-3.log` are genuine red runs retained as evidence of local compile and interactive-job classification repair.

## Required fresh receipt

A fresh registered full native run must execute the complete TIFF suite after shared-source coherence. Its required laws include:

- migrated retained `select-ifd` factory, proof and Config-only publication;
- neutral two-page fixture;
- English/German native button accessibility;
- selected preview data URI reopened by image-rs;
- selected IFD paint target and exact inverse;
- config text/binary roundtrip and exact inverse;
- stale publication clamp and reopen/example reset;
- all previous TIFF canonical, SQLite, history-edit and cancellation laws.

## Fresh green receipt

The required fresh run completed after the shared source settled.

Receipt `../🗑️generated/tiff-ifd-selection-native-5.log`: Nextest ran 116 tests in one binary; 116 passed, zero failed and zero skipped. The target includes `component-app-assembly`. The suite proves the retained Config-only route, exact selection mutation inverse, neutral two-page fixture, English/German native controls, selected preview, image-rs reopening of the actual rendered PNG, selected-IFD paint, artifact undo without config loss, stale selection clamp and reopen/example reset together with all prior TIFF canonical/SQLite/history/cancellation laws.

Receipt `../🗑️generated/tiff-ifd-selection-typescript-check.log`: `@semio-tech/stdio-tiff:check` passed all four package suites after the final config/action facets were mounted.
