import { load, set, edit } from "./lib.mjs";

const REG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/";
export const PATHS = { data: REG + "🔣️.json", schema: REG + "🧬️schema/🔣️.json", ts: REG + "🟦️.ts", tests: REG + "🧪️tests/🟦️.ts" };

export const GERMAN = {
  "Routes the neutral build operations; every operation provisions its own dependencies (see requires in the operations catalogue).": "Verteilt die werkzeugneutralen Build-Operationen; jede Operation stellt ihre Abhängigkeiten selbst bereit (siehe requires im Operationskatalog).",
  "Closure is free of installed packages; the work runs through cargo.": "Der Abschluss braucht keine installierten Pakete; die Arbeit läuft über cargo.",
  "The module reaches the TypeScript compiler through the normalization sources.": "Das Modul erreicht den TypeScript-Compiler über die Normalisierungsquellen.",
  "Closure is free of installed packages.": "Der Abschluss braucht keine installierten Pakete.",
  "Provisions the checkout itself and runs inside a half-installed workspace.": "Richtet den Checkout selbst ein und läuft in einem halb installierten Workspace.",
  "Reads the dashboard declaration and writes configuration files only.": "Liest die Dashboard-Deklaration und schreibt nur Konfigurationsdateien.",
  "The module reaches the TypeScript compiler and the JSON schema library.": "Das Modul erreicht den TypeScript-Compiler und die JSON-Schema-Bibliothek.",
  "Closure is free of installed packages; delegated work goes through bun repo.": "Der Abschluss braucht keine installierten Pakete; delegierte Arbeit läuft über bun repo.",
  "Delegates to bun repo run, which provisions itself.": "Delegiert an bun repo run, das sich selbst bereitstellt.",
  "Closure is free of installed packages; the scaffolder only writes files.": "Der Abschluss braucht keine installierten Pakete; der Generator schreibt nur Dateien.",
  "Runs the installed test runner and cargo for schema contracts.": "Führt den installierten Testläufer und cargo für Schema-Verträge aus.",
  "Runs the installed code-style tools.": "Führt die installierten Code-Stil-Werkzeuge aus.",
  "The verification lanes import the TypeScript compiler, the JSON schema library and other installed packages.": "Die Prüfspuren importieren den TypeScript-Compiler, die JSON-Schema-Bibliothek und weitere installierte Pakete.",
  "Runs the installed formatter.": "Führt den installierten Formatierer aus.",
  "The test domain loads the TypeScript compiler, the JSON schema library and other installed packages.": "Die Testdomäne lädt den TypeScript-Compiler, die JSON-Schema-Bibliothek und weitere installierte Pakete.",
  "The benchmarks run through the test domain.": "Die Benchmarks laufen über die Testdomäne.",
  "Closure is free of installed packages; the work runs through the native owner.": "Der Abschluss braucht keine installierten Pakete; die Arbeit läuft über den nativen Eigentümer.",
  "Shares the release module, which reaches the TypeScript compiler.": "Teilt sich das Release-Modul, das den TypeScript-Compiler erreicht.",
  "clean test delegates to the test domain, which loads installed packages.": "clean test delegiert an die Testdomäne, die installierte Pakete lädt.",
  "Git only; runs from hooks on a fresh clone.": "Nur Git; läuft aus Hooks auf einem frischen Klon.",
};

export function build() {
  const data = JSON.parse(load(PATHS.data));
  for (const command of data.commands) {
    const de = GERMAN[command.reason];
    if (!de) throw Error("no German reason for " + command.id + ": " + command.reason);
    command.reason = { en: command.reason, de };
  }
  set(PATHS.data, JSON.stringify({ $schema: "./🧬️schema/🔣️.json", ...data }, null, 2) + "\n");
  edit(PATHS.schema, `    "version": { "const": 1 },
    "languages"`, `    "$schema": { "type": "string" },
    "version": { "const": 1 },
    "languages"`);
  edit(PATHS.schema, `        "reason": { "type": "string", "minLength": 1 },`, `        "reason": { "$ref": "#/definitions/LocalizedText" },`);
  edit(PATHS.ts, `requires: RootCommandRequiresV1; reason: string;`, `requires: RootCommandRequiresV1; reason: RootCommandTextV1;`);
  set(PATHS.tests, load(PATHS.tests).replace(/\n*$/, "\n") + `
test("the registry data points at its schema and every reason is written in every declared language", () => {
  expect(join(import.meta.dir, "..", data.$schema)).toBe(join(import.meta.dir, "..", "🧬️schema/🔣️.json"));
  expect(JSON.parse(readFileSync(join(import.meta.dir, "..", data.$schema), "utf8"))).toEqual(schema);
  for (const command of registry.commands) for (const language of registry.languages) expect(command.reason[language]?.length, \`\${command.id} \${language}\`).toBeGreaterThan(8);
});
`);
}
