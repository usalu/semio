"""🧷 Union-sandbox differential for the ui-scene carve: simulates the crates as modules of one crate and compares rustc error multisets against the original tree."""
import collections, json, os, re, shutil, subprocess, sys
import oracle
from lib import *
from model import *
from synth import mounts


def owned_files(tree, rel):
    out = [rel]
    text = rd(tree + "/" + rel)
    d = os.path.dirname(rel)
    for p, name in mounts(text):
        child = os.path.normpath(os.path.join(d, p))
        out += owned_files(tree, child)
    return out


def build_union(src_tree, dst, roots, *, strip_alloc_after_first=True, rewrite=True):
    shutil.rmtree(dst, ignore_errors=True)
    os.makedirs(dst)
    shutil.copytree(src_tree, dst + "/🎬️scene")
    lib = []
    first_alloc = [True]
    for modname, rel in roots:
        lib.append('#[path = "🎬️scene/%s"]\npub mod %s;' % (rel, modname))
        if not rewrite:
            continue
        for f in sorted(set(owned_files(dst + "/🎬️scene", rel))):
            path = dst + "/🎬️scene/" + f
            t = rd(path)
            t = re.sub(r"(?<![\w:])crate::", "crate::%s::" % modname, t)
            t = re.sub(r"pub\(crate\)", "pub(in crate::%s)" % modname, t)
            t = re.sub(r"(?<![\w:])(semio_framework_ui_scene_\w+)(::| as )", r"crate::\1\2", t)
            if "scene_pack_wire" in t and modname != "semio_framework_ui_scene_doc":
                t = re.sub(r"scene_pack_wire,\s*", "", t, count=1)
                ls = t.split("\n")
                k = 0
                while k < len(ls) and ls[k].startswith("//!"):
                    k += 1
                t = "\n".join(ls[:k] + ["use crate::scene_pack_wire;"] + ls[k:])
            if "#[global_allocator]" in t:
                if first_alloc[0] or not strip_alloc_after_first:
                    first_alloc[0] = False
                else:
                    t = t.replace("#[global_allocator]", "")
                    t = re.sub(r"\nstatic OBSERVED_ALLOCATOR\s*:\s*ObservedAllocator\s*=\s*ObservedAllocator;", "", t)
            open(path, "w", encoding="utf-8").write(t)
    open(dst + "/lib.rs", "w", encoding="utf-8").write("\n".join(lib) + "\n")
    oracle.patch_tree(dst)
    return dst + "/lib.rs"


def norm(msg):
    msg = re.sub(r"crate::semio_framework_ui_scene_\w+::", "", msg)
    msg = re.sub(r"`semio_framework_ui_scene_\w+`", "`CRATE`", msg)
    return msg


def counter(out):
    c = collections.Counter()
    for (code, msg, loc), n in out.items():
        c[(code, norm(msg))] += n
    return c


def compare(base_out, new_out, show=40):
    b, n = counter(base_out), counter(new_out)
    only_new = n - b
    only_base = b - n
    print("baseline errors", sum(b.values()), "new errors", sum(n.values()), "only-new", sum(only_new.values()), "only-baseline", sum(only_base.values()))
    for (code, msg), c in list(only_new.items())[:show]:
        print("  NEW ", c, code, msg[:200])
    for (code, msg), c in list(only_base.items())[:show]:
        print("  GONE", c, code, msg[:200])
    return sum(only_new.values()), sum(only_base.values())


def rustc_run(root, lib, name, flags):
    out, err = oracle.run(root, lib, name, flags)
    return out, err
