#!/usr/bin/env python3
"""Builds the EN/DE message catalog into src/schema.json and rewrites the runtime sources to raise catalog messages."""
import json, re
def T(en, de): return {"en": en, "de": de}
M = {
 "hostUnknown": T("The hosting runtime is not in the runtime registry.", "Die Laufzeit ist nicht in der Laufzeit-Registratur eingetragen."),
 "notActivated": T("No runtime adapter is active yet; load the runtime facade first.", "Es ist noch kein Laufzeit-Adapter aktiv; lade zuerst die Laufzeit-Fassade."),
 "unknownRuntime": T("Unknown runtime {runtime}.", "Unbekannte Laufzeit {runtime}."),
 "adapterNotLoaded": T("Runtime adapter {runtime} is not loaded; await loadRuntimeAdapterV1 first.", "Der Laufzeit-Adapter {runtime} ist nicht geladen; warte zuerst auf loadRuntimeAdapterV1."),
 "capabilityMissing": T("Runtime {runtime} lacks capability {family}.", "Der Laufzeit {runtime} fehlt die Fähigkeit {family}."),
 "capabilityMissingReason": T("Runtime {runtime} lacks capability {family}: {reason}", "Der Laufzeit {runtime} fehlt die Fähigkeit {family}: {reason}"),
 "reasonEsbuild": T("the esbuild provider is not installed; set SEMIO_RUNTIME_ESBUILD to its module or install it beside the workspace", "der esbuild-Anbieter ist nicht installiert; setze SEMIO_RUNTIME_ESBUILD auf sein Modul oder installiere ihn neben dem Arbeitsbereich"),
 "reasonNativeNone": T("Node has no foreign function interface; set SEMIO_RUNTIME_NATIVE_PROVIDER to a module exporting nativeSystemV1", "Node hat keine Schnittstelle für native Funktionen; setze SEMIO_RUNTIME_NATIVE_PROVIDER auf ein Modul, das nativeSystemV1 exportiert"),
 "reasonNativeNoExport": T("{designation} exports no nativeSystemV1", "{designation} exportiert kein nativeSystemV1"),
 "bodyTooLarge": T("The request body is too large.", "Der Anfragekörper ist zu groß."),
 "commandRequired": T("A command is required.", "Ein Befehl ist erforderlich."),
 "globTooMany": T("The glob pattern expands to too many alternatives.", "Das Glob-Muster wird zu viele Alternativen."),
 "globNegated": T("Negated glob patterns cannot be scanned.", "Negierte Glob-Muster lassen sich nicht durchsuchen."),
 "globAbsolute": T("Absolute glob scan patterns are not supported.", "Absolute Glob-Suchmuster werden nicht unterstützt."),
 "jsoncUnterminated": T("Unterminated JSONC comment.", "Nicht abgeschlossener JSONC-Kommentar."),
 "pluginAsync": T("A loader plugin must set up synchronously.", "Ein Lade-Plugin muss sich synchron einrichten."),
 "moduleNotFound": T("Cannot find module '{specifier}' from '{from}'.", "Das Modul '{specifier}' ist von '{from}' aus nicht auffindbar."),
 "packageSubpath": T("Package subpath '{subpath}' is not exported by '{name}'.", "Der Paketunterpfad '{subpath}' wird von '{name}' nicht exportiert."),
 "semverInvalid": T("Invalid semantic version: {version}.", "Ungültige semantische Version: {version}."),
 "loaderUnsupported": T("The portable transpiler does not support the {loader} loader.", "Der portable Transpiler unterstützt den Lader {loader} nicht."),
 "stripperMissing": T("The host provides no TypeScript stripper.", "Die Laufzeit stellt keinen TypeScript-Entferner bereit."),
 "sqliteUnavailable": T("node:sqlite is unavailable in this runtime.", "node:sqlite ist in dieser Laufzeit nicht verfügbar."),
 "sqliteCannotOpen": T("Unable to open the database file: {path}", "Die Datenbankdatei lässt sich nicht öffnen: {path}"),
 "testRunnerOnly": T("The test runner is only available inside a test run of this runtime.", "Der Testläufer ist nur innerhalb eines Testlaufs dieser Laufzeit verfügbar."),
 "syntax": T("{format} syntax error: {detail} at {position}", "{format}-Syntaxfehler: {detail} bei {position}"),
 "toml.expectedEndOfLine": T("end of line expected", "Zeilenende erwartet"),
 "toml.expectedBracket": T("] expected", "] erwartet"),
 "toml.expectedEquals": T("= expected", "= erwartet"),
 "toml.expectedKey": T("key expected", "Schlüssel erwartet"),
 "toml.notATable": T("key {name} is not a table", "Schlüssel {name} ist keine Tabelle"),
 "toml.tableDefined": T("table {name} is already defined", "Tabelle {name} ist bereits definiert"),
 "toml.notAnArrayOfTables": T("key {name} is not an array of tables", "Schlüssel {name} ist keine Tabellenliste"),
 "toml.keyDefined": T("key {name} is already defined", "Schlüssel {name} ist bereits definiert"),
 "toml.expectedCommaOrBracket": T(", or ] expected", ", oder ] erwartet"),
 "toml.expectedCommaOrBrace": T(", or } expected", ", oder } erwartet"),
 "toml.unterminatedString": T("unterminated string", "nicht abgeschlossene Zeichenkette"),
 "toml.controlCharacter": T("control character in string", "Steuerzeichen in Zeichenkette"),
 "toml.invalidUnicodeEscape": T("invalid unicode escape", "ungültige Unicode-Escape-Folge"),
 "toml.invalidUnicodeScalar": T("invalid unicode scalar", "ungültiger Unicode-Wert"),
 "toml.invalidEscape": T("invalid escape", "ungültige Escape-Folge"),
 "toml.unterminatedLiteral": T("unterminated literal string", "nicht abgeschlossene Literal-Zeichenkette"),
 "toml.unterminatedMultiline": T("unterminated multiline string", "nicht abgeschlossene mehrzeilige Zeichenkette"),
 "toml.unterminatedMultilineLiteral": T("unterminated multiline literal string", "nicht abgeschlossene mehrzeilige Literal-Zeichenkette"),
 "toml.tooManyQuotes": T("too many quotes", "zu viele Anführungszeichen"),
 "toml.invalidDateTime": T("invalid date-time", "ungültiges Datum mit Uhrzeit"),
 "toml.invalidInteger": T("invalid integer", "ungültige Ganzzahl"),
 "toml.invalidValue": T("invalid value {token}", "ungültiger Wert {token}"),
 "yaml.invalidEscape": T("invalid escape", "ungültige Escape-Folge"),
 "yaml.unexpectedFlow": T("unexpected flow content", "unerwarteter Flow-Inhalt"),
 "yaml.expectedFlowSequence": T(", or ] expected in flow sequence", ", oder ] in Flow-Liste erwartet"),
 "yaml.expectedFlowMapping": T(", or } expected in flow mapping", ", oder } in Flow-Abbildung erwartet"),
 "yaml.unexpectedContent": T("unexpected content", "unerwarteter Inhalt"),
 "yaml.badIndentation": T("bad indentation", "falsche Einrückung"),
 "yaml.duplicateKey": T("duplicate key {key}", "doppelter Schlüssel {key}"),
 "yaml.tags": T("tags are not supported", "Tags werden nicht unterstützt"),
 "yaml.unknownAlias": T("unknown alias {name}", "unbekannter Alias {name}"),
}
sch = json.load(open("src/schema.json", encoding="utf8"))
sch["definitions"]["Messages"] = {"type": "object", "const": M, "languageVariables": ["SEMIO_LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"]}
json.dump(sch, open("src/schema.json", "w", encoding="utf8"), ensure_ascii=False, indent=2); open("src/schema.json", "a").write("\n")

