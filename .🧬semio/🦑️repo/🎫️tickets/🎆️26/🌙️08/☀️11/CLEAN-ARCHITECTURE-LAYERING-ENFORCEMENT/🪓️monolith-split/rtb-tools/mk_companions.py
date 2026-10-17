"""🏗️ The companion bins whose tests keep the module namespace of a facade: engine-attach-tests (Scenes and EngineCanvas standalone laws) and ui-engine-tests (inline laws module).
usage: mk_companions.py plan|apply [engine-attach|ui-engine]"""
import sys
from build import *
import inventory as INV
import manifest

APPLY = "apply" in sys.argv
ONLY = [a for a in sys.argv[1:] if a not in ("plan", "apply")]
SC_STANDALONE = EL + "/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs"
EG_STANDALONE = EL + "/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs"
ENG = lambda d: EL + "/⚙️EngineCanvas/🧪️tests/" + d + "/🦀️.rs"
SC_PARENT = EL + "/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"
EG_PARENT = EL + "/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"
IP_PARENT = EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"
ENGINE_ATTACH_TESTS = ["🕸️wgpu-node-graph", "🖌️wgpu-paint2d-engine", "🫳️node-graph-gestures", "🩺️metrics-invalidation-drain", "🔬️wgpu-catalogue-workflow-drop"]


def defined_names(path):
    return {i["name"] for i in itm.table(read(path)) if i["kind"] not in ("impl", "use", "mod")}


def mount_lines(folder, rows, indent):
    out = []
    for cfg, f, name in rows:
        rel_ = os.path.relpath(f, folder)
        c = cfg.strip()
        out.append((indent + c + "\n" if c else "") + f'{indent}#[path = "{rel_}"]\n{indent}mod {name};')
    return "\n".join(out)


def module_text(name, suffix, host_parent, files, include_files, tests, folder, extra_use=(), own=None):
    """Inline module with the facade's test namespace: imports, includes, test mounts. files = every file whose tokens count."""
    X.CURRENT[0] = suffix
    set_hosts([host_parent])
    texts = {}
    for f in files:
        texts[f] = FL.rewrite_paths(X.rewrite_heads(read(f)))
    tmpdir = SCRATCH + "/comp-" + suffix
    os.makedirs(tmpdir, exist_ok=True)
    tmp = []
    for i, (f, t) in enumerate(texts.items()):
        pth = tmpdir + f"/{name}-{i}.rs"
        write(pth, t)
        tmp.append(pth)
    aliases, lines, unresolved = full_imports(suffix, tmp)
    imports_ = merge_import_blocks(lines, simplify=True, own=lib_name(suffix))
    return texts, aliases, imports_, unresolved


def engine_attach():
    suffix = "engine-attach-tests"
    crates = crate_table()
    folder = T + "/" + crates[suffix]
    inv = INV.inventory()
    LOCAL_FACADE["scenes"] = defined_names(SC_STANDALONE)
    LOCAL_FACADE["engine_canvas"] = defined_names(EG_STANDALONE)
    tests = [ENG(d) for d in ENGINE_ATTACH_TESTS]
    rows = []
    for f in tests:
        row = inv[f][0]
        rows.append((clean_cfg_(row["cfg"]), f, row["name"]))
    sc_texts, sc_alias, sc_imports, sc_un = module_text("scenes", suffix, SC_PARENT, [SC_STANDALONE], [SC_STANDALONE], [], folder)
    eg_texts, eg_alias, eg_imports, eg_un = module_text("engine_canvas", suffix, EG_PARENT, [EG_STANDALONE] + tests, [EG_STANDALONE], tests, folder)
    print("unresolved:", sc_un, eg_un)
    first = "//! 🧷️ Companion tests of engine-attach: the Scenes and EngineCanvas standalone laws with the node graph, paint2d and gesture laws that share their helpers."
    parts = [first, "//!", "//! 🧭️ Each facade keeps its module name and test namespace here (`scenes`, `engine_canvas`): the standalone law files are included unchanged, the other laws are mounted from their place in the EngineCanvas tree.", "", "#![cfg(test)]", ""]
    aliases = list(dict.fromkeys(sc_alias + eg_alias))
    parts += aliases
    parts += ["", '#[path = "."]', "mod scenes {"] + indent(sc_imports + ["use super::engine_canvas;"]) + ["", f'    include!("{os.path.relpath(SC_STANDALONE, folder)}");', "}", "", '#[path = "."]', "mod engine_canvas {"] + indent(eg_imports) + ["", f'    include!("{os.path.relpath(EG_STANDALONE, folder)}");', ""]
    parts += [mount_lines(folder, rows, "    "), "}", ""]
    root = "\n".join(parts)
    return folder, root, {**sc_texts, **eg_texts}, [SC_STANDALONE, EG_STANDALONE], tests


def indent(blocks, pad="    "):
    return [pad + ln if ln.strip() else ln for blk in blocks for ln in blk.split("\n")]


def clean_cfg_(cfg):
    return "" if cfg.strip() in ("", "#[cfg(test)]") else cfg.strip()


def ui_engine():
    suffix = "ui-engine-tests"
    crates = crate_table()
    folder = T + "/" + crates[suffix]
    inv = INV.inventory()
    tests = {
        "engine_surface_attach_tests": (EG_PARENT, ENG("🧩️wgpu-engine-surfaces")),
        "board2d_engine_tests": (EG_PARENT, ENG("🔬️wgpu-board2d-engine")),
        "worker_cell_tests": (IP_PARENT, EL + "/🗣️Interpreter/🧵️worker-cell/🧪️tests/🦀️.rs"),
        "introspection_tests": (None, EL + "/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs"),
    }
    return suffix, folder, inv, tests


if __name__ == "__main__":
    if not ONLY or "engine-attach" in ONLY:
        folder, root, texts, includes, tests = engine_attach()
        print("engine-attach-tests root lines", len(root.split("\n")))
        if not APPLY:
            print(root[:7000])
        else:
            for f, text in texts.items():
                edit(f, text)
            edit(folder + "/🦀️.rs", root)
            mp = folder + "/📦️packages/🦀️rust/Cargo.toml"
            added = manifest.add_deps(mp, used_idents(root) | set().union(*[used_idents(x) for x in texts.values()]))
            print("deps added:", added)
