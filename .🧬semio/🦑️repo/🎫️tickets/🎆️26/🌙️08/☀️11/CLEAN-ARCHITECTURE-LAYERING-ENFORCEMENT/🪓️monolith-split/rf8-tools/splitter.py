"""Coverage-checked Rust source-file splitter used by the frs plans.

A plan is a dict:
  target   key of frslib.TARGETS
  imports  (a, b) 1-based inclusive lines of the root `use` statements that children may need (std imports)
  children list of child specs:
    kind    "inline" (a `mod x { .. }` block lifted to a file; module identity unchanged) or
            "cluster" (top-level items of the root lifted into a NEW module; private items widened to pub(super))
    folder  `<emoji>️<kebab>`; mod: snake name; doc: header lines (first char = unique emoji)
    inline:   head, tail (1-based lines of `mod x {` and its closing `}`), vis ("pub"|"")
    cluster:  ranges [(a, b), ...], at (optional anchor line for the mount block), vis ("" default)
Output goes to the stage dir; nothing in the repo is written here."""
import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
from rslex import lex, outline, multiline_literal_lines
from stubgen import mask

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
KEYWORDS = set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try union".split())


IMPLICIT_TRAIT_METHODS = {
    "Read": ["read", "read_to_end", "read_to_string", "read_exact", "bytes", "by_ref", "take", "chain"],
    "Write": ["write", "write_all", "flush", "write_fmt"],
    "BufRead": ["read_line", "lines", "fill_buf", "consume", "read_until"],
    "Seek": ["seek", "rewind", "stream_position"],
    "FromStr": ["from_str"],
    "Borrow": ["borrow"],
    "BorrowMut": ["borrow_mut"],
    "Deref": ["deref"],
    "DerefMut": ["deref_mut"],
    "Any": ["type_id"],
}

DECL_KW = {"fn", "struct", "enum", "trait", "type", "const", "static", "mod", "union", "macro_rules"}


def tokenize(text, fnptr_names=()):
    """(first_use, all_tokens) of masked code.
    first_use = identifiers that name an item through a path head or a call: not after `.`/`::`, not a declaration or
    binding, not inside derive lists, lowercase ones only when followed by `(`, `!` or `::`."""
    t = mask(text)
    first, allt = set(), set()
    for m in IDENT.finditer(t):
        s, e = m.start(), m.end()
        w = m.group(0)
        if w in KEYWORDS:
            continue
        allt.add(w)
        j = s - 1
        while j >= 0 and t[j] in " \t\n":
            j -= 1
        prev2 = t[j - 1:j + 1] if j >= 1 else ""
        if t[j:j + 1] == "." and prev2 != "..":
            continue
        if prev2 == "::":
            continue
        k = j
        while k >= 0 and (t[k].isalnum() or t[k] == "_"):
            k -= 1
        prevword = t[k + 1:j + 1]
        if prevword in DECL_KW:
            continue
        nxt = t[e:e + 2]
        n2 = t[e:].lstrip()[:2]
        if w[0].islower() or w[0] == "_":
            if not (n2[:1] in ("(", "!") or n2 == "::"):
                if not (w in fnptr_names and n2[:1] in (")", ",", ";", "}", "]") and prevword not in ("let", "mut", "ref", "fn", "for", "as")):
                    continue
        first.add(w)
    return first, allt


FIELD_RX = [re.compile(r"\.\s*([A-Za-z_]\w*)\b(?!\s*(?:\(|::))"), re.compile(r"\b([A-Za-z_]\w*)\s*:(?!:)"), re.compile(r"[{,]\s*([A-Za-z_]\w*)\s*(?=[,}])")]
METHOD_RX = [re.compile(r"\.\s*([A-Za-z_]\w*)\s*(?:::\s*<[^>]*>)?\s*\("), re.compile(r"::\s*([A-Za-z_]\w*)\b")]


