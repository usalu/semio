import os, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E
import genmirror, compgen, union

cid = "os-directory"
c = union.cfg_of(cid)
O = "🧰️framework/🛍️products/💻️os/🔨️modules/"
D = O + "📇️directory/"
PKG = "semio_framework_os_directory"
SNAP = os.path.join(union.W, cid, "snap")
live_lib = f"{E.ROOT}/{c['p_lib']}"
lib = f"{SNAP}/{c['p_lib']}"


def live(p):
    return p.replace(SNAP, E.ROOT) if isinstance(p, str) else p

if "--p" in sys.argv:
    for host_mp, name, host_file, d, tfile in genmirror.test_mounts(lib):
        host_file = live(host_file)
        t = E.read(host_file)
        E.pre_copy(host_file)
        E.write(host_file, genmirror.removal(t, d))
        E.note("edit", host_file)
    pc = D + "📦️packages/🦀️rust/Cargo.toml"
    E.edit(pc, "\n[dev-dependencies]\nsemio-framework-async-macros = { path = \"../../../../../../🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust\" }\n", "")

manifest = open(os.path.join(SNAP, D + "📦️packages/🦀️rust/Cargo.toml"), encoding="utf-8").read()
specs = compgen.dep_specs(manifest)


def mirror_header(hosts):
    return hosts


def uses_for(host_mp, host_file, test_files):
    used = genmirror.used_tokens(test_files)
    real = lambda mp: (PKG,) + tuple(mp)
    return genmirror.host_use_lines(host_file, host_mp, PKG, real, used)


mounts = {("::".join(m[0]) or "(root)"): (m[0], m[1], live(m[2]), m[3], live(m[4])) for m in genmirror.test_mounts(lib)}
PRE = os.path.join(union.W, cid, "snap")


def node(host, cdir):
    host_mp, name, host_file, d, tfile = mounts[host]
    files = [live(f) for f in genmirror.subtree_files(lib, host_mp + (name,))]
    uses = uses_for(host_mp, host_file, files)
    return host_mp, tfile, uses


def render_leaf(pad, name, real, tfile, cdir, uses):
    tdir = os.path.dirname(tfile)
    rel = os.path.relpath(tdir, cdir)
    out = [f'{pad}#[path = "{rel}"]', f"{pad}pub mod {name} {{", f"{pad}    pub use {PKG}::{real}::*;"]
    for u in uses:
        out.append(f"{pad}    {u}")
    out += ["", f'{pad}    #[path = "{os.path.basename(tfile)}"]', f"{pad}    mod tests;", f"{pad}}}"]
    return out


def render_root(cdir, hosts, title, emoji, allocator=None):
    lines = [f"//! {emoji} {title}", "//!", "//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.", "#![cfg(test)]", ""]
    if allocator:
        lines += [allocator, ""]
    tree = {}
    root_leaf = None
    for h in hosts:
        host_mp, tfile, uses = node(h, cdir)
        if not host_mp:
            root_leaf = (tfile, uses)
            continue
        cur = tree
        for seg in host_mp[:-1]:
            cur = cur.setdefault(seg, {})
        cur[host_mp[-1]] = ("leaf", host_mp, tfile, uses)
    lines += [f"pub use {PKG}::*;"]
    if root_leaf:
        tfile, uses = root_leaf
        lines += uses + ["", f'#[path = "{os.path.relpath(tfile, cdir)}"]', "mod tests;"]
    lines.append("")
    def emit(t, pad, prefix=()):
        out = []
        for k, v in t.items():
            if isinstance(v, tuple):
                _, host_mp, tfile, uses = v
                out += render_leaf(pad, k, "::".join(host_mp), tfile, cdir, uses)
            else:
                inner = emit(v, pad + "    ", prefix + (k,))
                while inner and inner[-1] == "":
                    inner.pop()
                out += [f'{pad}#[path = "."]', f"{pad}pub mod {k} {{", f"{pad}    pub use {PKG}::{'::'.join(prefix + (k,))}::*;", ""] + inner + [f"{pad}}}"]
            out.append("")
        return out
    lines += emit(tree, "")
    while lines and lines[-1] == "":
        lines.pop()
    return "\n".join(lines) + "\n"


C1 = D + "🧪️tests/🍯️companion-1/"
C2 = D + "🧪️tests/🍰️companion-2/"
hosts1 = ["schema", "access_policy"]
hosts2 = ["io::binary::checkpoint_pair", "schema::browser_actor", "schema::document_check_in", "(root)", "io::binary::artifact_hash", "io::binary::descriptor_digest"]


def companion(idx, cdirrel, hosts, title, emoji, crate_name, cid_, project, desc):
    cdir = f"{E.ROOT}/{cdirrel}"
    files = []
    for h in hosts:
        host_mp, name, host_file, d, tfile = mounts[h]
        files += [live(f) for f in genmirror.subtree_files(lib, host_mp + (name,))]
    import extcrates
    used = extcrates.used(files, list(specs))
    import re
    rtext = render_root(cdir, hosts, title, emoji)
    for ident in specs:
        if re.search(r"(?<![\w:])" + re.escape(ident) + r"\s*::", rtext):
            used.setdefault(ident, set()).add("root")
    dep_lines = []
    for ident in sorted(used):
        dep_lines.append(compgen.reroot(compgen.merged_line(specs[ident]), 2))
    pk = cdirrel + "📦️packages/🦀️rust/"
    depth = len(pk.rstrip("/").split("/"))
    cargo = compgen.cargo_text(crate_name, cid_, desc, depth, dep_lines, "semio-framework-os-directory", "../../../../📦️packages/🦀️rust", extra_sections="")
    E.create(pk + "Cargo.toml", cargo)
    E.create(pk + "📋️project.json", compgen.project_json(project))
    E.create(cdirrel + "🦀️.rs", render_root(cdir, hosts, title, emoji))
    return sorted(used)


if "--show" in sys.argv:
    print(render_root(f"{E.ROOT}/{C1}", hosts1, "Companion tests of os-directory (1 of 2): schema unit tests and access policy tests.", "🍯️"))
    print("=====")
    print(render_root(f"{E.ROOT}/{C2}", hosts2, "Companion tests of os-directory (2 of 2): binary IO, browser actor, document check-in and root unit tests.", "🍰️"))
if "--create" in sys.argv:
    u1 = companion(1, C1, hosts1, "Companion tests of os-directory (1 of 2): schema unit tests and access policy tests.", "🍯️", "semio-framework-os-directory-tests-1", "os-directory-tests-1", "@semio-tech/os-directory-tests-1-rs", "Companion tests of os-directory (1 of 2): schema unit tests and access policy tests")
    u2 = companion(2, C2, hosts2, "Companion tests of os-directory (2 of 2): binary IO, browser actor, document check-in and root unit tests.", "🍰️", "semio-framework-os-directory-tests-2", "os-directory-tests-2", "@semio-tech/os-directory-tests-2-rs", "Companion tests of os-directory (2 of 2): binary IO, browser actor, document check-in and root unit tests")
    print(u1, u2)
