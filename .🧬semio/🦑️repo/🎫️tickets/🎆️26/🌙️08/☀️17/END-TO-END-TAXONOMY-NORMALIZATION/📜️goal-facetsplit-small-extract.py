#!/usr/bin/env python3
# 🧩️ Facet-split restorer for the five small plugins owned by this worker
# (🏭️process, ➗️mathematical, 🖨️raster, 💡️reasoning, 📖️playbook).
#
# For each mutation directory D with an inlined direct leaf D/🦀️.rs carrying
# //#region 🔖️Diff / //#region 🔖️Inverse blocks, this:
#   1. extracts those bodies into D/🔺️diff/🦀️.rs and D/↩️inverse/🦀️.rs
#   2. rewrites the leaf's impl to delegate: super::diff::diff(...) / super::inverse::inverse(...)
#   3. qualifies the payload type as super::Payload in the new facet files
#
# Doc-comment headers are recovered from the pinned pre-collapse commit when available.
import re, subprocess, os, sys

REPO = "/Users/ueli/Documents/semio"
ORIG_COMMIT = "bb06c41f73f0122fbed315b7487428b976f99921"

def git_show(path):
    r = subprocess.run(["git", "show", f"{ORIG_COMMIT}:{path}"], cwd=REPO, capture_output=True, text=True)
    if r.returncode != 0:
        return None
    return r.stdout

def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as f:
        return f.read()

def write(path, content):
    full = os.path.join(REPO, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8") as f:
        f.write(content)

DIFF_RE = re.compile(r'[ \t]*//#region 🔖️Diff\n(.*?)\n[ \t]*//#endregion 🔖️Diff\n?', re.S)
INV_RE = re.compile(r'[ \t]*//#region 🔖️Inverse\n(.*?)\n[ \t]*//#endregion 🔖️Inverse\n?', re.S)

USE_LINE_RE = re.compile(r'^use\s+(.+);\s*$')

def parse_use_lines(text):
    lines = []
    for line in text.split("\n"):
        s = line.strip()
        m = USE_LINE_RE.match(s)
        if m:
            lines.append(s)
    return lines

def idents_for_use_line(line):
    inner = line[len("use "):-1]
    m = re.search(r'\{(.+)\}\s*$', inner)
    idents = set()
    if m:
        items = [x.strip() for x in m.group(1).split(",") if x.strip()]
        for it in items:
            if " as " in it:
                idents.add(it.split(" as ")[-1].strip())
            else:
                idents.add(it.split("::")[-1])
    else:
        if " as " in inner:
            idents.add(inner.split(" as ")[-1].strip())
        else:
            idents.add(inner.split("::")[-1])
    return idents

def used_word_set(body):
    return set(re.findall(r'\b[A-Za-z_][A-Za-z0-9_]*\b', body))

def filter_use_lines(use_lines, body):
    used = used_word_set(body)
    out = []
    for line in use_lines:
        idents = idents_for_use_line(line)
        if idents & used:
            out.append(line)
    return out

def qualify_payload_in_signature(fn_text, payload_name):
    idx = fn_text.index("{")
    sig = fn_text[:idx]
    rest = fn_text[idx:]
    sig2 = re.sub(rf'(?<!super::)\b{re.escape(payload_name)}\b', f'super::{payload_name}', sig)
    return sig2 + rest

def get_orig_doc(path):
    txt = git_show(path)
    if txt is None:
        return None
    lines = []
    for line in txt.split("\n"):
        if line.startswith("//!"):
            lines.append(line)
        elif lines:
            break
    if not lines:
        return None
    return "\n".join(lines)

def find_payload_name(leaf_text):
    m = re.search(r'pub struct (\w+)', leaf_text)
    if not m:
        raise ValueError("no payload struct found")
    return m.group(1)

def is_async_fn(fn_text):
    first_line = fn_text.split("\n", 1)[0]
    return "async fn" in first_line

def replace_impl_method(txt, method_name, call_expr):
    pattern = re.compile(
        rf'((?:async )?fn {method_name}\(&self,[^)]*\)\s*(?:->[^{{]*)?\{{)\n(.*?)\n(\s*)\}}',
        re.S
    )
    m2 = pattern.search(txt)
    if not m2:
        return txt, False
    indent_match = re.search(r'\n(\s*)\S', m2.group(0))
    body_indent = indent_match.group(1) if indent_match else "        "
    new_block = f"{m2.group(1)}\n{body_indent}{call_expr}\n{m2.group(3)}}}"
    return txt[:m2.start()] + new_block + txt[m2.end():], True

def process_mutation(D):
    leaf_rel = f"{D}/🦀️.rs"
    text = read(leaf_rel)
    dm = DIFF_RE.search(text)
    im = INV_RE.search(text)
    if not dm or not im:
        return f"SKIP (no regions found): {D}"

    diff_body = dm.group(1)
    inv_body = im.group(1)
    payload_name = find_payload_name(text)

    diff_async = is_async_fn(diff_body)
    inv_async = is_async_fn(inv_body)

    diff_body_q = qualify_payload_in_signature(diff_body, payload_name)
    inv_body_q = qualify_payload_in_signature(inv_body, payload_name)

    all_use_lines = parse_use_lines(text)

    diff_use = filter_use_lines(all_use_lines, diff_body_q)
    inv_use = filter_use_lines(all_use_lines, inv_body_q)

    diff_doc = get_orig_doc(f"{D}/🔺️diff/🦀️component.rs")
    if not diff_doc:
        diff_doc = f"//! 🔺️ Sparse diff builder for `{payload_name}`."
    inv_doc = get_orig_doc(f"{D}/↩️inverse/🦀️component.rs")
    if not inv_doc:
        inv_doc = f"//! ↩️ Inverse for `{payload_name}`."

    diff_file = diff_doc + "\n"
    if diff_use:
        diff_file += "\n" + "\n".join(diff_use) + "\n"
    diff_file += "\n//#region 🔖️Diff\n" + diff_body_q + "\n//#endregion 🔖️Diff\n"

    inv_file = inv_doc + "\n"
    if inv_use:
        inv_file += "\n" + "\n".join(inv_use) + "\n"
    inv_file += "\n//#region 🔖️Inverse\n" + inv_body_q + "\n//#endregion 🔖️Inverse\n"

    new_leaf = DIFF_RE.sub("", text)
    new_leaf = INV_RE.sub("", new_leaf)
    new_leaf = re.sub(r'\n{3,}', "\n\n\n", new_leaf)
    new_leaf = new_leaf.rstrip("\n") + "\n"

    diff_call = "super::diff::diff(self, base)" + (".await" if diff_async else "")
    inv_call = "super::inverse::inverse(self, base)" + (".await" if inv_async else "")

    new_leaf, ok1 = replace_impl_method(new_leaf, "diff", diff_call)
    new_leaf, ok2 = replace_impl_method(new_leaf, "inverse", inv_call)

    write(leaf_rel, new_leaf)
    write(f"{D}/🔺️diff/🦀️.rs", diff_file)
    write(f"{D}/↩️inverse/🦀️.rs", inv_file)

    status = "OK"
    if not ok1:
        status += " [WARN diff-delegate-not-replaced]"
    if not ok2:
        status += " [WARN inverse-delegate-not-replaced]"
    return f"{status}: {D}"


if __name__ == "__main__":
    dirs_file = sys.argv[1]
    with open(dirs_file, encoding="utf-8") as f:
        dirs = [l.strip() for l in f if l.strip()]
    for D in dirs:
        try:
            print(process_mutation(D))
        except Exception as e:
            print(f"ERROR: {D}: {e}")
