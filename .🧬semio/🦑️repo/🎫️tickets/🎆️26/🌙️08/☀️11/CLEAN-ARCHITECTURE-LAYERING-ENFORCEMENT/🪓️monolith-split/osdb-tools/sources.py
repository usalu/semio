"""📚 Source files of the pristine db tree as modules: parsed items, scope tables and parent links."""
import os
import re

from lib import *
from model import parse_items
from rs import strip, idents
from scope import MODULE_IDENT, ALL_MODULE_IDENTS, module_path, STANDALONE_MODULES


class Src:
    def __init__(self, fid, path, module, role, parent):
        self.fid = fid
        self.path = path
        self.module = module
        self.role = role
        self.parent = parent
        self.text = read(path)
        self.items, self.trail, self.lines = parse_items(self.text)
        self.stripped = strip(self.text)
        self.mounts = []
        self.defs = {}
        for i, it in enumerate(self.items):
            if it.kind in ('use', 'impl', 'extern crate', 'macro_call'):
                continue
            if it.kind == 'mod' and self._is_decl(it):
                continue
            if it.name and it.name != '_':
                self.defs.setdefault(it.name, []).append(i)

    def _is_decl(self, it):
        return strip('\n'.join(self.lines[it.header:it.end + 1])).rstrip().endswith(';')

    def mount_decls(self):
        """[(item index, module name, target path, is_test)] for `#[path]` module declarations."""
        out = []
        for i, it in enumerate(self.items):
            if it.kind != 'mod' or not self._is_decl(it):
                continue
            pa = None
            for a in it.attrs:
                m = re.match(r'#\[path = "([^"]+)"\]', a.strip())
                if m:
                    pa = m.group(1)
            if pa is None:
                continue
            tgt = os.path.normpath(os.path.join(os.path.dirname(self.path), pa))
            is_test = it.cfg_test or any('cfg(test)' in a for a in it.attrs)
            out.append((i, it.name, tgt, is_test))
        return out


def load_all(base=PRISTINE):
    """All module families: returns dict fid -> Src. fid = relative path from base."""
    srcs = {}

    def add(path, module, role, parent):
        fid = os.path.relpath(path, base)
        if fid in srcs:
            return fid
        s = Src(fid, path, module, role, parent)
        srcs[fid] = s
        for m in re.finditer(r'(?<![A-Za-z0-9_])include!\(\s*"([^"]+)"\s*\)', s.text):
            tgt = os.path.normpath(os.path.join(os.path.dirname(s.path), m.group(1)))
            if os.path.exists(tgt) and tgt.endswith('.rs'):
                add(tgt, module, 'test', fid)
        for i, name, tgt, is_test in s.mount_decls():
            if os.path.exists(tgt):
                keep = any('claimed-backend-laws' in a for a in s.items[i].attrs)
                cfid = add(tgt, module, 'keep' if keep else ('test' if is_test or s.role == 'test' else 'sub'), fid)
                s.mounts.append((name, cfid, is_test))
        return fid

    for ident in ALL_MODULE_IDENTS:
        add(module_path(ident, base), ident, 'main', None)
    return srcs


if __name__ == '__main__':
    srcs = load_all()
    for fid, s in srcs.items():
        print(s.role, s.module, fid[-60:], len(s.items), len(s.lines))
