"""🧵 Import synthesis for carved crates: external catalog, sibling pub names and trait-method needs."""
import re
from lib import *
from model import *

STRING = re.compile(r'b?r(#*)".*?"\1|b?"(?:\\.|[^"\\])*"', re.S)


SERDE_STR = re.compile(r'\b(?:default|deserialize_with|serialize_with|skip_serializing_if|with|bound)\s*=\s*"([^"]+)"')


def code_idents(text):
    t = strip_comments(text)
    extra = set()
    for m in SERDE_STR.finditer(t):
        extra |= idents(m.group(1))
    t = STRING.sub('""', t)
    return idents(t) | extra


def mounts(text):
    out = []
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', text):
        out.append((m.group(1), m.group(2)))
    return out


class Registry:
    def __init__(self, catalog):
        self.catalog = dict(catalog)
        self.pub = {}
        self.order = []
        self.up = {}
        self.dev = {}
        self.defined = {}
        self.imported = {}

    def add_crate(self, key, body_text, up, imported_names=(), dev=()):
        self.order.append(key)
        self.up[key] = list(up)
        self.dev[key] = list(dev)
        self.pub[key] = api_names(body_text)
        self.defined[key] = set(self.pub[key]) | set(private_names(body_text))
        self.imported[key] = set(imported_names)

    def sibling_of(self, name, key, test=False):
        for d in self.up[key] + (self.dev[key] if test else []):
            if name in self.pub[d]:
                return d
        return None

    def needs(self, key, text, exclude=(), provided=(), test=False):
        ids = code_idents(text)
        ext, sib = {}, {}
        for n in sorted(ids):
            if n in exclude or n in provided or n in self.defined.get(key, ()):
                continue
            d = self.sibling_of(n, key, test)
            if d:
                sib.setdefault(d, set()).add(n)
            elif n in self.catalog:
                ext[n] = self.catalog[n]
        for tr in used_trait_imports(strip_comments(STRING.sub('""', text))):
            if tr not in provided and tr not in self.defined.get(key, ()):
                d = self.sibling_of(tr, key, test)
                if d:
                    sib.setdefault(d, set()).add(tr)
                elif tr in self.catalog:
                    ext[tr] = self.catalog[tr]
        return ext, sib

    def import_lines(self, key, text, pub_names=(), exclude=(), provided=()):
        ext, sib = self.needs(key, text, exclude, provided)
        lines = []
        pubp, privp = [], []
        for n, full in ext.items():
            (pubp if n in pub_names else privp).append(full)
        for d, names in sib.items():
            for n in names:
                (pubp if n in pub_names else privp).append(ident(d) + "::" + n)
        out = []
        if privp:
            out.append(render_use(privp))
        if pubp:
            out.append(render_use(pubp, pub=True))
        return "\n".join(out), ext, sib


EMOJI_USED = set()


def doc(first, text, extra=()):
    ch = first
    if text.startswith(ch + " "):
        text = text[len(ch) + 1:]
    assert ch not in EMOJI_USED, "duplicate docstring emoji " + ch
    EMOJI_USED.add(ch)
    lines = ["//! " + ch + " " + text.split("\n")[0]] + ["//! " + l if l else "//!" for l in text.split("\n")[1:]]
    for e in extra:
        lines.append("//! " + e if e else "//!")
    return "\n".join(lines) + "\n"
