# IFC4 Ordered Diff Review

Reviewed the new Rust entity removal record, exact ordered absorb transport, inverse, central validation, text/binary removed rows, and TypeScript sparse diff implementation. The position transport correctly distinguishes inserted rows from original survivors and preserves remove/reinsert identity replacement semantics. No algebra defect was found in this source review.

## Actionable Findings

1. The TypeScript inverse assigns `restored.complex = structuredClone(entity.complex)`. Canonical IFC JSON omits an empty complex list (`skip_serializing_if` in Rust; optional property in the JSON schema). A simple entity decoded directly from such JSON has no `complex` property, so undoing a complex edit restores `undefined`; `applyEntity` then leaves the edited constituents in place because it treats undefined as an absent diff. Restore `entity.complex ?? []` and align the TypeScript persisted interface or normalize missing constituents at the decoder. Include a JSON-decoded simple-instance regression.

2. TypeScript `applyIfcDiff` checks duplicate base entity IDs before checking whether the diff edits entities. Rust validates duplicate IDs only inside an entities diff. Empty/header-only diffs therefore differ across implementations when passed a duplicate-ID base. Align validation scope or establish a common global base invariant.

## Tool Loading

Use Node `createRequire` rooted at the repository package: `var {createRequire}=await import("node:module"); var taskRequire=createRequire("C:/git/semio/package.json"); var ts=taskRequire("typescript");`. This works for CommonJS packages where direct dynamic imports exposed an empty tool namespace.

## Reproduction

Transpiled the production TypeScript diff source with the installed TypeScript compiler, imported the temporary ES module from the ticket generated directory, and exercised a simple IFCWALL with omitted `complex`. Forward editing inserted IFCOBJECT. The inverse carried an own `complex` property with value undefined, and replay left IFCOBJECT present: the source-review finding is reproduced against the actual production implementation. The generated module was deleted after evaluation. Native string code generation is disabled in node_repl; normal temporary module import worked.

## Additional Contract Findings

3. The text diff grammar still declares entity removals as `u64-list` of bare integers; the codec now prints `id:index`. Update `entities-diff` to an entity-removed list and define an `INT ":" INT` row so grammar conformance laws accept new outputs.

4. `IfcEntityRemoved.id` JSON schema uses minimum 1 while snapshot entities and mutation IDs use minimum 0 and the Rust/TypeScript appliers accept id 0. Use the same identity domain throughout or enforce a shared positive-ID rule across all surfaces.
