#!/usr/bin/env python3
"""🧪️ Moves a crate's public-API unit-test modules out of its lib-test unit into one `[[test]]` integration target.

For every `cfg(test)` module file of the crate whose path matches a target's regexes, the mount declaration is removed
from its parent, its `super::`/`crate::` paths become absolute `<crate>::<module path>` paths (the private `component`
glob segment dropped), and the target's root file mounts it under the same leaf name; the crate root's `extern crate
.. as ..` aliases are repeated in the target root, and the manifest gains the `[[test]]` table (ASCII name).

usage: 🧪️integration-tests.py <spec.json> plan|apply
spec: {"manifest": repo-rel Cargo.toml, "targets": [{"name": ascii, "root": repo-rel root file, "doc": "//! ..",
       "tests": [regex on repo-rel test file path]}]}
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CENSUS = os.path.join(HERE, "🕸️module-graph.py")
census = {}
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
Crate = census["Crate"]
CUT = os.path.join(HERE, "✂️crate-cut.py")
cut = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0].replace("\naggregator_layer = None", ""), CUT, "exec"), cut)
strip = cut["strip"]
SEP = r"\s*::\s*"


def main():
    spec_path, mode = sys.argv[1], sys.argv[2]
    spec = json.load(open(spec_path, encoding="utf-8"))
    manifest = os.path.join(REPO, spec["manifest"])
    mtext = open(manifest, encoding="utf-8").read()
    name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', mtext).group(1)
    ident = name.replace("-", "_")
    lib = re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', mtext).group(1)
    root = os.path.normpath(os.path.join(os.path.dirname(manifest), lib))
    crate = Crate(root)
    root_raw = open(root, encoding="utf-8").read()
    aliases = [line for line in root_raw.splitlines() if line.startswith("extern crate ")]
    removals, writes, path_edits = {}, {}, {}
    mount = re.compile(r'(?m)^[ \t]*(?:#\[cfg\(\s*test\s*\)\]\s*(?:[ \t]*//[^\n]*\n\s*)*)?#\[path\s*=\s*"(?P<path>[^"]+)"\]\s*(?:#\[cfg\(\s*test\s*\)\]\s*)?(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*;[ \t]*\n')
    claimed = set()
    for target in spec["targets"]:
        rules = [re.compile(r) for r in target["tests"]]
        chosen = sorted(f for f in crate.test_files if isinstance(f, str) and f not in crate.included and any(r.search(os.path.relpath(f, REPO)) for r in rules) and f not in claimed)
        claimed |= set(chosen)
        target_root = os.path.join(REPO, target["root"])
        mounts = []
        for f in chosen:
            found = None
            for parent in crate.files:
                text = open(parent, encoding="utf-8").read()
                for m in mount.finditer(text):
                    base = os.path.dirname(parent)
                    if os.path.normpath(os.path.join(base, m.group("path"))) == f or any(os.path.normpath(os.path.join(base, d, m.group("path"))) == f for d in ["."]):
                        found = (parent, m)
                        break
                if found:
                    break
            if not found:
                print(f"UNMOUNTED {os.path.relpath(f, REPO)}")
                continue
            parent, m = found
            removals.setdefault(parent, []).append((m.start(), m.end()))
            modpath = [s for s in crate.files[f] if s != "component"]
            owner = modpath[:-1]
            raw = open(f, encoding="utf-8").read()
            stripped = strip(raw)
            edits = []
            for p in re.finditer(r"(?<![\w:])(?P<head>(?:super" + SEP + r")+|crate" + SEP + r"|self" + SEP + r")", stripped):
                head = p.group("head")
                if head.startswith("crate"):
                    rep = ident + "::"
                elif head.startswith("self"):
                    rep = "::".join([ident] + modpath) + "::"
                else:
                    ups = head.count("super")
                    rep = "::".join([ident] + modpath[:len(modpath) - ups]) + "::"
                edits.append((p.start(), p.end(), rep))
            for p in re.finditer(r"(?<![\w:])super\s*::\s*\*", stripped):
                pass
            path_edits[f] = edits
            leaf = m.group("name") if m.group("name") not in ("tests", "unit") else re.sub(r"[^A-Za-z0-9_]", "_", "_".join(modpath[-2:]))
            if any(n == leaf for _, n in mounts) or any(re.search(r"\bmod " + leaf + r";", writes.get(target_root, "")) for _ in [0]):
                distinct = next((seg for seg in modpath if sum(1 for g in chosen if seg in crate.files[g]) == 1), None) or str(len(mounts))
                leaf = re.sub(r"[^A-Za-z0-9_]", "_", distinct + "_" + leaf)
            mounts.append((os.path.relpath(f, os.path.dirname(target_root)), leaf))
        mounted_text = "\n".join(open(os.path.normpath(os.path.join(os.path.dirname(target_root), p)), encoding="utf-8").read() for p, _ in mounts)
        used_aliases = [a for a in aliases if re.search(r"(?<![\w:])" + re.match(r"extern crate \S+ as (\w+);", a).group(1) + r"\s*::", mounted_text)] if all(re.match(r"extern crate \S+ as (\w+);", a) for a in aliases) else aliases
        body = target["doc"] + "\n\n" + "\n".join(used_aliases) + ("\n\n" if used_aliases else "") + "".join(f'#[path = "{p}"]\nmod {n};\n' for p, n in mounts)
        writes[target_root] = body
        row = f'\n[[test]]\nname = "{target["name"]}"\npath = "{os.path.relpath(target_root, os.path.dirname(manifest))}"\n'
        if f'name = "{target["name"]}"' not in mtext:
            mtext = mtext.rstrip("\n") + "\n" + row
        print(f"target {target['name']}: {len(mounts)} modules -> {target['root']}")
    for f in set(path_edits) | set(removals):
        raw = open(f, encoding="utf-8").read()
        edits = sorted(path_edits.get(f, []) + [(a, b, "") for a, b in removals.get(f, [])])
        out, last = [], 0
        for a, b, rep in edits:
            if a < last:
                continue
            out.append(raw[last:a]); out.append(rep); last = b
        out.append(raw[last:])
        text = "".join(out)
        writes[f] = re.sub(r"\n{3,}", "\n\n", text).rstrip("\n") + "\n" if f in removals else text
        if f in removals:
            print(f"unmount {len(removals[f])} in {os.path.relpath(f, REPO)}")
    if mode == "apply":
        import shutil, time
        backup = os.path.join(REPO, ".🧬semio/🦑️repo/⚡️cache/p8/backup", "it-" + time.strftime("%H%M%S"))
        for path in list(writes) + [manifest]:
            if os.path.exists(path):
                target = os.path.join(backup, os.path.relpath(path, REPO))
                os.makedirs(os.path.dirname(target), exist_ok=True)
                shutil.copy2(path, target)
        for path, text in writes.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)
        open(manifest, "w", encoding="utf-8").write(mtext)


main()
