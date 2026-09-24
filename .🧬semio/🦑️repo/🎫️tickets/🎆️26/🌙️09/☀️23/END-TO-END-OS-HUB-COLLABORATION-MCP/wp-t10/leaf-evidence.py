"""🔎️ Derives every mutation leaf's protocol outcome classes (applied / no-op / rejected) from its own code plus the
MutationOutcome-returning helpers it calls (resolved transitively within the same plugin), and flags the leaves that
need hand review. Writes wp-t10/generated/leaf-evidence.json."""
import json, os, re, collections
root = "/Users/ueli/Documents/semio/"
SKIP = {"node_modules", "target", "dist", "🗑️generated", ".git", "🧪️tests", "🧫️fixtures"}
LEAF_SKIP = SKIP | {"↩️inverse"}
NEW = re.compile(r"MutationOutcome(?:::<[^>]*>)?::new\s*\(")
EMPTY = re.compile(r"MutationOutcome(?:::<[^>]*>)?::empty\s*\(\s*\)")
NOOP_CODE = re.compile(r"\"mutation\.no-op\"|\"mutation\.noop\"|no_op\s*\(")
REJ = re.compile(r"MutationOutcome(?:::<[^>]*>)?::(?:error|fatal)\s*\(|MutationMessage::(?:error|fatal)\s*\(|Severity::(?:Error|Fatal)\b|PlanError::")
DEFAULT_DIFF = re.compile(r"MutationOutcome(?:::<[^>]*>)?::new\s*\(\s*(?:[A-Za-z0-9_:]+::default\(\)|Default::default\(\)|[^;{}]*?unwrap_or_default\(\))\s*\)")
FN = re.compile(r"\bfn\s+([a-z_][a-z0-9_]*)\s*(?:<[^{;]*?>)?\s*\(([^{;]*?)\)\s*->\s*([^{;]*?)\{", re.S)
CALL = re.compile(r"\b([a-z_][a-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
def strip(t): return re.sub(r"//[^\n]*", "", t)
def body(t, start):
    depth, i = 0, start
    while i < len(t):
        c = t[i]
        if c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return t[start:i + 1]
        i += 1
    return t[start:]
def direct(code):
    ev = set()
    if NEW.search(code): ev.add("applied")
    if EMPTY.search(code): ev.add("no-op")
    if NOOP_CODE.search(code): ev.add("no-op")
    if DEFAULT_DIFF.search(code): ev.add("no-op")
    if re.search(r"(?:Diff|Delta)::between\s*\(|\bdiff_between\s*\(|value_diff_between\s*\(", code): ev.add("no-op")
    if REJ.search(code): ev.add("rejected")
    return ev
def plugin_of(rel):
    """🧭️ The helper scope: one artifact crate (`🗿️artifacts/<artifact>`) where there is one, else the plugin."""
    parts = rel.split("/")
    if "🗿️artifacts" in parts: return "/".join(parts[:parts.index("🗿️artifacts") + 2])
    return "/".join(parts[:3]) if parts[0] == "✏️s" else "/".join(parts[:4])
helpers = collections.defaultdict(lambda: collections.defaultdict(list))
for base in ("✏️s", "🧰️framework"):
    for dp, ds, fs in os.walk(root + base):
        ds[:] = [d for d in ds if d not in SKIP]
        for f in fs:
            if not f.endswith(".rs"): continue
            p = os.path.join(dp, f)
            try: t = strip(open(p, encoding="utf-8").read())
            except UnicodeDecodeError: continue
            if "MutationOutcome" not in t and "PlanError" not in t: continue
            aliases = {"MutationOutcome", "PlanError"} | set(re.findall(r"\btype\s+([A-Z][A-Za-z0-9_]*)\s*(?:<[^=]*>)?\s*=\s*[^;]*MutationOutcome", t))
            for m in FN.finditer(t):
                if not any(a in m.group(3) for a in aliases): continue
                b = body(t, m.end() - 1)
                helpers[plugin_of(p[len(root):])][m.group(1)].append(b)
ARM = re.compile(r"\n\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Za-z0-9_]*Mutation::([A-Z][A-Za-z0-9_]*)\s*[({]")
def arm_of(body_text, variant):
    """🎯️ The part of a dispatcher helper that serves one aggregate variant: the text before its first arm plus that arm."""
    arms = list(ARM.finditer(body_text))
    if len(arms) < 2 or variant is None: return body_text
    mine = [i for i, m in enumerate(arms) if m.group(1) == variant]
    if not mine: return body_text
    i = mine[0]
    end = arms[i + 1].start() if i + 1 < len(arms) else len(body_text)
    return body_text[:arms[0].start()] + body_text[arms[i].start():end]
def resolve(plugin, code, seen, variant=None):
    ev = direct(code); via = []
    for name in set(CALL.findall(code)):
        if name in seen or name in ("diff", "new", "empty", "error", "fatal", "apply", "plan", "some", "ok", "err") or name not in helpers[plugin]: continue
        seen.add(name); via.append(name)
        for b in helpers[plugin][name]:
            e, v = resolve(plugin, arm_of(b, variant), seen, variant); ev |= e; via += v
    return ev, via
rows = {}
for base in ("✏️s", "🧰️framework"):
    for dirpath, dirs, files in os.walk(root + base):
        dirs[:] = [d for d in dirs if d not in {"node_modules", "target", "dist", "🗑️generated", ".git"}]
        if "🔣️.json" not in files or "🧬️mutations" not in dirpath.split("/")[:-1]: continue
        try: d = json.load(open(os.path.join(dirpath, "🔣️.json"), encoding="utf-8"))
        except (ValueError, UnicodeDecodeError): continue
        if not (isinstance(d, dict) and "semanticKind" in d and isinstance(d.get("outcomeClasses"), list)): continue
        text = ""
        for dp, ds, fs in os.walk(dirpath):
            ds[:] = [x for x in ds if x not in LEAF_SKIP]
            for f in fs:
                if f.endswith(".rs"): text += open(os.path.join(dp, f), encoding="utf-8").read() + "\n"
        code = strip(text)
        rel = dirpath[len(root):]
        ev, via = resolve(plugin_of(rel), code, set(), d.get("aggregateVariant"))
        composite = "CompositeMutationKind" in code or d.get("composition") == "composite"
        if composite: ev |= {"applied", "rejected"} if "PlanError" in code or "planner.call" in code else {"applied"}
        macro = re.search(r"\b([a-z_]+_impl)!\s*\(", code)
        rows[rel] = {"kind": d.get("semanticKind"), "declared": d["outcomeClasses"], "evidence": sorted(ev), "via": sorted(set(via)), "composite": composite, "macro": macro.group(1) if macro else None}
json.dump(rows, open(root + ".tmp-ticket/wp-t10/generated/leaf-evidence.json", "w"), ensure_ascii=False, indent=1)
c = collections.Counter(tuple(r["evidence"]) for r in rows.values())
for k, v in c.most_common(): print(v, k)
print("no evidence (non-stdio):", sum(1 for p, r in rows.items() if not r["evidence"] and "🗄️stdio" not in p))
print("macro leaves:", collections.Counter(r["macro"] for r in rows.values() if r["macro"]))
