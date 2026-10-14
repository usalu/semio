#!/usr/bin/env python3
"""✂️ Cuts top-level modules of `semio-framework-plugin` into an owner crate of their own.

Given a crate directory (a taxonomy node under the plugin) and a set of top-level modules whose closure lies inside the
crate and the crates cut before it, the tool:

1. writes `<dir>/🦀️.rs` (docs, the extern aliases the modules use, one `#[path] pub mod` per module) and
   `<dir>/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts}`, with the external dependencies the modules name;
2. removes the module declarations from the plugin root; the root's public `pub use <m>::…` statements and the `app`
   vocabulary now name the owner crate (the SDK facade the plugin already published, not a new alias);
3. rewrites every path: inside the new crate `crate::component::…` → `crate::…`, root re-export names → their owner
   module, `crate::__value`-style hygiene aliases → the real crate, `pub(crate)`/top-level `pub(super)` → `pub`; inside the
   plugin crate and every earlier-cut crate `crate::<m>::…` / `super::<m>::…` / grouped leaves → `<owner>::<m>::…`, and
   `$crate::<m>::…` in the plugin's exported macros → `$crate::__<short>::<m>::…` through a hidden hygiene re-export;
4. adds the dependency to the plugin manifest and the member to the root workspace;
5. reports every reference it could not place (a path into the plugin crate from the new crate, a test mount reading
   plugin-only fixtures) instead of guessing.

Files modified within the last 20 minutes by someone else are refused (peer guard). Usage:
extract-crate.py <crate name> <dir under the plugin> <module,…> --doc "<//! line>" [--dry] [--resume m=path,…]
(`--resume` supplies the `#[path]` of modules an interrupted run already removed from the plugin root.)
"""
import importlib.util, json, os, re, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("move", os.path.join(HERE, "🚚️move.py"))
mv = importlib.util.module_from_spec(spec); spec.loader.exec_module(mv)
fi, ri, PLUGIN = mv.fi, mv.ri, mv.PLUGIN
REPO = "/Users/ueli/Documents/semio"
ROOT = os.path.join(PLUGIN, "🦀️.rs")
ALIASES = {"store": "semio_framework_os_kernel", "protocol": "semio_framework_os_kernel", "dsl": "semio_framework_os_kernel", "vcs": "semio_framework_os_kernel"}
HYGIENE = {"__value": "semio_framework_value", "__kernel": "semio_framework_os_kernel", "__pack_json": "semio_framework_pack_json", "__diagnostic": "semio_framework_diagnostic", "__async": "semio_framework_async"}
GUARD = 1200


def plugin_deps():
    """📦️ The plugin manifest's dependency lines by the identifier Rust code uses."""
    text = open(os.path.join(PLUGIN, "📦️packages/🦀️rust/Cargo.toml"), encoding="utf-8").read()
    deps = text.split("[dependencies]", 1)[1].split("\n[", 1)[0]
    out = {}
    for line in deps.split("\n"):
        m = re.match(r"\s*([\w-]+)\s*=\s*(.*)$", line)
        if not m: continue
        key, spec_ = m.group(1), m.group(2)
        pkg = re.search(r'package\s*=\s*"([^"]+)"', spec_)
        out[key.replace("-", "_")] = (key, pkg.group(1) if pkg else key, spec_)
    return out


SELF = os.path.join(HERE, "🧩️self-touched.json")
TOUCHED = json.load(open(SELF)) if os.path.exists(SELF) else {}


def guarded(f):
    """🛡️ A file a peer changed in the last 20 minutes (any change this lane did not record itself)."""
    if not os.path.exists(f): return False
    m = os.path.getmtime(f)
    return time.time() - m < GUARD and abs(TOUCHED.get(f, -1) - m) > 1


def write(f, text, dry):
    if dry: return
    os.makedirs(os.path.dirname(f), exist_ok=True)
    open(f, "w", encoding="utf-8").write(text)
    TOUCHED[f] = os.path.getmtime(f)
    json.dump(TOUCHED, open(SELF, "w"), ensure_ascii=False, indent=0)


