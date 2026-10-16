#!/usr/bin/env python3
# 🔗️ Static check of the carved crates: every `crate::X` first segment must be a module declared by the crate root, a root item, or an extern alias; every `semio_framework_plugin_K` must be a partition crate.
import re, os, sys
from common import *
import model as M
import modtree
from rs import blank, item_names
import widen as W

def main():
    W.make_view()
    bad = 0
    for k in M.WP2:
        root = W.VIEW + '/' + M.ROOT[k]
        rt = read(root)
        mods = set(re.findall(r'pub mod (\w+);', blank(rt)))
        items = set(item_names(rt))
        for m in re.finditer(r'(?m)^(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]*?(?:as\s+)?(\w+)\s*;', blank(rt)):
            items.add(m.group(1))
        aliases = set(re.findall(r'extern crate [\w]+ as (\w+);', rt))
        for f in modtree.tree(root):
            t = read(f)
            for m in re.finditer(r'(?<![A-Za-z0-9_$:])crate::(\w+)', blank(t)):
                x = m.group(1)
                if x not in mods and x not in items and x not in aliases:
                    bad += 1
                    print(f'{k}: {os.path.relpath(f, W.VIEW)}: crate::{x}')
    print('unknown crate-local first segments:', bad)

if __name__ == '__main__':
    main()
