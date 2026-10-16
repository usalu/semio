"""include_str! of fixture JSON -> runtime read relative to the companion's CARGO_MANIFEST_DIR (edits the test sources in the repo; re-reads before editing)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import edit as E

INC = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')


def convert(test_files, manifest_dir_abs, only_json=True, dry=False, log=None, suffixes=None):
    n = 0
    for f in test_files:
        t = E.read(f)
        base = os.path.dirname(f if os.path.isabs(f) else os.path.join(E.ROOT, f))

        def sub(m):
            nonlocal n
            tgt = os.path.normpath(os.path.join(base, m.group(1)))
            if suffixes is not None and not tgt.endswith(tuple(suffixes)):
                return m.group(0)
            if suffixes is None and only_json and not tgt.endswith(".json"):
                return m.group(0)
            if not os.path.exists(tgt):
                return m.group(0)
            rel = os.path.relpath(tgt, manifest_dir_abs)
            n += 1
            return f'std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/{rel}")).expect("fixture").as_str()'
        nt = INC.sub(sub, t)
        if nt != t:
            if log is not None:
                log.append((f, len(INC.findall(t))))
            if not dry:
                E.pre_copy(f)
                E.write(f, nt)
                E.note("edit", f)
    return n


CONST = re.compile(r'^([ \t]*)((?:pub(?:\([^)]*\))?\s+)?)const (\w+): &(?:\'static\s+)?str\s*=\s*include_str!\(\s*"([^"]+)"\s*\);[ \t]*$', re.M)


def convert_statics(test_files, manifest_dir_abs, dry=False, log=None):
    """`const X: &str = include_str!(P);` -> `static X: LazyLock<String>` read at run time relative to CARGO_MANIFEST_DIR; code uses of X become `X.as_str()`;
    other include_str! of JSON -> inline run-time read. Returns the number of sites converted."""
    sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
    from stubgen import mask
    n = 0
    for f in test_files:
        t = E.read(f)
        base = os.path.dirname(f if os.path.isabs(f) else os.path.join(E.ROOT, f))
        names = []

        def defsub(m):
            nonlocal n
            tgt = os.path.normpath(os.path.join(base, m.group(4)))
            if not tgt.endswith(".json") or not os.path.exists(tgt):
                return m.group(0)
            rel = os.path.relpath(tgt, manifest_dir_abs)
            names.append(m.group(3))
            n += 1
            return f'{m.group(1)}{m.group(2)}static {m.group(3)}: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/{rel}")).expect("fixture"));'
        t2 = CONST.sub(defsub, t)
        if names:
            mk = mask(t2)
            out, last = [], 0
            rx = re.compile(r"(?<![\w$])(" + "|".join(map(re.escape, names)) + r")(?![\w])")
            for mm in rx.finditer(mk):
                line_start = mk.rfind("\n", 0, mm.start()) + 1
                line = t2[line_start:t2.find("\n", mm.start())]
                if line.lstrip().startswith(("static ", "pub static ")) and "LazyLock" in line:
                    continue
                out.append(t2[last:mm.end()])
                out.append(".as_str()")
                last = mm.end()
            out.append(t2[last:])
            t2 = "".join(out)
        before = t2
        t2 = INC.sub(lambda m: (lambda tgt: (f'std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/{os.path.relpath(tgt, manifest_dir_abs)}")).expect("fixture").as_str()' if tgt.endswith(".json") and os.path.exists(tgt) else m.group(0)))(os.path.normpath(os.path.join(base, m.group(1)))), t2)
        if t2 != before:
            n += len(INC.findall(before)) - len(INC.findall(t2))
        if t2 != t:
            if log is not None:
                log.append((f, len(names)))
            if not dry:
                E.pre_copy(f)
                E.write(f, t2)
                E.note("edit", f)
    return n
