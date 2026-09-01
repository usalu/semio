# `package-implementation-destination-unresolved` / `package-role-unresolved` — report

Full analysis and the taxonomy-vocabulary proposal: `📓️goal-pkgdest-census.md` (this ticket root).

## Fixed (engine, `🧹️normalization/🟦️.ts`, no `🔣️taxonomy.json` edits)

1. **`fixedSpecificity` scope-kind ranking** — graded a binary path-pattern/everything-else tie into
   7 explicit ranks so `sibling-fixed-filename-contract` (e.g. `nx-owned-node-package-manifest`)
   deterministically outranks `package-root` (e.g. `node-package-manifest`) instead of tying whenever
   both match, which every `package.json`/`tsconfig.json` in this repo does (sibling
   `📋️project.json` always present). Verified this is the *only* duplicate-`pathPattern` pair in the
   whole schema, so nothing else can flip.
2. **Rust declarative-grammar gaps** — `classifyGlue`'s Rust branch didn't recognize `pub type Alias
   = X;` aliases (cfg-gated backend selection, `🖥️host/🦀️backend_alias.rs`) or block-form `mod name {
   ...only mod/use/pub-use... }` wiring (`📡️replication/📦️glue.rs`'s namespacing re-export blocks) as
   declarative, so both fell to `"unresolved"`. Added the `type`-alias alternative and a recursive
   `stripDeclarativeRustModuleBlocks` that only strips a block whose *entire* body is itself
   declarative.
3. **TS/JS "config literal" misclassified as implementation** — the branch's only fallback for
   non-declaration content was `"implementation"`; added `isConfigDelegationModule` (imports/re-
   exports/const-bindings/one `export default`, zero function/class/control-flow anywhere) →
   `"thin-delegation"` for `export default { ... }` / `export default defineConfig({...})` /
   `export default someCall()` shaped files. **Found and fixed a pre-existing, unrelated regression
   while validating against the real files** (not synthetic): the original `class|namespace` guard at
   the top of the branch matched the literal substring "class" inside a *string literal* value
   (`../🏷️class-name-composition/…` in a real `vitest.config.ts` `include` list), forcing
   `"implementation"` before my new check was ever reached. Fixed by blanking string-literal contents
   (`stripStringLiterals`) before that guard runs too.

Exported two narrow, purpose-built testing entry points (`classifyPackageGlueContent`,
`fixedContractScopeSpecificityRank`) — internal types (`PackageGlueGrammar`, `FixedContractScope`)
stay unexported; only plain string-literal unions cross the boundary.

## Test (language-agnostic, data-vector, TDD — new suite, red before / green after)

`🧹️normalization/🧪️tests/🧪️package-boundary-classification/{🟦️.ts,🔣️.json}` — same shape as the
sibling `🧪️source-admission` suite (Ajv-validated JSON vectors + `bun:test`), wired into
`📦️packages/🟦️typescript/🧪️index.test.ts`'s import list. 10 cases: 3 real Rust-glue shapes (2
declarative fixes + 1 real-struct control), 4 real TS config-literal shapes (3 fixes + a synthetic
regression case for the string-literal-keyword bug), 1 real-implementation TS control, 2 scope-
specificity-ranking assertions. All 10 pass now; reverting any of the three engine changes makes the
corresponding case(s) fail (verified — the string-literal one especially: it failed with "expected
thin-delegation, got implementation" against the *fixed* config-literal logic until the
`stripStringLiterals` guard fix landed too). `bun test ./🧪️index.test.ts -t
"package boundary glue-content classification|taxonomy source admission projection"` → 100 pass, 0
fail (confirms the wiring doesn't break the sibling suite or the bundle's own load/parse).

## Verify — live `clean taxonomy plan`, same scope/baseline as the assignment

```
B=bb06c41f73f0122fbed315b7487428b976f99921
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules" --baseline "$B" --plan "$T/🗑️temp/pkgdest-plan-after.json" --workers 8
```

