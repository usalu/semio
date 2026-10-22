#!/usr/bin/env python3
"""Increment 2 (round 7): the last assumed languages (leases diagnostics, toolchain, resident, remote execution and cache, doctor) go through the shared resolver; the assumed-language allowance reaches 0."""
import re
from lib import *

PKG = "📦️packages/🟦️typescript/🏗️build/🗣️language/"


def imports(tree, rel, names, catalog=False, process_relative=None):
    """Adds `import {names} from <language module>` (and the catalog languages) after the last import of rel."""
    target = process_relative if process_relative else PKG + "🟦️.ts"
    text = tree.text(rel)
    line = 'import {%s} from "%s";\n' % (names, rel_import(rel, P + "/" + target))
    if catalog:
        line += 'import {CATALOG_LANGUAGES_V1} from "%s";\n' % rel_import(rel, P + "/" + PKG + "📖️catalog/🟦️.ts")
    last = 0
    for m in re.finditer(r"^import[^\n]*\n", text, re.M):
        last = m.end()
    tree.put(rel, text[:last] + line + text[last:])


def doctor(tree):
    D = L + "/🏗️build/🩺️doctor"
    core = D + "/🟦️.ts"
    text = tree.text(core)
    a = text.index("/** 🗣️ Chooses the output languages: the request, then the environment")
    b = text.index("/** ✂️ Shortens a process command line")
    tree.put(core, text[:a] + text[b:])
    cmd = D + "/⌨️command/🟦️.ts"
    tree.edit(cmd, "doctorTextV1,resolveDoctorLanguagesV1,type DoctorAreaV1", "doctorTextV1,type DoctorAreaV1")
    tree.edit(cmd, "languages=resolveDoctorLanguagesV1(parsed.language,context.environment),controller", "languages=resolveLanguagesV1(CATALOG_LANGUAGES_V1,args,context.environment,context.osLanguages),controller")
    tree.edit(cmd, "export type DoctorCommandContextV1=Readonly<{root:string;environment:NodeJS.ProcessEnv;", "export type DoctorCommandContextV1=Readonly<{root:string;environment:NodeJS.ProcessEnv;osLanguages?:OsLanguageListV1;")
    tree.edit(cmd, "const line=quiet?undefined:progressLine(languages[0]!,stage);if(line)say(line);", "const line=quiet?undefined:languages.flatMap(language=>progressLine(language,stage)??[]).join(\"\\n\");if(line)say(line);")
    imports(tree, cmd, "resolveLanguagesV1,type OsLanguageListV1", catalog=True)
    dtest = D + "/🧪️tests/⌨️command/🟦️.ts"
    tree.edit(dtest, "ports:{now:()=>1000000000000},options:{probes},...extra}", "osLanguages:()=>[],ports:{now:()=>1000000000000},options:{probes},...extra}")
    test = D + "/🧪️tests/🟦️.ts"
    text = tree.text(test)
    text = re.sub(r"[^\n]*resolveDoctorLanguagesV1\([^\n]*\n", "", text)
    text = text.replace("resolveDoctorLanguagesV1,", "").replace(",resolveDoctorLanguagesV1", "")
    tree.put(test, text)


