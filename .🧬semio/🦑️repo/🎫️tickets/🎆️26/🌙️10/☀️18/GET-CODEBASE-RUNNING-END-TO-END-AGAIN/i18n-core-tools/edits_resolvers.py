#!/usr/bin/env python3
"""Step 1 of the i18n-core build: one `resolveLanguageV1` everywhere. Deletes the local resolvers D1-D6 and every `?? "en"` / `[0]` language fallback."""
import re
from lib import *

LANGUAGE_FILE = LANG + "/🟦️.ts"
SHARED = "🏗️build/🗣️language/🟦️.ts"
CATALOG = "🏗️build/🗣️language/📖️catalog/🟦️.ts"


def resolver_import(tree, rel, process_prefix="📦️packages/🟦️typescript/"):
    """The two imports every migrated file gets: the shared resolver and the offered languages."""
    a = import_path(rel, process_prefix + SHARED)
    b = import_path(rel, process_prefix + CATALOG)
    return 'import {resolveLanguageV1} from "%s";\nimport {CATALOG_LANGUAGES_V1} from "%s";\n' % (a, b)


def add_after_imports(tree, rel, line):
    """Adds `line` right after the last top-level import line."""
    text = tree.text(rel)
    last = 0
    for match in re.finditer(r"^import[^\n]*\n", text, re.M):
        last = match.end()
    tree.put(rel, text[:last] + line + text[last:])


def typed_resolver(tree):
    f = LANGUAGE_FILE
    tree.edit(f, 'export type LanguageResolutionV1 = Readonly<{ language: string; source: string }>;', 'export type LanguageResolutionV1<T extends string = string> = Readonly<{ language: T; source: string }>;')
    tree.edit(f, 'export function resolveLanguageV1(languages: readonly string[], args: readonly string[], environment: LanguageEnvironmentV1, osLanguages: OsLanguageListV1 = osLanguageListV1): LanguageResolutionV1 {\n',
              'export function resolveLanguageV1<T extends string>(languages: readonly T[], args: readonly string[], environment: LanguageEnvironmentV1, osLanguages: OsLanguageListV1 = osLanguageListV1): LanguageResolutionV1<T> {\n  const offered: readonly string[] = languages;\n')
    tree.edit_regex(f, r"(?<![A-Za-z.])languages\.includes\(", "offered.includes(", count=5)
    tree.edit(f, 'return { language: flag, source: "--lang" };', 'return { language: flag as T, source: "--lang" };')
    tree.edit(f, 'return { language: tag, source: protocol.explicitVariable };', 'return { language: tag as T, source: protocol.explicitVariable };')
    tree.edit(f, 'return { language: tag, source: protocol.preferenceListVariable };', 'return { language: tag as T, source: protocol.preferenceListVariable };')
    tree.edit(f, 'return { language: tag, source: name };', 'return { language: tag as T, source: name };')
    tree.edit(f, 'return { language: tag, source: "os" };', 'return { language: tag as T, source: "os" };')


def d1_unit_cache(tree):
    launch = L + "/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts"
    tree.edit(launch, '''/** 🌐️ The language of operator messages: `SEMIO_LANGUAGE`, then the locale, English when neither names German. */
export function laneLanguageV1(environment:Readonly<Record<string,string|undefined>>):"en"|"de"{
 const value=(environment.SEMIO_LANGUAGE??environment.LC_ALL??environment.LC_MESSAGES??environment.LANG??"").toLowerCase();
 return value.startsWith("de")?"de":"en";
}

''', '')
    launcher = L + "/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
    tree.edit(launcher, "import {focusFromCargoArgumentsV1,laneLanguageV1,laneMessageV1,", "import {focusFromCargoArgumentsV1,laneMessageV1,")
    tree.edit(launcher, "const root=options.storage.backing.root,language=laneLanguageV1(options.environment);", "const root=options.storage.backing.root;")
    tree.edit(launcher, 'laneMessageV1("overflow",{base,lane:lane.paths.id},language)', 'laneMessageV1("overflow",{base,lane:lane.paths.id},resolveLanguageV1(CATALOG_LANGUAGES_V1,[],options.environment).language)')
    tree.edit(launcher, 'laneMessageV1("collected",applied,language)', 'laneMessageV1("collected",applied,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],options.environment).language)')
    add_after_imports(tree, launcher, resolver_import(tree, launcher))
    test = L + "/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts"
    tree.edit(test, "import {ensureUnitCacheShimV1,focusFromCargoArgumentsV1,laneLanguageV1,laneMessageV1,", "import {ensureUnitCacheShimV1,focusFromCargoArgumentsV1,laneMessageV1,")
    tree.edit(test, ' expect(laneLanguageV1({SEMIO_LANGUAGE:"de"})).toBe("de");expect(laneLanguageV1({LANG:"de_DE.UTF-8"})).toBe("de");expect(laneLanguageV1({LANG:"en_US.UTF-8"})).toBe("en");expect(laneLanguageV1({})).toBe("en");\n',
              ' const pick=(environment:Record<string,string>)=>resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment,()=>[]).language;expect(pick({SEMIO_LANGUAGE:"de"})).toBe("de");expect(pick({LANG:"de_DE.UTF-8"})).toBe("de");expect(pick({LANG:"en_US.UTF-8"})).toBe("en");expect(()=>pick({})).toThrow("--lang=en|de");\n')
    add_after_imports(tree, test, resolver_import(tree, test))


