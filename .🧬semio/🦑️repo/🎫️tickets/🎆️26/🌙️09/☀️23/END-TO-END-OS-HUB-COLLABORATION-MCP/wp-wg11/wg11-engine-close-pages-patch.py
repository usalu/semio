#!/usr/bin/env python3
"""♻️ WG11 session 14d — set for T6 (independent of the other WG11 sets): an engine surface's close ladder retires its buffers a PAGE
per grant, never a scalar per grant (coordinator 09:0x, open design item 3 — "bounded paged close like H14's retire-pages").

Measured (overlay build 4, `text_editor_large_explicit_draft_pages_cancel_and_refusal_without_losing_text`): closing a TextEditor
holding a 65 535-scalar document took > 262 144 close grants. Every retained buffer on the ladder popped ONE scalar per grant — the
editor host's text (`EditorHostRetirement`), the encoded scene pack, the sync cache's scene JSON and the delivery owner's latest and
in-flight snapshots — so a document cost ~6 grants per scalar across the copies. Popping a scalar releases no memory (a `String`
keeps its capacity until it drops), so those grants bought nothing but latency: the surface's id stayed unrecyclable and its host
held the maintenance lane for ~400 000 turns.

1. `EngineCanvas`: `retire_string_page`/`retire_bytes_page` release up to `ENGINE_CLOSE_PAGE_BYTES` (64 KiB, a char boundary for
   text) per grant — the pattern the prepared raster owners already use (`PREPARED_RASTER_PAGE_BYTES`). Every scalar-popping close
   helper on the ladder (`close_string`, `close_bytes`, the delivery owner's `close_string`, the node-graph selection / operator ids,
   the interaction domain's ids, the last note click) goes through them.
2. `✍️editor::EditorHostRetirement`: the text and the caret list retire a page per grant (`EDITOR_RETIREMENT_PAGE_BYTES`); items
   that own strings (tokens, spans, diagnostics …) still retire one per grant.
Laws: the editor retires a 1 MiB document in ≤ 20 grants; the populated graph/map/editor surface still closes one fuel turn at a
time but page by page (was `turns > 24 576`, the per-scalar count — now bounded both ways); the large-draft law's own close
ceiling holds with room.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/engine-close-pages/` and applies;
`--revert` restores.
"""

import difflib
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE_CANVAS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"
STANDALONE_LAWS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs"
EDITOR = ROOT / "🧰️framework/🔨️modules/✍️editor/🦀️.rs"
EDITOR_LAWS = ROOT / "🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/engine-close-pages"

ENGINE_CANVAS_EDITS = [
    (
        '''const ENGINE_SURFACE_ID_BYTE_CAPACITY: usize = 256;
''',
        '''const ENGINE_SURFACE_ID_BYTE_CAPACITY: usize = 256;

/// ♻️ The bytes one close grant releases from a retained buffer — a page, as the prepared raster owners and the store's paged
/// retirement do: a large document costs its size over the page in grants, never a grant per scalar.
const ENGINE_CLOSE_PAGE_BYTES: usize = 64 * 1024;

/// ♻️ Releases up to one page of `text` from its end, cutting on a char boundary; `false` once it is empty.
fn retire_string_page(text: &mut String) -> bool {
    if text.is_empty() {
        return false;
    }
    let mut cut = text.len().saturating_sub(ENGINE_CLOSE_PAGE_BYTES);
    while !text.is_char_boundary(cut) {
        cut += 1;
    }
    text.truncate(cut);
    true
}

/// ♻️ Releases up to one page of `bytes` from its end; `false` once it is empty.
fn retire_bytes_page(bytes: &mut Vec<u8>) -> bool {
    if bytes.is_empty() {
        return false;
    }
    bytes.truncate(bytes.len().saturating_sub(ENGINE_CLOSE_PAGE_BYTES));
    true
}
''',
    ),
    (
        '''    fn close_string(value: &mut Option<String>) -> bool {
        let Some(text) = value.as_mut() else {
            return false;
        };
        if text.pop().is_none() {
            *value = None;
        }
        true
    }

    fn close_bytes(value: &mut Option<Vec<u8>>) -> bool {
        let Some(bytes) = value.as_mut() else {
            return false;
        };
        if bytes.pop().is_none() {
            *value = None;
        }
        true
    }''',
        '''    fn close_string(value: &mut Option<String>) -> bool {
        let Some(text) = value.as_mut() else {
            return false;
        };
        if !retire_string_page(text) {
            *value = None;
        }
        true
    }

    fn close_bytes(value: &mut Option<Vec<u8>>) -> bool {
        let Some(bytes) = value.as_mut() else {
            return false;
        };
        if !retire_bytes_page(bytes) {
            *value = None;
        }
        true
    }''',
    ),
    (
        '''        if domain.id.pop().is_some() || domain.node_target_prefix.pop().is_some() || domain.edge_target_prefix.pop().is_some() || domain.handle_target_prefix.pop().is_some() {''',
        '''        if retire_string_page(&mut domain.id) || retire_string_page(&mut domain.node_target_prefix) || retire_string_page(&mut domain.edge_target_prefix) || retire_string_page(&mut domain.handle_target_prefix) {''',
    ),
    (
        '''            || cache.selection.as_mut().is_some_and(|ids| ids.last_mut().is_some_and(|id| id.pop().is_some()))''',
        '''            || cache.selection.as_mut().is_some_and(|ids| ids.last_mut().is_some_and(retire_string_page))''',
    ),
    (
        '''            || cache.operator_ids.as_mut().is_some_and(|ids| ids.last_mut().is_some_and(|id| id.pop().is_some()))''',
        '''            || cache.operator_ids.as_mut().is_some_and(|ids| ids.last_mut().is_some_and(retire_string_page))''',
    ),
    (
        '''                if self.last_note_click.as_mut().is_some_and(|(id, _)| id.pop().is_some()) {''',
        '''                if self.last_note_click.as_mut().is_some_and(|(id, _)| retire_string_page(id)) {''',
    ),
    (
        '''    fn close_string(value: &mut String) -> bool {
        value.pop().is_some()
    }''',
        '''    fn close_string(value: &mut String) -> bool {
        retire_string_page(value)
    }''',
    ),
]

