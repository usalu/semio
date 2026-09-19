---
name: clean
description: >-
  Clean the workspace of ticket junk, misplaced emoji paths, and oversized build
  artifacts. Triggers clean, cleanup, tidy — execute immediately.
---

# Clean

**Any trigger (`clean`, `cleanup`, `tidy`, `@clean`) = do it now.**

- **No** planning, preamble, or questions.
- **One** assistant turn: run clean, then reply with the command summary only.

## Execute

```bash
bun ./📜️script.ts clean
```

Dry-run only when the user explicitly asks:

```bash
bun ./📜️script.ts clean --dry
```

## What it removes

1. **Misplaced emoji mounts** under `.🧬semio/` (corrupted `*repo` / `*tickets` names such as `🧑‍🦑️repo`, `🧑️repo`, `🪷tickets`) and a root-level `🦑️repo/`.
2. **Gitignored paths inside ticket trees** (`**/🎫️tickets/**`), including generated `🧾️runs/` output at any nesting depth.
3. **Oversized paths inside ticket folders only** (`…/🎆️YY/🌙️MM/☀️DD/TICKETSLUG/…`): files > 5MB, subfolders > 10MB. The ticket slug folder itself and year/month/day parents are never size-deleted.
4. **Build artifacts** named `target`, `🎯️target*`, `dist`, `build`, `out` larger than **10GB**.
5. **Dev leftovers** — known dev executables (`bun`, `node`, `cargo`, `rustc`, `vite`, `esbuild`, `nx`, `turbo`, `cursor-agent`, …) plus command markers (`/semio/`, `node_modules/.bin`, `📜️script.ts`, `cursor-agent worker start`, …): every live match outside this script's ancestry and outside IDE hosts (Cursor/VS Code/Electron) is killed; zombies (including macOS `<defunct>` rows) are reaped by killing their still-live parent. Repeats until a pass finds nothing.

Fast path (same stray-process step as full `clean`, without filesystem work):

```bash
bun ./📜️script.ts clean stray-processes
```

## What it never removes

- `.🧬semio/🗺️map` (map tiles)
- `.🧬semio/🌐hub`
- `.🧬semio/🔗space`
- `.🧬semio/🦑️repo/⚡️cache`

## Reply

Paste the `bun ./📜️script.ts clean` stdout (or stderr on failure) in one fenced block. No extra commentary.