def d2_ledger(tree):
    ledger = L + "/⚡️caching/💾️storage/🧾️ledger/🟦️.ts"
    text = tree.text(ledger)
    start = text.index("/** 🗣️ Picks the output language: an explicit valid request")
    end = text.index("/** 💬️ Renders one catalogue message with its placeholders. */")
    tree.put(ledger, text[:start] + text[end:])
    command = L + "/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
    tree.edit(command, "formatCacheReportV1,resolveCacheLanguageV1,type CacheLanguageV1", "formatCacheReportV1,type CacheLanguageV1")
    tree.edit(command, "language:CacheLanguageV1=resolveCacheLanguageV1(parsed.language,context.environment);", "language:CacheLanguageV1=resolveLanguageV1(CATALOG_LANGUAGES_V1,args,context.environment).language;")
    add_after_imports(tree, command, resolver_import(tree, command))
    test = L + "/⚡️caching/💾️storage/🧾️ledger/🧪️tests/🟦️.ts"
    tree.edit(test, "requireDeclaredCacheSegmentsV1,resolveCacheLanguageV1,type CacheLanguageV1", "requireDeclaredCacheSegmentsV1,type CacheLanguageV1")
    text = tree.text(test)
    a = text.index('test("the language is the explicit request, then the environment, then the first catalogue language"')
    b = text.index('test("the status report renders the same in both languages from the same data"')
    tree.put(test, text[:a] + '''test("the language is the flag, then the environment, and nothing is assumed",()=>{
 const pick=(args:string[],environment:Record<string,string>)=>resolveLanguageV1(CATALOG_LANGUAGES_V1,args,environment,()=>[]).language;
 expect(pick(["--lang","de"],{LANG:"en_US.UTF-8"})).toBe("de");
 expect(pick([],{SEMIO_LANGUAGE:"de",LANG:"en_US.UTF-8"})).toBe("de");
 expect(pick([],{LC_ALL:"de_DE.UTF-8"})).toBe("de");
 expect(pick([],{LANG:"en_GB.UTF-8"})).toBe("en");
 expect(()=>pick([],{LANG:"fr_FR.UTF-8"})).toThrow("--lang=en|de");
 expect(()=>pick([],{})).toThrow("SEMIO_LANGUAGE=en|de");
 expect(()=>pick(["--lang","fr"],{})).toThrow("Unknown language: fr (en|de)");
});

''' + text[b:])
    add_after_imports(tree, test, resolver_import(tree, test))
    ctest = L + "/🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts"
    text = tree.text(ctest)
    n = text.count("environment:{},write:")
    text = text.replace("environment:{},write:", 'environment:{SEMIO_LANGUAGE:"en"},write:')
    tree.put(ctest, text)
    return n


