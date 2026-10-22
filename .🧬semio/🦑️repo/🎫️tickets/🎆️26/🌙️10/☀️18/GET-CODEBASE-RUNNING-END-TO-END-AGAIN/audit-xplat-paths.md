# 🧭 Cross-Platform Path Audit (Windows MAX_PATH, Bazel Budget)

Read-only audit by `audit-xplat-paths`. Source of paths: `git ls-files -s` (the index, which includes staged adds by other agents, not HEAD).
Tools: `audit-xplat-paths-tools/measure.py`, `measure2.py`, `measure3.py`, `measure4.py`, `measure5.py` (run as `nice -n 10 python3 -I <script>`, stdout only).

## Method

- Length = UTF-16 code units (`len(s.encode("utf-16-le")) / 2`), the unit Windows MAX_PATH counts. U+FE0F (variation selector) is 1 unit, astral emoji are 2 units.
- Checkout root: the literal `C:\Users\abcdefgh\semio\` is **24** units, not 25 as briefed. Sensitivity for 25 and 29 is given below.
- Code points are never above 240 for any path, so UTF-16 units are the binding metric.
- Bazel scenarios: the brief's prefix `C:\Users\abcdefgh\_bazel_x\abcdef12\execroot\_main\bazel-out\x64_windows-fastbuild\bin\` = 87 units. The brief uses an 8-hex output base, but Bazel's output base is a 32-hex MD5, so the realistic prefix is 111 units.

## 1. Direct checkout (root 24 units)

| Threshold | Files |
|---|---|
| > 256 units | **0** |
| = 256 units (exactly at limit, no margin) | 1 |
| > 250 units | 1 |
| > 240 units (risk band) | **42** (21 in plugin tree, 21 in ticket C) |

Total tracked: 195,945 files, 3 symlinks, 0 submodules. No Windows-invalid characters, reserved names (CON, NUL, COM1...) or trailing dot/space segments.

Sensitivity to the checkout root:

| Root | Files > 256 | Files > 259 (MAX_PATH with NUL) |
|---|---|---|
| `C:\Users\abcdefgh\semio\` (24) | 0 | 0 |
| `C:\Users\abcdefgh1\semio\` (25, brief) | **1** | 0 |
| `C:\Users\firstname\src\semio\` (29) | 1 | **1** |

Conclusion: any user name longer than 8 characters breaks the 256 rule with the current tree, and a typical root needs `core.longpaths`.

## 2. The 42 files over 240 units (top 50 = all of them, grouped)

**Group A, ticket C copies of framework source (21 files, 250 to 256 units).** Paths of the form `GET-CODEBASE-RUNNING-END-TO-END-AGAIN/<tool>-tools/staged*/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/...`. Tools: conc-cargo-tools (6), pm-seam-tools, xplat-linux-fix-tools, round3-tools, round4-tools, bzl-default-tools, cache-fix-tools, cargo-fix-tools, own-crash-tools, rcache-tools, conc-ts-tools, fixups6-tools.
- Worst: `...conc-cargo-tools/staged/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🧪️tests/🧫️fixtures/🔣️.json` = 256 units.

**Group B, plugin snapshot fixtures (21 files, 240 to 251 units).** `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<x>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/.../🧪️tests/🧫️fixtures/<case>/📸️snapshot/⬅️before|➡️after/🔣️.json`. Plugins: gis (6), reasoning, trinity, block, writer.
- Worst: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎥️set-camera/🧾️wire-witness/🦠️mutation/🔣️.json` = 250 units.

## 3. Bazel-prefixed overflow (estimate: prefix + rel path)

