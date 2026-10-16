"""🏗️ Core of the rtb tools: namespace imports of test files, item extraction into fixture modules, bin roots and manifests.
Loads the ri2 name indexes (Shell crates, frame core, root globs, parent imports) with a tolerant reader (the pre-flip chunk files are gone)."""
import collections, json, os, re, sys
import ri2shim as RS
from lib import *
import bins as B, flipfc as X, flip as FL

EXTRA_ROOT = {}
FIXTURE_P1 = {}
EXTRA_FACADE = {"scenes": {}, "engine_canvas": {}, "interpreter": {}}
_orig_map = X.map_root_path


LOCAL_FACADE = {}


def mapper(tokens_):
    crate_prefixed = bool(tokens_) and tokens_[0] == "crate"
    body = tokens_[1:] if crate_prefixed else tokens_
    if len(body) > 1 and body[0] in LOCAL_FACADE and body[1] in LOCAL_FACADE[body[0]]:
        return None
    if body and body[0] in EXTRA_ROOT and (crate_prefixed or len(body) > 1):
        return list(EXTRA_ROOT[body[0]]) + body[1:]
    if body and len(body) > 1 and body[0] in EXTRA_FACADE and body[1] in EXTRA_FACADE[body[0]]:
        return list(EXTRA_FACADE[body[0]][body[1]]) + body[2:]
    return _orig_map(tokens_)


X.map_root_path = mapper
X.map_crate_path_fc = mapper
FL.map_crate_path = mapper

SUPER_REF = re.compile(r"(?<![A-Za-z0-9_$])super::([a-z][a-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*)")


def host_uses(host_file):
    """local name -> [(cfg, mapped path tokens)] from the top-level `use` items of a host file."""
    res = collections.defaultdict(list)
    for cfg, vis, p, local, glob in uses(read(host_file), top_only=True):
        if glob or not local or not p or p[0] in ("super", "self"):
            continue
        try:
            if p[0] == "crate":
                p = X.map_root_path(p)
                if p is None:
                    continue
        except KeyError:
            continue
        if p[0] in X.P.MODULE_TO_PATH and p[0] not in ("shell",):
            p = X.map_root_path(p) or p
        res[local].append((cfg, p))
    return res


def set_hosts(files, extra=None):
    B.EXTRA_P1.clear()
    for f in files:
        for k, v in host_uses(f).items():
            B.EXTRA_P1.setdefault(k, [])
            for e in v:
                if e not in B.EXTRA_P1[k]:
                    B.EXTRA_P1[k].append(e)
    for k, v in list((extra or {}).items()) + list(FIXTURE_P1.items()):
        B.EXTRA_P1.setdefault(k, [])
        for e in v:
            if e not in B.EXTRA_P1[k]:
                B.EXTRA_P1[k].append(e)


def imports(suffix, files):
    """(extern-crate alias lines, use lines) the files need, resolved against the indexes and the hosts set by set_hosts."""
    X.CURRENT[0] = suffix
    return B.needs_bin(suffix, files)


def file_uses(path, tokens_):
    """The file's own explicit top-level `use` items whose local name is among the tokens: [(cfg, path tokens, local)]."""
    out = []
    for cfg, vis, p, local, glob in uses(read(path), top_only=True):
        if glob or not local or not p or local not in tokens_:
            continue
        if p[0] in ("super", "self"):
            continue
        out.append((cfg, p, local))
    return out


def render_file_uses(entries):
    lines, seen = [], set()
    for cfg, p, local in entries:
        piece = p[-1] if p[-1] == local else f"{p[-1]} as {local}"
        line = (cfg + "\n" if cfg else "") + f"use {'::'.join(p[:-1])}::{piece};" if len(p) > 1 else (cfg + "\n" if cfg else "") + f"use {piece};"
        if line not in seen:
            seen.add(line)
            lines.append(line)
    return lines


def widen(text):
    """pub(super|crate|self|in …) -> pub for the text of moved items."""
    return FL.vis_widen(text)


