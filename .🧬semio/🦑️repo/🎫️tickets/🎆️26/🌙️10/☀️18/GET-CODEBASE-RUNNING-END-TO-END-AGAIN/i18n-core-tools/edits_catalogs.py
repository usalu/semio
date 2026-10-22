#!/usr/bin/env python3
"""Step 2 of the i18n-core build: the first catalogs (verify progress, commit output, bootstrap progress) and the CLI boundary."""
import json, os, re
from lib import *
from mig import *
from scopes import CATALOGS

HERE_SPECS = os.path.join(HERE, "specs")
CATALOG_MODULE = "📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts"
ERROR_MODULE = "📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts"
PROGRESS_MODULE = "📦️packages/🟦️typescript/🏗️build/🗣️language/📣️progress/🟦️.ts"
LANGUAGE_SCHEMA = "🏗️build/🗣️language/📖️catalog/🧬️schema/🔣️.json"


def texts_module(name, c, texts_ts, ts_import):
    say, refuse, handle = c["say"], c.get("refuse"), c["handle"]
    title = name[:1].upper() + name[1:]
    out = '''import { bindCatalogV1, type TextValuesV1 } from "%s";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "%s";
import { sayV1, type ProgressLevelV1 } from "%s";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of %s in English and German. */
export const %s = bindCatalogV1("%s", catalog);

/** 🔑️ Every message key of this catalog. */
export type %sTextKeyV1 = keyof typeof catalog.messages;

/** 📢️ Says one line of this catalog as data; the CLI boundary renders it in the resolved language. */
export const %s = (key: %sTextKeyV1, values: TextValuesV1 = {}, level: ProgressLevelV1 = "info"): void => sayV1(%s, key, values, level);

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const %s = (code: RepoErrorCodeV1, key: %sTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(%s, code, key, values, options);
''' % (ts_import(CATALOG_MODULE), ts_import(ERROR_MODULE), ts_import(PROGRESS_MODULE), c["doc"], handle, name, title, say, title, handle, refuse, title, handle)
    if refuse is None:
        out = out.split("\n/** 🚨️ Builds a refusal")[0] + "\n"
        out = out.replace('import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "%s";\n' % ts_import(ERROR_MODULE), "")
    if name == "commit":
        out = out.replace('import { sayV1, type ProgressLevelV1 }', 'import { reportRepoErrorV1, sayV1, type ProgressLevelV1 }')
        out += '''
/** 🖨️ Prints a refusal caught inside the commit workflow: a `RepoErrorV1` is rendered in the resolved language, any other error by its message. */
export function reportCommitErrorV1(error: unknown): void {
  if (!reportRepoErrorV1(error)) console.error(error instanceof Error ? error.message : String(error));
}
'''
    return out


def test_module(name, c, owner_dir, same_keys, ts_import):
    title = name[:1].upper() + name[1:]
    return '''import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import { CATALOG_LANGUAGES_V1, defineCatalogV1, placeholdersV1, textV1 } from "%s";
import schema from "%s" with { type: "json" };
import catalog from "../🔣️.json" with { type: "json" };
import { %s } from "../🟦️.ts";

const LANGUAGE_NEUTRAL: readonly string[] = %s;
const owner = join(import.meta.dir, "..", "%s");
const sources = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? (["🧪️tests", "📖️texts", "node_modules", ".cache"].includes(entry.name) ? [] : sources(join(directory, entry.name))) : /\\.(?:ts|mjs)$/u.test(entry.name) ? [join(directory, entry.name)] : []);

test("the catalog satisfies the strict independent validator and the language contract", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(schema, catalog), JSON.stringify(ajv.errors)).toBe(true);
  expect(defineCatalogV1(catalog).languages).toEqual([...CATALOG_LANGUAGES_V1]);
  expect(%s.id).toBe("%s");
});

test("every message renders in both languages with exactly its placeholders and differs between them unless it is language-neutral", () => {
  for (const [key, leaf] of Object.entries(catalog.messages)) {
    const names = placeholdersV1(leaf.en), values = Object.fromEntries(names.map((name) => [name, "v" + name]));
    for (const language of CATALOG_LANGUAGES_V1) {
      const text = textV1(%s.catalog, key, language, values);
      for (const name of names) expect(text, `${key} ${language}`).toContain("v" + name);
    }
    expect(leaf.en === leaf.de, key).toBe(LANGUAGE_NEUTRAL.includes(key));
  }
});

test("every message is used by the code it belongs to and every key the code names exists", () => {
  const code = sources(owner).map((path) => readFileSync(path, "utf8")).join("\\n");
  const keys = Object.keys(catalog.messages);
  for (const key of keys) expect(code.includes(JSON.stringify(key)), `${key} is unused`).toBe(true);
  for (const match of code.matchAll(/\\b(?:%s|%s)\\(\\s*(?:"(?:usage|invalid|missing|refused|failed|breach|stale|unsupported)"\\s*,\\s*)?("(?:[^"\\\\]|\\\\.)*")/gu)) expect(keys, match[1]).toContain(JSON.parse(match[1]!));
});
''' % (ts_import(CATALOG_MODULE), ts_import(LANGUAGE_SCHEMA), c["handle"], json.dumps(sorted(same_keys), ensure_ascii=False), owner_dir, c["handle"], name, c["handle"], c["say"], c.get("refuse") or "refuseNoneV1")


