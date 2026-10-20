#!/usr/bin/env python3
# Generates staged/help.json (EN+DE) from one table and asserts every schema code, check and status is covered.
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
schema = json.load(open(HERE + "/staged/schema.json", encoding="utf-8"))
T = lambda en, de: {"en": en, "de": de}
codes = {
 "observation-error": T("A tool could not be observed.", "Ein Werkzeug konnte nicht beobachtet werden."),
 "plan-unavailable": T("A tool printed no task graph for the dry run.", "Ein Werkzeug hat für den Probelauf keinen Aufgabengraphen ausgegeben."),
 "task-missing-nx": T("Bazel plans a task that Nx does not.", "Bazel plant eine Aufgabe, die Nx nicht plant."),
 "task-missing-bazel": T("Nx plans a task that Bazel does not.", "Nx plant eine Aufgabe, die Bazel nicht plant."),
 "edge-missing-nx": T("Bazel orders two tasks that Nx leaves unordered.", "Bazel ordnet zwei Aufgaben, die Nx ungeordnet lässt."),
 "edge-missing-bazel": T("Nx orders two tasks that Bazel leaves unordered.", "Nx ordnet zwei Aufgaben, die Bazel ungeordnet lässt."),
 "root-differs": T("The tools start from different root tasks.", "Die Werkzeuge beginnen mit verschiedenen Wurzelaufgaben."),
 "continuous-edge-differs": T("The tools disagree on which dependency only has to be running.", "Die Werkzeuge sind uneins, welche Abhängigkeit nur laufen muss."),
 "closure-unavailable": T("The closure could not be compared with the Bazel query.", "Der Abschluss konnte nicht mit der Bazel-Abfrage verglichen werden."),
 "closure-task-missing-in-query": T("The planner selects a task that Bazel's cquery does not return.", "Der Planer wählt eine Aufgabe, die Bazels cquery nicht liefert."),
 "closure-task-extra-in-query": T("Bazel's cquery returns a task the planner does not select.", "Bazels cquery liefert eine Aufgabe, die der Planer nicht wählt."),
 "output-missing-nx": T("Only Bazel produced this declared output.", "Nur Bazel hat diese deklarierte Ausgabe erzeugt."),
 "output-missing-bazel": T("Only Nx produced this declared output.", "Nur Nx hat diese deklarierte Ausgabe erzeugt."),
 "output-kind-differs": T("The output is a file for one tool and a directory or link for the other.", "Die Ausgabe ist bei einem Werkzeug eine Datei und beim anderen ein Verzeichnis oder Link."),
 "output-link-differs": T("The symbolic links point to different targets.", "Die symbolischen Links zeigen auf verschiedene Ziele."),
 "output-mode-differs": T("The permission bits differ.", "Die Zugriffsrechte unterscheiden sich."),
 "output-bytes-differ": T("The output files differ byte for byte.", "Die Ausgabedateien unterscheiden sich Byte für Byte."),
 "output-root-differs": T("The tools attribute different declared roots to the task.", "Die Werkzeuge ordnen der Aufgabe verschiedene deklarierte Wurzeln zu."),
 "run-unavailable": T("A tool left no run result.", "Ein Werkzeug hat kein Laufergebnis hinterlassen."),
 "exit-differs": T("The tools end with different exit codes.", "Die Werkzeuge enden mit verschiedenen Exit-Codes."),
 "exit-unexpected": T("A tool ends with another exit code than the sample expects.", "Ein Werkzeug endet mit einem anderen Exit-Code, als die Stichprobe erwartet."),
 "reason-differs": T("The runs ended for different reasons.", "Die Läufe endeten aus verschiedenen Gründen."),
 "failed-tasks-differ": T("The tools report different failed tasks.", "Die Werkzeuge melden verschiedene fehlgeschlagene Aufgaben."),
 "test-status-differs": T("A test task has a different result.", "Eine Testaufgabe hat ein anderes Ergebnis."),
 "inputs-unavailable": T("The inputs of a task are known for only one tool.", "Die Eingaben einer Aufgabe sind nur für ein Werkzeug bekannt."),
 "input-file-only-nx": T("Nx hashes a file that Bazel does not hand to the action.", "Nx hasht eine Datei, die Bazel der Aktion nicht übergibt."),
 "input-file-only-bazel": T("Bazel hands the action a file that Nx does not hash.", "Bazel übergibt der Aktion eine Datei, die Nx nicht hasht."),
 "env-hashed-not-visible": T("Nx hashes an environment variable the Bazel action does not see.", "Nx hasht eine Umgebungsvariable, die die Bazel-Aktion nicht sieht."),
 "env-visible-not-hashed": T("The Bazel action sees an environment variable Nx does not hash and no documented rule explains.", "Die Bazel-Aktion sieht eine Umgebungsvariable, die Nx nicht hasht und die keine dokumentierte Regel erklärt."),
 "external-uncovered": T("Nx hashes external packages but Bazel receives no lock file.", "Nx hasht externe Pakete, aber Bazel erhält keine Sperrdatei."),
 "runtime-uncovered": T("Nx hashes a runtime value but Bazel receives no matching toolchain input.", "Nx hasht einen Laufzeitwert, aber Bazel erhält keine passende Toolchain-Eingabe."),
 "restore-missing": T("A cache restore lacks an output (or invents one) compared with the cold run.", "Eine Cache-Wiederherstellung lässt eine Ausgabe aus (oder erfindet eine) gegenüber dem kalten Lauf."),
 "restore-kind-differs": T("A cache restore changes the kind of an output.", "Eine Cache-Wiederherstellung ändert die Art einer Ausgabe."),
 "restore-mode-differs": T("A cache restore changes the permission bits of an output.", "Eine Cache-Wiederherstellung ändert die Zugriffsrechte einer Ausgabe."),
 "restore-bytes-differ": T("A cache restore changes the bytes of an output.", "Eine Cache-Wiederherstellung ändert die Bytes einer Ausgabe."),
}
checks = {
 "graph": T("Task graph", "Aufgabengraph"), "closure": T("Closure against Bazel cquery", "Abschluss gegen Bazel-cquery"),
 "outputs": T("Declared outputs", "Deklarierte Ausgaben"), "results": T("Exit codes and test results", "Exit-Codes und Testergebnisse"),
 "inputs": T("Inputs and environment", "Eingaben und Umgebung"), "restore": T("Cache restore", "Cache-Wiederherstellung"), "observation": T("Observation", "Beobachtung"),
}
status = {
 "equal": T("equal", "gleich"), "diverged": T("diverged", "abweichend"), "error": T("error", "Fehler"),
 "not-applicable": T("not applicable", "nicht anwendbar"), "unavailable": T("unavailable", "nicht verfügbar"),
}
labels = {
 "title": T("Build parity: Nx and Bazel", "Build-Parität: Nx und Bazel"),
 "sample": T("Sample", "Stichprobe"), "area": T("Area", "Bereich"), "target": T("Target", "Ziel"), "status": T("Status", "Status"),
 "check": T("Check", "Prüfung"), "phase": T("Phase", "Phase"), "code": T("Divergence", "Abweichung"), "subject": T("Subject", "Gegenstand"),
 "nx": T("Nx", "Nx"), "bazel": T("Bazel", "Bazel"), "divergences": T("Divergences", "Abweichungen"), "rules": T("Documented normalizations", "Dokumentierte Normalisierungen"),
 "rule": T("Rule", "Regel"), "applied": T("Applied", "Angewandt"), "justification": T("Justification", "Begründung"),
 "usage": T("Usage", "Aufruf"), "options": T("Options", "Optionen"), "examples": T("Examples", "Beispiele"),
 "totals": T("Totals", "Summen"), "samples": T("Samples", "Stichproben"), "none": T("No divergences.", "Keine Abweichungen."),
 "verdict-equal": T("Both implementations agree on every sample.", "Beide Implementierungen stimmen bei jeder Stichprobe überein."),
 "verdict-diverged": T("The implementations diverge.", "Die Implementierungen weichen voneinander ab."),
 "evidence": T("Evidence directory", "Beweisverzeichnis"), "recovered": T("Restored output roots of interrupted runs", "Wiederhergestellte Ausgabewurzeln unterbrochener Läufe"),
}
flags = [
 {"name": "--samples", "valueName": "file", "description": T("Sample set to verify (default: the authored set beside the verifier).", "Zu prüfende Stichprobenmenge (Standard: die mitgelieferte Menge neben dem Prüfer).")},
 {"name": "--only", "valueName": "ids", "description": T("Comma list of sample ids or area:<area> to verify.", "Kommaliste von Stichproben-IDs oder area:<Bereich>, die geprüft werden.")},
 {"name": "--plans-only", "valueName": None, "description": T("Compare only the dry-run task graphs; nothing is executed.", "Nur die Aufgabengraphen der Probeläufe vergleichen; nichts wird ausgeführt.")},
 {"name": "--out", "valueName": "directory", "description": T("Evidence directory; must be empty or absent (default: a fresh directory in the test artifact area).", "Beweisverzeichnis; muss leer oder nicht vorhanden sein (Standard: ein neues Verzeichnis im Testartefaktbereich).")},
 {"name": "--keep", "valueName": None, "description": T("Keep the captured outputs of samples without divergences.", "Erfasste Ausgaben von Stichproben ohne Abweichungen behalten.")},
 {"name": "--json", "valueName": None, "description": T("Print the report as JSON instead of tables.", "Den Bericht als JSON statt als Tabellen ausgeben.")},
 {"name": "--recover", "valueName": "directory", "description": T("Restore the output roots an interrupted verification set aside, then stop.", "Die von einer unterbrochenen Prüfung beiseitegelegten Ausgabewurzeln wiederherstellen und beenden.")},
 {"name": "--lang", "valueName": "en|de", "description": T("Language of help and report; otherwise SEMIO_LANGUAGE, then the locale, then English.", "Sprache von Hilfe und Bericht; sonst SEMIO_LANGUAGE, dann die Locale, dann Englisch.")},
 {"name": "--help", "valueName": None, "description": T("Print this help and stop.", "Diese Hilfe ausgeben und beenden.")},
]
doc = {
 "version": 1, "languages": ["en", "de"],
 "command": {
  "name": "verify build-parity",
  "synopsis": "verify build-parity [--samples=<file>] [--only=<ids>] [--plans-only] [--out=<directory>] [--keep] [--json] [--lang=en|de]",
  "summary": T("Runs every sample target with Nx and with Bazel into separate output roots and compares task graph, declared outputs byte for byte, exit codes and test results, and the inputs each tool sees.", "Führt jede Stichprobe mit Nx und mit Bazel in getrennte Ausgabewurzeln aus und vergleicht Aufgabengraph, deklarierte Ausgaben Byte für Byte, Exit-Codes und Testergebnisse sowie die Eingaben, die jedes Werkzeug sieht."),
  "flags": flags,
  "examples": [
   {"command": "repo verify build-parity --plans-only", "description": T("Compare only the planned task graphs of all samples.", "Nur die geplanten Aufgabengraphen aller Stichproben vergleichen.")},
   {"command": "repo verify build-parity --only=area:teaching", "description": T("Run the teaching samples with both tools and compare everything.", "Die Stichproben des Bereichs teaching mit beiden Werkzeugen ausführen und alles vergleichen.")},
   {"command": "repo verify build-parity --recover=<directory>", "description": T("Put back outputs after a cancelled verification.", "Ausgaben nach einer abgebrochenen Prüfung zurücklegen.")},
  ],
  "notes": [
   T("Nothing is normalized except what rules.json documents with a justification; every other difference is a divergence and ends the command with a non-zero exit code.", "Nichts wird normalisiert außer dem, was rules.json mit Begründung dokumentiert; jede andere Abweichung beendet den Befehl mit einem Exit-Code ungleich null."),
   T("Declared output roots are set aside before each run and restored afterwards; Ctrl-C restores them as well.", "Deklarierte Ausgabewurzeln werden vor jedem Lauf beiseitegelegt und danach zurückgelegt; auch Strg-C stellt sie wieder her."),
  ],
 },
 "labels": labels, "status": status, "checks": checks, "codes": codes,
}
enum = schema["definitions"]["Code"]["enum"]
missing = [c for c in enum if c not in codes]; extra = [c for c in codes if c not in enum]
assert not missing and not extra, (missing, extra)
assert set(checks) == set(schema["definitions"]["ReportedCheck"]["enum"])
for f in flags:
    if f["valueName"] is None: del f["valueName"]
json.dump(doc, open(HERE + "/staged/help.json", "w", encoding="utf-8"), indent=2, ensure_ascii=False); open(HERE + "/staged/help.json", "a").write("\n")
print("codes", len(codes))
