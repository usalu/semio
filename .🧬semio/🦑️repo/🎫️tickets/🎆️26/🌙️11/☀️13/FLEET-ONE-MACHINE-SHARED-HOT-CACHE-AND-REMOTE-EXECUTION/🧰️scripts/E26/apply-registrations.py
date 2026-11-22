import json, os, re, sys

LIB = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
MODULES = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
B = LIB + "🏗️build/"
CLI = MODULES + "⌨️cli/"
MCP = MODULES + "🔌️mcp/"
VERIFY = LIB + "🎛️commands/✅️verify/"

report = []


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def edit(path, anchor, new, marker=None):
    text = read(path)
    marker = marker or new
    if marker in text:
        report.append("already " + os.path.basename(path))
        return
    count = text.count(anchor)
    if count != 1:
        raise SystemExit("anchor count %d in %s: %r" % (count, path, anchor[:80]))
    write(path, text.replace(anchor, new))
    report.append("edited " + path.split("🧰️framework/")[-1])


OP = '''    {
      "name": "test-file",
      "requires": "none",
      "synopsis": "test-file [--json] [--bail] [--dry-run] [--] <path>...",
      "summary": { "en": "Run one or more test files exactly as their suite runner would, without reading the project graph.", "de": "Führt eine oder mehrere Testdateien genau so aus, wie ihr Suite-Runner es täte, ohne den Projektgraphen zu lesen." },
      "dryRun": true,
      "progress": true,
      "cancellable": true,
      "flags": ["--json", "--bail", "--dry-run"],
      "examples": [
        {
          "command": "repo test-file path/to/test-file.ts",
          "description": { "en": "Run one test file in its suite's birth, working directory, managed Python and hermetic temporary roots.", "de": "Führt eine Testdatei in der Geburt, dem Arbeitsverzeichnis, dem verwalteten Python und den hermetischen Temporärverzeichnissen ihrer Suite aus." }
        },
        {
          "command": "repo test-file --json --bail first.ts second.ts",
          "description": { "en": "Run several files, stop after the first failing one and print one structured result.", "de": "Führt mehrere Dateien aus, stoppt nach der ersten fehlschlagenden und gibt ein strukturiertes Ergebnis aus." }
        },
        {
          "command": "repo test-file --dry-run --json path/to/test-file.ts",
          "description": { "en": "Show which suite owns each file and how it would run, and run nothing.", "de": "Zeigt, welche Suite jede Datei besitzt und wie sie liefe, und führt nichts aus." }
        }
      ],
      "notes": [
        {
          "en": "Each file is routed by the suite table of the area that holds it (build, caching or process): the suite row owning the file is narrowed to that file, so the file runs in its own process under the strictness, budget, launcher (genuine birth), Original envelope with the native caller grant of that suite's target, managed Python binding and artifact sandbox its suite runner gives it, with the checkout as working directory; the suite's typecheck covers just that file. A file no suite owns runs plain (bun test in the checkout, Python bound when the source names it). No project graph is read, so it works while the graph is being rebuilt.",
          "de": "Jede Datei wird über die Suite-Tabelle des Bereichs geleitet, der sie enthält (build, caching oder process): Die Suite-Zeile, die die Datei besitzt, wird auf diese Datei verengt, sodass die Datei in einem eigenen Prozess mit Strenge, Budget, Starter (genuine birth), Original-Umschlag mit der nativen Aufruferberechtigung des Ziels der Suite, verwalteter Python-Bindung und Artefakt-Sandbox läuft, die ihr Suite-Runner ihr gibt, mit dem Checkout als Arbeitsverzeichnis; die Typprüfung der Suite deckt nur diese Datei ab. Eine Datei, die keine Suite besitzt, läuft schlicht (bun test im Checkout, Python gebunden, wenn die Quelle es nennt). Es wird kein Projektgraph gelesen, daher funktioniert es auch, während der Graph neu gebaut wird."
        },
        {
          "en": "A path is resolved against the current directory when it exists there, else against the checkout; it must be a regular TypeScript or JavaScript file inside the checkout. Stdout carries one line per file and the totals, or with --json one result document (version, status, seconds, bail, counts, one row per file with status passed|failed|skipped|not-run, bun's pass and fail counts and, for a failure, the error and the tail of its output); the test output itself goes to stderr under --json. A suite not declared for this host is reported skipped, which never reads as a pass. Exit code 0 when every file passed, 1 when a file failed, 2 when a file or an option is refused. Progress goes to stderr; Ctrl-C or SIGTERM cancels the run and its children.",
          "de": "Ein Pfad wird gegen das aktuelle Verzeichnis aufgelöst, wenn er dort existiert, sonst gegen den Checkout; er muss eine reguläre TypeScript- oder JavaScript-Datei im Checkout sein. Stdout trägt eine Zeile je Datei und die Summen oder mit --json ein Ergebnisdokument (Version, Status, Sekunden, bail, Zähler, eine Zeile je Datei mit Status passed|failed|skipped|not-run, den Pass- und Fail-Zahlen von bun und bei einem Fehler dem Fehler und dem Ende seiner Ausgabe); die Testausgabe selbst geht mit --json nach stderr. Eine Suite, die für diesen Rechner nicht deklariert ist, wird als übersprungen gemeldet, was nie als bestanden gilt. Exit-Code 0, wenn jede Datei bestand, 1, wenn eine Datei fehlschlug, 2, wenn eine Datei oder eine Option abgelehnt wird. Der Fortschritt geht nach stderr; Strg-C oder SIGTERM bricht den Lauf und seine Kinder ab."
        }
      ]
    },
'''


