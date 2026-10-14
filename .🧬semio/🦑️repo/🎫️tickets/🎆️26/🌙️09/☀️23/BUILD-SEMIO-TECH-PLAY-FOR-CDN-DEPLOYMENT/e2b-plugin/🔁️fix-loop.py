#!/usr/bin/env python3
"""🔁️ Compile-driven import and visibility fixer for the E2 plugin decomposition (E2b copy: e2b cache, e2b facet table, E1 splitter).

Runs one `cargo check` of `semio-framework-plugin` (through `🏗️cargo.sh`, JSON diagnostics), then applies the
mechanical fixes the compiler names:

- unresolved names (E0412/E0425/E0433/E0405/E0422/E0423/E0531/E0532/E0574, missing macros, traits not in scope):
  inserts the `use crate::…` import rustc suggests, preferring the owning module over the `app` vocabulary;
- private items (E0603/E0624/E0616): widens exactly the named item, method or field to `pub(crate)`;
- ambiguous glob names (E0659): imports the crate-local owner explicitly.

An import lands in the decomposed module that textually owns the error site (the file itself, the module that
`include!`s it, or the parent whose `use super::*` it reads), gated with `#[cfg(test)]` when the run checks the
test configuration. Usage: fix-loop.py <tag> [--test] [--target T] [--features F] [--apply]   (E2B_SNAP=1: check the live plugin inside the
private source snapshot `⚡️cache/e2b/snap`, whose broken peer crates carry private local fixes; diagnostics map back to
the live tree because both share one relative layout)
"""
import json, os, re, subprocess, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
PLUGIN = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
E2 = f"{REPO}/.🧬semio/🦑️repo/⚡️cache/e2b"
DEST = json.load(open(os.path.join(HERE, "🗂️destinations.json"), encoding="utf-8"))
DEST_FILES = {os.path.join(PLUGIN, d["dir"], "🦀️.rs"): d["module"] for d in DEST["destinations"]}
DEST_FILES.update({os.path.join(PLUGIN, f["dir"], "🦀️.rs"): f["module"] for f in DEST["files"]})
PRUNE_ALL = "--prune-all" in sys.argv
DEST_FILES_VERBATIM = {os.path.join(PLUGIN, f["dir"], "🦀️.rs") for f in DEST["files"]}
MODULES = set(DEST_FILES.values())
RESOLVE = {"E0412", "E0425", "E0433", "E0405", "E0422", "E0423", "E0531", "E0532", "E0574", "E0408", "E0599", "E0407"}


def run(tag, args):
    log = f"{E2}/logs/{tag}.json"
    os.makedirs(f"{E2}/logs", exist_ok=True)
    env = dict(os.environ, CARGO_TARGET_DIR=f"{E2}/target", CARGO_TERM_COLOR="never")
    cwd = REPO
    if os.environ.get("E2B_SNAP"):
        snap = f"{E2}/snap"
        subprocess.run(["rsync", "-a", "--delete", "--max-size=8m", "--exclude", "node_modules", "--exclude", "target", "--exclude", "🗑️generated", f"{PLUGIN}/", f"{snap}/{os.path.relpath(PLUGIN, REPO)}/"], check=True)
        env.update(CARGO_BUILD_BUILD_DIR=f"{E2}/snapbuild")
        cwd = snap
    cmd = [os.path.join(os.path.dirname(HERE), "🚦️cargo-slot.sh"), f"e2b-{tag}", "--", "cargo", "check", "-p", "semio-framework-plugin", "--lib", "--message-format=json"] + args
    with open(log, "w") as out:
        subprocess.run(cmd, cwd=cwd, env=env, stdout=out, stderr=subprocess.DEVNULL)
    msgs, foreign = [], set()
    for line in open(log):
        try:
            j = json.loads(line)
        except ValueError:
            continue
        if j.get("reason") == "compiler-message" and j["target"]["name"] == "semio_framework_plugin":
            msgs.append(j["message"])
        elif j.get("reason") == "compiler-message" and j["message"]["level"] == "error":
            foreign.add(j["target"]["name"])
    if foreign: print(f"[fix-loop] {tag}: DEPENDENCY BROKEN, plugin not checked: {sorted(foreign)}")
    elif '"reason":"build-finished"' not in open(log).read(): print(f"[fix-loop] {tag}: DEPENDENCY BROKEN, cargo stopped before building (manifest or resolver)")
    return msgs