def remote_cache(tree):
    B = L + "/🏗️build"
    rc = B + "/🌐️remote-cache/⌨️command/🟦️.ts"
    text = tree.text(rc)
    a = text.index("/** 🗣️ Picks the output languages: the explicit request")
    b = text.index("function both(key:string")
    tree.put(rc, text[:a] + text[b:])
    tree.edit(rc, "languages=remoteCacheLanguagesV1(flags[\"--lang\"],context.environment)", "languages=resolveLanguagesV1(CATALOG_LANGUAGES_V1,flags[\"--lang\"]===undefined?[]:[\"--lang\",flags[\"--lang\"]],context.environment)")
    tree.edit(rc, "languages=remoteCacheLanguagesV1(parsed.values[\"--lang\"],context.environment)", "languages=resolveLanguagesV1(CATALOG_LANGUAGES_V1,parsed.values[\"--lang\"]===undefined?[]:[\"--lang\",parsed.values[\"--lang\"]],context.environment)")
    imports(tree, rc, "resolveLanguagesV1", catalog=True)
    ex = B + "/🌐️remote-execution/⌨️command/🟦️.ts"
    tree.edit(ex, "import {remoteCacheLanguagesV1,type RemoteCacheLanguageV1}", "import {type RemoteCacheLanguageV1}")
    n = tree.text(ex).count("languages=remoteCacheLanguagesV1(flags[\"--lang\"],context.environment)")
    tree.edit(ex, "languages=remoteCacheLanguagesV1(flags[\"--lang\"],context.environment)", "languages=resolveLanguagesV1(CATALOG_LANGUAGES_V1,flags[\"--lang\"]===undefined?[]:[\"--lang\",flags[\"--lang\"]],context.environment)", count=n)
    tree.edit(ex, "language:languages.length===1?languages[0]!:\"none\"", "language:languages.length===1?languages.join(\"\"):\"none\"")
    imports(tree, ex, "resolveLanguagesV1", catalog=True)
    test = B + "/🌐️remote-cache/🧪️tests/🟦️.ts"
    tree.edit(test, "parseRemoteCacheArgumentsV1,remoteCacheLanguagesV1,remoteCacheStatusV1", "parseRemoteCacheArgumentsV1,remoteCacheStatusV1")
    tree.edit(test, ''' expect(remoteCacheLanguagesV1("de",{})).toEqual(["de"]);expect(remoteCacheLanguagesV1(undefined,{SEMIO_LANGUAGE:"en"})).toEqual(["en"]);expect(remoteCacheLanguagesV1(undefined,{LANG:"de_DE.UTF-8"})).toEqual(["de"]);
 expect(remoteCacheLanguagesV1(undefined,{LANG:"C"}),"no default language: both are printed").toEqual(["en","de"]);
 expect(()=>remoteCacheLanguagesV1("fr",{})).toThrow("Unbekannte Sprache");
''', ''' expect(resolveLanguagesV1(["en","de"],["--lang","de"],{},()=>[])).toEqual(["de"]);expect(resolveLanguagesV1(["en","de"],[],{SEMIO_LANGUAGE:"en"},()=>[])).toEqual(["en"]);expect(resolveLanguagesV1(["en","de"],[],{LANG:"de_DE.UTF-8"},()=>[])).toEqual(["de"]);
 expect(resolveLanguagesV1(["en","de"],[],{LANG:"C"},()=>[]),"no default language: both are printed").toEqual(["en","de"]);
 expect(()=>resolveLanguagesV1(["en","de"],["--lang","fr"],{},()=>[])).toThrow("Unbekannte Sprache");
''')
    imports(tree, test, "resolveLanguagesV1")


def resident(tree):
    R = L + "/🏗️build/🏡️resident"
    policy = R + "/📨️protocol/🎚️policy/🟨️.mjs"
    tree.edit(policy, "/** 🌐 Renders a message in each requested language that exists;", "/** 🥇 The preferred language of a client: the first of the ordered list it accepts, undefined for a list that names none. */\nexport function preferredLanguageV1(languages){return Array.isArray(languages)?languages.at(0):undefined;}\n\n/** 🌐 Renders a message in each requested language that exists;")
    tree.edit(R + "/📨️protocol/🎚️policy/🟨️.d.mts", "export function renderBothV1(", "export function preferredLanguageV1(languages:readonly string[]|undefined):string|undefined;\nexport function renderBothV1(")
    cmd = R + "/⌨️command/🟨️.mjs"
    tree.edit(cmd, "import {POLICY_V1,renderTextV1} from", "import {POLICY_V1,preferredLanguageV1,renderTextV1} from")
    tree.edit(cmd, "return text?text[languages[0]]??Object.values(text)[0]:key;", "return text?text[preferredLanguageV1(languages)]??Object.values(text)[0]:key;")
    svc = R + "/🏛️service/🟨️.mjs"
    tree.edit(svc, "import {POLICY_V1,SUPPORTED_LANGUAGES_V1,renderBothV1,", "import {POLICY_V1,SUPPORTED_LANGUAGES_V1,preferredLanguageV1,renderBothV1,")
    tree.edit(svc, "const language=context.languages[0],say=", "const language=preferredLanguageV1(context.languages),say=")
    tree.edit(svc, "const first=context.languages[0];", "const first=preferredLanguageV1(context.languages);")
    cli = R + "/🚪️client/🟨️.mjs"
    tree.edit(cli, "import {mintCapabilityV1} from \"../📨️protocol/🔑️capability/🟨️.mjs\";", "import {mintCapabilityV1} from \"../📨️protocol/🔑️capability/🟨️.mjs\";\nimport {preferredLanguageV1} from \"../📨️protocol/🎚️policy/🟨️.mjs\";")
    tree.edit(cli, "const text=frame.message[languages[0]]??Object.values(frame.message)[0];", "const text=frame.message[preferredLanguageV1(languages)]??Object.values(frame.message)[0];")
    tree.edit(cli, "const text=frame.message[languages[0]]??frame.message.en;", "const text=frame.message[preferredLanguageV1(languages)]??Object.values(frame.message)[0];")


