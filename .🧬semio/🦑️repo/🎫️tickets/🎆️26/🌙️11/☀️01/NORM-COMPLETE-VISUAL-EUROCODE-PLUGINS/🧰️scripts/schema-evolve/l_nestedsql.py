"""🪆 Nested collections in the learned SQLite codecs (every artifact but en1997, whose templates live in `l_sqlite`): the pair clone.

The parent is cloned from a flat reference exactly like a flat collection, the child is cloned from the same kind of flat reference and captured instead of inserted; the captured child statements are then merged into the parent's statements, in the dialect the artifact's own nested donors show: the child rows are written inside the parent's write loop (`let id=..insert(..)?;` and a loop over `parent.children`), counted and bounded inside the parent's counting and bound loops, read as groups keyed by the parent row (`groups(entities(..),&parent_rows,..)` and `ordered(groups.remove(&parent.rowid)..)`, or `siblings(&groups,parent.rowid)`) and attached to the parent record. Anything that does not line up reports `Unsupported` with the measured reason."""
import re
from core import Unsupported, singular_of, snake
from lex import code_mask, fn_bodies, match_close, split_items, statements
import l_codec as C


def last_token(table, short):
    """🔑 `en1993_crane_runway` -> `runway`: the foreign key column of a child table is named after the last word of its parent table."""
    return table[len(short) + 1:].split("_")[-1]


def fk_column(ddl, short, parent_tbl):
    """🪢 The child table's `document_id` reference becomes a reference to the parent row table."""
    pattern = re.compile(r"\bdocument_id(\s+INTEGER NOT NULL\s+REFERENCES\s+)\w+(\(id\))")
    if not pattern.search(ddl):
        raise Unsupported("the child DDL has no document_id reference to turn into a parent reference")
    return pattern.sub(lambda m: f"{last_token(parent_tbl, short)}_id{m.group(1)}{parent_tbl}{m.group(2)}", ddl, count=1)


def body_span(stmt, lang="rs"):
    """🫙 (open, close) of the `{ .. }` body of a `for` statement (the first brace outside parentheses and brackets)."""
    mask = code_mask(stmt, lang)
    depth = 0
    for k, ch in enumerate(stmt):
        if not mask[k]:
            continue
        if ch in "([":
            depth += 1
        elif ch in ")]":
            depth -= 1
        elif ch == "{" and depth == 0:
            return k, match_close(stmt, mask, k)
    raise Unsupported(f"no loop body in {stmt[:80]!r}")


def call_args(text, open_at, lang="rs"):
    """📞 Argument spans of the call whose `(` is at `open_at`: (items, index of the closing parenthesis)."""
    mask = code_mask(text, lang)
    close = match_close(text, mask, open_at)
    return split_items(text, mask, open_at + 1, close), close


def loop_var(head):
    """🔁 The element variable of a `for` header: `for(n,v)in` -> v, `for v in` -> v."""
    m = re.match(r"\s*for\s*\(\s*\w+\s*,\s*(\w+)\s*\)\s*in\b", head) or re.match(r"\s*for\s+(\w+)\s+in\b", head)
    if not m:
        raise Unsupported(f"loop header not understood: {head[:80]!r}")
    return m.group(1)


def entities_call(text):
    """🗃️ The `entities(..)?` call (with its `?`) inside a statement: (start, end) or None."""
    m = re.search(r"\bentities\(", text)
    if not m:
        return None
    items, close = call_args(text, m.end() - 1)
    end = close + 1
    if text[end:end + 1] == "?":
        end += 1
    return m.start(), end


class RsInfo:
    """📇 What the Rust merge needs about the pair: the entities, their tables and the row id binding the artifact's donors use."""

    def __init__(self, ctx, parent, child, ptbl, ctbl, text):
        self.ctx, self.S, self.n, self.ptbl, self.ctbl = ctx, parent, child, ptbl, ctbl
        self.field = child.names.plural_snake
        self.cv = self.field + "_list"
        self.group = f"{self.field}_by_{parent.names.snake}"
        ids = re.findall(r"\blet\s+(\w+)\s*=\s*\w+\.insert\w*\(\s*\"[a-z0-9_]+\"", text)
        self.idv = max(set(ids), key=ids.count) if ids else "id"
        self.remove_into = ""
        m = re.search(r"ordered\(\s*\w+\.remove\(&\w+\.rowid\)\.unwrap_or_default\(\)(\s*\.into_iter\(\))?", text)
        if m and m.group(1):
            self.remove_into = ".into_iter()"