def edit(path, pairs, regex=()):
    s = open(path, encoding="utf8").read()
    for old, new in pairs:
        if old not in s: raise SystemExit(f"MISSING in {path}: {old[:70]}")
        s = s.replace(old, new)
    for pat, repl in regex: s = re.sub(pat, repl, s)
    open(path, "w", encoding="utf8").write(s)

IMPORT = lambda depth: f'import {{ runtimeMessageV1 }} from "{depth}🧭️selection/🟦️.ts";\n'
# toml
t = open("src/toml.ts", encoding="utf8").read()
t = t.replace('''  private fail(message: string): never {
    let line = 1, column = 1;
    for (let at = 0; at < this.index && at < this.text.length; at++) { if (this.text[at] === "\\n") { line++; column = 1; } else column++; }
    throw new SyntaxError(`TOML ${message} at ${line}:${column}`);
  }''','''  private fail(id: string, parameters: Readonly<Record<string, string | number>> = {}): never {
    let line = 1, column = 1;
    for (let at = 0; at < this.index && at < this.text.length; at++) { if (this.text[at] === "\\n") { line++; column = 1; } else column++; }
    throw new SyntaxError(runtimeMessageV1("syntax", { format: "TOML", detail: runtimeMessageV1(id, parameters), position: `${line}:${column}` }));
  }''')
