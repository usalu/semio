"""📣 Rebuilds staged batch `say` (land after `umlaut` and `lang`) from the CURRENT live files: the hard-coded English lines `[bazel] waiting 0/58602460`, `[cargo] N diagnostic line(s) not shown`, `[nx-native] staged …`, `[repo cargo] … admitted …`, `[cargo-provenance]/[cargo-target-recipe] {json}` are catalog sentences in English and German."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stagelib import build, sub, after_all
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
F = "🧰️framework/"
NB = F + "🔨️modules/🏃️process/📦️artifacts/🏗️native-build/"
L = F + "🛍️products/🦑️repo/🔨️modules/📚️library/"
B = L + "🏗️build/"
DIAG_SCHEMA, DIAG, DIAG_T = NB + "🧬️schema/📣️diagnostics.json", NB + "📣️diagnostics/🟦️.ts", NB + "📣️diagnostics/🧪️tests/🟦️.ts"
NATIVE, NATIVE_T = NB + "🟦️.ts", NB + "🧪️tests/🟦️.ts"
LAUNCHER, LPROC = L + "🏃️process/🦀️cargo/🚪️launcher/🟦️.ts", L + "🏃️process/🟦️.ts"
TEXTS, SCRIPT, OPS, BACKEND = B + "📖️texts/🔣️.json", B + "🔧️tools/🏗️bazel/📜️script.ts", B + "🔧️tools/🏗️bazel/⚙️operations/🟦️.ts", B + "🔧️tools/🏗️bazel/🔌️backend/🟦️.ts"
def rel(target, source): return os.path.relpath(os.path.join(ROOT, target), os.path.dirname(os.path.join(ROOT, source))).replace(os.sep, "/")
LABELS = {
 "diagnosticsHidden": ("[cargo] {count} diagnostic line(s) not shown", "[cargo] {count} Diagnosezeile(n) nicht angezeigt"),
 "diagnosticsHiddenLogged": ("[cargo] {count} diagnostic line(s) not shown; complete log: {path}", "[cargo] {count} Diagnosezeile(n) nicht angezeigt; vollständiges Protokoll: {path}"),
 "staged": ("[nx-native] staged {count} deliverable(s) in {directory}", "[nx-native] {count} Ergebnisdatei(en) bereitgestellt in {directory}"),
 "repoCargoAdmittedShared": ("[repo cargo] shared store, {tokens} of {cores} cores admitted, rustc memory budget {budget} GiB", "[repo cargo] gemeinsamer Speicher, {tokens} von {cores} Kernen zugelassen, rustc-Speicherbudget {budget} GiB"),
 "repoCargoAdmittedIsolated": ("[repo cargo] isolated store, {tokens} of {cores} cores admitted, rustc memory budget {budget} GiB", "[repo cargo] isolierter Speicher, {tokens} von {cores} Kernen zugelassen, rustc-Speicherbudget {budget} GiB"),
 "provenanceReading": ("[cargo-provenance] reading build records: {work} file operation(s), {bytes} of {totalBytes} bytes", "[cargo-provenance] Build-Dateien werden gelesen: {work} Dateivorgang/-vorgänge, {bytes} von {totalBytes} Bytes"),
 "recipeReading": ("[cargo-target-recipe] reading the target recipe inputs: {work} file operation(s), {bytes} of {totalBytes} bytes", "[cargo-target-recipe] Eingaben des Target-Rezepts werden gelesen: {work} Dateivorgang/-vorgänge, {bytes} von {totalBytes} Bytes"),
}
def diag_schema():
    body = {"$schema": "http://json-schema.org/draft-07/schema#", "const": {"version": 1, "purpose": "native-build-diagnostics", "labels": {"en": {k: v[0] for k, v in LABELS.items()}, "de": {k: v[1] for k, v in LABELS.items()}}}}
    return (json.dumps(body, ensure_ascii=False, indent=2) + "\n").encode()
DIAG_TS = '''import{nativeProgressLanguagesV1}from"../../../🎛️owned-execution/🟦️.ts";
import diagnostics from"../🧬️schema/📣️diagnostics.json" with{type:"json"};

/** 🔑️ Every diagnostic sentence of the native build. */
export type NativeBuildDiagnosticV1=keyof typeof diagnostics.const.labels.en;

/** 📣️ Renders one diagnostic in the language the environment chooses, in English and German when it chooses none; one line per language. */
export function nativeBuildDiagnosticLinesV1(key:NativeBuildDiagnosticV1,values:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>=process.env):readonly string[]{
 return nativeProgressLanguagesV1(environment).map(language=>diagnostics.const.labels[language][key].replace(/\\{([A-Za-z]+)\\}/gu,(_token,name:string)=>String(values[name]??"")));
}

/** 🪧️ A progress sink that prints the observation rows of the cargo provenance as sentences instead of JSON. */
export function cargoProvenanceConsoleSinkV1(kind:"provenance"|"recipe",environment:Readonly<Record<string,string|undefined>>=process.env):(row:Readonly<{bytes:number;totalBytes:number;work:number}>)=>void{
 return row=>{for(const line of nativeBuildDiagnosticLinesV1(kind==="recipe"?"recipeReading":"provenanceReading",{work:row.work,bytes:row.bytes,totalBytes:row.totalBytes},environment))console.log(line);};
}
'''
DIAG_TEST = '''import { expect, test, spyOn } from "bun:test";
import { Database } from "bun:sqlite";
import diagnostics from "../../🧬️schema/📣️diagnostics.json";
import { cargoProvenanceConsoleSinkV1, nativeBuildDiagnosticLinesV1 } from "../🟦️.ts";

test("every native build diagnostic exists in English and German with the same placeholders and renders as the SQLite oracle says, in the chosen language or in both", () => {
  const database = new Database(":memory:"), labels = diagnostics.const.labels, holes = (text: string) => [...text.matchAll(/\\{([A-Za-z]+)\\}/gu)].map(match => match[1]).sort().join(",");
  try {
    expect(Object.keys(labels.en)).toEqual(Object.keys(labels.de));
    for (const key of Object.keys(labels.en) as (keyof typeof labels.en)[]) {
      expect(holes(labels.en[key]), key).toBe(holes(labels.de[key]));
      expect(labels.en[key], key).not.toBe(labels.de[key]);
      const values = Object.fromEntries([...labels.en[key].matchAll(/\\{([A-Za-z]+)\\}/gu)].map(match => [match[1]!, "v" + match[1]]));
      for (const [environment, languages] of [[{ SEMIO_LANGUAGE: "de" }, ["de"]], [{ LANG: "en_US.UTF-8" }, ["en"]], [{}, ["en", "de"]]] as [Record<string, string>, string[]][]) {
        const lines = nativeBuildDiagnosticLinesV1(key, values, environment);
        expect(lines, key).toHaveLength(languages.length);
        languages.forEach((language, index) => {
          const oracle = database.query("select replace(replace(replace(replace(replace(replace(json_extract(?1,'$.'||?2||'.'||?3),'{count}',?4),'{path}',?5),'{directory}',?6),'{tokens}',?7),'{cores}',?8),'{budget}',?9) line").get(JSON.stringify(labels), language, key, values.count ?? "", values.path ?? "", values.directory ?? "", values.tokens ?? "", values.cores ?? "", values.budget ?? "") as { line: string };
          const expected = oracle.line.replaceAll("{work}", "vwork").replaceAll("{bytes}", "vbytes").replaceAll("{totalBytes}", "vtotalBytes");
          expect(lines[index], `${key} ${language}`).toBe(expected);
        });
      }
    }
    for (const key of Object.keys(labels.en) as (keyof typeof labels.en)[]) { expect(labels.de[key], key).not.toMatch(/\\b(?:the|of|not|shown|staged)\\b/u); expect(labels.de[key], key).not.toMatch(/[a-zA-Z](?:ae|oe|ue)[a-zA-Z]/u); }
    const printed: string[] = [], spy = spyOn(console, "log").mockImplementation(((line: unknown) => { printed.push(String(line)); }) as typeof console.log);
    try { cargoProvenanceConsoleSinkV1("provenance", { SEMIO_LANGUAGE: "de" })({ bytes: 5, totalBytes: 10, work: 3 }); cargoProvenanceConsoleSinkV1("recipe", { SEMIO_LANGUAGE: "en" })({ bytes: 0, totalBytes: 0, work: 1 }); } finally { spy.mockRestore(); }
    expect(printed).toEqual(["[cargo-provenance] Build-Dateien werden gelesen: 3 Dateivorgang/-vorgänge, 5 von 10 Bytes", "[cargo-target-recipe] reading the target recipe inputs: 1 file operation(s), 0 of 0 bytes"]);
    expect(printed.join("\\n")).not.toContain("{");
  } finally { database.close(); }
});
'''
def native(t):
    imp = rel(DIAG, NATIVE).removesuffix("")
    t = sub(t, 'import { captureOwnedCommandV1, startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";', 'import { captureOwnedCommandV1, startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";\nimport { cargoProvenanceConsoleSinkV1, nativeBuildDiagnosticLinesV1 } from "./📣️diagnostics/🟦️.ts";')
    t = sub(t, '    if (this.suppressed > 0) rows.push(`[cargo] ${this.suppressed} diagnostic line(s) not shown${logPath ? `; complete log: ${logPath}` : ""}`);', '    if (this.suppressed > 0) rows.push(...nativeBuildDiagnosticLinesV1(logPath ? "diagnosticsHiddenLogged" : "diagnosticsHidden", { count: this.suppressed, path: logPath ?? "" }));')
    t = sub(t, '    console.log(`[nx-native] staged ${files.size} deliverables in ${relative(policy.cwd, staging).split(sep).join("/")}`);', '    for (const line of nativeBuildDiagnosticLinesV1("staged", { count: files.size, directory: relative(policy.cwd, staging).split(sep).join("/") })) console.log(line);')
    t = sub(t, 'cargoProvenanceProgressSinkV1(row=>console.log("[cargo-target-recipe] "+JSON.stringify(row)))', 'cargoProvenanceProgressSinkV1(cargoProvenanceConsoleSinkV1("recipe"))')
    return sub(t, 'cargoProvenanceProgressSinkV1(row=>console.log("[cargo-provenance] "+JSON.stringify(row)))', 'cargoProvenanceProgressSinkV1(cargoProvenanceConsoleSinkV1("provenance"))')
def native_t(t):
    t = sub(t, "expect(diagnostics.finish(false, path)).toEqual([`[cargo] ${spec.lines - spec.live} diagnostic line(s) not shown; complete log: ${path}`]);", "const chosen = process.env.SEMIO_LANGUAGE; process.env.SEMIO_LANGUAGE = \"en\";\n  try { expect(diagnostics.finish(false, path)).toEqual([`[cargo] ${spec.lines - spec.live} diagnostic line(s) not shown; complete log: ${path}`]); } finally { if (chosen === undefined) delete process.env.SEMIO_LANGUAGE; else process.env.SEMIO_LANGUAGE = chosen; }")
    return t
def launcher(t):
    t = '\n'.join([f'import{{nativeBuildDiagnosticLinesV1}}from"{rel(DIAG, LAUNCHER)}";', t]) if not t.startswith("#!") else t
    return sub(t, 'process.stderr.write(`[repo cargo] ${request.store} store, ${tokens} of ${availableParallelism()} cores admitted, rustc memory budget ${(broker.stats().budgetBytes/1024**3).toFixed(1)} GiB\\n`);', 'for(const line of nativeBuildDiagnosticLinesV1(request.store==="shared"?"repoCargoAdmittedShared":"repoCargoAdmittedIsolated",{tokens,cores:availableParallelism(),budget:(broker.stats().budgetBytes/1024**3).toFixed(1)}))process.stderr.write(line+"\\n");')
def lproc(t):
    t = f'import{{cargoProvenanceConsoleSinkV1}}from"{rel(DIAG, LPROC)}";\n' + t
    return sub(t, 'cargoProvenanceProgressSinkV1(row=>console.log("[cargo-provenance] "+JSON.stringify(row)))', 'cargoProvenanceProgressSinkV1(cargoProvenanceConsoleSinkV1("provenance"))')
BAZEL = {
 "bazel.prepare.waiting": ("[bazel] waiting for the download lease of the Bazel executable ({total} bytes)", "[bazel] wartet auf die Download-Sperre der Bazel-Programmdatei ({total} Bytes)"),
 "bazel.prepare.verifying": ("[bazel] verifying the Bazel executable ({total} bytes)", "[bazel] Bazel-Programmdatei wird geprüft ({total} Bytes)"),
 "bazel.prepare.ready": ("[bazel] the Bazel executable is ready ({total} bytes)", "[bazel] Bazel-Programmdatei ist bereit ({total} Bytes)"),
 "bazel.prepareBun.waiting": ("[bazel] waiting for the download lease of Bun for {platform} ({total} bytes)", "[bazel] wartet auf die Download-Sperre von Bun für {platform} ({total} Bytes)"),
 "bazel.prepareBun.verifying": ("[bazel] verifying Bun for {platform} ({total} bytes)", "[bazel] Bun für {platform} wird geprüft ({total} Bytes)"),
 "bazel.prepareBun.ready": ("[bazel] Bun for {platform} is ready ({total} bytes)", "[bazel] Bun für {platform} ist bereit ({total} Bytes)"),
 "bazel.catalogPreparing": ("[bazel] preparing the fixed embedded module catalog", "[bazel] der eingebettete feste Modulkatalog wird vorbereitet"),
 "bazel.plannerRetry": ("[bazel] the planner failed under the world witness, planning again without it: {reason}", "[bazel] der Planer ist unter den Weltbeobachtungen gescheitert, es wird ohne sie erneut geplant: {reason}"),
 "bazel.idleServersNotReaped": ("[bazel] idle servers not reaped: {reason}", "[bazel] ruhende Server nicht beendet: {reason}"),
 "bazel.cacheActivityNotRecorded": ("[bazel] cache activity not recorded: {reason}", "[bazel] Cache-Aktivität nicht aufgezeichnet: {reason}"),
 "bazel.noAffected": ("[bazel] no affected tasks", "[bazel] keine betroffenen Tasks"),
 "bazel.taskReady": ("[bazel] task {task} is ready at {address}", "[bazel] Task {task} ist erreichbar unter {address}"),
}
def texts(t):
    block = "".join(f'    {json.dumps(k)}: {{\n      "en": {json.dumps(v[0], ensure_ascii=False)},\n      "de": {json.dumps(v[1], ensure_ascii=False)}\n    }},\n' for k, v in BAZEL.items())
    return sub(t, '    "observation.closure-provider-incomplete": {', block + '    "observation.closure-provider-incomplete": {')
def script(t):
    t = f'import{{sayBuildV1}}from"{rel(B + "📖️texts/🟦️.ts", SCRIPT)}";\n' + t
    ready = '(event.phase==="ready"?"bazel.prepare.ready":event.phase==="verifying"?"bazel.prepare.verifying":"bazel.prepare.waiting")'
    t = sub(t, 'console.log(`[bazel] ${event.phase} ${event.bytes}/${event.total}`);', f'sayBuildV1({ready},{{total:event.total}});', 2)
    t = sub(t, 'console.log(`[bazel] bun ${id} ${event.phase} ${event.bytes}/${event.total}`);', 'sayBuildV1(event.phase==="ready"?"bazel.prepareBun.ready":event.phase==="verifying"?"bazel.prepareBun.verifying":"bazel.prepareBun.waiting",{platform:id,total:event.total});')
    t = sub(t, 'console.log("[bazel] preparing fixed embedded module catalog");', 'sayBuildV1("bazel.catalogPreparing");')
    t = sub(t, 'console.warn("[bazel] the planner failed under the world witness, planning again without it: "+String((error as Error)?.message??error).slice(0,300));', 'sayBuildV1("bazel.plannerRetry",{reason:String((error as Error)?.message??error).slice(0,300)},"warn");')
    t = sub(t, 'console.warn(`[bazel] idle servers not reaped: ${(error as Error).message}`);', 'sayBuildV1("bazel.idleServersNotReaped",{reason:(error as Error).message},"warn");')
    return sub(t, 'console.log(`[bazel] task-ready ${task.id} ${address}`);', 'sayBuildV1("bazel.taskReady",{task:task.id,address:String(address)});')
def ops(t):
    t = f'import{{sayBuildV1}}from"{rel(B + "📖️texts/🟦️.ts", OPS)}";\n' + t
    return sub(t, 'console.warn(`[bazel] cache activity not recorded: ${(error as Error).message}`);', 'sayBuildV1("bazel.cacheActivityNotRecorded",{reason:(error as Error).message},"warn");')
def backend(t):
    t = f'import{{sayBuildV1}}from"{rel(B + "📖️texts/🟦️.ts", BACKEND)}";\n' + t
    return sub(t, 'console.log("[bazel] no affected tasks");', 'sayBuildV1("bazel.noAffected");')
VERIFY = r'''PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}"
for d in "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🧪️tests" "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🧪️tests"; do (cd "$ROOT/$d" && bun test --timeout 60000 ./🟦️.ts) || true; done
for f in "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts" "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"; do bun build --no-bundle "$ROOT/$f" > /dev/null || exit 1; done
echo "say batch applied; compare the (fail) lines with the baseline in C/result-progress.md"
'''
DOC = "📣 Staged batch `say` (lane progress, land after umlaut and lang): the hard-coded English lines of the Bazel preparation, the native build diagnostics, `[repo cargo]` and the cargo provenance JSON rows are catalog sentences in English and German."
if __name__ == "__main__":
    edits = [(DIAG_SCHEMA, diag_schema()), (DIAG, DIAG_TS.encode()), (DIAG_T, DIAG_TEST.encode()), (NATIVE, native), (NATIVE_T, native_t), (LAUNCHER, launcher), (LPROC, lproc),
             (TEXTS, texts, after_all(["umlaut"], TEXTS)), (SCRIPT, script), (OPS, ops), (BACKEND, backend)]
    build(sys.argv[2] if len(sys.argv) > 2 else "say", ROOT, edits, VERIFY, DOC)
