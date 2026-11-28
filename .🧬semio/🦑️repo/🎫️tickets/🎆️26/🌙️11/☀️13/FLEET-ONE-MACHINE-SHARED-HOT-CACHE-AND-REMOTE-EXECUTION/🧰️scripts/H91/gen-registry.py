#!/usr/bin/env python3
"""H91: writes the failure-class registry (schema-first data) into the staged tree. Usage: gen-registry.py <stage-tree-root>"""
import json,os,sys
tree=sys.argv[1]
B="🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
C="🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
P="🔨️modules/🏃️process"
RD=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready"
HC=P+"/🏗️build/🌡️host-capacity"
def T(en,de):return {"en":en,"de":de}
classes={}
READ=("Read the refusal text: it names the cause and the next step.","Den Ablehnungstext lesen: er nennt Ursache und nächsten Schritt.")
def cls(code,owner,retry,en,de,xen,xde,action=None):
    if action is None and retry=="user":action=READ
    classes[code]={"title":T(en,de),"explanation":T(xen,xde),"owner":owner,"retry":retry,"action":None if action is None else T(*action)}
LOG=("Read ready-log-<stage>-failed.txt of the origin (repo fleet status --origin <id>).","Das Protokoll ready-log-<Stufe>-failed.txt des Origins lesen (repo fleet status --origin <id>).")
cls("stage.failed",RD,"backoff","Stage failed (not classified further)","Stufe fehlgeschlagen (nicht weiter eingeordnet)","A readiness stage ended with a non-zero exit and no more specific class matched its evidence; the supervisor retries with back-off.","Eine Bereitschaftsstufe endete mit einem Fehlerstatus, und keine genauere Klasse passte zu ihren Belegen; der Aufseher versucht es mit Wartezeit erneut.",LOG)
cls("stage.external-stop",RD,"backoff","Stage stopped from outside","Stufe von außen gestoppt","The stage process received a stop signal (SIGTERM, SIGHUP, SIGINT) that neither this supervisor, the deadline, the warden nor the runner sent; the worker stays Ready while it refreshes.","Der Stufenprozess erhielt ein Stopp-Signal (SIGTERM, SIGHUP, SIGINT), das weder dieser Aufseher, die Frist, der Wächter noch der Runner gesendet hat; der Worker bleibt bereit, solange er sich erneuert.",("The log line Ended by names the sender when it is known (outside-wrapper, outside-child).","Die Protokollzeile Ended by nennt den Sender, sofern bekannt (outside-wrapper, outside-child)."))
cls("stage.stalled",RD,"backoff","Stage stalled","Stufe hängt","No progress for the declared stall time, a declared step ran overdue or a named holder stayed silent; the process tree was stopped and the stage restarts.","Kein Fortschritt über die erklärte Stillstandszeit, ein erklärter Schritt war überfällig oder ein benannter Halter blieb stumm; der Prozessbaum wurde beendet und die Stufe startet neu.",LOG)
cls("stage.code-changed",RD,"after-change","Stage restarted for new code","Stufe wegen neuem Code neu gestartet","The code of the build system changed while the stage ran or waited, so it restarts at once with the current code.","Der Code des Build-Systems änderte sich, während die Stufe lief oder wartete; sie startet sofort mit dem aktuellen Code neu.")
cls("stage.timeout",RD,"backoff","Stage exceeded its time limit","Stufe über ihrer Zeitgrenze","The stage ran longer than its declared hard cap and was stopped.","Die Stufe lief länger als ihre erklärte harte Grenze und wurde gestoppt.",LOG)
cls("stage.spawn-failed",RD,"backoff","Stage could not start","Stufe konnte nicht starten","The stage process could not be spawned (missing binary, permission, resource limit).","Der Stufenprozess konnte nicht gestartet werden (fehlende Datei, Berechtigung, Ressourcengrenze).",LOG)
cls("stage.loop-failed",RD,"backoff","Readiness loop failed","Bereitschaftsschleife fehlgeschlagen","The supervisor loop itself threw before or after a stage; it restarts.","Die Aufseher-Schleife selbst warf vor oder nach einer Stufe einen Fehler; sie startet neu.",LOG)
cls("stage.waiting",RD,"wait","Stage waits for a person or a lease","Stufe wartet auf einen Menschen oder eine Lease","A human session or another holder keeps the lease the stage needs; nothing failed.","Eine menschliche Sitzung oder ein anderer Halter hält die benötigte Lease; nichts ist fehlgeschlagen.")
cls("stage.server",RD,"none","Machine serves instead of building","Maschine dient statt zu bauen","The machine runs as a server and has no stages to build; this is a state, not a failure.","Die Maschine läuft als Server und hat keine Stufen zu bauen; das ist ein Zustand, kein Fehler.")
cls("stage.tooling-compiler",RD,"user","Wrong compiler tooling","Falsches Compiler-Werkzeug","The dependency stage finished with a compiler that does not match the pin.","Die Abhängigkeitsstufe endete mit einem Compiler, der nicht zum Pin passt.",("Run repo tools setup all, then wait for the next attempt.","repo tools setup all ausführen und den nächsten Versuch abwarten."))
cls("source.transient",RD,"after-change","Source tree in motion","Quellbaum in Bewegung","A peer's half-finished edit or landing made a source file, import or module unreadable; the stage retries after the replicated tree settles.","Eine halbfertige Änderung eines Peers oder eine Landung machte eine Quelldatei, einen Import oder ein Modul unlesbar; die Stufe versucht es erneut, sobald sich der replizierte Baum beruhigt hat.")
cls("memory.low",RD,"wait","Not enough free memory to start","Zu wenig freier Speicher zum Start","The machine's free memory is below the floor for starting a stage; the supervisor waits.","Der freie Speicher der Maschine liegt unter der Grenze für den Start einer Stufe; der Aufseher wartet.")
cls("memory.cancelled","%s/🛡️warden"%HC,"backoff","Run cancelled by the memory warden","Lauf vom Speicherwächter abgebrochen","A process of the run exceeded its ceiling or the host was in danger; the warden cancelled it and recorded the cancel row.","Ein Prozess des Laufs überschritt seine Obergrenze oder der Rechner war in Gefahr; der Wächter brach ihn ab und vermerkte die Abbruchzeile.",("Carve the named unit or raise its ceiling in the admission policy.","Die genannte Einheit aufteilen oder ihre Obergrenze in der Zulassungsrichtlinie anheben."))
cls("disk.short",RD,"wait","Not enough disk space for the stage","Zu wenig Plattenplatz für die Stufe","Free space or inodes of a partition are below what the stage needs, or could not be measured; collectors run first.","Freier Platz oder Inodes einer Partition liegen unter dem Bedarf der Stufe oder ließen sich nicht messen; zuerst laufen die Sammler.",("repo cache gc, then repo doctor --only=storage.","repo cache gc, danach repo doctor --only=storage."))
cls("disk.guard",RD+"/🖼️image-guard","backoff","Disk guard stopped the stage","Plattenwächter stoppte die Stufe","Free space fell below the floor while the stage ran and the collection could not outrun it.","Der freie Platz fiel unter die Untergrenze, während die Stufe lief, und das Einsammeln konnte nicht mithalten.",("Free space on the named partition; the next start grows the image first.","Platz auf der genannten Partition schaffen; der nächste Start vergrößert zuerst das Image."))
cls("disk.store-full",RD,"user","Store or sparse image is full","Speicher oder Sparse-Image ist voll","The store reported no space left (ENOSPC or database is full) during the stage.","Der Speicher meldete keinen freien Platz mehr (ENOSPC oder Datenbank voll) während der Stufe.",("repo cache gc and repo doctor --fix.","repo cache gc und repo doctor --fix."))
cls("disk.image-full",RD+"/🖼️image-guard","backoff","Sparse image full or short","Sparse-Image voll oder zu knapp","A cache image has no room left; the heal grows it between stages when the host has space.","Ein Cache-Image hat keinen Platz mehr; die Heilung vergrößert es zwischen den Stufen, wenn der Rechner Platz hat.")
cls("graph.handover-churn",B+"/🕸️graph","after-change","Graph read handed over without progress","Graph-Lesen ohne Fortschritt übergeben","The repo graph service was superseded by newer code several times before one read finished; the client gave up and retries after the landing burst.","Der Graph-Dienst wurde mehrfach durch neueren Code abgelöst, bevor ein Lesevorgang endete; der Client gab auf und versucht es nach der Landungswelle erneut.")
cls("graph.slice-worker-exit",B+"/🕸️graph","backoff","Graph slice worker ended","Graph-Slice-Worker beendet","A slice worker of the graph read exited, often for memory on a 16 GB host.","Ein Slice-Worker des Graph-Lesens endete, oft wegen Speichers auf einem 16-GB-Rechner.",("Check repo doctor --only=capacity for the carving list.","repo doctor --only=capacity auf die Aufteilungsliste prüfen."))
cls("graph.no-sealed-generation",B+"/🕸️graph/🕰️generations","wait","No sealed graph generation on this machine","Keine versiegelte Graph-Generation auf dieser Maschine","A query that never builds the graph found no sealed generation.","Eine Abfrage, die nie den Graphen baut, fand keine versiegelte Generation.",("Wait for the readiness workspace stage or run repo graph.","Auf die Workspace-Stufe der Bereitschaft warten oder repo graph ausführen."))
cls("graph.unknown-owner",B+"/🕸️graph/🕰️generations","user","Unknown project in the closure","Unbekanntes Projekt in der Hülle","The named project is not an owner of the sealed generation.","Das genannte Projekt ist kein Eigentümer der versiegelten Generation.",("Check the project name with repo show projects.","Den Projektnamen mit repo show projects prüfen."))
cls("nx.run-stalled",B+"/🔧️tools/🧶️nx/🔌️backend","backoff","Owned Nx run stalled","Eigener Nx-Lauf hing","The liveness watchdog ended the Nx run (exit 124) after a long silence, typically while waiting on a peer's single-flight task.","Der Liveness-Wächter beendete den Nx-Lauf (Exit 124) nach langer Stille, meist beim Warten auf eine Single-Flight-Aufgabe eines Peers.")
cls("nx.run-failed",B+"/🔧️tools/🧶️nx/🔌️backend","backoff","Owned Nx run failed","Eigener Nx-Lauf fehlgeschlagen","The Nx run ended with a failure status; the task output names the cause.","Der Nx-Lauf endete mit einem Fehlerstatus; die Ausgabe der Aufgabe nennt die Ursache.",LOG)
cls("nx.recipe-refused",C+"/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection","user","Nx recipe refused","Nx-Rezept abgelehnt","A recipe of the Nx projection broke the closed schema.","Ein Rezept der Nx-Projektion verletzte das geschlossene Schema.")
cls("cargo.lock-refused",P+"/🦀️cargo-workspace/🔒️lock","user","Cargo resolver refused the lock","Cargo-Resolver lehnte das Lock ab","Cargo could not resolve or lock the workspace (offline, missing crate, conflicting versions).","Cargo konnte den Workspace nicht auflösen oder sperren (offline, fehlendes Crate, widersprüchliche Versionen).")
cls("tool.hook-timeout",B+"/🧰️tool-setup/⏱️hook-bound","backoff","Tool hook exceeded its bound","Werkzeug-Hook über seiner Grenze","A tool setup or verify hook ran longer than its declared bound.","Ein Setup- oder Prüf-Hook eines Werkzeugs lief länger als seine erklärte Grenze.")
cls("fs.rename-refused","🔨️modules/📁️filesystem/🧾️observation/📁️descriptor","backoff","Windows refused a folder rename","Windows lehnte das Umbenennen eines Ordners ab","A handle or antivirus scan kept a folder open so it could not be moved.","Ein Handle oder Virenscanner hielt einen Ordner offen, sodass er nicht verschoben werden konnte.")
cls("lease.physical-lock",P+"/🔒️leases/📁️physical","wait","Physical lock refused","Physische Sperre abgelehnt","The physical lock of a resource could not be taken.","Die physische Sperre einer Ressource ließ sich nicht nehmen.")
cls("lease.timeout",P+"/🔒️leases","wait","Lease wait timed out","Wartezeit auf Lease abgelaufen","A resource lease was not granted within its bound; the holder is named in the text.","Eine Ressourcen-Lease wurde nicht innerhalb ihrer Grenze erteilt; der Halter steht im Text.",("repo leases status shows the holder.","repo leases status zeigt den Halter."))
cls("capacity.refused",B+"/🌡️host-capacity","wait","Host capacity refused or timed out","Rechnerkapazität abgelehnt oder Zeit abgelaufen","The host capacity gate refused or did not admit the request in time.","Das Kapazitäts-Tor des Rechners lehnte die Anfrage ab oder ließ sie nicht rechtzeitig zu.",("repo doctor --only=capacity.","repo doctor --only=capacity."))
cls("disk.refused",P+"/🏗️build/💽️disk","user","Disk admission refused","Plattenzulassung abgelehnt","The disk admission or the low-space check refused the work.","Die Plattenzulassung oder die Prüfung auf wenig Platz lehnte die Arbeit ab.",("repo cache gc, then repeat.","repo cache gc, danach wiederholen."))
for code,fn,mod,en in [("build.refused","refuseBuildV1",B,("Build command refused","Build-Befehl abgelehnt")),("cache.refused","refuseCachingV1",C,("Caching refused","Caching abgelehnt")),("bootstrap.refused","refuseBootstrapV1",C+"/🚀️bootstrap",("Bootstrap refused","Bootstrap abgelehnt")),("process.refused","refuseProcessV1",P,("Process module refused","Prozessmodul abgelehnt")),("land.refused","refuseLandV1",B+"/🚉️land",("Landing refused","Landung abgelehnt")),("typecheck.refused","refuseTypecheckV1",B+"/🔬️typecheck",("Typecheck refused","Typprüfung abgelehnt")),("test-file.refused","refuseTestFileV1",B+"/🔬️test-file",("Test-file run refused","Testdatei-Lauf abgelehnt")),("ci-suite.refused","refuseCiSuiteV1",B+"/🧾️ci-report",("CI suite refused","CI-Suite abgelehnt"))]:
    cls(code,mod,"user",en[0],en[1],"A typed catalog refusal of its module (%s); its text names the cause and the next step in both languages."%fn,"Eine typisierte Katalog-Ablehnung ihres Moduls (%s); ihr Text nennt Ursache und nächsten Schritt in beiden Sprachen."%fn)
