#!/usr/bin/env python3
"""Increment 1 (round 7): the persisted local-only repo config as a language source, `repo config set language <de|en>` as an event."""
import json, os
from lib import *

LANGUAGE = LANG + "/🟦️.ts"
HELP = L + "/🏗️build/⚙️operations/📖️help/🟦️.ts"
CATALOG = L + "/🏗️build/⚙️operations/🔣️catalog.json"
OPS_SCHEMA = L + "/🏗️build/⚙️operations/🧬️schema/🔣️.json"
HELP_FIXTURE = L + "/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
SCRIPT = L + "/🏗️build/📜️script.ts"
DERIVED = os.path.join(HERE, "staged-config", "derived", "help.json")

REFUSAL_EN = "No language selected for texts. Run `repo config set language {choice}` once to remember it for this checkout, or pass --lang={choice}, set SEMIO_LANGUAGE={choice}, or run with a locale such as LANG=en_US.UTF-8 (resolution order: --lang, SEMIO_LANGUAGE, the repo config, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, the operating system language list)."
REFUSAL_DE = "Keine Sprache für Texte gewählt. Führe einmal `repo config set language {choice}` aus, um sie für diesen Checkout zu merken, übergib --lang={choice}, setze SEMIO_LANGUAGE={choice} oder starte mit einem Locale wie LANG=de_DE.UTF-8 (Reihenfolge: --lang, SEMIO_LANGUAGE, Repo-Konfiguration, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, Sprachliste des Betriebssystems)."


def protocol(tree):
    f = LANG + "/🔣️.json"
    tree.edit(f, '"explicitVariable": "SEMIO_LANGUAGE",\n', '"explicitVariable": "SEMIO_LANGUAGE",\n  "configSource": "config",\n')
    tree.edit(f, '"en": "No language selected for texts. Pass --lang={choice}, set SEMIO_LANGUAGE={choice}, or run with a locale such as LANG=en_US.UTF-8 (resolution order: --lang, SEMIO_LANGUAGE, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, the operating system language list).",', '"en": ' + json.dumps(REFUSAL_EN, ensure_ascii=False) + ',')
    tree.edit(f, '"de": "Keine Sprache für Texte gewählt. Übergib --lang={choice}, setze SEMIO_LANGUAGE={choice} oder starte mit einem Locale wie LANG=de_DE.UTF-8 (Reihenfolge: --lang, SEMIO_LANGUAGE, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, Sprachliste des Betriebssystems).",', '"de": ' + json.dumps(REFUSAL_DE, ensure_ascii=False) + ',')
    tree.edit(f, '"unknownFlag": {', '"unknownConfig": { "en": "The repo config names a language that is not offered: {value} ({choice}); fix it with `repo config set language {choice}`", "de": "Die Repo-Konfiguration nennt eine nicht angebotene Sprache: {value} ({choice}); korrigiere sie mit `repo config set language {choice}`" },\n    "unknownFlag": {')
    s = LANG + "/🧬️schema/🔣️.json"
    tree.edit(s, '"required": ["version", "environmentOrder", "explicitVariable", "preferenceListVariable",', '"required": ["version", "environmentOrder", "explicitVariable", "configSource", "preferenceListVariable",')
    tree.edit(s, '"explicitVariable": { "type": "string", "pattern": "^[A-Z_]+$" },\n', '"explicitVariable": { "type": "string", "pattern": "^[A-Z_]+$" },\n    "configSource": { "const": "config" },\n')
    tree.edit(s, '"required": ["en", "de", "unknownFlag", "unknownVariable"],', '"required": ["en", "de", "unknownFlag", "unknownVariable", "unknownConfig"],')
    tree.edit(s, '"unknownVariable": { "$ref": "#/definitions/Bilingual" }', '"unknownVariable": { "$ref": "#/definitions/Bilingual" },\n        "unknownConfig": { "$ref": "#/definitions/Bilingual" }')


