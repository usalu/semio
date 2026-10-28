import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

O = BUILD + "👁️observation/"
b = Batch("observation", "the observation module reads its racy window, memo budgets, block cut, sweep grace and progress strides from its schema (re-keys: the observation schema digest stamps every persisted graph record)")
b.add_defs(O + "🧬️schema/🔣️.json", [Def("limits", "The limits of the observation module: the racy window, the in-process memo budgets, the block cut and the sweep.", "Die Grenzen des Beobachtungsmoduls: das Racy-Fenster, die Speicherbudgets im Prozess, der Blockschnitt und der Sweep.", [
    ("racyMilliseconds", 2000, "A file stamped this close to the start of a computation may still change within one timestamp tick, so its result is never persisted (racy-git rule).", "Eine Datei, die so kurz vor dem Start einer Berechnung gestempelt wurde, kann sich noch im selben Zeitstempel-Takt aendern, daher wird ihr Ergebnis nie gespeichert (Racy-Git-Regel)."),
    ("sourceAnalysisBytes", 16777216, "The in-process budget of the analysis images of sources (16 MiB); the oldest image goes first.", "Das Prozessbudget der Analyse-Abbilder von Quellen (16 MiB); das aelteste Abbild geht zuerst."),
    ("sourceAnalysisOverheadBytes", 192, "The bytes one analysis image is charged on top of its image and nested observations.", "Die Bytes, die ein Analyse-Abbild zusaetzlich zu Abbild und verschachtelten Beobachtungen kostet."),
    ("settledReadBytes", 134217728, "The in-process budget of settled file reads (128 MiB); a read beyond it clears the memo.", "Das Prozessbudget beruhigter Dateilesungen (128 MiB); eine Lesung darueber leert den Speicher."),
    ("fileRecordImageBytes", 67108864, "The in-process budget of the JSON images of file analyses (64 MiB).", "Das Prozessbudget der JSON-Abbilder von Dateianalysen (64 MiB)."),
    ("fileRecordImageMaximumBytes", 1048576, "The largest JSON image of one file analysis that is remembered or stored (1 MiB).", "Das groesste JSON-Abbild einer Dateianalyse, das gemerkt oder gespeichert wird (1 MiB)."),
    ("verifiedBlockRows", 4194304, "The most rows of blocks this process verified and remembers at once.", "Die meisten Zeilen von Bloecken, die dieser Prozess verifiziert hat und gleichzeitig merkt."),
    ("blockMinimumRows", 32, "The fewest rows a block holds before the content-defined cut may end it.", "Die wenigsten Zeilen eines Blocks, bevor der inhaltsbestimmte Schnitt ihn beenden darf."),
    ("blockCutMask", 127, "The mask of the content-defined block cut: a row ends a block when its key hash under this mask is zero.", "Die Maske des inhaltsbestimmten Blockschnitts: eine Zeile beendet einen Block, wenn ihr Schluessel-Hash unter dieser Maske null ist."),
    ("blockRefusalMilliseconds", 30000, "How long a directory that refused a block publication is not asked again.", "Wie lange ein Verzeichnis, das eine Blockveroeffentlichung verweigert hat, nicht erneut gefragt wird."),
    ("detachedLayerRows", 64, "The rows of one detached observation layer.", "Die Zeilen einer abgetrennten Beobachtungsschicht."),
    ("leafReadChunkBytes", 65536, "The size of one read while an observation leaf is loaded (64 KiB).", "Die Groesse eines Lesevorgangs beim Laden eines Beobachtungsblatts (64 KiB)."),
    ("sweepGraceMilliseconds", 600000, "How long a superseded block stays before the sweep may remove it.", "Wie lange ein abgeloester Block bleibt, bevor der Sweep ihn entfernen darf."),
    ("fingerprintProgressRows", 128, "After how many source rows the preparation reports progress and checks for cancellation.", "Nach wie vielen Quellzeilen die Vorbereitung Fortschritt meldet und auf Abbruch prueft."),
    ("sweepHeaderProgressRows", 256, "After how many headers the sweep reports progress.", "Nach wie vielen Kopfzeilen der Sweep Fortschritt meldet."),
    ("sweepBlockProgressRows", 1024, "After how many blocks the sweep reports progress.", "Nach wie vielen Bloecken der Sweep Fortschritt meldet."),
    ("settledReadMaximumFileBytes", 4194304, "The largest file whose read is remembered as a settled read (4 MiB).", "Die groesste Datei, deren Lesung als beruhigte Lesung gemerkt wird (4 MiB)."),
    ("settleRounds", 64, "The most rounds a computation is repeated while the files it read keep changing.", "Die meisten Runden, die eine Berechnung wiederholt wird, solange sich die gelesenen Dateien weiter aendern."),
    ("holdNestingLimit", 64, "The deepest nesting of analyses whose recorded observations are checked again.", "Die tiefste Verschachtelung von Analysen, deren aufgezeichnete Beobachtungen erneut geprueft werden."),
    ("maximumIdentifierCharacters", 256, "The most characters of the identity of a registered source analysis.", "Die meisten Zeichen der Identitaet einer registrierten Quellanalyse."),
    ("nativeRealpathMaximumCharacters", 260, "A path shorter than this is resolved with the native realpath (the classic Windows path limit).", "Ein Pfad, der kuerzer ist, wird mit dem nativen realpath aufgeloest (die klassische Windows-Pfadgrenze)."),
])], key="$defs")
M = O + "🟨️.mjs"
b.subs(M, [
    ("import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from", "import{REVISION_POLICY_V1,captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"),
    ("fileURLToPath(observationSchemaURLV1),65536,true)", "fileURLToPath(observationSchemaURLV1),REVISION_POLICY_V1.maximumTextBytes,true)"),
    ("fileURLToPath(observationSchemaURLV1),65536).sha256", "fileURLToPath(observationSchemaURLV1),REVISION_POLICY_V1.maximumTextBytes).sha256"),
    ("import.meta.semioPhysicalSourceURL)),8388608).sha256", "import.meta.semioPhysicalSourceURL)),REVISION_POLICY_V1.maximumSourceBytes).sha256", 2),
    ("const RACY_MS = 2000;", "const RACY_MS = OBSERVATION_LIMITS_V1.racyMilliseconds;"),
    ("const SOURCE_ANALYSIS_BYTES = 16 * 1024 * 1024;", "const SOURCE_ANALYSIS_BYTES = OBSERVATION_LIMITS_V1.sourceAnalysisBytes;"),
    ("+192;\n if(size<=SOURCE_ANALYSIS_BYTES)", "+OBSERVATION_LIMITS_V1.sourceAnalysisOverheadBytes;\n if(size<=SOURCE_ANALYSIS_BYTES)"),
    ("id.length>256||id.trim()", "id.length>OBSERVATION_LIMITS_V1.maximumIdentifierCharacters||id.trim()"),
    ("path.length<260?fs.realpathSync.native(path)", "path.length<OBSERVATION_LIMITS_V1.nativeRealpathMaximumCharacters?fs.realpathSync.native(path)"),
    ("SETTLED_READ_BYTES = 128 * 1024 * 1024;", "SETTLED_READ_BYTES = OBSERVATION_LIMITS_V1.settledReadBytes;"),
    ("FILE_RECORD_IMAGE_BYTES = 64 * 1024 * 1024, FILE_RECORD_IMAGE_LIMIT = 1024 * 1024;", "FILE_RECORD_IMAGE_BYTES = OBSERVATION_LIMITS_V1.fileRecordImageBytes, FILE_RECORD_IMAGE_LIMIT = OBSERVATION_LIMITS_V1.fileRecordImageMaximumBytes;"),
    ("Buffer.byteLength(image)<=1024*1024)", "Buffer.byteLength(image)<=FILE_RECORD_IMAGE_LIMIT)"),
    ("temporary.length>256||[...temporary].length>256", "temporary.length>observationBlockPolicyV1.maximumPathUnits||[...temporary].length>observationBlockPolicyV1.maximumPathUnits"),
    ("bytes.length > 4 * 1024 * 1024) return;", "bytes.length > OBSERVATION_LIMITS_V1.settledReadMaximumFileBytes) return;"),
    ("{signal,limit=64,onRound}={}", "{signal,limit=OBSERVATION_LIMITS_V1.settleRounds,onRound}={}"),
    ("observationsHold(rows,64,new Set()", "observationsHold(rows,OBSERVATION_LIMITS_V1.holdNestingLimit,new Set()"),
    ("observationsHold(observations, limit = 64,", "observationsHold(observations, limit = OBSERVATION_LIMITS_V1.holdNestingLimit,"),
    ("received.length%128===0", "received.length%OBSERVATION_LIMITS_V1.fingerprintProgressRows===0"),
    ("Math.min(65536,bytes.length-offset)", "Math.min(OBSERVATION_LIMITS_V1.leafReadChunkBytes,bytes.length-offset)"),
    ("VERIFIED_BLOCK_ROWS=4194304;", "VERIFIED_BLOCK_ROWS=OBSERVATION_LIMITS_V1.verifiedBlockRows;"),
    ("const BLOCK_MINIMUM_ROWS=32,BLOCK_MAXIMUM_ROWS=4096,BLOCK_CUT_MASK=0x7f;", "const BLOCK_MINIMUM_ROWS=OBSERVATION_LIMITS_V1.blockMinimumRows,BLOCK_MAXIMUM_ROWS=observationBlockPolicyV1.maximumRows,BLOCK_CUT_MASK=OBSERVATION_LIMITS_V1.blockCutMask;"),
    ("BLOCK_REFUSAL_MILLISECONDS=30000;", "BLOCK_REFUSAL_MILLISECONDS=OBSERVATION_LIMITS_V1.blockRefusalMilliseconds;"),
    ("DETACHED_LAYER_ROWS=64", "DETACHED_LAYER_ROWS=OBSERVATION_LIMITS_V1.detachedLayerRows"),
    ("if(path.length>256)throw Error(\"Observation block path exceeds256 characters\")", "if(path.length>policy.maximumPathUnits)throw Error(\"Observation block path exceeds \"+policy.maximumPathUnits+\" characters\")"),
    ("graceMilliseconds=600000,now=Date.now()", "graceMilliseconds=OBSERVATION_LIMITS_V1.sweepGraceMilliseconds,now=Date.now()"),
    ("if(at%256===255){await progress?.({phase:\"sweep-headers\"", "if(at%OBSERVATION_LIMITS_V1.sweepHeaderProgressRows===OBSERVATION_LIMITS_V1.sweepHeaderProgressRows-1){await progress?.({phase:\"sweep-headers\""),
    ("if(at%1024===1023){await progress?.({phase:\"sweep-blocks\"", "if(at%OBSERVATION_LIMITS_V1.sweepBlockProgressRows===OBSERVATION_LIMITS_V1.sweepBlockProgressRows-1){await progress?.({phase:\"sweep-blocks\""),
])
b.sub(M, "/** ⏱️ Files stamped this close", "/** ‹E› The declared limits of the observation module; its schema file is their single source. */\nexport const OBSERVATION_LIMITS_V1 = Object.freeze({...sourceAnalysisSchemaV1.$defs.limits.const});\n\n/** ⏱️ Files stamped this close")
KEYS = ["settledReadMaximumFileBytes", "settleRounds", "holdNestingLimit", "racyMilliseconds", "sourceAnalysisBytes", "sourceAnalysisOverheadBytes", "settledReadBytes", "fileRecordImageBytes", "fileRecordImageMaximumBytes", "verifiedBlockRows", "blockMinimumRows", "blockCutMask", "blockRefusalMilliseconds", "detachedLayerRows", "leafReadChunkBytes", "sweepGraceMilliseconds", "fingerprintProgressRows", "sweepHeaderProgressRows", "sweepBlockProgressRows", "maximumIdentifierCharacters", "nativeRealpathMaximumCharacters"]
b.sub(O + "🟨️.d.mts", "/** 🧾️ Describes exactly one byte image", "/** ‹E› The declared limits of the observation module: its schema file is their single source. */\nexport const OBSERVATION_LIMITS_V1: Readonly<{" + ";".join(k + ":number" for k in KEYS) + "}>;\n\n/** 🧾️ Describes exactly one byte image")
b.sub(O + "🧪️tests/🧫️fixtures/🔣️.json", '"declarationExports": [\n      "adoptInferenceAuthorityV1"', '"declarationExports": [\n      "OBSERVATION_LIMITS_V1",\n      "adoptInferenceAuthorityV1"')
T1 = LIB + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts"
b.subs(T1, [
    ("const{captureBoundedSourceRevisionV1,physicalSourceOriginV1,implementationPinnedV1}=await import(args.revisionModule)", "const{REVISION_POLICY_V1,captureBoundedSourceRevisionV1,physicalSourceOriginV1,implementationPinnedV1}=await import(args.revisionModule)"),
    ("'captureBoundedSourceRevisionV1','physicalSourceOriginV1','implementationPinnedV1','staleImplementationV1',args.functions.join", "'REVISION_POLICY_V1','captureBoundedSourceRevisionV1','physicalSourceOriginV1','implementationPinnedV1','staleImplementationV1',args.functions.join"),
    ("beforeSchema,captureBoundedSourceRevisionV1,physicalSourceOriginV1,", "beforeSchema,REVISION_POLICY_V1,captureBoundedSourceRevisionV1,physicalSourceOriginV1,"),
    ('expect(source).toContain("8388608");expect(source).toContain("65536");', 'expect(source.includes("REVISION_POLICY_V1")||source.includes("8388608")&&source.includes("65536")).toBe(true);'),
])
T3 = BUILD + "🧪️tests/🕸️graph/🔁️settle-repeat/🟦️.ts"
b.subs(T3, [
    ('import corpus from"./🔣️.json";\n', 'import corpus from"./🔣️.json";\nimport observationSchema from"../../../👁️observation/🧬️schema/🔣️.json";\n'),
    ("RACY_MS:0}", "RACY_MS:0,OBSERVATION_LIMITS_V1:observationSchema.$defs.limits.const}"),
])
b.row("observation", "library", "🏗️build/👁️observation/🧬️schema/🔣️.json", ["limits"], ["🏗️build/👁️observation/🟨️.mjs"], container="$defs", minimum_number=60, exports=[("🏗️build/👁️observation/🟨️.mjs", "OBSERVATION_LIMITS_V1", "limits")])
b.row("revision-observation", "library", "🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json", ["Policy"], ["🏗️build/👁️observation/🟨️.mjs"], minimum_number=8000)
b.write(tests=["./" + O + "🧪️tests/🟦️.ts", "./" + LAW + "🟦️.ts"])
