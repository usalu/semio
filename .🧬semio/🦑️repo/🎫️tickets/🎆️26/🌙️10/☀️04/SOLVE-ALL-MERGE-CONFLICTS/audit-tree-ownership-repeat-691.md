# Immutable Tree and Ownership Exploration — Repeat 691

Read-only inspection of base `1bae6115bc56bf5e1b01212774dd76877a5f64fe`, HEAD `36fbe9da786b71ec7b6946a1fc1cbb4e9d4d25ec`, stash `a09b7afdff001185403f0decb7cb0714a7bb839e`. Root AGENTS.md was read. Initial tracked working tree was clean. No source edits, modifying Git operations, builds, tests, compiler, runtime or dependency resolution were performed. Report/inventory files are the only writes.

## Immutable Inventory

| Measure | Count |
| --- | ---: |
| Base tracked tree paths | 161543 |
| HEAD tracked tree paths | 162636 |
| Stash tracked tree paths | 165026 |
| HEAD delta paths | 3114 |
| HEAD additions / deletions / modifications | 1129 / 36 / 1949 |
| Stash delta paths | 8846 |
| Stash additions / deletions / modifications | 3744 / 261 / 4841 |
| Paths changed on both sides | 192 |
| HEAD-only delta paths | 2922 |
| Stash-only delta paths | 8654 |
| Overlap modification/modification | 186 |
| Overlap HEAD deletion/stash modification | 6 |
| Overlap with identical final content | 0 |

All comparisons disable rename detection, so dissolution and destination additions stay explicit. Stash first parent is exactly the supplied base; its second parent is `312eeccddc651ff01fab8ddc6759e617438e6861`; no third untracked parent exists. Full line-oriented immutable delta inventories and overlap matrix are under `🗑️generated/{head-delta,stash-delta,overlap}-repeat-691.txt` for root integration, to remove on ticket completion.

## High-Risk Dissolution

The six delete/modify paths all have HEAD=D / stash=M:

- `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs`
- Same command-routes staging owner, `🤝️group/🦀️.rs`.
- Same command-routes staging owner, `🚪️ingress/🦀️.rs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs`.

HEAD adds instance dispatch ownership under `📨️dispatch/📥️staging/` (typed-command, producers, host-events, emit, group-history, bundle), instance render ownership under `🖼️render/📥️staging/` (reserved-jobs, history-view, tool-machine, interaction, bundle), and associated dispatch crate manifest/project. It also adds `📥️staging/🏗️ctx-builders` and command-route archive-merge/actions owners while deleting older lease/dispatch/ingress/group owners. Therefore path-level deletion cannot mean loss of behavior by assumption, and restoring stash files at old owners cannot be the default. Framework lane must trace each stash-edited symbol into HEAD owners and preserve substantive changes there.

HEAD root build-owners metadata adds host effects, engine, guest-laws, guest-runtime, imports, operation, post-turn-relay, process-transport, routers, shard-executor, shard-laws and shard owners. Global manifest/lock/build-owners preservation must follow these HEAD ownership additions when integrating stash metadata.

## Partition and Unmapped Ownership

HEAD delta top owners: framework 1563; ✏️s 920; ticket/repository metadata 577; Hub 35; semio-tech 4; mit-bestand 3; root tests 3; Storybook 2; launch 2; teaching 1; build-owners/script/Cargo manifest/Cargo lock one each.

- High domain: `✏️s/`, teaching, Course and Proctor. Preserve HEAD-only files before applying the stash subset.
- High framework: framework and Hub, excluding Course/Proctor and global Cargo/locks. Own all six delete/modify symbol dispositions.
- Root/global: Cargo manifest/lock, launch/seed, root script, build-owners, and metadata integration.
- Explicitly assign to root/global the otherwise unmapped seven overlap files: root diff-only-law-gate source and its lanes source, Cargo.toml, Cargo.lock, launch.json, launch.seed.jsonc and root script.
- Also assign these HEAD-only outliers to root/global: `.storybook/📖️stories/🧭️coordination/🟦️.ts`; `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts`; root outcome-law-gate; three mit-bestand files (presentation/demonstrator Vite builders and demonstrator engine-producers test); four semio-tech Play files (Vite builder, site script, publication test, release-job-order test); build-owners metadata.
- Ticket/repository history must remain a separate preservation lane rather than being overwritten by an unrelated domain bulk operation.

## Preservation Assertions for Integration

1. Every one of 2922 HEAD-only delta paths retains its HEAD blob/deletion unless a documented source-owner relocation requires a deliberate merge.
2. Every one of 8654 stash-only paths retains stash blob/deletion, subject to explicit dissolved-owner disposition; deletion conflicts with newly introduced owning modules need review even without path overlap.
3. All 192 overlapping paths require deliberate review because none have identical final content. No whole-side selection can establish preservation globally.
4. All six delete/modify cases require a symbol-level old-owner-to-current-owner ledger, including test assertions, rather than revival of deleted module ownership.
5. Root metadata keeps newly added HEAD host/crate/build-owner rows and integrates stash rows; lock identities, launch entries and package ownership remain aligned with actual owner manifests.
6. Outlier source/test files listed above are assigned before any broad restore/apply operation; ticket content is never silently treated as generated output.
7. Final static inventory compares both immutable deltas and records each intentional exception. This exploration does not assert compilation, runtime behavior, syntax correctness or semantic completeness.
