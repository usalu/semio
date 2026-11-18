# Norm pane audit (read-only)

Scope: 15 `standards` panes, norm composition, artifacts. Read-only, no builds, no git writes.
CAT = `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json`; CARGO = `🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust/Cargo.toml`; DESC = `🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json`.

Result: no mismatches for the 15 norm variants; one live port collision outside norm (see 3).

## 1. Playground rows (CARGO) and app ids
| variant | row | compositionConfigPath (line, file exists) | app (line, in DESC apps) | ports react/wgpu (line) |
|---|---|---|---|---|
| din4108 | 20 | 22, yes | s.norm.din4108@1/*#editor (23, yes) | 6091/6191 (24) |
| din16798 | 26 | 28, yes | s.norm.din16798@1/*#editor (29, yes) | 6092/6192 (30) |
| din18599 | 32 | 34, yes | s.norm.din18599@1/*#editor (35, yes) | 6093/6193 (36) |
| en1990 | 38 | 40, yes | s.norm.en1990@1/*#editor (41, yes) | 6094/6194 (42) |
| en1991 | 44 | 46, yes | s.norm.en1991@1/*#editor (47, yes) | 6095/6195 (48) |
| en1992 | 50 | 52, yes | s.norm.en1992@1/*#editor (53, yes) | 6096/6196 (54) |
| en1993 | 56 | 58, yes | s.norm.en1993@1/*#editor (59, yes) | 6097/6197 (60) |
| en1994 | 62 | 64, yes | s.norm.en1994@1/*#editor (65, yes) | 6098/6198 (66) |
| en1995 | 68 | 70, yes | s.norm.en1995@1/*#editor (71, yes) | 6099/6199 (72) |
| en1996 | 74 | 76, yes | s.norm.en1996@1/*#editor (77, yes) | 6100/6200 (78) |
| en1997 | 80 | 82, yes | s.norm.en1997@1/*#editor (83, yes) | 6101/6201 (84) |
| en1998 | 86 | 88, yes | s.norm.en1998@1/*#editor (89, yes) | 6102/6202 (90) |
| en1999 | 92 | 94, yes | s.norm.en1999@1/*#editor (95, yes) | 6103/6203 (96) |
| iso16757 | 98 | 100, yes | s.norm.iso16757@1/*#editor (101, yes) | 6104/6204 (102) |
| vdi3805 | 104 | 106, yes | s.norm.vdi3805@1/*#editor (107, yes) | 6105/6205 (108) |
compositionConfigPath resolves from the repo root (not the Rust package dir).

## 2. Example id and exampleLabel
CAT: `example` = variant line +1, `exampleLabel` = +2. DESC: `manifest.examples[i]` id line; `label.native.en` in same object.

| variant | pane example (CAT line) | DESC id line | dialect matches app | exampleLabel == en label |
|---|---|---|---|---|
| din4108 | demo (300) | 45712 | yes | yes |
| din16798 | compliant-office (309) | 45752 | yes | yes |
| din18599 | demo (318) | 45832 | yes | yes |
| en1990 | high-consequence-office (327) | 45932 | yes | yes |
| en1991 | de-office-compliant (336) | 46072 | yes | yes |
| en1992 | compliant-office-frame (345) | 46112 | yes | yes |
| en1993 | heb240-compliant (354) | 46212 | yes | yes |
| en1994 | composite-floor-beam (363) | 46252 | yes | yes |
| en1995 | glulam-floor-beam (372) | 46312 | yes | yes |
| en1996 | loadbearing-wall (381) | 46392 | yes | yes |
| en1997 | demo (390) | 46432 | yes | yes |
| en1998 | seismic-rc-frame (399) | 46492 | yes | yes |
| en1999 | aluminium-roof-purlin (408) | 46572 | yes | yes |
| iso16757 | demo (417) | 46612 | yes | yes |
| vdi3805 | demo (426) | 46652 | yes | yes |
Artifact check: each `🏅️standards/.../📚️examples/` holds a dir for the pane example (all 15 pass).

## 3. Ports
- Norm ports are unique across the live repo: react 6091-6105 and wgpu 6191-6205, no collisions with other live rows (158 live rows scanned).
- Live collision outside norm (react 6276 / wgpu 6376):
  - `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🧩️component/📦️packages/🦀️rust/Cargo.toml:83` (stdio-idml, port row 83)
  - `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🧿️semio/🧩️component/📦️packages/🦀️rust/Cargo.toml:29` (stdio-semio-brep)
- Ticket copies `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/wp-st{1,2}/payload/` replicate 79 playground rows each (316 rows total with them). Copies of checked-in Cargo.toml files inside a ticket violate CLAUDE.md; remove or exclude them.

## 4. Side findings (not norm mismatches)
- vdi3805 artifact has example dirs `❌️failing` and `❌️failing-blatt-4-to-19` with no DESC entries.
- en1991 `🧬subjects` and `🎬️demo-session` dirs are not examples; no action.
