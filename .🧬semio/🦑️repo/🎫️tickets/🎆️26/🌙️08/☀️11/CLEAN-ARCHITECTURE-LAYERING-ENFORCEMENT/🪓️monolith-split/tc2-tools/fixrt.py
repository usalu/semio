"""🧪 tc2: turn compiled test fixtures (`include_str!` / `include_bytes!` of a plain string literal) into run-time reads relative to CARGO_MANIFEST_DIR.
usage: fixrt.py <manifest-dir> <file>...            dry run
       fixrt.py --apply <manifest-dir> <file>...    write (originals go to tc2-tools/orig/)
       fixrt.py --rebase <old-manifest-dir> <new-manifest-dir> <file>...   re-relativise existing runtime_fixture_* paths for a file that moves to another crate"""
import hashlib, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/ttc")
from ttcstub import mask

ORIG = HERE + "/orig"
DECL_RX = re.compile(r"(?P<head>(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<kw>const|static)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*:\s*&\s*(?:'static\s+)?(?P<ty>str|\[u8\])\s*=\s*)$")
INC_RX = re.compile(r"include_(?P<k>str|bytes)!\s*\(")
STR_RX = re.compile(r'\s*"((?:[^"\\]|\\.)*)"\s*,?\s*\)')

HELPER_TEXT = '''
fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
'''
HELPER_BYTES = '''
fn runtime_fixture_bytes(relative: &str) -> &'static [u8] {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_slice())
}
'''


def rel_to_manifest(file_abs, literal, manifest_dir):
    target = os.path.normpath(os.path.join(os.path.dirname(file_abs), literal))
    assert os.path.exists(target), ("missing fixture", file_abs, literal, target)
    return os.path.relpath(target, manifest_dir)


MAN_RX = re.compile(r'include_(str|bytes)!\s*\(\s*concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*)"\s*\)\s*\)')


def rewrite_text(txt, file_abs, manifest_dir, only=None, old_manifest_dir=None):
    if old_manifest_dir is not None:
        def msub(m):
            target = os.path.normpath(os.path.join(old_manifest_dir, m.group(2).lstrip("/")))
            assert os.path.exists(target), ("missing manifest-relative fixture", file_abs, target)
            fn = "runtime_fixture_text" if m.group(1) == "str" else "runtime_fixture_bytes"
            return f'{fn}("{os.path.relpath(target, manifest_dir)}")'
        txt = MAN_RX.sub(msub, txt)
    sites = []
    for m in INC_RX.finditer(txt):
        line_start = txt.rfind("\n", 0, m.start()) + 1
        if txt[line_start:m.start()].lstrip().startswith("//"):
            continue
        s = STR_RX.match(txt, m.end())
        if not s:
            sites.append(("manual", m.start(), m.end(), None, None, None))
            continue
        kind = m.group("k")
        if only is not None and os.path.normpath(os.path.join(os.path.dirname(file_abs), s.group(1))) not in only:
            continue
        rel = rel_to_manifest(file_abs, s.group(1), manifest_dir)
        before = txt[:m.start()]
        d = DECL_RX.search(before[max(0, len(before) - 240):])
        tail = txt[s.end():s.end() + 4].lstrip()
        if d and tail.startswith(";"):
            sites.append(("decl", m.start() - len(d.group("head")), s.end(), kind, rel, d))
        else:
            sites.append(("inline", m.start(), s.end(), kind, rel, None))
    manual = [x for x in sites if x[0] == "manual"]
    out = txt
    names = []
    for kind, a, b, k, rel, d in sorted([x for x in sites if x[0] != "manual"], key=lambda x: -x[1]):
        lit = rel.replace("\\", "/")
        if kind == "inline":
            fn = "runtime_fixture_text" if k == "str" else "runtime_fixture_bytes"
            out = out[:a] + f'{fn}("{lit}")' + out[b:]
        else:
            ty = "&'static str" if k == "str" else "&'static [u8]"
            fn = "runtime_fixture_text" if k == "str" else "runtime_fixture_bytes"
            vis = d.group("vis") or ""
            name = d.group("name")
            names.append(name)
            out = out[:a] + f'{vis}static {name}: std::sync::LazyLock<{ty}> = std::sync::LazyLock::new(|| {fn}("{lit}"))' + out[b:]
    if names:
        masked = mask(out)
        spans = []
        for name in set(names):
            for m in re.finditer(r"(?<![A-Za-z0-9_.])" + re.escape(name) + r"(?![A-Za-z0-9_])", masked):
                before = masked[max(0, m.start() - 40):m.start()]
                if re.search(r"static\s+$", before):
                    continue
                spans.append((m.start(), m.end(), name))
        for a, b, name in sorted(spans, reverse=True):
            prev = out[:a].rstrip()[-1:]
            nxt = out[b:].lstrip()[:1]
            bare = prev in "(,[" and nxt in "),]"
            out = out[:a] + (f"*{name}" if bare else f"(*{name})") + out[b:]
    out = place_helpers(out)
    return out, len([x for x in sites if x[0] != "manual"]), manual


