/** 🪞️ Projects the JSON Schema exports of a scope into the other formats its module provides: the GraphQL type, the protobuf message, the TypeScript type and its typed `parse<Export>` entry point.
 *
 * Execution contract: every export must exist in every format the scope provides. The normative JSON Schema already states the
 * contract, so each projection is derivable from it, and a derived projection is the only kind that cannot drift from the contract
 * it projects. It never invents a Rust type: a Rust entity that exists nowhere is reported, because the Rust file is the crate's real code.
 *
 * `closing` limits the TypeScript pass to the parsers existing code already calls but never declared (and what those need), so a call
 * can never dangle. `replace` regenerates the named parsers that exist but are wrong (for example a typed map an earlier guard left untyped),
 * and `restate` regenerates every parser the audit finds drifted for the named reasons. A regenerated parser keeps its place in the file.
 * A parser is emitted only when every parser it calls is declared on disk or emitted in the same pass, and a parser never resolves to
 * itself. A parser refuses an unknown property exactly where its schema states `additionalProperties: false` (through `<prefix>Closed`,
 * which a file receives with its first such parser), and nowhere else. The freshness gate (`check`) fails on a write the closing projection
 * would still make, a call that dangles, a parser that only calls itself, a schema-typed map read as an opaque object, a parser that
 * accepts an unknown property its schema refuses, any other parser that differs from its projection, and a call of `<prefix>Closed` that
 * no helper block defines. The only parsers left out are the ones in the registry of hand-written parsers no schema can own yet
 * (`🧱️unowned`), and the gate fails on a listed parser that no longer drifts.
 *
 * Documented limits: a schema property named like an array index (`"0"`, `"1"`) cannot keep its document order in a JavaScript object, so
 * its export is refused; integers beyond 2^53 stay exact, other floats print in JavaScript form (`1.0` as `1`); the tracked and untracked
 * files are listed through git, a tree that is no work tree is walked instead.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts
 */
import { existsSync } from "node:fs";
import { dirname, relative } from "node:path";
import { join } from "node:path";
import { codePointOrder, escapeRegExp, jsonString, jsonValue, type Dict } from "./🔡️json-text/🟦️.ts";
import { declaredOptionalFields, declares, declaresParser, definesParser, existingHelper, importsParser, parserRange } from "./📖️declared-types/🟦️.ts";
import { Context, parserKey, parserKeyParts, renderParser, type Owner, type ParserKey } from "./🧾️parser-render/🟦️.ts";
import { renderGraphql, renderProto, renderTsType } from "./📐️type-render/🟦️.ts";
import { PARSER_REGION, helpersBlock, withClosedHelper } from "./🚪️guard-helpers/🟦️.ts";
import { loadUnowned, unownedKey, type UnownedParser } from "./🧱️unowned/🟦️.ts";
import { FORMATS, LEAF, SchemaCatalog, SourceStore, buildOwnerIndex, exportsOf, facetOf, helperPrefix, leafPath, portable, readDocument } from "./🗂️documents/🟦️.ts";

//#region 🔖️Options
const TS = "🟦️typescript";
const JSON_SCHEMA = "🔣️jsonschema";
const RUST = "🦀️rust";

/** 🔎️ Why a projected parser no longer equals what the current schema and renderers produce: `self-call` only delegates to itself and never returns, `untyped-map` reads a schema-typed map as an opaque object, `open` equals the projection without the refusal of unknown properties that its schema requires, `unrenderable` has no projection at all, `stale` differs in any other way. */
export type DriftReason = "self-call" | "untyped-map" | "open" | "stale" | "unrenderable";

/** 🔎️ The drift reasons `restate` can regenerate. */
export type RestateReason = Exclude<DriftReason, "unrenderable">;

const RESTATE_REASONS: readonly RestateReason[] = ["self-call", "untyped-map", "open", "stale"];

/** 🧭️ How one projection run is scoped and what it may write. */
export interface ProjectionOptions {
  readonly root: string;
  readonly mode: "report" | "apply";
  readonly under: string;
  readonly formats: ReadonlySet<string>;
  readonly closing: boolean;
  readonly replace: ReadonlySet<string>;
  readonly restate: ReadonlySet<RestateReason>;
  readonly unowned?: readonly UnownedParser[];
  readonly catalog?: SchemaCatalog;
  readonly signal?: AbortSignal;
  readonly onProgress?: (module: string) => void;
}

