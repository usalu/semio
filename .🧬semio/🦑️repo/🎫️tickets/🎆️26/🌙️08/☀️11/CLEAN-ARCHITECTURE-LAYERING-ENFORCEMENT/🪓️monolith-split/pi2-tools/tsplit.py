#!/usr/bin/env python3
"""✂️ Splits one oversized test module into pieces: the module body is cut into items (with the comments before each item), tests go to pieces, helpers used by more than one piece form a shared `core` fragment that every piece `include!`s, helpers used by one piece stay in that piece.
Library: `Module.load(path, inline_mod)`, `plan_pieces(module, assign)`, `render(...)`."""
import re, collections, os
from lib import *
import rsparse as P
from rs import blank

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
TEST_ATTR = re.compile(r"#\[\s*(?:[A-Za-z_][\w]*::)*(?:async_)?test\s*[\]\(]|#\[\s*cfg_attr\(\s*test\s*,\s*test\s*\)\s*\]")
DEF_IN_MACRO = re.compile(r"\b(?:static|const|struct|enum|fn|type|mod|trait|union)\s+([A-Za-z_]\w*)")
REGION = re.compile(r"[ \t]*//#(end)?region[^\n]*\n?")


class Item:
    __slots__ = ("origin", "origin_file", "emb", "src_kind", "idx", "kind", "name", "defs", "attrs", "text", "body", "is_test", "has_tests", "ids", "lines", "bytes", "deps", "self_ty", "vis", "src", "gap")

    def __repr__(self):
        return f"<{self.idx} {self.kind} {self.name} t={self.is_test}>"


def impl_self(clean_head):
    hh = clean_head.split("{", 1)[0]
    if hh.lstrip().startswith("impl"):
        hh = hh.replace("impl", "", 1)
        hh = re.sub(r"^\s*<[^{}]*?>\s*(?=[A-Za-z_(&!])", " ", hh, count=1) if hh.lstrip().startswith("<") else hh
    hh = hh.split(" where ")[0]
    if " for " in hh:
        hh = hh.split(" for ", 1)[1]
    hh = re.sub(r"<.*", "", hh)
    m = IDENT.findall(hh)
    return m[-1] if m else None


INC = re.compile(r'include!\(\s*(?:concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\)|"([^"]+)")\s*\)\s*;')
MANIFEST_DIR = PLUG + "/" + PKGDIR


def include_target(base_file, body):
    m = INC.search(body)
    if not m:
        return None
    if m.group(1):
        return os.path.normpath(MANIFEST_DIR + "/" + m.group(1).lstrip("/"))
    return os.path.normpath(os.path.join(os.path.dirname(base_file), m.group(2)))


EMB_RX = re.compile(r'(include_str!|include_bytes!)\(\s*(?:concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\)|"([^"]+)")\s*\)')


def embeds_from_text(txt, here):
    out = {}
    clean = blank(txt)
    for m in EMB_RX.finditer(txt):
        if clean[m.start():m.start() + len(m.group(1))] != m.group(1):
            continue
        tgt = os.path.normpath(MANIFEST_DIR + "/" + m.group(2).lstrip("/")) if m.group(2) else os.path.normpath(os.path.join(os.path.dirname(here), m.group(3)))
        if os.path.exists(tgt):
            out[tgt] = os.path.getsize(tgt)
    return out


def embeds_of(body, base_file, included_rel=None):
    """{path: bytes} of the files an item embeds with include_str!/include_bytes! (also inside the file an include item pulls in)"""
    out = {}
    def scan(txt, here):
        clean = blank(txt)
        for m in EMB_RX.finditer(txt):
            if clean[m.start():m.start() + len(m.group(1))] != m.group(1):
                continue
            tgt = os.path.normpath(MANIFEST_DIR + "/" + m.group(2).lstrip("/")) if m.group(2) else os.path.normpath(os.path.join(os.path.dirname(here), m.group(3)))
            if os.path.exists(tgt):
                out[tgt] = os.path.getsize(tgt)
    if base_file:
        scan(body, base_file)
    if included_rel:
        f = PLUG + "/" + included_rel
        if os.path.exists(f):
            scan(read(f), f)
    return out


