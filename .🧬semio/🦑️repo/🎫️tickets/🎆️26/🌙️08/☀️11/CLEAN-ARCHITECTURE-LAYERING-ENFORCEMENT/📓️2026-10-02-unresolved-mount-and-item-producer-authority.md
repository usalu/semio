# Unresolved Mount And Item Producer Authority

Read-only source inspection, no compiler/tests run.

## First Known Mount Survives Unknown Alternative

Current inspectRustModuleGraph skips module.unresolved before computing/poisoning its canonical target key (discovery/🟦️.ts:8627–8640). A previously known sealed target can therefore remain published despite a competing authored unresolved declaration. Preserve unknown declaration facts as authority poison even if their target is missing, malformed or cannot be resolved.

Minimal root fixture:

```rust
#[cfg(not(feature="rewrite"))]
#[path="leaf.rs"] mod sealed;
#[cfg(feature="rewrite")]
#[cfg_attr(feature="rewrite", unknown_rewrite)]
#[path="other.rs"] mod sealed;
fn main(){sealed::run();}
```

leaf.rs contains the existing module-level load template and one finite invocation; other.rs physically exists. Default native compilation succeeds because the unknown attribute branch is removed by cfg. The all-authored graph must refuse finite authority for canonical sealed, retain both authored declarations and unknown rewriting evidence, and not publish a successful known target. Reverse declaration order is a required sibling. Repeat with r#sealed for the second declaration after canonical raw identity support. Native enabled unknown_rewrite needs a real procedural provider before claiming native success; the default proof requires none.

Compute all module-key obligations before publishing targets/contexts, or retain a poisoned-key set and suppress both prior and later grants. Merely deleting graph.targets after contexts have been queued is insufficient: local-block seal currently accepts physical contexts without a target lookup. Include template definition inside leaf's ordinary function in a hostile sibling to prove that poisoned membership does not survive via context authority. Same known target versus unknown alternative should refuse even if both path strings coincide; ambiguity concerns authority, not only differing physical leaves.

## Item Producers Within Macro Visibility

A finite template's authored invocation scan cannot prove completeness if an item producer in its visible lexical scope can synthesize calls. Concrete module-level shape: define load; add `#[cfg_attr(feature="rewrite", rewrite)] fn helper(){}`; retain an ordinary authored load!("safe.txt") in run(). Default native success requires no provider. An active real attribute may emit a helper body containing load!("escape.txt"). The module template must remain unresolved until that provider's emission authority is owned.

Concrete function-local shape:

```rust
fn main(){
    macro_rules! load { ($p:literal) => { include_str!($p) }; }
    #[cfg_attr(feature="rewrite", rewrite)]
    fn helper(){}
    let text=load!("safe.txt");
    println!("{}",text);
}
```

The unknown attribute is on a separate nested item, so refusing attributes on the containing main function does not close it. The nested item's generated body can access the textually visible macro. Equivalent dormant derive producer on a local struct can emit an invocation or helper item. No claim that a particular existing public macro behaves this way is needed; use an actual tiny owned provider in the enabled native oracle to demonstrate emitted call/input and source spans.

Closed scope facts should retain item producer obligations (attribute/derive/macro kind, exact source span, lexical scope and metadata conditions), share the existing metadata parser, and make finite seal completeness depend on no unresolved producer in the macro's reachable visibility. Conditions retain all authored configurations. Inert lint/doc/cfg metadata stays separately classified; do not blanket-refuse all attributes or blanket-trust all derives. Actual provider identity plus an owned expansion contract may discharge the obligation later.

Static definition-file literal inputs stay inventoried even while dynamic proof refuses. Unknown producers outside a function-local macro's visibility should not poison it without a demonstrated route; use lexical scope/offsets and actual item boundaries, not whole-file substring bans.

Required test pairing: default native-success/typed refusal, enabled tiny-provider native emitted escape dependency, physically missing escape input after emission, and inert metadata positive sibling. No scanner-only or empty-pass conclusion is asserted.
