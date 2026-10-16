"""Plan-driven Rust source-file splitter of the svm slice.

usage: split.py <plan.py> [--install]

Reads the pristine snapshot of a monolith, cuts it into child modules by line range, regenerates imports, region markers and
visibility, and stages the result under 🗑️generated/svm-work/<name>/stage (mirroring the real directory). `--install` re-checks the
real file against the snapshot and writes the staged files into the real tree. Writes only to the sandbox unless `--install`."""
import collections, hashlib, importlib.util, json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from rslex import lex, outline
from rsmask import mask, idents, parse_use, render_use

ROOT = "/Users/ueli/Documents/semio"
F = os.path.dirname(HERE)
WORK = F + "/🗑️generated/svm-work"
REGION = re.compile(r'^(\s*)//\s*#(end)?region\s+(.*?)\s*$')
DECL_KW = {"fn", "struct", "enum", "trait", "const", "static", "type", "union"}
EXPORT_KW = DECL_KW | {"macro_rules!"}
FN_LINE = re.compile(r'\s*(pub(?:\([^)]*\))?\s+)?((?:const\s+|async\s+|unsafe\s+)*)fn\s+(\w+)')
FIELD_LINE = re.compile(r'\s*(pub(?:\([^)]*\))?\s+)?(?:r#)?([A-Za-z_]\w*)\s*:(?!:)')


def read(p):
    with open(p, encoding="utf-8") as f:
        return f.read()


