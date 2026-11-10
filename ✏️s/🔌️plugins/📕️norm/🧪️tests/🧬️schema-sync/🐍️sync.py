#!/usr/bin/env python3
"""🧬 Schema-sync fixer: JSON schema := intended current Rust model. The TS, proto and GraphQL twins are generator output (`repo ./📜️script.ts schema twins generate`), never written here.

Usage: python3 -I 🐍️sync.py [--apply] [--artifact en1991 ...] [--no-shared] [--report FILE]
Dry-run by default. Every file is re-read right before it is written; unchanged files are never touched.
Rust is never edited. Types the Rust side does not define (macro-defined diff rows, external crates) keep their JSON.
"""
import importlib.util, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("schema_check", os.path.join(HERE, "🐍️check.py"))
C = importlib.util.module_from_spec(spec)
sys.modules["schema_check"] = C
spec.loader.exec_module(C)

TYPEY = {"type", "format", "items", "$ref", "enum", "minimum", "maximum", "const"}
APPLY = False
LOG = []


def qn(n): return n if re.match(r"^[A-Za-z_]\w*$", n) else json.dumps(n)


def dump_indent(j): return json.dumps(j, indent=2, ensure_ascii=False) + "\n"


def dump_compact(d):
    c = lambda v: json.dumps(v, separators=(",", ":"), ensure_ascii=False)
    rows = []
    for k, v in d.items():
        if k in ("$defs", "definitions") and isinstance(v, dict):
            body = ",\n".join(f"    {c(dk)}:{c(dv)}" for dk, dv in v.items())
            rows.append(f"  {c(k)}:{{\n{body}\n  }}")
        else: rows.append(f"  {c(k)}:{c(v)}")
    return "{\n" + ",\n".join(rows) + "\n}\n"


def write_text(path, new, old):
    if new == old: return False
    if not path.endswith(".json"): raise ValueError("schema-sync writes JSON schemas only, never generated twin " + path)
    LOG.append(path)
    if APPLY:
        cur = open(path, encoding="utf-8").read()
        if cur != old:
            LOG.append("SKIPPED (changed meanwhile) " + path)
            return False
        open(path, "w", encoding="utf-8").write(new)
    return True


def int_node(rt):
    n = {"type": "integer"}
    if rt in C.INT_BOUNDS:
        lo, hi = C.INT_BOUNDS[rt]
        n["minimum"] = lo
        if hi is not None: n["maximum"] = hi
    return n


class Builder:
    def __init__(self, defs, defkey, enums, finder):
        self.defs, self.defkey, self.enums, self.finder = defs, defkey, enums, finder
        self.pending = {}

    def node(self, t):
        """🌳️ Returns (node, optional) or None when the Rust type cannot be lowered."""
        t = re.sub(r"\s+", " ", t.strip())
        m = re.match(r"^(?:std::)?(?:option::)?Option<(.*)>$", t)
        if m:
            r = self.node(m.group(1))
            if not r: return None
            n = dict(r[0])
            if isinstance(n.get("type"), str): n["type"] = [n["type"], "null"]
            return n, True
        m = re.match(r"^(?:std::vec::)?Vec<(.*)>$", t)
        if m:
            r = self.node(m.group(1))
            return ({"type": "array", "items": r[0]}, False) if r else None
        m = re.match(r"^(?:Box|Arc|Rc)<(.*)>$", t)
        if m: return self.node(m.group(1))
        if t in ("String", "&str") or t.endswith("::String"): return {"type": "string"}, False
        if t == "bool": return {"type": "boolean"}, False
        if t in ("f32", "f64"): return {"type": "number", "format": "double" if t == "f64" else "float"}, False
        if t in C.INT_RUST: return int_node(t), False
        name = re.sub(r"<.*", "", t.split("::")[-1])
        if name in self.enums: return {"type": "string", "enum": list(self.enums[name])}, False
        if name in self.defs: return {"$ref": f"#/{self.defkey}/{name}"}, False
        cand = self.finder(name)
        if cand is not None:
            self.pending[name] = cand
            self.defs[name] = {}
            return {"$ref": f"#/{self.defkey}/{name}"}, False
        return None


