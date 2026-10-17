"""🧮 rw2t: for every WP2 law file, which names come from where (own module namespace, other crates, externals) and the lowest crate that can host it."""
import sys
from ns import *
from analyze import LAWFILES
HOST = {  # law -> (crate, mounting file module name hint)
 "wgpu-gumball-dispatch": "canvas2d-host", "wgpu-canvas2d": "scene-canvas2d", "wgpu-ui-image-loading": "scene-canvas2d",
 "wgpu-raster-frame-cost": "scene-ink", "wgpu-ink-canvas": "scene-ink", "wgpu-event-feed": "scene-feeds", "wgpu-graph-timeline": "scene-feeds",
 "wgpu-table": "scene-tables", "wgpu-block-list": "scene-tables", "wgpu-diff-view": "scene-tables", "wgpu-virtual-file-system": "scene-text-vfs",
 "wgpu-text-editor": "scene-text-vfs", "wgpu-unit": "icon-host", "png-export": "icon-host", "gpu-export": "icon-host", "svg-export": "icon-host",
 "wgpu-icon-render": "icon-host", "wgpu-surface-context-menu": "scene-render", "wgpu-render-entry": "scene-render",
}
def setup():
    for c in set(HOST.values()):
        register_crate(c)
def parent_of(suffix, file):
    return PARENT[(suffix, file)][0]
BIND = re.compile(r"(?:\blet\s+(?:mut\s+)?|\bfor\s+(?:mut\s+)?|\bSome\(\s*(?:mut\s+)?|\bOk\(\s*(?:mut\s+)?|\bErr\(\s*(?:mut\s+)?|\bref\s+(?:mut\s+)?|\(\s*(?:mut\s+)?)(\w+)\s*[:=,)\s]|\|([^|]*)\|")
def bound_names(text):
    clean = blank(text)
    out = set()
    for m in re.finditer(r"\blet\s+(?:mut\s+)?(\w+)", clean): out.add(m.group(1))
    for m in re.finditer(r"\blet\s+(?:mut\s+)?\(([^)]*)\)", clean):
        for w in re.findall(r"\w+", m.group(1)): out.add(w)
    for m in re.finditer(r"\bfor\s+(?:mut\s+)?(\w+)", clean): out.add(m.group(1))
    for m in re.finditer(r"\bfor\s+\(([^)]*)\)", clean):
        for w in re.findall(r"\w+", m.group(1)): out.add(w)
    for m in re.finditer(r"\|([^|\n]*)\|", clean):
        for w in re.findall(r"(?<![\w:])(\w+)\s*(?::|,|$)", m.group(1)): out.add(w)
    for m in re.finditer(r"\bfn\s+\w+\s*(?:<[^>]*>)?\(([^)]*)\)", clean):
        for w in re.findall(r"(\w+)\s*:", m.group(1)): out.add(w)
    for m in re.finditer(r"\b(?:Some|Ok|Err|Cell|Box)\(\s*(?:mut\s+|ref\s+)?(\w+)\s*\)", clean): out.add(m.group(1))
    return out
def called_names(text):
    clean = blank(text)
    return set(m.group(1) for m in re.finditer(r"(?<![\w.:])(\w+)\s*(?:\(|!|::)", clean))
def classify(name, ix, alias):
    path = LAWFILES[name]
    suffix = HOST[name]
    P = parent_of(suffix, path)
    nsP = ns(suffix, P)
    text = read(path)
    toks = free_tokens(text)
    loc = local_names(text)
    bound = bound_names(text)
    called = called_names(text)
    res = dict(own={}, cross={}, ext={}, unk=[], P=P)
    for w in toks:
        if w in loc or w in PRIMS:
            continue
        if w[0].islower() and w in bound and w not in called:
            continue
        if w in nsP:
            res["own"][w] = nsP[w]
        elif w in ix:
            res["cross"][w] = ix[w]
        elif w in alias:
            res["ext"][w] = sorted(alias[w])
        else:
            res["unk"].append(w)
    return res
if __name__ == "__main__":
    setup()
    ix = build_index()
    alias = alias_table(list(CRATES))
    for name in (sys.argv[1:] or LAWFILES):
        r = classify(name, ix, alias)
        suffix = HOST[name]
        owners = collections.defaultdict(list)
        for w, lst in r["cross"].items():
            for c, mp, cf in lst:
                owners[c].append(w)
        print("##", name, "host", suffix, "| own", len(r["own"]), "cross", {c: len(v) for c, v in owners.items()}, "ext", len(r["ext"]), "unk", len(r["unk"]))
        for c, v in owners.items():
            print("     ", c, sorted(v)[:30])