def leases(tree):
    order = P + "/🔒️leases/🧭️order/🟦️.ts"
    text = tree.text(order)
    a = text.index("/** 🗣️ Picks the message language: `SEMIO_LANGUAGE`, then the locale, then English. */")
    b = text.index("const fill=(template:string")
    tree.put(order, text[:a] + text[b:])
    tree.edit(order, "language=leaseLanguageV1(),message=formatLeaseViolationV1(language,row);", "languages=resolveLanguagesV1(CATALOG_LANGUAGES_V1,[],process.env),message=languages.map(language=>formatLeaseViolationV1(language,row)).join(\"\\n\");")
    tree.edit(order, "process.stderr.write(fill(schema.definitions.Messages.const.observed[language],{message})+\"\\n\")", "process.stderr.write(languages.map(language=>fill(schema.definitions.Messages.const.observed[language],{message:formatLeaseViolationV1(language,row)})).join(\"\\n\")+\"\\n\")")
    add_language_import(tree, order, "resolveLanguagesV1")
    status = P + "/🔒️leases/📋️status/🟦️.ts"
    tree.edit(status, "leaseBlockersV1,leaseLanguageV1,leaseOrderPolicyV1", "leaseBlockersV1,leaseOrderPolicyV1")
    tree.edit(status, "language=leaseLanguageV1(context.environment),limit=", "language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],context.environment).language,limit=")
    add_language_import(tree, status, "resolveLanguageV1")


def add_language_import(tree, rel, names):
    text = tree.text(rel)
    target = P + "/" + PKG
    lang = 'import {%s} from "%s";\nimport {CATALOG_LANGUAGES_V1} from "%s";\n' % (names, rel_import(rel, target + "🟦️.ts"), rel_import(rel, target + "📖️catalog/🟦️.ts"))
    last = 0
    for m in re.finditer(r"^import[^\n]*\n", text, re.M):
        last = m.end()
    tree.put(rel, text[:last] + lang + text[last:])


def toolchain(tree):
    f = P + "/🧪️testing/🧰️toolchain/🟦️.ts"
    tree.edit(f, '''export function describeMissingTestToolV1(resolution:TestToolResolutionV1,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const language=[environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG].map(value=>value?.toLowerCase().split(/[._@-]/u)[0]).find(tag=>tag!==undefined&&(protocol.languages as readonly string[]).includes(tag))??protocol.languages[0]!;
 return protocol.messages.missing[language as "en"].replaceAll("{tool}",resolution.tool).replaceAll("{tried}",resolution.tried.join(", "));
}''', '''export function describeMissingTestToolV1(resolution:TestToolResolutionV1,environment:Readonly<Record<string,string|undefined>>=process.env,osLanguages?:OsLanguageListV1):string{
 return resolveLanguagesV1(protocol.languages,[],environment,osLanguages).map(language=>protocol.messages.missing[language as "en"].replaceAll("{tool}",resolution.tool).replaceAll("{tried}",resolution.tried.join(", "))).join("\\n");
}''')
    tree.edit(f, "/** 🗣️ Renders the missing-tool message in an explicit language: SEMIO_LANGUAGE, then the locale, then the first declared language. */", "/** 🗣️ Renders the missing-tool message in the language the shared resolver chooses, in every declared language when none is chosen. */")
    text = tree.text(f)
    line = 'import {resolveLanguagesV1,type OsLanguageListV1} from "%s";\n' % rel_import(f, P + "/🏗️build/🗣️language/🟦️.ts")
    last = 0
    for m in re.finditer(r"^import[^\n]*\n", text, re.M):
        last = m.end()
    tree.put(f, text[:last] + line + text[last:])
    t = P + "/🧪️testing/🧰️toolchain/🧪️tests/🟦️.ts"
    tree.edit(t, ' expect(describeMissingTestToolV1(resolveTestToolV1("python",english),{})).toContain("Test tool");', ' expect(describeMissingTestToolV1(resolveTestToolV1("python",english),{},()=>[]).split("\\n")).toEqual([expect.stringContaining("Test tool python not found"),expect.stringContaining("Test-Werkzeug python nicht gefunden")]);')


def run(tree):
    import step_defaults_sinks
    step_defaults_sinks.run(tree)
    doctor(tree)
    remote_cache(tree)
    resident(tree)
    leases(tree)
    toolchain(tree)