def member_refs(text):
    """(field names, method/assoc names) referenced in masked code: `.f` / `f:` / shorthand `{ f, }` versus `.m(` / `::m`."""
    t = mask(text)
    fields, methods = set(), set()
    for rx in FIELD_RX:
        fields.update(m.group(1) for m in rx.finditer(t))
    for rx in METHOD_RX:
        methods.update(m.group(1) for m in rx.finditer(t))
    return fields, methods


def path_mounts(txt, base):
    """[(kind, target path)] of every `#[path]` out-of-line mount and `include!` of a file's text.
    An inline `#[path = "." | ".."] mod x { .. }` re-bases the `#[path]` mounts it contains."""
    out = []
    stack = []
    pending = None
    for line in mask_keep_strings(txt).split("\n"):
        st = line.strip()
        m = re.match(r'^#\[\s*path\s*=\s*"([^"]+)"\s*\]$', st)
        cur = stack[-1] if stack else base
        if m:
            if m.group(1) in (".", ".."):
                pending = os.path.normpath(os.path.join(cur, m.group(1)))
            else:
                out.append(("path", os.path.normpath(os.path.join(cur, m.group(1)))))
            continue
        if re.match(r"^(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{\s*$", st) and line == line.lstrip():
            stack.append(pending or cur)
            pending = None
            continue
        if line == "}" and stack:
            stack.pop()
            continue
        if st.startswith("mod ") or st.startswith("pub mod ") or st.startswith("pub(crate) mod "):
            pending = None
        for m2 in re.finditer(r'\binclude!\(\s*"([^"]+)"\s*\)', line):
            out.append(("include", os.path.normpath(os.path.join(cur, m2.group(1)))))
    return out


def crate_files(lib_abs):
    """Files reachable from a lib root via #[path] mounts and include!: [abs path]."""
    seen, order = set(), []

    def visit(p):
        p = os.path.normpath(p)
        if p in seen or not os.path.isfile(p):
            return
        seen.add(p); order.append(p)
        for kind, target in path_mounts(read(p), os.path.dirname(p)):
            visit(target)
    visit(lib_abs)
    return order


def mask_keep_strings(text):
    """Blank comments only (strings kept) so #[path = ".."] can be read."""
    t = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), text)
    return re.sub(r"/\*.*?\*/", lambda m: re.sub(r"[^\n]", " ", m.group(0)), t, flags=re.S)


def attrs_before(ls, start_idx):
    """cfg attribute lines among the contiguous attribute/doc lines ending right before start_idx."""
    out = []
    i = start_idx - 1
    while i >= 0:
        s = ls[i].strip()
        if s.startswith("#[") or s.startswith("///") or s.startswith("//"):
            if re.match(r"^#\[cfg\(", s):
                out.append(ls[i].strip())
            i -= 1
            continue
        break
    return list(reversed(out))


def parse_use_line(line):
    """`use a::b::{X, Y};` -> (prefix 'a::b', [names]) ; `use a::b::X;` -> ('a::b', ['X'])."""
    m = re.match(r"^\s*use\s+(.+?);\s*$", line)
    assert m, line
    body = m.group(1)
    mm = re.match(r"^(.*?)::\{(.*)\}$", body)
    if mm:
        return mm.group(1), [x.strip() for x in mm.group(2).split(",") if x.strip()]
    pre, _, last = body.rpartition("::")
    return pre, [last.strip()]


def name_of_use_item(item):
    m = re.match(r"^(\w+)(?:\s+as\s+(\w+))?$", item)
    if m:
        return m.group(2) or m.group(1)
    return item


def fmt_use(prefix, items, vis=""):
    head = (vis + " " if vis else "") + "use "
    base = (prefix + "::") if prefix else ""
    if len(items) == 1:
        return f"{head}{base}{items[0]};"
    return f"{head}{base}{{{', '.join(items)}}};"


_RF_CACHE = {}
USE_RX = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?use\s.+;$")


