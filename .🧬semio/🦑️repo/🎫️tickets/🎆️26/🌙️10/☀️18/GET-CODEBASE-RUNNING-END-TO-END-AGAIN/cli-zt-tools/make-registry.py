"""Authors the root command registry data (staged/02-registry.json)."""
import json, os
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
here = os.path.dirname(os.path.abspath(__file__))
t = lambda en, de: {"en": en, "de": de}
labels = {
    "tagline": t("Monorepo command router. Commands that need installed dependencies install them first.", "Befehlsrouter des Monorepos. Befehle, die installierte Abhängigkeiten brauchen, installieren sie zuerst."),
    "usage": t("Usage", "Verwendung"),
    "commands": t("Commands", "Befehle"),
    "requires": t("Needs", "Benötigt"),
    "requiresNone": t("nothing; runs on a fresh clone", "nichts; läuft auf einem frischen Klon"),
    "requiresDependencies": t("the locked dependencies, installed automatically on first use", "die gesperrten Abhängigkeiten, bei der ersten Nutzung automatisch installiert"),
    "legend": t("* installs the locked dependencies first when they are missing or changed", "* installiert zuerst die gesperrten Abhängigkeiten, wenn sie fehlen oder geändert sind"),
    "provisioning": t("Command {command} needs the locked dependencies; running `bun repo dependencies ensure` (minutes on a fresh clone, Ctrl+C cancels).", "Befehl {command} braucht die gesperrten Abhängigkeiten; führe `bun repo dependencies ensure` aus (auf einem frischen Klon Minuten, Strg+C bricht ab)."),
    "provisioned": t("Dependencies ready after {seconds} s; starting {command}.", "Abhängigkeiten nach {seconds} s bereit; starte {command}."),
    "provisionFailed": t("Command {command} needs the locked dependencies and `bun repo dependencies ensure` failed: {reason}. Fix the cause, run `bun repo dependencies ensure`, then retry.", "Befehl {command} braucht die gesperrten Abhängigkeiten und `bun repo dependencies ensure` ist fehlgeschlagen: {reason}. Ursache beheben, `bun repo dependencies ensure` ausführen und erneut versuchen."),
    "unknownCommand": t("Unknown command: {command}", "Unbekannter Befehl: {command}"),
}
rows = [
    ("repo", f"{L}/🏗️build/📜️script.ts", "RepoScript", "none", "Routes the neutral build operations; every operation provisions its own dependencies (see requires in the operations catalogue).", "repo [--tool=nx|bazel] <operation> [options] [-- <arguments>]", t("Tool-neutral build entry (Nx or Bazel): run, graph, cache, tools, dependencies.", "Werkzeugneutraler Build-Einstieg (Nx oder Bazel): run, graph, cache, tools, dependencies.")),
    ("os", f"{L}/🎛️commands/🧩️os/🟦️.ts", "OsScript", "none", "Closure is free of installed packages; the work runs through cargo.", "os run <bundle>.studio [--node <id>] [--watch] [--dry]", t("Headless OS studio: computes a workflow without a UI.", "Headless-OS-Studio: berechnet einen Workflow ohne Oberfläche.")),
    ("components", f"{L}/🎛️commands/🚀️release/🟦️.ts", "ComponentsScript", "dependencies", "The module reaches the TypeScript compiler through the normalization sources.", "components describe|iterate|profile-independence …", t("Describes, iterates and proves plugin components.", "Beschreibt, iteriert und prüft Plugin-Komponenten.")),
    ("semio", f"{L}/🎛️commands/🧩️os/🟦️.ts", "SemioScript", "none", "Closure is free of installed packages; the work runs through cargo.", "semio inspect|verify|open|convert <file>", t("Universal processor for .semio files: inspect, verify, open, convert.", "Universeller Prozessor für .semio-Dateien: inspect, verify, open, convert.")),
    ("examples", f"{L}/🎛️commands/📚️examples/🟦️.ts", "ExamplesScript", "none", "Closure is free of installed packages.", None, t("Lists and verifies plugin example units.", "Listet Plugin-Beispieleinheiten auf und prüft sie.")),
    ("setup", f"{L}/🎛️commands/🔧️setup/🟦️.ts", "SetupScript", "none", "Provisions the checkout itself and runs inside a half-installed workspace.", None, t("Provisions this checkout: dependencies, tools, native stores.", "Richtet diesen Checkout ein: Abhängigkeiten, Werkzeuge, native Speicher.")),
    ("agents", f"{L}/🤖️agent-clients/🎮️command/🟦️.ts", "AgentsScript", "none", "Reads the dashboard declaration and writes configuration files only.", "agents write|check", t("Writes or checks the MCP configuration of the agent clients.", "Schreibt oder prüft die MCP-Konfiguration der Agent-Clients.")),
    ("dev", f"{L}/🎛️commands/🛠️dev/🟦️.ts", "DevScript", "dependencies", "The module reaches the TypeScript compiler and the JSON schema library.", "dev storybook|mcp …", t("Starts development servers: Storybook, MCP.", "Startet Entwicklungsserver: Storybook, MCP.")),
    ("generate", f"{L}/🎛️commands/🏭️generate/🟦️.ts", "GenerateScript", "none", "Closure is free of installed packages; delegated work goes through bun repo.", "generate config [--check]|taxonomy|plugin-glue|scale-fixture", t("Regenerates derived files: config, taxonomy, plugin glue, scale fixture.", "Erzeugt abgeleitete Dateien neu: Konfiguration, Taxonomie, Plugin-Glue, Skalierungs-Fixture.")),
    ("scale-fixture", f"{L}/🎛️commands/🧱️scale-fixture/🟦️.ts", "ScaleFixtureScript", "none", "Delegates to bun repo run, which provisions itself.", "scale-fixture check", t("Checks the seeded 50x50 scale fixture against its generator.", "Prüft das gesäte 50x50-Skalierungs-Fixture gegen seinen Generator.")),
    ("new", f"{L}/🏗️authoring/🎮️command/🟦️.ts", "CleanMechanismNewScript", "none", "Closure is free of installed packages; the scaffolder only writes files.", "new artifact|standard|subset|mutation …", t("Scaffolds an artifact, standard, subset or mutation.", "Legt ein Artefakt, einen Standard, ein Subset oder eine Mutation an.")),
    ("schema", f"{L}/🎛️commands/🧬️schema/🟦️.ts", "SchemaScript", "dependencies", "Runs the installed test runner and cargo for schema contracts.", None, t("Schema catalogue generation, invariant checks and the generated index.", "Schema-Katalog erzeugen, Invarianten prüfen und generierten Index pflegen.")),
    ("lint", f"{L}/🎛️commands/🧹️code-style/🟦️.ts", "LintScript", "dependencies", "Runs the installed code-style tools.", None, t("Checks code style with the installed tools.", "Prüft den Code-Stil mit den installierten Werkzeugen.")),
    ("verify", f"{L}/🎛️commands/✅️verify/🌳️root/🟦️.ts", "RootVerifyScript", "dependencies", "The verification lanes import the TypeScript compiler, the JSON schema library and other installed packages.", "verify <lane> [arguments]", t("Runs the repository verification lanes and gates.", "Führt die Verifikationsbahnen und Gates des Repositorys aus.")),
    ("format", f"{L}/🎛️commands/🧹️code-style/🟦️.ts", "FormatScript", "dependencies", "Runs the installed formatter.", None, t("Formats the workspace with the installed formatter.", "Formatiert den Workspace mit dem installierten Formatierer.")),
    ("test", f"{L}/🎛️commands/🧪️test/🚦️root/🟦️.ts", "RootTestScript", "dependencies", "The test domain loads the TypeScript compiler, the JSON schema library and other installed packages.", "test entrypoint | test <phase> [level]", t("Runs the repository test platform.", "Führt die Testplattform des Repositorys aus.")),
    ("bench", f"{L}/🎛️commands/🧪️test/🟦️.ts", "BenchScript", "dependencies", "The benchmarks run through the test domain.", "bench plugins [--renderer native|react|wgpu]", t("Runs the repository benchmarks.", "Führt die Benchmarks des Repositorys aus.")),
    ("stdio", f"{L}/🎛️commands/🗄️stdio/🟦️.ts", "StdioScript", "none", "Closure is free of installed packages; delegated work goes through bun repo.", None, t("Runs the stdio ledger and its structural gates.", "Führt das stdio-Ledger und seine strukturellen Gates aus.")),
    ("build", f"{L}/🏗️build/📜️script.ts", "BuildScript", "none", "Delegates to bun repo run, which provisions itself.", "build [canvas fonts|storybook|assets|repo-cli|repo-server|repo-vscode]", t("Builds workspace slices: canvas fonts, Storybook, assets, repo clients.", "Baut Workspace-Teile: Canvas-Schriften, Storybook, Assets, Repo-Clients.")),
    ("release", f"{L}/🎛️commands/🚀️release/🟦️.ts", "ReleaseScript", "dependencies", "The module reaches the TypeScript compiler through the normalization sources.", None, t("Compiles the release generation of the components.", "Kompiliert die Release-Generation der Komponenten.")),
    ("cpp", f"{L}/🏗️build/🛠️cmake/🟦️.ts", "CppScript", "none", "Closure is free of installed packages; the work runs through the native owner.", "cpp [setup|configure|build|test|all]", t("Configures, builds and tests the C++ presets.", "Konfiguriert, baut und testet die C++-Presets.")),
    ("publish", f"{L}/🎛️commands/🚀️release/🟦️.ts", "PublishScript", "dependencies", "Shares the release module, which reaches the TypeScript compiler.", "publish os-hub|os-mcp", t("Publishes the built deliverables.", "Veröffentlicht die gebauten Liefergegenstände.")),
    ("clean", f"{L}/🧼️workspace-cleanup/🎮️command/🟦️.ts", "CleanScript", "dependencies", "clean test delegates to the test domain, which loads installed packages.", None, t("Removes generated test state and other marked generated output.", "Entfernt erzeugten Teststatus und andere markierte erzeugte Ausgaben.")),
    ("micro-commit", f"{L}/🎛️commands/📝️commit/🟦️.ts", "MicroCommitScript", "none", "Git only; runs from hooks on a fresh clone.", None, t("Writes micro-commit templates from the staged index.", "Schreibt Mikro-Commit-Vorlagen aus dem bereitgestellten Index.")),
    ("commit", f"{L}/🎛️commands/📝️commit/🟦️.ts", "CommitScript", "none", "Git only; runs from hooks on a fresh clone.", None, t("Bundles micro-commits into a signed squash commit.", "Bündelt Mikro-Commits zu einem signierten Squash-Commit.")),
]
commands = []
for id_, module, export, requires, reason, synopsis, summary in rows:
    row = {"id": id_, "module": module, "export": export, "requires": requires, "reason": reason}
    if synopsis: row["synopsis"] = synopsis
    row["summary"] = summary
    commands.append(row)
data = {"version": 1, "languages": ["en", "de"], "labels": labels, "commands": commands}
out = os.path.join(here, "staged", "02-registry.json")
open(out, "w", encoding="utf-8").write(json.dumps(data, ensure_ascii=False, indent=2) + "\n")
print(len(commands), out)
