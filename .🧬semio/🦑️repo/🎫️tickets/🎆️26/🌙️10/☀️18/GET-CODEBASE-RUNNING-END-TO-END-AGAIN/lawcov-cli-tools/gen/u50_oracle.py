import posixpath

for row in corpus["external"]:
    line = row["line"]
    bad = "unlinked crate" in line
    parts = line.split("\t")
    if len(parts) >= 3 and re.fullmatch(r"cannot find `[A-Za-z0-9_]+` in the crate root", parts[1]):
        bad = True
    if re.fullmatch(r"E0432", parts[0]) and len(parts) >= 2 and re.fullmatch(r"unresolved import `semio_[A-Za-z0-9_]+`", parts[1]):
        bad = True
    if parts[0] == "E0220" and len(parts) >= 2 and re.fullmatch(r"associated type `[A-Za-z0-9_]+` not found for `[A-Z][A-Za-z0-9_]*`", parts[1]):
        bad = True
    if (not bad) != row["expected"]:
        failures.append({"external": line})
for row in corpus["placeless"]:
    if row["defect"]:
        continue
    parts = row["line"].split("\t")
    got = row["line"]
    if len(parts) >= 2 and (parts[1].startswith("cannot find attribute ") or parts[1].startswith("cannot find derive macro ")):
        got = "\t".join(parts[:2])
    elif len(parts) >= 3 and re.match(r"cannot find (type|struct, variant or union type|value|function) `[^`]+` in this scope$", parts[1]):
        got = "\t".join(parts[:3])
    if got != row["expected"]:
        failures.append({"placeless": row["line"], "got": got})
for row in corpus["treeError"]:
    prefixes = ("an inner attribute is not permitted", "couldn't find file", "file not found for module", "circular modules", "couldn't read", "expected ", "unexpected ", "this file contains an unclosed", "unclosed delimiter", "mismatched closing delimiter")
    if row["message"].startswith(prefixes) != row["expected"]:
        failures.append({"treeError": row["message"]})
for row in corpus["mounted"]:
    got = [posixpath.normpath(posixpath.join(posixpath.dirname(row["file"]), p)) for p in re.findall(r'#\[path\s*=\s*"([^"]+\.rs)"\]', row["text"])]
    if got != row["expected"]:
        failures.append({"mounted": row["name"], "got": got})
for row in corpus["normalize"]:
    s = row["line"]
    s = re.sub(r"crate::__c_[a-z0-9_]+", "crate", s)
    s = re.sub(r"__c_[a-z0-9_]+(::)?", "", s)
    s = re.sub(r"<sandbox>(?:/\.\.)+", "<sandbox>/..", s)
    if s != row["expected"]:
        failures.append({"normalize": row["line"], "got": s})
for row in corpus["subjects"]:
    if row["defect"]:
        continue
    line = row["line"]
    got = []
    m = re.match(r"^E0433\tcannot find `([^`]+)` in `[^`]+`", line)
    if m:
        got = [m.group(1)]
    else:
        m = re.match(r"^E0(?:412|422|425|432|433)\t(?:unresolved imports? |cannot find (?:type|value|struct, variant or union type|function) )(.*)$", line)
        if m:
            subject = m.group(1).split(": ")[0].split(" in this scope")[0].split(" in module")[0]
            got = [x.split("::")[-1] for x in re.findall(r"`([^`]+)`", subject)]
    if got != row["expected"]:
        failures.append({"subjects": line, "got": got})