def rs_class(new, tbl, entity):
    """🏷️ Role of a cloned Rust statement: W write loop, ENT rows declaration, GRP groups declaration, LOOP read loop, B bound loop, VEC vector declaration."""
    s = new.strip()
    if f'"{tbl}"' in s and re.search(r"\binsert\w*\(", s):
        return "W"
    if re.match(r"let\s+(?:mut\s+)?\w+\s*=\s*groups\(", s):
        return "GRP"
    if re.search(r"\b%s\s*\{" % re.escape(entity), s) and ".push(" in s:
        return "LOOP"
    if f'"{tbl}"' in s and "entities(" in s and s.startswith("let"):
        return "ENT"
    if re.match(r"let\s+mut\s+\w+\s*=\s*Vec::new\(\)\s*;", s):
        return "VEC"
    if s.startswith("for") and re.search(r"\b(?:add|repeated)\(", s):
        return "B"
    return "OTHER"


def nest_loop(parent_stmt, child_loop, info, pv, source_re):
    """🪺 Appends a copy of the child loop to the end of the parent loop body; the child's iteration source (`self.children`) turns into `parent_var.children`."""
    o, c = body_span(child_loop)
    head = child_loop[:o]
    head, n = re.subn(source_re, lambda m: m.group(1) + pv + "." + info.field, head, count=1)
    if not n:
        raise Unsupported(f"child loop does not iterate {info.field}: {head[:80]!r}")
    inner = head + child_loop[o:]
    po, pc = body_span(parent_stmt)
    gap = "" if parent_stmt[:pc].rstrip().endswith(("}", ";")) else ";"
    return parent_stmt[:pc].rstrip() + gap + inner + parent_stmt[pc:]


def source_pattern(info):
    return r"(\bin\s+&?)(?:\w+\.)*" + re.escape(info.field) + r"\b"


def item_append(stmt, entity, item):
    """➕ Adds a member to the struct literal `Entity{..}` of a statement (behind the last member)."""
    m = re.search(r"\b%s\s*\{" % re.escape(entity), stmt)
    if not m:
        raise Unsupported(f"struct literal of {entity} not found")
    close = match_close(stmt, code_mask(stmt, "rs"), m.end() - 1)
    inner = stmt[:close].rstrip()
    return inner + ("" if inner.endswith(",") else ",") + " " + item + stmt[len(inner):close] + stmt[close:]


def merge_rust(rc, clones, cap, info):
    """🧩 Merges the captured child statements `cap` into the parent's cloned statements `clones` (see the module docstring)."""
    ctx, S, n = info.ctx, info.S, info.n
    parent = [dict(c, cls=rs_class(c["new"], info.ptbl, S.entity)) for c in clones]
    child = [dict(c, cls=rs_class(c["new"], info.ctbl, n.entity)) for c in cap]
    by = lambda rows, cls: next((r for r in rows if r["cls"] == cls), None)
    pw, cw = by(parent, "W"), by(child, "W")
    if not pw or not cw:
        raise Unsupported("write statements of the reference not found for the pair")
    ploop, cloop = by(parent, "LOOP"), by(child, "LOOP")
    if not ploop or not cloop:
        raise Unsupported("read loops of the reference not found for the pair")
    idv = info.idv
    receiver = re.search(r"(?<![\w.])(\w+(?:\.\w+)*\.insert\w*\(\s*\"%s\")" % re.escape(info.ptbl), pw["new"])
    if not receiver:
        raise Unsupported("insert call of the parent not found")
    pv = loop_var(pw["new"][: body_span(pw["new"])[0]])
    pw["new"] = pw["new"][: receiver.start(1)] + f"let {idv}=" + pw["new"][receiver.start(1):]
    cw_text = re.sub(r"Cell::Integer\(\s*1\s*\)", f"Cell::Integer({idv})", cw["new"], count=1)
    pw["new"] = nest_loop(pw["new"], cw_text, info, pv, source_pattern(info))
    child.remove(cw)
    pb, cb = by(parent, "B"), by(child, "B")
    if pb and cb:
        bv = loop_var(pb["new"][: body_span(pb["new"])[0]])
        pb["new"] = nest_loop(pb["new"], cb["new"], info, bv, source_pattern(info))
        child.remove(cb)
    sibling = "siblings(" in ploop["new"]
    if sibling:
        merge_siblings(rc, parent, child, ploop, cloop, info)
    else:
        merge_groups(rc, parent, child, ploop, cloop, info)
    extra = count_loops(rc, info)
    out = [{k: v for k, v in r.items() if k != "cls"} for r in parent]
    out += extra
    return out


def child_loop_text(cloop, info, header_fn):
    """🧵 The child's read loop rewritten to build the parent's child vector: header from `header_fn`, pushes into the local vector."""
    o, _ = body_span(cloop["new"])
    head = header_fn(cloop["new"][:o])
    text = head + cloop["new"][o:]
    text = re.sub(r"(?:\b\w+\.)*%s\.(push|len)\(" % re.escape(info.field), lambda m: f"{info.cv}.{m.group(1)}(", text)
    return f"let mut {info.cv}=Vec::new();" + text