def parse_items(text, lo, hi, src="main", base_file=None, expand=True):
    clean = blank(text)
    stmts = P.statements(text, clean, lo, hi)
    items, prev = [], lo
    for i, s in enumerate(stmts):
        it = Item()
        it.idx, it.src = i, src
        it.kind, it.name, it.vis, it.attrs = s.kind, s.name, s.vis, s.attrs or ""
        it.gap = text[prev:s.start]
        it.body = text[s.start:s.end]
        it.text = text[prev:s.end]
        prev = s.end
        cbody = clean[s.start:s.end]
        it.is_test = s.kind == "fn" and bool(TEST_ATTR.search(it.attrs))
        it.has_tests = it.is_test or (s.kind in ("mod", "other") and bool(TEST_ATTR.search(it.body)))
        it.ids = set(IDENT.findall(cbody))
        it.defs = set()
        if s.name and s.kind not in ("use", "impl"):
            it.defs.add(s.name)
        if s.kind == "other":
            it.defs |= set(DEF_IN_MACRO.findall(cbody))
        it.self_ty = impl_self(cbody[:800]) if s.kind == "impl" else None
        it.lines = it.body.count("\n") + 1
        it.bytes = len(it.body.encode("utf8"))
        it.deps = set()
        it.src_kind = None
        it.origin = "main"
        it.origin_file = base_file
        it.src = src
        if expand and base_file:
            inner = it.body if it.kind == "other" else (text[s.body[0]:s.body[1]] if (it.kind == "mod" and s.inline) else "")
            tgt = include_target(base_file, inner) if inner and "include!" in cbody else None
            if tgt is None and it.kind == "mod" and not s.inline:
                pm = re.search(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', it.attrs)
                if pm:
                    tgt = os.path.normpath(os.path.join(os.path.dirname(base_file), pm.group(1)))
                    it.src_kind = "pathmod"
            if tgt and os.path.exists(tgt):
                itx = read(tgt)
                icl = blank(itx)
                it.ids |= set(IDENT.findall(icl))
                for s2 in P.statements(itx, icl, 0, len(itx)):
                    if s2.name and s2.kind not in ("use", "impl"):
                        it.defs.add(s2.name)
                    if s2.kind == "other":
                        it.defs |= set(DEF_IN_MACRO.findall(icl[s2.start:s2.end]))
                if TEST_ATTR.search(itx):
                    it.has_tests = True
                it.lines += itx.count("\n") + 1
                it.bytes += len(itx.encode("utf8"))
                it.src = rel(tgt)
                if it.kind == "other":
                    it.src_kind = "include"
        it.emb = embeds_of(it.body, base_file or src, it.src if it.src_kind else None)
        items.append(it)
    tail = text[prev:hi]
    return items, tail


class Module:
    """pre = text before the item area, items, tail = text after the last item, post = text after the area"""

    def __init__(self, path, pre, items, tail, post, wrapper):
        self.path, self.pre, self.items, self.tail, self.post, self.wrapper = path, pre, items, tail, post, wrapper
        self.link()

    @staticmethod
    def load(path, inline_mod=None):
        text = read(path)
        clean = blank(text)
        if inline_mod:
            sts = P.statements(text, clean, 0, len(text))
            m = next(s for s in sts if s.kind == "mod" and s.inline and s.name == inline_mod)
            lo, hi = m.body
            items, tail = parse_items(text, lo, hi, base_file=path)
            wrapper = (text[m.start:lo], text[hi:m.end])
            outer = [x for x in sts if x is not m]
            pre = text[:m.start]
            post = text[m.end:]
            return Module(path, pre, items, tail, post, wrapper)
        items, tail = parse_items(text, 0, len(text), base_file=path)
        return Module(path, "", items, tail, "", None)

    def link(self):
        macros = {it.name for it in self.items if it.kind == "macro_rules!" and it.name}
        for it in self.items:
            if it.kind == "other" and not it.src_kind:
                m = re.match(r"\s*(?:[A-Za-z_]\w*::)*([A-Za-z_]\w*)!", it.body)
                if m and m.group(1) in macros:
                    it.defs |= {x for x in it.ids if x[:1].isupper() and x != m.group(1)}
        by = collections.defaultdict(list)
        for it in self.items:
            for d in it.defs:
                by[d].append(it.idx)
        for it in self.items:
            it.deps = set()
            for n in it.ids:
                for j in by.get(n, ()):
                    if j != it.idx and self.items[j].kind != "use":
                        it.deps.add(j)
        for it in self.items:
            if it.kind == "impl" and it.self_ty:
                for j in by.get(it.self_ty, ()):
                    self.items[j].deps.add(it.idx)
                    it.deps.add(j)

    def closure(self, roots):
        seen, st = set(), list(roots)
        while st:
            x = st.pop()
            for d in self.items[x].deps:
                if d not in seen:
                    seen.add(d)
                    st.append(d)
        return seen


def strip_unbalanced_regions(text):
    stack, bad = [], set()
    marks = list(REGION.finditer(text))
    for k, m in enumerate(marks):
        if m.group(1):
            if stack:
                stack.pop()
            else:
                bad.add(k)
        else:
            stack.append(k)
    bad |= set(stack)
    if not bad:
        return text
    out, last = [], 0
    for k, m in enumerate(marks):
        if k in bad:
            out.append(text[last:m.start()])
            last = m.end()
    out.append(text[last:])
    return "".join(out)


def plan(module, assign, always_core=()):
    """assign: {test item idx: piece name}. Returns (core idx set, {piece: (private helper idxs, tests)}, unused)."""
    pieces = collections.defaultdict(list)
    for i, p in assign.items():
        pieces[p].append(i)
    reach = {}
    for p, tests in pieces.items():
        reach[p] = module.closure(tests) | set(tests)
    users = collections.defaultdict(set)
    for p, r in reach.items():
        for x in r:
            if not module.items[x].has_tests:
                users[x].add(p)
    core, private = set(always_core), collections.defaultdict(set)
    for x, ps in users.items():
        if len(ps) >= 2 or x in always_core:
            core.add(x)
        else:
            private[next(iter(ps))].add(x)
    allh = {it.idx for it in module.items if not it.has_tests and it.kind != "use"}
    unused = allh - set(users)
    return core, private, unused


def cluster(module, tests, universal, budget, extra_fixed=None, max_files=22):
    """greedy affinity clustering of test items into pieces: a piece's size = tests + helper items it needs outside `universal` + the files they embed; returns pieces (dicts with tests, helpers, bytes)"""
    its = module.items
    S = {t: {x for x in module.closure([t]) if not its[x].has_tests} - universal for t in tests}
    def emb_of(idxs):
        e = {}
        for x in idxs:
            e.update(its[x].emb)
        return e
    cost = lambda s: sum(its[x].bytes for x in s)
    base_emb = emb_of(universal)
    order = sorted(tests, key=lambda t: -(its[t].bytes + cost(S[t])))
    pieces = []
    for t in order:
        best, bestd = None, None
        for p in pieces:
            e = dict(p["emb"]); e.update(its[t].emb); e.update(emb_of(S[t]))
            e = {k: v for k, v in e.items() if k not in base_emb}
            add = cost(S[t] - p["helpers"]) + its[t].bytes + sum(e.values()) - sum(p["emb"].values())
            if p["bytes"] + add <= budget and len(e) + len(base_emb) <= max_files:
                key = (add - its[t].bytes, p["bytes"])
                if bestd is None or key < bestd:
                    best, bestd = p, key
        if best is None:
            best = dict(tests=[], helpers=set(), bytes=0, emb={})
            pieces.append(best)
        e = dict(best["emb"]); e.update(its[t].emb); e.update(emb_of(S[t]))
        e = {k: v for k, v in e.items() if k not in base_emb}
        best["bytes"] += its[t].bytes + cost(S[t] - best["helpers"]) + sum(e.values()) - sum(best["emb"].values())
        best["emb"] = e
        best["helpers"] |= S[t]
        best["tests"].append(t)
    return pieces


# ----------------------------------------------------------------------------------------------------------------
# path rebasing and emission
# ----------------------------------------------------------------------------------------------------------------
REL_MACRO = re.compile(r'(include!|include_str!|include_bytes!)\(\s*"([^"]+)"\s*\)')
CONCAT_INC = re.compile(r'include!\(\s*concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\)\s*\)')
PATH_ATTR_RX = re.compile(r'(#\[\s*path\s*=\s*)"([^"]+)"(\s*\])')


def rebase(text, old_file, new_file):
    """literal relative paths of include macros and `#[path]` attributes written for old_file's directory, valid from new_file's directory; `include!(concat!(env!("CARGO_MANIFEST_DIR"), ..))` becomes a literal relative include"""
    clean = blank(text)
    old_dir, new_dir = os.path.dirname(old_file), os.path.dirname(new_file)
    out, last = [], 0
    edits = []
    for m in CONCAT_INC.finditer(text):
        if clean[m.start():m.start() + 8] != "include!":
            continue
        target = os.path.normpath(MANIFEST_DIR + "/" + m.group(1).lstrip("/"))
        edits.append((m.start(), m.end(), 'include!("' + os.path.relpath(target, new_dir) + '")'))
    for m in REL_MACRO.finditer(text):
        if clean[m.start():m.start() + len(m.group(1))] != m.group(1):
            continue
        if any(a <= m.start() < b for a, b, _ in edits):
            continue
        target = os.path.normpath(os.path.join(old_dir, m.group(2)))
        edits.append((m.start(), m.end(), m.group(1) + '("' + os.path.relpath(target, new_dir) + '")'))
    for m in PATH_ATTR_RX.finditer(text):
        if clean[m.start():m.start() + 2] != "#[":
            continue
        target = os.path.normpath(os.path.join(old_dir, m.group(2)))
        edits.append((m.start(), m.end(), m.group(1) + '"' + os.path.relpath(target, new_dir) + '"' + m.group(3)))
    if old_dir == new_dir and not any(CONCAT_INC.search(text[a:b]) for a, b, _ in edits):
        return text
    for a, b, rep in sorted(edits):
        out.append(text[last:a])
        out.append(rep)
        last = b
    out.append(text[last:])
    return "".join(out)


DOC_LINE = re.compile(r"^[ \t]*//![^\n]*\n?", re.M)


def lead_docs(module):
    """the `//!` lines at the start of the module body (they stay with every piece of an inline module)"""
    if not module.items:
        return ""
    gap = module.items[0].gap
    docs = "".join(m.group(0) for m in DOC_LINE.finditer(gap))
    return docs


def strip_lead_docs(module):
    if not module.items:
        return
    it = module.items[0]
    new_gap = DOC_LINE.sub("", it.gap)
    it.text = new_gap + it.body
    it.gap = new_gap


MERGE_BELOW = 6000


def fragments(module, units, always_core=()):
    """(universal core idx set, {frozenset(units): idx list} shared fragments, {unit: private idx list}); `units` maps unit name -> test item idxs"""
    names = list(units)
    cl = {u: {x for x in module.closure(units[u]) if not module.items[x].has_tests and module.items[x].kind != "use"} for u in names}
    allh = {it.idx for it in module.items if not it.has_tests and it.kind != "use"}
    sig = {}
    for x in allh:
        s = frozenset(u for u in names if x in cl[u])
        sig[x] = s
    unused = [x for x, s in sig.items() if not s]
    core = {x for x, s in sig.items() if (len(s) == len(names) and s) or x in always_core}
    shared = collections.defaultdict(list)
    private = collections.defaultdict(list)
    for x, s in sig.items():
        if x in core or not s:
            continue
        if len(s) == 1:
            private[next(iter(s))].append(x)
        else:
            shared[s].append(x)
    cost = lambda v: sum(module.items[x].bytes for x in v)
    small = sorted((s for s, v in shared.items() if cost(v) < MERGE_BELOW), key=lambda s: (len(s), cost(shared[s])))
    if unused:
        last = names[-1]
        for x in unused:
            private.setdefault(last, []).append(x)
    for s in small:
        if s not in shared:
            continue
        sup = [s2 for s2 in shared if s2 != s and s2 >= s and s2 not in small]
        if sup:
            tgt = min(sup, key=lambda s2: (cost(shared[s2]), len(s2)))
            shared[tgt] = list(shared[tgt]) + list(shared.pop(s))
        else:
            core |= set(shared.pop(s))
    for v in shared.values():
        v.sort()
    for v in private.values():
        v.sort()
    return core, dict(shared), dict(private)


def sibling(orig_path, suffix):
    d = os.path.dirname(orig_path)
    return os.path.join(os.path.dirname(d), os.path.basename(d) + suffix, os.path.basename(orig_path))


def items_text(module, idxs, new_file, indent=""):
    out = []
    for i in idxs:
        it = module.items[i]
        out.append(rebase(it.text, module.path, new_file))
    return "".join(out)


def emit(module, units, piece_dirs, docs_emoji, unit_docs, always_core=(), extra_header=""):
    """files = {path: text}. The original path becomes the core fragment; each unit becomes the piece file piece_dirs[unit]; shared fragments go to `<orig dir>-core-<k>`.
    inline module containers keep their wrapper (and `pre` text) in every piece."""
    orig = module.path
    uses = [it for it in module.items if it.kind == "use"]
    docs = lead_docs(module)
    strip_lead_docs(module)
    core, shared, private = fragments(module, units, always_core)
    files = {}
    frag_paths = {}
    for k, (sig, idxs) in enumerate(sorted(shared.items(), key=lambda kv: min(kv[1])), 1):
        frag_paths[sig] = sibling(orig, f"-core-{k}")
    def frag_text(idxs, path):
        body = items_text(module, sorted(idxs), path)
        return strip_unbalanced_regions(f"// {docs_emoji} Shared fixtures of the split test module `{os.path.basename(os.path.dirname(orig))}` ({len(idxs)} items), included by the pieces that use them.\n" + body)
    files[orig] = frag_text(core, orig)
    for sig, idxs in shared.items():
        files[frag_paths[sig]] = frag_text(idxs, frag_paths[sig])
    for u, tests in units.items():
        path = piece_dirs[u]
        fr = [frag_paths[s] for s in sorted(frag_paths, key=lambda s: min(shared[s])) if u in s]
        inc = [orig] + fr
        head = f"//! {docs_emoji} {unit_docs[u]}\n\n"
        use_txt = "".join(rebase(x.body, orig, path) + "\n" for x in uses)
        inc_txt = "".join(f'include!("{os.path.relpath(p, os.path.dirname(path))}");\n' for p in inc)
        priv = items_text(module, sorted(private.get(u, [])), path)
        tst = items_text(module, sorted(tests), path)
        if module.wrapper:
            open_, close_ = module.wrapper
            ind = "    "
            body = docs + "".join(ind + l + "\n" if l.strip() else "\n" for l in (use_txt + inc_txt).split("\n")[:-1]) + priv + tst
            pre = module.pre
            text = head + (rebase(pre, orig, path) if pre.strip() else "") + open_ + "\n" + body + close_ + "\n"
        else:
            text = head + docs + ("\n" if docs else "") + use_txt + inc_txt + priv + tst
        files[path] = strip_unbalanced_regions(text)
    return files, core, shared, private


def _vnorm(s):
    s = s.replace("#[doc(hidden)]", "")
    s = re.sub(r"\bpub(?:\s*\([^)]*\))?\s+", "", s)
    return re.sub(r"\s+", "", s)


def verify(module, files, new_files_for_rebase=None):
    """every non-use item body appears in the output (tests exactly once, helpers at least once; visibility widening is ignored); returns a list of problems"""
    problems = []
    blob = {p: _vnorm(t) for p, t in files.items()}
    for it in module.items:
        if it.kind == "use":
            continue
        of = it.origin_file or module.path
        n = sum(t.count(_vnorm(rebase(it.body, of, p))) for p, t in blob.items())
        if it.has_tests and n != 1:
            problems.append(("test count", it.idx, it.name, n))
        elif n < 1:
            problems.append(("missing", it.idx, it.kind, it.name))
    return problems


def parse_check(files):
    """rustc -Zunpretty=normal parse of each output text; returns [(path, first error line)]"""
    import subprocess, tempfile
    bad = []
    for p, t in files.items():
        with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False, encoding="utf8", dir=SCRATCH) as h:
            h.write(t)
            name = h.name
        try:
            r = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "-Zunpretty=normal", name], capture_output=True, text=True)
            if r.returncode != 0:
                err = next((l for l in r.stderr.split("\n") if l.startswith("error")), r.stderr[:200])
                bad.append((rel(p), err))
        finally:
            os.remove(name)
    return bad


