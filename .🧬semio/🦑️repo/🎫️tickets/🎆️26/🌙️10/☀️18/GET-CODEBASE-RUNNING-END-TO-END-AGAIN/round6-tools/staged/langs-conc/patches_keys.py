# 🔑 Nx keys: python, dotnet, cmake and go projects hash the pin of their toolchain (one spawn-free fingerprint command, the same contract Bazel digests), the workspace project hashes neither the Python nor the CMake files of the repository root, and the uv named input no longer carries the root lock.
import json, os
from layout import *
from patchlib import chain

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.environ.get("LC_STAGED") or os.path.join(HERE, "staged")
POLICY = CACHING + "/🔣️policy.json"
INFERENCE = B + "/📥️inputs/🟨️.mjs"
BUILD_POLICY = B + "/🔣️policy.json"
NX = ROOT + "/nx.json"
STYLING_DOTNET = ROOT + "/🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🔷️dotnet/📋️project.json"
FINGERPRINT = 'bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts" fingerprint '

def toolchain_names(kind):
    return json.load(open(STAGED + "/toolchains-policy.json", encoding="utf8"))[kind]["environment"]

def listing(items, indent):
    return ",\n".join(indent + json.dumps(item, ensure_ascii=False) for item in items)

PYTHON_OLD = (
    '    "python": {\n      "files": [\n        "pyproject.toml",\n        "uv.lock",\n        "conftest.py",\n        ".python-version"\n      ],\n'
    '      "environment": [\n        "PYTHONPATH",\n        "PYTHONHASHSEED",\n        "UV_PYTHON"\n      ],\n'
    '      "commands": [\n        "uv --version",\n        "python --version"\n      ]\n    },\n'
)

def python_new():
    return ('    "python": {\n      "files": [],\n      "environment": [\n' + listing(toolchain_names("python"), "        ") + '\n      ],\n'
            '      "commands": [\n' + listing([FINGERPRINT + "python"], "        ") + '\n      ]\n    },\n')

ENVIRONMENTS = {
    "dotnet": ["DOTNET_ROLL_FORWARD", "Configuration", "RuntimeIdentifier"],
    "cmake": ["CC", "CXX", "CFLAGS", "CXXFLAGS", "LDFLAGS", "CMAKE_TOOLCHAIN_FILE", "CMAKE_GENERATOR"],
    "go": ["GOOS", "GOARCH", "CGO_ENABLED", "CGO_CFLAGS", "CGO_LDFLAGS", "GOFLAGS", "GOTOOLCHAIN"],
}
COMMANDS = {"dotnet": "dotnet --version", "cmake": "cmake --version", "go": "go version"}

def policy(text):
    edits = [("policy-python", PYTHON_OLD, python_new())]
    for kind in ("dotnet", "cmake", "go"):
        before = '      "environment": [\n' + listing(ENVIRONMENTS[kind], "        ") + '\n      ],\n      "commands": [\n        ' + json.dumps(COMMANDS[kind]) + '\n      ]\n'
        after = '      "environment": [\n' + listing(toolchain_names(kind), "        ") + '\n      ],\n      "commands": [\n        ' + json.dumps(FINGERPRINT + kind, ensure_ascii=False) + '\n      ]\n'
        edits.append(("policy-" + kind, before, after))
    return chain(text, edits)

def inference(text):
    return chain(text, [
        ("inference-python", '  if (existsSync(join(workspaceRoot, root, "pyproject.toml"))) tools.push("python");\n', '  const workspaceProject = root === "." || root === "";\n  if (!workspaceProject && existsSync(join(workspaceRoot, root, "pyproject.toml"))) tools.push("python");\n'),
        ("inference-cmake", '  if (existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");\n', '  if (!workspaceProject && existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");\n'),
        ("inference-commands", '    inputs.push(...contract.commands.map((runtime) => ({ runtime })));\n', '    inputs.push(...contract.commands.map((runtime) => ({ runtime: tool === "python" ? `${runtime} ${JSON.stringify(root)}` : runtime })));\n'),
        ("inference-uv", '  if (tools.includes("python")) inputs.push({ runtime: `uv run --locked --no-sync --project ${JSON.stringify(root)} python --version` });\n', ''),
    ])

def uv(text):
    return chain(text, [("uv-named-input", '      "{projectRoot}/pyproject.toml",\n      "{workspaceRoot}/uv.lock"\n', '      "{projectRoot}/pyproject.toml",\n      "{projectRoot}/uv.lock"\n')])

def styling_dotnet(text):
    return chain(text, [("styling-dotnet-runtime", '        { "runtime": "dotnet --version" },\n', '')])

def edits():
    return [(POLICY, policy), (INFERENCE, inference), (BUILD_POLICY, uv), (NX, uv), (STYLING_DOTNET, styling_dotnet)]
