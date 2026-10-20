# 🧾️ Staged tax6 library patch: guarded read-modify-write edits of 🔣️taxonomy.json and anchored text edits of library modules. usage: python3 apply.py [--check] [--root <library root>] [--only <op id>]
import json, os, sys, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
args = sys.argv[1:]
check = "--check" in args
root = args[args.index("--root") + 1] if "--root" in args else os.path.join(REPO, LIB)
only = args[args.index("--only") + 1] if "--only" in args else None
ops = []

def op(name):
    def wrap(fn):
        ops.append((name, fn))
        return fn
    return wrap

def taxonomy_path():
    return os.path.join(root, "🔣️taxonomy.json")

def edit_taxonomy(mutate):
    path = taxonomy_path()
    before = open(path, encoding="utf8").read()
    digest = hashlib.sha256(before.encode()).hexdigest()
    schema = json.loads(before)
    changed = mutate(schema)
    if not changed:
        return "already applied"
    after = json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
    if check:
        return "would write"
    if hashlib.sha256(open(path, encoding="utf8").read().encode()).hexdigest() != digest:
        raise SystemExit("taxonomy changed under the edit; rerun")
    tmp = path + ".tax6"
    open(tmp, "w", encoding="utf8").write(after)
    os.replace(tmp, path)
    return "written"

def directory_entries(relative):
    base = os.path.join(REPO, relative)
    return sorted(name for name in os.listdir(base) if os.path.isfile(os.path.join(base, name)))

def exact_contract(path, authority, reason, verification):
    return {"pathPattern": path, "authority": authority, "reason": reason, "configurability": "unconfigurable", "scope": {"kind": "exact-path", "path": path}, "verification": verification, "expires": None}

def child(parent, slug):
    return next(name for name in sorted(os.listdir(parent)) if name == slug or name.endswith(slug) and not name[:-len(slug)].isascii())

def resolve(*slugs):
    current = REPO
    for slug in slugs:
        current = os.path.join(current, child(current, slug))
    return os.path.relpath(current, REPO)

def resolve_in(base, *slugs):
    current = base
    for slug in slugs:
        current = os.path.join(current, child(current, slug))
    return current

def edit_text(path, old, new):
    text = open(path, encoding="utf8").read()
    if new in text:
        return "already applied"
    if text.count(old) != 1:
        raise SystemExit(f"anchor not unique in {path}: {text.count(old)}")
    if check:
        return "would write"
    tmp = path + ".tax6"
    open(tmp, "w", encoding="utf8").write(text.replace(old, new))
    os.replace(tmp, path)
    return "written"

def copy_new(relative_from_new, destination):
    source = os.path.join(HERE, "new", relative_from_new)
    if os.path.exists(destination) and open(destination, "rb").read() == open(source, "rb").read():
        return "already applied"
    if check:
        return "would write"
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    open(destination, "wb").write(open(source, "rb").read())
    return "written"

@op("print-latex-package-contracts")
def print_latex(_):
    directory = resolve("framework", "products", "print", "latex")
    def mutate(schema):
        contracts = schema["fixedFilenameContracts"]
        added = {}
        for name in directory_entries(directory):
            stem, extension = os.path.splitext(name)
            contract_id = f"print-latex-{stem}" + ("-class" if extension == ".cls" else "")
            if contract_id in contracts:
                continue
            added[contract_id] = exact_contract(f"{directory}/{name}", "LaTeX package lookup", "The literal basename is the package identity declared by ProvidesPackage and resolved by RequirePackage", "@semio-tech/print:test-quick and tectonic template compilation")
        if not added:
            return False
        ordered = {}
        keys = list(contracts)
        last = max(i for i, k in enumerate(keys) if k.startswith("print-latex-"))
        for i, k in enumerate(keys):
            ordered[k] = contracts[k]
            if i == last:
                ordered.update(dict(sorted(added.items())))
        schema["fixedFilenameContracts"] = ordered
        return True
    return edit_taxonomy(mutate)

