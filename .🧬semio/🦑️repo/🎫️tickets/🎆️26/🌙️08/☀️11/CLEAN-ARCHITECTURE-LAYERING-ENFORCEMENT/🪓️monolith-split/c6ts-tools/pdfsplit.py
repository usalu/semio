#!/usr/bin/env python3
"""c6ts: lossless split of the PDF 1.7 snapshot schema leaf and the native JSON schema leaf.

usage: pdfsplit.py plan|apply|verify
"""
import os, re, sys, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pdfdomains import D

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
BASE = ROOT + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"
S = BASE + "/🧬️schema/📸️snapshot"
N = BASE + "/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema"
ORIG_S = GEN + "/c6ts-orig/pdf-snapshot.ts"
ORIG_N = GEN + "/c6ts-orig/pdf-native-schema.ts"
VS = "️"

def d(emoji, slug):
    return emoji + VS + slug

DIR = {
    "cos": (d("🧩", "cos"), "🧩"),
    "document": (d("📄", "document"), "📄"),
    "metadata": (d("📇", "metadata"), "📇"),
    "navigation": (d("🎯", "navigation"), "🎯"),
    "form": (d("📝", "form"), "📝"),
    "color": (d("🌈", "color"), "🌈"),
    "content": (d("🖋", "content"), "🖋"),
    "image": (d("🖼", "resource"), "🖼"),
    "shading": (d("🖼", "resource") + "/" + d("🌈", "shading"), "🌅"),
    "state": (d("🖼", "resource") + "/" + d("🎚", "state"), "🎚"),
    "font": (d("🔤", "font"), "🔤"),
    "annotation": (d("📌", "annotation"), "📌"),
    "annotation-kind": (d("📌", "annotation") + "/" + d("🏷", "kind"), "🏷"),
}
ROLES_DIR = d("🪪", "stream-roles")
OP_GROUPS = [
    ("path", d("🛤", "path"), "🛤", "pathOperatorVariants", "beginText"),
    ("text", d("🔤", "text"), "✒", "textOperatorVariants", "setStrokeColorSpace"),
    ("paint", d("🎨", "paint"), "🎨", "paintOperatorVariants", None),
]
DOMAIN_OF = {n: dom for dom, ns in D.items() for n in ns}
CONST = {
    "cos": "cosDefinitions", "document": "documentDefinitions", "metadata": "metadataDefinitions",
    "navigation": "navigationDefinitions", "form": "formDefinitions", "color": "colorDefinitions",
    "content": "contentDefinitions", "image": "imageDefinitions", "shading": "shadingDefinitions",
    "state": "stateDefinitions", "font": "fontDefinitions", "annotation": "annotationDefinitions",
    "annotation-kind": "annotationKindDefinitions", "roles": "streamRoleDefinitions",
}
DOC = {
    "cos": "COS objects, indirect references and stream filters",
    "document": "pages and indirect objects",
    "metadata": "document information, encryption and viewer settings",
    "navigation": "actions, destinations, outlines and page labels",
    "form": "interactive forms and optional content",
    "color": "color spaces and functions",
    "content": "content stream operands and text",
    "image": "image XObjects",
    "shading": "shadings and patterns",
    "state": "graphics state, soft masks and form XObjects",
    "font": "fonts, encodings and character maps",
    "annotation": "annotations and appearances",
    "annotation-kind": "annotation kinds",
}