def resolver(tree):
    f = LANGUAGE
    tree.edit(f, "let remembered: readonly string[] | undefined;", '''/** 🧱️ Which source refused to choose a language: nothing named one, a wrong flag, a wrong variable or a wrong repo config. */
export type LanguageRefusalReasonV1 = "none" | "flag" | "variable" | "config";

/** 🚫️ The bilingual refusal to choose a language; `reason` lets an emitter without a CLI boundary tell "nothing chosen" from "a wrong choice". */
export class LanguageRefusalV1 extends Error {
  readonly reason: LanguageRefusalReasonV1;
  constructor(reason: LanguageRefusalReasonV1, message: string) {
    super(message);
    this.name = "LanguageRefusal";
    this.reason = reason;
  }
}

let remembered: readonly string[] | undefined;''')
    tree.edit(f, '''/** 🚫️ The refusal that names every way to choose a language, in every offered language; it never depends on a language being chosen. */
export function languageRefusalV1(languages: readonly string[]): Error {''', '''/** 🧷️ The refusal that names every way to choose a language, in every offered language; it never depends on a language being chosen. */
export function languageRefusalV1(languages: readonly string[]): LanguageRefusalV1 {''')
    tree.edit(f, '  return new Error(languages.map((language) => fill((protocol.refusal as unknown as Record<string, string>)[language] ?? protocol.refusal.en)).join("\\n"));', '  return new LanguageRefusalV1("none", languages.map((language) => fill((protocol.refusal as unknown as Record<string, string>)[language] ?? protocol.refusal.en)).join("\\n"));')
    tree.edit(f, 'function refuse(row: Readonly<Record<"en" | "de", string>>, languages: readonly string[], values: Readonly<Record<string, string>>): Error {', 'function refuse(reason: LanguageRefusalReasonV1, row: Readonly<Record<"en" | "de", string>>, languages: readonly string[], values: Readonly<Record<string, string>>): LanguageRefusalV1 {')
    tree.edit(f, '  return new Error(languages.map((language) => fill(row[language as "en" | "de"] ?? row.en)).join("\\n"));', '  return new LanguageRefusalV1(reason, languages.map((language) => fill(row[language as "en" | "de"] ?? row.en)).join("\\n"));')
    tree.edit(f, '/** 🗣️ Chooses the language of texts from the offered languages; no language is assumed. Order: `--lang`, `SEMIO_LANGUAGE` (both must name an offered language), `LANGUAGE` (first offered entry),', '/** 🗣️ Chooses the language of texts from the offered languages; no language is assumed. Order: `--lang`, `SEMIO_LANGUAGE`, the language the repo config names (all three must name an offered language), `LANGUAGE` (first offered entry),')
    tree.edit(f, 'osLanguages: OsLanguageListV1 = osLanguageListV1): LanguageResolutionV1<T> {', 'osLanguages: OsLanguageListV1 = osLanguageListV1, configured?: string): LanguageResolutionV1<T> {')
    tree.edit(f, 'throw refuse(protocol.refusal.unknownFlag,', 'throw refuse("flag", protocol.refusal.unknownFlag,')
    tree.edit(f, 'throw refuse(protocol.refusal.unknownVariable,', 'throw refuse("variable", protocol.refusal.unknownVariable,')
    tree.edit(f, '''    return { language: tag as T, source: protocol.explicitVariable };
  }
''', '''    return { language: tag as T, source: protocol.explicitVariable };
  }
  if (configured !== undefined) {
    if (!offered.includes(configured)) throw refuse("config", protocol.refusal.unknownConfig, languages, { value: configured });
    return { language: configured as T, source: protocol.configSource };
  }
''')
    text = tree.text(f)
    tree.put(f, text.rstrip("\n") + '''

/** 🌐️ For emitters that run without a CLI boundary: the chosen language, or every offered language when nothing is chosen, because no language is assumed; a wrong explicit choice is still refused. */
export function resolveLanguagesV1<T extends string>(languages: readonly T[], args: readonly string[], environment: LanguageEnvironmentV1, osLanguages: OsLanguageListV1 = osLanguageListV1, configured?: string): readonly T[] {
  try {
    return [resolveLanguageV1(languages, args, environment, osLanguages, configured).language];
  } catch (error) {
    if (error instanceof LanguageRefusalV1 && error.reason === "none") return languages;
    throw error;
  }
}
''')


