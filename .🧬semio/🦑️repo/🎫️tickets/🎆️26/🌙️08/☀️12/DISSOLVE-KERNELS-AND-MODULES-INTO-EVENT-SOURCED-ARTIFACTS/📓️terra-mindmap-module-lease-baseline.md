# Mindmap Module Semantic Refactor Lease Baseline

## Lease

- Owner: `✏️s/🔨️modules/💭️mindmap/**`.
- Governing goal: `🎯aioptimizedrepo/🎯singlefilerepo`.
- Ticket: `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`.
- Scope: active Rust component, package manifest, local glue/script/project registrars, and any local tests or fixtures beneath the owner only.
- Excluded: root workspace registrars and all non-owner consumer files. Required changes to those paths are recorded here rather than made.

## Instruction Baseline

Read completely before audit: repository `AGENTS.md`, `✏️s/AGENTS.md`, `✏️s/🔨️modules/AGENTS.md`, and `✏️s/🔨️modules/💭️mindmap/AGENTS.md`.

The ticket amendment binds the structural decision: an active semantic component has one maximally specific `<collection>/<specific>/component.<language>` identity. A module is retained only for at least two independent terminal production components at their lowest common semantic owner; glue, test-only consumers, language mirrors, and call sites inside one component do not qualify.

## Dirty-State Quarantine

The owner was clean in both index and worktree at baseline. Its six tracked files had no staged or unstaged diff. The workspace had unrelated dirty paths, including the root prompt, framework kernel/machine/platform, OS renderer sources, and repo-library TypeScript exports; none are writable by this lease.

## Content Hashes

| Path | SHA-256 |
| --- | --- |
| `AGENTS.md` | `2866278919bad0f723ae94b18d0f0ca1807aea4701ca3c0a6202324fa43dcdb2` |
| `📦️packages/🦀️rust/Cargo.toml` | `f88cfa2e9e8d4ab52dd867ee21deaddf6c05216b21b7cc9e2532dab67c245e62` |
| `📦️packages/🦀️rust/📋️project.json` | `5b82680e1e7f8cb061e918762cb9d4f24659d2ea73f3ff3134aac07ebf4f631f` |
| `📦️packages/🦀️rust/📜️script.ts` | `f3cc7299aeca5289826595dcd1da1681989d6d38e70ac95f206080d5f9280bfc` |
| `📦️packages/🦀️rust/📦️glue.rs` | `9018c81a18f622e5aa3b74ac317de20c1e315dcc9ffcdce95680907b34f6fea9` |
| `🧩️extension/🦀️component.rs` | `2a7d64131888c73fabd69f78f420d0d434f7339a3a08593eb6cadae0c9c0616c` |

## First Measured Consumer Candidates

- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml` declares `semio-s-mindmap`; its WIRES inspection component imports `semio_s_mindmap` and uses `MindmapExtension`, `DefaultMindmapExtension`, and `TopicId`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` declares `semio-s-mindmap`; its 2D engine component re-exports `semio_s_mindmap as mindmap`.
- Root `Cargo.toml` owns the workspace-member and workspace-dependency registrations; it is outside the lease and must be changed only by its registrar owner if a relocation is justified.

## Pending Decision

The active owner initially contains one authored terminal production component, `🧩️extension/🦀️component.rs`. The final report will prove whether its semantic responsibility belongs in a non-module owner and enumerate every required consumer/registrar edit before any path is moved.

## What Changed

