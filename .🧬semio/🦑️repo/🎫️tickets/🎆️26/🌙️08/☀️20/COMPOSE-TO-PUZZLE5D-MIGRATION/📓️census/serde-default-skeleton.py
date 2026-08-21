#!/usr/bin/env python3
"""Reads Rust snapshot schema sources and prints the exact serde-JSON a `Default`-shaped value of
one type serializes to. Used only to derive the mechanical skeleton of a fixture snapshot for the
large stdio artifacts (dwg/dxf); the interesting field values and every diff are handcrafted.

usage: serde-default-skeleton.py <root-type> <rust-file> [<rust-file> ...]
"""
import json
import re
import sys


def camel(name: str) -> str:
    head, *rest = name.split("_")
    return head + "".join(p[:1].upper() + p[1:] for p in rest)


TYPES = {}


def parse(paths):
    for path in paths:
        src = open(path, encoding="utf8").read()
        src = re.sub(r"^\s*//[^\n]*$", "", src, flags=re.M)
        for match in re.finditer(r"((?:^#\[[^\n]*\]\n)+)pub (struct|enum) (\w+)\s*\{(.*?)\n\}", src, re.S | re.M):
            attrs, kind, name, body = match.groups()
            TYPES[name] = {"kind": kind, "attrs": attrs, "body": body}


def container_rename(attrs):
    m = re.search(r'rename_all\s*=\s*"([^"]+)"', attrs)
    return m.group(1) if m else None


def container_tag(attrs):
    tag = re.search(r'\btag\s*=\s*"([^"]+)"', attrs)
    content = re.search(r'\bcontent\s*=\s*"([^"]+)"', attrs)
    return (tag.group(1) if tag else None, content.group(1) if content else None)


def split_fields(body):
    out, attrs, depth, buf = [], "", 0, ""
    for line in body.split("\n"):
        stripped = line.strip()
        if not stripped:
            continue
        if stripped.startswith("#["):
            attrs += stripped + "\n"
            continue
        buf += " " + stripped
        depth += stripped.count("<") - stripped.count(">")
        if depth <= 0 and buf.rstrip().endswith(","):
            out.append((attrs, buf.strip().rstrip(",")))
            attrs, buf, depth = "", "", 0
    if buf.strip():
        out.append((attrs, buf.strip().rstrip(",")))
    return out


def default_for(ty: str):
    ty = ty.strip()
    if ty.startswith("Box<") or ty.startswith("Option<"):
        inner = ty[ty.index("<") + 1: ty.rindex(">")]
        return None if ty.startswith("Option<") else default_for(inner)
    if ty.startswith("Vec<") or ty.startswith("VecDeque<"):
        return []
    if ty.startswith("HashMap<") or ty.startswith("BTreeMap<"):
        return {}
    if ty.startswith("["):
        inner, _, count = ty[1:-1].rpartition(";")
        return [default_for(inner)] * int(count.strip())
    if ty.startswith("("):
        return [default_for(p) for p in ty[1:-1].split(",") if p.strip()]
    if ty in ("String", "&str"):
        return ""
    if ty == "bool":
        return False
    if ty in ("f32", "f64"):
        return 0.0
    if re.fullmatch(r"[iu](8|16|32|64|128|size)", ty):
        return 0
    if ty in TYPES:
        return default_value(ty)
    raise SystemExit(f"unknown type: {ty!r}")


def default_value(name: str):
    spec = TYPES[name]
    rename = container_rename(spec["attrs"])
    if spec["kind"] == "struct":
        out = {}
        for attrs, field in split_fields(spec["body"]):
            m = re.match(r"pub (\w+)\s*:\s*(.+)", field)
            if not m:
                continue
            key, ty = m.group(1), m.group(2)
            value = default_for(ty)
            skip = re.search(r'skip_serializing_if\s*=\s*"([^"]+)"', attrs)
            if skip and ((skip.group(1).endswith("is_none") and value is None) or (skip.group(1).endswith("is_empty") and value in ([], {}, ""))):
                continue
            rn = re.search(r'#\[serde\(rename\s*=\s*"([^"]+)"\)\]', attrs)
            out[rn.group(1) if rn else (camel(key) if rename == "camelCase" else key)] = value
        return out
    variants = re.findall(r"((?:^\s*#\[[^\n]*\]\n)*)\s*([A-Z]\w*)\s*(\{[^{}]*\}|\([^()]*\))?\s*,", spec["body"], re.M)
    chosen = next((v for v in variants if "default" in v[0]), variants[0])
    tag, content = container_tag(spec["attrs"])
    label = chosen[1][:1].lower() + chosen[1][1:] if rename == "camelCase" else chosen[1]
    payload = chosen[2] or ""
    if not payload:
        return {tag: label} if tag else label
    if payload.startswith("{"):
        # serde_derive/internals/ast.rs: a container-level `rename_all` on an ENUM renames the
        # VARIANTS only. Variant FIELDS follow the variant's own `rename_all`, falling back to the
        # container's `rename_all_fields` — never to the container's `rename_all`.
        field_rename = container_rename(chosen[0]) or (re.search(r'rename_all_fields\s*=\s*"([^"]+)"', spec["attrs"]) or [None, None])[1]
        inner = {}
        for attrs, field in split_fields(payload[1:-1]):
            m = re.match(r"(\w+)\s*:\s*(.+)", field)
            if m:
                inner[camel(m.group(1)) if field_rename == "camelCase" else m.group(1)] = default_for(m.group(2))
        return ({tag: label} | inner) if tag else {label: inner}
    inner = default_for(payload[1:-1])
    if tag and content:
        return {tag: label, content: inner}
    return {label: inner}


if __name__ == "__main__":
    parse(sys.argv[2:])
    print(json.dumps(default_value(sys.argv[1]), indent=2))
