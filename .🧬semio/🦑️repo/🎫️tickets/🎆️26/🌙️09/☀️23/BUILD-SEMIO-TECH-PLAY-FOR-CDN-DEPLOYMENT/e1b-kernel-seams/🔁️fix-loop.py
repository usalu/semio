#!/usr/bin/env python3
"""🔁️ Compile-driven import and visibility fixer for wave E1b (adapted from `e1-os-kernel/🔁️fix-loop.py`).

Runs `cargo check -p semio-framework-os-kernel --lib --message-format=json [CARGO_ARGS]` through `🚦️cargo-slot.sh` with
the private E1b dirs and applies only what rustc names, only inside the `--files` set:
- unresolved name: inserts the `use` rustc suggests, preferring the owning-module spelling (`super::<module>::X`) over a
  root-level path;
- private item, method or field (E0603, E0624, E0616, E0451, E0423) with `--widen`: the definition becomes `pub(crate)`, and so
  does a type named by a more visible signature (`private_interfaces`, `private_bounds`).
Repeats until clean or a round fixes nothing. Usage: fix-loop.py --files F.. [--widen] [--rounds N] [-- CARGO_ARGS]
"""
import json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
E1B = f"{REPO}/.🧬semio/🦑️repo/⚡️cache/e1b"
SLOT = os.path.join(os.path.dirname(HERE), "🚦️cargo-slot.sh")
RESOLVE = {"E0412", "E0425", "E0433", "E0405", "E0422", "E0423", "E0531", "E0532", "E0574", "E0408", "E0407", "E0599"}
ROOT_DIR = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"


def check(extra):
    env = dict(os.environ, CARGO_TARGET_DIR=f"{E1B}/t", CARGO_BUILD_BUILD_DIR=f"{E1B}/b", CARGO_TERM_COLOR="never")
    cmd = [SLOT, "e1b-kernel", "--", "cargo", "check", "-p", "semio-framework-os-kernel", "--lib", "--message-format=json"] + extra
    out = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True).stdout
    msgs, ok = [], False
    for line in out.split("\n"):
        try: j = json.loads(line)
        except ValueError: continue
        if j.get("reason") == "compiler-message" and j["target"]["name"] == "semio_framework_os_kernel": msgs.append(j["message"])
        if j.get("reason") == "build-finished": ok = j.get("success", False)
    if not ok and not any(m["level"] == "error" for m in msgs):
        msgs.append({"level": "error", "message": "cargo failed outside semio-framework-os-kernel", "spans": [], "children": [], "code": None})
    return msgs


def absf(f):
    return os.path.normpath(f if os.path.isabs(f) else os.path.join(REPO, f))


def widen_line(f, line_no):
    lines = open(f, encoding="utf-8").read().split("\n")
    l = lines[line_no - 1]
    m = re.match(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?)((?:default\s+|const\s+|async\s+|unsafe\s+|extern\s+\"[^\"]*\"\s+)*)(fn|struct|enum|trait|type|const|static|mod|union)\b", l)
    if not m or m.group(2).strip() in ("pub", "pub(crate)"): return False
    lines[line_no - 1] = m.group(1) + "pub(crate) " + m.group(3) + l[m.end(2) + len(m.group(3)):]
    open(f, "w", encoding="utf-8").write("\n".join(lines))
    return True


def widen_at(f, line_no, column):
    lines = open(f, encoding="utf-8").read().split("\n")
    l = lines[line_no - 1]
    i = len(l.encode("utf-8")[:column - 1].decode("utf-8", "ignore"))
    if l[i:].startswith("pub"): return False
    lines[line_no - 1] = l[:i] + "pub(crate) " + l[i:]
    open(f, "w", encoding="utf-8").write("\n".join(lines))
    return True


def widen_field(files, struct, field):
    for f in files:
        src = open(f, encoding="utf-8").read()
        for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + re.escape(struct) + r"\b", src, re.M):
            ob = src.find("{", m.end()); semi = src.find(";", m.end()); paren = src.find("(", m.end())
            if field.isdigit():
                if paren < 0 or (0 <= ob < paren) or (0 <= semi < paren): continue
                depth, j, start, fields = 0, paren, paren + 1, []
                while j < len(src):
                    ch = src[j]
                    if ch in "([{<": depth += 1
                    elif ch in ")]}>":
                        depth -= 1
                        if depth == 0: fields.append(start); break
                    elif ch == "," and depth == 1: fields.append(start); start = j + 1
                    j += 1
                k = fields[int(field)]
                while src[k].isspace(): k += 1
                if src.startswith("pub", k): return False
                open(f, "w", encoding="utf-8").write(src[:k] + "pub(crate) " + src[k:]); return True
            if ob < 0 or (0 <= semi < ob): continue
            depth, j = 0, ob
            while j < len(src):
                if src[j] == "{": depth += 1
                elif src[j] == "}":
                    depth -= 1
                    if depth == 0: break
                j += 1
            body = src[ob:j]
            fm = re.search(r"([{,]\s*(?:#\[[^\]]*\]\s*)*(?:///[^\n]*\n\s*)*)((?:pub\(super\)\s+)?)(" + re.escape(field) + r")\s*:", body)
            if not fm: continue
            k = ob + fm.start(2)
            open(f, "w", encoding="utf-8").write(src[:k] + "pub(crate) " + src[ob + fm.start(3):]); return True
    return False


