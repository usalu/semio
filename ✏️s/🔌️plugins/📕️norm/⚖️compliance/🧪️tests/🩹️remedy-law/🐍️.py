"""🩹 Remedy law: every check construction site of the ten Eurocode artifacts and the shared norm crates that can fail or warn attaches a remedy, or names the post-pass that attaches it.

`python -I 🐍️.py` exits non-zero and lists every offending site. Sites are the `CheckResult::assess(` chains and every `.build()` chain below `✏️s/🔌️plugins/📕️norm` (test folders and the non-Eurocode artifacts are not scanned; `--all` adds them). A site is clean when it cannot fail (`.informative`, `.not_applicable`, a constant `Pass`/`Info`/`UnverifiedBasis` status, an `info_floor` minimum), carries `.remedy(` or a remedy helper, binds a variable that later receives a remedy, or names `.remedies_attached_by("fn")` with `fn` defined in the same artifact or in a shared crate. The Rust side enforces the same law at run time: `CheckBuilder::build` rejects a failing check without a remedy in every profile (`MissingRemedy`).

Static rules on top of the presence rule: a typed remedy (`at_least`, `at_most`, `exactly`) never targets a text or enum field of the artifact's snapshot schema, an empty `one_of` is an advisory (`applicable: false`, `Remedy::advisory`), and no solver falls back to an unreachable value (`unwrap_or(mean * 2^40)`). The dynamic half of the law runs in Rust over every palette document of every code (`remedy_law_findings` of the app-surface edit rules, test `every_palette_document_obeys_the_remedy_law` in each editor): the target exists and is numeric (or an enum with a valid token), the remedy applies to a document that still decodes, the written value lies inside the field range, and a re-evaluation shows a lower utilisation or severity.

The scanner is independent of the Rust kernel and of the mutation vector harness; the self test pins its verdicts on synthetic sites.
"""
import json, pathlib, re, sys

NORM = pathlib.Path(__file__).resolve().parents[3]
ALL = "--all" in sys.argv
SKIP = {"target", "node_modules", "__pycache__"}
CAN_FAIL = re.compile(r"\.(utilization|minimum|stations|sampled|field|status|warn_above|warn_near_limit)\(")
GATED = re.compile(r"\.(stations|sampled|field|warn_above|warn_near_limit)\(")
REMEDY = re.compile(r"\.remedy\(|\.remedies_attached_by\(|remedies|remedy\w*\(|\bwith_remed")
DEFERRED = re.compile(r'\.remedies_attached_by\(\s*"([^"]+)"')
TYPED = re.compile(r"Remedy::(?:at_least|at_most|exactly)\(")
EMPTY_ONE_OF = re.compile(r"Remedy::one_of\((?:[^;]*?),\s*(?:Vec::new\(\)|vec!\[\])\s*,")
UNREACHABLE_FALLBACK = re.compile(r"unwrap_or\([^;]*\bpowi\(\s*\d{2,}")


def artifact_of(rel):
    return rel[1] if rel[0] == "🗿️artifacts" and len(rel) > 1 else None


def sources():
    for p in NORM.rglob("*.rs"):
        rel = p.relative_to(NORM).parts
        if SKIP.intersection(rel) or any(x.endswith("tests") for x in rel):
            continue
        artifact = artifact_of(rel)
        if artifact is not None and not ALL and not re.search(r"en199\d$", artifact):
            continue
        yield p


def strip(src):
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i)); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            out.append('"' + src[i + 1:j] .replace("\n", "\n") if False else '"' + "".join("\n" if ch == "\n" else " " for ch in src[i + 1:j]) + '"'); i = j + 1
        elif c == "'" and re.match(r"'(\\.|[^\\'])'", src[i:i + 4]):
            m = re.match(r"'(\\.|[^\\'])'", src[i:i + 4])
            out.append(" " * len(m.group())); i += len(m.group())
        else:
            out.append(c); i += 1
    return "".join(out)


