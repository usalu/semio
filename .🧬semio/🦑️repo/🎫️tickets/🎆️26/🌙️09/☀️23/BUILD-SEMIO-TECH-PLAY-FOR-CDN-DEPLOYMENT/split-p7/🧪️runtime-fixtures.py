#!/usr/bin/env python3
"""🧪️ Turns every test-time `include_str!`/`include_bytes!` of a data file into a runtime read relative to the owning
package's `CARGO_MANIFEST_DIR` (the raster precedent), so no test compilation unit embeds its fixtures.

Test code is every test-gated module file of a family crate, every `[[test]]` file, and every `#[cfg(test)]` span of a
library file. A `const NAME: &str = include_str!(…);` becomes a `static NAME: LazyLock<&str>` read once (its uses
become `*NAME`); an inline `include_str!(…)` becomes a leaked `String` read, so both stay `&'static str`
(`&'static [u8]` for bytes). Rust sources pulled in with `include!` stay.

usage: 🧪️runtime-fixtures.py <config.json> plan|apply
"""
import importlib.util, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec = importlib.util.spec_from_file_location("family_cut", os.path.join(HERE, "✂️family-cut.py"))
fc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fc)
EMBED = re.compile(r'include_(?P<kind>str|bytes)!\s*\(\s*"(?P<path>[^"]+)"\s*\)')


def reader(kind, rel):
    call = "read_to_string" if kind == "str" else "read"
    return f'std::fs::{call}(concat!(env!("CARGO_MANIFEST_DIR"), "/{rel}")).expect("{rel}").leak()'


def convert(path, package_dir, whole):
    raw = open(path, encoding="utf-8").read()
    text = fc.strip(raw)
    spans = [(0, len(raw))] if whole else fc.test_spans(text)
    edits, consts = [], []
    for m in EMBED.finditer(raw):
        if text[m.start():m.start() + 8] != "include_" or not any(a <= m.start() < b for a, b in spans):
            continue
        target = os.path.normpath(os.path.join(os.path.dirname(path), m.group("path")))
        if target.endswith(".rs"):
            continue
        rel = os.path.relpath(target, package_dir)
        line_start = raw.rfind("\n", 0, m.start()) + 1
        line_end = raw.find("\n", m.end())
        line = raw[line_start:line_end]
        const = re.match(r"(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?const\s+(?P<name>[A-Z_][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?(?P<ty>str|\[u8\])\s*=\s*$", raw[line_start:m.start()])
        if const and raw[m.end():line_end].strip() == ";":
            ty = "&'static str" if const.group("ty") == "str" else "&'static [u8]"
            edits.append((line_start, line_end, f"{const.group('indent')}{const.group('vis') or ''}static {const.group('name')}: std::sync::LazyLock<{ty}> = std::sync::LazyLock::new(|| {reader(m.group('kind'), rel)});"))
            consts.append((const.group("name"), line_end))
        else:
            edits.append((m.start(), m.end(), reader(m.group("kind"), rel)))
    for name, after in consts:
        for u in re.finditer(r"(?<![\w.:*])" + name + r"\b(?!\s*:)", text):
            if u.start() > after:
                edits.append((u.start(), u.end(), f"*{name}"))
    if not edits:
        return None
    out, last = [], 0
    for a, b, rep in sorted(edits):
        out.append(raw[last:a])
        out.append(rep)
        last = b
    out.append(raw[last:])
    return "".join(out)


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    stem = os.path.basename(config_path).rsplit(".", 1)[0]
    plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), encoding="utf-8"))["family"]
    artifact = os.path.join(REPO, config["artifact"])
    monolith = config["crate"].replace("_", "-")
    changed = 0
    for name, info in plan.items():
        package_dir = os.path.join(artifact, "📦️packages/🦀️rust") if name == monolith else os.path.join(artifact, info["owner"], "📦️packages/🦀️rust")
        jobs = [(f, True) for f in info["test"]] + [(f, False) for f in info["lib"]]
        jobs += [(f, True) for t in info["targets"] for f in t["files"]]
        for rel, whole in jobs:
            path = os.path.join(artifact, rel)
            new = convert(path, package_dir, whole)
            if new is None:
                continue
            changed += 1
            print(f"{name}: {rel}")
            if mode == "apply":
                open(path, "w", encoding="utf-8").write(new)
    print(f"files={changed}")


main()
