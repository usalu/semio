#!/usr/bin/env python3
"""c6ts: lossless split of the Architect SQLite snapshot leaf into domain children.

usage: archsplit.py plan|apply|verify
"""
import os, re, sys, collections

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ORIG = F + "/🗑️generated/c6ts-orig/architect-sqlite.ts"
LEAF_DIR = ROOT + "/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot"
VS = "️"

def d(emoji, slug):
    return emoji + VS + slug

CHILDREN = {
    "cells": (d("📏", "cells"), "📏️ Scalar cell codecs and the row cursor of the Architect SQLite projection."),
    "backing": (d("💰", "backing"), "💰️ Ordered row writers and the row-owning reader of the Architect SQLite projection."),
    "symbols": (d("🏷", "symbols"), "🏷️ Closed vocabularies of the Architect SQLite symbol columns."),
    "trace": (d("🔗", "trace"), "🔗️ Trace link rows of the Architect SQLite projection."),
    "decision": (d("🗃", "decision-records"), "🗃️ Decision record registers of the Architect SQLite projection."),
    "requirements": (d("📋", "requirements"), "📋️ Requirement registers of the Architect SQLite projection."),
    "structure": (d("🏗", "program-structure"), "🏗️ Program structure registers of the Architect SQLite projection."),
    "root": (d("📜", "document-root"), "📜️ Document level singletons of the Architect SQLite projection: meta, project definition, governance and child slots."),
}

MODEL_TYPES = {"ProgramArtifact", "ArtifactChild"}

def read():
    return open(ORIG, encoding="utf-8").read().split("\n")

def parse(L):
    imports = []
    i = 0
    while i < len(L) and (L[i].startswith("import") or L[i].startswith("/**") and i == 0):
        if L[i].startswith("import"):
            imports.append(L[i])
        i += 1
    head_doc = L[0]
    while L[i] == "":
        i += 1
    body_start = i
    units = []
    cur = None
    pending_doc = []
    for j in range(body_start, len(L)):
        l = L[j]
        if l == "" and cur is None:
            continue
        if l and l[0] not in " }":
            if l.startswith("/**") and l.rstrip().endswith("*/") and not re.match(r"/\*\*.*\*/\s*\S", l):
                pending_doc.append(l)
                continue
            cur = {"lines": pending_doc + [l], "start": j + 1 - len(pending_doc)}
            pending_doc = []
            units.append(cur)
        else:
            cur["lines"].append(l)
    for u in units:
        while u["lines"] and u["lines"][-1] == "":
            u["lines"].pop()
    return head_doc, imports, units

def unit_name(u):
    for l in u["lines"]:
        m = re.match(r"(?:export )?(?:async )?(?:function|class|const|type) (\w+)", l)
        if m:
            return m.group(1)
    raise SystemExit("no name: " + u["lines"][0][:80])

def import_specs(imports):
    table = {}
    for line in imports:
        m = re.match(r'import (type )?(?:\{(.*)\}|\* as (\w+)) from "([^"]+)";', line)
        t, names, ns, spec = m.groups()
        if ns:
            table["*" + ns] = (spec, False)
        else:
            for item in names.split(","):
                item = item.strip()
                if not item:
                    continue
                ty = bool(t) or item.startswith("type ")
                nm = item.replace("type ", "").strip()
                table[nm] = (spec, ty)
    return table

