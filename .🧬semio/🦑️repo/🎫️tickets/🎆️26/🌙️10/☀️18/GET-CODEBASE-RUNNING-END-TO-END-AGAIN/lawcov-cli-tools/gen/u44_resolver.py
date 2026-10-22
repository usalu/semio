import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🪞unmirror/🔍resolver"
CR = lambda n, d: {"name": n, "dir": d, "root": d + "/lib.rs"}
A, B, Cc = CR("a-crate", "a"), CR("b-crate", "b"), CR("c-crate", "c")
def q_path(at, path, consumed, hits):
    return {"op": "path", "at": at, "name": "", "path": path, "from": "", "ignoreVisibility": False, "file": "", "needle": "", "expect": None if consumed is None else {"consumed": consumed, "hits": hits}, "expectHits": [], "expectFlag": False, "expectNode": ""}
def q_member(at, name, hits, frm="", ign=False):
    return {"op": "member", "at": at, "name": name, "path": [], "from": frm, "ignoreVisibility": ign, "file": "", "needle": "", "expect": None, "expectHits": hits, "expectFlag": False, "expectNode": ""}
def q_opaque(at, flag):
    return {"op": "opaque", "at": at, "name": "", "path": [], "from": "", "ignoreVisibility": False, "file": "", "needle": "", "expect": None, "expectHits": [], "expectFlag": flag, "expectNode": ""}
def q_node(file, needle, node):
    return {"op": "nodeAt", "at": "", "name": "", "path": [], "from": "", "ignoreVisibility": False, "file": file, "needle": needle, "expect": None, "expectHits": [], "expectFlag": False, "expectNode": node}
def case(name, crates, files, queries, cut=(), api=()):
    return {"name": name, "crates": crates, "files": files, "cut": list(cut), "api": list(api), "queries": queries}
