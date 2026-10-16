# 🔎️ Item index of the plugin tree: top-level items per file, module mounts, `use` re-exports; the lookup the flip engine resolves `crate::module::name` paths with.
import os, re, collections
from common import *
from rs import blank, expand_use

HEAD = re.compile(r"(?m)^[ \t]*(?P<attrs>(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)(?P<vis>pub(?:[ \t]*\([^)]*\))?[ \t]+)?(?P<mods>(?:(?:async|unsafe|const|extern(?:[ \t]+\"[^\"]*\")?|default)[ \t]+)*)(?P<kind>fn|struct|enum|trait|type|const|static|mod|union|macro_rules!)[ \t]+(?:mut[ \t]+)?(?P<name>[A-Za-z_][A-Za-z0-9_]*)")
USE = re.compile(r"(?m)^[ \t]*(?P<attrs>(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)(?P<vis>pub(?:[ \t]*\([^)]*\))?[ \t]+)?use[ \t]+(?P<body>[^;]*);")
MOUNT = re.compile(r"(?P<attrs>(?:#\[[^\]]*\]\s*)*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*;")
PATHATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')
THREAD = re.compile(r"thread_local!\s*\{")


def depth_map(clean):
    d, out = 0, []
    for ch in clean:
        out.append(d)
        if ch == "{": d += 1
        elif ch == "}": d -= 1
    return out


class FileInfo:
    def __init__(self, path):
        self.path = path
        try:
            self.text = read(path)
        except OSError:
            self.text = ''
        self.clean = blank(self.text)
        dm = depth_map(self.clean)
        self.items = {}
        for m in HEAD.finditer(self.clean):
            if dm[m.start("kind")] != 0:
                continue
            self.items.setdefault(m.group("name"), []).append(dict(kind=m.group("kind"), vis=(m.group("vis") or "").strip(), pos=m.start("kind"), attrs=m.group("attrs")))
        self.uses = []
        for m in USE.finditer(self.clean):
            if dm[m.start()] != 0:
                continue
            vis = (m.group("vis") or "").strip()
            attrs = self.text[m.start("attrs"):m.end("attrs")].strip()
            for path, alias, glob in expand_use(m.group("body")):
                self.uses.append(dict(vis=vis, attrs=attrs, path=path, alias=alias, glob=glob))
        self.mounts = []
        for m in MOUNT.finditer(self.clean):
            if dm[m.start("name")] != 0:
                continue
            attrs = self.text[m.start("attrs"):m.end("attrs")]
            pm = PATHATTR.search(attrs)
            self.mounts.append(dict(name=m.group("name"), vis=(m.group("vis") or "").strip(), attrs=attrs.strip(), path=pm.group(1) if pm else None, test=bool(re.search(r"cfg\(\s*test\s*\)", attrs))))
        self.includes = [m.group(1) for m in INCLUDE.finditer(self.text) if self.clean[m.start():m.start() + 8] == "include!"]
        self.inline = {}
        for m in re.finditer(r"(?m)^[ \t]*(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*(?:pub(?:[ \t]*\([^)]*\))?[ \t]+)?mod[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*\{", self.clean):
            if dm[m.start()] != 0:
                continue
            depth, j = 1, m.end()
            while j < len(self.clean) and depth:
                depth += (self.clean[j] == "{") - (self.clean[j] == "}")
                j += 1
            body, bclean = self.text[m.end():j - 1], self.clean[m.end():j - 1]
            uses = []
            for um in USE.finditer(bclean):
                for path, alias, glob in expand_use(um.group("body")):
                    uses.append(dict(vis=(um.group("vis") or "").strip(), path=path, alias=alias, glob=glob))
            items = {}
            for hm in HEAD.finditer(bclean):
                items.setdefault(hm.group("name"), []).append(dict(kind=hm.group("kind"), vis=(hm.group("vis") or "").strip()))
            self.inline[m.group(1)] = dict(uses=uses, items=items, includes=[x.group(1) for x in INCLUDE.finditer(body)])
        self.statics = []
        for m in THREAD.finditer(self.clean):
            j = m.end(); depth = 1
            while j < len(self.clean) and depth:
                depth += (self.clean[j] == "{") - (self.clean[j] == "}")
                j += 1
            for s in re.finditer(r"static\s+([A-Za-z_][A-Za-z0-9_]*)", self.clean[m.end():j]):
                self.statics.append(s.group(1))


class Tree:
    """lazy file cache + module-tree helpers on a plugin directory (live repo or a sandbox copy)"""
    def __init__(self, plug=PLUG):
        self.plug = plug
        self.cache = {}

    def info(self, rel):
        if rel not in self.cache:
            self.cache[rel] = FileInfo(self.plug + "/" + rel)
        return self.cache[rel]

    def exists(self, rel):
        return os.path.exists(self.plug + "/" + rel)

    def child_rel(self, rel, mount):
        if mount["path"]:
            return os.path.normpath(os.path.join(os.path.dirname(rel), mount["path"]))
        stem = os.path.splitext(os.path.basename(rel))[0]
        base = os.path.join(os.path.dirname(rel), stem)
        for cand in (os.path.join(base, mount["name"] + ".rs"), os.path.join(base, mount["name"], "mod.rs")):
            if self.exists(cand):
                return cand
        return None

    def children(self, rel):
        fi = self.info(rel)
        out = []
        for m in fi.mounts:
            c = self.child_rel(rel, m)
            if c and self.exists(c):
                out.append((m["name"], c, m))
        for inc in fi.includes:
            c = os.path.normpath(os.path.join(os.path.dirname(rel), inc))
            if self.exists(c):
                out.append(("<include>", c, None))
        return out

    def tree(self, rel):
        seen, order = set(), []
        def walk(r):
            if r in seen or not self.exists(r):
                return
            seen.add(r); order.append(r)
            for _, c, _ in self.children(r):
                walk(c)
        walk(rel)
        return order
