"""🏗️ Moves the remaining monolith test mounts into the test bins.
usage: mk_bins.py plan | apply [bin ...]"""
import json, sys
from build import *
import inventory as INV
import manifest

APPLY = "apply" in sys.argv
ONLY = [a for a in sys.argv[1:] if a not in ("plan", "apply")]
TD = EL + "/🐚️Shell/🧪️tests/"
UE = lambda d: E + "/🧪️tests/" + d + "/🦀️.rs"
SHELL_T = lambda d: TD + d + "/🦀️.rs"
ENG_T = lambda d: EL + "/⚙️EngineCanvas/🧪️tests/" + d + "/🦀️.rs"

# bin suffix -> test directory names (Shell tests under 🐚️Shell/🧪️tests, root tests under E/🧪️tests)
SHELL_PLAN = {
    "shell-tests-windows-1": ["🔬️wgpu-panel-anchor-model", "🪟️wgpu-window-pane-chrome"],
    "shell-tests-windows-2": ["🎬️wgpu-window-actions-search-panes", "📏️wgpu-window-measures", "🧲️engine-surface-retention", "🛟️panel-window-reservation", "🛰️wgpu-dock-close-reopen-journals"],
    "shell-tests-chrome-3": ["🔬️wgpu-shell-chrome-parity"],
    "shell-tests-chrome-4": ["🎨️wgpu-theme-editor-and-accessibility"],
    "shell-tests-chrome-5": ["🧭️wgpu-navbar-footer-parity", "🖥️wgpu-display-conflicts-marketplace", "⚙️settings-general-layout"],
    "shell-tests-chrome-6": ["🌓️appearance-tour-and-footer-pills", "🚪️wgpu-host-door-remainder"],
    "shell-tests-input-1": ["🔬️wgpu-shell-input"],
    "shell-tests-input-2": ["🔬️wgpu-command-registry", "⌨️wgpu-shell-shortcuts-palette", "🎯️presented-input-authority"],
    "shell-tests-input-3": ["🎓️tour-overlay-and-chord-glyphs", "⌨️wgpu-chord-example-role-parity", "🎡️wgpu-wheel-and-escape-routing", "🪟️window-lifecycle-template-drag", "🎯️wgpu-pointer-hit-ownership"],
    "shell-tests-docs-1": ["🧪️wgpu-time-travel"],
    "shell-tests-docs-2": ["🔗️hub-projection-workspace", "🧪️wgpu-local-folders", "🔀️wgpu-document-relay", "🔬️wgpu-shell-boot-isolation"],
    "shell-tests-docs-3": ["👕️board-presence", "🔄️sync-owner-lifetime", "🔄️detached-sync-task-manager"],
}


ROOT_PLAN = {
    "runtime-mailbox-tests": [E + "/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs", REPO + "/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/📥️enqueue/🦀️.rs"],
}
HOST_PLAN = {
    "shell-tests-chrome-2": ([EL + "/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs"], ["use semio_framework_os_renderer_wgpu_dock::*;"]),
}
SHELL_EXTRA = {
    "shell-tests-docs-2": ["use semio_framework_os_renderer_wgpu_doors::directory_door::*;"],
}


def snake(dirname):
    s = re.sub(r"^[^A-Za-z0-9]+", "", dirname)
    s = re.sub(r"^wgpu-", "", s)
    return s.replace("-", "_") + "_tests"


def mount_name(row, dirname):
    name = row["name"]
    if name and name.endswith("_host"):
        name = name[:-5]
    return name if name and name != "tests" else snake(dirname)


def effective_cfg(inv, f, row):
    """The cfg a test file runs under: its own mount cfg, or the cfg of the test host that mounts it."""
    if row["cfg"].strip():
        return row["cfg"]
    host_rows = inv.get(row["host"])
    if host_rows and "test-hosts" in row["host"]:
        return host_rows[0]["cfg"]
    return row["cfg"]


def clean_cfg(cfg):
    return "" if cfg.strip() in ("", "#[cfg(test)]") else cfg.strip() + "\n"


def parse_root(root_text):
    """(doc lines, extern-crate lines, import text, mount blocks, rest) of a bin root."""
    lines = root_text.split("\n")
    doc = [l for l in lines if l.startswith("//!")]
    m0 = re.search(r"(?m)^#!\[cfg\(test\)\]\n", root_text)
    body = root_text[m0.end():] if m0 else root_text
    mi = re.search(r"(?m)^(?:#\[cfg[^\n]*\]\n)*#\[path = ", body)
    imports_txt = body[:mi.start()] if mi else body
    rest = body[mi.start():] if mi else ""
    mounts = re.findall(r"((?:#\[cfg[^\n]*\]\n)*)#\[path = \"([^\"\n]+)\"\]\n(pub )?mod ([A-Za-z_0-9]+);", rest)
    return doc, imports_txt, mounts, rest


