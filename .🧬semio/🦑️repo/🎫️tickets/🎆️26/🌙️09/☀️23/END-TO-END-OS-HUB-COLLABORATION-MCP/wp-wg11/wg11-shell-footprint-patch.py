#!/usr/bin/env python3
"""🧵️ WG11 session 14d — set for the first train after the chain: the wgpu shell's own state fits a bounded thread.

Measured (LW1 `wg11-laws-4.txt`: the shell-turn law `a_framework_setting_dispatch_completes_on_a_one_mebibyte_thread` overflows its own
1 MiB thread in debug; WG11 `-Zprint-type-sizes` of the post-T4 renderer, `.🧬semio/🌐hub/s14-wg11-captures/type-sizes-1.txt`, and the
frame table `frames-0642-all.txt`): the shell-turn boxing worked (`dispatch_action` poll 841 KB → 76 KB), but `ShellState` itself is
**400 888 bytes** — `icon_export: Option<IconExportBatch>` 278 008 (the batch inlines four phase owners: `rendering` 102 704, `source`
(`GpuContext`) 100 904, `preparing` 45 280, `scene` 26 984), `world3d_states: AdmittedSurfaceMap<World3dState>` 54 856 (it inlines one
rejected and one retired whole entry, 25 872 + 25 864, beside its heap-first slot table) and `component_world3d_retirement` 25 864. So
`ShellState::new`'s own frame is 803 KB and the law's closure 410 KB: the 1 MiB thread overflows before the dispatch starts.

Fix: the four icon-export phase owners, the map's rejected/retired owners and the shell's World3d retirement owner live in `Box`es
(each exists only while its phase runs) → `ShellState` ≈ 49 KB. The heap-first slot budget pins
(`⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`) take the measured element sizes (World3dState entry 23 536 → 25 872,
EngineSurfaceSlot 76 472 → 80 288 + its owner 16 → 32, grown since the pin) and the map's new owner size (54 856 → 3 136).
Law (new): `a_shell_state_stays_small_enough_to_live_on_a_bounded_thread` — `size_of::<ShellState>()` ≤ 64 KiB; the shell-turn law is the
behavioural witness.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/shell-footprint/` and applies; `--revert`
restores the backups.
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
ICON_EXPORT = ENGINE / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📤️icon-export/🦀️.rs"
SHELL = ENGINE / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
SCENES = ENGINE / "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"
SETTINGS_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs"
BUDGETS = ROOT / "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/shell-footprint"

ICON_EDITS = [
    ("    preparing: Option<IconExportScenePreparation>,\n", "    preparing: Option<Box<IconExportScenePreparation>>,\n"),
    ("    scene: Option<IconExportPreparedScene>,\n", "    scene: Option<Box<IconExportPreparedScene>>,\n"),
    ("    source: Option<GpuContext>,\n", "    source: Option<Box<GpuContext>>,\n"),
    ("    rendering: Option<IconGpuPngExport>,\n", "    rendering: Option<Box<IconGpuPngExport>>,\n"),
    ("Ok(preparation) => { self.preparing = Some(preparation); self.phase = Phase::Prepare; }", "Ok(preparation) => { self.preparing = Some(Box::new(preparation)); self.phase = Phase::Prepare; }"),
    ("                        self.scene = preparation.take_prepared();\n", "                        self.scene = preparation.take_prepared().map(Box::new);\n"),
    ("                    Ok(source) => self.source = Some(source),\n", "                    Ok(source) => self.source = Some(Box::new(source)),\n"),
    ("        match IconGpuPngExport::new(self.source.as_ref().expect(\"accepted device\"), width, height, packet) {\n            Ok(rendering) => { self.rendering = Some(rendering); self.phase = Phase::Render; }",
     "        match IconGpuPngExport::new(self.source.as_deref().expect(\"accepted device\"), width, height, packet) {\n            Ok(rendering) => { self.rendering = Some(Box::new(rendering)); self.phase = Phase::Render; }"),
    ("            if let Ok(source) = answer { self.source = Some(source); }\n", "            if let Ok(source) = answer {\n                self.source = Some(Box::new(source));\n            }\n"),
    ("        let detail = self.rendering.as_ref().map(IconGpuPngExport::progress)\n            .or_else(|| self.preparing.as_ref().map(IconExportScenePreparation::progress))",
     "        let detail = self.rendering.as_deref().map(IconGpuPngExport::progress)\n            .or_else(|| self.preparing.as_deref().map(IconExportScenePreparation::progress))"),
]

SCENES_EDITS = [
    ("    rejected: Option<AdmittedSurfaceRejected<T>>,\n    retired: Option<AdmittedSurfaceCloseOwner<T>>,\n    closing: bool,\n}",
     "    rejected: Option<Box<AdmittedSurfaceRejected<T>>>,\n    retired: Option<Box<AdmittedSurfaceCloseOwner<T>>>,\n    closing: bool,\n}"),
    ("            self.retired = Some(AdmittedSurfaceCloseOwner { id, value: previous });\n", "            self.retired = Some(Box::new(AdmittedSurfaceCloseOwner { id, value: previous }));\n"),
    ("            return Err(AdmittedSurfaceRejected { fault: AdmittedSurfaceFault::RejectedPending, ..rejected });\n        }\n        self.rejected = Some(rejected);\n        Ok(())",
     "            return Err(AdmittedSurfaceRejected { fault: AdmittedSurfaceFault::RejectedPending, ..rejected });\n        }\n        self.rejected = Some(Box::new(rejected));\n        Ok(())"),
    ("        assert!(self.rejected.is_none(), \"surface producer must stop while one exact rejected owner is retained\");\n        self.rejected = Some(rejected);\n",
     "        assert!(self.rejected.is_none(), \"surface producer must stop while one exact rejected owner is retained\");\n        self.rejected = Some(Box::new(rejected));\n"),
    ("        if let Some(retired) = self.retired.take() {\n            return Some(retired);\n        }",
     "        if let Some(retired) = self.retired.take() {\n            return Some(*retired);\n        }"),
]

SHELL_EDITS = [
    ("    component_world3d_retirement: Option<crate::scenes::AdmittedSurfaceCloseOwner<World3dState>>,\n",
     "    component_world3d_retirement: Option<Box<crate::scenes::AdmittedSurfaceCloseOwner<World3dState>>>,\n"),
    ("            self.component_world3d_retirement = Some(owner);\n            return Ok(false);",
     "            self.component_world3d_retirement = Some(Box::new(owner));\n            return Ok(false);"),
]

SIZE_LAW = '''
/// 🧵️ LAW (ticket 26/09/23 session 14d, WG11 — the shell-turn law still overflowed its 1 MiB thread: `ShellState` was 400 888 bytes, so
/// `ShellState::new`'s own debug frame was 803 KB before any dispatch): the shell's state is small enough to live on a bounded thread —
/// every phase owner that exists only while its phase runs (icon export, admitted World3d rejection/retirement) lives in a `Box`.
#[test]
fn a_shell_state_stays_small_enough_to_live_on_a_bounded_thread() {
    let bytes = std::mem::size_of::<ShellState>();
    assert!(bytes <= 64 * 1024, "ShellState is {bytes} bytes; a phase owner moved inline again");
}
'''


def budgets_after(source: str) -> str:
    value = json.loads(source)
    by_owner = {table["owner"]: table for table in value["tables"]}
    changes = {
        "scenes::AdmittedSurfaceMap<World3dState>": {"elementSizeBytes": (23536, 25872), "ownerSizeBytes": (50184, 3136)},
        "engine_canvas::EngineSurfaceRegistry": {"elementSizeBytes": (76472, 80288), "ownerSizeBytes": (16, 32)},
    }
    for owner, fields in changes.items():
        table = by_owner.get(owner) or sys.exit(f"budget table {owner} is gone")
        for field, (old, new) in fields.items():
            if table[field] != old:
                sys.exit(f"{owner}.{field} is {table[field]}, expected the pinned {old}")
            table[field] = new
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


FILES = [ICON_EXPORT, SCENES, SHELL, SETTINGS_LAWS, BUDGETS]


def plans():
    read = {path: path.read_text(encoding="utf-8") for path in FILES}
    if json.dumps(json.loads(read[BUDGETS]), ensure_ascii=False, indent=2) + "\n" != read[BUDGETS]:
        sys.exit("the budget fixture is not in json.dumps(indent=2) form — re-render would reformat it")
    if "fn a_shell_state_stays_small_enough_to_live_on_a_bounded_thread" in read[SETTINGS_LAWS]:
        sys.exit("size law already present — landed already")
    return [
        (ICON_EXPORT, read[ICON_EXPORT], replaced(ICON_EXPORT, read[ICON_EXPORT], ICON_EDITS)),
        (SCENES, read[SCENES], replaced(SCENES, read[SCENES], SCENES_EDITS)),
        (SHELL, read[SHELL], replaced(SHELL, read[SHELL], SHELL_EDITS)),
        (SETTINGS_LAWS, read[SETTINGS_LAWS], read[SETTINGS_LAWS].rstrip("\n") + "\n" + SIZE_LAW),
        (BUDGETS, read[BUDGETS], budgets_after(read[BUDGETS])),
    ]


def main():
    if "--revert" in sys.argv:
        for path in FILES:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(FILES)} files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crate: semio-framework-os-renderer-wgpu; async slot-budget fixture)")


if __name__ == "__main__":
    main()
