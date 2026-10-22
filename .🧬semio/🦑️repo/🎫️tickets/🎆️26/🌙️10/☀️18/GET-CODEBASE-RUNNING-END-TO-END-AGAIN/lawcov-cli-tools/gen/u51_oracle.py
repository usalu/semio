from collections import Counter

for row in corpus["compare"]:
    files = {}
    inline = set()
    problems = []
    for crate in row["carved"]:
        for m in crate:
            if m["inline"]:
                inline.add(m["path"])
                continue
            old = files.get(m["path"])
            if old is not None and (old["file"] != m["file"] or old["cfgTest"] != m["cfgTest"]):
                problems.append("module %s is declared twice: %s and %s" % (m["path"], old["file"], m["file"]))
            else:
                files.setdefault(m["path"], m)
    expected = {}
    for m in row["original"]:
        if not m["inline"]:
            expected[m["path"]] = m
    for path, m in expected.items():
        got = files.get(path)
        if got is None:
            problems.append("module %s (%s) is missing from the carved crates" % (path, m["file"]))
        elif got["file"] != m["file"] or got["cfgTest"] != m["cfgTest"]:
            problems.append("module %s changed: %s -> %s" % (path, m["file"], got["file"]))
    for path in files:
        if path not in expected:
            problems.append("module %s appears only in the carved crates" % path)
    for m in row["original"]:
        if m["inline"] and m["path"] not in inline and m["path"] not in files:
            problems.append("inline module %s is missing" % m["path"])
    if problems != row["problems"] or len(expected) != row["originalCount"] or len(files) != row["unionCount"]:
        failures.append({"compare": row["name"], "got": problems})
for row in corpus["diff"]:
    left = Counter(row["original"])
    right = Counter(row["candidate"])
    only_candidate = []
    remaining = Counter(left)
    for item in row["candidate"]:
        if remaining[item] > 0:
            remaining[item] -= 1
        else:
            only_candidate.append(item)
    only_original = sorted((left - right).elements())
    if only_candidate != row["onlyCandidate"] or only_original != sorted(row["onlyOriginal"]):
        failures.append({"diff": row["name"]})
for row in corpus["plain"]:
    got = "::".join(s for s in row["segments"] if s != "" and s != "crate" and not s.startswith("__c_"))
    if got != row["expected"]:
        failures.append({"plain": row["segments"], "got": got})
for row in corpus["cascade"]:
    tainted = set()
    for e in row["entries"]:
        if "unlinked crate" in e["message"] or "unlinked crate" in e["label"]:
            tainted.update(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", e["sourceLine"]))
    kept = []
    for index, e in enumerate(row["entries"]):
        if e["code"] != "E0432" or not e["message"].startswith("unresolved import"):
            kept.append(index)
            continue
        paths = [p.split("::") for p in re.findall(r"`([^`]+)`", e["message"])]
        def plain(segments):
            return "::".join(s for s in segments if s != "" and s != "crate" and not s.startswith("__c_"))
        if not paths or not all(p[-1] in tainted or plain(p[:-1]) in row["globModules"] for p in paths):
            kept.append(index)
    if kept != row["kept"]:
        failures.append({"cascade": row["name"], "got": kept})
