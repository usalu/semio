# Whole Template Physical Scope Followup

Read-only source review; no scanner/native test or compilation run. The following are closed fixture recommendations, not measured runtime receipts.

## Concrete Inner Module Metadata Grant

Graph-facts parseScope (discovery/🟦️.ts:8675–8681) consumes inner #![...] attributes and retains only their conditional flag. Unknown inner metadata never becomes the unresolved field now added to outer module declarations. A genuine private out-of-line leaf can therefore receive the current finite template seal despite authored conditional module rewriting metadata:

```rust
// root.rs
#[path="leaf.rs"] mod sealed;
fn main(){ sealed::run(); }
```

```rust
// leaf.rs
#![cfg_attr(any(), rewrite)]
macro_rules! load { ($p:literal) => { include_str!($p) }; }
pub fn run(){ let text=load!("input.txt"); println!("{}",text); }
```

With a canonical manifest and input.txt, default native success is expected because rewrite is inactive. The all-authored policy must refuse unknown module rewriting authority, rather than treat the only metadata as harmless conditional evidence. The scanner's local function plain-body classification does not fix a module-level template. Add a nested inline parent with the same inner attribute and a privately mounted child leaf; inherited unresolved proof must prevent child authority as well. Record source-level unresolved metadata and propagate it through physically mounted contexts, including inner crate/module metadata, without dropping independent static input facts.

## Canonical Module Symbol Identity

Module declaration names and lexical paths remain name.text (discovery:6733,8700; graph child construction:8637), whereas finite macro symbols now canonicalize raw identifiers. Mutually exclusive root declarations `#[cfg(not(feature="alternate"))] #[path="one.rs"] mod sealed;` and `#[cfg(feature="alternate")] #[path="two.rs"] mod r#sealed;` are one Rust module symbol but become separate graph.targets keys. Native default and enabled branches independently compile; all-config graph should retain both mount facts under canonical sealed identity and mark conflicting physical targets ambiguous. Each leaf should then refuse module-level finite authority. This is a graph identity false grant relative to the existing same-key ambiguity rule, not proof that private macros automatically escape. Preserve declaration offsets/raw source spelling while canonicalizing module identity; keyword matching remains authored so r#mod is never a module keyword.

## Occurrence Path Coherence

Execution source/🏃️execution/🟦️.ts:115–119 checks every expansion's shared scope and monotonic offsets, but maps occurrences to expansion objects only, losing reference.modulePath for later checks. For a local-block template it selects physical contexts from the first reference.modulePath; for module templates it uses expansion.scope.modulePath. Require every occurrence reference's canonical modulePath to match the template lexical scope, and preserve each occurrence's distinct invocation/template offset. Closed API-negative row: two occurrences share definition/scope but the second claims another modulePath; typed scope refusal. Native positive sibling is a real free function inside an ordinary inline module with two input sites/invocations and exact sourceScope context. Current generated scanner rows are internally consistent; no authored source counterexample for this particular metadata mismatch is established.

## Mount And Source Chain Review

The current graph retains distinct JSON contexts, including mount origin/sourceChain. Immediate include origins are refused by the executor; same physical file mounted through include and mod cannot silently collapse. Inline scopes use matching sourceScope to distinguish contexts within the same physical file; direct inline module-level templates are conservatively refused by mount.inline. Private out-of-line child contexts retain immediate parent scope/path/chain proof. Missing manifest or cycle-invalid manifests cannot grant the current seal.

The execution parent lookup verifies the immediate predecessor but does not recursively reject an include-emitted ancestor. If the intended contract disallows any include-origin ancestry, add a canonical chain-origin law and recursive proof rather than relying on string sourceChain alone. This is a contract boundary to decide explicitly; no standalone macro escape exploit is asserted here.

Retain all configurations and all physical occurrences in each row. No scanner-only empty result or fullpass claim is warranted.
