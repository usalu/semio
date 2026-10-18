#!/usr/bin/env python3
"""📖 C5C9 text-reader oracle without node_modules: mimics `readModuleTreeSource` (root, `#[path]` and literal `include!` children in declaration order, `🧪️tests` mounts and mounts above the scope skipped, cfg ignored) and re-runs the string assertions of the re-pointed TS readers against the tree text; the same assertions run on the pre-split snapshot for comparison."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
OD = "🧰️framework/🛍️products/💻️os/🔨️modules"
PATH_RX = re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')
INC_RX = re.compile(r'^\s*include!\(\s*"([^"]+)"\s*\)\s*;', re.M)
V = "️"


def children(path, text, scope):
    base = os.path.dirname(path)
    out = []
    for m in PATH_RX.finditer(text):
        out.append((os.path.normpath(os.path.join(base, m.group(1))), os.path.dirname(os.path.normpath(os.path.join(base, m.group(1))))))
    for m in INC_RX.finditer(text):
        out.append((os.path.normpath(os.path.join(base, m.group(1))), scope))
    res = []
    for target, sc in out:
        rel = os.path.relpath(target, scope).split(os.sep)
        if "🧪" + V + "tests" in rel or rel[0] == "..":
            continue
        res.append((target, sc))
    return res


def tree(root, override=None):
    files, seen = [], set()
    def visit(p, scope):
        if p in seen:
            return
        seen.add(p)
        text = override if (override is not None and p == root) else open(p, encoding="utf-8").read()
        files.append((p, text))
        for c, sc in children(p, text, scope):
            visit(c, sc)
    visit(root, os.path.dirname(root))
    return files


def join(files):
    return "".join(t if t.endswith("\n") else t + "\n" for _, t in files)


def check(name, ok):
    print(("PASS " if ok else "FAIL ") + name)
    return ok


def main():
    allok = True
    for label, override_dc, override_dd in [("PRE", open(HERE + "/orig/dc.rs", encoding="utf-8").read(), open(HERE + "/orig/dd.rs", encoding="utf-8").read()), ("POST", None, None)]:
        print("=====", label)
        dcr = os.path.join(REPO, OD, "📇️directory/🔌️client/🦀️.rs")
        ddr = os.path.join(REPO, OD, "🗣️dsl/✨️derive/🦀️.rs")
        dct = join(tree(dcr, override_dc))
        ddf = tree(ddr, override_dd)
        ddt = join(ddf)
        print("dc tree files", len(tree(dcr, override_dc)), "bytes", len(dct), "| dd tree files", len(ddf), "bytes", len(ddt))
        ok = True
        ok &= check("lease: no partial surface predicate (matches_surface / DocumentSocketSurfaceExpectationV1 absent)", "matches_surface" not in dct and "DocumentSocketSurfaceExpectationV1" not in dct)
        ok &= check("journey: tag-7 socket hello encoding present", "bytes.extend_from_slice(&[0, 7, 1, 1, schema.len() as u8]);" in dct and "let schema = b\"" in dct)
        ok &= check("directory-checks: session authority", "DirectorySessionAuthorityV1::parse_canonical_json" in dct and "DIRECTORY_SESSION_AUTHORITY_MAX_BYTES" in dct)
        ok &= check("directory-checks: event page", "pub async fn event_page" in dct and "CanonicalDirectoryEventPageV1" in dct and "DIRECTORY_EVENT_PAGE_MAX_BYTES" in dct)
        ok &= check("directory-checks: ack frontier", "pub fn stream_acknowledged" in dct and "pub fn acknowledge(&mut self, through: u64)" in dct)
        ok &= check("directory-checks: bootstrap", "pub struct DirectoryEventPageBootstrapV1" in dct and "pub enum DirectoryBootstrapTransition" in dct)
        ok &= check("credential order: directory_socket_hello_v1() present", "directory_socket_hello_v1()" in dct)
        i = ddt.find("fn diff_text_tokens("); j = ddt.find("pub fn expand_diff_binary(")
        ok &= check("puzzle: diffText window contains parse_exact", i >= 0 and j > i and "::semio_framework_dsl_record::parse_exact(" in ddt[i:j])
        a = ddt.find("fn expand_mutations("); b = ddt.find("fn mutation_relative_path(")
        ok &= check("crate-boundaries: emission window ordered", 0 <= a < b)
        em = ddt[a:b]
        idents = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", em))
        print("   emission identifier count", len(idents), "bytes", len(em))
        ok &= check("macro-exports: no #[proc_macro_derive( in owner tree", re.search(r"#\[proc_macro_derive\((\w+)", ddt) is None)
        ok &= check("foreign: foreign_step_source present, foreign_steps( absent", re.search(r"fn foreign_step_source<'a>", ddt) is not None and re.search(r"fn foreign_steps\(|pub fn plan_foreign_steps", ddt) is None)
        fn_names = sorted(re.findall(r"^(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)", ddt, re.M))
        print("   top-level-ish fn count", len(fn_names))
        for n in ["expand_derive_composite_mutation", "expand_mutation_leaf", "expand_derive_mutations", "expand_dsl_document", "diff_text_tokens", "diff_binary_tokens"]:
            ok &= check("record-owner: fn %s present once" % n, fn_names.count(n) == 1)
        ok &= check("diff-codecs: expand_dsl_diff retired", "expand_dsl_diff" not in fn_names)
        allok &= ok
        open(HERE + "/../🗑️generated/c5c9-readers-%s-emission.txt" % label.lower(), "w", encoding="utf-8").write("\n".join(sorted(idents)))
        open(HERE + "/../🗑️generated/c5c9-readers-%s-fns.txt" % label.lower(), "w", encoding="utf-8").write("\n".join(fn_names))
    print("READERS", "OK" if allok else "FAIL")


if __name__ == "__main__":
    main()