# ----------------------------------------------------------------------------------------------------------------
# include-fragment support (files that are `include!`d, so they have no module docs and no `use super`)
# ----------------------------------------------------------------------------------------------------------------
def make_include_item(module, idx, include_path, orig_stmt_attrs=""):
    """a synthetic item `include!("<rel>");` standing for the whole file include_path; rel is written for module.path"""
    itx = read(include_path) if os.path.exists(include_path) else ""
    icl = blank(itx)
    it = Item()
    it.idx = idx
    it.kind, it.name, it.vis, it.attrs = "other", None, "", orig_stmt_attrs
    rel_ = os.path.relpath(include_path, os.path.dirname(module.path))
    it.gap = ""
    it.body = f'include!("{rel_}");'
    it.text = "\n" + it.body + "\n"
    it.src = rel(include_path)
    it.src_kind = "include"
    it.ids = set(IDENT.findall(icl))
    it.defs = set()
    for s2 in P.statements(itx, icl, 0, len(itx)):
        if s2.name and s2.kind not in ("use", "impl"):
            it.defs.add(s2.name)
        if s2.kind == "other":
            it.defs |= set(DEF_IN_MACRO.findall(icl[s2.start:s2.end]))
    it.is_test = False
    it.has_tests = bool(TEST_ATTR.search(itx))
    it.lines = itx.count("\n") + 1
    it.bytes = len(itx.encode("utf8"))
    it.deps = set()
    it.self_ty = None
    it.emb = embeds_from_text(itx, include_path)
    return it