def plain_item_vis(text, name, kind):
    """Adds `pub` in front of a top-level item whose head has none."""
    return re.sub(r"(?m)^((?:async |const |unsafe )*)(fn|struct|enum|const|static|type) " + re.escape(name) + r"\b", lambda m: "pub " + m.group(0), text, count=1) if not re.search(r"(?m)^pub (?:async |const |unsafe )*" + kind + " " + re.escape(name) + r"\b", text) else text


def strip_test_cfg(attr):
    """A cfg attribute of the moved text with the `test` predicate dropped (the fixture crate is test-only); None when nothing remains."""
    inner = re.sub(r"^#\[cfg\((.*)\)\]$", r"\1", attr)
    inner = inner.strip()
    if inner == "test":
        return None
    m = re.fullmatch(r"all\(test,\s*(.*)\)", inner)
    if m:
        return f"#[cfg({m.group(1)})]"
    m = re.fullmatch(r"all\((.*),\s*test\)", inner)
    if m:
        return f"#[cfg({m.group(1)})]"
    if re.search(r"\btest\b", inner):
        raise ValueError("cfg with test: " + attr)
    return attr


def extract(src, names, impl_heads=True):
    """Cuts the closure of `names` (items of src) out of src. Returns (src_without, moved_text_by_order, moved_names)."""
    text = read(src)
    lines = text.split("\n")
    tbl = [i for i in itm.table(text) if i["kind"] not in ("use", "mod")]
    by = collections.defaultdict(list)
    for i in tbl:
        if i["kind"] != "impl":
            by[i["name"]].append(i)
    want, queue = set(), list(names)
    while queue:
        n = queue.pop()
        if n in want or n not in by:
            continue
        want.add(n)
        for it in by[n]:
            for t in tokens("\n".join(lines[it["start"] - 1:it["end"]])):
                if t in by and t not in want:
                    queue.append(t)
    spans = []
    for n in want:
        for it in by[n]:
            spans.append((it["start"], it["end"], it))
    for it in tbl:
        if it["kind"] == "impl":
            head = it["name"]
            if any(re.search(r"\b" + re.escape(n) + r"\b", head) for n in want):
                spans.append((it["start"], it["end"], it))
    spans.sort(key=lambda s: s[0])
    for a, b in zip(spans, spans[1:]):
        assert a[1] < b[0], ("overlapping spans", a[2]["name"], b[2]["name"])
    moved = []
    for s, e, it in spans:
        blk = lines[s - 1:e]
        out = []
        for l in blk:
            if l.lstrip().startswith("#[cfg("):
                new = strip_test_cfg(l.strip())
                if new is None:
                    continue
                out.append(l.replace(l.strip(), new))
            else:
                out.append(l)
        t = widen("\n".join(out))
        if it["kind"] != "impl":
            t = plain_item_vis(t, it["name"], it["kind"])
        moved.append(t)
    keep = []
    cut = set()
    for s, e, it in spans:
        cut.update(range(s - 1, e))
    prev_blank = False
    for idx, l in enumerate(lines):
        if idx in cut:
            continue
        keep.append(l)
    new_src = re.sub(r"\n{3,}", "\n\n", "\n".join(keep))
    return new_src, moved, sorted(want)


def sibling_refs(text, names):
    """`super::<module>::NAME` for NAME among the moved names -> NAME; returns (text, used names)."""
    used = set()

    def sub(m):
        if m.group(2) in names:
            used.add(m.group(2))
            return m.group(2)
        return m.group(0)

    return SUPER_REF.sub(sub, text), used


# -- cfg expressions ---------------------------------------------------------------------------------------------------------------
def _parse_cfg(s):
    s = s.strip()
    m = re.match(r"(all|any|not)\((.*)\)$", s, re.S)
    if m:
        parts, depth, cur = [], 0, ""
        for ch in m.group(2):
            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
            if ch == "," and depth == 0:
                parts.append(cur)
                cur = ""
            else:
                cur += ch
        if cur.strip():
            parts.append(cur)
        return (m.group(1), [_parse_cfg(p) for p in parts])
    return ("atom", s)


