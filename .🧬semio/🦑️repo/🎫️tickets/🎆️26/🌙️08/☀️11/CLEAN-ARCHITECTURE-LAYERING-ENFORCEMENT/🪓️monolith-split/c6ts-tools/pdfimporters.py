#!/usr/bin/env python3
"""c6ts: repoint every importer of the PDF 1.7 snapshot leaf to the domain type files.

usage: pdfimporters.py dry|apply
"""
import os, re, sys, unicodedata, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
src = open(T + "/pdfsplit.py", encoding="utf-8").read().replace("\nmain()\n", "\n")
ns = {"__file__": T + "/pdfsplit.py"}
exec(compile(src, "pdfsplit", "exec"), ns)
D, DOMAIN_OF, S, BASE, ROOT, rel, type_path = ns["D"], ns["DOMAIN_OF"], ns["S"], ns["BASE"], ns["ROOT"], ns["rel"], ns["type_path"]

def fold(p):
    return unicodedata.normalize("NFC", p).replace("️", "")

LEAF = fold(S + "/🟦️.ts")
STAY = {"PdfSnapshot", "schema", "validateAgainst", "registerSchemaDocument", "SchemaRefusal"}
IEEE = "@semio-tech/framework/value/ieee754"
STMT = re.compile(r'^(?P<kw>import|export)\s+(?P<type>type\s+)?\{(?P<names>[^}]*)\}\s*from\s*(?P<q>["\'])(?P<spec>[^"\']+)(?P=q);?[ \t]*$', re.M)
INLINE = re.compile(r'import\(["\']([^"\']+)["\']\)\.(\w+)')
ANYSPEC = re.compile(r'["\'](\.[^"\'\n]*)["\']')

def resolves_to_leaf(file, spec):
    if not spec.startswith("."):
        return False
    return fold(os.path.normpath(os.path.join(os.path.dirname(file), spec))) == LEAF

def target_for(name):
    if name in STAY or name.startswith("parse"):
        return "leaf"
    if name == "Binary64":
        return "ieee"
    if name in DOMAIN_OF and DOMAIN_OF[name] != "roles":
        return DOMAIN_OF[name]
    raise SystemExit("cannot place imported name " + name)

def spec_for(file, target):
    if target == "leaf":
        return rel(file, S + "/🟦️.ts")
    if target == "ieee":
        return IEEE
    return rel(file, type_path(target))

def render(kw, stmt_type, groups, q='"'):
    out = []
    for spec, items in groups.items():
        all_type = stmt_type or all(t for _, t in items)
        names = ", ".join(n if (all_type or not t) else "type " + n for n, t in items)
        out.append("%s %s{ %s } from %s%s%s;" % (kw, "type " if all_type else "", names, q, spec, q))
    return "\n".join(out)

def candidates():
    skip = ("node_modules", "/dist/", "📤️dist", "/target/")
    for root, dirs, files in os.walk(BASE):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "dist", "📤️dist", "target")]
        for f in files:
            if f.endswith(".ts") or f.endswith(".tsx"):
                yield os.path.join(root, f)
    pdf_root = os.path.dirname(os.path.dirname(BASE.rsplit("/🏅️standards", 1)[0] + "/x"))
    for extra in (ROOT + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🟦️.ts",):
        yield extra

def main():
    apply = sys.argv[1] == "apply"
    newfiles = set()
    for dom in D:
        if dom != "roles":
            newfiles.add(fold(type_path(dom)))
    changed = []
    for f in candidates():
        if fold(f) in newfiles or fold(f) == LEAF:
            continue
        if "/🧬️schema/📸️snapshot/🧬️schema/" in f or f.startswith(S + "/🧬️schema/"):
            continue
        text = open(f, encoding="utf-8").read()
        new = text
        def sub(m):
            if not resolves_to_leaf(f, m.group("spec")):
                return m.group(0)
            groups = collections.OrderedDict()
            for item in m.group("names").split(","):
                item = item.strip()
                if not item:
                    continue
                ty = item.startswith("type ")
                nm = item[5:].strip() if ty else item
                tgt = target_for(nm)
                groups.setdefault(spec_for(f, tgt), []).append((nm, ty))
            return render(m.group("kw"), bool(m.group("type")), groups, m.group("q"))
        new = STMT.sub(sub, new)
        def sub_inline(m):
            if not resolves_to_leaf(f, m.group(1)):
                return m.group(0)
            tgt = target_for(m.group(2))
            return 'import("%s").%s' % (spec_for(f, tgt), m.group(2))
        new = INLINE.sub(sub_inline, new)
        for sm in ANYSPEC.finditer(new):
            if resolves_to_leaf(f, sm.group(1)):
                line = new[new.rfind("\n", 0, sm.start()) + 1:new.find("\n", sm.end())]
                if not re.search(r'\bfrom\b', line) or re.search(r'\{[^}]*\b(Pdf[A-Z]\w*|ObjRef|Binary64)\b', line) and not re.search(r'parse', line):
                    print("RESIDUAL", os.path.relpath(f, BASE)[-60:], line[:160])
        if new != text:
            changed.append(f)
            if apply:
                open(f, "w", encoding="utf-8").write(new)
            else:
                for a, b in zip(text.split("\n"), new.split("\n")):
                    if a != b:
                        print(os.path.relpath(f, BASE)[-80:], "\n  -", a[:200], "\n  +", b[:200].replace("\n", " | "))
                        break
    print(("applied" if apply else "would change"), len(changed), "files")

main()
