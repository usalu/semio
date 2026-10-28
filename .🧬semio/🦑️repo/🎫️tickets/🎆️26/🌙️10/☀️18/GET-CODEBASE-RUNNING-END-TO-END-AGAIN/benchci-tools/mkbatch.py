# coding: utf-8
"""🛂️ Builds the staged batch `acceptance` (lane benchci) on the CURRENT live files: the new module `📏️incremental/🏎️bench/🛂️acceptance` (schema, budget table, logic, runner, command, laws, Python oracle, fixture of recorded gate logs), the route and option union in the bench command, the subcommand law of the bench tests, the operations catalogue, the README (EN+DE) and the nightly workflow `.github/workflows/acceptance.yml`.
usage: python3 -I mkbatch.py [--help-from FILE]   (ROOT = the live checkout above this file; anchored edits stop with exit 2 when an anchor moved)
       --help-from FILE: the regenerated help snapshot (from `helpgen.ts` run in a mirror with the batch applied) is written as the help fixture of the batch."""
import hashlib, json, os, re, shutil, stat, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import data, mkfixtures

ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and os.path.dirname(ROOT) != ROOT:
    ROOT = os.path.dirname(ROOT)
STAGED = os.path.join(HERE, "staged", "acceptance")
VS = "️"


def find(parent, glyph):
    for name in os.listdir(os.path.join(ROOT, parent)):
        if name.startswith(glyph):
            return name
    sys.exit("no %s in %s" % (glyph, parent))


LIB = "🧰" + VS + "framework/🛍" + VS + "products/🦑" + VS + "repo/🔨" + VS + "modules/📚" + VS + "library"
BUILD = LIB + "/🏗" + VS + "build"
INCR = BUILD + "/📏" + VS + "incremental"
BENCH = INCR + "/🏎" + VS + "bench"
ACC = BENCH + "/🛂" + VS + "acceptance"
for must in (BUILD, INCR, BENCH):
    if not os.path.isdir(os.path.join(ROOT, must)):
        sys.exit("missing " + must)
NAMES = {n: find(BENCH, g) for n, g in {"SCHD": "🧬", "CMD": "⌨", "RUN": "🏃", "TESTS": "🧪", "SLOD": "🎯"}.items()}
OPS = BUILD + "/⚙" + VS + "operations"
OPSTESTS = OPS + "/" + find(OPS, "🧪")
JSONF, TSF, PYF = "🔣" + VS + ".json", "🟦" + VS + ".ts", "🐍" + VS + ".py"
TOKENS = {
    "SCHD": "🧬" + VS + "schema", "JSON": JSONF, "TS": TSF, "PY": PYF, "CMD": "⌨" + VS + "command", "RUN": "🏃" + VS + "runner",
    "FIXD": "🧫" + VS + "fixtures", "ORACLED": "🐍" + VS + "oracle", "BUDGETSF": "🎯" + VS + "budgets/" + JSONF, "SLOF": "🎯" + VS + "slo/" + JSONF,
    "OPS": "⚙" + VS + "operations/🔣" + VS + "catalog.json", "HOSTCAP": "🏗" + VS + "build/🌡" + VS + "host-capacity/" + TSF, "EXEC": "🎛" + VS + "execution/" + TSF,
    "PROCESS": "../" * 9 + "🔨" + VS + "modules/🏃" + VS + "process/📦" + VS + "packages/🟦" + VS + "typescript", "ROUTING": "🧭" + VS + "routing/" + TSF,
}
VALIDATOR = "🔨" + VS + "modules/🧬" + VS + "schema/✅" + VS + "validator/📦" + VS + "packages/🟦" + VS + "typescript/" + TSF
CONTAINS_EMOJI = re.compile("([\U0001F000-\U0001FFFF☀-➿])(?!️)")


def read(rel):
    with open(os.path.join(ROOT, rel), "rb") as handle:
        return handle.read()


def sha(data_bytes):
    return hashlib.sha256(data_bytes).hexdigest()


