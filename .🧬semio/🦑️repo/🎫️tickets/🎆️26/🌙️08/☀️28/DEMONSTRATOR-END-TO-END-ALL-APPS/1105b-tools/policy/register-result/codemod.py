import re, sys, os, json, subprocess, shutil
ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
OUT = os.path.join(TICKET, "🗑️generated", "1105b", "policy")
STAGE = os.path.join(OUT, "stage")
ERR_PATH = "semio_plugin::io::RegistrationFault"
import re as _re
SIG_RE = _re.compile(r'^(\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?)fn register\(\)\s*\{\s*$')
STUB_RE = _re.compile(r'^(\s*(?:pub(?:\([^)]*\))?\s+)?)fn register\(\)\s*\{\s*\}\s*$')
EXPECT_RE = re.compile(r'\.expect\("static Stdio[^"]*"\)')
CALL_RE = re.compile(r'^(\s*)((?:[A-Za-z_][A-Za-z0-9_]*::)*)register\(\);\s*$')
REGISTER_CALL_NAMES = None

def strip_code(line, in_str):
    out = []; i = 0
    while i < len(line):
        c = line[i]
        if in_str:
            if c == '\\': i += 2; continue
            if c == '"': in_str = False
            i += 1; continue
        if c == '"': in_str = True; i += 1; continue
        if line.startswith('//', i): break
        out.append(c); i += 1
    return ''.join(out), in_str

def find_fn_end(lines, start):
    depth = 0; in_str = False; opened = False
    for idx in range(start, len(lines)):
        code, in_str = strip_code(lines[idx], in_str)
        for ch in code:
            if ch == '{': depth += 1; opened = True
            elif ch == '}':
                depth -= 1
                if opened and depth == 0: return idx
    raise SystemExit(f"FAIL-CLOSED: unbalanced braces from line {start+1}")

def process(rel, report):
    path = os.path.join(ROOT, rel)
    lines = open(path, encoding="utf-8").read().split("\n")
    sig_idx = [i for i, l in enumerate(lines) if SIG_RE.match(l) or STUB_RE.match(l)]
    if not sig_idx:
        report["manual_defs_missing"].append(rel); return None
    ranges = []
    for i in sig_idx:
        if STUB_RE.match(lines[i]):
            ranges.append((i, i, True))
        else:
            ranges.append((i, find_fn_end(lines, i), False))
    edits = {}
    for s, e, oneliner in ranges:
        if oneliner:
            if s in edits: raise SystemExit(f"FAIL-CLOSED: overlap {rel}:{s+1}")
            m = STUB_RE.match(lines[s])
            edits[s] = [f"{m.group(1)}fn register() -> Result<(), {ERR_PATH}> {{ Ok(()) }}"]
            report["defs"] += 1
            continue
        if e == s: raise SystemExit(f"FAIL-CLOSED: one-line register body {rel}:{s+1}")
        report["defs"] += 1
        m = SIG_RE.match(lines[s])
        if not m: raise SystemExit(f"FAIL-CLOSED: signature {rel}:{s+1}")
        edits[s] = [f"{m.group(1)}fn register() -> Result<(), {ERR_PATH}> {{"]
        for k in range(s + 1, e):
            L = lines[k]
            if L.strip() == "" : edits[k] = [L]; continue
            m = EXPECT_RE.search(L)
            if m:
                report["expects"] += 1
                if len(EXPECT_RE.findall(L)) != 1: raise SystemExit(f"FAIL-CLOSED: multi expect {rel}:{k+1}")
                if L.strip().startswith(".expect("):
                    new = L[: m.start()] + "?" + L[m.end():]
                    if L.strip().startswith('.expect(') and new.strip() == "?;" or new.strip() == "?":
                        pass
                else:
                    new = L[: m.start()] + "?" + L[m.end():]
                edits[k] = [new]
                continue
            cm = CALL_RE.match(L)
            if cm:
                report["calls"] += 1
                edits[k] = [f"{cm.group(1)}{cm.group(2)}register()?;"]
                continue
            if re.search(r'\bexpect\(', L) and 'static Stdio' in L:
                raise SystemExit(f"FAIL-CLOSED: unmatched expect {rel}:{k+1}: {L.strip()[:120]}")
            if re.search(r'\bregister\(', L) and not L.lstrip().startswith("//"):
                report["manual_calls"].append(f"{rel}:{k+1}: {L.strip()[:140]}")
        # insert Ok(()) before closing brace line e
        closing = lines[e]
        if closing.strip() != "}": raise SystemExit(f"FAIL-CLOSED: closing line not bare brace {rel}:{e+1}: {closing.strip()[:80]}")
        indent = "    "
        edits[e] = [f"{indent}Ok(())", closing]
    new_lines = []
    for i, l in enumerate(lines):
        if i in edits: new_lines.extend(edits[i])
        else: new_lines.append(l)
    return "\n".join(new_lines)

def main():
    files = []
    for dp, dn, fn in os.walk(os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio")):
        dn[:] = [d for d in dn if d not in ("node_modules", "target")]
        for f in fn:
            if not f.endswith(".rs"): continue
            full = os.path.join(dp, f)
            rel = os.path.relpath(full, ROOT)
            if "🧪️tests" in rel: continue
            src = open(full, encoding="utf-8", errors="replace").read()
            if _re.search(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn register\(\)\s*\{', src):
                files.append(rel)
    files.sort()
    open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "files.txt"), "w", encoding="utf-8").write("\n".join(files) + "\n")
    report_extra = {}
    report = {"defs": 0, "expects": 0, "calls": 0, "manual_defs_missing": [], "manual_calls": [], "files": 0}
    os.makedirs(STAGE, exist_ok=True)
    changed = []
    for rel in files:
        new = process(rel, report)
        if new is None: continue
        dst = os.path.join(STAGE, rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        open(dst, "w", encoding="utf-8").write(new)
        changed.append(rel)
    report["files"] = len(changed)
    listed = set(files)
    boundary = []
    for dp, dn, fn in os.walk(os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio")):
        dn[:] = [d for d in dn if d not in ("node_modules", "target")]
        for f in fn:
            if not f.endswith(".rs"): continue
            full = os.path.join(dp, f); rel = os.path.relpath(full, ROOT)
            if rel in listed or "/mirror/" in rel: continue
            for k, L in enumerate(open(full, encoding="utf-8", errors="replace"), 1):
                if re.search(r'(?<![A-Za-z0-9_])(?:[A-Za-z_][A-Za-z0-9_]*::)*register\(\)|register_subset_validator\(|register_composer_entries\(', L) and not L.lstrip().startswith("//"):
                    boundary.append(f"{rel}:{k}: {L.strip()[:140]}")
    report["boundary_outside_list"] = boundary
    json.dump({"report": report, "changed": changed}, open(os.path.join(OUT, "counts.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps({k: (v if not isinstance(v, list) else len(v)) for k, v in report.items()}, ensure_ascii=False))

if __name__ == "__main__":
    main()
