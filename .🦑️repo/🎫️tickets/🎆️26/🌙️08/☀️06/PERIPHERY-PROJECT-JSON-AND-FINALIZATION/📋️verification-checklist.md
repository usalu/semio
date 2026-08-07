# Verification Checklist — W10/W11

Ticket `26/08/06/PERIPHERY-PROJECT-JSON-AND-FINALIZATION` · snapshot `2026-08-06T12:59:25.597Z`.

## Live inventory snapshot

| Metric | Count |
|--------|------:|
| Remaining `⚡️implementations` / `⚡️implementation` dirs | **92** |
| Singular `⚡️implementation` | 1 |
| Outside `🧰️framework` | 0 |
| `📦️glue.rs` under `📦️packages` | **72** |
| `📦️lib.rs` under `📦️packages` | **0** |
| `🟦️glue.ts` under `📦️packages` | **0** |
| `📦️index.ts(x)` under `📦️packages` | **49** |
| `📋️project.json` referencing impl | 87 |
| Stale Shape V2 project.json (cwd/inputs) | **0** |

Regenerate: `bun ./📜️script.ts inventory` from this ticket folder.

## Periphery applied (verify locally)

- [x] `.dependency-cruiser.cjs` loads taxonomy from `📚️library/🔣️taxonomy.json` (`node -e "require(./.dependency-cruiser.cjs)"`)
- [x] `REPO_CLIENT_GO` / `REPO_MCP_GO` point at `⌨️cli` / `🔌️mcp` module roots
- [x] `.devcontainer/post-create.sh` builds `./…/🔌️mcp`
- [x] `.gitignore` ignores `**/📦️packages/**/Cargo.lock`
- [x] dsl-derive + os-host Shape V2 `📋️project.json` no longer reference impl paths
- [x] Taxonomy `entryFilenames` / `packagingFileNames` use `📦️glue.rs` + `🟦️glue.ts`
- [x] `assert_taxonomy_components` requires **artifact-owned** `📚️examples` and **forbids** plugin-root `📚️examples` (already correct — no code change)

## Pre-W10 gate (blocking)

- [ ] Impl dir count → **0** (excl. compose exempt)
- [ ] `📦️lib.rs` under packages → **0** (done: 0)
- [ ] TS package entries migrated off `index.ts(x)` → `🟦️glue.ts` (now 49 remaining)
- [ ] `cargo metadata` green
- [ ] `go.work` / Go builds via module roots (already aligned)
- [ ] Vitest `KNOWN_BROKEN` cleaned
- [ ] Plugin registry path no longer under `⚡️implementations`
- [ ] Registrar flip: dep-cruiser + policy + verify gate → error on forbiddenPathSegments

## Do not

- Physically delete OS/compiler `⚡️implementations` from this ticket
- Edit root `Cargo.toml` / `package.json` / `bun.lock` / `nx.json` / `go.work` in periphery passes unless a listed safe one-liner (go.work already clean)
