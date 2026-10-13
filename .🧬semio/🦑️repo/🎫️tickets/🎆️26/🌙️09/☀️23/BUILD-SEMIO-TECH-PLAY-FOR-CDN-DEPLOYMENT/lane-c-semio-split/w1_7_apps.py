#!/usr/bin/env python3
"""✏️ W1 step 7 (split plan 1h): `component-app-assembly` gating is gone; each subset root compiles its own editor and
viewer, the root keeps the envelope's `SemioAny` editor and viewer as its `editor` / `viewer` modules."""
import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import rewrite_uses, rewrite_linear

root = read(ROOT_RS)
lines = root.split("\n")


def top_block(name):
    for i, l in enumerate(lines):
        if l == "pub mod %s {" % name and lines[i - 1] == '#[path = "."]' and lines[i - 2] == '#[cfg(feature = "component-app-assembly")]':
            return i - 2, block_end(lines, i)
    raise SystemExit(name)


def child_blocks(lo, hi):
    out = {}
    i = lo
    while i < hi:
        m = re.match(r"^    pub mod semio_(\w+) \{$", lines[i])
        if m:
            e = block_end(lines, i)
            out[m.group(1)] = (i, e)
            i = e + 1
        else:
            i += 1
    return out


new_root_parts = {}
for kind in ("editor", "viewer"):
    lo, hi = top_block(kind)
    kids = child_blocks(lo, hi)
    assert sorted(kids) == sorted(NAMES + ["base"]), sorted(kids)
    for name, (a, e) in kids.items():
        emoji = EMOJI[name]
        body = [l[8:] if l.startswith(" " * 8) else l.lstrip() for l in lines[a + 1:e]]
        if name == "base":
            body = [l.replace('#[path = "🏅️standards/', '#[path = "🏅️standards/') for l in body]
            new_root_parts[kind] = '#[path = "."]\npub mod %s {\n' % kind + "\n".join("    " + l if l else l for l in body) + "\n}"
            continue
        body = [l.replace('#[path = "🏅️standards/🔖️v1/🪆️subsets/%s/' % emoji, '#[path = "') for l in body]
        assert not any("🏅️standards" in l for l in body), (kind, name)
        rp = SUBSETS + "/" + emoji + "/🦀️.rs"
        r = read(rp)
        anchor = "\nsemio_framework_value::artifact_retire_"
        i = r.index(anchor)
        r = r[:i].rstrip("\n") + '\n#[path = "."]\npub mod %s {\n' % kind + "\n".join("    " + l if l else l for l in body) + "\n}\n" + r[i:]
        write(rp, r)
for kind in ("viewer", "editor"):
    lo, hi = top_block(kind)
    lines[lo:hi + 1] = new_root_parts[kind].split("\n")
write(ROOT_RS, "\n".join(lines))
f = SUBSETS + "/✉️base/🦀️.rs"
write(f, read(f).replace('#[cfg(feature = "component-app-assembly")]\n#[path = "."]\npub mod editor {', '#[path = "."]\npub mod editor {'))


def mapper(seg):
    head = seg[:1]
    if head not in (["crate"], ["semio_s_artifact_stdio_semio"]) or len(seg) < 3 or seg[1] not in ("editor", "viewer"):
        return None
    m = re.match(r"semio_(\w+)$", seg[2])
    if not m:
        return None
    if m.group(1) == "base":
        return head + [seg[1]] + seg[3:]
    return head + ["standards", "v1", "subsets", m.group(1), seg[1]] + seg[3:]


linear = [(r"\b(crate|semio_s_artifact_stdio_semio)::(editor|viewer)::semio_base::", r"\1::\2::"),
          (r"\b(crate|semio_s_artifact_stdio_semio)::(editor|viewer)::semio_(%s)::" % "|".join(NAMES), r"\1::standards::v1::subsets::\3::\2::")]
changed = 0
for p in list(semio_rust_files()) + consumer_rust_files():
    s0 = read(p)
    s1, a = rewrite_linear(s0, linear)
    s2, b = rewrite_uses(s1, mapper)
    if s2 != s0:
        write(p, s2)
        changed += 1
comp = SEMIO + "/🧩️composition/🔣️.json"
c = read(comp)
c = re.sub(r"semio_s_artifact_stdio_semio::(editor|viewer)::semio_base::", r"semio_s_artifact_stdio_semio::\1::", c)
c = re.sub(r"semio_s_artifact_stdio_semio::(editor|viewer)::semio_(%s)::" % "|".join(NAMES), r"semio_s_artifact_stdio_semio::standards::v1::subsets::\2::\1::", c)
write(comp, c)
print("rewrote", changed, "files")
