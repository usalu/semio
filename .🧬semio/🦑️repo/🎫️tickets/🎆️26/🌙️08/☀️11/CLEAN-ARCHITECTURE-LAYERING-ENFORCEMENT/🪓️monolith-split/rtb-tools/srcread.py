"""Converts the compile-time source embeds of a test file into run-time reads of the test-support crate.
usage: srcread.py <file>... [--apply] [--mdir <manifest dir>]
 - include_str!("….rs") / concat!(include_str!(…rs)…)   -> repo_text("…") / repo_texts(&[…]); a `const NAME: &str` that holds one becomes `fn NAME() -> &'static str` and its uses become `NAME()`
 - include_str!/include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "…")) -> a file-relative literal (the text no longer depends on the manifest depth)
 - crate::RENDERER_SOURCE & co                           -> renderer_source() & co
 - a join("../../../..") engine-root idiom               -> engine_root()
EMBED_LIMIT=<bytes>: any other include_str!/include_bytes! of a file at least that large (JSON fixtures, GLB) becomes repo_text / repo_bytes.
Anything else that still names CARGO_MANIFEST_DIR is reported."""
import os, re, sys
from lib import *

TS = "semio_framework_os_renderer_wgpu_test_support"
TEXT_SUFFIXES = (".rs", ".toml")
EMBED_LIMIT = int(os.environ.get("EMBED_LIMIT", "0"))
TOP_MANIFEST = T + "/📦️packages/🦀️rust"
SRC_FN = {"RENDERER_SOURCE": "renderer_source", "SHELL_SOURCE": "shell_source", "SCENES_SOURCE": "scenes_source", "ENGINE_CANVAS_SOURCE": "engine_canvas_source", "INTERPRETER_SOURCE": "interpreter_source"}
SRC_PATHS = re.compile(r"(?<![A-Za-z0-9_$])(?:crate::(?:(?:shell|scenes|engine_canvas|interpreter)::)?|super::)(" + "|".join(SRC_FN) + r")\b")


def balanced(text, open_at):
    """Index just past the parenthesis group that opens at text[open_at] == '('."""
    clean = blank(text)
    depth, i = 0, open_at
    while i < len(clean):
        if clean[i] == "(":
            depth += 1
        elif clean[i] == ")":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise ValueError("unbalanced")


def eval_arg(arg, file, mdir):
    """Absolute path an include argument names, or None."""
    a = arg.strip()
    m = re.fullmatch(r'"((?:[^"\\]|\\.)*)"', a)
    if m:
        return os.path.normpath(os.path.join(os.path.dirname(file), m.group(1))), "literal"
    m = re.fullmatch(r'concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*(.*)\)', a, re.S)
    if m:
        parts = re.findall(r'"((?:[^"\\]|\\.)*)"', m.group(1))
        rest = re.sub(r'\s*"((?:[^"\\]|\\.)*)"\s*,?', "", m.group(1))
        if rest.strip():
            return None, "complex"
        return os.path.normpath(mdir + "".join(parts)), "manifest"
    return None, "complex"


def relpath_from(file, target):
    r = os.path.relpath(target, os.path.dirname(file))
    return r


