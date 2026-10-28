# Retained Paragraph Regions

The framework paragraph owner accepts a sequence of line-box regions while retaining one complete original UTF-8 story analysis, Unicode line-break result, logical metrics and bidi analysis. A region pause reports the next original byte offset, exclusive planned line ordinal and region ordinal. The caller supplies another region or explicitly finishes the placed prefix as overset. Outline publication uses each line’s retained region origin, width and local row. Changed source ownership is refused by the native owner. Cancellation and retirement retain the existing original work authority.

## Contract

`FontParagraphJob.supplyRegion({width,height,origin})`, `flowPause()`, `finishFlow()` and `linePlacement(index)` have native `FontParagraphCursor` snake-case counterparts. Regions are supplied before the first work or at the `region` pause. `sourceEnd` includes consumed CRLF and other hard separators. Empty and fully consumed flow stories do not create another trailing empty line. A region shorter than one line box pauses without consuming text. Output glyph `line` indexes the retained line placement, which includes `region` for the Layout frame mapping.

## Actual Verification

Five language-neutral fixtures exercise three columns, different next widths, original multibyte UTF-8 and CRLF offsets, a region too short for a line, and explicit overset prefix completion. Across grants 1, 7 and 4096, the actual TypeScript owner passed 15 region runs and 15 positioned outline pixel comparisons against independent OpenType.js and Sharp. Five paused cancellation paths passed. The original eight paragraph cases still pass 24 placements/pixel comparisons and 40 cancellation paths. Strict TypeScript checking covered 332 sources with zero diagnostics. Both region and original suites are mounted in the permanent Bun test entry. The region schema and fixture are checked in beside both implementations.

Native region owner and mounted fixture tests parse without grammar errors. Native compilation and execution remain unverified because the authorized repo native runner is unavailable; this is not a native test pass. The new owner has not yet been connected to the production Layout linked-frame scene and export paths. That integration must feed full-story regions rather than creating a new paragraph analysis for each tail substring.
