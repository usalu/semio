# Ownership and Handoffs

Ticket: `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`.
Plan: `📋️master-plan.md`. Frozen contract: `📋️contract-freeze.md`.
Start commit: `63686457bdcf0e7ba57a6598a4e224ec6c739f8e` (2026-08-16 02:50:31 +0200).

## Roles

- **Coordinator (Opus 5, this session):** ticket docs, contract freeze, lease allocation, serial
  barrier gates, audit dispatch, hand-repair, `ticket_close` with the explicit ticket path.
- **Executors (Sonnet 5, `general-purpose`):** one per lease. Write only inside the lease.
- **Scouts / auditors (Haiku 4.5, `Explore`):** read-only.

## Rules every executor is given verbatim

1. Never `isolation: "worktree"`.
2. Never a modifying git command (`commit`, `stash`, `checkout`, `restore`, `reset`, …).
3. Never `ticket_close` / `ticket_reopen`.
4. Write only inside the lease. Re-read the target region immediately before each `Edit`. Edit by
   region. Never rewrite a whole file. Never revert a foreign hunk.
5. Scratch/logs go in this ticket folder as `.txt` (`.log` is gitignored and `ticket_close` drops it).
6. Research/summaries are markdown files in the ticket folder, referenced by path — never pasted.
7. Report to `📓️<lane>-report.md`: what landed, `file:line` anchors, exact commands + results, and
   what is NOT done and why.
8. No claim of a passing test without running it; no claim of runtime behaviour without a console
   log. Temporary logs are prefixed `[DEBUG] ` and removed before the lane closes.
9. `//#region 🔖️Name` / `//#endregion`; emoji-first docstrings; no comments inside definitions.
10. Schema-first, Rust + TypeScript twins, en/de with no default language, event-sourced CQRS, no
    legacy/compat/deprecation, no migration scripts.
11. Cargo gates: `RUSTC_WRAPPER="" cargo check -p <crate> --all-targets --keep-going`, **serially**.

## ⚠️ Live concurrent sessions — contended files

Two peer tickets are writing this tree right now (mtimes confirmed within 60 s of ticket start):

| file | peer ticket | peer lease | our lane | resolution |
|---|---|---|---|---|
| `🔌️plugin/🦀️component.rs` | 26/08/16 PLUGIN-DEPENDENCIES (W1-B) | regions `🔖️Emit`, `VcsArtifactApp`, `🔖️Exchange`, `🧪️testkit` | 0-B, 0-D | 0-D takes a **new** region only. 0-B's viewer/editor traits + adapters go in a **new** region; its `VcsArtifactApp`/`testkit` touches are deferred to the W0 barrier and made by the coordinator or a follow-up lane once the peer's W1-B report exists. |
| `🔌️plugin/🦀️component.rs` | 26/08/16 FULL-STDIO (W0-A) | whole file | 0-B, 0-D | same |
| `🔌️plugin/🏗️builder/🦀️component.rs` | 26/08/16 PLUGIN-DEPENDENCIES (W1-A) | `.depends_on` / `.contributes` / `ArtifactContribution` | 0-B | 0-B adds `viewer`/`editor` methods **beside** `document_app` and deletes `document_app` only at W2 close, when the last call site is rewired. Re-read before every edit. |
| root `📜️script.ts` | both peers | policy regions | 1-E, 3-A | policies are **appended**; no existing policy is reshaped before W3. |
| `🔣️taxonomy.json` | peer W0-I (done 02:02) | — | 0-I (coordinator) | additive keys only in W0. |
| `📜️world.wit` | both peers | — | **nobody** | contract §3: no WIT change is needed. No lane opens this file. |
| `📡️spr/🧵️channel/🦀️component.rs` | peer W0-B (done) | tags 22–26 | 0-C | append after the true last variant read off disk; never trust a tag number from a document. |

## Wave 0 leases

| Lane | Model | Exclusive lease | Deliverable |
|---|---|---|---|
| **0-I coordinator** | Opus | ticket docs; `🔣️taxonomy.json` (additive keys); `📚️library/🔍️discovery/🟦️component.ts` + its test | contract frozen; taxonomy schemaVersion 5 additive; repo-lib test no new failures |
| **0-A manifest spine** | Sonnet | `🛂️manifest/🦀️component.rs` regions around `AppDefinition` / `ModeDefinition` / `PanelTabKind`; ts-rs regen | C1: `AppRole`, `AppRef`, `AppDefinition.{role,dialect}`, `surface_app_id`/`parse_surface_app_id`, `PanelTabKind::SettingsDefaultApps`; unit tests |
| **0-B SDK spine** | Sonnet | `🔌️plugin/🦀️component.rs` **new region `🔖️Surfaces`** only; `🔌️plugin/🏗️builder/🦀️component.rs` (additive methods) | C2 §2.1–2.2, §2.4 minus the `VcsArtifactApp` guard; `cargo check -p semio-framework-plugin --all-targets` |
| **0-C channel + config spine** | Sonnet | `📡️spr/🧵️channel/🦀️component.rs`; `💻️os/🟦️component.ts` `AppChannelCodec` + tests; `💻️os/🧫️fixtures/📡️channel/`; new `💻️os/🎚️config/**`; new `💻️os/🎮️commands/**` | C3 commands + C4 schema quintuple + both triads; golden vectors decode identically Rust/TS |
| **0-D window kits** | Sonnet | `🔌️plugin/🦀️component.rs` **new region `🔖️WindowKits`**; new `🔌️plugin/📦️packages/🟦️typescript/🪟️window-kits/` | seven kits, en/de labels, unit tests rendering `UiNode` |
| **0-H scouts ×3** | Haiku | read-only | `📓️subset-inventory.md`, `📓️consumer-inventory.md`, `📓️shell-inventory.md` |

0-B and 0-D share one 15 k-line file at two disjoint **new** regions. Both append their region at the
end of the `app` module rather than splicing into existing code, so neither can collide with the
other or with the two peer sessions.

## Waves 1–4

Opened at each barrier; see `📋️master-plan.md`. Touched-region ledger is appended below as lanes
report.

## Touched-region ledger

(appended by the coordinator at each barrier)