def d3_concurrent(tree):
    D = L + "/⚡️caching/🚀️bootstrap/📦️dependencies"
    core = D + "/🚦️concurrent/🟦️.ts"
    text = tree.text(core)
    start = text.index("/** 🗣️ Picks the message language like the help does")
    end = text.index("/** 🧮️ Is the admission verdict for building one more generation")
    tree.put(core, text[:start] + text[end:])
    store = D + "/💾️storage/🟦️.ts"
    tree.edit(store, "concurrentEventV1,concurrentLanguageV1,concurrentLeaseResourceV1,", "concurrentEventV1,concurrentLeaseResourceV1,")
    tree.edit(store, "control:ResourceLeaseControl,language:string=concurrentLanguageV1(process.env)):(event:ConcurrentEventV1)=>Promise<void>{", "control:ResourceLeaseControl,language?:string):(event:ConcurrentEventV1)=>Promise<void>{")
    tree.edit(store, "concurrentTextV1(language,name,{seconds:", "concurrentTextV1(language??resolveLanguageV1(concurrentProtocolV1.languages,[],process.env).language,name,{seconds:")
    tree.edit_regex(store, r"concurrentTextV1\(concurrentLanguageV1\(process\.env\),", "concurrentTextV1(resolveLanguageV1(concurrentProtocolV1.languages,[],process.env).language,", count=2)
    add_after_imports(tree, store, 'import {resolveLanguageV1} from "%s";\n' % import_path(store, "📦️packages/🟦️typescript/" + SHARED))
    tree.edit_regex(store, r"language=concurrentLanguageV1\(process\.env\)", "language=resolveLanguageV1(concurrentProtocolV1.languages,[],process.env).language")
    generation = D + "/🧾️generation/💾️storage/🟦️.ts"
    tree.edit(generation, 'import {concurrentLanguageV1} from "../../🚦️concurrent/🟦️.ts";', 'import {concurrentProtocolV1} from "../../🚦️concurrent/🟦️.ts";')
    tree.edit(generation, "language:string=concurrentLanguageV1(process.env)", "language:string=resolveLanguageV1(concurrentProtocolV1.languages,[],process.env).language")
    add_after_imports(tree, generation, 'import {resolveLanguageV1} from "%s";\n' % import_path(generation, "📦️packages/🟦️typescript/" + SHARED))
    reader = D + "/🚦️concurrent/💾️storage/🟦️.ts"
    tree.edit(reader, "import {concurrentEventV1,concurrentLanguageV1,concurrentLeaseResourceV1,", "import {concurrentEventV1,concurrentLeaseResourceV1,")
    tree.edit(reader, "concurrentTextV1(concurrentLanguageV1(process.env),", "concurrentTextV1(resolveLanguageV1(protocol.languages,[],process.env).language,")
    add_after_imports(tree, reader, 'import {resolveLanguageV1} from "%s";\n' % import_path(reader, "📦️packages/🟦️typescript/" + SHARED))
    test = D + "/🚦️concurrent/🧪️tests/🟦️.ts"
    tree.edit(test, ''' const expectedLanguage=oracle("language",fixture.language);
 for(const [index,row] of fixture.language.entries())expect(core.concurrentLanguageV1(row.environment),row.name).toBe(expectedLanguage[index].language);
''', '')
    tree.edit(test, """expect(admit).toContain('concurrentTextV1(concurrentLanguageV1(process.env),"admissionRefused"');""", """expect(admit).toContain('concurrentTextV1(resolveLanguageV1(concurrentProtocolV1.languages,[],process.env).language,"admissionRefused"');""")
    oracle = D + "/🚦️concurrent/🧪️tests/🐍️oracle.py"
    text = tree.text(oracle)
    a = text.index("def language(cases):")
    b = text.index('if __name__ == "__main__":')
    text = text[:a] + text[b:]
    text = text.replace('"wait": wait, "language": language, "admission"', '"wait": wait, "admission"')
    text = text.replace("pointer evolution, retention decisions, wait scheduling and message language.", "pointer evolution, retention decisions and wait scheduling.")
    text = text.replace("<pointer|collect|wait|language|admission|budget|stale>", "<pointer|collect|wait|admission|budget|stale>")
    tree.put(oracle, text)
    edit_json(tree, D + "/🚦️concurrent/🧪️tests/🔣️.json", lambda d: d.pop("language"))
    return text


