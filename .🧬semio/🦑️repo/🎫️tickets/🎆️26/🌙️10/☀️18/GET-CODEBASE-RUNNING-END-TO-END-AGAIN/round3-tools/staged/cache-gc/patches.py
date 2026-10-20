import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


def staged(name):
    with open(os.path.join(STAGED, name), encoding="utf-8") as handle:
        return handle.read()


class Drift(Exception):
    pass


def replace_once(text, old, new, label):
    if new in text:
        return text
    count = text.count(old)
    if count != 1:
        raise Drift("%s: anchor found %d times" % (label, count))
    return text.replace(old, new)


def apply(text, rows, label):
    for index, (old, new) in enumerate(rows):
        text = replace_once(text, old, new, "%s[%d]" % (label, index))
    return text


CACHING_INDEX = [
    (
        'import { join } from "node:path";\n',
        'import { join } from "node:path";\nimport {requireDeclaredCacheSegmentsV1} from "./💾️storage/🧾️ledger/🟦️.ts";\n',
    ),
    (
        '/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`). */\n'
        'export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string {\n'
        '  return join(',
        '/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`); it refuses every home the cache ledger does not declare. */\n'
        'export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string {\n'
        '  requireDeclaredCacheSegmentsV1(segments);\n'
        '  return join(',
    ),
]

CACHING_POLICY = [
    ('    "cargo": {\n      "budgetBytes": 85899345920,', '    "cargo": {\n      "budgetBytes": 13957595136,'),
    ('      "incremental": {\n        "budgetBytes": 25769803776,', '      "incremental": {\n        "budgetBytes": 6442450944,'),
]

NX_JSON = [
    (
        '  "cacheDirectory": ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts",\n  "analytics": false',
        '  "cacheDirectory": ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts",\n  "maxCacheSize": "3GB",\n  "analytics": false',
    ),
]

PROVIDERS_TEST = [
    (
        'expect(repoCacheDirectory("/repository", "a", "b")).toBe(join(repositoryMetaDirectory("/repository"), "⚡️cache", "a", "b"));',
        'expect(repoCacheDirectory("/repository", "go", "b")).toBe(join(repositoryMetaDirectory("/repository"), "⚡️cache", "go", "b"));',
    ),
]

BUILD_SCRIPT = [
    (
        '    if(command==="cache"&&["serve","remote"].includes(resolved.args[1]??"")){',
        '    if(command==="cache"&&["status","gc"].includes(resolved.args[1]??"")&&!resolved.args.includes("--tasks")){\n'
        '      const {runCacheLedgerV1}=await import("./🧹️cache-gc/⌨️command/🟦️.ts");\n'
        '      await runCacheLedgerV1(root,resolved.args.slice(1),invocation,phaseControl);\n'
        '      return;\n'
        '    }\n'
        '    if(command==="cache"&&["serve","remote"].includes(resolved.args[1]??"")){',
    ),
]

OPERATIONS = [
    (
        ' if(args.length!==1||operation!=="status"&&operation!=="clear")throw Error("Expected repo cache status|clear");',
        ' if(operation!=="status"&&operation!=="clear"||args.length!==1&&!(operation==="status"&&args.length===2&&args[1]==="--tasks"))throw Error("Expected repo cache status|clear (status accepts --tasks)");',
    ),
]