/** 📊️ What one projection run did or would do. */
export interface ProjectionReport {
  readonly mode: string;
  readonly under: string;
  readonly closing: boolean;
  readonly added: readonly (readonly [string, number])[];
  readonly filesTouched: number;
  readonly refused: readonly string[];
  readonly rustAbsent: readonly string[];
  readonly touched: readonly string[];
}

/** 🧭️ The formats projected when none is named. */
export const DEFAULT_FORMATS: readonly string[] = ["🔗️graphql", "🛰️protobuf", TS];

/** 🧭️ The default scope: every plugin and app contract. */
export const DEFAULT_SCOPE = "✏️s";

/** 🧭️ The scopes the freshness gate covers. */
export const CHECKED_SCOPES: readonly string[] = ["✏️s", "🌎️hub"];

const VALUE_FLAGS = new Set(["--under", "--formats", "--replace", "--restate"]);
const SWITCH_FLAGS = new Set(["--closing", "--all"]);

/** 🔤️ Reads `<report|apply|check> [--under <path>] [--formats a,b] [--closing] [--replace A,B] [--restate open,stale,… | all] [--all]`; a mistyped flag is refused. */
export function parseProjectionArguments(root: string, argv: readonly string[]): ProjectionOptions & { readonly command: string; readonly all: boolean } {
  const named = argv[0] !== undefined && !argv[0].startsWith("--");
  const command = named ? argv[0]! : "report";
  if (!["report", "apply", "check"].includes(command)) throw new Error(`[schema parsers] unknown command ${JSON.stringify(command)} (expected report | apply | check).`);
  for (let at = named ? 1 : 0; at < argv.length; at += 1) {
    const token = argv[at]!;
    if (VALUE_FLAGS.has(token)) {
      const found = argv[at + 1];
      if (found === undefined || found.startsWith("--")) throw new Error(`[schema parsers] ${token} requires a value.`);
      at += 1;
    } else if (!SWITCH_FLAGS.has(token)) throw new Error(`[schema parsers] unknown argument ${JSON.stringify(token)}.`);
  }
  const value = (flag: string): string | undefined => {
    const at = argv.indexOf(flag);
    return at < 0 ? undefined : argv[at + 1];
  };
  const formats = new Set((value("--formats") ?? DEFAULT_FORMATS.join(",")).split(","));
  for (const format of formats) if (!DEFAULT_FORMATS.includes(format)) throw new Error(`[schema parsers] unknown format ${JSON.stringify(format)} (expected ${DEFAULT_FORMATS.join(" | ")}).`);
  const restated = (value("--restate") ?? "").split(",").filter((reason) => reason !== "");
  const restate = new Set<RestateReason>(restated.includes("all") ? RESTATE_REASONS : (restated as RestateReason[]));
  for (const reason of restate) if (!RESTATE_REASONS.includes(reason)) throw new Error(`[schema parsers] unknown restate reason ${JSON.stringify(reason)} (expected ${RESTATE_REASONS.join(" | ")} | all).`);
  return { command, root, mode: command === "apply" ? "apply" : "report", under: value("--under") ?? DEFAULT_SCOPE, formats, closing: argv.includes("--closing"), replace: new Set(value("--replace")?.split(",") ?? []), restate, all: argv.includes("--all") };
}
//#endregion 🔖️Options

//#region 🔖️Plans
interface Entry {
  readonly type: string | undefined;
  readonly typed: boolean;
  readonly parser: string | undefined;
  readonly local: ReadonlySet<string>;
  readonly foreign: ReadonlySet<ParserKey>;
}

interface Plan {
  readonly entries: Map<string, Entry>;
  readonly exports: ReadonlySet<string>;
  readonly helper: string;
}

class Counter {
  private readonly counts = new Map<string, number>();

  add(key: string): void {
    this.counts.set(key, (this.counts.get(key) ?? 0) + 1);
  }

