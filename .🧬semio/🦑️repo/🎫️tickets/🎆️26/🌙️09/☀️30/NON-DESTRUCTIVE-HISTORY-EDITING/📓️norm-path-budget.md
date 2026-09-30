# 📓️ Norm path budget — measurements and options (decision input, nothing changed)

Measured 2026-09-30 by W2-W-norm-2. Command: `python3 🧪️norm-path-budget.py` (ticket root); output in
`🗑️generated/w2w-norm-2/path-budget.txt`. It walks every file on disk under `✏️s/🔌️plugins/📕️norm` (build output excluded)
and every tracked or untracked file of the repository that exists on disk. The budget is the taxonomy's 240 UTF-8 bytes,
which `verify taxonomy` reports as `path-too-long` and `mutation-pair-path-budget`.

## 1. Where norm stands

- **Norm files:** 9,392, of which **814 exceed 240 bytes**.
  - 737 are mutation evidence under `🧫️fixtures/🧬️mutations`.
  - 71 are canonical test sources under `🧬️schema/🧬️mutations/<leaf>/🧪️tests/<case>/🦀️.rs`.
  - 6 lie outside the subsets.
- **Evidence files:** 2,516, of which 737 are over budget.

| Artifact | Files | Over 240 |
|---|---|---|
| ⚖️en1990 | 551 | 0 |
| ⚡️din18599 | 422 | 49 |
| 🌍️en1997 | 385 | 0 |
| 🌬️din16798 | 656 | 97 |
| 🏋️en1991 | 1,082 | 2 |
| 🏛️en1992 | 550 | 0 |
| 🏭️vdi3805 | 472 | 93 |
| 📇️iso16757 | 615 | 152 |
| 🔩️en1993 | 647 | 0 |
| 🧩️en1994 | 457 | 36 |
| 🧱️din4108 | 677 | 106 |
| 🪨️en1996 | 856 | 146 |
| 🪵️en1995 | 948 | 127 |
| 🪶️en1999 | 369 | 0 |
| 🫨️en1998 | 526 | 0 |

The artifacts with 0 are exactly those whose conversion has not committed full bundles yet, or whose leaf and case
names are short.

**Segment bytes of an evidence path** (min / median / max). Example path:
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/<a>` + `/🏅️standards/🔖️1/🪆️subsets/✳️any` + `/🧫️fixtures/🧬️mutations` + `/<leaf>` + `/<case>` + `<tail>`.

| Segment | Bytes |
|---|---|
| prefix to the artifact (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/<a>`) | 64 / 65 / 67 |
| standard + subset (`/🏅️standards/🔖️1/🪆️subsets/✳️any`) | 51 / 51 / 51 |
| `/🧫️fixtures/🧬️mutations` | 33 |
| `/<leaf>` (for example `/↔️change-concentrated-bearing-length`) | 14 / 27 / 50 |
| `/<case>` (for example `/↔️applies-change-concentrated-bearing-length`) | 6 / 22 / 63 |
| tail `/📸️snapshot/⬅️before/🔣️.json` | 42 |
| tail `/📸️snapshot/➡️after/🔣️.json` | 41 |
| tail `/🦠️mutation/🔣️.json` | 29 |
| tail `/🎯️outcome/🔣️.json` | 28 |
| tail `/🔺️diff/🚫️.absent` | 27 |
| tail `/🔺️diff/🔣️.json` | 25 |

**Arithmetic.** The fixed part is 65 + 51 + 33 + 42 = 191 bytes before the leaf and the case. That leaves **49 bytes
for `/<leaf>/<case>` together**. The median leaf alone takes 27 of them.

**Longest path:** 281 bytes, from EN 1996.

- Makeup: 65 + 51 + 33 + leaf 41 + case 49 + tail 42.
- Path: `…/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/↔️change-concentrated-bearing-length/↔️applies-change-concentrated-bearing-length/📸️snapshot/⬅️before/🔣️.json`

**Note on the case names.** The case directory repeats the leaf's kind (`applies-change-…`). That scheme predates this
WP: it reuses the existing canonical test directory, so the vector registry resolves. It is also what makes the 71
schema-side test paths exceed the budget.

## 2. Repository context (blast radius)

- **Repository files:** 112,138, of which **7,353 exceed 240 bytes**.

  | Plugin | Files over 240 |
  |---|---|
  | 🔋️energy | 1,251 |
  | 🗄️stdio | 1,171 |
  | 📕️norm | 808 |
  | 🏛️architect | 793 |
  | 🧩️puzzle | 407 |
  | 📸️remodel | 302 |
  | 🌀️procedural | 268 |
  | 🏗️fem | 265 |
  | 🧱️block | 208 |
  | 🎥️shooting | 170 |
  | 🌍️gis | 162 |
  | 🀄️wfc | 146 |

  Norm is one instance of a repository-wide pattern.
