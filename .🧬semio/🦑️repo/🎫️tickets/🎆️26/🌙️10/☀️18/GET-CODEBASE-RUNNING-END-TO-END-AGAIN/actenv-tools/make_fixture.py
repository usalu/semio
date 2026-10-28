"""🧫️ Writes the law fixture of the action environment channel: hand-authored environment rows with their hand-authored expectation, and the read inventory (inventory.py output) split into delivered and withheld names. usage: make_fixture.py INVENTORY_TXT OUT_JSON"""
import json, sys
WITHHELD = {
    "REPO_ROOT": ("Fallback root of the repository door; the issued SEMIO_WORKSPACE_ROOT wins.", "Ausweichwurzel der Repository-Tuer; das ausgestellte SEMIO_WORKSPACE_ROOT gewinnt."),
    "REPO_MCP_BIN": ("Helper of the installed-frontend tests, which run outside actions.", "Hilfsvariable der Installiert-Frontend-Tests, die ausserhalb von Aktionen laufen."),
    "REPO_CLI_BIN": ("Helper of the installed-frontend tests, which run outside actions.", "Hilfsvariable der Installiert-Frontend-Tests, die ausserhalb von Aktionen laufen."),
    "OUT_DIR": ("Set by Cargo for every build script itself, never ambient.", "Wird von Cargo fuer jedes Build-Skript selbst gesetzt, nie aus der Umgebung."),
    "GITHUB_EVENT_NAME": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_OUTPUT": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_REF": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_EVENT_PATH": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_TOKEN": ("A credential; read by `repo ci` in the workflow process only.", "Eine Zugangsdate; wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_REPOSITORY": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_REPOSITORY_ID": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_RUN_ID": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "GITHUB_SHA": ("Read by `repo ci` in the workflow process only.", "Wird nur vom `repo ci` im Workflow-Prozess gelesen."),
    "WASM_BINDGEN_VERSION": ("A consistency check against Cargo.lock; the lock is authoritative, an ambient value can only make a build fail.", "Eine Konsistenzpruefung gegen Cargo.lock; die Sperrdatei ist massgeblich, ein Umgebungswert kann einen Build nur scheitern lassen."),
    "COMPOSE_SERVER_ADDR": ("Compose client address and credential of a Go command line tool that runs outside actions.", "Compose-Client-Adresse und Zugangsdaten eines Go-Kommandozeilenwerkzeugs, das ausserhalb von Aktionen laeuft."),
    "COMPOSE_SERVER_TOKEN": ("Compose client address and credential of a Go command line tool that runs outside actions.", "Compose-Client-Adresse und Zugangsdaten eines Go-Kommandozeilenwerkzeugs, das ausserhalb von Aktionen laeuft."),
    "PLAYGROUND_LOCKED_EXAMPLE_ID": ("A variant parameter that is an undeclared input of a cached task; a task that needs it declares it as an `env` input.", "Ein Varianten-Parameter, der eine nicht deklarierte Eingabe einer gecachten Aufgabe ist; eine Aufgabe, die ihn braucht, deklariert ihn als `env`-Eingabe."),
    "PLAYGROUND_APP_KIND": ("A variant parameter that is an undeclared input of a cached task; a task that needs it declares it as an `env` input.", "Ein Varianten-Parameter, der eine nicht deklarierte Eingabe einer gecachten Aufgabe ist; eine Aufgabe, die ihn braucht, deklariert ihn als `env`-Eingabe."),
    "VSCODE_INSPECTOR_OPTIONS": ("An editor debugger switch; not a build input.", "Ein Debugger-Schalter des Editors; keine Build-Eingabe."),
    "CACHE_PROBE_MODE": ("Read by the parent-side cache probe, not by an action.", "Wird von der Cache-Sonde im Elternprozess gelesen, nicht von einer Aktion."),
    "TEST_SHARD_STATUS_FILE": ("Supplied by the Bazel test runner into the action birth process.", "Wird vom Bazel-Testlaeufer in den Geburtsprozess der Aktion gegeben."),
    "TEST_SHARD_INDEX": ("Supplied by the Bazel test runner into the action birth process.", "Wird vom Bazel-Testlaeufer in den Geburtsprozess der Aktion gegeben."),
    "TEST_TOTAL_SHARDS": ("Supplied by the Bazel test runner into the action birth process.", "Wird vom Bazel-Testlaeufer in den Geburtsprozess der Aktion gegeben."),
    "XML_OUTPUT_FILE": ("Supplied by the Bazel test runner into the action birth process.", "Wird vom Bazel-Testlaeufer in den Geburtsprozess der Aktion gegeben."),
    "F3_GROUP_NAMES": ("A switch of a hand-run authoring tool.", "Ein Schalter eines von Hand gestarteten Authoring-Werkzeugs."),
    "F3_SLICES": ("A switch of a hand-run authoring tool.", "Ein Schalter eines von Hand gestarteten Authoring-Werkzeugs."),
}
names = [line.split()[0] for line in open(sys.argv[1], encoding="utf8") if line.strip()]
HOSTILE = {"NEO4J_PASSWORD": "pw", "NEO4J_URI": "bolt://x", "NEO4J_USERNAME": "u", "NEO4J_DATABASE": "d", "SSH_AUTH_SOCK": "/tmp/agent.sock", "GITHUB_TOKEN": "t", "NPM_TOKEN": "t", "AWS_SECRET_ACCESS_KEY": "k", "AWS_ACCESS_KEY_ID": "k", "ANTHROPIC_API_KEY": "k", "CLAUDE_CODE_SESSION_ID": "s", "CLAUDECODE": "1", "XPC_FLAGS": "0x0", "XPC_SERVICE_NAME": "0", "__CFBundleIdentifier": "app", "TERM_SESSION_ID": "w0t0p0", "SECURITYSESSIONID": "q", "LaunchInstanceID": "l", "DISPLAY": ":0", "WAYLAND_DISPLAY": "wayland-0", "XDG_SESSION_ID": "1", "XDG_SESSION_TYPE": "x11", "DBUS_SESSION_BUS_ADDRESS": "unix:path=/run/user/1000/bus", "COMMAND_MODE": "unix2003", "OSLogRateLimit": "64", "MallocNanoZone": "0", "MCP_CONNECTION_NONBLOCKING": "true", "API_TIMEOUT_MS": "600000", "BAGGAGE": "b", "DISABLE_AUTOUPDATER": "1", "GIT_EDITOR": "true", "MANPATH": "/usr/share/man", "AI_AGENT": "claude", "USE_LOCAL_OAUTH": "1", "COREPACK_ENABLE_AUTO_PIN": "0", "SHLVL": "1", "PWD": "/x", "_": "/usr/bin/bun", "HTTPS_PROXY": "http://u:p@proxy", "NODE_AUTH_TOKEN": "t", "DATABASE_URL": "postgres://u:p@h/db", "npm_config__authToken": "t", "npm_config_registry": "https://r"}
ESSENTIAL = {"PATH": "/usr/bin", "HOME": "/home/dev", "USER": "dev", "LOGNAME": "dev", "SHELL": "/bin/zsh", "TMPDIR": "/tmp/x", "TMP": "/tmp/x", "TEMP": "/tmp/x", "LANG": "en_US.UTF-8", "LC_ALL": "en_US.UTF-8", "TERM": "xterm-256color", "CI": "true", "DEVCONTAINER": "true"}
OWN = {"SEMIO_BUILD_GENERATION_PIN": "pin", "SEMIO_HOST_CAPACITY_GRANT": "grant", "SEMIO_HOST_CAPACITY_PLAN": "plan", "SEMIO_SCRIPT_PROCESS_INVOCATION": "{}", "SEMIO_BAZEL_CONTROL_URL": "tcp://127.0.0.1:1/x", "SEMIO_BAZEL_WORKER_PLAN": "{}", "SEMIO_BAZEL_SELECTION_CHAIN": "a", "SEMIO_BUILD_MODE": "release", "SEMIO_LANGUAGE": "de", "SEMIO_AGENT_SESSION": "agent", "SEMIO_WORKSPACE_ROOT": "/w", "SEMIO_BUILD_TOOL": "bazel", "NX_DAEMON": "false"}
TOOLS = {"RUSTUP_TOOLCHAIN": "x", "RUSTUP_HOME": "/r", "RUSTFLAGS": "-C", "CARGO_ENCODED_RUSTFLAGS": "x", "CARGO_HOME": "/c", "CARGO_NET_OFFLINE": "true", "CARGO_TARGET_DIR": "/t", "RUSTC_WRAPPER": "w", "CC": "cc", "CXX": "c++", "CFLAGS": "-O2", "SDKROOT": "/sdk", "MACOSX_DEPLOYMENT_TARGET": "14.0", "CPATH": "/inc", "LIBRARY_PATH": "/lib", "DEVELOPER_DIR": "/d", "GOROOT": "/go", "GOFLAGS": "-mod=readonly", "PLAYWRIGHT_BROWSERS_PATH": "/pw", "BUN_INSTALL": "/b", "BUN_RUNTIME_TRANSPILER_CACHE_PATH": "/b/c", "NODE": "/n", "NODE_OPTIONS": "--x", "NODE_USE_SYSTEM_CA": "1", "PYTHONDONTWRITEBYTECODE": "1", "DOTNET_ROOT": "/dn", "XDG_CACHE_HOME": "/xc", "GIT_CONFIG_GLOBAL": "/g", "VITE_S_HUB_URL": "u", "SSL_CERT_FILE": "/c.pem", "WGPU_BACKEND": "metal", "SQLITE3_LIB_DIR": "/s", "LIBZ_SYS_STATIC": "1", "npm_execpath": "/b/bun", "npm_node_execpath": "/b/node", "npm_command": "run-script", "npm_config_user_agent": "bun/1"}
SECRETISH = {"CARGO_REGISTRY_TOKEN": "t", "CARGO_REGISTRIES_CRATES_IO_TOKEN": "t", "BUN_AUTH_TOKEN": "t", "SEMIO_SERVICE_TOKEN": "t", "SEMIO_REMOTE_CACHE_TOKEN": "t", "SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT": "T", "SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN": "t", "SEMIO_REMOTE_CACHE_SERVER_TLS_KEY": "k", "SEMIO_REMOTE_EXECUTION_WORKER_TOKEN": "t", "SEMIO_HUB_LIVE_PASSWORD": "p", "SEMIO_TWO_HUMAN_USER1_PASSWORD": "p", "NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN": "t", "SEMIO_TEST_SECRET": "s"}
def row(name, environment, kept, declared=()): return {"name": name, "environment": environment, "declared": list(declared), "kept": sorted(kept)}
envs = [
    row("a shell full of desktop, agent, session and secret variables keeps only the host essentials", {**ESSENTIAL, **HOSTILE}, ESSENTIAL),
    row("the repository's own variables of a nested repo invocation all survive", {**ESSENTIAL, **OWN, **HOSTILE}, {**ESSENTIAL, **OWN}),
    row("toolchain variables of cargo, native tools, Go, Bun, Node, Python, .NET and git survive", {**ESSENTIAL, **TOOLS, **HOSTILE}, {**ESSENTIAL, **TOOLS}),
    row("a credential-like name is withheld inside every allowed class", {**ESSENTIAL, **SECRETISH, "CARGO_HOME": "/c", "SEMIO_TEST_ARTIFACT_DIR": "/a"}, {**ESSENTIAL, "CARGO_HOME": 0, "SEMIO_TEST_ARTIFACT_DIR": 0}),
    row("a declared env input is delivered, a credential-like declared one too, an undeclared sibling is not", {**ESSENTIAL, "FIXTURE_DECLARED": "1", "Proctor_Url": "u", "FIXTURE_DECLARED_TOKEN": "t", "FIXTURE_UNDECLARED": "1", "FIXTURE_UNDECLARED_TOKEN": "t"}, {**ESSENTIAL, "FIXTURE_DECLARED": 0, "Proctor_Url": 0, "FIXTURE_DECLARED_TOKEN": 0}, ["FIXTURE_DECLARED", "PROCTOR_URL", "fixture_declared_token"]),
    row("Windows names compare without case and a mixed-case secret is still withheld", {"Path": "p", "SystemRoot": "C:\\Windows", "ProgramFiles(x86)": "x", "LocalAppData": "l", "ComSpec": "c", "PATHEXT": ".EXE", "Neo4j_Password": "p", "Github_Token": "t", "VSCMD_ARG_TGT_ARCH": "x64"}, ["Path", "SystemRoot", "ProgramFiles(x86)", "LocalAppData", "ComSpec", "PATHEXT", "VSCMD_ARG_TGT_ARCH"]),
]
for r in envs: r["kept"] = sorted(r["kept"])
withheld = [{"name": n, "reason": {"en": WITHHELD[n][0], "de": WITHHELD[n][1]}} for n in sorted(WITHHELD)]
missing = [n for n in WITHHELD if n not in names]
if missing: sys.exit("withheld names not in the inventory: " + ",".join(missing))
fixture = {
    "description": "Environments with hand-authored expectations, and the environment names the repository's own code reads (inventory.py over the library and process trees, 2026-10-27): `delivered` must reach an action, `withheld` must not (each with its reason in English and German).",
    "wrappers": [".", "../../📥️caller", "../../⚙️operations/🌱️exec-environment"],
    "environments": envs,
    "reads": {"delivered": sorted(n for n in names if n not in WITHHELD), "withheld": withheld},
}
json.dump(fixture, open(sys.argv[2], "w", encoding="utf8"), ensure_ascii=False, indent=1)
