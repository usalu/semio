# W2 — Dead barrel-export-path detector (`index-lint`)

Scope: one new lint, `PluginIndexExportPathLintScript`, detecting dead relative `from "..."`
specifiers in every plugin's `📦️index.ts` barrel. Report-only, standalone, never wired into
`verify`/`plugin lint`. This is a **detector only** — the actual fix (repointing 517 dead paths at
the migrated `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/` shape) is deliberately NOT done here, per this
agent's boundary (no plugin `📦️index.ts` edits, no `✏️s/🔌️plugins/**` edits), and remains unowned.

## What was added

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`

- `//#region 🔖️PluginIndexExportPathLint` (~:1666–1723, inserted directly after
  `//#endregion 🔖️CapabilityLayeringLint` and before `class TestScript`):
  - `PLUGIN_BARREL_RELATIVE_EXPORT_PATTERN` (~:1684) — `/from\s+"(\.[^"]+)"/g`, matches every
    relative `from "..."` specifier in a barrel file.
  - `resolvesPluginBarrelExport(baseDir, spec)` (~:1689) — tries, in order: the literal path,
    `+".ts"`, `+".tsx"`, `+"/📦️index.ts"`, `+"/index.ts"`, exactly the resolution set the ticket's
    verified 517/567 count used.
  - `class PluginIndexExportPathLintScript extends BundleScript` (~:1693) — for every
    `✏️s/🔌️plugins/<plugin>/📦️packages/🟦️typescript/📦️index.ts` that exists on disk, counts total
    vs. dead relative specifiers, prints one `console.warn` line per plugin with a
    dead-path breach (`[plugin-index-export-path-lint] WARN <path>: <dead>/<total> ... (<cause>)`),
    and a final `[DEBUG]` summary line with the grand total. **`run()` never throws** — there is no
    `blocking`/`grandfathered` split like `PluginCapabilityLintScript`/`CapabilityLayeringLintScript`
    use; it is unconditionally non-blocking.
  - Cause message: when any dead spec for a plugin contains `🗿️artifacts/`, the WARN line names
    "likely pre-standards path (`🗿️artifacts/<a>/🧬️schema/…`) against the migrated
    `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/` tree"; otherwise a generic "target does not exist on disk".
    In practice every breach this run found was the artifacts-path case.
