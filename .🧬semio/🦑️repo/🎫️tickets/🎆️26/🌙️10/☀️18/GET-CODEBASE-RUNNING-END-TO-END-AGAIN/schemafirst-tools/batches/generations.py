import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

G = BUILD + "🕸️graph/🕰️generations/"
b = Batch("generations", "the generation store reads its declared constants from its schema (the single source, no longer a literal the schema merely repeats) plus the limits of the claims, the lease port and the loader (re-keys: the generation module bytes)")
S = G + "🧬️schema/🔣️.json"
b.add_defs(S, [Def("Policy", "The declared names and limits of the graph generation store: the single source the module, its loader and its lease port read.", "Die deklarierten Namen und Grenzen des Graph-Generationenspeichers: die einzige Quelle, die Modul, Lader und Lease-Port lesen.", [
    ("version", 1, "The version of the policy.", "Die Version der Richtlinie."),
    ("pointer", "current.json", "The name of the pointer file to the newest generation.", "Der Name der Zeigerdatei auf die neueste Generation."),
    ("manifests", "m", "The directory of the generation manifests.", "Das Verzeichnis der Generationsmanifeste."),
    ("fragments", "o", "The directory of the owner fragments.", "Das Verzeichnis der Eigentuemer-Fragmente."),
    ("indexes", "x", "The directory of the fact indexes.", "Das Verzeichnis der Faktenindizes."),
    ("frontiers", "f", "The directory of the discovery frontiers.", "Das Verzeichnis der Entdeckungsfronten."),
    ("envOwner", "@env", "The pseudo owner that holds the facts about the environment.", "Der Pseudo-Eigentuemer, der die Fakten ueber die Umgebung haelt."),
    ("policies", ["fresh", "current"], "The graph policies a command may name.", "Die Graph-Richtlinien, die ein Befehl nennen darf."),
    ("retainGenerations", 4, "How many sealed generations stay besides the newest.", "Wie viele versiegelte Generationen neben der neuesten bleiben."),
    ("graceMilliseconds", 600000, "How long a superseded generation stays before the sweep may remove it.", "Wie lange eine abgeloeste Generation bleibt, bevor der Sweep sie entfernen darf."),
    ("verifyChunkRows", 4096, "How many fact rows one verification call checks.", "Wie viele Faktenzeilen ein Pruefaufruf kontrolliert."),
    ("verifyParallelChunks", 4, "How many verification chunks run at once.", "Wie viele Pruefabschnitte gleichzeitig laufen."),
    ("bisectBudgetCalls", 64, "The most calls the bisection of a failing chunk may spend.", "Die meisten Aufrufe, die das Halbieren eines fehlerhaften Abschnitts ausgeben darf."),
    ("racyMilliseconds", 2000, "A directory stamp this close to its capture cannot vouch for unchanged names (racy-git rule).", "Ein Verzeichnisstempel so kurz nach seiner Erfassung kann nicht fuer unveraenderte Namen buergen (Racy-Git-Regel)."),
    ("readConcurrency", 32, "How many files of the generation store are read at once (a frontier check runs twice as many).", "Wie viele Dateien des Generationsspeichers gleichzeitig gelesen werden (eine Frontpruefung laeuft doppelt so viele)."),
    ("publishWaitMilliseconds", 30000, "How long a publication waits for the lease of a peer.", "Wie lange eine Veroeffentlichung auf die Lease eines Peers wartet."),
    ("pollMaximumMilliseconds", 250, "The longest pause between two polls of a lease.", "Die laengste Pause zwischen zwei Abfragen einer Lease."),
    ("maximumOwners", 65536, "The most owners one generation holds.", "Die meisten Eigentuemer, die eine Generation haelt."),
    ("maximumManifestBytes", 134217728, "The most bytes of one manifest (128 MiB).", "Die meisten Bytes eines Manifests (128 MiB)."),
    ("maximumFragmentBytes", 67108864, "The most bytes of one fragment (64 MiB).", "Die meisten Bytes eines Fragments (64 MiB)."),
    ("maximumIndexBytes", 536870912, "The most bytes of one fact index (512 MiB).", "Die meisten Bytes eines Faktenindex (512 MiB)."),
    ("compactionFactor", 2, "An index is refined when it holds more than this many classes per owner above the floor and has grown by the growth factor.", "Ein Index wird verfeinert, wenn er ueber dem Sockel mehr als so viele Klassen je Eigentuemer haelt und um den Wachstumsfaktor gewachsen ist."),
    ("compactionFloor", 1024, "The number of classes and of row ids an index may hold above its factor before it is refined.", "Die Zahl der Klassen und Zeilennummern, die ein Index ueber seinem Faktor halten darf, bevor er verfeinert wird."),
    ("compactionGrowth", 1.25, "The growth of the number of classes against the index before that, together with the factor, makes an index refined.", "Das Wachstum der Klassenzahl gegenueber dem vorigen Index, das zusammen mit dem Faktor einen Index verfeinern laesst."),
    ("compactionDuplication", 1.5, "An index is also refined when its row ids exceed the floor and this many times its rows.", "Ein Index wird auch verfeinert, wenn seine Zeilennummern den Sockel und so viel Mal seine Zeilen uebersteigen."),
    ("brotliQuality", 5, "The Brotli quality of persisted documents.", "Die Brotli-Qualitaet gespeicherter Dokumente."),
    ("brotliWindow", 22, "The Brotli window size (log2) of persisted documents.", "Die Brotli-Fenstergroesse (log2) gespeicherter Dokumente."),
    ("reasonRows", 16, "How many rows name the reason a generation is refreshed.", "Wie viele Zeilen den Grund nennen, warum eine Generation erneuert wird."),
    ("stampVerification", True, "Whether directory stamps shorten the verification.", "Ob Verzeichnisstempel die Pruefung verkuerzen."),
    ("waitNoticeMilliseconds", 20000, "After how long a waiting command says so.", "Nach welcher Zeit ein wartender Befehl das meldet."),
    ("sweepWaitMilliseconds", 120000, "How long the sweep of superseded observation blocks waits for the generation lease.", "Wie lange der Sweep abgeloester Beobachtungsbloecke auf die Generations-Lease wartet."),
    ("hintMaximumAgeMilliseconds", 1000, "The oldest age at which the invoking build's hint is still trusted.", "Das hoechste Alter, bei dem der Hinweis des aufrufenden Builds noch gilt."),
    ("textMaximumCharacters", 4096, "The most characters of a bounded text field of a generation document.", "Die meisten Zeichen eines begrenzten Textfelds eines Generationsdokuments."),
    ("reasonMaximumCharacters", 256, "The most characters of the reason a generation records.", "Die meisten Zeichen des Grundes, den eine Generation festhaelt."),
    ("rowKeyTextLimit", 128, "A row key longer than this many characters is replaced by its digest.", "Ein Zeilenschluessel, der laenger ist, wird durch seinen Hash ersetzt."),
    ("leaseStaleMilliseconds", 600000, "After how long the lock of a silent owner is taken over.", "Nach welcher Zeit die Sperre eines stillen Eigentuemers uebernommen wird."),
    ("expansionMaximumProjects", 32, "The most projects the expansion of one command may add by default.", "Die meisten Projekte, die die Erweiterung eines Befehls standardmaessig hinzufuegen darf."),
    ("expansionMaximumBytes", 16777216, "The most bytes the expansion of one command may add by default (16 MiB).", "Die meisten Bytes, die die Erweiterung eines Befehls standardmaessig hinzufuegen darf (16 MiB)."),
    ("withheldReasonCharacters", 2048, "The most characters of the reason a project is withheld.", "Die meisten Zeichen des Grundes, warum ein Projekt zurueckgehalten wird."),
    ("pathDisplayCharacters", 160, "The most characters of a path shown in a reason.", "Die meisten Zeichen eines Pfads, der in einem Grund gezeigt wird."),
])])
M = G + "🟨️.mjs"
b.sub(M, 'from"./✂️cutoff/🟨️.mjs";\n', 'from"./✂️cutoff/🟨️.mjs";\nimport schema from"./🧬️schema/🔣️.json"with{type:"json"};\n')
b.cut(M, "/** 🕰️ The declared names and limits of the graph generation store;", "hintMaximumAgeMilliseconds:1000,\n});\n", "/** ‹E› The declared names and limits of the graph generation store; the schema file is their single source. */\nexport const GENERATION_POLICY_V1=Object.freeze({...schema.definitions.Policy.const,policies:Object.freeze([...schema.definitions.Policy.const.policies])});\n")
b.subs(M, [
    ("(value,what,maximum=4096)=>", "(value,what,maximum=P.textMaximumCharacters)=>"),
    ('requireText(value.reason,"reason",256)', 'requireText(value.reason,"reason",P.reasonMaximumCharacters)'),
    ("export const ROW_KEY_TEXT_LIMIT_V1=128;", "export const ROW_KEY_TEXT_LIMIT_V1=P.rowKeyTextLimit;"),
    ("{staleMilliseconds=600000,", "{staleMilliseconds=P.leaseStaleMilliseconds,"),
])
extra = "readonly textMaximumCharacters:number;readonly reasonMaximumCharacters:number;readonly rowKeyTextLimit:number;readonly leaseStaleMilliseconds:number;readonly expansionMaximumProjects:number;readonly expansionMaximumBytes:number;readonly withheldReasonCharacters:number;readonly pathDisplayCharacters:number}"
b.sub(G + "🟨️.d.mts", "readonly sweepWaitMilliseconds:number}", "readonly sweepWaitMilliseconds:number;" + extra)
Ld = G + "🔌️load/🟨️.mjs"
b.subs(Ld, [
    ("{maximumProjects:32,maximumBytes:16777216}", "{maximumProjects:GENERATION_POLICY_V1.expansionMaximumProjects,maximumBytes:GENERATION_POLICY_V1.expansionMaximumBytes}"),
    ('"en").slice(0,2048)', '"en").slice(0,GENERATION_POLICY_V1.withheldReasonCharacters)'),
    ("display(String(first)).slice(0,160)", "display(String(first)).slice(0,GENERATION_POLICY_V1.pathDisplayCharacters)"),
])
C = G + "🧩️claims/"
b.add_defs(C + "🧬️schema/🔣️.json", [Def("Policy", "The declared constants of the claim registry of the generation documents.", "Die deklarierten Konstanten des Anspruchsregisters der Generationsdokumente.", [
    ("version", 1, "The version of the policy.", "Die Version der Richtlinie."),
    ("directory", "c", "The directory of the claim documents.", "Das Verzeichnis der Anspruchsdokumente."),
    ("chunkOwners", 128, "How many owners one claim chunk covers.", "Wie viele Eigentuemer ein Anspruchsabschnitt abdeckt."),
    ("tailOwners", 32, "A chunk takes all open owners when at most chunkOwners plus this many are open, so no tiny last chunk remains.", "Ein Abschnitt nimmt alle offenen Eigentuemer, wenn hoechstens chunkOwners plus so viele offen sind, damit kein winziger letzter Abschnitt bleibt."),
    ("pollMilliseconds", 5000, "How long a waiting computer waits for the lease of a peer before it reads the claims again; a claim file unreadable for ten times this long is removed.", "Wie lange ein wartender Rechner auf die Lease eines Peers wartet, bevor er die Ansprueche erneut liest; eine Anspruchsdatei, die zehnmal so lange unlesbar ist, wird entfernt."),
    ("noticeMilliseconds", 20000, "After how long a waiting computer says so.", "Nach welcher Zeit ein wartender Rechner das meldet."),
    ("maximumClaims", 1024, "The most claim files one read of the registry returns; the rest is reported as truncated.", "Die meisten Anspruchsdateien, die ein Lesen des Registers liefert; der Rest wird als abgeschnitten gemeldet."),
    ("maximumOwners", 65536, "The most owners one claim covers.", "Die meisten Eigentuemer, die ein Anspruch abdeckt."),
    ("slotMegabytes", 4096, "The memory one computing slot needs.", "Der Speicher, den ein Rechenplatz braucht."),
    ("slotMemoryBasisPoints", 5000, "The share of the host memory the slots may use, in basis points.", "Der Anteil des Hostspeichers, den die Plaetze nutzen duerfen, in Basispunkten."),
    ("maximumSlots", 4, "The most computing slots at once.", "Die meisten Rechenplaetze gleichzeitig."),
    ("environmentSlots", "SEMIO_GRAPH_COMPUTERS", "The environment variable that forces the number of slots.", "Die Umgebungsvariable, die die Zahl der Plaetze erzwingt."),
    ("failedRetryMilliseconds", 60000, "For how long the failed claim of the same content covers its owners, so they are not retried at once.", "Wie lange der fehlgeschlagene Anspruch desselben Inhalts seine Eigentuemer abdeckt, damit sie nicht sofort erneut versucht werden."),
    ("failureReasonCharacters", 200, "The most characters of the reason a failed claim records.", "Die meisten Zeichen des Grundes, den ein fehlgeschlagener Anspruch festhaelt."),
    ("lockNameCharacters", 120, "The most trailing characters of an owner name in the name of its lock file.", "Die meisten abschliessenden Zeichen eines Eigentuemernamens im Namen seiner Sperrdatei."),
])])
Cm = C + "🟨️.mjs"
b.subs(Cm, [
    ('.replace(/\\s+/gu," ").slice(0,200)', '.replace(/\\s+/gu," ").slice(0,P.failureReasonCharacters)'),
    ('.toString("base64url").slice(-120)', '.toString("base64url").slice(-P.lockNameCharacters)'),
])
b.sub(C + "🟨️.d.mts", "readonly failedRetryMilliseconds:number}", "readonly failedRetryMilliseconds:number;readonly failureReasonCharacters:number;readonly lockNameCharacters:number}")
K = G + "🧱️chunk-classes/"
b.add_defs(K + "🧬️schema/🔣️.json", [Def("Policy", "The declared constants of the chunked construction of the fact index.", "Die deklarierten Konstanten der abschnittsweisen Konstruktion des Faktenindex.", [
    ("version", 1, "The version of the policy.", "Die Version der Richtlinie."),
    ("chunkCutBits", 8, "An owner's run list is cut after every row whose number hashes to zero in this many bits.", "Die Laufliste eines Eigentuemers wird nach jeder Zeile geschnitten, deren Nummer in so vielen Bits zu null hasht."),
    ("bulkUpsertRows", 1048576, "A patch with at least this many owner and row pairs rebuilds the classes in bulk instead of row by row.", "Ein Patch mit mindestens so vielen Paaren aus Eigentuemer und Zeile baut die Klassen im Ganzen statt Zeile fuer Zeile neu."),
    ("scratchSlackRows", 1024, "The headroom of the row marker table above the row count.", "Der Spielraum der Zeilenmarkierungstabelle ueber der Zeilenzahl."),
    ("scratchInitialEntries", 4096, "The initial size of the scratch list of one owner.", "Die Anfangsgroesse der Arbeitsliste eines Eigentuemers."),
])])
Km = K + "🟨️.mjs"
b.subs(Km, [("new Int32Array(rowCount+1024),scratch=new Uint32Array(1<<12)", "new Int32Array(rowCount+P.scratchSlackRows),scratch=new Uint32Array(P.scratchInitialEntries)"), ("Math.max(seen.length*2,id+1024)", "Math.max(seen.length*2,id+P.scratchSlackRows)")])
b.sub(K + "🟨️.d.mts", "readonly bulkUpsertRows:number}", "readonly bulkUpsertRows:number;readonly scratchSlackRows:number;readonly scratchInitialEntries:number}")
rel = lambda *p: "🏗️build/🕸️graph/🕰️generations/" + "".join(p)
b.row("generations", "library", rel("🧬️schema/🔣️.json"), ["Policy"], [rel("🟨️.mjs"), rel("🔌️load/🟨️.mjs")], exports=[(rel("🟨️.mjs"), "GENERATION_POLICY_V1", "Policy")], minimum_text=8, allow=[(rel("🔌️load/🟨️.mjs"), 1000, "The divisor of a duration in milliseconds that is reported in seconds; it merely shares the value of hintMaximumAgeMilliseconds.")])
b.row("generation-claims", "library", rel("🧩️claims/🧬️schema/🔣️.json"), ["Policy"], [rel("🧩️claims/🟨️.mjs")], exports=[(rel("🧩️claims/🟨️.mjs"), "CLAIMS_POLICY_V1", "Policy")])
b.row("generation-chunks", "library", rel("🧱️chunk-classes/🧬️schema/🔣️.json"), ["Policy"], [rel("🧱️chunk-classes/🟨️.mjs")], exports=[(rel("🧱️chunk-classes/🟨️.mjs"), "INDEX_CHUNK_POLICY_V1", "Policy")], minimum_number=1000)
b.write(tests=["./" + G + "🧪️tests/🟦️.ts", "./" + LAW + "🟦️.ts"])
