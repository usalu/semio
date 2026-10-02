# Bounded Macro Grammar and PDF Context Closure

Read-only current-source audit, 2026-10-01. Root reports scanner receipts PDF88/88 expansions and Home15/15; this audit ran no scanner/test/compiler. Runtime corpus proof remains Root-owned.

## Remaining concrete acceptance defects

In `📚️library/🔍️discovery/🟦️.ts:6540–6615`, expression parsing now rejects the earlier malformed grouped expressions and closure bodies through recursive syntax. Strict comma segments and attribute-token macro_export detection close earlier specific gaps. Three new native-false laws are warranted before complete grammar claims:

```rust
macro_rules! load { ($p:literal, $unused:expr) => { include_str!($p) }; }
fn main() { let _ = load!("input.txt", -mut value); }
```

The unary branch skips a following mut token for every unary operator. Only & borrow grammar admits mut; `-mut value`, `!mut value`, `*mut value` must not become valid irrelevant expr fragments. Type resolution of value is immaterial because the macro does not emit this argument.

```rust
// same macro
fn main() { let _ = load!("input.txt", mut); }
// separately: load!("input.txt", _)
```

The atom branch accepts identifier-kind tokens except its explicit denylist; bare mut and underscore are not valid expr fragments. Token kind is not identifier-expression authority. Distinguish allowed self/path keywords and literals from reserved/unusable atom forms with an explicit closed set.

```rust
// same macro
fn main() { let _ = load!("input.txt", 1 < 2 < 3); }
```

The Pratt loop allows repeated comparison precedence; native Rust prohibits chained comparison syntax. Add native-false mixed/equal comparison chains plus a valid parenthesized comparison case. These predictions have not been compiled here. Keep refusals structural, not type checking: undefined names or invalid operand types in a discarded syntactically valid argument are different from malformed matcher fragments.

## Actual PDF parent chain

Artifact package `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml:3,15–16` declares semio-s-artifact-stdio-pdf with lib source ../../🦀️.rs. That artifact facade at627–629 mounts the 1.7 base mutations source in the real `standards::v1_7::subsets::base::schema::mutations` module. Base mutations source `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:289–291` privately mounts `tests_lopdf_vectors` through `#[cfg(test)] #[path="🧪️tests/⚖️lopdf-vectors/🦀️.rs"] mod tests_lopdf_vectors;`.

The actual lopdf test source owns one local macro_rules lopdf_vector at95, forty-four direct invocations at106–171, two data include sites at99, and no mod declarations, include! code emission, macro_use, macro_export or macro reexports. The parent mount has no macro_use attribute. Thus this specific macro has a **closed module-leaf context** once graph mount provenance establishes each actual compiler context and confirms no escape attribute/reexport. It is not the general case of a root macro with external child modules. The unrelated PDF-derived schema/IO modules contain include! code mounts; these belong to sibling contexts and do not automatically expose this private leaf macro. Closure checks must use source/module lexical reachability, not whole-artifact substring bans.

Home committed! is even narrower: defined inside vector()'s function block and used in three match arms in the same function; graph authority plus block scope/no nested external emission closes that local visibility region. Do not infer the same seal for every test file merely because it is under tests.

## Graph-context contract required

Existing RustModuleContext at discovery8516 contains crateRoot, manifestPath, modulePath, sourceScope, moduleBase and sourceChain. Existing module facts contain name/modulePath/visibility/inline/pathTarget/conditional; include facts contain modulePath/path/conditional. They prove physical mounts but do not yet encode macro_use/export/lexical declaration ordering or macro alias routes. They cannot alone attest a macro namespace seal.

Add a schema-owned finite proof input over graph contexts, not caller-provided boolean exemptions:

- Source locator and actual macro definition UTF16 range; enclosing lexical block/module range; every invocation range and ancestor macro-token context.
- Exact incoming source mounts per Cargo context, mount kind (external-module vs include), parent source/module scope, mount offset, all attribute tokens and relevant configuration conditions. A source included into an existing module inherits that module's textual namespace; a private module mount without macro_use does not export its local macro back to the parent.
- All outgoing external file-module/include emission edges in the macro's visibility region, plus use/reexport/export/conditional-export facts. Resolve these children and propagate scope or return unresolved reachability. Explicit absence is derived from parsed source facts and graph, not assumed from fixture role.
- A finite local proof is accepted only if every source context is sealed or every reachable child/alias/generated invocation is accounted. Unknown parent mount context, conditional escape, helper-produced call, macro_use or cross-file dynamic span origin yields typed unresolved-macro-scope. Static definition-origin data inputs still remain inventoried.

Build this after initial syntax/reference inventory and graph construction so scanner cannot authorize closure before mounts exist. Return syntax template obligations separately from resolved expanded references; a failed obligation must not remove ordinary/static input facts. Module graph should consume ordinary code mounts independent of successful finite data-template binding to avoid the circular dependency graph→bindings→graph. Each proof records the actual graph context so the same source mounted in two packages does not acquire authority from only one.

## Minimum remaining reachability laws

Keep genuine native tests for a sealed private file-module leaf, macro_use exporting that same module, include!-mounted equivalent, inherited external child macro call after a valid root call, earlier helper transcriber, unknown outer macro dropping nested tokens, reexport alias, conditional export and multiple source contexts. Use two physical directory contents and dep-info to prove cross-file literal span origin if support is added; otherwise assert typed refusal while retaining constant definition facts. No exemption for PDF/Home is needed: their actual parsed lexical leaf scopes satisfy the same general graph seal.
