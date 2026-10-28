"""🧾 Rebuilds staged batch `rows` (land AFTER `lang`) from the CURRENT live files: the machine rows `[script] {json}` go to pipes and machine readers only; a person at a terminal reads the sentences."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stagelib import build, sub, after
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
D = "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/"
SCHEMA, PROC, PTEST = D + "🧬️schema/📈️progress.json", D + "🟦️.ts", D + "🧪️tests/📈️progress/🟦️.ts"
ROWS = '''    "rows": {
      "variable": "SEMIO_SCRIPT_PROGRESS",
      "values": [
        "machine",
        "human"
      ],
      "terminalDefault": "human",
      "otherwiseDefault": "machine"
    },
'''
def schema(t): return sub(t, '    "longActivity": {\n', ROWS + '    "longActivity": {\n')
def proc(t):
    t = sub(t, 'function createScriptProcessProgressWriterV1(write:(line:string)=>void,environment:Readonly<Record<string,string|undefined>>,now:()=>number=()=>performance.now()){', '/** 🧾️ Whether the typed machine rows `[script] {json}` are written next to the sentences: a pipe or a log (a machine reader) gets them, a person at a terminal reads the sentences only; `SEMIO_SCRIPT_PROGRESS=machine|human` decides explicitly. */\nexport function scriptProcessMachineRowsV1(environment:Readonly<Record<string,string|undefined>>,terminal:boolean):boolean{const rows=progressSchema.const.rows,choice=environment[rows.variable];return choice===rows.values[0]?true:choice===rows.values[1]?false:!terminal;}\nfunction createScriptProcessProgressWriterV1(write:(line:string)=>void,environment:Readonly<Record<string,string|undefined>>,now:()=>number=()=>performance.now(),machine:boolean=true){')
    t = sub(t, 'const emit=(row:Row)=>{write(row.raw);if(row.message!==null)', 'const emit=(row:Row)=>{if(machine)write(row.raw);if(row.message!==null)')
    return sub(t, 'createScriptProcessProgressWriterV1(line=>{process.stderr.write(line);},process.env);', 'createScriptProcessProgressWriterV1(line=>{process.stderr.write(line);},process.env,undefined,scriptProcessMachineRowsV1(process.env,process.stderr.isTTY===true));')
TEST = r'''
test("the machine rows go to pipes and machine readers and are left out for a person at a terminal while every sentence stays, decided by the declared variable and checked against SQLite", async () => {
  const database = new Database(":memory:"), { scriptProcessMachineRowsV1 } = await import("../../🟦️.ts");
  try {
    const cases: [Record<string, string>, boolean, number][] = [[{}, true, 0], [{}, false, 1], [{ SEMIO_SCRIPT_PROGRESS: "machine" }, true, 1], [{ SEMIO_SCRIPT_PROGRESS: "human" }, false, 0], [{ SEMIO_SCRIPT_PROGRESS: "other" }, true, 0], [{ SEMIO_SCRIPT_PROGRESS: "other" }, false, 1]];
    for (const [environment, terminal, expected] of cases) {
      const oracle = database.query("select case ?1 when 'machine' then 1 when 'human' then 0 else 1-?2 end rows").get(environment.SEMIO_SCRIPT_PROGRESS ?? null, terminal ? 1 : 0) as { rows: number };
      expect(oracle.rows).toBe(expected);
      expect(scriptProcessMachineRowsV1(environment, terminal), JSON.stringify([environment, terminal])).toBe(expected === 1);
    }
    const row = fixture.cases.find(candidate => candidate.expected !== null)!, progress = row.progress as ScriptProgress;
    const saved = process.env.SEMIO_SCRIPT_PROGRESS, names = contract.const.sourceAdmission.systemVariables, languages = Object.fromEntries(names.map(name => [name, process.env[name]]));
    for (const name of names) delete process.env[name];
    process.env.LANG = "en";
    try {
      for (const [choice, machine] of [["human", false], ["machine", true]] as const) {
        process.env.SEMIO_SCRIPT_PROGRESS = choice;
        const writes: string[] = [], spy = spyOn(process.stderr, "write").mockImplementation(((text: unknown) => { writes.push(String(text)); return true; }) as typeof process.stderr.write);
        try {
          await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "process-progress-fixture", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async invocation => { await invocation.control.publish(progress); await invocation.control.yieldContinuation(); });
        } finally { spy.mockRestore(); }
        expect(writes, choice).toEqual([...(machine ? [`[script] ${JSON.stringify(progress)}\n`] : []), `[script] ${row.expected}\n`]);
      }
    } finally {
      if (saved === undefined) delete process.env.SEMIO_SCRIPT_PROGRESS; else process.env.SEMIO_SCRIPT_PROGRESS = saved;
      for (const [name, value] of Object.entries(languages)) if (value === undefined) delete process.env[name]; else process.env[name] = value;
    }
  } finally { database.close(); }
});
'''
def ptest(t): return t.rstrip("\n") + "\n" + TEST
VERIFY = r'''PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress"
bun test ./🟦️.ts
echo "rows batch: GREEN"
'''
DOC = "🧾 Staged batch `rows` (lane progress, land after `lang`): the typed machine rows `[script] {json}` go to pipes and machine readers only; a person at a terminal reads the sentences."
if __name__ == "__main__":
    build(sys.argv[2] if len(sys.argv) > 2 else "rows", ROOT, [(SCHEMA, schema, after("lang", SCHEMA)), (PROC, proc, after("lang", PROC)), (PTEST, ptest, after("lang", PTEST))], VERIFY, DOC)
