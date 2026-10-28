import re, sys, os, json
# streams files; prints one JSON row per cwd occurrence with class
files = [l.rstrip("\n") for l in open(sys.argv[1], encoding="utf8")]
pat = re.compile(r"process\.cwd\(\)|Deno\.cwd\(\)|Bun\.cwd|os\.Getwd\(\)|std::env::current_dir\(\)|env::current_dir\(\)")
rows = 0
for f in files:
    try:
        text = open(f, encoding="utf8", errors="replace").read()
    except OSError:
        continue
    for m in pat.finditer(text):
        a, b = max(0, m.start() - 90), min(len(text), m.end() + 90)
        w = text[a:b].replace("\n", " ")
        pre = text[max(0, m.start() - 60):m.start()]
        cls = "other"
        if re.search(r"(withRepository\w*|withStorybookStorage\w*|withFixtureRustProjects\w*|withBuildInference\w*|withRepositoryBuildInferenceDataset\w*|openRepository\w*|RepositoryCache\w*|Storage\w*|Artifacts\w*)\(\s*(\{[^}]*workspace:\s*)?$", pre) or re.search(r"\.cache/b", w):
            cls = "store"
        elif re.search(r"(readFileSync|readFile|Bun\.file|readdirSync|existsSync|statSync|lstatSync|resolve|join|readdir)\(\s*(resolve\(|join\()?\s*$", pre) or re.search(r"(readFileSync|Bun\.file|join|resolve)\([^()]*$", pre):
            cls = "path"
        elif re.search(r"cwd\s*:\s*$", pre):
            cls = "spawn-cwd"
        elif re.search(r"(root|workspace|repo|repository|checkout)\w*\s*(=|\?\?)\s*$", pre, re.I) or re.search(r"(SEMIO_WORKSPACE_ROOT|REPO_ROOT)[^;]*\?\?\s*$", pre):
            cls = "bind-root"
        print(json.dumps({"f": f, "o": m.start(), "c": cls, "w": w[:200]}, ensure_ascii=False))
        rows += 1
print(rows, file=sys.stderr)