def rel(a, b):
    return os.path.relpath(a, os.path.dirname(b))


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    doc = sys.argv[sys.argv.index("--doc") + 1]
    if doc in args: args.remove(doc)
    if "--resume" in sys.argv: args.remove(sys.argv[sys.argv.index("--resume") + 1])
    dry = "--dry" in sys.argv
    name, cdir, mods = args[0], os.path.join(PLUGIN, args[1]), args[2].split(",")
    ident = name.replace("-", "_")
    short = "__" + name.replace("semio-framework-plugin-", "").replace("-", "_")
    root_file, manifest = os.path.join(cdir, "🦀️.rs"), os.path.join(cdir, "📦️packages/🦀️rust/Cargo.toml")
    if os.path.exists(root_file): sys.exit(f"[extract] {root_file} exists")
    fi.walk(f"{PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{PLUGIN}/📦️packages/🦀️rust")
    owner_files = {}
    for m in fi.modules:
        s = m.path.split("::")
        if len(s) >= 3 and s[2] in mods and m.rng[0] == 0: owner_files.setdefault(m.file, s[2])
    if "--resume" in sys.argv:
        for spec_ in sys.argv[sys.argv.index("--resume") + 1].split(","):
            m_, p_ = spec_.split("=", 1)
            if m_ in owner_files.values(): continue
            f_ = os.path.join(PLUGIN, p_)
            fi.walk(f_, f"crate::component::{m_}", False, os.path.dirname(f_))
        for m in fi.modules:
            s_ = m.path.split("::")
            if len(s_) >= 3 and s_[2] in mods and m.rng[0] == 0: owner_files.setdefault(m.file, s_[2])
    crate_files = set(owner_files)
    prior = json.load(open(os.path.join(HERE, "🧩️owner-crates.json"))) if os.path.exists(os.path.join(HERE, "🧩️owner-crates.json")) else {}
    moved_to = {m: c["ident"] for c in prior.values() for m in c["modules"]}
    moved_to.update({m: ident for m in mods})
    spec2 = importlib.util.spec_from_file_location("imports", os.path.join(HERE, "🧹️imports.py"))
    im = importlib.util.module_from_spec(spec2); spec2.loader.exec_module(im)
    index = im.owner_index()
    root = open(ROOT, encoding="utf-8").read()
    lines = root.split("\n")
    decl = re.compile(r"^(?:pub(?:\(crate\))?\s+)?mod\s+(\w+)\s*;\s*$")
    kept, blocks, facade, i = [], {}, [], 0
    while i < len(lines):
        m = decl.match(lines[i])
        if not m or m.group(1) not in mods:
            kept.append(lines[i]); i += 1; continue
        start = len(kept)
        while start > 0 and re.match(r"^\s*(#\[|///|// )", kept[start - 1]): start -= 1
        head = kept[start:]; del kept[start:]
        blocks[m.group(1)] = head
        i += 1
        while i < len(lines) and re.match(rf"^pub use (?:self::)?{m.group(1)}::", lines[i]):
            stmt = [lines[i]]
            while not lines[i].rstrip().endswith(";"): i += 1; stmt.append(lines[i])
            i += 1
            facade.append("\n".join(stmt).replace(f"pub use {m.group(1)}::", f"pub use {ident}::{m.group(1)}::", 1))
    missing = set(mods) - set(blocks)
    if missing and "--resume" in sys.argv:
        for spec_ in sys.argv[sys.argv.index("--resume") + 1].split(","):
            m_, p_ = spec_.split("=", 1)
            if m_ in missing: blocks[m_] = [f'#[path = "{p_}"]']
        missing = set(mods) - set(blocks)
    if missing: sys.exit(f"[extract] not declared at the plugin root: {sorted(missing)}")
    problems, ext_used, aliases_used, features, hygiene_used, dollar_names = [], set(), set(), set(), set(), set()
    root_reexports = {}
    for stmt in re.findall(r"(?ms)^pub use ([^;]+);", open(ROOT, encoding="utf-8").read()):
        for segs, al, _, _ in mv.parse_use(stmt):
            if segs and segs[0] not in ("app", "self", "crate", "super", "component") and segs[-1] != "*": root_reexports[al or segs[-1]] = "::".join(segs)
    deps = plugin_deps()
    new_texts = {}
    for f in sorted(crate_files):
        raw = open(f, encoding="utf-8").read()
        t = raw.replace("crate::component::", "crate::")
        for h, real in HYGIENE.items():
            if re.search(rf"\$crate::{h}\b", t): hygiene_used.add(h)
            t = re.sub(rf"(?<!\$)\bcrate::{h}\b", real, t)

        def root_name(mm):
            n = mm.group(2)
            if n in mods or n in ALIASES: return mm.group(0)
            if n in moved_to and moved_to[n] != ident: return f"{moved_to[n]}::{n}{mm.group(3)}"
            own = index.get(n)
            if not own and mm.group(1): dollar_names.add(n); return mm.group(0)
            if own:
                om = own.split("::")[1]
                if om in mods: return f"{mm.group(1)}crate::{om}::{n}{mm.group(3)}"
                if om in moved_to: return f"{moved_to[om]}::{om}::{n}{mm.group(3)}"
                problems.append(f"{os.path.relpath(f, PLUGIN)}: crate::{n} lives in plugin module {om}")
            return mm.group(0)
        t = re.sub(r"(\$?)\bcrate::(\w+)(\b(?!::))", lambda mm: root_name(mm), t)

        def mod_path(mm):
            n = mm.group(2)
            if n in mods or n in ALIASES: return mm.group(0)
            if n in moved_to: return ("::" if mm.group(1) else "") + f"{moved_to[n]}::{n}::"
            problems.append(f"{os.path.relpath(f, PLUGIN)}: crate::{n}:: is a plugin module")
            return mm.group(0)
        t = re.sub(r"(\$?)\bcrate::(\w+)::", mod_path, t)
        if owner_files.get(f) and f in owner_files:
            t = re.sub(r"(?<![\w:])super::(\w+)", lambda mm: (f"crate::{mm.group(1)}" if mm.group(1) in mods or mm.group(1) in ALIASES else (f"{moved_to[mm.group(1)]}::{mm.group(1)}" if mm.group(1) in moved_to else mm.group(0))), t) if is_top_file(f, owner_files) else t
            t = re.sub(r"\bpub\(super\)\s+", "pub ", t) if is_top_file(f, owner_files) else t
        t = re.sub(r"\bpub\(crate\)\s+", "pub ", t)
        cl = ri.clean(t)
        for k in deps:
            if re.search(rf"(?<![\w:]){re.escape(k)}::", cl) or re.search(rf"\b{re.escape(k)}\s*::\s*\{{", cl) or re.search(rf"#\[derive\([^\]]*\b{re.escape(k)}::", cl): ext_used.add(k)
        for a in ALIASES:
            if re.search(rf"(?<![\w:]){a}::", cl) or re.search(rf"\bcrate::{a}\b", cl) or re.search(rf"\buse\s+crate::\{{[^}}]*\b{a}\b", cl): aliases_used.add(a)
        features |= set(re.findall(r'feature\s*=\s*"([\w-]+)"', cl))
        for pm in re.finditer(r"\bcrate::(\w+)", cl):
            n = pm.group(1)
            if n not in mods and n not in ALIASES and n not in ("self",) and not n[0].isupper() and n not in moved_to and index.get(n) is None:
                problems.append(f"{os.path.relpath(f, PLUGIN)}: unresolved crate::{n}")
        new_texts[f] = t
    if "semio_framework_os_kernel" in deps and aliases_used: ext_used.add("semio_framework_os_kernel")
    body = [doc]
    for a in sorted(aliases_used): body.append(f"extern crate {ALIASES[a]} as {a};")
    for h in sorted(hygiene_used): body.append(f"#[doc(hidden)]\npub use {HYGIENE[h]} as {h};")
    for n in sorted(dollar_names):
        body.append(f"#[doc(hidden)]\npub use {root_reexports.get(n, 'semio_framework::' + n)};")
        if n not in root_reexports: problems.append(f"$crate::{n} has no explicit plugin root re-export; assumed semio_framework::{n}")
    for m in sorted(mods):
        head = [h for h in blocks[m] if not h.lstrip().startswith("#[path")]
        target = next(h for h in blocks[m] if h.lstrip().startswith("#[path"))
        p = re.search(r'"([^"]+)"', target).group(1)
        body += head + [f'#[path = "{rel(os.path.normpath(os.path.join(PLUGIN, p)), root_file)}"]', f"pub mod {m};"]
    root_text = "\n".join(body) + "\n"
    deps_toml = []
    for k in sorted(ext_used):
        key, pkg, spec_ = deps[k]
        spec_ = re.sub(r'path\s*=\s*"([^"]+)"', lambda mm: f'path = "{rel(os.path.normpath(os.path.join(PLUGIN, "📦️packages/🦀️rust", mm.group(1))), manifest + "/x")}"', spec_)
        deps_toml.append(f"{key} = {spec_}")
    for c in prior.values():
        if any(m in moved_to and moved_to[m] == c["ident"] for m in c["modules"]) and any(re.search(rf"\b{c['ident']}::", t) for t in new_texts.values()):
            deps_toml.append(f'{c["name"]} = {{ path = "{rel(os.path.join(PLUGIN, c["dir"], "📦️packages/🦀️rust"), manifest + "/x")}" }}')
    feats = sorted(f for f in features if f in ("artifact-app-testing", "media-turn-testing", "reserved-producer-testing", "original-actor-context-testing", "component-guest-async"))
    ws = rel(REPO, manifest + "/x")
    cargo = f'''[package]
workspace = "{ws}"
name = "{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{doc[4:].strip()}"

[package.metadata.semio]
role = "framework"
id = "{name.replace("semio-framework-", "")}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"
''' + (("\n[features]\n" + "".join(f"{x} = []\n" for x in feats)) if feats else "") + "\n[dependencies]\n" + "\n".join(deps_toml) + "\n"
    proj_root = os.path.relpath(os.path.dirname(manifest), REPO)
    project = {"name": "@semio-tech/" + name.replace("semio-", ""), "root": proj_root, "sourceRoot": os.path.relpath(cdir, REPO), "projectType": "library", "tags": ["lang:rust", "role:framework", "family:os-plugin"],
               "targets": {"check": {"executor": "repo:script", "options": {"command": "bun 📜️script.ts check", "cwd": "{projectRoot}", "forwardAllArgs": True}},
                           "test": {"executor": "repo:script", "options": {"command": "bun 📜️script.ts test", "cwd": "{projectRoot}", "forwardAllArgs": True}}}}
    up = rel(os.path.join(REPO, "🧰️framework"), manifest + "/x")
    script = f'''#!/usr/bin/env bun
import {{ receiveScriptProcessInvocation }} from "{up}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {{ runCargo }} from "{up}/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import {{ BundleScript, ScriptRouter }} from "{up}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {{ runScriptMain }} from "{up}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🧠️ Type-checks the `{name}` owner crate. */
class CheckScript extends BundleScript {{
  async run(segments: string[]): Promise<void> {{
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }}
}}

/** 🧪️ Runs the `{name}` owner crate's library laws. */
class TestScript extends BundleScript {{
  async run(segments: string[]): Promise<void> {{
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }}
}}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript), {{ invocation: original }}));
'''
    # plugin side rewrites
    plugin_changes = {}
    for f in mv.crate_files():
        if f in crate_files: continue
        s = open(f, encoding="utf-8").read()
        t = s
        for m in mods:
            t = re.sub(rf"\$crate::(?:component::)?{m}::", f"$crate::{short}::{m}::", t)
            t = re.sub(rf"(?<![\w$])crate::(?:component::)?{m}\b", f"{ident}::{m}", t)
            t = re.sub(rf"(?<![\w:])super::{m}\b", f"{ident}::{m}", t) if f == ROOT or is_plugin_top(f) else t
        t = regroup(t, mods, ident)
        if t != s: plugin_changes[f] = t
    rt = "\n".join(kept)
    for m in mods:
        rt = re.sub(rf"(?<![\w:])super::{m}\b", f"{ident}::{m}", rt)
        rt = re.sub(rf"\$crate::(?:component::)?{m}::", f"$crate::{short}::{m}::", rt)
        rt = re.sub(rf"(?<![\w$])crate::(?:component::)?{m}\b", f"{ident}::{m}", rt)
    rt = regroup(rt, mods, ident)
    hyg = f"#[doc(hidden)]\npub use {ident} as {short};"
    anchor = "pub use semio_framework_value as __value;"
    if anchor in rt and hyg not in rt: rt = rt.replace(anchor, anchor + "\n" + hyg, 1)
    if facade:
        rt = rt.replace("//#region 🏛️OwnershipFacet", "\n".join(facade) + "\n//#region 🏛️OwnershipFacet", 1) if "//#region 🏛️OwnershipFacet" in rt else rt + "\n" + "\n".join(facade) + "\n"
    plugin_changes[ROOT] = rt
    print(f"[extract] {name}: {len(mods)} modules, {len(crate_files)} files; deps {sorted(ext_used)}; aliases {sorted(aliases_used)}; features {feats}; plugin files rewritten {len(plugin_changes)}")
    for p in sorted(set(problems)): print("   PROBLEM", p)
    if dry: return
    blocked = [os.path.relpath(f, REPO) for f in list(new_texts) + list(plugin_changes) + [os.path.join(PLUGIN, "📦️packages/🦀️rust/Cargo.toml")] if guarded(f)]
    if blocked: sys.exit("[extract] peer guard, nothing written; changed by someone else in the last 20 minutes: " + ", ".join(blocked))
    for f, t in new_texts.items(): write(f, t, dry)
    for f, t in plugin_changes.items(): write(f, t, dry)
    write(root_file, root_text, dry)
    write(manifest, cargo, dry)
    write(os.path.join(os.path.dirname(manifest), "📋️project.json"), json.dumps(project, ensure_ascii=False, indent=2) + "\n", dry)
    write(os.path.join(os.path.dirname(manifest), "📜️script.ts"), script, dry)
    pm = os.path.join(PLUGIN, "📦️packages/🦀️rust/Cargo.toml")
    pt = open(pm, encoding="utf-8").read()
    line = f'{name} = {{ path = "{rel(os.path.dirname(manifest), pm)}" }}'
    if line not in pt: write(pm, pt.replace("[dependencies]\n", "[dependencies]\n" + line + "\n", 1), dry)
    rm = os.path.join(REPO, "Cargo.toml")
    rtext = open(rm, encoding="utf-8").read()
    member = f'    "{os.path.relpath(os.path.dirname(manifest), REPO)}",'
    anchor = '    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",'
    if member not in rtext:
        if guarded(rm): print(f"[extract] DEFERRED: root workspace member {member.strip()} (root Cargo.toml changed by a peer in the last 20 minutes)")
        else: write(rm, rtext.replace(anchor, anchor + "\n" + member, 1), dry)
    prior[name] = {"name": name, "ident": ident, "dir": os.path.relpath(cdir, PLUGIN), "modules": mods}
    json.dump(prior, open(os.path.join(HERE, "🧩️owner-crates.json"), "w"), ensure_ascii=False, indent=1)