def abspath(f):
    return os.path.normpath(f if os.path.isabs(f) else os.path.join(REPO, f))


includers = {}
parents = {}


def index_tree():
    for f in list(DEST_FILES) + [os.path.join(PLUGIN, "🦀️.rs")]:
        if not os.path.exists(f): continue
        src = open(f, encoding="utf-8").read()
        d = os.path.dirname(f)
        for m in re.finditer(r'include!\(\s*"([^"]+)"\s*\)', src):
            includers[os.path.normpath(os.path.join(d, m.group(1)))] = f
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+;', src):
            parents[os.path.normpath(os.path.join(d, m.group(1)))] = f


def owner_file(f):
    seen = set()
    while f not in DEST_FILES and f not in seen:
        seen.add(f)
        if f in includers: f = includers[f]; continue
        if f in parents:
            src = open(f, encoding="utf-8").read()
            if re.search(r"^\s*use\s+super::\*\s*;", src, re.M): f = parents[f]; continue
        break
    return f


def suggestions(msg):
    out = []
    for c in msg.get("children", []):
        for s in c.get("spans", []):
            r = s.get("suggested_replacement")
            if r and r.strip().startswith("use "):
                out.append(r.strip())
    return out


def score(u):
    p = re.match(r"use\s+([^;]+);", u).group(1)
    if p.startswith("crate::app::") or "plugin_app_close_prelude" in p: return (3, len(p))
    if p.startswith("crate::component::app::"): return (3, len(p))
    if p.startswith("crate::") and p.split("::")[1] in MODULES: return (0, len(p))
    if p.startswith("crate::"): return (1, len(p))
    return (2, len(p))


def insert_import(f, line, gate):
    src = open(f, encoding="utf-8").read()
    stmt = (f"#[cfg({gate})] " if gate else "") + line
    lines = src.split("\n")
    if any(l.strip() == stmt for l in lines): return False
    narrower = f"#[cfg(test)] {line}"
    if gate and gate != "test" and narrower in lines:
        lines[lines.index(narrower)] = stmt
        open(f, "w", encoding="utf-8").write("\n".join(lines))
        return True
    i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].startswith("#![")): i += 1
    lines.insert(i, stmt)
    open(f, "w", encoding="utf-8").write("\n".join(lines))
    return True


def widen_at(f, line_no, name=None):
    lines = open(f, encoding="utf-8").read().split("\n")
    i = line_no - 1
    l = lines[i]
    m = re.match(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?)((?:default\s+|const\s+|async\s+|unsafe\s+|extern\s+\"[^\"]*\"\s+)*)(fn|struct|enum|trait|type|const|static|mod|union|macro_rules!)\b", l)
    if not m:
        return False
    if m.group(2).strip() in ("pub", "pub(crate)"): return False
    lines[i] = m.group(1) + "pub(crate) " + m.group(3) + l[m.end(2) + len(m.group(3)):]
    open(f, "w", encoding="utf-8").write("\n".join(lines))
    return True


_regions = None


def regions():
    """🗺️ (module path, file, span) of every module of the crate tree, `include!`d files under their includer's path."""
    global _regions
    if _regions is None:
        import importlib.util
        spec = importlib.util.spec_from_file_location("facet_imports", os.path.join(HERE, "🧭️facet-imports.py"))
        fi = importlib.util.module_from_spec(spec); spec.loader.exec_module(fi)
        fi.walk(f"{PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{PLUGIN}/📦️packages/🦀️rust")
        _regions = [("::".join(x for x in m.path.split("::")[1:] if x != "component"), m.file, m.rng) for m in fi.modules]
    return _regions


def find_struct(qualified):
    path, _, name = qualified.rpartition("::")
    path = "::".join(x for x in path.split("::") if x and x not in ("component", "crate"))
    pat = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + re.escape(name) + r"\b", re.M)
    hits = []
    for mpath, f, (a, b) in regions():
        if path and mpath != path: continue
        src = open(f, encoding="utf-8").read()
        for m in pat.finditer(src, a, b):
            if (f, m.start()) not in hits: hits.append((f, m.start()))
    if len(hits) == 1: return hits[0]
    return None, None


