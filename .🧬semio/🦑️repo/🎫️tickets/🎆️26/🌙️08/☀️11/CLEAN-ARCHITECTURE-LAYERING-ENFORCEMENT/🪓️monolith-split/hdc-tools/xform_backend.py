"""Turns the forwarding layout of one hub directory backend into capability-trait impls, staged outside the repo.
Reads the pre-edit snapshot (SC/hdc-orig), writes SC/hdc-stage/<backend>/..."""
import os, re, sys, shutil, collections
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
from portparse import parse_members

ORIG = SC + "/hdc-orig/🌎️hub/📇️directory"
STAGE = SC + "/hdc-stage"
RELOCATE = [("admin_overview_counts", "👤️accounts", "🛡️administration", "AdminOperations", "start"), ("rebuild_projections", "📜️event-log", "📽️projections", "🔖️Projections", "end"), ("rebuild_projections_controlled", "📜️event-log", "📽️projections", "🔖️Projections", "end")]
DOC_REF = re.compile(r"HubDirectory::(\w+)")
DOC_FIXES = [
    ("`HubDirectory` over SQLite (rusqlite)", "The directory capabilities over SQLite (rusqlite)"),
    ("(satisfying the shared `HubDirectory` interface)", "(satisfying the shared capability traits of `semio_hub_directory`)"),
    ("`HubDirectory` over PostgreSQL", "The directory capabilities over PostgreSQL"),
    ("`HubDirectory` over Neo4j (neo4rs)", "The directory capabilities over Neo4j (neo4rs)"),
]
VIS = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)(async\s+fn|fn)\b")


def find_impl_blocks(lines, struct):
    """[(start_idx, end_idx)] of top-level `impl Struct {` blocks (end = index of closing '}')."""
    out = []
    i = 0
    while i < len(lines):
        if lines[i] == "impl %s {" % struct:
            j = i + 1
            while lines[j] != "}":
                j += 1
            out.append((i, j))
            i = j
        i += 1
    return out


def fix_docs(line):
    return DOC_REF.sub(lambda m: "%s::%s" % (TRAIT_OF[m.group(1)], m.group(1)) if m.group(1) in TRAIT_OF else m.group(0), line)


def strip_vis(first_line):
    return VIS.sub(lambda m: m.group(1) + m.group(2), first_line, count=1)


def member_text(member, keep_vis):
    lines = list(member["doc"]) + list(member["lines"])
    if not keep_vis:
        k = len(member["doc"])
        lines[k] = strip_vis(lines[k])
    return [fix_docs(l) for l in lines]


def relocate(sources, struct):
    """sources: {section: text}. Moves the RELOCATE members between section files (text level)."""
    texts = dict(sources)
    for name, src, dst, region, where in RELOCATE:
        s_lines = texts[src].split("\n")
        blocks = find_impl_blocks(s_lines, struct)
        found = None
        for a, b in blocks:
            items = parse_members(s_lines[a + 1:b])
            idx = 0
            pos = a + 1
            for it in items:
                n = len(it["lines"]) + (len(it["doc"]) if it["kind"] == "member" else 0)
                if it["kind"] == "member" and it["name"] == name:
                    found = (pos, pos + n, items, a, b)
                pos += n
        assert found, (name, src)
        p0, p1, items, a, b = found
        moved = s_lines[p0:p1]
        # drop one adjacent blank line
        if p1 < len(s_lines) and s_lines[p1] == "":
            del s_lines[p0:p1 + 1]
        else:
            del s_lines[p0:p1]
        texts[src] = "\n".join(s_lines)
        d_lines = texts[dst].split("\n")
        dblocks = find_impl_blocks(d_lines, struct)
        ins = None
        for a, b in dblocks:
            for k in range(a + 1, b):
                if d_lines[k].startswith("    //#region") and region in d_lines[k] and ins is None:
                    if where == "start":
                        ins = k + 1
                    else:
                        e = k + 1
                        while not d_lines[e].startswith("    //#endregion"):
                            e += 1
                        ins = e
        if ins is None:
            a, b = dblocks[-1]
            ins = b
        if where == "start":
            add = moved + [""]
        else:
            add = [""] + moved if d_lines[ins - 1].strip() != "" else moved
        d_lines[ins:ins] = add
        texts[dst] = "\n".join(d_lines)
    return texts