cls("remote.unreachable",B+"/🌐️remote-cache","backoff","Remote cache or executor unreachable","Remote-Cache oder -Ausführer nicht erreichbar","The fleet cache or a peer did not answer; work continues locally and the upload waits in the spool.","Der Flotten-Cache oder ein Peer antwortete nicht; die Arbeit läuft lokal weiter und der Upload wartet im Spool.")
cls("monitor.degraded",B+"/🛡️fail-open","none","A monitor or estimator degraded, work went on","Ein Monitor oder Schätzer fiel aus, die Arbeit lief weiter","A monitor, estimator or telemetry source failed (timeout, missing file, parse error). The work proceeded without it and printed a bilingual warning; only a measured over-limit may fail work.","Ein Monitor, Schätzer oder eine Telemetriequelle fiel aus (Zeitüberschreitung, fehlende Datei, Lesefehler). Die Arbeit lief ohne sie weiter und gab eine zweisprachige Warnung aus; nur eine gemessene Grenzüberschreitung darf Arbeit scheitern lassen.",("Read the warning; repair the named source when convenient.","Die Warnung lesen; die genannte Quelle bei Gelegenheit reparieren."))
cls("internal.unclassified",P+"/🏗️build/🗣️language/📣️progress","none","Internal error without a class","Interner Fehler ohne Klasse","An exception reached the CLI boundary that no typed refusal and no registry rule covers; the boundary prints the message with a stable reference.","Eine Ausnahme erreichte die CLI-Grenze, die keine typisierte Ablehnung und keine Regel der Registry abdeckt; die Grenze gibt die Meldung mit einer stabilen Referenz aus.",("Report the reference; add a class or a typed refusal at the throwing module.","Die Referenz melden; am werfenden Modul eine Klasse oder typisierte Ablehnung ergänzen."))
internals={
 "internal.graph":("graph and inputs","Graph und Eingaben",B+"/🕸️graph"),
 "internal.cache":("caching and artifacts","Caching und Artefakte",C),
 "internal.remote":("remote cache and execution","Remote-Cache und -Ausführung",B+"/🌐️remote-execution"),
 "internal.capacity":("capacity, memory and scheduling","Kapazität, Speicher und Planung",HC),
 "internal.cli":("command line, catalog and texts","Kommandozeile, Katalog und Texte",B+"/⌨️command"),
 "internal.tools":("tools and bootstrap","Werkzeuge und Bootstrap",B+"/🔧️tools"),
 "internal.verification":("tests, type check, doctor and reports","Tests, Typprüfung, Doctor und Berichte",B+"/🔬️test-file"),
 "internal.landing":("landing","Landung",B+"/🚉️land"),
 "internal.process":("process runtime, leases and state","Prozess-Laufzeit, Leases und Zustand",P),
 "internal.storage":("storage, disk and machine state","Speicher, Platte und Maschinenzustand",P+"/🏗️build/💽️disk"),
 "internal.ready":("readiness supervisor","Bereitschafts-Aufseher",RD),
}
for code,(en,de,owner) in internals.items():
    cls(code,owner,"none","Unexpected fault in "+en,"Unerwarteter Fehler in "+de,"A plain exception of a module in this area, rendered at the CLI boundary as an internal error with a reference; the class tells who owns it.","Eine einfache Ausnahme eines Moduls dieses Bereichs, an der CLI-Grenze als interner Fehler mit Referenz ausgegeben; die Klasse nennt den Eigentümer.",("Report the reference to the module owner.","Die Referenz dem Eigentümer des Moduls melden."))