BA = "pub mod a { pub fn f() {} pub struct S; pub mod b { pub fn g() {} } }\nfn private() {}\npub mod p2 {}\nmod hidden_mod { pub fn q() {} }\n"
cases = [
 case("keywords and nesting", [A, B], {"a/lib.rs": BA, "b/lib.rs": ""}, [
  q_path("a_crate", ["a", "f"], 2, ["item a_crate::a::f"]), q_path("a_crate", ["a", "b"], 2, ["module a_crate::a::b"]),
  q_path("a_crate::a::b", ["super", "f"], 2, ["item a_crate::a::f"]), q_path("a_crate::a::b", ["super", "super", "a"], 3, ["module a_crate::a"]),
  q_path("a_crate::a", ["crate", "a", "b", "g"], 4, ["item a_crate::a::b::g"]), q_path("a_crate::a", ["self", "b"], 2, ["module a_crate::a::b"]),
  q_path("a_crate", ["super"], None, []), q_path("a_crate", ["nosuch"], None, []), q_path("a_crate", ["private"], 1, ["item a_crate::private"]),
  q_path("b_crate", ["a_crate", "private"], 1, ["module a_crate => a_crate"]), q_path("b_crate", ["a_crate", "a", "f"], 3, ["item a_crate::a::f => a_crate::a::f"]),
  q_path("b_crate", ["a_crate", "hidden_mod", "q"], 1, ["module a_crate => a_crate"]), q_path("a_crate", ["hidden_mod", "q"], 2, ["item a_crate::hidden_mod::q"]),
  q_path("a_crate", ["a_crate", "a", "f"], 3, ["item a_crate::a::f"]),
  q_member("a_crate", "private", ["item a_crate::private"]), q_member("a_crate", "private", [], frm="b_crate"), q_member("a_crate", "private", ["item a_crate::private"], frm="b_crate", ign=True)]),
 case("visibility restrictions", [A, B], {"a/lib.rs": "pub mod m { pub(crate) fn hidden() {} pub fn shown() {} }\nmod p { pub fn q() {} }\n", "b/lib.rs": ""}, [
  q_path("a_crate", ["m", "hidden"], 2, ["item a_crate::m::hidden"]), q_path("b_crate", ["a_crate", "m", "hidden"], 2, ["module a_crate::m => a_crate::m"]),
  q_path("b_crate", ["a_crate", "m", "shown"], 3, ["item a_crate::m::shown => a_crate::m::shown"]), q_path("b_crate", ["a_crate", "p"], 1, ["module a_crate => a_crate"]),
  q_member("a_crate::m", "hidden", ["item a_crate::m::hidden"], frm="b_crate", ign=True), q_member("a_crate::m", "hidden", [], frm="b_crate")]),
 case("explicit imports and aliases", [A, B], {"a/lib.rs": "pub mod m { pub fn f() {} pub struct T; }\nuse m::f as g;\nuse m::T;\nuse self::m::f as h;\nuse crate::m as mm;\npub use m::T as Pub;\n", "b/lib.rs": ""}, [
  q_member("a_crate", "g", ["item a_crate::m::f"]), q_member("a_crate", "T", ["item a_crate::m::T"]), q_member("a_crate", "h", ["item a_crate::m::f"]), q_member("a_crate", "mm", ["module a_crate::m"]), q_member("a_crate", "f", []),
  q_member("a_crate", "g", [], frm="b_crate"), q_member("a_crate", "Pub", ["item a_crate::m::T"], frm="b_crate"), q_path("a_crate", ["mm", "f"], 2, ["item a_crate::m::f"]), q_path("a_crate", ["g"], 1, ["item a_crate::m::f"])]),
 case("re-exports cross the crate boundary", [A, B, Cc], {"a/lib.rs": "pub mod a { pub fn f() {} pub mod inner { pub fn z() {} } }\n", "b/lib.rs": "pub use a_crate::a::f;\npub use a_crate::a as alias;\npub use a_crate::a::inner::z as zed;\n", "c/lib.rs": ""}, [
  q_path("c_crate", ["b_crate", "f"], 2, ["item a_crate::a::f => a_crate::a::f"]), q_path("c_crate", ["b_crate", "alias", "inner", "z"], 4, ["item a_crate::a::inner::z => a_crate::a::inner::z"]),
  q_path("c_crate", ["b_crate", "zed"], 2, ["item a_crate::a::inner::z => a_crate::a::inner::z"]), q_member("b_crate", "f", ["item a_crate::a::f => a_crate::a::f"], frm="c_crate"), q_member("b_crate", "alias", ["module a_crate::a => a_crate::a"], frm="c_crate")]),
 case("a cut edge marks the foreign path", [A, B, Cc], {"a/lib.rs": "pub mod a { pub fn f() {} }\n", "b/lib.rs": "pub use a_crate::a::f;\n", "c/lib.rs": ""}, [
  q_path("c_crate", ["b_crate", "f"], 2, ["item a_crate::a::f => a_crate::a::f cut"]), q_member("b_crate", "f", ["item a_crate::a::f => a_crate::a::f cut"], frm="c_crate")], cut=["a_crate::a::f"]),
 case("an api leaf is addressed through its re-exporting crate", [A, B, Cc], {"a/lib.rs": "pub mod a { pub fn f() {} }\n", "b/lib.rs": "pub use a_crate::a::f;\n", "c/lib.rs": ""}, [
  q_path("c_crate", ["b_crate", "f"], 2, ["item a_crate::a::f => b_crate::f"]), q_member("b_crate", "f", ["item a_crate::a::f"], frm="c_crate")], api=["a_crate::a::f"]),
 case("globs, shadowing and foreign globs", [A, B, Cc], {"a/lib.rs": "pub mod m { pub fn f() {} pub fn g() {} pub mod n {} }\npub use m::*;\npub fn f() {}\n", "b/lib.rs": "pub use a_crate::m::*;\n", "c/lib.rs": ""}, [
  q_member("a_crate", "f", ["item a_crate::f"]), q_member("a_crate", "g", ["item a_crate::m::g"]), q_member("a_crate", "n", ["module a_crate::m::n"]), q_member("a_crate", "nothing", []),
  q_path("c_crate", ["b_crate", "g"], 2, ["item a_crate::m::g => a_crate::m::g"]), q_path("c_crate", ["b_crate", "n"], 2, ["module a_crate::m::n => a_crate::m::n"]), q_path("c_crate", ["a_crate", "g"], 2, ["item a_crate::m::g => a_crate::g"])]),
 case("glob cycles terminate", [A], {"a/lib.rs": "pub mod a { pub use super::b::*; pub fn fa() {} }\npub mod b { pub use super::a::*; pub fn fb() {} }\n"}, [
  q_member("a_crate::a", "fb", ["item a_crate::b::fb"]), q_member("a_crate::b", "fa", ["item a_crate::a::fa"]), q_member("a_crate::b", "zz", []), q_member("a_crate::a", "fa", ["item a_crate::a::fa"]),
  q_path("a_crate::a", ["fb"], 1, ["item a_crate::b::fb"])]),
 case("external imports and macros", [A], {"a/lib.rs": "use std::collections::HashMap;\nuse serde::Serialize as Ser;\nmake!(Alpha, Beta);\npub mod m {}\nmacro_rules! decl { () => { pub fn made() {} pub struct Made; } }\ndecl!();\npub mod plain { pub fn x() {} }\n"}, [
  q_member("a_crate", "HashMap", ["item a_crate::HashMap"]), q_member("a_crate", "Ser", ["item a_crate::Ser"]), q_member("a_crate", "Alpha", ["item a_crate::Alpha"]), q_member("a_crate", "Beta", ["item a_crate::Beta"]),
  q_member("a_crate", "Gamma", []), q_member("a_crate", "made", ["item a_crate::made"]), q_member("a_crate", "Made", ["item a_crate::Made"]), q_opaque("a_crate", True), q_opaque("a_crate::m", False), q_opaque("a_crate::plain", False)]),
 case("opaque follows glob imports", [A], {"a/lib.rs": "pub mod filled { gen!(Z); }\npub mod user { pub use super::filled::*; }\npub mod clean { pub fn c() {} }\npub mod viaclean { pub use super::clean::*; }\n"}, [
  q_opaque("a_crate::filled", True), q_opaque("a_crate::user", True), q_opaque("a_crate::clean", False), q_opaque("a_crate::viaclean", False), q_member("a_crate::user", "Z", ["item a_crate::filled::Z"])]),
 case("path attributes, missing files and inline path scope", [A], {"a/lib.rs": "#[path = \"x/y.rs\"]\npub mod m;\nmod z;\npub mod inl { #[path = \"w.rs\"] pub mod q; }\n#[path = \"nope.rs\"]\npub mod gone;\n", "a/x/y.rs": "pub fn deep() {}\npub mod sub { pub fn s() {} }\n", "a/inl/w.rs": "pub fn wq() {}\n"}, [
  q_path("a_crate", ["m", "deep"], 2, ["item a_crate::m::deep"]), q_path("a_crate", ["m", "sub", "s"], 3, ["item a_crate::m::sub::s"]), q_path("a_crate", ["z"], 1, ["module a_crate::z"]),
  q_path("a_crate", ["z", "x"], 1, ["module a_crate::z"]), q_path("a_crate", ["inl", "q", "wq"], 3, ["item a_crate::inl::q::wq"]), q_path("a_crate", ["gone"], 1, ["module a_crate::gone"]), q_path("a_crate", ["gone", "x"], 1, ["module a_crate::gone"])]),
 case("enum variants and nested items complete the leaf", [A, B, Cc], {"a/lib.rs": "pub mod m { pub enum E { V } pub fn f() {} }\nuse m::E::V;\nuse m::f::extra;\n", "b/lib.rs": "pub use a_crate::m::E::V;\n", "c/lib.rs": ""}, [
  q_member("a_crate", "V", ["item a_crate::m::E"]), q_member("a_crate", "extra", ["item a_crate::m::f"]), q_path("c_crate", ["b_crate", "V"], 2, ["item a_crate::m::E => a_crate::m::E::V"])]),
 case("offsets map to the innermost module", [A], {"a/lib.rs": "pub mod a { pub fn f() {} pub mod b { pub fn g() {} } }\nfn tail() {}\n"}, [
  q_node("a/lib.rs", "fn g", "a_crate::a::b"), q_node("a/lib.rs", "fn f", "a_crate::a"), q_node("a/lib.rs", "fn tail", "a_crate"), q_node("a/lib.rs", "pub mod a", "a_crate")]),
]
rep = lambda inner, outer, seg: None
merge = [
 {"name": "nothing found anywhere", "inner": None, "outer": None, "segment": "s", "expected": None},
 {"name": "the way in extends the path", "inner": None, "outer": {"ident": "x", "segs": ["a"], "cut": False}, "segment": "b", "expected": {"ident": "x", "segs": ["a", "b"], "cut": False}},
 {"name": "the way in keeps its cut flag when extended", "inner": None, "outer": {"ident": "x", "segs": [], "cut": True}, "segment": "b", "expected": {"ident": "x", "segs": ["b"], "cut": True}},
 {"name": "a path found inside wins and takes the cut of the way in", "inner": {"ident": "y", "segs": ["q"], "cut": False}, "outer": {"ident": "x", "segs": ["a"], "cut": True}, "segment": "b", "expected": {"ident": "y", "segs": ["q"], "cut": True}},
 {"name": "a cut inner path stays cut", "inner": {"ident": "y", "segs": ["q"], "cut": True}, "outer": {"ident": "x", "segs": ["a"], "cut": False}, "segment": "b", "expected": {"ident": "y", "segs": ["q"], "cut": True}},
 {"name": "an inner path without a way in is kept", "inner": {"ident": "y", "segs": ["q"], "cut": False}, "outer": None, "segment": "b", "expected": {"ident": "y", "segs": ["q"], "cut": False}},
 {"name": "both cut", "inner": {"ident": "y", "segs": [], "cut": True}, "outer": {"ident": "x", "segs": [], "cut": True}, "segment": "b", "expected": {"ident": "y", "segs": [], "cut": True}},
]
texts = [{"rep": {"ident": "a_crate", "segs": [], "cut": False}, "expected": "a_crate"}, {"rep": {"ident": "a_crate", "segs": ["m", "f"], "cut": True}, "expected": "a_crate::m::f"}]
keywords = [{"head": h, "expected": e} for h, e in [("crate", True), ("self", True), ("super", True), ("Self", False), ("a", False), ("", False), ("crate_", False)]]
universe = {"crates": [{"name": "my-crate", "ident": "my_crate"}, {"name": "plain", "ident": "plain"}, {"name": "a-b-c", "ident": "a_b_c"}], "overlay": {"x/lib.rs": "pub mod m {}"}, "reads": [{"path": "x/lib.rs", "expected": "pub mod m {}"}, {"path": "x/missing.rs", "expected": None}], "missingRoot": "ghost", "unknownRoot": "nobody"}
corpus = {"version": 1, "cases": cases, "merge": merge, "texts": texts, "keywords": keywords, "universe": universe}
rp = S.obj({"ident": S.str, "segs": S.strs(), "cut": S.bool})
orp = S.one(S.null, rp)
crate = S.obj({"name": S.str, "dir": S.str, "root": S.str})
query = S.obj({"op": S.enum("path", "member", "opaque", "nodeAt"), "at": S.str, "name": S.str, "path": S.strs(), "from": S.str, "ignoreVisibility": S.bool, "file": S.str, "needle": S.str, "expect": S.one(S.null, S.obj({"consumed": S.int, "hits": S.strs()})), "expectHits": S.strs(), "expectFlag": S.bool, "expectNode": S.str})
schema = S.obj({"version": S.const(1), "cases": S.arr(S.obj({"name": S.str, "crates": S.arr(crate, 1), "files": S.map(S.str), "cut": S.strs(), "api": S.strs(), "queries": S.arr(query, 1)}), 1),
 "merge": S.arr(S.obj({"name": S.str, "inner": orp, "outer": orp, "segment": S.str, "expected": orp}), 1), "texts": S.arr(S.obj({"rep": rp, "expected": S.str}), 1), "keywords": S.arr(S.obj({"head": S.str, "expected": S.bool}), 1),
 "universe": S.obj({"crates": S.arr(S.obj({"name": S.str, "ident": S.str}), 1), "overlay": S.map(S.str), "reads": S.arr(S.obj({"path": S.str, "expected": S.one(S.null, S.str)}), 1), "missingRoot": S.str, "unknownRoot": S.str})})