  clear(): void {
    this.counts.clear();
  }

  entries(): [string, number][] {
    return [...this.counts];
  }
}

const sortedKeys = (keys: Iterable<ParserKey>): ParserKey[] => [...keys].sort(codePointOrder);

/** 🧭️ Decides which parsers are emitted: one is emitted only when every parser it calls is declared on disk or emitted in the same pass, so a call can never dangle. */
function settle(root: string, plans: ReadonlyMap<string, Plan>, closing: boolean, sources: SourceStore, refused: string[], replaces: (target: string, name: string) => boolean): Set<ParserKey> {
  const candidates = new Map<ParserKey, Entry>();
  for (const [target, plan] of plans) for (const [name, entry] of plan.entries) if (entry.parser !== undefined && entry.typed) candidates.set(parserKey(target, name), entry);
  let emit = new Set(candidates.keys());
  if (closing) {
    const needed = new Set<ParserKey>();
    const stack: ParserKey[] = [];
    for (const [target, plan] of plans) {
      const source = sources.read(target);
      const called = new Set([...source.matchAll(/\bparse([A-Z][A-Za-z0-9]*)\(/gu)].map((match) => match[1]!));
      for (const name of plan.exports) if (replaces(target, name)) called.add(name);
      for (const name of [...called].sort(codePointOrder)) {
        if (plan.exports.has(name) && ((!definesParser(source, name) && !importsParser(source, name)) || replaces(target, name))) stack.push(parserKey(target, name));
      }
    }
    while (stack.length > 0) {
      const key = stack.pop()!;
      if (needed.has(key)) continue;
      const [owner, name] = parserKeyParts(key);
      const entry = candidates.get(key);
      if (entry === undefined) {
        refused.push(`dangling parse${name} ${portable(root, owner)} has no renderable parser`);
        continue;
      }
      needed.add(key);
      const text = sources.read(owner);
      for (const callee of [...entry.local].sort(codePointOrder)) if (!definesParser(text, callee) && !importsParser(text, callee)) stack.push(parserKey(owner, callee));
      for (const foreignKey of sortedKeys(entry.foreign)) {
        const [foreignOwner, callee] = parserKeyParts(foreignKey);
        if (!definesParser(sources.read(foreignOwner), callee)) stack.push(foreignKey);
      }
    }
    emit = needed;
  }
  const available = (owner: string, name: string): boolean => emit.has(parserKey(owner, name)) || definesParser(sources.read(owner), name) || importsParser(sources.read(owner), name);
  let changed = true;
  while (changed) {
    changed = false;
    for (const key of sortedKeys(emit)) {
      const entry = candidates.get(key)!;
      const [owner] = parserKeyParts(key);
      const missing = [
        ...[...entry.local].sort(codePointOrder).filter((name) => !available(owner, name)).map((name) => `parse${name}`),
        ...sortedKeys(entry.foreign).map(parserKeyParts).filter(([foreignOwner, name]) => !available(foreignOwner, name)).map(([, name]) => `parse${name}`),
      ];
      if (missing.length > 0) {
        refused.push(`parser ${parserKeyParts(key)[1]} ${portable(root, owner)} needs ${missing.join(", ")}`);
        emit.delete(key);
        changed = true;
      }
    }
  }
  return emit;
}
//#endregion 🔖️Plans

//#region 🔖️Source
const importLines = (target: string, wanted: ReadonlyMap<string, ReadonlySet<string>>): string =>
  [...wanted.keys()]
    .sort(codePointOrder)
    .map((owner) => {
      const path = relative(dirname(target), owner).split("\\").join("/");
      const specifier = path.startsWith(".") ? path : `./${path}`;
      return `import { ${[...wanted.get(owner)!].sort(codePointOrder).map((name) => `parse${name}`).join(", ")} } from ${jsonString(specifier, false)};`;
    })
    .join("\n");

function withImports(source: string, lines: string): string {
  if (lines === "") return source;
  if (source.startsWith("/**")) {
    const end = source.indexOf("*/");
    if (end !== -1) {
      const cut = source.indexOf("\n", end) + 1;
      return `${source.slice(0, cut)}${lines}\n${source.slice(cut)}`;
    }
  }
  return `${lines}\n${source}`;
}

function dropDuplicateType(source: string, name: string): string {
  const declarations = source.match(new RegExp(`^\\s*(?:export\\s+)?(?:interface|type)\\s+${escapeRegExp(name)}\\b`, "gm"));
  if (declarations === null || declarations.length <= 1) return source;
  const duplicate = new RegExp(`^export type ${escapeRegExp(name)}\\b[^\\n]*;\\n\\n?`, "m").exec(source);
  return duplicate === null ? source : source.slice(0, duplicate.index) + source.slice(duplicate.index + duplicate[0].length);
}

function restateParser(source: string, name: string, text: string, path: string): string {
  const range = parserRange(source, name);
  if (range === undefined) throw new Error(`parse${name} is not a projected function in ${path}`);
  return dropDuplicateType(source.slice(0, range[0]) + text + source.slice(range[1]), name);
}
//#endregion 🔖️Source

//#region 🔖️Projection
const rendererOf = (format: string, name: string, definition: Dict): string | undefined => (format === "🔗️graphql" ? renderGraphql(name, definition) : format === "🛰️protobuf" ? renderProto(name, definition) : renderTsType(name, definition));

/** 🪞️ Projects the exports of every schema module under `options.under`; `report` only plans, `apply` writes the leaves. */
export function projectParsers(options: ProjectionOptions): ProjectionReport {
  const { root, mode, under, formats, closing, replace } = options;
  const catalog = options.catalog ?? new SchemaCatalog(root);
  const restated = new Set<ParserKey>();
  if (options.restate.size > 0) {
    const unowned = new Set((options.unowned ?? loadUnowned().parsers).map((row) => unownedKey(row.path, row.name)));
    for (const row of auditParsers({ root, under, catalog, signal: options.signal })) {
      if (row.reason !== "unrenderable" && options.restate.has(row.reason) && !unowned.has(unownedKey(row.path, row.name))) restated.add(parserKey(join(root, row.path), row.name));
    }
  }
  const replaces = (target: string, name: string): boolean => replace.has(name) || restated.has(parserKey(target, name));
  const index: ReadonlyMap<string, Owner> = buildOwnerIndex(root, catalog);
  const sources = new SourceStore();
  const promised = new Set<ParserKey>();
  const added = new Counter();
  const refused: string[] = [];
  const rustAbsent: string[] = [];
  const touched = new Set<string>();
  const plans = new Map<string, Plan>();
  const modules = catalog.modules(under);
  if (modules.length === 0) throw new Error(`[schema parsers] no schema module under ${JSON.stringify(under)}.`);
  const documents = new Map<string, Dict | undefined>();
  const load = (path: string): Dict | undefined => {
    if (!documents.has(path)) documents.set(path, readDocument(root, path));
    return documents.get(path);
  };
  for (const scanPass of [0, 1]) {
    const dry = scanPass === 0;
    plans.clear();
    added.clear();
    refused.length = 0;
    rustAbsent.length = 0;
    touched.clear();
    for (const module of modules) {
      options.signal?.throwIfAborted();
      options.onProgress?.(module);
      const provided = FORMATS.filter((format) => existsSync(`${root}/${module}/${LEAF[format as keyof typeof LEAF]}`));
      const moduleDocumentPaths = catalog.documents(module);
      const siblings = moduleDocumentPaths.map((other) => leafPath(root, module, facetOf(module, other), TS));
      for (const path of moduleDocumentPaths) {
        const document = load(path);
        if (document === undefined) continue;
        const facet = facetOf(module, path);
        const exports = exportsOf(document);
        if (exports.size === 0) continue;
        for (const format of provided) {
          if (format === JSON_SCHEMA) continue;
          const target = leafPath(root, module, facet, format as keyof typeof LEAF);
          if (!existsSync(target)) continue;
          const relativeTarget = portable(root, target);
          const source = sources.read(target);
          const appended: string[] = [];
          const helper = existingHelper(source) ?? helperPrefix(document.$id);
          let plan: Plan | undefined;
          if (format === TS) {
            plan = plans.get(target);
            if (plan === undefined) {
              plan = { entries: new Map(), exports: new Set(exports.keys()), helper };
              plans.set(target, plan);
            }
          }
          for (const [name, definition] of exports) {
            const restricted = definition["x-semio-formats"];
            if (Array.isArray(restricted) && !restricted.includes(format)) continue;
            let present = declares(format, sources.declarations(target), name);
            let typeText: string | undefined;
            if (!present && formats.has(format) && !(closing && format === TS)) {
              const rendered = rendererOf(format, name, definition);
              if (rendered === undefined) {
                refused.push(`${format} ${name} ${relativeTarget}`);
                continue;
              }
              typeText = rendered;
              if (format !== TS) {
                appended.push(rendered);
                added.add(format);
              }
              present = true;
            } else if (!present && format === TS && closing) {
              typeText = renderTsType(name, definition);
              present = typeText !== undefined;
            } else if (!present && format === RUST) {
              rustAbsent.push(`${name} ${relativeTarget}`);
              continue;
            }
            if (format === TS && (!declaresParser(source, name) || replaces(target, name))) {
              const context = new Context(helper, typeof document.$id === "string" ? document.$id : undefined, index, target, siblings, sources, promised);
              const rendered = present ? renderParser(name, definition, context, declaredOptionalFields(sources.declarations(target), name)) : undefined;
              if (rendered === undefined && present) refused.push(`parser ${name} ${relativeTarget}`);
              const local = new Set(context.local);
              local.delete(name);
              plan!.entries.set(name, { type: typeText, typed: present, parser: rendered, local, foreign: new Set(context.foreign) });
            }
          }
          if (format === TS || appended.length === 0 || mode !== "apply" || dry) continue;
          sources.write(target, `${source.trimEnd()}\n\n${appended.join("\n").trimEnd()}\n`);
          touched.add(relativeTarget);
        }
      }
    }
    for (const [owner, plan] of plans) for (const [name, entry] of plan.entries) if (entry.parser !== undefined) promised.add(parserKey(owner, name));
  }
  const emit = settle(root, plans, closing || replace.size > 0 || restated.size > 0, sources, refused, replaces);
  for (const [target, plan] of plans) {
    const keep = [...emit].map(parserKeyParts).filter(([owner]) => owner === target).map(([, name]) => name).sort(codePointOrder);
    if (keep.length === 0) continue;
    let source = sources.read(target);
    const blocks: string[] = [];
    const foreign = new Map<string, Set<string>>();
    const relativeTarget = portable(root, target);
    let closedCalls = false;
    for (const name of keep) {
      const entry = plan.entries.get(name)!;
      if (entry.type !== undefined && !declares(TS, sources.declarations(target), name)) {
        blocks.push(entry.type);
        added.add(TS);
      }
      closedCalls ||= entry.parser!.includes(`${plan.helper}Closed(`);
      if (replaces(target, name) && definesParser(source, name)) {
        added.add("restated");
        if (mode === "apply") source = restateParser(source, name, entry.parser!, relativeTarget);
      } else {
        blocks.push(entry.parser!);
        added.add("parser");
      }
      for (const key of entry.foreign) {
        const [owner, callee] = parserKeyParts(key);
        if (owner === target || definesParser(source, callee) || importsParser(source, callee)) continue;
        if (!foreign.has(owner)) foreign.set(owner, new Set());
        foreign.get(owner)!.add(callee);
      }
    }
    touched.add(relativeTarget);
    if (mode !== "apply") continue;
    if (source.includes(PARSER_REGION)) {
      if (closedCalls && existingHelper(source) !== plan.helper) throw new Error(`the guard prefix of ${relativeTarget} is not ${plan.helper}`);
      if (closedCalls) source = withClosedHelper(source, plan.helper);
    } else if (blocks.length > 0) blocks.unshift(helpersBlock(plan.helper));
    const block = blocks.join("\n").trimEnd();
    sources.write(target, withImports(block === "" ? source : `${source.trimEnd()}\n\n${block}\n`, importLines(target, foreign)));
  }
  return { mode, under, closing, added: added.entries(), filesTouched: touched.size, refused: [...refused], rustAbsent: [...rustAbsent], touched: [...touched].sort(codePointOrder) };
}

//#endregion 🔖️Projection

//#region 🔖️Audit
/** 🔎️ A projected parser that no longer equals what the current schema and renderers produce. */
export interface ParserDrift {
  readonly path: string;
  readonly name: string;
  readonly reason: DriftReason;
  readonly actual: string;
  readonly expected: string | undefined;
}

/** 🔎️ What one audit found: the drifted parsers, and the leaves whose parsers call `<prefix>Closed` without a helper block that defines it. */
export interface ParserAudit {
  readonly drift: ParserDrift[];
  readonly helperless: string[];
}

const mapConversions = (text: string): number => text.split("Object.entries(").length - 1;

const SELF_CALL = /^export function parse(\w+)\(value: unknown, at = "\$"\): \1 \{\n  return parse\1\(value[^\n]*\n\}\n/gmu;

/** 🔎️ Re-renders every parser below the helper region of a leaf and lists the ones whose text differs from the projection of the current schema, and the leaves that call `<prefix>Closed` without defining it. */
export function auditLeaves(options: Pick<ProjectionOptions, "root" | "under" | "catalog" | "signal" | "onProgress">): ParserAudit {
  const { root, under } = options;
  const catalog = options.catalog ?? new SchemaCatalog(root);
  const index: ReadonlyMap<string, Owner> = buildOwnerIndex(root, catalog);
  const sources = new SourceStore();
  const promised = new Set<ParserKey>();
  const drift: ParserDrift[] = [];
  const helperless: string[] = [];
  const audited = new Set<string>();
  for (const module of catalog.modules(under)) {
    options.signal?.throwIfAborted();
    options.onProgress?.(module);
    const paths = catalog.documents(module);
    const siblings = paths.map((other) => leafPath(root, module, facetOf(module, other), TS));
    for (const path of paths) {
      const document = readDocument(root, path);
      const target = leafPath(root, module, facetOf(module, path), TS);
      if (document === undefined || !existsSync(target)) continue;
      const source = sources.read(target);
      const region = source.indexOf(PARSER_REGION);
      if (region < 0) continue;
      const helper = existingHelper(source) ?? helperPrefix(document.$id);
      const relativeTarget = portable(root, target);
      if (!audited.has(target)) {
        audited.add(target);
        if (source.includes(`${helper}Closed(`) && !new RegExp(`^export const ${escapeRegExp(helper)}Closed\\b`, "m").test(source)) helperless.push(relativeTarget);
      }
      for (const [name, definition] of exportsOf(document)) {
        const range = parserRange(source, name);
        if (range === undefined || range[0] < region) continue;
        const found = source.slice(range[0], range[1]);
        const context = (closed: boolean): Context => new Context(helper, typeof document.$id === "string" ? document.$id : undefined, index, target, siblings, sources, promised, closed);
        const rendered = renderParser(name, definition, context(true), declaredOptionalFields(sources.declarations(target), name));
        if (rendered === found) continue;
        const open = rendered === undefined ? undefined : renderParser(name, definition, context(false), declaredOptionalFields(sources.declarations(target), name));
        const reason: DriftReason = rendered === undefined ? "unrenderable" : open === found ? "open" : mapConversions(rendered) > mapConversions(found) ? "untyped-map" : "stale";
        drift.push({ path: relativeTarget, name, reason, actual: found, expected: rendered });
      }
      for (const call of source.matchAll(SELF_CALL)) {
        if (call.index < region) continue;
        const already = drift.findIndex((row) => row.path === relativeTarget && row.name === call[1]);
        if (already >= 0) drift[already] = { ...drift[already]!, reason: "self-call" };
        else drift.push({ path: relativeTarget, name: call[1]!, reason: "self-call", actual: call[0], expected: undefined });
      }
    }
  }
  return { drift, helperless };
}

/** 🔎️ The drifted parsers of a scope; see `auditLeaves`. */
export const auditParsers = (options: Pick<ProjectionOptions, "root" | "under" | "catalog" | "signal" | "onProgress">): ParserDrift[] => auditLeaves(options).drift;

/** 🚦️ The freshness verdict of the projection over a scope. */
export interface ParserCheck {
  readonly pending: readonly string[];
  readonly dangling: readonly string[];
  readonly helperless: readonly string[];
  readonly drift: readonly ParserDrift[];
  readonly orphans: readonly UnownedParser[];
  readonly unowned: number;
  readonly ok: boolean;
}

/** 🚦️ Freshness gate over the checked scopes: the closing projection must write nothing and leave no call dangling, every parser must equal the projection of its schema (the listed unowned parsers excepted), every call of `<prefix>Closed` needs its helper, and a listed parser must still drift. */
export function checkParsers(options: Pick<ProjectionOptions, "root" | "signal" | "onProgress" | "unowned"> & { readonly scopes?: readonly string[] }): ParserCheck {
  const pending: string[] = [];
  const dangling: string[] = [];
  const helperless: string[] = [];
  const found: ParserDrift[] = [];
  const catalog = new SchemaCatalog(options.root);
  const requested = options.scopes ?? CHECKED_SCOPES;
  const scopes = requested.filter((scope) => catalog.modules(scope).length > 0);
  if (scopes.length === 0) throw new Error(`[schema parsers] no schema module under ${JSON.stringify(requested)}.`);
  const registry = options.unowned ?? loadUnowned().parsers;
  for (const under of scopes) {
    const closing = projectParsers({ ...options, catalog, under, mode: "report", formats: new Set([TS]), closing: true, replace: new Set(), restate: new Set(), unowned: registry });
    pending.push(...closing.touched);
    dangling.push(...closing.refused.filter((row) => row.startsWith("dangling ") || /^parser \S+ \S+ needs /u.test(row)));
    const audit = auditLeaves({ ...options, catalog, under });
    found.push(...audit.drift);
    helperless.push(...audit.helperless);
  }
  const listed = new Set(registry.map((row) => unownedKey(row.path, row.name)));
  const drifted = new Set(found.map((row) => unownedKey(row.path, row.name)));
  const drift = found.filter((row) => !listed.has(unownedKey(row.path, row.name)));
  const orphans = registry.filter((row) => scopes.some((scope) => row.path === scope || row.path.startsWith(`${scope}/`)) && !drifted.has(unownedKey(row.path, row.name)));
  return { pending, dangling, helperless, drift, orphans, unowned: found.length - drift.length, ok: pending.length === 0 && dangling.length === 0 && helperless.length === 0 && drift.length === 0 && orphans.length === 0 };
}

/** 🚦️ The verdict as the gate prints it. */
export function renderParserCheck(check: ParserCheck): string {
  const lines = [
    ...check.pending.map((path) => `PENDING ${path}`),
    ...check.dangling.map((row) => `DANGLING ${row}`),
    ...check.helperless.map((path) => `HELPERLESS ${path}`),
    ...check.drift.map((row) => `DRIFT ${row.reason} ${row.name} ${row.path}`),
    ...check.orphans.map((row) => `ORPHAN ${row.name} ${row.path}`),
  ];
  lines.push(`[schema parsers check] ${check.ok ? "fresh" : "stale"}: pending=${check.pending.length} dangling=${check.dangling.length} helperless=${check.helperless.length} drift=${check.drift.length} orphans=${check.orphans.length} unowned=${check.unowned}`);
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Audit

//#region 🔖️Report
/** 🧾️ The report as the command prints it: one JSON summary line, then the first refusals, the Rust entities that exist nowhere and every touched leaf. */
export function renderProjectionReport(report: ProjectionReport): string {
  const summary = { mode: report.mode, under: report.under, closing: report.closing, added: Object.fromEntries(report.added), filesTouched: report.filesTouched, refused: report.refused.length, rustAbsent: report.rustAbsent.length };
  const lines = [jsonValue(summary, false)];
  for (const row of report.refused.slice(0, 60)) lines.push(`REFUSED ${row}`);
  for (const row of report.rustAbsent.slice(0, 30)) lines.push(`RUST-ABSENT ${row}`);
  for (const path of report.touched) lines.push(`TOUCHED ${path}`);
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Report