ready={"stage-failed":"stage.failed","interrupted":"stage.external-stop","stage-stalled":"stage.stalled","stage-stall-exhausted":"stage.stalled","stage-step-overdue":"stage.stalled","stage-holder-silent":"stage.stalled","stage-code-changed":"stage.code-changed","stage-code-stale":"stage.code-changed","stage-code-stale-waiting":"stage.code-changed","stage-timeout":"stage.timeout","spawn-failed":"stage.spawn-failed","loop-failed":"stage.loop-failed","human-running":"stage.waiting","lease-held":"stage.waiting","server":"stage.server","tooling-compiler":"stage.tooling-compiler","source-transient":"source.transient","memory-low":"memory.low","memory-cancelled":"memory.cancelled","disk-short":"disk.short","disk-unmeasured":"disk.short","disk-collecting":"disk.short","disk-consumers":"disk.short","disk-unfit":"disk.short","disk-guard":"disk.guard","store-full":"disk.store-full","image-full":"disk.image-full","image-short":"disk.image-full"}
runs=[
 {"outcome":"interrupted","class":"stage.external-stop"},
 {"outcome":"stage-timeout","class":"stage.timeout"},
 {"outcome":"spawn-failed","class":"stage.spawn-failed"},
 {"outcome":"stalled","class":"stage.stalled"},
 {"outcome":"stage-failed","signal":"SIGTERM","class":"stage.external-stop"},
 {"outcome":"stage-failed","signal":"SIGHUP","class":"stage.external-stop"},
 {"outcome":"stage-failed","signal":"SIGINT","class":"stage.external-stop"},
 {"outcome":"stage-failed","exit":143,"class":"stage.external-stop"},
 {"outcome":"stage-failed","exit":129,"class":"stage.external-stop"},
 {"outcome":"stage-failed","exit":130,"class":"stage.external-stop"},
 {"outcome":"stage-failed","exit":124,"class":"nx.run-stalled"},
 {"outcome":"stage-failed","class":"stage.failed"},
]
patterns=[
 {"pattern":"ends \\(superseded\\) before this graph read finished","class":"graph.handover-churn"},
 {"pattern":"Slice worker exited|slice-worker","class":"graph.slice-worker-exit"},
 {"pattern":"nx\\.run-stalled|Owned Nx invocation failed \\(124\\)","class":"nx.run-stalled"},
 {"pattern":"nx\\.run-failed|Owned Nx invocation failed","class":"nx.run-failed"},
 {"pattern":"SIGTERM reached this run|was ended by signal SIG(TERM|HUP|INT)|Ended by|SIGTERM reached the stage wrapper","class":"stage.external-stop"},
 {"pattern":"No space left on device|ENOSPC|database or disk is full|Directory partition capacity exceeded","class":"disk.store-full"},
 {"pattern":"cancel overdue|memory warden|Speicherwächter","class":"memory.cancelled"},
 {"pattern":"Closure reference proof leaf changed|ENOENT[^\\n]*lstat|Cannot find module|Module not found","class":"source.transient"},
 {"pattern":"Bounded relative file read required","class":"internal.ready"},
 {"pattern":"EPERM[^\\n]*unlink|EBUSY","class":"fs.rename-refused"},
 {"pattern":"Internal error|Interner Fehler","class":"internal.unclassified"},
 {"pattern":"\\[fail-open\\]","class":"monitor.degraded"},
]
refusals={"functions":{"refuseBuildV1":"build.refused","refuseCachingV1":"cache.refused","refuseBootstrapV1":"bootstrap.refused","refuseProcessV1":"process.refused","refuseLandV1":"land.refused","refuseTypecheckV1":"typecheck.refused","refuseTestFileV1":"test-file.refused","refuseCiSuiteV1":"ci-suite.refused","refuseCapacityV1":"capacity.refused","refuseDiskRecoveryV1":"disk.refused","refuseDiskAdmissionV1":"disk.refused","refuseLowSpaceV1":"disk.refused","refuseHeavyOwnerV1":"capacity.refused","refuseBudgetV1":"capacity.refused","refuseInputFramesV1":"internal.graph"},
 "keys":[{"function":"refuseProcessV1","prefix":"disk.","class":"disk.refused"},{"function":"refuseProcessV1","prefix":"lease.","class":"lease.timeout"},{"function":"refuseBuildV1","prefix":"nx.run-stalled","class":"nx.run-stalled"},{"function":"refuseBuildV1","prefix":"nx.run-failed","class":"nx.run-failed"},{"function":"refuseBuildV1","prefix":"capacity.","class":"capacity.refused"},{"function":"refuseCachingV1","prefix":"remote","class":"remote.unreachable"}]}
