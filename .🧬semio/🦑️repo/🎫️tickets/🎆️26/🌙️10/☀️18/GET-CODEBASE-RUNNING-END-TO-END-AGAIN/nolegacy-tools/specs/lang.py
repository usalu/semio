"""🗣️ Batch `nl-lang`: the last 26 findings of the repository language law. English-only refusals become catalog messages in English and German (process, caching, bootstrap, build catalogs; the closure loader's refusal table; the generation pin's own texts), internal invariants of self-contained programs carry machine tokens, and the allowlist rows follow the files with a lower ceiling. Based on compliance's `lang` batch (catalogs and allowlist) and on nl-lease (fleet status)."""
import json, os, re

TITLE = "🗣️ Staged batch `nl-lang` (lane nolegacy): the repository language law goes from 26 findings to 0; 43 English-only refusals become catalog messages in English and German or machine tokens in self-contained programs, the allowlist rows follow with a lower ceiling. Apply after compliance `lang` and nl `lease`."
P = "🧰️framework/🔨️modules/🏃️process/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
CATALOG = {
    "process": (P + "📖️texts/🟦️.ts", P + "📖️texts/🔣️.json", "refuseProcessV1"),
    "build": (L + "🏗️build/📖️texts/🟦️.ts", L + "🏗️build/📖️texts/🔣️.json", "refuseBuildV1"),
    "caching": (L + "⚡️caching/📖️texts/🟦️.ts", L + "⚡️caching/📖️texts/🔣️.json", "refuseCachingV1"),
    "bootstrap": (L + "⚡️caching/🚀️bootstrap/📖️texts/🟦️.ts", L + "⚡️caching/🚀️bootstrap/📖️texts/🔣️.json", "refuseBootstrapV1"),
}
SITES_TS = L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts"
SITES_SCHEMA = L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json"
ALLOW = L + "⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json"
EDITS, MESSAGES = {}, {}


def relative(source, target):
    path = os.path.relpath(target, os.path.dirname(source))
    return path if path.startswith(".") else "./" + path


def add_import(source, target, names):
    line = 'import {%s} from "%s";' % (names, relative(source, target))
    def edit(text):
        rows = text.split("\n")
        found = [i for i, row in enumerate(rows[:400]) if re.match(r'^import\b.*(from\s*["\'][^"\']+["\']|^import\s+["\'][^"\']+["\'])\s*(with\s*\{[^}]*\})?\s*;?\s*$', row)]
        last = max(found) if found else -1
        rows.insert(last + 1, line)
        return "\n".join(rows)
    return edit


def refuse(rel, catalog, sites):
    EDITS.setdefault(rel, []).append(add_import(rel, CATALOG[catalog][0], CATALOG[catalog][2]))
    EDITS[rel].extend((old, new, "any") for old, new, _count in sites)


def message(catalog, key, en, de):
    MESSAGES.setdefault(catalog, {})[key] = {"en": en, "de": de}


def site(rel, old, new, count=1):
    EDITS.setdefault(rel, []).append((old, new, count))


# ---------------------------------------------------------------- process module
f = P + "📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🟦️.ts"
refuse(f, "process", [('throw Error("Unit-aligned Native image size at or above the minimum filesystem required")', 'throw refuseProcessV1("invalid","nativeBuild.image-size-invalid",{bytes,minimum:image.minimumFilesystemBytes,unit:image.unitBytes})', 1)])
message("process", "nativeBuild.image-size-invalid", "Native image size {bytes} must be a multiple of {unit} bytes and at least {minimum} bytes", "Die Größe {bytes} des Native-Images muss ein Vielfaches von {unit} Bytes und mindestens {minimum} Bytes sein")
f = P + "📦️artifacts/🏗️native-build/💾️storage/🟦️.ts"
refuse(f, "process", [('throw Error("Unowned Native namespace path retained")', 'throw refuseProcessV1("refused","nativeBuild.namespace-path-unowned",{path})', 1)])
message("process", "nativeBuild.namespace-path-unowned", "A Native namespace path nobody owns is still present: {path}", "Ein Native-Namensraumpfad ohne Besitzer ist noch vorhanden: {path}")

