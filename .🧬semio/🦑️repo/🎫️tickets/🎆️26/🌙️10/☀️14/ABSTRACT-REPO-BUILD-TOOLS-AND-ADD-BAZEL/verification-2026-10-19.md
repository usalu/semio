# Workspace Recovery Verification — 2026-10-19

## Status

**The complete codebase is not yet qualified as running.** The implemented repairs and focused runtime contracts below pass. Complete dependency provisioning, installed Nx execution, and Bazel cold/warm integration remain blocked or unverified. The covering ticket remains open; no goal was opened, reopened or closed.

The fleet used GPT 6.1 Sol High execution agents and GPT 6.1 Sol Low read-only exploration/audits, rotating across the maximum four concurrent slots. Low is the supported setting corresponding to the requested Light setting. All work used the shared checkout, without modifying Git commands or worktrees.

## Implemented and Verified

| Area | Result | Evidence |
| --- | --- | --- |
| Public root command startup | 4 tests passed, 41 assertions; direct root test command and independent esbuild startup output agree | [Entrypoint report](./entrypoint-2026-10-19.md) |
| Neutral selection, help, Windows descriptor/publication and dependency platform admission | 18 tests passed, 1,051 assertions | [Execution report and exact files](./execution-2026-10-19.md) |
| Windows conditional file publication | 3 tests passed, 65 assertions, included in the preceding aggregate; five positive byte cases and eleven hostile cases | [Publication report](./publication-2026-10-19.md) |
| Descriptor implementation and both defining suites | Strict TypeScript compilation passed without diagnostics | [Execution report](./execution-2026-10-19.md) |
| Bazel selected inventory and action fingerprints | 4 tests passed, 100 assertions; independent AJV, SQLite, Minimatch and actual issued Bun-child oracles | [Bazel report and exact files](./bazel-2026-10-19.md) |
| Incomplete fixed-image birth preservation | 2 tests passed, 52 assertions; normal dependency command crossed the prior missing-receipt failure and retained original inputs | [Storage report and exact files](./storage-2026-10-19.md) |
| Pre-effect Windows host privilege admission | 2 tests passed, 86 assertions; actual public dependency invocation refused before allocation | [Storage report](./storage-2026-10-19.md) |
| Independent source and runtime audit | No reproducible additional P1/P2 defect; startup and synchronous Windows read independently rerun | [Integration audit](./audit-integration-2026-10-19.md) |
| Launch routing and path audit | JSON parses, owned interfaces and targets resolve in source; scoped new-file audit found no path over 256 UTF16 units | [Final audit](./final-audit-2026-10-19.md) |

The root script loads command owners, policy checks and verification lanes only when selected. Its Repo route uses the original build CLI birth contract. Developer command registration no longer imports the OS MCP gateway before that route is selected. Bilingual IDE launches expose setup, build, test, run, dev and selected tasks through the neutral interface.

Windows rooted storage now supports ordinary ancestor access, empty stream censuses, the sharing required by parent-relative rename, opaque preservation/retention, conditional file publication and synchronous descriptor reads. These primitives retain original bytes and identity checks instead of constructing workspace links manually.

Bazel now avoids repository-wide fallback traversal for a selected input closure. Fingerprints use the actual issued task environment, distinguish absent from empty environment variables, and frame each runtime result to prevent concatenation collisions. Local action/disk caching exists in the backend; shared remote caching remains intentionally disabled by its current local action contract. Conservative dependency-output snapshots may still cause avoidable invalidation, as documented in the Bazel audit.

## Actual Remaining Blockers

1. **Host storage privilege.** The current Windows process token does not contain `SeManageVolumePrivilege`. Actual `Mount-DiskImage` returned HRESULT `0x80070522`. The new admission gate reports that prerequisite before opening leases, recovering inputs or allocating images. The current Native image interface requires a genuine finite mounted filesystem; an ordinary directory cannot truthfully substitute for that contract. No synthetic volume, receipt, global permission change or UAC bypass was introduced.
2. **Full producer qualification.** Because mounted cache backing is unavailable, complete locked dependency provisioning and installed Nx/Bazel execution have not passed here. Missing workspace aliases in the unmanaged dependency environment also prevent the full aggregate suites from loading. Mounted-root reparse handling remains an explicit unverified Windows integration point.
3. **Pinned Bazel isolated launch.** The exact authored Bazel 8.4.2 Windows x64 binary was checksum/size verified. Its actual owned `version` invocation failed with `ClassNotFoundException` before any build in the disposable Unicode ticket workspace. The class exists in the installed server JAR. Temporary SUBST/8.3 aliases and a scoped UTF8 JVM probe did not resolve that launcher path issue. This does not establish a failure in the normal short ASCII managed cache path, which could not be provisioned due to the first blocker. No cold/warm, unrelated-edit, selected-edit, disk-cache or cancellation acceptance is claimed from that attempt.
4. **Other desktop environments.** Full native macOS, Linux and devcontainer execution were not run in this Windows session. The original host Node version also differs from the repository pin, as recorded in the initial audit.

The next qualification must run the supported setup and backend contracts from a host with the required storage capability, then verify the actual mounted producer, installed Nx tasks, pinned Bazel cache reuse and selective invalidation. The IDE launch file contains the supported entry points. Startup tests alone do not satisfy that qualification.

## Files and Cleanup

Coordinator-owned source/configuration changes:

- `📜️script.ts`
- `📋️project.json` — only the new command-startup target belongs to this fleet; concurrent unrelated changes were preserved.
- `.vscode/launch.json` — shared launch configuration; concurrent additions were preserved.
- `🧪️tests/🧭️entrypoint/🟦️.ts`
- `🧪️tests/🧭️entrypoint/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts`
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` — shared targeted Windows publication changes; execution-agent additions were preserved.
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json` — explicit Windows publication protocol.
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/📄️publication/🟦️.ts`

The execution, storage and Bazel reports contain their exact additional file lists. Coordination, audits and research Markdown remain in this ticket. Handcrafted proof inputs remain under `bazel-smoke/`; compiler/tool outputs, downloaded proof binaries, logs and temporary drive mappings were removed after evidence was recorded. Retained original VHD generations remain at the exact `.cache/b/birth-*` paths listed in the storage report and are not disposable generated output. No source was copied into the ticket as a versioned snapshot. No AGENTS.md file was edited.
