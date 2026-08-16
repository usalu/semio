# W2-C — Registry, Gates, Launch Report

Lane: **W2-C registry/gates/launch** (Sonnet 5). Contract: `📋️contract-freeze.md` §3/§4. Built on
`📓️w0-i-report.md` (composite taxonomy + `plugin-dependency/parity` + `plugin-dependency/contribution-target`
gates), `📓️dependency-inventory.md` (61 declarations / 40 owners), `📋️ownership-and-handoffs.md`.

## Files touched (exact lease)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts` — exclusive lease.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/*` — regenerated (not hand-edited).
- `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json` — my lease's launch config; edited the seed and regenerated, per `🖥️launch.ts`'s own instruction to never hand-edit the output.
- `📜️script.ts` (repo root) — two one-line `export` additions only (see "Out-of-lease fix" below).

## 1. `dependsOn` in the generated catalog

`PluginRegistryEntry`/`PluginBuildTarget` gained `readonly dependsOn: readonly string[]`, sitting beside
`contributes`/`consumes`. New `parseCargoPluginDependencyIds(manifestText, ownId)` in the registry script
scans each crate's own Cargo manifest text for every `semio-s-plugin-<id>` entry (both the renamed
`key = { …, package = "semio-s-plugin-x" }` shape and the plain `semio-s-plugin-x = { … }` shape) —
byte-identical regex to the root policy script's `policyCargoPluginDependencyIds` (`📜️script.ts:7562`),
so the catalog and the `plugin-dependency/parity` gate can never disagree about what one crate's Cargo
dependency set is. `parsePluginCargo` calls it and, per contract freeze §4 rule 1, places an extension's
`extends` target first: `dependsOn = extendsHost ? [extendsHost, ...cargoIds.filter(id => id !== extendsHost)] : cargoIds`.

Verified against `📓️dependency-inventory.md`: e.g. `demonstrator → [cad, gis, procedural, process,
puzzle, sourcing, stdio]`, `procedural → [flow-extension-brep, …7 extensions…, stdio]`,
`flow-extension-brep → [flow, stdio]` (extends-target first). A crate's own registry-derived
`dependsOn` total across all 59 entries is 80, not the inventory's 61 — the 19-entry gap is real
extension `extends` edges (contract §4 rule 1) with **no** matching Cargo dependency (an extension
crate never literally `Cargo`-depends on its host; the wasm-component composition supplies that link,
not `rustc`). Confirmed by mechanically reproducing the root policy's exact owner-root/id-exclusion
logic standalone: it also yields 61. Not a bug — both numbers are correct for what they each measure.

No `VersionReq` is emitted: path-based sibling deps in this workspace carry no `version =` field (spot
checked `➗️mathematical`'s and others' `Cargo.toml`), so nothing is derivable there yet; `dependsOn`
stays `readonly string[]`, matching `contributes`/`consumes`'s shape. The runtime `.depends_on(id,
VersionReq)` API (W1-A) remains the authority for the versioned edge once plugins adopt it — this
catalog field is the pre-adoption, ground-truth (Cargo-derived) view the `plugin-dependency/parity`
gate's medium-priority direction already establishes.

**Handoff, not done here (out of lease):** `PluginCatalogTarget`/`PluginCatalog` in
`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` (owned by W2-B / not W2-C's lease) do not yet carry
`dependsOn` — `🟦️catalog.ts`'s `toCatalogTarget` still only maps `pluginId/wasmOut/role/contributes/consumes`.
The generated data is there; wiring it into the runtime `PluginCatalog` the browser host actually reads
is a follow-up for whoever owns that file.

## 2. Transitive closure in `resolveRegistryPluginIdsForFilter`

The function now does two additive passes over the same `ids: Set<string>`:
1. unchanged single-level topic scan (`contributes ∩ consumes` against the target entry) — kept because
   some consumption (e.g. `demonstrator` consuming `forms.questionKind`) is not backed by any Cargo edge;
2. **new**: a worklist closure over every gathered id's `dependsOn` edges, transitively, until fixed point.

Before this change, a plugin's dev session never pulled in its own Cargo dependencies unless a
contributes/consumes topic happened to also match — `demonstrator`'s dev session (`bun nx dev demonstrator`
or equivalent) never built/loaded `cad`, `gis`, `procedural`, `process`, `puzzle`, `sourcing`, or `stdio`,
even though the playground scout findings (`📓️scout-2-group-undo-and-hosts.md`) call out demonstrator's
cad↔puzzle **instances** as the browser proof for this ticket's pilots. Verified live:

```
resolveRegistryPluginIdsForFilter("demonstrator") →
  cad, demonstrator, flow, flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text},
  gis, procedural, process, process-extension-{concrete,metal,robotic,wood}, puzzle, sourcing, stdio
