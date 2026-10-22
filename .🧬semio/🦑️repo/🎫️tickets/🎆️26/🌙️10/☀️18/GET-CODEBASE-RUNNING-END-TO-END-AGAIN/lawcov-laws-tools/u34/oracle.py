SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🟦.ts")
TEXT = open(SOURCE, encoding="utf8").read()


def strings(body):
    return [json.loads('"%s"' % s) for s in re.findall(r'"((?:[^"\\]|\\.)*)"', body)]


def const_list(name):
    return strings(re.search(r"export const %s = \[(.*?)\] as const;" % name, TEXT, re.S).group(1))


def file_constants():
    return {m.group(1): m.group(2) for m in re.finditer(r'^export const (INTERACTIVITY_[A-Z0-9_]*FILE) = "([^"]*)";', TEXT, re.M)}


def allowlist():
    body = re.search(r"export const INTERACTIVITY_AUDIT_ALLOWLIST[^=]*= \[(.*?)\n\];", TEXT, re.S).group(1)
    out = []
    for m in re.finditer(r"\{\s*file: \"([^\"]*)\",\s*lineHint: (\d+),\s*pattern: \"(\w+)\",.*?inScope: (true|false),(?:\s*expectedNeverToMatch: (true))?", body, re.S):
        out.append({"file": m.group(1), "lineHint": int(m.group(2)), "pattern": m.group(3), "inScope": m.group(4) == "true", "expectedNeverToMatch": m.group(5) == "true"})
    return out


def run(c):
    a, r = c.get("args", []), c["rule"]
    if r == "pattern":
        rx = re.compile(c["args"][0]["source"], A)
        return [bool(rx.search(s)) for s in c["args"][0]["samples"]]
    if r == "constants":
        return {n: const_list(n) for n in ("INTERACTIVITY_AUDIT_UI_ROOTS", "INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES", "INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS", "INTERACTIVITY_AUDIT_PRODUCTION_ROOTS")}
    if r == "allowlist":
        return allowlist()
    if r == "fileConstants":
        return sorted(file_constants())
    raise KeyError(r)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
