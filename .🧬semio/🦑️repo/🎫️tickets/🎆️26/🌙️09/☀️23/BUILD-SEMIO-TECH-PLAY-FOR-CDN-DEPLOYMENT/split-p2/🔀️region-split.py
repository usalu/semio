#!/usr/bin/env python3
"""🔀️ Splits one Rust source file into sibling sub-topic files by its own `// #region` sections (lane P2).

Every top-level item inherits the region it sits in; regions map to layers through the spec. Items a layer references in
another layer are made `pub` and imported by the referencing layer's file through that layer's module path. Top-level
`use` items (outside every region) are copied into each layer whose text names one of their imported identifiers.

usage: 🔀️region-split.py plan|emit <file.rs> <spec.json>
spec: {"layers": {layer: {"regions": [region...], "file": path, "module": "crate::…", "header": "//! …\n"}}, "keep": layer}
       `keep` receives the original file (its header comment and every region not named elsewhere).
"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {"__name__": "item_split", "__file__": os.path.join(HERE, "🔀️item-split.py")}
exec(compile(open(os.path.join(HERE, "🔀️item-split.py"), encoding="utf-8").read().rsplit("\nif __name__ ==", 1)[0], "item-split", "exec"), ns)
parse = ns["parse"]

def regions_of(src):
    marks = [(m.start(), m.group(1), m.group(2).strip()) for m in re.finditer(r"(?m)^// #(end)?region\b[ \t]*([^\n]*)$", src)]
    return marks

def code_start(it):
    """🎯️ Offset where the item's own code begins (after its leading comments, region markers included)."""
    return it.start + len(it.stripped) - len(it.stripped.lstrip())

MARK = re.compile(r"(?m)^[ \t]*// #(?:end)?region\b[^\n]*\n?")

def assign(src, items, spec):
    marks = regions_of(src)
    layer_of_region = {r: l for l, cfg in spec["layers"].items() for r in cfg.get("regions", [])}
    def region_at(pos):
        stack = []
        for at, end, name in marks:
            if at > pos:
                break
            stack = stack[:-1] if end else stack + [name]
        return stack[-1] if stack else None
    out = {}
    for it in items:
        region = region_at(code_start(it))
        out[it.i] = layer_of_region.get(region, spec["keep"]) if region else (None if it.kind == "use" or not it.stripped.strip() else spec.get("outside", spec["keep"]))
    return out

def publicized(text):
    """🔓️ Makes the item's own declaration `pub`, skipping its doc comments and attributes."""
    lines = text.split("\n")
    depth = 0
    for index, line in enumerate(lines):
        stripped = line.strip()
        if depth == 0 and (not stripped or stripped.startswith("//")):
            continue
        if depth > 0 or stripped.startswith("#["):
            depth += stripped.count("[") - stripped.count("]")
            continue
        if re.match(r"pub\s*\((?:crate|super)\)\s+", stripped):
            lines[index] = re.sub(r"pub\s*\((?:crate|super)\)\s+", "pub ", line, count=1)
        elif not re.match(r"pub\b", stripped) and re.match(r"(?:const\s+fn|fn|struct|enum|trait|type|const|static|union)\b", stripped):
            lines[index] = line[: len(line) - len(line.lstrip())] + "pub " + stripped
        break
    return "\n".join(lines)

def main():
    mode, path, spec_path = sys.argv[1], sys.argv[2], sys.argv[3]
    spec = json.load(open(spec_path, encoding="utf-8"))
    src, items = parse(path)
    layer = assign(src, items, spec)
    names = {}
    for it in items:
        if it.kind not in ("impl", "use", "macro", "?"):
            names.setdefault(it.name, []).append(it)
    for it in items:
        if it.kind == "impl" and it.target in names and layer[it.i] is None:
            layer[it.i] = layer[names[it.target][0].i]
    needs = collections.defaultdict(set); publicize = set()
    for it in items:
        if layer[it.i] is None:
            continue
        for ident in it.idents:
            for target in names.get(ident, []):
                if target.i != it.i and layer[target.i] not in (None, layer[it.i]) and (target.kind != "mod" or ident in it.paths):
                    needs[layer[it.i]].add((layer[target.i], target.name)); publicize.add(target.i)
    totals = collections.Counter(layer[it.i] for it in items for _ in range(it.text.count("\n") + 1))
    print("# lines per layer:", dict(totals))
    if mode == "plan":
        for (a, b, n) in sorted({(a, b, n) for a, s in needs.items() for (b, n) in s}):
            print(f"  {a} -> {b}: {n}")
        return
    uses = [it for it in items if layer[it.i] is None and it.kind == "use"]
    dropped = [it.name for it in items if layer[it.i] is None and it.kind != "use" and it.stripped.strip()]
    assert not dropped, f"items outside every region: {dropped}"
    first = items[0]
    file_head = first.text[: code_start(first) - first.start]
    file_head = MARK.sub("", file_head)
    marks = regions_of(src)
    def region_of(it):
        stack = []
        for at, end, name in marks:
            if at > code_start(it):
                break
            stack = stack[:-1] if end else stack + [name]
        return stack[-1] if stack else None
    for name, cfg in spec["layers"].items():
        chosen = [it for it in items if layer[it.i] == name]
        blocks, current = [], None
        for it in chosen:
            text = MARK.sub("", it.text if it is not first else it.text[code_start(it) - it.start:]).strip("\n")
            if it.i in publicize:
                text = publicized(text)
            region = region_of(it)
            if region != current:
                if current:
                    blocks.append(f"// #endregion {current}")
                if region:
                    blocks.append(f"// #region {region}")
                current = region
            blocks.append(text)
        if current:
            blocks.append(f"// #endregion {current}")
        body = "\n\n".join(blocks)
        head = file_head if name == spec["keep"] else cfg.get("header", "")
        used = []
        for it in uses:
            code = it.text[code_start(it) - it.start:].strip("\n")
            imported = set(re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\b", re.sub(r"^\s*use\s+", "", it.stripped))) - {"use", "crate", "super", "self", "as", "std", "collections"}
            if any(re.search(r"\b" + re.escape(ident) + r"\b", body) for ident in imported):
                used.append(code)
        imports = collections.defaultdict(set)
        for other, item_name in needs[name]:
            imports[spec["layers"][other]["module"]].add(item_name)
        lines = used + [f"use {module}::{{{', '.join(sorted(items_))}}};" for module, items_ in sorted(imports.items())]
        text = head.strip("\n") + "\n\n" + "\n".join(lines) + "\n\n" + cfg.get("mounts", "") + body + "\n"
        os.makedirs(os.path.dirname(cfg["file"]), exist_ok=True)
        open(cfg["file"], "w", encoding="utf-8").write(text)
        print(f"wrote {cfg['file']}: {len(chosen)} items")

main()
