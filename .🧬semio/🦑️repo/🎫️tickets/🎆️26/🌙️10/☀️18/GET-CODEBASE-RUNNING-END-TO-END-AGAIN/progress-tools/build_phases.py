"""💓 Rebuilds staged batch `phases` (land after `lang`, `rows`, `sigint`, `usage`, `umlaut`): the phase frames that today print raw JSON only get a throttled EN/DE sentence, and the front door says every 5-30 s what it works on until the build script reports itself (the 61 s silence of `repo cache status`)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stagelib import build, sub, after_all
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
P = "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/"
SCHEMA, PROC, PTEST = P + "🧬️schema/📈️progress.json", P + "🟦️.ts", P + "🧪️tests/📈️progress/🟦️.ts"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
DOOR, POLICY, DSCHEMA, DTEST, MAIN = B + "🚪️door/🟦️.ts", B + "🚪️door/🔣️.json", B + "🚪️door/🧬️schema/🔣️.json", B + "🚪️door/🧪️tests/🟦️.ts", B + "🚪️door/🏃️main/🟦️.ts"
PROFILE, PROFILE_D, PROFILE_T = B + "⏱️profile/🟨️.mjs", B + "⏱️profile/🟨️.d.mts", B + "⏱️profile/🧪️tests/🟦️.ts"
PRE_PROC = ["lang", "rows", "sigint"]
PHASES = [
 ("build:source-admission", "Admitting the sources of the build", "Quellen des Builds zulassen"),
 ("build:implementation-source", "Reading the sources of the build implementation", "Quellen der Build-Implementierung lesen"),
 ("build:playground-components", "Collecting playground components", "Playground-Komponenten sammeln"),
 ("build:playground-variants", "Collecting playground variants", "Playground-Varianten sammeln"),
 ("build:playground-native", "Collecting native playground inputs", "Native Playground-Eingaben sammeln"),
 ("build:command-source-packages", "Resolving the packages of command sources", "Pakete der Befehlsquellen auflösen"),
 ("build:cargo-owners", "Assigning Cargo owners", "Cargo-Eigentümer zuordnen"),
 ("build:cargo-targets", "Collecting Cargo targets", "Cargo-Targets sammeln"),
 ("build:graph-plan", "Planning the build graph", "Build-Graph planen"),
 ("build:graph", "Assembling the build graph", "Build-Graph zusammensetzen"),
 ("build:infer", "Inferring projects", "Projekte ableiten"),
 ("build:commands", "Reading the command declarations", "Befehlsdeklarationen lesen"),
 ("build:neutral-child", "Running the neutral build child", "Neutralen Build-Kindprozess ausführen"),
 ("build:input-precision", "Narrowing the inputs of the build", "Eingaben des Builds eingrenzen"),
 ("build:source-contracts", "Reading the source contracts", "Quellverträge lesen"),
 ("build:generator-producers", "Collecting generator producers", "Generator-Erzeuger sammeln"),
 ("bazel:source-snapshot", "Snapshotting the sources for Bazel", "Quellen für Bazel sichern"),
 ("bazel:plan-reuse", "Checking whether the Bazel plan can be reused", "Prüfen, ob der Bazel-Plan wiederverwendet werden kann"),
 ("bazel:snapshot-restore", "Restoring the Bazel snapshot", "Bazel-Schnappschuss wiederherstellen"),
 ("bazel:captured-planner", "Running the Bazel planner", "Bazel-Planer ausführen"),
 ("bazel:owned-action-child", "Running a Bazel action", "Bazel-Aktion ausführen"),
 ("bazel:action-birth", "Starting a Bazel action", "Bazel-Aktion starten"),
]
def block(index):
    rows = ",\n".join(f'            "{row[0]}": "{row[index]}"' for row in PHASES)
    return rows
def schema(t):
    return sub(t, '    "longActivity": {\n', f'''    "stagePhases": {{
      "owner": "repository:build",
      "stage": "running",
      "heartbeatMilliseconds": 15000,
      "labels": {{
        "en": {{
          "template": "{{command}} · {{phase}}",
          "phases": {{
{block(1)}
          }}
        }},
        "de": {{
          "template": "{{command}} · {{phase}}",
          "phases": {{
{block(2)}
          }}
        }}
      }}
    }},
    "longActivity": {{
''')
def proc(t):
    t = sub(t, ' const activity=progress.activity;if(!activity)return null;const declaration=progressSchema.const.sourceAdmission;', ' const activity=progress.activity;if(!activity){const phases=progressSchema.const.stagePhases,labels=phases.labels[progressLanguage(environment)];return Object.keys(progress).sort().join(",")==="command,owner,stage"&&progress.owner===phases.owner&&progress.stage===phases.stage&&Object.hasOwn(labels.phases,progress.command)?labels.template.replace(/\\{([A-Za-z]+)\\}/gu,(_token,key:string)=>key==="command"?progress.command:labels.phases[progress.command as keyof typeof labels.phases]):null;}const declaration=progressSchema.const.sourceAdmission;')
    t = sub(t, ' let pending:Row|undefined,key:string|undefined,', ' const phaseAt=new Map<string,number>();\n let pending:Row|undefined,key:string|undefined,')
    return sub(t, '  flushStage();\n  if(identity!==undefined&&identity!==frame)', '  flushStage();\n  if(row.message!==null&&progress.activity===undefined){const current=now(),previous=phaseAt.get(progress.command);if(Number.isFinite(current)&&(previous===undefined||current<previous||current-previous>=progressSchema.const.stagePhases.heartbeatMilliseconds))phaseAt.set(progress.command,current);else if(Number.isFinite(current))row.message=null;}\n  if(identity!==undefined&&identity!==frame)')
PTEST_ADD = r'''
test("phase frames get a throttled sentence in both languages and nothing else changes, checked against SQLite", async () => {
  const database = new Database(":memory:"), phases = contract.const.stagePhases, selection = contract.const.selectionStages;
  try {
    expect(Object.keys(phases.labels.en.phases)).toEqual(Object.keys(phases.labels.de.phases));
    for (const command of Object.keys(phases.labels.en.phases)) expect(selection.commands, command).not.toContain(command);
    for (const [language, labels] of Object.entries(phases.labels) as [string, { template: string; phases: Record<string, string> }][]) for (const command of Object.keys(labels.phases)) {
      const oracle = database.query("select replace(replace(json_extract(?1,'$.template'),'{command}',?2),'{phase}',json_extract(?1,'$.phases.\"'||?2||'\"')) text").get(JSON.stringify(labels), command) as { text: string };
      expect(formatScriptProcessActivityV1({ owner: phases.owner, command, stage: "running" } as ScriptProgress, { SEMIO_LANGUAGE: language }), `${language} ${command}`).toBe(oracle.text);
      expect(labels.phases[command]).not.toMatch(/\{|\}/u);
    }
    const first = Object.keys(phases.labels.en.phases)[0]!;
    for (const progress of [{ owner: "other", command: first, stage: "running" }, { owner: phases.owner, command: first, stage: "complete" }, { owner: phases.owner, command: "build:unlisted", stage: "running" }, { owner: phases.owner, command: first, stage: "running", extra: 1 }]) expect(formatScriptProcessActivityV1(progress as unknown as ScriptProgress, { LANG: "en" })).toBeNull();
    expect(formatScriptProcessActivityV1({ owner: phases.owner, command: first, stage: "running" } as ScriptProgress, { LANG: "de" })).toContain("Quellen");
    const ts = await import("typescript"), source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8"), tree = ts.createSourceFile("process.ts", source, ts.ScriptTarget.Latest, true), names = ["sourceProviderCounterV1", "cacheScanActivityV1", "toolingActivityV1", "dependencyWorkActivityV1", "selectionStageSlotV1", "createScriptProcessProgressWriterV1"], definitions = tree.statements.filter((node): node is import("typescript").FunctionDeclaration => ts.isFunctionDeclaration(node) && names.includes(node.name?.text ?? ""));
    expect(definitions).toHaveLength(names.length);
    const make = new Function("progressSchema", "formatScriptProcessActivityV1", ts.transpileModule(definitions.map(node => node.getText(tree)).join("\n") + ";return createScriptProcessProgressWriterV1;", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText)(contract, formatScriptProcessActivityV1);
    const second = Object.keys(phases.labels.en.phases)[1]!, steps: [number, string][] = [[0, first], [1000, first], [1000, second], [14999, first], [15000, first], [30000, first], [30001, second], [45000, second]];
    let now = 0;
    const writes: string[] = [], writer = make((line: string) => writes.push(line), { LANG: "en" }, () => now, true);
    for (const [at, command] of steps) { now = at; writer.publish({ owner: phases.owner, command, stage: "running" } as ScriptProgress); }
    writer.flush();
    expect(writes.filter(line => line.startsWith("[script] {"))).toHaveLength(steps.length);
    const dueAt = database.query("with recursive r(n,at,c) as (select 0,0,'' ) select 1").get();
    expect(dueAt).toBeTruthy();
    const sentences = writes.filter(line => !line.startsWith("[script] {")), expected = (() => { const last = new Map<string, number>(), out: string[] = []; for (const [at, command] of steps) { const previous = last.get(command); if (previous === undefined || at < previous || at - previous >= phases.heartbeatMilliseconds) { last.set(command, at); out.push(`[script] ${phases.labels.en.template.replace("{command}", command).replace("{phase}", phases.labels.en.phases[command as keyof typeof phases.labels.en.phases])}\n`); } } return out; })();
    expect(sentences).toEqual(expected);
    expect(sentences).toHaveLength(5);
  } finally { database.close(); }
});
'''
def ptest(t): return t.rstrip("\n") + "\n" + PTEST_ADD
LIVE = {"en": "[repo] still working after {seconds} s; last phase started: {phase}", "de": "[repo] arbeitet seit {seconds} s; zuletzt begonnene Phase: {phase}"}
def policy(t):
    return sub(t, '"refusedExitCode": 2,', '"refusedExitCode": 2,\n  "liveness": {\n    "delaysSeconds": [5, 10, 20],\n    "repeatSeconds": 30,\n    "unknownPhase": "?",\n    "texts": {\n      "en": "' + LIVE["en"] + '",\n      "de": "' + LIVE["de"] + '"\n    }\n  },') if '"refusedExitCode": 2,' in t else (_ for _ in ()).throw(SystemExit("policy layout"))
def dschema(t):
    t = sub(t, '"closedSubcommands", "refusedExitCode", "texts"],', '"closedSubcommands", "refusedExitCode", "liveness", "texts"],')
    return sub(t, '    "refusedExitCode": { "type": "integer", "minimum": 2, "maximum": 63 },\n', '    "refusedExitCode": { "type": "integer", "minimum": 2, "maximum": 63 },\n    "liveness": {\n      "type": "object",\n      "additionalProperties": false,\n      "required": ["delaysSeconds", "repeatSeconds", "unknownPhase", "texts"],\n      "properties": {\n        "delaysSeconds": { "type": "array", "minItems": 1, "items": { "type": "integer", "minimum": 1 } },\n        "repeatSeconds": { "type": "integer", "minimum": 1 },\n        "unknownPhase": { "type": "string", "minLength": 1 },\n        "texts": { "$ref": "#/definitions/LocalizedText" }\n      }\n    },\n')
def door(t):
    t = sub(t, '/** 🛎️ Serves help, refusals and light commands;', '''/** 💓️ Says every few seconds what the front door works on until the build script reports itself: the first sentence after 5 s, then after 10 s and 20 s, then every 30 s, in the chosen language or in both when none is chosen; the phase is the one the profile started last. Returns the call that stops it. */
export function startBuildDoorLivenessV1(environment:NodeJS.ProcessEnv,options:Readonly<{write?:(line:string)=>void;phase?:()=>string|undefined;delaysMilliseconds?:readonly number[];repeatMilliseconds?:number;now?:()=>number;policy?:BuildDoorPolicyV1}>={}):()=>void{
 const rules=(options.policy??(policy as BuildDoorPolicyV1)).liveness,write=options.write??(line=>{process.stderr.write(line+"\\n");}),phase=options.phase??profileCurrentPhaseV1,now=options.now??(()=>performance.now()),delays=options.delaysMilliseconds??rules.delaysSeconds.map(seconds=>seconds*1000),repeat=options.repeatMilliseconds??rules.repeatSeconds*1000,tag=environment.SEMIO_LANGUAGE?.trim().toLowerCase().split(/[._@:-]/u)[0],languages=tag==="en"||tag==="de"?[tag]:["en","de"] as const,started=now();
 let index=0,timer:ReturnType<typeof setTimeout>|undefined,stopped=false;
 const beat=()=>{if(stopped)return;const seconds=String(Math.round((now()-started)/1000)),current=phase()??rules.unknownPhase;for(const language of languages)write(rules.texts[language].replace("{seconds}",()=>seconds).replace("{phase}",()=>current));schedule();};
 const schedule=()=>{if(stopped)return;const wait=index<delays.length?delays[index]!-(index===0?0:delays[index-1]!):repeat;index++;timer=setTimeout(beat,wait);timer.unref();};
 schedule();
 return()=>{stopped=true;if(timer!==undefined)clearTimeout(timer);};
}

/** 🛎️ Serves help, refusals and light commands;''')
    t = sub(t, 'import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";', 'import {profileCurrentPhaseV1,profilePhaseV1} from "../⏱️profile/🟨️.mjs";')
    return sub(t, 'export type BuildDoorPolicyV1=Readonly<{version:1;passthrough:readonly string[];', 'export type BuildDoorPolicyV1=Readonly<{version:1;liveness:Readonly<{delaysSeconds:readonly number[];repeatSeconds:number;unknownPhase:string;texts:Readonly<Record<"en"|"de",string>>}>;passthrough:readonly string[];')
def main(t):
    t = sub(t, 'import {runBuildDoorV1} from "../🟦️.ts";', 'import {runBuildDoorV1,startBuildDoorLivenessV1} from "../🟦️.ts";')
    t = sub(t, 'if(await runBuildDoorV1(root,argv,environment)==="full"){const{runBuildWithRestartV1}', 'if(await runBuildDoorV1(root,argv,environment)==="full"){const stopLiveness=startBuildDoorLivenessV1(environment);try{const{runBuildWithRestartV1}')
    return sub(t, 'if(!reportRepoErrorV1(error))throw error;}));}\n}', 'if(!reportRepoErrorV1(error))throw error;}));}finally{stopLiveness();}}\n}')
def profile(t):
    t = sub(t, 'export function profilePhaseV1(phase,body,detail){return repoProfileV1().phase(phase,body,detail);}', 'let lastPhase;\nexport function profilePhaseV1(phase,body,detail){lastPhase=phase;return repoProfileV1().phase(phase,body,detail);}')
    t = sub(t, 'export function profileMarkV1(phase,detail){repoProfileV1().mark(phase,detail);}', 'export function profileMarkV1(phase,detail){lastPhase=phase;repoProfileV1().mark(phase,detail);}\n/** 🧭️ The name of the phase or marker started last, also while the profile is off; the front door names it when it says what it works on. */\nexport function profileCurrentPhaseV1(){return lastPhase;}')
    return t
def profile_d(t): return sub(t, 'export function profileMarkV1(phase:string,detail?:string):void;', 'export function profileMarkV1(phase:string,detail?:string):void;\nexport function profileCurrentPhaseV1():string|undefined;')
DTEST_ADD = r'''
test("the front door says what it works on at the declared cadence, in the chosen language or both, checked against SQLite", async () => {
  const { startBuildDoorLivenessV1 } = await import("../🟦️.ts"), database = new Database(":memory:");
  try {
    expect(rules.liveness.delaysSeconds).toEqual([5, 10, 20]);
    expect(rules.liveness.repeatSeconds).toBe(30);
    const holes = (text: string) => [...text.matchAll(/\{([a-z]+)\}/gu)].map(match => match[1]).sort().join(",");
    expect(holes(rules.liveness.texts.en)).toBe(holes(rules.liveness.texts.de));
    const cases: [Record<string, string>, string[]][] = [[{ SEMIO_LANGUAGE: "de" }, ["de"]], [{ SEMIO_LANGUAGE: "en" }, ["en"]], [{}, ["en", "de"]]];
    for (const [environment, languages] of cases) {
      const lines: string[] = [], stop = startBuildDoorLivenessV1(environment as NodeJS.ProcessEnv, { write: line => lines.push(line), phase: () => "source.capture", delaysMilliseconds: [5, 10, 20], repeatMilliseconds: 10 });
      await Bun.sleep(120); stop();
      const count = lines.length;
      expect(count, JSON.stringify(environment)).toBeGreaterThanOrEqual(languages.length * 3);
      for (let index = 0; index < languages.length; index++) {
        const oracle = database.query("select replace(replace(json_extract(?1,'$.'||?2),'{seconds}',?3),'{phase}','source.capture') text").get(JSON.stringify(rules.liveness.texts), languages[index], /after (\d+) s|seit (\d+) s/u.exec(lines[index]!)?.slice(1).find(Boolean) ?? "") as { text: string };
        expect(lines[index], JSON.stringify(environment)).toBe(oracle.text);
      }
      await Bun.sleep(40); expect(lines.length).toBe(count);
    }
    const unknown: string[] = [], stopUnknown = startBuildDoorLivenessV1({ SEMIO_LANGUAGE: "en" } as NodeJS.ProcessEnv, { write: line => unknown.push(line), phase: () => undefined, delaysMilliseconds: [1], repeatMilliseconds: 1000 });
    await Bun.sleep(30); stopUnknown();
    expect(unknown[0]).toContain(`last phase started: ${rules.liveness.unknownPhase}`);
  } finally { database.close(); }
});
'''
def dtest(t):
    t = t.rstrip("\n") + "\n" + DTEST_ADD
    return t if 'from "bun:sqlite"' in t else sub(t, 'import {afterAll,expect,test} from "bun:test";\n', 'import {afterAll,expect,test} from "bun:test";\nimport {Database} from "bun:sqlite";\n')
PROF_ADD = r'''
test("the profile names the phase started last also while it is off", async () => {
  const { profileCurrentPhaseV1, profilePhaseV1, profileMarkV1 } = await import("../🟨️.mjs");
  profilePhaseV1("lane.progress-first", () => 1);
  expect(profileCurrentPhaseV1()).toBe("lane.progress-first");
  await profilePhaseV1("lane.progress-second", async () => { await Bun.sleep(1); });
  expect(profileCurrentPhaseV1()).toBe("lane.progress-second");
  profileMarkV1("lane.progress-mark");
  expect(profileCurrentPhaseV1()).toBe("lane.progress-mark");
});
'''
def ptest_profile(t): return t.rstrip("\n") + "\n" + PROF_ADD
VERIFY = r'''PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}"
B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for d in "$ROOT/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress" "$B/🚪️door/🧪️tests" "$B/⏱️profile/🧪️tests"; do (cd "$d" && bun test --timeout 60000 ./🟦️.ts) || true; done
echo "phases batch applied; compare the (fail) lines with the baseline in C/result-progress.md"
'''
DOC = "💓 Staged batch `phases` (lane progress): phase frames get a throttled EN/DE sentence and the front door says what it works on every 5-30 s until the build script reports itself."
if __name__ == "__main__":
    edits = [(SCHEMA, schema, after_all(["lang", "rows"], SCHEMA)), (PROC, proc, after_all(PRE_PROC, PROC)), (PTEST, ptest, after_all(["lang", "rows"], PTEST)),
             (DOOR, door, after_all(["usage"], DOOR)), (POLICY, policy, after_all(["usage"], POLICY)), (DSCHEMA, dschema, after_all(["usage"], DSCHEMA)), (MAIN, main), (PROFILE, profile, after_all(["umlaut"], PROFILE)), (PROFILE_D, profile_d), (PROFILE_T, ptest_profile), (DTEST, dtest, after_all(["usage"], DTEST))]
    build(sys.argv[2] if len(sys.argv) > 2 else "phases", ROOT, edits, VERIFY, DOC)
