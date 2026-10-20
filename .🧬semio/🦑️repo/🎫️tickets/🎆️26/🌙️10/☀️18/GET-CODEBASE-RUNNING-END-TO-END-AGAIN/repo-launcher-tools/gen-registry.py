"""Generates the launcher runtime registry from the pinned bun release (single reading of the existing pins) and the authored EN/DE messages. Usage: gen-registry.py <repo-root> <out.json>"""
import json, sys, os
root, out = sys.argv[1], sys.argv[2]
bun_schema = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json")
release = json.load(open(bun_schema, encoding="utf-8"))["definitions"]["Release"]["const"]
T = lambda en, de: {"en": en, "de": de}
messages = {
 "runtimeUnknown": T("Runtime {name} is not declared in the registry (declared: {declared}).", "Die Laufzeit {name} ist in der Registratur nicht deklariert (deklariert: {declared})."),
 "runtimeUnsupported": T("{runtime} {version} has no pinned distribution for {platform}/{arch}{libc}.", "Für {runtime} {version} gibt es keine gepinnte Distribution für {platform}/{arch}{libc}."),
 "entryUnknown": T("The repository implementation has no entry {name} (declared: {declared}).", "Die Repository-Implementierung hat keinen Einstieg {name} (deklariert: {declared})."),
 "entryMissing": T("The repository implementation entry is missing: {path}", "Der Einstieg der Repository-Implementierung fehlt: {path}"),
 "provisioning": T("Provisioning {runtime} {version} for {platform}/{arch}: first use, {megabytes} MiB from {source}.", "Bereitstellung von {runtime} {version} für {platform}/{arch}: erste Nutzung, {megabytes} MiB von {source}."),
 "downloading": T("Downloading {runtime} {version}: {percent}% ({done} of {total} MiB).", "Lade {runtime} {version}: {percent}% ({done} von {total} MiB)."),
 "waitingForPeer": T("Another process is provisioning {runtime} {version}; waiting for it.", "Ein anderer Prozess stellt {runtime} {version} bereit; warte darauf."),
 "ready": T("{runtime} {version} is ready: {path}", "{runtime} {version} ist bereit: {path}"),
 "downloadFailed": T("Download of {url} failed: {reason}. Retry, point SEMIO_RUNTIME_MIRROR at a mirror of {source}, or pre-seed {directory} with the verified distribution.", "Der Download von {url} ist fehlgeschlagen: {reason}. Wiederhole ihn, richte SEMIO_RUNTIME_MIRROR auf einen Spiegel von {source} oder lege die geprüfte Distribution in {directory} vor."),
 "integrityFailed": T("The {what} of {archive} is {actual}, the registry pins {expected}. Nothing was installed.", "Die {what} von {archive} ist {actual}, die Registratur pinnt {expected}. Es wurde nichts installiert."),
 "extractFailed": T("The executable {member} cannot be extracted from {archive}: {reason}", "Die ausführbare Datei {member} lässt sich nicht aus {archive} entpacken: {reason}"),
 "probeFailed": T("The provisioned {runtime} reports {actual}, the registry pins {expected}. Nothing was installed.", "Die bereitgestellte Laufzeit {runtime} meldet {actual}, die Registratur pinnt {expected}. Es wurde nichts installiert."),
 "pathTooLong": T("{path} is longer than {limit} characters; use a shorter SEMIO_RUNTIME_CACHE.", "{path} ist länger als {limit} Zeichen; verwende ein kürzeres SEMIO_RUNTIME_CACHE."),
 "cacheUnavailable": T("No cache directory is available: {reason}. Set SEMIO_RUNTIME_CACHE.", "Es gibt kein Cache-Verzeichnis: {reason}. Setze SEMIO_RUNTIME_CACHE."),
 "cancelled": T("Cancelled.", "Abgebrochen."),
 "handOffFailed": T("Cannot start {path}: {reason}", "{path} lässt sich nicht starten: {reason}"),
 "domainNeedsInstall": T("repo {verb} needs the full native command, this is the launcher only. Install it once: repo run repo-mcp:install", "repo {verb} braucht den vollständigen nativen Befehl, dies ist nur der Launcher. Einmalig installieren: repo run repo-mcp:install"),
 "helpUsage": T("repo [--tool=nx|bazel] <command> [arguments]   Repository command: build, test, dependencies, tools, cache and more.", "repo [--tool=nx|bazel] <Befehl> [Argumente]   Repository-Befehl: Build, Test, Abhängigkeiten, Werkzeuge, Cache und mehr."),
 "helpBuild": T("Commands: {verbs}", "Befehle: {verbs}"),
 "helpRuntime": T("The JavaScript runtime ({runtime} {version}) is provisioned on first use from the pinned registry; no manual install is needed.", "Die JavaScript-Laufzeit ({runtime} {version}) wird bei der ersten Nutzung aus der gepinnten Registratur bereitgestellt; keine manuelle Installation nötig."),
 "helpVersion": T("repo --version    Versions of the launcher, the runtime and the implementation.", "repo --version    Versionen von Launcher, Laufzeit und Implementierung."),
 "versionRuntimeReady": T("runtime {runtime} {version}: ready, {path}", "Laufzeit {runtime} {version}: bereit, {path}"),
 "versionRuntimeAbsent": T("runtime {runtime} {version}: not provisioned yet, fetched on first use", "Laufzeit {runtime} {version}: noch nicht bereitgestellt, wird bei der ersten Nutzung geladen"),
 "versionImplementation": T("implementation {version} ({source})", "Implementierung {version} ({source})"),
 "versionImplementationUnknown": T("implementation: version unreadable ({source})", "Implementierung: Version nicht lesbar ({source})"),
 "languageRefusal": T("No language selected for texts. Pass --lang={choice}, set SEMIO_LANGUAGE={choice}, or run with a locale such as LANG=en_US.UTF-8 (resolution order: --lang, SEMIO_LANGUAGE, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, the operating system language list).", "Keine Sprache für Texte gewählt. Übergib --lang={choice}, setze SEMIO_LANGUAGE={choice} oder starte mit einem Locale wie LANG=de_DE.UTF-8 (Reihenfolge: --lang, SEMIO_LANGUAGE, LANGUAGE, LC_ALL, LC_MESSAGES, LANG, Sprachliste des Betriebssystems)."),
 "languageUnknownFlag": T("Unknown language: {value} ({choice})", "Unbekannte Sprache: {value} ({choice})"),
 "languageUnknownVariable": T("{name} names a language that is not offered: {value} ({choice})", "{name} nennt eine nicht angebotene Sprache: {value} ({choice})"),
}
distributions = [{"platform": r["platform"], "arch": r["arch"], "libc": "musl" if r["musl"] else ("gnu" if r["platform"] == "linux" else "any"), "archive": r["archive"], "bytes": r["bytes"], "sha256": r["sha256"]} for r in release["distributions"]]
registry = {
 "$schema": "./🧬️schema/🔣️.json",
 "contract": 1,
 "languages": ["en", "de"],
 "language": {"protocol": "🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🔣️.json"},
 "routing": {"catalogue": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json"},
 "selected": "bun",
 "runtimes": {
  "bun": {
   "version": release["version"],
   "executable": {"unix": "bun", "windows": "bun.exe"},
   "probe": {"arguments": ["--version"]},
   "download": {"base": release["release"], "format": "zip", "member": "{stem}/{executable}", "maximumExtractedBytes": 268435456},
   "launch": {"before": [], "options": {"watch": ["--watch"], "cwd": ["--cwd", "{value}"]}, "environment": {}},
   "distributions": distributions,
  }
 },
 "implementation": {
  "version": {"file": "🧰️framework/🛍️products/🦑️repo/package.json", "key": "version"},
  "entries": {
   "build": {"runtime": None, "script": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts", "prefix": ["repo"]},
   "workspace": {"runtime": None, "script": "📜️script.ts", "prefix": []},
  },
 },
 "cache": {"environment": "SEMIO_RUNTIME_CACHE", "mirrorEnvironment": "SEMIO_RUNTIME_MIRROR", "runtimeEnvironment": "SEMIO_RUNTIME", "toolsPartition": [".cache", "b", "t", "m"], "user": ["semio", "runtimes"], "maximumPathLength": 256},
 "messages": messages,
}
for entry in registry["implementation"]["entries"].values(): del entry["runtime"]
json.dump(registry, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
open(out, "a", encoding="utf-8").write("\n")