def catalogue():
    path = B + "⚙️operations/🔣️catalog.json"
    edit(path, '    {\n      "name": "exec",\n      "requires": "dependencies",\n      "synopsis": "exec --projects', OP + '    {\n      "name": "exec",\n      "requires": "dependencies",\n      "synopsis": "exec --projects', marker='"name": "test-file",')
    text = read(path)
    old = '"operations": ["run", "run-many", "affected"],\n      "description": { "en": "Stop at the first failing task.", "de": "Stoppt bei der ersten fehlschlagenden Task." }'
    new = '"operations": ["run", "run-many", "affected", "test-file"],\n      "description": { "en": "Stop at the first failing task (test-file: after the first failing test file).", "de": "Stoppt bei der ersten fehlschlagenden Task (test-file: nach der ersten fehlschlagenden Testdatei)." }'
    edit(path, old, new)
    old = '"operations": ["run", "run-many", "affected", "exec", "cache", "remote-cache", "doctor"],'
    edit(path, old, old.replace('"doctor"]', '"doctor", "test-file"]'))
    text = read(path)
    old = "With cache gc or remote-cache gc: list what would be removed and remove nothing.\","
    new = "With cache gc or remote-cache gc: list what would be removed and remove nothing. With test-file: print the route of every file and run nothing.\","
    edit(path, old, new)
    old = "Mit cache gc oder remote-cache gc: listet auf, was entfernt würde, und entfernt nichts.\""
    new = "Mit cache gc oder remote-cache gc: listet auf, was entfernt würde, und entfernt nichts. Mit test-file: gibt den Ablauf jeder Datei aus und führt nichts aus.\""
    edit(path, old, new)
    text = read(path)
    block = re.search(r'"name": "--json",\n      "value": "none",\n      "operations": \[[^\]]*\]', text)
    if block is None:
        raise SystemExit("--json block not found")
    if '"test-file"' not in block.group(0):
        write(path, text.replace(block.group(0), block.group(0)[:-1] + ', "test-file"]'))
        report.append("edited --json operations")
    edit(B + "⚙️operations/🧬️schema/🔣️.json", '"affected", "exec", "watch"', '"affected", "test-file", "exec", "watch"')
    edit(B + "⚙️operations/🧪️tests/🛗️requires/🔣️.json", '"none": ["reset", "tools"', '"none": ["test-file", "reset", "tools"')


