# Audit nct-1019: no-cross-technology and ui-foundation-closure

Auditor `aud-nct`, read-only on source. Brief: `brief-1019.md`. Inputs: `result-lay15.md`, `lay13-tools/` (per-area method). Outputs: `🗑️generated/aud-nct-tools/` (cruise config, runner, extraction) and `🗑️generated/aud-nct-run/` (per-area JSON, `edges.json`, `head-preexist.json`, `appendix2.md`). Only read-only git commands were used (`log`, `show HEAD:`, `ls-files`, `diff --quiet`).

## Result

- Measured now: `no-cross-technology` 431, `ui-foundation-closure` 22. That is 453 distinct violating edges from 163 importing files. Lay15 final was 290 and 22.
- Since lay15 the no-cross-technology count rose by 141, all in the areas `splug2` (68 to 167), `splug3` (25 to 51), `splug4` (66 to 72), `splug1` (30 to 36), `mit` (10 to 14). Other areas are unchanged.
- The rise is produced by `tax2` (router-law codemod, `lift.ts` and `restructure.ts`). It wrote 135 files that account for 415 of the 453 edges (92%). These are owner or sibling modules that `tax2` split out of `📜️script.ts` routers: 92 package-commands modules and 43 sibling modules. The 158 package-commands modules on disk are all `tax2` writes; 66 of them carry no violation. Lay15 attributed the package-commands wave to peers. The attribution was wrong: all 158 package-commands modules on disk are `tax2` writes.
- Mechanism: both rules exempt `📜️script.ts` on the importing side (`from.pathNot`). When `tax2` moved code out of a script into a `🟦️.ts` module, the same relative imports became visible. 363 of the 415 tax2 edges (about 87%) resolve to a specifier the HEAD script already contained (approximate: resolved-target match over the HEAD script text). The count rose by relocation of existing coupling, not by new coupling, but the relocated modules are in the wrong place (see groups G1 and G2).

## Attribution

| creator | files | edges | evidence |
|---|---:|---:|---|
| tax2 `lift`, package-commands owner module `<owner>/🧪️tests/🔬️<lang>-package-commands/🟦️.ts` | 73 | 219 | ledger `lift written-accepted`, `lift.ts:93` |
| tax2 `restructure`, package-commands owner module | 19 | 60 | ledger `restructure written-accepted`, `restructure.ts:125` |
| tax2 `lift`, sibling module next to a script | 13 | 54 | ledger; 4 `🌱️value` files carry 20 ui-foundation edges |
| tax2 `plain-lift`, sibling module | 29 | 78 | ledger |
| tax2 `restructure`, sibling module | 1 | 4 | ledger (`♻️mit-bestand/📋️bericht/🔨️modules/📄️documents`) |
| fixups4 | 1 | 3 | `result-fixups4.md` line 10 (new `🧪️tests/🧿️semio-member-store-producer-source`, untracked, 19:23) |
| pre-existing: committed in HEAD, or edited in place | 24 | 32 | `git log` commits `0c3e78944af` (10-14) to `bd9e4e02bd0` (10-19); edges already in HEAD |
| unattributed | 3 | 3 | untracked `🧭️coordination/{🎞️scenes,🔁️story-reducers}` (19:00); gitignored `🤖️generated/🧩️plugins` (15:13) |

Executor notes:
- `result-tax2.md` does not exist. The tax2 work is `tax2-tools/` (last write 20:04). Its ledger `🗑️generated/tax2/ledger.jsonl` was present at 20:00 (used for this attribution) but has since been removed; `aud-nct-run/edges.json` and `tax2-pc-writes.json` keep the extracted rows.
- `tax` (`result-tax.md`, 18:08) and `cli-fix` (`result-cli-fix.md`, 18:05): no file in the violation set.
- `law-sweep`: no report or tool directory exists in the ticket. The latest law report is `result-law5.md` (10-18 22:18).
- `fixups4`: one file (G5).

## Method

