#!/usr/bin/env python3
"""📦 Installs authored unit tests (work/<slug>/{test.ts,corpus.json,schema.json,oracle.py}) into a unit's own 🧪tests directory, live or as a staged set, resolving `@LIB/`, `@UNIT/` and `@FW/` import macros to exact relative paths whose names carry the real emoji variation selectors.

Usage: lc.py live <slug> <unit>   copy into the unit (refuses to overwrite)
       lc.py stage <slug> <unit>  copy into staged/<slug>/ and record it in staged/manifest.json
       lc.py check <slug> <unit>  print the resolved destination and imports only
`<unit>` is the library-relative directory with or without variation selectors."""
import json, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
FE = "️"
FW = os.path.join(ROOT, "🧰" + FE + "framework")
LIB = os.path.join(FW, "🛍" + FE + "products", "🦑" + FE + "repo", "🔨" + FE + "modules", "📚" + FE + "library")
HERE = os.path.dirname(os.path.abspath(__file__))
TESTS = "🧪" + FE + "tests"
NAMES = {
    "test.ts": "🟦" + FE + ".ts",
    "corpus.json": "🔣" + FE + ".json",
    "schema.json": os.path.join("🧬" + FE + "schema", "🔣" + FE + ".json"),
    "oracle.py": os.path.join("🐍" + FE + "oracle", "🐍" + FE + ".py"),
}


TOKENS = {"@CORPUS": NAMES["corpus.json"], "@SCHEMA": NAMES["schema.json"], "@ORACLE": NAMES["oracle.py"]}


def strip(name):
    return name.replace(FE, "")


def resolve(base, norm, must=True):
    cur = base
    for part in [p for p in norm.split("/") if p]:
        found = None
        if os.path.isdir(cur):
            for entry in os.listdir(cur):
                if strip(entry) == strip(part):
                    found = entry
                    break
        if found is None:
            if must:
                raise SystemExit("unresolved " + part + " under " + cur)
            found = part
        cur = os.path.join(cur, found)
    return cur


def macros(text, tests_dir, unit_dir):
    def sub(match):
        kind, rest = match.group(1), match.group(2)
        base = {"LIB": LIB, "UNIT": unit_dir, "FW": FW}[kind]
        target = resolve(base, rest)
        path = os.path.relpath(target, tests_dir).replace(os.sep, "/")
        return path if path.startswith(".") else "./" + path
    text = re.sub(r"@(LIB|UNIT|FW)/([^\"'`\s]+)", sub, text)
    for token, name in TOKENS.items():
        text = text.replace(token, "./" + name.replace(os.sep, "/"))
    return text


def build_tree(src, dst, trie):
    os.makedirs(dst, exist_ok=True)
    for entry in os.listdir(src):
        link = os.path.join(dst, entry)
        if entry not in trie and not os.path.lexists(link):
            os.symlink(os.path.join(src, entry), link)
    for name, sub in trie.items():
        if sub is not None and os.path.isdir(os.path.join(src, name)):
            build_tree(os.path.join(src, name), os.path.join(dst, name), sub)


def ops_for(groups):
    rows = []
    for group in [g for g in groups.split(",") if g]:
        rows += json.load(open(os.path.join(HERE, "staged", "fixes", group + ".json"), encoding="utf8"))
    return rows


def seed_tests(over, real, mine):
    os.makedirs(over, exist_ok=True)
    if os.path.isdir(real):
        for entry in os.listdir(real):
            link = os.path.join(over, entry)
            if entry not in mine and not os.path.lexists(link):
                os.symlink(os.path.join(real, entry), link)


def overlay_dir(slug, unit_dir, groups=""):
    base = os.path.join("/private/tmp/lawcov-build", slug)
    top = os.path.join(base, os.path.basename(FW))
    rel = os.path.relpath(unit_dir, FW).split(os.sep)
    trie = {}
    cursor = trie
    for part in rel:
        cursor = cursor.setdefault(part, {})
    cursor[TESTS] = None
    rows = ops_for(groups)
    for row in rows:
        cursor = trie
        for part in os.path.relpath(os.path.join(LIB, row["file"]), FW).split(os.sep)[:-1]:
            cursor = cursor.setdefault(part, {})
        cursor[os.path.basename(row["file"])] = None
        for extra in row.get("also", []):
            cursor = trie
            for part in os.path.relpath(os.path.join(LIB, extra), FW).split(os.sep)[:-1]:
                cursor = cursor.setdefault(part, {})
            cursor[os.path.basename(extra)] = None
    build_tree(FW, top, trie)
    for row in rows:
        for extra in row.get("also", []):
            dest = os.path.join(top, os.path.relpath(os.path.join(LIB, extra), FW))
            if not os.path.exists(dest):
                shutil.copy(os.path.join(LIB, extra), dest)
    for row in rows:
        text = open(os.path.join(LIB, row["file"]), encoding="utf8").read()
        if row["marker"] in text:
            continue
        if text.count(row["old"]) != 1:
            raise SystemExit("patch anchor drift: " + row["file"])
        dest = os.path.join(top, os.path.relpath(os.path.join(LIB, row["file"]), FW))
        current = open(dest, encoding="utf8").read() if os.path.exists(dest) else text
        if row["marker"] not in current:
            open(dest, "w", encoding="utf8").write(current.replace(row["old"], row["new"]))
    node = os.path.join(base, "node_modules")
    if not os.path.lexists(node):
        os.symlink(os.path.join(ROOT, "node_modules"), node)
    return os.path.join(top, *rel, TESTS)


