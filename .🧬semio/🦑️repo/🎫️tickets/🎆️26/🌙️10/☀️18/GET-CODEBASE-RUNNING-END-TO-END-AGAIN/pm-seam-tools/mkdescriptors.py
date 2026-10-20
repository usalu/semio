#!/usr/bin/env python3
"""🧱️ Writes the schema-first manager descriptors (one JSON per adapter) and the selection policy; the bun recipe is cut verbatim from the live install schema so key order matches the live hash input."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import paths
live = json.load(open(os.path.join(paths.ROOT, paths.DEPENDENCIES, "💾️storage/🧬️schema/📥️install.json"), encoding="utf-8"))
recipe_bun = live["$defs"]["ProducerRecipeProtocol"]["const"]
base = os.path.join(paths.NEW, paths.MANAGERS)
HOME = ["HOME", "USERPROFILE"]
common_keys = ["HOME", "USERPROFILE", "XDG_CONFIG_HOME", "XDG_CACHE_HOME", "TMPDIR", "TMP", "TEMP"]
isolation = {"HOME": "role:home", "USERPROFILE": "role:home", "XDG_CONFIG_HOME": "role:config", "XDG_CACHE_HOME": "role:cache", "TMPDIR": "role:temporary", "TMP": "role:temporary", "TEMP": "role:temporary"}
clear_keys = ["HOME", "USERPROFILE", "XDG_CONFIG_HOME", "XDG_CACHE_HOME", "TMPDIR", "TMP", "TEMP"]
kinds = {"clone": "clone", "clonefile": "clone", "copy": "copy", "copyfile": "copy", "hardlink": "alias", "symlink": "alias"}
def kinds_of(*names): return {name: kinds[name] for name in names}
def recipe(kind, extra, prefixes, keys=clear_keys, backends=None, version=1):
    value = {"version": version, "kind": kind, "backends": backends or {}}
    value.update(extra)
    value.update({"clearEnvironmentPrefixes": prefixes, "clearEnvironmentKeys": keys, "temporary": "tmp", "home": "home", "config": "config"})
    return value
def descriptor(id, ecosystem, toolchain, executable, runtime, manifests, identity, locks, configurations, environment, recipe_value, backend_kinds, operations, installed, resolution, generation, component):
    return {"version": 1, "id": id, "ecosystem": ecosystem, "toolchain": toolchain, "executable": executable, "runtime": runtime, "manifests": manifests,
            "manifestIdentity": identity, "locks": locks, "configurations": configurations, "environment": environment, "recipe": recipe_value,
            "backendKinds": backend_kinds, "operations": operations, "installed": installed, "resolution": resolution, "generationDirectory": generation, "graphComponent": component}
SEMVER = "^([0-9]+\\.[0-9]+\\.[0-9]+)"
items = {
 "🍞️bun": descriptor("bun", "javascript", "javascript", "bun",
   {"hostVersionKey": "bun", "probe": {"arguments": ["--version"], "pattern": SEMVER}},
   ["package.json"], {"excludedTopLevelFields": ["scripts", "semio"], "bootstrapFields": ["dependencies", "devDependencies", "optionalDependencies", "patchedDependencies"]},
   [{"name": "bun.lock", "role": "primary", "format": "bun-lock"}],
   {"scope": ["bunfig.toml"], "home": [".bunfig.toml", ".npmrc"], "config": [".bunfig.toml"]},
   {"set": {**isolation, "NODE_ENV": "literal:{nodeEnvironment}", "BUN_INSTALL_CACHE_DIR": "role:cache", "BUN_INSTALL_GLOBAL_DIR": "role:global", "BUN_INSTALL_BIN": "role:bin"}},
   recipe_bun, kinds_of("clonefile", "copyfile", "hardlink", "symlink"),
   {"lock": {"arguments": ["install", "--lockfile-only", "--{lifecycle}", "--linker={linker}", "--backend={backend}", "--cache-dir", "{cache}"]},
    "sync": {"arguments": ["install", "--frozen-lockfile", "--{lifecycle}", "--linker={linker}", "--backend={backend}", "--cache-dir", "{cache}"]},
    "verify": {"arguments": ["install", "--frozen-lockfile", "--dry-run", "--{lifecycle}", "--linker={linker}", "--backend={backend}", "--cache-dir", "{cache}"]}},
   {"containers": ["node_modules/.bun"], "entry": "{name}@{version}", "match": "listed", "nameTransform": "identity", "nameEscape": {"/": "+"}, "fold": "identity", "suffix": "^(\\+[0-9a-f]{16})?$", "sourcePrefix": None},
   {"profiles": ["bun-import", "bun-require"], "strictPrefix": "bun-"}, "j", "bun-lock"),
 "📦️npm": descriptor("npm", "javascript", "javascript", "npm",
   {"hostVersionKey": None, "probe": {"arguments": ["--version"], "pattern": SEMVER}},
   ["package.json"], {"excludedTopLevelFields": ["scripts", "semio"], "bootstrapFields": ["dependencies", "devDependencies", "optionalDependencies", "overrides"]},
   [{"name": "package-lock.json", "role": "primary", "format": "package-lock"}, {"name": "npm-shrinkwrap.json", "role": "secondary", "format": "package-lock"}],
   {"scope": [".npmrc"], "home": [".npmrc"], "config": []},
   {"set": {**isolation, "NODE_ENV": "literal:{nodeEnvironment}", "npm_config_cache": "role:cache", "npm_config_prefix": "role:global"}},
   recipe("javascript-install-projection", {"lifecycle": "ignore-scripts", "strategy": "hoisted", "cache": "npm-cache", "global": "npm-global", "bin": "npm-bin", "nodeEnvironment": "development"}, ["npm_config_", "NPM_CONFIG_"], clear_keys + ["NODE_ENV", "NODE_OPTIONS"]),
   {},
   {"lock": {"arguments": ["install", "--package-lock-only", "--{lifecycle}", "--install-strategy={strategy}", "--cache", "{cache}"]},
    "sync": {"arguments": ["ci", "--{lifecycle}", "--install-strategy={strategy}", "--cache", "{cache}"]},
    "verify": {"arguments": ["ci", "--dry-run", "--{lifecycle}", "--install-strategy={strategy}", "--cache", "{cache}"]}},
   {"containers": ["node_modules"], "entry": "{name}", "match": "exact", "nameTransform": "identity", "nameEscape": {}, "fold": "identity", "suffix": None, "sourcePrefix": None},
   {"profiles": ["node-import", "node-require"], "strictPrefix": None}, "j", None),
 "🦀️cargo": descriptor("cargo", "rust", "cargo", "cargo",
   {"hostVersionKey": None, "probe": {"arguments": ["--version"], "pattern": "^cargo (\\S+)"}},
   ["Cargo.toml"], {"excludedTopLevelFields": [], "bootstrapFields": ["dependencies", "dev-dependencies", "build-dependencies", "workspace"]},
   [{"name": "Cargo.lock", "role": "primary", "format": "cargo-lock"}],
   {"scope": [".cargo/config.toml", "rust-toolchain.toml"], "home": [".cargo/config.toml"], "config": []},
   {"set": {**isolation, "CARGO_HOME": "role:cache", "CARGO_TARGET_DIR": "role:temporary", "CARGO_NET_OFFLINE": "literal:{offline}"}},
   recipe("rust-install-projection", {"offline": "false", "cache": "cargo-home", "global": "cargo-global", "bin": "cargo-bin"}, ["CARGO_"]),
   {},
   {"lock": {"arguments": ["generate-lockfile", "--manifest-path", "{manifest}"]},
    "sync": {"arguments": ["fetch", "--locked", "--manifest-path", "{manifest}"]},
    "verify": {"arguments": ["metadata", "--locked", "--format-version", "1", "--manifest-path", "{manifest}"]}},
   {"containers": ["registry/src/*"], "entry": "{name}-{version}", "match": "exact", "nameTransform": "identity", "nameEscape": {}, "fold": "identity", "suffix": None, "sourcePrefix": "registry+"},
   {"profiles": [], "strictPrefix": None}, "r", None),
 "🐹️go": descriptor("go", "go", "go", "go",
   {"hostVersionKey": None, "probe": {"arguments": ["version"], "pattern": "^go version go(\\S+)"}},
   ["go.mod", "go.work"], {"excludedTopLevelFields": [], "bootstrapFields": []},
   [{"name": "go.sum", "role": "primary", "format": "go-sum"}, {"name": "go.work.sum", "role": "secondary", "format": "go-sum"}],
   {"scope": ["go.work"], "home": [], "config": ["go/env"]},
   {"set": {**isolation, "GOMODCACHE": "role:cache", "GOPATH": "role:global", "GOFLAGS": "literal:{flags}", "GOTOOLCHAIN": "literal:local", "GOPROXY": "literal:{proxy}"}},
   recipe("go-install-projection", {"flags": "-mod=readonly", "proxy": "https://proxy.golang.org,direct", "cache": "go-mod-cache", "global": "go-path", "bin": "go-bin"}, ["GO"], clear_keys + ["GOFLAGS"]),
   {},
   {"lock": {"arguments": ["mod", "tidy"]}, "sync": {"arguments": ["mod", "download"]}, "verify": {"arguments": ["mod", "verify"]}},
   {"containers": ["."], "entry": "{name}@{version}", "match": "exact", "nameTransform": "go-escape", "nameEscape": {}, "fold": "identity", "suffix": None, "sourcePrefix": None},
   {"profiles": [], "strictPrefix": None}, "g", None),
 "🐍️uv": descriptor("uv", "python", "python", "uv",
   {"hostVersionKey": None, "probe": {"arguments": ["--version"], "pattern": "^uv " + SEMVER[1:]}},
   ["pyproject.toml"], {"excludedTopLevelFields": [], "bootstrapFields": ["project", "dependency-groups", "tool"]},
   [{"name": "uv.lock", "role": "primary", "format": "uv-lock"}],
   {"scope": ["uv.toml", ".python-version"], "home": [], "config": ["uv/uv.toml"]},
   {"set": {**isolation, "UV_CACHE_DIR": "role:cache", "UV_PYTHON_INSTALL_DIR": "role:global", "UV_NO_PROGRESS": "literal:1"}},
   recipe("python-install-projection", {"cache": "uv-cache", "global": "uv-python", "bin": "uv-bin"}, ["UV_"], backends={"darwin": "clone", "linux": "copy", "win32": "copy"}),
   kinds_of("clone", "copy", "hardlink", "symlink"),
   {"lock": {"arguments": ["lock", "--cache-dir", "{cache}"]},
    "sync": {"arguments": ["sync", "--locked", "--link-mode={backend}", "--cache-dir", "{cache}"]},
    "verify": {"arguments": ["lock", "--check", "--cache-dir", "{cache}"]}},
   {"containers": [".venv/lib/*/site-packages", ".venv/Lib/site-packages"], "entry": "{name}-{version}.dist-info", "match": "listed", "nameTransform": "pep-escape", "nameEscape": {}, "fold": "pep-escape", "suffix": None, "sourcePrefix": None},
   {"profiles": [], "strictPrefix": None}, "p", None),
 "🔷️dotnet": descriptor("dotnet", "dotnet", "dotnet", "dotnet",
   {"hostVersionKey": None, "probe": {"arguments": ["--version"], "pattern": SEMVER}},
   ["Directory.Packages.props", "global.json"], {"excludedTopLevelFields": [], "bootstrapFields": []},
   [{"name": "packages.lock.json", "role": "primary", "format": "nuget-lock"}],
   {"scope": ["NuGet.Config", "nuget.config", "global.json"], "home": [".nuget/NuGet/NuGet.Config"], "config": []},
   {"set": {**isolation, "NUGET_PACKAGES": "role:cache", "DOTNET_CLI_HOME": "role:home", "DOTNET_CLI_TELEMETRY_OPTOUT": "literal:1", "DOTNET_NOLOGO": "literal:1"}},
   recipe("dotnet-install-projection", {"cache": "nuget-packages", "global": "dotnet-global", "bin": "dotnet-bin"}, ["DOTNET_", "NUGET_"]),
   {},
   {"lock": {"arguments": ["restore", "{manifest}", "--use-lock-file", "--packages", "{cache}"]},
    "sync": {"arguments": ["restore", "{manifest}", "--locked-mode", "--packages", "{cache}"]}},
   {"containers": ["."], "entry": "{name}/{version}", "match": "exact", "nameTransform": "lowercase", "nameEscape": {}, "fold": "identity", "suffix": None, "sourcePrefix": None},
   {"profiles": [], "strictPrefix": None}, "n", None),
}
def write(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2); handle.write("\n")
for folder, value in items.items():
    write(os.path.join(base, folder, "🔣️.json"), value)
write(os.path.join(base, "🔣️policy.json"), {"version": 1, "selection": {"javascript": "bun", "rust": "cargo", "go": "go", "python": "uv", "dotnet": "dotnet"}})
print(len(items), "descriptors")
