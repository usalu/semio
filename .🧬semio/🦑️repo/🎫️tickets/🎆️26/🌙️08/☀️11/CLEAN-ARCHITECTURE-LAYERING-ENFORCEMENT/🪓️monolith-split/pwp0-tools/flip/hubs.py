# 🪝️ Hub generation for the integration pass: every hub / root unit of a flipped module becomes an `extern crate`, a re-export of the entry module, or a thin shim module over the crates that hold pieces of the old module tree; facade test hosts for files that stay in the facade.
import os, re, collections
from common import *
from rs import blank, expand_use
import model

HUB_FILES = None
UNIT = re.compile(r"(?P<pre>(?:[ \t]*(?:///[^\n]*|#\[[^\]\n]*\])[ \t]*\n)*)(?P<ind>[ \t]*)(?P<vis>pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+(?P<name>[A-Za-z_][A-Za-z0-9_]*)[ \t]*;[ \t]*\n")
PATHATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')


def libname(k):
    return PFX + k.replace("-", "_")


def hub_files(fb):
    return ["🦀️.rs"] + ["🪝️mounts/" + h + "/🦀️.rs" for h in sorted(os.listdir(fb.plug + "/🪝️mounts"))]


def units_of(text, rel):
    clean = blank(text)
    out = []
    base = os.path.dirname(rel)
    for m in UNIT.finditer(clean):
        pre = text[m.start("pre"):m.end("pre")]
        pm = PATHATTR.search(pre)
        if not pm:
            continue
        out.append(dict(start=m.start(), end=m.end(), pre=pre, vis=(m.group("vis") or "").strip(), name=m.group("name"), file=os.path.normpath(os.path.join(base, pm.group(1))), ind=m.group("ind")))
    return out


def strip_path_attr(pre):
    return "".join(l + "\n" for l in pre.split("\n") if l.strip() and not PATHATTR.search(l))


def old_tree_owners(fb, f):
    """crates owning production files of the old module tree of f"""
    res = collections.OrderedDict()
    for g in fb.w.T.tree(f):
        if "/🧪️tests/" in g or g.startswith("🧪️tests/") or g in fb.w.ext_of:
            continue
        j = fb.owner(g)
        if j in fb.real:
            res.setdefault(j, []).append(g)
    return res


def translate_stmt(fb, j, child, stmt, host_mod=None):
    """a removed `use name::{..}` of the old parent -> `pub use <module expr>::{..}` for the shim"""
    m = re.match(r"(?:pub(?:\([^)]*\))?\s+)?use\s+(.*);\s*$", stmt.strip(), re.S)
    if not m:
        return []
    lines = []
    if host_mod:
        modexpr = f"super::{host_mod}"
    else:
        modexpr = libname(j) if child == fb.L.root.get(j) else f"{libname(j)}::{fb.entry_name(j, child)}"
    for path, alias, glob in expand_use(m.group(1)):
        if not path:
            continue
        rest = path[1:] if path[0] not in ("self",) else path[2:]
        tail = "::".join(rest)
        a = f" as {alias}" if alias else ""
        if glob:
            lines.append(f"    pub use {modexpr}{('::' + tail) if tail else ''}::*;")
        elif tail:
            lines.append(f"    pub use {modexpr}::{tail}{a};")
        else:
            lines.append(f"    pub use {modexpr}{a};" if a else f"    pub use {modexpr};")
    return lines


def host_name(fb, parent_unit_name, h):
    return f"{parent_unit_name}_{h['name']}"


