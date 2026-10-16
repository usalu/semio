"""🔐️ Definition scan for the relocated code: every method, associated function and field the moved modules reach must be
defined with a visibility that reaches them. Usage: privacy_scan.py DEFS_LIST_FILE USER_FILE...

DEFS_LIST_FILE lists the crate's source files (one per line, e.g. from rustc dep-info). Each user file is scanned for
`.name(`, `Type::name(`, `.name` field reads and `name:` struct-literal fields; every definition of that name in the defs
files is reported with its visibility, and a name whose every definition is narrower than `pub(crate)` (and not
defined in the user file itself) is flagged.
"""
import re, sys, pathlib
defs_files = [l.strip() for l in open(sys.argv[1]) if l.strip()]
users = sys.argv[2:]
KEYWORDS = {"iter", "into_iter", "map", "collect", "clone", "unwrap", "expect", "is_some", "is_none", "contains", "then_some", "filter", "find", "any", "all", "push", "insert", "remove", "get", "as_str", "to_string", "into", "len", "is_empty", "chain", "copied", "flat_map", "find_map", "extend", "with_capacity", "abs", "powi", "ln", "exp", "max", "clamp", "windows", "first", "last", "retain", "as_ref", "cloned", "reverse", "from_str", "parse", "map_err", "ok_or_else", "and_then", "unwrap_or_else", "eq", "fmt", "new", "default"}
def strip(text):
    text = re.sub(r"//[^\n]*", "", text)
    text = re.sub(r'"(?:\\.|[^"\\])*"', '""', text)
    return text
fn_def = re.compile(r"(?m)^\s*((?:pub(?:\([^)]*\))?\s+)?)(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)")
field_def = re.compile(r"(?m)^\s*((?:pub(?:\([^)]*\))?\s+)?)(\w+)\s*:\s*[^:=,;]")
defs = {}
for f in defs_files:
    try:
        text = strip(pathlib.Path(f).read_text())
    except Exception:
        continue
    for m in fn_def.finditer(text):
        defs.setdefault(m.group(2), []).append(("fn", m.group(1).strip() or "private", f))
    for m in re.finditer(r"(?s)\bstruct\s+\w+[^{;(]*\{(.*?)\n\}", text):
        for fm in field_def.finditer(m.group(1)):
            defs.setdefault(fm.group(2), []).append(("field", fm.group(1).strip() or "private", f))
for u in users:
    text = strip(pathlib.Path(u).read_text())
    own = {m.group(2) for m in fn_def.finditer(text)}
    names = set(re.findall(r"\.(\w+)\s*\(", text)) | set(re.findall(r"\b[A-Z]\w*::(\w+)\s*\(", text)) | set(re.findall(r"\.([a-z_]\w*)\b(?!\s*\()", text)) | set(re.findall(r"(?m)[{,]\s*([a-z_]\w*)\s*:", text))
    for n in sorted(names - KEYWORDS - own):
        d = defs.get(n)
        if not d:
            continue
        vis = {v for _, v, _ in d}
        ok = any(v == "pub" or v.startswith("pub(crate)") for v in vis)
        if not ok:
            print(f"FLAG {u.split('/')[-3:]}: {n} -> {sorted({(k, v, f.split('🔨️modules/')[-1][:70]) for k, v, f in d})}")
