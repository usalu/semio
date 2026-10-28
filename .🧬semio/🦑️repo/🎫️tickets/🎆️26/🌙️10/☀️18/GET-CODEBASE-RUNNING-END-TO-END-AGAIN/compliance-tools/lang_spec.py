#!/usr/bin/env python3
"""🗣️ Writes batches/lang*.json: the English-only prose literals that grew since HEAD become catalog messages in English and German (the language law of the build library: no new prose literal passes to Error/console). python3 -I lang_spec.py"""
import json, os, subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and os.path.dirname(ROOT) != ROOT: ROOT = os.path.dirname(ROOT)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
P = "🧰️framework/🔨️modules/🏃️process/"
ALLOW = L + "⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json"
TEXTS = {
    "proc": (P + "📖️texts/🟦️.ts", P + "📖️texts/🔣️.json", "refuseProcessV1"),
    "build": (L + "🏗️build/📖️texts/🟦️.ts", L + "🏗️build/📖️texts/🔣️.json", "refuseBuildV1"),
    "cache": (L + "⚡️caching/📖️texts/🟦️.ts", L + "⚡️caching/📖️texts/🔣️.json", "refuseCachingV1"),
    "boot": (L + "⚡️caching/🚀️bootstrap/📖️texts/🟦️.ts", L + "⚡️caching/🚀️bootstrap/📖️texts/🔣️.json", "refuseBootstrapV1"),
}

class Batch:
    def __init__(self, name, title):
        self.name, self.title, self.files, self.manual, self.keys = name, title, [], [], {}
    def file(self, rel):
        if rel not in self.files: self.files.append(rel)
    def use(self, rel, catalog, extra=()):
        self.file(rel)
        names = ",".join([TEXTS[catalog][2], *extra])
        self.manual.append({"rel": rel, "add_import": "import {%s} from \"{FROM}\";" % names, "from_rel": TEXTS[catalog][0]})
    def site(self, rel, old, new, count="any"):
        self.file(rel)
        self.manual.append({"rel": rel, "old": old, "new": new, "count": count, "optional": True})
    def message(self, catalog, key, en, de):
        self.file(TEXTS[catalog][1])
        self.keys.setdefault(TEXTS[catalog][1], {})[key] = {"en": en, "de": de}
    def write(self, verify=()):
        manual = list(self.manual)
        for rel, rows in self.keys.items(): manual.append({"rel": rel, "json_add": {"messages": dict(sorted(rows.items()))}})
        COMBINED["files"] += [x for x in self.files if x not in COMBINED["files"]]
        COMBINED["manual"] += manual
COMBINED = {"files": [], "manual": []}
EMOJI_FILES = {x for x in json.load(open(os.path.join(HERE, "emoji-batches.json"), encoding="utf8")).values() for x in x}