def emit_fragments(module, units, piece_dirs, emoji, unit_docs, always_core=(), keep_uses=False):
    """like emit() for a file that is itself `include!`d: no module docs, `//` header lines only, no `use` header (fragments carry none)"""
    orig = module.path
    core, shared, private = fragments(module, units, always_core)
    files, frag_paths = {}, {}
    for k, (sig, idxs) in enumerate(sorted(shared.items(), key=lambda kv: min(kv[1])), 1):
        frag_paths[sig] = sibling(orig, f"-core-{k}")
    def body(idxs, path):
        return items_text(module, sorted(idxs), path)
    files[orig] = strip_unbalanced_regions(f"// {emoji} Shared fixtures of the split fragment `{os.path.basename(os.path.dirname(orig))}`, included by its parts.\n" + body(core, orig))
    for sig, idxs in shared.items():
        files[frag_paths[sig]] = strip_unbalanced_regions(f"// {emoji} Shared fixtures of `{os.path.basename(os.path.dirname(orig))}` ({len(idxs)} items) for {', '.join(sorted(sig))}.\n" + body(idxs, frag_paths[sig]))
    for u, tests in units.items():
        path = piece_dirs[u]
        fr = [frag_paths[s] for s in sorted(frag_paths, key=lambda s: min(shared[s])) if u in s]
        files[path] = strip_unbalanced_regions(f"// {emoji} {unit_docs[u]}\n" + body(private.get(u, []), path) + body(sorted(tests), path))
    return files, core, shared, private, frag_paths