def best_candidate(cands, jrec, enums):
    best = None
    for cand in cands:
        rec = C.Rec(jrec.name, C.rust_fields(cand[1], enums))
        probe = []
        C.compare_type("x", jrec.name, jrec, "rust", rec, probe, rust_meta=True)
        if best is None or len(probe) < len(best[1]): best = (cand, probe, rec)
        if not probe: break
    return best


def sync_record(name, node, cands, enums, builder, tagfirst, state_ok=True):
    """🔧️ Mutates node (a JSON record schema). Returns True when changed."""
    if not cands: return False
    jf = {}
    props = node.get("properties") or {}
    jr = C.Rec(name, [])
    req = set(node.get("required", []))
    for k, v in props.items():
        ty, nullable, bounds = C.json_type(v, {}, {})
        jr.fields.append(C.Field(k, ty, optional=(k not in req) or nullable))
    best = best_candidate(cands, jr, enums)
    cand = best[0]
    fs = cand[1][0]
    if any(f[4] for f in fs): LOG.append(f"SKIP {name}: flatten field"); return False
    rnames = [f[0] for f in fs]
    overlap = len(set(rnames) & set(props))
    if props and overlap * 2 < len(set(props)): LOG.append(f"SKIP {name}: low overlap {overlap}/{len(props)} with {cand[0]}"); return False
    newprops, newreq = {}, []
    changed = False
    for k in list(props):
        if k in tagfirst:
            newprops[k] = props[k]
            if k in req: newreq.append(k)
    for wire, ftype, st, raw, flat in fs:
        old = props.get(wire)
        rty, ropt, rt = C.rust_ty(ftype, enums)
        if old is not None:
            oty, onull, ob = C.json_type(old, {}, {})
            same = C.cmp_field_types(oty, rty, "rust") is None and ((wire not in req) or onull) == ropt
            if same and rty.kind == "integer" and rt in C.INT_BOUNDS: same = C.bounds_within(ob, C.INT_BOUNDS[rt])
            if same and rty.kind == "ref" and oty.kind == "ref": same = C.PREFIX.sub("", oty.name) == C.PREFIX.sub("", rty.name)
            if same and rty.kind == "enum" and oty.kind == "enum": same = oty.members == rty.members
            if same and rty.kind == "array" and oty.kind == "array": same = C.cmp_field_types(oty, rty, "rust") is None
            if same:
                newprops[wire] = old
                if not ropt or (wire in req and onull): newreq.append(wire)
                continue
            tonly = C.cmp_field_types(oty, rty, "rust") is None and not (rty.kind == "integer" and rt in C.INT_BOUNDS and not C.bounds_within(ob, C.INT_BOUNDS[rt]))
            if tonly and rty.kind in ("ref", "enum", "array", "string", "number", "boolean", "integer"):
                n = dict(old)
                if not ropt and isinstance(n.get("type"), list): n["type"] = next(x for x in n["type"] if x != "null")
                newprops[wire] = n
                if not ropt or (wire in req and onull): newreq.append(wire)
                changed = True
                continue
        low = builder.node(ftype)
        if low is None:
            if old is not None:
                newprops[wire] = old
                if wire in req: newreq.append(wire)
            continue
        lowered, opt = low
        if old is not None:
            n = {k: v for k, v in old.items() if k not in TYPEY}
            n2 = dict(lowered)
            if lowered.get("type") == "integer" and old.get("type") in ("integer", ["integer", "null"]):
                for bnd in ("minimum", "maximum"):
                    if bnd in old and rt in C.INT_BOUNDS and C.bounds_within((old.get("minimum"), old.get("maximum")), C.INT_BOUNDS[rt]): n2[bnd] = old[bnd]
            if lowered.get("type") == "number" and old.get("type") in ("number", ["number", "null"]):
                for bnd in ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"):
                    if bnd in old: n2[bnd] = old[bnd]
            newprops[wire] = {**n2, **n}
            changed = True
        else:
            n = dict(lowered)
            if st: n["x-semio-state"] = st
            newprops[wire] = n
            changed = True
        if not opt: newreq.append(wire)
        elif isinstance(lowered.get("type"), list) and wire in req: newreq.append(wire)
    if list(newprops) != list(props): changed = True
    if newreq != list(node.get("required", [])): changed = True
    if not changed: return False
    node["properties"] = newprops
    if newreq: node["required"] = newreq
    else: node.pop("required", None)
    if "type" not in node: node["type"] = "object"
    return True


