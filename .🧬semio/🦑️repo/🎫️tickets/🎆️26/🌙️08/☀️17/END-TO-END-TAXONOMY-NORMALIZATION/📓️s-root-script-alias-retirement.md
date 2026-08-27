# Root Script Alias Retirement

## Implementation release

The root command router no longer creates, overwrites, or recreates ASCII `script.ts` at main startup or during `SetupScript.runGit`. Its Windows hardlink fallback was removed with that alias path. The unrelated `CLAUDE.md`/`GEMINI.md` setup behavior and root `project.json` alias remain unchanged.

The mutation-law self-scan exemption now contains only the schema-authorized `📜️script.ts`. Root command examples and the existing Neo4j devcontainer wrapper's description use that canonical filename. Its executable command was already canonical.

**No actual root alias was deleted.** Its exact physical retirement remains with the coordinator. Because the existing policy walker considers non-directory symlinks, broad policy scans should follow that retirement; the retained physical alias no longer has a compatibility exemption.

## Exact physical preimage

Read-only observations:

| Field | Value |
| --- | --- |
| Path | `script.ts` |
| Node kind | Symbolic link |
| Filesystem mode | `0755` |
| Link-text size | 16 bytes |
| Link target | `📜️script.ts` |
| Link-text SHA-256 | `57db1a7a0a0b4c74f507af2645c048e8e170e25ba00682d91231376bf9c1a3ad` |
| Git index mode | `120000` |
| Git index object | `91d51be192428c743e55fff9c1abb6e0791bca25` |

The canonical target was a regular file, mode `0644`, 2,721,869 bytes, SHA-256 `41b79b924e5f26fd4fec5907e7f2e75adaccf0a031ff9ee926012491432d00cc` at the target snapshot. Other root-router work continues concurrently, so this target hash is an observation, not a future source-drift exemption. The alias link-text tuple is the relevant deletion preimage.

Safe retirement is removal of that exact symlink only after rechecking its node kind and link text. Do not follow it, delete its target, replace an unexpected regular file, or modify `project.json`. The taxonomy `root-script` fixed contract authorizes `**/📜️script.ts`, not ASCII `script.ts`.

## Consumer census

The no-follow census enumerated 49,569 tracked/untracked nonignored Git candidates with opaque exclusions applied before filesystem access. It searched source/configuration/documentation text and found 431 lexical mentions across 163 files. Exact compressed evidence is retained in `🧪️root-script-alias/📇️census/🔣️.json`. A second census of additional configuration, project, schema, feature, and extensionless text names examined a 49,572-candidate snapshot and found zero additional mentions. No symlink ancestor was traversed.

There is no remaining executable root-ASCII-alias caller in that census. Root package scripts, root Nx targets, the launch seed, `.mcp.json`, Cursor/VS Code MCP configuration, and the Neo4j wrapper already execute `📜️script.ts`.

Remaining lexical mentions are principally retained `.cursor/plans` history, comments and generated provenance labels, owner-local styling manifest labels, three immutable plugin `AGENTS.md` instructions, and repo-library README guidance. The README belongs to the separately frozen 40-leaf owner catalog; its stale prose was not changed opportunistically under this alias task. Those are not executable dependencies on the root alias. No historical plan or `AGENTS.md` was edited.

The first census attempt exceeded the default subprocess output buffer before source scanning; it was rerun with a bounded 64 MiB Git metadata capture. All reported counts come from completed runs. No actual `compose/` or `temp/compose/` path was read or traversed, and no Git mutation occurred.

## Test-first evidence

The language-neutral vectors define the canonical filename, retired filename, two entrypoints, three platform behaviors, and absent/regular-file/symlink preimages. The existing schema filename is checked with Ajv, and the actual startup/setup source regions are extracted with the TypeScript parser. Bun and TypeScript independently compile and execute each isolated filesystem fixture; their resulting files must agree.

Before the change: **0 pass, 3 fail, 7 assertions**. Both actual code regions recreated the alias, and the parser found three forbidden runtime alias literals.

After the change:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️root-script-alias.test.ts'
3 pass, 0 fail, 67 assertions, 5.04 seconds
```

The runtime matrix covers 18 entrypoint/platform/preimage combinations with each compiler. Startup and setup leave the retired path exactly as found. Windows symlink-privilege failure still exercises the unrelated instruction-alias hardlink behavior. Actual repo setup/Git configuration was not executed; unrelated setup infrastructure is stubbed inside the isolated fixture.

## Changed files

- Root `📜️script.ts`: only alias startup/setup/self-policy regions and its command-example line.
- `.devcontainer/neo4j-mcp-run.sh`: canonical filename in its existing description only.
- Ticket `🧪️root-script-alias.test.ts`, language-neutral vector, compressed census, and this report.

No fixture process remains from the alias packet. The separate live CAD plan capture was still running when this report was written; it does not affect alias test status.

## Coordinator Physical Retirement

The coordinator subsequently rechecked the exact alias preimage above and removed only `/Users/ueli/Documents/semio/script.ts` using `apply_patch`. The postcheck found the alias absent. The canonical `📜️script.ts` remained the same regular file, mode `0644`, inode `116368958`, 2,721,890 bytes, SHA-256 `d48fa7de135069b1871bacfd96aa7387f3f8ae7b00b923123e1d1361cd77822d`, immediately before and after removal. The read-only scoped Git diff reports only the alias deletion for that path.

The deleted item was a sixteen-byte symbolic-link payload, not the command router. Its exact link text and indexed preimage are retained above, making the link recoverable if explicitly requested. No `project.json` alias or actual Compose tree was touched. This records one completed physical retirement, not whole-monorepo convergence.