def mod_blocks(masked):
    blocks, stack = [], []
    for i, ch in enumerate(masked):
        if ch == "{":
            stack.append((i, bool(re.search(r"(?<![A-Za-z0-9_])mod\s+[A-Za-z_][A-Za-z0-9_]*\s*$", masked[max(0, i - 120):i]))))
        elif ch == "}" and stack:
            o, ismod = stack.pop()
            if ismod:
                blocks.append((o, i))
    return blocks


def place_helpers(out):
    masked = mask(out)
    blocks = mod_blocks(masked)
    inserts = {}
    for fn, helper in (("runtime_fixture_text", HELPER_TEXT), ("runtime_fixture_bytes", HELPER_BYTES)):
        for m in re.finditer(r"(?<![A-Za-z0-9_])" + fn + r"\(", masked):
            if re.search(r"fn\s+$", masked[max(0, m.start() - 6):m.start()]):
                continue
            inner = [b for b in blocks if b[0] < m.start() < b[1]]
            target = min(inner, key=lambda b: b[1] - b[0]) if inner else None
            scope = masked[target[0]:target[1]] if target else masked
            if re.search(r"fn\s+" + fn + r"\b", scope) and not target:
                continue
            if target and re.search(r"fn\s+" + fn + r"\b", scope):
                continue
            inserts.setdefault((target[1] if target else len(out), fn), helper)
    for (pos, fn), helper in sorted(inserts.items(), key=lambda kv: -kv[0][0]):
        if pos == len(out):
            out = out.rstrip("\n") + "\n" + helper
        else:
            out = out[:pos].rstrip(" \t") + "\n".join(("    " + l if l.strip() else l) for l in helper.split("\n")) + out[pos:]
    return out


def rebase_text(txt, file_abs, old_dir, new_dir):
    def sub(m):
        target = os.path.normpath(os.path.join(old_dir, m.group(2)))
        assert os.path.exists(target), ("missing fixture on rebase", file_abs, target)
        return f'{m.group(1)}("{os.path.relpath(target, new_dir)}")'
    return re.sub(r'(runtime_fixture_(?:text|bytes))\("([^"]+)"\)', sub, txt)


def save_orig(path):
    os.makedirs(ORIG, exist_ok=True)
    data = open(path, "rb").read()
    h = hashlib.sha256(path.encode()).hexdigest()[:16]
    dst = f"{ORIG}/{h}.rs"
    if not os.path.exists(dst):
        open(dst, "wb").write(data)
        with open(f"{ORIG}/index.tsv", "a") as f:
            f.write(f"{h}\t{hashlib.sha256(data).hexdigest()}\t{path}\n")


def main():
    a = sys.argv[1:]
    apply = False
    if a[0] == "--apply":
        apply = True
        a = a[1:]
    if a[0] == "--rebase":
        old, new, files = a[1], a[2], a[3:]
        for f in files:
            t = open(f).read()
            n = rebase_text(t, f, old, new)
            if n != t:
                save_orig(f)
                open(f, "w").write(n)
                print("rebased", f)
        return
    manifest_dir, files = a[0], a[1:]
    total = 0
    for f in files:
        t = open(f).read()
        n, c, manual = rewrite_text(t, f, manifest_dir)
        total += c
        if manual:
            print("MANUAL", f, [t[x[1]:x[1] + 100].replace("\n", " ") for x in manual])
        if apply and n != t:
            save_orig(f)
            open(f, "w").write(n)
    print(("applied " if apply else "dry ") + f"{total} sites in {len(files)} files")


if __name__ == "__main__":
    main()