errors={"CargoLockRefused":"cargo.lock-refused","CargoLockRefusedV1":"cargo.lock-refused","NxRecipeRefused":"nx.recipe-refused","NxRecipeRefusedV1":"nx.recipe-refused","ToolHookTimeout":"tool.hook-timeout","ToolHookTimeoutV1":"tool.hook-timeout","WindowsRenameRefused":"fs.rename-refused","PhysicalLockError":"lease.physical-lock","PhysicalLockErrorV1":"lease.physical-lock","ResourceLeaseTimeoutErrorV1":"lease.timeout","SlotWaitTimeoutV1":"lease.timeout","ProductLaneBusyErrorV1":"lease.timeout","ProductLaneWaitBoundErrorV1":"lease.timeout","GrpcErrorV1":"internal.remote","RemoteExecutionContentErrorV1":"internal.remote","RemoteCacheDirectoryBusyV1":"remote.unreachable","FleetPublicationRefusedV1":"remote.unreachable","DiskFullV1":"disk.store-full"}
codes={"GENERATION_NONE_SEALED":"graph.no-sealed-generation","GENERATION_UNKNOWN_OWNER":"graph.unknown-owner"}
modules=[]
def add(path,c):modules.append({"path":path,"class":c})
add(B,"internal.cli");add(C,"internal.cache");add(P,"internal.process")
Bmap={"🕸️graph":"internal.graph","🪪️graph-state":"internal.graph","📥️inputs":"internal.graph","📥️source":"internal.graph","🏛️sealed":"internal.graph","🔭️witness":"internal.graph","🧩️units":"internal.graph","📏️incremental":"internal.graph","📏️workspace-scale":"internal.graph","🎯️targets":"internal.graph","🧮️task-model":"internal.graph","🧭️selection":"internal.graph","🧭️invocation":"internal.graph","🧊️revision":"internal.graph","🧊️bounded":"internal.graph",
"🧹️cache-gc":"internal.cache","🗄️cas":"internal.cache","🧾️store-receipt":"internal.cache","📈️counter-ledger":"internal.cache","🌐️remote-cache":"internal.remote","🌐️remote-execution":"internal.remote","👷️workers":"internal.remote","🧵️workers":"internal.remote",
"🌡️host-capacity":"internal.capacity","🧱️job-limits":"internal.capacity","🪢️slots":"internal.capacity","🗓️scheduler":"internal.capacity","🧠️memory":"internal.capacity","🐢️speed":"internal.capacity","⏱️profile":"internal.capacity","🔒️leases":"internal.process",
"🔧️tools":"internal.tools","🧰️tool-registry":"internal.tools","🧰️tool-setup":"internal.tools","🛠️cmake":"internal.tools","🚀️bootstrap":"internal.tools","🔌️backends":"internal.tools",
"🔬️test-file":"internal.verification","🔬️typecheck":"internal.verification","🧾️ci-report":"internal.verification","🩺️diagnostics":"internal.verification","🩺️doctor":"internal.verification","🩺️preflight":"internal.verification","🪞️parity":"internal.verification","📐️conformance":"internal.verification","🗂️suites":"internal.verification","🛡️fail-open":"internal.verification","🧯️failure-classes":"internal.verification","🪪️agent-sessions":"internal.process",
"🚉️land":"internal.landing","🚢️fleet-status":"internal.cli"}
for name in sorted(os.listdir(os.path.join(tree,"..","..")) if False else []):pass
import subprocess
root=os.environ["H91_ROOT"]
def dirs(rel):
    full=os.path.join(root,"🧰️framework",rel)
    return sorted(d for d in os.listdir(full) if os.path.isdir(os.path.join(full,d)) and d not in("node_modules","🧪️tests",".cache","target","🧫️fixtures","🧬️schema","🧰️toolchains"))
