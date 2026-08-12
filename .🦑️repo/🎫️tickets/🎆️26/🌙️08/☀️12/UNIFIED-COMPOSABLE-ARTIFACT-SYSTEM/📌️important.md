# Important — read first, every agent

Full design + rationale: `/Users/ueli/.claude/plans/the-current-artifact-system-eager-scone.md` (also copied as `📓️design-composition.md` + `📓️design-stdio-subsets.md` + `📓️orchestration.md` in this ticket for the raw agent transcripts).

## Hard rules (repo-wide, non-negotiable)

- No git-modifying commands (`commit`/`stash`/`checkout`/`reset`/worktrees). Live shared tree — another session (SEMANTIC-MUTATIONS-OVERHAUL, "SMO") is editing concurrently.
- All temp files, logs, scratch scripts inside THIS ticket folder. Scratch logs are `.txt`, never `.log` (`*.log` is repo-gitignored and silently drops from `ticket_close`'s file list).
- `bun` + `nx`; permanent scripts only in `script.ts` at the relevant directory; `project.json`/`package.json` just call it.
- Regions (`//#region Name` / `//#endregion Name`) for structure; no new files outside the existing taxonomy shapes; `[DEBUG] ` prefix on any temporary logging, removed before a wave is reported done.
- Greenfield: no compatibility layers, no legacy support, no deprecations. Regenerate fixtures/examples; never upcast/migrate them.
- Semantic mutation vocabulary only: `SetSnapshot`/`NoMutation`/`CollectionMutation` (public) are BANNED. Use the existing `🧬️mutations/<slug>/{🦠️mutation,🔺️diff,↩️inverse}` triad shape and `MutationKind`/`SemanticMutation`/`#[derive(Mutations)]`.
- Never run bare `cargo check` — always scope `-p <crate>`. A red workspace is not a red plugin.
- `CARGO_TARGET_DIR=<this-ticket>/🎯️target` for every cargo invocation (shared; the flock serializes concurrent checks — "Blocking waiting for file lock on build directory" is normal, wait, don't kill it).
- Never close this shared ticket. Never edit `📓️status.md` except the orchestrator. Append your report to your assigned `📓️waveN-reports/` file only.

## Hot-file ownership (binding)

| File / subtree | Owner | Everyone else |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | W1 mechanism agent, then frozen | read-only; file a `sharedFileRequests` entry |
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, `🧬️schema/🦀️component.rs`, `🛂️manifest/🦀️component.rs` | W1 mechanism agent | read-only |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/**`, `🏪️store/**`, `🌿️vcs/**` | W1 mechanism agent | read-only |
| `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` | W1 mechanism agent | read-only |
| `✏️s/🔌️plugins/🗄️stdio/**` incl. `📦️glue.rs` + `🦀️component.rs` | W2 stdio agent, then W5 serializer | read-only; consume stdio types only |
| `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json` | W2 stdio agent, then W5 serializer | read-only |
| repo-root `📜️script.ts` | W6 ratchet agent only | read-only, even for allowlist entries — request it |
| `🦑️repo/…/📚️library/🔣️taxonomy.json`, its TS `📦️index.ts` | W6 ratchet agent | read-only |
| `✏️s/🔌️plugins/<P>/**` minus `📦️glue.rs`/`📦️index.ts` | that plugin's one fan-out agent | nobody else, ever |
| `✏️s/🔌️plugins/<P>/📦️packages/🦀️rust/📦️glue.rs`, TS `📦️index.ts` | W5 serializer (fan-out agents file `🔧️patches/<P>-glue-patch.txt` + `## sharedFileRequests`) | — |

## Coordination with SEMANTIC-MUTATIONS-OVERHAUL (SMO)

SMO is running wave-2 mass fan-out across `✏️s/🔌️plugins/**/🧬️mutations/**` right now, in another session, on this same tree.

1. **Plugin-level mutual exclusion.** Before touching plugin `<P>`, check `../SEMANTIC-MUTATIONS-OVERHAUL/📓️wave2-reports/` (and `📓️wave1-reports/`) for reports covering ALL of `<P>`'s artifacts. Only fully-covered plugins are "SMO-clear." See `📓️smo-clearance.md` for the snapshot computed at ticket-open time — **re-check before dispatch, it goes stale fast.**
2. **Never touch `🧬️mutations/**` of an uncleared plugin.**
3. **stdio is the big overlap risk** — claim it explicitly in `📓️status.md` before W2 starts.
4. **The one forbidden collision**: our W1 and SMO's eventual wave-4 ratchet both want to write `🔌️plugin/🦀️component.rs`. Whoever starts first wins; the other waits. Check SMO's `📓️status.md` before starting W1.
5. **Transient failure protocol**: on a compile error in a file OUTSIDE your boundary — `git status --porcelain` + `stat` the file; if modified today by not-you, it's concurrent churn. Retry `cargo check -p <your-crate>` up to 3× at 60s intervals. If it persists, record it under `## Concurrent-churn observations` in your report, prove zero errors originate in your own boundary (grep the cargo output for your path), report `blocked-mechanism`, and stop. Never "fix" someone else's file.

## Repo conventions learned during this ticket (obey these)

1. **Derive crates keep two byte-identical copies.** `<module>/✨️derive/🦀️component.rs` and `<module>/✨️derive/📦️packages/🦀️rust/📦️glue.rs` must stay identical — Cargo compiles the *glue* copy, so editing only `component.rs` silently does nothing. Verified true for both `🧬️schema/✨️derive` and `🗣️dsl/✨️derive`. Edit one, then mirror it exactly, then `diff -q` the pair before reporting done.
2. **`mcp__repo__file_integrate` misbehaved** on that mirroring (wrapped the whole file in a nested `mod helpers {}`, duplicating everything). Mirror by hand and verify with `diff -q`.
3. **`#[link(...)]` is unusable as a custom field attribute** — `link` is a built-in Rust attribute (extern-block FFI) and applying it to a field is a hard error (E0659/E0539/E0459), not a lint. The composition link-slot attribute is therefore `#[link_slot(roles("a", "b"))]`. The child attribute `#[child(kind = "s.stdio.mesh")]` is fine as-is.
4. **Additive struct fields still break struct literals** (serde `default` only affects (de)serialization, not Rust construction). After adding a field, grep for `TypeName {` across the whole workspace — not just your crate — and either fix the literals or file them under `sharedFileRequests`.
5. **Adding an enum variant is expensive** where the enum is matched exhaustively. Measure with `grep -rln "EnumName::"` before committing to it. (`Shape::` matches in ~20 files — see deviation D1 in `📓️status.md`.)

## Report shape (every wave)

Follow the shape of SMO's `📓️wave2-reports/norm-en1994-1-any-report.md` as the reference: what changed, files touched, verification commands + results, `## sharedFileRequests` (file, region, reason, patch file path), `## Concurrent-churn observations`, honest pass/fail — never claim a test passed without running it.
