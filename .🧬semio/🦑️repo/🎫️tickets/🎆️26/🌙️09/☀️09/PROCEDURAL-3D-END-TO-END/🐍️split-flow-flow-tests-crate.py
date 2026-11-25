"""✂️ Splits the production code out of `semio-s-artifact-flow-flow-tests` into three production crates (io, examples-demo, editor-surface). NOT APPLIED. Default is a dry run that prints the plan; `--apply` writes. Usage: python3 -I this.py [--apply]
Design and file lists: 📓️flow-test-crate-leak-2026-11-24.md."""
import os, re, subprocess, sys

ROOT = subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()
APPLY = "--apply" in sys.argv
W = f"{ROOT}/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
A = f"{W}/🏅️standards/🔖️1/🪆️subsets/✳️any"
TESTS = f"{A}/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust"
PKG = "📦️packages/🦀️rust"
SCHEMA = "semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema"
SRC = {
    "text": f"{A}/🚪️io/📝️text/🦀️.rs",
    "imp_txt": f"{A}/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs",
    "imp_json": f"{A}/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs",
    "exp_txt": f"{A}/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs",
    "exp_json": f"{A}/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs",
    "demo": f"{A}/📚️examples/🎬️demo/🦀️.rs",
    "transient": f"{A}/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🦀️.rs",
    "generations": f"{A}/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs",
    "preview": f"{A}/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs",
    "inspection": f"{A}/✏️editor/📌️panels/🔍️inspection/🦀️.rs",
}
INFER = "semio_s_artifact_flow_flow_inferences::editor::flow"
CRATES = {
    "io": dict(name="semio-s-artifact-flow-flow-io", dir=f"{A}/🚪️io/{PKG}", desc="flow io: text, import and export leaves"),
    "examples": dict(name="semio-s-artifact-flow-flow-examples-demo", dir=f"{A}/📚️examples/🎬️demo/{PKG}", desc="flow examples: demo"),
    "surface": dict(name="semio-s-artifact-flow-flow-editor-surface", dir=f"{A}/✏️editor/🖼️surface/{PKG}", desc="flow editor surface: transient, generations, preview windows and inspection panel"),
}


def ident(n):
    return n.replace("-", "_")


def inline(tree, depth=0, mirror_dir=""):
    """Emits nested `#[path = "."]` inline modules; a str leaf is a raw statement, a tuple (kind, key) a file module."""
    out, pad = [], "    " * depth
    for name, node in tree.items():
        if isinstance(node, str):
            out.append(f"{pad}{node}")
        elif isinstance(node, tuple):
            rel = os.path.relpath(SRC[node[1]], mirror_dir)
            out += [f'{pad}#[path = "{rel}"]', f"{pad}pub mod {name};"] if node[0] == "mod" else [f'{pad}#[path = "{rel}"]', f"{pad}mod component;", f"{pad}pub use component::*;"]
        else:
            out += [f'{pad}#[path = "."]', f"{pad}pub mod {name} {{"] + inline(node, depth + 1, mirror_dir) + [f"{pad}}}"]
    return out


def leaf(path_names, key):
    t = (("comp", key))
    for n in reversed(path_names):
        t = {n: t}
    return t


def merge(a, b):
    for k, v in b.items():
        a[k] = merge(a[k], v) if k in a and isinstance(v, dict) else v
    return a


def tree_io(md):
    io = {
        "text": ("mod", "text"),
        "import": {"deserializers": {"artifacts": {"txt": {"v_utf_8": {"any": ("comp", "imp_txt")}}, "json": {"v_rfc8259": {"any": ("comp", "imp_json")}}}}},
        "export": {"serializers": {"artifacts": {"txt": {"v_utf_8": {"any": ("comp", "exp_txt")}}, "json": {"v_rfc8259": {"any": ("comp", "exp_json")}}}}},
    }
    return {"standards": {"v1": {"subsets": {"any": {"schema": {"_": f"pub use {SCHEMA}::*;", "_s": f"pub use {SCHEMA}::snapshot;"}, "io": io}}}}}


def tree_examples():
    return {"examples": {"demo": ("comp", "demo")}}


def tree_surface():
    return {"editor": {"flow": {
        "_v": f"pub use {INFER}::editor_vocabulary;",
        "_t": f"pub use {INFER}::terminology;",
        "modes": {"edit": {"windows": {"main": {"transient": ("mod", "transient")}}}, "generate": {"windows": {"generations": ("mod", "generations"), "preview": ("mod", "preview")}}},
        "panels": {"inspection": ("mod", "inspection")},
    }}}


ROOT_HEAD = """extern crate semio_framework_schema as framework_schema;
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
use semio_framework_artifact_playbook_playbook as playbook;

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
"""
ROOT_EXTRA = {
    "io": "\npub mod snapshot {\n    pub mod schema {\n        pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::snapshot::*;\n    }\n}\n",
    "examples": "\npub use semio_s_artifact_flow_flow_schema::{flow_genesis_content_pack};\n",
    "surface": "",
}


