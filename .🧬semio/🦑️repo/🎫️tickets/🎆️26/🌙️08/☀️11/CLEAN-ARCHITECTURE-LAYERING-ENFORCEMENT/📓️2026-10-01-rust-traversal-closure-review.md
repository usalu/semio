# Rust Traversal Closure Review — 2026-10-01

Read-only review of current raw-inline traversal implementation and four language-neutral native fixture cases. No production edits, compiler launches, or independent runtime execution by reviewer. Root reported genuine prepatch 12-pass/1-fail/265-assertion RED; final green evidence is pending and not claimed here.

## Verdict and Bounded Blockers

Raw physical prefix retention closes the previously identified `missing/..`/ignored-symlink-parent blind spot for scanner-generated inline anchors and direct literal includes. Required directory order is retained before parent navigation, and census checks those physical inputs before trusting canonical leaf. The four fixtures include two independent compiler failures plus two actual compiled binary outputs distinguishing specific source/text from neutral decoys. This is stronger than a compiler-success-only check.

Two bounded follow-ups remain in inspected live bytes: **root-directory `.` target is generated but rejected by input census**, and **optional directories/default-empty metadata must be made required** as Root intends. Public inlineBase coupling/portable raw form must be validated before generated-reference skipping or resolution. These do not require any broad producer change or legacy optional fallback.

## Traversal Walk

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:6527` now retains raw nested anchor components using interpolation rather than posix.normalize. This preserves a `missing/..` or `node_modules/..` path component all the way to physical traversal.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:43,47-59` builds raw base plus literal path and walks segments in order. Every named component preceding another raw segment is recorded as a required directory before `..` pops the current path. With `general/missing/../leaf.rs`, traversal retains `general`, `general/missing`, then final `general/leaf.rs`. With nested explicit/raw anchor and a default nested module, the intermediate default name is also retained. `.` and repeated separators do not introduce a fictitious named component, and an attempted parent pop at empty root rejects escape. Deduplicating directories preserves the first visit order; repeated same lexical physical directory after parent navigation does not require a second no-follow read during the same census.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:37-46` lstats each retained directory before final input. Missing prefix stops with missing-input; symlink prefix stops linked-input; regular file prefix maps unexpected-input-kind to non-directory-ancestor. Canonical final .rs still must be censused. Final file paths ending `file/.` or `file/..` retain file as a required directory and therefore fail before the normalized leaf can hide wrong node kind. No-follow checks remain correct when a prefix ancestor itself is the invalid component.

Cancellation checks happen during each directory inspect and each ancestry iteration. Existing per-batch node/absent caching avoids repeated lstat reads across required prefixes. Every required prefix is canonical at the point it is stored; raw parent navigation is never fed directly to join/lstat.

## Root Directory Sentinel — Concrete Current Mismatch

Resolver `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:59` emits `to: "."` when a valid directory mount resolves to the workspace root. Example root source `source.rs`:

```rust
#[path="."] mod anchored { #[path="leaf.rs"] pub mod leaf; }
```

Its first directory reference has target `.`, which `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:18` rejects as noncanonical. The same applies to a nested owner mount `#[path=".."]` resolving exactly to root, even though it does not escape. Current accepted corpus cases anchor beneath `general`, so none catches this sentinel path. Add one genuine root anchor accepted vector and a root-directory node-kind/symlink rejection pair.

Minimal handling: accept **exact `.` only for directory=true**, lstat the supplied root itself, reject root symlink or file through existing typed node-kind rules, and treat the root as a first-class directory input. Do not generally permit `.` segments in canonical target strings; do not allow a file include to treat root as a file. Pure rustSourceTargetProblem also needs deliberate root representation rather than considering root inventory impossible. This closes required directory authority without granting Cargo package ownership to virtual root manifests.

## Required Target Metadata and Public Reference Coupling

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:8` still declares `directories?: readonly string[]`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:39` still uses `directories ?? []`. Make directories a required target field and update every direct synthetic caller with an explicit owned directory inventory. Production resolver always emits it, so optional behavior is unnecessary. Keep no compatibility default. For tests intentionally checking just a direct canonical leaf, explicit [] is clear; tests claiming raw traversal must use the actual resolver's retained prefixes.

Minimal inlineBase validation contract should be checked **before** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:34` skips generated references:

- If inlineBase is present, reference.kind must be path; modulePath must be a nonempty array of valid lexical module names; reference.base must be absent.
- inlineBase must be a nonempty string, portable and relative: reject absolute POSIX form, drive prefix, backslash, NUL/control corruption. Keep legitimate raw `.`/`..`/repeated-separator structure because physical traversal must see it; do not normalize it as validation.
- Do not let manually supplied inlineBase on include/include_str/include_bytes, generated base, manifest base, or empty modulePath be silently ignored. Reject inconsistent fact combinations rather than selecting an unrelated path origin.
- Enforce the same fact coupling in the language-neutral reference fixture schema. Existing schema's inlineBase:string alone is insufficient. Add closed rejection vectors for each incompatible base/kind/scope and nonportable form; distinguish API rejection from actual scanner derivation.

A caller can still fabricate semantically plausible inlineBase or directories in a low-level function call; the source scanner must remain the authority supplying these facts in production. Require a branded proof only if a genuinely untrusted serialized API will accept these facts; that is not necessary to make this local scanner's exported type coherent. Do not invent broad proof infrastructure just for trusted internal unit callers.

## Four New Native Traversal Fixtures

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts:167-192` loads closed traversal vectors, writes original sources plus declared files, creates declared symlink, invokes rustc on original source, and runs successful binaries to compare stdout. It independently compares native success status with each vector and checks deduplicated typed census failures. Four current vectors:

| Vector | Compiler/Runtime Expectation | Gate Failure |
|---|---|---|
| missing-inline-prefix-before-parent | compilation fails despite neutral leaf existing | missing-input general/missing |
| linked-inline-prefix-before-parent | binary outputs VALUE 222 from specific leaf, not neutral 111 | linked-input general/node_modules |
| missing-literal-prefix-before-parent | compilation fails despite neutral text existing | missing-input general/missing |
| linked-literal-prefix-before-parent | binary outputs specific-input, not neutral text | linked-input general/node_modules |

These are real source and binary behavior, not extracted fixture helpers or stand-ins. The current directory traversal metadata may produce repeated reports for the same required prefix from multiple references; deduplicating the test's identical failures is appropriate, but actual report counts should be described as reference-level input failures, not unique missing filesystem nodes.

## Scope Still Outside This Closure

Graph moduleBase derivation for unanchored provenance can still normalize paths inside inspectRustModuleGraph; the present closure proves source-derived **anchored** raw bases and literal include paths. Do not assert every context-derived raw path across all graph transitions is now physically preserved without equivalent prefix evidence. Missing/unsupported references still keep verification red but may suppress other diagnostics/counters in that same source, as recorded in prior audit.

Root/taxonomy/policy bootstrap symlink validation is independent of this traversal patch and unchanged. The root-directory sentinel fix should use actual root lstat, not turn raw-root symlink acceptance into an unrelated fallback. Existing include-file-origin and virtual Cargo-root nonauthority vectors must continue to hold.

No blocking issue found in the four newly supplied traversal shapes by source reasoning. Close root sentinel plus strict metadata contract, retain live native proof, then rerun the complete gate as warranted by new production changes.