def attach(ploop, info, pv_code):
    """🧷 Child loop at the start of the parent's loop body, the vector as a member of the parent literal."""
    o, _ = body_span(ploop["new"])
    ploop["new"] = ploop["new"][: o + 1] + pv_code + ploop["new"][o + 1:]
    ploop["new"] = item_append(ploop["new"], info.S.entity, f"{info.field}: {info.cv}")


def merge_groups(rc, parent, child, ploop, cloop, info):
    """🗂️ Dialect of `groups(entities(..),&parent_rows,ctrl)?` + `ordered(groups.remove(&p.rowid)..,p.rowid,ctrl)?`."""
    pent = next((r for r in parent if r["cls"] == "ENT"), None)
    cent = next((r for r in child if r["cls"] == "ENT"), None)
    pv = loop_var(ploop["new"][: body_span(ploop["new"])[0]])
    prefix = ""
    if pent:
        m = re.match(r"\s*let\s+(?:mut\s+)?(\w+)\s*=", pent["new"])
        prow = m.group(1)
    else:
        a, b = entities_call(ploop["new"][: body_span(ploop["new"])[0]]) or (None, None)
        if a is None:
            raise Unsupported("rows of the parent are read neither by a declaration nor by an entities(..) call in the loop header")
        prow = f"{info.S.names.snake}_rows"
        call = ploop["new"][a:b]
        ploop["new"] = ploop["new"][:a] + prow + ploop["new"][b:]
        prefix = f"let {prow}={call if call.endswith('?') else call + '?'};"
    if cent:
        m = re.search(r"=\s*(entities\(.*\)\?)\s*;?\s*$", cent["new"], re.S)
        if not m:
            raise Unsupported("rows declaration of the child not understood")
        ccall = m.group(1)
        child.remove(cent)
    else:
        o, _ = body_span(cloop["new"])
        span = entities_call(cloop["new"][:o])
        if not span:
            raise Unsupported("rows of the child are read neither by a declaration nor by an entities(..) call in the loop header")
        ccall = cloop["new"][span[0]:span[1]]
        ccall = ccall if ccall.endswith("?") else ccall + "?"
    entities_args, _ = call_args(ccall, ccall.index("("))
    ctrl = ccall[entities_args[-1][0]:entities_args[-1][1]]
    space = ", " if ", " in ccall else ","
    groups = f"let mut {info.group}=groups({ccall}{space}&{prow}{space}{ctrl})?;"
    if pent:
        pent["new"] += "\n" + groups if "\n" in pent["new"] else " " + groups
    else:
        ploop["new"] = prefix + groups + ploop["new"]

    def header(head):
        m = re.search(r"\bordered\(", head)
        if not m:
            raise Unsupported(f"read loop header of the child has no ordered(..): {head[:80]!r}")
        args, close = call_args(head, m.end() - 1)
        if len(args) != 3:
            raise Unsupported("ordered(..) of the child has no three arguments")
        sep = ", " if ", " in head[m.end():close] else ","
        new_args = [f"{info.group}.remove(&{pv}.rowid).unwrap_or_default(){info.remove_into}", f"{pv}.rowid", head[args[2][0]:args[2][1]]]
        return head[: m.end()] + sep.join(new_args) + head[close:]
    attach(ploop, info, child_loop_text(cloop, info, header))
    child.remove(cloop)


def merge_siblings(rc, parent, child, ploop, cloop, info):
    """🗂️ Dialect of `groups(&child_rows,&parent_rows,ctrl)?` + `siblings(&groups,parent.rowid)`."""
    pent = next((r for r in parent if r["cls"] == "ENT"), None)
    if not pent:
        raise Unsupported("rows declaration of the parent not found")
    prow = re.match(r"\s*let\s+(?:mut\s+)?(\w+)\s*=", pent["new"]).group(1)
    cgrp = next((r for r in child if r["cls"] == "GRP"), None)
    cent = next((r for r in child if r["cls"] == "ENT"), None)
    if not cgrp:
        raise Unsupported("groups declaration of the child not found")
    pgrp = next((r for r in parent if r["cls"] == "GRP"), None)
    if not pgrp:
        raise Unsupported("groups declaration of the parent not found")
    m = re.match(r"\s*let\s+(?:mut\s+)?(\w+)\s*=\s*groups\(", cgrp["new"])
    old = m.group(1)
    args, close = call_args(cgrp["new"], cgrp["new"].index("groups(") + len("groups"))
    sep = ", " if ", " in cgrp["new"] else ","
    decl = f"let {info.group}=groups({cgrp['new'][args[0][0]:args[0][1]]}{sep}&{prow}{sep}{cgrp['new'][args[2][0]:args[2][1]]})?;"
    pgrp["new"] += (" " if "\n" not in pgrp["new"] else "\n") + decl
    if cent:
        pent["new"] += (" " if "\n" not in pent["new"] else "\n") + cent["new"]
        child.remove(cent)
    child.remove(cgrp)
    pv = loop_var(ploop["new"][: body_span(ploop["new"])[0]])

    def header(head):
        out, n = re.subn(r"siblings\(\s*&\s*%s\s*,\s*1\s*\)" % re.escape(old), f"siblings(&{info.group},{pv}.rowid)", head, count=1)
        if not n:
            raise Unsupported(f"read loop header of the child does not use siblings(&{old},1): {head[:80]!r}")
        return out
    attach(ploop, info, child_loop_text(cloop, info, header))
    child.remove(cloop)


