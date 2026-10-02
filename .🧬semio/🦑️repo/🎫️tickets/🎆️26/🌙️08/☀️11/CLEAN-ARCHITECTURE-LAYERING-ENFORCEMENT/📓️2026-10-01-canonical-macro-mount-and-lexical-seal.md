# Canonical Macro Mount and Lexical Seal

Read-only current scanner/graph/source review, 2026-10-01. No source edits, tests, compiler or generation. This narrows the current enforcement policy as requested: accept private PDF file-module and Home function-local proofs; refuse module-level crate-root, public/conditional-exporting mount, include origin and missing manifest until broader reachability is implemented.

## Minimal sound algorithm

1. Syntax discovery emits pending template facts with definition/transcriber/invocation offsets, source lexical module path, and exact nearest enclosing **container kind**. Do not equate every nearest brace with a function/local block. Classify function body, closure body and ordinary expression block versus module/impl/type/macro token-tree containers. Unsupported container kind stays unresolved. Keep static definition-origin literal inputs even if a dynamic obligation cannot seal.
2. Build strict physical Rust graph independently from pending data-template proofs, using all ordinary module/code-include references and valid authored Cargo roots. Every accepted template must have at least one real manifest-proven source context; conventional lib.rs/main.rs is not manifest authority. Validate every context/origin alternative, not just first.
3. For a module-level template, require an incoming **private out-of-line module** boundary for its actual source/module scope, no macroUse or cfg_attr macro_use, no template export or macro reexport, and no outgoing external code emission within its postdefinition visibility region. Root-level module templates have no shielding incoming private boundary and are refused by this bounded cut. Public mounts remain refused even though public visibility alone does not export macro_rules names; that is an intentional conservative scope, not a Rust claim.
4. For local-block templates, require an actual ordinary local container and full declaration/invocation containment, no template export/reexport, no ancestor unknown macro token-tree context and no external file-module/include emission in the visible block/subtree after definition. A private module mount is unnecessary for the lexical seal; actual source/Cargo context is still required. Include-emitted local code may be lexically sealed only if exact physical source origin/context is retained and no unknown code emission occurs; refuse rather than infer when the graph cannot identify that origin.
5. In both branches, gather every direct invocation in the proven region and all unsupported generated/alias/external candidates before success; one unsupported reachable occurrence invalidates the dynamic proof. Failed proof yields a typed unresolved-macro-scope problem plus preserved static facts. Do not return an empty successful references array.

## Incoming origin must be first-class or exhaustively recomputed

Current `RustModuleContext` at discovery8521 contains crateRoot/manifestPath/modulePath/sourceScope/moduleBase/sourceChain. `addContext` at8555 deduplicates without sourceChain or incoming kind. Module traversal creates child at8579; include traversal creates child at8591 without a distinguishing origin. `graph.targets` records module membership keys but not include edges. A target entry plus sourceChain is insufficient to prove an exclusive module mount, because alternate include paths can collapse into the same existing context.

Smallest canonical addition is a required incoming-origin union on context (or a separate graph origin multimap keyed by context identity):

- crate-root: canonical manifest and root locator;
- module: parentSource, parent module/source scope, exact module declaration offset/name, physical target, visibility, inline, macroUse;
- include: parentSource, exact include offset, parent scope, physical target/base.

Use only graph-produced validated facts. Preserve all origin alternatives by including this discriminator in dedup or storing a deterministic union. Keep sourceChain for cycle/cross-file provenance; it cannot replace the union. `macroUse?:true` belongs to actual parsed module attributes, including any cfg_attr nested form; absence is proven by parsing. Conditional cfg(test) alone remains permitted for the private PDF test boundary. A cfg_attr capable of adding macro_use invalidates local-module exclusivity across configurations.

A minimal no-new-context-field implementation may instead enumerate **all** graph-backed incoming module facts and ordinary include references for the target under each real crate/context, requiring no matching include origin. It must inspect alternative source chains and roots, not reconstruct one edge from the retained context. With current dedup it is easier and safer to preserve mount origins explicitly.

## Why the original sources accept

PDF package lib is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs`, declared by artifact Cargo manifest lib path ../../🦀️.rs. Facade627–629 mounts the 1.7 base mutations owner. Its `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:289–291` privately mounts tests_lopdf_vectors under cfg(test), with no macro_use. The test source contains only the local definition/direct invocations plus ordinary functions, no outgoing mod/include!/export/reexport. This yields the required private file-module origin per actual artifact/test context. No owner-specific exemption needed.

Home `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/.../🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs:15–40` defines committed! inside ordinary vector() function and calls it only in three match arms within the same function. Its scope ends at the function close; the later oracle/subject functions do not share its macro. The nearest brace is genuinely a function body, with no external code mounts in the visibility region. Resolve actual test-host/package context separately; a path string under tests does not prove that context.

## Essential hostile graph rows

Keep paired real native/source facts for private module accepted versus public mount refused; direct and cfg_attr macro_use refused; include-only/dual module+include refused; root source refused; physical parent-mount deletion and manifest deletion refused; Home-style function block accepted versus an otherwise identical inline module scope or code-emitting block refused. Native compiler success on conservative refusals establishes that the gate scope is deliberately bounded, not that refusal models a compiler error.

Current scanner's outgoing guard at6642 rejects postdefinition file-module declarations/include! anywhere before scopeEnd; it cannot prove incoming origins. Whole-source candidate scanning may conservatively poison an unrelated earlier same-name occurrence; narrowing that must preserve helper-generated reachability detection. No scanner-only branch should mark a module template sealed before the graph proof.

The latest bounded expression syntax fixes address the earlier unary mut, first-atom reserved keywords and comparison chains. Remaining string/number-followed-struct and keyword-in-path/member/pattern/closure issues were reported in the previous scope-wire report. Undefined names/incorrect operand types in a discarded syntactically valid expr are outside syntax proof and should not force compiler/name-resolution emulation.