def boundary(tree):
    rel = L + "/🏗️build/📜️script.ts"
    spec = import_path(rel, PROGRESS_MODULE)
    text = tree.text(rel)
    for main in ("runBuildMainV1", "runBuildCliV1"):
        for dirname in ("dirname", "dir"):
            old = 'await profilePhaseV1("cli.run",()=>%s(findWorkspaceRoot(import.meta.%s),process.argv.slice(2)));' % (main, dirname)
            if text.count(old) == 1:
                tree.edit(rel, old, old[:-2] + '.catch(async error=>{const{reportRepoErrorV1}=await import("%s");if(!reportRepoErrorV1(error))throw error;}));' % spec)
                return
    tree.edit(rel, 'await profilePhaseV1("cli.run",()=>runBuildMainV1(findWorkspaceRoot(import.meta.dirname),process.argv.slice(2)));', "")


def migrate_catalog(tree, name, c, spec=None, test_owner=".."):
    """Rewrites the sites a spec describes, writes the catalog, its texts module and its test; returns [(file, rewritten, left)]."""
    blocks = parse_spec(spec or os.path.join(HERE_SPECS, name + ".spec"))
    catalog, used, report = {}, set(), []
    texts_ts = c["texts"] + "/🟦️.ts"
    refuse = c.get("refuse")
    for rel, scope, entries in blocks:
        if rel not in c["files"]:
            raise Refusal("spec block for an unknown file: " + rel)
        done, left = migrate_file(tree, rel, scope, entries, catalog, c["say"], refuse or "refuseNoneV1", used)
        text = tree.text(rel)
        names = [n for n in (c["say"], refuse) if n and re.search(r"(?<![A-Za-z0-9_$])" + n + r"\(", text)]
        if name == "commit" and rel.endswith("/🔀️run-commit/🟦️.ts"):
            n = tree.text(rel).count("console.error(e instanceof Error ? e.message : String(e));")
            tree.edit(rel, "console.error(e instanceof Error ? e.message : String(e));", "reportCommitErrorV1(e);", count=n)
            names.append("reportCommitErrorV1")
        if name == "commit" and rel.endswith("/🪙️micro-commit/🟦️.ts"):
            tree.edit(rel, "console.error(e instanceof Error ? e.message : String(e));", "reportCommitErrorV1(e);")
            names.append("reportCommitErrorV1")
        text = tree.text(rel)
        last = 0
        for match in re.finditer(r"^import[^\n]*\n", text, re.M):
            last = match.end()
        line = 'import {%s} from "%s";\n' % (",".join(names), rel_import(rel, texts_ts))
        tree.put(rel, text[:last] + line + text[last:])
        report.append((rel.replace(L + "/", ""), done, left))
    document = {"version": 1, "id": name, "languages": ["en", "de"], "messages": catalog}
    tree.put(c["texts"] + "/🔣️.json", json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    tree.put(texts_ts, texts_module(name, c, texts_ts, lambda target: rel_import(texts_ts, P + "/" + target)))
    same = [k for k, v in catalog.items() if v["en"] == v["de"]]
    test_ts = c["texts"] + "/🧪️tests/🟦️.ts"
    tree.put(test_ts, test_module(name, c, c.get("owner", test_owner), same, lambda target: rel_import(test_ts, P + "/" + target)))
    return report


def run(tree):
    boundary(tree)
    report = {}
    for name, c in CATALOGS.items():
        report[name] = migrate_catalog(tree, name, c)
    return report
