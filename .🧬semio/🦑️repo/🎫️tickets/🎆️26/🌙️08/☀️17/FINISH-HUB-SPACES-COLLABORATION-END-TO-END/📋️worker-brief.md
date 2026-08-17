# Worker brief (embedded in every lane prompt)

Ticket folder `$T = .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/`.

1. Read `$T/📋️contract-freeze.md` and your lane row in `$T/📋️ownership-and-handoffs.md` before
   touching anything. `/Users/ueli/Documents/semio/CLAUDE.md` binds you.
2. **Edit only inside your lease.** If you need a file or region that is not yours, STOP editing it,
   write a `sharedFileRequest:` block in your report (file, region, exact change, why), and continue
   with the rest of your lane.
3. Other sessions are editing this tree live. **Re-read a region immediately before every `Edit`.**
   Never `Write` over an existing file you did not create. Never revert or "clean up" a foreign
   change. If `cargo` blocks on the target lock, wait and retry — never kill it.
4. **No git-modifying commands** (`commit`, `stash`, `checkout`, `restore`, `worktree`, …). Read-only
   git (`log`, `status`, `diff`) is fine and is how you attribute a red build:
   `git log --date=iso -- <file>` (commit *messages* carry a frozen fake date — only `--date=iso`
   is truthful).
5. **Never call `ticket_close`, `ticket_reopen` or `ticket_open`.** The coordinator owns the ticket.
6. All scratch, logs and probe scripts go inside `$T`, named `🧪️<lane>-*.txt` (**never `.log`** —
   `*.log` is gitignored repo-wide and would be dropped from the ticket's file list).
7. Write `$T/📓️<lane>-report.md` with: Changed files · Commands run + result counts (paste the real
   tail) · Blockers (`file:line` + `git log --date=iso` attribution) · sharedFileRequests · What is
   NOT done. Never claim a test passes that you did not run.
8. Run **only your scoped checks** (`cargo check -p <crate>`, `cargo test -p <crate> --lib`,
   `bun nx run <project>:test`). **Never `cargo check --workspace`** — peers keep it red.
9. Code style (CLAUDE.md): schema-first, Rust + TS twins, `//#region 🔖️Name` / `//#endregion`
   structure, docstrings start with a unique emoji, no comments inside definitions, concise code,
   en + de strings for every user-visible label, no external runtime libraries, no legacy/compat
   layers or migrations, no CRUD and no CRDTs (event-sourced folds only).
10. Temporary logging is prefixed `[DEBUG] ` and removed before you report (or listed explicitly if
    you left it in deliberately).
