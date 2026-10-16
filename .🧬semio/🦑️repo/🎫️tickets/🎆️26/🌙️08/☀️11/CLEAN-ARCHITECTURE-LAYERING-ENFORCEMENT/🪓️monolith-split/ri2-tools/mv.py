#!/usr/bin/env python3
# 🚚️ Item mover of RENDERER-INTEGRATION-2: cuts the planned items (ri2-tools/moves.json) out of their source files and appends them to the (crate, slug) staging file of their new home crate.
# Region pairs are repeated balanced at the destination; relative `#[path]`/`include_str!` literals are rebased; the `use` items of the source file the moved text needs travel along; visibility is widened to `pub`.
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/w1")
sys.path.insert(0, HERE)
from model import *
from rsedit import Doc, wrap_regions
import flip as F

MOVES_PATH = os.environ.get("RI2_MOVES", HERE + "/moves.json")
MOVES = json.load(open(MOVES_PATH))
PUB_HEAD = re.compile(r"^(\s*)((?:async\s+|const\s+|unsafe\s+|extern\s+\"[^\"]*\"\s+)*)(fn|struct|enum|const|static|type|trait|mod|union)\b")


def staging_dir(suffix):
    return STG + "/" + next(d for d in os.listdir(STG) if staging_crate(d) == suffix)


def module_name(suffix, slug):
    return "staged_" + suffix[len("shell-"):].replace("-", "_") + "__" + re.sub(r"^[^A-Za-z0-9]+", "", slug).replace("-", "_")


def rebase(text, src_dir, dst_dir):
    def sub_path(m):
        rel = m.group(2)
        if rel.startswith("/"):
            return m.group(0)
        absolute = os.path.normpath(os.path.join(src_dir, rel))
        return m.group(1) + os.path.relpath(absolute, dst_dir) + m.group(3)
    text = re.sub(r'(#\[\s*path\s*=\s*")([^"]+)("\s*\])', sub_path, text)
    text = re.sub(r'(include(?:_str|_bytes)?!\(\s*")([^"]+)(")', sub_path, text)
    return text


def widen_item(lines):
    out = []
    head_done = False
    text = "\n".join(lines)
    if "thread_local!" in text:
        return [re.sub(r"^(\s+)static\b", r"\1pub static", l) if re.match(r"^\s+static\b", l) else l for l in lines]
    for i, l in enumerate(lines):
        if not head_done and not l.lstrip().startswith(("///", "#[", "//", "pub", "impl", "//#", "use")) and l.strip():
            m = PUB_HEAD.match(l)
            if m and l[0] not in " \t":
                l = "pub " + l
            head_done = True
        elif not head_done and l.lstrip().startswith(("pub", "impl")):
            head_done = True
        out.append(l)
    return out


def widen_struct_fields(lines):
    out = []
    body = "\n".join(lines)
    if not re.search(r"(?m)^(?:pub\s+)?struct\s", body):
        return lines
    for l in lines:
        m = re.match(r"^(    )(?!pub\b)([a-z_][A-Za-z0-9_]*)\s*:", l)
        out.append(("    pub " + l[4:]) if m else l)
    return out


def widen_impl(lines):
    if not lines or " for " in "\n".join(lines[:6]).split("{")[0]:
        return lines
    out = []
    for l in lines:
        m = re.match(r"^(    )((?:async\s+|const\s+|unsafe\s+)*)(fn|const)\b", l)
        out.append(("    pub " + l[4:]) if m and not l.startswith("    pub") else l)
    return out


def used_uses(src_text, moved_text):
    toks = tokens(moved_text)
    out = []
    for cfg, vis, p, local, glob in uses(src_text, top_only=True):
        if glob:
            continue
        if local in toks and p and p[0] not in ("super",):
            out.append((cfg, vis, p, local))
    return out


def header_for(suffix, slug):
    folder = TABLE_T[suffix]
    emoji = re.match(r"^(\S+?)(?=[a-z])", folder).group(1)
    return f"//! {emoji} `{suffix}` staging of `{re.sub(r'^[^A-Za-z0-9]+', '', slug)}`: RENDERER-INTEGRATION-2, the items the direction graph homes in this crate (they were in a higher crate or in the old chunk file).\n\nuse super::*;\n"


TABLE_T = crate_table()


def raw_head(doc, it):
    for l in doc.lines[it["s"]:it["e"] + 1]:
        if not l.lstrip().startswith(("///", "#[", "//")):
            return l.strip()
    return ""