def classify(units, table):
    sets = collections.defaultdict(set)
    for nm, (spec, ty) in table.items():
        if spec.endswith("/🪪️entity-foundation/🟦️.ts"):
            sets["foundation"].add(nm)
        elif spec.endswith("/🏗️program-structure/🟦️.ts"):
            sets["structure"].add(nm)
        elif spec.endswith("/📋️requirements/🟦️.ts"):
            sets["requirements"].add(nm)
        elif spec.endswith("/🗃️decision-records/🟦️.ts"):
            sets["decision"].add(nm)
    where = {}
    for u in units:
        n = unit_name(u)
        u["name"] = n
        if n in ("Cursor", "Cells", "fail", "text", "boolean", "uint32", "uint64", "float", "optional", "textField", "optionalTextField", "note", "symbol", "timestamp"):
            w = "cells"
        elif n in ("ordered", "Reader", "header", "quantity"):
            w = "backing"
        elif re.match(r"[A-Z]\w*$", n) and u["lines"][0].startswith("const "):
            w = "symbols"
        elif n in ("trace", "readTrace"):
            w = "trace"
        elif n in ("count", "forecast", "programSnapshotToSqliteDatabase", "programSnapshotFromSqliteDatabase"):
            w = "leaf"
        else:
            m = re.match(r"(project|restore)(\w+)$", n)
            if not m:
                raise SystemExit("unclassified " + n)
            ent = m.group(2)
            if ent in ("ProgramMeta", "ProjectDefinition", "Governance", "Child"):
                w = "root"
            elif ent in sets["structure"]:
                w = "structure"
            elif ent in sets["requirements"]:
                w = "requirements"
            elif ent in sets["decision"]:
                w = "decision"
            else:
                raise SystemExit("unclassified entity " + ent)
        u["where"] = w
        where[n] = w
    return where