for d in dirs(B):
    add(B+"/"+d,Bmap.get(d,"internal.cli"))
add(RD,"internal.ready")
for new in ("🧯️failure-classes","🛡️fail-open"):
    if not any(m["path"]==B+"/"+new for m in modules):add(B+"/"+new,"internal.verification")
Cmap={"🦀️cargo":"internal.cache","🚀️bootstrap":"internal.tools","🩺️environment":"internal.tools","🕸️graph":"internal.graph","📥️inference":"internal.graph","🔏️inputs":"internal.graph","🔍️discovery":"internal.graph"}
for d in dirs(C):add(C+"/"+d,Cmap.get(d,"internal.cache"))
Pmap={"🔒️leases":"internal.process","💾️state":"internal.storage","🆓️free-space":"internal.storage","🛠️repository-tooling":"internal.tools","🧰️tools":"internal.tools","🦀️cargo-workspace":"internal.tools","🧪️testing":"internal.verification"}
for d in dirs(P):
    if d=="🏗️build":continue
    add(P+"/"+d,Pmap.get(d,"internal.process"))
Pbmap={"🌡️host-capacity":"internal.capacity","🎟️capacity":"internal.capacity","💽️disk":"internal.storage","🗃️state":"internal.storage","🏠️machine-state":"internal.storage","📈️metrics":"internal.capacity"}
add(P+"/🏗️build","internal.process")
for d in dirs(P+"/🏗️build"):add(P+"/🏗️build/"+d,Pbmap.get(d,"internal.process"))
registry={"version":1,"internal":"internal.unclassified","scopes":[B,C,P],"classes":dict(sorted(classes.items())),"ready":ready,"runs":runs,"patterns":patterns,"refusals":refusals,"errors":errors,"codes":codes,"modules":sorted(modules,key=lambda row:row["path"])}
out=os.path.join(tree,"🧰️framework",B,"🧯️failure-classes/🔣️.json")
os.makedirs(os.path.dirname(out),exist_ok=True)
open(out,"w",encoding="utf8").write(json.dumps(registry,ensure_ascii=False,indent=1)+"\n")
print(out,len(classes),"classes",len(modules),"modules")