for old,new in [('this.fail("expected end of line")','this.fail("toml.expectedEndOfLine")'),('this.fail("expected ]")','this.fail("toml.expectedBracket")'),('this.fail("expected =")','this.fail("toml.expectedEquals")'),('this.fail("expected key")','this.fail("toml.expectedKey")'),
 ('this.fail(`key ${name} is not a table`)','this.fail("toml.notATable", { name })'),('this.fail(`table ${path.join(".")} is already defined`)','this.fail("toml.tableDefined", { name: path.join(".") })'),('this.fail(`key ${name} is not an array of tables`)','this.fail("toml.notAnArrayOfTables", { name })'),('this.fail(`key ${name} is already defined`)','this.fail("toml.keyDefined", { name })'),
 ('this.fail("expected , or ]")','this.fail("toml.expectedCommaOrBracket")'),('this.fail("expected , or }")','this.fail("toml.expectedCommaOrBrace")'),('this.fail("unterminated string")','this.fail("toml.unterminatedString")'),('this.fail("control character in string")','this.fail("toml.controlCharacter")'),
 ('this.fail("invalid unicode escape")','this.fail("toml.invalidUnicodeEscape")'),('this.fail("invalid unicode scalar")','this.fail("toml.invalidUnicodeScalar")'),('this.fail("invalid escape")','this.fail("toml.invalidEscape")'),('this.fail("unterminated literal string")','this.fail("toml.unterminatedLiteral")'),
 ('this.fail("unterminated multiline string")','this.fail("toml.unterminatedMultiline")'),('this.fail("unterminated multiline literal string")','this.fail("toml.unterminatedMultilineLiteral")'),('this.fail("too many quotes")','this.fail("toml.tooManyQuotes")'),('this.fail("invalid date-time")','this.fail("toml.invalidDateTime")'),
 ('this.fail("invalid integer")','this.fail("toml.invalidInteger")'),('this.fail(`invalid value ${JSON.stringify(token)}`)','this.fail("toml.invalidValue", { token: JSON.stringify(token) })')]:
    if old not in t: raise SystemExit("toml missing "+old)
    t = t.replace(old,new)
t = IMPORT("../../") + t
open("src/toml.ts","w",encoding="utf8").write(t)
# yaml
y = open("src/yaml.ts", encoding="utf8").read()
y = y.replace('function fail(line: LineV1 | undefined, message: string): never { throw new SyntaxError(`YAML ${message}${line === undefined ? "" : ` at line ${line.number}`}`); }','function fail(line: LineV1 | undefined, id: string, parameters: Readonly<Record<string, string | number>> = {}): never { throw new SyntaxError(runtimeMessageV1("syntax", { format: "YAML", detail: runtimeMessageV1(id, parameters), position: line === undefined ? "-" : String(line.number) })); }')
for old,new in [('fail(line, "invalid escape")','fail(line, "yaml.invalidEscape")'),('fail(this.line, "unexpected flow content")','fail(this.line, "yaml.unexpectedFlow")'),('fail(this.line, "expected , or ] in flow sequence")','fail(this.line, "yaml.expectedFlowSequence")'),('fail(this.line, "expected , or } in flow mapping")','fail(this.line, "yaml.expectedFlowMapping")'),
 ('fail(this.lines[this.position], "unexpected content")','fail(this.lines[this.position], "yaml.unexpectedContent")'),('fail(line, "bad indentation")','fail(line, "yaml.badIndentation")'),('fail(line, `duplicate key ${key}`)','fail(line, "yaml.duplicateKey", { key })'),('fail(line, "tags are not supported")','fail(line, "yaml.tags")'),('fail(line, `unknown alias ${name}`)','fail(line, "yaml.unknownAlias", { name })')]:
    if old not in y: raise SystemExit("yaml missing "+old)
    y = y.replace(old,new)