def closure_size(entry, overrides):
    """(files, bytes, lines, texts) reachable from entry through literal `include!` and `#[path]` mounts (recursing) and include_str!/include_bytes! (leaf files); overrides = {path: text} for files not yet on disk"""
    seen, stack = {}, [(entry, True)]
    rx = re.compile(r'(include!|include_str!|include_bytes!)\(\s*(?:concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\)|"([^"]+)")\s*\)')
    while stack:
        f, rec = stack.pop()
        if f in seen:
            continue
        txt = overrides.get(f)
        if txt is None:
            if not os.path.exists(f):
                continue
            txt = read(f) if f.endswith(".rs") else open(f, "rb").read().decode("utf8", "replace")
        seen[f] = (txt, rec)
        if not rec or not f.endswith(".rs"):
            continue
        clean = blank(txt)
        d = os.path.dirname(f)
        for m in rx.finditer(txt):
            if clean[m.start():m.start() + len(m.group(1))] != m.group(1):
                continue
            tgt = os.path.normpath(MANIFEST_DIR + "/" + m.group(2).lstrip("/")) if m.group(2) else os.path.normpath(os.path.join(d, m.group(3)))
            stack.append((tgt, m.group(1) == "include!"))
        for m in PATH_ATTR_RX.finditer(txt):
            if clean[m.start():m.start() + 2] == "#[":
                stack.append((os.path.normpath(os.path.join(d, m.group(2))), True))
    texts = {f: v[0] for f, v in seen.items()}
    by = sum(len(t.encode("utf8")) for t in texts.values())
    ln = sum(sum(1 for l in blank(t).split("\n") if l.strip()) for f, (t, rec) in seen.items() if f.endswith(".rs") and rec)
    return len(seen), by, ln, texts


