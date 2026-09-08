# WP0 Pass A — Repository Classification Ledger

Commit audited: `0a0bb743800e92924221e4bd6484c083c3989965` (`git rev-parse HEAD` at run time). The worktree is dirty
and shared with concurrent sessions; every byte in this ledger was read from git's object store via
`git ls-tree -r -z --long HEAD` and `git cat-file --batch`, never from the working directory.

Script: `wp0-ledger.py` (repo-root relative, reproducible — rerun with `python3 wp0-ledger.py`).
Outputs: `📊️wp0-ledger.json` (757 KB, aggregate + capped listings), `🗑️generated/wp0-ledger-full.jsonl`
(84,485 lines, one row per tracked entry, no cap).

## Method

1. `git ls-tree -r -z --long HEAD` parsed NUL-safe (no shell quoting of emoji paths); 84,485 entries.
2. Each path classified into **one primary bucket** by priority order (below) plus a set of independent
   tags recording every rule it also matches.
3. A single `git cat-file --batch` pass (67,325 unique blob SHAs, content-addressed cache so identical
   blobs are read once) computed, for every blob's first 8 KB: LFS-pointer, binary (NUL-byte) and
   oversized (>2 MB) flags. `.json`-named blobs outside `historical-ticket`/`vendor-or-build` (22,344
   path-entries, 15,200 distinct blobs after dedup) were additionally read in full and parsed for schema
   content flags.
4. Cross-platform integrity checks (case-fold, NFC normalization, variation-selector stripping, length,
   trailing space/dot) ran over all 84,485 raw paths.

### Bucket priority (a path gets exactly one primary bucket; all matching rules are still recorded as tags)

`historical-ticket` > `vendor-or-build` > `schema-module-file` > `fixture-or-test-tree` > `schema-named-file`
> `schema-format-file` > `json-other` > `source` > `config` > `doc` > `asset` > `other`.

Rationale: being physically inside a ticket folder or a `🧬️schema/` module is the strongest structural
signal for this audit's purpose (schema ownership), so those win over filename-pattern buckets. Extension
fallback (`json-other`/`source`/`config`/`doc`/`asset`) only applies once none of the structural/pattern
rules matched; **`.json` wins over `config-ext`** in that fallback, so `package.json`/`tsconfig.json`/`nx.json`
outside historical-ticket land in `json-other` (tagged `config-ext` too) — see "Surprising results" below.

## Bucket counts

| bucket | count |
|---|---:|
| schema-module-file | 37,540 |
| historical-ticket | 22,777 |
| source | 7,115 |
| fixture-or-test-tree | 6,532 |
| doc | 4,897 |
| asset | 2,507 |
| json-other | 1,237 |
| other | 1,146 |
| schema-format-file | 428 |
| config | 195 |
| schema-named-file | 110 |
| (submodule) | 1 |
| **total** | **84,485** |

## Tag counts (a path can carry several)

| tag | count |
|---|---:|
| schema-module-file | 37,540 |
| json | 26,659 |
| source-ext | 24,072 |
| historical-ticket | 22,777 |
| fixture-or-test-tree | 21,693 |
| doc-ext | 12,607 |
| schema-format-file | 6,245 |
| asset-ext | 4,240 |
| schema-named-file | 2,528 |
| config-ext | 440 |

## JSON content flags (non-historical-ticket/vendor blobs only; 22,360 path-entries carry flags — see
methodology note below on the 28-entry overspill)

| flag | count |
|---|---:|
| hasDefs=False | 21,682 |
| hasDefs=True | 678 |
| hasDollarId=False | 20,664 |
| hasDollarId=True | 1,696 |
| hasDollarSchema=False | 18,930 |
| hasDollarSchema=True | 3,430 |
| hasXSemioKeys=False | 21,698 |
| hasXSemioKeys=True | 662 |
| isBooleanSchema=False | 22,360 (0 boolean-schema blobs found) |
| isObjectTypeWithProperties=False | 18,987 |
| isObjectTypeWithProperties=True | 3,373 |

