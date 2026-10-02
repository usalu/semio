# Raw Identifiers And Attribute Compile Inputs

Read-only source audit on 2026-10-01; no tests or compiler run. Parent reports an actual provider receipt of 16 laws / 607 assertions; this audit does not independently establish passing behavior.

## Canonical Symbol Identity

`rustTokens` preserves raw identifier spelling at discovery/🟦️.ts:6408–6415. Finite template discovery stores that spelling, duplicate-name refusal compares `candidate.name === template.name` at6654, invocation coverage compares token text at6658, and recursive template rejection compares transcriber token spelling at6648. Rust raw and ordinary identifiers refer to the same non-keyword symbol. Therefore these comparisons admit a concrete missing-input route:

```rust
fn main() {
    macro_rules! load { ($path:literal) => { include_str!($path) }; }
    let _ = load!("safe.txt");
    let _ = r#load!("escape.txt");
}
```

Both files existing makes the native program valid by language reasoning; the scanner currently binds only the ordinary-spelling call. Reverse definition/invocation spelling must also be covered. Defining `load` and later redefining `r#load` in a lexical scope is another canonical duplicate/shadowing case: refuse the finite proof rather than invent shadowing support. Qualified `self::r#load!`, outer-helper `r#load!`, and recursive raw-spelled invocation must trigger the same conservative refusal as ordinary spelling.

Introduce a pure symbol comparison function removing the syntactic r# prefix from identifier tokens, retaining original UTF16 offsets and authored spelling separately. Apply it to definition names, invocation matching, recursion, duplicate definitions and use-alias identity. Keyword recognition must not simply use canonical text globally: raw identifiers such as r#mod are identifiers, not module keywords. Builtin compile macro spellings (`r#include_str!`) likewise require a native-positive/reference assertion and canonical macro dispatch. Metavariable binding names should use canonical identifier identity consistently where the Rust grammar accepts raw metavariable names; prove this separately with tiny native cases rather than assume spelling distinctions.

The execution alias detector currently splits a rendered use specifier on punctuation; r#load splits to r/load, so an ordinary definition may happen to match. A raw-spelled definition string r#load will not match those pieces. Canonical symbol tokens are preferable to this accidental string behavior.

## Actual Authored Attribute Inputs

A multiline search over Rust sources under framework, S plugins and Hub found these concrete first-party inputs:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs:2045`: doc concat includes `../🧪️tests/🚫️image-description-required/🦀️.rs` inside a compile_fail documentation fence.
- Same file:2046: doc concat includes `../🧪️tests/🖼️image-described/🦀️.rs` inside a successful documentation fence.

They are compile-time source dependencies of the authored builder file. `visit` consumes attribute ranges for classification, then skips those ranges without examining their include macros (discovery:6687–6746). Generic bracket recursion does not repair the skipped outer attributes. Existing authored examples therefore require physical input inventory even before introducing hostile corpus cases.

## Minimal Closed Obligations

Walk authored outer and inner attribute token ranges explicitly for known compile-input builtin calls, including doc=concat!(...) and nested cfg_attr. Retain each include site offset, definition-file origin and current modulePath. Existing literal/concat/env path expression rules can resolve the builtin argument; no procedural output inference is necessary. Inventory every authored cfg branch under the gate's all-config contract, including inactive cfg_attr predicates. Do not count token substrings in string documentation or comments.

Required language-neutral/native paired rows:

1. Direct `#[doc = include_str!("doc.txt")]` and crate inner `#![doc = include_str!("doc.txt")]`: one input each, exact offsets; physical deletion must fail native compilation and gate resolution.
2. The two actual builder concat forms: each retained as its own source fact, unchanged relative file origin.
3. `#[cfg_attr(all(), doc = include_str!("doc.txt"))]`: native success, one fact.
4. Dormant `#[cfg_attr(any(), doc = include_str!("other.txt"))]`: native succeeds even if other.txt is absent; all-config gate inventories and reports the missing input. Explicitly distinguish that expected configuration difference.
5. Dynamic unresolved attribute builtin input: typed unsupported-expression; no silently empty reference list.
6. Documentation string containing textual `include_str!(...)`: zero builtin facts, proven string-token decoy.
7. Mixed function attribute plus finite local template: attribute input remains inventoried while unknown function rewriting metadata conservatively refuses the finite template scope.

An arbitrary procedural attribute accepting tokens that spell include_str! does not prove that Rust expands those tokens as an input. Either constrain extraction to owned builtin-expanded attribute value contexts (doc/cfg_attr doc), or mark opaque attribute compile-looking tokens unresolved; do not silently grant expansion authority. Real quote/procedural emitted includes remain a separate provider/consumer closure problem.
