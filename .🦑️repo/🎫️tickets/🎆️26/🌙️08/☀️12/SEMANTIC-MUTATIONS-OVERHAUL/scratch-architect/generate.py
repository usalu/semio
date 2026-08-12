#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os, re, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from migrate import main as build_plan, MUT_DIR, GLUE, ROOT

RS_LEAF = "🦀️component.rs"
TS_LEAF = "🟦️component.ts"

def write(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

def dedupe_keep_order(items):
    seen = set()
    out = []
    for it in items:
        if it not in seen:
            seen.add(it)
            out.append(it)
    return out

def build_mutation_rs(p):
    from migrate import filter_uses
    uses = filter_uses(p["_mut_uses"], p["struct_body"])
    uses = dedupe_keep_order(sorted(uses))
    header = (
        f"//! 🦠️ ProgramSnapshot mutation — `{p['kind_slug']}` leaf ({p['verb']}). Split from the\n"
        f"//! pre-migration `{p['old_dir']}` noun-keyed triad per Wave C's one-triad-dir-per-variant\n"
        f"//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`\n"
        f"//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.\n\n"
    )
    body = header + "\n".join(uses) + "\n\n" + p["struct_body"].strip("\n") + "\n"
    return body

def build_diff_rs(p):
    from migrate import filter_uses
    other = [t for t in p["_diff_uses"] if not t[0].startswith("use super::mutation::")]
    uses = filter_uses(other, p["diff_fn"])
    uses = [f"use super::mutation::{p['struct_name']};"] + dedupe_keep_order(sorted(uses))
    header = f"//! 🔺️ Sparse diff construction for the `{p['kind_slug']}` mutation leaf — real handcrafted\n//! `ProgramDiff` builder, never apply-then-capture. Split from `{p['old_dir']}` per Wave C.\n\n"
    doc = (p["diff_doc"] + "\n") if p["diff_doc"] else ""
    return header + "\n".join(uses) + "\n\n" + doc + p["diff_fn"].strip("\n") + "\n"

OLD_QUALIFIED_RE = re.compile(r"super::super::\w+::mutation::(\w+)")

def resolve_cross_refs(fn_text, self_module, sibling_symbols, name_to_module):
    """Fully-qualify every occurrence of a sibling payload-struct name with its owning module's
    absolute-from-mutations-root path, matching the inline fully-qualified convention already
    used by the pre-existing 🗺️set-adjacency/🧹clear-adjacency cross-references. Two passes:
    (1) rewrite already-fully-qualified `super::super::<old-module>::mutation::Sym` paths (the
    2 pre-existing cross-refs point at now-renamed old noun dirs) to the new module names;
    (2) fully-qualify any remaining BARE sibling references (not already preceded by `::`,
    i.e. not the `ProgramMutation::Sym` enum-variant path segment)."""
    def rewrite_old_qualified(m):
        sym = m.group(1)
        target_module = name_to_module.get(sym)
        if not target_module:
            return m.group(0)
        return f"super::super::{target_module}::mutation::{sym}"

    out = OLD_QUALIFIED_RE.sub(rewrite_old_qualified, fn_text)
    for sym in sibling_symbols:
        target_module = name_to_module.get(sym)
        if not target_module:
            continue
        if sym not in out:
            continue
        if target_module == self_module:
            qualified = f"super::mutation::{sym}"
        else:
            qualified = f"super::super::{target_module}::mutation::{sym}"
        out = re.sub(r"(?<!::)\b" + re.escape(sym) + r"\b", qualified, out)
    return out

def build_inverse_rs(p, name_to_module):
    from migrate import filter_uses
    siblings = []
    for full, syms, base, is_brace in p["_inv_uses"]:
        if full.startswith("use super::mutation::"):
            siblings = syms
            break
    inv_fn_qualified = resolve_cross_refs(p["inv_fn"], p["module_ident"], siblings, name_to_module)
    other = [t for t in p["_inv_uses"] if not t[0].startswith("use super::mutation::")]
    uses = filter_uses(other, inv_fn_qualified)
    uses = dedupe_keep_order(sorted(uses))
    header = f"//! ↩️ Inverse (undo) construction for the `{p['kind_slug']}` mutation leaf — computed from\n//! captured pre-state (`base`), never by structurally inverting the diff. Split from\n//! `{p['old_dir']}` per Wave C.\n\n"
    doc = (p["inv_doc"] + "\n") if p["inv_doc"] else ""
    return header + "\n".join(uses) + "\n\n" + doc + inv_fn_qualified.strip("\n") + "\n"

RUST_TO_TS = {
    "String": "string", "str": "string", "bool": "boolean", "EntityId": "string",
    "f32": "number", "f64": "number", "u8": "number", "u16": "number", "u32": "number", "u64": "number",
    "i8": "number", "i16": "number", "i32": "number", "i64": "number", "usize": "number",
}

def field_ts_type(rust_ty):
    rust_ty = rust_ty.strip()
    m = re.match(r"^Vec<(.+)>$", rust_ty)
    if m:
        return field_ts_type(m.group(1)) + "[]"
    m = re.match(r"^Option<(.+)>$", rust_ty)
    if m:
        return field_ts_type(m.group(1)) + " | undefined"
    m = re.match(r"^Box<(.+)>$", rust_ty)
    if m:
        return field_ts_type(m.group(1))
    return RUST_TO_TS.get(rust_ty, rust_ty)

def build_mutation_ts(p):
    fields = re.findall(r"pub\s+(\w+):\s*([^,\n]+),", p["struct_body"])
    lines = [f"/** {p['verb'].capitalize()} — mirrors 🦠️mutation/{RS_LEAF}'s `{p['struct_name']}`. */",
             f"export interface {p['struct_name']} {{"]
    for name, ty in fields:
        camel = re.sub(r"_([a-z])", lambda m: m.group(1).upper(), name)
        lines.append(f"  {camel}: {field_ts_type(ty)};")
    lines.append("}")
    return "\n".join(lines) + "\n"

def build_diff_ts(p):
    return (
        f"/** 🔺️ Mirrors `diff(payload, base)` → ProgramDiff (see sibling {RS_LEAF} for the real\n"
        f" *  handcrafted logic — this is a type-level mirror only). */\n"
        f"export type Diff{p['struct_name']} = (payload: {p['struct_name']}, base: ProgramSnapshot) => ProgramDiff;\n"
    )

def build_inverse_ts(p):
    return (
        f"/** ↩️ Mirrors `inverse(payload, base)` → ProgramMutation[] (see sibling {RS_LEAF} for the\n"
        f" *  real handcrafted logic — this is a type-level mirror only). */\n"
        f"export type Inverse{p['struct_name']} = (payload: {p['struct_name']}, base: ProgramSnapshot) => ProgramMutation[];\n"
    )

def main():
    plan, name_to_module = build_plan()
    new_dirs = []
    for p in plan:
        new_dir_path = os.path.join(MUT_DIR, p["new_dir"])
        new_dirs.append(p["new_dir"])
        write(os.path.join(new_dir_path, "🦠️mutation", RS_LEAF), build_mutation_rs(p))
        write(os.path.join(new_dir_path, "🦠️mutation", TS_LEAF), build_mutation_ts(p))
        write(os.path.join(new_dir_path, "🔺️diff", RS_LEAF), build_diff_rs(p))
        write(os.path.join(new_dir_path, "🔺️diff", TS_LEAF), build_diff_ts(p))
        write(os.path.join(new_dir_path, "↩️inverse", RS_LEAF), build_inverse_rs(p, name_to_module))
        write(os.path.join(new_dir_path, "↩️inverse", TS_LEAF), build_inverse_ts(p))
    print(f"Wrote {len(plan)} new triad dirs.", file=sys.stderr)

    with open("/tmp/architect_new_dirs.json", "w", encoding="utf-8") as f:
        json.dump(new_dirs, f, ensure_ascii=False)
    with open("/tmp/architect_name_to_module.json", "w", encoding="utf-8") as f:
        json.dump(name_to_module, f, ensure_ascii=False)
    with open("/tmp/architect_plan_full.json", "w", encoding="utf-8") as f:
        json.dump([{k: v for k, v in p.items() if not k.startswith("_")} for p in plan], f, ensure_ascii=False)

if __name__ == "__main__":
    main()