@op("unicode-ucd-source-contracts")
def unicode_sources(_):
    directory = resolve("framework", "modules", "2d", "text", "font", "shape", "unicode", "data", "sources")
    def mutate(schema):
        contracts = schema["fixedFilenameContracts"]
        added = {}
        for name in directory_entries(directory):
            stem = os.path.splitext(name)[0].lower().replace(".", "-")
            contract_id = f"unicode-ucd-{stem}"
            if contract_id in contracts:
                continue
            added[contract_id] = exact_contract(f"{directory}/{name}", "Unicode Consortium UCD", "The literal basename is the upstream Unicode Character Database file name recorded with url and sha256 in the sibling provenance", "unicode data tables generator digest check against the recorded sha256")
        if not added:
            return False
        keys = list(contracts)
        ordered = {}
        for k in keys:
            ordered[k] = contracts[k]
        ordered.update(dict(sorted(added.items())))
        schema["fixedFilenameContracts"] = ordered
        return True
    return edit_taxonomy(mutate)

@op("policy-document-directory-kind")
def policy_kind(_):
    def mutate(schema):
        kinds = schema["semanticDirectoryKinds"]
        if "policy-document" in kinds:
            return False
        reference = kinds["repo-test-policy"]["emoji"]
        kinds["policy-document"] = {"emoji": reference, "slugPattern": "^policy$", "allowEmojiOnly": False}
        return True
    return edit_taxonomy(mutate)

@op("package-implementation-destination-keeps-module-path")
def destination(_):
    path = os.path.join(resolve_in(root, "normalization", "fixed-contracts"), "🟦️.ts")
    old = """  const ownerCanonical = canonicalDirectoryByPath.get(location.owner) ?? location.owner.normalize("NFC");
  const fileName = basename(canonical.path);
  const stem = canonical.stem?.normalize("NFC") ?? "";
  if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und"))) return ownerCanonical ? `${ownerCanonical}/${fileName}` : fileName;
  if (canonical.semanticDirectoryName) return ownerCanonical ? `${ownerCanonical}/${canonical.semanticDirectoryName}/${fileName}` : `${canonical.semanticDirectoryName}/${fileName}`;
  const semantic = matchDirectoryKind(stem, taxonomy, directoryKindByPath.get(location.owner));
  if (!semantic.kind) return null;
  const directoryName = `${semantic.kind.emoji}${stem}`.normalize("NFC");
  return ownerCanonical ? `${ownerCanonical}/${directoryName}/${fileName}` : `${directoryName}/${fileName}`;
}"""
    new = """  const ownerCanonical = canonicalDirectoryByPath.get(location.owner) ?? location.owner.normalize("NFC");
  const packageCanonical = canonicalDirectoryByPath.get(location.packageRoot) ?? location.packageRoot.normalize("NFC");
  const fileName = basename(canonical.path);
  const relativePath = canonical.path.startsWith(`${packageCanonical}/`) ? canonical.path.slice(packageCanonical.length + 1) : fileName;
  const modulePrefix = relativePath === fileName ? "" : `${relativePath.slice(0, relativePath.length - fileName.length)}`;
  const at = (path: string): string => ownerCanonical ? `${ownerCanonical}/${path}` : path;
  const stem = canonical.stem?.normalize("NFC") ?? "";
  if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und")) || canonical.semanticDirectoryName) return at(relativePath);
  const semantic = matchDirectoryKind(stem, taxonomy, directoryKindByPath.get(location.owner));
  if (!semantic.kind) return null;
  const directoryName = `${semantic.kind.emoji}${stem}`.normalize("NFC");
  return at(`${modulePrefix}${directoryName}/${fileName}`);
}"""
    return edit_text(path, old, new)

def kind_file(directory, extension):
    return next(name for name in sorted(os.listdir(directory)) if name.endswith(extension) and len(name) <= 12)

def sibling_name(directory, existing_slug, new_slug):
    return child(directory, existing_slug).replace(existing_slug, new_slug)

@op("package-implementation-destination-test")
def destination_test(_):
    tests = resolve_in(root, "normalization", "tests")
    fixtures = os.path.join(tests, child(tests, "fixtures"))
    template = os.path.join(tests, child(tests, "package-boundary-classification"))
    suite = sibling_name(tests, "package-boundary-classification", "package-implementation-destination")
    fixture_suite = sibling_name(fixtures, "package-boundary-classification", "package-implementation-destination")
    test_name = kind_file(template, ".ts")
    json_name = kind_file(os.path.join(fixtures, child(fixtures, "package-boundary-classification")), ".json")
    return copy_new("destination/test.ts", os.path.join(tests, suite, test_name)) + " / " + copy_new("destination-fixture/fixture.json", os.path.join(fixtures, fixture_suite, json_name))

for name, fn in ops:
    if only and name != only:
        continue
    print(f"{name}: {fn(None)}")