def count_loops(rc, info):
    """🧮 Per-parent counting loops of the artifact's chains (`for v in &self.donors{add(v.children.len())?;}`) cloned for the new parent and its child; the donors are the loops over the reference parent's plural that count its nested array."""
    ctx = info.ctx
    if not ctx.nref:
        return []
    donor_plural = ctx.nref.plural_snake
    nested = ctx.nested_ref_names().plural_snake
    stmts, _ = rc.stmts()
    out = []
    text = rc.text
    for a, b in stmts:
        s = text[a:b]
        if not s.startswith("for"):
            continue
        try:
            o, c = body_span(s)
        except Unsupported:
            continue
        head, body = s[:o], s[o:]
        if not re.search(r"(?:\bself|\b\w+)\.%s\b" % re.escape(donor_plural), head) or ".len()" not in body:
            continue
        lens = set(re.findall(r"\.(\w+)\.len\(\)", body)) - {donor_plural}
        if nested not in lens:
            continue
        new_body = reduce_counts(body, nested, lens, info.field)
        if new_body is None:
            continue
        renamed = re.sub(r"\.%s\b" % re.escape(donor_plural), "." + info.S.names.plural_snake, head + new_body)
        out.append({"a": a, "b": b, "orig": s, "new": renamed, "ind": C.indent_of(text, a)})
    return out


def reduce_counts(body, nested, lens, child):
    """➗ The donor loop body counts its nested arrays; the clone counts only the child: `add(add(rows,x.a.len())?,x.b.len())?` -> `add(rows,x.child.len())?`; None when the body mixes other counts."""
    others = lens - {nested}
    if not others:
        return re.sub(r"\.%s\.len\(\)" % re.escape(nested), f".{child}.len()", body)
    pattern = re.compile(r"add\(add\(([^,()]+),\s*(\w+)\.\w+\.len\(\)\)\?,\s*\2\.\w+\.len\(\)\)\?")
    if len(lens) == 2:
        new, n = pattern.subn(lambda m: f"add({m.group(1)},{m.group(2)}.{child}.len())?", body)
        if n:
            return new
    return None


def run_rust(ctx, S, n, pf, cf, sql):
    """🦀 Child capture, then the parent clone with the merge."""
    cap, holder = [], {}

    def child(ref, tbl, rf, sql_):
        cap.clear()
        return C.rust_generic(ctx, ref, tbl, rf, cf, sql_, target=n, capture=cap)
    rcc, _ = C.try_refs(ctx, child, "Rust codec (child)")
    ctbl = rcc.new_tbl

    def parent(ref, tbl, rf, sql_):
        ptbl = C.new_table(ctx, ref, tbl, S.names)
        text = ctx.plan.read(ctx.layout.p(C.SQLITE + "/🦀️.rs"))
        info = RsInfo(ctx, S, n, ptbl, ctbl, text)
        return C.rust_generic(ctx, ref, tbl, rf, pf, sql_, merge=lambda rc, clones: merge_rust(rc, clones, cap, info))
    return C.try_refs(ctx, parent, "Rust codec (parent)")


TS_FOR = re.compile(r"for\(let (\w+)=0;\1<([\w$.]+)\.length;\1\+\+\)\{const (\w+)=\2\[\1\]!")
TS_ORDERED = re.compile(r"\bordered\(")


def fresh(taken, choices):
    return next(c for c in choices if not re.search(r"(?<![\w$.])%s(?![\w$])" % c, taken))


def ts_rename(text, old, new):
    return re.sub(r"(?<![\w$.])%s(?![\w$])" % re.escape(old), new, text)