The component census proved that the former mindmap package has only one real terminal consumer: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/📌️panels/🔍️inspection/🦀️component.rs`. Its `DefaultWiresExtension` constructs, hydrates, displays, and validates every use of `DefaultMindmapExtension`, `MindmapExtension`, and `TopicId`.

`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/⚙️engine/🦀️component.rs` had only `pub use semio_s_mindmap as mindmap;`. A whole-repository read found no use of that forwarding path or of `mindmap::` in the puzzle plugin. It was a dead forwarding dependency, not a second terminal semantic component.

The WIRES inspector now owns its cohesive private graph working representation directly: `TopicId`, the topic map, local topic lookup, and the canvas-board graph extension implementation. `WiresExtension` owns the topic-label contract alongside its existing WIRES-specific relationship and fixed-identity validation contract. No persisted shape, mutation, diff, or event-sourcing contract changed: this is an ephemeral fixture-hydration representation in the inspector.

The puzzle forward re-export and both direct `semio-s-mindmap` Cargo dependencies were removed. The added `topic_lookup_stays_local_to_the_wires_extension` test preserves the moved lookup behavior.

The retired module code remains intentionally intact pending the root registrar change. A dangling Cargo workspace member would make the shared build fail, and the lease does not authorize root registrar edits.

## Files Touched

- Updated: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/📌️panels/🔍️inspection/🦀️component.rs`.
- Updated: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml`.
- Updated: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/⚙️engine/🦀️component.rs`.
- Updated: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`.
- Created: this baseline/report and `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🔧️patches/terra-mindmap-registrar-request.patch.md`.
- Not removed: `/Users/ueli/Documents/semio/✏️s/🔨️modules/💭️mindmap/**`, pending its root registrar update. `/Users/ueli/Documents/semio/✏️s/🔨️modules/💭️mindmap/AGENTS.md` is not mutable under the repository instructions.

## Consumer And Mount Evidence

- The former package mount resolves from `/Users/ueli/Documents/semio/✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust/📦️glue.rs` to `/Users/ueli/Documents/semio/✏️s/🔨️modules/💭️mindmap/🧩️extension/🦀️component.rs`; it is active, not a candidate for blind deletion.
- The WIRES inspector is actively mounted by `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs:465-466`.
- The puzzle engine is actively mounted by `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs:1755-1757`; its removed mindmap re-export had zero downstream consumers.
- Post-edit scan: no live `semio_s_mindmap` import remains. `semio-s-mindmap` remains only in the retired owner, root workspace dependency, root lockfile, and historical prose. No live `MindmapExtension`, `DefaultMindmapExtension`, or `RelationshipId` consumer remains outside the retired owner.
- The `🔨️modules/💭️mindmap` location therefore fails the ticket's at-least-two-independent-terminal-component rule. The unique specific home is the existing WIRES inspection component, not a new generic or compatibility module.

## Verification Commands Run, With Real Output

```text
$ RUSTC_WRAPPER='' CARGO_TARGET_DIR=<ticket>/🎯️target bun x nx run @semio-tech/mindmap-rs:test-quick
Finished `test` profile [unoptimized] target(s) in 1.05s
Nextest: 0 tests run, 0 passed, 0 skipped
NX Successfully ran target test-quick for project @semio-tech/mindmap-rs

$ RUSTC_WRAPPER='' CARGO_TARGET_DIR=<ticket>/🎯️target bun x nx run @semio-tech/mindmap-rs:test --skip-nx-cache
Finished `test` profile [unoptimized] target(s) in 0.36s
Nextest: 0 tests run, 0 passed, 0 skipped
NX Successfully ran target test for project @semio-tech/mindmap-rs

$ git diff --check -- <four changed consumer paths>
exit 0
```

The old mindmap package has no executable tests. Its test targets compiled successfully before dissolution and reported zero discovered tests. The WIRES test that covers the moved topic lookup was added but cannot yet execute because both consumer packages compile `semio-s-plugin-stdio` first.

```text
$ RUSTC_WRAPPER='' CARGO_TARGET_DIR=<ticket>/🎯️target bun x nx run @semio-tech/reasoning-mindmap-plugin:test --skip-nx-cache
error: couldn't read `.../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/.../💡️inferences/📐️geometry/🦀️component.rs`: No such file or directory
--> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:2247:33
NX Running target test for project @semio-tech/reasoning-mindmap-plugin failed

$ RUSTC_WRAPPER='' CARGO_TARGET_DIR=<ticket>/🎯️target bun x nx run @semio-tech/puzzle-plugin:test --skip-nx-cache
error: couldn't read `.../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/.../💡️inferences/📐️geometry/🦀️component.rs`: No such file or directory
--> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:2247:33
NX Running target test for project @semio-tech/puzzle-plugin failed
```

## sharedFileRequests

| File | Required change | Reason | Patch request |
| --- | --- | --- | --- |
| `/Users/ueli/Documents/semio/Cargo.toml` | Remove the mindmap workspace-member entry and `[workspace.dependencies].semio-s-mindmap` entry. | The consumer migration makes the package unreachable; the member must disappear before owner code is removed. | `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🔧️patches/terra-mindmap-registrar-request.patch.md` |
| `/Users/ueli/Documents/semio/Cargo.lock` | Regenerate through the authorized root workflow after the manifest update if Cargo removes the retired package record. | The lockfile currently retains `semio-s-mindmap`. | Same request file |
| `/Users/ueli/Documents/semio/✏️s/🔨️modules/💭️mindmap/**` except `AGENTS.md` | Delete the retired component and package files after the root update. | Keeping the package after the one-consumer collapse would preserve an invalid module. | Same request file |

## Concurrent-Churn Observations

Both post-edit consumer test targets stop in the same unrelated stdio mount before compiling their target crates. The target file is absent on disk, while `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:2247` still mounts it. Disk has 24 GiB free, so this is not a storage failure. The root coordinator reports that the glTF mount relocation is in progress; no stdio file was read for modification or changed by this lease.

The package-test commands caused two generated lockfile dependency-list deletions for `semio-s-mindmap` in `/Users/ueli/Documents/semio/Cargo.lock`. This lease did not edit that root file and will make no further Cargo/Nx invocation until the central coordinator reconciles the lockfile with the root registrar update.

## Honest Pass/Fail

- Pass: semantic classification, atomic direct-consumer migration, dead puzzle dependency removal, mount/referrer audit, old package scoped Nx tests, and diff hygiene.
- Blocked-churn: execution of the changed reasoning and puzzle package tests. Both are blocked before target compilation by the same external stdio missing mount.
- Pending registrar: root workspace and lockfile update, then explicit coordinator confirmation before retired owner code is deleted. No alias, adapter, migration, or compatibility forwarding was introduced.

## Paused/Releasable Checkpoint

The direct-consumer implementation and registrar request are ready for the central checkpoint. Do not remove any retired mindmap file until the root member/dependency deletion is visibly complete. Reactivation validation is: confirm the two root `Cargo.toml` entries are absent, inspect the coordinator-owned `Cargo.lock` reconciliation, delete the five requested retired code files while retaining `AGENTS.md`, re-run the complete live-referrer scan, and run the reasoning and puzzle Nx tests after the unrelated stdio glTF mount is repaired.