- Per-area cruise, lay13 method. Config `🗑️generated/aud-nct-tools/area-config.cjs` wraps the lint base `🧰️framework/.../🧹️lint/🕸️dependency-boundaries/🟨️.cjs` and keeps only the rules `^no-cross-technology*` and `ui-foundation-closure`. Traversal is limited to the area (`doNotFollow`), as in lay13. Runner `cruise-all.sh`, `perl alarm 570`, node 8 GB heap, sequential.
- Areas: the 11 lay15 areas, plus `hub`, plus `rest` (top-level folders that no area covered: `🎓️teaching`, `🏢️semio-tech`, `👴️leutwiler`, `🧪️tests`, `🧫️fixtures`, `🧬️schema`; 0 edges).
- Cruise window 19:50 to 20:01 local. `tax2` kept writing until at least 20:04, so the numbers are a snapshot.
- Extraction `edges.mjs`: deduplicated by rule, from and to; specifier and dependency types from the module record. Attribution: `tax2` ledger membership (module or script path), `git log`, `git show HEAD:<script>` to resolve the same specifiers in the HEAD script (approximate: resolved-target match), report and tool-directory greps, mtimes.

Per-area (lay15 final, now; ui-foundation unchanged):

| area | no-cross-technology lay15 to now | ui-foundation-closure |
|---|---|---|
| mit | 10 to 14 | 0 |
| hub | 31 to 31 | 0 |
| splug1 | 30 to 36 | 0 |
| splug2 | 68 to 167 | 0 |
| splug3 | 25 to 51 | 0 |
| splug4 | 66 to 72 | 0 |
| smod | 60 to 60 | 0 |
| fwmodules | 0 | 22 to 22 |
| repo, osrest, osrenderer, fwrest, rest | 0 | 0 |

## Groups and the correct structure

### G1: package-commands owner modules (92 files, 279 edges), tax2 lift and restructure

What happens: `lift.ts` (`ownerOf`, line 93) and `restructure.ts` (line 125) take the command classes out of `<owner>/📦️packages/<lang>/📜️script.ts` and write them to `<owner>/🧪️tests/🔬️<lang>-package-commands/🟦️.ts`. The script keeps its imports and imports the new module. Example: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️tests/🔬️rust-package-commands/🟦️.ts` imports `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`. The HEAD script imported the same path. Targets: framework `🏃️process` (routing, execution, capacity, cargo testing) 262 edges, repo `📚️library` 12, other framework 5. The three hub package-commands modules (`🌎️hub/🧩️compositions/{📕️norm,🌍️gis}`, `🌎️hub/🔨️modules/🛡️admin`) carry 9 of the no-cross-technology edges.

Correct structure:
1. Production command code does not belong under `🧪️tests/`. The module belongs beside its script in the owning package (`<owner>/📦️packages/<lang>/🟦️.ts`). Fix `ownerOf` in `lift.ts` and `restructure.ts`.
2. Cross-technology access goes through the framework's workspace specifier (`@semio-tech/framework/<key>`, `aliased-workspace`). The rule exempts that dependency type. A `../../…/🧰️framework/…` relative path is what the rule flags. The export keys need the deps owner (`package.json` is frozen).
3. A Rust-technology check that needs Rust sources reads them as text (or calls the Rust tool). It does not import TypeScript from another technology. This is the brief's rule for package-commands tests.
4. The repo CLI closure (`⌨️cli`, `🏗️build`) stays relative per the brief. Those modules sit in the framework technology and produce no edges in the repo area.

### G2: sibling modules lifted from scripts in value, hub and mit (13 files, 54 edges)

- `🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/🟦️.ts`, `…/🧬️retained-clone/📦️original-slot/🟦️.ts`, `…/🛬️decode/🧳️owned/🌳️value/👥️tuple/🟦️.ts`, `…/🌱️value/🟦️.ts`: 20 ui-foundation edges. The `value` foundation now imports `🏃️process` (execution, owned-execution, capacity, cargo testing, routing). The HEAD `📜️script.ts` of `🧳️record` already imports `🏃️process` routing, owned-execution and cargo modules (checked with `git show HEAD:`). Scripts are exempt from ui-foundation-closure.
- `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🟦️.ts` (8 edges) and `♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/🟦️.ts` and similar (10 edges): as G1.

Correct structure: `🌱️value` is a foundation module and may depend only on ui and other foundation modules. The verification bodies must not sit in value. Move them to the composition root or to `🏃️process/🧪️testing`, and keep the value runtime free of process imports. If a seam is needed, use the bind-once host pattern from lay15 (ui resident). Do not add a `🧪️tests` exemption.

### G3: plain-lift sibling modules in plugins and hub (29 files, 78 edges)

Examples: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/…/⚙️engine/📺️renderer/🎛️repl/📺️view-state/🟦️.ts`, `…/🧲️geometry-targets/…`, `…/🪩️canvas/🟦️.tsx`, `✏️s/🔌️plugins/🏗️fem/…/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts`. All 78 specifiers are already in the HEAD scripts.

