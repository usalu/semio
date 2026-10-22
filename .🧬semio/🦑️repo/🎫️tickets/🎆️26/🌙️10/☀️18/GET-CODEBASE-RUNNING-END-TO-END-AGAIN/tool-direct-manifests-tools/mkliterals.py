"""📝️ Writes literals.json: the anchored multi-line edits of the set (CI composites, the build catalogue and the tools command). Every `old` must occur exactly `count` times in the base; `requires` names text that must already be in the file (a set that has to land first)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
CAT = B + "/⚙️operations/🔣️catalog.json"
CMD = B + "/🧰️tool-registry/⌨️command/🟦️.ts"
VERIFY = L + "/🎛️commands/✅️verify/🟦️.ts"
rows = []
def add(id, group, path, old, new, count=1, requires=None, note=None, witness=None):
    rows.append({"id": id, "group": group, "path": path, "old": old, "new": new, "count": count, **({"requires": requires} if requires else {}), **({"note": note} if note else {}), **({"witness": witness} if witness else {})})

add("native-repo-input", "ci", ".github/actions/native-repo/action.yml",
    "inputs:\n  bun-version:\n    description: Repository Bun pin.\n    required: true\n  node-version:\n",
    "inputs:\n  node-version:\n", witness="inputs:\n  node-version:\n",
    note={"en": "The composite no longer takes a runtime pin: the launcher provisions it.", "de": "Das Composite nimmt keinen Laufzeit-Pin mehr: Der Launcher richtet sie ein."})
add("native-repo-launcher", "ci", ".github/actions/native-repo/action.yml",
    "    - uses: ./.github/actions/bun-repo\n      with:\n        bun-version: ${{ inputs.bun-version }}\n        node-version: ${{ inputs.node-version }}\n",
    "    - uses: ./.github/actions/repo-launcher\n      with:\n        node-version: ${{ inputs.node-version }}\n",
    note={"en": "The repo-launcher set deletes the bun-repo composite; this one still used it.", "de": "Das repo-launcher-Set löscht das bun-repo-Composite; dieses nutzte es noch."})
add("dashboard-bun-pin", "ci", ".github/workflows/dashboard.yml", "  BUN_VERSION: 1.4.2\n", "", witness="  NODE_VERSION: 24.15.0")
add("dashboard-bun-input", "ci", ".github/workflows/dashboard.yml", "          bun-version: ${{ env.BUN_VERSION }}\n", "", witness="          node-version: ${{ env.NODE_VERSION }}")
NODE_STEP = ("    - name: Check that the workflow Node pin equals the repository pin\n      if: ${{ inputs.node-version != '' }}\n      shell: bash\n      env:\n        NODE_VERSION: ${{ inputs.node-version }}\n      run: |\n")
NODE_OLD = NODE_STEP + "        node -e \"const p=require('./package.json');if(p.engines.node!==process.env.NODE_VERSION)throw Error('package.json pins node '+p.engines.node+', the workflow pins '+process.env.NODE_VERSION)\"\n"
NODE_NEW = NODE_STEP + "        repo tools exec --light node -- -e \"const p=require('./package.json');if(p.engines.node!==process.env.NODE_VERSION)throw Error('package.json pins node '+p.engines.node+', the workflow pins '+process.env.NODE_VERSION)\"\n"
LAST = "    - name: Print the launcher, runtime and implementation versions\n"
add("launcher-node-check-remove", "ci", ".github/actions/repo-launcher/action.yml", NODE_OLD, "", witness="repo tools exec --light node -- -e",
    note={"en": "The pin check asks the registry's Node, so it runs after the launcher is installed.", "de": "Die Pin-Prüfung fragt das Node der Registratur und läuft deshalb nach der Installation des Launchers."})
add("launcher-node-check-insert", "ci", ".github/actions/repo-launcher/action.yml", LAST, NODE_NEW + LAST)

BENCH_TEST = B + "/📏️incremental/🏎️bench/🧪️tests/🟦️.ts"
add("bench-workflow-test", "ci", BENCH_TEST,
    'for(const wanted of ["bun repo preflight","bun repo tools setup all","bun repo dependencies sync --all","bun repo bench list","bun repo incremental prepare --fresh",\'bun repo bench run "${args[@]}"\',',
    'for(const wanted of ["repo preflight","repo tools setup all","repo dependencies sync --all","repo bench list","repo incremental prepare --fresh",\'repo bench run "${args[@]}"\',',
    note={"en": "The bench job of the build workflow runs `repo ...` after the declarations are swept, so its test expects the same.", "de": "Der Bench-Job des Build-Workflows startet nach dem Abräumen der Deklarationen `repo ...`; sein Test erwartet dasselbe."})
EXEC_OLD = "synopsis\": \"tools exec [--cwd=<path>] [--light] <tool> -- <arguments>\", \"flags\": [\"--cwd\", \"--light\"],"
add("c1-synopsis-top", "catalogue-tools-exec", CAT,
    "\"synopsis\": \"tools setup [{backends}|toolchains|all]|verify [tool...]|list [--json]|exec <tool> -- <arguments>\"",
    "\"synopsis\": \"tools setup [{backends}|toolchains|all]|verify [tool...]|list [--json]|exec <tool|command> -- <arguments>\"", requires="tools exec [--cwd=<path>]")
add("c1-synopsis-exec", "catalogue-tools-exec", CAT, EXEC_OLD, EXEC_OLD.replace("<tool>", "<tool|command>"), requires="tools exec [--cwd=<path>]")
S_OLD_EN = "Run a registered tool with the sanitized environment of the registry, under machine capacity unless it is light, with progress and cancellation."
S_OLD_DE = "Führt ein registriertes Werkzeug mit der bereinigten Umgebung der Registratur aus, unter Rechnerkapazität, sofern es nicht leicht ist, mit Fortschritt und Abbruch."
add("c1-summary", "catalogue-tools-exec", CAT,
    "\"summary\": { \"en\": \"%s\", \"de\": \"%s\" }" % (S_OLD_EN, S_OLD_DE),
    "\"summary\": { \"en\": \"%s <tool> is a registry id (cargo, node, uv) or an executable a registered tool owns (rustc, rustfmt, python3, bunx).\", \"de\": \"%s <tool> ist eine Id der Registratur (cargo, node, uv) oder ein Programm, das ein registriertes Werkzeug besitzt (rustc, rustfmt, python3, bunx).\" }" % (S_OLD_EN, S_OLD_DE), requires="tools exec [--cwd=<path>]")
CARGO_END = "\"de\": \"Führt Cargo über die Registratur aus, statt es direkt aufzurufen.\" } }\n      ],"
add("c1-examples", "catalogue-tools-exec", CAT, CARGO_END,
    "\"de\": \"Führt Cargo über die Registratur aus, statt es direkt aufzurufen.\" } },\n"
    "        { \"command\": \"repo tools exec --light rustc -- -vV\", \"description\": { \"en\": \"Ask an executable that a registered tool owns (rustc belongs to Cargo) for its version.\", \"de\": \"Fragt ein Programm, das ein registriertes Werkzeug besitzt (rustc gehört zu Cargo), nach seiner Version.\" } },\n"
    "        { \"command\": \"repo tools exec uv -- run python3 <script>.py\", \"description\": { \"en\": \"Run a Python script through uv without calling uv directly.\", \"de\": \"Führt ein Python-Skript über uv aus, ohne uv direkt aufzurufen.\" } }\n      ],", requires="tools exec [--cwd=<path>]")
NOTE_END = "und repo verify tool-spawns verweigert die Starts, die sie umgehen.\" }\n      ]"
add("c1-note", "catalogue-tools-exec", CAT, NOTE_END,
    "und repo verify tool-spawns verweigert die Starts, die sie umgehen.\" },\n"
    "        { \"en\": \"A command after -- of exec, watch and capacity run is started as written: pass a repo command or repo tools exec <tool> -- ..., never a bare tool; repo verify tool-declarations refuses declarative files that do.\", \"de\": \"Ein Befehl nach -- von exec, watch und capacity run wird so gestartet, wie er geschrieben steht: Übergib einen repo-Befehl oder repo tools exec <tool> -- ..., niemals ein nacktes Werkzeug; repo verify tool-declarations verweigert deklarative Dateien, die es tun.\" }\n      ]", requires="tools exec [--cwd=<path>]")
add("c1-import", "catalogue-tools-exec", CMD, "toolMessageV1, toolRootV1, ToolErrorV1,", "toolForCommandV1, toolMessageV1, toolRootV1, ToolErrorV1,", requires="async function exec(")
add("c1-resolve", "catalogue-tools-exec", CMD, "entry = id === undefined ? undefined : findToolV1(id);", "entry = id === undefined ? undefined : findToolV1(id) ?? toolForCommandV1(id);", requires="async function exec(")
add("c1-run", "catalogue-tools-exec", CMD, "await runToolV1(entry.id, tail, { root, cwd, env:", "await runToolV1(entry.id, tail, { root, cwd, command: entry.id === id ? undefined : id, env:", requires="async function exec(")
add("c1-doc", "catalogue-tools-exec", CMD, "`repo tools exec [--cwd=<path>] [--light] <tool> -- <arguments>`. */\nasync function exec(", "`repo tools exec [--cwd=<path>] [--light] <tool|command> -- <arguments>` where the command is an executable a registered tool owns. */\nasync function exec(", requires="async function exec(")
VERIFY_OLD = "    if (segments[0] === \"zero-touch\") {\n      this.runZeroTouch(segments.slice(1));\n      return;\n    }\n"
VERIFY_NEW = VERIFY_OLD + "    if (segments[0] === \"tool-declarations\") {\n      const { runDirectToolDeclarationLawV1 } = await import(\"../../⚖️laws/🧰️direct-tool-declarations/⌨️command/🟦️.ts\");\n      process.exitCode = runDirectToolDeclarationLawV1(this.root, segments.slice(1));\n      return;\n    }\n"
add("law-verify-dispatch", "law", VERIFY, VERIFY_OLD, VERIFY_NEW, requires="zero-touch")
json.dump({"version": 1, "rows": rows}, open(os.path.join(HERE, "literals.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rows), "rows")