def _render_cfg(node):
    if node[0] == "atom":
        return node[1]
    return f"{node[0]}(" + ", ".join(_render_cfg(c) for c in node[1]) + ")"


def _eval_cfg(node, truths):
    """Simplifies a cfg tree: atoms in `truths` become True; returns True, False or a node."""
    kind = node[0]
    if kind == "atom":
        return truths.get(node[1].replace(" ", ""), node)
    kids = [_eval_cfg(c, truths) for c in node[1]]
    if kind == "not":
        k = kids[0]
        if k is True:
            return False
        if k is False:
            return True
        return ("not", [k])
    if kind == "all":
        if any(k is False for k in kids):
            return False
        kids = [k for k in kids if k is not True]
        if not kids:
            return True
        return kids[0] if len(kids) == 1 else ("all", kids)
    if any(k is True for k in kids):
        return True
    kids = [k for k in kids if k is not False]
    if not kids:
        return False
    return kids[0] if len(kids) == 1 else ("any", kids)


def simplify_test_cfg(attrs_text):
    """The cfg attribute lines of `attrs_text` with `test` and `feature = "testkit"` taken as true. Returns (text, alive)."""
    out = []
    for l in [x.strip() for x in attrs_text.split("\n") if x.strip()]:
        m = re.fullmatch(r"#\[cfg\((.*)\)\]", l)
        if not m:
            out.append(l)
            continue
        r = _eval_cfg(_parse_cfg(m.group(1)), {"test": True, 'feature="testkit"': True})
        if r is False:
            return "", False
        if r is True:
            continue
        out.append(f"#[cfg({_render_cfg(r)})]")
    return "\n".join(out), True


def merge_import_blocks(blocks, simplify=False, exclude_locals=(), own=None):
    """Parses the use lines of several blocks, dedupes by (cfg, local name) and renders them grouped by (cfg, parent path)."""
    text = "\n".join(blocks)
    entries = []
    for cfg, vis, p, local, glob in uses(text):
        if simplify and cfg:
            cfg, alive = simplify_test_cfg(cfg)
            if not alive:
                continue
        if not glob and local in exclude_locals:
            continue
        p = list(p)
        if own and p and p[0] == own:
            p = ["crate"] + p[1:]
            if len(p) == 1:
                continue
        entries.append((cfg.replace("\n", " ").strip(), tuple(p), local, glob))
    seen, groups, order, taken = set(), collections.OrderedDict(), [], {}
    for cfg, p, local, glob in entries:
        key = (cfg, p, local, glob)
        if key in seen:
            continue
        seen.add(key)
        if glob:
            order.append(("glob", cfg, p))
            continue
        if taken.get((cfg, local), p) != p:
            continue
        taken[(cfg, local)] = p
        groups.setdefault((cfg, p[:-1]), []).append(p[-1] if p[-1] == local else f"{p[-1]} as {local}")
    lines = []
    locals_seen = {}
    for (cfg, parent), names in sorted(groups.items(), key=lambda kv: ("::".join(kv[0][1]), kv[0][0])):
        names = sorted(set(names))
        body = names[0] if len(names) == 1 else "{" + ", ".join(names) + "}"
        head = "::".join(parent)
        lines.append((cfg + "\n" if cfg else "") + (f"use {head}::{body};" if head else f"use {body};"))
    for kind, cfg, p in order:
        lines.append((cfg + "\n" if cfg else "") + f"use {'::'.join(p)}::*;")
    return lines


# -- include rebasing and use insertion ------------------------------------------------------------------------------------------
INC = re.compile(r'(include_(?:str|bytes)!\(\s*)"([^"]+)"(\s*\))')
PATHATTR = re.compile(r'(#\[\s*path\s*=\s*)"([^"]+)"(\s*\])')