def d4_bazel(tree):
    B = L + "/🏗️build/🔧️tools/🏗️bazel"
    sessions = B + "/🧵️sessions/🟦️.ts"
    tree.edit(sessions, '''/** 🌐️ Chooses the language of progress lines: an explicit `SEMIO_LANGUAGE`, else the locale, else English. */
export function bazelSessionLanguageV1(environment:Readonly<Record<string,string|undefined>>):BazelLanguageV1{
 const value=(environment.SEMIO_LANGUAGE??environment.LC_ALL??environment.LC_MESSAGES??environment.LANG??"").toLowerCase();return value.startsWith("de")?"de":"en";
}
''', '')
    tree.edit(sessions, "timeoutMilliseconds?:number;language?:BazelLanguageV1;preference?", "timeoutMilliseconds?:number;language:BazelLanguageV1;preference?")
    tree.edit(sessions, "language=request.language??\"en\",", "language=request.language,")
    script = B + "/📜️script.ts"
    tree.edit(script, "bazelSessionCommandV1,bazelSessionLanguageV1,bazelSessionResourcesV1", "bazelSessionCommandV1,bazelSessionResourcesV1")
    tree.edit(script, "language:bazelSessionLanguageV1(process.env),", "language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,")
    add_after_imports(tree, script, resolver_import(tree, script))
    test = B + "/🧵️sessions/🧪️tests/🟦️.ts"
    tree.edit(test, "bazelSessionCommandV1,bazelSessionLanguageV1,bazelSessionMessageV1,", "bazelSessionCommandV1,bazelSessionMessageV1,")
    tree.edit(test, ' for(const row of fixture.languages)expect(bazelSessionLanguageV1(row.environment) as string,JSON.stringify(row.environment)).toBe(row.expected);\n',
              ' for(const row of fixture.languages){const pick=()=>resolveLanguageV1(CATALOG_LANGUAGES_V1,[],row.environment,()=>[]).language as string;if(row.expected===null)expect(pick,JSON.stringify(row.environment)).toThrow("--lang=en|de");else expect(pick(),JSON.stringify(row.environment)).toBe(row.expected);}\n')
    add_after_imports(tree, test, resolver_import(tree, test))
    counts = {}
    for rel in (test, B + "/🧵️sessions/🧪️tests/🔬️concurrency/🟦️.ts", P + "/🪪️agent-sessions/🧪️tests/🔗️adapters/🟦️.ts"):
        total = 0
        text = tree.text(rel)
        for name in ("acquireBazelSessionV1", "withBazelSessionV1"):
            def fix(arg):
                s = arg.lstrip()
                if not s.startswith("{") or "language:" in s:
                    return arg
                lead = arg[:len(arg) - len(s)]
                return lead + '{language:"en",' + s[1:]
            text, n = rewrite_calls(text, name, 2, fix)
            total += n
        tree.put(rel, text)
        counts[rel[-40:]] = total
    edit_json(tree, B + "/🧵️sessions/🧪️tests/🧫️fixtures/🔣️.json", lambda d: [row.__setitem__("expected", None) for row in d["languages"] if row["environment"] in ({"LC_ALL": "C", "LANG": "de_DE.UTF-8"}, {})])
    schema = B + "/🧵️sessions/🧪️tests/🧫️fixtures/🧬️schema.json"
    tree.edit(schema, '"expected": {"enum": ["en", "de"]}}}},\n    "overrides"', '"expected": {"enum": ["en", "de", null]}}}},\n    "overrides"')
    return counts


