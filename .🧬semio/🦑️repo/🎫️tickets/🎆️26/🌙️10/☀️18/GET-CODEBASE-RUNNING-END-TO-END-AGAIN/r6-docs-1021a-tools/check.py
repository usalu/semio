#!/usr/bin/env python3
"""Read-only verification of the staged fragments (EN and DE, rendered against the live tree) without running any repo command.

(1) every `repo ...` command (fenced or inline) names an operation, subcommand and flags of the live catalogue; the operations that are only staged are
    checked against the tokens their staged tool directories contain; the root command `verify build-parity` against the argument grammar of its module;
(2) every flag in backticks is known; every SEMIO_* variable occurs in a result file of the ticket or in the live tree policy;
(3) every backticked path resolves in the tree; a path of a staged feature must have a resolving parent and its last segment must occur in a result file;
(4) fences balance, table rows have the header's column count, heading emoji carry U+FE0F, EN and DE carry equal numbers of headings, tables, fences and commands;
(5) no secret-like token, no emoji in the German text that the English text lacks (headings only), no leftover macro.
Usage: check.py [--root DIR]   (read only)"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import lib

TICKET = os.path.dirname(HERE)
ROOT, LIB = lib.ROOT, os.path.join(lib.ROOT, lib.LIB)
cat = lib.catalog()
operations = {o["name"]: o for o in cat["operations"]}
flags = {f["name"]: f for f in cat["flags"]}
GLOBAL = {f["name"] for f in cat["flags"] if "operations" not in f} | {"--tool", "--workers", "--help", "--lang", "--service"}
states, _ = lib.probe_states()
failures, notes = [], []


def fail(message):
    failures.append(message)


results_text = ""
for name in os.listdir(TICKET):
    if name.startswith("result-") and name.endswith(".md"):
        results_text += open(os.path.join(TICKET, name), encoding="utf8").read()


def tool_has(directory, token):
    base = os.path.join(TICKET, directory)
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in ("backup", "node_modules", "exp", "verify")]
        for f in fn:
            if f.endswith((".py", ".json", ".ts", ".md", ".sh", ".mjs")):
                try:
                    if token in open(os.path.join(dp, f), encoding="utf8").read():
                        return True
                except (UnicodeDecodeError, OSError):
                    pass
    return False


STAGED = {
    "leases": {"dir": "conc-leases-tools", "subcommands": {"status"}, "flags": {"--json", "--all", "--lease-directory", "--violations"}},
    "concurrency": {"dir": "conc-harness-tools", "subcommands": {"list", "plan", "run"}, "flags": {"--workloads", "--sessions", "--criteria", "--implementations", "--root", "--file", "--json"}},
    "cache": {"dir": "cache-gc-tools", "subcommands": {"gc"}, "flags": {"--dry-run", "--area", "--json", "--tasks"}},
    "tools": {"dir": "tool-iface-tools", "subcommands": {"list", "verify", "exec"}, "flags": {"--only", "--cwd", "--light", "--json"}},
    "service": {"dir": "hot-cache-tools", "subcommands": {"start", "status", "stop", "issue"}, "flags": set()},
    "session": {"dir": "agent-session-tools", "subcommands": {"open", "status", "close", "list"}, "flags": {"--agent", "--client", "--client-pid", "--owner-pid", "--weight", "--export", "--dead", "--force", "--json", "--all"}},
    "remote-cache": {"dir": "rcache-server-tools", "subcommands": {"serve", "status", "gc"}, "flags": {"--port", "--host", "--directory", "--max-bytes", "--read-only", "--tls-certificate", "--tls-key", "--dry-run", "--json"}},
    "bench": {"dir": "perf-slo-tools", "subcommands": {"list", "run", "compare"}, "flags": {"--slo", "--runs", "--baseline", "--save-baseline", "--current", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json"}},
    "conformance": {"dir": "backend-contract-tools", "subcommands": {"list", "plan", "run"}, "flags": {"--scenario", "--implementations", "--keep", "--file", "--json"}},
    "units": {"dir": "polyglot-units-tools", "subcommands": {"plan", "run", "emit", "exec"}, "flags": {"--for", "--receipts", "--concurrency"}},
}
EXTRA_FLAGS = {"--service", "--watch", "--cwd", "--version", "--graph-policy", "--check", "--ecosystem", "--remote_cache", "--undo"}
VERIFY_SUBCOMMANDS = {"build-parity": None, "tool-spawns": {"--report", "--json", "--strict", "--prune", "--accept-new"}}
STAGED_ENV = {"SEMIO_LEASE_ORDER": "conc-leases-tools", "SEMIO_LEASE_REGISTRY_DIRECTORY": "conc-leases-tools", "SEMIO_LEASE_INHERITED": "conc-leases-tools",
              "SEMIO_LEASE_TRACE": "conc-harness-tools", "SEMIO_BUILD_GRAPH_POLICY": "conc-graph-tools", "SEMIO_BUILD_GRAPH_VERIFY": "conc-graph-tools",
              "SEMIO_BAZEL_SERVERS": "conc-bazel-tools", "SEMIO_BAZEL_SERVICES": "conc-bazel-tools", "SEMIO_CARGO_LANE": "conc-cargo-tools",
              "SEMIO_UNIT_CACHE_SLOTS": "conc-cargo-tools", "SEMIO_RUNTIME": "repo-launcher-tools", "SEMIO_RUNTIME_CACHE": "repo-launcher-tools",
              "SEMIO_RUNTIME_MIRROR": "repo-launcher-tools", "SEMIO_TOOL_GUARD": "tool-iface-tools", "SEMIO_PACKAGE_MANAGER_": "pm-seam-tools",
              "SEMIO_AGENT_SESSION": "agent-session-tools", "SEMIO_AGENT_SESSION_DIRECTORY": "agent-session-tools", "SEMIO_RESIDENT_EXECUTORS": "hot-cache-tools",
              "SEMIO_REMOTE_CACHE_SERVER_READ_ONLY": "rcache-server-tools", "SEMIO_NATIVE_STORAGE_BACKEND": "storage-seam-tools"}
for name, spec in STAGED.items():
    for token in sorted(spec["flags"] - {"--json", "--file", "--root", "--implementations", "--all", "--port", "--host", "--directory", "--max-bytes", "--client", "--force"}) + sorted(spec["subcommands"] - {"list", "plan", "run", "status", "open", "close", "start", "stop", "verify", "serve", "compare"}):
        if not tool_has(spec["dir"], token):
            fail(f"staged token `{token}` of `{name}` not found in {spec['dir']}")
for variable, directory in STAGED_ENV.items():
    if not tool_has(directory, variable):
        fail(f"staged variable {variable} not found in {directory}")

parity = open(os.path.join(LIB, "🏗️build/🪞️parity/⌨️command/🟦️.ts"), encoding="utf8").read()
sets = re.findall(r"new Set\(\[(.*?)\]\)", parity)
VERIFY_FLAGS = set(re.findall(r'"(--[a-z-]+)"', " ".join(sets[:2]))) | {"--help", "--lang"}
if len(VERIFY_FLAGS) < 7:
    fail("could not read the verify build-parity grammar")

BAZEL_FLAGS = {"--local_resources", "--jobs"}
KNOWN_FLAGS = EXTRA_FLAGS | BAZEL_FLAGS | set(flags) | GLOBAL | VERIFY_FLAGS | set().union(*[spec["flags"] for spec in STAGED.values()]) | VERIFY_SUBCOMMANDS["tool-spawns"] | STAGED["leases"]["flags"] | STAGED["concurrency"]["flags"] | STAGED["cache"]["flags"] | {"--graph-policy", "--store", "--configuration"}


for identifier, spec in lib.PROBES.items():
    if spec[0] == "exists" and not states[identifier] and spec[1].split("/")[-1] not in results_text:
        fail(f"probe `{identifier}`: the last segment of {spec[1]} occurs in no result file (mistyped?)")


def commands_in(text):
    found, fence = [], False
    for row in text.split("\n"):
        if row.startswith("```"):
            fence = not fence
            continue
        if fence:
            candidate = re.sub(r"^(?:[A-Z_]+=\S+\s+)+", "", row.strip())
            if candidate and not candidate.startswith(("#", "$env:")) and re.match(r"^(?:bun )?repo\s", candidate):
                found.append(candidate)
        else:
            for m in re.finditer(r"`((?:bun )?repo\s[^`]*)`", row):
                found.append(m.group(1).replace("\\|", "|"))
    return found


def check_command(command):
    words = re.sub(r"^bun\s+", "", command).split()[1:]
    if "--" in words:
        words = words[: words.index("--")]
    words = [w for w in words if not (w.startswith("<") or w.startswith("["))]
    positional = [w for w in words if not w.startswith("-")]
    given = [w for w in words if w.startswith("-")]
    if not positional:
        return
    op = positional[0]
    if op.startswith((".", "<")) or op in ("args", "dashboard"):
        return
    if op == "verify":
        sub = positional[1:2]
        if not sub or sub[0] not in VERIFY_SUBCOMMANDS:
            fail(f"`{command}`: only verify build-parity and verify tool-spawns are documented")
            return
        allowed_verify = (VERIFY_FLAGS if sub[0] == "build-parity" else VERIFY_SUBCOMMANDS[sub[0]] | {"--json"}) | GLOBAL
        for word in given:
            if word.split("=")[0] not in allowed_verify:
                fail(f"flag not accepted by verify {sub[0]} in `{command}`: {word}")
        return
    live = operations.get(op)
    staged = STAGED.get(op)
    if live is None and staged is None:
        fail(f"unknown operation in `{command}`: {op}")
        return
    names = {s["name"] for s in (live or {}).get("subcommands", [])} | (staged["subcommands"] if staged else set())
    if len(positional) > 1 and names:
        if op == "cache" and positional[1] == "remote":
            pass
        elif op == "tools" and positional[1] == "setup":
            pass
        elif any(part not in names for part in positional[1].split("|")):
            fail(f"unknown subcommand in `{command}`: {positional[1]}")
    allowed = set((live or {}).get("flags", [])) | (staged["flags"] if staged else set()) | GLOBAL
    for word in given:
        name = word.split("=")[0]
        if name not in KNOWN_FLAGS:
            fail(f"flag not known in `{command}`: {name}")
        elif name not in allowed:
            fail(f"flag {name} not offered by operation {op} in `{command}`")


RUNTIME = re.compile(r"^(?:\.cache|\.🧬semio/|~/|\$XDG|%LOCALAPPDATA%|/private|<|\{|nx/|bazel/|m/|s0|d0)")
policy_text = open(lib.POLICY, encoding="utf8").read()


def resolve(token):
    if "<" in token or "*" in token or token.startswith("http") or " " in token or RUNTIME.match(token):
        return None
    t = token[len("📚️library/"):] if token.startswith("📚️library/") else token
    bases = [ROOT, os.path.join(ROOT, lib.PRODUCT), os.path.join(ROOT, lib.PRODUCT, "🔨️modules"), LIB, os.path.join(LIB, "🏗️build"), os.path.join(LIB, "🏗️build/🔧️tools/🏗️bazel"),
             os.path.join(LIB, "⚡️caching"), os.path.join(ROOT, lib.PROCESS), os.path.join(ROOT, lib.PROCESS, "🧪️testing"), os.path.join(LIB, "🏗️build/🕸️graph/📥️inference")]
    if token.startswith("📚️library/"):
        bases = [LIB]
    for base in bases:
        if os.path.exists(os.path.join(base, t)):
            return "ok"
    segments = [s for s in t.split("/") if s]
    for base in bases:
        depth = 0
        while depth < len(segments) and os.path.exists(os.path.join(base, *segments[: depth + 1])):
            depth += 1
        if depth >= 1 and all(seg in results_text for seg in segments[depth:]):
            return "staged"
    return "missing"


def paths_in(text):
    out, fence = [], False
    for row in text.split("\n"):
        if row.startswith("```"):
            fence = not fence
            continue
        if fence:
            continue
        for m in re.finditer(r"`([^`]+)`", row):
            token = m.group(1)
            if "/" in token and re.search(r"[^\x00-\x7f]", token) and not token.startswith(("repo ", "bun ")):
                out.append(token)
            elif re.fullmatch(r"[^\s`]+\.(?:json|ts|mjs|md)", token) and re.search(r"[^\x00-\x7f]", token):
                out.append(token)
    return out


def structure(name, text):
    lines = text.split("\n")
    fences = sum(1 for l in lines if l.startswith("```"))
    if fences % 2:
        fail(f"{name}: unbalanced code fences")
    fence, block, tables = False, [], 0
    for l in lines + [""]:
        if l.startswith("```"):
            fence = not fence
        if not fence and l.startswith("|"):
            block.append(l)
        elif block:
            tables += 1
            cells = [len(re.split(r"(?<!\\)\|", b.strip().strip("|"))) for b in block]
            if len(set(cells)) != 1:
                fail(f"{name}: table column counts differ {cells} in the table starting `{block[0][:50]}`")
            block = []
    headings = [l for l in lines if re.match(r"^#{3,4} ", l)]
    return {"headings": len(headings), "tables": tables, "fences": fences // 2, "commands": len(commands_in(text))}, headings


stats = {}
for language in ("en", "de"):
    parts = lib.sections(language)
    text = "\n\n".join(lib.render(content, language, states) for _, content in parts)
    open(os.path.join(HERE, f"rendered-{language}.md.txt"), "w", encoding="utf8").write(text + "\n") if "--keep" in sys.argv else None
    for command in commands_in(text):
        check_command(command)
    for word in sorted(set(re.findall(r"`(--[a-z][a-z_-]*)", text))):
        if word not in KNOWN_FLAGS:
            fail(f"{language}: flag not known: {word}")
    for variable in sorted(set(re.findall(r"\bSEMIO_[A-Z0-9_]+\b", text))):
        if variable not in STAGED_ENV and variable not in results_text and variable not in policy_text:
            fail(f"{language}: variable not found in any result file: {variable}")
    tally = {"ok": 0, "staged": 0, "missing": 0}
    for token in paths_in(text):
        status = resolve(token)
        if status is None:
            continue
        tally[status] += 1
        if status == "missing":
            fail(f"{language}: path does not resolve: {token}")
    notes.append(f"{language}: paths ok={tally['ok']} staged={tally['staged']} missing={tally['missing']}, {len(text)} characters")
    stats[language], headings = structure(language, text)
    for h in headings:
        first = h.split(" ", 2)[1]
        if language == "en" and ord(first[0]) > 0x2000 and not first.endswith("️"):
            fail(f"heading emoji without U+FE0F: {h}")
    if re.search(r"(?:ghp_[A-Za-z0-9]{20,}|\bsk-[A-Za-z0-9]{20,}|AKIA[0-9A-Z]{12,})", text):
        fail(f"{language}: secret-like token")
    fenced = False
    for row in text.split("\n"):
        if row.startswith("```"):
            fenced = not fenced
        elif not fenced and row.count("`") % 2:
            fail(f"{language}: odd number of backticks: {row[:90]}")
    stats[language]["_text"] = text
if {k: v for k, v in stats["en"].items() if k != "_text"} != {k: v for k, v in stats["de"].items() if k != "_text"}:
    fail("EN and DE structure differ: " + json.dumps({l: {k: v for k, v in s.items() if k != '_text'} for l, s in stats.items()}))
for h_en, h_de in zip([l for l in stats["en"]["_text"].split("\n") if l.startswith("### ")], [l for l in stats["de"]["_text"].split("\n") if l.startswith("### ")]):
    pass
common = lambda t: sorted(set(re.findall(r"\bSEMIO_[A-Z0-9_]+\b", t)))
if common(stats["en"]["_text"]) != common(stats["de"]["_text"]):
    fail("EN and DE name different SEMIO_* variables: " + str(set(common(stats['en']['_text'])) ^ set(common(stats['de']['_text']))))
cm = lambda t: sorted(set(re.findall(r"`(repo [^`]+)`", t)))
if cm(stats["en"]["_text"]) != cm(stats["de"]["_text"]):
    fail("EN and DE inline commands differ: " + str(set(cm(stats['en']['_text'])) ^ set(cm(stats['de']['_text'])))[:600])
def numbers(language):
    found = collections.Counter()
    for name, content in lib.sections(language):
        text = re.sub(r"`[^`]*`", "", lib.render(content, language, states))
        for m in re.findall(r"\d+", re.sub(r"(?<=\d)[.,](?=\d)", "", text)):
            found[(name, m)] += 1
    return found


en_numbers, de_numbers = numbers("en"), numbers("de")
for key in sorted(set(en_numbers) | set(de_numbers)):
    if en_numbers[key] != de_numbers[key]:
        fail(f"number {key[1]} occurs {en_numbers[key]}x in EN and {de_numbers[key]}x in DE ({key[0]})")
notes.append("structure " + json.dumps({l: {k: v for k, v in s.items() if k != '_text'} for l, s in stats.items()}))
edits_path = os.path.join(HERE, "staged", "edits.json")
if os.path.isfile(edits_path):
    edits = json.load(open(edits_path, encoding="utf8"))["edits"]
    extra = "\n".join(e["new"] for e in edits if not e["id"].startswith("sections-"))
    for command in commands_in(extra):
        check_command(command)
    for edit in edits:
        if edit["id"].startswith("commands-"):
            for row in edit["new"].split("\n"):
                if row.startswith("repo "):
                    check_command(re.split(r"\s{2,}#", row)[0].replace("|clear", "").replace("status [--tasks]|gc [--dry-run]", "status"))
    tally = {"ok": 0, "staged": 0, "missing": 0}
    for token in paths_in(extra):
        status = resolve(token)
        if status is None:
            continue
        tally[status] += 1
        if status == "missing":
            fail(f"edits.json: path does not resolve: {token}")
    notes.append(f"edits.json: {len(edits)} edits, paths ok={tally['ok']} staged={tally['staged']} missing={tally['missing']}")
    for edit in edits:
        for variable in re.findall(r"\bSEMIO_[A-Z0-9_]+\b", edit["new"]):
            if variable not in STAGED_ENV and variable not in results_text and variable not in policy_text:
                fail(f"edits.json {edit['id']}: variable not found: {variable}")
for n in notes:
    print(n)
if failures:
    print("FAIL")
    for f in failures:
        print("  " + f)
    sys.exit(1)
print("ok")
