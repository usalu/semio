STATUS: FAIL, do not apply. W4 patches and scripts: 2 blocking (A1 new `.unwrap()` in genesis production with a `[DEBUG]` log, A2 hard-coded absolute path), 4 medium (A3 to A6), 3 low items (A7 covers 11 scripts). W3 production edits (not from patches): 1 blocking (B1 new `.unwrap()` in ifc work.rs), 2 medium (B2 `[DEBUG]` log, B3 new limit constants), plus carried panic sites listed below.

Scope and snapshot: taken Nov 3 06:48 CET. Candidate list: C/1102-tools/*/patches/ files (not `_superseded`) modified after 2026-11-02 20:00. That gave 22 files: 6 `.diff`/`.patch`/`.ts` (actinfra, gis), 14 `.py`, 1 `.rs.txt`, 1 `.md` note (`gen/patches/step-context-owner-NOTES.md`, read, no code). Read in full.
W3 scope: 190 files under `🧰️framework/`, `✏️s/`, `🌎️hub/`, `♻️mit-bestand/` modified after 2026-11-02 21:45. 25 of those are `Cargo.lock` (excluded) and 5 more have no content diff vs HEAD. Read-only `git diff HEAD --unified=0` over the other 165 files, 1396 added lines, saved to `/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/audit/w3.diff`. The diff includes uncommitted work from other lanes, so the mtime filter only approximates "W3".

## Part A: W4 patches and apply scripts

| # | Patch / script | Line | Rule | Snippet (<=80 chars) |
|---|---|---|---|---|
| A1 | runtime/patches/apply-debug-genesis-measure.py | 13 | BLOCKING new `.unwrap()` in production (genesis cursor); also the `[DEBUG]` eprintln (lines 7, 13) must be removed after the window | `let c=self.cursor.as_ref().unwrap();eprintln!("[DEBUG] genesis measure` |
| A2 | gen/patches/step-context-owner-remount.py | 2 | BLOCKING zero-touch and cross-platform: absolute machine path | `p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/...` |
| A3 | stale/patches/hydration-job-authority.py | 12 | MEDIUM re-emits a `.expect(` in production (maintenance ladder). It is a split of an existing expect, not new, but the replacement restates it | `let owner=active.hydration.as_mut().expect("recursive document parent...` |
| A4 | testport/patches/durable-group-test-visibility.py | 14, 15, 19, 20 | MEDIUM widens production fields from `pub(super)`/private to `pub` so tests can read them. Adds public surface for test access only; prefer a test-support accessor | `("    pub(super) decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,` |
| A5 | testport/patches/member-open-dictionary-public-types.py | 12, 13 | MEDIUM same as A4: `pub mod index`, `pub struct DictionaryOwners`, `pub struct PendingByte`, `pub struct DictionaryLookup`, `pub struct RetainedDictionaryIndex` | `rep("🦀️.rs",[("#[path = \"📇️index/🦀️.rs\"]\nmod index;","...pub mod index;` |
| A6 | runtime/patches/apply-genesis-pack-batch.py | 24 | MEDIUM new hard-coded production limit `GENESIS_CURSOR_UNITS_PER_TURN: usize = 512` with a bounded loop. It is a new limit, not a raise, and the docstring says so. Needs an explicit OK from the limits owner | `const GENESIS_CURSOR_UNITS_PER_TURN:usize=512;` |
| A7 | 11 scripts, line 2 (energy/editor-test-dev-dependency.py, energy/effect-only-action-producer.py, energy/hydration-binds-job-authority.py, runtime/NOT-APPLIED-puzzle3d-window-config-pack-receiving.py, runtime/apply-debug-genesis-measure.py, runtime/apply-genesis-pack-batch.py, runtime/apply-puzzle3d-window-config-birth.py, stale/hydration-job-authority.py, testport x3) | 2 (hydration-binds: 2) | LOW docstring must start with an emoji (all Python docstrings here start with plain text) | `"""Fail-closed patch (repo root): the energy editor lib-test target` |
| A8 | flowext/patches/flow-examples-dsl-fixture.py | 2 | LOW overwrites `🗣️.dsl.semio` unconditionally (`open(F,"w")`), no existence or anchor check, unlike the other scripts | `open(F,"w").write("""semio flow.flow.dsl v1` |
| A9 | puzzle/patches/window-config-pack-receiving.rs.txt | 1 | LOW a `//` note at the top of the snippet becomes a source comment if pasted verbatim | `// Append to ✏️s/🔌️plugins/🧩️puzzle/...` |
| A10 | runtime/patches/apply-puzzle3d-window-config-birth.py, energy/effect-only-action-producer.py | (see A7) | INFO new English error text in `ValueError::literal(...)` in production, not UI strings, acceptable | `"original Puzzle 3D window config birth is not funded"` |

Checked and clean in W4:
- actinfra/patches/jco-strip-debug.diff: no unwrap, no comments, no limit raise. The temp file `slim` is removed in `finally`. OK.
- actinfra/patches/retention-intermediates-norows.diff: the replaced docstring starts with an emoji. No new allow or limit. OK.
- actinfra/patches/strip-debug/module.ts and test.ts, actinfra/patches/retention-size-law.test.ts: docstrings start with an emoji (`🪶️`). No comments in bodies. Test files only for the test. OK.
- gis/patches/store-replay-retire-unborn-registry-birth.patch: one added line, no violation.
- energy/patches/effect-only-action-producer.py: production edits use `ok_or_else` and `?`. The `.unwrap()` calls are only in the appended `#[test]` law (target `🧪️tests/`), allowed.
- actinfra/patches/prototype-wasm-debug0.py: flips `"debug": 1` to `0` in the build profile schema and fixture. No policy rule, noted for the coordinator as a behaviour change, no docstring.
- No `#[allow(`, `[DEBUG]` in diff patches, compat or deprecation text, raised numeric limits in production (except the new constant in A6), or single-language UI strings in W4.

## Part B: W3 production edits (no patch file)

| # | File (under the tree) | Line in w3.diff | Rule | Snippet (<=80 chars) |
|---|---|---|---|---|
| B1 | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/work.rs | 1404 | BLOCKING new `.unwrap()` (net +1 vs HEAD in this file) | `fn work_demands(...)... self.instance.as_ref().unwrap())` |
| B2 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🏁️host-close/🦀️.rs | 1992 | MEDIUM `[DEBUG]` eprintln in production. Prefix is correct but it must be removed before the ticket closes | `eprintln!("[DEBUG] host close stall phase={:?} grant={:?}"` |
| B3 | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🦀️.rs | 1047, 1053, 185, 196 | MEDIUM new hard-coded limits in production, none present at HEAD in this file: `PREVIEW_FUEL = 64 << 20`, `inputs.len() > 1024`, `max_work: 100_000_000`, `max_pixels: 16_777_216`, `max_edges: 65536` | `const PREVIEW_FUEL: usize = 64 << 20;` |

Carried, not new (counted against HEAD per file, net zero, so existing sites rewritten in place). Track as follow-up, do not count as W3 regressions:
- drawing schema: `unsafe{&*(root as*const DrawingSnapshot)}` and `.expect("checked Drawing owner")` at `🧬️schema/✅️validation/🦀️.rs:956`, `unreachable!()` at `🎬️scene/📷️raster/🦀️.rs:1001`, `.expect`/`unreachable!` at `🔎️lookup/🔗️owned/🦀️.rs:1113,1119`, `.expect` at `🔎️lookup/🦀️.rs:1162`, `.expect("Draw owner retirement")` at `🧬️schema/📸️snapshot/🦀️.rs:1185`, `unreachable!()` at `🎯️paint/🦀️.rs:1250`.
- flow host-close: five `.expect("cold host ... authority")` at `🏁️host-close/🦀️.rs:1992`.
- IFC details: `.unwrap()` sites removed in `field_edit.rs`, `type-guidance.rs`, `machine-domain.rs`, `provider/🦀️.rs` (net negative, good).

Checked and clean in W3:
- Comments inside function bodies: none (no `//` lines in added production code).
- `#[allow(`, `#![allow(`: none added.
- Docstrings: 26 new doc lines, all start with an emoji.
- Single-language strings: every added native/UI label pairs `native_en` with `native_de`; no en-only or de-only string. The `Locale::En`/`Locale::De` hits are all in `🧪️tests/`.
- Raised existing limits: none (numeric net-delta per file shows only the new constants in B3). No timeout or deadline raised in production (the only `setTimeout` is in test files).
- `console.log`, `println!`, `dbg!` in production: none, apart from B2.
- Compat, deprecation, legacy, shims: none.

## Summary by blocking fix

Must fix before apply or before the next patch window:
1. A1: remove the `.unwrap()` (use `ok_or_else(...)?`) and the `[DEBUG]` lines when the window closes.
2. A2: replace the absolute path with a repo-relative one (`semio`-root resolved from the script location).
3. B1: `self.instance.as_ref().unwrap()` in work.rs, replace with `ok_or_else(...)?`.

Medium, decide with the owner: A3 (`.expect` re-emitted), A4 and A5 (visibility widened for tests), A6 (new genesis batch limit), B2 (`[DEBUG]` removal), B3 (new drawing limits, need an explicit justification).

Low: A7 (docstring emoji), A8 (unconditional overwrite), A9 (comment leaks into source).

Per-group counts:
- W4 patches and scripts: 22 files reviewed. 2 blocking (A1, A2), 4 medium (A3 to A6), 3 low items (A7 covers 11 scripts with docstrings lacking an emoji, plus A8, A9).
- W3 production: 165 files with diffs, 1396 added lines. 1 blocking (B1), 2 medium (B2, B3), carried sites listed above.