def find_struct_by_name(name):
    pat = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + re.escape(name) + r"\b", re.M)
    import glob
    cands = list(DEST_FILES) + [os.path.join(PLUGIN, "🦀️.rs")] + [g for g in glob.glob(PLUGIN + "/**/🦀️.rs", recursive=True) if "/🧪️tests/" not in g and "/📦️packages/" not in g]
    for f in cands:
        if not os.path.exists(f): continue
        src = open(f, encoding="utf-8").read()
        m = pat.search(src)
        if m: return f, m.start()
    return None, None


def widen_field(struct, field):
    f, pos = find_struct(struct)
    if not f: return False
    src = open(f, encoding="utf-8").read()
    ob = src.index("{", pos)
    depth, j = 0, ob
    while j < len(src):
        if src[j] == "{": depth += 1
        elif src[j] == "}":
            depth -= 1
            if depth == 0: break
        j += 1
    body = src[ob:j]
    m = re.search(r"[{,\n](\s*)((?:pub\(super\)\s+)?)(" + re.escape(field) + r")\s*:", body)
    if not m: return False
    k = ob + m.start(2)
    src = src[:k] + "pub(crate) " + src[ob + m.start(3):]
    open(f, "w", encoding="utf-8").write(src)
    return True


def widen_type(name):
    import glob
    pat = re.compile(r"^(\s*)((?:pub\(super\)\s+)?)((?:struct|enum|type|trait|union)\s+" + re.escape(name) + r"\b)", re.M)
    for f in list(DEST_FILES) + [g for g in glob.glob(PLUGIN + "/**/🦀️.rs", recursive=True) if "/📦️packages/" not in g]:
        if not os.path.exists(f): continue
        src = open(f, encoding="utf-8").read()
        m = pat.search(src)
        if m:
            src = src[:m.start(2)] + "pub(crate) " + src[m.start(3):]
            open(f, "w", encoding="utf-8").write(src)
            return True
    return False


def imported_names(src, gated=False):
    names = set()
    for m in re.finditer(r"^(#\[cfg\([^\]]*\)\]\s*)?(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);", src, re.M):
        if bool(m.group(1)) != gated: continue
        body = m.group(2)
        for part in re.split(r"[{},]", body):
            part = part.strip()
            if not part or part.endswith("*"): continue
            al = re.search(r"\bas\s+(\w+)$", part)
            names.add(al.group(1) if al else part.split("::")[-1])
    return names


def dedupe(f):
    src = open(f, encoding="utf-8").read()
    defined = set(re.findall(r"^(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union)\s+(\w+)", src, re.M))
    plain = imported_names(src, False)
    out, seen, dropped = [], set(), 0
    for line in src.split("\n"):
        m = re.match(r"^(#\[cfg\([^\]]*\)\]\s*)?use\s+[\w:]+::(\w+);$", line)
        if m:
            name = m.group(2)
            if name in defined or (m.group(1) and name in plain) or line in seen:
                dropped += 1; continue
            seen.add(line)
        out.append(line)
    if dropped: open(f, "w", encoding="utf-8").write("\n".join(out))
    return dropped


def prune(msgs):
    """✂️ Applies rustc's machine-applicable removals of unused imports inside the decomposed modules."""
    edits = defaultdict(list)
    for m in msgs:
        if m["level"] != "warning" or (m.get("code") or {}).get("code") != "unused_imports": continue
        for c in m.get("children", []):
            for s in c.get("spans", []):
                if s.get("suggested_replacement") == "" and s.get("suggestion_applicability") == "MachineApplicable":
                    f = abspath(s["file_name"])
                    if (f in DEST_FILES or PRUNE_ALL and f.startswith(PLUGIN) and "/📦️packages/" not in f) and f not in DEST_FILES_VERBATIM and s["byte_end"] <= header_end(f):
                        edits[f].append((s["byte_start"], s["byte_end"], c["message"].startswith("remove the whole")))
    total = 0
    for f, es in edits.items():
        data = open(f, "rb").read()
        es = sorted(set(es), key=lambda e: (-e[0], -e[1]))
        last = None
        for a, b, whole in es:
            if last is not None and b > last: continue
            if whole:
                k = a
                while True:
                    ls = data.rfind(b"\n", 0, max(k - 1, 0)) + 1
                    prev = data[ls:k].strip()
                    if k > 0 and prev and (prev.startswith(b"///") or prev.startswith(b"//") or prev.startswith(b"#[")):
                        k = ls; continue
                    break
                a = k
                if data[b:b + 1] == b"\n": b += 1
            data = data[:a] + data[b:]
            last = a
            total += 1
        txt = data.decode("utf-8")
        txt = re.sub(r"\n{3,}", "\n\n", txt)
        open(f, "w", encoding="utf-8").write(txt)
    return total


