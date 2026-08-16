# Ownership and Handoffs

Every lane writes **only inside its lease**. A lane that must touch a file outside its lease **stops
and reports to the coordinator** in its `📓️<lane>-report.md` — it does not edit and it does not ask
another lane to edit for it.

All paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/` unless they start with
`🧰️`, `✏️s`, `.vscode` or `📋️`/`📜️` at repo root. Repo root is
`/Users/ueli/Documents/semio`.

## Shared-tree discipline (binding on every writer)

1. **Re-read the target region immediately before every edit.** Other sessions are editing these
   same files right now.
2. **`Edit` only, region-local.** Never a whole-file `Write` on an existing file. Never a reformat.
   Never revert a foreign change you did not make.
3. **Attribute before you blame.** `git log --date=iso -- <file>` is the only real date; commit
   *messages* embed a frozen fake template string (`🎆️26🌙️06☀️04`) and must never be parsed.
4. **No git-modifying commands. Ever.** No `commit`, `stash`, `checkout`, `restore`, `reset`, `add`,
   `rebase`, `merge`. **No git worktrees**, and never `isolation: "worktree"`.
5. **Never call `ticket_close` or `ticket_reopen`.** Only the coordinator closes this ticket.
6. **Scratch files go in the ticket folder as `.txt`** (`*.log` is gitignored repo-wide and would be
   silently dropped from the close manifest). Reports go to `📓️<lane>-report.md`.
7. **Validate every claim.** Never write "tests pass" without having run them and pasted the actual
   pass/fail counts into the report.

## Concurrent live tickets

| Ticket | Status | Overlap with us | Rule |
|---|---|---|---|
| `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` | **closed** | `🏪️store` Composition/Transaction, `🔌️plugin` Emit/Exchange/`VcsArtifactApp`/testkit, `🖥️host`, channel tags 19–26, `AppChannelCodec`/`AppChannelClient` | Closed ⇒ its regions are free and its tag range is final. Lanes 1-C, 1-E, 2-A, 2-B may proceed without negotiation. Read its `📓️w2-a-report.md` / `📓️w2-w3-barrier.md` before restructuring anything it just built. |
| `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` | **open, active** | `🗄️stdio/**` (incl. the 34 legacy artifact enums), `🔌️plugin`, `📜️world.wit` | Lane **3-E** wraps stdio's `diff` return types **minimally** — return type only, nothing else. It must not restructure a stdio enum, must not touch `📜️world.wit`, and records every collision in `📓️w3-e-…-report.md` as a `sharedFileRequest` entry. Expect churn: a stdio compile failure is theirs until `git log --date=iso` proves otherwise. Their `📓️`/`🧪️` files show the stdio tree was failing to compile all session for reasons unrelated to us. |
| `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` | **open, active** | `🔌️plugin` app ids and `VcsArtifactApp` (new `🔖️Surfaces` region), `💻️os/🎚️config` opening-preferences triads, `🖱️ui`/renderer shell surfaces, root `📜️script.ts` taxonomy policies, `.vscode/launch.json` | Their `📌️important.md` parks a `VcsArtifactApp` role guard and testkit work until a peer report lands. Lane **2-A** adds `VcsArtifactApp::preview` as an **additive method** and does not touch `🔖️Surfaces`. Lane **2-D** adds the `🛡️change-merge-policy` config triad as a **new triad folder**, never editing their `OpeningPreferences` triads. The coordinator owns `📜️script.ts` and `.vscode/launch.json` and appends only. |

## Lease table

### W0 — kernel spine + derive (must finish before anything else)

| Lane | Model | Exclusive lease | Notes |
|---|---|---|---|
| **0-A** kernel spine | Sonnet | `📡️spr/🎮️command/**`, `📡️spr/🧾️wire/🦀️component.rs` region `🔖️Policies`, `📡️spr/🦀️component.rs`, `📡️spr/🔀️crdt/**` (**delete**), new `📡️spr/⚔️conflict/**`, `🗣️dsl/⚠️diagnostic/**`, `🕸️graph/🗣️dsl` (`Hint`→`Info` only), `🌿️vcs` (`apply_mutation`, `Errors`), `📡️spr/🧪️testkit` (delete crdt helpers + bench group, fixtures), and the **mechanical return-type adaptation** of every in-crate `impl Mutation`/`impl MutationKind`: `🏪️store` `replay_mutations`/`ingest_remote` (**minimal wrap only** — 1-A owns the real algorithm), `🪐️space`, `🔁️workflow`, `🌊️flow/🌿️vcs`, `♾️infinite …/dag`, `💻️os/🎚️config`; glue `📦️glue.rs` `#[path]` entries | Delivers C1–C5 + C10. Acceptance: kernel crate compiles; `cargo test -p semio-framework-os-kernel --lib -- os_spr::command` green. |
| **0-B** derive | Sonnet | `🗣️dsl/✨️derive/🦀️component.rs` **and** `🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` — the two are **byte-identical mirrors**; both must be edited and `diff` between them must be empty | `#[derive(Mutations)]` / `#[derive(CompositeMutation)]` emit outcome-returning `diff`, emit no `validate`, and carry no `ConflictRule` into `register_calls`. |

### W1 — kernel breadth (after W0 barrier)

| Lane | Lease |
|---|---|
| **1-A** store | `🏪️store/🦀️component.rs` regions `🔖️ArtifactStore`, `🔖️Authority`, `🔖️Schemas` (`ArtifactCommand` 15/16), `🔖️Backbone`; `📡️spr/🔗️causal`. **Not** `🔖️Composition`/`🔖️Space`/`🔖️CompositionCoordinator` (that is 1-E). |
| **1-B** history | `📡️spr/📜️history/**` |
| **1-C** channel | `📡️spr/🧵️channel/🦀️component.rs`, `💻️os/🟦️component.ts` **`AppChannelCodec` region + its tests only**, `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/**`, `📡️wire` `ApplyOutcome`. **Sole owner of the `CHANNEL_VERSION` 10→11 bump.** |
| **1-D** testkit | `📡️spr/🧪️testkit/🦀️component.rs` region `🔖️Laws` (0-A already removed the crdt helpers there) |
| **1-E** composition | `🏪️store/🦀️component.rs` regions `🔖️Composition`, `🔖️Space`, `🔖️CompositionCoordinator` |

`💻️os/🟦️component.ts` is split by region: **1-C owns `AppChannelCodec`**, **2-C owns
`AppChannelClient` + the public api**. They must not cross.

### W2 — edges (after W1 lands its region; may overlap W3)

| Lane | Lease |
|---|---|
| **2-A** guest SDK | `🔌️plugin/🦀️component.rs` regions `VcsArtifactApp` (additive `preview` only), `Emit`, `Exchange`, `plugin_runtime`, its testkit; `🔌️plugin/🏗️builder/**`. Must not touch `🔖️Surfaces`. |
| **2-B** Rust host | `🔌️plugin/🖥️host/🦀️component.rs`, `🏃️run/**` |
| **2-C** TS host + kernel | `💻️os/🟦️component.ts` (`AppChannelClient`, public api, `faultMessages()`), `🎠️kernel/🟦️component.ts`, renderer boot |
| **2-D** shell UI + i18n | `📺️renderer/🧑️‍🎨️engine/🧱️elements/{ChromePanels,EventFeedHost,DiffViewHost,ShellSync,ShellHost,Shell}`, `🖱️ui/🧱️elements/📜️HistoryTable`, `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` (**i18n keys only**), new triad `💻️os/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/**` |
| **2-E** hub + db | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, `🛢️db/{📄️artifact,⚔️conflict,⌨️cli,👁️preview}/**` |

### W3 — plugin fan-out (starts right after the W0 barrier, parallel with W1/W2)

Each lane owns whole plugin trees under `✏️s/🔌️plugins/<id>/`. **No W3 lane may edit anything under
`🧰️framework/`** — the kernel is W0/W1/W2 territory. Norm lanes additionally never touch
`📕️norm/🎚️config` or `👥️presence` (coordinator-held).

| Lane | Plugins (leaf count) |
|---|---|
| 3-A | `📕️norm` {din16798, en1998, en1999, en1997} (159) |
| 3-B | `📕️norm` {en1992, en1991, en1996, en1994, en1990} (121) |
| 3-C | `📕️norm` {din4108, iso16757, en1995, vdi3805, en1993, din18599} (112) |
| 3-D | `🏛️architect` (266) |
| 3-E | `🗄️stdio` (125 + 34 legacy enums) — **coordinate with FULL-STDIO, minimal wraps only** |
| 3-F | `🧱️block` + `🧩️puzzle` (193) |
| 3-G | `🏗️fem`, `🌀️procedural`, `📸️remodel`, `🗒️note`, `🎥️shooting` (185) |
| 3-H | remaining 20 plugins (239) |

### W4 — coordinator-only lease

Root `📜️script.ts` (`policy…` region), `.vscode/launch.json` (group `4_gate` + `3_dev`),
`📋️project.json`, and all ticket docs. **No lane may edit these three files.** A lane that needs a
gate changed reports it.

## Handoff points

1. **W0 barrier → W1 + W3.** Nothing in W1 or W3 may start until the coordinator has posted
   `📓️w0-barrier.md` green. W3 then runs concurrently with W1/W2 because plugin crates cannot
   compile until their own lane lands.
2. **1-C → 2-C.** 2-C's vitest parity test asserts against the golden vectors 1-C bakes under
   `🧫️fixtures/📡️channel/`. If 1-C has not landed them, 2-C writes the test and reports the
   fixtures as pending rather than inventing bytes.
3. **1-A → 2-B/2-E.** The `MergeReport`/`DispatchReport`/`Conflict` shapes come from `⚔️conflict`
   (0-A) and the store methods from 1-A. 2-B and 2-E code against the frozen C5/C6 signatures.
4. **1-D → every W3 lane.** W3 facet tests call the `🔖️Laws` helpers. If a helper is not there yet,
   the lane writes the facet test against the frozen helper name and reports it as pending.
5. **W3 → W2/W3 barrier.** Per-plugin `cargo test -p semio-s-plugin-<id> --lib` is run **inside** the
   lane (that scope is allowed); repo-wide `cargo check --workspace` is the coordinator's, serial.