def language_tests(tree):
    t = LANG + "/🧪️tests/🟦️.ts"
    tree.edit(t, 'import { languageFlagV1, languageRefusalV1, languageSubtagV1, osLanguageListV1, resolveLanguageV1 } from "../🟦️.ts";', 'import { languageFlagV1, languageRefusalV1, languageSubtagV1, osLanguageListV1, resolveLanguageV1, resolveLanguagesV1 } from "../🟦️.ts";')
    tree.edit(t, 'type Row = Readonly<{ name: string; args: string[]; environment: Record<string, string>; os: string[];', 'type Row = Readonly<{ name: string; args: string[]; environment: Record<string, string>; os: string[]; configured?: string;')
    tree.edit(t, 'resolveLanguageV1(fixture.languages, row.args, row.environment, () => row.os);', 'resolveLanguageV1(fixture.languages, row.args, row.environment, () => row.os, row.configured);')
    tree.edit(t, 'for (const choice of ["--lang=en|de", "SEMIO_LANGUAGE=en|de"])', 'for (const choice of ["repo config set language en|de", "--lang=en|de", "SEMIO_LANGUAGE=en|de"])')
    tree.edit(t, '    if (row.refusal === "variable") expect(lines[0])', '    if (row.refusal === "config") expect(lines[0]).toContain("The repo config names a language that is not offered");\n    if (row.refusal === "config") expect(lines[1]).toContain("Die Repo-Konfiguration nennt eine nicht angebotene Sprache");\n    if (row.refusal === "variable") expect(lines[0])')
    tree.put(t, tree.text(t).rstrip("\n") + '''

test("the repo config names the language before the locale and after the explicit sources, and the refusal names it first", () => {
  expect(languageRefusalV1(["en", "de"]).message.split("\\n").every((line) => line.indexOf("repo config set language en|de") < line.indexOf("--lang=en|de"))).toBe(true);
  expect(resolveLanguageV1(["en", "de"], [], { LANG: "en_US.UTF-8", LANGUAGE: "en" }, () => ["en-US"], "de")).toEqual({ language: "de", source: "config" });
});

test("an emitter without a CLI boundary gets the chosen language, every language when none is chosen and still a refusal for a wrong choice", () => {
  expect(resolveLanguagesV1(["en", "de"], [], { SEMIO_LANGUAGE: "de" }, () => [])).toEqual(["de"]);
  expect(resolveLanguagesV1(["en", "de"], [], {}, () => [])).toEqual(["en", "de"]);
  expect(resolveLanguagesV1(["en", "de"], [], {}, () => [], "de")).toEqual(["de"]);
  expect(() => resolveLanguagesV1(["en", "de"], ["--lang=fr"], {}, () => [])).toThrow("Unbekannte Sprache");
  expect(() => resolveLanguagesV1(["en", "de"], [], {}, () => [], "fr")).toThrow("Die Repo-Konfiguration");
});
''')
    f = LANG + "/🧪️tests/🔣️.json"
    rows = '''    { "name": "the repo config beats the locale", "args": [], "environment": { "LANG": "en_US.UTF-8", "LANGUAGE": "en" }, "os": ["en-US"], "configured": "de", "expected": { "language": "de", "source": "config" } },
    { "name": "the repo config resolves when nothing else does", "args": [], "environment": {}, "os": [], "configured": "en", "expected": { "language": "en", "source": "config" } },
    { "name": "the explicit variable beats the repo config", "args": [], "environment": { "SEMIO_LANGUAGE": "en" }, "os": [], "configured": "de", "expected": { "language": "en", "source": "SEMIO_LANGUAGE" } },
    { "name": "the flag beats the repo config", "args": ["--lang=en"], "environment": {}, "os": [], "configured": "de", "expected": { "language": "en", "source": "--lang" } },
    { "name": "an unoffered repo config is refused", "args": [], "environment": { "LANG": "de_DE.UTF-8" }, "os": ["de-DE"], "configured": "fr", "refusal": "config" },
'''
    tree.edit(f, '    { "name": "unoffered flag is refused"', rows + '    { "name": "unoffered flag is refused"')


