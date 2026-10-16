"""pfs Rust file splitter: a hub file keeps its docs, uses and unclaimed lines; claimed line ranges become child modules
`<folder>/🦀️.rs` mounted with `#[path]` and glob re-exported; private items used outside their child are widened to
`pub(super)` (usage-based, frs.widen_edits); every child starts with `use super::*;`. Output goes to the stage dir only.
usage: pfs_rs.py <plan-module>   (plan modules define SPEC)"""
import os, re, sys, importlib, collections, json
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
from rslex import lex, outline, multiline_literal_lines
from stubgen import mask
import splitter
from splitter import tokenize, member_refs

SERDE_ATTR = re.compile(r'(?:deserialize_with|serialize_with|with|default|skip_serializing_if|getter|from|into|try_from)\s*=\s*"([A-Za-z_][A-Za-z0-9_:]*)"')


class Shim(splitter.Plan):
    def __init__(self):
        pass

    def widen2(self, body, ext_first, ext_all, ext_fields, ext_methods):
        """Usage-based widening: items by call/path use (fn) or any token (types, consts); members only of types that other files can name."""
        text = "\n".join(body) + "\n"
        depths, inlit = lex(text)
        edits = {}
        visible = set()
        items = outline(text)
        for it in items:
            kind = it["kind"]
            hi = self._head_index(body, it)
            head = body[hi]
            name = it["name"]
            if kind in ("fn", "struct", "enum", "trait", "const", "static", "type", "union") and name:
                if it["vis"]:
                    visible.add(name)
                    continue
                used = name in ext_first if kind == "fn" else name in ext_all
                if used:
                    m = re.match(r"^(\s*)(.*)$", head)
                    edits[hi] = (m.group(1) + "pub(super) " + m.group(2), f"{kind} {name}")
                    visible.add(name)
        structs = {it["name"] for it in items if it["kind"] in ("struct", "enum") and it["name"]}
        signatures = []
        for it in items:
            if it["kind"] == "fn" and it["name"] in visible and not it["vis"]:
                hi = self._head_index(body, it)
                k = hi
                sig = body[k]
                while "{" not in sig and k + 1 < len(body) and k < it["end"]:
                    k += 1
                    sig += " " + body[k]
                signatures.append(sig.split("{")[0])
        flow = set()
        for sig in signatures:
            for m in re.finditer(r"[A-Za-z_]\w*", sig):
                if m.group(0) in structs:
                    visible.add(m.group(0))
                    flow.add(m.group(0))
        for it in items:
            kind = it["kind"]
            hi = self._head_index(body, it)
            if kind == "struct" and it["name"] in visible:
                self._widen_struct(body, it, hi, depths, ext_all | ({it["name"]} if it["name"] in flow else set()), ext_fields, edits)
            if kind == "impl":
                head = body[hi]
                m = re.match(r"^\s*impl(?:<[^>]*>)?\s+([A-Za-z_]\w*)", head)
                if m and m.group(1) in visible and not re.search(r"\bfor\b", head.split("{")[0]):
                    self._widen_impl(body, it, hi, depths, ext_methods, edits)
        for it in items:
            if it["kind"] in ("struct", "enum") and it["name"] in flow and not it["vis"]:
                hi = self._head_index(body, it)
                if hi in edits:
                    nl, label = edits[hi]
                    edits[hi] = (re.sub(r"^(\s*)", r"\1pub(super) ", nl, count=1), label + " + struct")
                else:
                    m = re.match(r"^(\s*)(.*)$", body[hi])
                    edits[hi] = (m.group(1) + "pub(super) " + m.group(2), f"{it['kind']} {it['name']} (flows out of a widened fn)")
        return [(hi, nl, label) for hi, (nl, label) in sorted(edits.items())]


def blank(l):
    return l.strip() == ""