def d5_generation(tree):
    G = L + "/🏗️build/🕸️graph/🕰️generations/🔌️load"
    mjs = G + "/🟨️.mjs"
    text = tree.text(mjs)
    start = text.index("/** 🌐️ Picks the language of the process like every other progress text")
    end = text.index("/** 🗣️ Renders one message in the language of the environment. */")
    tree.put(mjs, text[:start] + text[end:])
    tree.edit(mjs, "/** 🗣️ Renders one message in the language of the environment. */\nexport function renderGenerationMessageV1(key,fields,environment){\n const template=GENERATION_MESSAGES_V1[selectGenerationLanguageV1(environment)][key];",
              "/** 🗣️ Renders one message in the language the caller resolved with the shared resolver. */\nexport function renderGenerationMessageV1(key,fields,language){\n const template=GENERATION_MESSAGES_V1[language]?.[key];")
    d = G + "/🟨️.d.mts"
    tree.edit(d, 'export function selectGenerationLanguageV1(environment:Readonly<Record<string,string|undefined>>):"en"|"de";\n', '')
    tree.edit(d, "fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string;", 'fields:Readonly<Record<string,string|number>>,language:"en"|"de"):string;')
    graph = L + "/🏗️build/🕸️graph/🟦️.ts"
    tree.edit(graph, "renderGenerationMessageV1(key as never,fields,process.env)", "renderGenerationMessageV1(key as never,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)")
    add_after_imports(tree, graph, resolver_import(tree, graph))
    test = G + "/🧪️tests/🟦️.ts"
    tree.edit(test, ",selectGenerationLanguageV1,shapeComputeResultV1}", ",shapeComputeResultV1}")
    tree.edit(test, 'seconds:3},{})).toBe(', 'seconds:3},"en")).toBe(')
    tree.edit(test, 'seconds:3},{LANG:"de_DE.UTF-8"})).toBe(', 'seconds:3},"de")).toBe(')
    tree.edit(test, 'renderGenerationMessageV1("other" as never,{},{})', 'renderGenerationMessageV1("other" as never,{},"en")')
    tree.edit(test, '\nfor(const row of fixture.language)test("language "+JSON.stringify(row.env),()=>{expect(selectGenerationLanguageV1(row.env)).toBe(row.expected as "en"|"de");});\n', '\n')
    tree.edit(test, 'renderGenerationMessageV1("stale",{generation:2},{})', 'renderGenerationMessageV1("stale",{generation:2},"en")')
    tree.edit(test, 'renderGenerationMessageV1("stale",{generation:2},{LANG:"de_DE.UTF-8"})', 'renderGenerationMessageV1("stale",{generation:2},"de")')
    tree.edit_regex(test, r'createMockHost\(dir,\{(\.\.\.options,)?environment:\{LANG:"de_DE\.UTF-8"\}\}\)', lambda m: 'createMockHost(dir,{' + (m.group(1) or '') + 'language:"de"})')
    mjs_text = tree.text(mjs)
    for old, new in (("current=new Map(),environment})", "current=new Map(),language})"), ("{},environment)", "{},language)"), ("(key,fields,environment)", "(key,fields,language)"), ("{project,why},environment)", "{project,why},language)"), ("environment=host.environment??{};", "language=host.language;"), ("current,environment})", "current,language})")):
        if old in mjs_text:
            tree.edit(mjs, old, new)
    tree.edit_if_present(d, 'current?:ReadonlyMap<string,unknown>;environment:Readonly<Record<string,string|undefined>>}', 'current?:ReadonlyMap<string,unknown>;language:"en"|"de"}')
    tree.edit_if_present(d, '"say"|"workspaceRoot"|"separator"|"environment">', '"say"|"workspaceRoot"|"separator"|"language">')
    tree.edit_if_present(d, 'separator?:string;environment?:Readonly<Record<string,string|undefined>>;namedInputs', 'separator?:string;language:"en"|"de";namedInputs')
    tree.edit_if_present(graph, 'workspaceRoot:root,separator:sep,environment:process.env,namedInputs', 'workspaceRoot:root,separator:sep,language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,namedInputs')
    tree.edit_if_present(world, 'workspaceRoot:dir,namedInputs:{default:[]}', 'workspaceRoot:dir,language:options.language??"en",namedInputs:{default:[]}')
    world = L + "/🏗️build/🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts"
    tree.edit(world, "options:{signal?:AbortSignal;environment?:Record<string,string>;gate?", "options:{signal?:AbortSignal;language?:\"en\"|\"de\";gate?")
    tree.edit(world, "renderGenerationMessageV1(key as never,fields,options.environment??{})", "renderGenerationMessageV1(key as never,fields,options.language??\"en\")")
    edit_json(tree, G + "/🧪️tests/🔣️.json", lambda d: d.pop("language"))

    def shrink(document):
        document["required"].remove("language")
        del document["properties"]["language"]
    edit_json(tree, G + "/🧪️tests/🧬️schema/🔣️.json", shrink)