class TsInfo:
    """📇 What the TypeScript merge needs about the pair."""

    def __init__(self, ctx, parent, child, ptbl, ctbl, text, child_type):
        self.ctx, self.S, self.n, self.ptbl, self.ctbl, self.child_type = ctx, parent, child, ptbl, ctbl, child_type
        self.field = child.names.plural_camel
        self.cv = self.field + "List"
        self.group = f"{self.field}By{parent.entity}"
        ids = re.findall(r"\b(?:const|let)\s+(\w+)\s*=\s*await\s+write\(", text)
        self.idv = max(set(ids), key=ids.count) if ids else "id"
        m = re.search(r"\b((?:\w*[gG]roups))\(\s*await\s+entities\(", text)
        self.groups_fn = m.group(1) if m else "groups"
        self.counters = re.search(r"\blet ((?:\w+Id=1n,?)+);", text)


def ts_write_parts(stmt, table):
    m = re.search(r"await write\(\s*(\w+)\s*,\s*\"%s\"\s*," % re.escape(table), stmt)
    if not m:
        raise Unsupported(f"write call of {table} not found")
    args, close = call_args(stmt, stmt.index("(", m.start()), "ts")
    return m, args, close


def ts_nest_write(pw, cw, info):
    """✍️ Parent write: the row id is bound (`const id=await write(..)`, or the explicit id expression is named), the child loop with its own variables is appended to the loop body."""
    idv = info.idv
    m, args, close = ts_write_parts(pw, info.ptbl)
    explicit = not pw[args[2][0]:args[2][1]].lstrip().startswith("[")
    if explicit:
        expr = pw[args[2][0]:args[2][1]]
        pw = pw[: m.start()] + f"const {idv}={expr};" + pw[m.start():args[2][0]] + idv + pw[args[2][1]:]
    else:
        pw = pw[: m.start()] + f"const {idv}=" + pw[m.start():]
    mp = TS_FOR.match(pw)
    if not mp:
        raise Unsupported("parent write loop has an unknown shape")
    pidx, psrc, pval = mp.groups()
    mc = TS_FOR.match(cw.strip())
    if not mc:
        raise Unsupported("child write loop has an unknown shape")
    cidx, csrc, cval = mc.groups()
    taken = pw
    nidx, nval = fresh(taken, "kjmq"), fresh(taken, "acdeb")
    inner = cw.strip()
    source = "\x01SRC\x01"
    inner = inner.replace(csrc, source)
    inner = ts_rename(ts_rename(inner, cidx, nidx), cval, nval).replace(source, f"{pval}.{info.field}")
    cm, cargs, cclose = ts_write_parts(inner, info.ctbl)
    if inner[cargs[2][0]:cargs[2][1]].lstrip().startswith("["):
        inner = re.sub(r"\[\s*1n\s*,", f"[{idv},", inner, count=1)
    else:
        if not info.counters:
            raise Unsupported("the artifact writes explicit row ids but declares no `let xId=1n` counters to extend")
        inner = inner[: cargs[2][0]] + f"{info.field}Id++" + inner[cargs[2][1]:]
        info.new_counter = f"{info.field}Id"
        inner = re.sub(r"\[\s*1n\s*,", f"[{idv},", inner, count=1)
    o, c = body_span(pw, "ts")
    if "fields(" in pw:
        pw = re.sub(r"(fields\(\s*%s\s*,\s*\[)([^\]]*)(\]\))" % re.escape(pval), lambda mm: mm.group(1) + mm.group(2) + f',"{info.field}"' + mm.group(3), pw, count=1)
        if re.search(r"\bcollection\(%s\." % re.escape(pval), pw) or re.search(r"function collection\b", info.ctx.plan.read(info.ctx.layout.p(C.SQLITE + "/🟦️.ts"))):
            pw = re.sub(r"(fields\(\s*%s\s*,\s*\[[^\]]*\]\);)" % re.escape(pval), lambda mm: mm.group(1) + f"collection({pval}.{info.field});", pw, count=1)
        o, c = body_span(pw, "ts")
    gap = "" if pw[:c].rstrip().endswith((";", "}")) else ";"
    return pw[:c].rstrip() + gap + inner + pw[c:], pval


def ts_loop_parts(loop):
    """🔄 (header end index = start of the body, element variable, pushed vector path) of a read loop; a braceless body is wrapped into braces."""
    m = re.match(r"for\(const (\w+) of ", loop)
    if not m:
        raise Unsupported(f"read loop not understood: {loop[:80]!r}")
    mask = code_mask(loop, "ts")
    close = match_close(loop, mask, 3)
    return m.group(1), close


def ts_wrap(loop):
    var, close = ts_loop_parts(loop)
    body = loop[close + 1:].strip()
    if not body.startswith("{"):
        loop = loop[: close + 1] + "{" + body.rstrip(";") + "}"
    return loop


def ts_member(loop, field, value):
    """➕ `field:value` as the last member of the first `.push({..})` object literal."""
    m = re.search(r"\.push\(\s*\{", loop)
    if not m:
        raise Unsupported("no push({..}) literal in the read loop")
    close = match_close(loop, code_mask(loop, "ts"), m.end() - 1)
    inner = loop[:close].rstrip()
    return inner + ("" if inner.endswith(",") else ",") + f"{field}:{value}" + loop[close:]