def convert(file, text, mdir):
    notes = []
    clean = blank(text)
    # 1. concat!(include_str!(…rs), …) groups
    edits = []  # (start, end, replacement)
    consumed = []
    for m in re.finditer(r"(?<![A-Za-z0-9_$])concat!\(", clean):
        end = balanced(text, m.end() - 1)
        inner = text[m.end():end - 1]
        incs = list(re.finditer(r"include_str!\(", blank(inner)))
        if not incs:
            continue
        pieces = []
        ok = True
        pos = 0
        ci = blank(inner)
        for im in incs:
            e2 = balanced(inner, im.end() - 1)
            between = ci[pos:im.start()].strip(" ,\n\t")
            if between:
                ok = False
            pos = e2
            p, kind = eval_arg(inner[im.end():e2 - 1], file, mdir)
            if p is None or not p.endswith(TEXT_SUFFIXES):
                ok = False
            pieces.append(p)
        if ok and not ci[pos:].strip(" ,\n\t"):
            rels = [os.path.relpath(p, REPO) for p in pieces]
            for r_ in rels:
                if not os.path.exists(REPO + "/" + r_):
                    notes.append(f"missing source {r_}")
            edits.append((m.start(), end, f"{TS}::repo_texts(&[{', '.join(chr(34) + r_ + chr(34) for r_ in rels)}])"))
            consumed.append((m.start(), end))
    # 2. single includes
    for m in re.finditer(r"(?<![A-Za-z0-9_$])include_(str|bytes)!\(", clean):
        if any(s <= m.start() < e for s, e in consumed):
            continue
        end = balanced(text, m.end() - 1)
        arg = text[m.end():end - 1]
        p, kind = eval_arg(arg, file, mdir)
        if p is None:
            notes.append(f"complex include {arg.strip()[:80]}")
            continue
        if not os.path.exists(p):
            notes.append(f"missing include target {os.path.relpath(p, REPO)}")
            continue
        if p.endswith(TEXT_SUFFIXES) and m.group(1) == "str":
            edits.append((m.start(), end, f'{TS}::repo_text("{os.path.relpath(p, REPO)}")'))
        elif EMBED_LIMIT and os.path.getsize(p) >= EMBED_LIMIT:
            fn = "repo_text" if m.group(1) == "str" else "repo_bytes"
            edits.append((m.start(), end, f'{TS}::{fn}("{os.path.relpath(p, REPO)}")'))
        elif kind == "manifest":
            edits.append((m.start(), end, f'include_{m.group(1)}!("{relpath_from(file, p)}")'))
    # 3. *_SOURCE paths
    for m in SRC_PATHS.finditer(clean):
        edits.append((m.start(), m.end(), f"{TS}::{SRC_FN[m.group(1)]}()"))
    # 4. engine-root idiom
    idiom = re.compile(r'(?:std::path::)?PathBuf::from\(env!\("CARGO_MANIFEST_DIR"\)\)\.join\("\.\./\.\./\.\./\.\."\)\.canonicalize\(\)\.expect\("engine root"\)')
    for m in idiom.finditer(text):
        edits.append((m.start(), m.end(), f"{TS}::engine_root()"))
    edits.sort()
    out, last = [], 0
    for s, e, r_ in edits:
        if s < last:
            continue
        out.append(text[last:s])
        out.append(r_)
        last = e
    out.append(text[last:])
    new = "".join(out)
    # 5. consts that now hold a run-time read become functions
    converted = []
    cre = re.compile(r"(?m)^([ \t]*)((?:pub(?:\([^)]*\))?[ \t]+)?)(?:const|static) ([A-Z][A-Z0-9_]*): &(?:'static )?str = ([^;]*(?:\n[^;]*)*?);[ \t]*$")
    changed = True
    while changed:
        changed = False
        for m in cre.finditer(blank(new) if False else new):
            name, init = m.group(3), m.group(4)
            if (f"{TS}::" in init or any(c + "()" in init for c in converted)) and name not in converted:
                converted.append(name)
                body = init.strip()
                repl = f"{m.group(1)}#[allow(non_snake_case)]\n{m.group(1)}{m.group(2)}fn {name}() -> &'static str {{\n{m.group(1)}    {body}\n{m.group(1)}}}"
                new = new[:m.start()] + repl + new[m.end():]
                changed = True
                break
    for name in converted:
        cl = blank(new)
        outp, lastp = [], 0
        for m in re.finditer(r"(?<![A-Za-z0-9_$])" + name + r"\b(?!\s*[(!])", cl):
            before = cl[max(0, m.start() - 8):m.start()]
            if re.search(r"fn\s+$", before):
                continue
            outp.append(new[lastp:m.end()])
            outp.append("()")
            lastp = m.end()
        outp.append(new[lastp:])
        new = "".join(outp)
        notes.append(f"const {name} -> fn")
    left = [l.strip()[:120] for l in blank(new).split("\n") if "CARGO_MANIFEST_DIR" in l]
    for l in left:
        notes.append("manifest-dir left: " + l)
    return new, notes


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if "--all-tests" in sys.argv:
        args = [REPO + "/" + p for p in test_sources() if "/🧪️tests/" in p or "worker-cell" in p or "outcome" in p or "sqlite" in p]
    mdir = TOP_MANIFEST
    if "--mdir" in sys.argv:
        mdir = sys.argv[sys.argv.index("--mdir") + 1]
        args = [a for a in args if a != mdir]
    for f in args:
        t = read(f)
        n, notes = convert(f, t, mdir)
        print(("CHANGE " if n != t else "same   ") + os.path.relpath(f, E)[-80:], notes)
        if "--apply" in sys.argv and n != t:
            edit(f, n)