def phase_a(unit):
    key, base, d, twins = unit
    if "🦀️.rs" not in twins and not (os.path.basename(d) == "🧬️schema" and "🧬️mutations" in d): return
    path = os.path.join(d, "🔣️.json")
    text = open(path, encoding="utf-8").read()
    j = json.loads(text)
    if "$schema" not in j: return
    fmt = "indent" if dump_indent(j) == text else "compact" if dump_compact(j) == text else "indent"
    defkey = "$defs" if "$defs" in j else "definitions" if "definitions" in j else "$defs"
    defs = j.get(defkey, {})
    C.REF_CTX["dir"] = d
    jm = C.parse_json(path)
    rm = C.build_rust(d, base, list(jm.recs))
    enums = rm.enums
    cache = {}

    def finder(name):
        if name in cache: return cache[name]
        own, _ = C.rust_module(d, base, [name])
        ix, _ = C.artifact_index(base)
        c = own.get(name) or ix.get(name) or C.shared_index().get(name)
        cache[name] = c
        return c

    builder = Builder(defs, defkey, enums, finder)
    changed = False
    targets = []
    if j.get("properties") is not None: targets.append((j.get("title"), j))
    for n, dn in list(defs.items()):
        if isinstance(dn, dict) and "properties" in dn: targets.append((n, dn))
    for n, nd in targets:
        cands = rm.cands.get(n)
        if not cands:
            if os.path.basename(d) == "🔺️diff" and nd.get("properties") and "required" not in nd and n != j.get("title"):
                nd["required"] = list(nd["properties"])
                changed = True
            continue
        if sync_record(n, nd, cands, enums, builder, {"mutation"}): changed = True
    guard = 0
    while builder.pending and guard < 50:
        guard += 1
        name, cand = builder.pending.popitem()
        nd = {"type": "object", "additionalProperties": False}
        defs[name] = nd
        sync_record(name, nd, cand, enums, builder, set())
        changed = True
    if defs and defkey not in j: j[defkey] = defs
    if changed:
        write_text(path, dump_compact(j) if fmt == "compact" else dump_indent(j), text)


def main(argv):
    global APPLY
    arts, shared, rep = [], True, None
    i = 0
    while i < len(argv):
        a = argv[i]
        if a == "--apply": APPLY = True
        elif a == "--artifact": arts.append(argv[i + 1]); i += 1
        elif a == "--no-shared": shared = False
        elif a == "--report": rep = argv[i + 1]; i += 1
        else: sys.exit("unknown argument " + a)
        i += 1
    units = C.discover_units(arts or C.ARTIFACTS, shared, [], False)
    for u in units:
        try:
            phase_a(u)
        except Exception as e:
            LOG.append(f"ERROR {u[2]}: {type(e).__name__}: {e}")
    print(("applied" if APPLY else "dry-run"), "files changed:", len([x for x in LOG if not x.startswith(("SKIP", "ERROR"))]))
    for x in LOG:
        if x.startswith(("SKIP", "ERROR")): print(x)
    if rep: open(rep, "w", encoding="utf-8").write("\n".join(LOG) + "\n")


if __name__ == "__main__":
    main(sys.argv[1:])
