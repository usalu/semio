# Function Attribute Scope Proof Plan

Read-only source review; no tests or compilers were run. Existing 30 macroOwners and 17 law bodies remain unchanged.

The discovery provider `library/🔍️discovery/🟦️.ts:6744` currently requires zero outer attributes to issue a local-block seal. The shared `rustMetadataAttributes` at 8383 recursively inventories every cfg_attr consequence independently of predicate truth; cfg only contributes conditional status. Executor `library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:145–164` additionally requires coherent invocation module paths and live admitted manifest contexts, excludes include mounts, and checks strict definition/template/invocation/body ordering. Retain these checks.

Minimal provider cut: replace the zero-attribute condition with a complete metadata proof, using the existing recursive parser once. Permit only explicitly owned transparent function attributes allow/warn/deny/forbid/expect/doc (and test only with an independently registered native test witness). cfg/cfg_attr must have structurally valid predicate/consequence syntax; malformed metadata must not become an empty safe items list. Any unknown consequence, including dormant consequences, denies the local seal. Keep current bounded signature and async/unsafe/const/extern refusal. Do not retry fn after discarding attributes.

Current inner-attribute branch at 6718–6732 inventories doc inputs but then continues without revoking the enclosing local seal. An unknown inner consequence needs to poison the entire body before publishing any expansion facts; likewise existing nested item producer rejection must remain. Scan body authority before assigning template.scope, rather than revoking only later templates. The existing dormant outer row at fixture 2643 and nested item row at 2910 are native=true refusal witnesses and must retain that outcome.

Use this exact common body for proposed hand-authored rows:

```rust
pub fn run() {
macro_rules! load { ($p:literal) => { (include_str!("input.txt"), include_str!($p)) }; }
let (a,b)=load!("input.txt"); println!("{} {}",a,b);
}
fn main(){run();}
```

| Authored modification | Expected policy facts | Native expectation, not executed |
|---|---|---|
| None | local-block; two expansion facts; root mount required | Valid |
| Prefix #[allow(dead_code)] | Same seal and two facts | Valid |
| Prefix #[doc=include_str!("doc.txt")] | Same seal, two expansion facts plus independent authored doc input | Valid with doc.txt |
| Prefix #[cfg(all())] | Same all-authored facts and conditional metadata | Valid |
| Prefix #[cfg(any())], change main to fn main(){} | Same all-authored facts despite dormant function | Valid; dormant input absent from active dep-info is not an inventory exclusion |
| Prefix #[cfg_attr(any(), allow(dead_code))] or #[cfg_attr(all(), allow(dead_code))] | Same seal; recursively proved known consequence | Valid |
| Prefix #[cfg_attr(any(), foreign_transform)] | unsupported-expression; no successful finite-binding authority | Valid because consequence dormant; existing row retained |
| First body line #![doc=include_str!("inner.txt")] | Independent inner doc input plus both expansion facts | Expected valid with inner.txt; must establish native witness |
| First body line #![cfg_attr(any(), foreign_transform)] | Deny whole body authority, including templates preceding later unknown item producers | Expected valid dormant consequence; native witness needed |

Goldens must compute actual UTF16 brace offsets from exact authored bytes, keep root/inline modulePath coherence, and assert constant definition-file facts separately from invocation expansion facts. Active rustc dep-info cannot prove dormant authored completeness. Each positive needs original runtime bytes and physical input deletion; dormant rows need explicit authored facts independently of the native active input set. Unknown rewriting rows need native-success plus typed refusal, with no quote exemption.
