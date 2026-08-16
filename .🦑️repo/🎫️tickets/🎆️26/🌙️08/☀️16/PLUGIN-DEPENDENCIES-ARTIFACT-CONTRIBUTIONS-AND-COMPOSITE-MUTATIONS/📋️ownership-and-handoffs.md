# Ownership and Handoffs

Ticket: `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` · Goal `🎯aioptimizedrepo` · Issue https://github.com/usalu/semio/issues/2558
Start commit: `7ad8955884` (2026-08-15 23:56:01 +0200). Master plan: `📋️master-plan.md`. Frozen contract: `📋️contract-freeze.md`.

## Roles

- **Coordinator (Opus 5, main session):** ticket docs, contract freeze, lease allocation, serial gates at barriers, audits, remediation, `ticket_close`.
- **Executors (Sonnet 5):** one per lease. Write only inside the lease. Report into `📓️<lane>-report.md`. Never `ticket_close`, never `git commit/stash/checkout`, never worktrees.
- **Scouts/Auditors (Haiku 4.5, `Explore`):** read-only.

## Shared-tree rules (mandatory for every writer)

1. Other live sessions edit these same files — notably ticket `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` whose W0 leases name `🔌️plugin/🦀️component.rs`, `🏪️store/🦀️component.rs`, `🚪️io/🦀️component.rs`, and `📜️world.wit`.
2. Re-read the target region immediately before editing. Edit region-locally with `Edit`; never rewrite a whole file; never revert or reformat foreign changes.
3. Attribute failures with `git log --date=iso` against the start commit before blaming your own change. Commit-message dates are template text — never parse them.
4. Scratch/logs go in this ticket folder, `.txt` (not `.log`, which is gitignored).
5. No `git commit`/`stash`/`checkout`/worktrees. Auto-commit runs on this tree.

## Wave 0 leases (active)

| Lane | Model | Exclusive lease | Deliverable |
|---|---|---|---|
| **0-A protocol spine** | Sonnet 5 | `📡️spr/🎮️command/🦀️component.rs`; `🗣️dsl/✨️derive/🦀️component.rs`; `📡️spr/📜️history/🦀️component.rs`; plus one-line `origin:` fixups at `MutationMeta` literals in `📡️spr/{🔀️crdt,🔗️causal,🧪️testkit}` and `🏪️store/🦀️component.rs` | §1 of the contract freeze + law tests |
| **0-B channel spine** | Sonnet 5 | `📡️spr/🧵️channel/🦀️component.rs`; `🧰️framework/🛍️products/💻️os/🟦️component.ts` (`🔖️AppChannelCodec` + `🧪️Tests` regions only); new `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/**` | §2 + cross-language golden vectors |
| **0-C manifest spine** | Sonnet 5 | `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`; `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`; `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` (type declarations region only) | §3/§4 manifest types, `VersionReq`, dependency-graph pure functions + tests |
| **0-D WIT spine** | Sonnet 5 | `📜️world.wit`; guest bindgen glue regions `component`/`extension_component` in `🔌️plugin/🦀️component.rs`; host bindgen signature wiring in `🔌️plugin/🖥️host/🦀️component.rs`; `🌐plugin-web-materialize.ts` | §6 with stub exports so every plugin still builds |
| **0-I coordinator** | Opus 5 | ticket docs; `🔣️taxonomy.json`; `📚️library/🔍️discovery/🟦️component.ts`; root `📜️script.ts` policy region; `.vscode/launch.json` | ✅ done — see `📓️w0-i-report.md`: `🧩️plan` composite shape in the taxonomy + its validators, composite-aware triad/impl gates, new dependency-parity and contribution-target gates, derived `📓️dependency-inventory.md` |

Cross-lane rule for W0: 0-A and 0-B both live in `semio-framework-os-kernel` but in different `#[path]` module files — no shared file. 0-D touches two files also leased by later waves; in W0 it may only add bindgen wiring/stubs, never business logic.