def rebase_paths(text, src_file, dst_file):
    """Re-bases the relative literals of include_str!/include_bytes!/#[path] so that a text moved from src_file to dst_file still names the same files."""
    sd, dd = os.path.dirname(src_file), os.path.dirname(dst_file)

    def fix(m):
        lit = m.group(2)
        if os.path.isabs(lit):
            return m.group(0)
        target = os.path.normpath(os.path.join(sd, lit))
        return m.group(1) + '"' + os.path.relpath(target, dd) + '"' + m.group(3)

    clean = blank(text)
    out, last = [], 0
    for rx in (INC, PATHATTR):
        pass
    # blank() erases string contents, so match the call heads on the blanked text and read literals from the original
    edits = []
    for m in re.finditer(r'include_(?:str|bytes)!\(\s*"', text):
        if clean[m.start():m.start() + 8] != "include_":
            continue
        mm = INC.match(text, m.start())
        if mm:
            edits.append((mm.start(), mm.end(), fix(mm)))
    for m in re.finditer(r'#\[\s*path\s*=\s*"', text):
        mm = PATHATTR.match(text, m.start())
        if mm:
            edits.append((mm.start(), mm.end(), fix(mm)))
    edits.sort()
    for s, e, r_ in edits:
        out.append(text[last:s])
        out.append(r_)
        last = e
    out.append(text[last:])
    return "".join(out)


def add_use(text, line):
    """Inserts a `use` line before the first top-level `use`, or after the leading inner docs."""
    if line in text:
        return text
    m = re.search(r"(?m)^use ", text)
    if m:
        return text[:m.start()] + line + "\n" + text[m.start():]
    lines = text.split("\n")
    i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].startswith("#![")):
        i += 1
    lines[i:i] = [line, ""] if i == 0 else ["", line]
    return "\n".join(lines)


def uses_name(text, name):
    return re.search(r"(?<![A-Za-z0-9_$])" + re.escape(name) + r"\b", blank(text)) is not None


def used_idents(text):
    """Identifiers that head a path or an `extern crate` in the code of `text`."""
    clean = blank(text)
    out = set(re.findall(r"(?<![A-Za-z0-9_:$.])([A-Za-z_][A-Za-z0-9_]*)\s*::", clean))
    out |= set(re.findall(r"extern\s+crate\s+([A-Za-z0-9_]+)", clean))
    return out


def all_test_files():
    return [REPO + "/" + p for p in test_sources() if "/🧪️tests/" in p or "worker-cell" in p or "outcome" in p or "sqlite" in p or "test-hosts" in p]


# -- fallback resolution through the export index of all renderer crates ------------------------------------------------------------
import exports as EXP

_INDEX = [None]
KEYWORDS_AND_STD = set("""as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn
bool char str u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 String Vec Option Some None Result Ok Err Box Default Clone Copy Send Sync Sized Drop Fn FnMut FnOnce Iterator IntoIterator From Into ToString ToOwned AsRef AsMut
PartialEq Eq PartialOrd Ord Debug Display Arc Rc Mutex HashMap HashSet BTreeMap BTreeSet VecDeque Cell RefCell Ordering Duration Instant""".split())


def export_index():
    if _INDEX[0] is None:
        _INDEX[0] = EXP.build_index()
    return _INDEX[0][0]


def defining_crates():
    export_index()
    return _INDEX[0][1]


def is_test_crate(suffix):
    return suffix.startswith("shell-tests-") or suffix.endswith("-tests") or "-tests-" in suffix or suffix in ("fixtures", "test-support")