def help_door(tree):
    tree.edit(HELP, "export function resolveBuildHelpLanguageV1(languages:readonly string[],args:readonly string[],environment:LanguageEnvironmentV1,osLanguages?:OsLanguageListV1):string{\n return resolveLanguageV1(languages,args,environment,osLanguages).language;", "export function resolveBuildHelpLanguageV1(languages:readonly string[],args:readonly string[],environment:LanguageEnvironmentV1,osLanguages?:OsLanguageListV1,configured?:string):string{\n return resolveLanguageV1(languages,args,environment,osLanguages,configured).language;")
    tree.edit(HELP, "export function resolveBuildCliLanguageV1(args:readonly string[],environment:LanguageEnvironmentV1,osLanguages?:OsLanguageListV1):string{\n return resolveBuildHelpLanguageV1((catalogue as BuildHelpCatalogV1).languages,args,environment,osLanguages);", "export function resolveBuildCliLanguageV1(args:readonly string[],environment:LanguageEnvironmentV1,osLanguages?:OsLanguageListV1,configured?:string):string{\n return resolveBuildHelpLanguageV1((catalogue as BuildHelpCatalogV1).languages,args,environment,osLanguages,configured);")
    tree.edit(HELP, "(resolution order: --lang, SEMIO_LANGUAGE, LANGUAGE", "(resolution order: --lang, SEMIO_LANGUAGE, LANGUAGE") if False else None
    old = "  const chosen=resolveBuildCliLanguageV1(args,environment);\n"
    new = '''  const configPrefix=args[0]==="repo"||args[0]==="build"?1:0;
  if(args[configPrefix]==="config"){const{runConfigCliV1}=await import("./🎚️config/⌨️command/🟦️.ts"),{buildHelpForArgumentsV1}=await import("./⚙️operations/📖️help/🟦️.ts");process.exitCode=await runConfigCliV1(root,args.slice(configPrefix+1),{environment,args,help:language=>buildHelpForArgumentsV1(["config"],{...environment,SEMIO_LANGUAGE:language})});return;}
  const explicitLanguage=(environment.SEMIO_LANGUAGE??"")!==""||args.some(argument=>argument==="--lang"||argument.startsWith("--lang=")),chosen=resolveBuildCliLanguageV1(args,environment,undefined,explicitLanguage?undefined:(await import("./🎚️config/⌨️command/🟦️.ts")).configuredLanguageForDoorV1(root,args,environment));
'''
    tree.edit(SCRIPT, old, new)


