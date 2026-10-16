"""Closure of helper items inside a test file: the helper plus every top-level item of the same file its body names."""
import sys
from lib import *

def table(path):
    text = read(path)
    tbl = [i for i in itm.table(text) if i["kind"] not in ("impl", "use", "mod")]
    return text, tbl

def closure(path, names):
    text, tbl = table(path)
    lines = text.split("\n")
    by = {}
    for i in tbl:
        by.setdefault(i["name"], []).append(i)
    impls = [i for i in itm.table(text) if i["kind"] == "impl"]
    want, queue = set(), list(names)
    while queue:
        n = queue.pop()
        if n in want or n not in by:
            continue
        want.add(n)
        for it in by[n]:
            body = "\n".join(lines[it["start"] - 1:it["end"]])
            for t in tokens(body):
                if t in by and t not in want and t != n:
                    queue.append(t)
    return text, tbl, want

if __name__ == "__main__":
    specs = {
        "🔬️wgpu-panel-anchor-model": ["host_test_shell"],
        "🔬️wgpu-command-registry": ["test_app"],
        "🪟️wgpu-window-pane-chrome": ["split_pane_shell", "engaged_pane_shell"],
        "🎬️wgpu-window-actions-search-panes": ["actions_shell"],
        "🔬️wgpu-shell-input": ["tree_pointer_record", "paint_component_pointer_documents", "retained_world_sequence_probe", "mouse_pointer", "pointer_interaction"],
        "🔬️wgpu-shell-boot-isolation": ["boot_manifest"],
        "🌓️appearance-tour-and-footer-pills": ["tour_shell", "tour_introduction"],
    }
    for d, names in specs.items():
        p = EL + "/🐚️Shell/🧪️tests/" + d + "/🦀️.rs"
        text, tbl, want = closure(p, names)
        lines = text.split("\n")
        usedby_rest = {}
        # which closure members are also named by items outside the closure
        rest_tokens = set()
        for it in tbl:
            if it["name"] not in want:
                rest_tokens |= tokens("\n".join(lines[it["start"] - 1:it["end"]]))
        shared = sorted(n for n in want if n in rest_tokens and n not in names)
        print(d, "closure:", sorted(want), "| also used by remaining items:", shared)