def write(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf-8") as f:
        f.write(t)


def load_plan(path):
    spec = importlib.util.spec_from_file_location("plan", path)
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m.PLAN


def sha(t):
    return hashlib.sha256(t.encode("utf-8")).hexdigest()


def impl_info(head_text):
    h = re.sub(r'\s+', ' ', head_text)
    m = re.match(r'\s*(?:unsafe\s+)?impl\s*(<.*?>)?\s*(.*)', h)
    rest = m.group(2) if m else h
    rest = rest.split(' where ')[0].split('{')[0].strip()
    if ' for ' in rest:
        tr, ty = rest.split(' for ', 1)
        return True, tr.strip(), ty.strip()
    return False, None, rest.strip()


def type_name(ty):
    m = re.search(r'[A-Za-z_]\w*', ty.replace('&', ' ').replace("'_", ' ').replace("'a", ' '))
    return m.group(0) if m else None


class Item:
    def __init__(self, d, lines, depths, owner):
        self.__dict__.update(d)
        self.owner = owner
        self.decl = self.start
        while self.decl < self.end and (lines[self.decl].strip().startswith(("///", "#[", "//", "#!")) or depths[self.decl] != 0):
            self.decl += 1
        self.text = "\n".join(lines[self.start:self.end + 1])


class Refs:
    """Receiver-aware references of one file (or the foreign descendants): non-self `.name`/`.name(`, `self.name` per impl type,
    `Type::name` paths and struct literal field lists."""

    def __init__(self):
        self.recv_fields = set()
        self.recv_methods = set()
        self.self_fields = set()
        self.self_methods = set()
        self.path_names = set()
        self.literal = set()
        self.ids = set()


def scan_refs(mlines, impl_types, refs):
    text = "\n".join(mlines)
    starts = []
    pos = 0
    for l in mlines:
        starts.append(pos)
        pos += len(l) + 1

    def line_of(off):
        lo, hi = 0, len(starts) - 1
        while lo < hi:
            mid = (lo + hi + 1) // 2
            if starts[mid] <= off:
                lo = mid
            else:
                hi = mid - 1
        return lo

    refs.ids |= idents(text)
    for m in re.finditer(r'\.\s*([A-Za-z_0-9]\w*)\s*(\(|::<)?', text):
        name, call = m.group(1), m.group(2)
        k = m.start() - 1
        while k >= 0 and text[k] in ' \t\n':
            k -= 1
        j = k
        while j >= 0 and (text[j].isalnum() or text[j] == '_'):
            j -= 1
        recv = text[j + 1:k + 1]
        prev = text[j] if j >= 0 else ''
        is_self = recv == 'self' and prev != '.'
        it = impl_types[line_of(m.start())]
        if is_self:
            if it:
                (refs.self_methods if call else refs.self_fields).add((it, name))
        else:
            (refs.recv_methods if call else refs.recv_fields).add(name)
    for m in re.finditer(r'\b([A-Za-z_]\w*)\s*::\s*([A-Za-z_]\w*)', text):
        tn, name = m.group(1), m.group(2)
        if tn == 'Self':
            tn = impl_types[line_of(m.start())]
        if tn:
            refs.path_names.add((tn, name))
    for m in re.finditer(r'\b([A-Za-z_]\w*)\s*\{', text):
        tn = m.group(1)
        if text[:m.start()].rstrip().endswith('::') and tn != 'Self':
            continue
        if tn == 'Self':
            tn = impl_types[line_of(m.start())]
        if not tn:
            continue
        depth = 0
        i = m.end() - 1
        j = i
        while j < len(text):
            if text[j] in '{([':
                depth += 1
            elif text[j] in '})]':
                depth -= 1
                if depth == 0:
                    break
            j += 1
        body = text[i + 1:j]
        d = 0
        cur = []
        parts = []
        for ch in body:
            if ch in '{([<':
                d += 1
            elif ch in '})]>':
                d -= 1
            if ch == ',' and d == 0:
                parts.append(''.join(cur))
                cur = []
            else:
                cur.append(ch)
        parts.append(''.join(cur))
        for part in parts:
            pm = re.match(r'\s*(?:ref\s+|mut\s+)*([A-Za-z_]\w*)\s*(?::(?!:)|$)', part)
            if pm:
                refs.literal.add((tn, pm.group(1)))


def main():
    plan = load_plan(sys.argv[1])
    plan["no_widen"] = list(plan.get("no_widen", [])) + json.loads(os.environ.get("SVM_EXTRA_NO_WIDEN", "[]"))
    install = "--install" in sys.argv
    quiet = "--quiet" in sys.argv
    name = plan["name"]
    orig_text = read(plan["orig"] if os.path.isabs(plan["orig"]) else F + "/" + plan["orig"])
    lines = orig_text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    N = len(lines)
    real_path = ROOT + "/" + plan["real"]
    real_dir = os.path.dirname(real_path)
    children = plan["children"]
    owner = [None] * N
    for c in children:
        for a, b in c["ranges"]:
            for i in range(a - 1, b):
                assert owner[i] is None, f"line {i+1} claimed twice ({owner[i]} and {c['key']})"
                owner[i] = c["key"]
    for i in range(N):
        if owner[i] is None:
            owner[i] = "parent"
    keys = ["parent"] + [c["key"] for c in children]
    depths, inlit = lex("\n".join(lines))
    items = []
    for d in outline("\n".join(lines)):
        it = Item(d, lines, depths, owner[d["start"]])
        for k in range(it.start, it.end + 1):
            assert owner[k] == it.owner, f"item {it.name or it.head} lines {it.start+1}-{it.end+1} straddles owners ({owner[k]} vs {it.owner} at {k+1})"
        items.append(it)

    stack_at = [None] * (N + 1)
    stack = []
    for i in range(N):
        stack_at[i] = list(stack)
        m = REGION.match(lines[i])
        if m:
            if m.group(2):
                assert stack and stack[-1][0] == m.group(3), f"unbalanced region at {i+1}"
                stack.pop()
            else:
                stack.append((m.group(3), lines[i]))
    assert not stack, "orig regions unbalanced"
    stack_at[N] = []

    use_stmts = []
    parent_use_lines = set()
    for it in items:
        if it.kind != "use":
            continue
        s0 = it.start
        while s0 < it.end and (REGION.match(lines[s0]) or not lines[s0].strip()):
            s0 += 1
        text = "\n".join(lines[s0:it.end + 1])
        head, trees = parse_use(mask(text))
        head = re.sub(r'\s+', ' ', head).strip()
        head = (head + " ") if head else ""
        use_stmts.append(dict(item=it, head=head, trees=trees, text=text, s0=s0))
        if it.owner == "parent":
            parent_use_lines.update(range(s0, it.end + 1))
    all_use_lines = set()
    for u in use_stmts:
        all_use_lines.update(range(u["s0"], u["item"].end + 1))

    masked_all = mask("\n".join(lines)).split("\n")
    impl_of_line = [None] * N
    for it in items:
        if it.kind == "impl":
            _, _, ty = impl_info(lines[it.decl])
            tn = type_name(ty)
            for k in range(it.start, it.end + 1):
                impl_of_line[k] = tn

    refs = {}
    for k in keys:
        ml, il = [], []
        for i in range(N):
            if owner[i] == k and i not in all_use_lines:
                ml.append(masked_all[i])
                il.append(impl_of_line[i])
        refs[k] = Refs()
        scan_refs(ml, il, refs[k])

    foreign = {}
    foreign_test = {}
    for i, l in enumerate(lines):
        m = re.match(r'\s*#\[path\s*=\s*"([^"]+)"\]', l)
        if m and owner[i] == "parent":
            p = os.path.normpath(os.path.join(real_dir, m.group(1)))
            if os.path.exists(p):
                k = i - 1
                while k >= 0 and (lines[k].strip().startswith("#[") or not lines[k].strip()):
                    if "cfg(test)" in lines[k]:
                        foreign_test[p] = True
                    k -= 1
                foreign[p] = read(p)
                foreign_test.setdefault(p, False)
    for p in plan.get("foreign", []):
        pp = p if os.path.isabs(p) else ROOT + "/" + p
        foreign[pp] = read(pp)
        foreign_test.setdefault(pp, False)
    refs["foreign"] = Refs()
    foreign_prod_ids, foreign_test_ids = set(), set()
    for p, t_ in foreign.items():
        ml = mask(t_).split("\n")
        scan_refs(ml, [None] * len(ml), refs["foreign"])
        (foreign_test_ids if foreign_test[p] else foreign_prod_ids).update(idents("\n".join(ml)))

    def others_of(k):
        return [x for x in refs if x != k]

    decls = {}
    for it in items:
        if it.name and it.kind in EXPORT_KW:
            assert it.name not in decls, f"duplicate top-level name {it.name}"
            decls[it.name] = it
    trait_methods = {}
    for it in items:
        if it.kind == "trait":
            ms = set()
            for k in range(it.decl + 1, it.end + 1):
                mm = FN_LINE.match(lines[k])
                if mm and depths[k] == 1:
                    ms.add(mm.group(3))
            trait_methods[it.name] = ms

    inherent_methods = set()
    for it in items:
        if it.kind == "impl":
            is_trait, _, _ = impl_info(lines[it.decl])
            if not is_trait:
                for k in range(it.decl + 1, it.end + 1):
                    mm = FN_LINE.match(lines[k])
                    if mm and depths[k] == 1:
                        inherent_methods.add(mm.group(3))

    widen = collections.OrderedDict()
    notes = []

    def need_widen(line_idx, why):
        if any(s in why for s in plan.get("no_widen", [])):
            notes.append(("no_widen", why))
            return
        widen.setdefault(line_idx, why)

    for it in items:
        if it.owner == "parent" or it.kind == "use":
            continue
        ow = it.owner
        if it.kind in EXPORT_KW and it.name and not it.vis:
            used = [k for k in others_of(ow) if it.name in refs[k].ids]
            if used:
                need_widen(it.decl, f"{it.kind} {it.name} (used in {used})")
        if it.kind == "struct" and it.name:
            hd = lines[it.decl]
            tup = re.match(r'(\s*)(?:pub(?:\([^)]*\))?\s+)?struct\s+\w+(?:<[^>]*>)?\s*\(\s*(pub(?:\([^)]*\))?\s+)?', hd)
            if tup and not tup.group(2) and not hd.rstrip().endswith('{'):
                tuple_hit = [k for k in others_of(ow) if (it.name, "0") in refs[k].self_fields or "0" in refs[k].recv_fields]
                if tuple_hit and not any(s in f"tuple field of {it.name}" for s in plan.get("no_widen", [])):
                    widen[("tuple", it.decl)] = f"tuple field of {it.name} (used in {tuple_hit})"
            for k in range(it.decl + 1, it.end + 1):
                if depths[k] != 1:
                    continue
                fm = FIELD_LINE.match(lines[k])
                if fm and not fm.group(1):
                    fname = fm.group(2)
                    hit = [x for x in others_of(ow) if fname in refs[x].recv_fields or (it.name, fname) in refs[x].self_fields or (it.name, fname) in refs[x].literal]
                    if hit:
                        need_widen(k, f"field {it.name}.{fname} (used in {hit})")
        if it.kind == "impl":
            is_trait, tr, ty = impl_info(lines[it.decl])
            tn = type_name(ty)
            if not is_trait and tn:
                for k in range(it.decl + 1, it.end + 1):
                    if depths[k] != 1:
                        continue
                    mm = FN_LINE.match(lines[k])
                    if mm and not mm.group(1):
                        mname = mm.group(3)
                        hit = [x for x in others_of(ow) if mname in refs[x].recv_methods or (tn, mname) in refs[x].self_methods or (tn, mname) in refs[x].path_names]
                        if hit:
                            need_widen(k, f"method {tn}::{mname} (used in {hit})")
                    cm = re.match(r'\s*const\s+(\w+)\s*:', lines[k])
                    if cm and any((tn, cm.group(1)) in refs[x].path_names for x in others_of(ow)):
                        need_widen(k, f"assoc const {tn}::{cm.group(1)}")

    for ln in plan.get("force_widen", []):
        widen.setdefault(ln - 1, "forced by plan")

    changed = True
    while changed:
        changed = False
        for ln in list(widen):
            if isinstance(ln, tuple):
                continue
            mm = FN_LINE.match(lines[ln])
            fm_ = FIELD_LINE.match(lines[ln])
            if not mm and not fm_:
                continue
            sig = []
            k = ln
            if mm:
                while k < N and '{' not in masked_all[k] and ';' not in masked_all[k]:
                    sig.append(masked_all[k])
                    k += 1
                sig.append(masked_all[min(k, N - 1)].split('{')[0])
            else:
                sig.append(masked_all[ln].split(':', 1)[1] if ':' in masked_all[ln] else masked_all[ln])
            sig_ids = idents("\n".join(sig))
            for n_ in sig_ids:
                d = decls.get(n_)
                if d and d.owner == owner[ln] and d.kind in ("struct", "enum", "type", "trait") and not d.vis and d.decl not in widen:
                    why = f"{d.kind} {n_} (named in the signature of widened line {ln+1})"
                    if any(s in why for s in plan.get("no_widen", [])):
                        continue
                    widen[d.decl] = why
                    changed = True

    imported = collections.OrderedDict()
    for u in use_stmts:
        for t in u["trees"]:
            if t.glob:
                imported[("*", u["item"].start)] = (u, t)
                continue
            b = t.alias or (t.path[-2] if t.path[-1] == "self" else t.path[-1])
            assert b not in imported, f"duplicate import {b}"
            imported[b] = (u, t)

    def render_group(need):
        out = []
        for _, (u, ts) in need.items():
            if len(ts) == len(u["trees"]):
                out.extend(u["text"].split("\n"))
            else:
                out.extend(render_use(u["head"], ts))
        return out

    def imports_for(pool):
        need = collections.OrderedDict()
        for b, (u, t) in imported.items():
            if isinstance(b, tuple) or b in pool:
                need.setdefault(id(u), (u, []))[1].append(t)
        return need

    parent_pool = set(refs["parent"].ids) | foreign_prod_ids | set(plan.get("parent_keep_imports", []))
    parent_need = collections.OrderedDict()
    test_need = collections.OrderedDict()
    for b, (u, t) in imported.items():
        pub = u["head"].strip().startswith("pub") or "cfg" in u["head"]
        if isinstance(b, tuple) or pub or b in parent_pool:
            parent_need.setdefault(id(u), (u, []))[1].append(t)
        elif b in foreign_test_ids:
            test_need.setdefault(id(u), (u, []))[1].append(t)

    def render_test_group(need):
        out = []
        for _, (u, ts) in need.items():
            out.extend(render_use("#[cfg(test)] " + u["head"], ts))
        return out

    child_uses, child_super = {}, {}
    for c in children:
        k = c["key"]
        pool = set(refs[k].ids) | set(c.get("force_import_ids", []))
        child_uses[k] = render_group(imports_for(pool))
        sup = []
        for n_, it in decls.items():
            if it.owner == k:
                continue
            if n_ in refs[k].ids:
                sup.append(n_)
            elif it.kind == "trait" and any(m_ in refs[k].recv_methods and m_ not in inherent_methods and sum(m_ in tm for tm in trait_methods.values()) == 1 for m_ in trait_methods.get(n_, ())):
                sup.append(n_)
        child_super[k] = sorted(set(sup) - set(c.get("no_super", [])))

    def widen_line(text):
        m = re.match(r'(\s*)(.*)$', text)
        return m.group(1) + "pub(super) " + m.group(2)

    def assemble(key, header, insert_after=None, insert_lines=(), skip_lines=frozenset(), regen_at=None, regen_lines=()):
        res = list(header)
        sstack = []
        prev_owned = False
        used_regen = False
        for i in range(N):
            if owner[i] != key:
                prev_owned = False
                continue
            if i in skip_lines:
                if regen_at is not None and i == regen_at and not used_regen:
                    res.extend(regen_lines)
                    used_regen = True
                if insert_after is not None and i == insert_after:
                    res.append("")
                    res.extend(insert_lines)
                    res.append("")
                continue
            if not prev_owned:
                want = stack_at[i]
                common = 0
                while common < len(sstack) and common < len(want) and sstack[common][0] == want[common][0]:
                    common += 1
                while len(sstack) > common:
                    nm, ml = sstack.pop()
                    res.append(ml.replace("#region", "#endregion", 1))
                for nm, ml in want[common:]:
                    res.append(ml)
                    sstack.append((nm, ml))
            prev_owned = True
            m = REGION.match(lines[i])
            if m:
                if m.group(2):
                    if sstack and sstack[-1][0] == m.group(3):
                        sstack.pop()
                        res.append(lines[i])
                else:
                    if not (sstack and sstack[-1][0] == m.group(3)):
                        sstack.append((m.group(3), lines[i]))
                        res.append(lines[i])
            else:
                text = lines[i]
                if i in widen:
                    text = widen_line(text)
                if ("tuple", i) in widen:
                    text = re.sub(r'(struct\s+\w+(?:<[^>]*>)?\s*\()', r'\1pub(super) ', text, count=1)
                res.append(text)
            if insert_after is not None and i == insert_after:
                res.append("")
                res.extend(insert_lines)
                res.append("")
        while sstack:
            nm, ml = sstack.pop()
            res.append(ml.replace("#region", "#endregion", 1))
        col = []
        for l in res:
            if l.strip() == "" and col and col[-1].strip() == "":
                continue
            col.append(l)
        return "\n".join(col).rstrip("\n") + "\n"

    out_files = {}
    for c in children:
        k = c["key"]
        hdr = [f"//! {c['doc']}", ""]
        hdr.extend(child_uses[k])
        if child_super[k]:
            hdr.append(f"use super::{child_super[k][0]};" if len(child_super[k]) == 1 else "use super::{" + ", ".join(child_super[k]) + "};")
        hdr.extend(c.get("extra_uses", []))
        hdr.append("")
        out_files[f"{real_dir}/{c['folder']}/🦀️.rs"] = assemble(k, hdr)

    mount = []
    for c in children:
        k = c["key"]
        any_pub = any(it.owner == k and it.vis == "pub" and it.kind in EXPORT_KW and it.name for it in items)
        mount.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
        mount.append(f'mod {c["mod"]};')
        mount.append(f'{"pub " if any_pub else ""}use {c["mod"]}::*;')
    ma = plan.get("mount_after")
    if ma is None:
        ma = max(u["item"].end for u in use_stmts if u["item"].owner == "parent")
    else:
        ma -= 1
    assert owner[ma] == "parent", f"mount_after line {ma+1} is not parent-owned"
    first_use = min(u["s0"] for u in use_stmts if u["item"].owner == "parent") if parent_use_lines else None
    out_files[real_path] = assemble("parent", [], insert_after=ma, insert_lines=mount, skip_lines=frozenset(parent_use_lines), regen_at=first_use, regen_lines=render_group(parent_need) + render_test_group(test_need))

    stage = f"{WORK}/{name}/stage"
    if os.path.exists(stage):
        shutil.rmtree(stage)
    for p, t in out_files.items():
        write(f"{stage}/{os.path.relpath(p, ROOT)}", t)

    write(f"{WORK}/{name}/widen.json", json.dumps({str(k): v for k, v in widen.items()}, ensure_ascii=False, indent=1))
    if quiet:
        return
    print(f"[{name}] orig {N} lines; widened {len(widen)} declarations")
    for k, v in widen.items():
        print(f"   widen line {(k[1] if isinstance(k, tuple) else k) + 1}: {v}")
    for c in children:
        print(f"   child {c['key']}: uses={child_uses[c['key']]} super={child_super[c['key']]}")
    for p, t in out_files.items():
        print(f"   {os.path.relpath(p, real_dir)}: {t.count(chr(10))} lines")
    print("   notes:", notes)
    write(f"{WORK}/{name}/widen.json", json.dumps({str(k): v for k, v in widen.items()}, ensure_ascii=False, indent=1))

    if install:
        assert sha(read(real_path)) == sha(orig_text), "REAL FILE CHANGED SINCE SNAPSHOT: aborting install"
        for p in out_files:
            if p != real_path:
                assert not os.path.exists(p), f"refusing to overwrite {p}"
        for p, t in out_files.items():
            if p != real_path:
                write(p, t)
        write(real_path, out_files[real_path])
        print("INSTALLED")


if __name__ == "__main__":
    main()
