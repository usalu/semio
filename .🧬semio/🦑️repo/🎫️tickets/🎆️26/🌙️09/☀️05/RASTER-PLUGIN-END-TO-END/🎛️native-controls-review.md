# Native Controls Integration Review

Read-only current-source review; no builds or tests executed.

New required config fields filterKind, filterAmount and imageGrid appear in TypeScript interface/parser, JSON Schema required list, GraphQL, Proto and Rust schema leaf. All three Rust fields have state(config). Runtime defaults and diff merge/apply/inverse carry them. Current explicit config DSL test literal and TypeScript fixture constructor include imageGrid; the earlier pending omission is now repaired. Focused JSON fixture scan found no concrete numeric brushSize/camera config object missing these fields in the Raster plugin.

Config filter amounts use existing per-filter vocabulary/ranges/integer laws consistently between JSON conditional constraints and TS parseApplyFilter/native invariant. Image-grid schema/parser/native valid agree on x/y 0–16384 and width/height 1–16384. They represent staged settings; they do not independently enforce output total-pixel or source crop bounds. Actual transform command must retain that final admission.

Action bridge setFilterAmount uses number_input to parse finite native numeric strings before typed decoding. setImageGrid uses patch_args to stringify native number/bool values, matching its String payload and u32/bool parsers. Native number controls have matching range/step and blur commit. No concrete numeric-transport defect found. Additional bridge regression coverage currently explicitly exercises string filter amount; numeric/bool image-grid transport should also receive authored bridge assertions.

Current census expectations have been raised to 33 total commands, 32 retained routes/contracts and 33 factory proofs; every_command includes all three added commands. Leaf config mutation descriptor schema files for filter-options and image-grid exist. Inspector operations source is still being authored by its owning agent; a completed geometry-control workflow was not inferred from partial source.

No Paint2dScene filter/grid carrier exists in the shared UI scene module at this checkpoint. The new native options are Inspector/session config, whereas React overlay currently retains filter, amount and resize dimensions locally. Thus native operations can dispatch authoritative edits, but cross-host shared option parity is not established. This scope limit was sent directly to native_controls. Its agent owns the final decision/wiring.

Existing generic parser mismatch remains: parseRasterConfig accepts unrecognized root properties while JSON config additionalProperties=false; this predates these fields. No new blocking defect was identified by this review. Runtime compilation, census execution, actual controls and publication remain unverified.