def find_item(doc, rec):
    hits = []
    for it in doc.items():
        kind_ok = it["kind"] == rec["kind"] or (rec["kind"] == "static" and it["kind"] == "thread_local!")
        if kind_ok and raw_head(doc, it) == rec["head"]:
            if rec["head"].startswith("thread_local!") and not re.search(r"\bstatic\s+" + re.escape(rec["name"]) + r"\b", "\n".join(doc.lines[it["s"]:it["e"] + 1])):
                continue
            hits.append(it)
    return hits


TOUCHED = []


def pull(suffix, dry=False):
    plan = [m for m in MOVES if m["dest"] == suffix and not m.get("done")]
    by_src = collections.defaultdict(list)
    for m in plan:
        by_src[m["src"]].append(m)
    pending = collections.defaultdict(lambda: collections.defaultdict(list))
    extra_uses = collections.defaultdict(list)
    cut_texts = {}
    for src, recs in by_src.items():
        text = read(src)
        doc = Doc(text)
        seen_heads = set()
        for rec in recs:
            key = (rec["kind"], rec["head"])
            if key in seen_heads:
                continue
            seen_heads.add(key)
            hits = find_item(doc, rec)
            if not hits:
                raise SystemExit(f"item not found: {rec['name']} {rec['head']} in {src}")
            if len(hits) > 1:
                # several definitions with the same head (cfg variants): take them all
                pass
            for h in sorted(hits, key=lambda x: -x["s"]):
                if not doc.balanced(h["s"], h["e"]):
                    raise SystemExit(f"region crossing: {rec['name']} in {src}")
                stack = doc.stack[h["s"]]
                body = doc.cut(h["s"], h["e"])
                dst_dir = staging_dir(suffix) + "/" + rec["slug"]
                dst = dst_dir + "/🦀️.rs"
                body_text = rebase("\n".join(body), os.path.dirname(src), dst_dir)
                body = body_text.split("\n")
                body = widen_item(body)
                if rec["kind"] in ("struct",):
                    body = widen_struct_fields(body)
                if rec["kind"] == "impl":
                    body = widen_impl(body)
                pending[dst][tuple(stack)].insert(0, body)
                extra_uses[dst] += used_uses(text, "\n".join(body))
        cut_texts[src] = doc.text()
    result = []
    if not dry:
        F.backup(suffix, list(by_src) + list(pending))
    for dst, groups in pending.items():
        exists = os.path.exists(dst)
        old = read(dst) if exists else header_for(suffix, os.path.basename(os.path.dirname(dst)))
        have = {(c, "::".join(p), l) for c, v, p, l, g in uses(old, top_only=True)}
        add_uses = []
        for cfg, vis, p, local in extra_uses[dst]:
            key = (cfg, "::".join(p), local)
            if key in have:
                continue
            have.add(key)
            leaf = p[-1]
            piece = leaf if leaf == local else f"{leaf} as {local}"
            add_uses.append((cfg + "\n" if cfg else "") + f"use {'::'.join(p[:-1])}::{piece};")
        out = [old.rstrip("\n")]
        if add_uses:
            marker = "use super::*;"
            if marker in old:
                out = [old.replace(marker, marker + "\n" + "\n".join(add_uses), 1).rstrip("\n")]
            else:
                out.append("\n".join(add_uses))
        out.append("")
        for stack, chunks in groups.items():
            body = []
            for c in chunks:
                body += c + [""]
            while body and body[-1] == "":
                body.pop()
            out += wrap_regions(list(stack), body) + [""]
        if not dry:
            write(dst, "\n".join(out).rstrip("\n") + "\n")
        slug = os.path.basename(os.path.dirname(dst))
        TOUCHED.append(dst)
        if not exists:
            result.append(dict(name=module_name(suffix, slug), file=dst, cfg=[], glob=True, slug=slug, vis="", reexport=["pub"], reexport_cfg=[""], new=True))
    if not dry:
        for src, text in cut_texts.items():
            write(src, text)
        for m in plan:
            m["done"] = True
        json.dump(MOVES, open(MOVES_PATH, "w"), ensure_ascii=False, indent=1)
    return result, sorted(set(by_src))


if __name__ == "__main__":
    suffix = sys.argv[1]
    res, srcs = pull(suffix, dry="--apply" not in sys.argv)
    print("new modules:", [r["name"] for r in res])
    print("sources:", [s.replace(SH + "/", "")[:60] for s in srcs])