def rustfmt_generated_use(line, original):
    """A generated one-line `use` statement formatted the way rustfmt prints it; original lines pass through untouched."""
    st = line.strip()
    if line != st or not USE_RX.match(st) or st in original:
        return line
    if st not in _RF_CACHE:
        import subprocess
        p = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=st + "\n", capture_output=True, text=True)
        out = p.stdout.strip("\n")
        _RF_CACHE[st] = out if p.returncode == 0 and out.replace(" ", "").replace("\n", "") .startswith(st.replace(" ", "")[:8]) and "Error" not in out else st
    return _RF_CACHE[st]


def rustfmt_block(lines):
    """A generated import block (cfg attributes included) laid out the way rustfmt prints it; unchanged on any failure."""
    if not lines:
        return lines
    import subprocess
    txt = "\n".join(lines) + "\n"
    p = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=txt, capture_output=True, text=True)
    out = [l for l in p.stdout.split("\n")]
    while out and out[-1] == "":
        out.pop()
    squash = lambda t: sorted(re.sub(r"[\s,{}]", "", t))
    if p.returncode != 0 or not out or squash("".join(out)) != squash("".join(lines)):
        return lines
    return out


def vis_spelling(v):
    """The `use` visibility that re-exports an item of visibility `v` without widening it."""
    v = v.strip()
    if not v:
        return ""
    if v == "pub(super)":
        return "pub(super)"
    return v


