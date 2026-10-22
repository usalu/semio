#!/usr/bin/env python3
"""🐍️ Independent oracle of the package manager seam (standard library only, Python 3.8+): re-derives every vector of the contract from the descriptors and fixtures with its own code.

usage: oracle.py mint [OUT]     writes the vectors (default: the sibling 🔣️.json)
       oracle.py check [FILE]   recomputes and compares; exit 1 on any difference
Only this oracle writes expected values; the TypeScript suite and any other implementation only compare against them.
"""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
MANAGERS = os.path.dirname(HERE)
FOLDERS = {"bun": "🍞️bun", "npm": "📦️npm", "cargo": "🦀️cargo", "go": "🐹️go", "uv": "🐍️uv", "dotnet": "🔷️dotnet"}
PLATFORMS = ["darwin", "linux", "win32", "freebsd"]
PROBES = {"bun": "1.3.14\n", "npm": "11.16.0\n", "cargo": "cargo 1.99.0-nightly (3efb1f477 2026-07-17)\n", "go": "go version go1.24.1 darwin/arm64\n",
          "uv": "uv 0.11.15 (3cffe97c2 2026-05-18 aarch64-apple-darwin)\n", "dotnet": "10.0.300\n"}


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def compact(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def canonical(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False, sort_keys=True)


def sha(data):
    return hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()


def descriptors():
    return {name: load(os.path.join(MANAGERS, folder, "🔣️.json")) for name, folder in FOLDERS.items()}


def scalars(recipe):
    return {key: value for key, value in recipe.items() if isinstance(value, str)}


def expand(template, values):
    def one(match):
        if values.get(match.group(1)) is None:
            raise KeyError(match.group(1))
        return values[match.group(1)]
    return re.sub(r"\{([A-Za-z][A-Za-z0-9]*)\}", one, template)


def backend(d, platform):
    name = d["recipe"]["backends"].get(platform)
    if name is None:
        return None
    if d["backendKinds"].get(name, "alias") == "alias":
        raise ValueError("alias")
    return name


def render(d, operation, request):
    values = dict(scalars(d["recipe"]))
    values.update(request.get("settings") or {})
    values.update({"backend": backend(d, request["platform"]), "cache": request.get("cache"), "directory": request.get("directory"), "manifest": request.get("manifest")})
    try:
        arguments = [expand(t, values) for t in d["operations"][operation]["arguments"]]
    except KeyError as error:
        return {"error": "Complete package manager request required: " + error.args[0]}
    return {"executable": d["executable"], "arguments": arguments, "directory": request.get("directory")}


def environment(d, root, ambient, separator="/"):
    out, recipe = {}, d["recipe"]
    for name, value in ambient.items():
        upper = name.upper()
        if any(upper.startswith(p.upper()) for p in recipe["clearEnvironmentPrefixes"]) or upper in recipe["clearEnvironmentKeys"]:
            continue
        out[name] = value
    for name, spec in d["environment"]["set"].items():
        kind, _, rest = spec.partition(":")
        out[name] = root + separator + recipe[rest] if kind == "role" else expand(rest, scalars(recipe))
    return out


def runtime(d, versions, probe):
    key = d["runtime"]["hostVersionKey"]
    if key is not None and versions.get(key) is not None:
        return versions[key]
    if probe is None:
        return {"error": "Resolvable package manager runtime required: " + d["id"]}
    found = re.search(d["runtime"]["probe"]["pattern"], probe.strip())
    return found.group(1) if found and found.group(1) else {"error": "Resolvable package manager runtime required: " + d["id"]}


def generation_key(d, case):
    host = case["host"]
    ident = runtime(d, host["versions"], case.get("probe"))
    digest = hashlib.sha256(compact([d["recipe"], case["root"], ident, host["platform"], host["arch"], case["mode"], case["members"]]).encode("utf-8"))
    for row in case["material"]:
        data = row["text"].encode("utf-8")
        digest.update(compact([row["path"], len(data)]).encode("utf-8"))
        digest.update(data)
    return digest.hexdigest()


def bootstrap_key(d, case):
    host = case["host"]
    ident = runtime(d, host["versions"], case.get("probe"))
    fields = [case["manifest"].get(name) for name in d["manifestIdentity"]["bootstrapFields"]]
    digest = hashlib.sha256(compact(fields + [ident, host["platform"], host["arch"]]).encode("utf-8"))
    digest.update(case["lock"].encode("utf-8"))
    return digest.hexdigest()


def seal(body):
    return dict(body, sha256=sha(canonical(body)))


# lock readers: deliberately written differently from the TypeScript ones (regular expressions and line scans instead of tokenizers)
def strip_jsonc(text):
    text = text.lstrip("\ufeff")
    text = re.sub(r'(?m)^\s*//[^\n]*$', "", text)
    return re.sub(r',(\s*[}\]])', r'\1', text)


def rows_bun(text):
    lock = json.loads(strip_jsonc(text))
    if lock.get("lockfileVersion") not in (1, 2, 3):
        raise ValueError("version")
    members = [{"name": row.get("name", path), "path": "." if path == "" else path} for path, row in lock.get("workspaces", {}).items()]
    packages = []
    for key, row in lock.get("packages", {}).items():
        at = row[0].rindex("@")
        version = row[0][at + 1:]
        if re.match(r"(workspace|file|link|portal):", version):
            continue
        integrity = next((x for x in row if isinstance(x, str) and re.match(r"sha\d+-", x)), None)
        packages.append({"name": row[0][:at], "version": version, "key": key, "source": row[1] if isinstance(row[1], str) and row[1] != "" else None, "integrity": integrity})
    return packages, members


def rows_npm(text):
    lock = json.loads(text)
    members, packages = [], []
    for key, row in lock["packages"].items():
        if key == "":
            members.append({"name": row.get("name", lock.get("name", "")), "path": "."})
        elif row.get("link") is True:
            continue
        elif "node_modules/" not in key:
            members.append({"name": row.get("name", key.rsplit("/", 1)[-1]), "path": key})
        else:
            packages.append({"name": row.get("name", key[key.rindex("node_modules/") + 13:]), "version": row.get("version", ""), "key": key, "source": row.get("resolved"), "integrity": row.get("integrity")})
    return packages, members


def blocks(text):
    return [b for b in re.split(r"(?m)^\[\[package\]\]\s*$", text)[1:]]


def scalar(block, name):
    found = re.search(r'(?m)^%s = "([^"]*)"\s*$' % name, block)
    return found.group(1) if found else None


def rows_cargo(text):
    packages, members = [], []
    for block in blocks(text):
        name, version, source = scalar(block, "name"), scalar(block, "version"), scalar(block, "source")
        if source is None:
            members.append({"name": name, "path": None})
        else:
            packages.append({"name": name, "version": version, "key": name + "@" + version, "source": source, "integrity": scalar(block, "checksum")})
    return packages, members


def rows_go(text):
    rows = {}
    for line in text.splitlines():
        if not line.strip():
            continue
        module, version, hashed = line.split()
        mod_only = version.endswith("/go.mod")
        pure = version[:-7] if mod_only else version
        key = module + "@" + pure
        if mod_only and key in rows:
            continue
        rows[key] = {"name": module, "version": pure, "key": key, "source": "go.mod" if mod_only else "zip", "integrity": hashed}
    return list(rows.values()), []


def rows_uv(text):
    packages, members = [], []
    for block in blocks(text):
        name, version = scalar(block, "name"), scalar(block, "version") or ""
        source = re.search(r'(?m)^source = \{ (\w+) = "([^"]*)"', block)
        if source.group(1) in ("editable", "virtual", "directory", "path"):
            members.append({"name": name, "path": source.group(2)})
            continue
        sdist = re.search(r'(?m)^sdist = \{[^\n]*?hash = "([^"]+)"', block)
        wheel = re.search(r'wheels = \[[^\]]*?hash = "([^"]+)"', block, re.S)
        packages.append({"name": name, "version": version, "key": name + "@" + version, "source": source.group(1) + ":" + source.group(2),
                         "integrity": sdist.group(1) if sdist else wheel.group(1) if wheel else None})
    return packages, members


def rows_dotnet(text):
    lock = json.loads(text)
    members, packages = [], []
    for table in lock["dependencies"].values():
        for name, row in table.items():
            if row.get("type") == "Project":
                if not any(m["name"] == name for m in members):
                    members.append({"name": name, "path": None})
            else:
                packages.append({"name": name, "version": row.get("resolved", ""), "key": name, "source": row.get("type"), "integrity": row.get("contentHash")})
    return packages, members


READERS = {"bun-lock": rows_bun, "package-lock": rows_npm, "cargo-lock": rows_cargo, "go-sum": rows_go, "uv-lock": rows_uv, "nuget-lock": rows_dotnet}


def parse_lock(d, text):
    packages, members = READERS[d["locks"][0]["format"]](text)
    unique, seen = [], set()
    for row in packages:
        marker = canonical(row)
        if marker not in seen:
            seen.add(marker)
            unique.append(row)
    unique.sort(key=lambda r: (r["name"], r["version"], r["key"]))
    members.sort(key=lambda r: (r["name"], r["path"] or ""))
    return {"version": 1, "format": d["locks"][0]["format"], "packages": unique, "members": members}


def fold(name, how):
    if how == "lowercase":
        return name.lower()
    if how == "go-escape":
        return re.sub(r"[A-Z]", lambda m: "!" + m.group(0).lower(), name)
    if how == "pep-escape":
        return re.sub(r"[-_.]+", "_", name.lower())
    return name


def expected_providers(d, graph):
    layout, rows = d["installed"], []
    for package in graph["packages"]:
        if package["version"] == "" or (layout["sourcePrefix"] is not None and not (package["source"] or "").startswith(layout["sourcePrefix"])):
            continue
        name = fold(package["name"], layout["nameTransform"])
        for source, target in layout["nameEscape"].items():
            name = name.replace(source, target)
        rows.append({"name": package["name"], "version": package["version"], "entry": layout["entry"].replace("{name}", name).replace("{version}", package["version"])})
    return rows


def simulate_walk(d, providers, listing):
    layout = d["installed"]
    names = {fold(n, layout["fold"]) for n in listing}
    if layout["suffix"] is not None:
        tail = re.compile(layout["suffix"])
        for n in listing:
            for m in re.finditer(r"\+", n):
                if tail.search(n[m.start():]):
                    names.add(fold(n[:m.start()], layout["fold"]))
    missing = [p for p in providers if fold(p["entry"], layout["fold"]) not in names]
    return {"status": "match", "examined": len(providers)} if not missing else {"status": "mismatch", "examined": len(providers), "missing": missing[:64], "truncated": len(missing) > 64}


def fixtures():
    return load(os.path.join(HERE, "🧫️fixtures", "🔣️.json"))


def listing_for(d, providers):
    layout, names = d["installed"], []
    for index, provider in enumerate(providers):
        entry = provider["entry"]
        if layout["match"] == "exact":
            names.append(entry)
        elif layout["suffix"] is not None and index % 2 == 1:
            names.append(entry + "+0123456789abcdef")
        elif layout["fold"] == "pep-escape":
            names.append(entry.upper() if index % 2 == 0 else entry)
        else:
            names.append(entry)
    return names


def mint():
    out = {"version": 1, "canonical": [], "managers": {}}
    for value in [{"b": 1, "a": [2, {"d": None, "c": "x"}]}, {"é": "ü", "A": True, "a": False}, [], {"k": "line\nbreak \"quoted\" \\"}]:
        out["canonical"].append({"value": value, "json": canonical(value), "sha256": sha(canonical(value))})
    all_fixtures = fixtures()
    for name, d in descriptors().items():
        entry = {"descriptorSha256": sha(canonical(d)), "render": [], "backend": [], "environment": [], "runtime": [], "key": [], "bootstrapKey": [], "lock": [], "providers": [], "receipt": []}
        for operation in d["operations"]:
            for platform in PLATFORMS:
                request = {"platform": platform, "cache": "/work/cache", "directory": "/work/scope", "manifest": "/work/scope/" + d["manifests"][0]}
                entry["render"].append({"operation": operation, "request": request, "expected": render(d, operation, request)})
            partial = {"platform": "linux"}
            entry["render"].append({"operation": operation, "request": partial, "expected": render(d, operation, partial)})
        for platform in PLATFORMS:
            entry["backend"].append({"platform": platform, "expected": backend(d, platform)})
        recipe = d["recipe"]
        ambient = {"PATH": "/usr/bin", "KEEP_ME": "yes"}
        ambient.update({p + "EXAMPLE": "drop" for p in recipe["clearEnvironmentPrefixes"]})
        ambient.update({k: "drop" for k in recipe["clearEnvironmentKeys"]})
        entry["environment"].append({"root": "/store/root", "separator": "/", "ambient": ambient, "expected": environment(d, "/store/root", ambient)})
        entry["environment"].append({"root": "C:\\store", "separator": "\\", "ambient": {"Path": "C:\\bin"}, "expected": environment(d, "C:\\store", {"Path": "C:\\bin"}, "\\")})
        embedded = {"versions": {d["runtime"]["hostVersionKey"]: "9.9.9"}, "probe": None} if d["runtime"]["hostVersionKey"] else None
        if embedded:
            entry["runtime"].append(dict(embedded, expected=runtime(d, embedded["versions"], None)))
        probed = {"versions": {}, "probe": PROBES[name]}
        entry["runtime"].append(dict(probed, expected=runtime(d, {}, PROBES[name])))
        entry["runtime"].append({"versions": {}, "probe": None, "expected": runtime(d, {}, None)})
        base = {"root": "/work/scope", "host": {"platform": "darwin", "arch": "arm64", "versions": {}}, "probe": PROBES[name], "mode": "locked", "members": [{"path": "w/0/" + d["manifests"][0]}],
                "material": [{"path": "/work/scope/" + d["manifests"][0], "text": "{\"name\":\"x\"}"}, {"path": "/work/scope/" + d["locks"][0]["name"], "text": "lock\u00e9"}]}
        for variant in [base, dict(base, mode="lock-rebuild"), dict(base, host=dict(base["host"], platform="linux")), dict(base, probe=PROBES[name].replace("1", "2", 1)), dict(base, material=base["material"][:1])]:
            entry["key"].append(dict(variant, expected=generation_key(d, variant)))
        if d["manifestIdentity"]["bootstrapFields"]:
            manifest = {field: {"left-pad": "1.3.0"} for field in d["manifestIdentity"]["bootstrapFields"][:2]}
            case = {"manifest": manifest, "host": base["host"], "probe": PROBES[name], "lock": "lock bytes\n"}
            entry["bootstrapKey"].append(dict(case, expected=bootstrap_key(d, case)))
        for fixture_name, fixture in all_fixtures.get(name, {}).items():
            graph = parse_lock(d, fixture["text"])
            providers = expected_providers(d, graph)
            listing = listing_for(d, providers)
            entry["lock"].append({"fixture": fixture_name, "expected": graph})
            walk = {"fixture": fixture_name, "providers": providers, "listing": listing, "expected": simulate_walk(d, providers, listing), "container": None}
            entry["providers"].append(walk)
            if providers:
                trimmed = listing[1:]
                entry["providers"].append({"fixture": fixture_name, "providers": providers, "listing": trimmed, "expected": simulate_walk(d, providers, trimmed), "container": None})
        body = {"version": 1, "purpose": "dependency-generation-receipt", "ecosystem": d["ecosystem"], "manager": {"id": d["id"], "runtime": "1.0.0"}, "scope": ".",
                "generation": {"key": "a" * 64, "location": d["generationDirectory"] + "/" + "a" * 20}, "lock": {"name": d["locks"][0]["name"], "sha256": "b" * 64}, "providers": {"examined": 3}}
        entry["receipt"].append({"body": body, "expected": seal(body)})
        out["managers"][name] = entry
    return out


def main():
    args = sys.argv[1:]
    default = os.path.join(HERE, "🔣️.json")
    if not args or args[0] not in ("mint", "check"):
        raise SystemExit(__doc__)
    path = args[1] if len(args) > 1 else default
    minted = mint()
    if args[0] == "mint":
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(minted, handle, ensure_ascii=False, indent=1)
            handle.write("\n")
        print("minted", path, os.path.getsize(path))
        return
    if load(path) != minted:
        print("MISMATCH between vectors and oracle")
        sys.exit(1)
    print("vectors equal the oracle")


main()