# ---------------------------------------------------------------- process module
b = Batch("langproc", "🗣️ Staged batch `cp-langproc` (lane compliance): the English-only refusals that grew in the process module become messages of its catalog in English and German (no new prose literal passes to `Error`).")
f = P + "🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts"
b.use(f, "proc"); b.site(f, 'throw Error("empty process table")', 'throw refuseProcessV1("invalid","capacity.empty-process-table")')
b.site(f, 'const legacy=pages("Pages free")', 'const settled=pages("Pages free")')
b.site(f, 'pages("File-backed pages")+pages("Pages purgeable"):legacy,compressed', 'pages("File-backed pages")+pages("Pages purgeable"):settled,compressed')
b.site(f, 'Math.min(legacy,reclaimable)', 'Math.min(settled,reclaimable)')
b.message("proc", "capacity.empty-process-table", "The process table is empty", "Die Prozesstabelle ist leer")
f = P + "🏗️build/🟦️typescript/🔁️incremental/🟦️.ts"
b.use(f, "proc")
b.site(f, 'throw new Error("Retained TypeScript programs must be an integer from 1 to "+retentionPolicy.maximumRetainedPrograms)', 'throw refuseProcessV1("invalid","typescript.retained-programs-range",{maximum:retentionPolicy.maximumRetainedPrograms})')
b.site(f, 'new Error("TypeScript check cancelled")', 'refuseProcessV1("cancelled","typescript.check-cancelled")')
b.site(f, 'throw new Error(`TypeScript check switches not admitted by the engine: ${args.join(" ")}`)', 'throw refuseProcessV1("unsupported","typescript.check-switches-not-admitted",{switches:args.join(" ")})')
b.message("proc", "typescript.retained-programs-range", "Retained TypeScript programs must be an integer from 1 to {maximum}", "Die Zahl der behaltenen TypeScript-Programme muss eine ganze Zahl von 1 bis {maximum} sein")
b.message("proc", "typescript.check-cancelled", "TypeScript check cancelled", "TypeScript-Prüfung abgebrochen")
b.message("proc", "typescript.check-switches-not-admitted", "TypeScript check switches not admitted by the engine: {switches}", "Die Engine lässt diese TypeScript-Prüfschalter nicht zu: {switches}")
f = P + "📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts"
b.use(f, "proc"); b.site(f, 'throw Error("Offered Cargo compiler writer custody was never entered")', 'throw refuseProcessV1("invalid","nativeBuild.writer-custody-not-entered")')
b.message("proc", "nativeBuild.writer-custody-not-entered", "Offered Cargo compiler writer custody was never entered", "Die angebotene Schreiber-Obhut des Cargo-Compilers wurde nie betreten")
f = P + "📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🟦️.ts"
b.use(f, "proc"); b.site(f, 'throw Error("Resolved Cargo metadata with a workspace root and packages required")', 'throw refuseProcessV1("invalid","nativeBuild.metadata-incomplete")')
b.message("proc", "nativeBuild.metadata-incomplete", "Resolved Cargo metadata with a workspace root and packages required", "Aufgelöste Cargo-Metadaten mit Workspace-Wurzel und Paketen sind erforderlich")
f = P + "📦️artifacts/📤️publication/🟦️.ts"
b.use(f, "proc"); b.site(f, 'throw Error("Explicit absolute publication lease directory required")', 'throw refuseProcessV1("invalid","publication.lease-directory-required")')
b.message("proc", "publication.lease-directory-required", "Explicit absolute publication lease directory required", "Ein ausdrücklich angegebenes absolutes Lease-Verzeichnis für die Veröffentlichung ist erforderlich")
b.write()

# ---------------------------------------------------------------- caching, bootstrap, cargo, workspaces
b = Batch("langcache", "🗣️ Staged batch `cp-langcache` (lane compliance): the English-only refusals that grew in the caching, bootstrap, cargo timing and cargo preparation modules become catalog messages in English and German; the allowance of the lane module follows the two sites that moved out of it.")
f = L + "⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts"
b.use(f, "cache")
b.site(f, 'throw Error("Cargo step admission requires a compiler configuration view")', 'throw refuseCachingV1("invalid","compiler.configuration-view-required")')
b.site(f, 'throw Error("A Cargo step selecting another configuration requires its own input scope")', 'throw refuseCachingV1("invalid","compiler.other-configuration-needs-scope")')
b.site(f, 'Promise.reject(Error("Live issued Cargo configuration view required"))', 'Promise.reject(refuseCachingV1("invalid","compiler.live-configuration-view-required"))')
b.message("cache", "compiler.configuration-view-required", "Cargo step admission requires a compiler configuration view", "Die Zulassung eines Cargo-Schritts erfordert eine Compiler-Konfigurationsansicht")
b.message("cache", "compiler.other-configuration-needs-scope", "A Cargo step selecting another configuration requires its own input scope", "Ein Cargo-Schritt, der eine andere Konfiguration wählt, braucht seinen eigenen Eingabebereich")
b.message("cache", "compiler.live-configuration-view-required", "Live issued Cargo configuration view required", "Eine aktuell ausgestellte Cargo-Konfigurationsansicht ist erforderlich")
f = L + "⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🟦️.ts"
b.use(f, "cache")
b.site(f, 'throw Error("Lane provenance file changed while it was retained: "+basename(destination))', 'throw refuseCachingV1("stale","lane.provenance-changed",{file:basename(destination)})')
b.site(f, 'throw Error("Lane provenance file changed while it was retained: "+basename(source))', 'throw refuseCachingV1("stale","lane.provenance-changed",{file:basename(source)})')
b.message("cache", "lane.provenance-changed", "Lane provenance file changed while it was retained: {file}", "Die Herkunftsdatei der Lane hat sich während der Aufbewahrung geändert: {file}")
f = L + "⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"
b.use(f, "boot"); b.site(f, 'throw Error("Dashboard executable sources record required")', 'throw refuseBootstrapV1("invalid","dashboard.sources-record-required")')
b.message("boot", "dashboard.sources-record-required", "Dashboard executable sources record required", "Der Quellenbeleg der Dashboard-Programmdatei ist erforderlich")
f = L + "⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🟦️.ts"
b.use(f, "boot"); b.site(f, 'throw Error("Closed current immutable tooling cohort required")', 'throw refuseBootstrapV1("invalid","tools.cohort-required")')
b.message("boot", "tools.cohort-required", "Closed current immutable tooling cohort required", "Eine geschlossene, aktuelle, unveränderliche Werkzeuggruppe ist erforderlich")
f = L + "🏃️process/🦀️cargo/⏱️timing/🟦️.ts"
b.use(f, "proc"); b.site(f, 'throw Error("Actual process id required")', 'throw refuseProcessV1("invalid","cargoTiming.process-id-required")')
b.message("proc", "cargoTiming.process-id-required", "Actual process id required", "Eine tatsächliche Prozess-ID ist erforderlich")
f = L + "🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/🟦️.ts"
b.use(f, "build"); b.site(f, 'throw Error("Bounded Cargo preparation record required")', 'throw refuseBuildV1("invalid","cargo.preparation-record-bounded")')
ft = L + "🏗️build/📖️texts/🧪️tests/🟦️.ts"
b.site(ft, 'foreignOwners = [join(owner, "..", "⚡️caching", "🚀️bootstrap", "🔌️nx-plugin", "🧮️engine-digest")]', 'foreignOwners = [join(owner, "..", "⚡️caching", "🚀️bootstrap", "🔌️nx-plugin", "🧮️engine-digest"), join(owner, "..", "🗂️workspaces", "🦀️cargo", "🛠️preparation", "👁️witness")]')
b.message("build", "cargo.preparation-record-bounded", "Bounded Cargo preparation record required", "Ein begrenzter Cargo-Vorbereitungsbeleg ist erforderlich")
b.file(L + "⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts")
b.write()