INSTALLED = os.path.join(HERE, "installed.json")


def digest(text):
    import hashlib
    return hashlib.sha256(text.encode("utf8")).hexdigest()


def load_installed():
    return json.load(open(INSTALLED, encoding="utf8")) if os.path.exists(INSTALLED) else {}


def main():
    action, slug, unit = sys.argv[1:4]
    unit_dir = resolve(LIB, unit)
    tests_dir = os.path.join(unit_dir, TESTS)
    work = os.path.join(HERE, "work", slug)
    names_file = os.path.join(work, "names.json")
    subdir, group, stage = "", "main", "base"
    if os.path.exists(names_file):
        for key, value in json.load(open(names_file, encoding="utf8")).items():
            if key == "subdir":
                subdir = value
            elif key == "group":
                group = value
            elif key == "stage":
                stage = value
            else:
                NAMES[key] = value.replace("/", os.sep)
        tests_dir = os.path.join(tests_dir, subdir) if subdir else tests_dir
        TOKENS.update({"@CORPUS": NAMES["corpus.json"], "@SCHEMA": NAMES["schema.json"], "@ORACLE": NAMES["oracle.py"]})
    plan = []
    for source, target in NAMES.items():
        src = os.path.join(work, source)
        if os.path.exists(src):
            plan.append((src, target))
    if action == "check":
        print(tests_dir)
        for src, target in plan:
            print(" ", target)
        return
    installed = load_installed()
    if action == "adopt":
        for src, target in plan:
            dest = os.path.join(tests_dir, target)
            if os.path.exists(dest):
                installed[os.path.relpath(dest, LIB)] = digest(open(dest, encoding="utf8").read())
        json.dump(installed, open(INSTALLED, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        return
    manifest_path = os.path.join(HERE, "staged", "manifest.json")
    manifest = json.load(open(manifest_path)) if os.path.exists(manifest_path) else {}
    for src, target in plan:
        text = open(src, encoding="utf8").read()
        if src.endswith(".ts") or src.endswith(".py"):
            text = macros(text, tests_dir, unit_dir)
        if action == "overlay":
            over = overlay_dir(slug, unit_dir, os.environ.get("LAWCOV_PATCH", ""))
            seed_tests(over, os.path.join(unit_dir, TESTS), {subdir.split("/")[0]} if subdir else {t.split(os.sep)[0] for _, t in plan})
            over = os.path.join(over, subdir) if subdir else over
            dest = os.path.join(over, target)
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            open(dest, "w", encoding="utf8").write(text)
            print("overlay", dest)
            if target == NAMES["test.ts"]:
                print("TEST", dest)
        elif action == "live":
            dest = os.path.join(tests_dir, target)
            key = os.path.relpath(dest, LIB)
            if os.path.exists(dest) and installed.get(key) != digest(open(dest, encoding="utf8").read()):
                raise SystemExit("refusing to overwrite " + dest)
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            open(dest, "w", encoding="utf8").write(text)
            installed[key] = digest(text)
            json.dump(installed, open(INSTALLED, "w", encoding="utf8"), ensure_ascii=False, indent=1)
            print("wrote", os.path.relpath(dest, ROOT))
            if target == NAMES["test.ts"]:
                print("TEST", dest)
        elif action == "stage":
            dest = os.path.join(HERE, "staged", slug, os.path.basename(src))
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            open(dest, "w", encoding="utf8").write(text)
            row = manifest.setdefault(slug, {"unit": unit, "files": {}, "group": group})
            row["group"] = group
            row["stage"] = stage
            row["files"][os.path.basename(src)] = os.path.relpath(os.path.join(tests_dir, target), LIB)
            if slug.endswith("-fixed"):
                row["overwrite"] = True
                row.setdefault("expect", {})[os.path.basename(src)] = installed.get(os.path.relpath(os.path.join(tests_dir, target), LIB))
            print("staged", dest)
    if action == "stage":
        json.dump(manifest, open(manifest_path, "w", encoding="utf8"), ensure_ascii=False, indent=1)


main()