STANDALONE_EDITS = [
    (
        '''fn populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time() {''',
        '''/// ♻️ The ladder closes one fuel turn at a time AND page by page: its 8 KiB buffers cost a grant each (not 8 192), so the whole
/// populated surface takes more than one turn per owner and far fewer than one per scalar.
fn populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time() {''',
    ),
    (
        '''    let turns = drive_engine_surface_close(&mut registry, token, &mut input);
    assert!(turns > 24_576);''',
        '''    let turns = drive_engine_surface_close(&mut registry, token, &mut input);
    assert!((16..8_192).contains(&turns), "a populated surface closes owner by owner, a page per grant: {turns}");''',
    ),
]

EDITOR_EDITS = [
    (
        '''/// 🧹️ Retained text-editor owner that releases one scalar or collection item per close grant.
pub struct EditorHostRetirement {''',
        '''/// ♻️ The bytes of text (or of a scalar list) one close grant releases — a page, never a scalar.
const EDITOR_RETIREMENT_PAGE_BYTES: usize = 64 * 1024;

/// 🧹️ Retained text-editor owner that releases one page of its text or caret list, or one string-owning item, per close grant.
pub struct EditorHostRetirement {''',
    ),
    (
        '''        if self.text.pop().is_some()
            || self.semantic_tokens.pop().is_some()
            || self.selectable_spans.pop().is_some()
            || self.diagnostics.pop().is_some()
            || self.placeholders.pop().is_some()
            || self.hover_occurrences.pop().is_some()
            || self.selection_occurrences.pop().is_some()
            || self.extra_carets.pop().is_some()
        {''',
        '''        if !self.text.is_empty() {
            let mut cut = self.text.len().saturating_sub(EDITOR_RETIREMENT_PAGE_BYTES);
            while !self.text.is_char_boundary(cut) {
                cut += 1;
            }
            self.text.truncate(cut);
            return false;
        }
        if !self.extra_carets.is_empty() {
            let keep = self.extra_carets.len().saturating_sub(EDITOR_RETIREMENT_PAGE_BYTES / std::mem::size_of::<usize>());
            self.extra_carets.truncate(keep);
            return false;
        }
        if self.semantic_tokens.pop().is_some() || self.selectable_spans.pop().is_some() || self.diagnostics.pop().is_some() || self.placeholders.pop().is_some() || self.hover_occurrences.pop().is_some() || self.selection_occurrences.pop().is_some()
        {''',
    ),
]

EDITOR_LAW_EDITS = [
    (
        '''use super::*;
''',
        '''use super::*;

/// ♻️ LAW (ticket 26/09/23 session 14d, WG11): a retired editor releases its text a page per grant — a 1 MiB document (with a
/// multi-byte scalar straddling every page edge) reaches terminal-empty in about 16 grants (at most 20), never one grant per scalar.
#[test]
fn a_retired_editor_releases_its_text_a_page_per_grant() {
    let mut host = EditorHost::new();
    host.set_text("🙂a".repeat(1024 * 1024 / 5));
    let mut retirement = EditorHostRetirement::new(host);
    let mut grants = 1;
    while !retirement.close_step() {
        grants += 1;
        assert!(grants <= 20, "a 1 MiB document retires page by page");
    }
    assert!(retirement.terminal_is_empty());
}
''',
    )
]

EDITS = {ENGINE_CANVAS: ENGINE_CANVAS_EDITS, STANDALONE_LAWS: STANDALONE_EDITS, EDITOR: EDITOR_EDITS, EDITOR_LAWS: EDITOR_LAW_EDITS}


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def main():
    if "--revert" in sys.argv:
        for path in EDITS:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(EDITS)} files restored from backups")
        return
    write = "--write" in sys.argv
    planned = []
    for path, edits in EDITS.items():
        source = path.read_text(encoding="utf-8")
        planned.append((path, source, replaced(path, source, edits)))
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-os-renderer-wgpu, semio-framework-editor)")


if __name__ == "__main__":
    main()