def d6_ci_report(tree):
    C = L + "/🏗️build/🧾️ci-report"
    core = C + "/🟦️.ts"
    text = tree.text(core)
    start = text.index("/** 🌍️ Picks the report language: the flag, then SEMIO_LANGUAGE")
    end = text.index("/** 💬️ Renders one message with `{name}` placeholders. */")
    tree.put(core, text[:start] + text[end:])
    script = C + "/📜️script.ts"
    tree.edit(script, "renderCiTestsV1,resolveCiLanguageV1,summarizeCiPlanV1", "renderCiTestsV1,summarizeCiPlanV1")
    tree.edit(script, 'const language=resolveCiLanguageV1(values.get("lang"),environment);', 'const flag=values.get("lang"),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,flag===undefined?[]:["--lang",flag],environment).language;')
    add_after_imports(tree, script, resolver_import(tree, script))
    test = C + "/🧪️tests/🟦️.ts"
    tree.edit(test, "renderCiTestsV1,resolveCiLanguageV1,summarizeCiJUnitV1", "renderCiTestsV1,summarizeCiJUnitV1")
    tree.edit(test, ''' for(const row of fixture.languages){
  if(row.expected===null)expect(()=>resolveCiLanguageV1(row.flag??undefined,row.environment),JSON.stringify(row)).toThrow("Language required");
  else expect(resolveCiLanguageV1(row.flag??undefined,row.environment),JSON.stringify(row)).toBe(row.expected as CiLanguageV1);
 }''', ''' for(const row of fixture.languages){
  const pick=()=>resolveLanguageV1(CATALOG_LANGUAGES_V1,row.flag===null?[]:["--lang",row.flag],row.environment,()=>[]).language;
  if(row.expected===null)expect(pick,JSON.stringify(row)).toThrow("en|de");
  else expect(pick(),JSON.stringify(row)).toBe(row.expected as CiLanguageV1);
 }''')
    tree.edit(test, 'expect(runCiReportV1(["tests","--dir",root],{},()=>{})).toBe(2);', 'expect(runCiReportV1(["tests","--dir",root],{SEMIO_LANGUAGE:"fr"},()=>{})).toBe(2);')
    add_after_imports(tree, test, resolver_import(tree, test))

    def fix(document):
        for row in document["languages"]:
            if row["flag"] == "fr" and row["environment"] == {"LANG": "de_DE"}:
                row["expected"] = None
    edit_json(tree, C + "/🧪️tests/🧫️fixtures/🔣️.json", fix)


def d7_agent_sessions(tree):
    messages = P + "/🪪️agent-sessions/🗣️messages/🟦️.ts"
    tree.edit(messages, '''/** 🌐️ Chooses the language: an explicit `SEMIO_LANGUAGE`, else the locale, else English. */
export function agentSessionLanguageV1(environment:Readonly<Record<string,string|undefined>>):AgentSessionLanguageV1{
 const value=(environment.SEMIO_LANGUAGE??environment.LC_ALL??environment.LC_MESSAGES??environment.LANG??"").toLowerCase();return value.startsWith("de")?"de":"en";
}
''', '')
    tree.edit(P + "/🪪️agent-sessions/🟦️.ts", "export {agentSessionLanguageV1,agentSessionMessageV1,", "export {agentSessionMessageV1,")
    capacity = L + "/🏗️build/🌡️host-capacity/⌨️command/🟦️.ts"
    tree.edit(capacity, "agentSessionCapacityRowsV1,agentSessionLanguageV1,agentSessionRegistryRootV1", "agentSessionCapacityRowsV1,agentSessionRegistryRootV1")
    tree.edit(capacity, "renderAgentSessionCapacityV1(sessions,agentSessionLanguageV1(context.environment))", "renderAgentSessionCapacityV1(sessions,language)")
    command = L + "/🏗️build/🪪️agent-sessions/⌨️command/🟦️.ts"
    tree.edit(command, "agentSessionEnvironmentV1,agentSessionLanguageV1,agentSessionMessageV1,", "agentSessionEnvironmentV1,agentSessionMessageV1,")
    tree.edit(command, "const language=agentSessionLanguageV1(context.environment),say=", "const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],context.environment).language,say=")
    add_after_imports(tree, command, resolver_import(tree, command))


def d9_resolution(tree):
    f = L + "/🏗️build/📥️inputs/🦀️resolution/🟦️.ts"
    tree.edit(f, "return (texts[language] ?? texts[policy.messages.languages[0]!] ?? \"\").replace(", "return (texts[language] as string).replace(")


def run(tree):
    typed_resolver(tree)
    d1_unit_cache(tree)
    d2_ledger(tree)
    d3_concurrent(tree)
    d4_bazel(tree)
    d5_generation(tree)
    d6_ci_report(tree)
    d7_agent_sessions(tree)
    d9_resolution(tree)


if __name__ == "__main__":
    import sys
    t = Tree(sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/i18n-core/r6")
    run(t)
    print("edited", len(t.changed()), "files")
    for rel in t.changed():
        print(" ", rel.replace(L + "/", "L/").replace(P + "/", "P/"))
