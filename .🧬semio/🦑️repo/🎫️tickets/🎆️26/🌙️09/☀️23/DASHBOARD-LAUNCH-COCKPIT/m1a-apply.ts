#!/usr/bin/env bun
/**
 * 🪡️ Inserts the hand-written M-1a declarations of `m1a-declarations.ts` into their owner manifests with
 * surgical text edits: only the added keys change, every other byte of a manifest stays as it is.
 *
 * `bun m1a-apply.ts` reports what would change; `bun m1a-apply.ts --write` writes. A manifest is re-read
 * immediately before its write, an anchor must occur exactly once, and the result must parse and carry
 * exactly the declared value, otherwise that manifest is left untouched. Re-running is a no-op.
 *
 * @see ./m1a-declarations.ts
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { OWNER_EDITS, type Declared, type OwnerEdit } from "./m1a-declarations.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const write = process.argv.includes("--write");

type Json = Record<string, any>;

/** 🧱️ Renders `"key": value` in the two-space style every manifest uses, indented by `indent` spaces. */
function member(key: string, value: unknown, indent: number): string {
  const pad = " ".repeat(indent);
  return `${pad}${JSON.stringify(key)}: ${JSON.stringify(value, null, 2).split("\n").join(`\n${pad}`)}`;
}

/** 🎯️ Replaces the single occurrence of `anchor`; throws when it is absent or ambiguous. */
function once(text: string, anchor: string, replacement: string, what: string): string {
  const count = text.split(anchor).length - 1;
  if (count !== 1) throw new Error(`${what}: anchor occurs ${count} times`);
  return text.replace(anchor, () => replacement);
}

function applyProject(text: string, manifest: Json, declared: Declared, name: string): string {
  if (isDeepStrictEqual(manifest.metadata?.semio?.dashboard, declared)) return text;
  if (manifest.metadata?.semio?.dashboard !== undefined) throw new Error("project dashboard declaration exists and differs");
  if (manifest.metadata?.semio !== undefined) {
    const keys = Object.keys(manifest.metadata.semio);
    const last = keys[keys.length - 1]!;
    const anchor = `${member(last, manifest.metadata.semio[last], 6)}\n    }`;
    return once(text, anchor, `${member(last, manifest.metadata.semio[last], 6)},\n${member("dashboard", declared, 6)}\n    }`, "metadata.semio");
  }
  if (manifest.metadata !== undefined) {
    const keys = Object.keys(manifest.metadata);
    const last = keys[keys.length - 1]!;
    const anchor = `${member(last, manifest.metadata[last], 4)}\n  }`;
    return once(text, anchor, `${member(last, manifest.metadata[last], 4)},\n${member("semio", { dashboard: declared }, 4)}\n  }`, "metadata");
  }
  const anchor = `  "name": ${JSON.stringify(name)},\n`;
  return once(text, anchor, `${anchor}${member("metadata", { semio: { dashboard: declared } }, 2)},\n`, "name");
}

function applyTarget(text: string, manifest: Json, target: string, declared: Declared): string {
  const existing = manifest.targets?.[target];
  if (existing === undefined) throw new Error(`target ${target} is not declared in the manifest`);
  if (isDeepStrictEqual(existing.metadata?.semio?.dashboard, declared)) return text;
  if (existing.metadata !== undefined) throw new Error(`target ${target} already carries metadata`);
  const anchor = `\n    ${JSON.stringify(target)}: {\n`;
  return once(text, anchor, `${anchor}${member("metadata", { semio: { dashboard: declared } }, 6)},\n`, `target ${target}`);
}

function removeEnv(text: string, manifest: Json, target: string, keys: readonly string[]): string {
  const env = manifest.targets?.[target]?.options?.env;
  if (env === undefined || keys.every((key) => !(key in env))) return text;
  if (!isDeepStrictEqual(Object.keys(env).sort(), [...keys].sort())) throw new Error(`target ${target} env holds other keys too`);
  const command = manifest.targets[target].options.command as string;
  const anchor = `${member("command", command, 8)},\n${member("env", env, 8)},\n`;
  return once(text, anchor, `${member("command", command, 8)},\n`, `env of ${target}`);
}

