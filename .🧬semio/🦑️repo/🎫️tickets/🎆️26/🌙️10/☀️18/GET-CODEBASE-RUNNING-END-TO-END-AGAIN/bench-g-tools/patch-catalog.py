"""Adds `--history` to the `bench` entry of the operations catalogue by text edits (the file is hand-formatted, so no JSON round trip).
usage: patch-catalog.py <catalog.json>   (idempotent: refuses when the flag is already there)
"""
import sys

path = sys.argv[1]
s = open(path, encoding="utf8").read()
if '"name": "--history"' in s:
    raise SystemExit("already patched")

def once(old, new):
    global s
    if s.count(old) != 1:
        raise SystemExit("anchor not unique (%d): %s" % (s.count(old), old[:70]))
    s = s.replace(old, new)

once('''    {
      "name": "--current",
''', '''    {
      "name": "--history",
      "value": "required",
      "valueName": "folder",
      "operations": ["bench"],
      "description": { "en": "bench run: store the report in this folder as <UTC start>-<host class>.json and compare with the newest earlier report of this host class when no --baseline is given; bench compare: compare the two newest reports of this host class.", "de": "bench run: legt den Bericht in diesem Ordner als <UTC-Start>-<Rechnerklasse>.json ab und vergleicht ohne --baseline mit dem jüngsten früheren Bericht dieser Rechnerklasse; bench compare: vergleicht die beiden jüngsten Berichte dieser Rechnerklasse." }
    },
    {
      "name": "--current",
''')
once('"synopsis": "bench run [--slo=<ids|all>] [--runs=<n>] [--baseline=<report>] [--save-baseline=<report>] [--prepare]", "flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--markdown"',
     '"synopsis": "bench run [--slo=<ids|all>] [--runs=<n>] [--baseline=<report>] [--save-baseline=<report>] [--history=<folder>] [--prepare]", "flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--history", "--markdown"')
once('"synopsis": "bench compare --current=<report> --baseline=<report>", "flags": ["--current", "--baseline", "--markdown", "--json"], "summary": { "en": "Compare two stored reports of the same host class and name every regression beyond the tolerance of its objective.", "de": "Vergleicht zwei gespeicherte Berichte derselben Rechnerklasse und nennt jede Verschlechterung über die Toleranz ihres Ziels hinaus." }',
     '"synopsis": "bench compare --current=<report> --baseline=<report> | --history=<folder>", "flags": ["--current", "--baseline", "--history", "--markdown", "--json"], "summary": { "en": "Compare two stored reports of the same host class, name every regression beyond the tolerance of its objective and every blocking objective over its budget.", "de": "Vergleicht zwei gespeicherte Berichte derselben Rechnerklasse und nennt jede Verschlechterung über die Toleranz ihres Ziels sowie jedes verbindliche Ziel über dem Budget." }')
once('"flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--current", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json"],\n      "examples"',
     '"flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--current", "--history", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json"],\n      "examples"')
open(path, "w", encoding="utf8").write(s)
print("patched")
