#!/usr/bin/env python3
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=LIB+"🏗️build/"
DOMAINS=[
 ("resident-service",[B+"🏡️resident/"],
  "The resident build service: the warm process pool, the watcher, the journal, its protocol and endpoint, consent and the graph hooks. It decides when and where a command runs and which process serves it, never what the command writes; an edit of this code moves no task key and no project of the build graph.",
  "Der residente Build-Dienst: Warmprozess-Pool, Beobachter, Journal, Protokoll und Endpunkt, Einwilligung und Graph-Haken. Er entscheidet, wann und wo ein Befehl läuft und welcher Prozess ihn bedient, nie was der Befehl schreibt; eine Änderung dieses Codes bewegt keinen Task-Schlüssel und kein Projekt des Build-Graphen."),
 ("graph-demand",[B+"🪪️graph-state/🙋️wanted/"],
  "The demand registry of the graph server: which projects a command wants served first, its cost estimate, reader and server. It orders the work of the graph server, never what a project infers or what a task computes.",
  "Das Bedarfsregister des Graph-Servers: welche Projekte ein Befehl zuerst bedient haben möchte, seine Kostenschätzung, Leser und Server. Es ordnet die Arbeit des Graph-Servers, nie was ein Projekt ableitet oder was ein Task berechnet."),
 ("remote-execution",[B+"🌐️remote-execution/"],
  "Remote execution: the scheduler, worker, agent, sidecars, load reports, offload admission and wire protocol. They decide on which machine an action runs, never what it computes; the same action yields the same result wherever it runs.",
  "Entfernte Ausführung: Scheduler, Worker, Agent, Sidecars, Lastberichte, Auslagerungszulassung und Drahtprotokoll. Sie entscheiden, auf welcher Maschine eine Aktion läuft, nie was sie berechnet; dieselbe Aktion liefert überall dasselbe Ergebnis."),
 ("memory-governor",[B+"🧠️memory/"],
  "The memory governor: pressure sampling, the grant of memory to a plan stage and the settling of retained memory. It decides how much memory a stage may hold, never what it produces.",
  "Der Speicher-Regler: Drucksamplung, die Speicherzuteilung an eine Planstufe und das Abräumen gehaltenen Speichers. Er entscheidet, wie viel Speicher eine Stufe halten darf, nie was sie erzeugt."),
 ("diagnostics",[B+"🩺️doctor/",B+"🚢️fleet-status/",B+"⏱️profile/",B+"📏️incremental/"],
  "Diagnostics and measurement: the doctor, the fleet status, the profiler and the incremental benchmarks. They observe and report builds, never change what a build writes.",
  "Diagnose und Messung: der Doktor, der Flottenstatus, der Profiler und die inkrementellen Messungen. Sie beobachten und berichten Builds, ändern nie was ein Build schreibt."),
 ("worker-supervision",[B+"👷️workers/",B+"🧵️workers/",B+"🪪️agent-sessions/",B+"🗓️scheduler/",B+"🧱️job-limits/"],
  "Supervision of work: the worker pool and its protocol, agent sessions, the job scheduler and its learning, and the job limits. They decide when, how many and under which limits actions run, never what they compute.",
  "Beaufsichtigung der Arbeit: der Worker-Pool und sein Protokoll, Agentensitzungen, der Job-Scheduler und sein Lernen sowie die Job-Grenzen. Sie entscheiden, wann, wie viele und unter welchen Grenzen Aktionen laufen, nie was sie berechnen."),
]
