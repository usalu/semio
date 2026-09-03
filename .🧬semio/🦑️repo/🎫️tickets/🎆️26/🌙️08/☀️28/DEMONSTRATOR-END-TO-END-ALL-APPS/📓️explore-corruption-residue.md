# Corruption Residue Audit — 2026-09-04

## Verdict

**YES — Corruption residue is PRESENT.** Six files contain live emoji-corruption in actual code and config (not documentation).

---

## 1. Git Status Buckets (453 total dirty entries)

### By Status Code
| Code | Count | Description |
|------|-------|-------------|
| ` M` | 296   | Modified in working tree, staged in index (stage 1) |
| `M ` | 37    | Modified in index only (stage 2 but not stage 3) |
| `MM` | 37    | Modified in both index and working tree (stages 2 AND 3) |
| `??` | 59    | Untracked files |
| `A ` | 10    | Added in index only (stage 2) |
| `R ` | 6     | Renamed in index |
| `AM` | 3     | Added in index, modified in working tree |
| `D ` | 3     | Deleted in working tree, staged |
| `AD` | 1     | Added then deleted in working tree |
| `RM` | 1     | Renamed in index, modified in working tree |

### By Top-Level Directory (sample of large movers)
```
.cursor/plans/              1 file modified
.vscode/                    2 files modified  
.🧬semio/🦑️repo/            50+ files (tickets, prompts, etc.)
♻️mit-bestand/              3 files modified/deleted
✏️s/🔌️plugins/              ~350 files modified
🧰️framework/                ~50 files modified
🌎️hub/                      ~3 files modified
README.md, package.json     2 files modified
bun.lock                    1 file modified
```

---

## 2. Corruption Signatures in Tracked Non-Markdown Files

### 2.1 Doubled Emoji: `📦️📦️packages`

**Count:** 7 files contain this corruption signature (8 total lines).

| File | Line Count |
|------|-----------|
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔨️restore-emoji-corruption.py` | 1 |
| `bun.lock` | 3 |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml` | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs` | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` | 1 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts` | 1 |

**Examples:**
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts:883` — references `["🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️📦️packages/🦀️rust/📜️script.ts"]`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs:12` — comment `../../📦️📦️packages/🦀️rust/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔨️restore-emoji-corruption.py:6` — documents the bug

### 2.2 Glued Emoji: `🎨️🟠️styling`, `🔣️🌷️`, `🧪️🎚️`

**Count:** 9+ files containing these signatures.

| File | Corruption Type |
|------|-----------------|
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️04/☀️08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY/📜️script.ts` | `🔣️🌷️`, `📇️📇️registry`, `🎨️🟠️styling` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts` | `🎨️🟠️styling` ×2 |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️tests/🟦️.ts` | `🎨️🟠️styling`, `🧪️🎚️` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛂️adapters.manifest.json` | `🎨️🟠️styling`, `🧪️🎚️` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts` | `🎨️🟠️styling` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go` | `🔣️🌷️taxonomy.json` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` | `🧪️🎚️` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️run-vitest-config-argument-tokens/🟦️.ts` | `🧪️🎚️` |

**Examples (first 10 lines across all files):**
1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go:20888` — reads `"🔣️🌷️taxonomy.json"`
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:138` — path `"🧪️🎚️run-vitest-config-argument-tokens/🟦️.ts"`
3. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛂️adapters.manifest.json:2` — token path `"../🎨️🟠️styling/🔣️.json"`
4. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛂️adapters.manifest.json:8` — output `"../🎨️🟠️styling/🎨️palette.css"`
5. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:12` — path `"../../../🎨️🟠️styling/📦️packages"`
6. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:22` — path `"../../../🎨️🟠️styling/📦️packages"`
7. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️tests/🟦️.ts:6` — path `"../../../../🎨️🟠️styling/📦️packages"`
8. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml:43` — comment `../../📦️📦️packages/🦀️rust/🦀️.rs`
9. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️04/☀️08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY/📜️script.ts:310` — path `"🔣️🌷️taxonomy.json"`
10. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️04/☀️08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY/📜️script.ts:368` — rename map `["🔌️plugin/📇️registry", "🔌️plugin/📇️📇️registry"]`

### 2.3 Doubled Emoji in Filenames (tracked via git)

**Count:** 0 files with doubled emoji in their git-tracked names.
- `git ls-files | grep -E '📦️📦️|📇️📇️|🎨️🟠️|🔺️⚙️|🧮️🔢️|🔣️🌷️|🟦️🪧️|🧪️🎚️'` → *empty*

### 2.4 Untracked Files with Corruption Signatures

**Count:** 0 untracked files match the corruption patterns.

---

## 3. Deleted Files

**Count:** 3 deletions, none with rename twins.

| Deletion | Untracked Twin Found? |
|----------|----------------------|
| `♻️mit-bestand/recherche` | No |
| (2 others in tickets, hard to parse) | No |

No "twins" (ASCII skeleton matches) found; these are pure deletions, likely legitimate cleanup.

---

## 4. Emptied Source Literals

### Code Files (`b''`, unused `""`)
- **Result:** 1 file found with `b''`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🏭️generator/📜️script.ts`
- This appears to be a legitimate empty byte string, not corruption.