# ---------------------------------------------------------------- build library
b = Batch("langbuild", "🗣️ Staged batch `cp-langbuild` (lane compliance): the English-only refusals and warnings that grew in the build library (Bazel script, snapshot, sessions, witness, command closure, graph) become catalog messages in English and German.")
Z = L + "🏗️build/"
f = Z + "🔧️tools/🏗️bazel/📜️script.ts"
b.use(f, "build", ("sayBuildV1",))
b.site(f, 'throw Error("git ls-files failed")', 'throw refuseBuildV1("failed","bazel.git-ls-files-failed")')
b.site(f, 'reject(Error("git answer exceeds 1 MiB"))', 'reject(refuseBuildV1("refused","bazel.git-answer-too-large",{limit:"1 MiB"}))')
b.site(f, 'reject(Error("git answer exceeds 16 MiB"))', 'reject(refuseBuildV1("refused","bazel.git-answer-too-large",{limit:"16 MiB"}))')
b.site(f, 'reject(Error("Bazel source inventory exceeds 64 MiB"))', 'reject(refuseBuildV1("refused","bazel.source-inventory-too-large",{limit:"64 MiB"}))')
b.site(f, 'new Error(`Bazel source entry unreadable (${kind}): ${path}`)', 'refuseBuildV1("failed","bazel.source-entry-unreadable",{kind,path})')
b.site(f, 'throw new Error("Closed Bazel fingerprint plan required: "+label)', 'throw refuseBuildV1("invalid","bazel.fingerprint-plan-invalid",{label})')
b.site(f, 'console.warn("[bazel] plan reuse unavailable, planning completely: "+String((error as Error)?.message??error).slice(0,300))', 'sayBuildV1("bazel.plan-reuse-unavailable",{reason:String((error as Error)?.message??error).slice(0,300)},"warn")')
b.site(f, 'console.warn("[bazel] the planner failed under the world witness, planning again without it: "+String((error as Error)?.message??error).slice(0,300))', 'sayBuildV1("bazel.planner-failed-under-witness",{reason:String((error as Error)?.message??error).slice(0,300)},"warn")')
b.site(f, 'console.warn("[bazel] plan record failed: "+String((error as Error)?.message??error).slice(0,300))', 'sayBuildV1("bazel.plan-record-failed",{reason:String((error as Error)?.message??error).slice(0,300)},"warn")')
b.site(f, 'console.warn(`[bazel] idle servers not reaped: ${(error as Error).message}`)', 'sayBuildV1("bazel.idle-servers-not-reaped",{reason:(error as Error).message},"warn")')
b.message("build", "bazel.git-ls-files-failed", "git ls-files failed", "git ls-files ist fehlgeschlagen")
b.message("build", "bazel.git-answer-too-large", "git answer exceeds {limit}", "Die Antwort von git überschreitet {limit}")
b.message("build", "bazel.source-inventory-too-large", "Bazel source inventory exceeds {limit}", "Das Bazel-Quellenverzeichnis überschreitet {limit}")
b.message("build", "bazel.source-entry-unreadable", "Bazel source entry unreadable ({kind}): {path}", "Bazel-Quelleintrag nicht lesbar ({kind}): {path}")
b.message("build", "bazel.fingerprint-plan-invalid", "Closed Bazel fingerprint plan required: {label}", "Ein geschlossener Bazel-Fingerabdruckplan ist erforderlich: {label}")
b.message("build", "bazel.plan-reuse-unavailable", "[bazel] plan reuse unavailable, planning completely: {reason}", "[bazel] Planwiederverwendung nicht verfügbar, es wird vollständig geplant: {reason}")
b.message("build", "bazel.plan-record-failed", "[bazel] plan record failed: {reason}", "[bazel] Der Planbeleg konnte nicht geschrieben werden: {reason}")
f = Z + "🔧️tools/🏗️bazel/📸️snapshot/🟦️.ts"
b.use(f, "build")
b.site(f, 'throw Error(`Bazel source alias collision: ${name}: ${known} and ${folder}`)', 'throw refuseBuildV1("invalid","bazel.source-alias-collision",{name,known,folder})')
b.site(f, 'throw Error("Bounded Bazel source snapshot plan required")', 'throw refuseBuildV1("invalid","bazel.snapshot-plan-bounded")')
b.site(f, 'throw Error("Closed Bazel source snapshot plan required")', 'throw refuseBuildV1("invalid","bazel.snapshot-plan-closed")')
b.message("build", "bazel.source-alias-collision", "Bazel source alias collision: {name}: {known} and {folder}", "Kollision von Bazel-Quellaliasen: {name}: {known} und {folder}")
ts = Z + "🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🟦️.ts"
b.site(ts, '.rejects.toThrow("alias collision")', '.rejects.toThrow("source-alias-collision")')
b.message("build", "bazel.snapshot-plan-bounded", "Bounded Bazel source snapshot plan required", "Ein begrenzter Bazel-Quellabbildplan ist erforderlich")
b.message("build", "bazel.snapshot-plan-closed", "Closed Bazel source snapshot plan required", "Ein geschlossener Bazel-Quellabbildplan ist erforderlich")
f = Z + "🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts"
b.use(f, "build"); b.site(f, 'throw Error("Non-negative Bazel server budget required")', 'throw refuseBuildV1("invalid","bazel.server-budget-invalid")')
b.message("build", "bazel.server-budget-invalid", "Non-negative Bazel server budget required", "Ein nicht negatives Bazel-Serverbudget ist erforderlich")
f = Z + "🔭️witness/🟨️.mjs"
b.use(f, "build")
b.site(f, 'throw Error("Declared language and message required")', 'throw refuseBuildV1("invalid","witness.language-and-message-required")')
b.site(f, 'throw Error("Bounded world witness required")', 'throw refuseBuildV1("invalid","witness.bounded-required")')
b.site(f, 'throw Error("Closed world witness required")', 'throw refuseBuildV1("invalid","witness.closed-required")')
b.site(f, 'throw Error("Closed world observation required")', 'throw refuseBuildV1("invalid","witness.observation-closed-required")')
b.message("build", "witness.language-and-message-required", "Declared language and message required", "Eine deklarierte Sprache und Nachricht sind erforderlich")
b.message("build", "witness.bounded-required", "Bounded world witness required", "Ein begrenzter Weltnachweis ist erforderlich")
b.message("build", "witness.closed-required", "Closed world witness required", "Ein geschlossener Weltnachweis ist erforderlich")
b.message("build", "witness.observation-closed-required", "Closed world observation required", "Eine geschlossene Weltbeobachtung ist erforderlich")
f = Z + "🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
b.site(f, 'throw new Error(`Duplicate script provider group ${name}`)', 'throw refuseBuildV1("invalid","assembly.duplicate-script-provider-group",{name})')
b.site(f, '/**\n * @param {Record<string, any>} json\n * @param {string} root\n * @param {string} projectDir\n */\nexport async function projectWithDefaults', '/**\n * The project configuration completed with its declared defaults, inferred inputs and targets.\n * @param {Record<string, any>} json\n * @param {string} root\n * @param {string} projectDir\n */\nexport async function projectWithDefaults')
b.message("build", "assembly.duplicate-script-provider-group", "Duplicate script provider group {name}", "Doppelte Anbietergruppe für Skripte {name}")
f = Z + "🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"
b.use(f, "build")
b.site(f, 'throw Error(`Expected ${GRAPH_MEMO_POLICY_V1.derive} / Erwartet ${GRAPH_MEMO_POLICY_V1.derive}`)', 'throw refuseBuildV1("usage","memo.derive-variable-required",{variable:GRAPH_MEMO_POLICY_V1.derive})')
b.site(f, 'throw Error(`Closed derivation request required: ${file} / Geschlossene Ableitungsanfrage erforderlich: ${file}`)', 'throw refuseBuildV1("invalid","memo.derive-request-invalid",{file})')
b.message("build", "memo.derive-variable-required", "Expected {variable}", "Erwartet: {variable}")
b.message("build", "memo.derive-request-invalid", "Closed derivation request required: {file}", "Eine geschlossene Ableitungsanfrage ist erforderlich: {file}")
f = Z + "📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts"
b.use(f, "build")
b.site(f, 'throw new Error(`Landlock can only allow: the plan refuses ${rule.path}`)', 'throw refuseBuildV1("unsupported","landlock.allow-only",{path:rule.path})')
b.message("build", "landlock.allow-only", "Landlock can only allow: the plan refuses {path}", "Landlock kann nur erlauben: der Plan verweigert {path}")
f = Z + "📥️source/🔏️closure/🚪️command/🟦️.ts"
b.use(f, "build")
b.site(f, 'throw Error("Adopted captured command snapshot required")', 'throw refuseBuildV1("invalid","closure.adopted-snapshot-required")')
b.site(f, 'throw Error("Bounded adopted command publication required")', 'throw refuseBuildV1("invalid","closure.adopted-publication-bounded")')
b.message("build", "closure.adopted-snapshot-required", "Adopted captured command snapshot required", "Ein übernommenes erfasstes Befehlsabbild ist erforderlich")
b.message("build", "closure.adopted-publication-bounded", "Bounded adopted command publication required", "Eine begrenzte Veröffentlichung des übernommenen Befehls ist erforderlich")
fix = Z + "🧭️invocation/🧪️tests/🔗️process-chain/🔣️.json"
b.site(fix, '"noAdoption": "Adopted captured command snapshot required"', '"noAdoption": "build.closure.adopted-snapshot-required"')
b.site(fix, '"digestMismatch": "Bounded adopted command publication required"', '"digestMismatch": "build.closure.adopted-publication-bounded"')
b.write()