def build(spec):
    name = spec["target"]
    t = TARGETS[name]
    src = read(snap_dir(name) + "/" + t["file"])
    ls = lines_of(src)
    assert not multiline_literal_lines(src), "multi-line literals present"
    cov = Coverage(len(ls))
    base = os.path.dirname(t["file"])
    children = spec["children"]
    for c in children:
        for a, b in c["ranges"]:
            cov.claim(c["mod"], a, b)
    for k, o in enumerate(cov.owner):
        if k and o is None:
            pass
    hub_lines = [i for i in range(1, len(ls) + 1) if cov.owner[i] is None]
    for i in hub_lines:
        if not blank(ls[i - 1]) and i not in spec.get("hub_keep", set()) and not any(a <= i <= b for a, b in spec.get("hub_keep_ranges", [])):
            raise SystemExit(f"unclaimed non-blank line {i}: {ls[i-1][:80]}")
    bodies = {}
    for c in children:
        body = []
        for a, b in c["ranges"]:
            body.extend(ls[a - 1:b])
        while body and blank(body[-1]):
            body.pop()
        while body and blank(body[0]):
            body.pop(0)
        bodies[c["mod"]] = body
    hub_text = "\n".join(ls[i - 1] for i in hub_lines)
    shim = Shim()
    out_files = {}
    report = {}
    for c in children:
        mod = c["mod"]
        others = hub_text + "\n" + "\n".join("\n".join(bodies[m]) for m in bodies if m != mod)
        ext_first, ext_all = tokenize(others)
        ext_fields, ext_methods = member_refs(others)
        for m in SERDE_ATTR.finditer(others.replace("\\\"", "")):
            ext_all.add(m.group(1).split("::")[-1])
            ext_first.add(m.group(1).split("::")[-1])
        body = list(bodies[mod])
        rep = []
        if c.get("widen", True):
            for ln, newline, label in shim.widen2(body, ext_first, ext_all, ext_fields, ext_methods):
                body[ln] = newline
                rep.append(label)
        for forced in c.get("force_pub_super", []):
            hit = [i for i, l in enumerate(body) if re.match(rf"^\s*(?:async\s+)?(?:fn|struct|enum|const|static|type)\s+{forced}\b", l)]
            assert len(hit) == 1, (mod, forced, hit)
            i = hit[0]
            if not body[i].lstrip().startswith("pub"):
                m = re.match(r"^(\s*)(.*)$", body[i])
                body[i] = m.group(1) + "pub(super) " + m.group(2)
                rep.append(f"forced {forced}")
        header = c["doc"]
        uses = c.get("uses", ["use super::*;"])
        text = "\n".join(header) + "\n\n" + "\n".join(uses) + "\n\n" + "\n".join(body) + "\n"
        rel = os.path.normpath(os.path.join(base, c["folder"], "🦀️.rs"))
        out_files[rel] = text
        report[mod] = rep
    hub = []
    first_line = {c["mod"]: min(a for a, _ in c["ranges"]) for c in children}
    inserted = set()
    pending = collections.OrderedDict()
    anchors = collections.defaultdict(list)
    for c in children:
        anchors[first_line[c["mod"]]].append(c)
    docs_after = spec.get("hub_doc_after")
    for i in range(1, len(ls) + 1):
        if i in anchors:
            for c in anchors[i]:
                blk = []
                blk.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
                blk.append(f"mod {c['mod']};")
                if c.get("glob", True):
                    blk.append(f"{c.get('reexport', '')}{' ' if c.get('reexport') else ''}use {c['mod']}::*;")
                hub.extend(blk)
                hub.append("")
        if cov.owner[i] is not None:
            continue
        if spec.get("hub_drop") and i in spec["hub_drop"]:
            continue
        hub.append(spec.get("hub_sub", {}).get(ls[i - 1], ls[i - 1]))
        if docs_after and i == docs_after:
            hub.extend(spec["hub_doc"])
    # collapse blank runs in hub
    cleaned = []
    for l in hub:
        if blank(l) and cleaned and blank(cleaned[-1]):
            continue
        cleaned.append(l)
    while cleaned and blank(cleaned[-1]):
        cleaned.pop()
    out_files[t["file"]] = "\n".join(cleaned) + "\n"
    return out_files, report, ls


def stage(name, out_files):
    import shutil
    sd = stage_dir(name)
    if os.path.exists(sd):
        shutil.rmtree(sd)
    for rel, txt in out_files.items():
        write(os.path.join(sd, rel), txt)


def losslessness(ls, out_files):
    new = []
    for txt in out_files.values():
        new.extend(lines_of(txt))
    gone, added = diff_multiset(ls, new)
    return gone, added


def main():
    plan = importlib.import_module(sys.argv[1])
    out, report, ls = build(plan.SPEC)
    name = plan.SPEC["target"]
    stage(name, out)
    gone, added = losslessness(ls, out)
    print("files:", {os.path.relpath(k, os.path.dirname(TARGETS[name]["file"])): len(lines_of(v)) for k, v in out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:3000])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:3000])
    for m, r in report.items():
        print("widened", m, len(r), r[:80])


if __name__ == "__main__":
    main()
