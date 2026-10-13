#!/usr/bin/env python3
"""🔁️ Compile-driven visibility and import fixer for E1 store moves (`🚚️move-items.py`).

Runs `cargo check -p semio-framework-os-kernel --lib` (JSON, private E1 dirs, through `🚦️cargo-slot.sh`) and applies
only the fixes rustc names, and only inside the store root and the files listed with `--files`:
- private item or method (E0603, E0624): the `defined here` item becomes `pub(crate)`;
- private field (E0616, E0451): that field becomes `pub(crate)`;
- unresolved name: the `use` rustc suggests is inserted after the file's import preamble, preferring `super::`/`crate::`.
Repeats until the check is clean or a round fixes nothing. Usage: fix-loop.py --files F [F ...] [--rounds N] [-- CARGO_ARGS]
"""
import json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
E1 = f"{REPO}/.🧬semio/🦑️repo/⚡️cache/e1/os-kernel"
STORE_ROOT = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
SLOT = os.path.join(os.path.dirname(HERE), "🚦️cargo-slot.sh")
RESOLVE = {"E0412", "E0425", "E0433", "E0405", "E0422", "E0423", "E0531", "E0532", "E0574", "E0408", "E0407", "E0599"}


def check(extra):
    env = dict(os.environ, CARGO_TARGET_DIR=f"{E1}/t", CARGO_BUILD_BUILD_DIR=f"{E1}/b", CARGO_INCREMENTAL="0", CARGO_TERM_COLOR="never")
    cmd = [SLOT, "e1-os-kernel", "--", "cargo", "check", "-p", "semio-framework-os-kernel", "--lib", "--message-format=json"] + extra
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


def widen_tuple_field(src, open_paren, index):
    depth, j, start, fields = 0, open_paren, open_paren + 1, []
    while j < len(src):
        ch = src[j]
        if ch in "([{<": depth += 1
        elif ch in ")]}>":
            depth -= 1
            if depth == 0: fields.append(start); break
        elif ch == "," and depth == 1: fields.append(start); start = j + 1
        j += 1
    if index >= len(fields): return None
    k = fields[index]
    while src[k].isspace(): k += 1
    if src.startswith("pub", k): return None
    return src[:k] + "pub(crate) " + src[k:]


def widen_field(files, struct, field):
    for f in files:
        src = open(f, encoding="utf-8").read()
        if field.isdigit():
            for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + re.escape(struct) + r"\b[^({;]*\(", src, re.M):
                out = widen_tuple_field(src, m.end() - 1, int(field))
                if out:
                    open(f, "w", encoding="utf-8").write(out)
                    return True
            continue
        for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + re.escape(struct) + r"\b", src, re.M):
            ob = src.find("{", m.end())
            semi = src.find(";", m.end())
            if ob < 0 or (0 <= semi < ob): continue
            depth, j = 0, ob
            while j < len(src):
                if src[j] == "{": depth += 1
                elif src[j] == "}":
                    depth -= 1
                    if depth == 0: break
                j += 1
            body = src[ob:j]
            fm = re.search(r"([{,]\s*(?:#\[[^\]]*\]\s*)*)((?:pub\(super\)\s+)?)(" + re.escape(field) + r")\s*:", body)
            if not fm: continue
            k = ob + fm.start(2)
            open(f, "w", encoding="utf-8").write(src[:k] + "pub(crate) " + src[ob + fm.start(3):])
            return True
    return False


def insert_use(f, stmt):
    src = open(f, encoding="utf-8").read()
    if re.search(r"^" + re.escape(stmt) + r"$", src, re.M): return False
    lines = src.split("\n")
    i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].startswith("#![")): i += 1
    lines.insert(i, stmt)
    open(f, "w", encoding="utf-8").write("\n".join(lines))
    return True


def rank(u):
    p = u[4:-1]
    return (0 if p.startswith("super::") else 3 if p.startswith("crate::os_store::component::") else 1 if p.startswith("crate::") else 2, len(p))


def main():
    args = sys.argv[1:sys.argv.index("--")] if "--" in sys.argv else sys.argv[1:]
    extra = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    files, rounds, i = [], 12, 0
    while i < len(args):
        if args[i] == "--rounds": rounds = int(args[i + 1]); i += 2; continue
        if args[i] == "--files":
            i += 1
            while i < len(args) and not args[i].startswith("--"): files.append(absf(args[i])); i += 1
            continue
        i += 1
    editable = set(files) | {STORE_ROOT}
    for r in range(rounds):
        msgs = check(extra)
        errors = [m for m in msgs if m["level"] == "error"]
        print(f"[fix-loop] round {r}: {len(errors)} errors")
        if not errors: return 0
        fixed, unresolved = 0, []
        widen, fields, uses = set(), set(), {}
        for m in errors:
            code = (m.get("code") or {}).get("code")
            prim = [s for s in m["spans"] if s["is_primary"]] or m["spans"]
            site = absf(prim[0]["file_name"]) if prim else None
            if code in ("E0603", "E0624"):
                for s in m["spans"] + [s for c in m.get("children", []) for s in c.get("spans", [])]:
                    if "defined here" in (s.get("label") or "") or ("defined here" in " ".join(c.get("message", "") for c in m.get("children", [])) and not s["is_primary"]):
                        if absf(s["file_name"]) in editable: widen.add((absf(s["file_name"]), s["line_start"]))
                continue
            if code in ("E0616", "E0451"):
                mm = re.match(r"fields? (.*) of struct `(?:[\w:]+::)?(\w+)", m["message"])
                if mm:
                    for fd in re.findall(r"`(\w+)`", mm.group(1)): fields.add((mm.group(2), fd))
                else: unresolved.append(m["message"])
                continue
            if (code in RESOLVE or (code is None and m["message"].startswith("cannot find"))) and site in editable:
                sugg = [s["suggested_replacement"].strip() for c in m.get("children", []) for s in c.get("spans", []) if (s.get("suggested_replacement") or "").strip().startswith("use ")]
                if sugg:
                    uses.setdefault(site, set()).add(min(sugg, key=rank)); continue
            unresolved.append(f"{code} {m['message']} @ {prim[0]['file_name'] if prim else '?'}:{prim[0]['line_start'] if prim else '?'}")
        for f, l in sorted(widen): fixed += widen_line(f, l)
        for st, fd in sorted(fields): fixed += widen_field(sorted(editable), st, fd)
        for f, us in uses.items():
            for u in sorted(us): fixed += insert_use(f, ("#[cfg(test)] " if "--profile" in extra and "test" in extra else "") + u)
        print(f"[fix-loop]   fixed {fixed}; unresolved {len(unresolved)}")
        for u in unresolved[:40]: print("     ", u[:300])
        if not fixed: return 1
    return 1


if __name__ == "__main__":
    sys.exit(main())