| Scenario | Prefix | > 256 | > 259 |
|---|---|---|---|
| Brief (8-hex base under `_bazel_x`) | 87 | **18,764** | 15,299 |
| Real 32-hex base under `_bazel_x` | 111 | **47,600** | 43,073 |
| Short user root `C:\b\` + 32-hex base | 89 | 21,050 | 17,615 |
| Source link `s` (`\execroot\_main\s\`), real base | 77 | 8,522 | 6,381 |

Budget for generated outputs with a real base: 256 - 111 = **145 units** for the relative path (today the longest relative path is 232).

Where the 18,764 (brief scenario) come from:
- `✏️s/🔌️plugins/**`: 17,522 (88,881 files; median 166, p90 202, p99 222 units; plugin tree is 45% of tracked files).
- `.🧬semio/🦑️repo/🎫️tickets/**`: 1,044 (GET-CODEBASE 445, INTERACTIVE-JOB-RUNTIME-REFACTOR 238, CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT 139, END-TO-END-OS-HUB-COLLABORATION-MCP 81, FIXTURES-ARE-TESTING-EXAMPLES-ONLY 42).
- `🧰️framework/🛍️products/**`: 198.

## 4. Directories and segments contributing most length

- Plugin subtrees by file count: `✏️s/🔌️plugins/🗄️stdio` 27,937; `📕️norm` 11,152; `🏙️bim` 10,858; `🔋️energy` 6,146; `🏗️fem` 3,880; `🧩️puzzle` 3,784.
- Most-used heavy segments (count, units): `🗿️artifacts` (88,181, 12u); `🏅️standards` (85,355, 12u); `🪆️subsets` (85,272, 10u); `🔖️1` (58,315, 4u); `🧬️mutations` (46,722, 12u); `🧪️tests` (39,416, 8u); `🧬️schema` (33,054, 9u); `🧫️fixtures` (30,100, 11u); `🗄️stdio` (27,937, 8u).
- Emoji overhead: each emoji with VS16 costs 1 unit more than its code points. VS16 appears in 195,450 paths.
- Effect of removing VS16 (no other change): Bazel brief 18,764 to **4,559**; Bazel real base 47,600 to 30,056.
- Effect of also removing the `🔖️1/🪆️subsets/` layers (58,247 files): plugin overflow in the brief scenario 17,522 to **593**; real base 40,801 to 11,356.

## 5. Ticket folder: tracked source copies (forbidden by AGENTS)

Tracked ticket files: 72,524 (37% of tracked). Tracked files whose bytes exactly equal a tracked non-ticket file (blob >= 512 B): **1,492**, of which 1,261 have a source extension (.ts .rs .py .mjs .cjs .sh .json .toml ...).

| Ticket slug | Exact copies |
|---|---|
| GET-CODEBASE-RUNNING-END-TO-END-AGAIN (C) | 1,355 |
| CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT | 62 |
| END-TO-END-OS-HUB-COLLABORATION-MCP | 26 |
| ARCHITECTURE-COURSE-ON-BAZEL | 19 |
| MAP-VECTOR-TILES | 6 |
| 8 more slugs | 1 to 4 each |

Path-mirror copies (a segment equal to a repo top-level dir such as `🧰️framework`, `✏️s`): 943 tracked files (C 433, CLEAN-ARCHITECTURE 324, END-TO-END-OS-HUB 118, SEMANTIC-MUTATIONS-OVERHAUL 28).

Examples (copy <- original):
- `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/block-inventory-owner-inputs/📥️inputs/🚪️command/🦀️.rs` <- `🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🚪️command/🦀️.rs`
- `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/pack-error-producer-proposal/🔌️capture/🟦️.ts` <- `🧰️framework/🔨️modules/🎒️pack/⚠️error/🔌️capture/🟦️.ts`
- `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/puzzle-board-scene-projection-inputs/🧫️fixtures/🔣️.json` <- `✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🧪️tests/🧫️fixtures/🔣️.json`

Also in C: `conc-cargo-tools/staged/...`, `round3-tools/staged/conc-leases/staged/...` and similar `staged` trees, which hold framework source verbatim.

## 6. Git and setup expectations (grep inside 🧰️framework/🛍️products/🦑️repo)

- `🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts` writes `core.symlinks=true` and `core.longpaths=true` into the repo-local `.git/config` (`REPO_LOCAL_GIT_CONFIG`). Its docstring says tracked paths reach **210** UTF-16 units below the clone root. Measured maximum is **232** (256 total with the 24-unit root). The docstring is stale.
- The same file warns when Windows `LongPathsEnabled` is 0 ("tracked fixtures and build outputs exceed 260 characters").
- `README.md` (EN line 668, DE line 1538): Windows needs `LongPathsEnabled` and a short ASCII checkout root; Bazel budget `len(output base) + len(\execroot\_main\s\) + 256` against 259, or 32766 with long paths.
- `README.md` line 670: "No cache path may exceed 256 characters on any host." Tracked paths already sit at this limit (one path at 256).
- Fixtures `🪟️windows-checkout-paths` and `🥾️cross-platform-bootstrap` cover LongPathsEnabled parsing. I did not find a check that enforces the 256-unit rule on tracked paths; the Bazel `paths` test covers the Bazel budget only (not opened in detail).

## 7. Recommendations (per offender group)

1. **Ticket C staged copies (Group A and 1,355 copies):** delete the `*-tools/staged*` copies and point helpers at the original `🧰️framework/...` paths. This fixes the AGENTS violation and removes 21 of 42 over-240 files. Index removal needs a git command the auditor may not run; owner action.
2. **Plugin snapshot fixtures (Group B):** rename `📸️snapshot` to `📸️s` (saves 7 units per file) and keep the `⬅️`/`➡️` markers; or move fixture cases to a top-level `🧫️fixtures/<plugin>/...` tree, which drops the `🧪️tests/🧫️fixtures` chain (19 units).
3. **Plugin `🏅️standards/🔖️1/🪆️subsets/✳️any`:** flatten to `🏅️standards/✳️any`. Saves 16 units per file across 58,247 files; the largest single lever after VS16.
4. **VS16 (U+FE0F) in names:** drop it from segments whose base emoji already has emoji presentation, and add a path lint. Saves 1 unit per emoji segment (Bazel overflow 18,764 to 4,559 in the brief scenario). Check rendering on macOS and Windows before renaming.
5. **Plugin top subtrees (`🗄️stdio`, `📕️norm`, `🏙️bim`):** these hold most of the overflow. Shorten the repeated `🗿️artifacts` (12u) and `🧬️mutations`/`🧬️schema` segments, or split the plugins into a shallower layout.
6. **Ticket tree prefix:** `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/<SLUG>/` costs 44 units plus the slug. Keep slugs short (under 30 units), avoid `staged/...` nesting, and never copy source into tickets (see section 5).
7. **Bazel output base:** `--output_user_root=C:\b` saves 22 units against `C:\Users\abcdefgh\_bazel_x`, but the scenario still has 21,050 overflows. Combine with items 3 and 4.
8. **Root budget and docs:** fix the 210 docstring to 232 in `setup/🟦️.ts`; add a test that fails when any tracked relative path exceeds 232 UTF-16 units (256 minus the 24-unit root), or add a `repo preflight` check on the checkout root length. Document that a root longer than 24 units requires `core.longpaths` and that 3 tracked symlinks require Developer Mode or `SeCreateSymbolicLink` on Windows.

## 8. Symlinks (native Windows)

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/wbs-tools/sb-config/🎨️styling`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/headroom/.nx/installation`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🟦️typescript/.cfb3defcde0`

Each needs `core.symlinks=true` and Developer Mode on Windows; the third one is in the product tree, so it is a real checkout risk.