`schema-module-file` leaf-kind breakdown (top): 16,157 `🔣️.json`, 10,733 `🦀️.rs`, 2,494 `🟦️.ts`, 2,049
`🧬️.schema.json`, 1,455 `🛰️.proto`, 1,453 `🔗️.graphql`, 446 `📖️.grammar.semio`, 403 `📡️.protocol.semio`,
402 each of `🌶️.spicy`/`🔠️.abnf`/`🥋️.ksy`/`🅰️.g4`/`🔤️.ebnf`, 214 `🚫️.absent` (zero-byte mutation-test
diff markers), 37 `🔣️.schema.json`. Full per-file rows for this bucket are only in the JSONL (too large for
the 2 MB summary cap) — see "Deviation from spec" below.

`schema-format-file` (428, all outside a module): 103 `.grammar.semio`/`.protocol.semio`, 47 `.graphql`,
46 each `.spicy`/`.abnf`/`.ksy`/`.g4`/`.ebnf`/`.proto`, 2 `.sql`.

## Coverage (entries that could not be fully read/classified — the audit's denominator gaps)

- **Submodules (mode 160000):** 1 — `♻️mit-bestand/🔎️recherche` (commit `92036c7c…`), content not enumerable
  from this repo's object store; out of scope for the ledger's blob-level checks by definition.
- **Symlinks (mode 120000):** 15, including **`CLAUDE.md` itself** (repo root) and 9 files under
  `.🧬semio/…/PRINT-SOLID-HEADING-CHIP-ROW-PARITY/harness/{font,tex}/…`. Confirmed via direct
  `git ls-tree HEAD CLAUDE.md` (mode `120000`), not a parsing artifact.
- **Git LFS pointers:** 0 found (repo does not use LFS at HEAD).
- **Binary blobs (NUL in first 8 KB):** 4,288, mostly `asset` (2,116), `historical-ticket` (924, screenshots/
  attachments inside ticket folders), `fixture-or-test-tree` (771, mostly `♻️mit-bestand` logo/report assets),
  `other` (476, `.glb`/`.3dm` 3D assets). **One false positive** is worth flagging: a `schema-module-file`
  `.g4` (ANTLR grammar) at
  `✏️s/🔌️plugins/🗄️stdio/…/rfc8259/…/🧬️schema/📸️snapshot/📝️text/🅰️.g4` contains a literal
  `\x00-\x1f` control-character range inside a lexer rule (`~["\\\x00-\x1f]`) — it decodes as valid UTF-8
  text, the NUL is source code, not binary content. The naive "NUL in first 8 KB" heuristic (as specified)
  flags it anyway; noted here so a schema-content auditor isn't misled by the `isBinary=true` flag on this file.
- **Oversized (>2 MB):** 201 blobs. Largest: a 23.3 MB TIFF fixture under `stdio`'s tiff artifact tests, a
  22.3 MB PDF and 16.1 MB MP4 under `♻️mit-bestand/…präsentation…`, several TIFF fixtures duplicated
  across `baseline`/`document` subsets (identical 17.5 MB content, same blob, two paths), a **12.4 MB
  Markdown file** (`.🧬semio/…/ZERO-WARNINGS-…/📓️2026-09-07-verification.md` — a pasted build log, not
  prose), and an **11.5 MB JSON fixture** (`…step/…/expected.mesh.json`) that was still parsed successfully
  (under the 20 MB parse cap).
- **JSON parse errors:** 12, all JSONC-with-comments files misclassified as strict JSON by extension:
  `.claude/settings.json`, `.cursor/hooks.json`, `.devcontainer/devcontainer.json`, `.factory/hooks.json`,
  `.github/hooks/{compose-repo,repo}.json`, `.kiro/agents/{compose,repo}.json`, `.vscode/launch.json`,
  `.windsurf/hooks.json`, a module `tsconfig.json`, and one nested `.vscode/launch.json`. These are tool
  config files using `//` comments; not schema-relevant, but recorded so they don't silently vanish from
  the denominator.
- **Invalid path encoding (lone UTF-8 surrogates):** 0.
- **Executables (mode 100755):** 57 (sampled in the JSON summary).

## Cross-platform integrity collisions

- **Case-insensitive duplicates:** 0.
- **NFC/NFD normalization collisions:** 0 (no two distinct raw paths normalize to the same NFC form; no
  path found in non-NFC form colliding with another).
