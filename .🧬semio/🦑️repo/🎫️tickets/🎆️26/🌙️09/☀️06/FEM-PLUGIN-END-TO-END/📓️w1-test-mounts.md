# 🗺️ W1 — stale pre-truncation fem test-case names outside the crate entry

Scope: the 43 mutation test-case directories renamed by commit `b0dfa0f09b` (2026-09-05 19:04,
"Windows-checkout validation", `R100` renames) under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/*/🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/`.
The truncated on-disk names are authoritative; every literal that still spelled the long name was
rewritten. Predecessor's work (43 crate-entry `#[path]` mounts) is untouched and still resolves.

Mapping: `🗺️truncated-names.tsv` (43 rows, `old<TAB>new`).
Script: `🔨️fix-stale-names.py` (`--check` to dry-run, `--verbose` to print the mapping).
Resolver: `🔨️resolve-mounts.py` gained a `--tree` mode (see §4).

**No compiler was run.** Nothing here claims `cargo check` passes; the claims below are exactly the
static resolutions and greps listed in §4.

---

## 1. Classification rule

Every reference appears in one of two literal *forms*, and the form decides how it is classified:

* **FULL identity** — `🚫️removes-the-unreferenced-timber-material` (leading emoji + kebab rest).
* **STEM identity** — `removes-the-unreferenced-timber-material` (the emoji stripped).

The authority for the two being distinct-but-bound is the test coordinator,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`:

* line 769 — a mutation-catalog scenario is valid only when
  `leadingEmojiIdentity(directoryName).rest === id`, `directoryName` passes the path-emoji statute
  and contains no separator. So **the directory name is authoritative and the `id` is derived from
  it**, never the other way round.
* lines 1471–1500 — `join(sourceTests, scenario.directoryName)` is a real filesystem join;
  a mismatch is reported as `mutation-vector-source-id-mismatch`.
* lines 1516–1527 — every physical child of a `🧪️tests` directory must be registered, else
  `mutation-vector-unregistered`.
* line 766 — `id` must match `MUTATION_ID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*$/`; every truncated stem
  (`removes-the-30f7a2`, `converts-beam-e1-into-a-5d21f5`, …) satisfies it.

The Python host (`…/📦️packages/🐍️python/🐍️.py`) carries no case names at all — it resolves fixtures
from the URIs the `.feature` declares (`uri_in(ctx, needle)`), so the `.feature` is upstream of it.

### The three classes

| class | rule | what it is | action |
|---|---|---|---|
| **(a) path-bearing** | FULL form and (`/`-adjacent, or on a `"directoryName":` line, or inside a Gherkin table row) | `include_str!("…/<case>/…")`, oracle `directoryName`, the `fixture` column that a step interpolates into `asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/…` | **must** become the on-disk truncated name |
| **(b) identity key** | STEM form on an `"id":` line; FULL form as a bare element of a registry array | oracle catalog `scenarios[].id`; `memberNames` of `semanticDirectoryMemberKinds["members-of-tests"]` in `🔣️taxonomy.json` | made consistent with the **directory**, which is the authoritative side |
| **(c) prose** | everything else | the case `🦀️.rs` module docstring, `<kind>/<case>` prefixes in Rust assertion messages, two Gherkin description sentences, three JSON/TS `notes` fields | renamed for consistency — each is a *pointer* to a directory inside an otherwise untouched sentence, so the sentence stays grammatical (`… the committed vector for this very kind is named \`🚫️removes-node-n3-without-6eab3f\`, so the non-cascade IS the specified behaviour`) |

A Gherkin *title* is never a key here — the coordinator takes scenario ids from `@id-…` tags
(`mutate-<kind>`, `inverse-<kind>`, `spec-vector-<kind>`), which name mutation kinds, not cases —
so no title was touched.

Replacement order is FULL first, then STEM, which is sound because (asserted at script start) no old
name or old stem is a substring of another, and no new stem collides with an old stem.

---

## 2. Per-class counts

```
                 files    path    key   prose
🦀️.rs               63     430      0     727
🥒️.feature          12      43      0       2
🔣️.json             12      43     43       2
📜️script.ts          1       0      0       2
🔣️taxonomy.json      1       0     42       0
────────────────────────────────────────────
TOTAL                89     516     85     733     (1334 occurrences)
```

* `🦀️.rs` 63 = 43 case files (docstring + 16–17 assertion labels each → prose only) +
  20 subset/`✳️any` mirror files (5 subsets × 2 artifacts × 2 levels; `include_str!` only → path).
* `🥒️.feature` 12 = 10 subset features (`fixture` column) + the two `🔄️round-trips-the-committed-document`
  features (one prose sentence each).
* `🔣️.json` 12 = 10 subset `🔮️oracle/🔣️.json` (43 `directoryName` + 43 `id`) + the 2d
  `🌐️any/🔮️oracle/🔣️.json` and `🕸️mesh/🧫️fixtures/🔣️.json` `notes` fields.
* `📜️script.ts` = `◻️2d/…/🌐️any/🏭️generator/📜️script.ts`, the generator that emits those `notes`
  (rewritten together so a regeneration stays byte-identical).

The 3 files that carried a stem but no full name (the generator + its two emitted `notes`) were
invisible to a full-name grep — they are why the survey was run on both forms.

---

## 3. The one thing this could not fix: `t008`

`b0dfa0f09b` truncated
`◻️2d/…/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-modal-count-and-halves-the-deformation-scale`
to **`t008`** — a degenerate fallback with **no leading emoji**. Its 42 siblings kept theirs.

Consequences, both verified by reading the gate source:

* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:1117` **throws**
  (`member names must be NFC emoji-leading evidence`) if any `memberNames` entry lacks a leading
  emoji. Registering `t008` in the taxonomy would take the whole taxonomy — and every registry tool
  and dev boot with it — offline repo-wide.
* the test coordinator's line 769 `pathEmojiStatuteFindings(…)` reports a `missing` finding for it,
  i.e. one **reported breach** on this catalog (`directoryName must be one canonical NFC test-case
  identity`), not a crash.

Decision taken (renaming directories is out of W1's remit):

* the **path-bearing and key** sites got the on-disk truth — `include_str!`, the `fixture` column,
  and the oracle catalog's `directoryName`/`id` all say `t008`, so the fixture reads and the
  scenario/directory binding are correct;
* `🔣️taxonomy.json` was **left carrying the stale long name** for this one case (the script prints
  `REGISTRY SKIPPED (no leading emoji, would break taxonomy normalization): t008`), because a
  phantom extra `memberNames` entry is inert (`memberNames` is only ever read with `.includes`)
  whereas the correct entry is fatal.

**Coordinator decision needed — one directory rename.** Rename
`…/🎛️update-analysis-settings/🧪️tests/t008` to an emoji-leading truncation consistent with its
siblings (suggested: `🔢️doubles-the-modal-3fbb1a`, matching the `🔢️doubles-the-7b5381` sibling in
3d), edit that one row of `🗺️truncated-names.tsv`, and re-run `🔨️fix-stale-names.py` — it is
mapping-driven and idempotent, so this is a two-line change. Until then this case is the only
registry/statute breach left in fem.

---

## 4. Verification (all static; no compiler run)

**(i) crate-entry mounts + (ii) every `include_str!` in the tree**
`python3 🔨️resolve-mounts.py --tree`:

```
── ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs
   checked:    343
   resolved:   343
   unresolved: 0
── include_str! across ✏️s/🔌️plugins/🏗️fem
   checked:    855
   resolved:   855
   unresolved: 0
TOTAL unresolved: 0
```

`--tree` is the new mode: it walks every `🦀️.rs` under the plugin and resolves each `include_str!`
against *its own* directory — the rule rustc applies once the entry has mounted the file.

**(iii) repo-wide grep for the 43 old names and the 43 old stems**

`grep -rlF` over `✏️s`, `🧰️framework` (incl. `…/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/`)
and `.storybook`, for both forms → exactly **one** hit:

* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:10268` —
  `🔢️doubles-the-modal-count-and-halves-the-deformation-scale`, the deliberate survivor of §3.

Justified survivors outside those roots — historical ticket records, deliberately not rewritten
(a closed ticket's evidence must keep describing the tree it was written against):

* this ticket: `📓️explore-fem2d-editor.md`, `📓️explore-fem3d-editor.md`,
  `📓️explore-history-drift-gates.md`, `📓️status.md`, `🗺️truncated-names.tsv`;
* `🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📸️source-index-capture-66/…/🔣️taxonomy.json` (a run capture);
* `🎆️26/🌙️08/☀️20/COMPOSE-TO-PUZZLE5D-MIGRATION/📓️census/📓️fixtures-fem.md`;
* `🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/{w14-audit,w15-audit}/*.json`,
  `w16-cross-language/🐍️derive-fem{2,3}d-frame.py`;
* `🎆️26/🌙️09/☀️02/SEPARATE-…-EVERY-MUTATION/🔨️b3-{emit-fem2d,emit-fem3d,fem2d-data}.py`.

**Extra cross-checks run after the rewrite**

* every `🔣️.json` under the fem plugin still parses — 0 failures.
* oracle catalogs vs disk: 50 declared scenarios ↔ 50 physical case directories, set-equal in both
  directions (no `mutation-vector-unregistered`, no phantom); `stem(directoryName) == id` for all
  50; all 50 ids kebab-case. One breach: `t008` has no leading emoji (§3).
* all 22 `🥒️.feature` files: every `asset://` and `local://` URI resolves, including the
  `<dir>`/`<fixture>` templates expanded across their `Examples` rows — 0 unresolved.
* `🔣️taxonomy.json` `members-of-tests` vs disk, restricted to fem: 71 test-directory names on disk,
  70 registered, only `t008` missing (§3). The taxonomy diff is exactly 42 added + 42 removed lines,
  all bare quoted array elements — no structural edit. `memberNames` is *not* stored sorted
  (`🧹️normalization/🟦️.ts:1118` sorts on load), so entries were replaced in place, not re-ordered.
* Gherkin `Examples` tables whose cells changed were re-padded, so the pipes stay aligned; untouched
  table blocks were left byte-identical.

**(iv) same-hazard check elsewhere**

* `✏️s/🔨️modules/🏗️fem` — no `🧪️tests` directories, no directory basename longer than 40
  characters, and `b0dfa0f09b` renamed nothing under it. Not affected.
* subset-level `🧪️tests/<name>/🧫️fixtures/*.snapshot.json` — only two names exist
  (`🏗️timber-portal-frame.snapshot.json`, `🧊️steel-frame.snapshot.json`), neither truncated by
  `b0dfa0f09b`; every `local://` reference to them resolves (checked above). No hazard.

---

## 5. Files changed

* 88 under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/…`
  (63 `🦀️.rs`, 12 `🥒️.feature`, 12 `🔣️.json`, 1 `🏭️generator/📜️script.ts`);
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (42 `members-of-tests` entries);
* ticket: `🔨️fix-stale-names.py` (new), `🔨️resolve-mounts.py` (`--tree` mode), this report.