y = IMPORT("../../") + y
open("src/yaml.ts","w",encoding="utf8").write(y)
# others
edit("src/glob.ts",[('throw Error("Glob pattern expands to too many alternatives")','throw Error(runtimeMessageV1("globTooMany", {}))'),('throw Error("Negated glob patterns cannot be scanned")','throw Error(runtimeMessageV1("globNegated", {}))')],[(r'throw Error\("Absolute glob scan patterns are not supported"\)','throw Error(runtimeMessageV1("globAbsolute", {}))')])
s=open("src/glob.ts",encoding="utf8").read(); open("src/glob.ts","w",encoding="utf8").write(s.replace('import type { RuntimeGlobScanOptionsV1','import { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";\nimport type { RuntimeGlobScanOptionsV1',1))
s=open("src/jsonc.ts",encoding="utf8").read().replace('throw new SyntaxError("Unterminated JSONC comment")','throw new SyntaxError(runtimeMessageV1("jsoncUnterminated", {}))'); open("src/jsonc.ts","w",encoding="utf8").write(IMPORT("../../")+s)
edit("src/plugin.ts",[('throw new Error("A loader plugin must set up synchronously")','throw new Error(runtimeMessageV1("pluginAsync", {}))')]); s=open("src/plugin.ts",encoding="utf8").read(); open("src/plugin.ts","w",encoding="utf8").write(s.replace('import type { RuntimeBundleBuilderV1','import { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";\nimport type { RuntimeBundleBuilderV1',1))
edit("src/resolve.ts",[("throw new Error(`Cannot find module '${specifier}' from '${base}'`)","throw new Error(runtimeMessageV1(\"moduleNotFound\", { specifier, from: base }))"),("throw new Error(`Package subpath '${subpath}' is not exported by '${name}'`)","throw new Error(runtimeMessageV1(\"packageSubpath\", { subpath, name }))")]) if False else None
s=open("src/resolve.ts",encoding="utf8").read()
s=s.replace("throw new Error(`Cannot find module '${specifier}' from '${base}'`)",'throw new Error(runtimeMessageV1("moduleNotFound", { specifier, from: base }))').replace("throw new Error(`Package subpath '${subpath}' is not exported by '${name}'`)",'throw new Error(runtimeMessageV1("packageSubpath", { subpath, name }))')
s=s.replace('import { fileURLToPath } from "node:url";','import { fileURLToPath } from "node:url";\nimport { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";',1)
open("src/resolve.ts","w",encoding="utf8").write(s)
s=open("src/semver.ts",encoding="utf8").read().replace("throw new TypeError(`Invalid semantic version: ${a === undefined ? left : right}`)",'throw new TypeError(runtimeMessageV1("semverInvalid", { version: a === undefined ? left : right }))'); open("src/semver.ts","w",encoding="utf8").write(IMPORT("../../")+s)
s=open("src/serve.ts",encoding="utf8").read().replace('new Error("Request body too large")','new Error(runtimeMessageV1("bodyTooLarge", {}))').replace('import type { RuntimeServeOptionsV1','import { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";\nimport type { RuntimeServeOptionsV1',1); open("src/serve.ts","w",encoding="utf8").write(s)
s=open("src/source.ts",encoding="utf8").read().replace("throw new Error(`The portable transpiler does not support the ${loader} loader`)",'throw new Error(runtimeMessageV1("loaderUnsupported", { loader }))').replace('throw new Error("The host provides no TypeScript stripper")','throw new Error(runtimeMessageV1("stripperMissing", {}))'); open("src/source.ts","w",encoding="utf8").write(s.replace('import type { RuntimeImportRecordV1','import { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";\nimport type { RuntimeImportRecordV1',1))
s=open("src/spawn.ts",encoding="utf8").read().replace('throw new TypeError("A command is required")','throw new TypeError(runtimeMessageV1("commandRequired", {}))'); open("src/spawn.ts","w",encoding="utf8").write(s.replace('import type { RuntimeByteStreamV1','import { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";\nimport type { RuntimeByteStreamV1',1))
s=open("src/sqlite.ts",encoding="utf8").read().replace('throw new Error("node:sqlite is unavailable in this runtime")','throw new Error(runtimeMessageV1("sqliteUnavailable", {}))').replace("throw new Error(`unable to open database file: ${path}`)",'throw new Error(runtimeMessageV1("sqliteCannotOpen", { path }))'); open("src/sqlite.ts","w",encoding="utf8").write(s.replace('import type { RuntimeSqliteChangesV1','import { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";\nimport type { RuntimeSqliteChangesV1',1))
s=open("src/testapi-bun.ts",encoding="utf8").read().replace('throw new Error("The Bun test runner is only available inside `bun test`")','throw new Error(runtimeMessageV1("testRunnerOnly", {}))'); open("src/testapi-bun.ts","w",encoding="utf8").write(s.replace('import type { RuntimeTestApiV1','import { runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";\nimport type { RuntimeTestApiV1',1))
# esbuild/native capability reasons
s=open("src/esbuild.ts",encoding="utf8").read().replace('throw new RuntimeCapabilityMissingV1("bundle", "node", "the esbuild provider is not installed; set SEMIO_RUNTIME_ESBUILD to its module or install it beside the workspace")','throw new RuntimeCapabilityMissingV1("bundle", "node", runtimeMessageV1("reasonEsbuild", {}))'); open("src/esbuild.ts","w",encoding="utf8").write(s.replace('import { RuntimeCapabilityMissingV1 } from "../../🚫️missing/🟦️.ts";','import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";'))
s=open("src/native.ts",encoding="utf8").read().replace('"Node has no foreign function interface; set SEMIO_RUNTIME_NATIVE_PROVIDER to a module exporting nativeSystemV1"','runtimeMessageV1("reasonNativeNone", {})').replace("`${designation} exports no nativeSystemV1`",'runtimeMessageV1("reasonNativeNoExport", { designation })'); open("src/native.ts","w",encoding="utf8").write(s.replace('import { RuntimeCapabilityMissingV1 } from "../../🚫️missing/🟦️.ts";','import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";'))
print("ok")
