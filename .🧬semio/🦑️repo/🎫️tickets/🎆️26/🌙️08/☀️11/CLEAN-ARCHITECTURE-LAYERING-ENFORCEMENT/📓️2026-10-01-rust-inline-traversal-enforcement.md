# Rust Inline and Physical Traversal Enforcement

Active schema-first execution record in the existing clean-architecture ticket. This extends the physical source-input gate; it does not claim complete Rust symbol-provider analysis, complete macro expansion or whole-repository cleanliness.

## Anchored Inline Scope

`RustCompileReference.inlineBase` records the raw file-relative directory proven by an explicit source-level inline `#[path]` anchor. Nested inline scopes extend that raw base. Unanchored orphan scopes still require real graph provenance. Literal include macros continue using the physical file origin, and the anchor never grants Cargo manifest authority.

The closed portable corpus initially added six actual compiler shapes: orphan current base, nested explicit override, nested default after an explicit anchor, conventional mounted source, explicitly mounted source, and separate include origin. The original registered prepatch replay passed eleven tests and failed the new law after actual rustc accepted its first mounted-source case: missing module provenance, 231 assertions. The implementation then genuinely passed twelve tests/262 assertions in 5.77 seconds (6.7 seconds uncached Nx), and full Repo Library typecheck passed in 13.5 seconds.

The completed all-owner gate on that frontier remained RED: 24,079 files, 60,672 successfully parsed reference facts, 86 forbidden edges and 42 source problems. The nine generated-adapter inline obligations resolved. Two existing norm sources now fail closed on multiple inline path attributes, alongside the three previous unsupported macro/include source cases and 37 missing-input problems. Unsupported source expressions still suppress other parseable reference facts in their source; these counts do not claim every emitted dependency was resolved.

## Physical Prefix Traversal

Read-only review identified a real defect inherited from normalization: `missing/..` or `node_modules/..` can erase a physical component before lstat. The closed four-case corpus exercises both inline mounts and literal includes, with a missing intermediate directory and a symlink followed by parent navigation.

The real Rust compiler rejected the first missing-prefix case, while the old census returned no failures. That is a genuine producer RED; the old physical normalization accepted an input that the compiler could not read. Keeping raw inline bases, recording each required directory in traversal order and inspecting those directories before the canonical leaf then genuinely passed thirteen laws/275 assertions in 4.30 seconds (4.7 seconds uncached Nx).

The two positive symlink compiler executions independently observed the specific owner's `222` value and `specific-input` text, despite a different neutral file at the normalized lexical address. Both must be refused as linked inputs by the gate. This proves physical redirection, rather than merely asserting a source string matches an implementation. Missing-prefix compiler failures and existing legitimate parent-navigation cases retain their original assertions and 45-second law limits.

## Closed Public Metadata and Root Directory

The next closed corpus adds eight invalid/contradictory inline-reference shapes and one valid root `#[path="."]` shape. An AJV strictRequired declaration error was corrected as a schema harness defect; it is not reported as a producer failure. The corrected genuine RED was twelve passing laws, two failures and 279 assertions: a manufactured include-kind inline base was accepted, and a valid root directory target was refused.

`RustSourceTarget.directories` is now mandatory, and all actual canonical-input callers explicitly supply their complete known directory list. There is no optional legacy shape or fallback that silently omits traversal. Runtime and schema validate that inline bases belong to scoped path facts, are portable relative spellings, cannot carry manifest/generated authority, and agree with supplied graph contexts. The valid workspace root directory receives a physical lstat check. The actual registered replay passed fourteen laws/297 assertions in 5.3 seconds uncached Nx. Its first typecheck found a test-only readonly/union TS2769 mismatch; correcting the actual corpus type and spreading the readonly expected failures then genuinely passed the complete two-program library typecheck in 11.4 seconds.

## Linear Physical Inventory and Current Gate

The all-owner physical traversal initially completed in 5 minutes 4 seconds with 24,081 files, 60,680 successfully parsed references, 86 forbidden edges and six census problems. It failed on actual findings, not a timeout. All 36 drawing fixture locator problems had disappeared; the remaining six were five unsupported source expressions, including two redundant Norm inline attributes, and one missing Hub actor fixture.

The canonical inventory interface is now a first-party `ReadonlyMap` of physical node kinds. Actual input checks reuse that inventory without repeatedly allocating whole-map arrays. Target/edge deduplication uses a stable linear Set pass with one serialization per fact. There is no old-array compatibility overload. The final actual registered corpus passed fourteen laws/297 assertions in 5.67 seconds (Nx 6.3 seconds), and the complete two-program typecheck passed in 15.5 seconds. Original law budgets and assertions remain intact.

The actual optimized complete gate finished in 1 minute 57 seconds with 24,082 files, 60,686 successfully parsed references, the same 86 forbidden edges and six census problems. Concurrent authored additions changed the source census by one file/six facts; no exact-cohort benchmark claim is made. The two identical duplicate Norm `#[path="."]` attributes were subsequently corrected at their authored owners; one properly indented identical attribute remains at each original inline module. The actual scanner receipt saw 189 and 243 references in these sources. The fresh complete gate then genuinely completed in 1 minute 12 seconds with 24,082 files, 61,123 successfully parsed references, 86 forbidden edges and four problems. The two Norm unsupported-source problems disappeared; three distinct template-source problems and the missing Hub actor fixture remain. Original artifacts, fixtures and native assertions were not changed.

## Owned Implementation and Evidence

- Repo Library `🔍️discovery/🟦️.ts`, the existing owned Rust syntax scanner.
- Repo Library `🕸️dependencies/🧭️direction/🦀️source/🟦️.ts`, typed input resolution and direction edges.
- Repo Library `🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts`, physical input inspection.
- Existing Rust-source-direction test, JSON schema and JSON fixture owner.

All executable proof uses the existing registered `@semio-tech/repo-lib:test-rust-source-direction` and `typecheck` routes with Bun/Nx and ticket-owned output. No new script file, external runtime parser, test exemption or generated catalog patch was introduced. The separate inline and traversal read-only reviews document remaining bootstrap/source-root authority and lexical-provider limits.
