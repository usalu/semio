#!/usr/bin/env python3
"""🎟️ LB2 prepared patch p1 — the stdio details panel honours the process-wide `UiValue` arena instead of faulting.

Measured cause (S16 program matrix, stdio/csv redo, en): `ui.snapshot-details.arguments: snapshot details UI admission
failed` = `UiMapBuilder::try_new()` refused by the ONE-page `UiValue` arena (128 rows × 5 collections) while previous
generations were still being retired. The SDK's tree windowing already ends a window early on the framework's capacity
refusal (`ui.fixed-capacity`), but (1) it never read `ui_value_headroom()` before starting a row, and (2) the details
panel reported arena refusals under its own codes, so the refusal escaped as a fatal assembly error.

Hunks: SDK `tree_window_indexed_rows` starts a row only while the arena can price one (`ui_value_headroom().rows() > 0`)
and counts the rows it leaves unbuilt for want of arena credit (`take_arena_unbuilt_rows`, re-exported); the reactor turn
notes each surface whose render stopped short and renders it once more as soon as the arena can price the missing rows
(the host's window request depends on geometry only, so it would never ask again);
details `ui_args`/`pointer_argument` report arena refusals as `ui.fixed-capacity`; law `🧪️tests/🎟️details-arena-headroom`
+ fixture + schema; contract `[[test]] details_arena_headroom`.

Usage: lb2-p1-arena-budget.py [--dry-run | --write] [--root <repo-or-overlay root>]  (default --dry-run on the live tree)"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
PAYLOAD = Path(__file__).resolve().parent / "payload" / "p1"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
DETAILS = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs"
MANIFEST = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml"
LAW = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/🎟️details-arena-headroom/🦀️.rs"
FIXTURE = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧫️fixtures/🎟️details-arena-headroom/🔣️.json"
SCHEMA = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧬️schema/🎟️details-arena-headroom/🔣️.json"

SDK_DOC_OLD = """    /// and items it and the unbuilt rest held; any other error propagates. Rows are built with the
"""
SDK_DOC_NEW = """    /// and items it and the unbuilt rest held; any other error propagates. A row the process-wide `UiValue`
    /// arena can no longer price is not started at all: [`semio_framework_ui_contract::ui_value_headroom`] must
    /// still admit one row of [`semio_framework_ui_contract::UI_VALUE_ROW_COLLECTIONS`] collections, because the
    /// arena is ONE page shared by every panel and by the previous generations the reactor is still retiring, and
    /// a window that stops there stamps its full extent like any other short run. Rows are built with the
"""
SDK_CODE_OLD = """            windows.path.borrow_mut().push(id.to_owned());
            let materialised = row(index);
"""
SDK_CODE_NEW = """            if semio_framework_ui_contract::ui_value_headroom().rows() == 0 {
                windows.refund(slice.len - built);
                ARENA_UNBUILT_ROWS.with(|rows| rows.set(rows.get() + slice.len - built));
                break;
            }
            windows.path.borrow_mut().push(id.to_owned());
            let materialised = row(index);
"""
ARENA_FN_ANCHOR = """fn error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "snapshot details UI admission failed")
}
"""
ARENA_FN = ARENA_FN_ANCHOR + """
/// 🎟️ A refusal of the process-wide `UiValue` arena is the framework's capacity refusal, so the tree window
/// building this row ends there with a shorter run instead of faulting the whole render — see
/// `semio_framework_plugin::tree_window_indexed_section` and law `🧪️tests/🎟️details-arena-headroom`.
fn arena(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("snapshot details {stage} admission failed: the UiValue arena has no free credit"))
}
"""
SITES = [
    ('UiMapBuilder::try_new().ok_or_else(|| error("ui.snapshot-details.arguments"))', 'UiMapBuilder::try_new().ok_or_else(|| arena("argument map"))'),
    ('.map_err(|_| error("ui.snapshot-details.argument"))', '.map_err(|_| arena("argument entry"))'),
    ('UiListBuilder::try_new().ok_or_else(|| error("ui.snapshot-details.path-chunks"))', 'UiListBuilder::try_new().ok_or_else(|| arena("path chunk list"))'),
    ('.map_err(|_| error("ui.snapshot-details.path-chunks-capacity"))', '.map_err(|_| arena("path chunk"))'),
]
TEST_ANCHOR = """[dev-dependencies]
"""
TEST_ROWS = """[[test]]
name = "details_arena_headroom"
path = "../../🧪️tests/🎟️details-arena-headroom/🦀️.rs"