# ---------------------------------------------------------------- two files that were red at HEAD
b = Batch("langtail", "x")
# tests that pinned the old English texts
wt = P + "📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/✍️writer/🟦️.ts"
b.site(wt, 'String(item).includes("Offered Cargo compiler writer custody was never entered")', 'String(item).includes("nativeBuild.writer-custody-not-entered")')
lt = L + "⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🪢️link-less/🟦️.ts"
b.site(lt, '.toThrow("changed while it was retained")', '.toThrow("lane.provenance-changed")')
it = P + "🏗️build/🟦️typescript/🧪️tests/🔁️incremental/🟦️.ts"
b.site(it, '.rejects.toThrow("not admitted")', '.rejects.toThrow("check-switches-not-admitted")')
f = P + "🧪️testing/🗂️suites/🏃️run/🟦️.ts"
b.file(f); b.site(f, 'throw new Error(`Unknown test suite: ${run.suite}`)', 'throw refuseProcessV1("invalid","script.suite-unknown",{suite:run.suite,known:table.suites.map(row=>row.id).join(", ")})')
f = L + "🟨️.mjs"
b.file(f); b.site(f, 'throw Error(`A dependency input of a group holding project references cannot be projected for Nx: ${key}:${name}`)', 'throw refuseBuildV1("invalid","nx.dependency-input-not-projectable",{key,name})')
b.message("build", "nx.dependency-input-not-projectable", "A dependency input of a group holding project references cannot be projected for Nx: {key}:{name}", "Eine Abhängigkeitseingabe einer Gruppe mit Projektverweisen kann nicht für Nx projiziert werden: {key}:{name}")
b.write()