def render(path, depth):
    text = open(os.path.join(HERE, "src", path), encoding="utf8").read()
    text = text.replace("@@VALIDATOR@@", "../" * depth + VALIDATOR)
    for key, value in TOKENS.items():
        text = text.replace("@@%s@@" % key, value)
    if "@@" in text:
        sys.exit("unresolved token in %s: %s" % (path, re.findall(r"@@\w+@@", text)[:3]))
    text = re.sub(r'(/\*\* |""")' + CONTAINS_EMOJI.pattern, lambda m: m.group(1) + m.group(2) + VS, text)
    return text.encode("utf8")


def edit(rel, pairs):
    text = read(rel).decode("utf8")
    for old, new in pairs:
        if text.count(old) != 1:
            print("DEAD ANCHOR in %s: %r (%d)" % (rel[-60:], old[:90], text.count(old)))
            sys.exit(2)
        text = text.replace(old, new)
    return text.encode("utf8")


def catalog_edits():
    rel = OPS + "/🔣" + VS + "catalog.json"
    summary_old = read(rel).decode("utf8")
    flag_block = '''    {
      "name": "--prepare",
      "value": "none",
      "operations": ["bench"],
'''
    i = summary_old.index(flag_block)
    j = summary_old.index("    },\n", i) + len("    },\n")
    new_flags = '''    {
      "name": "--in-place",
      "value": "none",
      "operations": ["bench"],
      "description": { "en": "bench acceptance: run the scenarios that edit and restore source files in this checkout (a CI runner, a private clone); nobody else may edit it meanwhile.", "de": "bench acceptance: führt die Szenarien, die Quelldateien ändern und zurücksetzen, in diesem Checkout aus (CI-Runner, private Kopie); währenddessen darf ihn niemand sonst bearbeiten." }
    },
'''
    text = summary_old[:j] + new_flags + summary_old[j:]
    pairs = [
        ('{"en": "Prepare the disposable copy first with `repo incremental prepare` (or pass --prepare).", "de": "Zuerst die Wegwerf-Kopie mit `repo incremental prepare` vorbereiten (oder --prepare übergeben)."}',
         '{"en": "bench list|run|compare: prepare the disposable copy first with `repo incremental prepare` (or pass --prepare); bench acceptance runs on the checkout itself and needs no copy.", "de": "bench list|run|compare: zuerst die Wegwerf-Kopie mit `repo incremental prepare` vorbereiten (oder --prepare übergeben); bench acceptance läuft im Checkout selbst und braucht keine Kopie."}'),
        ('"en": "bench run|compare: also write the report as a markdown file.", "de": "bench run|compare: schreibt den Bericht zusätzlich als Markdown-Datei."',
         '"en": "bench run|compare|acceptance: also write the report as a markdown file.", "de": "bench run|compare|acceptance: schreibt den Bericht zusätzlich als Markdown-Datei."'),
        ('      "operations": ["incremental", "conformance"],\n      "description": { "en": "incremental plan|run: comma list of scenario ids, or all.", "de": "incremental plan|run: Kommaliste von Szenario-IDs oder all." }',
         '      "operations": ["incremental", "conformance", "bench"],\n      "description": { "en": "incremental plan|run and bench acceptance: comma list of scenario ids, or all (bench acceptance also takes kind:<noop|peer|edit> and product:<course|print|dashboard>; without --in-place, all leaves the scenarios that edit files out).", "de": "incremental plan|run und bench acceptance: Kommaliste von Szenario-IDs oder all (bench acceptance nimmt auch kind:<noop|peer|edit> und product:<course|print|dashboard>; ohne --in-place lässt all die Szenarien aus, die Dateien ändern)." }'),
        ('      "synopsis": "bench list|run|compare",', '      "synopsis": "bench list|run|compare|acceptance",'),
        ("in a disposable checkout copy, judge them against the SLO table and compare with a stored baseline.\", \"de\"", "in a disposable checkout copy, judge them against the SLO table and compare with a stored baseline; `bench acceptance` holds the acceptance scenarios of the three products (hot no-op, peer edit, one-line edit with byte-exact revert) on the checkout itself to declared budgets.\", \"de\""),
        ("beurteilt sie anhand der Zieltabelle und vergleicht mit einer gespeicherten Referenz.\" },", "beurteilt sie anhand der Zieltabelle und vergleicht mit einer gespeicherten Referenz; `bench acceptance` hält die Abnahmeszenarien der drei Produkte (heißer Leerlauf, fremde Änderung, einzeilige Änderung mit byteidentischer Rücknahme) im Checkout selbst an deklarierte Budgets.\" },"),
        ('''"flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--current", "--history", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json"],
      "examples": [''', '''"flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--current", "--history", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json", "--scenario", "--in-place"],
      "examples": ['''),
        ('''        { "command": "repo bench run --slo=kind:noop --baseline=bench-baseline.json", "description": { "en": "Measure the warm no-op objectives and compare with the stored baseline.", "de": "Misst die Ziele für den warmen Leerlauf und vergleicht mit der gespeicherten Referenz." } }
''', '''        { "command": "repo bench run --slo=kind:noop --baseline=bench-baseline.json", "description": { "en": "Measure the warm no-op objectives and compare with the stored baseline.", "de": "Misst die Ziele für den warmen Leerlauf und vergleicht mit der gespeicherten Referenz." } },
        { "command": "repo bench acceptance --in-place --file=acceptance.json --markdown=acceptance.md", "description": { "en": "Run every acceptance scenario on this checkout and fail when a blocking budget is broken.", "de": "Führt jedes Abnahmeszenario in diesem Checkout aus und schlägt fehl, wenn ein verbindliches Budget verletzt ist." } }
'''),
        ('jedes verbindliche Ziel über dem Budget." } }\n      ],',
         'jedes verbindliche Ziel über dem Budget." } },\n        { "name": "acceptance", "synopsis": "bench acceptance [--scenario=<ids|all>] [--in-place]", "flags": ["--scenario", "--in-place", "--markdown", "--file", "--json"], "summary": { "en": "Run the acceptance scenarios (hot no-op three times, peer edit, one-line edit with byte-exact revert) of the course, print and the dashboard through the official route on this checkout, with the exact wall clock and the physical footprint, judge them against the declared budgets and fail when a blocking budget is broken.", "de": "Führt die Abnahmeszenarien (heißer Leerlauf dreimal, fremde Änderung, einzeilige Änderung mit byteidentischer Rücknahme) von Kurs, Druck und Dashboard über den offiziellen Weg in diesem Checkout aus, mit exakter Laufzeit und physischem Speicherbedarf, beurteilt sie anhand der deklarierten Budgets und schlägt fehl, wenn ein verbindliches Budget verletzt ist." } }\n      ],'),
    ]
    out = text.encode("utf8")
    holder = {}
    text2 = text
    for old, new in pairs:
        if text2.count(old) != 1:
            print("DEAD ANCHOR in catalog: %r (%d)" % (old[:90], text2.count(old)))
            sys.exit(2)
        text2 = text2.replace(old, new)
    json.loads(text2)
    return rel, text2.encode("utf8")