def build(fb):
    hub_changes = {}
    unit_of_file = {}
    hubs = {}
    for rel in hub_files(fb):
        t = fb.text(rel)
        hubs[rel] = t
        for u in units_of(t, rel):
            unit_of_file.setdefault(u["file"], u["name"])
    host_files = {}
    host_mounts = []
    for h in fb.hosts:
        pu = unit_of_file.get(h["parent"])
        if not pu:
            fb.problems.append(("host parent is not a hub unit", h["parent"], h["child"]))
            continue
        h["host_mod"] = host_name(fb, pu, h)
    for rel, t in hubs.items():
        units = units_of(t, rel)
        out, last = [], 0
        for u in units:
            F, M = u["file"], u["name"]
            K0 = fb.owner(F)
            if K0 not in fb.keys:
                continue
            owners = old_tree_owners(fb, F)
            foreign = [j for j in owners if j != K0]
            extras = [e for e in fb.shim_extra.get(F, [])]
            hosts = [h for h in fb.hosts if h["parent"] == F and "host_mod" in h]
            attrs = strip_path_attr(u["pre"])
            vis = u["vis"] or ""
            root = fb.L.root.get(K0)
            if F == root:
                main = f"{libname(K0)}"
            else:
                main = f"{libname(K0)}::{fb.entry_name(K0, F)}"
            if not foreign and not extras and not hosts:
                if F == root:
                    v = vis if vis else "pub(crate)" if False else vis
                    text = f"{attrs}{(vis + ' ') if vis else ''}extern crate {libname(K0)} as {M};\n"
                else:
                    text = f"{attrs}{(vis + ' ') if vis else ''}use {main};\n"
            else:
                body = [f"    pub use {main}::*;"]
                for j in foreign:
                    if j.startswith("TOP"):
                        continue
                    jr = fb.L.root.get(j)
                    body.append(f"    pub use {libname(j)}::*;")
                for j, child, stmt in extras:
                    body += translate_stmt(fb, j, child, stmt, None)
                    cname = None
                for h in hosts:
                    body.append(f"    pub use super::{h['host_mod']}::*;")
                lines = "\n".join(dict.fromkeys(body))
                text = f"{attrs}{(vis + ' ') if vis else 'pub(crate) '}mod {M} {{\n{lines}\n}}\n"
            out.append(t[last:u["start"]])
            out.append(text)
            last = u["end"]
            fb.log.append(("hub-unit", rel, M, K0, foreign))
        out.append(t[last:])
        nt = "".join(out)
        if nt != t:
            hub_changes[rel] = nt
    # hosts: files + mounts in hub 7
    hub7 = "🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
    mount_lines = []
    for h in fb.hosts:
        if "host_mod" not in h:
            continue
        pu = unit_of_file[h["parent"]]
        rel = f"🪝️mounts/🧵️wp7-instance-tier2/🧪️hosts/{h['host_mod']}/🦀️.rs"
        parent_text = fb.w.T.info(h["parent"]).text
        uses = []
        for m in re.finditer(r"(?m)^((?:#\[[^\]\n]*\][ \t]*\n)*)(?!pub)(use[ \t]+[^;]*;)[ \t]*$", blank(parent_text)):
            uses.append((parent_text[m.start(1):m.end(1)] + parent_text[m.start(2):m.end(2)]).strip())
        base = os.path.dirname(rel)
        child_rel = os.path.relpath(h["child"], base)
        body = [f"//! 🧪️ Facade host of `{h['name']}` (was a child module of `{pu}`): the names it uses are in scope as in the old parent."]
        body += uses
        body.append(f"#[allow(unused_imports)]\npub use crate::{pu}::*;")
        body += h["cfg"]
        body += [f'#[path = "{child_rel}"]', f"pub(crate) mod {h['name']};", ""]
        text = "\n".join(body)
        host_files[rel] = text
        mount_lines += [f"#[path = \"{os.path.relpath(rel, os.path.dirname(hub7))}\"]", f"mod {h['host_mod']};"]
    if mount_lines:
        t7 = hub_changes.get(hub7) or hubs[hub7]
        region = "//#region 🧪️Wp0HostsIntegration1\n"
        end = "//#endregion 🧪️Wp0HostsIntegration1\n"
        add = region + "\n".join(mount_lines) + "\n" + end
        t7 = (t7.rstrip("\n") + "\n" + add)
        hub_changes[hub7] = t7
    return hub_changes, host_files
