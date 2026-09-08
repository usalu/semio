# Remaining Lint Review

Pass 458 checked the final WASI 428 primary spans against the current source, including complete source lines and UTF-8 byte offsets. Of the old diagnostics, 194 still exactly matched; this is a source-review inventory, not a fresh compiler result or a complete remaining-warning count. Fresh native, browser, and strict WASI checks remain queued behind active compilation.

Pass 459 collects documentation, default construction, and ineffective attribute diagnostics for manual review before guarded edits. Generated inventories are temporary ticket outputs.


Pass 461 replaces six field-wise default implementations with derives and supplies four Default implementations that delegate to their existing constructors. Full diagnostic text and UTF-8 spans were checked before edits; complete file contents were compared immediately before each write. This changes no initialized field value or ownership lifecycle. Compiler validation remains pending.


Pass 462 fixes accidental Markdown lists, detached documentation, and obsolete ineffective lint attributes. Snapshot documentation now sits on existing explicit snapshot exports; stale annotations for removed declarations are removed. Program child-field documentation now accompanies the actual fields. Six Sequence WASM exports now document allocation provenance, aliasing, buffer validity, and destruction requirements under Safety headings. This pass changes documentation and ineffective attributes only; it adds no lint suppression and makes no runtime claim.


Pass 465 makes four mutation-report serializers, two empty separator builders, the Note command-unit expansion helper, and the generation retirement state helper return their successful value directly. Their callers retain real parse/reduction errors and extent refusals; the erased retirement trait still wraps its required Result at the boundary. No fallible branch or owner was removed. Compiler and existing behavior verification remain pending.


Pass 467 parsed all 51 files changed by passes 461, 462, 464 and 465 with rustfmt in edition 2021 and skipped module traversal. 0 files failed syntax parsing. This validates source syntax only; type checking, lint validation, linking and runtime tests are still required.


Pass 468 replaces the mounted-domain retirement loop, which always returned after one iteration, with one conditional while preserving its one-item close budget. It merges identical grid-exclusion branches without changing their condition and replaces two fixed hourly schedule loops with slice fills over the identical 07:00–18:00 interval. Compiler and existing mesh/energy runtime checks are pending.


Pass 469 replaces twelve public unit-error signatures with domain-owned, allocation-free error types for fixed FEM capacity/index admission and Presentation projection lock access. The same failure conditions occur before the same mutations; existing callers still map errors to their existing operation codes. Rejected capacity requests retain their requested and maximum values. Publication backpressure continues returning Ok(false) and retaining the original owner. The scalar buffer and compressed image rope now supply is_empty alongside len. Compiler and existing admission/publication runtime checks remain pending.


Pass 474 places the fixed Process/Layout extent callbacks and the Note configuration preparation callback directly in their existing constructor calls. Their Option/Result signatures are required by those constructor contracts. The callbacks retain their exact prior values and evaluation order, with no new allowance or fallback behavior. Compiler and existing retained-command checks remain pending.


Pass 481 removes three unused pack_err_as_text adapters from CAD, Layout and Flow IO. A cross-product search through Rust, TypeScript, JSON and Markdown under s/framework and the root script found only these three definitions. Actual codec registrations do not call them. No public runtime export or parsing path was changed; fresh compiler verification remains pending.


Pass 480 borrows drawing cache inputs and mutation-report message slices at their read-only boundaries. Layout and Equation topology builders instead consume the buffers already passed to them, moving edge and queue strings rather than cloning them. Node order, neighbor order, indegree/depth calculation, and JSON output are preserved. Compiler and existing topology/drawing checks remain pending.