def transform_section(text, struct, section, traits):
    """Returns (new_text, implemented_traits)."""
    lines = text.split("\n")
    blocks = find_impl_blocks(lines, struct)
    trait_names = [t for t, _ in traits]
    method_trait = {m: t for t, ms in traits for m in ms}
    out = []
    cursor = 0
    implemented = collections.OrderedDict()
    for a, b in blocks:
        out += lines[cursor:a]
        cursor = b + 1
        items = parse_members(lines[a + 1:b])
        i = 0
        inh = []          # inherent member blocks (list of (blank_before, lines))
        region = None     # dict(open=line, inh=[], tr=OrderedDict)
        buf_blank = False
        def flush_inherent(buf):
            res = []
            if buf:
                res.append("impl %s {" % struct)
                for k, (bb, ls) in enumerate(buf):
                    if k and bb:
                        res.append("")
                    res += ls
                res.append("}")
            return res
        def emit_traits(tr):
            res = []
            for t, mems in sorted(tr.items(), key=lambda kv: trait_names.index(kv[0])):
                implemented[t] = True
                if res:
                    res.append("")
                res.append("impl %s for %s {" % (t, struct))
                for k, (bb, ls) in enumerate(mems):
                    if k and bb:
                        res.append("")
                    res += ls
                res.append("}")
            return res
        pieces = []
        top = dict(inh=[], tr=collections.OrderedDict())

        def flush_top():
            body = flush_inherent(top["inh"])
            tr = emit_traits(top["tr"])
            if body or tr:
                pieces.append(body + ([""] if body and tr else []) + tr)
            top["inh"] = []
            top["tr"] = collections.OrderedDict()

        for it in items:
            kind = it["kind"]
            if kind == "blank":
                buf_blank = True
                continue
            if kind == "marker":
                line = it["lines"][0]
                if line.strip().startswith("//#region"):
                    flush_top()
                    region = dict(open=line[4:], inh=[], tr=collections.OrderedDict())
                else:
                    assert region is not None, line
                    body = flush_inherent(region["inh"])
                    tr = emit_traits(region["tr"])
                    seg = [region["open"]] + body + ([""] if body and tr else []) + tr
                    pieces.append(("REGION", seg, line[4:], region))
                    region = None
                buf_blank = False
                continue
            holder = region if region else top
            if kind == "other":
                holder["inh"].append((buf_blank, it["lines"]))
                buf_blank = False
                continue
            name = it["name"]
            if name in method_trait:
                holder["tr"].setdefault(method_trait[name], []).append((buf_blank, member_text(it, keep_vis=False)))
            else:
                holder["inh"].append((buf_blank, member_text(it, keep_vis=True)))
            buf_blank = False
        flush_top()
        assert region is None
        # emit pieces
        first = True
        for pc in pieces:
            if isinstance(pc, tuple):
                _, seg, close, reg = pc
                if not first:
                    out.append("")
                out += seg
                out.append(close)
            else:
                if not first:
                    out.append("")
                out += pc
            first = False
    out += lines[cursor:]
    return "\n".join(out), list(implemented)


def add_empty_impls(text, struct, section, traits, implemented):
    """Backends that do not implement a whole trait declare the opt-out with an empty impl (defaults)."""
    missing = [t for t, _ in traits if t not in implemented]
    if not missing:
        return text
    lines = text.split("\n")
    for t in missing:
        prev = [x for x, _ in traits][[x for x, _ in traits].index(t) - 1]
        # place after the impl block of the previous trait in section order
        idx = None
        for k, l in enumerate(lines):
            if l == "impl %s for %s {" % (prev, struct):
                idx = k
        assert idx is not None, (t, prev, section)
        j = idx
        while lines[j] != "}":
            j += 1
        add = ["", "/// 🚫️ This backend stores no agent delegations: the trait defaults answer `unavailable`.", "impl %s for %s {}" % (t, struct)]
        lines[j + 1:j + 1] = add
    return "\n".join(lines)


def main():
    backend_dir = sys.argv[1]
    struct = BACKENDS[backend_dir]
    root_orig = ORIG + "/" + backend_dir
    sections = {}
    for sec in SECTIONS:
        p = root_orig + "/" + sec + "/🦀️.rs"
        if os.path.exists(p):
            sections[sec] = read(p)
    sections = relocate(sections, struct)
    shutil.rmtree(STAGE + "/" + backend_dir, ignore_errors=True)
    report = {}
    for sec, text in sections.items():
        traits = SECTIONS[sec]
        new, impl = transform_section(text, struct, sec, traits)
        new = add_empty_impls(new, struct, sec, traits, impl)
        write(STAGE + "/" + backend_dir + "/" + sec + "/🦀️.rs", new)
        report[sec] = impl
    # the untouched-by-trait children get doc fixes only (connection, rows, schema, creation-v1 tests are not staged)
    # root
    root = read(root_orig + "/🦀️.rs")
    cut = root.index("/// 🚪️ The port adapter")
    adapter_end = root.index("\n}\n", root.index("impl HubDirectory for %s {" % struct)) + 3
    tail = root[adapter_end:]
    head = root[:cut]
    assert head.rstrip().endswith("mod event_log;"), head[-80:]
    new_root = head.rstrip("\n") + "\n" + tail
    traits_all = sorted(t for sec in SECTIONS.values() for t, _ in sec)
    out_lines = new_root.split("\n")
    hit = [i for i, l in enumerate(out_lines) if l.startswith("use semio_hub_directory::{") and "HubDirectory" in l]
    assert len(hit) == 1
    out_lines[hit[0]] = merge_use_list(out_lines[hit[0]], {"HubDirectory"}, traits_all)
    new_root = "\n".join(out_lines)
    for old_t, new_t in DOC_FIXES:
        new_root = new_root.replace(old_t, new_t)
    write(STAGE + "/" + backend_dir + "/🦀️.rs", new_root)
    print(backend_dir, {k: v for k, v in report.items()})


if __name__ == "__main__":
    main()