# ----------------------------------------------------------------------------------------------------------------
# flattening of included flat files and origin-aware emission
# ----------------------------------------------------------------------------------------------------------------
def origin_tag(path):
    d = os.path.basename(os.path.dirname(path))
    return re.sub(r"^[^A-Za-z0-9]+", "", d) or "x"


def flatten(module, only=lambda it: True):
    """replace every `include!` item of a flat file by the items of that file (tagged with their origin); nested includes are flattened too"""
    def expand(items):
        out = []
        for it in items:
            if it.src_kind == "include" and it.kind == "other" and only(it):
                tgt = PLUG + "/" + it.src
                itx = read(tgt)
                sub, tail = parse_items(itx, 0, len(itx), src=rel(tgt), base_file=tgt)
                tag = origin_tag(tgt)
                for s in expand(sub):
                    if s.origin == "main":
                        s.origin, s.origin_file = tag, tgt
                    out.append(s)
            else:
                out.append(it)
        return out
    module.items = expand(module.items)
    for i, it in enumerate(module.items):
        it.idx = i
    module.link()


def emit2(module, units, dirs, emoji, docs, always_core=()):
    """files = {path: text}; the original path becomes the core fragment, shared fragments go to `<orig dir>-core-<k>`, items of other origins than the module's own file go to a `-x` sibling of the file that would hold them (so the module's own files keep only their original text). Pieces are flat modules (`use` header, fragment includes, private items, tests)."""
    orig = module.path
    uses = [it for it in module.items if it.kind == "use"]
    lead = lead_docs(module)
    strip_lead_docs(module)
    core, shared, private = fragments(module, units, always_core)
    files, frag_paths = {}, {}
    for k, (sig, idxs) in enumerate(sorted(shared.items(), key=lambda kv: min(kv[1])), 1):
        frag_paths[sig] = sibling(orig, f"-core-{k}")
    def body(idxs, path, main):
        out = []
        for i in sorted(idxs):
            it = module.items[i]
            if (it.origin == "main") == main:
                out.append(rebase(it.text, it.origin_file or module.path, path))
        return "".join(out)
    xp = lambda p: sibling(p, "-x")
    def put_fragment(idxs, path, label):
        m = body(idxs, path, True)
        x = body(idxs, xp(path), False)
        files[path] = strip_unbalanced_regions(f"// {emoji} {label}\n" + m)
        if x.strip():
            files[xp(path)] = strip_unbalanced_regions(f"// {emoji} {label} (items that came from included fixture files)\n" + x)
    put_fragment(core, orig, f"Shared fixtures of the split test module `{os.path.basename(os.path.dirname(orig))}`, included by every piece that uses them.")
    for sig, idxs in shared.items():
        put_fragment(idxs, frag_paths[sig], f"Fixtures of `{os.path.basename(os.path.dirname(orig))}` shared by {', '.join(sorted(sig))}.")
    for u, tests in units.items():
        path = dirs[u]
        fr = [frag_paths[s] for s in sorted(frag_paths, key=lambda s: min(shared[s])) if u in s]
        inc = []
        for p in [orig] + fr:
            inc.append(p)
            if xp(p) in files:
                inc.append(xp(p))
        priv = private.get(u, [])
        pm = body(priv + sorted(tests), path, True)
        px = body(priv + sorted(tests), xp(path), False)
        if px.strip():
            files[xp(path)] = strip_unbalanced_regions(f"// {emoji} Items of {unit_docs_label(docs[u])} that came from included fixture files.\n" + px)
            inc.append(xp(path))
        inc_txt = "".join(f'include!("{os.path.relpath(p, os.path.dirname(path))}");\n' for p in inc)
        hay = pm + px + "".join(files[p] for p in inc if p in files)
        use_txt = "".join(prune_test_uses(rebase(x.body, x.origin_file or module.path, path), hay) + "\n" for x in uses)
        use_txt = "".join(l + "\n" for l in use_txt.split("\n") if l.strip())
        head = f"//! {emoji} {docs[u]}\n\n"
        files[path] = strip_unbalanced_regions(head + lead + ("\n" if lead else "") + use_txt + inc_txt + pm)
    return files, core, shared, private


