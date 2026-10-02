# Macro Owner Corpus Review

Read-only review, 2026-10-01. Inspected eleven current macroOwners rows and new law at `📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts:312–365`. No compiler/test/edit action. Native outcomes below are source-reasoned, not executed.

## Current specification

All eleven rows declare native=true. Their root sources provide main, leaf run() is public, literals exist, macro_use and cfg_attr(all(),macro_use) permit the escaped parent call, and include-emitted source contains valid items/expressions. No obvious native source contradiction found. The #[lib] Cargo root is separately used for graph authority while rustc compiles the same source as a binary; this is legitimate source-context evidence but not a claim that a Cargo lib test ran.

Local scope ranges are correct as **half-open UTF16 ranges**: function-local-template start13 points at `{`, end164 points at newline immediately after its `}`; code-emission version ends189, likewise after `}`. Schema/implementation must use closing-token end, not closing-token start. ASCII rows cannot independently establish UTF16 behavior under emoji/non-BMP source prefixes; add one offset case if that unit is public contract.

Every base source has one constant include_str("input.txt") and one dynamic include_str($path); one direct invocation therefore must retain exactly two occurrence facts, even though both targets are input.txt. The harness asserts two refs and identical scope metadata, correct for occurrence inventory. It does not yet assert exact template/invocation offsets or different site identities; add that to avoid two accidentally duplicated copies satisfying the count.

Private file-module and function-local rows should accept bounded scope. Macro_use, conditional macro_use, public module, root template, include-only, dual origin and absent physical ownership should produce unresolved-template-scope. Function-local code emission produces scanner unsupported-expression before graph scope, matching the explicit current policy.

## Missing graph collision and alias proof

Current include-and-module-dual-origin uses modulePath included versus included_again. It tests two origin kinds exist, but their context keys differ, so it does not close the existing same-key/context dedup hazard or graph.targets collision. Add mutually exclusive cfg mounts under the **same module name**:

```rust
#[cfg(not(feature="included"))] #[path="leaf.rs"] mod sealed;
#[cfg(feature="included")] mod sealed { include!("leaf.rs"); }
fn main(){sealed::run();}
```

Native default selects private module and succeeds; authored all-config graph must retain the include alternative and refuse. A corresponding enabled-feature native run can establish actual second branch reachability. Do not let graph.targets first-writer conflict discard the second branch before origin proof.

Add actual macro namespace alias reachability in a private leaf:

```rust
macro_rules! load { ($path:literal) => { (include_str!("input.txt"),include_str!($path)) }; }
pub(crate) use load as exposed;
pub fn run(){let _=load!("input.txt");}
// parent: fn main(){sealed::run();let _=sealed::exposed!("escape.txt");}
```

This should be native-success with typed unresolved-template-scope. The parser must recognize grouped/renamed macro reexports, not only load! invocations. A local nonpublic alias can likewise introduce another same-file occurrence unknown to finite name matching. Retain refusal until alias expansion is implemented, while keeping constant facts.

Also add cfg_attr(any(),macro_use) with no escaped current parent call: native selected configuration has no export but all-config scope still refuses capability. The current all() case only proves active macro_use, not dormant conditional escape. The function classifier still needs attributed/outer-macro/impl-nearest-brace cases before claiming that nearest brace denotes a genuine function block.

## Deletion and harness details

physically-delete-manifest actually writes then rmSyncs Cargo.toml and asserts it absent via glob; live graph excludes it, native direct rustc still succeeds, and the gate must refuse missing manifest authority. This is genuine physical filesystem absence, not report filtering.

physically-delete-parent-mount instantiates a root whose authored bytes are already `fn main(){}` while retaining orphan leaf. This is a legitimate absent-mount snapshot and should refuse; it does not itself execute a prior→deleted mutation. The accepted sibling row supplies the before state, so describe it as physical current-source mount absence unless a paired rewrite is added. No runtime sealed-output expectation is asserted for this row, appropriate because root never calls leaf.

The native harness unconditionally launches the binary after comparing row.native. All current rows are native=true, so no contradiction now; if any native=false owner row is added, guard binary execution or it will fail by missing executable rather than assert the intended refusal. Caught source scanning errors become [] only in temporary graph evidence preparation; the whole gate is subsequently required to report the scanner error. Do not reuse that temporary catch as production success behavior.

The test checks complete report.problems/violations and preserves >=2 reference count on unresolved scope. That is substantive scope closure evidence, not empty/fullpass. No runtime passing claim is made by this audit.
