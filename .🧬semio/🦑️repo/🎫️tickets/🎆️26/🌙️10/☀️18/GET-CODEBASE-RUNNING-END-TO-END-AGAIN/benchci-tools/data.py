"""🗂️ Authored data of the acceptance module: the schema and the budget table as Python dicts that `mkbatch.py` writes as JSON (the JSON files are the single source; nothing reads this file at runtime)."""
import json

DOOR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts"


def t(en, de):
    return {"en": en, "de": de}


def ref(name):
    return {"$ref": "#/definitions/" + name}


def obj(required, properties, optional=()):
    props = dict(properties)
    return {"type": "object", "additionalProperties": False, "required": list(required), "properties": props}


def nullable(inner):
    return {"oneOf": [inner, {"type": "null"}]}


NUM = {"type": "number"}
INT = {"type": "integer", "minimum": 0}
BOOL = {"type": "boolean"}
STR = {"type": "string", "minLength": 1}


def array(items, minimum=0, unique=False):
    out = {"type": "array", "items": items}
    if minimum:
        out["minItems"] = minimum
    if unique:
        out["uniqueItems"] = True
    return out


POLICY = {
    "version": 1,
    "id": "^[a-z][a-z0-9]*(-[a-z0-9]+)*$",
    "taskId": "^\\S+:\\S+$",
    "kinds": ["noop", "peer", "edit"],
    "rolesOfKind": {"noop": ["prime", "hot"], "peer": ["prime", "control", "peer", "follow"], "edit": ["prime", "edit", "revert"]},
    "roles": ["prime", "hot", "control", "peer", "follow", "edit", "revert"],
    "metrics": ["wallMs", "peakFootprintMb", "executedActions"],
    "units": {"wallMs": "ms", "peakFootprintMb": "MB", "executedActions": "count"},
    "statistics": ["max", "min", "p50"],
    "minimumSamples": {"max": 1, "min": 1, "p50": 3},
    "clockGrainMs": {"exact": 0, "poll": 5000},
    "primeRuns": 2,
    "footprintIntervalMs": 1000,
    "saturatedLoadPerCore": 1.5,
    "quietLoadPerCore": 0.5,
    "maximumPathLength": 255,
    "maximumLogBytes": 268435456,
    "maximumObservedLineCharacters": 4096,
    "killGraceMs": 10000,
    "closeGraceMs": 5000,
    "journalDirectory": ".cache/acceptance",
    "journalFile": "journal.json",
    "edit": {"nonce": "^[0-9a-z]{6,32}$", "marker": "[acceptance]"},
    "environment": {"keep": ["SEMIO_BUILD_BUDGET_MS"], "prefixes": ["SEMIO_REMOTE_CACHE_", "SEMIO_HOST_CAPACITY_"]},
    "options": {"valued": ["--scenario"], "plain": ["--in-place"], "shared": ["--file", "--markdown", "--lang"], "flag": ["--json"]},
    "served": ["^graph: \\d+ projects served from the memo", "^graph: \\d+ Projekte aus dem Memo"],
    "door": DOOR,
    "tool": "bazel",
    "statuses": ["pass", "fail", "unproven"],
    "reasons": ["no-samples", "run-failed", "clock-grain", "host-saturated", "resident-only", "not-landed", "foreign-write", "skipped", "aborted"],
    "verdicts": ["pass", "fail", "unproven"],
    "loadClasses": ["quiet", "busy", "saturated", "unknown"],
    "outcomeStatuses": ["ran", "skipped", "failed"],
    "measures": ["footprint", "resident"],
    "clockKinds": ["exact", "poll"],
    "enforcements": ["blocking", "advisory"],
    "languages": ["en", "de"],
}