def insert_use(f, stmt):
    src = open(f, encoding="utf-8").read()
    if re.search(r"^" + re.escape(stmt) + r"$", src, re.M): return False
    lines = src.split("\n"); i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].startswith("#![")): i += 1
    lines.insert(i, stmt)
    open(f, "w", encoding="utf-8").write("\n".join(lines)); return True


def depth_of(f):
    return 1


def normalize(u, f):
    p = u[4:-1].strip()
    p = re.sub(r"^crate::os_store::component::", "super::", p) if os.path.dirname(f) != ROOT_DIR else re.sub(r"^crate::os_store::component::", "self::", p)
    return f"use {p};"


def rank(u):
    p = u[4:-1]
    segs = p.split("::")
    owning = len(segs) >= 3 and segs[0] in ("super", "self") and segs[1] not in ("super",)
    return (0 if owning else 1 if p.startswith("crate::os_") or p.startswith("semio_framework") else 2, len(p))


def main():
    args = sys.argv[1:sys.argv.index("--")] if "--" in sys.argv else sys.argv[1:]
    extra = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    files, rounds, widen, i = [], 12, False, 0
    while i < len(args):
        if args[i] == "--rounds": rounds = int(args[i + 1]); i += 2; continue
        if args[i] == "--widen": widen = True; i += 1; continue
        if args[i] == "--files":
            i += 1
            while i < len(args) and not args[i].startswith("--"): files.append(absf(args[i])); i += 1
            continue
        i += 1
    editable = set(files)
    test = "--profile" in extra and "test" in extra
    for r in range(rounds):
        msgs = check(extra)
        errors = [m for m in msgs if m["level"] == "error"]
        interfaces = [m for m in msgs if m["level"] == "warning" and (m.get("code") or {}).get("code") in ("private_interfaces", "private_bounds") and any(absf(s["file_name"]) in editable for c in m.get("children", []) if c["message"].startswith("but ") for s in c.get("spans", []))] if widen else []
        print(f"[fix-loop] round {r}: {len(errors)} errors, {len(interfaces)} private-interface warnings", flush=True)
        if not errors and not interfaces: return 0
        fixed, unresolved, wid, fields, uses, spans_to_widen = 0, [], set(), set(), {}, set()
        if widen:
            for m in msgs:
                code = (m.get("code") or {}).get("code")
                if m["level"] == "warning" and code in ("private_interfaces", "private_bounds"):
                    for s in [s for c in m.get("children", []) if c["message"].startswith("but ") for s in c.get("spans", [])]:
                        if absf(s["file_name"]) in editable: wid.add((absf(s["file_name"]), s["line_start"]))
        for m in errors:
            code = (m.get("code") or {}).get("code")
            prim = [s for s in m["spans"] if s["is_primary"]] or m["spans"]
            site = absf(prim[0]["file_name"]) if prim else None
            if widen and code in ("E0603", "E0624"):
                for s in [s for s in m["spans"] if not s["is_primary"]] + [s for c in m.get("children", []) for s in c.get("spans", []) if not s.get("suggested_replacement")]:
                    if absf(s["file_name"]) in editable: wid.add((absf(s["file_name"]), s["line_start"]))
                continue
            if widen and code == "E0423" and "private fields" in m["message"]:
                for s in m["spans"] + [s for c in m.get("children", []) for s in c.get("spans", [])]:
                    if "private field" in (s.get("label") or "") and absf(s["file_name"]) in editable: spans_to_widen.add((absf(s["file_name"]), s["line_start"], s["column_start"]))
                continue
            if widen and code in ("E0616", "E0451"):
                mm = re.match(r"fields? (.*) of (?:struct|variant) `(?:[\w:]+::)?(\w+)", m["message"])
                if mm:
                    for fd in re.findall(r"`(\w+)`", mm.group(1)): fields.add((mm.group(2), fd))
                continue
            if (code in RESOLVE or (code is None and m["message"].startswith("cannot find"))) and site in editable:
                sugg = [s["suggested_replacement"].strip() for c in m.get("children", []) for s in c.get("spans", []) if (s.get("suggested_replacement") or "").strip().startswith("use ")]
                if sugg:
                    best = min((normalize(u, site) for u in sugg), key=rank)
                    uses.setdefault(site, set()).add(best); continue
            unresolved.append(f"{code} {m['message']} @ {prim[0]['file_name'] if prim else '?'}:{prim[0]['line_start'] if prim else '?'}")
        for f, l in sorted(wid): fixed += widen_line(f, l)
        for f, l, c in sorted(spans_to_widen, reverse=True): fixed += widen_at(f, l, c)
        for st, fd in sorted(fields): fixed += widen_field(sorted(editable), st, fd)
        for f, us in uses.items():
            for u in sorted(us): fixed += insert_use(f, ("#[cfg(test)] " if test else "") + u)
        print(f"[fix-loop]   fixed {fixed}; unresolved {len(unresolved)}", flush=True)
        for u in unresolved[:60]: print("     ", u[:300])
        if not fixed: return 1
    return 1


if __name__ == "__main__":
    sys.exit(main())
