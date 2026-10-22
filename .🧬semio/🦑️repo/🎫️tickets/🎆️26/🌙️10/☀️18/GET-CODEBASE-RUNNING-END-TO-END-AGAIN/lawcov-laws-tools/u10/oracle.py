SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🟦.ts")
text = open(SOURCE, encoding="utf8").read()


def literal_values(body):
    return [json.loads('"%s"' % m) for m in re.findall(r'"((?:[^"\\]|\\.)*)"', re.sub(r"//[^\n]*", "", body))]


def parse(name):
    m = re.search(r"^export const %s\s*=\s*(?:new Set<string>\(\[(.*?)\]\)|\[(.*?)\]|(\"(?:[^\"\\]|\\.)*\"));" % re.escape(name), text, re.M | re.S)
    if m.group(1) is not None:
        return {"kind": "set", "values": sorted(literal_values(m.group(1)), key=lambda s: s.encode("utf-16-be"))}
    if m.group(2) is not None:
        return {"kind": "array", "values": literal_values(m.group(2))}
    return {"kind": "string", "values": literal_values(m.group(3))}


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: parse(c["args"][0]) for c in cases})
