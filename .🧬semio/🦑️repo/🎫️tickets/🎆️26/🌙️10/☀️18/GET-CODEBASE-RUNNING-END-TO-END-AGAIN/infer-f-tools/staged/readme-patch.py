"""Adds the infer-f bullets (EN + DE) to the repo README after the queue-coalescing bullets; run once with the batch."""
import sys
p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/README.md"
s = open(p, encoding="utf8").read()
en = "- **Inference progress and workers (graph builder).** The progress line of the inference phase names the projects that are running, how many run and for how long the oldest one has run, re-announced every 5 s, so a slow project is never anonymous (`Inferring projects: 474 of 566 projects · 1 running, longest 540 s: @scope/name`; German: `… · 1 laufen, längstes 540 s: …`). The inference farm no longer leaves a saturated host to the parent process: the moment the farm engages (the delta is cold) the worker that the capacity grant of the builder already pays for (`SEMIO_HOST_CAPACITY_PLAN`: one job and its memory) starts a fresh process, the host is asked for more workers in parallel, and any host grant of at least one worker starts a pool of its own (before, a grant below two workers meant that the parent inferred every project alone: on 2026-10-26 a builder needed more than 100 minutes for 566 projects while a fresh worker process infers a project in 0.4 s on average, the heaviest in 50-80 s and 2-3 GB).\n"
de = "- **Inferenz-Fortschritt und Arbeitsprozesse (Graph-Builder).** Die Fortschrittszeile der Inferenzphase nennt die Projekte, die gerade laufen, wie viele laufen und wie lange das älteste läuft, alle 5 s neu, damit ein langsames Projekt nie anonym bleibt (`Projekte ableiten: 474 von 566 Projekte · 1 laufen, längstes 540 s: @scope/name`). Die Inferenz-Farm überlässt einen ausgelasteten Host nicht mehr dem Elternprozess: Sobald die Farm einsetzt (das Delta ist kalt), startet der Arbeitsprozess, den die Kapazitätszusage des Builders ohnehin bezahlt (`SEMIO_HOST_CAPACITY_PLAN`: ein Job und sein Speicher), als frischer Prozess, beim Host werden parallel weitere Arbeitsprozesse angefragt, und jede Host-Zusage von mindestens einem Arbeitsprozess startet einen eigenen Pool (vorher bedeutete eine Zusage unter zwei Arbeitsprozessen, dass der Elternprozess jedes Projekt allein ableitete: am 2026-10-26 brauchte ein Builder für 566 Projekte über 100 Minuten, während ein frischer Arbeitsprozess ein Projekt im Mittel in 0,4 s ableitet, das schwerste in 50-80 s und 2-3 GB).\n"
for anchor, text in (("- **Queue coalescing (graph builder).**", en), ("- **Warteschlangen-Bündelung (Graph-Builder).**", de)):
    at = s.index(anchor)
    end = s.index("\n", at) + 1
    if text[:40] in s:
        sys.exit("already applied")
    s = s[:end] + text + s[end:]
open(p, "w", encoding="utf8").write(s)
print("readme patched")
