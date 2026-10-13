#!/usr/bin/env python3
"""🗂️ Regroups the component root's production module declarations into one `//#region <Facet>` per facet.

The facet of every module comes from `🧭️facet-imports.py` (the facet table). A declaration block is the run of `//`
comment lines, docs and attributes above one file-backed `mod`, the declaration itself and the `pub use <mod>::…;`
statements right after it; test-only (`cfg(test)`/`cfg(any(test, …))`) mounts and inline modules stay where they are.
Each facet region lists its blocks by module name, so the root reads as the facet table. It compiles nothing.
Usage: regions.py [--dry]
"""
import importlib.util, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("facet_imports", os.path.join(HERE, "🧭️facet-imports.py"))
fi = importlib.util.module_from_spec(spec); spec.loader.exec_module(fi)
ROOT = os.path.join(fi.PLUGIN, "🦀️.rs")
TITLES = {"ownership": "🏛️OwnershipFacet", "window": "🪟️WindowFacet", "declaration": "📜️DeclarationFacet", "publication": "📬️PublicationFacet", "jobs": "👷️JobsFacet", "reactor": "⚛️ReactorFacet", "instance": "🧵️InstanceFacet"}


def facets():
    fi.walk(f"{fi.PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{fi.PLUGIN}/📦️packages/🦀️rust")
    top = {}
    for m in fi.modules:
        s = m.path.split("::")
        if len(s) == 3 and s[2] not in top: top[s[2]] = m
    return {k: fi.facet_of(v) for k, v in top.items()}


def main():
    fac = facets()
    lines = open(ROOT, encoding="utf-8").read().split("\n")
    for t in TITLES.values():
        lines = [l for l in lines if l not in (f"//#region {t}", f"//#endregion {t}")]
    blocks, keep, i = {}, [], 0
    decl = re.compile(r"^(?:pub(?:\(crate\))?\s+)?mod\s+(\w+)\s*;\s*$")
    while i < len(lines):
        m = decl.match(lines[i])
        if not m or fac.get(m.group(1)) not in TITLES:
            keep.append(lines[i]); i += 1; continue
        name, start = m.group(1), len(keep)
        while start > 0 and re.match(r"^\s*(#\[|///|// )", keep[start - 1]): start -= 1
        head = keep[start:]
        if any(re.search(r"#\[cfg\((?:any\()?test\b", h) for h in head):
            keep.append(lines[i]); i += 1; continue
        del keep[start:]
        body = head + [lines[i]]; i += 1
        while i < len(lines) and re.match(rf"^pub use (?:self::)?{name}::", lines[i]):
            while not lines[i].rstrip().endswith(";"): body.append(lines[i]); i += 1
            body.append(lines[i]); i += 1
        blocks.setdefault(fac[name], []).append((name, body))
    text = "\n".join(keep)
    anchor = "pub mod app {"
    regions = []
    for f, t in TITLES.items():
        regions += [f"//#region {t}"] + [l for _, b in sorted(blocks.get(f, [])) for l in b] + [f"//#endregion {t}"]
    k = text.index(anchor)
    cut = text.rfind("//#region ⚖️AppLaws", 0, k)
    k = cut if cut >= 0 else k
    out = re.sub(r"\n{3,}", "\n\n", text[:k].rstrip("\n") + "\n\n" + "\n".join(regions) + "\n" + text[k:])
    print({f: len(b) for f, b in blocks.items()})
    if "--dry" not in sys.argv: open(ROOT, "w", encoding="utf-8").write(out)


if __name__ == "__main__":
    main()