# ---------------------------------------------------------------- caching and bootstrap
f = L + "⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts"
refuse(f, "caching", [('if(!owner?.live)throw Error("Actual retained compiler constructor required");', 'if(!owner?.live)throw refuseCachingV1("stale","compiler.retained-store-required");', 3)])
message("caching", "compiler.retained-store-required", "The compiler store is no longer retained by the run that opened it", "Der Compiler-Speicher wird vom Lauf, der ihn geöffnet hat, nicht mehr gehalten")
f = L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs"
refuse(f, "caching", [('throw Error("Closed retained snapshot ledger required")', 'throw refuseCachingV1("invalid","headroom.retention-ledger-invalid")', 1)])
message("caching", "headroom.retention-ledger-invalid", "The retained snapshot ledger row needs exactly a byte count and a time", "Die Zeile des Abbildbuchs braucht genau eine Bytezahl und einen Zeitpunkt")
f = L + "🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts"
refuse(f, "process", [
    ('throw Error("Build scheduler executable absent after its build")', 'throw refuseProcessV1("failed","nativeHelpers.scheduler-absent",{path:built??""})', 1),
    ('throw Error("Unit cache compiler wrapper absent after its build")', 'throw refuseProcessV1("failed","nativeHelpers.wrapper-absent",{path:wrapper})', 1),
])
message("process", "nativeHelpers.scheduler-absent", "The build scheduler executable is missing after its build: {path}", "Das Programm des Build-Planers fehlt nach seinem Build: {path}")
message("process", "nativeHelpers.wrapper-absent", "The unit cache compiler wrapper is missing after its build: {path}", "Der Compiler-Wrapper des Unit-Caches fehlt nach seinem Build: {path}")
f = L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"
refuse(f, "bootstrap", [
    ('throw Error("No-follow authorizer directory required")', 'throw refuseBootstrapV1("refused","tools.authorizer-directory-linked",{path})', 1),
    ('throw Error("Authorizer source ancestry changed during capture")', 'throw refuseBootstrapV1("stale","tools.authorizer-ancestry-changed")', 1),
])
message("bootstrap", "tools.authorizer-directory-linked", "An authorizer directory must be a real directory, not a symbolic link: {path}", "Ein Autorisierer-Verzeichnis muss ein echtes Verzeichnis sein, kein symbolischer Link: {path}")
message("bootstrap", "tools.authorizer-ancestry-changed", "A directory above an authorizer source changed while it was captured; run the command again", "Ein Verzeichnis über einer Autorisierer-Quelle hat sich während der Erfassung geändert; führe den Befehl erneut aus")
site(L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️ancestry/🟦️.ts", '.rejects.toThrow("No-follow authorizer directory required")', '.rejects.toThrow("tools.authorizer-directory-linked")')
f = L + "⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts"
refuse(f, "bootstrap", [('throw Error("Bounded selected tooling generations required")', 'throw refuseBootstrapV1("invalid","tools.selection-bounded",{maximum:protocol.maximumCandidates})', 1)])
message("bootstrap", "tools.selection-bounded", "Select at most {maximum} distinct tooling generations by their 64-digit identity", "Wähle höchstens {maximum} verschiedene Werkzeuggenerationen über ihre 64-stellige Identität")
site(L + "⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🎯️selection/🟦️.ts", 'throw Error(\\"Bounded selected tooling generations required\\")', 'throw refuseBootstrapV1(\\"invalid\\",\\"tools.selection-bounded\\",{maximum:protocol.maximumCandidates})')

# ---------------------------------------------------------------- build library
f = L + "🏗️build/🏃️execution/🔒️hermetic/🔌️providers/🔷️typescript/🟦️.ts"
refuse(f, "build", [("throw new Error(`TypeScript check failed (${answer.exitCode}): ${request.project}`)", 'throw refuseBuildV1("failed","hermetic.typescript-check-failed",{project:request.project,status:answer.exitCode})', 1)])
message("build", "hermetic.typescript-check-failed", "The TypeScript check of {project} failed with status {status}", "Die TypeScript-Prüfung von {project} ist mit Status {status} fehlgeschlagen")
f = L + "🏗️build/🏃️execution/🔒️hermetic/🟦️.ts"
refuse(f, "build", [("if (!engine) throw new Error(`Unknown hermetic service: ${name}`);", 'if (!engine) throw refuseBuildV1("invalid", "hermetic.service-unknown", { name });', 1)])
message("build", "hermetic.service-unknown", "No hermetic service is declared under the name {name}", "Unter dem Namen {name} ist kein hermetischer Dienst deklariert")
f = L + "🏗️build/🏡️resident/📜️journal/🟦️.ts"
refuse(f, "build", [
    ('throw Error("The change journal cannot vouch: "+reason+" / Der Änderungsjournal-Dienst kann nicht einstehen: "+reason);', 'throw refuseBuildV1("refused","journal.cannot-vouch",{reason});', 1),
    ('throw Error("--root and --directory required / --root und --directory erforderlich");', 'throw refuseBuildV1("usage","journal.arguments-required");', 1),
])
message("build", "journal.cannot-vouch", "The change journal cannot vouch for the workspace: {reason}", "Das Änderungsjournal kann nicht für den Arbeitsbereich einstehen: {reason}")
message("build", "journal.arguments-required", "The change journal service needs --root and --directory", "Der Dienst des Änderungsjournals braucht --root und --directory")
site(L + "🏗️build/🏡️resident/📜️journal/🧪️tests/🟦️.ts", ".rejects.toThrow(/cannot vouch/u);", ".rejects.toThrow(/journal\\.cannot-vouch/u);")
f = L + "🏗️build/🏡️resident/🔭️watch/🏗️provision/🟨️.mjs"
refuse(f, "build", [('throw new Error("The watcher output root is an image that is not mounted: "+outputRoot+"; it is installed once a build has mounted the tools partition");', 'throw refuseBuildV1("missing","watcher.output-root-not-mounted",{path:outputRoot});', 1)])
message("build", "watcher.output-root-not-mounted", "The watcher output root is an image that is not mounted: {path}; it is installed once a build has mounted the tools partition", "Die Ausgabewurzel des Beobachters ist ein nicht eingehängtes Image: {path}; er wird installiert, sobald ein Build die Werkzeugpartition eingehängt hat")
site(L + "🏗️build/🏡️resident/🧪️tests/🏗️provision/🟦️.ts", ".rejects.toThrow(/not mounted/u);", ".rejects.toThrow(/watcher\\.output-root-not-mounted/u);")
f = L + "🏗️build/🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts"
refuse(f, "build", [("if(!slotPattern.test(slot))throw Error(`Bazel session slot required: ${slot}`);", 'if(!slotPattern.test(slot))throw refuseBuildV1("invalid","bazel.session-slot-invalid",{slot});', 1)])
message("build", "bazel.session-slot-invalid", "Not a Bazel session slot: {slot}", "Kein Slot einer Bazel-Sitzung: {slot}")
f = L + "🏗️build/🕸️graph/⏱️phases/🟨️.mjs"
refuse(f, "build", [
    ('if(!Array.isArray(running))throw Error("Running work list required");', 'if(!Array.isArray(running))throw refuseBuildV1("invalid","graph.phase-running-list-required",{phase});', 1),
    ('if(!state)throw Error("Begun graph phase required");', 'if(!state)throw refuseBuildV1("invalid","graph.phase-not-begun",{phase});', 1),
])
message("build", "graph.phase-running-list-required", "The graph phase {phase} needs the list of the work running in it", "Die Graph-Phase {phase} braucht die Liste der Arbeit, die in ihr läuft")
message("build", "graph.phase-not-begun", "The graph phase {phase} was never begun", "Die Graph-Phase {phase} wurde nie begonnen")
f = L + "🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs"
refuse(f, "build", [('throw Object.assign(new Error(`Rust sources of ${root} cannot be read statically (${set.reason}${set.message ? ": " + set.message : ""}); a key derived from its directory would miss the files its mounts reach outside of it`), { code: "BUILD_RUST_SOURCES_UNRESOLVED", set });', 'throw Object.assign(refuseBuildV1("refused", "graph.rust-sources-unresolved", { root, reason: set.message ? set.reason + ": " + set.message : set.reason }), { set });', 1)])
message("build", "graph.rust-sources-unresolved", "The Rust sources of {root} cannot be read statically ({reason}); a key derived from its directory would miss the files its mounts reach outside of it", "Die Rust-Quellen von {root} lassen sich nicht statisch lesen ({reason}); ein aus seinem Verzeichnis abgeleiteter Schlüssel verfehlte die Dateien, die seine Einbindungen außerhalb davon erreichen")
site(L + "🏗️build/🕸️graph/📥️inference/🧪️tests/🦀️mounted-sources/🟦️.ts", 'expect(thrown?.code,"an unresolved crate refuses the precision derivation").toBe("BUILD_RUST_SOURCES_UNRESOLVED");', 'expect(thrown?.key,"an unresolved crate refuses the precision derivation").toBe("build.graph.rust-sources-unresolved");')
f = L + "🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs"
refuse(f, "build", [('if(names.length>P.maximumSuspects)throw Error("Bounded number of derived facts required");', 'if(names.length>P.maximumSuspects)throw refuseBuildV1("refused","graph.cutoff-suspects-unbounded",{count:names.length,maximum:P.maximumSuspects});', 1)])
message("build", "graph.cutoff-suspects-unbounded", "{count} derived facts exceed the cutoff bound of {maximum}", "{count} abgeleitete Fakten überschreiten die Grenze von {maximum} für den Abschnitt")
f = L + "🏗️build/🕸️graph/🕰️generations/🧷️pin/🟨️.mjs"
site(f, 'if(text===undefined)throw Error("Known generation pin message required: "+key);', 'if(text===undefined)throw Error("generation-pin-message-undeclared:"+key);')
site(f, 'throw Error("Closed generation pin input required");', 'throw generationPinRefusalV1("inputInvalid");')
site(f, 'if(Buffer.byteLength(text)>P.maximumBytes)throw Error("Bounded generation pin required");', 'if(Buffer.byteLength(text)>P.maximumBytes)throw generationPinRefusalV1("unbounded",{maximum:P.maximumBytes});')
site(L + "🏗️build/🕸️graph/🕰️generations/🧷️pin/🧪️tests/🟦️.ts", '.toThrow("Known generation pin message required");', '.toThrow("generation-pin-message-undeclared");')


def pin_schema(text):
    doc = json.loads(text)
    const = doc["definitions"]["Messages"]["const"]
    const["en"]["inputInvalid"] = "graph: the generation pin of this build cannot be written: its owner, generation, manifest or owners are not closed"
    const["de"]["inputInvalid"] = "Graph: die Generationsanheftung dieses Builds lässt sich nicht schreiben: Besitzer, Generation, Manifest oder Besitzerliste sind nicht geschlossen"
    const["en"]["unbounded"] = "graph: the generation pin of this build exceeds {maximum} bytes"
    const["de"]["unbounded"] = "Graph: die Generationsanheftung dieses Builds überschreitet {maximum} Bytes"
    return json.dumps(doc, indent=1, ensure_ascii=False) + "\n"


EDITS[L + "🏗️build/🕸️graph/🕰️generations/🧷️pin/🧬️schema/🔣️.json"] = [pin_schema]
f = L + "🏗️build/🚢️fleet-status/📥️readers/🟦️.ts"
refuse(f, "build", [
    ("if(bad.length)throw Error(`generation pointer: ${JSON.stringify(bad[0])}`);", 'if(bad.length)throw refuseBuildV1("invalid","fleet.generation-pointer-invalid",{detail:JSON.stringify(bad[0])});', 1),
    ("if(shape.length)throw Error(`generation manifest: ${JSON.stringify(shape[0])}`);", 'if(shape.length)throw refuseBuildV1("invalid","fleet.generation-manifest-invalid",{path,detail:JSON.stringify(shape[0])});', 1),
])
message("build", "fleet.generation-pointer-invalid", "The graph generation pointer does not match its schema: {detail}", "Der Zeiger der Graph-Generation passt nicht zu seinem Schema: {detail}")
message("build", "fleet.generation-manifest-invalid", "The graph generation manifest {path} does not match its schema: {detail}", "Das Manifest der Graph-Generation {path} passt nicht zu seinem Schema: {detail}")
f = L + "🧪️testing/🌱️genuine-birth/🟦️.ts"
refuse(f, "build", [('if (artifacts === undefined) throw Error("Test artifact directory required");', 'if (artifacts === undefined) throw refuseBuildV1("missing", "testing.artifact-directory-required");', 1)])
message("build", "testing.artifact-directory-required", "The genuine birth test needs its test artifact directory", "Der Test der echten Geburt braucht sein Verzeichnis für Testartefakte")
f = L + "🧹️normalization/🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts"
refuse(f, "build", [(
    "  for (let left = 0; left < generatorRoots.length; left++) for (let right = left + 1; right < generatorRoots.length; right++) {\n    const a = generatorRoots[left];\n    const b = generatorRoots[right];\n    if(!a||!b)throw new Error(\"Required generator root pair is missing\");\n    if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) throw new Error(`Taxonomy v7 generator output roots overlap: ${a.id}:${a.path} and ${b.id}:${b.path}`);\n  }",
    "  for (const [left, a] of generatorRoots.entries()) for (const b of generatorRoots.slice(left + 1)) {\n    if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) throw refuseBuildV1(\"invalid\", \"taxonomy.generator-output-roots-overlap\", { left: `${a.id}:${a.path}`, right: `${b.id}:${b.path}` });\n  }",
    1)])
message("build", "taxonomy.generator-output-roots-overlap", "Taxonomy v7 generator output roots overlap: {left} and {right}", "Ausgabewurzeln der Generatoren in Taxonomie v7 überlappen sich: {left} und {right}")

# ---------------------------------------------------------------- Bazel hermetic runner: a self-contained program, machine tokens on its socket protocol
f = L + "🏗️build/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🍞️bun/🟦️.ts"
site(f, 'if(message.token!==held.token)throw new Error("service token refused");', 'if(message.token!==held.token)throw new Error("hermetic-service:token-refused");')
site(f, 'if(!active)throw new Error("no active request");', 'if(!active)throw new Error("hermetic-service:no-active-request");')
site(f, 'throw new Error("service path escapes the execution root");', 'throw new Error("hermetic-service:path-escapes-root");')

# ---------------------------------------------------------------- closure loader: its own declared refusal table, passed into the functions it embeds
C = L + "🏗️build/📥️source/🔏️closure/"
REFUSE_TYPE = "refuse:(code:ClosureRefusalCodeV1,fields:Readonly<Record<string,string>>)=>Error"
f = C + "🍞️bun/🟦️.ts"
site(f, 'import{closureRefusalMessagesV1,closureRefusalTextV1,closureTargetSchemesV1}from"', 'import{closureRefusalMessagesV1,closureRefusalTextV1,closureTargetSchemesV1,type ClosureRefusalCodeV1}from"')
site(f, "targets:ReadonlyMap<string,string>):string{", "targets:ReadonlyMap<string,string>," + REFUSE_TYPE + "):string{")
site(f, 'throw Error("Closed compiler comment required")', 'throw refuse("compiler-comment-unclosed",{})')
site(f, 'throw Error("Closed compiler builtin-read import coverage refused")', 'throw refuse("builtin-read-coverage-refused",{})')
site(f, 'throw Error("Mutable compiler builtin read remained")', 'throw refuse("builtin-read-remained",{})')
site(f, 'throw Error("Immutable implementation image changed")', 'throw refuse("implementation-image-changed",{})', 3)
site(f, 'throw Error("Closed immutable implementation image required")', 'throw refuse("implementation-image-required",{})')
site(f, 'throw Error("Closed captured import resolution required")', 'throw refuse("import-resolution-required",{})')
site(f, "throw Error('Bounded immutable implementation snapshot required')", "throw refusal('snapshot-bounded',{})", 4)
site(f, "throw Error('Immutable implementation snapshot changed')", "throw refusal('snapshot-changed',{})", 3)
site(f, "throw Error('Immutable implementation digest required')", "throw refusal('digest-required',{})", 2)
site(f, "throw Error('Immutable snapshot shortened')", "throw refusal('snapshot-shortened',{})")
site(f, "hash,path,check);for(const[name,map]of resolutions)", "hash,path,check,refusal);for(const[name,map]of resolutions)")
site(f, "redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets);", "redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets,refusal);")
site(f, "closureRefusalTextV1=(${closureRefusalTextV1.toString()}),refusalMessages=${JSON.stringify(closureRefusalMessagesV1)},rootScheme=", "rootScheme=")
site(f, "refusal=(code,fields)=>Error(closureRefusalTextV1(refusalMessages,code,fields,process.env)),sourceImportRefusal=", "sourceImportRefusal=")
site(f, "\n${acquisition}\n", "\nconst closureRefusalTextV1=(${closureRefusalTextV1.toString()}),refusalMessages=${JSON.stringify(closureRefusalMessagesV1)},refusal=(code,fields)=>Error(closureRefusalTextV1(refusalMessages,code,fields,process.env));\n${acquisition}\n")


site(f, "isAbsolute(value:string):boolean},check:()=>void):{images:", "isAbsolute(value:string):boolean},check:()=>void," + REFUSE_TYPE + "):{images:")
t = C + "🧪️tests/📥️stream/🟦️.ts"
site(t, 'import{buildSourceClosureStreamLoaderV1,buildSourceClosureIdentityV1,type BuildSourceClosureV1}from"../../🟦️.ts";', 'import{buildSourceClosureStreamLoaderV1,buildSourceClosureIdentityV1,type BuildSourceClosureV1}from"../../🟦️.ts";\nimport{closureRefusalV1}from"%s";' % relative(t, SITES_TS))
site(t, "parser,builtinTargetsV1)", "parser,builtinTargetsV1,closureRefusalV1)", "any")
site(t, "joinerV1,()=>{})", "joinerV1,()=>{},closureRefusalV1)", 2)
site(t, "legacyRedirectV1", "referenceRedirectV1", 2)


def sites_schema(text):
    doc = json.loads(text)
    rows = doc["$defs"]["Messages"]["const"]
    again = ("Fix: run the command again; the outermost command restarts with the implementation now on disk.", "Behebung: Führe den Befehl erneut aus; der äußerste Befehl startet mit der Implementierung neu, die jetzt auf der Platte liegt.")
    add = {
        "compiler-comment-unclosed": ("Closed compiler comment required in a captured module", "Ein geschlossener Kommentar ist in einem erfassten Modul erforderlich", "Fix: close the block comment of the captured source.", "Behebung: Schließe den Blockkommentar der erfassten Quelle."),
        "builtin-read-coverage-refused": ("Closed compiler builtin-read import coverage refused: the rewrite did not count every built-in import", "Geschlossene Abdeckung der Built-in-Leseimporte verweigert: die Umschreibung hat nicht jeden Built-in-Import gezählt", "Fix: import fs, fs/promises and module with literal specifiers in the captured source.", "Behebung: Importiere fs, fs/promises und module in der erfassten Quelle mit literalen Specifiern."),
        "builtin-read-remained": ("Mutable compiler builtin read remained after the rewrite", "Ein veränderlicher Built-in-Lesezugriff blieb nach der Umschreibung bestehen", "Fix: import fs, fs/promises and module with literal specifiers in the captured source.", "Behebung: Importiere fs, fs/promises und module in der erfassten Quelle mit literalen Specifiern."),
        "implementation-image-changed": ("Immutable implementation image changed", "Unveränderliches Implementierungsabbild hat sich geändert") + again,
        "implementation-image-required": ("Closed immutable implementation image required", "Ein geschlossenes unveränderliches Implementierungsabbild ist erforderlich") + again,
        "import-resolution-required": ("Closed captured import resolution required", "Eine geschlossene erfasste Importauflösung ist erforderlich") + again,
        "snapshot-bounded": ("Bounded immutable implementation snapshot required", "Ein begrenztes unveränderliches Implementierungsabbild ist erforderlich", "Fix: keep the captured implementation within the declared snapshot bound.", "Behebung: Halte die erfasste Implementierung innerhalb der deklarierten Abbildgrenze."),
        "snapshot-changed": ("Immutable implementation snapshot changed", "Unveränderliches Implementierungsabbild hat sich geändert") + again,
        "digest-required": ("Immutable implementation digest required", "Ein Digest der unveränderlichen Implementierung ist erforderlich", "Fix: start the loader with the 64-digit digest of its snapshot.", "Behebung: Starte den Lader mit dem 64-stelligen Digest seines Abbilds."),
        "snapshot-shortened": ("Immutable snapshot shortened while it was read", "Unveränderliches Abbild wurde während des Lesens gekürzt") + again,
    }
    for code, (en, de, fix_en, fix_de) in add.items():
        assert code not in rows
        rows[code] = {"en": {"refusal": en, "fix": fix_en}, "de": {"refusal": de, "fix": fix_de}}
    return json.dumps(doc, indent=1, ensure_ascii=False) + "\n"


EDITS[SITES_SCHEMA] = [sites_schema]


# ---------------------------------------------------------------- catalogs
def catalog_edit(catalog):
    def edit(text):
        doc = json.loads(text)
        for key, row in MESSAGES[catalog].items():
            assert key not in doc["messages"], key
            doc["messages"][key] = row
        doc["messages"] = dict(sorted(doc["messages"].items()))
        return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
    return edit


for name in MESSAGES: EDITS[CATALOG[name][1]] = [catalog_edit(name)]

# ---------------------------------------------------------------- allowlist: rows follow the files, the ceiling falls by what they lost
ROWS = {
    P + "📜️script.ts": 1,
    P + "🔒️leases/📦️namespace/📒️baseline/🟦️.ts": 0,
    P + "📦️artifacts/🏗️native-build/💾️storage/🟦️.ts": 95,
    L + "⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts": 7,
    L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs": 57,
    L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts": 16,
    L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts": 25,
    L + "⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts": 76,
    L + "🏗️build/🏃️execution/🔒️hermetic/🔌️providers/🔷️typescript/🟦️.ts": 1,
    L + "🏗️build/🏃️execution/🔒️hermetic/🟦️.ts": 3,
    L + "🏗️build/🏡️resident/🔭️watch/🏗️provision/🟨️.mjs": 3,
    L + "🏗️build/📥️inputs/🔏️revision/🟨️.mjs": 14,
    L + "🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs": 25,
    L + "🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts": 16,
    L + "🏗️build/🕸️graph/⏱️phases/🟨️.mjs": 2,
    L + "🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs": 8,
    L + "🧪️testing/🌱️genuine-birth/🟦️.ts": 5,
    L + "🧹️normalization/🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts": 48,
}


def allowlist(text):
    doc = json.loads(text)
    before = sum(doc["files"].values())
    for rel, count in ROWS.items():
        if count: doc["files"][rel] = count
        else: doc["files"].pop(rel, None)
    after = sum(doc["files"].values())
    assert after <= before, (before, after)
    assert doc["ceiling"] >= after
    doc["ceiling"] = after
    return json.dumps(doc, indent=1, ensure_ascii=False) + "\n"


EDITS[ALLOW] = [allowlist]
TESTS = [
    "./" + C + "🧪️tests/📥️stream/🟦️.ts",
    "./" + L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️ancestry/🟦️.ts",
    "./" + L + "⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🎯️selection/🟦️.ts",
    "./" + L + "🏗️build/🏡️resident/📜️journal/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/🏡️resident/🧪️tests/🏗️provision/🟦️.ts",
    "./" + L + "🏗️build/🕸️graph/📥️inference/🧪️tests/🦀️mounted-sources/🟦️.ts",
    "./" + L + "🏗️build/🕸️graph/🕰️generations/🧷️pin/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/🚢️fleet-status/🧪️tests/🟦️.ts",
    "./" + P + "📖️texts/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/📖️texts/🧪️tests/🟦️.ts",
    "./" + L + "⚡️caching/📖️texts/🧪️tests/🟦️.ts",
    "./" + L + "⚡️caching/🚀️bootstrap/📖️texts/🧪️tests/🟦️.ts",
    "./" + L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts",
    "./" + L + "⚖️laws/🗣️language-catalogs/🧪️tests/🗣️corpus/🟦️.ts",
    "./" + L + "🏗️build/🕸️graph/⏱️phases/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts",
]

# ---------------------------------------------------------------- docstring emojis of the touched files: one per file
site(L + "⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts", "/** 🎯️ Reads immutable exact selected identities without granting payload or ledger authority. */", "/** 🪪️ Reads immutable exact selected identities without granting payload or ledger authority. */")
site(L + "🏗️build/🏃️execution/🔒️hermetic/🟦️.ts", "/** 🛎️ The Bazel mnemonic and instance bound shared by all service actions. */", "/** 🏷️ The Bazel mnemonic and instance bound shared by all service actions. */")
site(L + "🏗️build/🏃️execution/🔒️hermetic/🟦️.ts", "/** 🛎️ Resolves the service of a class;", "/** 🔎️ Resolves the service of a class;")
site(L + "🏗️build/🕸️graph/⏱️phases/🟨️.mjs", "/** 🖨️ Writes the summary of a finished construction:", "/** 📋️ Writes the summary of a finished construction:")
site(L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts", "/** 🧭️ Opens the empty ancestry ledger of one capture. */", "/** 📒️ Opens the empty ancestry ledger of one capture. */")
site(L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts", "/** 🔁️ Serves a capture from its memo when", "/** ♻️ Serves a capture from its memo when")
site(L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs", "/** 📡️ Writes one bounded diagnostic before the unchanged uncaught worker primary. */", "/** 🩺️ Writes one bounded diagnostic before the unchanged uncaught worker primary. */")
site(L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️ancestry/🟦️.ts", '.rejects.toThrow("ancestry changed during capture")', '.rejects.toThrow("tools.authorizer-ancestry-changed")')
site(L + "🏗️build/🚢️fleet-status/🧪️tests/🟦️.ts", ".rejects.toThrow(/generation pointer/u);", ".rejects.toThrow(/fleet\\.generation-pointer-invalid/u);")
site(L + "🏗️build/📖️texts/🧪️tests/🟦️.ts", 'join(owner, "..", "🗂️workspaces", "🦀️cargo", "🛠️preparation", "👁️witness")];', 'join(owner, "..", "🗂️workspaces", "🦀️cargo", "🛠️preparation", "👁️witness"), join(owner, "..", "🧪️testing", "🌱️genuine-birth"), join(owner, "..", "🧹️normalization", "🔣️taxonomy", "🏭️generators-packages-and-exclusions")];')
site(L + "🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs", "/** 🎯️ The suspects a verification scoped to the project names `names` asks again:", "/** 🔍️ The suspects a verification scoped to the project names `names` asks again:")
