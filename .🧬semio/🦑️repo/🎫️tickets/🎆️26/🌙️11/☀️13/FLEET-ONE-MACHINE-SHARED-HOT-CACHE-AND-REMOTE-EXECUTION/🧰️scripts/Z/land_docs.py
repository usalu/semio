#!/usr/bin/env python3
"""land_docs.py check|apply: the port wording of the remote-execution help (catalog and its help snapshot) and the fleet bullet of the README, as anchored edits on the live files, read right before the write."""
import os,sys
LIVE='/Users/Shared/semio';P=LIVE+'/🧰️framework/🛍️products/🦑️repo'
OPS=P+'/🔨️modules/📚️library/🏗️build/⚙️operations'
mode=sys.argv[1] if len(sys.argv)>1 else 'check'
EN_OLD='Run the service on 127.0.0.1:8980 with the shared store and one local worker; hermetic Bazel actions can then run remotely.'
EN_NEW='Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.'
DE_OLD='Betreibt den Dienst auf 127.0.0.1:8980 mit dem gemeinsamen Speicher und einem lokalen Worker; hermetische Bazel-Aktionen können dann remote laufen.'
DE_NEW='Betreibt den Dienst mit dem gemeinsamen Speicher und einem lokalen Worker: gRPC für Build-Werkzeuge auf 127.0.0.1:8980, HTTP für Worker, Status und die Lasttabelle der Flotte auf 127.0.0.1:8981; hermetische Bazel-Aktionen können dann remote laufen.'
README_EN_ANCHOR='- **In-house service.** `repo remote-execution serve|status|workers`'
README_DE_ANCHOR='- **Hauseigener Dienst.** `repo remote-execution serve|status|workers`'
BULLET_EN='- **Fleet scheduler, workers and offload.** On a joined fleet machine the replica daemon runs the build agent (worker and load reporter, one process) and, on the group server, the scheduler; nothing is configured by hand. Ports: the scheduler\'s gRPC listener is 8980 (the group\'s facts publish it as `execution.endpoint`), its HTTP listener is 8981 (workers, `GET /healthz`, `GET`/`POST /v1/fleet/load`, the table `repo fleet status` reads). The agent reports its machine\'s load every 5 seconds and keeps its worker registered; free disk space is judged with every admission, not once at start: a worker runs as many actions at once as its volume allows (`worker.disk`: 3 GiB stay free for the machine and 0.5 GiB per action, the largest compiler output measured is 140 MB), says `disk-full` with the numbers while it refuses for that reason and serves again within seconds of the space coming back, without a restart. A role that crashes (for example on a module that is missing for a moment while a landing replaces a folder) is started again by its launcher after 1 second, doubling to 15 seconds. `--offload=always|auto|never` (or `SEMIO_OFFLOAD`) decides per unit; `auto` hands a unit to another machine only when this machine would make it wait. Every decline and fall-back is counted under its reason in `.cache/f/offload-counters/reason/`, and every offloaded unit is logged in `x/offload.jsonl` of the unit cache with where it ran, its round trip and the bytes shipped, uploaded and returned. A worker refuses every action its policy does not name (`🧱️actions`); what the planner sends must pass that policy, which the determinism test asserts.'
BULLET_DE='- **Flotten-Scheduler, Worker und Auslagerung.** Auf einem beigetretenen Flottenrechner startet der Replikat-Daemon den Build-Agenten (Worker und Lastmelder, ein Prozess) und auf dem Gruppenserver den Scheduler; nichts wird von Hand eingerichtet. Ports: Der gRPC-Listener des Schedulers ist 8980 (die Flottenfakten der Gruppe veröffentlichen ihn als `execution.endpoint`), sein HTTP-Listener ist 8981 (Worker, `GET /healthz`, `GET`/`POST /v1/fleet/load`, die Tabelle, die `repo fleet status` liest). Der Agent meldet die Last seines Rechners alle 5 Sekunden und hält seinen Worker angemeldet; der freie Speicherplatz wird bei jeder Zulassung beurteilt, nicht einmal beim Start: Ein Worker führt so viele Aktionen gleichzeitig aus, wie sein Datenträger erlaubt (`worker.disk`: 3 GiB bleiben für den Rechner frei und 0,5 GiB je Aktion, die grösste gemessene Compiler-Ausgabe ist 140 MB), nennt `disk-full` mit den Zahlen, solange er deshalb ablehnt, und arbeitet binnen Sekunden wieder, wenn der Platz zurück ist, ohne Neustart. Eine Rolle, die abstürzt (etwa an einem Modul, das für einen Moment fehlt, während eine Landung einen Ordner ersetzt), startet ihr Starter nach 1 Sekunde neu, mit Verdopplung bis 15 Sekunden. `--offload=always|auto|never` (oder `SEMIO_OFFLOAD`) entscheidet je Einheit; `auto` gibt eine Einheit nur dann an einen anderen Rechner ab, wenn dieser Rechner sie warten lassen würde. Jede Ablehnung und jeder Rückfall wird unter seinem Grund in `.cache/f/offload-counters/reason/` gezählt, und jede ausgelagerte Einheit steht in `x/offload.jsonl` des Einheiten-Caches mit Ort, Umlaufzeit und den gesendeten, hochgeladenen und zurückgekommenen Bytes. Ein Worker lehnt jede Aktion ab, die seine Richtlinie nicht nennt (`🧱️actions`); was der Planer sendet, muss diese Richtlinie bestehen, was der Determinismus-Test prüft.'
def plan():
    out=[]
    for rel,pairs in ((OPS+'/🔣️catalog.json',[(EN_OLD,EN_NEW),(DE_OLD,DE_NEW)]),(OPS+'/🧪️tests/🧫️fixtures/📖️help/🔣️.json',[(EN_OLD,EN_NEW),(DE_OLD,DE_NEW)])):
        s=open(rel,encoding='utf8').read();n=0
        for a,b in pairs:
            n+=s.count(a);s=s.replace(a,b)
        out.append((rel,s,n))
    s=open(P+'/README.md',encoding='utf8').read();lines=s.split('\n');n=0
    for index,line in enumerate(lines):
        if line.startswith(README_EN_ANCHOR) and BULLET_EN not in s:lines.insert(index+1,BULLET_EN);n+=1;break
    for index,line in enumerate(lines):
        if line.startswith(README_DE_ANCHOR) and BULLET_DE not in '\n'.join(lines):lines.insert(index+1,BULLET_DE);n+=1;break
    out.append((P+'/README.md','\n'.join(lines),n))
    return out
result=plan()
for rel,text,n in result:print(n,'edit(s)',rel.split('/')[-2:])
if mode=='apply':
    for rel,text,n in result:
        if n>0:
            with open(rel,'w',encoding='utf8') as f:f.write(text)
    print('applied')
