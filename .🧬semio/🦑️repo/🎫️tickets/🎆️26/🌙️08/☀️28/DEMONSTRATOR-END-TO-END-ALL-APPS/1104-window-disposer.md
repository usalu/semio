# Puzzle Window Disposer Birth — 2026-11-04

The actual runtime task-resume test failed with UnsupportedOwner for the production Puzzle3dWindowConfigOwner. Its bounded store disposer factory existed, but its owner implementation omitted store_disposer_birth, leaving the required allocation birth unsupported.

Added the explicit production declaration delegating to bounded_window_config_store_disposer_birth::<Self>(). The helper quotes the actual ArtifactDocumentStoreDisposer layout with depth one; no generic/no-op/default disposer, guard change, budget increase or dependency was introduced.

Added a focused Rust unit law alongside the existing Puzzle window tests and a language-neutral disposer-birth.json fixture. The law asks the actual production owner for its birth demand, constructs the actual bounded disposer, and compares the quote capacity to std::alloc::Layout::for_value of that concrete disposer behind its trait object. Existing serde_json parses the neutral depth/copy/release expectations (1/0/0). This checks physical layout independently of repeating the helper formula.

The existing native UnsupportedOwner failure is the actual red evidence. A source check also confirmed the required production declaration was absent before the edit. The new Rust law has been authored but not executed in this lane; Runtime owns the single native r3 rebuild covering its genesis/Deflate changes and this production owner correction. diff --check passes. No Cargo, dependency lock or sync was launched here.

Stable Source paths: Puzzle3d editor/🪟️window/🦀️.rs, its 🧪️tests/🔬️unit/🦀️.rs and disposer-birth.json. Light independently audits other reachable owners; this patch does not sweep unrelated bounded factories or change the trait default.

## Demo Owner Closure Batch

After Puzzle's actual native r3 rebuild compiled and passed the prior UnsupportedOwner refusal, added the same explicit bounded birth to six demonstrated production Source files: CAD shared window-config macro (four concrete CAD pane owners), FEM3d model and results owners, GIS map editor owner, Sourcing curation Grid owner and Flow main owner. This adds nine concrete declarations through six owner implementations. No unrelated Writer/WFC/Generation2d owners or trait default were modified.

Before these edits, the retained neutral `1104-window-disposer.json` contract and `1104-window-disposer.mjs` conformance failed with six bounded factories and zero explicit births. After the exact six insertions, it passes. The final conformance also covers the already-paired Puzzle3d and Energy editor/viewer owners: nine Source implementations, twelve concrete owners, every bounded factory paired to the exact matching bounded birth helper. Ajv checks the neutral contract. This is Source conformance evidence; semantic native execution remains a separate validation.

Added four native physical-layout laws where existing test seams exist (GIS map editor, FEM3d model/results, Sourcing grid). Their shared filter is `bounded_store_disposer_birth_matches_layout`; each parses its adjacent neutral disposer-birth.json via existing serde_json and compares its actual owner quote to the real trait-object disposer layout, depth one and zero copy/release demand. No test/runtime/package dependencies were added. Flow and CAD receive the production declaration and shared conformance coverage without new test harness plumbing.

A physical recursive Rust Source scan (including untracked files) checks the actual Generation3d and Process trees for WindowConfigOwner/build_store_disposer omissions; Generation2d is a separate application and was excluded. The narrowed demo owner batch is stable and handed directly to Runtime for its next coordinated native seal-fix rebuild. diff --check passes. No Cargo or dependency operation was run in this lane.
