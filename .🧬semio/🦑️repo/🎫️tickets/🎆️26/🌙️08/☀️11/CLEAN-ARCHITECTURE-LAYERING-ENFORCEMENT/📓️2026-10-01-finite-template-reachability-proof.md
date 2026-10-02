# Finite Template Reachability Proof

Read-only source reasoning, 2026-10-01. No production edits, tests, compiler or generation. This follows Root's explicit contract: constant definition-file input facts remain inventoried even in unused/exported templates; dynamic finite success requires all possible authored invocation facts to be closed. Examples are proposed native law inputs, not executed outcomes.

## Separate proof states

A direct literal include_str/include_bytes in a template has a known authored source-origin fact and stays in the all-template inventory even with no supported invocation proof. It must not erase unresolved dynamic obligations elsewhere in the same template. For supported finite local invocations, record expansion occurrences for constant and substituted sites separately; do not conflate one static definition fact with N invocation facts. Unsupported external/helper contexts may retain constant facts while returning an explicit unresolved finite-template obligation. No input-template failure may silently become an empty GREEN source result.

## Conservative token rules

1. Parse all attributes attached to macro_rules before declaring dynamic local proof. Direct `#[macro_export]` fails local-only authority. Any `#[cfg_attr(...)]` capable of introducing macro_export also fails: `#[cfg_attr(feature="export",macro_export)]`, multiple trailing attributes, and nested cfg_attr forms. This all-config gate cannot decide one current cfg evaluation and call the definition local. For the current bounded implementation, rejecting **all cfg_attr on a dynamically bound template** is conservative and simpler than implementing recursive meta grammar. If supporting other cfg_attr forms, walk nested paired meta arguments and reject macro_export anywhere in the attribute subtree, never substring-search comments/strings. Conditional macro definitions also need explicit all-config scope handling; do not use one branch's binding list as universal proof.
2. Detect aliases/reexports that make the macro namespace reachable outside the finite scan: `pub(crate) use load;`, `pub use load as renamed;`, grouped use imports/reexports and qualified paths. Local unqualified proof must reject these as `external-macro-reachability` (proposed typed reason), or fully trace the alias graph. Qualified invocations already fail bounded authority; renamed unqualified calls must not evade that refusal. Public wildcard reexports require namespace-aware proof or conservative refusal when they can expose the template; package/export identity alone is insufficient.
3. Detect invocation candidates across the **whole source token tree**, not only post-definition direct calls. An occurrence of `load ! (...)` inside any macro_rules transcriber, repetition or nested template is `template-created-invocation`; reject that template's dynamic finite proof even if other direct calls bind literal paths. A call inside a transcriber **before** load's definition still counts as unsupported authored expansion mechanism. Scanning only definition-end→scope-end misses that case. For direct invocation candidates, require ordinary lexical Rust context; any ancestor macro invocation token tree is unsupported unless that outer macro's expansion semantics are explicitly proven. `opaque!(load!("x"))` may discard or rewrite the nested tokens; it is not a direct Rust invocation merely because the scanner sees name! syntax.
4. A supported transcriber must not manufacture additional invocations through helper/forwarding/recursive macros. Reject unknown macro calls in its body that can create input/derive/module code, unless a closed expansion dependency graph proves them. Root's direct self-recursion check is necessary but not complete for mutual recursion/helper forwarding. Retain known include_str/include_bytes/concat/env calls as their explicitly owned data-expression grammar; ordinary function calls are not macro calls. A helper name can precede or follow the definition, and a qualified helper can live in another crate.
5. Most critically, local macros are textually inherited into external child modules. A root `macro_rules! load` followed by `mod child;` can be invoked unqualified from child.rs without macro_export or use. A same-file-only proof with one root invocation therefore does not close the child invocation set. When an external file-module declaration or include! code emission exists inside the template's lexical visibility region, either resolve its complete physical module graph and inventory those occurrences or refuse dynamic local proof as `external-macro-reachability`. Root-defined template scope can encompass a child mod declaration nested in an inline module; only checking top-level mod names is insufficient. cfg/cfg_attr file mounts count in all-config scope. Unknown helper expansion that emits a module is another unresolved code-emission context.
6. Duplicate names, shadowing definitions, scopes and lexical order must be bounded explicitly. Until implemented, any same-name definition refuses the local finite proof. A candidate before definition or outside its block cannot be counted as a valid binding for that definition. An inline child can inherit a root macro without changing include data source origin. Definition source scope, invocation lexical scope and include file origin are three separate facts.

## Exact minimal counterexamples

```rust
#[cfg_attr(feature="export", macro_export)]
macro_rules! load { ($p:literal) => { include_str!($p) }; }
fn main() { let _ = load!("input.txt"); }
```

The dynamic proof is refused even when the current compiler invocation has export disabled; the authored feature configuration permits external callers. Preserve any separate constant template sites.

```rust
macro_rules! call_later { () => { load!("second.txt") }; }
macro_rules! load { ($p:literal) => { include_str!($p) }; }
fn main() { let _ = load!("first.txt"); let _ = call_later!(); }
```

The earlier helper transcriber cannot be omitted while accepting only first.txt. Its generated call is unresolved under current finite grammar. Scan all token-owned candidates and return typed template-created-invocation, or implement actual helper expansion closure.

```rust
macro_rules! load { ($p:literal) => { include_str!($p) }; }
mod child;
fn main() { let _ = load!("first.txt"); }
// child.rs: pub const TEXT: &str = load!("second.txt");
```

Per-source finite scanning misses the child occurrence while the graph compiles it. Dynamic local proof must refuse the external-child visibility region until graph-backed macro scope propagation exists. Do not guess second.txt's base: independently prove compiler span/file origin and then resolve physical prefixes.

```rust
macro_rules! load { ($p:literal) => { include_str!($p) }; }
macro_rules! discard { ($($tt:tt)*) => {}; }
discard!(load!("never-expanded.txt"));
fn main() { let _ = load!("first.txt"); }
```

Authored-all-templates may conservatively retain literal definition facts, but nested candidate tokens are not proof that a native invocation actually expanded. Reject a finite actual-expansion claim for unknown outer macro context instead of producing a fabricated expansion occurrence.

## Source origin and procedural distinction

A constant data include token authored inside a declarative macro has its known definition-file source origin for the retained static fact. When expanding a passed metavariable literal across files, compiler spans combine definition and invocation tokens; current same-file cases cannot prove which foreign file anchors the effective include resolution. Add native cases with different contents under both definition and invocation directories, plus dep-info and exact binary output. Until that real origin is proven, keep cross-file dynamic authority unresolved. Do not rebase include data to Rust inline module directory context.

Procedural quote is different: #taxonomy_dependency/#descriptor_dependency/#payload_schema_dependency are computed absolute portable targets emitted from validated **consumer** authority, not declarative template substitutions. Their consumer span/provider and authority projection must be proven with real derive consumers. Detecting cfg_attr/helper macro reachability does not solve those procedural obligations and must not introduce a blanket quote exemption.

Recommended closed cases: conditional export both compiler feature states; earlier/later helper; nested outer macro discarding versus expanding tokens; external child module plus valid root invocation; local reexport alias; mutual recursion; static-only unused/exported template remains inventoried; mixed static/dynamic unsupported template retains static facts and reports unresolved dynamic authority. Each law should assert typed refusal/provenance/counts, with native outcomes used to prove reachability/origin rather than mislabeled as scanner acceptance.