## Barriers

After each wave the coordinator runs serially: `cargo check -p <touched crates>`, focused `cargo test -p …`, `bun nx run …` for touched TS packages, then dispatches read-only Haiku audits (spine parity, ownership/taxonomy, evidence honesty) before opening the next wave's leases.

Gate commands (crate names verified):
- `cargo test -p semio-framework-os-kernel --lib` (0-A, 0-B)
- `cargo test -p semio-framework --lib` (0-C)
- `cargo check -p semio-framework-plugin -p semio-framework-plugin-host` (0-D)
- `cargo check -p semio-s-plugin-flow -p semio-s-plugin-cad` (0-D sample plugin builds)

## Wave 1 leases (opened at the W0 barrier)

| Lane | Model | Exclusive lease | Deliverable |
|---|---|---|---|
| **1-A guest SDK** | Sonnet 5 | `🔌️plugin/🏗️builder/🦀️component.rs`; in `🔌️plugin/🦀️component.rs` ONLY the regions `🔖️ArtifactDeclaration`, a new `🔖️ArtifactContribution`, the `🧩️Extension` region's `ExtensionBundle`, and the two `wire_list_artifact_mutations`/`wire_artifact_mutation_plan` placeholder fns W0-D left in `plugin_runtime` | `.depends_on` / `.contributes` on both builders; `ArtifactContribution` + registration gates (contract §4); the two wire exports for real |
| **1-B guest transactions** | Sonnet 5 | in `🔌️plugin/🦀️component.rs` ONLY the regions `🔖️Emit`, `VcsArtifactApp`, `🔖️Exchange`, `🧪️testkit` | `TransactionProposal` on foreign steps; prepare/commit/rollback/undo/redo; one-pending-transaction rule and the frozen rejection codes (contract §5.5-§5.10) |
| **1-C store coordinator** | Sonnet 5 | `🏪️store/🦀️component.rs` regions `🔖️Composition`/`SpaceMember`; `🌿️vcs/🦀️component.rs` | `TransactionCoordinator` with `Owned`/`Peer` member relation (owned path byte-identical in behaviour); group undo/redo across peers; `MutationOrigin` in envelopes |

⚠️ 1-A and 1-B share one 14k-line file at non-overlapping regions. Both MUST re-read the exact region immediately before each `Edit`, keep every edit region-local, and never reformat or move a region boundary. A lane that finds its anchor changed re-reads rather than forcing.

## Scout findings that bind later waves

`📓️scout-1-pilot-targets.md` and `📓️scout-2-group-undo-and-hosts.md` (read both before starting W1/W2/W3):

- The demonstrator playground snapshot holds **no** artifact references — P3's "playground composite" is replaced by a Rust-host two-plugin e2e as the primary proof, with the demonstrator's cad↔puzzle **instances** as the browser proof.
- `CompositionCoordinator::undo_group` already exists but reverses only each member's **tail** edit — correct for this protocol precisely because a member commits one `Edit` per transaction. W1-C extends, not invents.
- The ownership check at `🏪️store/🦀️component.rs:5938` is the only blocker to peer members; W1-C adds a relation mode instead of weakening the owned path.
- The browser host retains **no** document pack per instance — W2-B must add a pack cache on `AppChannelClient` before a contributor can plan against a target.
- Nothing today considers dependents on hot reload or unload, in either host.

## Handoffs to later waves

- W1 (guest SDK, guest transactions, store coordinator) consumes W0's frozen types; `🔌️plugin/🦀️component.rs` splits into three region-scoped leases, so W1 lanes must not touch each other's regions.
- W2 (Rust host, TS host, registry/gates/launch) consumes W1.
- W3 pilots (flow composite, cad↔aec-building contribution, cross-artifact transaction) prove the mechanisms end to end.
- W4 conformance vectors + audits + close.