def door():
    edit(B + "🚪️door/🔣️.json", '{ "operation": "scheduler", "export": "runSchedulerCliV1", "call": "root-args-environment" }', '{ "operation": "scheduler", "export": "runSchedulerCliV1", "call": "root-args-environment" },\n    { "operation": "test-file", "export": "runTestFileCliV1", "call": "root-args-environment" }', marker='"operation": "test-file"')
    edit(B + "🚪️door/🟦️.ts", ' scheduler:()=>import("../🗓️scheduler/⌨️command/🟦️.ts"),', ' scheduler:()=>import("../🗓️scheduler/⌨️command/🟦️.ts"),\n "test-file":()=>import("../🔬️test-file/⌨️command/🟦️.ts"),', marker='"test-file":()=>import')
    case = '''    {
      "name": "capacity status is light",
      "args": [
        "repo",
        "capacity",
        "status"
      ],'''
    new = '''    {
      "name": "test-file is light and keeps its files after the separator",
      "args": [
        "repo",
        "test-file",
        "--json",
        "--",
        "a.ts"
      ],
      "language": "en",
      "route": {
        "kind": "light",
        "operation": "test-file",
        "args": [
          "test-file",
          "--json",
          "--",
          "a.ts"
        ]
      }
    },
''' + case
    edit(B + "🚪️door/🧪️tests/🧫️fixtures/🔣️.json", case, new, marker='"name": "test-file is light and keeps its files after the separator"')
    anchor = '    {\n      "id": "doctor-entry",\n'
    forbid = ["graph", "native-build", "test-harness", "cargo", "caller", "invocation", "selection", "resident", "bootstrap", "store-receipt", "tools", "revision", "routing", "leases", "host-capacity", "sealed"]
    row = (
        '    {\n      "id": "test-file-entry",\n      "kind": "import",\n      "module": "../../../🔬️test-file/⌨️command/🟦️.ts",\n      "forbid": [\n'
        + ",\n".join('        "%s"' % name for name in forbid)
        + '\n      ],\n      "maximumModules": 24,\n      "maximumBytes": 120000\n    },\n'
    )
    edit(B + "🚪️door/🧪️tests/🚦️startup/🔣️.json", anchor, row + anchor, marker='"id": "test-file-entry"')


def command():
    for path in (B + "⌨️command/🧭️selection/🔣️.json", B + "⌨️command/🧭️selection/🧬️schema/🔣️.json"):
        edit(path, '"capacity", "fleet", "scheduler"]', '"capacity", "fleet", "scheduler", "test-file"]')


def suites():
    anchor = '''    {
      "id": "build-generation-claims",'''
    new = '''    {
      "id": "build-test-file",
      "budgetMs": 300000,
      "typecheck": "none",
      "trees": [
        "🔬️test-file"
      ],
      "platforms": [
        "darwin",
        "linux",
        "win32"
      ]
    },
''' + anchor
    edit(B + "🗂️suites/🔣️.json", anchor, new, marker='"id": "build-test-file"')


def policy():
    anchor = '      "!{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/**/*",\n'
    edit(B + "🔣️policy.json", anchor, anchor + '      "!{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/**/*",\n', marker="🏗️build/🔬️test-file/**/*")


def cli():
    edit(CLI + "🧬️schema/🚪️entrypoint/🔣️.json", '"exec",\n        "watch"', '"exec",\n        "test-file",\n        "watch"', marker='"test-file",')
    edit(CLI + "🧫️fixtures/🚪️entrypoint/🔣️.json", '"exec",\n    "watch"', '"exec",\n    "test-file",\n    "watch"', marker='"test-file",')
    anchor = '    {\n      "id": "conformance",\n      "argv": ["conformance", "--opaque=value"],\n      "kind": "build",\n      "arguments": ["conformance", "--opaque=value"]\n    },\n'
    edit(CLI + "🧫️fixtures/🚪️entrypoint/🔣️.json", anchor, anchor + '    {\n      "id": "test-file",\n      "argv": ["test-file", "--opaque=value"],\n      "kind": "build",\n      "arguments": ["test-file", "--opaque=value"]\n    },\n', marker='"id": "test-file"')