def unit_docs_label(doc):
    return re.sub(r"\s*\(cut from.*$", "", doc).rstrip(".")


def prune_test_uses(use_body, haystack):
    """drop the imported `test_*` names (the shared test-law functions of other test roots) that the piece does not mention, so a piece depends on a provider root only when it calls it; other imports stay (trait imports have no mention)"""
    m = re.match(r"^(\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+)(.*?);\s*$", use_body, re.S)
    if not m:
        return use_body
    head, body = m.group(1), m.group(2)
    def used(name):
        return re.search(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])", haystack) is not None
    if "{" in body:
        pre, inner = body.split("{", 1)
        inner = inner.rsplit("}", 1)[0]
        parts = [x.strip() for x in inner.split(",") if x.strip()]
        keep = [x for x in parts if not x.split(" as ")[-1].strip().startswith("test_") or used(x.split(" as ")[-1].strip())]
        if len(keep) == len(parts):
            return use_body
        if not keep:
            return ""
        return head + pre + "{" + ", ".join(keep) + "};"
    last = body.split("::")[-1].split(" as ")[-1].strip()
    if last.startswith("test_") and not used(last):
        return ""
    return use_body


# ----------------------------------------------------------------------------------------------------------------
# support modules (shared fixtures as real modules) and piece emission v3
# ----------------------------------------------------------------------------------------------------------------
def scc_order(nodes, deps):
    """strongly connected components of the graph restricted to `nodes`, dependencies first (iterative Tarjan)"""
    index, low, onstack, stack, out, counter = {}, {}, set(), [], [], [0]
    for root in sorted(nodes):
        if root in index:
            continue
        work = [(root, iter(sorted(d for d in deps.get(root, ()) if d in nodes)))]
        index[root] = low[root] = counter[0]; counter[0] += 1; stack.append(root); onstack.add(root)
        while work:
            v, it = work[-1]
            adv = False
            for w in it:
                if w not in index:
                    index[w] = low[w] = counter[0]; counter[0] += 1; stack.append(w); onstack.add(w)
                    work.append((w, iter(sorted(d for d in deps.get(w, ()) if d in nodes))))
                    adv = True
                    break
                elif w in onstack:
                    low[v] = min(low[v], index[w])
            if adv:
                continue
            work.pop()
            if work:
                low[work[-1][0]] = min(low[work[-1][0]], low[v])
            if low[v] == index[v]:
                comp = []
                while True:
                    w = stack.pop(); onstack.discard(w); comp.append(w)
                    if w == v:
                        break
                out.append(sorted(comp))
    return out


def supports(module, units, limit_bytes, always_core=()):
    """(support groups [list of item idx lists, dependencies first], private {unit: idx list}); shared = used by two units or more"""
    its = module.items
    names = list(units)
    cl = {u: {x for x in module.closure(units[u]) if not its[x].has_tests and its[x].kind != "use"} for u in names}
    allh = {it.idx for it in its if not it.has_tests and it.kind != "use"}
    sig = {x: [u for u in names if x in cl[u]] for x in allh}
    private = collections.defaultdict(list)
    shared = set()
    for x, s in sig.items():
        if not s:
            private[names[-1]].append(x)
        elif len(s) == 1 and x not in always_core:
            private[s[0]].append(x)
        else:
            shared.add(x)
    deps = {x: {d for d in its[x].deps if d in shared and d != x} for x in shared}
    comps = scc_order(shared, deps)
    groups, cur, curb = [], [], 0
    cost = lambda comp: sum(its[x].bytes for x in comp) + sum(sum(its[x].emb.values()) for x in comp)
    for comp in comps:
        c = cost(comp)
        if cur and curb + c > limit_bytes:
            groups.append(cur); cur, curb = [], 0
        cur += comp; curb += c
    if cur:
        groups.append(cur)
    return groups, {u: sorted(v) for u, v in private.items()}