STR = re.compile(r'"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'')
IDENT = re.compile(r"(?<![\w$.])([A-Za-z_$][\w$]*)")

def idents(text):
    return set(IDENT.findall(STR.sub('""', text)))

def rel(from_file, to_file):
    r = os.path.relpath(to_file, os.path.dirname(from_file))
    return r if r.startswith(".") else "./" + r

def type_path(dom):
    return S + "/" + DIR[dom][0] + "/🟦️.ts"

def defs_path(base, dom):
    if dom == "roles":
        return base + "/🧬️schema/" + ROLES_DIR + "/🟦️.ts" if base == S else base + "/" + ROLES_DIR + "/🟦️.ts"
    return (base + "/🧬️schema/" if base == S else base + "/") + DIR[dom][0] + "/🟦️.ts"

def op_path(base, slug):
    return (base + "/🧬️schema/" if base == S else base + "/") + DIR["content"][0] + "/" + slug + "/🟦️.ts"

class Doc:
    pass

def parse_schema(L, kind):
    sc = next(i for i, l in enumerate(L) if l.startswith("export const schema"))
    dl = next(i for i in range(sc, len(L)) if L[i] == '  "$defs": {')
    end = next(i for i in range(dl, len(L)) if L[i].startswith("} as const;"))
    close = end - 1
    assert L[close] == "  }", L[close]
    starts = [i for i in range(dl + 1, close) if re.match(r'^    "\w+": \{', L[i])]
    blocks = collections.OrderedDict()
    for k, s in enumerate(starts):
        e = starts[k + 1] if k + 1 < len(starts) else close
        name = re.match(r'^    "(\w+)"', L[s]).group(1)
        blocks[name] = L[s:e]
    return sc, dl, end, close, blocks

def dedent_block(lines, last):
    out = [l[2:] if l.startswith("  ") else l for l in lines]
    out[-1] = out[-1].rstrip(",") + ("" if last else ",")
    return out

def defs_file(doc, const, blocks_lines):
    out = ["/** " + doc + " */", "export const " + const + " = {"]
    for i, b in enumerate(blocks_lines):
        out += dedent_block(b, i == len(blocks_lines) - 1)
    out.append("} as const;")
    return "\n".join(out) + "\n"

def split_op(block):
    assert block[0] == '    "PdfOp": {' and block[1] == '      "oneOf": [', block[:2]
    assert block[-2] == "      ]" and block[-1] in ("    }", "    },"), block[-2:]
    inner = block[2:-2]
    els = []
    cur = None
    for l in inner:
        if l == "        {":
            cur = [l]
            els.append(cur)
        else:
            cur.append(l)
    names = []
    for e in els:
        m = re.search(r'"op": \{\s*"const": "(\w+)"', "\n".join(e))
        names.append(m.group(1))
    return block, els, names

def op_groups(els, names):
    cuts = [names.index(g[4]) for g in OP_GROUPS if g[4]]
    ranges = [(0, cuts[0]), (cuts[0], cuts[1]), (cuts[1], len(els))]
    return ranges

def variants_file(doc, const, els):
    out = ["/** " + doc + " */", "export const " + const + " = ["]
    for i, e in enumerate(els):
        lines = [l[6:] if l.startswith("      ") else l for l in e]
        lines[-1] = lines[-1].rstrip(",") + ("" if i == len(els) - 1 else ",")
        out += lines
    out.append("] as const;")
    return "\n".join(out) + "\n"

def content_defs_file(base, doc, blocks_for_dom, op_block, groups):
    here = defs_path(base, "content")
    out = ["/** " + doc + " */"]
    for g in OP_GROUPS:
        out.append("import { " + g[3] + ' } from "' + rel(here, op_path(base, g[1])) + '";')
    out.append("")
    out.append("export const " + CONST["content"] + " = {")
    items = []
    for name, lines in blocks_for_dom:
        if name == "PdfOp":
            ol = ['  "PdfOp": {', '    "oneOf": [']
            for gi, g in enumerate(OP_GROUPS):
                ol.append("      ..." + g[3] + ("," if gi < len(OP_GROUPS) - 1 else ""))
            ol += ["    ]", "  }"]
            items.append(ol)
        else:
            items.append(dedent_block(lines, False))
    for i, it in enumerate(items):
        it = list(it)
        it[-1] = it[-1].rstrip(",") + ("" if i == len(items) - 1 else ",")
        out += it
    out.append("} as const;")
    return "\n".join(out) + "\n"

def build_defs_files(base, blocks, kind):
    files = {}
    for dom in list(D.keys()):
        names = [n for n in D[dom] if n in blocks]
        if not names:
            continue
        order = [n for n in blocks if n in names]
        if dom == "roles":
            doc = "🪪\ufe0f JSON Schema definitions of admitted stream roles."
            files[defs_path(base, "roles")] = defs_file(doc, CONST["roles"], [blocks[n] for n in order])
            continue
        doc = "%s\ufe0f JSON Schema definitions of the PDF 1.7 %s." % (DIR[dom][1], DOC[dom])
        if dom == "content":
            block, els, opnames = split_op(blocks["PdfOp"])
            ranges = op_groups(els, opnames)
            for g, (a, b) in zip(OP_GROUPS, ranges):
                gdoc = "%s\ufe0f JSON Schema variants of the PDF 1.7 %s operators." % (g[2], {"path": "graphics state, path and clipping", "text": "text and glyph width", "paint": "color, painting and marked content"}[g[0]])
                files[op_path(base, g[1])] = variants_file(gdoc, g[3], els[a:b])
            files[defs_path(base, dom)] = content_defs_file(base, doc, [(n, blocks[n]) for n in order], block, ranges)
        else:
            files[defs_path(base, dom)] = defs_file(doc, CONST[dom], [blocks[n] for n in order])
    return files

def spread_imports(base, leaf, doms):
    lines = []
    for dom in doms:
        lines.append("import { " + CONST[dom] + ' } from "' + rel(leaf, defs_path(base, dom)) + '";')
    return lines

SCHEMA_DOMS = list(D.keys())

def build_native():
    L = open(ORIG_N, encoding="utf-8").read().split("\n")
    sc, dl, end, close, blocks = parse_schema(L, "native")
    files = build_defs_files(N, blocks, "native")
    leaf = N + "/🟦️.ts"
    present = [dom for dom in SCHEMA_DOMS if any(n in blocks for n in D[dom])]
    head = L[:sc]
    assert len(head) == 1 and head[0].startswith("/** "), head
    out = head + spread_imports(N, leaf, present) + [""] + L[sc:dl + 1]
    for i, dom in enumerate(present):
        out.append("    ..." + CONST[dom] + ("," if i < len(present) - 1 else ""))
    out += L[close:end + 1] + L[end + 1:]
    files[leaf] = "\n".join(out).rstrip("\n") + "\n"
    return files, blocks, present

def parse_types(L, ts, te):
    blocks = collections.OrderedDict()
    cur = None
    for i in range(ts, te):
        m = re.match(r"^export (?:interface|type) (\w+)", L[i])
        if m:
            cur = m.group(1)
            blocks[cur] = []
        if cur:
            blocks[cur].append(L[i])
    for k in blocks:
        while blocks[k] and blocks[k][-1] == "":
            blocks[k].pop()
    return blocks

def type_imports(here, text, own):
    ids = idents(text)
    groups = collections.OrderedDict()
    for nm in sorted(ids):
        if nm in own:
            continue
        if nm == "Binary64":
            groups.setdefault("@semio-tech/framework/value/ieee754", []).append(nm)
        elif nm == "ArtifactRef":
            groups.setdefault("@semio-tech/framework/schema/artifact-reference", []).append(nm)
        elif nm in DOMAIN_OF and nm in TYPE_BLOCKS:
            groups.setdefault(rel(here, type_path(DOMAIN_OF[nm])), []).append(nm)
    return ["import type { " + ", ".join(v) + ' } from "' + k + '";' for k, v in groups.items()]

TYPE_BLOCKS = {}

def build_snapshot():
    global TYPE_BLOCKS
    L = open(ORIG_S, encoding="utf-8").read().split("\n")
    sc, dl, end, close, blocks = parse_schema(L, "snapshot")
    ts = next(i for i, l in enumerate(L) if l.startswith("export interface PdfSnapshot"))
    doc_line = next(i for i in range(ts, sc) if L[i].startswith("/** 🔣️ The JSON Schema document"))
    tblocks = parse_types(L, ts, doc_line)
    TYPE_BLOCKS = tblocks
    files = build_defs_files(S, blocks, "snapshot")
    for dom in D:
        names = [n for n in D[dom] if n in tblocks]
        if not names:
            continue
        order = [n for n in tblocks if n in names]
        here = type_path(dom)
        body = "\n\n".join("\n".join(tblocks[n]) for n in order)
        imps = type_imports(here, body, set(order))
        text = "/** " + DIR[dom][1] + "\ufe0f PDF 1.7 snapshot types of " + DOC[dom] + ". */\n" + "\n".join(imps) + ("\n" if imps else "") + "\n" + body + "\n"
        files[here] = text
    leaf = S + "/🟦️.ts"
    present = [dom for dom in SCHEMA_DOMS if any(n in blocks for n in D[dom])]
    reg = next(i for i, l in enumerate(L) if l.startswith("//#region 🚪️Validation"))
    snap = "\n".join(tblocks["PdfSnapshot"])
    leaf_types = {"PdfSnapshot"}
    parser_text = "\n".join(L[reg:])
    all_ids = idents(snap) | idents(parser_text)
    groups = collections.OrderedDict()
    for dom in D:
        names = sorted(n for n in D[dom] if n in tblocks and n in all_ids)
        if names:
            groups[rel(leaf, type_path(dom))] = names
    imp = []
    imp.append("/** 🧬\ufe0f Canonical owned PDF1.7 domain with explicit admission from its native JSON schema. */")
    imp.append('import { validateRoleInputs, type PdfAdmittedStreamRole } from "../🪪️stream-roles/🟦️.ts";')
    if "Binary64" in all_ids:
        imp.append('import type { Binary64 } from "@semio-tech/framework/value/ieee754";')
    if "parseBinary64" in all_ids:
        imp.append('import { parseBinary64 } from "@semio-tech/framework/value/ieee754";')
    for k, v in groups.items():
        imp.append("import type { " + ", ".join(v) + ' } from "' + k + '";')
    imp += spread_imports(S, leaf, present)
    out = imp + [""] + tblocks["PdfSnapshot"] + [""] + L[doc_line:dl + 1]
    for i, dom in enumerate(present):
        out.append("    ..." + CONST[dom] + ("," if i < len(present) - 1 else ""))
    out += L[close:]
    files[leaf] = "\n".join(out).rstrip("\n") + "\n"
    return files, blocks, tblocks, present, L, ts

def main():
    cmd = sys.argv[1]
    nfiles, nblocks, npresent = build_native()
    sfiles, sblocks, tblocks, spresent, SL, ts = build_snapshot()
    if cmd == "plan":
        for name, files in (("native", nfiles), ("snapshot", sfiles)):
            print("==", name)
            for p, t in sorted(files.items()):
                print(len(t.split("\n")) - 1, p.replace(BASE, "").replace("/🧬️schema", "/S"))
    elif cmd == "apply":
        for files in (nfiles, sfiles):
            for p, t in files.items():
                os.makedirs(os.path.dirname(p), exist_ok=True)
                open(p, "w", encoding="utf-8").write(t)
        print("written", len(nfiles) + len(sfiles))
    elif cmd == "verify":
        verify(nfiles, sfiles, sblocks, nblocks, SL)

def canon(o):
    return json.dumps(o, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def extract_schema(text):
    i = text.index("export const schema")
    j = text.index("{", i)
    k = text.index("} as const;", j) + 1
    return text[j:k]

CANON = "const canon=(o:unknown):string=>JSON.stringify(o,(k,v)=>v&&typeof v===\"object\"&&!Array.isArray(v)?Object.fromEntries(Object.entries(v).sort(([a],[b])=>a<b?-1:a>b?1:0)):v);\n"

def run_bun(source, tag):
    tmp = GEN + "/c6ts-eval-%s.ts" % tag
    open(tmp, "w", encoding="utf-8").write(source)
    r = subprocess.run(["bun", "run", tmp], capture_output=True, text=True, cwd=ROOT)
    os.remove(tmp)
    if r.returncode != 0:
        raise SystemExit("eval failed: " + r.stderr[:2000])
    return r.stdout.strip().splitlines()[-1]

def eval_schema(leaf_text, leaf_path, tag):
    imports = [l for l in leaf_text.split("\n") if l.startswith("import { ") and "Definitions }" in l]
    fixed = []
    for l in imports:
        m = re.match(r'import \{ (\w+) \} from "(.+)";', l)
        fixed.append('import { %s } from "%s";' % (m.group(1), os.path.normpath(os.path.join(os.path.dirname(leaf_path), m.group(2)))))
    body = "export const schema = " + extract_schema(leaf_text) + ";\n" + CANON + "console.log(canon(schema));\n"
    return run_bun("\n".join(fixed) + "\n" + body, tag)

def eval_original(text, tag):
    return run_bun("export const schema = " + extract_schema(text) + ";\n" + CANON + "console.log(canon(schema));\n", tag + "-orig")

def verify(nfiles, sfiles, sblocks, nblocks, SL):
    ok = True
    for tag, orig, leaf in (("native", ORIG_N, N + "/🟦️.ts"), ("snapshot", ORIG_S, S + "/🟦️.ts")):
        got = eval_schema(open(leaf, encoding="utf-8").read(), leaf, tag)
        want = eval_original(open(orig, encoding="utf-8").read(), tag)
        same = got == want
        print(tag, "schema canonical equal (bun evaluated):", same, len(got), len(want))
        ok &= same
    print("OK" if ok else "FAILED")

main()
