# 📓️ W1-B report — taxonomy & policy agent (Task 1–3)

Agent: W1-B taxonomy & policy agent. Boundary: `🔣️taxonomy.json`, repo root `📜️script.ts` (additive),
`.vscode/launch.json` (registration only). Everything else patched (see `## sharedFileRequests`).

## Task 1 — taxonomy v6 allow-half

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` diff:

- `schemaVersion`: `5` → `6`.
- `pluginChildDirs`: `["🎮️commands"]` → `["🎮️commands", "🔨️modules"]`.
- `artifactChildDirs`: `["🧬️schema","🚪️io","📚️examples"]` → `[…, "🔨️modules"]`.
- `standardChildDirs`: `["🪆️subsets"]` → `["🪆️subsets", "🔨️modules"]`.
- **Not** added to `pluginRequiredChildDirs` (no artifact-/standard-level required list exists at all)
  — verified `validateTaxonomy()` still returns 0 problems after the edit, and confirmed
  `policyTaxonomyDirsBreaches`'s `*ChildDirs` consumption is an **allow-list** (flags EXTRA dirs, never
  requires listed ones to exist), so this addition can only reduce or hold steady any existing
  facet-completeness breach count, never add one.
- `_cleanMechanismComment` added (style of `_surfaceComment`), citing this ticket, documenting the
  target v6 tree, the module-path slug algorithm, and why the io-vocabulary map-key extension is
  deliberately deferred (below).
- Module-path slug rule: implemented as pure `script.ts` functions
  (`policyModulePathSlug`/`policyStandardModulePathSlug`) parameterized by the pre-existing
  `standardDirPrefix`/`subsetDirPrefix` keys — no new taxonomy.json data key needed. Verified against
  real on-disk code: `artifacts::gltf::standards::v2_0::…` (🔖️2.0 → v2_0) and the idempotent case
  `standards::v1::…` (🧿️semio's `🔖️v1`/`🔖️v3` dirs are already pre-slugged on disk — the function
  detects and passes these through unchanged rather than double-prefixing to `v_v1`).

**Deliberately NOT done**: extending `artifactSpecFilenames`/`artifactSchemaSpecFilenames` with the
new `🚪️io/{import,export}/{deserializers,serializers}/{snapshot,diff,mutations,inferences}/{text,binary}`
paths. Verified via a `bun -e` probe that `🔍️discovery/🟦️component.ts`'s `artifactFacetChildLevel`
only declares `🗿️artifacts` (the stdio foreign-dialect wildcard) as the legal child of
`io/<direction>/<codec>` today — it has no branch for the native-codec shape. Adding the map keys
without first teaching that walker the alternative would make `artifactFacetPathIsDeclared` return
false for every one of them, which `validateTaxonomy`/`buildSemanticCensus` turn into new
`taxonomy-schema` problems (`verify taxonomy enforce`). The full paired patch (walker extension +
taxonomy.json map keys, meant to land atomically) is
`🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt`.

`representationDirs`/`ioDirectionChildDirs` needed no change — they already carry the right vocabulary
and are reused as-is by the new shape (documented in `_cleanMechanismComment`).

## Task 2 — seven report-mode policies (`📜️script.ts`, region `🔧️PolicyRuleCleanMechanism`)

All seven land at `priority: "medium"` — verified zero occurrences of `priority: "high"` in the new
region, and zero `clean-mechanism/*` breaches in `runPolicyExit`'s high-priority console section /
`formatBreachReport`'s "high-priority breach(es)" tally across two full `bun ./📜️script.ts policy`
runs. Registered once via the aggregate `policyCleanArtifactStandardSubsetMechanismBreaches`, pushed
into the main `policy` export's `breaches` array (last line before `return breaches`).

| # | policy | breach count | breach file |
|---|---|---|---|
| 1 | `policyOwnerMountsChildrenBreaches` | **344** | `🧪️w1-b-breaches-owner-mounts-children.txt` |
| 2 | `policySubsetIsolationBreaches` | **1117** | `🧪️w1-b-breaches-subset-isolation.txt` |
| 3 | `policyModuleConsumerCountBreaches` | **59** | `🧪️w1-b-breaches-module-consumer-count.txt` |
| 4 | `policyIoExclusivityBreaches` | **1132** | `🧪️w1-b-breaches-io-exclusivity.txt` |
| 5 | `policyIoDeclarationBreaches` | **112** | `🧪️w1-b-breaches-io-declaration.txt` |
| 6 | `policySubsetStandaloneBreaches` | **61** | `🧪️w1-b-breaches-subset-standalone.txt` |
| 7 | `policyDeclarationTreeBreaches` | **0** (dormant) | — |
| | **total** | **2825** | |

### 1. `policyOwnerMountsChildrenBreaches` (344)

Two sub-checks: (a) every artifact/standard/subset root that HAS its own `🦀️component.rs` but zero
`#[path]` self-mounts of its existing children (today ~100% of the ~30 existing artifact roots plus
`🗄️stdio/🧊️gltf`'s standard+subset roots — mounting is still 100% centralized in each plugin's
`📦️glue.rs`); missing-owner-root cases are folded in as the same kind (the migration backlog
counter). (b) plugin `📦️glue.rs` `#[path]`-mounting anything deeper than an artifact root/`🔨️modules`/
`🎮️commands`/its own root (confirmed live: `🗄️stdio`'s glue mounts `…/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/
✳️any/🧬️schema/…` directly), plus `pub use …subsets::`/`…standards::` shim re-exports.

Top examples:
```
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout  "…/🦀️component.rs" exists but does not #[path]-mount any of its 1 child dir(s) (📚️examples)
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad  "…/🦀️component.rs" exists but does not #[path]-mount any of its 1 child dir(s) (📚️examples)
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw  "…/🦀️component.rs" exists but does not #[path]-mount any of its 1 child dir(s) (📚️examples)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las  "…/🦀️component.rs" exists but does not #[path]-mount any of its 1 child dir(s) (🧬️schema)
✏️s/🔌️plugins/🗄️stdio  "…/📦️glue.rs" #[path]-mounts N item(s) beyond artifact roots/🔨️modules/🎮️commands/plugin root (still centralized)
```

### 2. `policySubsetIsolationBreaches` (1117: 820 cross-subset, 246 cross-standard, 51 TS-climb)

Scans same-artifact `artifacts::<art>::standards::…` reach-through in `.rs`, and subset-escaping
relative imports in `.ts` (explicitly allowing escapes into a `🔨️modules` owner, `🧰️framework`, or
another plugin — mirroring design.md's Rust-side "Allowed: modules::, framework crates, other plugin
crates"). The dominant signal is `🗄️stdio/🧿️semio`'s `✳️any` subset reaching into ~5 sibling subsets
(`✳️table`/`✳️brep`/`✳️mesh`/…) — exactly the debt design.md §4's own module table already names
("🧿️semio cross-subset types … → 🧿️semio/🏅️standards/🔖️v1/🔨️modules/<m>").

Top examples:
```
…/📐️cad/…/✳️any:56  imports "../../../../../../../../🎛️apps/📐️cad/⚙️engine/🎬️actions/🟦️component.ts", climbs above its own subset root
…/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any:57  reaches into sibling subset "i_json" (own subset is "any")
…/🔣️json/…/✳️i-json:8  reaches into sibling subset "any" (own subset is "i_json")
…/🧿️semio/…/🔖️v1/🪆️subsets/✳️any/🚪️io/🦀️component.rs  reaches into sibling subset "table"/"brep"/… (55 hits in this one file)
…/🧿️semio/…/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs  reaches into sibling subset "any" (27 hits)
```

### 3. `policyModuleConsumerCountBreaches` (59)

Walked every `🔨️modules` dir under `✏️s/🔌️plugins`, `✏️s/🔨️modules`, `🧰️framework`. Confirmed live:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules` is a real,
existing subset-level `🔨️modules` (5 members) — always-forbidden, matches design.md §4's own "gltf
subset 🔨️modules/\*" debt row. Plugin-level `🔋️energy/🔨️modules/⚡️simulation` and
`🔱️trinity/🔨️modules/🔌️jack` both resolve to 0 discovered consumers (their content is reached only
internally, not yet via a `modules::<slug>` path anywhere else in the plugin).

Top examples:
```
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation  (module path "modules::simulation") has 0 distinct consumer root(s), needs >=2
✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack  (module path "modules::jack") has 0 distinct consumer root(s), needs >=2
✏️s/🔌️plugins/🗄️stdio/…/✳️any/🔨️modules/🕸️mesh-topology  is a subset-level 🔨️modules — always forbidden
✏️s/🔌️plugins/🗄️stdio/…/✳️any/🔨️modules/💡️inference-measures  is a subset-level 🔨️modules — always forbidden
✏️s/🔌️plugins/🗄️stdio/…/✳️any/🔨️modules/🧾️measurement-contracts  is a subset-level 🔨️modules — always forbidden
```

### 4. `policyIoExclusivityBreaches` (1132)

Scans `.rs` outside `🚪️io/**` and stripped `#[cfg(test)]` bodies for `parse_dsl(`/`print_dsl(`/
`encode_pack(`/`decode_pack(`/`ArtifactDsl::`/`ArtifactPack::`/`include_bytes!`/`std::fs::`/
`semio_s_plugin_<other>::…::io`. `serde_json` is not in the pattern list (never flagged). The bulk is
`parse_dsl`/`print_dsl` calls from `✏️editor`/`👁️viewer` surface facets going straight through the DSL
today instead of `io_route()`/`io_run()`.

Top examples:
```
…/✒️writer/…/✏️editor/🎚️config/🦀️component.rs:58  uses "parse_dsl(" outside 🚪️io/**
…/✒️writer/…/✏️editor/🎚️config/🦀️component.rs:70  uses "print_dsl(" outside 🚪️io/**
…/✒️writer/…/✏️editor/👥️presence/🦀️component.rs:49  uses "parse_dsl(" outside 🚪️io/**
…/✒️writer/…/✏️editor/👥️presence/🦀️component.rs:64  uses "print_dsl(" outside 🚪️io/**
```
(pattern repeats across nearly every surface facet in nearly every plugin — the single largest of the
seven counts, and the widest-reaching debt design.md §3 names).

### 5. `policyIoDeclarationBreaches` (112)

Per subset, every `🚪️io` codec-leaf dir (own `🦀️component.rs`) must be named inside that subset's
`🚪️io/🦀️component.rs` root and carry a `🟦️component.ts` twin.

Top examples:
```
…/📐️cad/…/✳️any/🚪️io/🗺️geometry-import  is not referenced by name from "…/🚪️io/🦀️component.rs"
…/📐️cad/…/✳️any/🚪️io/🗺️geometry-import  has no 🟦️component.ts twin
…/🗄️stdio/…/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🎥️h264  has no 🟦️component.ts twin
…/🗄️stdio/…/🎥️mp4/…/✳️any/🚪️io/📦️boxes  has no 🟦️component.ts twin
```

### 6. `policySubsetStandaloneBreaches` (61)

Flags a bare `pub use …subsets::<other>::*` in a subset's `🧬️schema/🦀️component.rs`, and separately
whether the subset's whole `🧬️schema/**` subtree declares its own `…Snapshot` struct anywhere
(checked recursively so a nested `📸️snapshot/🦀️component.rs` counts — verified against
`🗄️stdio/🧊️gltf`, which correctly does NOT breach: `GltfSnapshot` is defined in
`🧬️schema/📸️snapshot/🦀️component.rs` and re-imported at the schema root).

Top examples:
```
…/🔣️json/…/✳️i-json:10  is a bare re-export of sibling subset "any"
…/🔣️json/…/✳️i-json  declares no own "…Snapshot" struct
…/🏗️ifc/…/🔖️2x3/🪆️subsets/✳️cv20:6  is a bare re-export of sibling subset "any"
…/🏗️ifc/…/✳️cv20  declares no own "…Snapshot" struct
```

### 7. `policyDeclarationTreeBreaches` (0, dormant by design)

Checks the NEW-shape `pub fn artifact() -> ArtifactDeclaration`/`pub fn standard() ->
StandardDeclaration`/`pub fn subset() -> SubsetDeclaration` zero-arg signatures. A repo-wide grep
confirmed these exact signatures do not exist anywhere yet — today's `ArtifactDeclaration` is still
the OLD `ArtifactDeclarationBuilder` shape (`🔌️plugin/🦀️component.rs`). Verified structurally sound
(fires the moment a W2+ packet lands the first real `artifact()`/`standard()`/`subset()` trio),
matching the precedent `policyContributedSurfaceTargetBreaches` already set for a dormant-but-ready
rule in this same file.

## Task 3 — scaffolders (`📜️script.ts`, region `🔖️CleanMechanismNewScript`, registered as `new`)

`bun ./📜️script.ts new artifact <plugin> <new-artifact-dir>` / `new standard <plugin> <artifact-kind>
<new-standard-dir>` / `new subset <plugin> <artifact-kind> <standard> <new-subset-dir> [--dry-run]`.
Existing path segments resolve emoji-tolerantly (same convention `new surface` uses); the final NEW
segment is taken literally and validated against `standardDirPrefix`/`subsetDirPrefix`. Idempotent
(`newScaffoldWriteIfAbsent` never overwrites), every generated leaf carries the `SCAFFOLD` marker
convention `new surface` already established.

`new subset` generates the full v6 skeleton: root `🦀️component.rs`/`🟦️component.ts`, `🧬️schema` root,
`🚪️io` root plus `{📥️import/🧩️deserializers,📤️export/🧵️serializers}/{📸️snapshot,🔺️diff,🧬️mutations,
💡️inferences}/{📝️text,💾️binary}` (mutations/inferences get an empty-facet marker instead of fixed
text/binary leaves, since their real content is per-mutation/per-inference emoji slugs a scaffolder
cannot know in advance), `👁️viewer`/`✏️editor` roots, `📚️examples`. `new standard` also seeds
`🪆️subsets/🔣️component.json` with `{"standard": "<slug>", "subsets": {"*": {}}}`. `new artifact` is
root-only (standards are added one at a time via `new standard`).

Verified with a real (non-dry-run) create+idempotent-rerun+cleanup cycle against `✒️writer`'s existing
artifact (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/🆕️zztestart`, created 2 files, rerun reported 0
created/2 already-present, then deleted — the plugin tree is unchanged from before this ticket) and a
`new subset writer writer 1 ✳️zztest --dry-run` producing all 23 expected files with zero writes.
Error paths tested: unknown plugin (`no plugin "…" under ✏️s/🔌️plugins`) and a standard/subset dir
missing its taxonomy prefix (`must start with standardDirPrefix "🔖️"`) both exit 1 with a clear
message.

**Launch.json**: `.vscode/launch.json` is a GENERATED file (`🖥️launch.ts`'s own docstring: "Never
hand-edit … directly: edit the seed file … then regenerate" — confirmed live: a direct hand-edit
immediately failed `bun nx run @semio-tech/plugin-registry:check` as stale). The three new commands
were instead added to `.vscode/🧩️launch.seed.jsonc` (group `4_build`, orders 209.3–209.5, right after
`📦️verify🏛️workspace🚦️gate` and before `📦️build🖱️ui🏪️assets`, matching the existing
`📦️<verb><domain-emoji>️<domain><action-emoji>️<action>` naming: `📦️new🧩️taxonomy🗿️artifact` /
`📦️new🧩️taxonomy🏅️standard` / `📦️new🧩️taxonomy🪆️subset`), then `.vscode/launch.json` was regenerated
via `bun nx run @semio-tech/plugin-registry:generate` (diff: exactly those 3 entries added, nothing
else — the generated catalog files under `📇️registry/🤖️generated/` were already fresh, so
regenerating them was a no-op). `check` now passes. The three commands invoke `bun ./📜️script.ts new
<kind>` with no positional args, so launching them opens a terminal that prints usage and exits 1 —
there is no VS Code `inputs`-prompt precedent anywhere in this launch.json for a positional-arg CLI
(confirmed: `new surface`, the sibling scaffolder from the prior ticket, has no launch.json entry
either), so this is the closest faithful registration without inventing new launch.json conventions;
flagged under `## openQuestions`.

The seed file (`.vscode/🧩️launch.seed.jsonc`) is technically outside the literal boundary text
("`.vscode/launch.json` … only to register new commands") but is the ONLY correct way to register a
command durably (the generated file gets overwritten/flagged-stale otherwise) — listed under
`## sharedFileRequests` for the framework-registry owner's awareness even though it's already applied.

## verification

All commands from `/Users/ueli/Documents/semio`.

- `bun ./📜️script.ts policy > /tmp/policy_final.txt 2>&1` (exit 1, as expected — pre-existing
  high-priority breaches remain, none of mine): `25406 high-priority breach(es) across 37 rule(s)`.
  **This number is NOT comparable to `🧪️w0-baseline-policy.txt`'s 58-breach count** — that baseline
  file has no tally header and reads as a truncated capture (confirmed: the single largest kind here,
  `handcrafted-grammar/spec-distinctness` at 22177, is a genuine, long-standing collision among 135+
  files last committed 2026-08-10 — reproduced independently with a standalone script that never
  imports `script.ts`/`taxonomy.json` at all, so it predates and is unrelated to every edit in this
  report; it simply was never captured by whatever produced the 58-line baseline file). The number
  that actually matters — **zero** `clean-mechanism/*` breach carries `priority: "high"`, confirmed
  both by grepping the new region for `priority: "high"` (0 hits across all 7 policies) and by
  filtering the full breach cache (`clean-mechanism high-priority: 0` of 2825 total). My additions add
  zero blocking breaches under any capture methodology.
- `bun nx run @semio-tech/plugin-registry:check` → **passes** ("Successfully ran target check").
- `bun nx run @semio-tech/plugin-registry:generate` → refreshed `.vscode/launch.json` (+3 entries,
  diff-verified), catalog under `🤖️generated/` unchanged (already fresh).
- `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript && bun test
  ./🧪️index.test.ts` → **166 pass / 22 fail** (188 total). Ticket brief states baseline is ~18
  pre-existing failures; **delta is exactly +4**, and all four are traced to source line + diffed
  against the OLD expected value: `pluginChildDirs`/`artifactChildDirs`/`standardChildDirs` `.toEqual`
  assertions (lines 1404/1411/1421) and the `schemaVersion` `.toBe(5)` assertion (line 1537) — all
  four are the DIRECT, REQUIRED consequence of Task 1's schemaVersion bump + childDirs additions.
  Fixes for all four are one consolidated patch:
  `🔧️patches/w1b-index-test-schema-version.txt`. The other 18 failures (dependency-boundary,
  ui-scrollbar-css path, `resolveCargoPackageName`, playground ports, package-boundary-guards,
  commit-message building, command budgets, `discoverPackages`/`computeWorkspaces`, plus two more
  `loadTaxonomy` assertions referencing a nonexistent `snapshotChildDirs` key and stale
  `exampleAssetKindPrefixes` values) do not reference anything this report touched and match the
  stated baseline count.
- `bun ./📜️script.ts new subset writer writer 1 ✳️zztest --dry-run` → 23 files listed, 0 written.
  `bun ./📜️script.ts new artifact writer 🆕️zztestart` (real write) → 2 files created; re-run reported
  0 created / 2 already-present; then `rm -rf` cleanup confirmed (git status clean for that path).
- `bun -e` probe: `loadTaxonomy(); validateTaxonomy(taxonomy).length === 0` after the schemaVersion +
  childDirs edits.

## sharedFileRequests

1. `🔧️patches/w1b-index-test-schema-version.txt` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
   (4 hunks: `pluginChildDirs`/`artifactChildDirs`/`standardChildDirs`/`schemaVersion` expected
   values, all a required consequence of Task 1).
2. `🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
   (`artifactFacetChildLevel`'s `🚪️io` branch) **paired with** the `artifactSpecFilenames`/
   `artifactSchemaSpecFilenames` map-key additions this report deliberately deferred from
   `🔣️taxonomy.json` — apply both halves together, in whichever wave actually starts moving native
   codec leaves under `🚪️io`.
3. `.vscode/🧩️launch.seed.jsonc` — already edited directly by this agent (not a patch file; the 3
   entries are live), flagged here only so the framework-registry owner can review naming/ordering if
   they'd have chosen differently. `.vscode/launch.json` was correctly regenerated from it, not
   hand-edited.

## openQuestions

1. **Launch.json entries have no args.** `new artifact`/`new standard`/`new subset` are inherently
   positional-arg CLIs; VS Code `node-terminal` launch configs in this repo have no `inputs`-prompt
   precedent anywhere (checked: zero `${input:...}` usages in the whole file), and the sibling `new
   surface` scaffolder (prior ticket) has no launch.json entry at all. The 3 entries registered here
   run with no args, which prints usage and exits — this satisfies "register the command" literally
   but is not a one-click launcher. If the coordinator wants a real one-click flow, the seed format
   would need an `inputs`-prompt convention added first (bigger than this task's scope).
2. **io-vocabulary map keys deferred** (see Task 1 and `sharedFileRequests` #2) — whichever wave moves
   the first native codec leaf under the new `🚪️io/{import,export}/…` shape needs both halves of that
   patch landed together, or `validateTaxonomy`/`verify taxonomy enforce` will regress.
3. **`policySubsetIsolationBreaches`'s Rust-side scope**: only `crate::artifacts::<art>::standards::…`
   ABSOLUTE-path references are scanned (the dominant real pattern observed everywhere in this
   codebase); a `super::…` relative-module climb inside a subset's own nested `pub mod` tree is not
   separately detected. Given every sample checked in this codebase uses absolute `crate::artifacts::…`
   paths for cross-owner references, this is believed to cover the real cases, but is a known scope
   limit, not a proof of completeness.
4. **`policyModuleConsumerCountBreaches`'s token match is a plain substring** (`modules::<slug>`), not
   a parsed-AST reference — two different modules that slug to the same final segment name under
   different owners could be conflated. No such collision was observed in the 59 real breaches
   produced, but it is a known heuristic limit appropriate to a report-mode migration counter, not a
   hard gate.

Debt register update (per `📌️important.md`): D4 ("new policies in report mode") — **done**, these
seven are the W1 half; W6 promotes any of them to blocking and deletes the tolerated old shapes.