- **Variation-selector collisions:** **1.** Two files in the same historical-ticket directory differ only
  by the presence of U+FE0F on `🧪`:
  `…FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🧪test-final2.txt` (no VS) vs.
  `…/🧪️test-final2.txt` (with VS). Both exist simultaneously at HEAD — a real filesystem-portability risk
  (case/VS-insensitive filesystems, e.g. default macOS/Windows, cannot check out both). Historical-ticket
  scratch file, not schema-relevant, but flagged per the audit's integrity-check requirement.
- **Paths over 240 characters:** 0.
- **Trailing space/dot in any path segment:** 0.

## Deviation from spec (documented, not silent)

The task asked for full `[{path, blob, size, tags, flags}]` rows in `📊️wp0-ledger.json` for
`schema-module-file`, `schema-named-file`, `schema-format-file`, and schema-flagged JSON in fixture/test
trees and `json-other`, while keeping the file under 2 MB. `schema-module-file` alone has 37,540 entries;
a full per-row listing for it would run well past 2 MB on its own (est. 10–15 MB). To honor both
constraints without dropping anything: the summary JSON includes full rows for `schema-named-file` (110),
`schema-format-file` (428), and schema-flagged JSON in fixture/test-tree or json-other buckets (880) —
all comfortably small — plus an aggregate `schemaModuleLeafKinds` breakdown for `schema-module-file`.
The complete per-file rows for `schema-module-file` (and every other bucket) are in
`🗑️generated/wp0-ledger-full.jsonl` (filter `bucket=="schema-module-file"`); nothing was dropped, only
moved out of the size-capped file. Final summary size: 757,488 bytes.

## Methodology note: 28-entry JSON-flag overspill

`byFlag` counts (e.g. `isBooleanSchema=False` = 22,360) exceed the strict "non-historical-ticket/vendor
`.json` path-entries" count (22,344) by a small margin, and exceed count of unique qualifying blobs by
more. Cause: the `git cat-file --batch` pass is content-addressed (dedup by blob SHA, 15,200 unique blobs
for 22,344+ path-entries), and the "needs full parse" decision is made **per blob**, not per path — so if
any one path referencing a given blob is outside `historical-ticket`/`vendor-or-build`, every path sharing
that identical blob (including duplicate copies that happen to sit inside a historical ticket) also gets
JSON flags computed and counted. This is conservative (more coverage, never less) and does not affect
completeness of the denominator; it is noted here so downstream auditors don't over-interpret exact
byFlag-vs-bucket-count arithmetic.

## Surprising results

1. **44% of the entire repository (37,540 / 84,485 tracked entries) lives inside a `🧬️schema/` module
   directory.** This corroborates the master plan's ~495-module estimate and confirms schema modules are
   the dominant structural feature of this repo, not a peripheral concern.
2. **`CLAUDE.md` at the repo root is a symlink** (mode `120000`), not a regular file — confirmed directly
   via `git ls-tree`, independent of this script's parsing.
3. Config files with `$schema` references (`nx.json`, `tsconfig.json`, `package.json`, `📋️project.json`)
   land in `json-other` rather than `config`, purely as a consequence of the bucket-priority rule
   (`.json` extension wins over `config-ext` in the fallback tier). Not a bug — flagged so a later auditor
   filtering `bucket=="config"` doesn't miss these.
4. Content-level duplication is significant: 22,344 qualifying JSON path-entries resolve to only 15,200
   distinct blobs — roughly a third of schema-relevant JSON leaf files are byte-identical copies of
   another file's content (plausible given ~2,000+ mutation-leaf `🧬️.schema.json` stubs and 214 zero-byte
   `🚫️.absent` markers).
5. One `.g4` grammar file inside a `schema-module-file` legitimately contains a raw `\x00` byte as part of
   its own lexer source (an ANTLR character-class exclusion), tripping the binary-NUL heuristic as a false
   positive — see Coverage section.
6. A 12.4 MB Markdown file inside a historical ticket (a pasted build-verification log) is by far the
   largest non-schema oversized entry — larger than most of the tracked media assets.

## Reproducing

```
cd /Users/ueli/Documents/semio
python3 "./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/wp0-ledger.py"
```
Runtime ~100s (67,325 unique blobs, ~2.5 GB total tracked content read once via `git cat-file --batch`).
