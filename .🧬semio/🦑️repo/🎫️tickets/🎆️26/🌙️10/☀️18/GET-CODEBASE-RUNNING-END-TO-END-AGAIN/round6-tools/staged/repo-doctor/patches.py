import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


def staged(name):
    with open(os.path.join(STAGED, name), encoding="utf-8") as handle:
        return handle.read()


class Drift(Exception):
    pass


FLAGS = """    {
      "name": "--fix",
      "value": "none",
      "operations": ["doctor"],
      "description": { "en": "doctor: apply the declared safe repairs for the findings (publications of dead processes, stale lease registry rows, gone agent sessions); with --dry-run list what they would remove.", "de": "doctor: wendet die deklarierten sicheren Reparaturen für die Befunde an (Veröffentlichungen toter Prozesse, veraltete Lease-Registerzeilen, beendete Agent-Sitzungen); mit --dry-run wird aufgelistet, was sie entfernen würden." }
    },
    {
      "name": "--only",
      "value": "required",
      "valueName": "area[,area]",
      "operations": ["doctor"],
      "description": { "en": "doctor: limit the report to these areas (tools, storage, leases, capacity, sessions, graph, dependencies, caches, processes, host).", "de": "doctor: beschränkt den Bericht auf diese Bereiche (tools, storage, leases, capacity, sessions, graph, dependencies, caches, processes, host)." }
    },
    {
      "name": "--strict",
      "value": "none",
      "operations": ["doctor"],
      "description": { "en": "doctor: exit with status 1 for warnings as well as errors.", "de": "doctor: beendet sich mit Status 1 auch bei Warnungen, nicht nur bei Fehlern." }
    },
    {
      "name": "--facts",
      "value": "none",
      "operations": ["doctor"],
      "description": { "en": "doctor: print the raw facts document the findings are derived from, as JSON, instead of the report.", "de": "doctor: gibt statt des Berichts das rohe Faktendokument aus, aus dem die Befunde abgeleitet sind, als JSON." }
    }
"""

OPERATION = """    {
      "name": "doctor",
      "requires": "none",
      "synopsis": "doctor [--json] [--fix [--dry-run]] [--only=<area>[,<area>]] [--strict] [--facts]",
      "summary": { "en": "Answer in one shot why work is waiting, slow or failing: tool pins, storage images, leases, capacity grants, agent sessions, graph and dependency generations, caches, running repo processes and the host, each problem with its owner and the command that resolves it.", "de": "Beantwortet auf einmal, warum Arbeit wartet, langsam ist oder scheitert: Werkzeug-Pins, Speicherabbilder, Leases, Kapazitätszusagen, Agent-Sitzungen, Graph- und Abhängigkeits-Generationen, Caches, laufende repo-Prozesse und der Rechner, jedes Problem mit seinem Eigentümer und dem Befehl, der es löst." },
      "implementations": { "nx": "neutral", "bazel": "neutral" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "flags": ["--json", "--fix", "--dry-run", "--only", "--strict", "--facts"],
      "examples": [
        { "command": "repo doctor", "description": { "en": "See who waits for whom, what is slow and what is broken, with the command that fixes each finding.", "de": "Zeigt, wer auf wen wartet, was langsam und was kaputt ist, mit dem Befehl, der jeden Befund behebt." } },
        { "command": "repo doctor --only=leases,capacity", "description": { "en": "Look only at leases, capacity and the processes behind them.", "de": "Betrachtet nur Leases, Kapazität und die Prozesse dahinter." } },
        { "command": "repo doctor --fix --dry-run", "description": { "en": "List what the safe repairs would remove.", "de": "Listet auf, was die sicheren Reparaturen entfernen würden." } },
        { "command": "repo doctor --fix", "description": { "en": "Remove dead-process publications, stale lease registry rows and gone agent sessions, then read the state again.", "de": "Entfernt Veröffentlichungen toter Prozesse, veraltete Lease-Registerzeilen und beendete Agent-Sitzungen und liest danach den Zustand erneut." } },
        { "command": "repo doctor --json", "description": { "en": "The report as JSON for tools and agents.", "de": "Der Bericht als JSON für Werkzeuge und Agenten." } }
      ],
      "notes": [
        { "en": "doctor reads state only: it takes no lease and no capacity grant, so it answers even while a build holds everything, and it never starts a heavy command.", "de": "doctor liest nur Zustand: Es nimmt weder ein Lease noch eine Kapazitätszusage und antwortet daher auch dann, wenn ein Build alles hält; es startet nie einen schweren Befehl." },
        { "en": "Areas: tools, storage, leases, capacity, sessions, graph, dependencies, caches, processes, host. Exit status 1 when an error remains (a warning too with --strict), 130 when cancelled.", "de": "Bereiche: tools, storage, leases, capacity, sessions, graph, dependencies, caches, processes, host. Exit-Status 1, wenn ein Fehler bleibt (mit --strict auch eine Warnung), 130 bei Abbruch." },
        { "en": "--fix applies only repairs that are declared safe in the doctor policy; every other finding names the command to run or whom to ask.", "de": "--fix wendet nur Reparaturen an, die in der doctor-Policy als sicher deklariert sind; jeder andere Befund nennt den Befehl oder wen man fragen soll." }
      ]
    }
"""

