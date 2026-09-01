# Census — `package-implementation-destination-unresolved` / `package-role-unresolved`

Scope: `🧰️framework/🔨️modules`. Baseline `bb06c41f73f0122fbed315b7487428b976f99921`, read from
`$T/🗑️temp/🔣️vocab-plan.json` before any edit (that shared scratch artifact has since been
overwritten/rotated by a concurrent worker in the shared `🗑️temp/` — my own captured copy of the
`.unresolved` rows for these two codes is preserved at `🗑️temp/pkgdest/rows.json`, 115 rows).

Measured baseline for my two codes: **113** `package-implementation-destination-unresolved` + **2**
`package-role-unresolved` = **115**. The assignment brief quoted 117+2=119; I could not reproduce
119 from a captured artifact and report the number I actually measured rather than the assigned one
(CLAUDE.md: never assume). The 4-row gap is plausibly concurrent taxonomy.json edits between when
the brief's number was captured and when I captured mine — not investigated further, out of scope.

## Mechanism recap (confirmed by reading `🧹️normalization/🟦️.ts`, not assumed)

For every file inside a `📦️packages/<lang>` boundary, `classifyPackageRole` runs `classifyGlue`
against the boundary's `packageGlueGrammar`. `packageGlueGrammar.*.allowedRoles` never includes
`"implementation"` — `packageBoundaryRules.*.implementationRole = "problem"` by schema validation
(line ~1542), so ANY file whose content classifies `"implementation"` is *always* a violation; the
question is only whether `packageImplementationDestination` can compute where it should move
(→ `package-implementation-destination-unresolved` if not) or whether `classifyGlue` itself cannot
decide declaration vs. implementation at all (→ `package-role-unresolved`).

## Group-by-file-shape (all 115 rows)

| filename shape | n | ecosystem |
|---|---:|---|
| `🦀️*.rs` real GPU/backend/runtime source (`resources.rs`, `pipelines.rs`, `layout.rs`, `frame.rs`, `scene_target.rs`, `types.rs`, `context.rs`, `dispatch.rs`, `entity.rs`, `reconcile.rs`, `surface.rs`, …) | 83 | 🦀️rust |
| `🟦️*.ts` real TS source (`mailbox.ts`, `wire-turn.ts`, `shard-client.ts`, `shard-runtime.ts`, `turn-scheduler.ts`, `runtime.ts`, `render.ts`, `build-tooling.ts`) | 8 | 🟦️typescript |
| `🧪️vitest.config.ts` (data-literal config) | 7 | 🟦️typescript |
| `package.json` | 8 | 🟦️typescript |
| `tsconfig.json` | 2 | 🟦️typescript |
| `🎨️postcss.config.ts` / `🟦️eslint.config.ts` (data/call-literal config) | 2 | 🟦️typescript |
| `🟦️vitest.setup.ts` (real setup code) | 1 | 🟦️typescript |
| `📦️glue.rs` | 2 | 🦀️rust |
| `🦀️backend_alias.rs` (cfg-gated `pub type` glue) | 1 | 🦀️rust |
| `🟨️*.js` real source | 2 | 🟨️javascript |
| `tests/*.rs` (Cargo integration-test fixtures) | 2 | 🦀️rust |
| `uv.lock` | 1 | 🐍️python |
| `🔣️*.schema.json` | 1 | 🦀️rust boundary |

By co-occurring OTHER violation on the same row (grouping, not double-counting my two codes):

| co-occurring code(s) | n | disposition |
|---|---:|---|
| `fixed-contract-ambiguous` | 10 | **fixed** — engine bug, §1 |
| *(none — pure `package-role-unresolved`)* | 2 | **fixed** — engine bug, §2 |
| `semantic-stem-unresolved` only | 97 | 8 **fixed** (§3, TS/JS config-literal misclassification); 1 blocked by a cross-ecosystem gate one layer earlier than `classifyGlue`, already being worked concurrently (§3 tail); 1 needs a new `fixedFilenameContracts` entry (`uv.lock`, §4); **87 genuine vocabulary gap**, §5 |
| `reference-syntax-unsupported` + `semantic-stem-unresolved` | 5 | 3 **fixed** (§3); 2 remain — vocabulary gap **and** a `reference-syntax-unsupported` issue outside my class |
| `semantic-stem-ambiguous` only | 2 | **not fixed** — `render.ts`/`wire-turn.ts`, §6 |