resolveRegistryPluginIdsForFilter("sourcing-module-beams") → sourcing-module-beams, sourcing, stdio   (2-hop closure)
resolveRegistryPluginIdsForFilter("flow-extension-brep")   → flow-extension-brep, flow, stdio
```

`sourcing-module-beams → sourcing → stdio` is a genuine 2-hop chain the old code could not reach at all.
No other consumer needed patching: `generatePluginRegistry`'s `filterPlaygroundPlugin` option,
`buildPlaygroundSession`, `validatePlaygroundSessions`, and `os/dev`'s `📜️script.ts` (outside my lease,
confirmed via grep — it only imports `generatePluginRegistry`/`isHostPluginFilter`/`writePlaygroundSession`)
all funnel through this one function, so they inherit the fix without their own edits.

## 3. Launch entries

Added to `.vscode/🧩️launch.seed.jsonc`'s `4_gate` presentation group, immediately after the last existing
entry (`⚖️gate🏗️artifact-builder-migrated`, order `410.95`) and before the next unrelated group
(`🎥️render🎬️animate-video`) — same shape as every sibling `⚖️gate<x>` entry (`node-terminal`,
`bun -e 'const m = await import("./📜️script.ts"); const b = m.policy<X>Breaches(process.cwd()); …
process.exit(b.length === 0 ? 0 : 1);'`):

- `⚖️gate🔗️plugin-dependency-parity` — order `410.96` — calls `policyPluginDependencyParityBreaches`.
- `⚖️gate🎯️contribution-target` — order `410.97` — calls `policyContributionTargetBreaches`.

**Out-of-lease fix required to make these work:** both functions existed in root `📜️script.ts` but were
never `export`ed (unlike every sibling `policy*Breaches`, e.g. `policyArtifactBuilderMigratedBreaches`),
so the launch entries' `import("./📜️script.ts")` would have resolved `m.policyPluginDependencyParityBreaches`
to `undefined` and thrown at runtime — confirmed by reproducing the exact failure before fixing. Added the
`export` keyword to both function declarations (`📜️script.ts:7581`, `7628`) — no other line touched,
verified via `git diff` showing exactly those two `-function`/`+export function` hunks.

**Stale-seed find, restored (not caused by this lane):** `.vscode/🧩️launch.seed.jsonc`'s pre-existing
`⚖️gate🗄️stdio-catalog` entry still had the retired `bun -e '...policyStdioCatalogBreaches...'` command,
while the already-committed `.vscode/launch.json` (commit `63686457bd`, 2026-08-16 02:50:31) has
`bun nx run workspace:stdio-quick` — the FULL-STDIO ticket's own landed change. `git log --date=iso`
confirms the seed's last commit is `7ad8955884` (this ticket's own start commit, i.e. untouched since
before this ticket began) while the generated output was committed later — a pre-existing staleness,
not introduced here. Running `generate` from the stale seed would have silently reverted that other
ticket's launch entry. Restored the seed's one line to match the already-committed command before
regenerating, per the shared-tree rule to never revert foreign changes; did not touch anything else in
that block. `git diff` on `.vscode/launch.json` now shows exactly 22 pure-addition lines (the two new
gate entries), nothing else.

## Gate output

`bun ./📜️script.ts check` from `📇️registry/` (exit 0):
```
plugin registry catalog is fresh (59 plugin crates, 58 playgrounds, 23 framework packages); .vscode/launch.json is fresh.
```
(same pre-existing `manifest-without-marker`/`ambiguous-lang-shape`/`unknown-lang` discovery-problem
noise `📓️w0-i-report.md` already documented — unrelated to this lane, not gating.)

`bun ./📜️script.ts policy` from repo root (exit 1 overall — 24,729 pre-existing high-priority breaches
across unrelated rules, none touched by this lane; confirmed by diffing two runs' summaries a few
minutes apart — only `os-state-authority/item-scope-global` moved, 334→320, from other live sessions'
concurrent edits, not mine). Isolated the two gates this ticket owns directly:
```
plugin-dependency/parity: 61 (medium: 61)
plugin-dependency/contribution-target: 0
```
Matches `📓️w0-i-report.md`'s baseline exactly (61 medium / 0) — no regression from the registry/launch
changes in this lane.

## Notes for later waves

- `dependsOn` is Cargo-ground-truth today; once plugins adopt `.depends_on(id, VersionReq)` (W1-A's API),
  the `plugin-dependency/parity` gate's medium findings shrink to zero and the registry's `dependsOn`
  stays valid unchanged — it never read the runtime API, only Cargo.
- W2-B (or whoever owns `🎠️kernel/🟦️component.ts`) should add `dependsOn` to `PluginCatalogTarget` and
  wire it through `🟦️catalog.ts`'s `toCatalogTarget` if the browser host needs the dependency graph at
  runtime (today it's registry-build-time / dev-filter only).
- The stdio-catalog seed/launch.json staleness pattern (committed generated output diverging from its
  own seed) is worth a repo-wide audit outside this ticket — flagged, not investigated further here.