# ---------------------------------------------------------------- wave 2 (files that peers changed after the first cut)
b = Batch("langwave2", "x")
T = P + "🧪️testing/"
f = T + "🎚️budget/⏲️contention/🟦️.ts"
b.use(f, "proc")
b.site(f, 'throw Error("Closed test budget contention policy required")', 'throw refuseProcessV1("invalid","testing.contention-policy-required")')
b.site(f, 'throw Error("Invalid test command budget")', 'throw refuseProcessV1("invalid","testing.command-budget-invalid")')
b.site(f, 'throw Error("Stretch of at least one required")', 'throw refuseProcessV1("invalid","testing.stretch-invalid")')
b.message("proc", "testing.contention-policy-required", "Closed test budget contention policy required", "Eine geschlossene Konkurrenzrichtlinie des Testbudgets ist erforderlich")
b.message("proc", "testing.command-budget-invalid", "Invalid test command budget", "Ungültiges Budget des Testbefehls")
b.message("proc", "testing.stretch-invalid", "Stretch of at least one required", "Eine Dehnung von mindestens eins ist erforderlich")
f = T + "📥️invocation/🟦️.ts"
b.use(f, "proc")
b.site(f, 'throw Error("Finite closed build test envelope required")', 'throw refuseProcessV1("invalid","testing.envelope-required")')
b.site(f, 'throw Error("One test process receives one birth: "+exact.owner+" declares another envelope than the one "+declared.owner+" declared")', 'throw refuseProcessV1("invalid","testing.birth-single",{owner:exact.owner,declared:declared.owner})')
b.site(f, 'throw Error("Build test envelope declared after the first callback: "+exact.owner)', 'throw refuseProcessV1("invalid","testing.envelope-late",{owner:exact.owner})')
b.site(f, 'Promise.reject(Error("Registered build test grant issuer required: "+grant.kind+" for "+envelope.owner))', 'Promise.reject(refuseProcessV1("invalid","testing.grant-issuer-required",{kind:grant.kind,owner:envelope.owner}))')
b.site(f, 'differ=()=>Error("Inherited build test capabilities differ from the declared envelope of "+envelope.owner)', 'differ=()=>refuseProcessV1("invalid","testing.capabilities-differ",{owner:envelope.owner})')
b.message("proc", "testing.envelope-required", "Finite closed build test envelope required", "Eine endliche, geschlossene Hülle des Build-Tests ist erforderlich")
b.message("proc", "testing.birth-single", "One test process receives one birth: {owner} declares another envelope than the one {declared} declared", "Ein Testprozess erhält genau eine Geburt: {owner} deklariert eine andere Hülle als die von {declared} deklarierte")
b.message("proc", "testing.envelope-late", "Build test envelope declared after the first callback: {owner}", "Die Hülle des Build-Tests wurde nach dem ersten Rückruf deklariert: {owner}")
b.message("proc", "testing.grant-issuer-required", "Registered build test grant issuer required: {kind} for {owner}", "Ein registrierter Aussteller der Build-Test-Berechtigung ist erforderlich: {kind} für {owner}")
b.message("proc", "testing.capabilities-differ", "Inherited build test capabilities differ from the declared envelope of {owner}", "Die geerbten Fähigkeiten des Build-Tests weichen von der deklarierten Hülle von {owner} ab")
f = P + "🪪️agent-sessions/🧭️ancestry/🪟️native/🟦️.ts"
b.use(f, "proc")
b.site(f, 'throw Error("Process snapshot refused")', 'throw refuseProcessV1("refused","agentSessions.process-snapshot-refused")')
b.site(f, 'throw Error("Empty process snapshot")', 'throw refuseProcessV1("invalid","agentSessions.process-snapshot-empty")')
b.message("proc", "agentSessions.process-snapshot-refused", "Process snapshot refused", "Prozessabbild abgelehnt")
b.message("proc", "agentSessions.process-snapshot-empty", "Empty process snapshot", "Leeres Prozessabbild")
f = L + "⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"
b.site(f, 'throw Error("Dashboard artifact changed during installation")', 'throw refuseBootstrapV1("stale","dashboard.artifact-changed")')
b.message("boot", "dashboard.artifact-changed", "Dashboard artifact changed during installation", "Das Dashboard-Artefakt hat sich während der Installation geändert")
f = Z + "📥️source/🔏️closure/🚪️command/🟦️.ts"
b.site(f, 'throw Error("Authenticated captured build command entry required")', 'throw refuseBuildV1("invalid","closure.command-entry-authentication-required")')
b.message("build", "closure.command-entry-authentication-required", "Authenticated captured build command entry required", "Ein beglaubigter erfasster Build-Befehlseintrag ist erforderlich")
f = Z + "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs"
b.use(f, "build")
b.site(f, 'throw Error("Declared graph farm temporary directory kind required: " + kind)', 'throw refuseBuildV1("invalid","graph.farm-directory-kind-required",{kind})')
b.message("build", "graph.farm-directory-kind-required", "Declared graph farm temporary directory kind required: {kind}", "Eine deklarierte Art des temporären Graph-Farm-Verzeichnisses ist erforderlich: {kind}")
f = Z + "🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"
b.use(f, "build")
b.site(f, 'throw Error("Canonical command closure reference required: "+JSON.stringify(input).slice(0,256))', 'throw refuseBuildV1("invalid","graph.command-closure-reference-required",{input:JSON.stringify(input).slice(0,256)})')
b.site(f, "/**\n * @param {string[]} configFiles\n * @param {unknown} options\n * @param {{ workspaceRoot: string }} context\n */\n/** 🛡️ Admits", "/** 🛡️ Admits")
b.message("build", "graph.command-closure-reference-required", "Canonical command closure reference required: {input}", "Ein kanonischer Verweis auf den Befehlsabschluss ist erforderlich: {input}")
f = Z + "🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
b.use(f, "build")
b.site(f, 'throw Error("Generator contracts of the parent inference required")', 'throw refuseBuildV1("invalid","graph.generator-contracts-required")')
b.message("build", "graph.generator-contracts-required", "Generator contracts of the parent inference required", "Die Generatorverträge der übergeordneten Inferenz sind erforderlich")
for rel in [P + "📜️script.ts", L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts", Z + "📥️inputs/🔏️revision/🟨️.mjs", Z + "📥️inputs/🧮️digest/📥️read/🟨️.mjs"]: b.file(rel)
# machine tokens where a catalog cannot be reached (the text renderer of the fleet status, a worker script, the test-birth helpers)
f = Z + "🚢️fleet-status/🟦️.ts"
b.file(f); b.site(f, 'throw Error(`Unknown fleet text ${key}`)', 'throw Error(`fleet-text-unknown:${key}`)')
f = L + "🧪️testing/🪪️native-birth/🟦️.ts"
b.file(f); b.site(f, 'throw Error("Checkout holding MODULE.bazel required for a bare library test")', 'throw Error("library-test-checkout-required")')
# allowance rows that only went stale because peers moved sites out: the files join the batch so the allowlist is re-cut from them
for rel in [P + "📦️artifacts/🏗️native-build/🟦️.ts", L + "⚡️caching/📦️artifacts/🏃️contract-capture/🟦️.ts", L + "🏃️process/🦀️cargo/🚪️launcher/🟦️.ts", Z + "🔧️tools/🏗️bazel/⚙️operations/🟦️.ts", Z + "🔧️tools/🏗️bazel/🔌️backend/🟦️.ts"]: b.file(rel)
# tests that pinned the old English texts
def pin(old, new, under=""):
    out = subprocess.run(["git", "grep", "-l", "-F", "--", old, "--", "🧰️framework"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    for rel in out:
        if rel and ("🧪️tests" in rel) and rel.endswith(".ts") and under in rel: b.site(rel, old, new)
pin('.toBe("Generator contracts of the parent inference required")', '.toBe("invalid build.graph.generator-contracts-required {}")')
pin('fault.message==="Authenticated captured build command entry required"&&fault.code===undefined', 'fault.message==="invalid build.closure.command-entry-authentication-required {}"')
pin('expect(error.message).toBe("Authenticated captured build command entry required")', 'expect(error.message).toBe("invalid build.closure.command-entry-authentication-required {}")')
pin('expect(error.code).toBeUndefined()', 'expect(error.code).toBe("invalid")', "🏷️loader-identity/")
pin('.toThrow("Closed test budget contention policy required")', '.toThrow("testing.contention-policy-required")')
pin('.toThrow("Invalid test command budget")', '.toThrow("testing.command-budget-invalid")', "⏲️contention/")
pin('.toThrow("Stretch of at least one required")', '.toThrow("testing.stretch-invalid")')
pin('.toContain("Finite closed build test envelope required")', '.toContain("testing.envelope-required")')
pin('.toContain("One test process receives one birth")', '.toContain("testing.birth-single")')
pin('.toContain("Registered build test grant issuer required: native-build-caller")', '.toContain("testing.grant-issuer-required")')
pin('.toContain("Inherited build test capabilities differ from the declared envelope of test:declared-writer")', '.toContain("testing.capabilities-differ")')
b.write()

# ---------------------------------------------------------------- sealed implementation
b = Batch("langsealed", "🗣️ Staged batch `cp-langsealed` (lane compliance): the English-only guards of the sealed implementation layer render from its own declared texts (English and German, ASCII German like the rest of the module); the two guards inside the text renderer carry machine tokens.")
S = Z + "🏛️sealed/"
b.file(S + "🔣️.json"); b.file(S + "🧬️schema/🔣️.json")
f = S + "🟦️.ts"
b.file(f)
b.site(f, 'throw Error("Declared sealed implementation text required: "+key)', 'throw Error("sealed-text-undeclared:"+key)')
b.site(f, 'throw Error("Sealed implementation text placeholder required: "+name)', 'throw Error("sealed-text-placeholder-missing:"+name)')
b.site(f, 'throw Error("workspace root with paired build and package markers was not found")', 'throw Error(sealedTextV1(sealedLanguagesV1(),"workspaceRootMissing"))')
f = S + "📤️promotion/🟦️.ts"
b.site(f, 'throw Error("Closed blocked record required")', 'throw Error(sealedTextV1(sealedLanguagesV1(environment,policy),"blockedRecordInvalid",{},policy))')
f = S + "📥️execution/🟦️.ts"
b.site(f, 'throw Error("Bounded captured command publication required")', 'throw Error(sealedTextV1(sealedLanguagesV1(environment),"commandPublicationBounded"))')
f = S + "🔁️promoter/🟦️.ts"
b.file(f)
b.site(f, 'import{sealedEntryOfV1,sealedPolicyV1}from"../🟦️.ts";', 'import{sealedEntryOfV1,sealedLanguagesV1,sealedPolicyV1,sealedTextV1}from"../🟦️.ts";')
b.site(f, 'throw Error("Bounded admitted compiler image required")', 'throw Error(sealedTextV1(sealedLanguagesV1(),"compilerImageBounded"))')
b.site(f, 'throw Error("Bounded admitted compiler closure required")', 'throw Error(sealedTextV1(sealedLanguagesV1(),"compilerClosureBounded"))')
texts = {
    "workspaceRootMissing": ("[implementation] No workspace root with paired build and package markers was found.", "[implementation] Es wurde keine Workspace-Wurzel mit zusammengehoerigen Build- und Paketmarken gefunden."),
    "blockedRecordInvalid": ("[implementation] The record of the blocked promotion is not closed.", "[implementation] Der Beleg der blockierten Befoerderung ist nicht geschlossen."),
    "commandPublicationBounded": ("[implementation] The captured command exceeds the bound of its publication.", "[implementation] Der erfasste Befehl ueberschreitet die Grenze seiner Veroeffentlichung."),
    "compilerImageBounded": ("[implementation] The admitted compiler image exceeds its bound.", "[implementation] Das zugelassene Compiler-Abbild ueberschreitet seine Grenze."),
    "compilerClosureBounded": ("[implementation] The admitted compiler closure exceeds its bound.", "[implementation] Der zugelassene Compiler-Abschluss ueberschreitet seine Grenze."),
}
b.manual.append({"rel": S + "🔣️.json", "json_add": {"texts": {k: {"en": en, "de": de} for k, (en, de) in sorted(texts.items())}}, "indent": 2})
schema = S + "🧬️schema/🔣️.json"
b.site(schema, '"selfCheckEnded", "stale", "refused", "unpromoted"],', '"selfCheckEnded", "stale", "refused", "unpromoted", ' + ", ".join('"%s"' % k for k in sorted(texts)) + '],')
b.site(schema, '        "unpromoted": { "$ref": "#/definitions/Text" }\n', '        "unpromoted": { "$ref": "#/definitions/Text" },\n' + ",\n".join('        "%s": { "$ref": "#/definitions/Text" }' % k for k in sorted(texts)) + "\n")
b.write()
files = COMBINED["files"] + [x for x in b.files if x not in COMBINED["files"]]
json.dump({"title": "🗣️ Staged batch `cp-lang` (lane compliance): the English-only prose literals that grew since HEAD (process, caching, bootstrap, cargo, Bazel, witness, graph memo, command closure, sealed layer) become catalog messages in English and German; the language-law allowlist follows the files.", "files": files, "manual": COMBINED["manual"], "emoji": [x for x in files if x in EMOJI_FILES], "allowlist": ALLOW, "verify": []}, open(os.path.join(HERE, "batches", "lang.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("specs written")