def mcp():
    anchor = '    { "operation": "preflight", "kind": "query" },\n'
    entry = """    {
      "operation": "test-file",
      "kind": "execute",
      "bind": { "--json": true },
      "positional": [
        {
          "name": "files",
          "argv": true,
          "description": {
            "en": "Test files as an array of paths, relative to the checkout or absolute; each runs as its suite runner would run it, without reading the project graph (the result lists one row per file with status, counts and, for a failure, the tail of its output).",
            "de": "Testdateien als Array von Pfaden, relativ zum Checkout oder absolut; jede läuft so, wie ihr Suite-Runner sie ausführen würde, ohne den Projektgraphen zu lesen (das Ergebnis führt eine Zeile je Datei mit Status, Zählern und bei einem Fehler dem Ende ihrer Ausgabe auf)."
          }
        }
      ]
    },
"""
    edit(MCP + "🖼️assets/🏗️build-surface.json", anchor, entry + anchor, marker='"operation": "test-file"')
    golden = MCP + "🧪️tests/🧫️fixtures/🏗️build-surface.json"
    edit(golden, '    "test",\n    "tools_list",', '    "test",\n    "test_file",\n    "tools_list",', marker='"test_file",')
    text = read(golden)
    if '"id": "test-file-basic"' not in text:
        vectors = """    {
      "id": "test-file-basic",
      "tool": "test_file",
      "arguments": {
        "files": [
          "a/tests/one.ts"
        ]
      },
      "valid": true,
      "argv": [
        "test-file",
        "--json",
        "--",
        "a/tests/one.ts"
      ],
      "environment": {},
      "agent": ""
    },
    {
      "id": "test-file-flags",
      "tool": "test_file",
      "arguments": {
        "files": [
          "a.ts",
          "b.ts"
        ],
        "bail": true,
        "dry_run": true
      },
      "valid": true,
      "argv": [
        "test-file",
        "--json",
        "--bail",
        "--dry-run",
        "--",
        "a.ts",
        "b.ts"
      ],
      "environment": {},
      "agent": ""
    },
    {
      "id": "test-file-empty-files",
      "tool": "test_file",
      "arguments": {
        "files": []
      },
      "valid": false
    },
    {
      "id": "test-file-no-files",
      "tool": "test_file",
      "arguments": {},
      "valid": false
    }
"""
        tail = "    }\n  ]\n}"
        if not text.rstrip().endswith(tail):
            raise SystemExit("golden tail layout unexpected")
        stripped = text.rstrip()
        write(golden, stripped[: -len(tail)] + "    },\n" + vectors + "  ]\n}\n")
        report.append("edited golden vectors")


def verify():
    lanes = VERIFY + "🏁️gate-lanes/🟦️.ts"
    imp = 'import { policyJsonDuplicateKeyBreaches } from "../../../⚖️laws/🔑️json-duplicate-keys/🟦️.ts";\n'
    edit(lanes, imp, imp + 'import { packageExportReportV1 } from "../../../⚖️laws/📦️package-export-targets/🟦️.ts";\n', marker="package-export-targets/🟦️.ts")
    step = '    sayVerifyV1("gates.json-duplicate-keys");\n'
    edit(lanes, step, '    this.runPackageExportTargets();\n' + step, marker="this.runPackageExportTargets();")
    anchor = "  /** 🏛️ Executes architecture contract checks contributed by their owning build projects. */\n"
    method = (
        "  /** 📦️ Fails when a tracked package.json `exports` target (neither a pattern nor generated output) names a file that does not exist: the missing target breaks every importer at load time. */\n"
        "  protected runPackageExportTargets(): void {\n"
        '    sayVerifyV1("gates.package-export-targets");\n'
        "    const breaches = [...packageExportReportV1(this.root)].flatMap(([path, missing]) => missing.map((target) => `${path}: ${target}`));\n"
        "    if (breaches.length === 0) return;\n"
        "    for (const breach of breaches) console.error(`[verify] package-export-target: ${breach}`);\n"
        '    throw refuseVerifyV1("breach", "gates.package-export-target-breach", { breaches: breaches.length });\n'
        "  }\n\n"
    )
    edit(lanes, anchor, method + anchor, marker="protected runPackageExportTargets")
    lane = '    if (segments[0] === "policy-breach") {\n'
    block = (
        '    if (segments[0] === "package-export-targets") {\n'
        '      if (segments.length !== 1) throw refuseVerifyV1("usage","run.expected-package-export-targets");\n'
        "      this.runPackageExportTargets();\n"
        "      return;\n"
        "    }\n"
    )
    edit(VERIFY + "🟦️.ts", lane, block + lane, marker='segments[0] === "package-export-targets"')
    texts = VERIFY + "📖️texts/🔣️.json"
    key = '    "gates.json-duplicate-keys": {\n'
    new = (
        '    "gates.package-export-targets": {\n'
        '      "en": "[verify] every package.json export target exists (a missing target breaks every importer at load time)…",\n'
        '      "de": "[verify] jedes Export-Ziel einer package.json existiert (ein fehlendes Ziel bricht jeden Importeur beim Laden)…"\n'
        "    },\n"
        '    "gates.package-export-target-breach": {\n'
        '      "en": "[verify] {breaches} package.json export target(s) point at a missing file",\n'
        '      "de": "[verify] {breaches} Export-Ziel(e) einer package.json zeigen auf eine fehlende Datei"\n'
        "    },\n"
        '    "run.expected-package-export-targets": {\n'
        '      "en": "Expected verify package-export-targets",\n'
        '      "de": "Erwartet: verify package-export-targets"\n'
        "    },\n"
    )
    edit(texts, key, new + key, marker='"gates.package-export-targets"')


