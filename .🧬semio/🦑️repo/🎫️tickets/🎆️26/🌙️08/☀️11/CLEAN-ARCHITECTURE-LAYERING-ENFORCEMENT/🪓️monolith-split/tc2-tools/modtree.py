"""🌳 tc2: module tree of a crate (mounts through `#[path]`, inline `mod x { .. }` blocks and `#[path = "."]` nests), read-only.
Module(name, chain, file, parent_file, test_gated, span, attrs, vis, inline): `chain` is the list of module names from the crate root; `file` is the file that holds the module body (for inline modules: the file that contains the block); `span` is the text span of the declaration (attributes included) in `parent_file`."""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/ttc")
from ttcstub import mask

MOD_RX = re.compile(r"(?<![A-Za-z0-9_])mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*(;|\{)")
PATH_ATTR_RX = re.compile(r"#\[\s*path\s*=\s*\"([^\"]*)\"\s*\]")


class Module:
    def __init__(self, **kw):
        self.__dict__.update(kw)

    def __repr__(self):
        return f"Module({'::'.join(self.chain)} in {os.path.basename(os.path.dirname(self.file))} test={self.test_gated})"


def attr_groups_before(masked, raw, pos):
    """Contiguous outer attributes (and a visibility) directly before `pos`; returns (start, [attr texts], vis)."""
    i = pos
    vis = ""
    j = i
    while True:
        k = j
        while k > 0 and masked[k - 1] in " \t\r\n":
            k -= 1
        m = re.search(r"pub(?:\s*\([^)]*\))?\s*$", masked[:k])
        if m and k == j and False:
            pass
        if m:
            vis = m.group(0).strip()
            j = m.start()
            continue
        if k > 0 and masked[k - 1] == "]":
            depth = 0
            p = k - 1
            while p >= 0:
                if masked[p] == "]":
                    depth += 1
                elif masked[p] == "[":
                    depth -= 1
                    if depth == 0:
                        break
                p -= 1
            if p > 0 and masked[p - 1] == "#":
                j = p - 1
                continue
        break
    return j, vis


def parse_file(path, dir_base, chain, parent_file, out, text=None, offset_chain=None):
    raw = open(path, encoding="utf-8").read() if text is None else text
    masked = mask(raw)
    return _scan(path, raw, masked, 0, len(raw), dir_base, chain, out)


def _scan(path, raw, masked, lo, hi, dir_base, chain, out):
    pos = lo
    while True:
        m = MOD_RX.search(masked, pos, hi)
        if not m:
            return
        name, term = m.group(1), m.group(2)
        decl_start, vis = attr_groups_before(masked, raw, m.start())
        attrs = raw[decl_start:m.start()]
        attrs_masked_text = raw[decl_start:m.start()]
        path_attr = PATH_ATTR_RX.search(attrs)
        test_gated = bool(re.search(r"#\[\s*cfg\s*\(\s*(?:any\s*\(\s*)?test\b", attrs))
        if term == ";":
            end = m.end()
            if path_attr:
                child = os.path.normpath(os.path.join(dir_base, path_attr.group(1)))
            else:
                a = os.path.join(dir_base, name + ".rs")
                b = os.path.join(dir_base, name, "mod.rs")
                child = a if os.path.exists(a) else b
            mod = Module(name=name, chain=chain + [name], file=child, parent_file=path, test_gated=test_gated, span=(decl_start, end), attrs=attrs, vis=vis, inline=False, exists=os.path.exists(child))
            out.append(mod)
            if os.path.exists(child):
                parse_file(child, os.path.dirname(child), chain + [name], path, out)
            pos = end
        else:
            depth = 0
            p = m.end() - 1
            while p < hi:
                if masked[p] == "{":
                    depth += 1
                elif masked[p] == "}":
                    depth -= 1
                    if depth == 0:
                        break
                p += 1
            end = p + 1
            if path_attr:
                inner_dir = os.path.normpath(os.path.join(dir_base, path_attr.group(1)))
            else:
                inner_dir = os.path.join(dir_base, name)
            mod = Module(name=name, chain=chain + [name], file=path, parent_file=path, test_gated=test_gated, span=(decl_start, end), attrs=attrs, vis=vis, inline=True, exists=True)
            out.append(mod)
            _scan(path, raw, masked, m.end(), p, inner_dir, chain + [name], out)
            pos = end


def build(lib_root):
    out = []
    parse_file(lib_root, os.path.dirname(lib_root), [], None, out)
    return out


if __name__ == "__main__":
    mods = build(sys.argv[1])
    for m in mods:
        print("::".join(m.chain), "TEST" if m.test_gated else "", "inline" if m.inline else m.file.split("/")[-2] if m.file else "", m.vis)
