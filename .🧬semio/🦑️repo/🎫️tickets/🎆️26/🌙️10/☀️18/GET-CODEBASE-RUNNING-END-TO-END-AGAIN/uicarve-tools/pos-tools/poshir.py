"""🌳 POS2 HIR culprit scan. Root cause of the rustc "early stop" (RESULT-PRIVACY-ORACLE-SWEEP-2 section 4): when a trait reference in a bound, supertrait, `dyn`, `impl Trait` or impl header has `Res::Err`
(unresolved, not a trait, or imported through a failed `use`), ty lowering raises a fatal error and no body is type-checked.
`rustc -Zunpretty=hir-tree` prints every HIR node even for crates with resolution errors, including `TraitRef { path: Path { span, res: Err } }` for exactly those references (also the silent ones through failed imports).
This module extracts those spans with an awk filter (the dump is gigabytes for a large union) and rewrites each, in the sandbox only, to a numbered stub trait `crate::__ext_cull::Tr<n>` (appended to the union root).
Every rewrite is reported (`hir_cull` in the result JSON) so a real carve defect is never hidden."""
import os, re, subprocess

AWK = r'''{ l4 = l3; l3 = l2; l2 = l1; l1 = $0 }
/^ *res: Err,$/ && l4 ~ /TraitRef \{$/ && l3 ~ /path: Path \{$/ && l2 ~ /span: / { s = l2; sub(/^ *span: /, "", s); sub(/ \(#[0-9]+\),?$/, "", s); print s }'''

SPAN_RX = re.compile(r"^(.*?):(\d+):(\d+): (\d+):(\d+)$")


def scan(sandbox_dir, rustc_cmd_prefix, cfg_args):
    """Returns [(file, l1, c1, l2, c2)] of all Res::Err trait references (deduplicated)."""
    cmd = ["rustc", *rustc_cmd_prefix, "--edition", "2021", "--crate-type", "lib", "--crate-name", "pos_union", "--cap-lints", "allow", "-Ztrim-diagnostic-paths=no", "-Zunpretty=hir-tree", *cfg_args, "__pos_root.rs"]
    pipeline = " ".join(shell_quote(c) for c in cmd) + " 2>/dev/null | awk " + shell_quote(AWK)
    p = subprocess.run(["sh", "-c", pipeline], capture_output=True, text=True, cwd=sandbox_dir)
    out, seen = [], set()
    for line in p.stdout.split("\n"):
        m = SPAN_RX.match(line.strip())
        if not m:
            continue
        k = (m.group(1), int(m.group(2)), int(m.group(3)), int(m.group(4)), int(m.group(5)))
        if k not in seen:
            seen.add(k)
            out.append(k)
    return out


def shell_quote(s):
    return "'" + s.replace("'", "'\\''") + "'"


PATH_PREFIX = re.compile(r"^(?:::)?[A-Za-z_][A-Za-z0-9_]*(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)*")


def split_args(argtext):
    out, cur, depth = [], "", 0
    for ch in argtext:
        if ch in "<([":
            depth += 1
        elif ch in ">)]":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur)
    return [a.strip() for a in out if a.strip()]


def rewrite_files(sandbox, sandbox_dir, spans, counter, root_path):
    """Rewrites every span in place (sandbox files only). Returns (report rows, new trait count)."""
    by = {}
    for f, l1, c1, l2, c2 in spans:
        by.setdefault(f, []).append((l1, c1, l2, c2))
    rows, defs = [], []
    for f, lst in by.items():
        full = os.path.join(sandbox_dir, f)
        if not f.startswith("__m_") or not os.path.exists(full):
            continue
        tx = open(full, encoding="utf-8").read()
        lines = tx.split("\n")
        edits = []
        for l1, c1, l2, c2 in sorted(lst, reverse=True):
            if l1 < 1 or l1 > len(lines):
                continue
            line = lines[l1 - 1]
            if l1 == l2:
                seg = line[c1 - 1:c2 - 1]
            else:
                seg = line[c1 - 1:]
                m = PATH_PREFIX.match(seg)
                if not m:
                    continue
                seg = m.group(0)
                c2 = c1 + len(seg)
                l2 = l1
            if not re.match(r"^(::)?[A-Za-z_]|^\$crate\b", seg) or "crate::__ext_cull::" in seg:
                continue
            args = []
            ga = re.match(r"^(.*?)(<.*>)$", seg, re.S)
            if ga:
                for a in split_args(ga.group(2)[1:-1]):
                    if a.startswith("'") or re.match(r"^[A-Za-z_]\w*\s*=", a):
                        continue
                    args.append(a)
            args = args[:6]
            counter[0] += 1
            name = f"crate::__ext_cull::Tr{counter[0]}"
            defs.append(counter[0])
            rep = name + ("<" + ", ".join(args) + ">" if args else "")
            edits.append((l1, c1, c2, rep, seg))
        for l1, c1, c2, rep, seg in edits:
            line = lines[l1 - 1]
            lines[l1 - 1] = line[:c1 - 1] + rep + line[c2 - 1:]
            rows.append(dict(file=f, line=l1, path=seg[:120]))
        sandbox.write(full, "\n".join(lines))
    if defs:
        root_tx = open(root_path, encoding="utf-8").read()
        decl = "pub mod __ext_cull {\n" + "\n".join(f"    pub trait Tr{n}<A0 = (), A1 = (), A2 = (), A3 = (), A4 = (), A5 = ()> {{}}" for n in defs) + "\n}\n"
        if "pub mod __ext_cull {" in root_tx:
            root_tx = root_tx.replace("pub mod __ext_cull {\n", "pub mod __ext_cull {\n" + "\n".join(f"    pub trait Tr{n}<A0 = (), A1 = (), A2 = (), A3 = (), A4 = (), A5 = ()> {{}}" for n in defs) + "\n", 1)
        else:
            root_tx += "\n" + decl
        sandbox.write(root_path, root_tx)
    return rows