"""

TURN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs"
SDK_REFUSAL_OLD = """                Err(error) if error.code == "ui.fixed-capacity" => {
                    unbuilt();
                    break;
                }
"""
SDK_REFUSAL_NEW = """                Err(error) if error.code == "ui.fixed-capacity" => {
                    unbuilt();
                    if semio_framework_ui_contract::ui_value_headroom().rows() == 0 {
                        ARENA_UNBUILT_ROWS.with(|rows| rows.set(rows.get() + slice.len - built));
                    }
                    break;
                }
"""
SDK_COUNTER_OLD = """    /// 🪟️ Pushes a slice's rows straight into a container builder's children. A row refused with
"""
SDK_COUNTER_NEW = """    thread_local! {
        /// 🎟️ Rows the windows of the render in progress left unbuilt because the process-wide `UiValue` arena had no
        /// credit for them — see [`take_arena_unbuilt_rows`].
        static ARENA_UNBUILT_ROWS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    /// 🎟️ Takes (and resets) the rows the renders since the last call left unbuilt for want of `UiValue` arena credit.
    /// The reactor reads it after every surface render and renders that surface once more when the arena can price
    /// them again; a law reads it to prove a short window was cut by the arena, not by the host's request.
    pub fn take_arena_unbuilt_rows() -> usize {
        ARENA_UNBUILT_ROWS.with(|rows| rows.replace(0))
    }

    /// 🪟️ Pushes a slice's rows straight into a container builder's children. A row refused with
"""
SDK_EXPORT_OLD = """pub use app::{tree_window_indexed_item, tree_window_indexed_section,"""
SDK_EXPORT_NEW = """pub use app::{take_arena_unbuilt_rows, tree_window_indexed_item, tree_window_indexed_section,"""
TURN_STARVED_OLD = """    Ok(taken)
}

impl DirtyPollOwners {
"""
TURN_STARVED_NEW = """    Ok(taken)
}

/// 🎟️ Surfaces whose last render stopped short for want of `UiValue` arena credit, each with the rows it left unbuilt
/// ([`crate::app::take_arena_unbuilt_rows`]). The arena is ONE page shared by every panel and by the generations the
/// close ladder is still retiring, so a quick edit → undo → redo renders against whatever is left; its windows then
/// stamp their full extent with a shorter run, and the host — whose window request depends on geometry only — never
/// asks again. [`redirty_arena_starved_surfaces`] renders such a surface once more as soon as the arena can price the
/// rows it is missing, so the rows arrive with the returned credit and no surface re-renders while it cannot gain.
struct ArenaStarvedSurfaces {
    slots: [Option<(u32, ui_contract::SurfaceId, usize)>; DIRTY_RENDER_CAPACITY],
}

impl ArenaStarvedSurfaces {
    /// 🎟️ Records `surface`'s latest render: short by `unbuilt` rows, or complete (`0`) — which forgets it.
    fn note(&mut self, instance: u32, surface: &str, unbuilt: usize) {
        let Ok(surface) = ui_contract::SurfaceId::try_from(surface) else { return };
        for slot in &mut self.slots {
            if slot.as_ref().is_some_and(|(owner, starved, _)| *owner == instance && *starved == surface) {
                *slot = None;
            }
        }
        if unbuilt == 0 {
            return;
        }
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some((instance, surface, unbuilt));
        }
    }
}

thread_local! {
    /// 🎟️ This actor's [`ArenaStarvedSurfaces`].
    static ARENA_STARVED: RefCell<ArenaStarvedSurfaces> = RefCell::new(ArenaStarvedSurfaces { slots: [const { None }; DIRTY_RENDER_CAPACITY] });
}

/// 🎟️ Re-dirties every arena-starved surface whose missing rows the `UiValue` arena can price again, and answers how many.
fn redirty_arena_starved_surfaces(dirty: &mut DirtyPollOwners) -> Result<usize, semio_framework::Fault> {
    let rows = ui_contract::ui_value_headroom().rows();
    ARENA_STARVED.with(|starved| {
        let mut taken = 0usize;
        for slot in starved.borrow_mut().slots.iter_mut() {
            let Some((instance, surface, _)) = slot.take_if(|(_, _, unbuilt)| *unbuilt <= rows) else { continue };
            dirty.try_surface(instance, surface).map_err(|_| reactor_close_fault("fixed dirty surface authority saturated"))?;
            taken += 1;
        }
        Ok(taken)
    })
}

impl DirtyPollOwners {
"""
TURN_REDIRTY_OLD = """    PATCHES.with(|patches| redirty_acknowledged_deferred_surfaces(patches, &mut dirty))?;
"""
TURN_REDIRTY_NEW = """    PATCHES.with(|patches| redirty_acknowledged_deferred_surfaces(patches, &mut dirty))?;
    redirty_arena_starved_surfaces(&mut dirty)?;
"""
TURN_TAKE_OLD = """        let surface_key = surface.as_ref().to_owned();
        let mounted = match native_close_key(runtime, instance) {
"""
TURN_TAKE_NEW = """        let surface_key = surface.as_ref().to_owned();
        let _ = crate::app::take_arena_unbuilt_rows();
        let mounted = match native_close_key(runtime, instance) {
"""
TURN_NOTE_OLD = """                        continue;
                    }
                    for update in presence {
"""
TURN_NOTE_NEW = """                        continue;
                    }
                    let unbuilt = crate::app::take_arena_unbuilt_rows();
                    ARENA_STARVED.with(|starved| starved.borrow_mut().note(instance, surface_key.as_str(), unbuilt));
                    for update in presence {
"""

problems, changed = [], {}


def text(rel):
    return changed.get(rel) if rel in changed else (ROOT / rel).read_text(encoding="utf-8")


def replace_once(rel, old, new, done_marker):
    current = text(rel)
    if done_marker in current:
        print(f"already applied: {rel} ({done_marker[:60]!r})")
        return
    count = current.count(old)
    if count != 1:
        problems.append(f"{rel}: expected 1× {old[:70]!r}, found {count}")
        return
    changed[rel] = current.replace(old, new, 1)


replace_once(SDK, SDK_DOC_OLD, SDK_DOC_NEW, "arena can no longer price is not started at all")
replace_once(SDK, SDK_CODE_OLD, SDK_CODE_NEW, "ui_value_headroom().rows() == 0 {\n                windows.refund(slice.len - built);")
replace_once(SDK, SDK_REFUSAL_OLD, SDK_REFUSAL_NEW, "ARENA_UNBUILT_ROWS.with(|rows| rows.set(rows.get() + slice.len - built));\n                    }\n                    break;")
replace_once(SDK, SDK_COUNTER_OLD, SDK_COUNTER_NEW, "static ARENA_UNBUILT_ROWS")
replace_once(SDK, SDK_EXPORT_OLD, SDK_EXPORT_NEW, "take_arena_unbuilt_rows, tree_window_indexed_item")
replace_once(TURN, TURN_STARVED_OLD, TURN_STARVED_NEW, "struct ArenaStarvedSurfaces")
replace_once(TURN, TURN_REDIRTY_OLD, TURN_REDIRTY_NEW, "redirty_arena_starved_surfaces(&mut dirty)?;")
replace_once(TURN, TURN_TAKE_OLD, TURN_TAKE_NEW, "let _ = crate::app::take_arena_unbuilt_rows();")
replace_once(TURN, TURN_NOTE_OLD, TURN_NOTE_NEW, "starved.borrow_mut().note(instance, surface_key.as_str(), unbuilt)")
replace_once(DETAILS, ARENA_FN_ANCHOR, ARENA_FN, "fn arena(stage: &'static str) -> PluginAssemblyError {")
for old, new in SITES:
    replace_once(DETAILS, old, new, new)
manifest = text(MANIFEST)
if 'name = "details_arena_headroom"' in manifest:
    print(f"already applied: {MANIFEST}")
elif manifest.count(TEST_ANCHOR) != 1:
    problems.append(f"{MANIFEST}: expected 1× [dev-dependencies]")
else:
    changed[MANIFEST] = manifest.replace(TEST_ANCHOR, TEST_ROWS + TEST_ANCHOR, 1)
for rel, source in [(LAW, "law.rs"), (FIXTURE, "fixture.json"), (SCHEMA, "schema.json")]:
    payload = (PAYLOAD / source).read_text(encoding="utf-8")
    target = ROOT / rel
    if target.exists():
        if target.read_text(encoding="utf-8") == payload:
            print(f"already applied: {rel}")
        else:
            problems.append(f"{rel} exists with different content")
        continue
    changed[rel] = payload
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={len(changed)} problems={len(problems)} write={WRITE}")
for rel in changed:
    print("  ", rel)
if WRITE and not problems:
    for rel, content in changed.items():
        target = ROOT / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)