ORACLE = r'''
def ident(name):
    return name.replace("-", "_")

TOK = re.compile(r'#\[[^\]]*\]|"(?:[^"\\]|\\.)*"|[A-Za-z_]\w*|::|[{}()\[\];,*=!<>&\-\.\']|\S')
DECL = ("fn", "struct", "enum", "const", "static", "trait", "type", "union")
PRELUDE = {"Some", "None", "Ok", "Err", "Self", "Vec", "String", "Option", "Result", "Box", "Default", "Clone", "Copy"}


class Mod:
    n = 0

    def __init__(self, crate, name, parent, vis, file, region):
        Mod.n += 1
        self.id, self.crate, self.name, self.parent, self.vis, self.file, self.region = Mod.n, crate, name, parent, vis, file, region
        self.children, self.items, self.weak, self.uses, self.opaque = {}, {}, set(), [], False


def group_end(toks, i):
    depth = 0
    while i < len(toks):
        if toks[i][0] in "({[":
            depth += 1
        elif toks[i][0] in ")}]":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return len(toks) - 1


def parse_tree(toks, i, prefix):
    leaves = []
    segs = list(prefix)
    while True:
        if toks[i][0] == "{":
            i += 1
            while toks[i][0] != "}":
                sub, i = parse_tree(toks, i, segs)
                leaves += sub
                if toks[i][0] == ",":
                    i += 1
            return leaves, i + 1
        if toks[i][0] == "*":
            return leaves + [(segs, None, True)], i + 1
        segs.append(toks[i][0])
        i += 1
        if i < len(toks) and toks[i][0] == "::":
            i += 1
            continue
        alias = None
        if i < len(toks) and toks[i][0] == "as":
            alias = toks[i + 1][0]
            i += 2
        return leaves + [(segs, alias, False)], i


class Parser:
    def __init__(self, files, crate_idents):
        self.files, self.nodes_by_file, self.macros, self.idents = files, {}, {}, crate_idents
        self.mounted = {}

    def load(self, crate, path, name, parent, vis, file_node_text=None):
        text = self.files.get(path)
        if text is None:
            return Mod(crate, name, parent, vis, path, (0, 0))
        node = Mod(crate, name, parent, vis, path, (0, len(text)))
        self.nodes_by_file.setdefault(path, []).append(node)
        toks = [(m.group(0), m.start()) for m in TOK.finditer(text)]
        self.items(node, toks, 0, text, path, posix_dir(path), True, posix_dir(path))
        return node

    def items(self, node, toks, i, text, path, filedir, in_file, base):
        while i < len(toks) and toks[i][0] != "}":
            attrs = []
            while toks[i][0].startswith("#["):
                attrs.append(toks[i][0])
                i += 1
            vis = ""
            if toks[i][0] == "pub":
                vis = "pub"
                i += 1
                if toks[i][0] == "(":
                    end = group_end(toks, i)
                    vis = "pub(" + "".join(t[0] for t in toks[i + 1:end]) + ")"
                    i = end + 1
            word = toks[i][0]
            if word == "mod":
                name = toks[i + 1][0]
                pathattr = None
                for a in attrs:
                    m = re.search(r'path\s*=\s*"([^"]*)"', a)
                    if m:
                        pathattr = m.group(1)
                rvis = "pub" if vis == "pub" else vis
                if toks[i + 2][0] == "{":
                    end = group_end(toks, i + 2)
                    child = Mod(node.crate, name, node, rvis, path, (toks[i + 2][1] + 1, toks[end][1]))
                    self.nodes_by_file[path].append(child)
                    node.children.setdefault(name, []).append(child)
                    self.items(child, toks, i + 3, text, path, filedir, False, norm(base + "/" + (pathattr or name)))
                    i = end + 1
                else:
                    node.children.setdefault(name, [])
                    if pathattr is None:
                        node.children[name].append(Mod(node.crate, name, node, rvis, path, (0, 0)))
                    else:
                        rel = norm((filedir if in_file else base) + "/" + pathattr)
                        node.children[name].append(self.load(node.crate, rel, name, node, rvis))
                    i += 3
                continue
            if word == "use":
                end = i
                while toks[end][0] != ";":
                    end += 1
                leaves, _ = parse_tree(toks[:end], i + 1, [])
                for segs, alias, glob in leaves:
                    node.uses.append({"vis": vis, "path": segs, "binding": alias or segs[-1], "glob": glob, "id": (path, toks[i][1], len(node.uses), node.id)})
                i = end + 1
                continue
            if word == "macro_rules" and toks[i + 1][0] == "!":
                name = toks[i + 2][0]
                end = group_end(toks, i + 3)
                body = text[toks[i + 3][1]:toks[end][1] + 1]
                self.macros[(path, name)] = re.findall(r"\b(?:static|const|fn|struct|enum|type|trait|mod)\s+([A-Za-z_]\w*)", body)
                node.items[name] = ""
                i = end + 1
                continue
            if word in DECL:
                node.items[toks[i + 1][0]] = vis
                i = skip(toks, i + 2)
                continue
            if re.match(r"[A-Za-z_]\w*$", word) and toks[i + 1][0] == "!" and toks[i + 2][0] in "({[":
                end = group_end(toks, i + 2)
                body = text[toks[i][1]:toks[end][1] + 1]
                node.opaque = True
                for t in re.findall(r"[A-Za-z_]\w*", text[toks[i + 2][1]:toks[end][1] + 1]):
                    if re.match(r"[A-Z][A-Za-z0-9_]*$", t) and t not in PRELUDE:
                        node.weak.add(t)
                for d in self.macros.get((path, word), []):
                    node.weak.add(d)
                i = end + 1
                if i < len(toks) and toks[i][0] == ";":
                    i += 1
                continue
            i += 1
        return i


def skip(toks, i):
    while i < len(toks):
        if toks[i][0] == ";":
            return i + 1
        if toks[i][0] in "({[":
            end = group_end(toks, i)
            if toks[i][0] == "{":
                return end + 1
            i = end + 1
            continue
        i += 1
    return i


def posix_dir(p):
    return p.rsplit("/", 1)[0] if "/" in p else "."


def norm(p):
    out = []
    for part in p.split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            out.pop()
        else:
            out.append(part)
    return "/".join(out)


def modpath(n):
    names = []
    c = n
    while c.parent is not None:
        names.insert(0, c.name)
        c = c.parent
    return "::".join([n.crate] + names)


def rep_text(r):
    return "::".join([r[0]] + list(r[1])) + (" cut" if r[2] else "")


def show(h):
    kind, node, name, rep = h
    base = "module " + modpath(node) if kind == "module" else "item " + modpath(node) + "::" + name
    return base + (" => " + rep_text(rep) if rep else "")


def merge_rep(inner, outer, seg):
    if inner:
        return (inner[0], inner[1], True) if outer and outer[2] and not inner[2] else inner
    return (outer[0], tuple(outer[1]) + (seg,), outer[2]) if outer else None


class Resolver:
    def __init__(self, roots, crates, cut, api):
        self.roots, self.crates, self.cut, self.api, self.ign = roots, crates, set(cut), set(api), False

    def visible(self, vis, owner, frm):
        return vis == "pub" or self.ign or frm.crate == owner.crate

    def member(self, node, name, frm, stack=frozenset()):
        key = (node.id, name, frm.crate == node.crate, self.ign)
        if key in stack:
            return []
        stack = stack | {key}
        out = []
        for c in node.children.get(name, []):
            if self.visible(c.vis, node, frm):
                out.append(("module", c, None, None))
        if name in node.items and self.visible(node.items[name], node, frm):
            out.append(("item", node, name, None))
        for u in node.uses:
            if u["glob"] or u["binding"] != name or not self.visible(u["vis"], node, frm):
                continue
            hits = self.leaf(node, u, stack)
            if not hits and self.external(node, u):
                out.append(("item", node, name, None))
                continue
            cut = "::".join(u["path"]) in self.cut
            api = "::".join(u["path"]) in self.api
            for h in hits:
                if api:
                    out.append((h[0], h[1], h[2], None))
                elif cut and h[3]:
                    out.append((h[0], h[1], h[2], (h[3][0], h[3][1], True)))
                else:
                    out.append(h)
        shadowed = {h[0] for h in out}
        for u in node.uses:
            if not u["glob"] or not self.visible(u["vis"], node, frm):
                continue
            cut = "::".join(u["path"]) in self.cut
            for raw in self.leaf(node, u, stack):
                if raw[0] != "module":
                    continue
                target = (raw[0], raw[1], raw[2], (raw[3][0], raw[3][1], True) if cut and raw[3] else raw[3])
                for sub in self.member(target[1], name, frm, stack):
                    if sub[0] not in shadowed:
                        out.append((sub[0], sub[1], sub[2], merge_rep(sub[3], target[3], name)))
        if not out and name in node.weak:
            out.append(("item", node, name, None))
        return out

    def external(self, node, u):
        head = u["path"][0]
        return head not in ("crate", "self", "super") and head not in self.crates and head not in node.children and head not in node.items and not any(o["binding"] == head for o in node.uses)

    def leaf(self, node, u, stack):
        key = ("leaf", u["id"])
        if key in stack:
            return []
        res = self.path(node, u["path"], stack | {key})
        if not res:
            return []
        hits, consumed = res
        if consumed == len(u["path"]):
            return hits
        rest = u["path"][consumed:]
        return [(h[0], h[1], h[2], (h[3][0], tuple(h[3][1]) + tuple(rest), h[3][2]) if h[3] else None) for h in hits if h[0] == "item"]

    def path(self, node, path, stack=frozenset()):
        head = path[0]
        consumed = 1
        if head == "crate":
            hits = [("module", self.roots[node.crate], None, None)]
        elif head == "self":
            hits = [("module", node, None, None)]
        elif head == "super":
            cur, k = node, 0
            while k < len(path) and path[k] == "super" and cur is not None:
                cur, k = cur.parent, k + 1
            consumed = k
            hits = [("module", cur, None, None)] if cur is not None else []
        else:
            hits = self.member(node, head, node, stack)
            if not hits and head in self.crates:
                hits = [("module", self.roots[head], None, None if head == node.crate else (head, (), False))]
        if not hits:
            return None
        while consumed < len(path):
            nxt = []
            for h in hits:
                if h[0] != "module":
                    continue
                for sub in self.member(h[1], path[consumed], node, stack):
                    nxt.append((sub[0], sub[1], sub[2], merge_rep(sub[3], h[3], path[consumed])))
            if not nxt:
                break
            hits, consumed = nxt, consumed + 1
        return hits, consumed

    def opaque(self, node, seen=frozenset()):
        if node.id in seen:
            return False
        if node.opaque:
            return True
        for u in node.uses:
            if u["glob"] and any(h[0] == "module" and self.opaque(h[1], seen | {node.id}) for h in self.leaf(node, u, frozenset())):
                return True
        return False


def addr(roots, ref):
    parts = ref.split("::")
    n = roots[parts[0]]
    for p in parts[1:]:
        n = n.children[p][0]
    return n


def innermost(nodes, offset):
    best = None
    for n in nodes:
        if n.region[0] <= offset <= n.region[1] and (best is None or n.region[1] - n.region[0] <= best.region[1] - best.region[0]):
            best = n
    return best


for case in corpus["cases"]:
    idents = {ident(c["name"]) for c in case["crates"]}
    parser = Parser(case["files"], idents)
    roots = {}
    for c in case["crates"]:
        roots[ident(c["name"])] = parser.load(ident(c["name"]), c["root"], "", None, "pub")
    res = Resolver(roots, idents, case["cut"], case["api"])
    for q in case["queries"]:
        label = case["name"] + " / " + q["op"] + " " + json.dumps(q["path"] or q["name"] or q["needle"])
        if q["op"] == "path":
            r = res.path(addr(roots, q["at"]), q["path"])
            got = None if r is None else {"consumed": r[1], "hits": sorted(show(h) for h in r[0])}
            want = q["expect"] and {"consumed": q["expect"]["consumed"], "hits": sorted(q["expect"]["hits"])}
            ok = got == want
        elif q["op"] == "member":
            res.ign = q["ignoreVisibility"]
            node = addr(roots, q["at"])
            got = sorted(show(h) for h in res.member(node, q["name"], addr(roots, q["from"] or q["at"])))
            res.ign = False
            ok = got == sorted(q["expectHits"])
        elif q["op"] == "opaque":
            got = res.opaque(addr(roots, q["at"]))
            ok = got == q["expectFlag"]
        else:
            text = case["files"][q["file"]]
            node = innermost(parser.nodes_by_file[q["file"]], text.index(q["needle"]))
            got = modpath(node)
            ok = got == q["expectNode"]
        if not ok:
            failures.append({"case": label, "got": got})

for row in corpus["merge"]:
    t = lambda r: None if r is None else (r["ident"], tuple(r["segs"]), r["cut"])
    got = merge_rep(t(row["inner"]), t(row["outer"]), row["segment"])
    if got != t(row["expected"]):
        failures.append({"merge": row["name"], "got": got})
for row in corpus["texts"]:
    got = "::".join([row["rep"]["ident"]] + row["rep"]["segs"])
    if got != row["expected"].replace(" cut", ""):
        failures.append({"text": row["expected"], "got": got})
for row in corpus["keywords"]:
    if (row["head"] in ("crate", "self", "super")) != row["expected"]:
        failures.append({"keyword": row["head"]})
'''
oracle = lib.PY_HEAD.replace("{doc}", "Independent Python model of module-level name resolution (explicit imports, globs, re-exports, cuts, visibility, macros, path mounts) over the corpus.") + ORACLE + lib.PY_TAIL
body = '''const node=(u:Universe,ref:string):ModNode=>{const[head,...rest]=ref.split("::");let current=u.root(head!)!;for(const name of rest)current=current.children.get(name)![0]!;return current;};
const modPath=(n:ModNode):string=>{const names:string[]=[];for(let c:ModNode|null=n;c&&c.parent;c=c.parent)names.unshift(c.name);return [n.crate.ident,...names].join("::");};
const repShow=(r:Rep|null):string=>r?" => "+repText(r)+(r.cut?" cut":""):"";
const show=(h:Hit):string=>h.kind==="module"?"module "+modPath(h.node)+repShow(h.rep):"item "+modPath(h.node)+"::"+h.name+repShow(h.rep);
const build=(row:any):Universe=>{
 const u=new Universe("/virtual",new Map(Object.entries<string>(row.files)));for(const c of row.crates)u.addCrate(c.name,c.dir,c.root);
 u.cutEdge=b=>row.cut.includes(b.leaf.path.join("::"));u.apiLeaf=b=>row.api.includes(b.leaf.path.join("::"));for(const c of row.crates)u.root(c.name.replaceAll("-","_"));return u;
};
for(const row of corpus.cases)test("resolution: "+row.name,()=>{
 const u=build(row);
 for(const q of row.queries){
  const label=q.op+" "+JSON.stringify(q.path.length?q.path:q.name||q.needle);
  if(q.op==="path"){const r=u.resolvePath(node(u,q.at),q.path);expect(r===null?null:{consumed:r.consumed,hits:r.hits.map(show).sort()},label).toEqual(q.expect===null?null:{consumed:q.expect.consumed,hits:[...q.expect.hits].sort()});}
  else if(q.op==="member"){const at=node(u,q.at),from=node(u,q.from||q.at),hits=q.ignoreVisibility?u.memberIgnoringVisibility(at,q.name,from):u.member(at,q.name,from);expect(hits.map(show).sort(),label).toEqual([...q.expectHits].sort());}
  else if(q.op==="opaque")expect(u.opaqueModule(node(u,q.at)),label).toBe(q.expectFlag);
  else{const text=row.files[q.file]!,found=u.nodeAt(q.file,text.indexOf(q.needle));expect(found?modPath(found):null,label).toBe(q.expectNode);}
 }
 expect(u.ignoreVisibility).toBe(false);
});
for(const row of corpus.merge)test("mergeRep: "+row.name,()=>{expect(mergeRep(row.inner,row.outer,row.segment)).toEqual(row.expected);});
for(const row of corpus.texts)test("repText of "+row.expected,()=>{expect(repText(row.rep)).toBe(row.expected.replace(" cut",""));});
for(const row of corpus.keywords)test("isKeywordHead("+JSON.stringify(row.head)+")",()=>{expect(isKeywordHead(row.head)).toBe(row.expected);});
test("a universe names crates by extern identifier, reads through its overlay and loads roots lazily",()=>{
 const u=new Universe("/virtual",new Map(Object.entries<string>(corpus.universe.overlay)));
 for(const c of corpus.universe.crates)expect(u.addCrate(c.name,"x","x/lib.rs").ident).toBe(c.ident);
 expect([...u.crates.keys()].sort()).toEqual(corpus.universe.crates.map(c=>c.ident).sort());expect(u.loadedRoots()).toEqual([]);
 for(const r of corpus.universe.reads)expect(u.read(r.path)).toBe(r.expected);
 expect(u.root(corpus.universe.unknownRoot)).toBeNull();u.addCrate(corpus.universe.missingRoot,"nowhere","nowhere/lib.rs");expect(u.root(corpus.universe.missingRoot)).toBeNull();expect(u.loadedRoots()).toEqual([]);
 const loaded=u.root("my_crate")!;expect(loaded).not.toBeNull();expect(u.root("my_crate")).toBe(loaded);expect(u.loadedRoots()).toEqual(["my_crate"]);
});'''
ts = lib.ts_wrap(UNIT, "🔍️ Contract: the module-level resolver builds the module tree of every carved crate from its `#[path]` mounts, resolves `use` leaves and paths through explicit imports, globs and re-exports with visibility, records the foreign path (and cut flag) where a path crosses a crate boundary and reports macro-opaque modules; the corpus is language-agnostic and an independent Python model resolves every query again.",
 'import {Universe,isKeywordHead,mergeRep,repText,type Hit,type ModNode,type Rep} from "@@' + UNIT + '/🟦.ts@@";', body)
lib.write_unit(UNIT, corpus, schema, ts, oracle)
