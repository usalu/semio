"""🌐 Rebuilds staged batch `lang` from the CURRENT live files: [script] human lines honour SEMIO_LANGUAGE and the long-activity fallback exists in German, an interrupt says at once what happens, and the native heartbeat is a declared EN/DE sentence."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stagelib import build, sub
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..", ".."))
P = "🧰️framework/🔨️modules/🏃️process/"
PROC_DIR = P + "🧭️routing/📥️invocation/🏃️process/"
SCHEMA = PROC_DIR + "🧬️schema/📈️progress.json"
PROC = PROC_DIR + "🟦️.ts"
PTEST = PROC_DIR + "🧪️tests/📈️progress/🟦️.ts"
WD = PROC_DIR + "🐕️watchdog/🟦️.ts"
WDS = PROC_DIR + "🐕️watchdog/🧬️schema/🔣️.json"
WDT = PROC_DIR + "🐕️watchdog/🧪️tests/🟦️.ts"
OWN_DIR = P + "🎛️owned-execution/"
OWN = OWN_DIR + "🟦️.ts"
HEART = OWN_DIR + "🧬️schema/📣️heartbeat.json"
OWNT = OWN_DIR + "🧪️tests/🟦️.ts"
OWNF = OWN_DIR + "🧪️tests/🧫️fixtures/🔣️.json"
OWNP = OWN_DIR + "🧪️tests/📤️progress/🟦️.ts"
NATIVE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts"
CAPTURE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏃️contract-capture/🟦️.ts"

LONG = '''    "longActivity": {
      "labels": {
        "en": {
          "template": "{command} remains active on {resource} ({mode}, {seconds} s)",
          "modes": {
            "shared": "shared",
            "exclusive": "exclusive"
          }
        },
        "de": {
          "template": "{command} bleibt auf {resource} aktiv ({mode}, {seconds} s)",
          "modes": {
            "shared": "geteilt",
            "exclusive": "exklusiv"
          }
        }
      }
    },
'''
def schema(t):
    t = sub(t, '    "template": "{command} remains active on {resource} ({mode}, {seconds} s)",\n', LONG)
    return sub(t, '      "systemVariables": [\n        "SEMIO_DASHBOARD_LOCALE",\n        "LC_ALL",\n        "LC_MESSAGES",\n        "LANG"\n      ],\n      "fallback": [', '      "systemVariables": [\n        "SEMIO_LANGUAGE",\n        "SEMIO_DASHBOARD_LOCALE",\n        "LC_ALL",\n        "LC_MESSAGES",\n        "LANG"\n      ],\n      "fallback": [')
def proc(t):
    return sub(t, ' return `${progress.command} remains active on ${activity.resource} (${activity.mode}, ${Math.round(activity.elapsedMs/1000)} s)`;\n',
        ' const long=progressSchema.const.longActivity.labels[progressLanguage(environment)],longFields:Record<string,string>={command:progress.command,resource:activity.resource,mode:long.modes[activity.mode],seconds:String(Math.round(activity.elapsedMs/1000))};\n return long.template.replace(/\\{([A-Za-z]+)\\}/gu,(_token,key:string)=>longFields[key]!);\n')
def ptest(t):
    t = sub(t, '''  expect(new Ajv({strict:true}).compile(contract)(contract.const)).toBe(true);
  expect(compileJsonSchemaSubset(contract).validate(contract.const)).toEqual([]);
  try {
    for (const row of fixture.cases) {''', '''  expect(new Ajv({strict:true}).compile(contract)(contract.const)).toBe(true);
  expect(compileJsonSchemaSubset(contract).validate(contract.const)).toEqual([]);
  const names = contract.const.sourceAdmission.systemVariables, saved = Object.fromEntries(names.map(name => [name, process.env[name]]));
  for (const name of names) delete process.env[name];
  process.env.LANG = "en";
  try {
    for (const row of fixture.cases) {''')
    t = sub(t, '      expect(formatScriptProcessActivityV1(progress)).toBe(row.expected);\n', '''      expect(formatScriptProcessActivityV1(progress, { LANG: "en" })).toBe(row.expected);
      const german = database.query("select case when ?1 is null or ?1<?2 then null else replace(replace(replace(replace(json_extract(?3,'$.template'),'{command}',?4),'{resource}',?5),'{mode}',json_extract(?3,'$.modes.'||?6)),'{seconds}',cast(round(?1/1000.0) as integer)) end message").get(activity?.elapsedMs ?? null, contract.const.minimumElapsedMilliseconds, JSON.stringify(contract.const.longActivity.labels.de), progress.command, activity?.resource ?? null, activity?.mode ?? null) as {message:string|null};
      expect(formatScriptProcessActivityV1(progress, { SEMIO_LANGUAGE: "de" })).toBe(german.message);
      expect(formatScriptProcessActivityV1(progress, { SEMIO_LANGUAGE: "de", LANG: "en" })).toBe(german.message);
      if (row.expected !== null) expect(german.message).not.toContain("remains active");
''')
    return sub(t, '''  } finally { database.close(); }
});

test("provider presentation follows''', '''  } finally { database.close(); for (const [name, value] of Object.entries(saved)) if (value === undefined) delete process.env[name]; else process.env[name] = value; }
});

test("provider presentation follows''')
def wds(t):
    return sub(t, '''    "terminating": {''', '''    "interrupted": {
      "en": "{signal} received: the running work is cancelled and its leases are released; this process exits after {seconds} s at the latest.",
      "de": "{signal} empfangen: die laufende Arbeit wird abgebrochen und ihre Leases werden freigegeben; dieser Prozess endet spätestens nach {seconds} s."
    },
    "terminating": {''') if '    "terminating": {' in t else (_ for _ in ()).throw(SystemExit("watchdog schema layout changed"))
def wd(t):
    return sub(t, '  armed=true;const timer=setTimeout(end,grace);timer.unref();\n', '  armed=true;try{options.notice(scriptProcessWatchdogNoticeV1("interrupted",{signal,seconds:Math.round(grace/1000)},options.languages()));}catch{}const timer=setTimeout(end,grace);timer.unref();\n')
def wdt(t):
    t = sub(t, 'guard("SIGTERM",143);expect(exits).toEqual([]);await new Promise(done=>setTimeout(done,80));expect(exits).toEqual([143]);expect(notices[0]!.split("\\n").filter(Boolean).length).toBe(2);',
        'guard("SIGTERM",143);expect(exits).toEqual([]);expect(notices).toHaveLength(1);expect(notices[0]).toContain("SIGTERM received");expect(notices[0]).toContain("SIGTERM empfangen");expect(notices[0]!.split("\\n").filter(Boolean).length).toBe(2);await new Promise(done=>setTimeout(done,80));expect(exits).toEqual([143]);expect(notices).toHaveLength(2);expect(notices[1]).toContain("did not end this process");expect(notices[1]!.split("\\n").filter(Boolean).length).toBe(2);')
    return sub(t, 'second("SIGINT",130);second("SIGINT",130);expect(exits).toEqual([143,130]);', 'second("SIGINT",130);second("SIGINT",130);expect(exits).toEqual([143,130]);const silent:string[]=[],third=createScriptTerminationGuardV1({exit:()=>{},notice:text=>{silent.push(text);},languages:()=>["de"],graceMilliseconds:60000});third("SIGINT",130);expect(silent).toHaveLength(1);expect(silent[0]).toContain("SIGINT empfangen");expect(silent[0]).toContain("spätestens nach 60 s");expect(silent[0]).not.toContain("received");')
HEARTBEAT = {
  "$schema": "http://json-schema.org/draft-07/schema#",
  "const": {
    "version": 1,
    "purpose": "native-command-heartbeat",
    "languages": ["en", "de"],
    "systemVariables": ["SEMIO_LANGUAGE", "LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"],
    "labels": {
      "en": {"running": "[{label}] still running after {seconds} s"},
      "de": {"running": "[{label}] läuft seit {seconds} s"}
    }
  }
}
def own(t):
    t = sub(t, 'import outputSchema from "./🧬️schema/📬️output.json" with { type: "json" };\n', 'import outputSchema from "./🧬️schema/📬️output.json" with { type: "json" };\nimport heartbeat from "./🧬️schema/📣️heartbeat.json" with { type: "json" };\n')
    return sub(t, '''export function startNativeProgress(label: string, intervalMs = 10_000, output: (line: string) => void = (line) => { process.stderr.write(line + "\\n"); }): () => void {
  const started = Date.now(),
    progress = setInterval(() => output(`[${label}] running elapsedMs=${Date.now() - started}`), intervalMs);
  return () => clearInterval(progress);
}''', '''export function startNativeProgress(label: string, intervalMs = 10_000, output: (line: string) => void = (line) => { process.stderr.write(line + "\\n"); }, environment: Readonly<Record<string, string | undefined>> = process.env): () => void {
  const started = Date.now(), languages = nativeProgressLanguagesV1(environment),
    progress = setInterval(() => { const seconds = String(Math.round((Date.now() - started) / 1000)); for (const language of languages) output(heartbeat.const.labels[language].running.replace("{label}", () => label).replace("{seconds}", seconds)); }, intervalMs);
  return () => clearInterval(progress);
}

/** 🌐️ The languages a heartbeat is written in: the one the environment chooses, English and German when it chooses none. */
export function nativeProgressLanguagesV1(environment: Readonly<Record<string, string | undefined>>): readonly ("en" | "de")[] {
  for (const name of heartbeat.const.systemVariables) {
    const tag = environment[name]?.trim().toLowerCase().split(/[._@:-]/u)[0];
    if (tag === "en" || tag === "de") return [tag];
  }
  return heartbeat.const.languages as readonly ("en" | "de")[];
}''')
WORDS = "(?:still running after|läuft seit)"
def ownt(t):
    t = sub(t, 'const heartbeat = /\\[heartbeat\\] running elapsedMs=\\d+/u,', 'const heartbeat = /\\[heartbeat\\] (?:still running after|läuft seit) \\d+ s/u,')
    t = sub(t, 'import Ajv from "ajv";\n', 'import Ajv from "ajv";\nimport { Database } from "bun:sqlite";\nimport heartbeatSchema from "../🧬️schema/📣️heartbeat.json";\n')
    return sub(t, '''test("the progress heartbeat follows the stdout destination''', '''test("the progress heartbeat is declared in English and German, follows the chosen language and names both when none is chosen, checked against SQLite", async () => {
  const { startNativeProgress } = await import("../🟦️.ts"), database = new Database(":memory:");
  try {
    const cases: [Record<string, string>, string[]][] = [[{ SEMIO_LANGUAGE: "de" }, ["de"]], [{ SEMIO_LANGUAGE: "en", LANG: "de_DE.UTF-8" }, ["en"]], [{ LANG: "de_CH.UTF-8" }, ["de"]], [{ LC_ALL: "C", LANG: "en_US.UTF-8" }, ["en"]], [{}, ["en", "de"]], [{ LANG: "fr_FR.UTF-8" }, ["en", "de"]]];
    for (const [environment, languages] of cases) {
      const lines: string[] = [], stop = startNativeProgress("x:y $& z", 1, line => lines.push(line), environment);
      await Bun.sleep(40); stop();
      expect(lines.length, JSON.stringify(environment)).toBeGreaterThanOrEqual(languages.length);
      const seen = lines.slice(0, languages.length).map(line => languages.find(language => (database.query("select replace(replace(json_extract(?1,'$.'||?2||'.running'),'{label}',?3),'{seconds}',?4) text").get(JSON.stringify(heartbeatSchema.const.labels), language, "x:y $& z", /(\\d+) s$/u.exec(line)?.[1] ?? "") as { text: string }).text === line));
      expect(seen, JSON.stringify(environment)).toEqual(languages);
      for (const line of lines) expect(line).toMatch(/^\\[x:y \\$& z\\] (?:still running after|läuft seit) \\d+ s$/u);
      const count = lines.length; await Bun.sleep(20); expect(lines.length).toBe(count);
    }
    expect(heartbeatSchema.const.labels.en.running).not.toContain("elapsedMs");
    expect(heartbeatSchema.const.labels.de.running).not.toContain("elapsedMs");
  } finally { database.close(); }
});

test("the progress heartbeat follows the stdout destination''')
def ownf(t):
    return sub(t, '"pattern": "^\\\\[artifact-rust:fixture:build\\\\] running elapsedMs=[0-9]+$"', '"pattern": "^\\\\[artifact-rust:fixture:build\\\\] (?:still running after|läuft seit) [0-9]+ s$"')
def ownp(t):
    return sub(t, 'expect(stderr).toContain(`[${fixture.label}] running elapsedMs=`);', 'expect(stderr).toMatch(new RegExp(`\\\\[${fixture.label}\\\\] (?:still running after|läuft seit) \\\\d+ s`,"u"));')
def native(t):
    return sub(t, 'expect(result.stderr.includes("[native:owner-command] running elapsedMs="),row.id).toBe(row.wrapperProgress);', 'expect(/\\[native:owner-command\\] (?:still running after|läuft seit) \\d+ s/u.test(result.stderr),row.id).toBe(row.wrapperProgress);')
def capture(t):
    t = sub(t, 'import { runtimeSpawnSyncV1 } from', 'import { startNativeProgress } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";\nimport { runtimeSpawnSyncV1 } from')
    t = sub(t, '  const started = Date.now();\n  const progress = setInterval(() => console.log(`[artifact-package-contract] ${command} running elapsedMs=${Date.now() - started}`), 10_000);\n', '  const stopProgress = startNativeProgress(`artifact-package-contract:${command}`, 10_000, line => console.log(line));\n')
    return sub(t, '    clearInterval(progress);\n', '    stopProgress();\n')
VERIFY = r'''PY=python3
"$PY" -I "$HERE/stage.py" check --root "$ROOT" || true
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT"
P="🧰️framework/🔨️modules/🏃️process"
for t in "$P/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts" "$P/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts" "$P/🎛️owned-execution/🧪️tests/🟦️.ts" "$P/🎛️owned-execution/🧪️tests/📤️progress/🟦️.ts"; do
  (cd "$(dirname "$t")" && bun test "./$(basename "$t")") || exit 1
done
echo "lang batch: GREEN"
'''
DOC = "🌐 Staged batch `lang` (lane progress): the `[script]` human lines honour SEMIO_LANGUAGE, the long-activity fallback exists in German, an interrupt says at once what happens, and the native heartbeat is a declared EN/DE sentence."
if __name__ == "__main__":
    edits = [(SCHEMA, schema), (PROC, proc), (PTEST, ptest), (WDS, wds), (WD, wd), (WDT, wdt), (HEART, (json.dumps(HEARTBEAT, ensure_ascii=False, indent=2) + "\n").encode()),
             (OWN, own), (OWNT, ownt), (OWNF, ownf), (OWNP, ownp), (NATIVE, native), (CAPTURE, capture)]
    build(sys.argv[2] if len(sys.argv) > 2 else "lang", ROOT, edits, VERIFY, DOC)
