# W0-I — coordinator lane report

Lane: taxonomy additive keys + discovery validator + its tests. Model: Opus 5 (coordinator).

## What landed

### 1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`

`schemaVersion` **4 → 5**, plus a `_surfaceComment` recording the sequencing rule (surface keys land
first; app keys die only after the last `🎛️apps` dir is gone — the APA lesson that the Rust
`assert!` gate panics on all 33 plugins if the order is reversed).

Added, all **additive** — no existing key changed shape except the two noted:

| key | value |
|---|---|
| `viewerDirName` | `"👁️viewer"` |
| `editorDirName` | `"✏️editor"` |
| `surfaceRoles` | `["viewer","editor"]` |
| `surfaceDirNames` | `{"viewer":"👁️viewer","editor":"✏️editor"}` |
| `subsetSurfaceDirs` | `["👁️viewer","✏️editor"]` |
| `subsetRequiredSurfaceDirs` | `["👁️viewer","✏️editor"]` |
| `contributedSubsetChildDirs` | `["👁️viewer","✏️editor"]` |
| `surfaceChildDirs` | 9 entries, mirroring `appChildDirs` minus `⚙️engine` |
| `surfaceRequiredChildDirs` | `["🎭️modes","🎮️commands","🎚️config","👥️presence","🫧️transient"]` |
| `surfaceComponentLangs` | `["🦀️rust","🟦️typescript"]` |
| `surfaceSchemaSpecFilenames` | the three state-lane schema paths → `🔣️component.json` |
| `windowLeafLangs` | `["🦀️rust","🟦️typescript"]` |

Two existing keys extended: `subsetChildDirs += "👁️viewer","✏️editor"`.

**Not done, deliberately** — `semanticCollections` and `semanticAllowedOwnerLevels` were left alone.
The plan's C6 asked for both. Both were measured against the real validator and rejected; the full
reasoning is `📋️contract-freeze.md` §7.7 and §7.8. In short: `semanticCollectionSpec`
(`🔍️discovery/🟦️component.ts:1470`) matches by path suffix and a collection root must carry a
manifest whose members are in exact bijection with its direct child dirs — a surface's children are
taxonomy vocabulary words, so the addition would have created **286 phantom collection roots**. A
surface is a subset *facet*, exactly like `🧬️schema`/`🚪️io`, and neither of those is a
`semanticCollection` either.

`osChildDirs += "🎚️config"` is delegated to lane 0-C (it owns the OS config subtree and needs the key
in the same change).

### 2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`

- `Taxonomy` interface: 12 new readonly fields with emoji-first docstrings (`:157`–`:180`,
  `windowLeafLangs` at `:243`).
- New `//#region SurfaceContract` inside `validateTaxonomy`, after `StateLaneContract`. It enforces:
  - `surfaceRoles` is exactly `["viewer","editor"]` **in that order** — it is simultaneously the
    `AppRole` declaration order and the `u8` channel tag, so order is load-bearing, not cosmetic;
  - `surfaceDirNames` agrees with `viewerDirName`/`editorDirName` and covers every role;
  - every member of `subsetSurfaceDirs` / `subsetRequiredSurfaceDirs` / `contributedSubsetChildDirs`
    is a declared `surfaceDirNames` value AND present in `subsetChildDirs`, with no duplicates;
  - `surfaceChildDirs` includes `modesDirName`;
  - `surfaceRequiredChildDirs ⊆ surfaceChildDirs` and covers all three state lanes;
  - `surfaceComponentLangs` / `windowLeafLangs` are declared langs with `taxonomyLeafFilenames`
    entries;
  - `surfaceSchemaSpecFilenames` keys start with a declared `surfaceChildDirs` member and point at
    the normative jsonschema leaf.

### 3. `📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

Five new tests in the `validateTaxonomy` describe block: the frozen vocabulary; role-order is
load-bearing; dir-name/role-mapping cannot drift; required-child must be structural and cover every
lane; a surface dir outside `subsetChildDirs` is rejected. Each asserts a **negative** case by
mutating a copy of the real taxonomy, so none of them can pass vacuously.

Two existing tests updated:
- `"derives lifecycle capabilities…"` — `subsetChildDirs` pin extended with the two surface dirs.
  This one was **broken by my change** and fixed in the same wave, per the greenfield rule.
- `"reports a completeness dir missing from the structural set"` — was filtering `📡️spr` out of
  `artifactChildDirs`, but `📡️spr` has not been in that list since
  `449c584855086f7d44ecb00681c6d8a16a7b85da` (2026-08-10 14:13:28 +0200), so the filter was a no-op
  and the assertion could never fire. **A rule that looks installed and never fires.** Repointed at
  `🚪️io`, a member that really is in both the completeness and structural sets, preserving the
  test's stated intent. This failure predates the ticket and is not caused by it.

## Commands run and results

```
bun test 🧪️index.test.ts -t "the shipped vocabulary is internally consistent"   → 1 pass, 0 fail
bun test 🧪️index.test.ts -t "validateTaxonomy"                                  → 28 pass, 0 fail
bun test 🧪️index.test.ts   (full suite)                                          → 168 pass, 18 fail
```
Full output: `🧪️w0-i-repo-lib-test.txt`.

`validateTaxonomy()` against the **shipped** vocabulary returns `[]` with all 12 new keys and all new
checks live — that is the real acceptance signal, not the count.

### Failure accounting (enumerated, not estimated)

| | count |
|---|---:|
| failures before this lane | 19 |
| introduced by this lane | 1 (`subsetChildDirs` pin) — **fixed** |
| repaired by this lane | 1 (`📡️spr` dead sentinel) |
| failures after this lane | **18** |

Net: **0 new failures, 1 pre-existing failure repaired.**

## Not done, and why

The 18 remaining failures are foreign and pre-existing. Sampled and attributed:
- `loadTaxonomy > parses …` — `taxonomy.snapshotChildDirs` is `undefined`; the key does not exist on
  disk. A different ticket's vocabulary change, not this one's.
- `loadTaxonomy > describes the per-example …` — `exampleAssetKindPrefixes` keys on disk are
  `snapshot-text`/`mutations-binary`/…; the test pins the older `dsl`/`op`/`spr`/`pack` names. Disk is
  the SSOT, so the test is stale from a peer's rename.
- `discoverPackages > finds every migrated plugin …` — expects `✒️writer` to be `area: "mixed"`,
  disk says `clean`.
- The remaining 15 (`dependency-boundary`, `ui scrollbar styling`, `micro-commit`, `playground static
  sites` ×2, `package boundary guards`, `commit`, `command budgets` ×2, `resolveCargoPackageName` ×2,
  `discoverPackages` ×2, `computeWorkspaces`) touch nothing this lane changed.

These are **not** repaired here: two peer sessions are writing `📜️script.ts` and the plugin tree
right now, and each of these tests pins a vocabulary one of them may be mid-rename on. Guessing which
side is authoritative would risk reverting a live lane's in-flight work. Recorded in `📌️important.md`
for the W4 audit to re-check against a quiet tree.