DISPATCH = (
    '  if(diagnostic[0]==="doctor"){const{buildInvocationRequestsHelpV1}=await import("./🧭️invocation/🟦️.ts");'
    "if(buildInvocationRequestsHelpV1(diagnostic)){const{buildHelpForArgumentsV1}=await import(\"./⚙️operations/📖️help/🟦️.ts\");"
    "console.log(buildHelpForArgumentsV1(diagnostic,diagnosticEnvironment));return;}"
    'const{runDoctorCliV1}=await import("./🩺️doctor/⌨️command/🟦️.ts");'
    "process.exitCode=await runDoctorCliV1(root,diagnostic.slice(1),diagnosticEnvironment);return;}\n"
)
CAPACITY_ANCHOR = '  if(diagnostic[0]==="capacity"){'


def build_script(text):
    if 'diagnostic[0]==="doctor"' in text:
        return text
    if text.count(CAPACITY_ANCHOR) != 1:
        raise Drift("build-script: capacity dispatch anchor found %d times" % text.count(CAPACITY_ANCHOR))
    return text.replace(CAPACITY_ANCHOR, DISPATCH + CAPACITY_ANCHOR)


def add_to_list(text, pattern, value, label):
    match = re.search(pattern, text)
    if not match:
        raise Drift("%s: list anchor not found" % label)
    items = match.group(2)
    if '"%s"' % value in items:
        return text
    return text[: match.start(2)] + items + ', "%s"' % value + text[match.end(2):]


def catalog(text):
    if '"name": "doctor"' in text:
        return text
    flags_end = "    }\n  ],\n  \"operations\": ["
    if text.count(flags_end) != 1:
        raise Drift("catalog: end of the flags array found %d times" % text.count(flags_end))
    text = text.replace(flags_end, "    },\n" + FLAGS + "  ],\n  \"operations\": [")
    for flag in ("--json", "--dry-run"):
        match = re.search(r'"name": "%s",\s*"value": "[a-z]+",(?:\s*"valueName": "[^"\n]*",)?\s*"operations": \[([^\]\n]*)\]' % re.escape(flag), text)
        if not match:
            raise Drift("catalog: flag %s has no operations list" % flag)
        items = match.group(1)
        if '"doctor"' not in items:
            start = match.start(1)
            text = text[:start] + items + ', "doctor"' + text[start + len(items):]
    tail = "    }\n  ]\n}\n"
    if not text.endswith(tail):
        raise Drift("catalog: the operations array does not end the file")
    return text[: -len(tail)] + "    },\n" + OPERATION + "  ]\n}\n"


def operations_schema(text):
    return add_to_list(text, r'("OperationName": \{\s*"enum": \[)([^\]]*)(\])', "doctor", "operations-schema")


EDITORS = {"build-script": build_script, "catalog": catalog, "operations-schema": operations_schema}
