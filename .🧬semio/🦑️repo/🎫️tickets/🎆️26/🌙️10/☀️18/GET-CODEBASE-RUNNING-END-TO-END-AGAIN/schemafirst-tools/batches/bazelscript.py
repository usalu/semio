import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

Z = BUILD + "🔧️tools/🏗️bazel/"
b = Batch("bazelscript", "the Bazel script reads its source identity traversal, classification batch, immutable input ceiling, write chunk, reason lengths and cache collection throttle from the Bazel schema, and the path ceiling from its path budget (rebase on the live script: it is a hot file; the Bazel plan fingerprint may include the script bytes)")
b.add_defs(Z + "🧬️schema/🔣️.json", [Def("Limits", "The traversal, batch, byte, text and throttle limits of the Bazel script.", "Die Durchlauf-, Stapel-, Byte-, Text- und Drosselgrenzen des Bazel-Skripts.", [
    ("sourceIdentityDepth", 64, "The deepest directory nesting the identity of a source tree walks.", "Die tiefste Verzeichnisverschachtelung, die die Identitaet eines Quellbaums durchlaeuft."),
    ("sourceIdentityEntries", 1000000, "The most entries the identity of a source tree visits.", "Die meisten Eintraege, die die Identitaet eines Quellbaums besucht."),
    ("classifyBatch", 256, "How many paths are classified at once while the inputs of a plan are expanded.", "Wie viele Pfade gleichzeitig klassifiziert werden, waehrend die Eingaben eines Plans entfaltet werden."),
    ("immutableInputBytes", 25165824, "The most bytes of one immutable input the script writes: a snapshot, an overlay or the loader (24 MiB).", "Die meisten Bytes einer unveraenderlichen Eingabe, die das Skript schreibt: ein Schnappschuss, ein Overlay oder der Lader (24 MiB)."),
    ("writeChunkBytes", 65536, "The size of one write while an immutable input is written (64 KiB).", "Die Groesse eines Schreibvorgangs, waehrend eine unveraenderliche Eingabe geschrieben wird (64 KiB)."),
    ("scopeReasonCharacters", 200, "The most characters of the reason an out-of-date implementation scope reports.", "Die meisten Zeichen des Grundes, den ein veralteter Implementierungs-Scope meldet."),
    ("planReasonCharacters", 300, "The most characters of the reason a failed plan reuse or planner reports.", "Die meisten Zeichen des Grundes, den eine fehlgeschlagene Plan-Wiederverwendung oder ein fehlgeschlagener Planer meldet."),
    ("cacheCollectionThrottleMilliseconds", 120000, "How long after a cache collection another one of the same kind is answered with the earlier result.", "Wie lange nach einer Cache-Sammlung eine weitere derselben Art mit dem frueheren Ergebnis beantwortet wird."),
])])
M = Z + "📜️script.ts"
b.subs(M, [
    ("const inputExpansionPolicy=componentSchema.definitions.InputExpansionPolicy.const;\n", "const inputExpansionPolicy=componentSchema.definitions.InputExpansionPolicy.const;\nconst bazelLimits=componentSchema.definitions.Limits.const,repositoryPathUnits=componentSchema.definitions.PathBudget.const.repositoryUnits;\n"),
    ("depth>64||++visited>1_000_000", "depth>bazelLimits.sourceIdentityDepth||++visited>bazelLimits.sourceIdentityEntries"),
    ("offset+=256){checkScriptInvocation(invocation);const batch=pending.slice(offset,offset+256)", "offset+=bazelLimits.classifyBatch){checkScriptInvocation(invocation);const batch=pending.slice(offset,offset+bazelLimits.classifyBatch)"),
    ("if(path.length>256||Buffer.byteLength(contents)>maximumBytes)", "if(path.length>repositoryPathUnits||Buffer.byteLength(contents)>maximumBytes)"),
    ('if(temporary.length>256)throw new Error("Immutable Bazel input path exceeds 256 characters")', 'if(temporary.length>repositoryPathUnits)throw new Error("Immutable Bazel input path exceeds "+repositoryPathUnits+" characters")'),
    ("Math.min(65536,bytes.length-offset)", "Math.min(bazelLimits.writeChunkBytes,bytes.length-offset)"),
    ("JSON.stringify(keyed),25165824,original)", "JSON.stringify(keyed),bazelLimits.immutableInputBytes,original)"),
    ("JSON.stringify(overlay),25165824,original)", "JSON.stringify(overlay),bazelLimits.immutableInputBytes,original)"),
    ("buildSourceClosureLoaderV1,25165824,original)", "buildSourceClosureLoaderV1,bazelLimits.immutableInputBytes,original)"),
    ("String((error as Error).message).slice(0,200)", "String((error as Error).message).slice(0,bazelLimits.scopeReasonCharacters)"),
    ("String((error as Error)?.message??error).slice(0,300)", "String((error as Error)?.message??error).slice(0,bazelLimits.planReasonCharacters)", 3),
    ("lastCacheCollectionV1.at<120000", "lastCacheCollectionV1.at<bazelLimits.cacheCollectionThrottleMilliseconds"),
])
rel = lambda p: "🏗️build/🔧️tools/🏗️bazel/" + p
b.row("bazel-script", "library", rel("🧬️schema/🔣️.json"), ["Limits"], [rel("📜️script.ts")], minimum_number=200, allow=[(rel("📜️script.ts"), 256, "The octal file mode 0o400 (read-only) is numerically 256; it is a permission, not the classification batch.")])
b.write(tests=["./" + Z + "🧪️tests", "./" + LAW + "🟦️.ts"])
