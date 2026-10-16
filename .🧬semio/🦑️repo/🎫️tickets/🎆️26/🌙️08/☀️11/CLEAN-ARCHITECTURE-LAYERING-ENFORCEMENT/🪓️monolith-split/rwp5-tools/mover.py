# 🚚️ Item mover for the WP5 staging files: moves whole slugs or single items/methods between crate staging dirs inside a working copy, keeping region markers balanced and visibility workable.
import os, re, sys, shutil, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, rsedit
from rsedit import Doc

DOC_OPEN = "//! {emoji} `{crate}` staging of `{slug}`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate."

def path_of(root, d, slug): return f"{cfg.stg(root, d)}/{slug}/🦀️.rs"

def read(p): return open(p, encoding="utf-8").read()
def write(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf-8").write(t)

def new_file(emoji, crate, slug):
    return "\n".join([DOC_OPEN.format(emoji=emoji, crate=crate, slug=slug.split("️", 1)[-1] if "️" in slug else slug), "", "use super::*;", ""])

EMOJI = {"shell-session": "🚶", "shell-window-actions": "🪝", "shell-actions": "🔀", "shell-pointer": "🖐", "shell-view-paint": "🖍", "shell-view-chrome": "🍱", "shell-view-overlays": "🗨", "shell-view-panels": "🪀"}

def move_slug(root, src_dir, slug, dst_dir):
    a = path_of(root, src_dir, slug); b = path_of(root, dst_dir, slug)
    assert os.path.exists(a), a
    assert not os.path.exists(b), "merge needed " + b
    os.makedirs(os.path.dirname(b), exist_ok=True)
    shutil.move(a, b)
    os.rmdir(os.path.dirname(a))
    t = read(b)
    first = t.split("\n", 1)
    t = re.sub(r"`shell-[a-z-]+` staging of", "`%s` staging of" % cfg.DIR2CRATE[dst_dir], t, count=1).replace(src_dir[:1], dst_dir[:1], 1) if False else t
    write(b, t)

def find_member(doc, name, occurrence=0):
    hits = []
    for it in doc.items():
        if it["kind"] == "impl" and "ShellState" in it["head"] and " for " not in it["head"]:
            for m in doc.members(it):
                if m["name"] == name: hits.append(m)
    return hits

def find_item(doc, name):
    return [it for it in doc.items() if it["kind"] not in ("impl", "use", "mod") and re.match(r"[A-Za-z_][A-Za-z_0-9]*", it["name"] or "") and re.match(r"[A-Za-z_][A-Za-z_0-9]*", it["name"]).group(0) == name]

def widen_private(lines):
    out = []
    for l in lines:
        m = re.match(r"^(\s*)(fn|struct|enum|const|static|type|trait|async fn|unsafe fn|const fn)\b", l)
        out.append(re.sub(r"^(\s*)", r"\1pub(super) ", l, count=1) if m and not l.startswith("    ") else l)
    return out

class Plan:
    def __init__(self, root):
        self.root = root
        self.pending = collections.defaultdict(lambda: collections.defaultdict(list))
        self.log = []

    def take_item(self, src_dir, slug, name, dst_dir, which=None):
        p = path_of(self.root, src_dir, slug); doc = Doc(read(p))
        hits = find_item(doc, name)
        assert hits, (src_dir, slug, name)
        if which is None: assert len(hits) == 1, (name, len(hits))
        hs = hits if which is None else [hits[which]]
        for h in hs:
            assert doc.balanced(h["s"], h["e"]), ("region crossing", name)
            stack = doc.stack[h["s"]]
            txt = doc.cut(h["s"], h["e"])
            self.pending[(dst_dir, slug)][("free", stack)].append(txt)
            self.log.append(f"item {name} {src_dir}/{slug} -> {dst_dir} ({len(txt)} lines)")
            doc.reload()
            for _ in range(8):
                imp = next((i for i in doc.items() if i["kind"] == "impl" and not re.match(r"impl(?:<[^>]*>)?\s+ShellState\b", i["head"]) and re.match(r"impl\s+(?:[\w:<>, ]+\s+for\s+)?&?(?:\w+::)*" + re.escape(name) + r"\b", re.sub(r"^impl<[^>]*>", "impl", i["head"]))), None)
                if imp is None: break
                assert doc.balanced(imp["s"], imp["e"]), ("region crossing impl", name)
                itxt = doc.cut(imp["s"], imp["e"])
                self.pending[(dst_dir, slug)][("free", doc.stack[imp["s"]] if imp["s"] < len(doc.stack) else stack)].append(itxt)
                self.log.append(f"impl of {name} {src_dir}/{slug} -> {dst_dir}")
            hits = find_item(doc, name)
        write(p, doc.text())

    def locate(self, src_dir, name, kind):
        base = cfg.stg(self.root, src_dir); found = []
        for slug in sorted(os.listdir(base)):
            p = path_of(self.root, src_dir, slug)
            if not os.path.exists(p): continue
            doc = Doc(read(p))
            if (find_member(doc, name) if kind == "method" else find_item(doc, name)): found.append(slug)
        return found

    def take_method(self, src_dir, slug, name, dst_dir):
        if slug is None:
            fs = self.locate(src_dir, name, "method"); assert len(fs) >= 1, (src_dir, name)
            for s in fs: self.take_method(src_dir, s, name, dst_dir)
            return
        p = path_of(self.root, src_dir, slug); doc = Doc(read(p))
        hits = find_member(doc, name)
        assert hits, (src_dir, slug, name)
        while hits:
            h = hits[0]
            assert doc.balanced(h["s"], h["e"]), ("region crossing", name)
            stack = doc.stack[h["s"]]
            attrs = rsedit.impl_attrs(doc, h["impl"])
            txt = doc.cut(h["s"], h["e"])
            if attrs: txt = [a for a in ["    " + x for x in attrs if x not in "\n".join(txt[:6])]] + txt
            self.pending[(dst_dir, slug)][("method", stack)].append(txt)
            self.log.append(f"method {name} {src_dir}/{slug} -> {dst_dir} ({len(txt)} lines)")
            doc = Doc(doc.text()); hits = find_member(doc, name)
        write(p, doc.text())

    def flush(self):
        for (dst_dir, slug), groups in self.pending.items():
            p = path_of(self.root, dst_dir, slug)
            t = read(p) if os.path.exists(p) else new_file(EMOJI[cfg.DIR2CRATE[dst_dir]], cfg.DIR2CRATE[dst_dir], slug)
            out = [t.rstrip("\n"), ""]
            for (kind, stack), chunks in groups.items():
                body = []
                if kind == "method":
                    body.append("impl ShellState {")
                    for c in chunks: body += c + [""]
                    while body[-1] == "": body.pop()
                    body.append("}")
                else:
                    for c in chunks:
                        body += widen_private(c) + [""]
                    while body and body[-1] == "": body.pop()
                out += rsedit.wrap_regions(list(stack), body) + [""]
            write(p, "\n".join(out).rstrip("\n") + "\n")
        self.pending.clear()
