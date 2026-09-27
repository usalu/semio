# WP-ST1 — Per-Family Stdio Components (Option A), Prepared Patch For Window 3

Slice ST1, session 13 (2026-09-27 14:58, Opus executor). Coordinator = main. Input: `📓️wp-lb.md` § "Long-term plan: every
stdio subset openable without a monolithic component" (Option A), lb-p3/lb-p4. Patch: `wp-st1/st1-gen.py` (derives the
payload from the live tree) + `wp-st1/st1-apply.py` (`--dry-run` default / `--write`, `--root` for the overlay), payload
`wp-st1/payload/`, plan `wp-st1/plan.json`. Captures `wp-st1/generated/`. Overlay `.🧬semio/🌐hub/s13-st1-overlay/`, private
build dirs `.🧬semio/🌐hub/s13-st1-*`. No tree edits before window 3 (rule 33).

## Session 13

| item | state | evidence |
|------|-------|----------|
| partition | **10 packages, whole artifact kinds each** (table below); 176 apps = 18 base + 158 family | `plan.json`, `generated/st1-gen-1.txt` |
| files | 36 new (9 family crates × 4), 18 edited, 1 dir move (`🧩️composition` → taxonomy-canonical `🏘️composition`) | `generated/st1-dry-1.txt`, `generated/st1-dry.diff` |
| laws | Rust census `🗄️stdio/🧪️tests/🚢️shipped-fleet` (3 laws: per-package bounded declared fleet + deps; every app exactly once + details/edit actions; every kind opened by exactly one package); TS twin `testEditorCatalogContract` (Bun TOML + Ajv, union over packages); play coverage reads nested family descriptors | written, overlay run pending |
| measurements | pending (stdio-semio wasm32, functions + rustc peak RSS) | — |
| dry run on the live tree | **clean** 15:4x (18 edits / 36 new / 1 move, every anchor exactly once) | `generated/st1-dry-1.txt` |
| overlay checks | pending | — |

### Partition (re-derived from the live `🔌️plugin/🦀️.rs` library fleet, 88 subsets)

| package | dir (`🗄️stdio/🧩️extensions/…`) | kinds | subsets | apps |
|---|---|---|---|---|
| `stdio` (base, unchanged fleet) | — | csv, tsv, txt, json, xml, md, html | 9 | 18 |
| `stdio-image` | `🖼️image` | png, jpg, bmp, tiff, gif, svg | 11 | 22 |
| `stdio-media` | `🎵️media` | mp4, mp3, wav, avi | 4 | 8 |
| `stdio-cad` | `🛠️cad` | step, dxf, dwg | 10 | 20 |
| `stdio-bim` | `🏠️bim` | ifc, bcf | 6 | 12 |
| `stdio-mesh` | `🔺️mesh` | gltf, obj, stl, ply, las | 5 | 10 |
| `stdio-pdf` | `📘️pdf` | pdf | 10 | 20 |
| `stdio-office` | `💼️office` | docx, pptx, xlsx | 9 | 18 |
| `stdio-semio` | `🧿️semio` | semio | 19 | 38 |
| `stdio-binary` | `🔢️binary` | binary, deflate, zip, epw | 5 | 10 |

**Why whole kinds (and a ceiling of 40, not 24):** the host resolves "who opens this kind" per artifact kind
(`artifactKindActivationOwner` over `on-artifact-kind:` rows, `claimOwnedArtifactKinds` strips a dependent's rows its
dependency claims). Splitting `s.stdio.semio`'s 19 subsets over two packages would make two unrelated packages claim one
kind and route every semio document to the alphabetically first — whose router lacks the other half's subsets. So a
package ships whole kinds; semio alone is 38 apps; the ceiling becomes 40, to be confirmed by the stdio-semio wasm32
measurement. Consequence for stdio base: it keeps all 36 kind definitions + codecs (+ hub fence, native-codec
factories) but activates only on its 7 kinds; each family activates on its kinds and `.depends_on("stdio", tree_pin!())`.

### Design notes

- Location `🗄️stdio/🧩️extensions/<emoji><family>/` (taxonomy owner pattern `plugin-extension`, member kind `members-of-plan`;
  `role = "plugin"` + `depends-on = ["stdio"]`). Top-level plugin dirs were rejected: the launch-name generator
  (`🚀️launch/🏷️name-prefix`) resolves artifact folders from the TOP plugin dir, so nested families get injective names
  (`🛠️dev📘️stdio-pdf📖️pdf4️⃣1.4🗄️a⚛️react`), top-level ones would collide. Stdio's root already held `🧩️composition`
  (null taxonomy kind; canonical kind is `🏘️composition`), which would duplicate the sibling emoji of `🧩️extensions`
  (path-emoji statute, high) → moved to `🏘️composition` with its 4 referrers.
- Catalog/deployed names `<emoji>stdio-<family>`, first graphemes unique among the 60 catalog rows; emoji admitted
  (`st1-emoji-probe.ts`), no kind collisions (`st1-kind-probe*.ts`: `📑️pdf`→distribution-pdf and `💾️binary`→binary
  were rejected for that reason).
- Playground rows: one per shipped editor (79), variants `stdio-<format>[-<subset>]`, ports react 6219–6300 / wgpu
  6319–6400 skipping every used port and the MCP inspector's 6274/6277 (+6222 used by `.claude/launch.json`). Play
  panes: 9 new groups after "documents", 53 panes carry the curated example the editor's `examples()` publishes (id +
  `label.native.en` resolved through each artifact crate's `examples` module).
- Families link as libraries in stdio's test build with `plugin-root` off (workspace rows `default-features = false`);
  `plugin_exports!` symbols are wasm32-only anyway.
- Not in this patch (follow-ups, owner decision): hub publication of the families (`TRUSTED_BOOTSTRAP_ALL_PACKAGES`,
  open targets of a non-owner package over `s.stdio.*` kinds) — outcome 1 is the local `s` frontend; the base stdio
  descriptor, the 9 family descriptors, `🤖️generated` registry and `.vscode/launch.json` dev launchers regenerate in the
  chain after landing (`describe` / `plugin-registry:generate`); `workspace-contract` physicalFiles (1717) needs the
  families' materialized interface count.

### Log

- 14:58 start. Read AGENTS.md, preamble 13 (rules 1–34), wp-lb Option A + lb-p3/lb-p4.
- 15:10–15:40 exploration: stdio assembly (176-app library enum / 18-app shipped enum), extension precedent, demonstrator
  precedent (foreign-kind surfaces + `.depends_on`), registry discovery/catalog/activation-owner rules, taxonomy
  (members, JCO dist contracts, path-emoji statute), launch generator, play pane laws, hub publisher package list.
- 15:4x `st1-gen.py` + `st1-apply.py` written; generator 10 packages / 176 apps; **dry run on the live tree clean**
  (`generated/st1-dry-1.txt`).
