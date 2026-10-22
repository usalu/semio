BUDGET = TAXONOMY["libWiringLineBudget"]
OPEN = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn\s+\w+\s*(?:<[^>]*>)?\s*\([^;{]*\)[^;{]*|impl(?:<[^>]*>)?\s+[^;{]+)\{\s*$", A)
WIRING = re.compile(r'^\s*(//.*)?$|^\s*export\s+(type\s+)?(\*|\{[^}]*\})\s+from\s+"[^"]+";?\s*(//.*)?$|^\s*import\s+.+\s+from\s+"[^"]+";?\s*(//.*)?$')


def lib_shape(t, crates):
    out = []
    for c in crates:
        if c["shape"] != "taxonomy" or c["libRelPath"] not in t.files:
            continue
        content = t.files[c["libRelPath"]]
        lines = split_lines(content)
        starts, offset = [], 0
        for line in lines:
            starts.append(offset)
            offset += len(line) + 1
        body_lines, covered = 0, 0
        for i, line in enumerate(lines):
            no = i + 1
            if no <= covered or not OPEN.search(line):
                continue
            body = extract_body(content, starts[i])
            lst = split_lines(body)
            body_lines += sum(1 for l in lst if l.strip() and not l.strip().startswith("//"))
            covered = no + len(lst) - 1
        if body_lines > BUDGET:
            out.append(["taxonomy-lib-shape-" + c["libRelPath"], "taxonomy/lib-shape", "medium" if c["pluginId"] else "low", 0])
    return out


def barrel_shape(t):
    out = []
    for p in sorted(t.files):
        if not p.endswith("/📦packages/🟦typescript/package.json"):
            continue
        meta = json.loads(t.files[p])
        if "role" not in meta.get("semio", {}):
            continue
        entry = p[: -len("package.json")] + "🟦.ts"
        if entry not in t.files:
            continue
        non_wiring = [l for l in split_lines(t.files[entry]) if l.strip() and not WIRING.search(l)]
        if len(non_wiring) > BUDGET:
            out.append(["taxonomy-barrel-shape-" + p, "taxonomy/barrel-shape", "medium", 0])
    return out


def run(c):
    t = Tree(expand(c))
    return lib_shape(t, c["crates"]) if c["rule"] == "policyTaxonomyLibShapeBreaches" else barrel_shape(t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
