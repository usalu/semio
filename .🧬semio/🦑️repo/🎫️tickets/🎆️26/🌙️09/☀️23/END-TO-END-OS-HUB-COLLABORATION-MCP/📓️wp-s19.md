# WP-S19 — Plugin Extensions: Contributions Framework, i18n, Flow Extensions; Norm Fixtures/Decoders

Session 14 slice S19 (2026-09-27 18:2x, Opus 5.5). Continues [S17](📓️wp-s17.md) and [N1](📓️wp-n1.md). Ports 8110–8119 /
6610–6619. Scripts `wp-s19/`, expendable captures `wp-s19/generated/`, binaries `CARGO_TARGET_DIR=wp-s19/target`, durable
logs `.🧬semio/🌐hub/s14-s19-logs/`. Native cargo only through the `native` lane (build-fleet-b); overlay builds only through
the `overlay` lane with a private build-dir inside the overlay.

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | Verify S17's window-2 tool-run snapshot retirement + N1's test-only norm fixes on the current tree | in progress |
| 2 | S17 2b-2 contributions framework (`setContributions` → framework-reserved, non-ledgered job; 8 artifacts, ShellHost, TransientStore) | open |
| 3 | S17 2c flow Catalogue "Extensions" group empty (operator-keyed reachability cut) | open |
| 4 | S17 1b i18n `LocalizedLabel` catalogue text (after T14's H9-L) | open |
| 5 | N1 norm: production-derived fixture vectors, vdi3805/iso16757 decoder drift, norm reds, norm in `s` + hub | open |
| 6 | 26 plugin extensions: load in `s`, artifacts exposed, en + de | open |

### Session 14 Log

- 18:2x started; read AGENTS.md, preambles 14/13/12, `fleet-14-agents.md` (no "CHAIN LAUNCHED" yet), `wp-s17.md`, `wp-n1.md`,
  `audit-s13-window3-inventory.md`, fleet-13 log 14:00–16:0x. Lanes free, 0 rustc, disk 141 GiB, swap 7.6/9.2 GB.
- 18:2x predecessor captures read: S17 `land-tool-run-1-check.txt` (kernel + SDK `--lib --tests`) **EXIT 0 16:22**, wasm32
  kernel + SDK + procedural **EXIT 0 17:03** — both on the 15:2x–16:2x tree. N1 `wp-n1/generated/lane-run-1.txt` (16:01–16:12):
  norm `--lib` **29 passed / 1 FAILED** (`en1992_set_active_example_loads_liquid_retaining_fem_anchor`: "must load the example
  through setSnapshot"), `--test compliance_gate` **FAILED** (10/15 families: "no applicable remedy cleared the Fail"; the test
  prints a `[DEBUG] compliance-gate fleet:` line), emitter build **E0432** (`protocol::value::{FromValue, ToValue}` moved).
  So N1's window-2 fixes compile but the two tests are still red → item 1/5 work.
- 18:36 item-1 chain `wp-s19/s19-item1.sh` (native lane, build-fleet-b; kernel + SDK + gen3d `--lib --tests`, gen3d law, norm
  `--lib` + `compliance_gate`) → `.🧬semio/🌐hub/s14-s19-logs/item1-1-*.txt`. 18:48 check **EXIT 101**: NOT S17's code — a peer's
  in-flight value-derive edit (`🌱️value/✨️derive/⚙️expansion/🦀️.rs` +904 lines, `🔁️codec` +225, untracked `🧪️tests/🧭️typed-path/`,
  files written 18:35–18:49) breaks `semio-framework-3d` (E0282 `🥽️mesh` 231), `semio-framework-actor` (8× E0282 `🚪️lifetime`,
  `📤️return`) and the kernel (5× E0034 "multiple `from_value`" in `🏪️store/🧩️composition/🗄️durable-group`, E0282 store 2892).
  Stopped my queued law/norm steps (they compile the same crates; killed my 43205 + waiter 59083, ticket removed). Re-run when
  the peer's edit compiles.
- 18:4x overlay `.🧬semio/🌐hub/s14-s19-overlay` (APFS clonefile of 78 384 tracked + untracked-unignored files + 18 gitignored
  `🤖️generated` dirs, 5 min; `wp-s19/s19-overlay.py <root>`, re-run = sync changed files); `cargo metadata` resolves.
- 18:5x item 5 census `wp-s19/s19-norm-example-census.py`: the N1 surface red is a PRODUCT defect, not a stale test —
  en1992's editor offers 5 examples in its navbar (`examples()`), `setActiveExample` resolves only 2 (hand-kept match), so
  picking "Liquid-retaining tank" / both prestressed beams is a silent no-op. Same drift: en1994 (2 of 3 missing:
  `composite_floor_beam`, `…_failing`), en1998 (2 of 4: `seismic_multipart`, `…_fail`), iso16757 (`broken`). Root fix =
  every family resolves `setActiveExample` through its own `examples()` roster (en1995 already does) + a law over EVERY roster
  example of all 15 families (the current surface law checks one example per family).
- 19:0x set `norm-examples` written in the overlay (codemod `wp-s19/s19-norm-examples.py <root>`, idempotent; staged by
  `wp-s19/s19-stage.py` = whole-file `.old`/`.new` pairs + three-way-merge apply, dry run default, backups under
  `.🧬semio/🌐hub/s14-s19-backup/`): shared `app_surface::roster_example_snapshot` (empty id → empty doc, id outside the
  roster → no-op, unparsable body → named fault `norm.set-active-example-invalid`, never a fallback); en1992/en1994/en1997/
  en1998/iso16757/din4108 `setActiveExample` resolve through `<Editor as ArtifactEditor>::examples()`; en1997's silent
  `decode(...).unwrap_or(fallback)` is gone; rosters completed (en1997 + `compliant`, `noncompliant`; din4108 +
  `failing-thin-insulation` — mounted example modules the picker hid); law `🔬️surface` `<family>_roster_examples_all_load`
  ×15 (every roster example loads via `setActiveExample`; roster ids == the `pub const ID`s of `📚️examples/*/🦀️.rs`).
  Overlay builds pin the Codex peer's in-flight value-derive files to HEAD inside the overlay only (`s19-overlay-pin.sh`,
  `s19-overlay-protect.txt`). First overlay run died on a gitignored generated source (`🔤️tokens/🦀️.rs`) → overlay now also
  clones ignored `🤖️generated*`/`🦀️.rs`/`🔤️tokens`/`🕸️bindings` entries; a sync clobbered the unstaged edits once → the sync
  now skips staged + protected paths and every edit is a re-runnable codemod.
- 19:1x N1's emitter copied to `wp-s19/emitter/` (import `protocol::{FromValue, ToValue}` — the E0432) and built as an
  overlay workspace member (`.s19-emitter`, shares the overlay build-dir). Overlay proof chain `s19-norm-overlay.sh` pid 52330
  → `s14-s19-logs/norm-ov-2-{test,check,emitter}.txt`.
- Matrix fact (S16, `🧮️program-matrix/🟦️.ts` `normSnapshot`): a norm row stages the FIRST mutation fixture case that has
  `📸️snapshot/➡️after/🔣️.json`; 12/15 rows fail because those snapshots are stale against the schema or absent → the
  production-derived fixture regeneration (N1's `n1-vectors.py` → `n1-materialize.ts`) is what makes norm 15/15 in the matrix.
