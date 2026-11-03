"""🎬 Splits a scene crate: moves all module dirs except the named facade ones into a sibling `<scene>-members` crate (pub items), keeps the SceneKit impl, live scene, diagrams and tests in the facade. Usage: scene_members.py en1991 (see ARTS). Idempotent."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

ARTS = {"en1991": ("🏋️en1991", ["📈️diagrams", "🌈️fields", "📦️packages", "🧪️tests", "🔮️oracles"], "members", "The complete scene"), "en1993": ("🔩️en1993", ["📈️diagrams", "📈️entity-diagrams", "📈️annex-diagrams", "📦️packages", "🧪️tests", "🔮️oracles"], "scene_members", "The complete scene")}


def run(code):
    emo, keep, alias, marker = ARTS[code]
    art = NORM + "/🗿️artifacts/" + emo
    base = art + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
    F = base + "/🎬️scene"
    M = base + "/🎬️scene-members"
    old = "semio-s-artifact-norm-%s-scene" % code
    new = old + "-members"
    if os.path.exists(M + "/🦀️.rs"):
        print("exists")
        return
    root = rd(F + "/🦀️.rs")
    cut = root.index("#[path = \"🧰️kit")
    head = root[:cut]
    mods = [d for d in sorted(os.listdir(F)) if os.path.isdir(F + "/" + d) and d not in keep]
    for d in mods:
        shutil.move(F + "/" + d, M + "/" + d) if os.path.exists(M) else (os.makedirs(M), shutil.move(F + "/" + d, M + "/" + d))
    for dp, dn, fn in os.walk(M):
        top = os.path.dirname(dp) == M
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                s = rd(p)
                t = s.replace("pub(crate)", "pub")
                if top and f == "🦀️.rs":
                    t = t.replace("pub(super)", "pub")
                if t != s:
                    wr(p, t)
    body = root[cut:]
    mod_re = re.compile(r'(#\[path = "[^"]+"\]\n)(pub )?mod (\w+);\n')
    mount_block = body[: body.index("use kit::Out;")]
    tail = body[body.index("use kit::Out;"):]
    members_mounts = "\n".join(l for l in mount_block.split("\n"))
    members_mounts = members_mounts[: members_mounts.index("#[path = \"📈️diagrams")] + members_mounts[members_mounts.index("#[path = \"🕹️handles"):]
    members_mounts = mod_re.sub(lambda m: m.group(1) + "pub mod %s;\n" % m.group(3), members_mounts)
    imports_end = tail.index("/// 🎯️")
    proj_at = re.search(r"/// \S+ " + marker, tail).start()
    imports = tail[:imports_end]
    sceneinput = tail[imports_end: proj_at]
    sceneinput = sceneinput.replace("pub(crate) struct Ctx", "pub struct Ctx")
    frag_start = tail.index("/// 🧩️ The part of the scene")
    frag_end = tail.index("/// 🧲️ The live-scene contract")
    fragment = tail[frag_start:frag_end]
    project = tail[proj_at:frag_start]
    facade_rest = tail[frag_end:]
    facade_rest = facade_rest[: facade_rest.index("#[cfg(test)]")], facade_rest[facade_rest.index("#[cfg(test)]"):]
    ext = head[head.index("extern crate"):]
    ext_m = ext.replace("extern crate", "extern crate", 1)
    doc_m = "//! 🧩️ EN %s scene members: the kit, layout, module families and drag handles below the scene facade [`%s`]; every item the facade, the diagrams and the tests reach is `pub`.\n\n" % (code[2:], old.replace("-", "_"))
    use_names = lambda s, text: s
    members_body = doc_m + ext_m + members_mounts + imports + sceneinput + fragment
    all_m = ""
    for dp, dn, fn in os.walk(M):
        for f in fn:
            if f.endswith(".rs"):
                all_m += rd(os.path.join(dp, f))
    members_body = prune(members_body, all_m)
    wr(M + "/🦀️.rs", members_body)
    modnames = []
    for m in re.finditer(r'#\[path = "([^"]+)/🦀️\.rs"\]\npub mod (\w+);', members_body):
        modnames.append(m.group(2))
    priv = [n for n in modnames if n not in ("layout", "handles", "kit")]
    reexp = re.search(r"pub use kit::\{[^}]*\};", members_body).group(0).replace("pub use kit::", "pub use %s::kit::" % alias)
    facade_head = head + "extern crate %s as %s;\n\n" % (new.replace("-", "_"), alias)
    uses = "use {A}::{{kit, {P}}};\n{R}\npub use {A}::layout;\npub use {A}::layout::{{Layout, Placement}};\npub use {A}::handles;\npub use {A}::handles::handles;\npub use {A}::{{fragment, SceneInput}};\n\n#[path = \"📈️diagrams/🦀️.rs\"]\npub mod diagrams;\npub use diagrams::diagrams;\n\n".format(A=alias, P=", ".join(priv), R=reexp)
    fi = imports.replace("use kit::Out;\n", "")
    facade_all = ""
    for dp, dn, fn in os.walk(F):
        for f in fn:
            if f.endswith(".rs") and "packages" not in dp and "fields" not in dp and not (dp == F and f == "🦀️.rs"):
                facade_all += rd(os.path.join(dp, f))
    fbody = facade_head + uses + fi + "\n" + project + facade_rest[0] + facade_rest[1]
    for crate_alias in re.findall(r"extern crate \w+ as (\w+);", fbody):
        if crate_alias != alias and not re.search(r"\b%s::" % crate_alias, facade_all + fbody.replace("extern crate", "")):
            fbody = re.sub(r"extern crate \w+ as %s;\n" % crate_alias, "", fbody)
    fbody = prune(fbody, facade_all)
    wr(F + "/🦀️.rs", fbody)
    pkg = M + "/📦️packages/🦀️rust"
    fm = rd(F + "/📦️packages/🦀️rust/Cargo.toml")
    deps = re.search(r"\[dependencies\]\n(.*?)\n\n", fm, re.S).group(1)
    mt = fm.replace('name = "%s"' % old, 'name = "%s"' % new).replace(re.search(r'description = "[^"]*"', fm).group(0), 'description = "EN %s scene members: kit, layout, module families and drag handles"' % code[2:])
    wr(pkg + "/Cargo.toml", mt)
    ft = fm.replace("[dependencies]\n", "[dependencies]\n%s = { workspace = true }\n" % new, 1)
    wr(F + "/📦️packages/🦀️rust/Cargo.toml", ft)
    register_workspace(art, new, pkg)
    script_register(art, old, new)
    lock_register(old, new, new_deps=toml_deps(mt))
    print("split", old, "->", new, len(mods))


def prune(src, corpus):
    """Drops unused names from the single-line `use` imports of the root text."""
    words = set(re.findall(r"\b\w+\b", corpus + re.sub(r"(?m)^use .*$", "", src)))

    def one(m):
        path, names = m.group(1), m.group(2)
        keep = [n for n in (x.strip() for x in names.split(",")) if n and (n.split(" as ")[-1] in words)]
        return "use %s::{%s};" % (path, ", ".join(keep)) if keep else ""

    def single(m):
        n = m.group(1).split("::")[-1].split(" as ")[-1]
        return m.group(0) if n in words else ""

    src = re.sub(r"(?m)^use ([\w:]+)::\{([^}]*)\};", one, src)
    src = re.sub(r"(?m)^use ([\w:]+);", single, src)
    return re.sub(r"\n{3,}", "\n\n", src)


run(sys.argv[1])
