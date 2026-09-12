# Lane Q3 — Policy-Breach Gates Through Nx (2026-09-12)

Scope (phase 2 follow-up to lane P5): the 12 `⚖️gate…` policy-rule launch entries P5 explicitly left as
inline `bun -e` evals (no CLI surface existed to route them through Nx), plus the 3 confirmed-dead
`animate-video` entries P5 also left as-is pending confirmation.

## 1. New `verify policy-breach <rule>` subcommand

Root `📜️script.ts`'s `VerifyScript` already had the exact family/subcommand shape to reuse: several
existing single-rule gates (`verify mutation-outcome-law`, `verify semantic-vocabulary`, `verify
package-purity`, `verify abstraction-ownership`) each call one `policy*Breaches(repoRoot)` function,
print its findings, and throw when high-priority breaches exist. Added:

- A module-level `POLICY_BREACH_GATES: Record<string, (repoRoot: string) => BreachRecord[]>` map
  (placed immediately above `class VerifyScript`) pairing each of the 12 launch-entry slugs to its
  existing breach function (`policyArtifactBuilderBreaches`, `policyArtifactDecomposerBreaches`,
  `policySchemaRepresentationBreaches`, `policyIoSerializerMatrixBreaches`,
  `policyIoTerminalityBreaches`, `policyCodecFidelityBreaches`, `policyStandardsCoverageBreaches`,
  `policyArtifactAnalyzerBreaches`, `policyArtifactComposerBreaches`,
  `policyArtifactBuilderMigratedBreaches`, `policyPluginDependencyParityBreaches`,
  `policyContributionTargetBreaches`) — no new policy logic, only a CLI surface.
- `VerifyScript.run()`: a `segments[0] === "policy-breach"` branch calling a new private
  `runPolicyBreach(rule)` method.
- `runPolicyBreach`: looks up `rule` in `POLICY_BREACH_GATES`, prints `breaches.length` then one
  `kind | scope | summary` line per breach (byte-identical to the old inline eval's
  `console.log(b.length); for (...) console.log(x.kind, "|", x.scope, "|", x.summary)`), and throws when
  `breaches.length > 0` (unknown rule also throws, listing known rules).

Verified byte-identical stdout against the original inline eval for `schema-representation` (334 lines,
`diff` clean) and matching exit codes across all 12 rules run individually: 7 currently pass with 0
breaches / exit 0 (`artifact-builder`, `artifact-decomposer`, `io-serializer-matrix`, `io-terminality`,
`codec-fidelity`, `plugin-dependency-parity`, `contribution-target`); 5 currently fail with pre-existing
breaches / exit 1 (`schema-representation` 318, `standards-coverage` 436, `artifact-analyzer` 57,
`artifact-composer` 57, `artifact-builder-migrated` 56) — these breach counts are pre-existing policy
debt, unaffected by this lane (same functions, same repo state, only the CLI surface is new).

## 2. Nx targets — root `📋️project.json`

Added 12 authored targets after `verify-abstraction-ownership`, matching that block's shape exactly:

```json
"verify-policy-breach-<rule>": {
  "executor": "nx:run-commands",
  "cache": true,
  "options": { "command": "bun ./📜️script.ts verify policy-breach <rule>" },
  "outputs": []
}
```

for `<rule>` in `artifact-builder`, `artifact-decomposer`, `schema-representation`,
`io-serializer-matrix`, `io-terminality`, `codec-fidelity`, `standards-coverage`, `artifact-analyzer`,
`artifact-composer`, `artifact-builder-migrated`, `plugin-dependency-parity`, `contribution-target`.

Inputs are automatic: the caching `.mjs` plugin's `targetScriptClosure`/`genericTargetCommandInputs` scan
each target's `options.command` for a token ending in `📜️script.ts`, resolve it, and hash the root
script's own import closure — no `inputs` needed. Caching is additionally guaranteed independent of the
explicit `cache: true` by `verifyCommand()` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:29`),
which matches any target whose command contains the standalone word `verify` and forces `cache: true`
regardless — confirmed via `cacheableFamily`'s `^verify(?:-|$)` pattern too (both fire for these 12
names/commands).

## 3. Launch entries — `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` (identical edit in both)

All 12 `⚖️gate…` entries: `command` rewritten from the inline `bun -e 'const m = await
import("./📜️script.ts"); const b = m.policy…Breaches(process.cwd()); …'` to
`bun nx run workspace:verify-policy-breach-<rule>`. Names, `cwd`, `presentation.group`/`order` untouched.