class Plan:
    def __init__(self, spec):
        self.spec = spec
        self.t = TARGETS[spec["target"]]
        self.name = spec["target"]
        self.src = read(snap_dir(self.name) + "/" + self.t["file"])
        self.ls = lines_of(self.src)
        assert not multiline_literal_lines(self.src), "multi-line literals present: dedent/cut unsafe"
        self.cov = Coverage(len(self.ls))
        self.base = os.path.dirname(self.t["file"])

    # ---------- text helpers
    def seg(self, a, b):
        return self.ls[a - 1:b]

    def build(self):
        spec = self.spec
        ls = self.ls
        children = spec["children"]
        for c in children:
            c.setdefault("vis", "")
            c.setdefault("cfg", [])
        for c in children:
            if c["kind"] == "inline":
                self.cov.claim(c["mod"] + ":mount", c["head"], c["head"])
                self.cov.claim(c["mod"] + ":mount", c["tail"], c["tail"])
                self.cov.claim(c["mod"], c["head"] + 1, c["tail"] - 1)
                assert ls[c["head"] - 1].strip().endswith("{") and re.search(r"\bmod\s+" + c["mod"] + r"\b", ls[c["head"] - 1]), (c["mod"], ls[c["head"] - 1])
                assert ls[c["tail"] - 1] == "}", (c["mod"], ls[c["tail"] - 1])
                c["_ranges"] = [(c["head"] + 1, c["tail"] - 1)]
            else:
                for a, b in c["ranges"]:
                    self.cov.claim(c["mod"], a, b)
                c["_ranges"] = list(c["ranges"])
        owned = set()
        for i, o in enumerate(self.cov.owner):
            if o is not None and not o.endswith(":mount"):
                owned.add(i)
        # parent remainder text (used for token statistics)
        mount_lines = {c["mod"]: [] for c in children}
        parent_idx = [i for i in range(1, len(ls) + 1) if i not in owned]
        pre = set()
        for c in children:
            if c["kind"] == "inline":
                k = c["head"] - 1
                while k >= 1 and (ls[k - 1].strip().startswith("#[") or ls[k - 1].strip().startswith("//")):
                    pre.add(k)
                    k -= 1
        remainder = "\n".join(ls[i - 1] for i in parent_idx if not (self.cov.owner[i] or "").endswith(":mount") and i not in pre)
        ia0, ib0 = spec["imports"]
        rem_first, rem_all = tokenize("\n".join(ls[i - 1] for i in parent_idx if not (self.cov.owner[i] or "").endswith(":mount") and i not in pre and not (ia0 <= i <= ib0)))
        # child bodies
        bodies = {}
        for c in children:
            body = []
            for a, b in c["_ranges"]:
                body.extend(self.seg(a, b))
            if c["kind"] == "inline":
                body = dedent(body, 4)
            bodies[c["mod"]] = body
        # root import entries
        ia, ib = spec["imports"]
        imports = []
        for i in range(ia, ib + 1):
            line = ls[i - 1]
            if line.strip().startswith("use "):
                pre, names = parse_use_line(line)
                imports.append(dict(line=i, prefix=pre, names=names, cfg=attrs_before(ls, i - 1)))
        import_lower = {name_of_use_item(it) for im in imports for it in im["names"] if name_of_use_item(it)[:1].islower()}
        # global tokens
        snap_lib = snap_dir(self.name) + "/" + self.t["lib"]
        other_files = [p for p in crate_files(snap_lib) if os.path.normpath(p) != os.path.normpath(snap_dir(self.name) + "/" + self.t["file"])]
        other_first, other_all = set(), set()
        other_fields, other_methods = set(), set()
        for p in other_files:
            f, a = tokenize(read(p))
            other_first |= f
            other_all |= a
            fl, me = member_refs(read(p))
            other_fields |= fl
            other_methods |= me
        rem_fields, rem_methods = member_refs(remainder)
        child_mem = {c["mod"]: member_refs("\n".join(bodies[c["mod"]])) for c in children}
        child_tok = {c["mod"]: tokenize("\n".join(bodies[c["mod"]]), import_lower) for c in children}
        # defs per cluster
        cluster_defs = {}
        for c in children:
            if c["kind"] != "cluster":
                continue
            body = bodies[c["mod"]]
            txt = "\n".join(body) + "\n"
            its = outline(txt)
            defs = []
            for it in its:
                if it["name"] is None or it["kind"] in ("use",):
                    continue
                defs.append(dict(name=it["name"], kind=it["kind"], vis=it["vis"], start=it["start"], end=it["end"],
                                 cfg=attrs_before(body, self._head_index(body, it))))
            cluster_defs[c["mod"]] = (body, its, defs)
        report = dict(children={})
        # names defined at root level (importable by clusters): remainder items and other clusters' defs
        rem_txt = remainder
        rem_items = outline(rem_txt + "\n")
        rem_rem_lines = rem_txt.split("\n")
        variants = collections.defaultdict(list)
        for it in rem_items:
            if it["name"] is not None and it["kind"] in ("mod", "struct", "enum", "fn", "static", "const", "type", "trait", "union"):
                variants[it["name"]].append(tuple(attrs_before(rem_rem_lines, self._head_index(rem_rem_lines, it))))
            elif it["kind"] == "use":
                for nm in self._use_names(rem_rem_lines[it["start"]:it["end"] + 1]):
                    variants[nm].append(tuple(attrs_before(rem_rem_lines, self._head_index(rem_rem_lines, it))))
        rootlevel = {nm: (list(v[0]) if len(set(v)) == 1 else []) for nm, v in variants.items()}
        for c in children:
            if c["kind"] == "inline":
                rootlevel.setdefault(c["mod"], [])
        for mod, (_b, _its, defs) in cluster_defs.items():
            for d in defs:
                rootlevel[d["name"]] = d["cfg"]
        # widening + imports per cluster
        out_files, mount_blocks = {}, {}
        for c in children:
            mod = c["mod"]
            body = list(bodies[mod])
            first, allt = child_tok[mod]
            rep = dict(widened=[], imports=[], super_imports=[], reexport_pub=[], reexport_priv=[])
            if c["kind"] == "inline":
                for i, l in enumerate(body):
                    m2 = re.match(r'^(\s*)(include(?:_str|_bytes)?!\()"([^"/][^"]*)"\)', l)
                    if m2:
                        body[i] = l.replace(f'"{m2.group(3)}"', f'"../{m2.group(3)}"', 1)
                header = c["doc"]
                extra = c.get("extra_uses", [])
                text = "\n".join(header) + "\n" + ("\n" + "\n".join(extra) + "\n" if extra else "") + "\n" + "\n".join(body) + "\n"
                out_files[c["folder"]] = text
                blk = list(c.get("cfg", []))
                blk.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
                blk.append(f'{(c["vis"] + " ") if c["vis"] else ""}mod {mod};')
                mount_blocks[mod] = blk
                report["children"][mod] = rep
                continue
            body_text_lines, defs = cluster_defs[mod][0], cluster_defs[mod][2]
            for i, l in enumerate(body):
                if spec.get("path_dot_up") and l.strip() == '#[path = "."]':
                    body[i] = l.replace('"."', '".."')
            # include path fixups
            for i, l in enumerate(body):
                if re.search(r'include(?:_str|_bytes)?!\(\s*"[^"/]', l):
                    body[i] = re.sub(r'(include(?:_str|_bytes)?!\(\s*)"([^"/])', r'\1"../\2', l)
            ext_first = set(other_first) | rem_first
            ext_all = set(other_all) | rem_all
            for m2, (tk_first, tk_all) in child_tok.items():
                if m2 != mod:
                    ext_first |= tk_first
                    ext_all |= tk_all
            ext_fields = set(other_fields) | rem_fields
            ext_methods = set(other_methods) | rem_methods
            for m2, (fl, me) in child_mem.items():
                if m2 != mod:
                    ext_fields |= fl
                    ext_methods |= me
            msk = mask("\n".join(body)).split("\n")
            rep["super_refs"] = [body[i].strip()[:110] for i, l in enumerate(msk) if re.search(r"\b(?:super|self)::", l)]
            for i, l in enumerate(body):
                if re.match(r"^pub\(super\) ", l):
                    body[i] = "pub(in super::super) " + l[len("pub(super) "):]
                    rep.setdefault("pub_super_converted", []).append(l.strip()[:80])
            # candidate private definitions to widen
            edits = self.widen_edits(body, ext_all, ext_fields, ext_methods)
            for ln, newline, label in edits:
                body[ln] = newline
                rep["widened"].append(label)
            # imports from root std imports
            use_lines = []
            icfg = c.get("import_cfg", {})
            for im in imports:
                keep, special = [], []
                for it in im["names"]:
                    nm = name_of_use_item(it)
                    base = "self" if nm == "self" else nm
                    if base in first or self._implicit_trait(nm, "\n".join(body)):
                        (special if nm in icfg else keep).append(it)
                if keep:
                    use_lines.append((im["cfg"], fmt_use(im["prefix"], keep)))
                for it in special:
                    use_lines.append((list(im["cfg"]) + list(icfg[name_of_use_item(it)]), fmt_use(im["prefix"], [it])))
            if spec.get("glob_super"):
                use_lines = [([], "use super::*;")]
            for extra in c.get("extra_uses", []):
                use_lines.append(([], extra))
            own = {d["name"] for d in defs}
            sup = {}
            for nm in sorted(first if not spec.get("glob_super") else ()):
                if nm in rootlevel and nm not in own and nm not in set(spec.get("no_super_import", [])) and nm not in {name_of_use_item(x) for im in imports for x in im["names"]}:
                    sup.setdefault(tuple(rootlevel[nm]), []).append(nm)
            for cfgs, names in sup.items():
                use_lines.append((list(cfgs), f"use super::{{{', '.join(names)}}};" if len(names) > 1 else f"use super::{names[0]};"))
                rep["super_imports"].extend(names)
            header = c["doc"]
            hdr = "\n".join(header) + "\n"
            ulines = []
            for cf, ln in use_lines:
                ulines.extend(cf)
                ulines.append(ln)
            ulines = rustfmt_block(ulines)
            while body and not body[-1].strip():
                body.pop()
            rep["imports"] = [l for l in ulines if l.startswith("use ")]
            text = hdr + ("\n" + "\n".join(ulines) + "\n" if ulines else "") + "\n" + "\n".join(body) + "\n"
            out_files[c["folder"]] = text
            # mount block + reexports
            blk = list(c.get("cfg", []))
            blk.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
            blk.append(f'{(c["vis"] + " ") if c["vis"] else ""}mod {mod};')
            groups = collections.OrderedDict()
            for d in defs:
                if d["kind"] in ("use",):
                    continue
                nm = d["name"]
                vis_s = d["vis"].strip()
                is_pub = vis_s == "pub"
                external = nm in ext_first or nm in ext_all
                if not is_pub and not external:
                    continue
                if nm in self.spec.get("skip_import", []) and not is_pub:
                    continue
                key = ("pub " if is_pub else (vis_s + " " if vis_s.startswith("pub(") else ""), tuple(self.spec.get("reexport_cfg", {}).get(nm, d["cfg"])))
                groups.setdefault(key, []).append(nm)
                (rep["reexport_pub"] if is_pub else rep["reexport_priv"]).append(nm)
            for (vis, cfgs), names in sorted(groups.items(), key=lambda kv: (kv[0][0] != "", kv[0][0].startswith("pub "))):
                names = sorted(set(names))
                line = f"{vis}use {mod}::{{{', '.join(names)}}};" if len(names) > 1 else f"{vis}use {mod}::{names[0]};"
                blk.extend(cfgs)
                blk.append(line)
            mount_blocks[mod] = blk
            report["children"][mod] = rep
        # root imports pruning
        keep_root = {}
        root_consumers = set(rem_first) | (other_first - set())
        for m2, (tk_first, _) in child_tok.items():
            cc = [c for c in children if c["mod"] == m2][0]
            if cc["kind"] == "inline":
                root_consumers |= tk_first
        # parent assembly
        parent = []
        i = 1
        n = len(ls)
        inserted = set()
        anchors = {}
        for c in children:
            a = c["head"] if c["kind"] == "inline" else (c.get("at") or c["_ranges"][0][0])
            anchors.setdefault(a, []).append(c)
        skip_lines = set()
        for c in children:
            if c["kind"] == "inline":
                skip_lines.add(c["head"]); skip_lines.add(c["tail"])
                for k in range(c["head"] + 1, c["tail"]):
                    skip_lines.add(k)
            else:
                for a, b in c["_ranges"]:
                    for k in range(a, b + 1):
                        skip_lines.add(k)
        import_lines = {im["line"]: im for im in imports}
        for k in range(1, n + 1):
            if k in anchors:
                for c in anchors[k]:
                    parent.extend(mount_blocks[c["mod"]])
            if k in skip_lines:
                continue
            if k in import_lines:
                im = import_lines[k]
                drop = set(spec.get("root_drop", []))
                rcfg = spec.get("root_cfg", {})
                keep = [it for it in im["names"] if (spec.get("glob_super") or name_of_use_item(it) in root_consumers or name_of_use_item(it) in rem_first or name_of_use_item(it) in spec.get("root_force", [])) and name_of_use_item(it) not in drop]
                plain = [it for it in keep if name_of_use_item(it) not in rcfg]
                gated = [it for it in keep if name_of_use_item(it) in rcfg]
                if not keep:
                    continue
                if not gated and len(keep) == len(im["names"]):
                    parent.append(ls[k - 1])
                    continue
                if plain:
                    parent.append(f"use {im['prefix']}::{plain[0]};" if len(plain) == 1 else f"use {im['prefix']}::{{{', '.join(plain)}}};")
                groups = collections.OrderedDict()
                for it in gated:
                    groups.setdefault(tuple(rcfg[name_of_use_item(it)]), []).append(it)
                for cfgs, items in groups.items():
                    parent.extend(cfgs)
                    parent.append(f"use {im['prefix']}::{items[0]};" if len(items) == 1 else f"use {im['prefix']}::{{{', '.join(items)}}};")
                continue
            parent.append(ls[k - 1])
        for k in range(1, n + 1):
            if self.cov.owner[k] is None:
                self.cov.owner[k] = "parent"
        self.cov.check()
        tidy = []
        for l in parent:
            if l.strip() == "" and tidy and tidy[-1].strip() == "":
                continue
            tidy.append(l)
        parent = tidy
        self.out = {self.t["file"]: "\n".join(parent) + "\n"}
        for folder, txt in out_files.items():
            self.out[self.base + "/" + folder + "/🦀️.rs"] = txt
        orig_stripped = {l.strip() for l in ls}
        for rel in list(self.out):
            self.out[rel] = "\n".join(rustfmt_generated_use(l, orig_stripped) for l in self.out[rel].split("\n"))
        self.report = report
        return self

    # ---------- helpers
    def _head_index(self, lines, it):
        for i in range(it["start"], it["end"] + 1):
            s = lines[i].strip()
            if not (s.startswith("#[") or s.startswith("///") or s.startswith("//") or s == ""):
                return i
        return it["start"]

    def _use_names(self, lines):
        txt = " ".join(l.strip() for l in lines if not l.strip().startswith("#[") and not l.strip().startswith("//"))
        m = re.match(r"^(?:pub(?:\([^)]*\))?\s+)?use\s+(.+?);", txt)
        if not m:
            return []
        body = m.group(1)
        mm = re.match(r"^(.*?)::\{(.*)\}$", body)
        if mm:
            return [name_of_use_item(x.strip()) for x in mm.group(2).split(",") if x.strip()]
        return [name_of_use_item(body.rpartition("::")[2].strip())]

    def _implicit_trait(self, nm, text):
        t = mask(text)
        if nm == "Future":
            return bool(re.search(r"\.poll\(|\.as_mut\(\)\.poll", t))
        if nm == "Hash":
            return bool(re.search(r"\.hash\(", t))
        if nm == "Hasher":
            return bool(re.search(r"\.(finish|write_u\d+|write)\(", t)) and bool(re.search(r"[Hh]asher", t))
        methods = IMPLICIT_TRAIT_METHODS.get(nm)
        if methods:
            return bool(re.search(r"\.(?:" + "|".join(methods) + r")\s*(?:::\s*<[^>]*>)?\(", t))
        return False

    def widen_edits(self, body, ext_all, ext_fields, ext_methods):
        """Return [(line_index, new_line, label)] making private definitions referenced elsewhere pub(super)."""
        text = "\n".join(body) + "\n"
        depths, inlit = lex(text)
        edits = {}
        items = outline(text)
        pubtypes = {i["name"] for i in items if i["kind"] in ("struct", "enum", "trait", "union", "type") and i["vis"] == "pub" and i["name"]}
        for it in items:
            kind = it["kind"]
            hi = self._head_index(body, it)
            head = body[hi]
            if kind in ("fn", "struct", "enum", "trait", "const", "static", "type", "union", "mod") and not it["vis"] and it["name"]:
                if it["name"] in ext_all:
                    m = re.match(r"^(\s*)(.*)$", head)
                    edits[hi] = (m.group(1) + "pub(super) " + m.group(2), f"{kind} {it['name']}")
            if kind == "struct" and (not self.spec.get("gate_members") or it["vis"] == "pub" or it["name"] in ext_all):
                self._widen_struct(body, it, hi, depths, ext_all, ext_fields, edits)
            if kind == "impl":
                mt = re.match(r"^impl(?:<[^>]*>)?\s+([A-Za-z_]\w*)", head)
                tname = mt.group(1) if mt else None
                if not self.spec.get("gate_members") or (tname and (tname in ext_all or tname in pubtypes)):
                    self._widen_impl(body, it, hi, depths, ext_methods, edits)
        res = []
        for hi, (nl, label) in sorted(edits.items()):
            res.append((hi, nl, label))
        return res

    def _widen_struct(self, body, it, hi, depths, ext_all, ext_fields, edits):
        head = body[hi]
        if it["end"] == hi:
            base_line = edits[hi][0] if hi in edits else head
            m1 = re.match(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?struct\s+\w+(?:<[^{]*>)?(?:\s+where[^{]*)?\s*)\{(.*)\}\s*$", base_line)
            if m1:
                inner = m1.group(3)
                parts, cur, depth = [], "", 0
                for ch in inner:
                    if ch in "<([":
                        depth += 1
                    elif ch in ">)]" and not (ch == ">" and cur.endswith("-")):
                        depth -= 1
                    if ch == "," and depth == 0:
                        parts.append(cur)
                        cur = ""
                    else:
                        cur += ch
                parts.append(cur)
                out, changed = [], []
                for part in parts:
                    mf = re.match(r"^(\s*)(?!pub\b)([A-Za-z_]\w*)\s*:(.*)$", part, re.S)
                    if mf and mf.group(2) in ext_fields:
                        out.append(mf.group(1) + "pub(super) " + part[len(mf.group(1)):])
                        changed.append(mf.group(2))
                    else:
                        out.append(part)
                if changed:
                    edits[hi] = (m1.group(1) + m1.group(2) + "{" + ",".join(out) + "}", f"inline fields {it['name']}.{'/'.join(changed)}")
                return
        if "{" in head and "(" not in head.split("{")[0]:
            for k in range(hi + 1, it["end"] + 1):
                if depths[k] != 1:
                    continue
                line = body[k]
                m = re.match(r"^(\s*)(?!pub\b)(?:#\[[^\]]*\]\s*)?([A-Za-z_]\w*)\s*:", line)
                if m and not line.strip().startswith("#[") and not line.strip().startswith("//"):
                    nm = m.group(2)
                    if nm in ext_fields:
                        edits[k] = (m.group(1) + "pub(super) " + line[len(m.group(1)):], f"field {it['name']}.{nm}")
        elif re.search(r"struct\s+\w+(?:<[^>]*>)?\s*\(", head):
            m = re.search(r"(struct\s+\w+(?:<[^>]*>)?\s*\()(.*)$", head)
            inner = m.group(2)
            if inner.lstrip().startswith("pub"):
                return
            if it["name"] in ext_all:
                nl = head[:m.end(1)] + "pub(super) " + inner
                edits[hi] = (nl, f"tuple field {it['name']}.0")

    def _widen_impl(self, body, it, hi, depths, ext_methods, edits):
        head = body[hi]
        if re.search(r"\bfor\b", head.split("{")[0]):
            return
        for k in range(hi + 1, it["end"] + 1):
            if depths[k] != 1:
                continue
            line = body[k]
            m = re.match(r"^(\s*)(?!pub\b)((?:const\s+|async\s+|unsafe\s+|extern\s+\"[^\"]*\"\s+)*fn\s+(\w+)|const\s+(\w+)\s*:|type\s+(\w+))", line)
            if m:
                nm = m.group(3) or m.group(4) or m.group(5)
                if nm in ext_methods:
                    edits[k] = (m.group(1) + "pub(super) " + line[len(m.group(1)):], f"member {nm}")


def stage(plan):
    sd = stage_dir(plan.name)
    import shutil
    if os.path.exists(sd):
        shutil.rmtree(sd)
    for rel, txt in plan.out.items():
        write(f"{sd}/{rel}", txt)


def multiset_report(plan):
    new = []
    for txt in plan.out.values():
        new.extend(lines_of(txt))
    gone, added = diff_multiset(plan.ls, new)
    return gone, added