def main():
    helpfile = sys.argv[sys.argv.index("--help-from") + 1] if "--help-from" in sys.argv else None
    files = {}
    files[ACC + "/🧬" + VS + "schema/" + JSONF] = data.dump(data.SCHEMA).encode("utf8")
    files[ACC + "/🎯" + VS + "budgets/" + JSONF] = data.dump(data.budget_set()).encode("utf8")
    files[ACC + "/" + TSF] = render("logic.ts", 8)
    files[ACC + "/🏃" + VS + "runner/" + TSF] = render("runner.ts", 9)
    files[ACC + "/⌨" + VS + "command/" + TSF] = render("command.ts", 9)
    files[ACC + "/🧪" + VS + "tests/" + TSF] = render("test.ts", 9)
    files[ACC + "/🧪" + VS + "tests/🐍" + VS + "oracle/" + PYF] = open(os.path.join(HERE, "src", "oracle.py"), encoding="utf8").read().replace('"""🐍 ', '"""🐍' + VS + ' ', 1).encode("utf8")
    files[ACC + "/🧪" + VS + "tests/🧫" + VS + "fixtures/" + JSONF] = (json.dumps(mkfixtures.build(), ensure_ascii=False, indent=1) + "\n").encode("utf8")
    files[".github/workflows/acceptance.yml"] = open(os.path.join(HERE, "src", "acceptance.yml"), encoding="utf8").read().encode("utf8")
    bench_command = BENCH + "/" + NAMES["CMD"] + "/" + TSF
    files[bench_command] = edit(bench_command, [
        ('import schema from "../' + TOKENS["SCHD"] + "/" + JSONF + '" with {type:"json"};\n', 'import schema from "../' + TOKENS["SCHD"] + "/" + JSONF + '" with {type:"json"};\nimport acceptance from "../🛂' + VS + 'acceptance/' + TOKENS["SCHD"] + "/" + JSONF + '" with {type:"json"};\n'),
        ("export function benchOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{return {valued:valuedOptions,plain:plainOptions};}",
         "export function benchOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{const own=acceptance.definitions.Policy.const.options;return {valued:[...valuedOptions,...own.valued],plain:[...plainOptions,...own.plain]};}"),
        (" await runBenchCommandV1(root,args,await benchCommandPortsV1(root,invocation));",
         ' const rest=args.slice(args[0]==="bench"?1:0);\n if(rest[0]==="acceptance"){const{runAcceptanceV1}=await import("../🛂' + VS + 'acceptance/' + TOKENS["CMD"] + "/" + TSF + '");await runAcceptanceV1(root,rest.slice(1),invocation);return;}\n await runBenchCommandV1(root,args,await benchCommandPortsV1(root,invocation));'),
        ("/** 🍜️ The entry the build CLI calls for `repo bench`. */", "/** 🍜️ The entry the build CLI calls for `repo bench`; `repo bench acceptance` is the acceptance module's own command. */"),
    ])
    bench_tests = BENCH + "/" + NAMES["TESTS"] + "/" + TSF
    files[bench_tests] = edit(bench_tests, [('expect(entry.subcommands.map((row:any)=>row.name)).toEqual(["list","run","compare"]);', 'expect(entry.subcommands.map((row:any)=>row.name)).toEqual(["list","run","compare","acceptance"]);')])
    rel, body = catalog_edits()
    files[rel] = body
    readme = "🧰️framework/🛍️products/🦑️repo/README.md"
    readme_text = read(readme).decode("utf8")
    en_anchor, de_anchor = "- **Open.** Until the resident service is applied", "- **Offen.** Bis der residente Dienst angewendet ist"
    en_bullet = "- **Acceptance (`repo bench acceptance`).** The acceptance targets of the build system as a permanent, automated check on the checkout itself instead of a disposable copy: per product (course, print, dashboard) a hot no-op three times after two prime runs (the first builds, the second lets the graph memo settle), a peer-edit stage (comment lines appended to unrelated files and to a scheduling file of the build library when the first graph line appears, then restored; 0 re-executions), and a one-line edit with its byte-exact revert, all through the official `repo run` route with the exact wall clock of a monotonic clock and the physical footprint of the whole process tree sampled by the capacity governor's observer (macOS `phys_footprint`, Linux proportional set size, Windows private working set). The budgets are rows of `🛂️acceptance/🎯️budgets` (course hot no-op < 10 s and < 1.5 GB, course edit < 60 s, Rust edit < 120 s, dashboard no-op < 10 s, peer edits 0 re-executions; the dashboard no-op allows its one freshness worker, the dashboard edit and the print no-op time are advisory until a run holds them), each restating its bench objective by id where one exists. Verdicts are `pass`, `fail` or `unproven` with a reason (`clock grain`, `host saturated`, `resident only`, `not landed`, ...); a broken blocking budget, a failed run, an inexact restore, or a run that proved no blocking budget fails the command, after the JSON (`--file`) and markdown (`--markdown`) reports and the English or German text were written. `--in-place` is required for the scenarios that edit files (`all` leaves them out without it); edits are journaled under `.cache/acceptance` and a killed run is repaired by the next one. The nightly job `.github/workflows/acceptance.yml` (schedule and manual dispatch, `macos-26`, optional larger runner through the variable `SEMIO_ACCEPTANCE_RUNNER`) runs it per product (the course row is blocking and fails on a breach; the dashboard and print rows are advisory until their first green nightly run) and uploads the JSON and markdown reports.\n"
    de_bullet = "- **Abnahme (`repo bench acceptance`).** Die Abnahmeziele des Build-Systems als dauerhafte, automatische Prüfung im Checkout selbst statt in einer Wegwerf-Kopie: je Produkt (Kurs, Druck, Dashboard) ein heißer Leerlauf dreimal nach zwei Vorläufen (der erste baut, der zweite lässt das Graph-Memo sich setzen), eine Stufe für fremde Änderungen (Kommentarzeilen werden an unabhängige Dateien und an eine Planungsdatei der Build-Bibliothek angehängt, sobald die erste Graph-Zeile erscheint, und danach zurückgesetzt; 0 erneute Ausführungen) und eine einzeilige Änderung mit byteidentischer Rücknahme, alles über den offiziellen Weg `repo run` mit der exakten Laufzeit einer monotonen Uhr und dem physischen Speicherbedarf des ganzen Prozessbaums, gemessen vom Beobachter der Kapazitätssteuerung (macOS `phys_footprint`, Linux proportionale Größe, Windows privater Arbeitssatz). Die Budgets sind Zeilen von `🛂️acceptance/🎯️budgets` (heißer Kurs-Leerlauf < 10 s und < 1,5 GB, Kurs-Änderung < 60 s, Rust-Änderung < 120 s, Dashboard-Leerlauf < 10 s, fremde Änderungen 0 erneute Ausführungen; der Dashboard-Leerlauf erlaubt seinen einen Frische-Worker, die Dashboard-Änderung und die Zeit des Druck-Leerlaufs sind beratend, bis ein Lauf sie hält), jede wiederholt ihr Bench-Ziel mit dessen Kennung, wo eines existiert. Urteile sind `pass`, `fail` oder `unproven` mit Grund (`Uhrstufe`, `Rechner ausgelastet`, `nur resident`, `nicht eingetroffen`, ...); ein verletztes verbindliches Budget, ein fehlgeschlagener Lauf, eine ungenaue Rücknahme oder ein Lauf, der kein verbindliches Budget bewiesen hat, lässt den Befehl fehlschlagen, nachdem die JSON- (`--file`) und Markdown-Berichte (`--markdown`) und der Text auf Englisch oder Deutsch geschrieben wurden. `--in-place` ist für die Szenarien nötig, die Dateien ändern (`all` lässt sie ohne aus); Änderungen werden unter `.cache/acceptance` protokolliert und ein abgebrochener Lauf wird vom nächsten repariert. Der nächtliche Job `.github/workflows/acceptance.yml` (Zeitplan und manueller Start, `macos-26`, optional ein größerer Runner über die Variable `SEMIO_ACCEPTANCE_RUNNER`) führt ihn je Produkt aus (die Kurs-Zeile ist verbindlich und schlägt bei einer Verletzung fehl; die Zeilen für Dashboard und Druck sind beratend bis zu ihrem ersten grünen nächtlichen Lauf) und lädt die JSON- und Markdown-Berichte hoch.\n"
    for anchor in (en_anchor, de_anchor):
        if readme_text.count(anchor) != 1:
            print("DEAD ANCHOR in README:", anchor)
            sys.exit(2)
    files[readme] = readme_text.replace(en_anchor, en_bullet + en_anchor).replace(de_anchor, de_bullet + de_anchor).encode("utf8")
    if helpfile:
        files[OPSTESTS + "/🧫" + VS + "fixtures/📖" + VS + "help/" + JSONF] = (json.dumps(json.load(open(helpfile, encoding="utf8")), ensure_ascii=False, indent=2) + "\n").encode("utf8")
    shutil.rmtree(STAGED, ignore_errors=True)
    os.makedirs(os.path.join(STAGED, "files"))
    os.makedirs(os.path.join(STAGED, "base"))
    manifest = {}
    for number, (rel, body) in enumerate(files.items()):
        name = "%02d%s" % (number, os.path.splitext(rel)[1])
        live = os.path.join(ROOT, rel)
        base = None
        if os.path.exists(live):
            base = sha(open(live, "rb").read())
            shutil.copyfile(live, os.path.join(STAGED, "base", name))
        with open(os.path.join(STAGED, "files", name), "wb") as handle:
            handle.write(body)
        manifest[rel] = {"file": name, "base": base, "new": sha(body)}
        print(("new " if base is None else "edit"), rel[-100:])
    with open(os.path.join(STAGED, "manifest.json"), "w", encoding="utf8") as handle:
        json.dump(manifest, handle, ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    template = template.replace("Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern.", "Staged batch `acceptance` (lane benchci): `repo bench acceptance`, the permanent acceptance scenarios with declared budgets, and the nightly workflow that runs them.", 1)
    with open(os.path.join(STAGED, "stage.py"), "w", encoding="utf8") as handle:
        handle.write(template)
    os.chmod(os.path.join(STAGED, "stage.py"), 0o755)
    tests = [ACC + "/🧪" + VS + "tests/" + TSF, bench_tests, OPSTESTS + "/" + TSF, LIB + "/🏗" + VS + "build/🧾" + VS + "ci-report/🗂" + VS + "suites/🧪" + VS + "tests/🔀" + VS + "workflow/" + TSF]
    expected = {bench_tests: ["the product catalogue holds the brief's budgets for the dashboard, the architecture course and print, and every edited file exists"], OPSTESTS + "/" + TSF: ["the real CLI prints the own help of a command, also for preflight, without running it"]}
    with open(os.path.join(STAGED, "expected-failures.txt"), "w", encoding="utf8") as handle:
        for suite, names in expected.items():
            for name in names:
                handle.write(suite + "|" + name + "\n")
    verify = '''#!/bin/sh
# 🛂️ Laws of batch `acceptance` after apply: `sh verify.sh <MIRROR root with the batch applied>` (never the live root). The mirror holds the library, the process module, the launcher and `.github` only:
# the two tests listed in expected-failures.txt need trees a mirror does not copy (they fail the same way on the base files, measured), every other test of the suites must pass.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=${1:?mirror root}
case "$root" in */sm/*) ;; *) echo "refusing: $root is not a mirror" >&2; exit 2;; esac
python3 -I "$here/stage.py" check --root "$root" | grep -v '^applied' && { echo "batch is not fully applied in $root" >&2; exit 1; } || true
cd "$root"
for suite in %s; do
  echo "== $suite"
  out=$(bun test "./$suite" 2>&1 || true)
  echo "$out" | grep -E '^ *[0-9]+ (pass|fail)'
  fails=$(echo "$out" | sed -n 's/^(fail) \\(.*\\) \\[[0-9.]*ms\\]$/\\1/p' | sort)
  want=$(grep -F "$suite|" "$here/expected-failures.txt" | sed 's/^[^|]*|//' | sort || true)
  if [ "$fails" != "$want" ]; then echo "UNEXPECTED FAILURE SET in $suite:" >&2; echo "$fails" >&2; exit 1; fi
done
python3 -I "$here/../../../xplat5-tools/tests/test_stage_mode.py" "$here/stage.py"
echo "acceptance VERIFIED"
''' % " ".join('"%s"' % t for t in tests)
    with open(os.path.join(STAGED, "verify.sh"), "w", encoding="utf8") as handle:
        handle.write(verify)
    os.chmod(os.path.join(STAGED, "verify.sh"), 0o755)
    used = []
    for rel, body in files.items():
        if rel.startswith(ACC) and rel.endswith((".ts", ".py")):
            used += re.findall(r'(?:/\*\* |""")(\S)', body.decode("utf8"))
    print("docstring emoji:", len(used), "distinct:", len(set(used)))
    if len(used) != len(set(used)):
        print("DUPLICATES:", sorted({u for u in used if used.count(u) > 1}))
        sys.exit(3)


main()