def ts_child_loop(cloop, info, header_fn, pv):
    """🧵 The child's read loop: iterates the parent's group, with a fresh element variable (a `for(const v of f(v))` header is a temporal dead zone), pushes into the local vector."""
    cloop = ts_wrap(cloop)
    cvar, close = ts_loop_parts(cloop)
    nvar = fresh(cloop + pv, "acdeqz")
    head, body = cloop[: close + 1], cloop[close + 1:]
    head = header_fn(head)
    body = ts_rename(body, cvar, nvar)
    head = head.replace(f"for(const {cvar} of", f"for(const {nvar} of", 1)
    text = head + body
    text = re.sub(r"(?:[\w$]+\.)*%s\.(push|length)" % re.escape(info.field), lambda m: f"{info.cv}.{m.group(1)}", text)
    return f"const {info.cv}:{info.child_type}[]=[];" + text


def merge_ts(tc, clones, cap, info):
    """🧩 TypeScript counterpart of `merge_rust`."""
    S, n = info.S, info.n
    cls = lambda c, tbl: "W" if f'"{tbl}"' in c["new"] and "write(" in c["new"] else ("LOOP" if re.search(r"\.push\(\s*\{", c["new"]) else "OTHER")
    parent = [dict(c, cls=cls(c, info.ptbl)) for c in clones]
    child = [dict(c, cls=cls(c, info.ctbl)) for c in cap]
    pw = next((c for c in parent if c["cls"] == "W"), None)
    cw = next((c for c in child if c["cls"] == "W"), None)
    ploop = next((c for c in parent if c["cls"] == "LOOP"), None)
    cloop = next((c for c in child if c["cls"] == "LOOP"), None)
    if not (pw and cw and ploop and cloop):
        raise Unsupported("write and read statements of the reference not found for the pair")
    _, wargs, _ = ts_write_parts(pw["new"], info.ptbl)
    o = pw["new"][wargs[-1][0]:wargs[-1][1]]
    pw["new"], _ = ts_nest_write(pw["new"], cw["new"], info)
    head_text = ploop["new"]
    pv, close = ts_loop_parts(head_text)
    if ".get(1n)" in head_text:
        gget = f"{info.group}.get({pv}.rowid)??[]"

        def header(head):
            out, k = re.subn(r"of\s+[\w$]+\.get\(1n\)\?\?\[\]", f"of {gget}", head, count=1)
            if not k:
                raise Unsupported(f"child read loop does not iterate Groups.get(1n): {head[:80]!r}")
            return out
    else:
        gget = f"{info.group}.get({pv}.rowid)??[]"

        def header(head):
            m = TS_ORDERED.search(head)
            if not m:
                raise Unsupported(f"child read loop has no ordered(..): {head[:80]!r}")
            args, c2 = call_args(head, m.end() - 1, "ts")
            if len(args) == 2:
                new_args = [gget, head[args[1][0]:args[1][1]]]
            elif len(args) == 3:
                new_args = [gget, f"{pv}.rowid", head[args[2][0]:args[2][1]]]
            else:
                raise Unsupported(f"ordered(..) of the child read loop takes {len(args)} arguments")
            return head[: m.end()] + ",".join(new_args) + head[c2:]
    child_code = ts_child_loop(cloop["new"], info, header, pv)
    ploop["new"] = ts_wrap(ploop["new"])
    pv, close = ts_loop_parts(ploop["new"])
    ccalls = getattr(info, "child_decls", {})
    if ".get(1n)" in head_text:
        old_rows = tc.ref_rows_name(tc.text)
        if not old_rows or len(ccalls) != 1:
            raise Unsupported("rows declarators of the reference not found for the Groups dialect")
        prow = tc.rename(old_rows)
        crow, ccall = next(iter(ccalls.items()))
        tc.declarator_extra = {"entities": f",{crow}={ccall}", "groups": f",{info.group}=await groups({crow},{prow},{o})"}
    else:
        header_part = ploop["new"][: close + 1]
        m = TS_ORDERED.search(header_part)
        if not m:
            raise Unsupported("parent read loop has no ordered(..)")
        pargs, pclose = call_args(header_part, m.end() - 1, "ts")
        a1 = header_part[pargs[0][0]:pargs[0][1]]
        prefix = ""
        em = re.search(r"await\s+entities\(", a1)
        if em:
            ea, ec = call_args(a1, em.end() - 1, "ts")
            prow = f"{S.names.camel}Rows"
            prefix = f"const {prow}={a1[em.start():ec + 1]};"
            ploop["new"] = ploop["new"][: pargs[0][0]] + f"{prow}.values()" + ploop["new"][pargs[0][1]:]
        else:
            vm = re.search(r"([\w$]+)\.values\(\)", a1)
            if not vm:
                raise Unsupported(f"rows of the parent not understood: {a1[:60]!r}")
            prow = vm.group(1)
        cheader = cloop["new"][: ts_loop_parts(cloop["new"])[1] + 1]
        cm = TS_ORDERED.search(cheader)
        cargs, _ = call_args(cheader, cm.end() - 1, "ts")
        ca1 = cheader[cargs[0][0]:cargs[0][1]]
        cem = re.search(r"await\s+entities\(", ca1)
        if cem:
            cea, cec = call_args(ca1, cem.end() - 1, "ts")
            ccall = ca1[cem.start():cec + 1]
        elif len(ccalls) == 1:
            ccall = next(iter(ccalls.values()))
        else:
            raise Unsupported("rows of the child not understood")
        decl = f"const {info.group}=await {info.groups_fn}({ccall},{prow},{o});"
        ploop["new"] = prefix + decl + ploop["new"]
        pv, close = ts_loop_parts(ploop["new"][len(prefix + decl):])
        close += len(prefix + decl)
        ploop["new"] = ploop["new"]
    ploop["new"] = ts_member(ploop["new"], info.field, info.cv)
    ploop["new"] = ploop["new"][: close + 2] + child_code + ploop["new"][close + 2:]
    info.parent_loop_text = ploop["new"]
    return [{k: v for k, v in c.items() if k != "cls"} for c in parent]