_ri = None


def header_end(f):
    """📏️ Byte offset where the leading block of private `use` items of a decomposed module ends."""
    global _ri
    if _ri is None:
        import importlib.util
        spec = importlib.util.spec_from_file_location("rust_items", os.path.join(os.path.dirname(HERE), "e1-os-kernel", "🧱️rust_items.py"))
        _ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(_ri)
    raw = open(f, encoding="utf-8").read()
    cl = _ri.clean(raw)
    its, _ = _ri.items(raw, cl, 0, len(cl))
    end = 0
    for it in its:
        if it["kind"] != "use" or re.match(r"\s*pub", cl[it["head"]:it["end"]]): break
        end = it["end"]
    return len(raw[:end].encode("utf-8"))


def prune_spans(msgs):
    """✂️ Removes unused import leaves rustc reports without a removal suggestion, then drops emptied groups and statements."""
    spans = defaultdict(set)
    for m in msgs:
        if m["level"] != "warning" or (m.get("code") or {}).get("code") != "unused_imports": continue
        if any(s.get("suggested_replacement") == "" for c in m.get("children", []) for s in c.get("spans", [])): continue
        for s in m["spans"]:
            f = abspath(s["file_name"])
            if (f in DEST_FILES or PRUNE_ALL and f.startswith(PLUGIN) and "/📦️packages/" not in f) and f not in DEST_FILES_VERBATIM:
                spans[f].add((s["byte_start"], s["byte_end"]))
    total = 0
    for f, ss in spans.items():
        limit = header_end(f)
        ss = {(a, b) for a, b in ss if b <= limit}
        data = open(f, "rb").read()
        for a, b in sorted(ss, reverse=True):
            j = b
            while j < len(data) and data[j:j + 1] in b" \t\r\n": j += 1
            if data[j:j + 1] == b",":
                b = j + 1
            else:
                i = a
                while i > 0 and data[i - 1:i] in b" \t\r\n": i -= 1
                if data[i - 1:i] == b",": a = i - 1
            data = data[:a] + data[b:]
            total += 1
        txt = data.decode("utf-8")
        for _ in range(4):
            txt = re.sub(r"(?:,\s*)?\b[\w]+(?:::[\w]+)*::\{\s*\}", "", txt)
            txt = re.sub(r"\{\s*,\s*", "{", txt)
        lines, out = txt.split("\n"), []
        i = 0
        while i < len(lines):
            l = lines[i]
            if re.match(r"^\s*(?:#\[[^\]]*\]\s*)?(?:pub(?:\([^)]*\))?\s+)?use\s*(?:\{\s*\})?\s*;\s*$", l) or re.match(r"^\s*(?:#\[[^\]]*\]\s*)?(?:pub(?:\([^)]*\))?\s+)?use\s+[\w:]*\{\s*\}\s*;\s*$", l):
                while out and (out[-1].strip().startswith("///") or out[-1].strip().startswith("#[")): out.pop()
                i += 1; continue
            out.append(l); i += 1
        txt = re.sub(r"\n{3,}", "\n\n", "\n".join(out))
        open(f, "w", encoding="utf-8").write(txt)
    return total


def ungate(f, name, gate):
    """🔓️ Re-enables a gated preamble leaf (`//⟨…⟩`-tagged) that binds `name` in the configuration being checked."""
    if not os.path.exists(f): return False
    lines = open(f, encoding="utf-8").read().split("\n")
    for i, l in enumerate(lines):
        if "//⟨" not in l: continue
        m = re.match(r"^#\[cfg\((.*?)\)\]\s*(use\s+(.*?)(?:\s+as\s+(\w+))?;\s*//⟨.*)$", l)
        if not m: continue
        path, alias = m.group(3), m.group(4)
        bound = alias or path.split("::")[-1]
        if bound != name: continue
        if gate and m.group(1) == 'any(test, feature = "artifact-app-testing")' and gate in ("test", 'any(test, feature = "artifact-app-testing")'): continue
        lines[i] = m.group(2)
        open(f, "w", encoding="utf-8").write("\n".join(lines))
        return True
    return False


_owners = None
_usage = None


