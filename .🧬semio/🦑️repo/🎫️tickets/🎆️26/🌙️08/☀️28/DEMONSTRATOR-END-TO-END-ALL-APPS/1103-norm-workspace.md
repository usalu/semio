# EN1991 Workspace Registration — 2026-11-03

The focused Hub Cargo check passed Source admission after dependency synchronization, then failed before Rust compilation because the EN1991 mutations crate inherited an undeclared workspace dependency. Audit of all 98 declared EN1991 workspace members found exactly seven missing inherited registrations, all consumed by the mutations crate: dust-explosions, rail-load-groups, rail-derailments, snow-guards, local-fires, support-collisions and fatigue-models. Each target already exists as a canonical member with the expected package identity and explicit EN1991 workspace owner.

Added exactly seven `workspace.dependencies` name/path tables to `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/Cargo.toml`. No aliases, member code, lock file, Source guard or broader Norm behavior changed.

The retained language-neutral contract is `1103-norm-workspace.json`; `1103-norm-workspace.mjs` checks actual inherited dependency coverage, declared workspace ownership, exact canonical member/path pairs and every path dependency package identity. It compares the same normalized output from Bun.TOML and third-party smol-toml. Before the edit it exited 1 with exactly seven missing entries and no wrong identities. After the edit it exits 0: 98 members, no missing inherited dependencies, no invalid package identities, all seven fixture registrations exact. `repo tools exec --light git -- diff --check` also exits 0.

Validation command: `repo tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1103-norm-workspace.mjs`.

This is a targeted manifest regression repair. Native Cargo metadata/compile remains for the Flow lane to rerun, including any canonical Cargo lock refresh. No Cargo, dependency lock or sync was launched in this lane. Source edits and report are complete.

## EN1993 Follow-up and Full Norm Audit

The next native Cargo run passed EN1991 registration and refused nonexistent EN1993 inherited `*-more` dependencies. Expanded the retained TOML contract/audit to all 15 Norm artifact workspace owners, totaling 935 declared members. A separate read-only agent independently inspected all 950 physical Cargo manifests, including untracked files, confirming no package identities exist for these five dependencies anywhere in Norm artifacts. Existing path registrations in all owners resolve to the correct named packages.

The five affected mutation parents are `🪜buckling-lengths`, `🪛fastened-joints`, `🥫forming-gains`, `🧪material-ductility` and `⏳fatigue-stresses` under EN1993 mutations. Each falsely declared its own nonexistent `-more` crate and forwarded test-support to it, while no Rust source references those five crate identifiers. Their actual mutation leaf modules are already mounted locally. Each also contained a copied `pub use` from the real `member-warping-more` crate without declaring it. That real crate exports only five frame action/basis modules; copied fatigue/buckling lists additionally requested names it does not export and duplicated locally mounted mutation modules.

After the expanded regression failed with exactly five missing inherited keys, removed the five nonexistent dependency declarations and their test-support edges, and removed the five invalid copied cross-domain pub-use lines. This touches exactly five member Cargo manifests and five parent Rust module files; no crate names, workspace members, real implementation modules, tests or valid feature forwarding were removed.

The neutral fixture records all original local module identities: buckling-lengths 11, fastened-joints 22, forming-gains 8, material-ductility 5, fatigue-stresses 11 (57 total). The regression verifies these exact mounted identities remain, every source mount resolves, and public crate reexports have declared dependencies. Both Bun.TOML and third-party smol-toml agree on all 15 workspace/member dependency projections. Final result exits 0: 935 members, no missing inherited keys, no path/name mismatches; diff --check exits 0 after removing only trailing whitespace left by the deleted export lines.

The Source batch and retained test inputs are stable. Native Hub Cargo metadata/compile and any canonical Cargo.lock refresh remain with the Flow lane; this lane launched no Cargo or dependency sync.
