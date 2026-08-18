# 📓️ lease-request — P7-headless-workspace → P2-catalog `capability_from_contribution` typing break

**From**: terra, packet `P7-headless-workspace`
**To**: sol / whoever owns `🌉️mcp/🗂️catalog/🦀️component.rs` (P2-catalog, contested — NOT my `path_scope`)
**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️component.rs`

## What's broken (found while running `cargo check -p semio-framework-os-mcp --lib` for my own
packet — this blocks the WHOLE crate, not just `🏠️workspace`)

`ContributionSet.inference_services`/`mutation_services`/`io_entries`/`composer_entries` used to be
`Vec<DescriptorEntry>` (an untyped placeholder — the peer ticket's own status.md notes this was
`E1-describe`'s honest restraint: "leaving `menus`/`themes` untyped because nothing in the codebase
declares them yet"). Since then the peer ticket's `E1-describe` packet landed the REAL typed shapes:
`inference_services: Vec<ContributedInferenceMetadata>`, `mutation_services:
Vec<ContributedMutationMetadata>`, `io_entries: Vec<IoEntryDescriptor>`, `composer_entries:
Vec<ComposerEntryDescriptor>` (`🛂️manifest/🦀️component.rs` `ContributionSet` struct, confirmed via
`git log --date=iso` — `manifest/🦀️component.rs`'s last commit is AFTER `🗂️catalog/🦀️component.rs`'s
own last commit, so catalog's call sites simply haven't caught up yet).

`capability_from_contribution(plugin_id, category, entry: &manifest::DescriptorEntry, kind)` (line
364) is called 4 times (lines 615/618/621/624) with `entry` now typed as `&ContributedInferenceMetadata`
/ `&ContributedMutationMetadata` / `&IoEntryDescriptor` / `&ComposerEntryDescriptor` respectively —
none of which has a `DescriptorEntry`-shaped `.id: String` field (verified: none of the four types
declares an `id` field at all). 4× `error[E0308]: mismatched types`.

## The exact fix needed (verified field shapes, not guessed)

`capability_from_contribution` needs either (a) 4 separate typed callers, or (b) a small trait/closure
that extracts `(id: String, source_id: String)` per type. Concretely, per type:
- `ContributedInferenceMetadata` has no bare id; a reasonable id is
  `format!("{}#{}", contributor, inference_schema)` (mirrors the type's own doc: mutation ids use
  `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` — inference's own natural key is
  `contributor` + `inference_schema`).
- `ContributedMutationMetadata.mutation_id: String` — already the exact id shape
  (`"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"`), use directly.
- `IoEntryDescriptor` has `owner: ArtifactDialect`, `counterpart: ArtifactDialect`, `direction:
  IoEntryDirection` — no bare id; `format!("{}->{}", owner.to_coordinate(), counterpart.to_coordinate())`
  (or the reverse per `direction`) is a reasonable natural key (`ArtifactDialect::to_coordinate()`
  already exists, used elsewhere in this same file's `OpeningResolver`-adjacent code).
- `ComposerEntryDescriptor` has `writes: ArtifactDialect`, `reads: Vec<ArtifactDialect>` — no bare id;
  `writes.to_coordinate()` is the natural key (a composer entry's identity is what it WRITES).

## Status

Pending. Not touched by this packet (outside `path_scope`, `🎬️actions`/`🗂️catalog` collision-matrix
rule). Blocks `cargo test -p semio-framework-os-mcp` for every packet, not just P7 — flagged as
high-priority. `📓️terra-P7-report.md` records the exact re-check timestamps this packet ran while
waiting, and states plainly that the acceptance `cargo test`/`cargo build` commands could not be run
clean until this lands (or until this packet's own deadline forced it to stop waiting).
