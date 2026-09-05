# Windows checkout audit

Repo MCP (`repo`/`semio` servers) failed to connect this session (`invalid initialize
params` / `CONNECT_TIMEOUT`). Verified the underlying dispatcher is alive
(`./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client entity-emojis` prints emoji, not the
usage banner), so this is the known slow-boot/connect flakiness, not a broken CLI. Ticket
bookkeeping for this session was done by hand on disk per
`project-repo-mcp-may-fail-to-connect` memory.

## What already exists

`📜️script.ts` already implements a `windows-illegal` removal kind inside `clean` (the plain
workspace clean, wired to launch.json as `🧹clean` → `bun ./📜️script.ts clean`):

- `cleanIsWindowsIllegalName` ([📜️script.ts:21419](../../../../../../📜️script.ts)) flags: forbidden
  chars `<>:"|?*` and control chars 0x00-0x1F, trailing space/dot, leading space, empty/whitespace-only
  names, and reserved device names (`CON`, `PRN`, `AUX`, `NUL`, `COM1-9`, `LPT1-9`).
- `cleanCollectWindowsIllegal` walks the whole tree plus `git ls-files -z` (so it also catches
  illegal names inside directories it didn't need to descend into, and anything already committed).
- `cleanProjectRemovals` deliberately lets `windows-illegal` removals bypass the "don't touch an
  open ticket" protection (`allowedOpenTicket = cleanTicketFolderForPath(...)`) — illegal names are
  treated as unconditionally-removable debris even mid-ticket, unlike every other removal kind.
- `runWorkspaceClean` → `cleanRemovePath(..., row.kind === "windows-illegal")` actually deletes (not
  dry-run) unless `--dry`/`dry` is passed.

## Audit of the actual repo state

Ran two independent scans against `git ls-files -z` (72772 tracked/staged paths):

1. A first-pass Perl one-liner produced a false positive (byte-vs-character handling bug on my
   part, not in the codebase) claiming a whitespace-only filename existed under
   `.../🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/`. Verified directly
   with `git ls-tree`, `git ls-files -s`, and a `python3 os.listdir` pass — no such entry exists,
   on disk or in the index.
2. A corrected Node/Bun script (`scan-illegal.mjs`, UTF-8 safe) re-scanned every tracked path
   component against the exact same rules as `cleanIsWindowsIllegalName`: **0 illegal names found**
   out of 72772 tracked paths.
3. Also checked case-insensitive path collisions (NTFS is case-insensitive/case-preserving): **0
   collisions**.

So, as of this commit, there is no tracked file or directory with a Windows-illegal name by the
strict definition (forbidden char / reserved device name / trailing space or dot / empty name).
The existing `clean` windows-illegal mechanism is correct and has nothing to delete right now.

## The actual likely blocker: path length, not illegal characters

Checked UTF-16 code-unit length (how NTFS/Win32 actually measures path length) of every tracked
path. The classic Win32 `MAX_PATH` ceiling is 260 characters for the **full** path (drive + all
parent dirs + filename), and it is off by default on most machines — `git config core.longpaths`
must be enabled locally by every Windows developer, or the newer per-app long-path opt-in
(`LongPathsEnabled` registry key) must be set, before repos with deep paths can be checked out at
all.

- 204 tracked paths would already exceed 260 characters with just a modest ~35-character checkout
  root (e.g. `C:\Users\name\Documents\semio\`) added on top of their repo-relative length.
- The worst offenders are 293 UTF-16 units on their own, e.g.
  `🧰️framework/.../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️cargo-provider-binding/.../direct-local-library-binding/provider/src/lib.rs`
  and several `🕰️misplaced-cache-evidence/.../CACHEDIR.TAG` stub files at 260-276 units under
  `.🎆️26/🌙️04/☀️08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY/`.
- Both of the tickets that own the worst offenders are **currently open** (`SEMANTIC-MUTATIONS-OVERHAUL`
  status field is missing/malformed — grep found no `status` key at all, worth a separate look;
  `ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` is `"status": "open"`), and the git status at
  the start of this session shows another live process actively writing new files under
  `SEMANTIC-MUTATIONS-OVERHAUL/📸️source-index-capture-66/🧫️run-RpyIUM/` right now. Per
  `feedback-no-git-stash`/`live-predicate-not-derived-artifact` conventions, this session must not
  touch or delete another session's in-flight ticket content.

There's also a pre-existing, unrelated, now-closed ticket
(`.🎆️26/🌙️04/☀️09/FIX-WINDOWS-NATIVE-DEV-AND-BUILD-FLOWS-.../`) titled about Windows native dev/build
flows, but it was bulk-closed with a one-line "codex" description and no content — it doesn't cover
filename/path-length specifics, so it wasn't reopened; this ticket stays a separate, more scoped one.

## Verifying deletion actually happens (not just detection)

Read `cleanRemovePath` ([📜️script.ts:21569](../../../../../../📜️script.ts)): when a removal's kind is
`windows-illegal` and `dry` is false, it runs `git rm -f --` on the path first (so a *tracked*
illegal-named entry is actually removed from the index, not just the working tree — otherwise it
would silently reappear on the next checkout) and then `rmSync(..., { recursive: true, force: true })`
as a fallback for anything untracked. That is correct, real deletion, not a dry-run-only report.

Also re-ran the exact production function to be sure — imported `cleanIsWindowsIllegalName` straight
out of `📜️script.ts` (guarded by `import.meta.main`, so importing it doesn't trigger the CLI) and ran
it over all 72772 tracked paths directly: **0 illegal names**, matching the independent scan above.

## Conclusion / what changed

No code was changed. There is nothing in `📜️script.ts` to fix:

- `cleanIsWindowsIllegalName` / `cleanCollectWindowsIllegal` / `cleanRemovePath` already detect and
  really delete (via `git rm -f`, not a soft report) any Windows-illegal filename, including ones
  buried inside an open ticket (that removal kind deliberately bypasses ticket-open protection).
- The repo currently has **zero** such filenames (verified twice, once against the real production
  function), so there's nothing for `clean` to delete right now for the failure mode described.

**The actual blocker is almost certainly path length, not illegal characters** (see above: 204
tracked paths would exceed Win32's 260-char `MAX_PATH` with even a modest checkout root). That is
not something `clean` should "delete its way out of" — most of the long paths are legitimate,
deliberately deep taxonomy-driven content (e.g. norm/DIN16798 mutation-test fixtures at 244-246
chars), and two of the worst-offending ticket folders are open, one of them being actively written
to by another session during this investigation. Auto-deleting long-path content would destroy real
work for a length problem that has a standard, non-destructive fix on the Windows side instead:

```
git config --global core.longpaths true
```

plus enabling the Windows 10/11 `LongPathsEnabled` registry key (or the machine-wide Group Policy
equivalent) so NTFS/Win32 itself accepts paths beyond 260 characters. This is the fix to hand to the
Windows dev — not a repo-side deletion pass.

Did not touch `SEMANTIC-MUTATIONS-OVERHAUL` or `ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY`
content — both open, one actively being written to during this session.
