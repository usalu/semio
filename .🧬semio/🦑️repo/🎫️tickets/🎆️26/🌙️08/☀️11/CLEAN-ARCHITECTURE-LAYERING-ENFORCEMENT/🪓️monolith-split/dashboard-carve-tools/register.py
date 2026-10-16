"""📇 Repository registrations of the dashboard carve: workspace members, owner rows, launch routes (both launch files). Usage: python3 register.py"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import spec
from lib import rd, wr

ROOT = "/Users/ueli/Documents/semio"
DASH_REL = "/".join([n.FW, n.PRODUCTS, n.REPO, n.MODULES, n.DASHBOARD])
ORDER = sys.argv[1:] or ["dashboard_core", "dashboard_launch", "dashboard_repo_domain", "dashboard_registry", "dashboard_inventory", "dashboard_wire", "dashboard_preferences", "dashboard_labels", "dashboard_daemon", "dashboard_terminal", "dashboard_cli"]
assert all(key in spec.CRATES for key in ORDER)

def pkg_rel(key):
    return "/".join([DASH_REL, *spec.CRATES[key][0], n.PKG, n.RUST])

# ------------------------------------------------------------ workspace members
CARGO = os.path.join(ROOT, "Cargo.toml")
text = rd(CARGO)
start = text.index("members = [\n") + len("members = [\n")
end = text.index("\n]", start) + 1
lines = text[start:end].split("\n")[:-1]
new_lines = [f"    \"{pkg_rel(key)}\"," for key in ORDER]
for line in new_lines:
    assert line not in lines, line
    at = next((i for i, existing in enumerate(lines) if existing.encode("utf-8") > line.encode("utf-8")), len(lines))
    lines.insert(at, line)
text = text[:start] + "\n".join(lines) + "\n" + text[end:]
wr(CARGO, text)
print("members inserted", len(new_lines))

# ------------------------------------------------------------ owner rows
OWNERS = os.path.join(ROOT, "📋️build-owners.json")
text = rd(OWNERS)
def row(key):
    root = pkg_rel(key)
    return ('    {\n' f'      "name": "{spec.project(key)}",\n' f'      "root": "{root}",\n' f'      "manifest": "{root}/{n.PROJECT}",\n' '      "packages": []\n    },\n')
BLOCK = re.compile(r'    \{\n      "name": "(@semio-tech/repo-dashboard-[a-z-]+)",\n.*?\n    \},\n', re.S)
for key in ORDER:
    name = spec.project(key)
    assert f'"name": "{name}"' not in text, name
    blocks = list(BLOCK.finditer(text))
    assert blocks
    later = next((m for m in blocks if m.group(1) > name), None)
    at = later.start() if later else blocks[-1].end()
    text = text[:at] + row(key) + text[at:]
wr(OWNERS, text)
json.loads(text)
print("owner rows inserted", len(ORDER))

# ------------------------------------------------------------ launch routes
FILES = [os.path.join(ROOT, ".vscode", "launch.json"), os.path.join(ROOT, ".vscode", "🧩️launch.seed.jsonc")]
ANCHOR = [c["name"] for c in json.loads(rd(FILES[0]))["configurations"] if isinstance(c, dict) and c["name"].startswith("🦀️ semio-framework-pets-staging ") and c["name"].endswith("release") and "test" in c["name"]][0]
def configs(path):
    return json.loads(rd(path))["configurations"]
used = set()
for path in FILES:
    for c in configs(path):
        if isinstance(c, dict) and isinstance(c.get("presentation"), dict) and "order" in c["presentation"]:
            used.add(round(c["presentation"]["order"], 6))
def allocate(base, count):
    out, k = [], 0
    while len(out) < count:
        k += 1
        if k % 10 == 0:
            continue
        value = round(base + k / 100000, 6)
        if value in used:
            continue
        out.append(value)
        used.add(value)
    return out
template_names = [c["name"] for c in configs(FILES[0]) if isinstance(c, dict) and c["name"].startswith("🦀️ semio-framework-pets-contract ")]
assert len(template_names) == 6, template_names
parts = [name.split(" ") for name in template_names]
CRAB_TOKEN = parts[0][0]
GROUPS = [("build", "4_build", 900.017, parts[0][2]), ("check", "4_gate", 922.048, parts[2][2]), ("test", "4_test", 903.588, parts[4][2])]
flavours = [("prototype", parts[0][3]), ("release", parts[1][3])]
assert [g[3] for g in GROUPS] == [n.e(0x1F3D7, "build"), n.e(0x1F50E, "check"), n.e(0x1F9EA, "test")], GROUPS
assert [f[1] for f in flavours] == [n.e(0x1F6E0, "prototype"), n.e(0x1F680, "release")], flavours
assert CRAB_TOKEN == n.CRAB
allocation = {}
for target, group, base, label in GROUPS:
    orders = allocate(base, len(ORDER) * 2)
    for i, key in enumerate(ORDER):
        for j, (flavour, flabel) in enumerate(flavours):
            allocation[(key, target, flavour)] = orders[2 * i + j]
def fmt(value):
    return repr(value)
def blocks():
    out = []
    for key in ORDER:
        package, project = spec.package(key), spec.project(key)
        for target, group, base, label in GROUPS:
            for flavour, flabel in flavours:
                out.append(
                    '    {\n'
                    f'      "name": "{CRAB_TOKEN} {package} {label} {flabel}",\n'
                    '      "type": "node-terminal",\n'
                    '      "request": "launch",\n'
                    f'      "command": "bun repo run {project}:{target} --configuration={flavour} --parallel=1 -- --lib --offline",\n'
                    '      "cwd": "${workspaceFolder}",\n'
                    '      "presentation": {\n'
                    f'        "group": "{group}",\n'
                    f'        "order": {fmt(allocation[(key, target, flavour)])}\n'
                    '      }\n'
                    '    },\n')
    return "".join(out)
new_text = blocks()
for path in FILES:
    text = rd(path)
    for key in ORDER:
        assert f"\"{CRAB_TOKEN} {spec.package(key)} " not in text, key
    at = text.index(f"\"name\": \"{ANCHOR}\"")
    close = text.index("\n    },\n", at) + len("\n    },\n")
    text = text[:close] + new_text + text[close:]
    wr(path, text)
    got = json.loads(text)["configurations"]
    names_ = [c["name"] for c in got if isinstance(c, dict)]
    assert all(f"{CRAB_TOKEN} {spec.package(k)} {lab} {fl}" in names_ for k in ORDER for _, _, _, lab in GROUPS for _, fl in flavours)
    print(os.path.basename(path), "routes inserted", len(ORDER) * 6, "total", len(got))