Correct structure: a plugin reaches the framework only through its SDK or the workspace specifier (the rule `plugins-framework-sdk-only` already states this). A sibling module must not import framework internals by relative path. Use the alias, or keep the code in the exempt script until the alias keys exist.

### G4: restructure sibling (1 file, 4 edges)

`♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🟦️.ts`. Same fix as G3, alias imports.

### G5: fixups4 (1 file, 3 edges)

`✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🧿️semio-member-store-producer-source/🟦️.ts`, untracked, 19:23. `result-fixups4.md` creates it as a verify lane. It is a production verify module under `🧪️tests` with relative framework imports, the same pattern as G1. Correct structure: the lane reads the Rust sources as text (the report says the lane reads Rust outside the project), and the module moves out of `🧪️tests` next to its owner.

### G6: pre-existing in HEAD, or edited in place (24 files, 32 edges)

Not created by tax2. The 32 edges of these 24 tracked files were already present in their HEAD versions (heuristic check: 29 matched directly, 3 in the energy and gismap files use specifier forms the check does not parse). The 6 edges that are new in the working tree are G5 (3) and G7 (3). Examples:
- `🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🟦️.ts` (changed 15:13): `@semio-tech/framework/hash`. The alias passes no-cross-technology but ui-foundation-closure flags it, because `hash` is not a foundation module.
- `🧰️framework/🔨️modules/🕹️interaction/🟦️.ts` (committed in `bd9e4e02bd0`): a barrel `export *` into `📡️replication`.
- `✏️s/🔌️plugins/🏗️fem/🧪️tests/🧪️shared-authority/🟦️.ts` (7 edges; edited in place 19:30; `fem-fix` lists the suite; the edges are already in HEAD).
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🧪️tests/🌡️thermal/🟦️.ts`, `…/📐️cad/…/📺️renderer/…` (8 files, committed in `8f91299152c`), `✏️s/🔌️plugins/🧩️puzzle/…/⏳️precompute/🪣️fill/🎟️caller/🧪️tests/🟦️.ts`.

Correct structure: alias imports or moving the code to its owner, per G1 and G3.

### G7: unattributed (3 files, 3 edges)

- `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🎞️scenes/🟦️.ts` and `…/🔁️story-reducers/🟦️.ts` (untracked, 19:00). No report names these paths. `result-split-s.md` (15:28) names the folder as a split target, which is the only lead.
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts` (15:13, gitignored generated output).

## Recommendations

1. Stop the `tax2` lift and restructure codemod until `ownerOf` is fixed (`lift.ts:93`, `restructure.ts:125`), or revert the 158 package-commands modules and the 43 sibling modules. Moving a script's imports out of the exempt script only moves the count.
2. Fix G1 and G5 by alias imports (after the deps owner adds the keys) and by moving the modules out of `🧪️tests`. Text-read Rust sources instead of importing them.
3. Fix G2 at the layering level: the value foundation must not import process. This is a real layering violation the exemption had hidden.
4. The `package.json` and root `📜️script.ts` freeze (19:45) still applies. Queue the alias keys and the root and import edits until the lift.
5. Re-run this cruise after the codemod stops; the numbers are a snapshot taken 19:50 to 20:01.

## Verification

- Ran: per-area cruise for 13 areas (`cruise.log`, `aud-nct-*.json`), `edges.mjs`, `build-appendix` (`appendix2.md`), `head-preexist.mjs`, `c-head.mjs`.
- Not run: any build, test, cargo, or product activation. No source file was edited.

## Appendix: every violating edge, grouped by importing file

Format: `file (mtime local, git state, creator)`, then each edge as `rule: target via specifier`. Rule names: `xt` means no-cross-technology, `ui-foundation-closure` as is. Group IDs match the sections above.

