# Finite Scope Wire and Root Cases

Read-only current-code review, 2026-10-01; no source edits/tests/compiler. Latest atom/unary/comparison fixes are present. This recommends the smallest graph-backed seal, not a PDF/Home exception.

## Minimal owned fields and graph reuse

Add `macroUse?: true` to RustModuleGraphFact only when the actual module attributes contain macro_use, including nested cfg_attr meta tokens. Absence means parsed absence, not a default supplied by a caller. Existing visibility/inline/pathTarget/conditional fields remain useful. Preserve source offset for the module declaration if needed to resolve lexical region, because a same-named mount in another scope/order cannot authorize this source. No blanket macro_use inference from file names.

For expanded local data references, add a closed scope union alongside existing definitionOffset/templateOffset/invocationOffset:

- `scope: { kind: "local-block", startOffset, endOffset }` for an ordinary function/closure/block lexical region, with parser-proven container identity and no visible external code mounts. A module brace is not a local block merely because it is the nearest `{`.
- `scope: { kind: "module", modulePath }` for crate/file/inline-module scope. modulePath is source lexical scope, distinct from full Cargo context modulePath. A source container is already known to the per-source record; a cross-file expansion would need explicit source locators and is outside this cut.

Offsets are UTF16, integral, in source bounds and ordered; invocation and definition must lie in the declared scope, templateOffset inside the transcriber. Do not expose a caller-authored sealed:true boolean. Gate proves module scope through actual graph/canonical Cargo contexts and lexical facts, then emits proof provenance: crateRoot, manifestPath, sourceChain and the exact incoming module fact/key. Internal proof can remain separate from compile-reference wire rather than duplicating graph fields in every data edge.

For every module-level template context: either source equals a manifest-proven crateRoot with no unproved external visibility region; or its incoming edge is a private out-of-line module fact, without macroUse/reexport/export and with graph.targets mapping the actual module key to that source. Context sourceChain must agree with the physical parent. Reject orphan/unmanifested conventional root, include-emitted source, macro_use/cfg_attr macro_use, ambiguous mount or reexport route. For inline module scope within a source, verify that module's scope/escape facts, not just its containing physical file.

Important: existing context dedup compares crateRoot/manifest/modulePath/sourceScope/moduleBase and ignores sourceChain/edge kind. A file can be mounted through both an out-of-line module and an include! inside an equivalent inline module, producing indistinguishable kept contexts and a graph.targets entry for the module route. Therefore sourceChain + existence of targets is **necessary but not sufficient**. Inspect all actual incoming include references and module facts for the source under that crate/context, or retain a small mount-origin discriminator in context identity. Any include-emission alternative remains unresolved even if a private module alternative also exists. Do not let first traversal order grant module-only authority.

Initial module graph must use ordinary code mount references independent of finite data success; after graph construction, resolve pending finite scope obligations. Preserve static definition-origin data facts when a dynamic scope proof fails. Function-local proof can seal lexically without exported module authority, but still requires exact block kind and no external emissions in its visible region.

## Six language-neutral proof rows

1. **real-manifest-root**: canonical Cargo lib source defines load and one direct invocation, no escaping mounts. Accept module scope with actual manifest/root; conventional filename alone supplies no authority.
2. **private-file-module**: real root privately mounts leaf via path, leaf defines/invokes load and has no outgoing emission/reexports. Accept with exact parent key/sourceChain and absent macroUse. Use this same general shape for PDF.
3. **macro-use-parent**: same bytes as row2 except actual parent #[macro_use] (and a cfg_attr form in the hostile matrix). Refuse module scope, retain constant literal template facts; real native parent call demonstrates extra reachability.
4. **include-and-module-dual-mount**: one source is both include-emitted into inline module and privately mounted into another equivalent context. Refuse include alternative; assert no context dedup can erase it. A simpler include-only arrangement can be its native subcase.
5. **physical-parent-deletion**: start from row2 but physically remove the parent module declaration/mount while retaining leaf and manifest. Refuse orphan leaf scope; no stored prior graph/cache authority. Also independently remove/alter manifest root binding and require refusal, rather than conventional-root fallback.
6. **local-block-with-visible-emission**: function-local load has ordinary direct calls and an external include! or child module after its definition. Refuse unproved emitted invocation scope; removing the emission yields a valid lexical seal. Retain exact data target prefix/leaf failure checks separately from macro namespace proof.

All rows assert scope state, occurrence metadata/count, typed refusal and actual graph contexts. Native rustc validates reachability/origin on the corresponding accepted or deliberately native-success/refused forms. Deletion laws must instantiate the changed filesystem, not mutate a generated report or fake graph input. No test listed here was run by this audit.

## Current outgoing guard and remaining syntax issues

Current scanner examines all token positions up to lexical scopeEnd and refuses post-definition `mod ...;` or include! emission; it now notices predefinition helper transcriber calls through whole-source candidate scanning and rejects nested outer macro invocation contexts. This closes useful previous cases. The outgoing mod/include guard still reasons lexically without incoming graph proof and treats tokens inside unrelated templates/strings differently according to lexer; keep conservative refusal for ambiguous emitted code rather than weakening it to unblock an owner. Do not apply whole-file parent sibling include bans to PDF's sealed leaf.

Concrete bounded grammar counterexamples remain by source reasoning:

- Struct body parsing is applied after string/number atoms. Discarded expr `"x" {}` or `1 {}` is accepted as an empty struct-like body though Rust expr grammar rejects it. Only an admitted path atom can start struct literal syntax.
- First atom denies reserved keywords, but qualified path segments/member names/patterns/closure parameters still use identifier token kind alone: `value::fn`, `value.fn`, `{ let fn = 1; true }`, `|mut| true`. Add native refusal cases and use position-specific identifier grammar; Rust ident fragments permit some keywords that expr-path/pattern positions do not.

Undefined variable names, calling a boolean as a function, wrong operand types or nonexistent struct fields are type/name-resolution errors, not automatically defects in a bounded **syntax** validator, especially when a valid unused expr is discarded by the macro. Do not grow the grammar into type checking. The listed reserved-token and literal-followed-struct cases are malformed syntax and should refuse even when unused.
