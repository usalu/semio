SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🟦.ts")


def wiring():
    out = []
    for m in re.finditer(r"^( +)breaches\.push\(\.\.\.(\w+)\(([^)]*)\)\);", open(SOURCE, encoding="utf8").read(), re.M):
        out.append({"name": m.group(2), "args": m.group(3), "loop": len(m.group(1)) > 2})
    return out


def imports():
    text = open(SOURCE, encoding="utf8").read()
    out = {}
    for m in re.finditer(r'import \{([^}]*)\} from "([^"]+)";', text):
        for name in m.group(1).split(","):
            name = name.strip()
            if name and not name.startswith("type "):
                out[name] = m.group(2)
    return out


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: wiring() if c["rule"] == "wiring" else imports() for c in cases})