def rewrite_deps(from_dir, to_dir):
    """Copies the `[dependencies]` of the tests crate with every path re-relativised to the new crate directory."""
    text = open(f"{TESTS}/Cargo.toml", encoding="utf8").read()
    sec = text.split("[dependencies]\n", 1)[1].split("\n[", 1)[0]
    def fix(m):
        absolute = os.path.normpath(os.path.join(from_dir, m.group(1)))
        return f'path = "{os.path.relpath(absolute, to_dir)}"'
    return re.sub(r'path = "([^"]+)"', fix, sec).strip("\n")


def manifest(c):
    ws = os.path.relpath(W, c["dir"])
    return f'''[package]
workspace = "{ws}"
name = "{c["name"]}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "{c["desc"]}"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
{rewrite_deps(TESTS, c["dir"])}
'''


def write(path, content):
    print(("write " if APPLY else "plan  ") + os.path.relpath(path, ROOT))
    if APPLY:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf8").write(content)


def new_crates():
    trees = {"io": tree_io(0), "examples": tree_examples(), "surface": tree_surface()}
    for key, c in CRATES.items():
        md = f"{c['dir']}/🪞️mirror"
        mirror = "//! 🪞️ The mirrored module nest of this crate.\n\n" + "\n".join(inline(trees[key], 0, md)) + "\n"
        mirror = re.sub(r'#\[path = "\."\]\n(\s*)pub mod _\w*\s*\{[^}]*\}', "", mirror)
        write(f"{c['dir']}/Cargo.toml", manifest(c))
        write(f"{c['dir']}/🦀️.rs", f"//! ✂️ {c['desc']} — production code carved out of `semio-s-artifact-flow-flow-tests`.\n\n" + ROOT_HEAD + ROOT_EXTRA[key])
        write(f"{md}/🦀️.rs", mirror)


def edit(path, fn):
    old = open(path, encoding="utf8").read()
    new = fn(old)
    if old == new:
        print("nochg " + os.path.relpath(path, ROOT)); return
    print(("edit  " if APPLY else "plan  ") + os.path.relpath(path, ROOT))
    if APPLY:
        open(path, "w", encoding="utf8").write(new)


REPOINT = [
    (r"semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::", "semio_s_artifact_flow_flow_io::standards::v1::subsets::any::io::"),
    (r"semio_s_artifact_flow_flow_tests::snapshot\b", "semio_s_artifact_flow_flow_io::snapshot"),
    (r"semio_s_artifact_flow_flow_tests::examples\b", "semio_s_artifact_flow_flow_examples_demo::examples"),
    (r"semio_s_artifact_flow_flow_tests::editor::flow::(modes|panels)", r"semio_s_artifact_flow_flow_editor_surface::editor::flow::\1"),
]
CRATE_BY_PATTERN = {"semio_s_artifact_flow_flow_io": "io", "semio_s_artifact_flow_flow_examples_demo": "examples", "semio_s_artifact_flow_flow_editor_surface": "surface"}
PRODUCTION_CONSUMERS = [
    f"{W}/🦀️.rs", f"{A}/🚪️io/🦀️.rs", f"{A}/🧬️schema/🧬️mutations/{PKG}/🦀️.rs", f"{A}/👁️viewer/{PKG}/🦀️.rs", f"{A}/✏️editor/{PKG}/🦀️.rs",
    f"{A}/✏️editor/🎮️commands/🔗️connect-media-ports/{PKG}/🪞️mirror/🦀️.rs", f"{A}/✏️editor/🎮️commands/♻️retirement/🧪️tests/{PKG}/🪞️mirror/🦀️.rs",
]
TEST_MIRRORS = [f"{A}/✏️editor/🧪️tests/🔬️unit/{PKG}/🪞️mirror/🦀️.rs", f"{A}/✏️editor/🧪️tests/🔬️unit-2/{PKG}/🪞️mirror/🦀️.rs"]
MANIFESTS = [  # manifest dir -> production (new crates replace the tests dependency) or test-only (keeps it)
    (f"{W}/{PKG}", "prod"), (f"{A}/🧬️schema/🧬️mutations/{PKG}", "prod"), (f"{A}/👁️viewer/{PKG}", "prod"), (f"{A}/✏️editor/{PKG}", "prod"),
    (f"{A}/✏️editor/🎮️commands/🔗️connect-media-ports/{PKG}", "prod"), (f"{A}/✏️editor/📤️child-publication/{PKG}", "prod"), (f"{A}/✏️editor/🧵️retained/{PKG}", "prod"),
]
TEST_ONLY = [f"{A}/✏️editor/🎮️commands/♻️retirement/🧪️tests/{PKG}", f"{A}/✏️editor/🧪️tests/🔬️unit/{PKG}", f"{A}/✏️editor/🧪️tests/🔬️unit-2/{PKG}"]