def fallback_imports(suffix, files, have_locals, local_defs):
    """Use lines for names the host indexes did not resolve but exactly one renderer crate exports. Lowercase names only when called or used as a path head."""
    idx = export_index()
    text = "\n".join(read(f) for f in files)
    clean = blank(text)
    toks = tokens(text)
    lines, unresolved = [], []
    seen = {}
    for t in sorted(toks):
        if t in have_locals or t in local_defs or t in KEYWORDS_AND_STD or t not in idx:
            continue
        owners = [c for c in idx[t] if c != suffix and not is_test_crate(c)]
        if len(owners) > 1:
            definers = [c for c in owners if c in defining_crates().get(t, ())]
            if len(definers) == 1:
                owners = definers
        if not owners:
            continue
        if t[0].islower() and not re.search(r"(?<![A-Za-z0-9_$.])" + re.escape(t) + r"\s*(?:\(|::)", clean):
            continue
        if len(owners) > 1:
            unresolved.append((t, owners))
            continue
        c = owners[0]
        cfgs = [x for x in idx[t][c]]
        cfg = cfgs[0] if len(set(cfgs)) == 1 and cfgs[0] else ""
        seen.setdefault((cfg, c), []).append(t)
    for (cfg, c), names in sorted(seen.items()):
        names = sorted(names)
        body = names[0] if len(names) == 1 else "{" + ", ".join(names) + "}"
        lines.append((cfg + "\n" if cfg else "") + f"use {lib_name(c)}::{body};")
    return lines, unresolved


def resolved_locals(lines):
    """Local names bound by the given use lines."""
    out = set()
    for cfg, vis, p, local, glob in uses("\n".join(lines)):
        if not glob and local:
            out.add(local)
    return out


def full_imports(suffix, files, local_defs=()):
    """needs_bin imports plus the fallback; returns (aliases, merged use lines, unresolved ambiguous names)."""
    aliases, lines = imports(suffix, files)
    have = resolved_locals(lines) | {m for a in aliases for m in re.findall(r"as ([A-Za-z0-9_]+);", a)}
    defs = set(local_defs)
    for f in files:
        defs |= {i["name"] for i in itm.table(read(f)) if i["kind"] not in ("impl", "use")}
        defs |= {l for c, v, p, l, g in uses(read(f), top_only=True) if l}
    extra, unresolved = fallback_imports(suffix, files, have, defs)
    return aliases, lines + extra, unresolved


# -- the fixture homes (names -> crate paths) ---------------------------------------------------------------------------------------
def register_fixtures():
    fa, er, ea, ss = lib_name("frame-actions"), lib_name("engine-registry"), lib_name("engine-attach"), lib_name("scene-state")
    root = {
        "collect_fixture_actions": [fa],
        "engine_surface_law_guard": [er],
        "ENGINE_SURFACE_LAW_LOCK": [er],
        "with_engine_close_context": [er],
        "drop_engine_surface": [ea],
        "retire_engine_surface_fixture": [ea],
        "camera_fixture_grant": [ss],
        "camera_fixture_lease": [ss],
        "clear_camera_fixture": [ss],
        "sweep_expired_scene_camera_dispatches": [ss],
    }
    for name, p in root.items():
        EXTRA_ROOT[name] = p + [name]
    EXTRA_ROOT["camera_storage"] = ["semio_framework_os_renderer_camera"]
    for n in ("camera_fixture_grant", "camera_fixture_lease", "clear_camera_fixture", "sweep_expired_scene_camera_dispatches"):
        EXTRA_FACADE["scenes"][n] = [ss, n]
    EXTRA_FACADE["engine_canvas"]["retire_engine_surface_fixture"] = [ea, "retire_engine_surface_fixture"]
    for n in ("engine_surface_law_guard", "with_engine_close_context"):
        EXTRA_FACADE["engine_canvas"][n] = [er, n]
    for k, path in root.items():
        B.EXTRA_P1.setdefault(k, [])
    FIXTURE_P1.update({k: [("", p + [k])] for k, p in root.items()})
    shell = json.load(open(SCRATCH + "/fx-shell-names.json"))["names"] if os.path.exists(SCRATCH + "/fx-shell-names.json") else []
    for n in shell:
        FIXTURE_P1[n] = [("", [lib_name("fixtures"), "shell", n])]



register_fixtures()