**20 of 115 rows fixed** by the three engine changes below (verified with a live `clean taxonomy
plan` re-run, not just static analysis); **95 rows remain**, almost all requiring new
`semanticDirectoryKinds` vocabulary (§5), which I do not have permission to add.

## §1 — `package.json` / `tsconfig.json`: fixed by matching, not by moving the file

Root cause: `🔣️taxonomy.json` has exactly two `fixedFilenameContracts` pairs sharing one
`pathPattern` — `node-package-manifest` (`scope.kind: "package-root"`) vs.
`nx-owned-node-package-manifest` (`scope.kind: "sibling-fixed-filename-contract"`, requiring an
adjacent `nx-project-manifest` = `📋️project.json`, which every package boundary in this repo has) —
and the same for `typescript-config` vs. `nx-owned-typescript-config`. `fixedSpecificity()`
(`🧹️normalization/🟦️.ts:2963`, pre-fix) only ever distinguished `"path-pattern"` scope (rank 0) from
everything else (rank 1) — every OTHER scope kind tied. Whenever a `package.json` sits in a
package-root directory that also holds a sibling `📋️project.json` (i.e. always, in this repo), BOTH
contracts match with equal specificity → `fixed-contract-ambiguous` → `canonical.fixedId` stays
unset → `classifyPackageRole` never takes its `fixedId ⇒ "configuration"` shortcut →
`packageImplementationDestination` gets called and immediately returns `null` (its very first line
bails when `canonical.fileKind` is `null`, which it always is on the ambiguous-fixed-contract path)
→ `package-implementation-destination-unresolved`. Exactly matches the brief's prediction: "several
already are; work out why these instances are not matching."

Verified this is the **only** pattern collision in the whole schema (Python check against the live
`🔣️taxonomy.json`: exactly these two pairs share a `pathPattern`), so the fix cannot silently flip
any other file's contract elsewhere in the repo.

**Fix**: graded the specificity ladder from a binary 0/1 into 7 explicit ranks
(`FIXED_CONTRACT_SCOPE_SPECIFICITY`, `🧹️normalization/🟦️.ts`), narrowest wins:
`path-pattern(0) < repository-root(1) < directory-kind(2) < package-root(3) <
fixed-directory-contract(4) < sibling-fixed-filename-contract(5) < exact-path(6)`. The
`sibling-fixed-filename-contract` variant now deterministically wins over `package-root`, matching
its own stated purpose ("owned by an adjacent exact Nx project manifest"). Both contracts still leave
the file's on-disk name untouched (`configurability: "unconfigurable"`) — only which contract ID
gets attributed changes, which is exactly what unblocks `classifyPackageRole`'s `fixedId` shortcut.

**10 rows fixed**: the ambiguity disappears (10 `fixed-contract-ambiguous` gone) **and**
`package-implementation-destination-unresolved` disappears for the same 10 rows as a direct
consequence (role becomes `"configuration"` before `packageImplementationDestination` is ever
called).

## §2 — `package-role-unresolved` (both rows): declarative-grammar gaps, not misplaced code