- Router (~:2699–2706): `.register("index-lint", PluginIndexExportPathLintScript)`, added right
  after the existing `.register("layer-lint", CapabilityLayeringLintScript)` entry, with a comment
  explaining explicitly why this one is *not* folded into `"plugin"`/`"lint"` the way `layer-lint`
  was.

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json`

- Added an `"index-lint"` nx target, byte-for-byte the same shape as the existing `"layer-lint"`
  target (`nx:run-commands`, same `cwd`, same `env`, `command: "bun ./📜️script.ts index-lint"`).

## Reproduced counts — matches the verified 517/567 exactly

Standalone reproduction (outside the real script, same resolution algorithm) and the real
`index-lint` run both print **517/567 dead relative export path(s) across 31 plugin(s)** (33
plugins total; 2 clean: `🌀️procedural` 0/22 and `🗄️stdio` 0/28 — both already point at paths that
still exist). Per-plugin dead/total, sorted worst-first:

```
📕️norm: 180/180
🧩️puzzle: 36/36
🧱️block: 36/36
🏗️fem: 24/24
🌍️gis: 24/24
🔱️trinity: 24/24
📐️cad: 13/13
🔋️energy: 12/12   📖️playbook: 12/12   🖍️draw: 12/12   📏️layout: 12/12
➗️mathematical: 12/12   📋️forms: 12/12   🗒️note: 12/12   🌿️vcs: 12/12
✒️writer: 12/12   🕸️dag: 12/12   🌊️flow: 12/12   🖨️raster: 12/12
💠️lowpoly: 3/3   🎥️shooting: 3/3   🏛️architect: 3/3   🪵️sourcing: 3/3
📜️imperative: 3/3   🪐️space: 3/3   🎞️animate: 3/3   🎬️sequence: 3/3
💡️reasoning: 3/3   🎪️demonstrator: 3/3   🏭️process: 3/3   📸️remodel: 3/3
🌀️procedural: 0/22
🗄️stdio: 0/28
TOTAL: 517/567
```

This exactly matches the ticket's worst-offender list (`📕️norm` 180/180, `🧱️block` 36/36,
`🧩️puzzle` 36/36, `🔱️trinity` 24/24, `🌍️gis` 24/24, `🏗️fem` 24/24, `📐️cad` 13/13, then a tail of
12/12s) and the 517/567 total — no adjustment needed.

## Why it cannot gate

`PluginIndexExportPathLintScript.run()` has no failure path at all — it only ever `console.warn`s
per-plugin breaches and `console.log`s a summary, then returns. This is a different mechanism from
`PluginCapabilityLintScript`/`CapabilityLayeringLintScript`'s `KNOWN_*_VIOLATIONS` grandfather-set
pattern (those *do* throw on any failure not in a hand-picked, evidence-backed allowlist). That
pattern doesn't fit here: 517 individual dead-path strings have no sane per-entry grandfather list,
and unlike the capability/layering lints' backlog (a bounded, named set of crates whose fix is
tracked), the real fix here is repointing every one of 517 specifiers in `📦️index.ts` files this
agent is explicitly forbidden to touch, and which no session currently owns. So the check is wired
in as its own standalone router command (`index-lint`) and nx target, **never called from
`VerifyScript` or from the `"plugin"`/`"lint"` router entry** — confirmed by grep, the class name
`PluginIndexExportPathLintScript` appears exactly twice in the file: its own definition and the
`index-lint` registration.

## Verification — real output pasted

### `bun ./📜️script.ts index-lint` (direct)

Full output: `w2-index-lint-direct.txt` in this folder. Tail:

```
[DEBUG] plugin index export path lint: 517/567 dead relative export path(s) across 31 plugin(s) — REPORT ONLY, does not gate (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE)
```

(First attempt hit a parse error — `plugins/*/📦️packages` inside the docstring contains a literal
`*/`, which prematurely closed the `/** ... */` block comment. Fixed by rewording to
`plugins/<plugin>/📦️packages`; reran clean.)

### `bun nx run @semio-tech/framework-os-dev:index-lint`

Full output: `w2-index-lint-nx.txt`. Exit code: **0** (`echo "EXIT_CODE=$?"` → `EXIT_CODE=0`).
Tail: `NX   Successfully ran target index-lint for project @semio-tech/framework-os-dev`.

### Proof it does not change `plugin lint`'s pass/fail

`bun nx run @semio-tech/framework-os-dev:plugin lint` (the target the repo-root `verify gate` calls
unconditionally) after adding `index-lint`: full output `w2-index-lint-pluginlint-after.txt`.
Still fails with:

```
error: plugin capability lint failed (69 issue(s), 59 plugin package(s) evaluated)
```

**Identical** 69-issue count to the pre-existing baseline documented in `📓️w2-lint-report.md`
("FAILS — but not because of anything in this wave's scope... plugin capability lint failed (69
issue(s), 59 plugin package(s) evaluated)"), which that report traced to UCAS's in-flight stdio
rollout — unrelated to and unaffected by this addition. `index-lint` is not called anywhere in that
code path (confirmed by grep above), so this is the expected result, not a coincidence.

## Scope confirmation

```
$ git diff --stat -- "🧰️framework/.../🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts" \
                     "🧰️framework/.../🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json"
 .../📋️project.json | 12 ++++
 .../📜️script.ts    | 64 ++++++++++++++++++++++
 2 files changed, 76 insertions(+)

$ git diff --stat -- "📜️script.ts"     # repo-root script — empty, not touched
(no output)
```

`git status --porcelain -- "✏️s/🔌️plugins"` shows unrelated concurrent-session churn (renames under
`🌀️procedural`, deletions under `➗️mathematical`, an `AGENTS.md` edit) — none of it made by this
agent; no `📦️index.ts` under any plugin was opened or edited by this wave.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` — added
  `PluginIndexExportPathLintScript` + helpers (region `🔖️PluginIndexExportPathLint`) and the
  `"index-lint"` router registration.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json` — added
  the `"index-lint"` nx target.
- This report.
- Scratch: `w2-index-lint-direct.txt`, `w2-index-lint-nx.txt`, `w2-index-lint-pluginlint-after.txt`
  in this ticket folder (verification output pasted above).

Nothing else. No plugin `📦️index.ts`, no file under `✏️s/🔌️plugins/**`, and repo-root `📜️script.ts`
were opened or edited by this wave.

## Explicit note on scope

**The fix is not done here.** Repointing the 517 dead relative export paths in 31 plugins'
`📦️index.ts` barrels at the migrated `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/` shape is out of this
agent's boundary (editing `📦️index.ts` / `✏️s/🔌️plugins/**` was explicitly forbidden) and remains
unowned. This wave adds only the detector.
