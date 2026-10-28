"""🛑 Rebuilds staged batch `sigint` (land after `lang` and `rows`): an interrupted or terminated script process cancels its work, releases its leases and ends with exit status 130/143 and one short EN/DE notice, without Bun's raw uncaught-error dump."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stagelib import build, sub, after_all
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
D = "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/"
PROC, WDS, WDT = D + "🟦️.ts", D + "🐕️watchdog/🧬️schema/🔣️.json", D + "🐕️watchdog/🧪️tests/🟦️.ts"
PRE = ["lang", "rows"]
def proc(t):
    t = sub(t, ' const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds', ' let ended:{signal:string;code:number}|undefined;const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds')
    t = sub(t, 'interrupt=()=>{controller.abort(Error("Original Script process interrupted"));endProcess("SIGINT",130);}', 'interrupt=()=>{ended??={signal:"SIGINT",code:130};controller.abort(Error("Original Script process interrupted"));endProcess("SIGINT",130);}')
    t = sub(t, 'terminate=()=>{controller.abort(Error("Original Script process terminated"));endProcess("SIGTERM",143);}', 'terminate=()=>{ended??={signal:"SIGTERM",code:143};controller.abort(Error("Original Script process terminated"));endProcess("SIGTERM",143);}')
    return sub(t, 'error=>{try{presentation.flush();}catch{}throw error;}).finally(', 'error=>{try{presentation.flush();}catch{}if(ended!==undefined){try{process.stderr.write(scriptProcessWatchdogNoticeV1("cancelled",{signal:ended.signal},firstFailureLanguagesV1(process.env)));}catch{}process.exit(ended.code);}throw error;}).finally(')
def wds(t):
    return sub(t, '    "terminating": {', '''    "cancelled": {
      "en": "Cancelled ({signal}); leases released.",
      "de": "Abgebrochen ({signal}); Leases freigegeben."
    },
    "terminating": {''')
TEST = r'''
test("an interrupt or a termination ends the process with the conventional exit status and one short notice in the chosen language or both, with no raw error dump, checked against a Python subprocess oracle", async () => {
  const module = resolve(import.meta.dir, "../../🟦️.ts"), script = `const {createScriptProcessEnvelope,withScriptProcessEnvelope}=await import(${JSON.stringify(module)});await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"interrupt-fixture",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{process.stdout.write("ready\\n");await new Promise((_accept,fail)=>invocation.control.signal.addEventListener("abort",()=>fail(invocation.control.signal.reason)));});`;
  const clean = Object.fromEntries(Object.entries(process.env).filter(([name]) => !/^(SEMIO_LANGUAGE|LANGUAGE|LC_[A-Z]+|LANG)$/u.test(name)));
  const python = "import subprocess,sys,time,signal\np=subprocess.Popen([sys.argv[1],'--eval',sys.argv[2]],stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=__import__('json').loads(sys.argv[4]))\np.stdout.readline()\np.send_signal(getattr(signal,sys.argv[3]))\nerr=p.stderr.read().decode()\nprint(__import__('json').dumps({'code':p.wait(),'err':err}))";
  const cases: [string, NodeJS.Signals, Record<string, string>, number, string[], string[]][] = [
    ["de", "SIGINT", { SEMIO_LANGUAGE: "de" }, 130, ["Abgebrochen (SIGINT); Leases freigegeben."], ["Cancelled"]],
    ["en", "SIGINT", { SEMIO_LANGUAGE: "en", LANG: "de_DE.UTF-8" }, 130, ["Cancelled (SIGINT); leases released."], ["Abgebrochen"]],
    ["both", "SIGTERM", { LC_ALL: "C" }, 143, ["Cancelled (SIGTERM); leases released.", "Abgebrochen (SIGTERM); Leases freigegeben."], []]
  ];
  for (const [name, signal, extra, code, contains, absent] of cases) {
    const environment = { ...clean, ...extra };
    const oracle = JSON.parse(Bun.spawnSync(["python3", "-I", "-c", python, process.execPath, script, signal, JSON.stringify(environment)], { stdout: "pipe", stderr: "pipe" }).stdout.toString()) as { code: number; err: string };
    const child = Bun.spawn([process.execPath, "--eval", script], { env: environment, stdout: "pipe", stderr: "pipe" });
    await child.stdout.getReader().read();
    child.kill(signal);
    const [exitCode, err] = await Promise.all([child.exited, new Response(child.stderr).text()]);
    expect(oracle.code, name).toBe(code);
    expect(exitCode, name).toBe(code);
    for (const text of contains) { expect(err, name).toContain(text); expect(oracle.err, name).toContain(text); }
    for (const text of absent) expect(err, name).not.toContain(text);
    expect(err, name).not.toMatch(/\berror:|\bat interrupt\b|Bun v\d|^\s*\d+ \|/mu);
    expect(err.split("\n").filter(line => line.trim() !== "").length, name).toBeLessThanOrEqual(2 * contains.length);
  }
}, 60_000);
'''
def wdt(t):
    t = t.rstrip("\n") + "\n" + TEST
    return sub(t, 'import Ajv from"ajv";\n', 'import Ajv from"ajv";\nimport{resolve}from"node:path";\n')
VERIFY = r'''PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests"
bun test --timeout 60000 ./🟦️.ts
echo "sigint batch: GREEN (the two ps-based tests fail on base too)"
'''
DOC = "🛑 Staged batch `sigint` (lane progress, land after lang and rows): an interrupted or terminated script process ends with exit status 130/143 and one short EN/DE notice, without Bun's raw uncaught-error dump."
if __name__ == "__main__":
    build(sys.argv[2] if len(sys.argv) > 2 else "sigint", ROOT, [(PROC, proc, after_all(PRE, PROC)), (WDS, wds, after_all(PRE, WDS)), (WDT, wdt, after_all(PRE, WDT))], VERIFY, DOC)