Both real files are legitimate wiring, structurally identical in spirit to the already-accepted
exemplar `🔄️machine/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (33-line thin `mod` re-export) — the
grammar just couldn't prove it:

1. `📡️replication/📦️packages/🦀️rust/📦️glue.rs` (95 lines) — a crate-root re-export aggregator using
   `pub mod codec { #[path = "..."] mod x; pub use x::*; }` **block-form** module wiring to namespace
   several re-exports together. `classifyGlue`'s Rust declaration regex only recognized
   semicolon-terminated `mod x;`/`use x;` statements — a `mod x { ... }` block (even one containing
   nothing but further mod/use/pub-use statements) broke the flat grammar, and with zero `fn` in the
   file, `bodies.length > 0 ? "thin-delegation" : "unresolved"` fell to `"unresolved"`.
2. `🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️backend_alias.rs` (28 lines) — four `#[cfg(...)] pub type
   ActiveBackend = <concrete-backend>::Backend;` statements selecting the compiled backend at
   compile time (U3 in that packet's own docstring: no `dyn`/vtable, a `cfg`-selected concrete type
   alias instead). `pub type` aliases were never in the declaration regex's alternatives at all.

**Fix**, both empirically verified against the real files (`classifyPackageGlueContent`, exported for
testing) before touching either regex:
- Added a `type` alias alternative to the flat declarative-statement regex
  (`RUST_DECLARATIVE_STATEMENT_SEQUENCE`).
- Added `stripDeclarativeRustModuleBlocks`, which recursively strips `mod name { ... }`/`pub mod name
  { ... }` blocks whose entire body is *itself* purely declarative (same grammar, applied
  recursively) before the flat check runs — a non-declarative block is left untouched, so the outer
  check correctly still fails on its stray braces.

**2/2 rows fixed** — both now classify `"declaration"` (an allowed role for the `rust` grammar).

## §3 — TypeScript/JavaScript "config literal" misclassified as `implementation`

`classifyGlue`'s `typescript`/`javascript` branch had no case for pure-data default exports; its
final fallback called anything that wasn't a name-matched thin function `"implementation"`. Real
examples: `export default { root, test: {...} };` (`vitest.config.ts`), `export default
createUiReactLintConfig();` (`eslint.config.ts`), `const config: T = {...}; export default config;`
(`postcss.config.ts`). None of these are implementation by any reasonable reading — no function, no
class, no control flow, just imports/consts/one default export.

**Fix**: `isConfigDelegationModule` — splits content into top-level (bracket-depth-zero) statements
(`splitTopLevelStatements`, handles arbitrary nesting so a deeply-nested `test: { coverage: {
include: [...] } }` needs no bespoke depth), requires every one of them to be
import/re-export/`const`-binding/one-`export default`, and requires zero
`function`/`=>`/`class`/control-flow markers anywhere (string-literal contents blanked first via
`stripStringLiterals`, see below) → `"thin-delegation"`.

**Regression found and fixed while validating this against the real files, not a synthetic case**:
the *pre-existing* `class|namespace` guard at the very top of the TS/JS branch (`/\b(?:class|namespace)\b/`)
ran on raw content, so `🖱️ui`'s own React-target `vitest.config.ts` — whose real `include` list names
`../../../../🔨️modules/🏷️class-name-composition/🧪️slot.test.tsx` — matched `\bclass\b` against the
substring "class" inside "class-name-composition" and forced `"implementation"` before my new check
was ever reached. Fixed by reusing `stripStringLiterals` (blank all quoted-string contents, keep the
quotes) for that guard too, so a *data value* containing a keyword-like fragment can never be
misread as a keyword. This was a **pre-existing defect in the original grammar**, unrelated to my new
code, that I found empirically (real file, not by inspection) and fixed because it blocked one of my
own 9 target rows — documented here rather than silently folded in.

**8/9 rows fixed** (all real files, verified end-to-end with `classifyPackageGlueContent` *and* with
a live `clean taxonomy plan` re-run — not just the standalone classifier): 5 `🧪️vitest.config.ts`
instances (◻2d, kernel, actor, replication, react-target), `🧪️vitest.config.ts` under the 3d module,
`🎨️postcss.config.ts`, `🟦️eslint.config.ts`. Control group verified to correctly stay
`"implementation"`: `🟦️runtime.ts`, `🧪️render.ts`, `🟦️build-tooling.ts`, `📬️mailbox.ts`,
`🟦️vitest.setup.ts` — all genuinely substantial code (`runtime.ts` alone defines an error-boundary
class and five hook wrappers).

**1/9 NOT fixed, for a different, deeper reason than `classifyGlue`**:
`🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️vitest.config.ts` sits inside a **Rust** ecosystem package
boundary. `classifyPackageRole` checks `location.rule.allowedFileKindIds` *before* it ever calls
`classifyGlue` — `packageBoundaryRules.🦀️rust.allowedFileKindIds = ["rust-source"]` does not include
`typescript-source`, so this file is forced to `"implementation"` at that earlier gate and never
reaches the content classifier my fix touches. The repo's own convention (CLAUDE.md: every package
must drive its Nx tasks through `📜️script.ts`) puts a TS build script + vitest config in *every*
ecosystem's package directory regardless of the ecosystem's own language — `📜️script.ts` already has
exactly the needed escape hatch (`fixedFilenameContracts.root-script`, `scope.kind: "path-pattern"`,
unrestricted by ecosystem, listed in every `packageBoundaryRules.*.allowedFixedContractIds`) but
`🧪️vitest.config.ts` does not: `configurableEntryContracts.vitest-config-entry` is
`ecosystemId`-gated to `🟦️typescript` only, and no `fixedFilenameContracts` entry covers it
cross-ecosystem. **Not fixed here** — while investigating I found `🔣️taxonomy.json`'s
`packageSourceDispositions` already gaining new validator tokens (`tool-config-vitest`,
`tool-config-tailwind`, `tool-config-postcss`, `tool-config-eslint`,
`tool-config-dependency-cruiser`) in the *live, uncommitted* working tree while I worked — a
concurrent session is already building exactly this cross-ecosystem tool-config contract right now;
duplicating it here would conflict. Proposed shape, for that session or the taxonomy owner to fold
in if not already covered, mirroring `root-script` exactly:

