#!/usr/bin/env python3
"""🧮️ sf3 target 2 check: every call of a support function in a consumer passes exactly as many arguments as the support declares.

Read-only. Parses `pub fn name(...)` of the four support domain files (parameters counted at nesting depth 0) and every `alias::[oracle::]name(...)` call of the consumer unit tests and the supports themselves.
"""
import re, sys, glob, os

R = "/Users/ueli/Documents/semio"
G = f"{R}/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry"
IO = f"{R}/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry"
DOM = {
    "O": f"{G}/🧪️tests/🧰️oracle-support/🦀️.rs",
    "C": f"{G}/🧪️tests/🧰️construction-support/🦀️.rs",
    "P": f"{G}/⏱️phased-job/🧪️tests/🧰️support/🦀️.rs",
    "H": f"{G}/🧱️mesh-support/🧪️tests/🧰️mesh-harness/🦀️.rs",
}


def split_args(s):
    depth, cur, out = 0, "", []
    i = 0
    while i < len(s):
        ch = s[i]
        if ch in "([{<" and not (ch == "<" and not re.match(r"[A-Za-z_&']", s[i + 1:i + 2] or " ")):
            depth += 1
        elif ch in ")]}>" and not (ch == ">" and s[i - 1:i] in ("-", "=")):
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur); cur = ""
        else:
            cur += ch
        i += 1
    if cur.strip():
        out.append(cur)
    return out


def sig_arity(text):
    sigs = {}
    for m in re.finditer(r"pub fn (\w+)(?:<[^>]*>)?\(", text):
        start = m.end()
        depth, j = 1, start
        while depth:
            depth += {"(": 1, ")": -1}.get(text[j], 0)
            j += 1
        sigs[m.group(1)] = len(split_args(text[start:j - 1]))
    return sigs


def call_args(text, name_end):
    depth, j = 1, name_end
    while depth:
        depth += {"(": 1, ")": -1}.get(text[j], 0)
        j += 1
    return len(split_args(text[name_end:j - 1]))


sigs = {k: sig_arity(open(v, encoding="utf-8").read()) for k, v in DOM.items()}
for k, v in sigs.items():
    print(k, {n: c for n, c in v.items() if n in ("kind_of", "run_fixture", "assert_category", "refresh", "assert_deterministic", "assert_cancellable", "first_shape", "inputs_of", "shape_of")})

consumers = {}
for f in glob.glob(f"{G}/*/🧪️tests/🔬️unit/🦀️.rs") + glob.glob(f"{G}/*/*/🧪️tests/🔬️unit/🦀️.rs") + glob.glob(f"{IO}/*/🧪️tests/🔬️unit/🦀️.rs"):
    t = open(f, encoding="utf-8").read()
    for m in re.finditer(r"use (semio_s_artifact_procedural_3d_geometry_(oracle_support|construction_support|phased_job_support|mesh_harness)) as (\w+);", t):
        consumers.setdefault(f, []).append(({"oracle_support": "O", "construction_support": "C", "phased_job_support": "P", "mesh_harness": "H"}[m.group(2)], m.group(3)))
bad = 0
total = 0
for f, mounts in consumers.items():
    t = open(f, encoding="utf-8").read()
    for key, alias in mounts:
        for m in re.finditer(r"(?<![\w])" + alias + r"::(?:oracle::|base::)?(\w+)\(", t):
            name = m.group(1)
            via_oracle = "::oracle::" in m.group(0) or "::base::" in m.group(0)
            table = sigs["O"] if via_oracle else sigs[key]
            if name not in table and not via_oracle:
                table = sigs.get("O", {}) if key in ("C", "P", "H") and name in sigs["O"] else table
            if name not in table:
                continue
            n = call_args(t, m.end())
            total += 1
            if n != table[name]:
                bad += 1
                print("ARITY", f.split("📐️geometry/")[-1][:60], m.group(0), "calls with", n, "declared", table[name])
print("checked calls", total, "mismatches", bad)
for key, path in DOM.items():
    t = open(path, encoding="utf-8").read()
    for m in re.finditer(r"(?<![\w.])(kind_of|inputs_of|shape_of|run_fixture|assert_category)\(", t):
        decl = t[max(0, m.start() - 7):m.start()]
        if decl.endswith("fn ") or decl.endswith("pub fn "):
            continue
        name = m.group(1)
        n = call_args(t, m.end())
        want = sigs[key].get(name, sigs["O"].get(name))
        if want is not None and n != want and not decl.endswith("::"):
            print("INTERNAL ARITY", key, name, n, want, t[m.start() - 30:m.end() + 60].replace("\n", " "))
