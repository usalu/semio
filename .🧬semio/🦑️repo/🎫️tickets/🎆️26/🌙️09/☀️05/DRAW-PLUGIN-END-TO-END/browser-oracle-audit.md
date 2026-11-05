# Browser PDF Oracle Source Audit

Read-only inspection of current neutral cases/schema, check runner, raster helper, package script and target. No tests/builds/browser/source edits.

## Concrete Findings

- Neutral coverage is not protected against fixture drift. Schema accepts arbitrary duplicate case names and arbitrary relation assignments; runner only asserts passed=5. Five copies of the same-PDF case can therefore replace edited/empty/malformed coverage and still report five passed. Require exact unique case identities with expected relation coverage, and relation-specific required inputs. Runtime notified.
- Metadata-only vector currently changes PDF Info Title A to B while retaining identical rectangle content. Add an explicit unequal source-byte assertion for that case so its intended negative input cannot silently become identical bytes after future fixture/generator edits. Current source does generate unequal metadata bytes; this is a missing self-check, not evidence of a currently false result.

## Current Meaningful Proof By Inspection

The actual authored five vectors cover identical geometry/title, metadata-only title difference, additional nonoverlapping rectangle, empty page, and malformed bytes. Valid pairs both undergo independent PDF.js parsing, expected page-count and painting-operator checks, native canvas rendering, positive dimensions and visible nonwhite pixel checks. Same/metadata-only cases require equal pixel digests and explicit assertPdfDifference refusal; edited rectangle requires positive changed pixel count. Empty/malformed vectors require assertPdfPainting rejection. This is real rendered-content comparison, not PDF byte inequality. PDF generator is handcrafted fixture input; PDF.js and napi canvas are test-only independent parser/raster implementations behind the oracle interface.

Declared owning route is repo run @semio-tech/draw-js:browser-oracle-check, dispatching browser-acceptance oracle to the check.mjs child under issued resource control. Target is noncached; default/dependency inputs are declared. Current default globs include mjs/json, thus oracle helper/check/cases/schema are covered, with dist/cache/target/node_modules/generated exclusions. No browser dependency is needed by the standalone PDF-only check. Source command inspection does not establish actual admission or success.

Remaining proof: current declared oracle run must actually pass all required unique vectors; both localized real browser journeys must validate current Draw export bytes and fresh-folder reload. No original application gate is proven by this audit.