def readme():
    path = MODULES + "../README.md"
    path = os.path.normpath(path)
    text = read(path)
    if "`repo test-file`" in text:
        report.append("already readme")
        return
    en = (
        "- **Running a test file as its suite runner would: `repo test-file`.** `repo test-file [--json] [--bail] [--dry-run] [--] <path>...` (MCP tool `test_file`) runs one or more test files exactly as the suite runner of their area (build, caching or process) runs them, in 3 to 60 seconds and without reading the project graph: the suite row that owns the file is narrowed to it, so the file gets the birth of its suite (Original envelope with the native caller grant of the suite target, genuine-birth launcher), the checkout as working directory, the managed Python binding, a hermetic artifact sandbox, its strictness (the suite's typecheck covers just that file) and its budget; a file no suite owns runs plain, and a declared helper is refused as no test. Paths resolve against the current directory first, then the checkout. Stdout is one line per file and the totals, or with `--json` one document (per file `passed|failed|skipped|not-run`, bun's pass and fail counts and, for a failure, the error and the tail of its output; the test output goes to stderr); exit 0, 1 (a file failed) or 2 (refused); Ctrl-C or SIGTERM cancels the run and its children (exit 130). `--dry-run` prints the routing and runs nothing. Nobody needs the raw `bun test` incantation any more. Law: `📚️library/🏗️build/🔬️test-file/🧪️tests` (argument vectors and the routing of every test file of the three suite tables against an independent Python reading, a live run of a passing and a failing fixture with `--bail`, cancellation by SIGINT, the catalogue row, the schemas under two validators).\n"
    )
    de = (
        "- **Eine Testdatei so ausführen, wie ihr Suite-Runner es täte: `repo test-file`.** `repo test-file [--json] [--bail] [--dry-run] [--] <Pfad>...` (MCP-Werkzeug `test_file`) führt eine oder mehrere Testdateien genau so aus, wie der Suite-Runner ihres Bereichs (build, caching oder process) sie ausführt, in 3 bis 60 Sekunden und ohne den Projektgraphen zu lesen: Die Suite-Zeile, die die Datei besitzt, wird auf sie verengt, sodass die Datei die Geburt ihrer Suite (Original-Umschlag mit der nativen Aufruferberechtigung des Suite-Ziels, genuine-birth-Starter), den Checkout als Arbeitsverzeichnis, die verwaltete Python-Bindung, eine hermetische Artefakt-Sandbox, ihre Strenge (die Typprüfung der Suite deckt nur diese Datei ab) und ihr Budget erhält; eine Datei, die keine Suite besitzt, läuft schlicht, und ein deklarierter Helfer wird als Nicht-Test abgelehnt. Pfade werden zuerst gegen das aktuelle Verzeichnis aufgelöst, dann gegen den Checkout. Stdout ist eine Zeile je Datei und die Summen oder mit `--json` ein Dokument (je Datei `passed|failed|skipped|not-run`, die Pass- und Fail-Zahlen von bun und bei einem Fehler der Fehler und das Ende seiner Ausgabe; die Testausgabe geht nach stderr); Exit 0, 1 (eine Datei schlug fehl) oder 2 (abgelehnt); Strg-C oder SIGTERM bricht den Lauf und seine Kinder ab (Exit 130). `--dry-run` gibt den Ablauf aus und führt nichts aus. Niemand braucht die rohe `bun test`-Beschwörung mehr. Gesetz: `📚️library/🏗️build/🔬️test-file/🧪️tests` (Argumentvektoren und der Ablauf jeder Testdatei der drei Suite-Tabellen gegen eine unabhängige Python-Lesung, ein echter Lauf einer bestehenden und einer fehlschlagenden Fixture mit `--bail`, Abbruch per SIGINT, die Katalogzeile, die Schemas unter zwei Validatoren).\n"
    )
    lines = text.split("\n")
    out = []
    done = 0
    for line in lines:
        out.append(line)
        if line.startswith("- **Running a single file, declaring its birth, placing its scratch.**"):
            out.append(en.rstrip("\n"))
            done += 1
        elif line.startswith("- **Eine einzelne Datei ausführen, ihre Geburt deklarieren, ihren Scratch platzieren.**"):
            out.append(de.rstrip("\n"))
            done += 1
    if done != 2:
        raise SystemExit("readme anchors found: %d" % done)
    write(path, "\n".join(out))
    report.append("edited README")