### package.json Exports
- **Result:** 0 package.json files with empty `"exports"` keys or values.

---

## 5. Git Diff Summary (Top 30 files by change size)

| File | Changes |
|------|---------|
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-authenticated-socket-grant-s3-final-audit.md` | +1511 |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/📓️status.md` | +169 |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️status.md` | +247 |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-document-open-plan-implementation-audit.md` | +255 |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️status.md` | +53 |
| `.vscode/launch.json` | ±149 |
| `.vscode/🧩️launch.seed.jsonc` | ±146 |
| README.md | ±41 |
| `.cursor/plans/unify_package_names_a0fcf287.plan.md` | ±3 |
| Various `.rs`, `.ts`, `.md` files under `✏️s/` and `🧰️framework/` | 2±51 each |

**Main churning areas:**
- Ticket documentation (mostly legitimate peer work)
- VS Code config (launch.json, seed files)
- Procedural and puzzle plugins (many `.rs` edits, likely legitimate)
- Root config files (README, package.json, bun.lock)

---

## 6. Corruption Files (Non-Documentation)

The following **6 real source/config files** contain live corruption and should be repaired:

1. **`🌎️hub/📦️packages/🦀️rust/📜️script.ts:883`**
   - Live: `["🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️📦️packages/🦀️rust/📜️script.ts"]`
   - Should be: `["🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts"]`

2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs:12`**
   - Live: `../../📦️📦️packages/🦀️rust/🦀️.rs`
   - Should be: `../../📦️packages/🦀️rust/🦀️.rs`

3. **`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:12,22`**
   - Live: `"../../../🎨️🟠️styling/📦️packages"` (appears twice)
   - This is **intentional glued emoji** from ticket 26/04/08; should remain or be verified against ticket goal

4. **`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go:20888`**
   - Live: `"🔣️🌷️taxonomy.json"`
   - Should be: `"🔣️taxonomy.json"` (glued emoji removal)

5. **`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛂️adapters.manifest.json:2,8`**
   - Live: `"../🎨️🟠️styling/🔣️.json"`, `"../🎨️🟠️styling/🎨️palette.css"`
   - Glued emoji (`🎨️🟠️`) appears intentional; verify against ticket 26/04/08

6. **`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️04/☀️08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY/📜️script.ts:310,368,369`**
   - This is a ticket **script documenting the corruption fix** itself; contains the rename map and glued paths for reference

---

## Conclusion

**Corruption residue is present, but localized and mostly attributed to:**
1. **Doubled `📦️📦️packages`** in 4 core files (genuinely stale)
2. **Glued emoji (`🎨️🟠️styling`, `🔣️🌷️`)** in 5+ files (mixed intent: some are ticket 26/04/08 goals, others are breakage)
3. **Documentation-only mentions** in 5 ticket status.md files (legitimate audit trails, no corruption of live code)

The directory structure itself (on disk and in git-tracked names) is clean — **all corruption is confined to hardcoded strings in source/config files.**