- **Files under a `🏅️standards/<v>/🪆️subsets/<s>` layout:** 65,504. The largest shares are stdio 23,952, norm 9,228,
  energy 5,304, architect 3,267, puzzle 2,874 and fem 2,659.
- **Standard profiles:** 103, of which **75 are single-subset** (only `✳️any`). Those 75 hold 40,775 files.
- **Mutation evidence bundles (`🦠️mutation/🔣️.json`):** 3,344. By plugin: stdio 582, energy 580, norm 556, architect 270,
  fem 236, puzzle 218, remodel 136, framework 124, block 105, wfc 77, procedural 47, layout 45.

## 3. Options

"Remaining" counts the norm evidence files still over budget after the change. Where noted, it also counts the
schema-side test sources.

### A. Collapse the single-subset profile

**Change:** drop `/🏅️standards/<v>/🪆️subsets/✳️any` (51 bytes) where an artifact has one standard and only the `✳️any`
subset.

**Result:** remaining **0** of 737 evidence files. The schema side is 0 as well, because it saves the same 51 bytes.

**Blast radius:**

- Norm alone: 15 artifacts and 9,228 files move.
- Applied as a law: 75 single-subset profiles and 40,775 files across stdio, energy, architect and the other plugins.

**What has to change:**

- the taxonomy law: owner coordinates, `subsetCoordinatesOfOwner`, `case-above-subset` and the `mutationCatalogProblems`
  profile check;
- the catalogs' `standardDirectoryName` and `subsetDirectoryName`;
- every crate root `#[path]` mount (thousands of lines);
- the schema catalog paths, the TS imports, the oracle manifests' relative `$schema` paths, the descriptor `owner`
  fields and the nx inputs.

It is the only single change that clears norm. Being structural, it needs a coordinated wave with a renamer.

### B. Short case directories: one emoji plus a slug of at most about 12 bytes

**Change:** rename the case directories, both the fixture bundle and the canonical test directory, for example
`↔️applies-change-concentrated-bearing-length` → `🎯️sets`.

**Result:** remaining **7** evidence files. They are the longest EN 1996 leaf names with the before and after tails.
Capping the schema-side test directories at 20 bytes clears all **71** of them.

**Blast radius:** norm only.

- Case directories in `🧫️fixtures` and `🧪️tests`.
- The `🔮️oracles/🔣️.json` catalogs, the features, the Python `VECTORS` and the DIN V 18599 `include_str!` paths.
- Mechanical with `🧪️w2w-norm-2-cases.py`; about 620 bundles.

**Costs:**

- Descriptive scenario names are lost.
- The last 7 need shorter leaf names. Leaf names equal the semantic kind, so that is a kind rename, for example
  `change-concentrated-bearing-length` → `change-load-bearing-length`, a wire change.

### C. Flat evidence layout

**Change:** `<case>/🦠️mutation.json`, `<case>/⬅️before.json` and so on, instead of one directory per evidence file.
This saves 12–14 bytes per file.

**Result:** remaining **225**. On its own this is insufficient. Combined with A, the remaining count is 0.

**Blast radius:** the repository-wide bundle contract.

- `MUTATION_FIXTURE_FILES` and `bundleBreach` in the test harness.
- The payload-parity lint, the Rust `mutation_fixture_ops` and the document-contract oracle.
- Every Rust, Python and TS adapter that names bundle paths.
- Every feature's `shared://` URIs.
- 3,344 bundles in 12+ plugins.

### D. Drop U+FE0F from every path emoji

**Result:** remaining **0**. It saves about 3 bytes per emoji, around 10 emoji per path.

**Blast radius:** every emoji-named path in the repository, 112k files, plus the taxonomy member names, the schema
catalog and every reference. It also has to be squared with `pathEmojiPolicy.identity = single-emoji-grapheme`: text-
presentation emoji need FE0F to be a single emoji grapheme. **Not realistic.**

## 4. Summary for the decision

| Option | Norm evidence still over 240 (of 737) | Schema-side still over (of 71) | Scope |
|---|---|---|---|
| A: collapse single-subset profile | 0 | 0 | norm 9,228 files, or 40,775 as a law |
| B: short case directories | 7 (needs 2–3 kind renames for 0) | 0 | norm only, about 620 bundles |
| C: flat evidence files | 225 | 71 | 3,344 bundles, harness and lints |
| A + C | 0 | 0 | as A plus C |
| D: drop FE0F | 0 | 0 | whole repository |

- **Norm on its own:** B is the smallest change that almost clears it, and it touches only norm.
- **Repository-wide:** A clears norm completely and addresses the same 240-byte problem in stdio, energy and architect.
  It is a law-level change.
