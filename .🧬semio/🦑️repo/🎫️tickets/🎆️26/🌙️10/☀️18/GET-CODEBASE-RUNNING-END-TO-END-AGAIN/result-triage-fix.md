# Result triage-fix (2026-10-22)

Scope: minimal root fixes from `audit-cap-1022.md` section B (triage preview). Live edits, no git writes, no sub-agents, no builds beyond `bun test` and `bun build`.

## Changed files (library root `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`)

1. `🔍️discovery/✅️owned-file-approvals/🟦️.ts`
   - New helper `previewCommandRefusal(cause)`: keeps the caught validator reason in the message and attaches `cause`. Used at the two former `catch (cause)` sites (`direct` and `declared` parsing), so the reason from `generatorPreviewScriptArguments` / `parseGeneratorPreviewLiteralCommandV1` reaches the thrown message (was: generic message, reason only on `.cause`).
   - Owner-cwd spawn: `command` is now `process.versions.bun ? process.execPath : "bun"` (same guard as `🏃️process/🍞️bun` `viteDevInvocation` and the cargo relay at `🏃️process/🟦️.ts:144`; `runBun` uses `process.execPath`). The return line is shared by the `.`, framework and owner cases, so all three spawn the running bun instead of a PATH lookup.
2. `🧹️normalization/🔍️inventory-planning/🟦️.ts` (spawn failure, was line 316)
   - `result.error.message` is appended as `reason=...` and attached as `cause`. The stdout/stderr hashes are unchanged.
3. `🔍️discovery/🔐️taxonomy-authority/🟦️.ts` (was line 410)
   - `catch` now keeps the caught message in the problem text (`... in its owner project: <reason>`).
4. New test `🧪️tests/🔬️workspace-contract/🧾️owned-preview-refusal-reason/🟦️.ts` with data fixture `🔣️refusals.json` (language-agnostic data: one refusal case with expected reason substring, one accepted owner-cwd case with expected args and cwd).

## Commands and outcomes

- `bun test ./🧰️framework/.../🧾️owned-preview-refusal-reason/🟦️.ts` (run from `/Users/ueli/Documents/semio`): 2 pass, 0 fail.
- `bun test ./🧰️framework/.../🧾️generator-preview-and-dispositions/🟦️.ts`: 2 pass, 7 skip, 0 fail (existing neighbour test).
- `bun build --target=bun <file> --outfile /dev/null` for the three edited sources: built without errors (entry point output, no diagnostics).

## Not verified / open

- The inventory-planning spawn-failure path (`result.error`, e.g. ENOENT) and the taxonomy-authority problem text are not covered by a test; they are verified by reading and the bun build only. A live triage preview run was not done.
- Audit B2 said only the owner case spawns bare `bun`. Reading the code, all three cases return `command: "bun"`; the fix covers all three through the single return.
- The audit's suggested route through `./🔨️modules/🏃️process/📜️script.ts command` was not taken: that script has a `#!/usr/bin/env bun` shebang, so it still needs a bun spawn, and it needs a `--manifest` decision (UNKNOWN in the audit). The running-bun route is the existing neighbour convention.
- Node fallback: when `process.versions.bun` is absent the code still returns bare `"bun"` (same guard as neighbours).
- Messages are plain English strings in these modules; no EN/DE catalog or schema declaration exists for them, so none was added. Flagged for the i18n owner.
- No changes to `AGENTS.md`, no git writes, no processes started.
