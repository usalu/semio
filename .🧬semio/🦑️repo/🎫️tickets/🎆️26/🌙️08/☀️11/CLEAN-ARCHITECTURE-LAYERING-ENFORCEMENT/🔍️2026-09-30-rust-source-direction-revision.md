# Rust Source Direction Revision

The shared Rust lexer now feeds an authored compile-input contract that distinguishes source-relative, manifest-relative, and generated OUT_DIR paths. Unknown compile expressions reject inspection instead of disappearing. Direct module path attributes and recursively nested cfg_attr path attributes count only when attached to a module declaration; inline directory mounts are explicitly marked and checked as authored directory dependencies; unrelated metadata and path attributes on nonmodules contribute no dependency.

The existing Rust module graph supplies module bases and manifest provenance. Conventional sibling mounts retain their sibling directory; explicit mounts retain their physical parent directory. Inline path overrides now use the actual mount base, including when an inline module lives inside a conventional sibling. Optional compile-reference inputs let the graph propagate crate ownership into literal and manifest-relative include! expansions without duplicating the lexer.

The edge matcher honors both source and destination pathNot exclusions. A manifest-relative include with no owner, or a nested module path with no module context, rejects verification. Generated OUT_DIR inputs are explicitly classified separately from authored file inputs.

## Conditional Inline Mounts

A late audit demonstrated that selecting only the first cfg_attr inline path override can miss another platform's owner, as well as the default no-attribute base. The scanner now explicitly rejects any conditional inline path override. The portable rejection fixture includes the exact shape `#[cfg_attr(unix,path="safe")] #[cfg_attr(windows,path="../../specific")] mod mounted { #[path="leaf.rs"] mod child; }`, and a single conditional override with its default alternative. This keeps the shared graph's singular module-target contract explicit and prevents source boundary verification from claiming success for an unproven conditional mount. A future multi-context cfg contract must model every applicable base and its absence alternative before this rejection can be removed.

## Conservative Scope

This policy checks authored source in every cfg configuration and macro_rules template, including unused templates. Its purpose is to prevent authored general code from naming removable specific owners. It does not claim to enumerate the exact set of macros rustc expands for one compilation. The portable fixture explicitly checks an unused literal include template: the authored contract records the edge while rustc dep-info omits it. Fixed-arity local macro_rules invocations with provable literal arguments are evaluated using the shared tokens. The fixture validates this output against rustc. Unsupported repetition, ambiguous macro names, unbound metavariables, and unknown macro path expressions fail closed. No template exemption is introduced.

## Test-Driven Evidence

Schema and portable fixtures were extended first. The initial uncached Nx run failed all four tests, demonstrating missing environment classification, module contexts, exclusions, and rejection of unsupported expressions. The first corrected run passed four tests with 69 assertions, including six successful rustc nested-module dep-info proofs.

The live scan exposed a shared delimiter-pairing defect: Object.prototype names such as constructor were treated as opening delimiters. rustTokenPairs now checks own delimiter keys. A portable fixture compiles constructor and toString identifiers and compares its literal include against rustc dep-info.

A direct rustc proof under ticket generated output confirmed that an included mounted/source.rs inline module resolves its leaf to mounted/nested/leaf.rs. Further fixtures now cover this include ownership and a manifest-relative include inside an included source.

## Final Live Inventory

Executed final `bun nx run @semio-tech/repo-lib:lint-rust-source-direction --skip-nx-cache --outputStyle=stream` after framework_execution confirmed that the schema-owned neutral catalog descriptor witness/helper and catalog Rust source edits had settled. The final combined source gate inventoried 2,183 authored Rust files and 4,033 resolved authored references, finding zero strict layering edges. The command exited 1 after 36.5 seconds with caching explicitly skipped. Scanner code was unchanged during this final recheck. It exited nonzero because one source file contains unsupported emitted quote syntax: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:595`, including `include_str!(#taxonomy_dependency)` inside a proc-macro `quote!` output template. The scanner does not silently omit this source or report the live gate as passing. Four such symbolic include expressions occur in this emitted template source. Its dynamic consumer provenance requires a separate semantic code-generation contract; no filename-specific or blanket quote-macro exemption was added.

The final combined uncached Nx command `bun nx run-many -p @semio-tech/repo-lib -t test-rust-source-direction lint-rust-source-direction --skip-nx-cache --parallel=2` ran the portable test target successfully and failed only the live lint target for the explicitly unsupported quote input. The final streamed standalone command `bun nx run @semio-tech/repo-lib:test-rust-source-direction --skip-nx-cache --outputStyle=stream` exited zero with four passing tests, zero failures, and 91 observed assertions. The fixture corpus now contains twelve compiler source cases, seven contextual mount cases, seven direction cases, and eight explicit rejection cases. The nineteen valid source/mount cases compile through rustc dep-info, including literal macro parameters, current-directory and inline directory mounts, include ownership, and manifest-derived inputs. `OUT_DIR` paths containing traversal are rejected.

Inline directory matching checks both the normalized owner path and its trailing-slash selector form, so existing owner rules such as `^specific/` still reject a mount at the owner root `specific`. The valid empty `#[path=""]` mount retains its exact identity while resolving to the current directory. Dep-info parsing handles escaped spaces and normalizes platform separators for the portable compiler oracle.

The actual source inventory and resolved reference totals are runtime console evidence; the full gate remains red until emitted dependency provenance is proven. The final combined-source recheck introduced no new unsupported source category and found no source boundary regression. Scanner-owned generated proof output was removed earlier; this recheck wrote no retained scanner output and preserved framework_execution native/exact target caches. Root coordination owns ticket closure and shared generated-directory cleanup.


## Files Changed

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️rust-source-direction/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️rust-source-direction/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔍️2026-09-30-rust-source-direction-revision.md`