def usage():
    """📊️ How often each exact external `use` line already appears in the crate: the crate's own convention wins."""
    global _usage
    if _usage is None:
        from collections import Counter
        import glob
        _usage = Counter()
        for g in glob.glob(PLUGIN + "/**/🦀️.rs", recursive=True):
            for m in re.finditer(r"^\s*use\s+((?!crate|super|self)[\w:]+::(\w+));", open(g, encoding="utf-8").read(), re.M):
                _usage[f"use {m.group(1)};"] += 1
    return _usage


def owners():
    global _owners
    if _owners is None:
        import importlib.util
        spec = importlib.util.spec_from_file_location("imports", os.path.join(HERE, "🧹️imports.py"))
        im = importlib.util.module_from_spec(spec); spec.loader.exec_module(im)
        _owners = im.owner_index()
    return _owners


def main():
    tag = sys.argv[1]
    args = []
    gate = None
    if "--test" in sys.argv: args += ["--profile", "test"]; gate = "test"
    if "--target" in sys.argv: args += ["--target", sys.argv[sys.argv.index("--target") + 1]]
    if "--features" in sys.argv: args += ["--features", sys.argv[sys.argv.index("--features") + 1]]
    if "--gate" in sys.argv: gate = sys.argv[sys.argv.index("--gate") + 1]
    apply = "--apply" in sys.argv
    msgs = run(tag, args)
    if "--prune" in sys.argv:
        n_s = prune(msgs)
        n_l = prune_spans(msgs) if n_s == 0 else 0
        print(f"[fix-loop] {tag}: errors={sum(1 for m in msgs if m['level'] == 'error')} pruned {n_s} suggested, {n_l} leaf unused import spans (rerun to continue)")
        return
    index_tree()
    errors = [m for m in msgs if m["level"] == "error"]
    codes = defaultdict(int)
    for m in errors: codes[(m.get("code") or {}).get("code")] += 1
    print(f"[fix-loop] {tag}: {len(errors)} errors, {sum(1 for m in msgs if m['level'] == 'warning')} warnings", dict(codes))
    if not apply: return
    widen, fields, imports, unresolved, types = set(), set(), [], [], set()
    ungated = set()
    for m in errors:
        code = (m.get("code") or {}).get("code")
        spans = [s for s in m["spans"] if s["is_primary"]] or m["spans"]
        if not spans: unresolved.append(m["message"]); continue
        sp = spans[0]
        f = abspath(sp["file_name"])
        exp = sp.get("expansion")
        while exp:
            site = exp["span"]
            f = abspath(site["file_name"])
            exp = site.get("expansion")
        text = m["message"]
        handled = False
        for c in m.get("children", []):
            cm = c.get("message") or ""
            inacc = re.match(r"(?:\w+ )+`(crate::[\w:]+)` exists but is inaccessible", cm)
            for s in c.get("spans", []):
                if (code in ("E0603", "E0624") and "defined here" in cm) or inacc:
                    widen.add((abspath(s["file_name"]), s["line_start"])); handled = True
            if inacc and ("::tests::" in inacc.group(1) or "plugin_app_close_prelude" in inacc.group(1)):
                inacc = None
            if inacc:
                path = inacc.group(1)
                nm_ = path.split("::")[-1]
                if owners().get(nm_): path = owners()[nm_] + "::" + nm_
                imports.append((owner_file(f), f"use {path};")); handled = True
        for s2 in m["spans"]:
            if code in ("E0603", "E0624") and not s2["is_primary"] and "defined here" in (s2.get("label") or ""):
                widen.add((abspath(s2["file_name"]), s2["line_start"])); handled = True
        if code in ("E0603", "E0624"):
            if not handled: unresolved.append(f"{code} {text} @ {sp['file_name']}:{sp['line_start']}")
            continue
        if code == "E0451":
            mm = re.match(r"fields? (.*) of struct `((?:[\w:]+::)?)(\w+)(?:<[^`]*>)?` (?:is|are) private", text)
            if mm:
                for fd in re.findall(r"`(\w+)`", mm.group(1)): fields.add((mm.group(2).rstrip(":") + "::" + mm.group(3), fd))
            else: unresolved.append(f"{code} {text}")
            continue
        mt = re.match(r"type `(?:[\w:]+::)?(\w+)(?:<.*>)?` is private", text)
        if code is None and mt:
            types.add(mt.group(1)); continue
        if code == "E0616":
            mm = re.match(r"field `(\w+)` of struct `((?:[\w:]+::)?)(\w+)", text)
            if mm: fields.add((mm.group(2).rstrip(":") + "::" + mm.group(3), mm.group(1)))
            else: unresolved.append(f"{code} {text}")
            continue
        if handled: continue
        nm0 = re.match(r"(?:cannot find (?:[\w ,]+) `(\w+)`|failed to resolve: use of undeclared (?:type|crate or module) `(\w+)`|`(\w+)` is ambiguous)", text)
        if nm0:
            name0 = next(g for g in nm0.groups() if g)
            if ungate(owner_file(f), name0, gate): ungated.add((owner_file(f), name0)); continue
        sugg = suggestions(m)
        if sugg:
            best = min(sugg, key=score)
            nm_ = re.match(r"use\s+(?:[\w:]+::)?(\w+)(?:\s+as\s+\w+)?;", best)
            own_ = owners().get(nm_.group(1)) if nm_ else None
            if own_: best = f"use {own_}::{nm_.group(1)};"
            elif nm_:
                ext = [u for u in sugg if not re.match(r"use\s+crate::", u)]
                if ext: best = max(ext, key=lambda u: (usage().get(u.strip(), 0), -len(u)))
            mm2 = re.match(r"use\s+([\w:]+)::(\w+);", best)
            if mm2 and mm2.group(1).startswith("crate") and not re.match(r"^crate::(component::)?[a-z_0-9]+(::|$)", mm2.group(1)) or (mm2 and re.match(r"^crate::(plugin_app_close_prelude|app|__\w+|dsl|protocol|store|vcs)\b", mm2.group(1))):
                own = owners().get(mm2.group(2))
                external = sorted((u for u in sugg if not re.match(r"use\s+crate::", u)), key=lambda u: (u.count("::"), len(u)))
                if own: best = f"use {own}::{mm2.group(2)};"
                elif external: best = external[0]
                else: unresolved.append(f"{code} {text} (no owner for {best})"); continue
            if best.startswith("use crate::__semio_dispatch_"): best = best
            imports.append((owner_file(f), best))
            continue
        mm = re.match(r"`(\w+)` is ambiguous", text) or re.match(r"cannot find (?:macro|type|value|function|struct|trait|derive macro|attribute macro|tuple struct or tuple variant|unit struct, unit variant or constant|struct, variant or union type) `(\w+)`", text) or re.match(r"failed to resolve: use of undeclared (?:type|crate or module) `(\w+)`", text) or re.match(r"cannot find module or crate `(\w+)`", text)
        if mm:
            name = mm.group(1)
            owner = locate_definition(name)
            tgt = owner_file(f)
            if owner and DEST_FILES.get(tgt) != owner:
                imports.append((tgt, f"use crate::{owner}::{name};" if owner != "<crate>" else f"use crate::{name};"))
                continue
        unresolved.append(f"{code} {text} @ {sp['file_name']}:{sp['line_start']}")
    nw = sum(1 for f, l in sorted(widen) if widen_at(f, l))
    nf = sum(1 for st, fd in sorted(fields) if widen_field(st, fd))
    nt = sum(1 for t in sorted(types) if widen_type(t))
    print(f"[fix-loop] widened {nt} private types")
    ni = 0
    for t, line in dict.fromkeys(imports):
        if insert_import(t, line, gate):
            ni += 1
            if "--verbose" in sys.argv: print("   +", os.path.relpath(t, PLUGIN), line)
    nd = sum(dedupe(f) for f in DEST_FILES if os.path.exists(f))
    print(f"[fix-loop] dropped {nd} duplicate imports")
    print(f"[fix-loop] widened {nw} items, {nf} fields; inserted {ni} imports; ungated {len(ungated)}; unresolved {len(unresolved)}")
    for u in unresolved[:80]: print("   ", u)


def locate_definition(name):
    pat = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union)\s+" + re.escape(name) + r"\b", re.M)
    mac = re.compile(r"#\[macro_export\]\s*(?:#\[[^\]]*\]\s*)*macro_rules!\s*" + re.escape(name) + r"\b")
    hits = []
    for f, mod in DEST_FILES.items():
        if not os.path.exists(f): continue
        src = open(f, encoding="utf-8").read()
        if pat.search(src): hits.append(mod)
        if mac.search(src): return "<crate>"
    return hits[0] if len(hits) == 1 else None


if __name__ == "__main__":
    main()
