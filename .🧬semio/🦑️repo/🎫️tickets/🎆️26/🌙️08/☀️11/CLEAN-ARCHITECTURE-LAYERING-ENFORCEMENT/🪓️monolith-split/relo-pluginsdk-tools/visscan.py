"""🔎 Definition-level privacy scan for code moved out of the instance module subtree.

Rustc's typeck never runs under the patched oracle (externals are unresolved), so field (E0616), method (E0624) and
private-item uses across the new module boundary are invisible to it. This scan lists, for every file moved out of
`artifact_instance`, each `VcsArtifactApp` field, inherent method/associated fn and instance-parent free item it uses
whose visibility does not reach the moved file, and every method the moved files define that the instance subtree
calls with a visibility that no longer reaches it.

usage: visscan.py <plugin_dir> [--strip-vis name,name]   (the strip flag seeds a regression: it privatises names)
"""
import os, re, sys

INSTANCE_FILES = ["🧵️instance/🦀️.rs"] + [f"🧵️instance/{d}/🦀️.rs" for d in ("🚪️ingress", "📨️dispatch", "🖼️render", "🧷️command-routes", "🧾️typed-operations", "🔌️plugin-app-impl", "🛠️tool-machine", "⏳️operation-progress", "🔄️full-operation", "🤝️group", "🧩️extensions")] + [
    "🪜️close-ladder/🦀️.rs", "🪜️envelope-ladder/🦀️.rs", "🪜️maintenance-ladder/🦀️.rs", "♻️lifecycle/🏪️stores/🦀️.rs", "🎮️command/🎟️lease/🦀️.rs", "🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs", "🪆️child/👁️read/🧪️test-view/🦀️.rs"]
MOVED_FILES = ["⏪️time-travel/🧵️instance/🦀️.rs", "⏯️tool-run/🧵️instance/🦀️.rs", "⏪️time-travel/🧵️instance/🧪️tests/🦀️.rs", "⏯️tool-run/🧵️instance/🧪️tests/🦀️.rs"]
REACHES = ("pub", "pub(crate)")


def strip(text):
    text = re.sub(r'//[^\n]*', '', text)
    text = re.sub(r'/\*.*?\*/', '', text, flags=re.S)
    return re.sub(r'"(?:\\.|[^"\\])*"', '""', text)


def impl_blocks(text):
    for m in re.finditer(r'\bimpl\s*<[^{;]*?>\s*VcsArtifactApp\s*<[^{;]*?>\s*(?:where[^{]*)?\{', text):
        depth, i = 1, m.end()
        while depth and i < len(text):
            depth += {"{": 1, "}": -1}.get(text[i], 0)
            i += 1
        yield text[m.end():i - 1]


def methods(body):
    out, depth, i = [], 0, 0
    for m in re.finditer(r'[{}]|(?P<vis>pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?fn\s+(?P<name>\w+)', body):
        tok = m.group(0)
        if tok == "{":
            depth += 1
        elif tok == "}":
            depth -= 1
        elif depth == 0:
            out.append((m.group("name"), (m.group("vis") or "").replace(" ", "").strip() or "private"))
    return out


def main(root, stripped):
    read = lambda p: strip(open(os.path.join(root, p), encoding="utf-8").read())
    parent = read(INSTANCE_FILES[0])
    s = parent.index("pub struct VcsArtifactApp")
    e = parent.index("\n}\n", s)
    fields = {m.group(2): (m.group(1) or "private").replace(" ", "") for m in re.finditer(r'^\s{4}(pub(?:\([^)]*\))?\s+)?(\w+)\s*:', parent[s:e], re.M)}
    defs = {}
    for f in INSTANCE_FILES + MOVED_FILES:
        if not os.path.exists(os.path.join(root, f)):
            continue
        for body in impl_blocks(read(f)):
            for name, vis in methods(body):
                defs.setdefault(name, []).append((vis, f))
    free = {}
    depth = 0
    for m in re.finditer(r'[{}]|^(pub(?:\([^)]*\))?\s+)?(?:async\s+)?(fn|const|static|struct|enum|type)\s+(\w+)', parent, re.M):
        if m.group(0) == "{":
            depth += 1
        elif m.group(0) == "}":
            depth -= 1
        elif depth == 0 and m.group(3):
            free[m.group(3)] = (m.group(1) or "private").replace(" ", "")
    for n in stripped:
        fields = {k: ("private" if k == n else v) for k, v in fields.items()}
        defs = {k: [("private", f) for _, f in v] if k == n else v for k, v in defs.items()}
    findings = []
    moved_defs = {name for name, entries in defs.items() for _, f in entries if f in MOVED_FILES}
    for f in MOVED_FILES:
        if not os.path.exists(os.path.join(root, f)):
            continue
        text = read(f)
        for m in re.finditer(r'\b(?:self|app)\.(\w+)\b(?!\s*\()', text):
            n = m.group(1)
            if n in fields and fields[n] not in REACHES:
                findings.append(("field", n, fields[n], f))
        for m in re.finditer(r'VcsArtifactApp\s*\{([^}]*)\}', text):
            for n in re.findall(r'\b(\w+)\b\s*(?:[:,]|$)', m.group(1)):
                if n in fields and fields[n] not in REACHES:
                    findings.append(("field", n, fields[n], f))
        for m in re.finditer(r'(?:\b(?:self|app)\.|\bSelf::|VcsArtifactApp(?:::<[^>]*>)?::)(\w+)\s*(?:::<[^>]*>)?\s*\(', text):
            n = m.group(1)
            if n in moved_defs:
                continue
            for vis, df in defs.get(n, []):
                if vis not in REACHES:
                    findings.append(("method", n, vis, f + " <- " + df))
        for n, vis in free.items():
            if vis not in REACHES and re.search(r'(?<![\w.:])' + re.escape(n) + r'\b', text):
                findings.append(("item", n, vis, f))
    for f in INSTANCE_FILES:
        text = read(f)
        for m in re.finditer(r'(?:\b(?:self|app)\.|\bSelf::|VcsArtifactApp(?:::<[^>]*>)?::)(\w+)\s*(?:::<[^>]*>)?\s*\(', text):
            n = m.group(1)
            for vis, df in defs.get(n, []):
                if df in MOVED_FILES and vis not in REACHES and not any(d2 not in MOVED_FILES for _, d2 in defs[n]):
                    findings.append(("moved-method", n, vis, df + " <- " + f))
    for kind, n, vis, where in sorted(set(findings)):
        print(kind, n, vis, where)
    print("fields", len(fields), "private", sum(v not in REACHES for v in fields.values()), "methods", len(defs), "free", len(free), "findings", len(set(findings)))


if __name__ == "__main__":
    args = sys.argv[1:]
    stripped = []
    if "--strip-vis" in args:
        i = args.index("--strip-vis")
        stripped = args[i + 1].split(",")
        del args[i:i + 2]
    main(args[0], stripped)
