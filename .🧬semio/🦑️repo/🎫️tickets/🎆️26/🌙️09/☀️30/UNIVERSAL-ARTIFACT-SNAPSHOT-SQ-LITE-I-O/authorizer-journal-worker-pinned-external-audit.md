# Authorizer Journal Worker Pinned External Audit

Read-only current runtime closure and pinned module source audit, 2026-10-14. No producer edit, feature test or Native dispatch. Isolated dependency scanner only read source and wrote `🗑️generated/authorizer-worker-external-census.json`; it found38 runtime closure files.

New journal worker export from authorizers `📦️runtime/🟦️.ts:10` imports actual journal `👷️worker/📜️script.ts`. That worker28 requires three pinned Nx modules. `nx/src/native` was already admitted. Exactly two newly required external intrinsic names are `nx/src/utils/consume-messages-from-socket` and `nx/src/daemon/socket-utils`; refusal of the former is a genuine closed-list omission before readiness tests, not missing-await regression execution.

Full current pinned Nx set, independently scanned:

- nx/src/tasks-runner/create-task-graph
- nx/src/hasher/task-hasher
- nx/src/utils/workspace-context
- nx/src/daemon/client/client
- nx/src/native
- nx/src/utils/consume-messages-from-socket
- nx/src/daemon/socket-utils

Builtins (including node:sqlite) remain system modules. `require.resolve("nx/package.json")` in worker26 selects the actual loaded toolkit marker; it is a resolution lookup, not a scanned external runtime import or reason to admit nx wildcard/package aliases.

Owner repair: actual `🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts:10` should append only these two names to existing closed set. Share this exact first-party required-tool list with compiler owner `🔏️authorizers/📜️script.ts:21`, replacing its current broad `external:["nx/*","bun:sqlite"]` with the exact seven names plus existing independently justified bun:sqlite. Preserve source capture refusal for every unlisted package and immutable no-follow/hash/metafile coverage. No wildcard, package installation, product runtime dependency, seal disablement or catch-and-ignore needed. First-party string list interfaces export only repository-owned data, not Nx classes/types.

Actual pinned modules expose functions worker uses: consumeMessagesFromSocket and parseMessage from utils/consume-messages-from-socket; sendMessage from daemon/socket-utils (current signature supports forced serializer and delegates framing). Native Watcher/WorkspaceContext stay behind existing first-party journal adapters. Worker public interface consists of repository-owned FilesystemJournalWorkerOptionsV1 and repository-owned journal/control types; keep all runtime Nx types internal.

Receipt owner already ties actual bundle bytes to frozen manifest/sourceSha256, checks no-follow stable outputs, and worker rejects tooling identity mismatch before native/socket birth. Journal RPC `loadedFilesystemJournalToolingIdentityV1` validates loaded Nx package, immutable directory digest, authorizer receipt and exact patch file hashes. Adding required closed names changes compiler/capture source bytes and tooling identity naturally; no receipt format/protocol widening is necessary. New modules belong to existing pinned Nx installation and lock/patch authority; maintain actual SDK resolution and identity checks.

Root minimal @repo/test-cases await correction and updated real-bootstrap fixture reexports are separate saved source. Source78862 first attempt and HighStore86352 stopped during authorizer external closure admission; no readiness assertion RED or changed feature runtime can be claimed from them. Historical Source79366 stale parser remains authentic before evidence. Root owns next unchanged registered execution after closed-list correction.