def project():
    path = LIB + "⚡️caching/📋️project.json"
    text = read(path)
    if '"test-build-test-file"' in text:
        report.append("already project target")
        return
    dep = '        "test-build-tool-setup",\n'
    if text.count(dep) != 1:
        raise SystemExit("dependsOn anchor count %d" % text.count(dep))
    text = text.replace(dep, '        "test-build-test-file",\n' + dep)
    head = '    "test-build-tool-setup": {\n'
    if text.count(head) != 1:
        raise SystemExit("target anchor count %d" % text.count(head))
    block = (
        '    "test-build-test-file": {\n'
        '      "executor": "repo:script",\n'
        '      "cache": false,\n'
        '      "dependsOn": [],\n'
        '      "inputs": [\n'
        '        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/**/*"\n'
        '      ],\n'
        '      "outputs": [],\n'
        '      "options": {\n'
        '        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites",\n'
        '        "command": "repo ./📜️script.ts test build-test-file",\n'
        '        "forwardAllArgs": false\n'
        '      },\n'
        '      "metadata": {\n'
        '        "semio": {\n'
        '          "execution": {\n'
        '            "version": 1,\n'
        '            "kind": "source"\n'
        '          },\n'
        '          "cache": {\n'
        '            "version": 1,\n'
        '            "class": "live",\n'
        '            "reason": "exercises-host-resources"\n'
        '          }\n'
        '        }\n'
        '      }\n'
        '    },\n'
    )
    text = text.replace(head, block + head)
    write(path, text)
    report.append("edited caching project targets")


def guide():
    path = os.path.normpath(MODULES + "../📓️build-guide.md")
    edit(
        path,
        "- Graph-free single file: `nice -n 10 bun test ./<repo-relative file>` from the checkout root, always with `./`",
        "- Graph-free single file: `repo test-file [--json] <file>...` (MCP tool `test_file`) routes each file to the suite that owns it and runs it the way that suite's runner does, without a graph read; the raw form is `nice -n 10 bun test ./<repo-relative file>` from the checkout root, always with `./`",
        marker="- Graph-free single file: `repo test-file",
    )
    edit(
        path,
        "- Einzeldatei ohne Graph: `nice -n 10 bun test ./<Datei relativ zum Repo>` in der Checkout-Wurzel, immer mit `./`",
        "- Einzeldatei ohne Graph: `repo test-file [--json] <Datei>...` (MCP-Werkzeug `test_file`) leitet jede Datei zu der Suite, die sie besitzt, und führt sie so aus, wie der Läufer dieser Suite es täte, ohne Graph-Lesung; die rohe Form ist `nice -n 10 bun test ./<Datei relativ zum Repo>` in der Checkout-Wurzel, immer mit `./`",
        marker="- Einzeldatei ohne Graph: `repo test-file",
    )


if __name__ == "__main__":
    for name in sys.argv[1:]:
        globals()[name]()
    print("\n".join(report))
