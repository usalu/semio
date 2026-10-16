"""🏷️ Taxonomy names of the dashboard carve, spelled with explicit code points so the variation selector is never lost."""
import os

V = "️"
def e(cp, slug=""):
    return chr(cp) + V + slug

CRAB = e(0x1F980)
RS = CRAB + ".rs"
RUST = e(0x1F980, "rust")
PKG = e(0x1F4E6, "packages")
TESTS = e(0x1F9EA, "tests")
UNIT = e(0x1F52C, "unit")
PROJECT = e(0x1F4CB, "project.json")
TESTKIT = e(0x1F9F0, "testkit")
FEATURE = e(0x1F952, ".feature")
FW = e(0x1F9F0, "framework")
PRODUCTS = e(0x1F6CD, "products")
REPO = e(0x1F991, "repo")
MODULES = e(0x1F528, "modules")
DASHBOARD = e(0x1F39B, "dashboard")
LIBRARY = e(0x1F4DA, "library")

# existing dashboard folders
USAGE = e(0x2328, "usage")
PREFS = e(0x2699, "preferences")
KEYMAP = e(0x2328, "keymap")
DAEMON = e(0x1F300, "daemon")
IPC = e(0x2709, "ipc")
READY = e(0x1F7E2, "ready")
HTTP = e(0x1F4E1, "http")
REPLAY = e(0x1F4FC, "replay")
TRANSPORT = e(0x1F69A, "transport")
JOURNAL = e(0x1F4DC, "journal")
CLIENTS = e(0x1F465, "clients")
GROUPS = e(0x1F9E9, "groups")
SUPERVISOR = e(0x1F9E0, "supervisor")
CONTROL = e(0x1F579, "control")
CONNECTION = e(0x1F4CE, "connection")
COMMAND_TREE = e(0x1F333, "command-tree")
REGISTRY = e(0x1F3AE, "registry")
REPO_DOMAIN = e(0x1F3DB, "repo-domain")
PLAYGROUND = e(0x1F4C7, "playground-catalog")
INVENTORY = e(0x1F4DA, "inventory")
PLUGIN_REGISTRY = e(0x1F50C, "plugin-registry")
TERMINAL = e(0x1F5A5, "terminal")
LABELS = e(0x1F310, "labels")
LAUNCHER = e(0x1F680, "launcher")
WINDOWS = e(0x1FA9F, "windows")
PANES = e(0x1F4CB, "panes")
SESSIONS = e(0x1F4E1, "sessions")
CONTROLS = e(0x2328, "controls")
CLI = e(0x1F9ED, "cli")
ENTRY = e(0x1F6AA, "entrypoint")
INTEGRATION = e(0x1F9CA, "integration")

# new dashboard folders
CORE = e(0x1F9F1, "core")
LAUNCH = e(0x1F9FE, "launch")
WIRE = e(0x1F9F5, "wire")
LEASE = e(0x1F510, "lease")
LIFECYCLE = e(0x1F6A6, "lifecycle")
COMMAND = e(0x2328, "command")

def require(base, *parts):
    path = os.path.join(base, *parts)
    assert os.path.exists(path), path
    return path
