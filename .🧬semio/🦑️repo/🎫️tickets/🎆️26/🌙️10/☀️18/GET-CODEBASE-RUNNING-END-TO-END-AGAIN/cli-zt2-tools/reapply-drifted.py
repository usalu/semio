"""Re-applies the cli-zt2 edits of the six hot files (catalogue, its schema, its help snapshot, build script, input precision module and test) onto fresh copies of the live tree inside an overlay; usage: reapply-drifted.py <overlay>. Every anchor must occur exactly once or the script stops."""
import json, os, subprocess, sys
overlay = sys.argv[1]
here = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
O = f"{L}/🏗️build/⚙️operations"
def sub(path, old, new, count=1):
    full = os.path.join(overlay, path)
    text = open(full, encoding="utf-8", newline="").read()
    if text.count(old) != count: sys.exit(f"{path}: anchor count {text.count(old)} != {count}: {old[:90]!r}")
    open(full, "w", encoding="utf-8", newline="").write(text.replace(old, new))
subprocess.run([sys.executable, os.path.join(here, "catalog-edit.py"), os.path.join(overlay, O, "🔣️catalog.json"), os.path.join(here, "notes.json")], check=True)
C = f"{O}/🔣️catalog.json"
sub(C, '"en": "Help language (with --help); otherwise SEMIO_LANGUAGE, then the locale, then the first catalogue language.",', '"en": "Language of the texts; otherwise SEMIO_LANGUAGE, then LANGUAGE and the locale variables (LC_ALL, LC_MESSAGES, LANG), then the operating system language. No language is assumed: the command refuses when none resolves.",')
sub(C, '"de": "Sprache der Hilfe (mit --help); sonst SEMIO_LANGUAGE, dann die Locale, dann die erste Katalogsprache."', '"de": "Sprache der Texte; sonst SEMIO_LANGUAGE, dann LANGUAGE und die Locale-Variablen (LC_ALL, LC_MESSAGES, LANG), dann die Sprache des Betriebssystems. Es wird keine Sprache angenommen: Der Befehl verweigert sich, wenn keine bestimmt werden kann."')
sub(C, 'incremental, concurrency: the disposable repository copy (default: /tmp/sinc, on Windows <drive>:\\\\sinc); it must not lie inside or contain the checkout.', 'incremental, concurrency: the disposable repository copy (default: the folder sinc in the temporary directory of the process); it must not lie inside or contain the checkout, and its path stays short because absolute paths are limited to 256 characters.')
sub(C, 'incremental, concurrency: die Wegwerf-Kopie des Repositorys (Standard: /tmp/sinc, unter Windows <Laufwerk>:\\\\sinc); sie darf weder im Checkout liegen noch ihn enthalten.', 'incremental, concurrency: die Wegwerf-Kopie des Repositorys (Standard: der Ordner sinc im temporären Verzeichnis des Prozesses); sie darf weder im Checkout liegen noch ihn enthalten, und ihr Pfad bleibt kurz, weil absolute Pfade auf 256 Zeichen begrenzt sind.')
F = f"{O}/🧬️schema/🔣️.json"
sub(F, '"notes": { "type": "array", "items": { "type": "string", "minLength": 1 } },', '"notes": { "type": "array", "minItems": 1, "items": { "$ref": "#/definitions/LocalizedText" } },')
sub(F, '"examples", "usedBy"],', '"examples", "usedBy", "notes"],')
sub(F, '"usedBy": { "$ref": "#/definitions/LocalizedText" }\n', '"usedBy": { "$ref": "#/definitions/LocalizedText" },\n        "notes": { "$ref": "#/definitions/LocalizedText" }\n')
sub(F, '        "summary": { "$ref": "#/definitions/LocalizedText" }\n      }\n    },\n    "Example": {', '        "summary": { "$ref": "#/definitions/LocalizedText" },\n        "flags": { "type": "array", "uniqueItems": true, "items": { "$ref": "#/definitions/FlagName" } },\n        "examples": { "type": "array", "minItems": 1, "items": { "$ref": "#/definitions/Example" } }\n      }\n    },\n    "Example": {')
B = f"{L}/🏗️build/📜️script.ts"
sub(B, 'export async function runBuildCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void> {\n  const prefix=', 'export async function runBuildCliV1(root:string,typed:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void> {\n  const{normalizeBuildOptionsV1}=await import("./⚙️operations/🔤️options/🟦️.ts"),{resolveBuildCliLanguageV1}=await import("./⚙️operations/📖️help/🟦️.ts"),args=normalizeBuildOptionsV1(typed);\n  const chosen=resolveBuildCliLanguageV1(args,environment);\n  if(environment===process.env)process.env.SEMIO_LANGUAGE=chosen;\n  const prefix=')
sub(B, 'export async function runOwnedBuildInvocationV1(root:string,args:readonly string[],parent', 'export async function runOwnedBuildInvocationV1(root:string,typed:readonly string[],parent')
sub(B, ' const{withBuildCapacityV1}=await import("./🌡️host-capacity/🟦️.ts");\n await withBuildCapacityV1(args,process.env,granted=>', ' const{normalizeBuildOptionsV1}=await import("./⚙️operations/🔤️options/🟦️.ts"),args=normalizeBuildOptionsV1(typed),{withBuildCapacityV1}=await import("./🌡️host-capacity/🟦️.ts");\n await withBuildCapacityV1(args,process.env,granted=>')
P = f"{L}/🏗️build/📥️inputs/🎯️precision/🟦️.ts"
sub(P, 'export const inputPrecisionLanguageV1 = (document: Pick<InputPrecisionDocumentV1, "messages">, environment: Readonly<Record<string, string | undefined>>): string => resolveBuildHelpLanguageV1(document.messages.languages, [], environment);', 'export const inputPrecisionLanguageV1 = (document: Pick<InputPrecisionDocumentV1, "messages">, environment: Readonly<Record<string, string | undefined>>, osLanguages?: () => readonly string[]): string => resolveBuildHelpLanguageV1(document.messages.languages, [], environment, osLanguages);')
sub(P, '/** 🗣️ Chooses the progress language like the CLI help: SEMIO_LANGUAGE, then the locale, then the document\'s first language. */', '/** 🗣️ Chooses the progress language by the shared rule of the CLI help; no language is assumed. */')
PT = f"{L}/🏗️build/📥️inputs/🎯️precision/🧪️tests/🟦️.ts"
sub(PT, 'for (const row of fixture.messages as {environment: Record<string, string>; language: string}[]) expect(inputPrecisionLanguageV1(document, row.environment), JSON.stringify(row.environment)).toBe(row.language);', 'for (const row of fixture.messages as {environment: Record<string, string>; language?: string; refusal?: string}[]) {\n    if (row.refusal === undefined) expect(inputPrecisionLanguageV1(document, row.environment, () => []), JSON.stringify(row.environment)).toBe(row.language as string);\n    else expect(() => inputPrecisionLanguageV1(document, row.environment, () => []), JSON.stringify(row.environment)).toThrow(row.refusal);\n  }')
subprocess.run(["bun", os.path.join(here, "..", "cli-zt-tools", "snap-help.ts"), overlay, os.path.join(overlay, "snapshot.tmp.json")], check=True)
os.replace(os.path.join(overlay, "snapshot.tmp.json"), os.path.join(overlay, O, "🧪️tests/🧫️fixtures/📖️help/🔣️.json"))
print("reapplied")