def chain_end(s, start):
    depth = 0
    for i in range(start, len(s)):
        c = s[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth < 0:
                return i
        elif c == ";" and depth == 0:
            return i
    return len(s)


def chain_start(s, end):
    depth = 0
    for i in range(end - 1, -1, -1):
        c = s[i]
        if c in ")]}":
            depth += 1
        elif c in "([{":
            if depth == 0:
                return i + 1
            depth -= 1
        elif depth == 0 and (c in ";," or s.startswith("=>", i - 1)):
            return i + 1
    return 0


def non_failing(chain):
    if GATED.search(chain):
        return False
    if re.search(r"\.minimum\(", chain) and "info_floor(" in chain:
        return True
    return bool(re.search(r"\.status\((?:\w+::)*CheckStatus::(Pass|Info|NotApplicable|UnverifiedBasis)\)", chain))


def fn_end(s, pos):
    nxt = re.search(r"\bfn\s+\w+", s[pos:])
    return pos + nxt.start() if nxt else len(s)


def variable_remedied(name, text):
    n = re.escape(name)
    return bool(re.search(rf"\b{n}\b[^;]*\.\s*remed|\b{n}\s*=\s*[^;]*remed|\b{n}\.remedies\b|remed\w*\(\s*&?(?:mut\s+)?{n}\b", text))


def scan_text(raw, rel):
    s = strip(raw)
    found, sites = [], 0
    line = lambda pos: raw.count("\n", 0, pos) + 1
    for m in re.finditer(r"CheckResult::assess\(", s):
        sites += 1
        end = chain_end(s, m.start())
        chain = s[m.start():end]
        if not CAN_FAIL.search(chain) or REMEDY.search(chain) or non_failing(chain):
            continue
        bound = re.search(r"\blet\s+(?:mut\s+)?(\w+)\s*=$", s[max(0, m.start() - 80):m.start()].rstrip())
        if bound and variable_remedied(bound.group(1), s[end:fn_end(s, m.start())]):
            continue
        cid = re.search(r'"([^"]+)"', raw[m.end():m.end() + 200])
        found.append((rel, line(m.start()), cid.group(1) if cid else "?"))
    for m in re.finditer(r"\.(?:build|try_build)\(\)", s):
        start = chain_start(s, m.start())
        chain = s[start:m.start()]
        if "CheckResult::assess(" in chain or not CAN_FAIL.search(chain) or REMEDY.search(chain) or non_failing(chain):
            continue
        head = re.search(r"\b(\w+)\s*\.\s*(?:field|stations|sampled|utilization|minimum|status|warn_above|warn_near_limit)\(", chain)
        if head and variable_remedied(head.group(1), s[max(0, start - 6000):start]):
            continue
        found.append((rel, line(m.start()), "build-chain"))
    return sites, found


def scan_file(p):
    return scan_text(p.read_text(encoding="utf-8"), p.relative_to(NORM).as_posix())


def unresolved_deferrals():
    problems = []
    defined = {}
    for p in sources():
        rel = p.relative_to(NORM).parts
        artifact = artifact_of(rel) or "shared"
        text = p.read_text(encoding="utf-8")
        defined.setdefault(artifact, set()).update(re.findall(r"\bfn\s+(\w+)", text))
    for p in sources():
        rel = p.relative_to(NORM).parts
        artifact = artifact_of(rel) or "shared"
        raw = p.read_text(encoding="utf-8")
        for m in DEFERRED.finditer(strip(raw) if False else raw):
            if m.group(1) not in defined[artifact] | defined.get("shared", set()):
                problems.append((p.relative_to(NORM).as_posix(), raw.count("\n", 0, m.start()) + 1, f"post-pass {m.group(1)} is not defined in {artifact}"))
    return problems


def schema_text_fields(artifact_dir):
    base = artifact_dir / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any" / "🧬️schema"
    path = next((c for c in (base / "📸️snapshot" / "🔣️.json", base / "🔣️.json") if c.exists()), None)
    if path is None:
        return set()
    schema = json.loads(path.read_text(encoding="utf-8"))
    defs = schema.get("$defs", {}) or schema.get("definitions", {})
    numeric, text = set(), set()

    def walk(node):
        for name, prop in (node.get("properties") or {}).items():
            target = defs.get(prop["$ref"].split("/")[-1], {}) if "$ref" in prop else prop
            kind = target.get("type")
            if "enum" in target or kind == "string":
                text.add(name)
            elif kind in ("number", "integer"):
                numeric.add(name)
            elif kind == "array":
                item = target.get("items", {})
                walk(defs.get(item["$ref"].split("/")[-1], {}) if "$ref" in item else item)
            elif kind == "object":
                walk(target)

    for node in list(defs.values()) + [schema]:
        walk(node)
    return text - numeric


def first_argument(src, start):
    depth = 0
    for i in range(start, len(src)):
        c = src[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth < 0:
                return src[start:i]
        elif c == "," and depth == 0:
            return src[start:i]
    return src[start:]


def static_rules(raw, rel, text_fields):
    s = strip(raw)
    found = []
    line = lambda pos: raw.count("\n", 0, pos) + 1
    for m in TYPED.finditer(s):
        argument = raw[m.end():m.end() + len(first_argument(s, m.end()))]
        hit = [lit for lit in re.findall(r'"([A-Za-z][A-Za-z0-9_.\[\]=]*)"', argument) if lit.split(".")[-1] in text_fields]
        if hit:
            found.append((rel, line(m.start()), f"typed remedy on text or enum field {hit[0]}"))
    for m in EMPTY_ONE_OF.finditer(s):
        if "applicable: false" not in s[max(0, m.start() - 160):m.start()]:
            found.append((rel, line(m.start()), "empty one_of is applicable (use Remedy::advisory)"))
    for m in UNREACHABLE_FALLBACK.finditer(s):
        found.append((rel, line(m.start()), "solver falls back to an unreachable value"))
    return found


def scan():
    total, findings = 0, []
    for p in sorted(sources()):
        sites, found = scan_file(p)
        total += sites
        findings.extend(found)
    findings.extend(unresolved_deferrals())
    fields = {}
    for p in sorted(sources()):
        rel = p.relative_to(NORM).parts
        artifact = artifact_of(rel)
        if artifact is not None and artifact not in fields:
            fields[artifact] = schema_text_fields(NORM / "🗿️artifacts" / artifact)
        findings.extend(static_rules(p.read_text(encoding="utf-8"), p.relative_to(NORM).as_posix(), fields.get(artifact, set())))
    return total, sorted(set(findings))


HEAD = 'CheckResult::assess("id", "part", clause, subject, title)'
CASES = [
    ("failing utilization without remedy", f"fn a() {{ {HEAD}.utilization(x, y).annex(a).build(); }}", 1),
    ("failing minimum without remedy", f"fn a() {{ {HEAD}.minimum(x, y).build(); }}", 1),
    ("warning status without remedy", f"fn a() {{ {HEAD}.status(if ok {{ CheckStatus::Pass }} else {{ CheckStatus::Warning }}).build(); }}", 1),
    ("stations without remedy", f"fn a() {{ {HEAD}.stations(s).build(); }}", 1),
    ("explicit remedy", f"fn a() {{ {HEAD}.utilization(x, y).remedy(r).build(); }}", 0),
    ("remedy helper", f"fn a() {{ {HEAD}.utilization(x, y).remedy_of(r).build(); }}", 0),
    ("informative", f"fn a() {{ {HEAD}.informative(x).build(); }}", 0),
    ("not applicable", f"fn a() {{ {HEAD}.not_applicable(r).build(); }}", 0),
    ("constant pass", f"fn a() {{ {HEAD}.utilization(x, y).status(CheckStatus::Pass).build(); }}", 0),
    ("constant pass with warn_above", f"fn a() {{ {HEAD}.utilization(x, y).status(CheckStatus::Pass).warn_above(1.0).build(); }}", 1),
    ("info floor", f"fn a() {{ {HEAD}.minimum(x, info_floor(x)).build(); }}", 0),
    ("named post-pass", f'fn a() {{ {HEAD}.remedies_attached_by("post").utilization(x, y).build(); }}', 0),
    ("variable remedied later", f"fn a() {{ let mut b = {HEAD}.utilization(x, y); if bad {{ b = b.remedy(r); }} b.build(); }}", 0),
    ("variable never remedied", f"fn a() {{ let mut b = {HEAD}.utilization(x, y); b.build(); }}", 1),
    ("builder from a helper", "fn a() { base(k).utilization(x, y).build(); }", 1),
    ("builder from a helper with remedy", "fn a() { base(k).utilization(x, y).remedy(r).build(); }", 0),
    ("text in a string is not code", 'fn a() { log("CheckResult::assess(x).utilization(a, b).build()"); }', 0),
]


STATIC_CASES = [
    ("typed remedy on a text field", 'fn a() { Remedy::exactly(bref(b, "support"), d, d, t); }', {"support"}, 1),
    ("typed remedy on a numeric field", 'fn a() { Remedy::exactly(bref(b, "crackedLengthFraction"), d, d, t); }', {"support"}, 0),
    ("applicable empty one_of", 'fn a() { Remedy::one_of(subject, Vec::new(), text); }', set(), 1),
    ("advisory empty one_of", 'fn a() { Remedy { applicable: false, ..Remedy::one_of(subject, Vec::new(), text) }; }', set(), 0),
    ("one_of with options", 'fn a() { Remedy::one_of(subject, options, text); }', set(), 0),
    ("unreachable fallback", "fn a() { let n = solve().unwrap_or(mean * 2f64.powi(40)); }", set(), 1),
    ("reachable fallback", "fn a() { let n = solve().unwrap_or(mean * 2f64.powi(2)); }", set(), 0),
]


def self_test():
    failures = []
    for name, text, fields, expected in STATIC_CASES:
        found = static_rules(text, "case.rs", fields)
        if len(found) != expected:
            failures.append(f"{name}: expected {expected} findings, got {len(found)}")
    for name, text, expected in CASES:
        _, found = scan_text(text, "case.rs")
        if len(found) != expected:
            failures.append(f"{name}: expected {expected} findings, got {len(found)}")
    defined = {"post"}
    assert DEFERRED.search('x.remedies_attached_by("post")').group(1) in defined
    return failures


def main():
    failures = self_test()
    for failure in failures:
        print("self test failed:", failure)
    if failures:
        return 2
    total, findings = scan()
    print(f"remedy law: sites {total} findings {len(findings)} self-test cases {len(CASES) + len(STATIC_CASES)}")
    for rel, line, what in findings:
        print(f"{rel}:{line}: {what}")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
