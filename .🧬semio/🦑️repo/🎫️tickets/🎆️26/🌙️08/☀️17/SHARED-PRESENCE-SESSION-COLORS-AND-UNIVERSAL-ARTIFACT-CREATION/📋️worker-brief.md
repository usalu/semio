# Worker brief (embedded verbatim in every lane prompt)

`$T = .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION/`

1. Read `$T/📋️contract-freeze.md` and your lane row in `$T/📋️ownership-and-handoffs.md` before
   touching anything. `/Users/ueli/Documents/semio/CLAUDE.md` binds you.
2. **Edit only inside your lease.** If you need a file or region that is not yours, STOP editing it,
   write a `sharedFileRequest:` block in your report (file, region, exact change, why), and continue
   with the rest of your lane.
3. Other sessions are editing this tree live. **Re-read a region immediately before every `Edit`.**
   Never `Write` over an existing file you did not create. Never revert or "clean up" a foreign
   change. If `cargo` blocks on the target lock, wait and retry — never kill it.
4. **No git-modifying commands** (`commit`, `stash`, `checkout`, `restore`, `worktree`, …). Read-only
   git is fine and is how you attribute a red build: `git log --date=iso -- <file>` (commit
   *messages* carry a frozen fake date — only `--date=iso` is truthful).
5. **Never call `ticket_open`, `ticket_close` or `ticket_reopen`.** The coordinator owns the ticket.
   Never pass `isolation: "worktree"` to any tool — worktrees are forbidden in this repo.
6. All scratch, logs and probe scripts go inside `$T`, named `🧪️<lane>-*.txt` (**never `.log`** —
   `*.log` is gitignored repo-wide and would be dropped from the ticket's file list). Screenshots
   `🧪️<lane>-*.png`.
7. Write `$T/📓️<lane>-report.md` with: Changed files · Commands run + result counts (paste the real
   tail) · Blockers (`file:line` + `git log --date=iso` attribution) · sharedFileRequests · What is
   NOT done. Never claim a test passes that you did not run; the count in your report is the count in
   your log.
8. Run **only your scoped checks** (`cargo check -p <crate>`, `cargo test -p <crate> --lib`,
   `bun nx run <project>:test`). **Never `cargo check --workspace`** — peers keep it red. The hub is
   verified with **default features only** (`cargo check -p semio-hub`, `cargo test -p semio-hub --lib`,
   `--bin os-hub`); never `--all-features`, never `bun nx run os-hub:test*`. After any ABI/wire change,
   build the real wasm target (`cargo check -p <crate> --target wasm32-wasip2`) — native `cargo check`
   never compiles `#[cfg(target_arch = "wasm32")]` code.
9. Code style (CLAUDE.md): schema-first; Rust + TS twins byte-identical over the fixtures;
   `//#region 🔖️Name` / `//#endregion` structure; docstrings start with a unique emoji; no comments
   inside definitions; concise code; en + de strings for every user-visible label; no external
   runtime libraries; no legacy/compat layers, deprecations or migrations; no CRUD and no CRDTs
   (event-sourced folds only); **no `data-testid`** — use `data-ui-path` plus the §C7.9 grammar; every
   launch entry goes through the registry seed (`.vscode/🧩️launch.seed.jsonc` +
   `bun nx run @semio-tech/plugin-registry:generate`), never hand-edit `.vscode/launch.json`; one
   `📜️script.ts` per bundle.
10. Temporary logging is prefixed `[DEBUG] ` and removed before you report (or listed explicitly if
    you left it in deliberately).
11. **Presence rules** (§C7): the hub never decodes peer bytes; shells never fill `color`/`surface`
    (the client actor stamps them); the own actor is excluded from peer rendering; app-scope fields
    (`presence_pack`, `drag_ghost_json`, `ui`) are read only from same-`surface` peers while
    artifact-scope fields (`interaction`, `views`) are read from every peer; every overlay node emits
    `data-ui-path`, `data-peer-actor`, `data-peer-color` and its kind attribute in **both** renderers.
12. **Forbidden territory**: `✏️s/🔌️plugins/🗄️stdio/**` and `📜️world.wit` (the `FULL-STDIO` ticket is
    open and owns them). The wasm-broken crates `animate`, `layout`, `note` are known and stay broken —
    do not fix them here, report them as out of scope.