function expected(manifest: Json, edit: OwnerEdit): string[] {
  const faults: string[] = [];
  if (edit.project && !isDeepStrictEqual(manifest.metadata?.semio?.dashboard, edit.project)) faults.push("project declaration differs");
  for (const [target, declared] of Object.entries(edit.targets ?? {})) if (!isDeepStrictEqual(manifest.targets?.[target]?.metadata?.semio?.dashboard, declared)) faults.push(`target ${target} differs`);
  for (const [target, keys] of Object.entries(edit.removeEnv ?? {})) for (const key of keys) if (manifest.targets?.[target]?.options?.env?.[key] !== undefined) faults.push(`target ${target} still sets ${key}`);
  return faults;
}

/** 🧮️ Every member of `before` outside the declared paths must survive unchanged. */
function untouched(before: Json, after: Json, edit: OwnerEdit): boolean {
  const strip = (manifest: Json): Json => {
    const copy = structuredClone(manifest);
    if (copy.metadata?.semio) delete copy.metadata.semio.dashboard;
    if (copy.metadata?.semio && Object.keys(copy.metadata.semio).length === 0) delete copy.metadata.semio;
    if (copy.metadata && Object.keys(copy.metadata).length === 0) delete copy.metadata;
    for (const target of Object.keys(edit.targets ?? {})) {
      const node = copy.targets[target];
      if (node.metadata?.semio) delete node.metadata.semio.dashboard;
      if (node.metadata?.semio && Object.keys(node.metadata.semio).length === 0) delete node.metadata.semio;
      if (node.metadata && Object.keys(node.metadata).length === 0) delete node.metadata;
    }
    for (const [target, keys] of Object.entries(edit.removeEnv ?? {})) {
      const env = copy.targets[target].options?.env;
      if (!env) continue;
      for (const key of keys) delete env[key];
      if (Object.keys(env).length === 0) delete copy.targets[target].options.env;
    }
    return copy;
  };
  return isDeepStrictEqual(strip(before), strip(after));
}

let failed = 0;
for (const edit of OWNER_EDITS) {
  const path = join(root, edit.file);
  try {
    const before = readFileSync(path, "utf8");
    const manifest = JSON.parse(before) as Json;
    if (manifest.name !== edit.name) throw new Error(`manifest is ${manifest.name}, expected ${edit.name}`);
    let text = before;
    if (edit.project) text = applyProject(text, manifest, edit.project, edit.name);
    for (const [target, declared] of Object.entries(edit.targets ?? {})) text = applyTarget(text, manifest, target, declared);
    for (const [target, keys] of Object.entries(edit.removeEnv ?? {})) text = removeEnv(text, manifest, target, keys);
    const after = JSON.parse(text) as Json;
    const faults = expected(after, edit);
    if (faults.length) throw new Error(faults.join("; "));
    if (!untouched(manifest, after, edit)) throw new Error("a member outside the declarations changed");
    const added = text.split("\n").length - before.split("\n").length;
    if (text === before) console.log(`unchanged  ${edit.file}`);
    else if (!write) console.log(`would edit ${edit.file} (${added >= 0 ? "+" : ""}${added} lines)`);
    else {
      if (readFileSync(path, "utf8") !== before) throw new Error("manifest changed while editing; run again");
      writeFileSync(path, text);
      console.log(`edited     ${edit.file} (${added >= 0 ? "+" : ""}${added} lines)`);
    }
  } catch (error) {
    failed += 1;
    console.error(`FAILED     ${edit.file}: ${error instanceof Error ? error.message : String(error)}`);
  }
}
process.exit(failed ? 1 : 0);