| launch name | rule slug |
| --- | --- |
| `⚖️gate🏗️artifact-builder` | `artifact-builder` |
| `⚖️gate🪓️artifact-decomposer` | `artifact-decomposer` |
| `⚖️gate🧬️schema-representation` | `schema-representation` |
| `⚖️gate🚪️io-serializer-matrix` | `io-serializer-matrix` |
| `⚖️gate🗄️io-terminality` | `io-terminality` |
| `⚖️gate💾codec-fidelity` | `codec-fidelity` |
| `⚖️gate🏅️standards-coverage` | `standards-coverage` |
| `⚖️gate🧐️artifact-analyzer` | `artifact-analyzer` |
| `⚖️gate🎹️artifact-composer` | `artifact-composer` |
| `⚖️gate🏗️artifact-builder-migrated` | `artifact-builder-migrated` |
| `⚖️gate🔗️plugin-dependency-parity` | `plugin-dependency-parity` |
| `⚖️gate🎯️contribution-target` | `contribution-target` |

## 4. Removed 3 dead launch entries (both files)

`🎥️render🎬️animate-video`, `👁️preview🎬️animate-video`, `🧹️flush-cache🎬️animate-video` — confirmed dead
before removing: `cwd: "${workspaceFolder}/animate/video/rs"` does not exist anywhere in the repo (no
`animate/` directory at the workspace root; only unrelated `storybook-static/…/animate` and
`.storybook/stories/animate` paths exist), and root `📜️script.ts`'s router has no `render`/`preview`/
`flush-cache` registration (`rg '\.register\("render"|\.register\("preview"|\.register\("flush-cache"'`
→ 0 matches). Orphaned entries from a removed feature; deleted rather than routed through Nx since there
is no source to route to.

## Verification

- **`bun build --target=bun --no-bundle 📜️script.ts`** — exit 0, both before and after the launch-file
  edits.
- **JSONC/JSON parse + config counts** (`Bun.JSONC.parse` / `JSON.parse`):
  - `.vscode/launch.json`: parses; 2484 → 2481 (−3 removed dead entries; the 12 rewrites don't change
    count).
  - `.vscode/🧩️launch.seed.jsonc`: parses; 1394 → 1391 (−3, same reason).
  - `📋️project.json`: parses.
  - Both baselines (2484/1394) match lane P5's own reported counts exactly, confirming no interference
    from concurrent peer edits between P5's close and this lane's start.
- **`NX_DAEMON=false bunx nx show project workspace --json`**: all 12
  `verify-policy-breach-<rule>` targets present, each with `"cache": true`.
- **Two targets run twice** (`verify-policy-breach-artifact-builder`,
  `verify-policy-breach-contribution-target`, both currently 0 breaches / exit 0): first run ~1.5s /
  0% cache, second run **100% cache hit** ("Nx read the output from the cache instead of running the
  command for 1 out of 1 tasks"), ~450-590ms.
- **`git diff` sweep** on all 4 touched files confirms only the intended lines changed — `.vscode/*`
  diffs also show three unrelated concurrent-lane additions (two `🛠️dev🔧️procedural🏙️3d👁️viewer…`
  entries, the trinity-jack-shell/cad-brep-invoke `cargo` rewrites from lane P5) that this lane did not
  touch, confirming no collision.
- **Launch validator** (`bun ./📜️script.ts verify interactivity apps`): still fails with the same 782
  pre-existing, unrelated findings lane P5 already reported (stale 512-config capacity constant, 6
  missing exact `⚖️gate⚡️interactivity*` registrations, malformed `✏️s/🔌️plugins/✒️writer/**` plugin
  descriptors). None of the 782 lines reference any name this lane touched (`artifact-builder`,
  `contribution-target`, `policy-breach`, `animate-video`, etc. all absent from the failure list) —
  pre-existing red, unrelated to this lane, not fixed here.

## Files touched

- `📜️script.ts` — new `POLICY_BREACH_GATES` map, `VerifyScript` `policy-breach` branch +
  `runPolicyBreach` method.
- `📋️project.json` — 12 new `verify-policy-breach-<rule>` targets.
- `.vscode/launch.json` — 12 `⚖️gate…` command rewrites, 3 dead `animate-video` entries removed.
- `.vscode/🧩️launch.seed.jsonc` — identical 12 rewrites + 3 removals.

No files deleted beyond the 3 dead launch-entry blocks (source-level removal, not file deletion). Scratch
verification output was written under this ticket's `🗑️generated/q3/` during the session and deleted
once captured in this report, per ticket rules.