def widen_support(text):
    """every private or restricted definition of a support file becomes `pub` (items `#[doc(hidden)]`), thread-local statics too"""
    import widen as WD
    clean = blank(text)
    defs = WD.collect(text, clean, 0, len(text))
    names = {d[1] for d in defs}
    out = WD.apply_widen(text, defs, names)
    def tl(m):
        body = m.group(0)
        return re.sub(r"(?m)^(\s*(?:#\[[^\]]*\]\s*|///[^\n]*\n\s*)*)static ", r"\1pub static ", body)
    return re.sub(r"(?s)thread_local!\s*\{.*?\n\s*\}", tl, out)


def emit3(module, units, dirs, emoji, docs, supp_dir, supp_limit, always_core=(), support_prefix="support", ref="super"):
    """files, support module paths, groups. Support N lives at sibling(orig, '-support-N') (the first one at the original path), items of other origins in its `-x`; pieces use the support modules with `use super::<mod>::*;`"""
    orig = module.path
    its = module.items
    uses = [it for it in its if it.kind == "use"]
    lead = lead_docs(module)
    strip_lead_docs(module)
    groups, private = supports(module, units, supp_limit, always_core)
    files, spaths, smods = {}, [], []
    base = os.path.basename(os.path.dirname(orig))
    def body(idxs, path, main):
        out = []
        for i in sorted(idxs):
            it = its[i]
            if (it.origin == "main") == main:
                out.append(rebase(it.text, it.origin_file or module.path, path))
        return "".join(out)
    for n, g in enumerate(groups, 1):
        path = orig if n == 1 else sibling(orig, f"-{support_prefix}-{n}")
        spaths.append(path)
        smods.append(f"{re.sub(r'[^a-z0-9]+', '_', base.lower()).strip('_')}_{support_prefix}_{n}")
        m = body(g, path, True)
        xp = sibling(path, "-x")
        x = body(g, xp, False)
        head = f"// {emoji} Support module {n} of the split test module `{base}`: the fixtures that more than one piece uses, as real definitions the pieces import.\n"
        if n > 1:
            head += "".join(f"use {ref}::{smods[j]}::*;\n" for j in range(n - 1))
        hay = m + x
        uses_txt = "".join(prune_test_uses(rebase(x_.body, x_.origin_file or module.path, path), hay) + "\n" for x_ in uses)
        uses_txt = "".join(l + "\n" for l in uses_txt.split("\n") if l.strip())
        txt = head + uses_txt + m
        if x.strip():
            files[xp] = widen_support(strip_unbalanced_regions(f"// {emoji} Items of support module {n} that came from included fixture files.\n" + x))
            txt += f'include!("{os.path.relpath(xp, os.path.dirname(path))}");\n'
        files[path] = widen_support(strip_unbalanced_regions(txt))
    where = {x: n for n, g in enumerate(groups) for x in g}
    for u, tests in units.items():
        path = dirs[u]
        need = sorted({where[x] for x in module.closure(tests) if x in where} | {where[t] for t in tests if t in where})
        pm = body(private.get(u, []) + sorted(tests), path, True)
        px = body(private.get(u, []) + sorted(tests), sibling(path, "-x"), False)
        inc = ""
        if px.strip():
            files[sibling(path, "-x")] = strip_unbalanced_regions(f"// {emoji} Items of {unit_docs_label(docs[u])} that came from included fixture files.\n" + px)
            inc = f'include!("{os.path.relpath(sibling(path, "-x"), os.path.dirname(path))}");\n'
        hay = pm + px + "".join(files[p] for p in spaths)
        use_txt = "".join(prune_test_uses(rebase(x_.body, x_.origin_file or module.path, path), hay) + "\n" for x_ in uses)
        use_txt = "".join(l + "\n" for l in use_txt.split("\n") if l.strip())
        glob = "".join(f"use {ref}::{smods[n]}::*;\n" for n in need)
        lead_lines = lead.split("\n")
        inner = [l.strip() for l in lead_lines if l.strip().startswith("//!")]
        rest = "\n".join(l for l in lead_lines if not l.strip().startswith("//!")).strip("\n")
        head = f"//! {emoji} {docs[u]}\n" + "".join(l + "\n" for l in inner) + "\n"
        files[path] = strip_unbalanced_regions(head + rest + ("\n" if rest else "") + glob + use_txt + inc + pm)
    return files, spaths, smods, groups, private
