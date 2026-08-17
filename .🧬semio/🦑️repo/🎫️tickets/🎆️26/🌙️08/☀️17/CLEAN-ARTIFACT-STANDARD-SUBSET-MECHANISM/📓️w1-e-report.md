# 📓️ W1-E report — taxonomy/discovery agent (io native-codec vocabulary, corrected shape)

Agent: W1-E taxonomy/discovery agent. Boundary: `🔣️taxonomy.json`, `🔍️discovery/🟦️component.ts`, repo root
`📜️script.ts` (scaffolder only), this ticket's `🔧️patches/`. Everything else patched (see `## sharedFileRequests`).

## 0. Unplanned prerequisite — a live peer left `🔍️discovery/🟦️component.ts` uncompileable

Before any of my own edits, `bun ./📜️script.ts verify taxonomy enforce` failed with a hard compile error:
`"DISCOVERY_SKIP_DIRS" has already been declared` (two `const DISCOVERY_SKIP_DIRS` at module scope). `git status`
showed this file (and `🔣️taxonomy.json`, `📜️script.ts`) already modified-but-uncommitted, with a
`_languageNeutralityComment` referencing ticket `26/08/17/LANGUAGE-NEUTRAL-TAXONOMY-AND-PACKAGE-PURITY` — a live
peer, not this ticket. File mtime was 4.5 minutes old at the time I first touched it, and the file kept changing
under me for the rest of the session (confirmed by re-reading it between edits and by the final `git diff`, which
shows several more of the peer's own hunks landing in the same region after my fix).

Grepping confirmed the second `DISCOVERY_SKIP_DIRS` declaration had exactly one reference in the whole file (the
pre-existing first one, at the one call site) — a pure accidental duplicate, not two competing designs. I deleted
the newer, unused duplicate (its docstring + one line) to restore compileability for everyone, since this is a file
inside my own boundary and the fix touches zero design decisions. This is **not** part of my ticket's task list; it
is recorded here for traceability. The peer's own further edits since then (visible in the final diff below) show
they self-resolved their duplicate independently — my deletion and their later edits did not conflict.

`bun nx run @semio-tech/plugin-registry:check` remained blocked throughout, by an unrelated, separately pre-existing
bug: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` imports
`../../../../../../../../🧰️framework/…/📦️index.ts` — 8 `../` from a directory only 6 levels deep, resolving
**above** the repo root (`/Users/ueli/🧰️framework/…` instead of `/Users/ueli/Documents/semio/🧰️framework/…`).
`git show <ticket-start-commit>:<path>` proves this file did not exist at the ticket's start commit; its mtime
(16:48:52) is about an hour after ticket start and unrelated to any file I or the two known-live peer tickets
touch. I did not fix this — it is not a trivial, uncontroversial syntax collision like the one above, it is out of
my boundary, and CLAUDE.md forbids guessing at another owner's intended directory depth. Reported as `blocked-peer`
for the `check` gate specifically (see `## verification`).

## 1. Read the stale patch, re-derived for the corrected shape

`🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt` proposed a NEW branch in `artifactFacetChildLevel`
recognizing `schemaChildDirs` members as `io/<direction>/<codec>/<kind>` children — i.e. still the rejected mirror
(native codec duplicated under both `📥️import` and `📤️export`). Discarded per the ⚠️ CORRECTION in `📓️design.md` §1.

**Key discovery while re-deriving**: the walker already had a second, *correct*, precedent for exactly this shape —
`ioSemanticCollectionDirNames` (`["💡️inferences", "🧬️mutations"]`) already made `🚪️io/🧬️mutations/{📝️text,💾️binary}`
and `🚪️io/💡️inferences/{📝️text,💾️binary}` legal **directly under `🚪️io`, unsplit**, with zero import/export
involvement — because those two facets' io representation is genuinely a single bidirectional codec, exactly like
snapshot/diff now need to be. Real on-disk proof this precedent is live and working, not dead vocabulary:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/💡️inferences/` already has a real
`🔣️component.json` collection manifest declaring exactly `📝️text`/`💾️binary` as its two members.

**Conclusion**: the corrected shape needs **zero new branches** in `artifactFacetChildLevel`. Extending
`ioSemanticCollectionDirNames` to include `📸️snapshot`/`🔺️diff` reuses the exact mechanism already proven for
mutations/inferences — the `parents.length===1/2/3` branches under the `🚪️io` root already generalize over
whatever `ioSemanticCollectionDirNames` contains. This is the "clean long-term solution" CLAUDE.md asks for: no
new taxonomy key, no new walker code, one array extended.

## 2. The corrected vocabulary diff (`🔣️taxonomy.json`)

- `ioSemanticCollectionDirNames`: `["💡️inferences","🧬️mutations"]` → `["📸️snapshot","🔺️diff","💡️inferences","🧬️mutations"]`.
- `taxonomyLeafParentDirs`: added `"📸️snapshot"` (it was missing — `🔺️diff` was already present; a pre-existing
  asymmetry, now fixed as a required consequence of the `ioSemanticCollectionDirNames` membership check).
- `semanticCollections`: added `"🚪️io/📸️snapshot"` and `"🚪️io/🔺️diff"`, both `{"kind":"io","direction":"transport"}`
  — `"transport"` (not `"export"`) because both are genuinely bidirectional, matching `🧬️mutations`'s existing
  choice; `💡️inferences` alone stays `"export"` since inferences are derived-only, never imported.
- `artifactSpecFilenames`: added all 4 entries —
  `🚪️io/📸️snapshot/{📝️text→📖️component.grammar.semio, 💾️binary→📡️component.protocol.semio}`,
  `🚪️io/🔺️diff/{📝️text→📖️component.grammar.semio, 💾️binary→📡️component.protocol.semio}`.
- `artifactSchemaSpecFilenames`: added `🚪️io/📸️snapshot/📝️text→🔣️component.json`,
  `🚪️io/🔺️diff/📝️text→🔣️component.json` (mirroring the existing `🚪️io/🧬️mutations/📝️text` /
  `🚪️io/💡️inferences/📝️text` convention — binary reps carry no JSON Schema entry there either, pre-existing pattern,
  unchanged).
- `_cleanMechanismComment`: rewritten to state the corrected rule explicitly — `import`/`export` express direction
  and exist **only** for foreign dialects; the native codec is one bidirectional thing per facet at
  `🚪️io/<facet>/{📝️text,💾️binary}`, unsplit; states the one-impl-per-trait reason (`ArtifactDsl`/`ArtifactPack`,
  exactly one impl per type in Rust) inline so nobody re-derives the rejected mirror from the taxonomy comment alone.

The old `🧬️schema/*/{📝️text,💾️binary}` shape is untouched and stays legal (coexistence during migration, per the
ticket's explicit instruction — W6 removes it).

## 3. The walker change (`🔍️discovery/🟦️component.ts`)

`artifactFacetChildLevel` itself is **unchanged** — see §1. Two edits:
1. Doc comment on `ioSemanticCollectionDirNames` rewritten to describe all four native facets and the
   direction/foreign-dialect distinction (was previously undocumented beyond "owned coded boundary results").
2. `validateTaxonomy`'s required-membership check: `["💡️inferences","🧬️mutations"]` → `["📸️snapshot","🔺️diff","💡️inferences","🧬️mutations"]`
   — makes the corrected shape a permanent contract of this taxonomy, not just an incidental array value.

## 4. Scaffolder correction (`📜️script.ts`, `newScaffoldIoTree`) — not in the original 4 tasks, fixed anyway

W1-B's Task 3 scaffolder (`bun ./📜️script.ts new subset …`) generated the **rejected mirror shape**
(`🚪️io/{import/deserializers,export/serializers}/{snapshot,diff,mutations,inferences}/…`) because it was written
before the W2-P correction landed. Left as-is, every future `new subset` invocation would scaffold structurally
invalid directories matching neither the legal old shape nor the corrected new one. This is squarely inside my
boundary note ("repo root `📜️script.ts` … only if a policy needs to learn the corrected shape") and directly
downstream of the exact vocabulary I corrected, so I fixed it in the same pass rather than leave it half-done:
`newScaffoldIoTree` now iterates `ioSemanticCollectionDirNames` directly under `${ioRel}` (no direction/codec
split) — `📸️snapshot`/`🔺️diff` get `representationDirs` leaves, `🧬️mutations`/`💡️inferences` get the existing
empty-facet marker. No foreign-dialect (`import`/`export`) dirs are scaffolded generically, since a generic
scaffolder cannot know which foreign dialect a not-yet-written subset will consume — matches the ticket's own
recipe (`📓️design.md` §5, foreign leaves are per-dialect, hand-authored). Verified with a dry-run (see
`## verification`); no test in the repo covers this function (`grep -rl newScaffoldIoTree` across `*.ts` returns
only `📜️script.ts` itself), so no test-side fallout.

## 5. Proof the walker accepts the new shape

**(a) Direct predicate proof** — the exact function every consumer calls
(`bun -e` against the real loaded taxonomy, `📜️design.md`-shape paths):

```
validateTaxonomy problems: 0
🚪️io/📸️snapshot/📝️text                              => true
🚪️io/📸️snapshot/💾️binary                             => true
🚪️io/🔺️diff/📝️text                                   => true
🚪️io/🔺️diff/💾️binary                                  => true
🚪️io/🧬️mutations/📝️text                              => true   (unchanged, still legal)
🚪️io/💡️inferences/💾️binary                            => true   (unchanged, still legal)
🧬️schema/📸️snapshot/📝️text                           => true   (old shape still legal — coexistence)
🚪️io/📥️import/🧩️deserializers/🗿️artifacts            => true   (foreign-dialect shape untouched)
🚪️io/📥️import/🧩️deserializers/📸️snapshot             => false  (the REJECTED mirror — correctly refused)
🚪️io/📸️snapshot/📝️text/extra                         => false  (leaf, no further children)
```

**(b) Real disk proof** — built a throwaway fixture at
`.🧬semio/🦑️repo/🎫️tickets/…/🔬️w1e-scratch-proof/✏️s/🔌️plugins/🆕️zzw1e/🗿️artifacts/🆕️zzart/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/`
containing a real `🔣️component.json` collection manifest (2 members: `📝️text`, `💾️binary`, kind `io`, direction
`transport`) and one `🦀️component.rs` leaf under each member dir, then ran the actual production entry point,
`buildSemanticCensus(scratchRoot, {}, loadTaxonomy())` — the same function `verify taxonomy enforce` calls on the
real repo. Result: **0 problems mention `📸️snapshot`** (no `collection-manifest-missing`,
`manifest-child-missing`, `member-component-leaf-missing`, or `taxonomy-schema`). The census did report 12
unrelated problems, all about the fixture's own ancestor collections (`✏️s/🔌️plugins`, `🗿️artifacts`,
`🏅️standards`, `🪆️subsets`) missing *their* manifests — expected, since I deliberately built the minimal fixture
for only the one facet under test; zero of those 12 reference `🚪️io` or `📸️snapshot`. Scratch tree deleted after
(`rm -rf`; confirmed empty).

**(c) Scaffolder proof** — `bun ./📜️script.ts new subset writer writer 1 ✳️zzw1etest --dry-run` now lists 17 files
(down from W1-B's 23, since the import/export mirror halves are gone), including
`🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️component.rs`, `🚪️io/🔺️diff/{📝️text,💾️binary}/🦀️component.rs`,
`🚪️io/💡️inferences/📌️empty.md`, `🚪️io/🧬️mutations/📌️empty.md` — zero `📥️import`/`📤️export` paths. Dry-run only
(0 files written; the real `writer` plugin tree is untouched).

## verification

All commands from `/Users/ueli/Documents/semio`. Ticket start commit `101a6b4ea83acc82d6fbdc0607e6ae5d876825ae`.

### `bun ./📜️script.ts verify taxonomy enforce`

| when | error findings | notes |
|---|---|---|
| before (after only the DISCOVERY_SKIP_DIRS unblock, §0) | **10887** | `🧪️w1e-before-taxonomy-enforce.txt` |
| after (my taxonomy.json + discovery.ts edits) | **10789** | `🧪️w1e-after-taxonomy-enforce.txt` — **decreased**, satisfies "must not increase" |

The ticket brief's `10723` (W2-P) could not be reproduced or used as a baseline — the file did not even compile at
the start of this session (§0), and multiple other tickets are live in this same file/repo, so any two timestamps
will show drift regardless of my change. My change is additive-only to an allow-list (`artifactFacetPathIsDeclared`
can only go false→true from new keys, never true→false), so it is monotonically safe by construction; the measured
-98 is consistent with that but not solely attributable to me given concurrent peer commits between the two runs
(dominant error kind both times: `collection-manifest-shape` on `🪆️subsets/🔣️component.json`, pre-existing,
untouched by this ticket).

### `bun ./📜️script.ts policy`

| when | high-priority total | rules | clean-mechanism total | clean-mechanism high |
|---|---|---|---|---|
| before | 25313 | 36 | 2830 | 0 |
| after | 25326 | 37 | 2830 | 0 |

Full breach cache diffed by kind (`.🧬semio/🦑️repo/⚡️cache/breaches/compose.json`, snapshotted to
`🧪️w1e-before-breaches-compose.json` / `🧪️w1e-after-breaches-compose.json`):

| policy | before | after |
|---|---|---|
| `clean-mechanism/io-exclusivity` | 1134 | 1134 |
| `clean-mechanism/subset-isolation` | 1117 | 1117 |
| `clean-mechanism/owner-mounts-children` | 344 | 344 |
| `clean-mechanism/io-declaration` | 112 | 112 |
| `clean-mechanism/module-consumer-count` | 62 | 62 |
| `clean-mechanism/subset-standalone` | 61 | 61 |
| `clean-mechanism/declaration-tree` | 0 | 0 |

All seven identical before/after — my change touched none of the code these seven policies scan (pure taxonomy
vocabulary + doc comments). The +13 high-priority / +1-rule drift outside `clean-mechanism/*` is unrelated peer
churn (two other tickets are live in this repo this session, see `📓️status.md`). (Sub-counts vs W1-B's originally
reported 1132/1117/344/112/59/61 also drifted slightly — io-exclusivity +2, module-consumer-count +3 — between W1-B's
measurement and mine; that drift predates this report's own before/after window, which is internally flat.)

### `bun nx run @semio-tech/plugin-registry:check` (substitute for `bun ./📜️script.ts check`, which is not a real
subcommand — confirmed: `unknown command "check"`; `check` only exists as this nx target, the same one W1-B ran)

**blocked-peer, both before and after, identical failure**: `Cannot find module
'../../../../../../../../🧰️framework/…/📦️index.ts'` from `🔌️plugin/📇️registry/📜️script.ts` (proof of pre-existing,
non-ticket origin in `## 0` above). `🧪️w1e-before-registry-check.txt` / `🧪️w1e-after-registry-check.txt` are
byte-identical in failure signature — my change neither caused nor worsened this.

### `cd 🧰️framework/…/📦️packages/🟦️typescript && bun test ./🧪️index.test.ts`

Ticket-stated baseline: **188 run / 20 fail**. My own "before" run was itself blocked by a live peer mid-move
(`Cannot find module '…/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts'` — the peer had already relocated
the file to the owner root but not yet updated every reference; resolved by the time of my "after" run, see `🧪️w1e-before-repolib-tests.txt`).

**After**: **190 run / 22 fail** (`🧪️w1e-after-repolib-tests.txt`). Diffed against the ticket's stated baseline
categories and against a fresh grep of the 22 failures for every key this report touched
(`ioSemanticCollectionDirNames`, `artifactSpecFilenames`, `artifactSchemaSpecFilenames`,
`semanticCollections["🚪️io/…"]`, `taxonomyLeafParentDirs`): **exactly one** failure is caused by me —

```
🧪️index.test.ts:1537  expect(taxonomy.ioSemanticCollectionDirNames).toEqual(["💡️inferences", "🧬️mutations"]);
```

— a required, direct consequence of extending that array, exactly analogous to W1-B's `schemaVersion`/`childDirs`
fixes. Fixed via patch (see `## sharedFileRequests`, not applied directly — that file is out of my boundary). The
other 21 failures are unrelated: `snapshotChildDirs`/`osChildDirs`/`exampleAssetKindPrefixes` (pre-existing,
ticket brief and/or independent drift, none reference my keys), `resolveCargoPackageName`/`dependency-boundary`/
`ui-scrollbar-css`/`commit`/`micro-commit`/`command budgets`/`playground ports` (pre-existing, unrelated), and
`discoverPackages`/`discoverBurndown`/`computeWorkspaces` (the live `LANGUAGE-NEUTRAL-TAXONOMY-AND-PACKAGE-PURITY`
peer ticket's own package-purity feature and its still-in-flight package moves).

## sharedFileRequests

1. `🔧️patches/w1e-index-test-io-semantic-collection-dirs.txt` →
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:1537` — one-line fix,
   required consequence of the `ioSemanticCollectionDirNames` extension (see `## verification` above for the exact
   diff and reasoning).
2. `🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt` — **overwritten** with a short SUPERSEDED pointer (not
   deleted) so nobody applies the rejected mirror shape it used to describe; the real change landed directly since
   `🔍️discovery/🟦️component.ts` and `🔣️taxonomy.json` are both inside my own boundary.

## openQuestions

1. **`bun nx run @semio-tech/plugin-registry:check` is broken independent of this ticket** (§0) — a real, unrelated
   bug (off-by-two `../` in a script created ~an hour after this ticket's start commit, by neither of the two known
   live peer tickets nor this one). Someone needs to fix
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`'s two import lines (currently
   `../../../../../../../../🧰️framework/…`, should be 6 `../`, not 8) before `check`/`verify` (non-taxonomy modes)
   can run clean again. Flagged, not fixed — outside my boundary and not a trivial, uncontroversial one-line
   collision like the `DISCOVERY_SKIP_DIRS` duplicate was.
2. **The repo-lib "before" baseline for this session could not be independently reproduced** — my own first attempt
   was itself blocked by a live peer's in-flight file move (§0, resolved by the time of the "after" run). I relied
   on the ticket brief's stated 188/20 baseline plus a targeted diff of the 22 "after" failures against the keys I
   actually touched, rather than a byte-identical before/after pair. I'm confident in the attribution (only 1 of 22
   failures references anything I changed) but flag the methodology gap for the record.
3. **Debt not opened by me, but adjacent**: the scaffolder fix (§4) means any *already-scaffolded* subset from
   before this report (if any exist — none were found on disk under the rejected mirror shape) would need manual
   cleanup. None exist today (verified: `find . -type d -path "*🚪️io/📥️import*"` under `🗿️artifacts/**` finds only
   the pre-existing real stdio foreign-dialect leaves, never a scaffolder-generated native-codec one), so this is
   theoretical, not a live cleanup task.
