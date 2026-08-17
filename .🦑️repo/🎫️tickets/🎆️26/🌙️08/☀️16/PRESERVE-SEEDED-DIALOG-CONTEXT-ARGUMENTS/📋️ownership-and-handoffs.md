# Ownership, leases and handoffs

## A. Region leases against the live peer tickets

Checked at every barrier against `26/08/16/MUTATION-OUTCOMES-…/📋️ownership-and-handoffs.md` and
`…/📌️important.md`.

| File / region | Peer owner | Our lane | Rule |
|---|---|---|---|
| `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` — `submit_commands`, `merge_policy_from_env`, `encode_messages`, `messages_for_error`, `HubState.merge_policy`, `ClientFrame::Commands` arm | MUTATION-OUTCOMES 2-E | — | **forbidden** |
| `📦️bin.rs` — everything else (`handle_ws`, `Presence` arm, `HubState` new fields, `router`, `main`, `mod tests`) | — | 0-B, 1-B, 2-E | additive; re-read before every edit; if `git log --date=iso -- 📦️bin.rs` shows a commit < 30 min old, wait and re-read |
| `🌎️hub/📇️directory/**` | blanket "🌎️hub" claim but untouched since 08-12 | 1-A | ours (announced in `📌️important.md`) |
| `🛢️db/**`, `🏪️store/**`, `📡️spr/**`, framework `🌿️vcs`, `🗣️dsl/**`, `🔌️plugin` Emit/Exchange/runtime, `🔌️plugin/🖥️host`, `🏃️run`, `🎠️kernel/🟦️component.ts` | MUTATION-OUTCOMES | — | **forbidden** (consume as-is) |
| `💻️os/🟦️component.ts` — `AppChannelCodec`, `AppChannelClient`, public api region | MUTATION-OUTCOMES 1-C / 2-C | 1-C | only the NEW `🔖️HubBinding` region |
| `💻️os/🎚️config/**` existing triads | MUTATION-OUTCOMES | 1-C | only the NEW `🪪️identity` triad directory |
| React `ShellSync`, `ChromePanels`, `EventFeedHost`, `DiffViewHost`, `📜️HistoryTable` | MUTATION-OUTCOMES 2-D | — | **forbidden** |
| React `ShellHost/🟦️component.tsx`, `⚛️react/📦️index.tsx` | partially peer (conflict UI) | 2-C, 2-F, 3-A | only the identity / binding / routing / presence / check-in regions and an **appended** i18n block; re-read before every edit |
| `✏️s/🔌️plugins/🗄️stdio/**`, `📜️world.wit` | FULL-STDIO | — | **forbidden**; stdio's red wasm build is theirs to fix |
| `✏️s/🔌️plugins/🪐️space/**` | MUTATION-OUTCOMES 3-H fan-out (already landed per their `📓️w3-h-remaining-report.md`) | 1-E, 2-A, 2-B | ours |
| root `📜️script.ts`, `.vscode/launch.json`, root `📋️project.json` | peer coordinator | coordinator only | launch.json is **generated** — edit `.vscode/🧩️launch.seed.jsonc` and regenerate |
| `🧑‍💻️dev/📦️packages/🟦️typescript/{📜️script.ts,⚙️vite.config.ts}`, `📇️registry/**`, `🌎️hub/🔨️modules/🛡️admin/**`, `💻️os/🔨️modules/📇️directory/**`, `🖱️ui/🧱️elements/👥️PresenceBar/**` | none | 1-F, 2-E, 0-A, 2-F, 3-C | ours |

**Protocol for a foreign touch:** stop → `sharedFileRequest:` block in the lane report → the
coordinator either performs it (if trivially additive and the peer region is idle by
`git log --date=iso`, recorded under "Foreign touches" in `📌️important.md`), defers it to W4, or
redesigns around it. Never edit a peer ticket's folder.

## B. Lanes

| Lane | Model | Scope | Report |
|---|---|---|---|
| 0-C | Opus (coordinator) | ticket, master files, barriers, waves | `📓️w0-barrier.md` |
| 0-S1 | Haiku | scout: hub + db public API + bin.rs test helpers | `📓️scout-hub.md` |
| 0-S2 | Haiku | scout: shell identity/actor/presence/routing sites | `📓️scout-client.md` |
| 0-S3 | Haiku | scout: dev script / registry / gate insertion points | `📓️scout-gates.md` |
| 0-A | Sonnet | `💻️os/🔨️modules/📇️directory/**` schema + read model + fold + fixture | `📓️w0-a-report.md` |
| 0-B | Sonnet | hub port/env fixes (8787, script env merge) | `📓️w0-b-report.md` |
| 1-A | Sonnet | hub directory event log, decider, service, 3 backends | `📓️w1-a-report.md` |
| 1-B | Sonnet | hub REST/WS directory + admin API + presence per surface | `📓️w1-b-report.md` |
| 1-C | Sonnet | TS identity facet + directory client + backbone worker | `📓️w1-c-report.md` |
| 1-D | Sonnet | Rust identity + directory client twin | `📓️w1-d-report.md` |
| 1-E | Sonnet | `s.space` artifact (index doc, mutations, projection) | `📓️w1-e-report.md` |
| 1-F | Sonnet | dev configs: registry users dimension, seed, build lease | `📓️w1-f-report.md` |
| 2-A | Sonnet | Home app = table of spaces (editor + viewer) | `📓️w2-a-report.md` |
| 2-B | Sonnet | Space app = table of artifacts (editor + viewer) | `📓️w2-b-report.md` |
| 2-C | Sonnet | React shell: identity, auto-bind, directory relay, routing | `📓️w2-c-report.md` |
| 2-D | Sonnet | wgpu shell: same + presence consumption | `📓️w2-d-report.md` |
| 2-E | Sonnet | hub admin SPA + `/admin` serving + build ordering | `📓️w2-e-report.md` |
| 2-F | Sonnet | `👥️PresenceBar` element + presence chrome both shells | `📓️w2-f-report.md` |
| 3-A | Sonnet | save/check-in policy both shells | `📓️w3-a-report.md` |
| 3-B | Sonnet | opening relay `documentId`/`spaceId` (no channel bump) | `📓️w3-b-report.md` |
| 3-C | Sonnet | `verify collab` two-browser e2e | `📓️w3-c-report.md` |
| 3-D | Sonnet | wgpu parity + native smoke | `📓️w3-d-report.md` |
| 3-E | Sonnet | hub bun integration test package | `📓️w3-e-report.md` |
| 4-A/B/C | Haiku | audits: taxonomy/CQRS/i18n · evidence · leases | `📓️audit-w4-*.md` |

## C. Handoffs between lanes

- **0-A → 1-A/1-C/1-D/2-A/2-B**: the `DirectoryEvent`/`DirectoryCommand`/read-model types and the
  `fold`. Everyone imports them; nobody re-declares them.
- **1-A → 1-B**: `DirectoryService::execute(actor, command)` and the `HubDirectory` reads.
- **1-B → 1-C/1-D/2-E/3-E**: the live REST/WS surface exactly as frozen in C2.
- **1-C/1-D → 2-C/2-D**: `DirectoryClient` + identity fold + `PersistenceBinding.hub.surface`.
- **1-E → 2-B/3-A**: `SSpaceSnapshot` + the four mutations + `project_space_index_to_collection`.
- **1-F → 3-C**: the two user launchers, the compound, and the plugin-build lease the e2e relies on.
- **2-A/2-B/2-C/2-D/2-F/3-A → 3-C**: the frozen `data-row-id` / element-id grammar (C0).
