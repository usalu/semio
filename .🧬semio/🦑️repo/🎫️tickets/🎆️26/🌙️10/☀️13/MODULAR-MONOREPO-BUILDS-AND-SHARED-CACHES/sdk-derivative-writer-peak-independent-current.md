# SDK Derivative Writer Peak Independent Audit

Source-only review on 2026-10-15. No runtime, tests, image opens, or production changes.

## Formula and Authentication

`sdk/🎟️capacity/🧮️formula/🟦️.ts:12` derives derivative publication bytes from authored producer ceilings: one snapshot (24,117,248), two programs (16,777,216), installed inputs (16,777,216), sealed index (16,777,216), replacement output (33,554,432), and receipts (65,536), totaling 108,068,864. It retains the same copied extents plus per-entry allocation/metadata charge and 64 publication entries. Full-clone publication remains 134,217,728 bytes. This is a writer-specific closure rather than a caller-selected observed-free-space override.

`sdk/🎟️capacity/📥️physical/🟦️.ts:27` selects that formula only inside the private issuer factory using its internal derivative kind. Line 35 authenticates the projection through its private state before passing captured entry/extent facts. Public copied/serialized quotes cannot choose the scalar or kind because physical admission resolves the private issued map. No numeric admission override identified in this inspected path.

## Simultaneous Writers

`tools/📜️script.ts:165-177` creates installed inputs, copied Nx files, patched replacements, a captured Source snapshot, a compiled compiler program, the final authorizer program, and binary/tooling receipts. The original copied-file extent charge remains present alongside the full replacement allowance, so replacing a smaller original with a larger patched output does not rely on reclaiming original blocks. The captured snapshot and compiler program coexist with the final program, matching one snapshot and two outputs. `authorizers/📜️script.ts:20` writes final output through one temporary file then renames it; no extra simultaneous final-output copy is introduced by that rename. `authorizers/📦️snapshot/🟦️.ts:26` enforces each compiled output ceiling. The index seal's temporary-file rename likewise keeps one index content copy. Namespace rounding and overhead retain the common per-entry allowance.

Installed input accounting is separate from generated receipts. `tools/📜️script.ts:159` bounds the aggregate installed input buffers before writing. Their files already include authorizer manifests and derivative material; binary/tooling receipt content requires its separate ceiling. Input allowance alone does not establish receipt closure.

## Producer Coupling Re-Review

The producer guards are now visible. `tools/📜️script.ts:169-171` computes all actual patch postimages, counts every batch with `requireNxSdkDerivativePatchImagesV1`, and refuses above the same authored 32 MiB policy ceiling before the first unlink/write. Counting every batch is conservative when a file occurs in multiple patches. The helper in `sdk/📥️source/🔗️derivative/🩹️patch/🟦️.ts:5` reads capacity policy directly; it does not introduce a separate scalar.

Tools line 173 bounds the final serialized snapshot Buffer including newline and the compiled compiler output before writing either. The final authorizer output remains bounded by the compiler helper. Tools line 168 bounds binary receipt bytes before its write; line 178 bounds binary plus tooling receipt content together against the same authored 64 KiB ceiling before the tooling write. Thus a rejected aggregate does not first materialize oversized tooling receipt content. These three producer coupling gaps are closed by Source inspection, without runtime qualification.

## Child Environment Re-Review

The new `sdk/📥️source/🔗️derivative/🎛️environment/🟦️.ts:4-6` refuses nonempty inherited `BUN_OPTIONS`, `NODE_OPTIONS`, and `NODE_V8_COVERAGE`, then overrides runtime cache controls with schema-owned `BUN_RUNTIME_TRANSPILER_CACHE_PATH="0"` and `NODE_DISABLE_COMPILE_CACHE="1"`. Caller values cannot override those fields because the schema fields are spread last and the result is frozen. Tools script line 175 applies it before starting the exact `nx:authorizers` compiler child. `authorizers/📥️loading/🟦️.ts:14` applies it before every Node preflight capture. The previously identified inherited options/cache path for these child roles is closed by Source inspection. No new actionable defect identified in these exact callsites.

Parent Bun startup remains a separate limitation: replacing environment variables inside TypeScript after process startup cannot retrospectively constrain startup-selected caches, preloads, or startup options. A child guard only qualifies child launches. The parent's Bun.build intermediate behavior and runtime temporary-byte peak are not proven bounded by this file-output scalar. Store-provided TMPDIR routes supported children to tools backing, but a directory selection alone is not a universal byte bound. No external write or temporary peak was observed or runtime-probed here. Nx's focused runtime was active elsewhere and was not touched; this review does not authorize or report physical SDK publication success.

No additional concrete simultaneous writer omission established in the inspected publication closure. This does not qualify the whole repository's writer set. The prior physical refusal remains a refusal, and no successful artifact or runtime adoption is claimed. Publication/issuer callback failure-drain laws remain unqualified separately.