```
[clean taxonomy plan] moves=2160 roots=0 relocations=0 symlinks=0 removals=1 edits=4156 regenerations=8 unresolved=1224
```

My baseline (measured directly from `🗑️temp/🔣️vocab-plan.json` before any edit, same scope — the
brief quoted 117+2=119, I could not reproduce that number from a captured artifact and report what I
actually measured): `package-implementation-destination-unresolved=113`, `package-role-unresolved=2`.

| code | before | after | delta |
|---|---:|---:|---:|
| `package-role-unresolved` | 2 | **0** | −2 |
| `package-implementation-destination-unresolved` | 113 | **95** | −18 |
| **my class, total** | **115** | **95** | **−20** |

Cross-checked by path, not just by count: all 10 manifest rows and both role-unresolved rows are
gone; 8/9 targeted TS-config rows are gone. The 9th
(`🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️vitest.config.ts`) is blocked one layer *earlier* than
`classifyGlue` — `classifyPackageRole` checks `packageBoundaryRules.🦀️rust.allowedFileKindIds`
(`["rust-source"]`, no `typescript-source`) before ever calling the content classifier, because this
`.ts` file sits inside a **Rust** ecosystem package boundary (every ecosystem dir carries its own
`📜️script.ts`+vitest config regardless of language, per CLAUDE.md). `📜️script.ts` already has an
ecosystem-agnostic escape hatch (`fixedFilenameContracts.root-script`, `scope.kind: "path-pattern"`);
`🧪️vitest.config.ts` doesn't. Proposed patch in the census (§3 tail) — **not applied**: while
investigating I found `packageSourceDispositions` gaining new `tool-config-vitest` /
`tool-config-tailwind` / `tool-config-postcss` / `tool-config-eslint` /
`tool-config-dependency-cruiser` validator tokens in the live, uncommitted working tree mid-session —
a concurrent worker is already building exactly this cross-ecosystem tool-config contract; not
duplicating it.

`moves` rose 2082 → 2160 (+78) against the assignment brief's quoted baseline for this scope — I
cannot claim all of that: `🖱️ui/🎨️styling` (one of my nine TS-config targets) has concurrent,
uncommitted edits from another session right now (a deleted `🧪️index.test.ts`, an untracked new
`🧪️tests/` dir, a modified `📜️script.ts`), and the same run surfaced 5 fresh `collision-*` rows (one
each byte/case-fold/nfc/same-kind/vs16) at `🖱️ui/🎨️styling/🟦️.ts` between that session's
`📦️packages/🟦️typescript/📦️index.ts` and `🧪️tests/🟦️.ts` — pre-existing generic-stem-flattening
behavior I never touched, not caused by my changes (down from 44-per-variant at my original baseline
capture, so a different slice's near-complete cleanup, not a regression I introduced).

## Not fixed, not mine to fix (`🔣️taxonomy.json` — proposals only, in the census)

- **~89 rows** — genuine multi-file, single-language-per-target implementation (5 GPU render
  backends, `🧠️runtime`, `🧬️contract`, `🖥️host`, `🎬️scene`, `🎭️actor`, react-target) needing new,
  `parentKindIds`-*scoped* `semanticDirectoryKinds` entries — full stem list and scoping rationale in
  the census §5. Explicitly NOT proposed as unscoped globals (words like `types`/`context`/`event`
  would collide repo-wide).
- **2 rows** (`render.ts`, `wire-turn.ts`) — `semantic-stem-ambiguous`, caused by a decorative leading
  emoji (`🧪️`, `🖼️`) colliding with the broad `test-case`/`test-fixture-member`/`asset-*` vocabulary
  family, not a missing word. Census §6.
- **1 row** (`uv.lock`) — no `fixedFilenameContracts` entry exists for Python's lockfile at all.
  Exact patch in census §4.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — the three fixes + two
  testing exports.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🧪️package-boundary-classification/{🟦️.ts,🔣️.json}` — new test.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` — one new import line wiring the new suite in.