def repoint(src):
    for a, b in REPOINT:
        src = re.sub(a, b, src)
    return src


def dep_line(key, d):
    c = CRATES[key]
    return f'{c["name"]} = {{ path = "{os.path.relpath(c["dir"], d)}" }}'


def used_keys(path):
    s = open(path, encoding="utf8").read()
    return {CRATE_BY_PATTERN[k] for k in CRATE_BY_PATTERN if k in s}


def remove_block(src, header):
    """Removes the inline module starting at `header` (balanced braces) together with its preceding `#[path = "."]`."""
    m = re.search(r'(?:[ \t]*#\[path = "\."\]\n)?[ \t]*' + header + r'\s*\{', src)
    if not m:
        return src
    i, depth = m.end(), 1
    while depth:
        depth += {"{": 1, "}": -1}.get(src[i], 0); i += 1
    return src[:m.start()] + src[i:]


def tests_mirror(src):
    """Replaces the production file modules of the tests crate mirror by re-exports of the new crates (test sources keep resolving `crate::…`)."""
    src = re.sub(r'[ \t]*#\[path = "[^"]*🚪️io/📝️text/🦀️\.rs"\]\n[ \t]*pub mod text;', f"{' ' * 20}pub use semio_s_artifact_flow_flow_io::standards::v1::subsets::any::io::text;", src)
    src = remove_block(src, r"pub mod import")
    src = remove_block(src, r"pub mod export")
    src = src.replace("pub use semio_s_artifact_flow_flow_io::standards::v1::subsets::any::io::text;", "pub use semio_s_artifact_flow_flow_io::standards::v1::subsets::any::io::{text, import, export};", 1)
    src = remove_block(src, r"pub mod examples")
    src = remove_block(src, r"pub mod modes")
    src = remove_block(src, r"pub mod panels")
    src += "\npub use semio_s_artifact_flow_flow_examples_demo::examples;\npub mod editor_surface_reexport {\n    pub use semio_s_artifact_flow_flow_editor_surface::editor::flow::{modes, panels};\n}\n"
    return src


def manifests():
    for d, kind in MANIFESTS:
        f = f"{d}/Cargo.toml"
        def fn(s, d=d, kind=kind):
            keys = set()
            for p in PRODUCTION_CONSUMERS + [f"{d}/🦀️.rs", f"{d}/🪞️mirror/🦀️.rs"]:
                if os.path.exists(p) and p.startswith(d):
                    keys |= used_keys_after(p)
            lines = s.split("\n")
            tests_dep = [l for l in lines if l.startswith("semio-s-artifact-flow-flow-tests =")]
            lines = [l for l in lines if not l.startswith("semio-s-artifact-flow-flow-tests =")] if kind == "prod" else lines
            i = lines.index("[dependencies]") + 1
            lines[i:i] = [dep_line(k, d) for k in sorted(keys)]
            return "\n".join(lines)
        edit(f, fn)
    f = f"{TESTS}/Cargo.toml"
    edit(f, lambda s: s.replace("[dependencies]\n", "[dependencies]\n" + "\n".join(dep_line(k, TESTS) for k in sorted(CRATES)) + "\n", 1))


def used_keys_after(p):
    s = repoint(open(p, encoding="utf8").read())
    return {CRATE_BY_PATTERN[k] for k in CRATE_BY_PATTERN if k in s}


def locks_and_owners():
    print("lock  bridge/Cargo.lock, artifact Cargo.lock, 🌎️hub/Cargo.lock: add 3 [[package]] blocks (dependencies = copy of tests block), add them to the packages above that now depend on them,")
    print("      remove semio-s-artifact-flow-flow-tests from `dependencies` of the production packages that dropped it; run `repo cargo ... -- metadata --offline` unlocked once if unsure")
    print("owner 📋️build-owners.json: three new owners next to semio-s-artifact-flow-flow-tests (packages: [])")
    print("script 📦️packages/🦀️rust/📜️script.ts line ~196: add the three crate names to `packages`")
    print("members workspace Cargo.toml of the artifact: add the three package dirs (sorted)")


def workspace_members():
    def fn(s):
        add = "".join(f'    "{os.path.relpath(c["dir"], W)}",\n' for c in CRATES.values())
        return s.replace("members = [\n", "members = [\n" + add, 1)
    edit(f"{W}/Cargo.toml", fn)


def main():
    new_crates()
    for p in PRODUCTION_CONSUMERS + TEST_MIRRORS:
        edit(p, repoint)
    edit(f"{TESTS}/🪞️mirror/🦀️.rs", tests_mirror)
    manifests()
    workspace_members()
    locks_and_owners()
    print("APPLIED" if APPLY else "DRY RUN (nothing written)")


if __name__ == "__main__":
    main()
