"""Creates (or extends) the fixtures crate `semio-framework-os-renderer-wgpu-fixtures` with the Shell test helpers the test files share.
usage: mk_fixtures.py shell [--apply]"""
import json, os, re, sys
from build import *
import manifest

FX = "fixtures"
FX_DIR = T + "/🧰️fixtures"
FX_LIB = PFX + "fixtures"
APPLY = "--apply" in sys.argv

SHELL_SPECS = [
    ("🔬️wgpu-panel-anchor-model", ["host_test_shell", "host_test_apps"]),
    ("🔬️wgpu-command-registry", ["test_app"]),
    ("🪟️wgpu-window-pane-chrome", ["split_pane_shell", "engaged_pane_shell", "split_pane_app"]),
    ("🎬️wgpu-window-actions-search-panes", ["actions_shell"]),
    ("🔬️wgpu-shell-input", ["tree_pointer_record", "paint_component_pointer_documents", "retained_world_sequence_probe", "mouse_pointer", "pointer_interaction"]),
    ("🔬️wgpu-shell-boot-isolation", ["boot_manifest"]),
    ("🌓️appearance-tour-and-footer-pills", ["tour_shell", "tour_introduction"]),
]
TESTS = EL + "/🐚️Shell/🧪️tests/"


def transform_moved(text):
    t = X.rewrite_heads(text)
    t = FL.rewrite_paths(t)
    return t


def main():
    names_all, new_src_text, pieces, file_use_entries = set(), {}, [], []
    set_hosts([])
    dst = FX_DIR + "/🐚️shell/🦀️.rs"
    for d, names in SHELL_SPECS:
        p = TESTS + d + "/🦀️.rs"
        new_src, moved, want = extract(p, names)
        new_src_text[p] = new_src
        text_moved = "\n\n".join(moved)
        file_use_entries += file_uses(p, tokens(text_moved))
        for m in moved:
            pieces.append(rebase_paths(m, p, dst))
        names_all |= set(want)
        print(d, "moves", want)
    body = transform_moved("\n\n".join(pieces))
    body, _ = sibling_refs(body, names_all)
    tmp = SCRATCH + "/fx-shell-body.rs"
    write(tmp, body)
    aliases, use_lines = imports(FX, [tmp])
    fu = render_file_uses(file_use_entries)
    uniq = aliases + merge_import_blocks(fu + use_lines, simplify=True, exclude_locals=set(names_all))
    header = "//! 🐚️ Shell fixtures of the test bins: the shells, apps and documents several laws build their worlds from.\n//!\n//! 🧭️ Moved out of the Shell law files that held them, so that laws in different bins name them through this crate.\n\n"
    module = header + "\n".join(uniq) + "\n\n" + body.rstrip("\n") + "\n"
    print("module lines", len(module.split("\n")), "imports", len(uniq))
    return module, new_src_text, names_all


def apply(module, new_src_text, names_all):
    root = '//! 🧰️ Shared test fixtures of the wgpu renderer test bins: worlds, shells and apps that laws of several bins build from.\n//!\n//! 🧭️ Behind `testkit`: a bin lists this crate with `features = ["testkit"]`; each module holds what one layer of the renderer exports for tests.\n\n#![cfg(any(test, feature = "testkit"))]\n\n#[path = "🐚️shell/🦀️.rs"]\npub mod shell;\n'
    write(FX_DIR + "/🦀️.rs", root)
    write(FX_DIR + "/🐚️shell/🦀️.rs", module)
    idents = used_idents(module)
    mtext, miss = manifest.build_manifest("fixtures", "fixtures", "Shared test fixtures of the wgpu renderer test bins", idents, first_party_extra=[PKG_PREFIX + "ui-engine-tests"])
    mtext = mtext.replace("[dependencies]\n", '[features]\ntestkit = []\n\n[dependencies]\n', 1)
    write(FX_DIR + "/📦️packages/🦀️rust/Cargo.toml", mtext)
    write(FX_DIR + "/📦️packages/🦀️rust/📋️project.json", '{\n  "name": "@semio-tech/os-renderer-wgpu-fixtures-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n')
    print("manifest misses:", miss)
    fxpath = FX_LIB + "::shell::"
    for f in all_test_files():
        t = new_src_text.get(f, read(f))
        t2, used = sibling_refs(t, names_all)
        still = {n for n in names_all if uses_name(t2, n) and not re.search(r"(?m)^\s*(?:pub(?:\([^)]*\))? )?(?:async )?(?:fn|struct|const|static) " + re.escape(n) + r"\b", t2)}
        need = sorted(used | still)
        if need:
            body = need[0] if len(need) == 1 else "{" + ", ".join(need) + "}"
            t2 = add_use(t2, f"use {FX_LIB}::shell::{body};")
        if t2 != read(f):
            edit(f, t2)
            print("edited", os.path.relpath(f, E)[-70:], need)


if __name__ == "__main__":
    module, new_src_text, names_all = main()
    if APPLY:
        apply(module, new_src_text, names_all)