### G1a: tax2 lift, package-commands owner module (73 files, 219 edges)

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🀄️wfc/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:13, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:38, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:13, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/💠️lowpoly/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:13, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:43, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:13, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/⏱️budget/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/⏱️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:13, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts` via `../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:13, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🖨️raster/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧬️schema/🔮️oracles/✅️validator/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧬️schema/🔮️oracles/✅️validator/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:13, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/⏱️budget/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/⏱️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts` via `../../../../../🧰️framework/📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:57, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts` via `../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts` via `../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts` via `../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts`
- `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🍞️bun/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🍞️bun/🟦️.ts`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🧑‍💻dev/🏗️fem/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts`
- `✏️s/🧑‍💻dev/📋️forms/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts`
- `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts`
- `✏️s/🧑‍💻dev/📕️norm/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts`
- `✏️s/🧑‍💻dev/🗒️note/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:14, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts`
- `✏️s/🧑‍💻dev/🪐️space/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts`
- `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:14, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - hub → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts`
- `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - hub → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🖨️describe/🏷️component-receipt/🟦️.ts` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🖨️describe/🏷️component-receipt/🟦️.ts`

### G1b: tax2 restructure, package-commands owner module (19 files, 60 edges)

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/🗂️catalog/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/🫀️catalog-documents/🧪️tests/🔬️rust-package-commands/🟦️.ts` (mtime 10-19 19:49, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`

### G2: tax2 lift, sibling module next to a script (13 files, 54 edges)

- `♻️mit-bestand/📋️bericht/🔨️modules/👥️actor-network/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - mit → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - mit → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts`
  - mit → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - mit → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/📥️artifacts/🟦️.ts` via `../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/📥️artifacts/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts` via `../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts` via `../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - mit → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts` via `../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts` via `../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/🟦️.ts` (mtime 10-19 19:06, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🌎️hub/🧩️compositions/🗄️stdio/📇️publication/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🟦️.ts` (mtime 10-19 19:14, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - hub → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts` via `../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts`
- `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🌎️hub/🧩️compositions/🪐️space/🧪️testing/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🟦️.ts` (mtime 10-19 19:57, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📋️context/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📋️context/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - hub → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/⌨️native-entrypoint/📦️modules/🟦️.ts` via `../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/⌨️native-entrypoint/📦️modules/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts` via `../../../../../🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts` via `../../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` via `../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🟦️.ts` (mtime 10-19 19:07, untracked, tax2 ledger)
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`
  - ui-foundation-closure: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`

### G3: tax2 plain-lift, sibling module next to a script (29 files, 78 edges)

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔬️probes/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts` via `../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts` via `../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts`
- `🌎️hub/🧩️compositions/🗄️stdio/🏭️bridge/🟦️.ts` (mtime 10-19 19:35, untracked, tax2 ledger)
  - hub → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`

### G4: tax2 restructure, sibling module (1 files, 4 edges)

- `♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🟦️.ts` (mtime 10-19 19:42, untracked, tax2 ledger)
  - mit → framework: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` via `../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📦️artifacts/🎛️operation/🟦️.ts` via `../../../../🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📦️artifacts/🎛️operation/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📦️artifacts/📥️material/📤️producer/🟦️.ts` via `../../../../🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📦️artifacts/📥️material/📤️producer/🟦️.ts`
  - mit → framework: `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` via `../../../../🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts`

### G5: fixups4 (untracked, new 19:23) (1 files, 3 edges)

- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🧿️semio-member-store-producer-source/🟦️.ts` (mtime 10-19 19:23, untracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`

### G6: pre-existing in HEAD (committed before this session, or edited in place) (24 files, 32 edges)

- `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚀️renderer-boot-selection/🟦️.ts` (mtime 10-16 06:04, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/📺️renderer/🟦️.ts` (mtime 10-19 19:00, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🧪️testing/🎚️config/🟦️.ts` via `../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🧪️testing/🎚️config/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts` (mtime 10-19 19:30, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🌿️variant-field/🟦️.ts` via `../../../../../../../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🌿️variant-field/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🧪️tests/🧪️shared-authority/🟦️.ts` (mtime 10-19 19:30, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧪️color-input/🟦️.ts` via `../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧪️color-input/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️entity-id/🔣️.json` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️entity-id/🔣️.json`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️render-revision/🔣️.json` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️render-revision/🔣️.json`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️entity-id/🟦️.ts` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️entity-id/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🔣️.json` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🔣️.json`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🟦️.ts` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🧪️tests/🛍️canvas-catalogue/🟦️.ts` (mtime 10-16 06:35, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🔮️oracles/🖱️mounted-input/🟦️.tsx` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🔮️oracles/🖱️mounted-input/🟦️.tsx`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🧫️fixtures/🖱️input-contract/🔣️.json` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🧫️fixtures/🖱️input-contract/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🎛️repl/📺️view-state/🟦️.ts` (mtime 10-19 00:19, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts` via `../../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧲️geometry-targets/🎨️pick-styling/🟦️.ts` (mtime 10-19 00:19, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts` via `../../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧲️geometry-targets/🎯️pick-model/🟦️.ts` (mtime 10-19 00:19, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts` via `../../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧲️geometry-targets/🟦️.tsx` (mtime 10-19 00:19, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts` via `../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧾️repl-model/🟦️.tsx` (mtime 10-19 00:19, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts` via `../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🪩️canvas/🟦️.tsx` (mtime 10-19 00:19, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts` via `../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🧪️tests/🌡️thermal/🟦️.ts` (mtime 10-19 18:20, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/🟦️.ts` via `../../../../../../../../../🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/🟦️.ts`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🔲️enclosure/⚡️exchange/🟦️.ts` (mtime 10-19 12:29, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/↔️inverse/🟦️.ts` via `../../../../../../../../../🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/↔️inverse/🟦️.ts`
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/🟦️.ts` via `../../../../../../../../../🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/🟦️.ts`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧪️tests/🟦️.ts` (mtime 10-19 18:20, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/⏯️tool-run/📨️payload/🌊️stream/🟦️.ts` via `../../../../../../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/📨️payload/🌊️stream/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧪️tests/🔬️unit/🟦️.ts` (mtime 10-17 23:46, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🎨️paint/🟦️.ts` via `../../../../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🎨️paint/🟦️.ts`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🎯️pixel-selection/🟦️.ts` (mtime 10-17 18:16, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.ts` via `../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🪪️session-factory/🟦️.ts` (mtime 10-19 01:15, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🔮️oracles/🪪️session-double/🟦️.ts` via `../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🔮️oracles/🪪️session-double/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🎟️caller/🧪️tests/🟦️.ts` (mtime 10-19 18:12, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🧪️tests/🟦️.ts` via `./../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🧪️tests/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts` (mtime 10-16 21:14, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts` via `../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts`
- `✏️s/🧑‍💻dev/🚀️entry/🟦️.ts` (mtime 10-14 03:20, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts` via `../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts`
- `✏️s/🧪️tests/🏘️plugin-composition/🌀️procedural-formats/🟦️.ts` (mtime 10-16 20:10, tracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts` via `../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts`
- `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🧪️tests/🟦️.ts` (mtime 10-19 01:15, tracked)
  - hub → framework: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🎭️selection/🧪️tests/🧫️fixtures/🔣️.json` via `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🎭️selection/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🟦️.ts` (mtime 10-19 15:13, tracked)
  - ui-foundation-closure: `🧰️framework/📦️packages/🟦️typescript/🔏️hash/🟦️.ts` via `@semio-tech/framework/hash`
- `🧰️framework/🔨️modules/🕹️interaction/🟦️.ts` (mtime 10-19 19:03, tracked)
  - ui-foundation-closure: `🧰️framework/🔨️modules/📡️replication/🧾️wire/🪪️scoped-identity/🟦️.ts` via `../📡️replication/🧾️wire/🪪️scoped-identity/🟦️.ts`

### G7: unattributed (untracked or gitignored generated) (3 files, 3 edges)

- `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🎞️scenes/🟦️.ts` (mtime 10-19 19:00, untracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🟦️.ts` via `../../../../../../🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🟦️.ts`
- `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🔁️story-reducers/🟦️.ts` (mtime 10-19 19:00, untracked)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🟦️.ts` via `../../../../../../🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts` (mtime 10-19 15:13, ?)
  - ✏️s (plugins/dev) → framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts` via `../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts`
