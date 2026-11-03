#!/usr/bin/env python3
"""One-shot mechanical conversion NormFieldMeta -> FieldSpec for the artifact field-meta tables. Usage: python3 -I wp02-convert-field-meta.py FILE... (idempotent; reports literals it cannot convert)"""
import re, sys, pathlib

RUST = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🦀️.rs").read_text()
SYMBOLS = {}
for m in re.finditer(r'^\s+(\w+): "([^"]*)", "([^"]*)", \w+,', RUST, re.M):
    SYMBOLS[m.group(2)] = m.group(1)
    if m.group(3): SYMBOLS[m.group(3)] = m.group(1)

def match_brace(text, start):
    depth, i, in_str = 0, start, False
    while i < len(text):
        c = text[i]
        if in_str:
            if c == "\\": i += 1
            elif c == '"': in_str = False
        elif c == '"': in_str = True
        elif c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return i
        i += 1
    raise ValueError("unbalanced")

def split_top(text):
    parts, depth, cur, in_str, i = [], 0, "", False, 0
    while i < len(text):
        c = text[i]
        if in_str:
            cur += c
            if c == "\\": i += 1; cur += text[i]
            elif c == '"': in_str = False
        else:
            if c == '"': in_str = True
            if c in "([{": depth += 1
            elif c in ")]}": depth -= 1
            if c == "," and depth == 0: parts.append(cur.strip()); cur = ""
            else: cur += c
        i += 1
    if cur.strip(): parts.append(cur.strip())
    return parts

def convert(text, path):
    out, pos, left, used_unit = [], 0, 0, False
    for m in re.finditer(r"NormFieldMeta\s*\{", text):
        if m.start() < pos: continue
        end = match_brace(text, m.end() - 1)
        fields = {}
        for part in split_top(text[m.end():end]):
            k, _, v = part.partition(":")
            fields[k.strip()] = v.strip()
        ok = set(fields) == {"label_en", "label_de", "unit", "choices"} and all(fields[k].startswith('"') for k in ("label_en", "label_de"))
        unit, choices = fields.get("unit", ""), fields.get("choices", "")
        um = re.fullmatch(r'Some\("([^"]*)"\)', unit)
        cm = re.fullmatch(r"Some\((.*)\)", choices, re.S)
        if ok and (unit == "None" or um) and (choices == "None" or cm) and not (um and cm) and (not um or um.group(1) in SYMBOLS):
            en, de = fields["label_en"], fields["label_de"]
            if um: new, used_unit = f"FieldSpec::quantity({en}, {de}, UnitId::{SYMBOLS[um.group(1)]})", True
            elif cm: new = f"FieldSpec::choice({en}, {de}, {cm.group(1)})"
            else: new = f"FieldSpec::new({en}, {de})"
            out.append(text[pos:m.start()]); out.append(new); pos = end + 1
        else:
            left += 1
            print(f"MANUAL {path}: {text[m.start():min(end+1, m.start()+160)]!r}")
    out.append(text[pos:])
    text = "".join(out)
    text = re.sub(r"\bNormFieldMeta\b(?!Fn)", "FieldSpec", text)
    if used_unit and "UnitId" not in re.search(r"use semio_s_plugin_norm_app_surface::\{[^}]*\};", text).group(0):
        text = re.sub(r"(use semio_s_plugin_norm_app_surface::\{)([^}]*)\};", lambda x: f"{x.group(1)}{x.group(2)}, UnitId}};", text, count=1)
    return text, left

for f in sys.argv[1:]:
    p = pathlib.Path(f)
    text = p.read_text()
    new, left = convert(text, p.parent.parent.parent.parent.parent.parent.name if False else f.split("/")[-7] if False else p.name)
    if new != text: p.write_text(new)
    print("converted", f.split("🗿️artifacts/")[1].split("/")[0], "manual:", left)
