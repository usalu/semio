import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

I = BUILD + "🧭️invocation/🕸️graph/"
Mm = I + "🧠️memo/"
b = Batch("memo", "the graph memo reads the pointer, derivation, abort-cadence and plan-writer bounds from its policy and describes every constant in English and German (re-keys: the memo key carries the implementation digest of the memo files)")
b.add_defs(Mm + "🧬️schema/🔣️.json", [Def("Policy", "The declared constants of the graph memo: names, suffixes, byte and row bounds, the closed relevance table and the notices.", "Die deklarierten Konstanten des Graph-Memos: Namen, Endungen, Byte- und Zeilengrenzen, die geschlossene Relevanztabelle und die Meldungen.", [
    ("version", 1, "The version of the policy.", "Die Version der Richtlinie."),
    ("directory", "graph-memo", "The directory below the build store that holds the graph memo.", "Das Verzeichnis unterhalb des Build-Speichers, das das Graph-Memo haelt."),
    ("switch", "SEMIO_BUILD_GRAPH_MEMO", "The environment variable that switches the memo on or off (default on).", "Die Umgebungsvariable, die das Memo ein- oder ausschaltet (Standard ein)."),
    ("sidecar", "SEMIO_BUILD_GRAPH_MEMO_OUT", "The environment variable through which a planner child is told where to write its sealed slice.", "Die Umgebungsvariable, die einem Planer-Kind sagt, wohin es seinen versiegelten Ausschnitt schreibt."),
    ("derive", "SEMIO_BUILD_GRAPH_MEMO_DERIVE", "The environment variable that makes a child process answer a derivation request instead of planning.", "Die Umgebungsvariable, die ein Kindprozess dazu bringt, eine Ableitungsanfrage zu beantworten statt zu planen."),
    ("recordSuffix", ".json", "The suffix of a memo record.", "Die Endung eines Memo-Eintrags."),
    ("planSuffix", ".plan", "The suffix of the planned slice kept beside a record.", "Die Endung des geplanten Ausschnitts neben einem Eintrag."),
    ("relevanceSuffix", ".rel", "The suffix of the table of what each fact row reads.", "Die Endung der Tabelle dessen, was jede Faktenzeile liest."),
    ("recheckSuffix", ".chk", "The suffix of the facts the engine re-checks itself on every lookup.", "Die Endung der Fakten, die die Engine bei jeder Abfrage selbst neu prueft."),
    ("stampSuffix", ".lst", "The suffix of the table of directory stamps.", "Die Endung der Tabelle der Verzeichnisstempel."),
    ("editSuffix", ".rix", "The suffix of the edit index that maps every entry to the rows that name it.", "Die Endung des Aenderungsindex, der jeden Eintrag den Zeilen zuordnet, die ihn nennen."),
    ("maximumRecordBytes", 8192, "The most bytes of a memo record that is read.", "Die meisten Bytes eines Memo-Eintrags, die gelesen werden."),
    ("maximumPlanBytes", 268435456, "The most bytes of a planned slice that is read (256 MiB).", "Die meisten Bytes eines geplanten Ausschnitts, die gelesen werden (256 MiB)."),
    ("maximumRelevanceBytes", 67108864, "The most bytes of a relevance table that is read (64 MiB).", "Die meisten Bytes einer Relevanztabelle, die gelesen werden (64 MiB)."),
    ("maximumRecheckBytes", 268435456, "The most bytes of a re-check table that is read (256 MiB).", "Die meisten Bytes einer Neupruef-Tabelle, die gelesen werden (256 MiB)."),
    ("maximumStampBytes", 67108864, "The most bytes of a stamp table that is read (64 MiB).", "Die meisten Bytes einer Stempeltabelle, die gelesen werden (64 MiB)."),
    ("maximumEditBytes", 134217728, "The most bytes of an edit index that is read (128 MiB).", "Die meisten Bytes eines Aenderungsindex, die gelesen werden (128 MiB)."),
    ("maximumEditPaths", 256, "The most changed paths an edit refinement re-checks before it plans instead.", "Die meisten geaenderten Pfade, die eine Aenderungsverfeinerung neu prueft, bevor sie stattdessen plant."),
    ("maximumEditRows", 4096, "The most rows an edit refinement re-checks before it plans instead.", "Die meisten Zeilen, die eine Aenderungsverfeinerung neu prueft, bevor sie stattdessen plant."),
    ("maximumDerivedQuestions", 64, "The most derived facts the memo asks again in one child process.", "Die meisten abgeleiteten Fakten, die das Memo in einem Kindprozess erneut fragt."),
    ("maximumRows", 4000000, "The most fact rows a sealed slice may hold.", "Die meisten Faktenzeilen, die ein versiegelter Ausschnitt halten darf."),
    ("leafSymlinkCheckConcurrency", 64, "How many leaf paths are checked for symbolic links at once.", "Wie viele Blattpfade gleichzeitig auf symbolische Links geprueft werden."),
    ("environmentPrefixes", ["SEMIO_BUILD_", "SEMIO_GRAPH_", "SEMIO_REPO_"], "The prefixes of the environment variables that are part of the memo key.", "Die Praefixe der Umgebungsvariablen, die zum Memo-Schluessel gehoeren."),
    ("argumentsDenied", ["--parallel"], "The arguments that are not part of the memo key.", "Die Argumente, die nicht zum Memo-Schluessel gehoeren."),
    ("stamped", ["readdir", "readdirNames"], "The fact kinds whose directory listings are vouched for by directory stamps.", "Die Faktenarten, deren Verzeichnislisten durch Verzeichnisstempel verbuergt werden."),
    ("following", ["directories"], "The fact kinds that follow symbolic links.", "Die Faktenarten, die symbolischen Links folgen."),
    ("outputs", [".🧾️store"], "The directories a build writes its own results into; their listings are re-checked on every lookup.", "Die Verzeichnisse, in die ein Build seine eigenen Ergebnisse schreibt; ihre Listen werden bei jeder Abfrage neu geprueft."),
    ("environmentDenied", ["SEMIO_BUILD_GENERATION_PIN", "SEMIO_BUILD_GRAPH_MEMO_DERIVE", "SEMIO_BUILD_GRAPH_MEMO_EXPECT", "SEMIO_BUILD_GRAPH_MEMO_OUT", "SEMIO_BUILD_GRAPH_PLAN_OUT", "SEMIO_BUILD_GRAPH_REQUESTED_AT", "SEMIO_BUILD_GRAPH_YIELD_ROUND", "SEMIO_BUILD_GRAPH_YIELD_TICKET", "SEMIO_BUILD_INVOCATION_PID", "SEMIO_BUILD_INVOCATION_PIDS", "SEMIO_BUILD_PARALLEL", "SEMIO_BUILD_REUSE_GRAPH", "SEMIO_REPO_IMPLEMENTATION", "SEMIO_REPO_PROFILE", "SEMIO_REPO_SEAL"], "The environment variables that never belong to the memo key.", "Die Umgebungsvariablen, die nie zum Memo-Schluessel gehoeren."),
    ("relevance", {"file": "path", "exists": "path", "stat": "path", "lstat": "path", "readdir": "path", "readdirNames": "path", "fileExists": "path", "directoryExists": "path", "directories": "path", "typescriptText": "path", "digest": "path", "field": "path", "analysis": "analysis", "realpath": "link", "realpathNative": "link", "typescriptRealpath": "link", "resolve": "resolve", "derived": "text", "closure": "none", "stamp": "none", "registry": "none"}, "The closed table of what each fact kind reads.", "Die geschlossene Tabelle dessen, was jede Faktenart liest."),
    ("carry", KEEP, "Which generation-wide digests must be identical for a sealed slice to be served under a newer generation.", "Welche generationsweiten Hashes gleich sein muessen, damit ein versiegelter Ausschnitt unter einer neueren Generation geliefert wird."),
    ("texts", KEEP, "The English and German notices of the memo.", "Die englischen und deutschen Meldungen des Memos."),
    ("expect", "SEMIO_BUILD_GRAPH_MEMO_EXPECT", "The environment variable that carries the memo state a planner child is expected to find.", "Die Umgebungsvariable, die den Memo-Zustand traegt, den ein Planer-Kind vorfinden soll."),
    ("planOutput", "SEMIO_BUILD_GRAPH_PLAN_OUT", "The environment variable through which a planner child is told where to write the plan of the graph.", "Die Umgebungsvariable, die einem Planer-Kind sagt, wohin es den Plan des Graphen schreibt."),
    ("requestedAt", "SEMIO_BUILD_GRAPH_REQUESTED_AT", "The environment variable that carries the instant the graph was requested.", "Die Umgebungsvariable, die den Zeitpunkt traegt, zu dem der Graph angefordert wurde."),
    ("pointerMaximumBytes", 65536, "The most bytes of a generation pointer file that is read (64 KiB).", "Die meisten Bytes einer Generations-Zeigerdatei, die gelesen werden (64 KiB)."),
    ("deriveMaximumNames", 65536, "The most names a derivation request or answer lists.", "Die meisten Namen, die eine Ableitungsanfrage oder -antwort auffuehrt."),
    ("deriveNameCharacters", 256, "The most characters of one name in a derivation request or answer.", "Die meisten Zeichen eines Namens in einer Ableitungsanfrage oder -antwort."),
    ("abortCheckMask", 8191, "The mask of the row counter: the row loop checks for cancellation whenever the counter under it is zero.", "Die Maske des Zeilenzaehlers: die Zeilenschleife prueft auf Abbruch, wenn der Zaehler unter ihr null ist."),
    ("planFlushBytes", 1048576, "The size at which the plan writer flushes its buffer to the file (1 MiB).", "Die Groesse, ab der der Plan-Schreiber seinen Puffer in die Datei schreibt (1 MiB)."),
    ("planArrayChunkRows", 4096, "The array elements the plan writer serializes at once.", "Die Feldelemente, die der Plan-Schreiber auf einmal serialisiert."),
])])
M = Mm + "🟦️.ts"
b.sub(M, "leafSymlinkCheckConcurrency:number;\n", "leafSymlinkCheckConcurrency:number;planOutput:string;pointerMaximumBytes:number;deriveMaximumNames:number;deriveNameCharacters:number;abortCheckMask:number;planFlushBytes:number;planArrayChunkRows:number;\n")
b.sub(M, "readBounded(record.pointer.file,65536)", "readBounded(record.pointer.file,GRAPH_MEMO_POLICY_V1.pointerMaximumBytes)")
b.sub(M, "readBounded(sealed.pointer.file,65536)", "readBounded(sealed.pointer.file,GRAPH_MEMO_POLICY_V1.pointerMaximumBytes)")
b.sub(Mm + "🧮️derive/🟦️.ts", "value.length<=65536&&value.every(name=>typeof name===\"string\"&&name.length>0&&name.length<=256)", "value.length<=GRAPH_MEMO_POLICY_V1.deriveMaximumNames&&value.every(name=>typeof name===\"string\"&&name.length>0&&name.length<=GRAPH_MEMO_POLICY_V1.deriveNameCharacters)")
b.sub(Mm + "📤️relevance/🟦️.ts", "if((count&0x1fff)===0)", "if((count&GRAPH_MEMO_POLICY_V1.abortCheckMask)===0)")
G = I + "🟦️.ts"
b.sub(Mm + "📤️relevance/🟦️.ts", 'stem+".json"', "stem+GRAPH_MEMO_POLICY_V1.recordSuffix", 3)
b.sub(Mm + "📤️relevance/🟦️.ts", 'resources:{facts:"graph-memo"}', "resources:{facts:GRAPH_MEMO_POLICY_V1.directory}")
b.subs(G, [
    ('export const BUILD_GRAPH_PLAN_OUT_V1="SEMIO_BUILD_GRAPH_PLAN_OUT";', "export const BUILD_GRAPH_PLAN_OUT_V1=GRAPH_MEMO_POLICY_V1.planOutput;"),
    ('import{readBuildGraphAncestorSelectionV1}from"../🟦️.ts";', 'import{readBuildGraphAncestorSelectionV1}from"../🟦️.ts";\nimport{GRAPH_MEMO_POLICY_V1}from"./🧠️memo/🟦️.ts";'),
    ('"wx",0o600),limit=1048576;', '"wx",0o600),limit=GRAPH_MEMO_POLICY_V1.planFlushBytes;'),
    ("at+=4096){const text=JSON.stringify(value.slice(at,at+4096));", "at+=GRAPH_MEMO_POLICY_V1.planArrayChunkRows){const text=JSON.stringify(value.slice(at,at+GRAPH_MEMO_POLICY_V1.planArrayChunkRows));"),
])
rel = lambda p: "🏗️build/🧭️invocation/🕸️graph/" + p
b.row("memo", "library", rel("🧠️memo/🧬️schema/🔣️.json"), ["Policy"], [rel("🧠️memo/🟦️.ts"), rel("🧠️memo/🧮️derive/🟦️.ts"), rel("🧠️memo/📤️relevance/🟦️.ts"), rel("🟦️.ts")], exports=[(rel("🧠️memo/🟦️.ts"), "GRAPH_MEMO_POLICY_V1", "Policy")])
b.write(tests=["./" + Mm + "🧪️tests/🟦️.ts", "./" + LAW + "🟦️.ts"])
