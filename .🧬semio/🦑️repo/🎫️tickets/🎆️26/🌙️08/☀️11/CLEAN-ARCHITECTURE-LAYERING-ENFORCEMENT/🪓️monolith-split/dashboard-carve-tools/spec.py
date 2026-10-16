"""📐 The crate table of the dashboard carve: node folders, package names, dependencies."""
import names as n

PREFIX = "semio-framework-repo-dashboard-"
UI = n.e(0x1F5B1, "ui")  # 🖱️ui
UI_LOCALE = n.e(0x1F310, "locale")
UI_STYLING = n.e(0x1F3A8, "styling")

# key -> (node path parts below the dashboard folder, id slug, description)
CRATES = {
    "dashboard_core": ([n.CORE], "core", "repo dashboard core: the invocation split into verb, segments and flags, the plugin and playground catalog, the dev environment of a playground session and the content hash every persistent name derives from"),
    "dashboard_launch": ([n.LAUNCH], "launch", "repo dashboard launch contract: the processes a resolved launch starts, how a group stops, what a running task is called and the address its output announces"),
    "dashboard_repo_domain": ([n.REPO_DOMAIN], "repo-domain", "repo dashboard repo domain: tickets, goals, analysis, trees and statutes answered in process by the Rust domain crates"),
    "dashboard_registry": ([n.REGISTRY], "registry", "repo dashboard command registry: every runnable thing of the workspace as one entry, the inventory that discovers them and the command tree that projects them"),
    "dashboard_inventory": ([n.INVENTORY], "inventory", "repo dashboard inventory: reads the sources of a workspace into a snapshot, discovers the command registry from them in the background and projects it as launcher rows and wizard steps"),
    "dashboard_wire": ([n.DAEMON, n.WIRE], "wire", "repo dashboard daemon wire: frames, workspace paths, transport, readiness, replay, journal, lease and the connection a view holds"),
    "dashboard_labels": ([n.TERMINAL, n.LABELS], "labels", "repo dashboard labels: every string the native dashboard shows, in every language, in one compile-checked catalogue"),
    "dashboard_preferences": ([n.PREFS], "preferences", "repo dashboard preferences: the layered preference files and the declarative keymap"),
    "dashboard_daemon": ([n.DAEMON], "daemon", "repo dashboard daemon: the supervisor of a workspace, the views it serves, the control surface without a terminal and the lifecycle"),
    "dashboard_terminal": ([n.TERMINAL], "terminal", "repo dashboard terminal view: the launcher, the managed windows named after their tasks and the keymap-driven controls"),
    "dashboard_cli": ([n.CLI], "cli", "repo dashboard command line: commands, run, tasks, logs, stop, restart, kill and open without a terminal view"),
}

SERDE = ("serde", "workspace")
SERDE_JSON = ("serde_json", "workspace")
REPO_SIBLINGS = {
    "semio-framework-repo-workspace": n.e(0x1F3E0, "workspace"),
    "semio-framework-repo-model": n.e(0x1F4D0, "model"),
    "semio-framework-repo-events": n.e(0x1F4E1, "events"),
    "semio-framework-repo-providers": n.e(0x1F9E9, "providers"),
    "semio-framework-repo-codebase": n.e(0x1F5C2, "codebase"),
    "semio-framework-repo-statutes": n.e(0x1F4DC, "statutes"),
    "semio-framework-repo-tree": n.e(0x1F333, "tree"),
    "semio-framework-repo-tickets": n.e(0x1F3AB, "tickets"),
    "semio-framework-repo-goals": n.e(0x1F3AF, "goals"),
}
UI_DEPS = {"ui_locale": "ui_locale", "ui_tui": "ui_tui", "ui_styling": "ui_styling"}

# normal dependencies and dev dependencies per crate (dashboard crates by key, externals by name)
DEPS = {
    "dashboard_core": (["serde_json"], []),
    "dashboard_launch": (["serde"], []),
    "dashboard_repo_domain": (["dashboard_core", "serde", "serde_json", "semio-framework-repo-codebase", "semio-framework-repo-events", "semio-framework-repo-goals", "semio-framework-repo-model", "semio-framework-repo-providers", "semio-framework-repo-statutes", "semio-framework-repo-tickets", "semio-framework-repo-tree"], ["dashboard_core+testkit"]),
    "dashboard_registry": (["dashboard_core", "dashboard_launch", "dashboard_repo_domain", "serde", "serde_json", "ui_locale", "ui_tui"], []),
    "dashboard_inventory": (["dashboard_core", "dashboard_registry", "dashboard_repo_domain", "serde", "serde_json", "semio-framework-repo-tickets"], ["dashboard_launch"]),
    "dashboard_wire": (["dashboard_core", "dashboard_launch", "serde", "serde_json", "ui_tui"], []),
    "dashboard_labels": (["dashboard_launch", "dashboard_wire", "ui_locale", "ui_tui"], ["dashboard_core+testkit", "dashboard_preferences"]),
    "dashboard_preferences": (["dashboard_core", "dashboard_registry", "serde", "serde_json", "ui_locale"], ["dashboard_core+testkit"]),
    "dashboard_daemon": (["dashboard_labels", "dashboard_wire", "ui_tui"], ["dashboard_core+testkit", "dashboard_launch", "serde_json"]),
    "dashboard_terminal": (["dashboard_core", "dashboard_daemon", "dashboard_inventory", "dashboard_labels", "dashboard_launch", "dashboard_preferences", "dashboard_registry", "dashboard_wire", "ui_locale", "ui_styling", "ui_tui"], ["dashboard_core+testkit", "serde_json"]),
    "dashboard_cli": (["dashboard_daemon", "dashboard_inventory", "dashboard_registry", "dashboard_repo_domain", "dashboard_wire", "serde_json"], ["dashboard_core+testkit", "dashboard_launch"]),
}
FACADE_DEPS = ["dashboard_cli", "dashboard_core", "dashboard_daemon", "dashboard_inventory", "dashboard_labels", "dashboard_preferences", "dashboard_repo_domain", "dashboard_terminal", "dashboard_wire", "serde_json", "semio-framework-repo-workspace"]
FACADE_DEV = ["dashboard_core+testkit"]

def package(key):
    return PREFIX + CRATES[key][1]

def project(key):
    return "@semio-tech/repo-dashboard-" + CRATES[key][1] + "-rs"