DEFINITIONS = {
    "Policy": {"const": POLICY},
    "Id": {"type": "string", "pattern": POLICY["id"]},
    "TaskId": {"type": "string", "pattern": POLICY["taskId"]},
    "Path": {"type": "string", "minLength": 1, "maxLength": POLICY["maximumPathLength"]},
    "Text": obj(["en", "de"], {"en": STR, "de": STR}),
    "Kind": {"enum": POLICY["kinds"]},
    "Role": {"enum": POLICY["roles"]},
    "Metric": {"enum": POLICY["metrics"]},
    "Statistic": {"enum": POLICY["statistics"]},
    "Enforcement": {"enum": POLICY["enforcements"]},
    "Status": {"enum": POLICY["statuses"]},
    "Reason": nullable({"enum": POLICY["reasons"]}),
    "Verdict": {"enum": POLICY["verdicts"]},
    "LoadClass": {"enum": POLICY["loadClasses"]},
    "Labels": {"type": "object", "additionalProperties": ref("Text")},
    "Product": obj(["id", "title", "target", "edit"], {
        "id": ref("Id"), "title": ref("Text"), "target": ref("TaskId"),
        "edit": obj(["file", "comment"], {"file": ref("Path"), "comment": {"enum": ["//", "#"]}}),
    }),
    "Peer": obj(["trigger", "unrelated", "scheduling"], {
        "trigger": STR,
        "unrelated": array(obj(["file", "comment"], {"file": ref("Path"), "comment": {"enum": ["//", "#"]}}), 1),
        "scheduling": array(obj(["file", "comment"], {"file": ref("Path"), "comment": {"enum": ["//", "#"]}}), 1),
    }),
    "Scenario": obj(["id", "kind", "product", "title", "timeoutMs"], {
        "id": ref("Id"), "kind": ref("Kind"), "product": ref("Id"), "title": ref("Text"),
        "hot": {"type": "integer", "minimum": 1, "maximum": 20},
        "files": {"enum": ["unrelated", "scheduling"]},
        "timeoutMs": {"type": "integer", "minimum": 1000},
    }),
    "Budget": obj(["id", "title", "scenario", "role", "metric", "statistic", "maximum", "enforcement", "source"], {
        "id": ref("Id"), "title": ref("Text"), "scenario": ref("Id"), "role": ref("Role"), "metric": ref("Metric"),
        "statistic": ref("Statistic"), "maximum": {"type": "number", "minimum": 0}, "enforcement": ref("Enforcement"),
        "source": STR, "slo": ref("Id"), "note": ref("Text"),
    }),
    "Set": obj(["version", "labels", "products", "peer", "scenarios", "budgets"], {
        "version": {"const": 1}, "labels": ref("Labels"), "products": array(ref("Product"), 1),
        "peer": ref("Peer"), "scenarios": array(ref("Scenario"), 1), "budgets": array(ref("Budget"), 1),
    }),
    "Clock": obj(["kind", "grainMs"], {"kind": {"enum": POLICY["clockKinds"]}, "grainMs": INT}),
    "Phases": {"type": "object", "additionalProperties": NUM},
    "Record": obj(["scenario", "role", "index", "command", "exit", "killed", "wallMs", "clock", "peakMb", "measure", "executed", "served", "inferred", "phases", "loadPerCore"], {
        "scenario": ref("Id"), "role": ref("Role"), "index": INT, "command": STR,
        "exit": nullable({"type": "integer"}), "killed": nullable({"enum": ["timeout", "abort"]}),
        "wallMs": {"type": "number", "minimum": 0}, "clock": ref("Clock"),
        "peakMb": nullable({"type": "number", "minimum": 0}), "measure": nullable({"enum": POLICY["measures"]}),
        "executed": nullable(INT), "served": BOOL, "inferred": INT, "phases": ref("Phases"), "loadPerCore": nullable({"type": "number", "minimum": 0}),
    }),
    "Outcome": obj(["scenario", "status", "reason", "restored", "landed", "merged"], {
        "scenario": ref("Id"), "status": {"enum": POLICY["outcomeStatuses"]}, "reason": nullable(STR),
        "restored": nullable(BOOL), "landed": nullable(BOOL), "merged": nullable(BOOL),
    }),
    "Row": obj(["id", "scenario", "kind", "role", "metric", "statistic", "value", "maximum", "n", "status", "reason", "enforcement", "bound"], {
        "id": ref("Id"), "scenario": ref("Id"), "kind": {"enum": ["budget", "invariant"]}, "role": nullable(ref("Role")),
        "metric": nullable(ref("Metric")), "statistic": nullable(ref("Statistic")), "value": nullable(NUM), "maximum": nullable(NUM),
        "n": INT, "status": ref("Status"), "reason": ref("Reason"), "enforcement": ref("Enforcement"), "bound": {"enum": ["exact", "upper"]},
    }),
    "Tally": obj(["pass", "fail", "unproven", "blockingFailed", "advisoryFailed", "provenBlocking"], {
        "pass": INT, "fail": INT, "unproven": INT, "blockingFailed": INT, "advisoryFailed": INT, "provenBlocking": INT,
    }),
    "Host": obj(["platform", "arch", "cpus", "memoryBytes", "load1", "loadPerCore", "loadClass", "hostClass"], {
        "platform": STR, "arch": STR, "cpus": {"type": "integer", "minimum": 1}, "memoryBytes": INT,
        "load1": nullable(NUM), "loadPerCore": nullable(NUM), "loadClass": ref("LoadClass"), "hostClass": STR,
    }),
    "Report": obj(["version", "kind", "startedAt", "finishedAt", "host", "hostAfter", "language", "selected", "records", "outcomes", "rows", "tally", "verdict"], {
        "version": {"const": 1}, "kind": {"const": "acceptance-report"}, "startedAt": STR, "finishedAt": STR,
        "host": ref("Host"), "hostAfter": ref("Host"), "language": {"enum": POLICY["languages"]}, "selected": array(ref("Id")),
        "records": array(ref("Record")), "outcomes": array(ref("Outcome")), "rows": array(ref("Row")), "tally": ref("Tally"), "verdict": ref("Verdict"),
    }),
    "Line": {"oneOf": [{"type": "string"}, {"type": "array", "minItems": 2, "maxItems": 2, "items": [{"type": "string"}, {"type": "integer", "minimum": 2}]}]},
    "Case": obj(["id", "title", "source", "scenario", "role", "exit", "wallMs", "clock", "peakResidentMb", "log", "expect"], {
        "id": ref("Id"), "title": ref("Text"), "source": STR, "scenario": ref("Id"), "role": ref("Role"), "exit": {"type": "integer"},
        "wallMs": NUM, "clock": ref("Clock"), "peakResidentMb": nullable(NUM), "log": array(ref("Line")),
        "expect": obj(["executed", "served", "rows"], {"executed": nullable(INT), "served": BOOL, "rows": {"type": "object", "additionalProperties": ref("Status")}}),
    }),
    "Fixture": obj(["version", "about", "cases"], {"version": {"const": 1}, "about": STR, "cases": array(ref("Case"), 1)}),
}

SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Acceptance Scenarios and Budgets",
    "description": "The acceptance scenarios of the build system (hot no-op, peer edit, one-line edit with byte-exact revert per product), the budgets they are held to, the record of every official run and the report of `repo bench acceptance`; every budget is a declared, bilingual row, every verdict a closed row.",
    "type": "object",
    "additionalProperties": False,
    "required": ["version"],
    "properties": {"version": {"const": 1}},
    "definitions": DEFINITIONS,
}

PRODUCTS = [
    {"id": "course", "title": t("Architecture course", "Architekturkurs"), "target": "@teaching/architecture-course:build",
     "edit": {"file": "🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts", "comment": "//"}},
    {"id": "print", "title": t("Print (Rust)", "Druck (Rust)"), "target": "@semio-tech/print-rs:test",
     "edit": {"file": "🧰️framework/🛍️products/📓️print/🦀️.rs", "comment": "//"}},
    {"id": "dashboard", "title": t("Dashboard (Rust binary)", "Dashboard (Rust-Programm)"), "target": "@semio-tech/repo-dashboard-rs:build",
     "edit": {"file": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🦀️.rs", "comment": "//"}},
]

PEER = {
    "trigger": "^[Gg]raph: |Verifying graph facts|build:graph-plan|Inferring projects|Assembling the graph",
    "unrelated": [
        {"file": "CITATION.cff", "comment": "#"},
        {"file": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts", "comment": "//"},
    ],
    "scheduling": [
        {"file": "🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts", "comment": "//"},
    ],
}

MINUTE = 60000


def scenarios():
    rows = []
    for product in PRODUCTS:
        rows.append({"id": product["id"] + "-noop", "kind": "noop", "product": product["id"], "hot": 3,
                     "title": t("%s: hot no-op x3" % product["title"]["en"], "%s: heißer Leerlauf x3" % product["title"]["de"]), "timeoutMs": 90 * MINUTE})
    for product in PRODUCTS:
        rows.append({"id": product["id"] + "-peer-unrelated", "kind": "peer", "product": product["id"], "files": "unrelated",
                     "title": t("%s: edit of an unrelated file of another agent" % product["title"]["en"], "%s: Änderung einer fremden, unabhängigen Datei" % product["title"]["de"]), "timeoutMs": 90 * MINUTE})
    rows.append({"id": "course-peer-scheduling", "kind": "peer", "product": "course", "files": "scheduling",
                 "title": t("Architecture course: edit of a scheduling file of the build library", "Architekturkurs: Änderung einer Planungsdatei der Build-Bibliothek"), "timeoutMs": 90 * MINUTE})
    for product in PRODUCTS:
        rows.append({"id": product["id"] + "-edit", "kind": "edit", "product": product["id"],
                     "title": t("%s: one-line edit and byte-exact revert" % product["title"]["en"], "%s: einzeilige Änderung und byteidentische Rücknahme" % product["title"]["de"]), "timeoutMs": 90 * MINUTE})
    return rows


def budget(identifier, scenario, role, metric, statistic, maximum, enforcement, source, title, slo=None, note=None):
    row = {"id": identifier, "title": title, "scenario": scenario, "role": role, "metric": metric, "statistic": statistic, "maximum": maximum, "enforcement": enforcement, "source": source}
    if slo:
        row["slo"] = slo
    if note:
        row["note"] = note
    return row


BRIEF = "brief-wave-1026g targets"
BLOCK, ADV = "blocking", "advisory"


def budgets():
    rows = []
    name = {p["id"]: p["title"] for p in PRODUCTS}
    for pid in ("course", "dashboard", "print"):
        enforce_wall = ADV if pid == "print" else BLOCK
        wall_note = t("Print answers a hot no-op in 10-45 s today (gate runs g34-g45); it stays advisory until a quiet run holds 10 s.", "Druck beantwortet einen heißen Leerlauf heute in 10-45 s (Gate-Läufe g34-g45); bleibt beratend, bis ein ruhiger Lauf 10 s hält.") if pid == "print" else None
        rows.append(budget(pid + "-noop-wall", pid + "-noop", "hot", "wallMs", "max", 10000, enforce_wall, BRIEF + ": no-op < 10 s",
                           t("%s: hot no-op wall clock" % name[pid]["en"], "%s: heißer Leerlauf, Laufzeit" % name[pid]["de"]), "product-%s-noop-wall-p50" % pid, wall_note))
        rows.append(budget(pid + "-noop-footprint", pid + "-noop", "hot", "peakFootprintMb", "max", 1536, ADV if pid == "print" else BLOCK, BRIEF + ": no-op < 1.5 GB for the whole process tree",
                           t("%s: hot no-op physical footprint" % name[pid]["en"], "%s: heißer Leerlauf, physischer Speicherbedarf" % name[pid]["de"]), "product-%s-noop-rss-max" % pid))
        if pid == "dashboard":
            rows.append(budget(pid + "-noop-executed", pid + "-noop", "hot", "executedActions", "max", 1, BLOCK, BRIEF + ": 0 re-executions; the one allowed action is the freshness worker (`prototype is current`)",
                               t("%s: hot no-op re-executed actions" % name[pid]["en"], "%s: heißer Leerlauf, erneut ausgeführte Aktionen" % name[pid]["de"]), None,
                               t("The dashboard build runs one worker action that reports `prototype is current` and builds nothing; the SLO table asks for 0 and is stricter than the product delivers today.", "Der Dashboard-Bau führt eine Worker-Aktion aus, die `prototype is current` meldet und nichts baut; die SLO-Tabelle verlangt 0 und ist strenger, als das Produkt heute liefert.")))
        else:
            rows.append(budget(pid + "-noop-executed", pid + "-noop", "hot", "executedActions", "max", 0, BLOCK, BRIEF + ": 0 re-executions",
                               t("%s: hot no-op re-executed actions" % name[pid]["en"], "%s: heißer Leerlauf, erneut ausgeführte Aktionen" % name[pid]["de"]), "product-%s-noop-executed-max" % pid))
    peer_limit = {"course": 0, "print": 0, "dashboard": 1}
    for scenario in scenarios():
        if scenario["kind"] != "peer":
            continue
        pid, variant = scenario["product"], scenario["files"]
        for role in ("control", "peer", "follow"):
            what = {"control": ("baseline before the edit", "Grundlauf vor der Änderung"), "peer": ("run while the edit lands", "Lauf, während die Änderung eintrifft"), "follow": ("run after the byte-exact restore", "Lauf nach der byteidentischen Rücknahme")}[role]
            rows.append(budget("%s-%s-executed" % (scenario["id"], role), scenario["id"], role, "executedActions", "max", peer_limit[pid], BLOCK, BRIEF + ": peer edits cause 0 re-executions",
                               t("%s, %s edit: %s, re-executed actions" % (name[pid]["en"], variant, what[0]), "%s, %s-Änderung: %s, erneut ausgeführte Aktionen" % (name[pid]["de"], {"unrelated": "unabhängige", "scheduling": "Planungsdatei-"}[variant], what[1])),
                               "product-%s-peer-%s-executed-max" % (pid, variant) if role == "peer" and pid != "dashboard" else None))
    edit = {"course": (60000, BLOCK, "TypeScript edit < 60 s"), "print": (120000, BLOCK, "Rust edit < 120 s"), "dashboard": (120000, ADV, "Rust edit < 2 min")}
    for pid in ("course", "print", "dashboard"):
        limit, enforce, source = edit[pid]
        note = t("No green measurement of a dashboard edit exists yet (a cargo build of the binary); advisory until the first nightly run holds the budget.", "Von einer Dashboard-Änderung (Cargo-Bau des Programms) gibt es noch keine grüne Messung; beratend, bis der erste nächtliche Lauf das Budget hält.") if pid == "dashboard" else None
        rows.append(budget(pid + "-edit-wall", pid + "-edit", "edit", "wallMs", "max", limit, enforce, BRIEF + ": " + source,
                           t("%s: one-line edit to green" % name[pid]["en"], "%s: einzeilige Änderung bis grün" % name[pid]["de"]), "product-%s-edit-green-p50" % pid if pid in ("course", "dashboard") else None, note))
        rows.append(budget(pid + "-revert-wall", pid + "-edit", "revert", "wallMs", "max", limit, enforce, BRIEF + ": the byte-exact revert is bounded like the edit",
                           t("%s: byte-exact revert to green" % name[pid]["en"], "%s: byteidentische Rücknahme bis grün" % name[pid]["de"]), None, note))
    return rows


def labels():
    pairs = {
        "title": ("Build acceptance report", "Abnahmebericht des Builds"),
        "started": ("Started", "Beginn"), "finished": ("Finished", "Ende"), "host": ("Host class", "Rechnerklasse"),
        "cpus": ("Cores", "Kerne"), "memory": ("Memory", "Speicher"), "load": ("Load per core", "Last je Kern"),
        "summary": ("Summary", "Zusammenfassung"), "pass": ("pass", "bestanden"), "fail": ("fail", "verletzt"), "unproven": ("unproven", "unbewiesen"),
        "blockingFailed": ("blocking budgets broken", "verbindliche Budgets verletzt"), "advisoryFailed": ("advisory budgets broken", "beratende Budgets verletzt"),
        "provenBlocking": ("blocking budgets proven", "verbindliche Budgets bewiesen"),
        "verdictPass": ("ACCEPTANCE PASS", "ABNAHME BESTANDEN"), "verdictFail": ("ACCEPTANCE FAIL", "ABNAHME VERLETZT"), "verdictUnproven": ("ACCEPTANCE UNPROVEN: no blocking budget was proven", "ABNAHME UNBEWIESEN: kein verbindliches Budget wurde bewiesen"),
        "budget": ("Budget", "Vorgabe"), "scenario": ("Scenario", "Szenario"), "run": ("Run", "Lauf"), "statistic": ("Statistic", "Statistik"),
        "value": ("Value", "Wert"), "maximum": ("Maximum", "Höchstwert"), "samples": ("Samples", "Stichproben"), "status": ("Status", "Urteil"),
        "enforcement": ("Enforcement", "Durchsetzung"), "blocking": ("blocking", "verbindlich"), "advisory": ("advisory", "beratend"),
        "notes": ("Notes", "Hinweise"), "runs": ("Runs", "Läufe"), "wall": ("Wall clock", "Laufzeit"), "footprint": ("Footprint", "Speicherbedarf"),
        "executed": ("Executed", "Ausgeführt"), "memo": ("From memo", "Aus Memo"), "exit": ("Exit code", "Exit-Code"), "inferred": ("Inferred", "Neu abgeleitet"),
        "outcomes": ("Scenarios", "Szenarien"), "restored": ("Restored byte for byte", "Byteidentisch zurückgesetzt"), "landed": ("Edit landed during the run", "Änderung traf während des Laufs ein"),
        "yes": ("yes", "ja"), "no": ("no", "nein"), "ran": ("ran", "gelaufen"), "skipped": ("skipped", "übersprungen"), "failed": ("failed", "fehlgeschlagen"),
        "invariantExit": ("Every official run exits with 0", "Jeder offizielle Lauf endet mit 0"),
        "invariantRestored": ("Edited files are restored byte for byte", "Geänderte Dateien werden byteidentisch zurückgesetzt"),
        "invariantLanded": ("The peer edit lands while the run executes", "Die fremde Änderung trifft ein, während der Lauf ausgeführt wird"),
        "prime": ("prime (not judged)", "Vorlauf (nicht beurteilt)"), "hot": ("hot", "heiß"), "control": ("control", "Grundlauf"), "peer": ("peer edit", "fremde Änderung"),
        "follow": ("follow-up", "Folgelauf"), "edit": ("edit", "Änderung"), "revert": ("revert", "Rücknahme"),
        "no-samples": ("no sample of this run exists", "von diesem Lauf gibt es keine Stichprobe"),
        "run-failed": ("the run failed, so it measures nothing", "der Lauf ist fehlgeschlagen und misst deshalb nichts"),
        "clock-grain": ("the recorded clock is a poll with a coarse grain; the value lies within one grain of the maximum", "die aufgezeichnete Uhr fragt nur grob ab; der Wert liegt innerhalb einer Stufe des Höchstwerts"),
        "host-saturated": ("the host was saturated; load only ever inflates times", "der Rechner war ausgelastet; Last bläht Zeiten nur auf"),
        "resident-only": ("only the resident set was observed, which over-reads shared pages", "nur der resident Speicher wurde beobachtet, der geteilte Seiten mehrfach zählt"),
        "not-landed": ("the edit did not land while the run was executing, so the pass proves nothing", "die Änderung traf nicht während des Laufs ein, der Erfolg beweist daher nichts"),
        "foreign-write": ("another writer changed the file during the stage; its change was left in place", "ein anderer Schreiber hat die Datei während der Stufe geändert; seine Änderung blieb bestehen"),
        "skipped-reason": ("skipped: edits need --in-place", "übersprungen: Änderungen brauchen --in-place"),
        "aborted": ("the run was cancelled", "der Lauf wurde abgebrochen"),
        "skipped": ("skipped", "übersprungen"),
        "noteBounds": ("Values marked ≤ are upper bounds: the recorded clock rounds up to its grain.", "Mit ≤ markierte Werte sind obere Schranken: die aufgezeichnete Uhr rundet auf ihre Stufe auf."),
        "noteMeasure": ("Memory is the physical footprint of the whole process tree (macOS phys_footprint, Linux proportional set size, Windows private working set), sampled by the capacity governor's observer.", "Der Speicher ist der physische Bedarf des ganzen Prozessbaums (macOS phys_footprint, Linux proportionale Größe, Windows privater Arbeitssatz), gemessen vom Beobachter der Kapazitätssteuerung."),
        "noteMarks": ("! marks a blocking budget.", "! markiert ein verbindliches Budget."),
        "list": ("Acceptance scenarios and budgets", "Abnahmeszenarien und Budgets"), "name": ("Name", "Bezeichnung"), "source": ("Source", "Quelle"), "kind": ("Kind", "Art"),
        "noop": ("hot no-op", "heißer Leerlauf"),
        "quiet": ("quiet", "ruhig"), "busy": ("busy", "beschäftigt"), "saturated": ("saturated", "ausgelastet"), "unknown": ("unknown", "unbekannt"),
    }
    return {k: t(*v) for k, v in pairs.items()}


def budget_set():
    return {"version": 1, "labels": labels(), "products": PRODUCTS, "peer": PEER, "scenarios": scenarios(), "budgets": budgets()}


def dump(value):
    return json.dumps(value, ensure_ascii=False, indent=1) + "\n"


if __name__ == "__main__":
    print(len(budgets()), "budgets", len(scenarios()), "scenarios")