ICE_STACK_RX = re.compile(r"\[typeck_root\] type-checking `([^`]+)`")
ICE_IMPL_RX = re.compile(r"^(?P<crate>__m_[A-Za-z0-9_]+)::(?P<mods>(?:[A-Za-z0-9_]+::)*)(?:<impl at (?P<file>[^:]+(?::[^:]*)*?):(?P<l1>\d+):(?P<c1>\d+): (?P<l2>\d+):(?P<c2>\d+)>::)?(?P<fn>[A-Za-z0-9_]+)$")


def ice_items(stderr):
    """Bodies named by `query stack during panic` entries `[typeck_root] type-checking ...` of an internal compiler error (rustc panics abort every later body)."""
    if "internal compiler error" not in stderr and "panicked at" not in stderr:
        return []
    i = stderr.find("query stack during panic")
    seg = stderr[i:i + 4000] if i >= 0 else ""
    items = []
    for m in ICE_STACK_RX.finditer(seg):
        mm = ICE_IMPL_RX.match(m.group(1))
        if mm:
            items.append(mm.groupdict())
        else:
            items.append(dict(raw=m.group(1)))
        break
    return items


def _find_fn_body(masked, start, name):
    m = re.compile(r"\bfn\s+" + re.escape(name) + r"\b").search(masked, start)
    if not m:
        return None
    i, depth_p, depth_a = m.end(), 0, 0
    while i < len(masked):
        ch = masked[i]
        if ch in "([":
            depth_p += 1
        elif ch in ")]":
            depth_p -= 1
        elif ch == "<":
            depth_a += 1
        elif ch == ">" and masked[i - 1] != "-":
            depth_a -= 1
        elif ch == ";" and depth_p <= 0:
            return None
        elif ch == "{" and depth_p <= 0 and depth_a <= 0:
            break
        i += 1
    if i >= len(masked):
        return None
    a, depth = i, 0
    while i < len(masked):
        if masked[i] == "{":
            depth += 1
        elif masked[i] == "}":
            depth -= 1
            if depth == 0:
                return a, i + 1
        i += 1
    return None


def neutralise_bodies(sandbox, sandbox_dir, items, mask):
    """Replaces the body of every named fn with `{ loop {} }` (sandbox only). Returns report rows."""
    rows = []
    for it in items:
        if "raw" in it:
            rows.append(dict(file="?", line=0, fn=it["raw"], note="unattributed (not a plain fn path)"))
            continue
        crate = it["crate"]
        cdir = os.path.join(sandbox_dir, crate)
        cands = []
        if it.get("file"):
            full = os.path.join(sandbox_dir, it["file"])
            if os.path.exists(full):
                tx = open(full, encoding="utf-8").read()
                off = sum(len(l) + 1 for l in tx.split("\n")[:int(it["l1"]) - 1])
                cands.append((full, off))
        else:
            for root, _, files in os.walk(cdir):
                for fn in files:
                    if fn.endswith(".rs"):
                        full = os.path.join(root, fn)
                        tx = open(full, encoding="utf-8").read()
                        if re.search(r"\bfn\s+" + re.escape(it["fn"]) + r"\b", tx):
                            cands.append((full, 0))
        if not cands:
            rows.append(dict(file="?", line=0, fn=it["fn"], note="source not found"))
            continue
        for full, off in cands:
            tx = open(full, encoding="utf-8").read()
            mt = mask(tx)
            r = _find_fn_body(mt, off, it["fn"])
            if r is None:
                rows.append(dict(file=os.path.relpath(full, sandbox_dir), line=0, fn=it["fn"], note="no body"))
                continue
            a, b = r
            sandbox.write(full, tx[:a] + "{ loop {} }" + tx[b:])
            rows.append(dict(file=os.path.relpath(full, sandbox_dir), line=tx.count("\n", 0, a) + 1, fn=it["fn"], note="body replaced by loop {}"))
    return rows
