# repo doctor

English | [Deutsch](#deutsch)

`repo doctor` answers in one shot why work is waiting, slow or failing when many agents share one worktree. It only reads state: it takes no lease and no capacity grant, so it answers even while a build holds everything.

```
repo doctor [--json] [--fix [--dry-run]] [--only=<area>[,<area>]] [--strict] [--facts] [--lang=en|de]
```

- Areas: `tools` (registry pins against what is installed), `storage` (images: attached, free space against budget and floor, unsettled publications), `leases` (holders, waiters, who blocks whom, cycles, order violations, stale registry rows, dead queue tickets), `capacity` and `host` (grants, waiters and their reason, memory, swap, load, open files, attached images), `sessions` (live, idle and gone agents), `graph` (published generation, damaged store, implementation newer than the generation), `dependencies` (pointers, ready generations, newer scope inputs, superseded and unfinished generations), `caches` (dead-process publications, retired and undeclared roots) and `processes` (running repo processes with their phase).
- Every finding names its severity (`error`, `warn`, `info`), its owner and the exact `repo` command that resolves it, or says to wait or whom to ask. The wait chains show for every waiting process who it stands behind and the root cause.
- Exit status: 0 when no error remains (`--strict`: no warning either), 1 otherwise, 130 when cancelled. Progress goes to standard error; `--json` prints exactly one report, `--facts` the raw facts the findings are derived from.
- `--fix` applies only the repairs declared safe in `🔣️policy.json`: publications of dead processes in the graph image, lease registry rows of dead processes and folders of vanished lease directories, gone agent sessions. Each repair re-derives its targets from the live state and never follows links. `--fix --dry-run` lists what would go.

Design: schema first. `🧬️schema/🔣️.json` defines policy, rule catalogue, facts and report; `📖️rules` holds the rules with their EN/DE texts, owner and fix; `🔣️policy.json` the thresholds. A probe reads one area through ports and returns facts; `🧮️analysis` is a pure function from facts to findings; `🖨️report` renders. Add a rule: add it to the catalogue, emit it in the analysis, add a hand-stated case to `🧪️tests/🧫️fixtures/🔣️.json`; the tests enforce that every rule is exercised, that EN and DE texts use the same data, and that an independent Python implementation (`🧪️tests/🐍️.py`) reaches the same findings.

## Deutsch

`repo doctor` beantwortet auf einmal, warum Arbeit wartet, langsam ist oder scheitert, wenn viele Agenten ein Arbeitsverzeichnis teilen. Es liest nur Zustand: Es nimmt weder eine Lease noch eine Kapazitätszusage und antwortet daher auch dann, wenn ein Build alles hält.

- Bereiche: `tools` (Pins der Registry gegen das Installierte), `storage` (Images: eingehängt, freier Platz gegen Budget und Untergrenze, nicht abgeschlossene Veröffentlichungen), `leases` (Halter, Wartende, wer wen blockiert, Zyklen, Reihenfolgeverletzungen, veraltete Registerzeilen, Tickets toter Prozesse), `capacity` und `host` (Zusagen, Wartende und ihr Grund, Speicher, Auslagerung, Last, offene Dateien, eingehängte Images), `sessions` (lebende, untätige und beendete Agenten), `graph` (veröffentlichte Generation, beschädigter Speicher, Implementierung neuer als die Generation), `dependencies` (Zeiger, fertige Generationen, neuere Eingaben eines Bereichs, abgelöste und unfertige Generationen), `caches` (Veröffentlichungen toter Prozesse, stillgelegte und nicht deklarierte Wurzeln) und `processes` (laufende repo-Prozesse mit ihrer Phase).
- Jeder Befund nennt Schweregrad (`error`, `warn`, `info`), Eigentümer und den genauen `repo`-Befehl, der ihn behebt, oder sagt, dass man abwarten oder wen man fragen soll. Die Wartebäume zeigen für jeden wartenden Prozess, hinter wem er steht, und die Ursache.
- Exit-Status: 0, wenn kein Fehler bleibt (`--strict`: auch keine Warnung), sonst 1, 130 bei Abbruch. Der Fortschritt geht auf die Fehlerausgabe; `--json` gibt genau einen Bericht aus, `--facts` die rohen Fakten, aus denen die Befunde abgeleitet sind.
- `--fix` wendet nur Reparaturen an, die in `🔣️policy.json` als sicher deklariert sind: Veröffentlichungen toter Prozesse im Graph-Image, Lease-Registerzeilen toter Prozesse und Ordner verschwundener Lease-Verzeichnisse, beendete Agent-Sitzungen. Jede Reparatur leitet ihre Ziele neu aus dem lebenden Zustand ab und folgt nie Verknüpfungen. `--fix --dry-run` listet auf, was verschwinden würde.

Aufbau: Schema zuerst. `🧬️schema/🔣️.json` definiert Richtlinie, Regelkatalog, Fakten und Bericht; `📖️rules` enthält die Regeln mit ihren EN/DE-Texten, Eigentümer und Behebung; `🔣️policy.json` die Schwellen. Eine Sonde liest einen Bereich über Ports und liefert Fakten; `🧮️analysis` ist eine reine Funktion von Fakten zu Befunden; `🖨️report` stellt dar. Eine Regel ergänzen: in den Katalog aufnehmen, in der Analyse auslösen und einen von Hand formulierten Fall in `🧪️tests/🧫️fixtures/🔣️.json` aufnehmen; die Tests erzwingen, dass jede Regel geprüft wird, dass EN- und DE-Texte dieselben Daten benutzen und dass eine unabhängige Python-Implementierung (`🧪️tests/🐍️.py`) zu denselben Befunden kommt.