CATALOG = [
    (
        '      "name": "--dry-run",\n      "value": "none",\n      "operations": ["run", "run-many", "affected", "exec"],\n      "description": {\n'
        '        "en": "Print the task graph as a DryRunPlan on stdout and execute nothing; the same for every tool.",\n'
        '        "de": "Gibt den Task-Graphen als DryRunPlan auf stdout aus und führt nichts aus; für jedes Werkzeug gleich."\n      }',
        '      "name": "--dry-run",\n      "value": "none",\n      "operations": ["run", "run-many", "affected", "exec", "cache"],\n      "description": {\n'
        '        "en": "Print the task graph as a DryRunPlan on stdout and execute nothing; the same for every tool. With cache gc: list what would be removed and remove nothing.",\n'
        '        "de": "Gibt den Task-Graphen als DryRunPlan auf stdout aus und führt nichts aus; für jedes Werkzeug gleich. Mit cache gc: listet auf, was entfernt würde, und entfernt nichts."\n      }',
    ),
    (
        '      "name": "--json",\n      "value": "none",\n      "operations": ["graph", "show", "incremental", "capacity"],\n      "description": { "en": "Print lists as JSON instead of one entry per line.", "de": "Gibt Listen als JSON statt eines Eintrags je Zeile aus." }\n    },\n',
        '      "name": "--json",\n      "value": "none",\n      "operations": ["graph", "show", "incremental", "capacity", "cache"],\n      "description": { "en": "Print lists as JSON instead of one entry per line.", "de": "Gibt Listen als JSON statt eines Eintrags je Zeile aus." }\n    },\n'
        '    {\n      "name": "--area",\n      "value": "required",\n      "valueName": "id[,id]",\n      "operations": ["cache"],\n'
        '      "description": { "en": "cache status|gc: limit the report or the collection to these ledger areas (or retired, undeclared).", "de": "cache status|gc: beschränkt Bericht oder Einsammeln auf diese Bereiche des Verzeichnisses (oder retired, undeclared)." }\n    },\n'
        '    {\n      "name": "--tasks",\n      "value": "none",\n      "operations": ["cache"],\n'
        '      "description": { "en": "cache status: print the selected tool\'s task cache (a CacheStatus) instead of the cache ledger.", "de": "cache status: gibt den Task-Cache des gewählten Werkzeugs (einen CacheStatus) statt der Cache-Bilanz aus." }\n    },\n',
    ),
    (
        '      "synopsis": "cache status|clear|remote status|serve",\n      "summary": {\n'
        '        "en": "Show or clear the selected tool\'s local task cache, show the remote cache contract, or serve a remote cache.",\n'
        '        "de": "Zeigt oder leert den lokalen Task-Cache des gewählten Werkzeugs, zeigt den Remote-Cache-Vertrag oder stellt einen Remote-Cache bereit."\n      },',
        '      "synopsis": "cache status|gc|clear|remote status|serve",\n      "summary": {\n'
        '        "en": "Show and collect every declared cache home with its budget, clear the selected tool\'s local task cache, show the remote cache contract, or serve a remote cache.",\n'
        '        "de": "Zeigt und sammelt jede deklarierte Cache-Heimat mit ihrem Budget, leert den lokalen Task-Cache des gewählten Werkzeugs, zeigt den Remote-Cache-Vertrag oder stellt einen Remote-Cache bereit."\n      },',
    ),
    (
        '        {\n          "name": "status",\n          "synopsis": "cache status",\n          "summary": {\n'
        '            "en": "Print a CacheStatus: entries, bytes and, for Bazel, the recorded build activity.",\n'
        '            "de": "Gibt einen CacheStatus aus: Einträge, Bytes und, für Bazel, die aufgezeichnete Build-Aktivität."\n          }\n        },\n',
        '        {\n          "name": "status",\n          "synopsis": "cache status [--tasks]",\n          "summary": {\n'
        '            "en": "Print the cache ledger: every partition and area with its use against its budget, the retired roots and undeclared entries still on disk; with --tasks, a CacheStatus of the selected tool instead.",\n'
        '            "de": "Gibt die Cache-Bilanz aus: jede Partition und jeden Bereich mit Nutzung gegen Budget, die noch vorhandenen stillgelegten Wurzeln und nicht deklarierten Einträge; mit --tasks stattdessen einen CacheStatus des gewählten Werkzeugs."\n          }\n        },\n'
        '        {\n          "name": "gc",\n          "synopsis": "cache gc [--dry-run] [--area=<id>]",\n          "summary": {\n'
        '            "en": "Collect the cache: remove retired roots, units unused longer than their area allows and the oldest units beyond an area\'s budget, and entries nobody declared; it never removes a unit that is held by a lease or used recently.",\n'
        '            "de": "Sammelt den Cache ein: entfernt stillgelegte Wurzeln, länger als erlaubt unbenutzte Einheiten, die ältesten Einheiten über dem Budget eines Bereichs und nicht deklarierte Einträge; eine durch Lease gehaltene oder kürzlich benutzte Einheit wird nie entfernt."\n          }\n        },\n',
    ),
    (
        '      "flags": ["--port", "--host", "--directory", "--max-bytes"],',
        '      "flags": ["--port", "--host", "--directory", "--max-bytes", "--json", "--dry-run", "--area", "--tasks"],',
    ),
    (
        '      "examples": [\n        { "command": "repo cache status --tool=bazel",',
        '      "examples": [\n'
        '        { "command": "repo cache status", "description": { "en": "Show every cache partition and area against its budget.", "de": "Zeigt jede Cache-Partition und jeden Bereich gegen sein Budget." } },\n'
        '        { "command": "repo cache gc --dry-run", "description": { "en": "List what the collector would remove.", "de": "Listet auf, was der Einsammler entfernen würde." } },\n'
        '        { "command": "repo cache status --tasks --tool=bazel",',
    ),
    (
        '        "status prints a CacheStatus, clear prints a CacheClear; both stop at the next checkpoint when the caller cancels and publish progress while walking or removing entries.",',
        '        "status prints the cache ledger of schema cache-ledger: every fixed partition, every area inside a partition, every area of the repository and native cache roots with its declared budget, and what is retired or undeclared; gc collects it. Both publish progress, stop at the next checkpoint when the caller cancels, and take no lease for the report; gc takes an area\'s lease exclusively without waiting and never removes a unit whose SQLite lease file is locked, whose queue ticket belongs to a living process or that was used within the quiet time of its area. Areas kept by an owner (Nx at maxCacheSize, Cargo, Bazel, test evidence, the dashboard) are reported, not collected here.",\n'
        '        "status --tasks prints a CacheStatus, clear prints a CacheClear; both stop at the next checkpoint when the caller cancels and publish progress while walking or removing entries.",',
    ),
]

import re as _re


def _json_flag(text):
    pattern = _re.compile(r'("name": "--json",\n\s+"value": "none",\n\s+"operations": \[)([^\]]*)(\])')
    match = pattern.search(text)
    if match is None:
        raise Drift("catalog: --json flag not found")
    if '"cache"' not in match.group(2):
        text = text[:match.start(2)] + match.group(2) + ', "cache"' + text[match.end(2):]
    if '"name": "--area"' not in text:
        tail = CATALOG[1][1][CATALOG[1][1].index('    {\n      "name": "--area"'):]
        end = text.index("\n    },\n", match.start()) + len("\n    },\n")
        text = text[:end] + tail + text[end:]
    return text


CATALOG_WITHOUT_JSON = [pair for index, pair in enumerate(CATALOG) if index != 1]


EDITORS = {
    "caching-index": lambda text: apply(text, CACHING_INDEX, "caching-index"),
    "caching-policy": lambda text: apply(text, CACHING_POLICY, "caching-policy"),
    "nx-json": lambda text: apply(text, NX_JSON, "nx-json"),
    "providers-test": lambda text: apply(text, PROVIDERS_TEST, "providers-test"),
    "build-script": lambda text: apply(text, BUILD_SCRIPT, "build-script"),
    "operations": lambda text: apply(text, OPERATIONS, "operations"),
    "catalog": lambda text: apply(_json_flag(text), CATALOG_WITHOUT_JSON, "catalog"),
}