```json
"root-vitest-config": {
  "pathPattern": "**/🧪️vitest.config.ts",
  "authority": "Vitest",
  "reason": "Permanent per-package Vitest configuration, regardless of the package's own ecosystem language",
  "configurability": "unconfigurable",
  "scope": { "kind": "path-pattern" },
  "verification": "vitest --config",
  "expires": null
}
```
plus adding `"root-vitest-config"` to `packageBoundaryRules.🦀️rust.allowedFixedContractIds` (and
`🐍️python`'s, if the same cross-ecosystem `vitest.config.ts` pattern recurs there).

## §4 — `uv.lock`: no `fixedFilenameContracts` entry exists at all (taxonomy gap, proposed not applied)

`🖱️ui/🎨️styling/📦️packages/🐍️python/uv.lock` is the only Python-ecosystem row. Unlike `package.json`
(3 contracts) or `tsconfig.json` (4 contracts), grepping `🔣️taxonomy.json`'s
`fixedFilenameContracts` for `uv.lock` returns **nothing** — Python's lockfile was never registered.
Proposed patch (taxonomy owner):

```json
"python-uv-lockfile": {
  "pathPattern": "**/uv.lock",
  "authority": "uv",
  "reason": "Python package manager lockfile discovery",
  "configurability": "unconfigurable",
  "scope": { "kind": "package-root", "ecosystemId": "🐍️python" },
  "verification": "uv lock --check",
  "expires": null
}
```

## §5 — ~89 rows: genuine implementation, needs new `semanticDirectoryKinds` (proposed, NOT applied)

These are real, substantial, per-backend Rust/TypeScript source files sitting directly inside a
`📦️packages/<lang>/` boundary with **no sibling implementation at the owner level at all** — e.g.
`🖱️ui/🖼️render/🎯️targets/🍎️metal/` contains *only* `📦️packages/🦀️rust/` (verified on disk: no owner-
level files). There is nothing to "hoist to" in the sense the exemplar (`🧬️schema/✨️derive`) had —
these are not duplicates of anything; each backend (`🍎️metal`, `🌋️vulkan`, `🪟️d3d12`, `🧊️webgpu`,
`🧊️wgpu`) is single-language by nature (there is no meaningful cross-language "shared" Metal/Vulkan
pipeline implementation). `packageImplementationDestination`'s own logic (line ~3151) already
anticipates this: for a non-generic stem it looks for an existing `semanticDirectoryKinds` entry
matching the stem, scoped by the owner's own directory-kind context — the entries just don't exist
yet. Adding them, scoped, is squarely the intended mechanism (not a workaround): multiple distinct
real files at one owner level (`resources.rs`, `pipelines.rs`, `types.rs`, … — 7 in `🍎️metal` alone)
cannot all flatten to one kind-only `🦀️.rs`, so each genuinely needs its own semantic subdirectory —
"folders carry semantics" working as designed, just missing vocabulary.

**I did not add these to `🔣️taxonomy.json`** (out of scope, owned by another worker this session).
Registering them as unscoped **global** words would be unsafe — words like `types`, `context`,
`event`, `window`, `action`, `math` are exactly the kind of common English word likely to collide
with unrelated files anywhere else in a 40k-file repo. `semanticDirectoryKinds` already supports
`parentKindIds` scoping (see `library`/`test-case` entries for the existing pattern), and
`matchDirectoryKind` already passes the owner's own directory-kind as context
(`directoryKindByPath.get(location.owner)`) — so every proposed entry below should carry
`parentKindIds` restricted to the directory-kind(s) of its actual owner group, never left global.

Distinct stems needing a new, `parentKindIds`-scoped `semanticDirectoryKinds` entry (owner groups in
parentheses — the taxonomy owner should confirm the exact registered directory-kind id for each
group and use that as the `parentKindIds` value, not a global entry):

- **GPU render-target backends** (`🖼️render/🎯️targets/{🍎️metal,🌋️vulkan,🪟️d3d12,🧊️webgpu,🧊️wgpu}`, 5
  target directories): `resources`, `pipelines`, `types`, `frame`/`frame_buffers`, `scene_target`,
  `surface`/`surface_adapter`/`surface_state`, `memory`, `buffers`, `gpu`/`gpu_context`/`gpu_uniforms`,
  `msl`, `objective_c`, `vk_error`, `hlsl`, `world3d`/`world3d_snapshot`, `canvas2d_snapshot`, `math`,
  `scenes`, plus the `🧊️wgpu`-target-specific set: `action`, `arena`, `cursor`, `draw`, `flex`,
  `geometry`, `host`, `input`, `label`, `layout`, `minimap`, `mounted_layout`, `paint`, `prepared`,
  `reconcile`, `scene_slots`, `shaders`, `shell`, `theme`, `tree`.
- **`🖼️render` core** (owner-level, not target-specific): `dispatch`, `element`, `frame`, `layout`,
  `resource`, `scene`, `schedule`, `shader_contract`, `surface`.
- **`🧠️runtime`**: `context`, `dispatch`, `entity`, `gateway`, `present`, `reconcile`, `tracking`.
- **`🧬️contract`**: `accessibility`, `action`, `conformance`, `document`, `layout`, `limits`, `style`,
  `surface`, `text_edit`.
- **`🖥️host`**: `event`, `window` (`backend_alias` is already fixed, §2).
- **`🎬️scene`**: `canvas2d_snapshot`, `math`, `scenes`, `surface`, `world3d_snapshot`.
- **`🎭️actor`** (TypeScript): `mailbox`, `shard-client`, `shard-runtime`, `turn-scheduler` (`wire-turn`
  is ambiguous, §6, not a plain gap).
- **`🎯️targets/⚛️react`** (TypeScript, inside `🖱️ui/📦️packages/🟦️typescript`): `runtime`,
  `build-tooling`, `vitest.setup` (`render` is ambiguous, §6).

Note several stems recur across groups with genuinely different meaning (`surface` in a GPU target
vs. `🧬️contract`; `reconcile` in `🧠️runtime` vs. the `🧊️wgpu` target) — this is exactly why each
needs `parentKindIds` scoping rather than one shared global word; a single global `surface` kind
would force these unrelated concepts into the same bucket or collide outright.

## §6 — 2 rows: `semantic-stem-ambiguous`, a vocabulary *collision*, not a gap

`🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` → "Semantic stem matches multiple directory kinds:
asset-subject, test-fixture-asset". `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️render.ts` →
"...matches: test-case, test-fixture-member". Root cause, traced in `matchDirectoryKind`
(`🧹️normalization/🟦️.ts:2209`): both files carry a **decorative leading emoji** (`🖼️`, `🧪️`) that is
also the registered emoji for an unrelated, broadly-scoped semantic-directory family (`asset-*`,
`test-*`). `test-case`'s own `slugRegex` is permissive enough to match almost any bare word, and
`test-fixture-member`'s `parentKindIds` includes itself (recursive), so a file whose real stem
("render", "wire-turn") isn't literally named after any registered kind falls through `matchDirectoryKind`'s
`exact` check and lands in the ambiguous multi-candidate fallback — purely because of the emoji, not
the word. Neither file is a test or an asset (`render.ts` is a real ~113-line React DOM render
helper; `wire-turn.ts` is a ~197-line wire-format interpreter). **Not fixed** — the correct move is
either a taxonomy decision (register `render`/`wire-turn` as their own scoped kinds with a
non-colliding emoji) or a file-decoration correction (drop the misleading `🧪️`/`🖼️` prefix), both
outside this slice's authority (taxonomy.json / file-renaming decisions belong to the taxonomy
owner and the dev, respectively). Flagging so it isn't mistaken for part of §5's plain gap.