def patch_parent_alias(ctx, tc, info):
    """🧬 The parent's SQLite row type gets the child array (and omits the model member)."""
    pair = tc.type_pair
    if not pair:
        return
    path = tc.path
    text = ctx.plan.read(path)
    m = re.search(r"^export type %s=([^\n]*);\n" % re.escape(pair[1]), text, re.M)
    if not m:
        raise Unsupported(f"row type {pair[1]} not found")
    body, member = m.group(1), f"{info.field}:{info.child_type}[]"
    if ">&{" in body:
        cut = body.rfind(">&{")
        body = body[:cut] + f'|"{info.field}"' + body[cut:]
        body = body[:-1] + f";{member}}}"
    elif body.strip().startswith("{"):
        body = body.rstrip()[:-1] + f";{member}}}"
    else:
        body = f'Omit<{body},"{info.field}">&{{{member}}}'
    ctx.plan.write(path, text[: m.start(1)] + body + text[m.end(1):])


def run_ts(ctx, S, n, pf, cf, sql, n_tables):
    """🟦 Child capture, then the parent clone with the merge and the row type patch."""
    cap = []

    def child(ref, tbl, rf, sql_):
        cap.clear()
        return C.ts_generic(ctx, ref, tbl, rf, cf, sql_, n_tables + 1, target=n, capture=cap)
    tcc, _ = C.try_refs(ctx, child, "TypeScript codec (child)")
    ctype = tcc.type_pair[1] if tcc.type_pair else n.entity
    holder = {}

    def parent(ref, tbl, rf, sql_):
        ptbl = C.new_table(ctx, ref, tbl, S.names)
        text = ctx.plan.read(ctx.layout.p(C.SQLITE + "/🟦️.ts"))
        info = TsInfo(ctx, S, n, ptbl, tcc.new_tbl, text, ctype)
        info.child_decls = tcc.decls
        holder["info"] = info
        return C.ts_generic(ctx, ref, tbl, rf, pf, sql_, n_tables, merge=lambda tc, clones: merge_ts(tc, clones, cap, info))
    tcp, ref = C.try_refs(ctx, parent, "TypeScript codec (parent)")
    patch_parent_alias(ctx, tcp, holder["info"])
    counter = getattr(holder["info"], "new_counter", None)
    if counter:
        path = tcp.path
        text = ctx.plan.read(path)
        text = re.sub(r"(\blet (?:\w+Id=1n,?)+)(;)", lambda m: m.group(1) + f",{counter}=1n" + m.group(2), text, count=1)
        ctx.plan.write(path, text)
    return tcp, ref


