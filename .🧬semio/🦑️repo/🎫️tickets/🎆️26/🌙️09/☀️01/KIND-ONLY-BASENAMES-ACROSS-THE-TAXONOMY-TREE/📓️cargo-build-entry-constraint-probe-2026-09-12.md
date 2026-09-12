# Cargo Build Entry Native Constraint Probe

The coordinator inspected the two currently visible build.rs sources on 2026-09-12 and ran three isolated dependency-free Cargo controls. This is evidence for the package-metadata execution lane, not a source change or global acceptance.

## Native Result

Cargo 1.99.0-nightly (2f0e7011e 2026-07-05) consumed an explicit package.build entry for each private standalone workspace. The controls used the same one-function build hook and isolated target directories; offline compilation needed no dependencies. Raw output is under 🗑️generated/coordinator/cargo-build-basename-probe/result.json.

| Explicit build leaf | Exit | Native observation |
| --- | ---: | --- |
| 🦀️.rs | 101 | Cargo derives build_script_🦀️, and rustc rejects the crab and variation-selector characters in the crate name. |
| _.rs | 0 | The build hook executed and emitted its [DEBUG] native execution warning. |
| build.rs | 0 | The same build hook executed and emitted its warning. |

The native restriction concerns the derived crate identifier; it does not require the exact spelling build.rs. The catalog currently calls **/build.rs unconfigurable, and discovery has several exact build.rs branches. Those contracts need an evidence-based review. An anonymous ASCII leaf under a semantic owner is demonstrably possible with the installed native toolchain. Cross-platform fixture coverage and current Cargo producer/manifest authority still need execution before applying that choice to repository packages. Do not add a broad ASCII/source exception or claim this tiny compile validates the real renderer.

## Actual Source Ownership

The renderer package at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust has an explicit build = "build.rs" manifest. Its build.rs is a seven-line documented include of ../../🏗️builder/🦀️.rs; the substantive catalog projection already lives at that semantic builder owner. The comment correctly identifies Cargo's emoji crate-name failure, but its inference that this exact filename must remain plain is narrower than the demonstrated configurable anonymous alternative. Preserve the six-output WGPU generation lane and current package-authority fixtures when closing this entry binding; it is not permission to reintroduce a duplicate canonical source.

The actual Infinite package is under 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite, not the plugin tree. Its build.rs contains 134 text lines (135 newline segments, 5,775 bytes) of authored shortcode projection, icon-catalog traversal and output staging. It parses the generated shortcode catalog, validates stable identities, writes emoji/catalog/metabolism match arms and copies actual SVG referents. That behavior still needs implementation-neutral semantic ownership even if native entry metadata remains a tiny dispatch boundary. Its manifest also explicitly selects build.rs. The module's AGENTS.md only states its Infinite infrastructure purpose and adds no narrower rule.

The Infinite projection emits 🔎️shortcodes.rs into OUT_DIR and reads exact UI asset paths. Preserve public IDs/case, deduplicated source traversal, source/output bytes and include consumers. Decide persistent versus disposable native output through the existing producer contract; do not rename compiler-internal outputs merely to lower a source census. The current source first records the authored implementation ownership defect, separately from emitted-output admission.

## Required Follow-Up

Close the Cargo entry contract in the metadata lane with schema/portable fixtures and a native compiler oracle: configured anonymous source, default/disabled/malformed build selections, manifest-scoped binding and out-of-owner attempts. Rebase actual manifest, discovery/normalization, package identity and WGPU source-authority consumers together. Extract Infinite's shortcode concern and prove its real catalog behavior with private outputs. Preserve concurrent source work and avoid full renderer rebuild claims when only the small native fixture has executed.

## Retained Probe Input

The standalone manifest/source templates, invocation and expected native outcomes are retained in 🧪️cargo-build-entry-controls/🔣️.json at the ticket root. Keep that authored input when deleting the disposable compiled workspaces under generated. It records the tested compiler version; later toolchain behavior must be rerun rather than assumed.
