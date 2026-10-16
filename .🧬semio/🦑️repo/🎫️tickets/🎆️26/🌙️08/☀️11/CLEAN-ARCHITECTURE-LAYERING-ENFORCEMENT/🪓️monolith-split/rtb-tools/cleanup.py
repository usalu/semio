"""🧹️ Removes what only served the moved tests from the monolith: cfg(test) hosts, mounts, includes, source consts and test-host files.
usage: cleanup.py plan|apply"""
import shutil, sys
from lib import *
import cleanup_scan as CS

APPLY = "apply" in sys.argv
KEEP = [
    "renderer_io_retained_host",
    "paint_policy_tests",
    "RENDERER_HEAP_WITNESS",
]
ALL_OF = ["shell parent", "engine_canvas parent", "scenes parent", "interpreter parent", "hub wp3", "hub wp5"]
SELECT = {
    "renderer root": ["Mesh3dItem", "RENDERER_SOURCE"],
    "hub wp1": ["boot_axis_parity_tests", "frame_latency_diagnostics_host", "directory_door_host", "dock_unit_host"],
}
DELETE_DIRS = [
    R + "/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/⏱️frame-latency",
    R + "/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/📇️directory-door",
    R + "/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/🛰️dock-unit",
    R + "/📥️mounts/🧵️wp3-frame-core/🧪️test-hosts/🔎️interpreter-introspection",
    R + "/📥️mounts/🧵️wp3-frame-core/🧪️test-hosts/🧷️interpreter-command-wiring",
]
CHUNK_DIRS = ["⏪️time-travel", "📎️local-folders", "⚓️dock-model-and-measures", "🧾️panel-records", "⚙️settings-and-dock-persistence", "⌨️keyboard-and-palette", "🧱️chrome-paint-and-navbar", "🔑️storage-prefs-and-i18n", "🎨️theme-file-import-and-accessibility"]


def plan_file(name, path):
    text = read(path)
    stm = CS.statements(text)
    lines = text.split("\n")
    drop = []
    for a, b, head in stm:
        if any(k in text.split("\n")[a + 1] or k in head for k in KEEP):
            continue
        if name in SELECT and not any(k in head or k in " ".join(lines[a:b + 1]) for k in SELECT[name]):
            continue
        if name in ALL_OF or name in SELECT:
            drop.append((a, b))
    return text, lines, drop


def main():
    for name in list(ALL_OF) + list(SELECT):
        path = CS.FILES[name]
        text, lines, drop = plan_file(name, path)
        print(f"{name}: drop {len(drop)} statements")
        if APPLY and drop:
            cut = set()
            for a, b in drop:
                cut.update(range(a, b + 1))
            keep = [l for i, l in enumerate(lines) if i not in cut]
            new = re.sub(r"\n{3,}", "\n\n", "\n".join(keep))
            edit(path, new)
    for d in DELETE_DIRS:
        print("delete", os.path.relpath(d, E)[-70:], os.path.exists(d))
        if APPLY and os.path.exists(d):
            shutil.rmtree(d)
    for hub in (R + "/📥️mounts/🌱️wp1-foundation/🧪️test-hosts", R + "/📥️mounts/🧵️wp3-frame-core/🧪️test-hosts"):
        if APPLY and os.path.isdir(hub) and not os.listdir(hub):
            os.rmdir(hub)
    for c in CHUNK_DIRS:
        d = SH + "/" + c
        print("chunk dir", c, sorted(os.listdir(d)) if os.path.isdir(d) else None)
        if APPLY and os.path.isdir(d) and os.listdir(d) == ["🦀️.rs"]:
            shutil.rmtree(d)


if __name__ == "__main__":
    main()
