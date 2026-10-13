#!/usr/bin/env python3
"""🔧️ Wave 2 compile loop: runs one workspace check (JSON diagnostics) and applies the mechanical fixes the cut requires —
a missing crate dependency, or an item, module or field that another crate now names and so must be `pub`. Prints the
remaining non-mechanical errors.
usage: w2_fix.py <manifest> [extra cargo args...]"""
import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from ownership import CRATE, crate_roots

HERE = __file__.rsplit("/", 1)[0]
T = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
own = json.load(open(HERE + "/ownership.json"))
roots = crate_roots()
PKG = {k: (os.path.dirname(roots[k]) + "/📦️packages/🦀️rust" if k != "agg" else SEMIO + "/📦️packages/🦀️rust") for k in CRATE}
ws = toml_loads(read(SEMIO + "/Cargo.toml"))
WSDEPS = {k.replace("-", "_"): k for k in ws["workspace"]["dependencies"]}
WSDEPS.update({"dsl": "semio-framework-os-kernel", "protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel", "framework_schema": "semio-framework-schema", "value_derive": "semio-framework-value-derive", "graph_core": "semio-framework-graph"})
manifest = sys.argv[1]
args = sys.argv[2:]
SNAP = REPO + "/.🧬semio/🦑️repo/⚡️cache/s/snap"
use_snap = os.environ.get("SNAP") == "1"
if use_snap:
    subprocess.run(["rsync", "-a", "--delete", "--exclude", "target", SEMIO + "/", SNAP + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/"], check=True)
    manifest = SNAP + "/" + os.path.relpath(os.path.abspath(os.path.join(REPO, manifest)), REPO)
cmd = ["bash", HERE + "/🦀️cargo.sh", "580", "check", "--manifest-path", manifest, "--message-format", "json"] + args
proc = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True)
out = proc.stdout
if use_snap:
    out = out.replace(SNAP + "/", REPO + "/")
msgs = []
for line in out.splitlines():
    try:
        m = json.loads(line)
    except Exception:
        continue
    if m.get("reason") == "compiler-message" and m["message"]["level"] == "error":
        msgs.append(m)


def abs_path(target_src, rel):
    if not os.path.isabs(rel) and os.path.exists(os.path.join(SEMIO, rel)):
        return os.path.normpath(os.path.join(SEMIO, rel))
    p = rel if os.path.isabs(rel) else os.path.normpath(os.path.join(REPO, rel))
    if not os.path.exists(p):
        p = os.path.normpath(os.path.join(os.path.dirname(target_src), rel))
    return os.path.normpath(p)


def crate_of_target(m):
    name = m["target"]["name"]
    for k, n in CRATE.items():
        if n.replace("-", "_") == name:
            return k
    src = m["target"]["src_path"]
    for k in CRATE:
        if src.startswith(PKG[k].rsplit("/📦️packages", 1)[0]) and m["package_id"].split("#")[-1].startswith(CRATE[k] + "@"):
            return k
    for k in CRATE:
        if ("#" + CRATE[k] + "@") in m["package_id"] or ("/" + CRATE[k] + "#") in m["package_id"]:
            return k
    return None


add_deps, pubs, others, by_name = {}, set(), [], []
for m in msgs:
    msg = m["message"]
    text = msg["message"]
    key = crate_of_target(m)
    kind = m["target"]["kind"]
    mm = re.search(r"(?:cannot find module or crate|use of unresolved module or unlinked crate|failed to resolve: use of unresolved module or unlinked crate|cannot find) `([a-z_][a-z0-9_]*)`(?: in the crate root| in this scope)?", text)
    if mm and mm.group(1) in WSDEPS and key and ("unresolved" in text or "cannot find module or crate" in text or "in the crate root" in text):
        test = "test" in kind or "--tests" in args
        add_deps.setdefault((key, "dev" if test else "normal"), set()).add(WSDEPS[mm.group(1)])
        continue
    if msg.get("code") and msg["code"]["code"] == "E0432":
        hits = re.findall(r"`(semio_s_artifact_stdio_semio[a-z_]*)::(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Za-z_][A-Za-z0-9_]*)`", text)
        mapped = [(k, n) for c, n in hits for k in CRATE if CRATE[k].replace("-", "_") == c]
        if mapped:
            by_name.extend(mapped)
            continue
    if msg.get("code") and msg["code"]["code"] in ("E0364", "E0365"):
        nm = re.search(r"`([A-Za-z_][A-Za-z0-9_]*)` is only public within the crate", text)
        if nm and key:
            by_name.append((key, nm.group(1)))
            continue
    if msg.get("code") and msg["code"]["code"] in ("E0603", "E0624", "E0451", "E0616"):
        found = False
        for child in msg.get("children", []):
            for span in child.get("spans", []):
                if "defined here" in child["message"] or "is defined here" in child["message"] or span.get("label") and "defined here" in (span.get("label") or ""):
                    pubs.add((abs_path(m["target"]["src_path"], span["file_name"]), span["line_start"]))
                    found = True
        for span in msg["spans"]:
            if span.get("label") and "defined here" in span["label"]:
                pubs.add((abs_path(m["target"]["src_path"], span["file_name"]), span["line_start"])); found = True
        nm = re.search(r"(?:function|struct|enum|constant|module|type alias|trait|static|unit struct|tuple struct|macro) `([A-Za-z_][A-Za-z0-9_]*)` is private", text)
        if nm:
            for child in msg.get("children", []):
                for span in child.get("spans", []):
                    f = abs_path(m["target"]["src_path"], span["file_name"])
                    if f in own:
                        by_name.append((own[f], nm.group(1)))
        if not found:
            others.append(msg["rendered"])
        continue
    others.append(msg["rendered"])

if not msgs and proc.returncode != 0:
    print("errors=-1 fatal rc=%d" % proc.returncode)
    print("\n".join(l for l in proc.stderr.splitlines() if l.strip())[-2000:])
    sys.exit(1)
for (key, sec), deps in add_deps.items():
    p = PKG[key] + "/Cargo.toml"
    t = read(p)
    header = "[dependencies]\n" if sec == "normal" else "[dev-dependencies]\n"
    if header not in t:
        t = t.rstrip("\n") + "\n\n" + header
    for d in sorted(deps):
        if re.search(r"(?m)^%s = " % re.escape(d), t.split(header, 1)[1].split("\n[", 1)[0]):
            continue
        t = t.replace(header, header + "%s = { workspace = true }\n" % d, 1)
    write(p, t)
VIS = re.compile(r"^(\s*)(?:pub\s*\((?:crate|super|in [^)]*)\)\s*|)(?=(?:async\s+|const\s+|unsafe\s+|extern\s+\"C\"\s+)*(?:fn|struct|enum|mod|const|static|type|trait|use|union|macro)\b|[a-z_][a-z0-9_]*\s*:)")
changed = 0
for path, line in sorted(pubs):
    if not os.path.exists(path) or not path.startswith(SEMIO + "/"):
        others.append("FOREIGN VIS %s:%d" % (path, line))
        continue
    lines = read(path).split("\n")
    l = lines[line - 1]
    new = re.sub(r"^(\s*)pub\s*\((?:crate|super|in [^)]*)\)\s*", r"\1pub ", l, count=1)
    if new == l and not re.match(r"^\s*pub\b", l):
        new = re.sub(r"^(\s*)(?=(?:async\s+|const\s+|unsafe\s+)*(?:fn|struct|enum|mod|const|static|type|trait|use)\b|[a-z_][a-z0-9_]*\s*:)", r"\1pub ", l, count=1)
    if new != l:
        lines[line - 1] = new
        write(path, "\n".join(lines))
        changed += 1
    else:
        others.append("UNFIXED VIS %s:%d %s" % (path, line, l.strip()[:120]))
for key, name in by_name:
    hit = 0
    rx = re.compile(r"(?m)^(\s*)pub\s*\((?:crate|super)\)\s*((?:async\s+|const\s+|unsafe\s+)*(?:fn|struct|enum|type|const|static|trait|mod)\s+%s\b)" % re.escape(name))
    for f, k in own.items():
        if k != key:
            continue
        t = read(f)
        t2, c = rx.subn(r"\1pub \2", t)
        if c:
            write(f, t2)
            changed += c
            hit += c
    if not hit:
        others.append("UNPROMOTED %s::%s" % (key, name))
print("errors=%d deps=%s pub=%d remaining=%d" % (len(msgs), {"%s/%s" % k: sorted(v) for k, v in add_deps.items()}, changed, len(others)))
def located(o):
    m = re.search(r"--> ([^\n]+)", o)
    return m.group(1) if m else ""
semio_errors = [o for o in others if "app_testing" not in o and "🧰️framework" not in located(o) and "🗿️artifacts/📖️pdf" not in located(o) and "🗿️artifacts/🖊️dwg" not in located(o) and "🗿️artifacts/🧊️gltf" not in located(o)]
open(T + "/🗑️generated/lane-c/semio-errors.txt", "w").write("\n\n".join(semio_errors))
seen = set()
for o in others:
    first = o.split("\n")[0] + " " + (re.search(r"--> ([^\n]+)", o).group(1) if re.search(r"--> ([^\n]+)", o) else "")
    if first in seen:
        continue
    seen.add(first)
    print(first[:300])