TOPS = None


def is_top_file(f, owner_files):
    return f in owner_files and any(m.file == f and len(m.path.split("::")) == 3 for m in fi.modules)


def is_plugin_top(f):
    return any(m.file == f and len(m.path.split("::")) == 3 for m in fi.modules)


def regroup(text, mods, ident):
    """🌳️ Cuts grouped `use crate::{…}` / `use super::{…}` leaves under a moved module onto their own owner lines."""
    cl = ri.clean(text)
    edits = []
    for m, a, e in mv.statements(text, cl):
        body = text[a:e]
        leaves = mv.parse_use(body)
        hit = [(segs, al, ls, le) for segs, al, ls, le in leaves if len(segs) >= 2 and segs[0] in ("crate", "super") and [s for s in segs[1:] if s != "component"][:1] and [s for s in segs[1:] if s != "component"][0] in mods and "{" in body]
        if not hit: continue
        nb = body
        for segs, al, ls, le in sorted(hit, key=lambda x: -x[2]): nb = mv.cut_leaf(nb, ls, le)
        nb = mv.tidy(nb)
        head = m.group(1) + m.group(2) + m.group(3) + "use "
        new = "\n".join(f"{head}{ident}::{'::'.join(s for s in segs[1:] if s != 'component')}{' as ' + al if al else ''};" for segs, al, _, _ in hit)
        if len(hit) == len(leaves) or not nb: edits.append((m.start(), e + 1, new))
        else: edits.append((m.start(), e + 1, text[m.start():a] + nb + ";\n" + new))
    for s_, e_, t_ in sorted(edits, key=lambda x: -x[0]): text = text[:s_] + t_ + text[e_:]
    return text


if __name__ == "__main__":
    main()
