"""📄 Manifests and Nx projects of the dashboard carve, generated from the crate table (idempotent)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import spec
from lib import rd, wr

LOGICAL_ROOT = "/L"
LOGICAL_DASH = "/".join([LOGICAL_ROOT, n.FW, n.PRODUCTS, n.REPO, n.MODULES, n.DASHBOARD])
LOGICAL_MODULES = "/".join([LOGICAL_ROOT, n.FW, n.PRODUCTS, n.REPO, n.MODULES])
LOGICAL_FW_MODULES = "/".join([LOGICAL_ROOT, n.FW, n.e(0x1F528, "modules")])
UI_PKG = {
    "ui_locale": ("semio-framework-ui-locale", "/".join([LOGICAL_FW_MODULES, n.e(0x1F5B1, "ui"), n.e(0x1F310, "locale"), n.PKG, n.RUST]), None),
    "ui_tui": ("semio-framework-ui", "/".join([LOGICAL_FW_MODULES, n.e(0x1F5B1, "ui"), n.PKG, n.RUST]), ["tui-terminal"]),
    "ui_styling": ("semio-framework-ui-styling", "/".join([LOGICAL_FW_MODULES, n.e(0x1F5B1, "ui"), n.e(0x1F3A8, "styling"), n.PKG, n.RUST]), None),
}

def pkg_dir(key):
    return "/".join([LOGICAL_DASH, *spec.CRATES[key][0], n.PKG, n.RUST])

FACADE_DIR = "/".join([LOGICAL_DASH, n.PKG, n.RUST])

def rel(src, dst):
    return os.path.relpath(dst, src)

def dep_line(src, name, scratch=False):
    feature = None
    if name.endswith("+testkit"):
        name, feature = name[:-8], ["testkit"]
    if name in ("serde", "serde_json"):
        return f"{name} = {{ workspace = true }}"
    if name in UI_PKG:
        package, path, features = UI_PKG[name]
        feats = f", features = [{', '.join(repr(f).replace(chr(39), chr(34)) for f in features)}]" if features else ""
        return f"{name} = {{ path = \"{rel(src, path)}\"{feats}, package = \"{package}\" }}"
    if name in spec.CRATES:
        feats = ", features = [\"testkit\"]" if feature else ""
        return f"{name} = {{ path = \"{rel(src, pkg_dir(name))}\"{feats}, package = \"{spec.package(name)}\" }}"
    if name in spec.REPO_SIBLINGS:
        path = "/".join([LOGICAL_MODULES, spec.REPO_SIBLINGS[name], n.PKG, n.RUST])
        return f"{name} = {{ path = \"{rel(src, path)}\" }}"
    raise KeyError(name)

def manifest(key):
    node, ident, desc = spec.CRATES[key]
    src = pkg_dir(key)
    up = rel(src, LOGICAL_ROOT)
    normal, dev = spec.DEPS[key]
    lines = [
        "[package]",
        f"workspace = \"{up}\"",
        f"name = \"{spec.package(key)}\"",
        "version = \"0.1.0\"",
        "edition = \"2021\"",
        "rust-version.workspace = true",
        f"description = \"{desc}\"",
        "",
        "[package.metadata.semio]",
        "role = \"tool\"",
        f"id = \"repo-dashboard-{ident}\"",
        "",
        "[package.metadata.semio.rust-source-boundary]",
        "maxSourceBytes = 262144",
        "maxSourceFiles = 32",
        "maxActiveLines = 4000",
        "",
        "[lints]",
        "workspace = true",
        "",
        "[lib]",
        f"path = \"../../{n.RS}\"",
        "",
    ]
    if key == "dashboard_core":
        lines += ["[features]", "testkit = []", ""]
    lines.append("[dependencies]")
    lines += [dep_line(src, name) for name in normal]
    if dev:
        lines += ["", "[dev-dependencies]"] + [dep_line(src, name) for name in dev]
    return "\n".join(lines) + "\n"


def write_manifests(D):
    J = lambda *a: os.path.join(D, *a)
    for key in spec.CRATES:
        node = spec.CRATES[key][0]
        base = J(*node, n.PKG, n.RUST)
        wr(os.path.join(base, "Cargo.toml"), manifest(key))
        wr(os.path.join(base, n.PROJECT), json.dumps({"name": spec.project(key), "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + "\n")

    # facade manifest: the dependencies become the producer crates
    FACADE = J(n.PKG, n.RUST, "Cargo.toml")
    text = rd(FACADE)
    head = text[:text.index("[dependencies]")]
    deps = [dep_line(FACADE_DIR, name) for name in spec.FACADE_DEPS]
    wr(FACADE, head + "[dependencies]\n" + "\n".join(deps) + "\n")