def flat_bin(suffix, dirs_or_files):
    """Plan of one flat bin: new root text, new test texts, deps idents."""
    crates = crate_table()
    folder = T + "/" + crates[suffix]
    root = folder + "/🦀️.rs"
    old = read(root)
    doc, imports_txt, old_mounts, rest = parse_root(old)
    inv = INV.inventory()
    new = []
    for d in dirs_or_files:
        f = SHELL_T(d) if not os.path.isabs(d) else d
        rows = inv.get(f)
        assert rows, ("no mount found", d)
        row = rows[0]
        new.append((f, row, os.path.basename(os.path.dirname(f))))
    return folder, root, old, doc, imports_txt, old_mounts, new, inv


def plan_flat(suffix, dirs, apply=False, extra_lines=()):
    folder, root, old, doc, imports_txt, old_mounts, new, inv = flat_bin(suffix, dirs)
    X.CURRENT[0] = suffix
    hosts = sorted({r["host"] for f, r, d in new})
    set_hosts(hosts)
    texts, mounts = {}, []
    for f, row, d in new:
        t = B.transform_test_probe(read(f))
        texts[f] = t
        rel_ = os.path.relpath(f, folder)
        mounts.append((clean_cfg(effective_cfg(inv, f, row)), rel_, mount_name(row, d)))
    tmpdir = SCRATCH + "/bin-" + suffix
    os.makedirs(tmpdir, exist_ok=True)
    tmp_files = []
    for i, (f, t) in enumerate(texts.items()):
        p = tmpdir + f"/t{i}.rs"
        write(p, t)
        tmp_files.append(p)
    aliases, lines, unresolved = full_imports(suffix, tmp_files)
    old_entries = [l for l in re.findall(r"(?m)^(?:#\[[^\n]*\]\n)*use [^;]*;", imports_txt)]
    old_locals = set()
    for cfg, vis, p, local, glob in uses(imports_txt):
        if local and not glob:
            old_locals.add(local)
    old_aliases = re.findall(r"(?m)^(?:#\[[^\n]*\]\n)?extern crate [^;]*;", imports_txt)
    new_imports = merge_import_blocks(old_entries + lines + list(extra_lines), simplify=False)
    alias_lines = list(dict.fromkeys(old_aliases + aliases))
    return dict(folder=folder, root=root, old=old, doc=doc, aliases=alias_lines, imports=new_imports, old_mounts=old_mounts, mounts=mounts, texts=texts, unresolved=unresolved, idents=set(), hosts=hosts)


def render_root(p, uses_shell_source):
    first = p["doc"][0] if p["doc"] else "//! 🧪️ Companion test bin of the wgpu renderer."
    head = first + "\n//!\n//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below."
    parts = [head, "", "#![cfg(test)]", ""]
    if p["aliases"]:
        parts += p["aliases"]
    parts += p["imports"]
    if uses_shell_source:
        parts.append("use semio_framework_os_renderer_wgpu_test_support::shell_source;")
    parts.append("")
    for cfg, rel_, name in p["old_mounts_norm"]:
        parts.append(f'{cfg}#[path = "{rel_}"]\nmod {name};')
    for cfg, rel_, name in p["mounts"]:
        parts.append(f'{cfg}#[path = "{rel_}"]\nmod {name};')
    return "\n".join(parts) + "\n"


def run(suffix, dirs, extra_lines=()):
    p = plan_flat(suffix, dirs, extra_lines=extra_lines)
    p["old_mounts_norm"] = [(c, r, n) for c, r, vis, n in p["old_mounts"]]
    all_texts = list(p["texts"].values())
    ss = any("shell_source()" in t for t in all_texts) or "shell_source()" in "".join(read(os.path.normpath(os.path.join(p["folder"], r))) for c, r, n in p["old_mounts_norm"])
    text = render_root(p, ss)
    print(suffix, "+", len(p["mounts"]), "tests", "root lines", len(text.split("\n")), "unresolved:", p["unresolved"])
    if APPLY:
        for f, t in p["texts"].items():
            edit(f, t)
        edit(p["root"], text)
        mp = p["folder"] + "/📦️packages/🦀️rust/Cargo.toml"
        added = manifest.add_deps(mp, used_idents(text) | set().union(*[used_idents(t) for t in p["texts"].values()]))
        print("   deps added:", added)
    return p


if __name__ == "__main__":
    for suffix, dirs in SHELL_PLAN.items():
        if ONLY and suffix not in ONLY:
            continue
        run(suffix, dirs + ([UE("📇️wgpu-directory-door")] if suffix == "shell-tests-docs-2" else []), SHELL_EXTRA.get(suffix, ()))
    for suffix, dirs in ROOT_PLAN.items():
        if ONLY and suffix not in ONLY:
            continue
        run(suffix, [os.path.normpath(d) for d in dirs])
    for suffix, (dirs, extra) in HOST_PLAN.items():
        if ONLY and suffix not in ONLY:
            continue
        run(suffix, dirs, extra)