IDENT = re.compile(r"(?<![\w$])([A-Za-z_$][\w$]*)")
STR = re.compile(r'"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'')

def idents(text):
    t = STR.sub('""', text)
    out = set()
    for m in IDENT.finditer(t):
        s = m.start()
        if s >= 1 and t[s - 1] == "." and not (s >= 3 and t[s - 3:s] == "..."):
            continue
        out.add(m.group(1))
    return out

def convert_model(text, used_model):
    def rep(m):
        used_model.add(m.group(1))
        return m.group(1)
    return re.sub(r"\bmodel\.([A-Za-z_]\w*)", rep, text)

def plan():
    L = read()
    head_doc, imports, units = parse(L)
    table = import_specs(imports)
    where = classify(units, table)
    for u in units:
        u["text"] = "\n".join(u["lines"])
    per = collections.defaultdict(list)
    for u in units:
        per[u["where"]].append(u)
    return L, head_doc, imports, units, table, where, per

def child_rel(key):
    return "../" + CHILDREN[key][0] + "/🟦️.ts"

def bump(spec, depth_extra):
    return ("../" * depth_extra + spec) if spec.startswith("../") else spec

def build():
    L, head_doc, imports, units, table, where, per = plan()
    model_spec = table["*model"][0]
    out = {}
    external_types = {nm for nm, (_, ty) in table.items() if ty}
    needed_by_other = set()
    body_idents = {}
    for key, us in per.items():
        used_model = set()
        texts = []
        for u in us:
            t = convert_model(u["text"], used_model)
            texts.append(t)
        body_idents[key] = (idents("\n".join(texts)), used_model, texts)
    for key, (ids, um, texts) in body_idents.items():
        for nm in ids:
            if nm in where and where[nm] != key:
                needed_by_other.add(nm)
    for key, (ids, um, texts) in body_idents.items():
        is_leaf = key == "leaf"
        extra = 0 if is_leaf else 1
        lines = []
        if is_leaf:
            lines.append(head_doc)
        else:
            lines.append("/** " + CHILDREN[key][1] + " */")
        groups = collections.OrderedDict()
        def add(spec, nm, ty):
            groups.setdefault(spec, {})[nm] = ty
        for nm in sorted(um):
            add(bump(model_spec, extra), nm, nm in MODEL_TYPES)
        for nm, (spec, ty) in table.items():
            if nm.startswith("*"):
                continue
            if nm in ids:
                add(bump(spec, extra), nm, ty)
        for nm in sorted(ids):
            if nm in where and where[nm] != key and where[nm] != "leaf":
                other = where[nm]
                add(("./" if is_leaf else "../") + CHILDREN[other][0] + "/🟦️.ts", nm, nm == "Cells")
        for spec, names in groups.items():
            order = sorted(names, key=lambda s: (s.lower(), s))
            if all(names[n] for n in order):
                lines.append('import type { ' + ", ".join(order) + ' } from "' + spec + '";')
            else:
                lines.append('import { ' + ", ".join(("type " if names[n] else "") + n for n in order) + ' } from "' + spec + '";')
        lines.append("")
        for u, t in zip(per[key], texts):
            if u["name"] in needed_by_other and not t.startswith("export "):
                first = t.split("\n")
                m = re.match(r"(/\*\*.*\*/\n)?", t)
                if first[0].startswith("/**"):
                    first[1] = "export " + first[1]
                else:
                    first[0] = "export " + first[0]
                t = "\n".join(first)
            lines.append(t)
        out[key] = "\n".join(lines).rstrip("\n") + "\n"
    return out, per, where

def path_for(key):
    if key == "leaf":
        return LEAF_DIR + "/🟦️.ts"
    return LEAF_DIR + "/" + CHILDREN[key][0] + "/🟦️.ts"

def main():
    cmd = sys.argv[1]
    out, per, where = build()
    if cmd == "plan":
        for k, v in out.items():
            print(k, len(v.split("\n")) - 1, [u["name"] for u in per[k]][:3], "...")
    elif cmd == "apply":
        for k, v in out.items():
            p = path_for(k)
            os.makedirs(os.path.dirname(p), exist_ok=True)
            open(p, "w", encoding="utf-8").write(v)
            print("wrote", k, len(v.split("\n")) - 1)
    elif cmd == "trim":
        files = {k: path_for(k) for k in out}
        texts = {k: open(p, encoding="utf-8").read() for k, p in files.items()}
        imported = {k: set() for k in files}
        for k, t in texts.items():
            for m in re.finditer(r'^import (?:type )?\{([^}]*)\} from "([^"]+)";', t, re.M):
                spec = m.group(2)
                if not spec.startswith("."):
                    continue
                target = os.path.normpath(os.path.join(os.path.dirname(files[k]), spec))
                for k2, p2 in files.items():
                    if os.path.normpath(p2) == target:
                        for it in m.group(1).split(","):
                            imported[k2].add(it.strip().replace("type ", ""))
        for k, t in texts.items():
            out_lines = []
            for line in t.split(chr(10)):
                m = re.match(r"export (?:async )?(?:function|class|const|type) (\w+)", line)
                if m and m.group(1) not in imported[k] and k != "leaf":
                    line = line[len("export "):]
                elif m and k == "leaf" and m.group(1) not in ("programSnapshotToSqliteDatabase", "programSnapshotFromSqliteDatabase"):
                    line = line[len("export "):]
                out_lines.append(line)
            open(files[k], "w", encoding="utf-8").write(chr(10).join(out_lines))
        print("trimmed")
    elif cmd == "verify":
        L, head_doc, imports, units, table, where, per2 = plan()
        got = collections.Counter()
        for k in out:
            txt = open(path_for(k), encoding="utf-8").read().split("\n")
            body = [l for l in txt if not l.startswith("import ") and not l.startswith("/** ") or l.startswith("/** 📤️") or l.startswith("/** 📥️")]
            cur = None
            ulist = []
            for l in body:
                if l == "":
                    continue
                if l[0] not in " }":
                    if l.startswith("/**") and l.rstrip().endswith("*/") and not re.match(r"/\*\*.*\*/\s*\S", l):
                        ulist.append([l])
                        cur = ulist[-1]
                        continue
                    if cur is not None and len(cur) == 1 and cur[0].startswith("/**"):
                        cur.append(l)
                        continue
                    cur = [l]
                    ulist.append(cur)
                else:
                    cur.append(l)
            for ul in ulist:
                t = "\n".join(ul)
                t = re.sub(r"^(/\*\*[^\n]*\*/\n)?export ", lambda m: (m.group(1) or ""), t)
                got[t] += 1
        exp = collections.Counter()
        for u in units:
            t = re.sub(r"\bmodel\.([A-Za-z_]\w*)", r"\1", "\n".join(u["lines"]))
            t = re.sub(r"^(/\*\*[^\n]*\*/\n)?export ", lambda m: (m.group(1) or ""), t)
            exp[t] += 1
        miss = exp - got
        extra = got - exp
        print("units original:", sum(exp.values()), "new:", sum(got.values()), "missing:", len(miss), "extra:", len(extra))
        for t in list(miss)[:3]:
            print("MISSING", t[:200])
        for t in list(extra)[:3]:
            print("EXTRA", t[:200])

main()