OPERATION = {
    "name": "config", "requires": "none",
    "synopsis": "config set language <en|de> | unset language | show [--json]",
    "summary": {
        "en": "Keep the repo's local settings as an append-only event log in the git directory; today the language of every text. `config set language` is how a language is chosen once for this checkout, before any other command runs.",
        "de": "Hält die lokalen Einstellungen des Repos als Ereignisprotokoll im Git-Verzeichnis, an das nur angehängt wird; heute die Sprache aller Texte. `config set language` wählt einmal eine Sprache für diesen Checkout, bevor irgendein anderer Befehl läuft."
    },
    "implementations": {"nx": "neutral", "bazel": "neutral"},
    "dryRun": False, "progress": False, "cancellable": False,
    "subcommands": [
        {"name": "set", "synopsis": "config set language <en|de>", "summary": {
            "en": "Append a language-chosen event: from now on every text of this checkout is printed in this language, unless --lang or SEMIO_LANGUAGE says otherwise.",
            "de": "Hängt ein Ereignis \"Sprache gewählt\" an: Ab jetzt wird jeder Text dieses Checkouts in dieser Sprache ausgegeben, sofern nicht --lang oder SEMIO_LANGUAGE etwas anderes sagt."}},
        {"name": "unset", "synopsis": "config unset language", "summary": {
            "en": "Append a language-cleared event: the repo config names no language again and the locale decides.",
            "de": "Hängt ein Ereignis \"Sprache gelöscht\" an: Die Repo-Konfiguration nennt wieder keine Sprache und das Locale entscheidet."}},
        {"name": "show", "synopsis": "config show [--json]", "summary": {
            "en": "Print the configured language, the event log and the language that is in effect now with its source.",
            "de": "Gibt die konfigurierte Sprache, das Ereignisprotokoll und die jetzt wirksame Sprache mit ihrer Quelle aus."}},
    ],
    "flags": ["--json"],
    "examples": [
        {"command": "repo config set language de", "description": {"en": "Choose German for this checkout once; every later command answers in German.", "de": "Wählt einmal Deutsch für diesen Checkout; jeder spätere Befehl antwortet auf Deutsch."}},
        {"command": "repo config set language en", "description": {"en": "Choose English for this checkout.", "de": "Wählt Englisch für diesen Checkout."}},
        {"command": "repo config show", "description": {"en": "See which language is configured and which one is in effect.", "de": "Zeigt, welche Sprache konfiguriert und welche wirksam ist."}},
        {"command": "repo config unset language", "description": {"en": "Forget the choice and let the locale decide again.", "de": "Vergisst die Wahl und lässt wieder das Locale entscheiden."}},
    ],
    "notes": [
        {"en": "The log is append-only and local: .git/semio/config/events.jsonl, or the directory named by SEMIO_CONFIG_DIRECTORY. Nothing is rewritten; the last event of a setting wins.",
         "de": "Das Protokoll ist lokal und wird nur ergänzt: .git/semio/config/events.jsonl oder das Verzeichnis, das SEMIO_CONFIG_DIRECTORY nennt. Nichts wird umgeschrieben; das letzte Ereignis einer Einstellung gilt."},
        {"en": "Order of the language sources: --lang, SEMIO_LANGUAGE, the repo config, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, the operating system language list.",
         "de": "Reihenfolge der Sprachquellen: --lang, SEMIO_LANGUAGE, die Repo-Konfiguration, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, die Sprachliste des Betriebssystems."},
    ],
}


def operation(tree):
    text = tree.text(CATALOG)
    end = "\n  ]\n}\n"
    assert text.endswith(end)
    block = json.dumps(OPERATION, ensure_ascii=False, indent=2)
    block = "\n".join("    " + line for line in block.split("\n"))
    text = text[:-len(end)] + ",\n" + block + end
    tree.put(CATALOG, text)
    tree.edit(CATALOG, '"cache", "remote-cache", "doctor", "service", "session"],\n      "description": { "en": "Print lists as JSON', '"cache", "remote-cache", "doctor", "service", "session", "config"],\n      "description": { "en": "Print lists as JSON')
    tree.edit(OPS_SCHEMA, '"service", "session"]', '"service", "session", "config"]')
    if os.path.exists(DERIVED):
        tree.put(HELP_FIXTURE, open(DERIVED, encoding="utf8").read())


def texts_test(tree):
    from edits_catalogs import test_module
    base = L + "/🏗️build/🎚️config/📖️texts"
    test_ts = base + "/🧪️tests/🟦️.ts"
    c = {"handle": "CONFIG_TEXTS_V1", "say": "sayConfigV1", "refuse": "refuseConfigV1"}
    tree.put(test_ts, test_module("config", c, "..", [], lambda target: rel_import(test_ts, P + "/" + target)))


def run(tree):
    texts_test(tree)
    protocol(tree)
    resolver(tree)
    language_tests(tree)
    help_door(tree)
    operation(tree)