def admission(ctx, S, n, ptbl, ctbl, field_id):
    """🛂 Native admission: the flat clone of the parent becomes `let v=n::record(v)?; let parent=n::entity(..)?;` plus a child loop; the row counter learns the child rows."""
    path = ctx.layout.p(C.SQLITE + "/🛂️admission/🦀️.rs")
    if not ctx.plan.exists(path):
        return
    rs = ctx.plan.read(ctx.layout.p(C.SQLITE + "/🦀️.rs"))
    C.try_refs(ctx, lambda ref, tbl, rf, s: C.adm_generic(ctx, ref, tbl, rf, C.spec_fields(S), ptbl, field_id), "native admission (parent)")
    text = ctx.plan.read(path)
    m = re.search(r"(for\(index,(\w+)\)in n::list\(n::field\(root,%d\)\?\)\?\.iter\(\)\.enumerate\(\)\{)n::entity\(out,\"%s\",1,index,&n::cells\(n::record\(\2\)\?,(\[.*?\])\)\?,([^)]*?)\)\?;\}" % (field_id, re.escape(ptbl)), text)
    if not m:
        raise Unsupported("admission statement of the parent not understood")
    var, cells, const = m.group(2), m.group(3), m.group(4)
    typed = "Enumeration(" in text
    import l_sqlite
    cell = lambda f, i: "Enumeration(%d,&[%s])" % (i, ",".join('"%s"' % x for x in f.tokens)) if typed and f.ty == "enum" else l_sqlite.adm_cell(f, i)
    ccells = "Text(0)" + "".join("," + cell(f, i + 1) for i, f in enumerate(n.fields))
    reals = any(f.ty == "f64" for f in n.fields)
    cc = re.search(r"const (\w*%s\w*)\s*:\s*&\[FloatColumn\]" % re.escape(snake(n.entity).upper()), rs)
    cconst = cc.group(1) if reals and cc else "&[]"
    k = 1 + len(S.fields)
    new = (f"{m.group(1)}let {var}=n::record({var})?;let parent=n::entity(out,\"{ptbl}\",1,index,&n::cells({var},{cells})?,{const})?;"
           f"for(index,a)in n::list(n::field({var},{k})?)?.iter().enumerate(){{n::entity(out,\"{ctbl}\",parent,index,&n::cells(n::record(a)?,[{ccells}])?,{cconst})?;}}}}")
    text = text[: m.start()] + new + text[m.end():]
    count = re.search(r"fn count\(root:&R\)->Result<usize,ValueError>\{.*?Ok\(rows\)\}", text, re.S)
    if count:
        body = count.group(0)
        covered = any(int(a) <= field_id <= int(b) for a, b in re.findall(r"for id in (\d+)\.\.=(\d+)\{rows=n::add", body))
        add = ("" if covered else f"rows=n::add(rows,n::list(n::field(root,{field_id})?)?.len())?;") + f"for owner in n::list(n::field(root,{field_id})?)?{{rows=n::add(rows,n::list(n::field(n::record(owner)?,{k})?)?.len())?}}"
        text = text[: count.end() - len("Ok(rows)}")] + add + "Ok(rows)}" + text[count.end():]
    ctx.plan.write(path, text)


def run(ctx):
    """🪆 The learned nested path: DDL pair, Rust, TypeScript, native admission, contract; a statement shape the merge cannot read is reported as `Unsupported`."""
    try:
        return run_all(ctx)
    except (IndexError, AttributeError, StopIteration, KeyError, ValueError) as error:
        raise Unsupported(f"nested pair clone: unexpected statement shape ({type(error).__name__}: {error})")


def run_all(ctx):
    S, L = ctx.spec.collection, ctx.layout
    if len(S.nested) != 1:
        raise Unsupported(f"{len(S.nested)} nested collections: the pair clone takes exactly one child per parent")
    n = S.nested[0]
    sql_path = L.p(C.SQLITE + "/🗄️.sql")
    sql = ctx.plan.read(sql_path)
    old_sql, n_tables = sql, len(C.tables_of(sql))
    pf, cf = C.spec_fields(S), C.spec_fields(n)
    pddl, pref = C.try_refs(ctx, lambda ref, tbl, rf, s: C.ddl_generic(ctx, s, ref, tbl, rf, C.new_table(ctx, ref, tbl, S.names), pf), "sqlite DDL (parent)")
    ptbl = C.new_table(ctx, pref, C.table_of(ctx, pref, sql), S.names)
    if f"CREATE TABLE {ptbl}" in sql:
        return
    cddl, cref = C.try_refs(ctx, lambda ref, tbl, rf, s: C.ddl_generic(ctx, s, ref, tbl, rf, C.new_table(ctx, ref, tbl, n.names), cf), "sqlite DDL (child)")
    ctbl = C.new_table(ctx, cref, C.table_of(ctx, cref, sql), n.names)
    new_sql = sql.rstrip("\n") + "\n" + pddl + fk_column(cddl, ctx.short, ptbl)
    ctx.plan.write(sql_path, new_sql)
    run_rust(ctx, S, n, pf, cf, new_sql)
    run_ts(ctx, S, n, pf, cf, new_sql, n_tables)
    admission(ctx, S, n, ptbl, ctbl, len(ctx.snapshot["properties"]))
    C.contract(ctx, old_sql, new_sql)
    ctx.plan.note(f"sqlite codecs: nested pair {S.names.plural_camel}/{n.names.plural_camel} cloned and merged in the artifact's own dialect ({ptbl}, {ctbl})")
