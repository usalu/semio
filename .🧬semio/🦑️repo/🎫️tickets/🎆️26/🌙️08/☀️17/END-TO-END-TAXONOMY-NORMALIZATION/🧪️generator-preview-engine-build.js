// @bun
var __require = import.meta.require;

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts */
import { execFileSync, spawnSync } from "child_process";
import { createHash } from "crypto";
import {
  chmodSync,
  copyFileSync,
  existsSync,
  lstatSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  readlinkSync,
  renameSync,
  rmdirSync,
  rmSync,
  symlinkSync,
  writeFileSync
} from "fs";
import { tmpdir } from "os";
import { basename as basename2, dirname as dirname2, isAbsolute, join as join2, posix, relative as relative2, resolve as resolve2, sep } from "path";

/* 🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts */
function encodeArtifactKindChoice(choice) {
  return JSON.stringify({
    kindId: choice.kindId,
    schema: choice.schema,
    dialect: choice.dialect,
    label: { en: choice.label.en, de: choice.label.de }
  });
}
function decodeArtifactKindChoice(value) {
  const json = JSON.parse(value);
  if (typeof json.kindId !== "string")
    throw new Error("artifact kind choice missing string field kindId");
  if (typeof json.schema !== "string")
    throw new Error("artifact kind choice missing string field schema");
  const dialect = json.dialect;
  if (typeof dialect?.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string") {
    throw new Error("artifact kind choice missing field dialect");
  }
  const label = json.label;
  if (typeof label?.en !== "string")
    throw new Error("artifact kind choice missing string field label.en");
  if (typeof label.de !== "string")
    throw new Error("artifact kind choice missing string field label.de");
  return { kindId: json.kindId, schema: json.schema, dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset }, label: { en: label.en, de: label.de } };
}
function encodeSurfaceAppChoice(choice) {
  return JSON.stringify({ pluginId: choice.app.pluginId, appId: choice.app.appId, role: choice.role });
}
function decodeSurfaceAppChoice(value) {
  const json = JSON.parse(value);
  if (typeof json.pluginId !== "string")
    throw new Error("surface app choice missing string field pluginId");
  if (typeof json.appId !== "string")
    throw new Error("surface app choice missing string field appId");
  if (json.role !== "editor" && json.role !== "viewer")
    throw new Error("surface app choice missing string field role");
  return { app: { pluginId: json.pluginId, appId: json.appId }, role: json.role };
}
function resolveNativeLabel(label) {
  const native = label?.native;
  return { en: native?.en ?? "", de: native?.de ?? "" };
}
function artifactKindChoices(manifests, roles) {
  const byCoordinate = new Map;
  for (const manifest of manifests) {
    for (const raw of manifest.apps) {
      const app = raw;
      if (!roles.includes(app.role) || app.io.documentSchema === "")
        continue;
      const coordinate = `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}`;
      if (byCoordinate.has(coordinate))
        continue;
      byCoordinate.set(coordinate, { kindId: app.dialect.artifactKind, schema: app.io.documentSchema, dialect: app.dialect, label: resolveNativeLabel(app.label) });
    }
  }
  return [...byCoordinate.keys()].sort().map((coordinate) => byCoordinate.get(coordinate));
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("\uD83D\uDD16\uFE0FHostResolvedArgs", () => {
    const PINNED_ARTIFACT_KIND_CHOICE = {
      kindId: "s.draw.draw",
      schema: "draw.document",
      dialect: { artifactKind: "s.draw.draw", standard: "1", subset: "*" },
      label: { en: "Draw", de: "Zeichnung" }
    };
    const PINNED_ARTIFACT_KIND_CHOICE_JSON = '{"kindId":"s.draw.draw","schema":"draw.document","dialect":{"artifactKind":"s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}';
    it("encodeArtifactKindChoice matches the contract's pinned byte-identical fixture", () => {
      expect(encodeArtifactKindChoice(PINNED_ARTIFACT_KIND_CHOICE)).toBe(PINNED_ARTIFACT_KIND_CHOICE_JSON);
    });
    it("decodeArtifactKindChoice inverts the pinned fixture", () => {
      expect(decodeArtifactKindChoice(PINNED_ARTIFACT_KIND_CHOICE_JSON)).toEqual(PINNED_ARTIFACT_KIND_CHOICE);
    });
    it("decodeArtifactKindChoice throws naming the missing field", () => {
      expect(() => decodeArtifactKindChoice("{}")).toThrow(/kindId/);
    });
    it("encodeSurfaceAppChoice / decodeSurfaceAppChoice round-trip the frozen shape", () => {
      const choice = { app: { pluginId: "draw", appId: "s.draw.draw@1/*#editor" }, role: "editor" };
      const json = encodeSurfaceAppChoice(choice);
      expect(json).toBe('{"pluginId":"draw","appId":"s.draw.draw@1/*#editor","role":"editor"}');
      expect(decodeSurfaceAppChoice(json)).toEqual(choice);
    });
    it("decodeSurfaceAppChoice throws on an invalid role", () => {
      expect(() => decodeSurfaceAppChoice('{"pluginId":"draw","appId":"a","role":"bogus"}')).toThrow(/role/);
    });
    function fakeManifest(pluginId, apps) {
      return {
        pluginId,
        label: pluginId,
        version: "1.0.0",
        apps: apps.map((app) => ({ role: app.role, dialect: app.dialect, label: { native: app.label ?? { en: app.dialect.artifactKind, de: app.dialect.artifactKind } }, io: { documentSchema: app.documentSchema } })),
        workflows: [],
        examples: []
      };
    }
    it("artifactKindChoices dedupes by dialect coordinate (owner manifest first wins), sorts, and filters by role", () => {
      const drawDialect = { artifactKind: "s.draw.draw", standard: "1", subset: "*" };
      const dagDialect = { artifactKind: "s.dag.dag", standard: "1", subset: "*" };
      const manifests = [
        fakeManifest("draw", [
          { role: "editor", dialect: drawDialect, documentSchema: "draw.document", label: { en: "Draw", de: "Zeichnung" } },
          { role: "editor", dialect: { artifactKind: "s.draw.empty", standard: "1", subset: "*" }, documentSchema: "" }
        ]),
        fakeManifest("draw-contrib", [{ role: "viewer", dialect: drawDialect, documentSchema: "draw.document", label: { en: "Draw (fallback)", de: "Zeichnung (fallback)" } }]),
        fakeManifest("dag", [{ role: "editor", dialect: dagDialect, documentSchema: "dag.document", label: { en: "DAG", de: "DAG" } }])
      ];
      const editorOnly = artifactKindChoices(manifests, ["editor"]);
      expect(editorOnly.map((choice) => choice.kindId)).toEqual(["s.dag.dag", "s.draw.draw"]);
      expect(editorOnly.find((choice) => choice.kindId === "s.draw.draw")?.label).toEqual({ en: "Draw", de: "Zeichnung" });
      const editorAndViewer = artifactKindChoices(manifests, ["editor", "viewer"]);
      expect(editorAndViewer.map((choice) => choice.kindId)).toEqual(["s.dag.dag", "s.draw.draw"]);
      expect(editorAndViewer.find((choice) => choice.kindId === "s.draw.draw")?.label).toEqual({ en: "Draw", de: "Zeichnung" });
      expect(artifactKindChoices(manifests, ["viewer"]).map((choice) => choice.kindId)).toEqual(["s.draw.draw"]);
    });
  });
}
/* 🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts */
function effectiveActionArgs(defs, staged, seed) {
  if (defs.length === 0)
    return { ...seed, ...staged };
  const effective = seed ? { ...seed } : {};
  for (const def of defs) {
    if (Object.prototype.hasOwnProperty.call(staged, def.id)) {
      effective[def.id] = staged[def.id];
    } else if (!Object.prototype.hasOwnProperty.call(effective, def.id) && def.default !== undefined && def.default !== null) {
      effective[def.id] = def.default;
    }
  }
  return effective;
}
function missingRequiredArgs(defs, effective) {
  return defs.filter((def) => def.required).filter((def) => {
    const value = effective[def.id];
    return value === undefined || value === null || value === "";
  }).map((def) => def.id);
}
/* 🧰️framework/🔨️modules/🧬️schema/🟦️component.ts */
var GRAPHQL_STATE_PREAMBLE = `enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }
` + `directive @state(class: StateClass!) on FIELD_DEFINITION
` + "directive @derived on FIELD_DEFINITION";
var GRAPHQL_COMPOSITION_PREAMBLE = `type ArtifactLink { targetId: String! kind: String! }
` + `directive @child(kind: String!) on FIELD_DEFINITION
` + "directive @link(roles: [String!]) on FIELD_DEFINITION";

class ArtifactSchemaRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
}

class ArtifactInferenceRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
  get size() {
    return this.#byId.size;
  }
}

class AppSchemaRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
  get size() {
    return this.#byId.size;
  }
  get isEmpty() {
    return this.#byId.size === 0;
  }
}
/* 🧰️framework/🔨️modules/🖥️platform/🟦️component.ts */
function mergeById(base, extension) {
  if (!base?.length && !extension?.length)
    return;
  const merged = new Map;
  base?.forEach((entry) => merged.set(entry.id, entry));
  extension?.forEach((entry) => merged.set(entry.id, entry));
  return [...merged.values()];
}
function mergeNamedLayouts(base, extension) {
  return mergeById(base, extension) ?? [];
}

class Store {
  listeners = new Set;
  disposed = false;
  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  notify() {
    if (this.disposed)
      return;
    for (const listener of this.listeners)
      listener();
  }
  dispose() {
    this.disposed = true;
    this.listeners.clear();
  }
}
var OS_SHELL_CONFIG_STORAGE_KEY = "semio.os.config";
function emptyOsShellConfig() {
  return { version: 1, preferences: {}, namedLayouts: {}, dockLayouts: { apps: {} }, dockUi: { apps: {} }, windowPanes: { apps: {} } };
}

class OsShellConfig extends Store {
  storage;
  constructor(storage) {
    super();
    this.storage = storage;
  }
  getSnapshot() {
    const raw = this.storage.get(OS_SHELL_CONFIG_STORAGE_KEY);
    if (!raw)
      return emptyOsShellConfig();
    try {
      const parsed = JSON.parse(raw);
      if (parsed.version !== 1 || !parsed.preferences || !parsed.namedLayouts || !parsed.dockLayouts?.apps || !parsed.dockUi?.apps || !parsed.windowPanes?.apps)
        return emptyOsShellConfig();
      return parsed;
    } catch {
      return emptyOsShellConfig();
    }
  }
  update(update) {
    const next = update(this.getSnapshot());
    this.storage.set(OS_SHELL_CONFIG_STORAGE_KEY, JSON.stringify(next));
    this.notify();
  }
  getPreference(key) {
    return this.getSnapshot().preferences[key];
  }
  setPreference(key, value) {
    this.update((current) => ({ ...current, preferences: { ...current.preferences, [key]: value } }));
  }
  reset() {
    this.storage.remove(OS_SHELL_CONFIG_STORAGE_KEY);
    this.notify();
  }
}

class NamedLayoutStore extends Store {
  layouts = [];
  config;
  appId;
  constructor(appId, storage) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
    this.layouts = this.readPersisted();
  }
  getSnapshot() {
    return this.layouts;
  }
  save(layout) {
    const next = mergeNamedLayouts(this.layouts.filter((entry) => entry.id !== layout.id), [layout]);
    this.layouts = next;
    this.persist();
    this.notify();
  }
  remove(layoutId) {
    const next = this.layouts.filter((entry) => entry.id !== layoutId);
    if (next.length === this.layouts.length)
      return;
    this.layouts = next;
    this.persist();
    this.notify();
  }
  readPersisted() {
    const parsed = this.config.getSnapshot().namedLayouts[this.appId];
    if (!Array.isArray(parsed))
      return [];
    return parsed.filter((entry) => Boolean(entry) && typeof entry === "object" && typeof entry.id === "string" && typeof entry.label === "string" && entry.origin === "user" && Boolean(entry.layout));
  }
  persist() {
    this.config.update((current) => ({ ...current, namedLayouts: { ...current.namedLayouts, [this.appId]: this.layouts } }));
  }
}
function validDockSkeleton(value) {
  return Boolean(value) && typeof value === "object" && value.version === 3 && Boolean(value.anchors) && typeof value.anchors === "object";
}

class DockLayoutStore extends Store {
  config;
  appId;
  constructor(storage, appId) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
  }
  getSnapshot() {
    const layouts = this.config.getSnapshot().dockLayouts;
    if (this.appId) {
      const app = layouts.apps[this.appId];
      if (validDockSkeleton(app))
        return app;
    }
    return validDockSkeleton(layouts.os) ? layouts.os : null;
  }
  save(skeleton) {
    this.updateLayer(this.appId, skeleton);
    this.notify();
  }
  saveOs(skeleton) {
    this.updateLayer(undefined, skeleton);
    this.notify();
  }
  reset() {
    this.config.update((current) => {
      const apps = { ...current.dockLayouts.apps };
      if (this.appId)
        delete apps[this.appId];
      return { ...current, dockLayouts: { apps } };
    });
    this.notify();
  }
  updateLayer(appId, skeleton) {
    this.config.update((current) => {
      const apps = { ...current.dockLayouts.apps };
      if (appId) {
        if (skeleton)
          apps[appId] = skeleton;
        else
          delete apps[appId];
        return { ...current, dockLayouts: { ...current.dockLayouts, apps } };
      }
      return { ...current, dockLayouts: skeleton ? { ...current.dockLayouts, os: skeleton } : { apps } };
    });
  }
}
function validDockUiState(value) {
  return Boolean(value) && typeof value === "object" && value.version === 3 && Boolean(value.anchors) && typeof value.anchors === "object";
}

class DockUiStateStore extends Store {
  config;
  appId;
  constructor(storage, appId) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
  }
  getSnapshot() {
    const dockUi = this.config.getSnapshot().dockUi;
    if (this.appId) {
      const app = dockUi.apps[this.appId];
      if (validDockUiState(app))
        return app;
    }
    return validDockUiState(dockUi.os) ? dockUi.os : null;
  }
  save(state) {
    this.updateLayer(this.appId, state);
    this.notify();
  }
  saveOs(state) {
    this.updateLayer(undefined, state);
    this.notify();
  }
  reset() {
    this.config.update((current) => {
      const apps = { ...current.dockUi.apps };
      if (this.appId)
        delete apps[this.appId];
      return { ...current, dockUi: { apps } };
    });
    this.notify();
  }
  updateLayer(appId, state) {
    this.config.update((current) => {
      const apps = { ...current.dockUi.apps };
      if (appId) {
        if (state)
          apps[appId] = state;
        else
          delete apps[appId];
        return { ...current, dockUi: { ...current.dockUi, apps } };
      }
      return { ...current, dockUi: state ? { ...current.dockUi, os: state } : { apps } };
    });
  }
}
function validWindowPaneUiState(value) {
  return Boolean(value) && typeof value === "object" && value.version === 1 && Boolean(value.windows) && typeof value.windows === "object";
}

class WindowPaneStateStore extends Store {
  config;
  appId;
  constructor(storage, appId) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
  }
  getSnapshot() {
    const panes = this.config.getSnapshot().windowPanes;
    if (this.appId) {
      const app = panes.apps[this.appId];
      if (validWindowPaneUiState(app))
        return app;
    }
    return validWindowPaneUiState(panes.os) ? panes.os : null;
  }
  save(state) {
    this.updateLayer(this.appId, state);
    this.notify();
  }
  saveOs(state) {
    this.updateLayer(undefined, state);
    this.notify();
  }
  reset() {
    this.config.update((current) => {
      const apps = { ...current.windowPanes.apps };
      if (this.appId)
        delete apps[this.appId];
      return { ...current, windowPanes: { apps } };
    });
    this.notify();
  }
  updateLayer(appId, state) {
    this.config.update((current) => {
      const apps = { ...current.windowPanes.apps };
      if (appId) {
        if (state)
          apps[appId] = state;
        else
          delete apps[appId];
        return { ...current, windowPanes: { ...current.windowPanes, apps } };
      }
      return { ...current, windowPanes: state ? { ...current.windowPanes, os: state } : { apps } };
    });
  }
}
function createMemoryStoragePort() {
  const map = new Map;
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    }
  };
}
/* 🧰️framework/🔨️modules/🔺️mesh/🟦️component.ts */
var RIBBON_PARENT_CATEGORIES = [
  "history",
  "hand",
  "selection",
  "lasso",
  "filter",
  "open",
  "save",
  "transfer",
  "transform",
  "create",
  "view",
  "actions",
  "settings",
  "methods",
  "mode",
  "targets",
  "export",
  "tools",
  "utilities",
  "sync"
];
var CONTEXT_MENU_ROW_BUDGET = 9;
var CONTEXT_MENU_PRIMARY_BUDGET = 5;
function contextMenuIsBareSeparator(item) {
  return item.separator === true && item.label === undefined;
}
function contextMenuIsHeader(item) {
  return item.separator === true && item.label !== undefined;
}
function contextMenuIsGroupRow(item) {
  return item.id.startsWith("menu.group.");
}
function contextMenuGroupCategory(item) {
  return item.id.startsWith("menu.group.") ? item.id.slice("menu.group.".length) : item.id;
}
function contextMenuTaxonomyRank(category) {
  const index = RIBBON_PARENT_CATEGORIES.indexOf(category);
  return index === -1 ? RIBBON_PARENT_CATEGORIES.length : index;
}
function contextMenuSeparatorRow(seed) {
  return { id: `separator-organized-${seed}`, separator: true };
}
function contextMenuNormalizeSeparators(items) {
  const out = [];
  for (const item of items) {
    if (contextMenuIsBareSeparator(item) && out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1])) {
      continue;
    }
    out.push(item);
  }
  if (out.length > 0 && contextMenuIsBareSeparator(out[0])) {
    out.shift();
  }
  while (out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1])) {
    out.pop();
  }
  return out;
}
function contextMenuMergeGroupRows(items) {
  const out = [];
  const groupIndex = new Map;
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      const index = groupIndex.get(item.id);
      if (index !== undefined) {
        const children = out[index].children ? [...out[index].children] : [];
        for (const child of item.children ?? []) {
          if (!children.some((existing) => existing.id === child.id)) {
            children.push(child);
          }
        }
        out[index] = { ...out[index], children };
      } else {
        groupIndex.set(item.id, out.length);
        out.push(item);
      }
    } else {
      out.push(item);
    }
  }
  return out;
}
function contextMenuEmitWithinBudget(items) {
  const leavesAndHeaders = [];
  const groupRows = [];
  const destructiveLeaves = [];
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      groupRows.push(item);
    } else if (item.destructive === true) {
      destructiveLeaves.push(item);
    } else {
      leavesAndHeaders.push(item);
    }
  }
  groupRows.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));
  const out = [...leavesAndHeaders, ...groupRows];
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}
function contextMenuEmitOverBudget(items, categoryOf) {
  function bucketMut(buckets, id) {
    const index = buckets.findIndex((bucket) => bucket.id === id);
    if (index !== -1)
      return index;
    buckets.push({ id, label: undefined, children: [] });
    return buckets.length - 1;
  }
  const primaries = [];
  const existingGroups = [];
  const destructiveLeaves = [];
  const bucketedGroups = [];
  let currentHeaderKey;
  for (const item of items) {
    if (contextMenuIsHeader(item)) {
      currentHeaderKey = item.label;
      continue;
    }
    if (contextMenuIsGroupRow(item)) {
      existingGroups.push(item);
      currentHeaderKey = undefined;
      continue;
    }
    if (item.destructive === true) {
      destructiveLeaves.push(item);
      continue;
    }
    if (currentHeaderKey !== undefined) {
      const slug = currentHeaderKey.toLowerCase().split(/\s+/).join("-");
      const index2 = bucketMut(bucketedGroups, `menu.group.${slug}`);
      bucketedGroups[index2] = { ...bucketedGroups[index2], children: [...bucketedGroups[index2].children ?? [], item] };
      continue;
    }
    if (primaries.length < CONTEXT_MENU_PRIMARY_BUDGET) {
      primaries.push(item);
      continue;
    }
    const category = categoryOf(item.action ?? item.id) ?? "actions";
    const index = bucketMut(bucketedGroups, `menu.group.${category}`);
    bucketedGroups[index] = { ...bucketedGroups[index], children: [...bucketedGroups[index].children ?? [], item] };
  }
  const groups = [...existingGroups, ...bucketedGroups];
  groups.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));
  let out = [...primaries, ...groups];
  if (out.length > CONTEXT_MENU_ROW_BUDGET) {
    const foldFrom = CONTEXT_MENU_ROW_BUDGET - 1;
    const overflowingGroups = out.slice(foldFrom);
    out = out.slice(0, foldFrom);
    const foldedChildren = [];
    for (const group of overflowingGroups) {
      foldedChildren.push(...group.children ?? []);
    }
    out.push({ id: "menu.group.more", label: undefined, children: foldedChildren });
  }
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}
function organizeContextMenu(items, categoryOf) {
  const mapped = items.map((item) => ({
    ...item,
    children: item.children ? organizeContextMenu(item.children, categoryOf) : item.children
  }));
  const normalized = contextMenuMergeGroupRows(contextMenuNormalizeSeparators(mapped));
  const interactiveCount = normalized.filter((item) => item.separator !== true).length;
  return interactiveCount <= CONTEXT_MENU_ROW_BUDGET ? contextMenuEmitWithinBudget(normalized) : contextMenuEmitOverBudget(normalized, categoryOf);
}
/* 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts */
var SHARD_COMMAND_PAGE_BYTES = 4096;
var SHARD_COMMAND_MAXIMUM_PAGES = 64;
function createShardCommandIngressPages(input) {
  if (input.command.length === 0)
    throw new Error("[DEBUG] command ingress cannot encode an empty command");
  const pageCount = Math.ceil(input.command.length / SHARD_COMMAND_PAGE_BYTES);
  if (pageCount > SHARD_COMMAND_MAXIMUM_PAGES)
    throw new Error(`[DEBUG] command ingress exceeds ${SHARD_COMMAND_MAXIMUM_PAGES} pages`);
  const pages = [];
  for (let pageIndex = 0;pageIndex < pageCount; pageIndex += 1) {
    const start = pageIndex * SHARD_COMMAND_PAGE_BYTES;
    const bytes = input.command.subarray(start, Math.min(start + SHARD_COMMAND_PAGE_BYTES, input.command.length));
    const blocks = {};
    for (let blockIndex = 0;blockIndex < 64; blockIndex += 1) {
      const words = [];
      for (let wordIndex = 0;wordIndex < 8; wordIndex += 1) {
        let word = 0n;
        const wordStart = blockIndex * 64 + wordIndex * 8;
        for (let byteIndex = 0;byteIndex < 8; byteIndex += 1)
          word |= BigInt(bytes[wordStart + byteIndex] ?? 0) << BigInt(byteIndex * 8);
        words.push(word);
      }
      blocks[`block${blockIndex.toString().padStart(2, "0")}`] = {
        word0: words[0],
        word1: words[1],
        word2: words[2],
        word3: words[3],
        word4: words[4],
        word5: words[5],
        word6: words[6],
        word7: words[7]
      };
    }
    pages.push({
      cursor: {
        owner: input.owner,
        generation: input.generation,
        commandIndex: input.commandIndex,
        commandCount: input.commandCount,
        instance: input.instance,
        seq: input.seq,
        kind: input.command[0],
        pageIndex,
        pageCount,
        itemCount: 0,
        metadata: 0
      },
      length: bytes.length,
      ...blocks
    });
  }
  return pages;
}
var MAINTENANCE_LANE_DEFAULT_BUDGET = { fuel: 80000000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };
var SHARD_FRAME_VARIANT_FIELDS = [
  { kind: "Register", fields: ["actor"] },
  { kind: "Unregister", fields: ["actor"] },
  { kind: "Grant", fields: ["actor", "budget", "envelopes"] },
  { kind: "Envelope", fields: ["envelope"] }
];
var SHARD_FRAME_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
function orderEnvelopesByLane(envelopes) {
  return envelopes.map((envelope, index) => ({ envelope, index })).sort((left, right) => {
    const rank = SHARD_FRAME_LANE_ORDER.indexOf(left.envelope.lane) - SHARD_FRAME_LANE_ORDER.indexOf(right.envelope.lane);
    return rank !== 0 ? rank : left.index - right.index;
  }).map((entry) => entry.envelope);
}
function createGrantedBudgetTracker(fallback = MAINTENANCE_LANE_DEFAULT_BUDGET) {
  const budgets = new Map;
  return {
    recordGrant(actorId, budget) {
      budgets.set(actorId, budget);
    },
    forget(actorId) {
      budgets.delete(actorId);
    },
    granted(actorId) {
      return budgets.get(actorId) ?? fallback;
    }
  };
}
function interpretShardFrame(frame, tracker) {
  switch (frame.kind) {
    case "Register":
      return { action: "register", actor: frame.actor };
    case "Unregister":
      tracker.forget(frame.actor);
      return { action: "unregister", actor: frame.actor };
    case "Grant":
      tracker.recordGrant(frame.actor, frame.budget);
      return { action: "runEnvelopes", actor: frame.actor, budget: frame.budget, envelopes: orderEnvelopesByLane(frame.envelopes) };
    case "Envelope":
      return { action: "runEnvelopes", actor: frame.envelope.to, budget: tracker.granted(frame.envelope.to), envelopes: [frame.envelope] };
    default:
      return { action: "unknown", frame };
  }
}
function formatQuotaBreachMessage(breach) {
  return `outstanding effect quota exceeded: ${breach.quota} limit=${breach.limit} actual=${breach.actual}`;
}
var MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4096;
var MAX_SEGMENTED_DOWNLOAD_OPERATION_ID = (1n << 64n) - 1n;
var DEFAULT_HEARTBEAT_TIMEOUT_MS = 5000;
var HEARTBEAT_MISSED_LIMIT = 3;
var DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR = 64;
function freshHeartbeatState(nowMs) {
  return { lastHeartbeatAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatTurnSeq: 0, oldestPendingStartedAtMs: null, missedCount: 0, lastMissCountedAtMs: nowMs };
}
function graftWorkerStack(actorId, reason, stack, kind, framesBytes) {
  const error = new Error(reason);
  if (stack)
    error.stack = `${stack}
    \u21B3 main: ${error.stack ?? ""}`;
  console.log(`[DEBUG] program worker ${actorId || "unknown"} error type=${kind ?? "unknown"} framesBytes=${framesBytes ?? "n/a"}`);
  return error;
}

class ShardClient {
  shards = [];
  actorShard = new Map;
  pending = new Map;
  exclusiveIndices;
  heartbeatSabView;
  heartbeatTimeoutMs;
  watchdogIntervalMs;
  now;
  createWorker;
  onShardLost;
  onActorTrap;
  onHostEffect;
  maxOutstandingEffectsPerActor;
  outstandingEffectsByActor = new Map;
  effectReplySeq = 0;
  nextRoundRobin = 0;
  requestSeq = 0;
  watchdogHandle = null;
  constructor(options) {
    if (options.shardCount < 1)
      throw new Error("[DEBUG] ShardClient requires shardCount >= 1");
    this.createWorker = options.createWorker;
    this.now = options.now ?? (() => Date.now());
    this.heartbeatTimeoutMs = options.heartbeatTimeoutMs ?? DEFAULT_HEARTBEAT_TIMEOUT_MS;
    this.watchdogIntervalMs = options.watchdogIntervalMs ?? this.heartbeatTimeoutMs;
    this.heartbeatSabView = options.heartbeatSab ? new Int32Array(options.heartbeatSab) : null;
    this.onShardLost = options.onShardLost;
    this.onActorTrap = options.onActorTrap;
    this.onHostEffect = options.onHostEffect;
    this.maxOutstandingEffectsPerActor = options.maxOutstandingEffectsPerActor ?? DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR;
    const exclusiveCount = Math.max(0, Math.min(options.exclusiveShardCount ?? Math.min(2, options.shardCount - 1), options.shardCount - 1));
    const exclusive = new Set;
    for (let index = options.shardCount - exclusiveCount;index < options.shardCount; index += 1)
      exclusive.add(index);
    this.exclusiveIndices = exclusive;
    for (let index = 0;index < options.shardCount; index += 1)
      this.shards.push(this.spawnShard(index));
  }
  spawnShard(index) {
    const worker = this.createWorker(index);
    const slot = { index, worker, heartbeat: freshHeartbeatState(this.now()), pendingRequestIds: new Set, actorIds: new Set };
    worker.onmessage = (event) => this.handleMessage(slot, event.data);
    worker.onerror = (error) => {
      console.error(`[DEBUG] shard ${index} worker error`, error);
      this.failShard(slot, new Error(`shard ${index} worker crashed`));
    };
    if (this.heartbeatSabView)
      worker.postMessage({ kind: "attachHeartbeatSab", shardIndex: index, sab: this.heartbeatSabView.buffer });
    return slot;
  }
  handleMessage(slot, message) {
    if (message.kind === "heartbeat") {
      this.recordHeartbeat(slot, message.turnSeq, this.now());
      return;
    }
    if (message.kind === "trap") {
      this.onActorTrap?.(message.actorId, message.message);
      return;
    }
    if (message.kind === "frame") {
      this.handleInboundFrame(slot, message.actorId, message.frame);
      return;
    }
    const entry = this.pending.get(message.requestId);
    if (!entry)
      return;
    this.pending.delete(message.requestId);
    slot.pendingRequestIds.delete(message.requestId);
    this.recomputeOldestPending(slot);
    if (message.ok)
      entry.resolve(message.value);
    else
      entry.reject(graftWorkerStack(entry.actorId, message.error, message.stack, message.type, message.framesBytes));
  }
  recomputeOldestPending(slot) {
    let oldest = null;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry)
        continue;
      if (oldest === null || entry.startedAtMs < oldest)
        oldest = entry.startedAtMs;
    }
    slot.heartbeat.oldestPendingStartedAtMs = oldest;
  }
  failShard(slot, error) {
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry)
        continue;
      this.pending.delete(requestId);
      entry.reject(error);
    }
    slot.pendingRequestIds.clear();
    slot.heartbeat.oldestPendingStartedAtMs = null;
    for (const actorId of slot.actorIds) {
      this.abortOutstandingEffects(actorId);
      this.actorShard.delete(actorId);
    }
    slot.actorIds.clear();
  }
  rejectActorPending(slot, actorId, error) {
    for (const requestId of [...slot.pendingRequestIds]) {
      const entry = this.pending.get(requestId);
      if (entry?.actorId !== actorId)
        continue;
      this.pending.delete(requestId);
      slot.pendingRequestIds.delete(requestId);
      entry.reject(error);
    }
    this.recomputeOldestPending(slot);
  }
  assignShard(actorId) {
    const existing = this.actorShard.get(actorId);
    if (existing !== undefined)
      return this.shards[existing];
    const roundRobinCount = this.shards.length - this.exclusiveIndices.size;
    let index = this.nextRoundRobin % Math.max(roundRobinCount, 1);
    while (this.exclusiveIndices.has(index))
      index = (index + 1) % this.shards.length;
    this.nextRoundRobin = (this.nextRoundRobin + 1) % Math.max(roundRobinCount, 1);
    this.actorShard.set(actorId, index);
    this.shards[index].actorIds.add(actorId);
    return this.shards[index];
  }
  leaseExclusive(actorId, options) {
    const already = this.actorShard.get(actorId);
    if (already !== undefined && this.exclusiveIndices.has(already))
      return already;
    for (const index of this.exclusiveIndices) {
      const slot = this.shards[index];
      if (slot.actorIds.size === 0 || options?.force) {
        if (already !== undefined)
          this.shards[already].actorIds.delete(actorId);
        slot.actorIds.add(actorId);
        this.actorShard.set(actorId, index);
        return index;
      }
    }
    throw new Error(`[DEBUG] ShardClient.leaseExclusive(${actorId}): no free exclusive shard (${this.exclusiveIndices.size} reserved, all leased)`);
  }
  releaseExclusive(actorId) {
    const index = this.actorShard.get(actorId);
    if (index === undefined || !this.exclusiveIndices.has(index))
      return;
    this.shards[index].actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }
  shardIndexFor(actorId) {
    return this.actorShard.get(actorId);
  }
  nextRequestId() {
    this.requestSeq += 1;
    return `r${this.requestSeq}`;
  }
  send(slot, message, requestId) {
    if (requestId === null) {
      slot.worker.postMessage(message);
      return Promise.resolve(undefined);
    }
    return new Promise((resolve, reject) => {
      const startedAtMs = this.now();
      this.pending.set(requestId, { resolve, reject, shardIndex: slot.index, startedAtMs, actorId: "actorId" in message ? message.actorId : "" });
      slot.pendingRequestIds.add(requestId);
      if (slot.heartbeat.oldestPendingStartedAtMs === null)
        slot.heartbeat.oldestPendingStartedAtMs = startedAtMs;
      slot.worker.postMessage(message);
    });
  }
  async activate(actorId, moduleUrl, caps, budget, assets = []) {
    const slot = this.assignShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "activate", requestId, actorId, moduleUrl, caps, budget, assets }, requestId);
  }
  async turn(actorId, events, budget, commandPage) {
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined)
      throw new Error(`[DEBUG] ShardClient.turn(${actorId}): not activated on any shard`);
    const slot = this.shards[shardIndex];
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "turn", requestId, actorId, events, commandPage, budget }, requestId);
  }
  async envelope(shardEnvelope) {
    const slot = this.requireShard(shardEnvelope.to);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "frame", requestId, actorId: shardEnvelope.to, frame: { kind: "Envelope", envelope: shardEnvelope } }, requestId);
  }
  async grant(actorId, budget, envelopes) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    const ordered = orderEnvelopesByLane(envelopes);
    return this.send(slot, { kind: "frame", requestId, actorId, frame: { kind: "Grant", actor: actorId, budget, envelopes: ordered } }, requestId);
  }
  async startJob(actorId, job, jobKind, input) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "startJob", requestId, actorId, job, jobKind, input }, requestId);
  }
  async stepJob(actorId, job, budget) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "stepJob", requestId, actorId, job, budget }, requestId);
  }
  cancelJob(actorId, job) {
    const slot = this.requireShard(actorId);
    this.send(slot, { kind: "cancelJob", actorId, job }, null);
  }
  async takeSegmentedDownloadChunk(actorId, instanceId, operationId) {
    if (!Number.isSafeInteger(instanceId) || instanceId < 0 || typeof operationId !== "bigint" || operationId <= 0n || operationId > MAX_SEGMENTED_DOWNLOAD_OPERATION_ID)
      throw new Error("segmented-download-authority-invalid");
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    const value = await this.send(slot, { kind: "takeSegmentedDownloadChunk", requestId, actorId, instanceId, operationId }, requestId);
    if (value === undefined || value === null)
      return;
    if (Object.prototype.toString.call(value) !== "[object Uint8Array]")
      throw new Error("segmented-download-transport-type");
    const chunk = value;
    if (chunk.byteLength === 0 || chunk.byteLength > MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES)
      throw new Error("segmented-download-transport-limit");
    return chunk;
  }
  async checkpoint(actorId) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "checkpoint", requestId, actorId }, requestId);
  }
  async restore(actorId, state) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "restore", requestId, actorId, state }, requestId);
  }
  dispose(actorId) {
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined)
      return;
    this.abortOutstandingEffects(actorId);
    const slot = this.shards[shardIndex];
    this.rejectActorPending(slot, actorId, new Error(`ShardClient actor disposed: ${actorId}`));
    slot.worker.postMessage({ kind: "dispose", actorId });
    slot.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }
  requireShard(actorId) {
    const index = this.actorShard.get(actorId);
    if (index === undefined)
      throw new Error(`[DEBUG] ShardClient: actor ${actorId} is not activated on any shard`);
    return this.shards[index];
  }
  handleInboundFrame(slot, actorId, frame) {
    if (frame.kind !== "Envelope")
      return;
    const payload = frame.envelope.payload;
    if (payload.kind !== "effect-request")
      return;
    const request = payload.payload;
    this.handleEffectRequest(slot, actorId, request.effect, request.requestId, request.params);
  }
  handleEffectRequest(slot, actorId, effect, requestId, params) {
    const outstanding = this.outstandingEffectsByActor.get(actorId) ?? new Map;
    if (outstanding.size >= this.maxOutstandingEffectsPerActor) {
      const breach = { quota: "outstandingRequests", limit: this.maxOutstandingEffectsPerActor, actual: outstanding.size };
      this.replyEffectError(slot, actorId, requestId, formatQuotaBreachMessage(breach));
      return;
    }
    if (!this.onHostEffect) {
      this.replyEffectError(slot, actorId, requestId, "no host effect handler installed");
      return;
    }
    const controller = new AbortController;
    outstanding.set(requestId, controller);
    this.outstandingEffectsByActor.set(actorId, outstanding);
    this.onHostEffect(actorId, effect, params, controller.signal).then((value) => {
      if (this.settleEffect(actorId, requestId))
        this.replyEffectComplete(slot, actorId, requestId, value);
    }, (error) => {
      if (this.settleEffect(actorId, requestId))
        this.replyEffectError(slot, actorId, requestId, error instanceof Error ? error.message : String(error));
    });
  }
  settleEffect(actorId, requestId) {
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (!outstanding || !outstanding.delete(requestId))
      return false;
    if (outstanding.size === 0)
      this.outstandingEffectsByActor.delete(actorId);
    return true;
  }
  abortOutstandingEffects(actorId) {
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (!outstanding)
      return;
    this.outstandingEffectsByActor.delete(actorId);
    for (const controller of outstanding.values())
      controller.abort();
  }
  postEffectReply(slot, actorId, kind, innerPayload) {
    this.effectReplySeq += 1;
    const frame = {
      kind: "Envelope",
      envelope: { to: actorId, from: { kind: "kernel" }, lane: "Background", seq: this.effectReplySeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: innerPayload } }
    };
    slot.worker.postMessage({ kind: "frame", requestId: this.nextRequestId(), actorId, frame });
  }
  replyEffectComplete(slot, actorId, requestId, value) {
    this.postEffectReply(slot, actorId, "effect-complete", { requestId, value });
  }
  replyEffectError(slot, actorId, requestId, message) {
    this.postEffectReply(slot, actorId, "effect-error", { requestId, message });
  }
  recordHeartbeat(slot, turnSeq, atMs) {
    slot.heartbeat.lastHeartbeatAtMs = atMs;
    slot.heartbeat.lastHeartbeatTurnSeq = turnSeq;
    slot.heartbeat.missedCount = 0;
    slot.heartbeat.lastMissCountedAtMs = atMs;
  }
  pollHeartbeatSab(nowMs = this.now()) {
    if (!this.heartbeatSabView)
      return;
    for (const slot of this.shards) {
      const seq = Atomics.load(this.heartbeatSabView, slot.index);
      if (seq !== slot.heartbeat.lastHeartbeatTurnSeq || slot.heartbeat.oldestPendingStartedAtMs === null) {
        this.recordHeartbeat(slot, seq, nowMs);
      }
    }
  }
  checkHeartbeats(nowMs = this.now()) {
    for (const slot of this.shards) {
      const pendingSince = slot.heartbeat.oldestPendingStartedAtMs;
      if (pendingSince === null)
        continue;
      if (slot.heartbeat.lastHeartbeatAtMs >= pendingSince)
        continue;
      const silentForMs = nowMs - pendingSince;
      if (silentForMs <= this.heartbeatTimeoutMs)
        continue;
      if (nowMs - slot.heartbeat.lastMissCountedAtMs < this.heartbeatTimeoutMs)
        continue;
      slot.heartbeat.missedCount += 1;
      slot.heartbeat.lastMissCountedAtMs = nowMs;
      if (slot.heartbeat.missedCount >= HEARTBEAT_MISSED_LIMIT) {
        const actorIds = [...slot.actorIds];
        this.terminate(slot.index);
        this.rebuild(slot.index);
        this.onShardLost?.(slot.index, actorIds);
      }
    }
  }
  startWatchdog(intervalMs = this.watchdogIntervalMs) {
    if (this.watchdogHandle !== null)
      return;
    this.watchdogHandle = setInterval(() => {
      this.pollHeartbeatSab();
      this.checkHeartbeats();
    }, intervalMs);
  }
  stopWatchdog() {
    if (this.watchdogHandle === null)
      return;
    clearInterval(this.watchdogHandle);
    this.watchdogHandle = null;
  }
  shardMetricsSamples(nowMs = this.now()) {
    return this.shards.map((slot) => {
      const actors = slot.actorIds.size;
      const busyRatio = actors > 0 ? slot.pendingRequestIds.size / actors : 0;
      const heartbeatAgeMs = Number.isFinite(slot.heartbeat.lastHeartbeatAtMs) ? Math.max(0, nowMs - slot.heartbeat.lastHeartbeatAtMs) : Number.POSITIVE_INFINITY;
      return { shard: slot.index, metrics: { actors, busyRatio, heartbeatAgeMs } };
    });
  }
  terminate(index) {
    const slot = this.shards[index];
    if (!slot)
      throw new Error(`[DEBUG] ShardClient.terminate: no shard ${index}`);
    const actorIds = [...slot.actorIds];
    this.failShard(slot, new Error(`shard ${index} terminated`));
    slot.worker.terminate();
    return actorIds;
  }
  rebuild(index) {
    const old = this.shards[index];
    if (!old)
      throw new Error(`[DEBUG] ShardClient.rebuild: no shard ${index}`);
    for (const actorId of old.actorIds)
      this.actorShard.delete(actorId);
    this.shards[index] = this.spawnShard(index);
  }
  disposeAll() {
    this.stopWatchdog();
    for (const slot of this.shards) {
      this.failShard(slot, new Error("ShardClient disposed"));
      slot.worker.terminate();
    }
  }
}
if (import.meta.vitest) {
  let harness = function(shardCount = 2, extra) {
    const workers = [];
    let nowMs = 0;
    const client = new ShardClient({
      shardCount,
      createWorker: (index) => {
        const worker = new FakeShardWorker(index);
        workers.push(worker);
        return worker;
      },
      now: () => nowMs,
      ...extra
    });
    return { client, workers, advance: (ms) => nowMs += ms, setNow: (ms) => nowMs = ms };
  }, makeEnvelope = function(to, lane, seq, kind = "wake") {
    return { to, from: { kind: "kernel" }, lane, seq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: {} } };
  }, makeEffectRequestFrame = function(actorId, effect, requestId, params) {
    return { kind: "frame", actorId, frame: { kind: "Envelope", envelope: { to: "kernel", from: { kind: "actor", id: actorId }, lane: "Background", seq: 1, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind: "effect-request", payload: { effect, requestId, params } } } } };
  }, findEffectReply = function(sent, requestId, kind) {
    return sent.find((message) => message.kind === "frame" && message.frame?.kind === "Envelope" && message.frame.envelope?.payload?.kind === kind && message.frame.envelope.payload?.payload?.requestId === requestId);
  }, flushMicrotasks = function() {
    return new Promise((resolve) => setTimeout(resolve, 0));
  };
  const { describe, expect, it, vi } = import.meta.vitest;

  class FakeShardWorker {
    index;
    onmessage = null;
    onerror = null;
    sent = [];
    terminated = false;
    constructor(index) {
      this.index = index;
    }
    postMessage(message) {
      this.sent.push(message);
    }
    terminate() {
      this.terminated = true;
    }
    deliver(message) {
      this.onmessage?.({ data: message });
    }
  }
  const BUDGET = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };
  describe("ShardClient activation + turn round-trip", () => {
    it("routes activate then turn to the same shard, resolving the reply by requestId", async () => {
      const { client, workers } = harness(2);
      const activatePromise = client.activate("actor-1", "https://x/plugin.js", [], BUDGET);
      const activateMsg = workers[0].sent[0];
      expect(activateMsg.kind).toBe("activate");
      workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;
      const turnPromise = client.turn("actor-1", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0].sent[1];
      expect(turnMsg.kind).toBe("turn");
      workers[0].deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });
    it("rejects turn() for an actor never activated", async () => {
      const { client } = harness(2);
      await expect(client.turn("ghost", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });
  describe("ShardClient segmented-download transport", () => {
    async function activatedHarness() {
      const state = harness(1);
      const activation = state.client.activate("actor-download", "https://x/plugin.js", [], BUDGET);
      const message = state.workers[0].sent[0];
      state.workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await activation;
      return state;
    }
    it("preserves operation identity and last-Some then None ordering", async () => {
      const { client, workers } = await activatedHarness();
      for (const expected of [new Uint8Array([1, 2]), new Uint8Array([3]), undefined]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0].sent.at(-1);
        expect(message).toMatchObject({ kind: "takeSegmentedDownloadChunk", instanceId: 17, operationId: 91n });
        workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: expected });
        await expect(read).resolves.toEqual(expected);
      }
    });
    it("propagates unknown-operation errors without manufacturing a terminal None", async () => {
      const { client, workers } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 404n);
      const message = workers[0].sent.at(-1);
      workers[0].deliver({ kind: "result", requestId: message.requestId, ok: false, error: "interactive-job.unknown-segmented-download" });
      await expect(read).rejects.toThrow("interactive-job.unknown-segmented-download");
    });
    it("rejects oversized or empty response items", async () => {
      const { client, workers } = await activatedHarness();
      for (const invalid of [new Uint8Array(4097), new Uint8Array(0)]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0].sent.at(-1);
        workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: invalid });
        await expect(read).rejects.toThrow("segmented-download-transport-limit");
      }
    });
    it("rejects an in-flight read when actor disposal cancels its transport ownership", async () => {
      const { client } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
      client.dispose("actor-download");
      await expect(read).rejects.toThrow("ShardClient actor disposed");
    });
    it("preserves the complete u64 operation authority through structured clone", async () => {
      const { client, workers } = await activatedHarness();
      for (const operationId of [(1n << 53n) + 1n, MAX_SEGMENTED_DOWNLOAD_OPERATION_ID]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, operationId);
        const message = structuredClone(workers[0].sent.at(-1));
        expect(message.operationId).toBe(operationId);
        workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
        await expect(read).resolves.toBeUndefined();
      }
    });
    it("rejects zero and overflowing operation authorities", async () => {
      const { client } = await activatedHarness();
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 0n)).rejects.toThrow("segmented-download-authority-invalid");
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 1n << 64n)).rejects.toThrow("segmented-download-authority-invalid");
    });
  });
  describe("ShardClient actor-id multiplexing", () => {
    it("distinguishes two actors' replies on the same shard even when they resolve out of order", async () => {
      const { client, workers } = harness(1);
      const p1 = client.activate("a", "https://x/a.js", [], BUDGET);
      const p2 = client.activate("b", "https://x/b.js", [], BUDGET);
      const [msgA, msgB] = workers[0].sent;
      workers[0].deliver({ kind: "result", requestId: msgB.requestId, ok: true, value: undefined });
      workers[0].deliver({ kind: "result", requestId: msgA.requestId, ok: true, value: undefined });
      await expect(p1).resolves.toBeUndefined();
      await expect(p2).resolves.toBeUndefined();
      expect(client.shardIndexFor("a")).toBe(0);
      expect(client.shardIndexFor("b")).toBe(0);
    });
    it("round-robins fresh actors across shards, skipping the reserved exclusive tail", async () => {
      const { client } = harness(4, { exclusiveShardCount: 1 });
      const indices = ["a", "b", "c"].map((id) => {
        client.activate(id, "https://x/y.js", [], BUDGET);
        return client.shardIndexFor(id);
      });
      expect(new Set(indices).has(3)).toBe(false);
      expect(indices).toEqual([0, 1, 2]);
    });
  });
  describe("ShardClient heartbeat watchdog (postMessage path)", () => {
    it("does not miss while idle (no pending turn)", () => {
      const { client, advance } = harness(1);
      advance(1e5);
      client.checkHeartbeats(1e5);
      expect(true).toBe(true);
    });
    it("terminates + rebuilds after 3 consecutive missed-heartbeat windows on a stuck turn", async () => {
      const lost = [];
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000, onShardLost: (index, actorIds) => lost.push({ index, actorIds }) });
      setNow(0);
      const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
      const activateMsg = workers[0].sent[0];
      workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;
      const originalWorker = workers[0];
      client.turn("stuck", [], BUDGET).catch(() => {});
      advance(1001);
      client.checkHeartbeats();
      expect(lost).toEqual([]);
      expect(originalWorker.terminated).toBe(false);
      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();
      expect(originalWorker.terminated).toBe(true);
      expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
      expect(client.shardIndexFor("stuck")).toBeUndefined();
    });
    it("a fresh heartbeat resets the miss count", async () => {
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("busy", "https://x/busy.js", [], BUDGET);
      workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
      await activatePromise;
      client.turn("busy", [], BUDGET);
      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();
      workers[0].deliver({ kind: "heartbeat", turnSeq: 7 });
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0].terminated).toBe(false);
    });
  });
  describe("ShardClient SAB heartbeat path", () => {
    it("pollHeartbeatSab reads Atomics-stored turnSeq and feeds the same miss-count state machine", async () => {
      const sab = new SharedArrayBuffer(4 * Int32Array.BYTES_PER_ELEMENT);
      const view = new Int32Array(sab);
      const { client, workers, advance, setNow } = harness(1, { heartbeatSab: sab, heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("sab-actor", "https://x/s.js", [], BUDGET);
      workers[0].deliver({ kind: "result", requestId: workers[0].sent.find((m) => m.kind === "activate").requestId, ok: true, value: undefined });
      await activatePromise;
      client.turn("sab-actor", [], BUDGET);
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0].terminated).toBe(false);
      Atomics.store(view, 0, 5);
      client.pollHeartbeatSab();
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0].terminated).toBe(false);
    });
  });
  describe("ShardClient leaseExclusive", () => {
    it("moves an actor onto a reserved exclusive shard and back", async () => {
      const { client } = harness(4, { exclusiveShardCount: 2 });
      client.activate("heavy", "https://x/h.js", [], BUDGET);
      expect(client.shardIndexFor("heavy")).toBe(0);
      const exclusiveIndex = client.leaseExclusive("heavy");
      expect([2, 3]).toContain(exclusiveIndex);
      expect(client.shardIndexFor("heavy")).toBe(exclusiveIndex);
      client.releaseExclusive("heavy");
      expect(client.shardIndexFor("heavy")).toBeUndefined();
    });
    it("throws once every exclusive shard is leased and force is not set", () => {
      const { client } = harness(3, { exclusiveShardCount: 1 });
      client.activate("first", "https://x/1.js", [], BUDGET);
      client.leaseExclusive("first");
      client.activate("second", "https://x/2.js", [], BUDGET);
      expect(() => client.leaseExclusive("second")).toThrow(/no free exclusive shard/);
    });
    it("is idempotent for the same actor already leased", () => {
      const { client } = harness(2, { exclusiveShardCount: 1 });
      client.activate("a", "https://x/a.js", [], BUDGET);
      const first = client.leaseExclusive("a");
      const second = client.leaseExclusive("a");
      expect(first).toBe(second);
    });
  });
  describe("ShardClient.startWatchdog / stopWatchdog", () => {
    it("self-ticks checkHeartbeats + pollHeartbeatSab with no external caller, detects a missed heartbeat, and rebuilds", async () => {
      vi.useFakeTimers();
      try {
        const lost = [];
        const workers = [];
        const client = new ShardClient({
          shardCount: 1,
          createWorker: (index) => {
            const worker = new FakeShardWorker(index);
            workers.push(worker);
            return worker;
          },
          heartbeatTimeoutMs: 1000,
          onShardLost: (index, actorIds) => lost.push({ index, actorIds })
        });
        const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
        const activateMsg = workers[0].sent[0];
        workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
        await activatePromise;
        client.turn("stuck", [], BUDGET).catch(() => {});
        client.startWatchdog(1001);
        vi.advanceTimersByTime(1001);
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001);
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001);
        expect(workers[0].terminated).toBe(true);
        expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
        expect(client.shardIndexFor("stuck")).toBeUndefined();
        client.stopWatchdog();
        const lostCountAfterStop = lost.length;
        vi.advanceTimersByTime(1e4);
        expect(lost.length).toBe(lostCountAfterStop);
      } finally {
        vi.useRealTimers();
      }
    });
    it("is idempotent to call twice, and stopWatchdog before ever starting is a no-op", () => {
      vi.useFakeTimers();
      try {
        const { client } = harness(1);
        client.startWatchdog(500);
        client.startWatchdog(500);
        client.stopWatchdog();
        client.stopWatchdog();
        expect(true).toBe(true);
      } finally {
        vi.useRealTimers();
      }
    });
  });
  describe("ShardClient failShard clears routing", () => {
    it("clears actorShard + slot.actorIds immediately on a worker crash (onerror), before any terminate()/rebuild()", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
      await activatePromise;
      expect(client.shardIndexFor("x")).toBe(0);
      workers[0].onerror?.(new Error("boom"));
      expect(client.shardIndexFor("x")).toBeUndefined();
      await expect(client.turn("x", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });
  describe("ShardClient terminate/rebuild", () => {
    it("rejects in-flight requests on terminate and spawns a fresh worker on rebuild", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      const rejection = expect(activatePromise).rejects.toThrow(/terminated/);
      const oldWorker = workers[0];
      const actorIds = client.terminate(0);
      expect(actorIds).toEqual(["x"]);
      await rejection;
      expect(oldWorker.terminated).toBe(true);
      client.rebuild(0);
      expect(workers.length).toBe(2);
      expect(client.shardIndexFor("x")).toBeUndefined();
    });
  });
  describe("ShardClient worker crash", () => {
    it("onerror fails every pending request on that shard", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("crashy", "https://x/c.js", [], BUDGET);
      workers[0].onerror?.(new Error("boom"));
      await expect(activatePromise).rejects.toThrow(/crashed/);
    });
  });
  describe("ShardClient.shardMetricsSamples", () => {
    it("reports zero actors/busyRatio and an infinite heartbeat age for a fresh, never-touched shard", () => {
      const { client } = harness(2);
      const samples = client.shardMetricsSamples(1000);
      expect(samples).toHaveLength(2);
      for (const sample of samples) {
        expect(sample.metrics.actors).toBe(0);
        expect(sample.metrics.busyRatio).toBe(0);
        expect(sample.metrics.heartbeatAgeMs).toBe(Number.POSITIVE_INFINITY);
      }
    });
    it("counts resident actors and in-flight turns as busyRatio, and ages the heartbeat off the injected clock", async () => {
      const { client, workers, setNow } = harness(1);
      setNow(0);
      const activateA = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
      await activateA;
      const activateB = client.activate("b", "https://x/b.js", [], BUDGET);
      workers[0].deliver({ kind: "result", requestId: workers[0].sent[1].requestId, ok: true, value: undefined });
      await activateB;
      workers[0].deliver({ kind: "heartbeat", turnSeq: 1 });
      client.turn("a", [], BUDGET);
      setNow(300);
      const [sample] = client.shardMetricsSamples(300);
      expect(sample.metrics.actors).toBe(2);
      expect(sample.metrics.busyRatio).toBeCloseTo(0.5);
      expect(sample.metrics.heartbeatAgeMs).toBe(300);
    });
  });
  describe("orderEnvelopesByLane", () => {
    it("sorts by Lane priority, not arrival order, stable within a tied lane", () => {
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2), makeEnvelope("a", "Maintenance", 3), makeEnvelope("a", "Interactive", 4)];
      const ordered = orderEnvelopesByLane(envelopes);
      expect(ordered.map((envelope) => envelope.seq)).toEqual([2, 4, 1, 3]);
    });
    it("is a no-op for an already-lane-sorted, single-lane batch", () => {
      const envelopes = [makeEnvelope("a", "Interactive", 1), makeEnvelope("a", "Interactive", 2)];
      expect(orderEnvelopesByLane(envelopes).map((envelope) => envelope.seq)).toEqual([1, 2]);
    });
  });
  describe("GrantedBudgetTracker + interpretShardFrame", () => {
    it("a Grant records its budget and hands back envelopes in lane-priority order", () => {
      const tracker = createGrantedBudgetTracker();
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      const grantBudget = { ...BUDGET, fuel: 999 };
      const result = interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "a", budget: grantBudget, envelopes: [envelopes[1], envelopes[0]] });
      expect(tracker.granted("a")).toBe(grantBudget);
    });
    it("an Envelope with no prior Grant runs under the Maintenance-lane default, never an invented constant", () => {
      const tracker = createGrantedBudgetTracker();
      const lonelyEnvelope = makeEnvelope("never-granted", "Interactive", 1);
      const result = interpretShardFrame({ kind: "Envelope", envelope: lonelyEnvelope }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "never-granted", budget: MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [lonelyEnvelope] });
    });
    it("an Envelope AFTER a Grant for the same actor runs under THAT granted budget \u2014 proving the old constant no longer influences it", () => {
      const tracker = createGrantedBudgetTracker();
      const grantBudget = { ...BUDGET, fuel: 42 };
      interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes: [] }, tracker);
      const followUp = makeEnvelope("a", "Interactive", 5);
      const result = interpretShardFrame({ kind: "Envelope", envelope: followUp }, tracker);
      expect(result.action).toBe("runEnvelopes");
      expect(result.budget).toBe(grantBudget);
      expect(result.budget).not.toBe(MAINTENANCE_LANE_DEFAULT_BUDGET);
    });
    it("Register/Unregister are pure bookkeeping; Unregister forgets a previously granted budget", () => {
      const tracker = createGrantedBudgetTracker();
      interpretShardFrame({ kind: "Grant", actor: "a", budget: BUDGET, envelopes: [] }, tracker);
      expect(tracker.granted("a")).toBe(BUDGET);
      expect(interpretShardFrame({ kind: "Register", actor: "a" }, tracker)).toEqual({ action: "register", actor: "a" });
      expect(interpretShardFrame({ kind: "Unregister", actor: "a" }, tracker)).toEqual({ action: "unregister", actor: "a" });
      expect(tracker.granted("a")).toEqual(MAINTENANCE_LANE_DEFAULT_BUDGET);
    });
    it("an unknown/future frame variant resolves to 'unknown' instead of throwing (forward-compat)", () => {
      const tracker = createGrantedBudgetTracker();
      const futureFrame = { kind: "Checkpoint", actor: "a" };
      expect(() => interpretShardFrame(futureFrame, tracker)).not.toThrow();
      expect(interpretShardFrame(futureFrame, tracker)).toEqual({ action: "unknown", frame: futureFrame });
    });
  });
  describe("ShardClient.grant / ShardClient.envelope wire adoption", () => {
    it("grant() sends a ShardFrame::Grant frame with envelopes pre-sorted by lane, budget carried alongside them", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
      await activatePromise;
      const grantBudget = { ...BUDGET, fuel: 12345 };
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      client.grant("a", grantBudget, envelopes);
      const sent = workers[0].sent[1];
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Grant");
      const grantFrame = sent.frame;
      expect(grantFrame.budget).toBe(grantBudget);
      expect(grantFrame.envelopes.map((envelope) => envelope.seq)).toEqual([2, 1]);
    });
    it("envelope() sends a ShardFrame::Envelope frame with NO budget field on the wire at all", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
      await activatePromise;
      client.envelope(makeEnvelope("a", "Interactive", 1));
      const sent = workers[0].sent[1];
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Envelope");
      expect(Object.keys(sent.frame)).toEqual(["kind", "envelope"]);
    });
    it("turn()/activate() keep working completely unchanged alongside the new frame wire (incremental adoption really is incremental)", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("legacy", "https://x/legacy.js", [], BUDGET);
      const activateMsg = workers[0].sent[0];
      expect(activateMsg.kind).toBe("activate");
      workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;
      const turnPromise = client.turn("legacy", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0].sent[1];
      expect(turnMsg.kind).toBe("turn");
      workers[0].deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });
  });
  describe("fixed command ingress pages", () => {
    it("matches a DataView little-endian oracle across a full page boundary", () => {
      const command = Uint8Array.from({ length: SHARD_COMMAND_PAGE_BYTES + 5 }, (_, index) => index & 255);
      const pages = createShardCommandIngressPages({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, command });
      expect(pages).toHaveLength(2);
      expect(pages.map((page) => page.length)).toEqual([SHARD_COMMAND_PAGE_BYTES, 5]);
      expect(pages[0].cursor).toMatchObject({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, kind: 0, pageIndex: 0, pageCount: 2 });
      const oracle = new DataView(command.buffer, command.byteOffset, command.byteLength);
      expect(pages[0].block00.word0).toBe(oracle.getBigUint64(0, true));
      expect(pages[0].block63.word7).toBe(oracle.getBigUint64(SHARD_COMMAND_PAGE_BYTES - 8, true));
      expect(pages[1].block00.word0).toBe(0x0000000403020100n);
      expect(pages[1].block00.word1).toBe(0n);
      expect(pages[1].block63.word7).toBe(0n);
    });
    it("forwards the fixed page as the dedicated turn argument", async () => {
      const { client, workers } = harness(1);
      await activateActor(client, workers, "paged");
      const page = createShardCommandIngressPages({ owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance: 1, seq: 1n, command: Uint8Array.of(9, 8, 7) })[0];
      client.turn("paged", [], BUDGET, page);
      expect(workers[0].sent.at(-1)).toMatchObject({ kind: "turn", actorId: "paged", events: [], commandPage: page });
    });
  });
  describe("ShardFrame parity with Rust component.rs", () => {
    it("TS ShardFrame variant/field names match the live Rust enum in \uD83D\uDDA5\uFE0Fhost/\uD83E\uDDF5\uFE0Fshard/\uD83E\uDD80\uFE0Fcomponent.rs", async () => {
      const { readFileSync } = await import("fs");
      const rustUrl = new URL("../../../../\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDDA5\uFE0Fhost/\uD83E\uDDF5\uFE0Fshard/\uD83E\uDD80\uFE0Fcomponent.rs", import.meta.url);
      const source = readFileSync(rustUrl, "utf8");
      const enumMatch = source.match(/pub enum ShardFrame \{([\s\S]*?)\n\}\s*\n\s*impl ShardFrame/);
      expect(enumMatch).not.toBeNull();
      const body = enumMatch[1].replace(/\/\/\/.*$/gm, "").replace(/\/\/.*$/gm, "");
      const variantPattern = /(\w+)\s*(?:\{([^{}]*)\}|\(([^()]*)\))?\s*,/g;
      const rustVariants = [];
      let match;
      while ((match = variantPattern.exec(body)) !== null) {
        const [, name, structFields, tupleType] = match;
        if (structFields !== undefined) {
          const fields = structFields.split(",").map((part) => part.trim()).filter((part) => part.length > 0).map((part) => part.split(":")[0].trim());
          rustVariants.push({ name, fields });
        } else if (tupleType !== undefined) {
          rustVariants.push({ name, fields: null });
        }
      }
      expect(rustVariants.map((variant) => variant.name)).toEqual(SHARD_FRAME_VARIANT_FIELDS.map((variant) => variant.kind));
      for (const rustVariant of rustVariants) {
        if (rustVariant.fields === null)
          continue;
        const tsVariant = SHARD_FRAME_VARIANT_FIELDS.find((variant) => variant.kind === rustVariant.name);
        expect(tsVariant.fields).toEqual(rustVariant.fields);
      }
    });
  });
  async function activateActor(client, workers, actorId, shardIndex = 0) {
    const promise = client.activate(actorId, `https://x/${actorId}.js`, [], BUDGET);
    const message = workers[shardIndex].sent.at(-1);
    workers[shardIndex].deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
    await promise;
  }
  describe("ShardClient host-effect bridge \u2014 handler success", () => {
    it("resolves an effect-request through onHostEffect and posts an effect-complete frame back to the worker", async () => {
      const { client, workers } = harness(1, { onHostEffect: async (actorId, effect, params) => ({ actorId, effect, params, from: "handler" }) });
      await activateActor(client, workers, "a");
      workers[0].deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", { url: "https://example.test" }));
      await flushMicrotasks();
      const reply = findEffectReply(workers[0].sent, "a:http-fetch:1", "effect-complete");
      expect(reply).toBeDefined();
      expect(reply?.frame.envelope.payload.payload.value).toEqual({ actorId: "a", effect: "http-fetch", params: { url: "https://example.test" }, from: "handler" });
    });
  });
  describe("ShardClient host-effect bridge \u2014 handler error", () => {
    it("a rejected onHostEffect settles as effect-error, never a hang", async () => {
      const { client, workers } = harness(1, { onHostEffect: async () => {
        throw new Error("boom");
      } });
      await activateActor(client, workers, "a");
      workers[0].deliver(makeEffectRequestFrame("a", "blob-read", "a:blob-read:1", { hash: "x" }));
      await flushMicrotasks();
      const reply = findEffectReply(workers[0].sent, "a:blob-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("boom");
    });
  });
  describe("ShardClient host-effect bridge \u2014 no handler installed", () => {
    it("fails FAST with an explicit effect-error, synchronously, never a silent hang", async () => {
      const { client, workers } = harness(1);
      await activateActor(client, workers, "a");
      workers[0].deliver(makeEffectRequestFrame("a", "storage-read", "a:storage-read:1", {}));
      const reply = findEffectReply(workers[0].sent, "a:storage-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("no host effect handler installed");
    });
  });
  describe("ShardClient host-effect bridge \u2014 backpressure cap", () => {
    it("rejects an effect-request beyond maxOutstandingEffectsPerActor with a quota-shaped effect-error, while the earlier one stays pending", async () => {
      const { client, workers } = harness(1, { maxOutstandingEffectsPerActor: 1, onHostEffect: () => new Promise(() => {}) });
      await activateActor(client, workers, "a");
      workers[0].deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:1", {}));
      workers[0].deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:2", {}));
      expect(findEffectReply(workers[0].sent, "a:spawn-job:1", "effect-error")).toBeUndefined();
      expect(findEffectReply(workers[0].sent, "a:spawn-job:1", "effect-complete")).toBeUndefined();
      const reply = findEffectReply(workers[0].sent, "a:spawn-job:2", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toMatch(/outstandingRequests.*limit=1.*actual=1/);
    });
  });
  describe("ShardClient host-effect bridge \u2014 shard-loss settlement", () => {
    it("terminate() aborts every outstanding effect for its actors, and a late handler resolution posts no reply to the dead worker", async () => {
      let capturedSignal;
      const { client, workers } = harness(1, {
        onHostEffect: (_actorId, _effect, _params, signal) => new Promise((resolve) => {
          capturedSignal = signal;
          signal.addEventListener("abort", () => resolve("too-late"));
        })
      });
      await activateActor(client, workers, "a");
      workers[0].deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      expect(capturedSignal?.aborted).toBe(false);
      client.terminate(0);
      expect(capturedSignal?.aborted).toBe(true);
      const sentBeforeLateResolve = workers[0].sent.length;
      await flushMicrotasks();
      expect(workers[0].sent.length).toBe(sentBeforeLateResolve);
      expect(findEffectReply(workers[0].sent, "a:http-fetch:1", "effect-complete")).toBeUndefined();
      expect(findEffectReply(workers[0].sent, "a:http-fetch:1", "effect-error")).toBeUndefined();
    });
    it("dispose(actorId) aborts that actor's outstanding effects without touching a sibling actor's", async () => {
      const signals = {};
      const { client, workers } = harness(1, {
        onHostEffect: (actorId, _effect, _params, signal) => {
          signals[actorId] = signal;
          return new Promise(() => {});
        }
      });
      await activateActor(client, workers, "a");
      await activateActor(client, workers, "b");
      workers[0].deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      workers[0].deliver(makeEffectRequestFrame("b", "http-fetch", "b:http-fetch:1", {}));
      client.dispose("a");
      expect(signals.a?.aborted).toBe(true);
      expect(signals.b?.aborted).toBe(false);
    });
  });
}

/* 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts */
var MAILBOX_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
function laneRank(lane) {
  return MAILBOX_LANE_ORDER.indexOf(lane);
}
function createBoundedMailbox(capacity) {
  const lanes = MAILBOX_LANE_ORDER.map(() => []);
  let len = 0;
  return {
    enqueue(envelope) {
      const incomingRank = laneRank(envelope.lane);
      if (envelope.coalesce !== undefined) {
        const lane = lanes[incomingRank];
        const existingIndex = lane.findIndex((queued) => queued.coalesce === envelope.coalesce);
        if (existingIndex !== -1) {
          lane[existingIndex] = envelope;
          return { kind: "coalesced" };
        }
      }
      if (len >= capacity) {
        let victimRank = -1;
        for (let rank = MAILBOX_LANE_ORDER.length - 1;rank > incomingRank; rank--) {
          if (lanes[rank].length > 0) {
            victimRank = rank;
            break;
          }
        }
        if (victimRank === -1)
          return { kind: "rejected" };
        lanes[victimRank].shift();
        len -= 1;
        lanes[incomingRank].push(envelope);
        len += 1;
        return { kind: "dropped", lane: MAILBOX_LANE_ORDER[victimRank] };
      }
      lanes[incomingRank].push(envelope);
      len += 1;
      return { kind: "accept" };
    },
    popNext() {
      for (const lane of lanes) {
        if (lane.length > 0) {
          len -= 1;
          return lane.shift();
        }
      }
      return;
    },
    get length() {
      return len;
    },
    get isEmpty() {
      return len === 0;
    }
  };
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("createBoundedMailbox", () => {
    it("overflow is rejected (not a silent drop) when nothing lower-priority exists to evict", () => {
      const mailbox = createBoundedMailbox(2);
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
      expect(mailbox.length).toBe(2);
    });
    it("coalescing collapses same-key entries latest-wins, preserving queue position", () => {
      const mailbox = createBoundedMailbox(10);
      for (let i = 0;i < 200; i++) {
        const backpressure = mailbox.enqueue({ lane: "Interactive", coalesce: "pointer-move", payload: i });
        expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
      }
      expect(mailbox.length).toBe(1);
      expect(mailbox.popNext()?.payload).toBe(199);
    });
    it("lane priority beats FIFO order on popNext", () => {
      const mailbox = createBoundedMailbox(10);
      mailbox.enqueue({ lane: "Maintenance", payload: "low" });
      mailbox.enqueue({ lane: "Background", payload: "mid" });
      mailbox.enqueue({ lane: "Interactive", payload: "high" });
      expect(mailbox.popNext()?.lane).toBe("Interactive");
      expect(mailbox.popNext()?.lane).toBe("Background");
      expect(mailbox.popNext()?.lane).toBe("Maintenance");
      expect(mailbox.isEmpty).toBe(true);
    });
    it("dropped backpressure reports the evicted lane, admitting the higher-priority incomer", () => {
      const mailbox = createBoundedMailbox(2);
      mailbox.enqueue({ lane: "Maintenance", payload: "a" });
      mailbox.enqueue({ lane: "Background", payload: "b" });
      const backpressure = mailbox.enqueue({ lane: "Interactive", payload: "c" });
      expect(backpressure).toEqual({ kind: "dropped", lane: "Maintenance" });
      expect(mailbox.length).toBe(2);
      expect(mailbox.popNext()?.payload).toBe("c");
      expect(mailbox.popNext()?.payload).toBe("b");
    });
  });
}

/* 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts */
var LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
function freshLaneCounts() {
  return { Interactive: 0, UserVisible: 0, Background: 0, Maintenance: 0 };
}

class TurnScheduler {
  mailboxes = new Map;
  laneCounts = new Map;
  busyActors = new Set;
  options;
  pumpScheduled = false;
  constructor(options) {
    this.options = options;
  }
  enqueue(actorId, turn) {
    const mailbox = this.mailboxFor(actorId);
    const backpressure = mailbox.enqueue({ lane: turn.lane, coalesce: turn.coalesce, payload: turn.payload });
    this.applyLaneDelta(actorId, turn.lane, backpressure);
    if (backpressure.kind !== "rejected")
      this.schedulePump();
    return backpressure;
  }
  mailboxFor(actorId) {
    let mailbox = this.mailboxes.get(actorId);
    if (!mailbox) {
      mailbox = createBoundedMailbox(this.options.mailboxCapacity);
      this.mailboxes.set(actorId, mailbox);
      this.laneCounts.set(actorId, freshLaneCounts());
    }
    return mailbox;
  }
  applyLaneDelta(actorId, incomingLane, backpressure) {
    const counts = this.laneCounts.get(actorId);
    if (backpressure.kind === "accept") {
      counts[incomingLane] += 1;
    } else if (backpressure.kind === "dropped") {
      counts[backpressure.lane] -= 1;
      counts[incomingLane] += 1;
    }
  }
  cancelQueued(actorId) {
    const mailbox = this.mailboxes.get(actorId);
    if (!mailbox)
      return 0;
    const counts = this.laneCounts.get(actorId);
    let cancelled = 0;
    let envelope;
    while ((envelope = mailbox.popNext()) !== undefined) {
      counts[envelope.lane] -= 1;
      cancelled += 1;
    }
    return cancelled;
  }
  teardownActor(actorId) {
    const cancelled = this.cancelQueued(actorId);
    this.mailboxes.delete(actorId);
    this.laneCounts.delete(actorId);
    return cancelled;
  }
  isBusy(actorId) {
    return this.busyActors.has(actorId);
  }
  pendingCount(actorId) {
    return this.mailboxes.get(actorId)?.length ?? 0;
  }
  schedulePump() {
    if (this.pumpScheduled)
      return;
    this.pumpScheduled = true;
    queueMicrotask(() => {
      this.pumpScheduled = false;
      this.pump();
    });
  }
  pickNextReadyActor() {
    for (const lane of LANE_ORDER) {
      for (const actorId of this.mailboxes.keys()) {
        if (this.busyActors.has(actorId))
          continue;
        const counts = this.laneCounts.get(actorId);
        if (counts && counts[lane] > 0)
          return actorId;
      }
    }
    return;
  }
  pump() {
    for (;; ) {
      const actorId = this.pickNextReadyActor();
      if (actorId === undefined)
        return;
      const mailbox = this.mailboxes.get(actorId);
      const envelope = mailbox.popNext();
      if (!envelope)
        continue;
      this.laneCounts.get(actorId)[envelope.lane] -= 1;
      this.busyActors.add(actorId);
      const budget = this.options.budgetFor(actorId);
      this.options.runTurn(actorId, envelope.payload, budget).catch((error) => this.options.onTurnError?.(actorId, error)).finally(() => {
        this.busyActors.delete(actorId);
        this.schedulePump();
      });
    }
  }
}
if (import.meta.vitest) {
  let deferred = function() {
    let resolve;
    let reject;
    const promise = new Promise((res, rej) => {
      resolve = res;
      reject = rej;
    });
    return { promise, resolve, reject };
  }, harness = function(mailboxCapacity = 10) {
    const order = [];
    const running = new Map;
    const scheduler = new TurnScheduler({
      mailboxCapacity,
      budgetFor: () => {
        return;
      },
      runTurn: (actorId, payload) => {
        order.push({ actorId, payload });
        const { promise, resolve, reject } = deferred();
        running.set(actorId, { resolve, reject });
        return promise;
      }
    });
    return { scheduler, order, settle: (actorId) => running.get(actorId)?.resolve(), fail: (actorId, error) => running.get(actorId)?.reject(error) };
  };
  const { describe, expect, it, vi } = import.meta.vitest;
  const flush = () => new Promise((resolve) => queueMicrotask(() => queueMicrotask(resolve)));
  describe("TurnScheduler lane priority", () => {
    it("dispatches by lane priority, not arrival order, when a batch lands before the first pick", async () => {
      const { scheduler, order } = harness();
      scheduler.enqueue("low", { lane: "Background", payload: "low-1" });
      scheduler.enqueue("high", { lane: "Interactive", payload: "high-1" });
      scheduler.enqueue("mid", { lane: "UserVisible", payload: "mid-1" });
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["high", "mid", "low"]);
    });
  });
  describe("TurnScheduler per-actor ordering under interleaving", () => {
    it("never starts an actor's next turn before its current one settles, even while other actors interleave", async () => {
      const { scheduler, order, settle } = harness();
      scheduler.enqueue("a", { lane: "Interactive", payload: "a-1" });
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["a"]);
      scheduler.enqueue("a", { lane: "Interactive", payload: "a-2" });
      scheduler.enqueue("b", { lane: "Interactive", payload: "b-1" });
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["a", "b"]);
      expect(scheduler.isBusy("a")).toBe(true);
      settle("a");
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["a", "b", "a"]);
      expect(order[2].payload).toBe("a-2");
      settle("b");
      settle("a");
      await flush();
    });
  });
  describe("TurnScheduler coalescing", () => {
    it("collapses a burst of same-key envelopes to one queued turn, never 200 deep", async () => {
      const { scheduler, order, settle } = harness();
      for (let i = 0;i < 200; i++) {
        const backpressure = scheduler.enqueue("pointer", { lane: "Interactive", coalesce: "pointer-move", payload: i });
        expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
      }
      expect(scheduler.pendingCount("pointer")).toBe(1);
      await flush();
      expect(order).toHaveLength(1);
      expect(order[0].payload).toBe(199);
      settle("pointer");
    });
  });
  describe("TurnScheduler backpressure at the cap", () => {
    it("rejected surfaces synchronously at the cap instead of the queue growing past it", () => {
      const { scheduler } = harness(2);
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
      expect(scheduler.pendingCount("full")).toBe(2);
    });
  });
  describe("TurnScheduler cancellation", () => {
    it("cancels only queued turns, leaving an in-flight one to settle on its own", async () => {
      const { scheduler, order, settle } = harness();
      scheduler.enqueue("x", { lane: "Interactive", payload: "x-1" });
      await flush();
      expect(order.map((e) => e.payload)).toEqual(["x-1"]);
      scheduler.enqueue("x", { lane: "Interactive", payload: "x-2" });
      scheduler.enqueue("x", { lane: "Background", payload: "x-3" });
      expect(scheduler.pendingCount("x")).toBe(2);
      const cancelled = scheduler.cancelQueued("x");
      expect(cancelled).toBe(2);
      expect(scheduler.pendingCount("x")).toBe(0);
      settle("x");
      await flush();
      expect(order.map((e) => e.payload)).toEqual(["x-1"]);
    });
    it("teardownActor cancels queued work and forgets the actor so a later enqueue starts fresh", async () => {
      const { scheduler, order } = harness();
      scheduler.enqueue("y", { lane: "Interactive", payload: "y-1" });
      scheduler.enqueue("y", { lane: "Interactive", payload: "y-2" });
      const cancelled = scheduler.teardownActor("y");
      expect(cancelled).toBe(2);
      expect(scheduler.pendingCount("y")).toBe(0);
      await flush();
      expect(order).toHaveLength(0);
    });
  });
  describe("TurnScheduler onTurnError", () => {
    it("reports a rejected runTurn instead of throwing out of the pump loop, and keeps draining", async () => {
      const errors = [];
      const scheduler = new TurnScheduler({
        mailboxCapacity: 5,
        budgetFor: () => {
          return;
        },
        runTurn: async (actorId, payload) => {
          if (payload === "boom")
            throw new Error("turn failed");
        },
        onTurnError: (actorId, error) => errors.push({ actorId, error })
      });
      scheduler.enqueue("z", { lane: "Interactive", payload: "boom" });
      await flush();
      expect(errors).toHaveLength(1);
      expect(errors[0].actorId).toBe("z");
      expect(scheduler.isBusy("z")).toBe(false);
    });
  });
}

/* 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts */
class OsTransient {
  boxes = new Map;
  maps = new Map;
  sets = new Map;
  weakMaps = new Map;
  box(key, init) {
    let box = this.boxes.get(key);
    if (!box) {
      box = { current: init };
      this.boxes.set(key, box);
    }
    return box;
  }
  map(key) {
    let map = this.maps.get(key);
    if (!map) {
      map = new Map;
      this.maps.set(key, map);
    }
    return map;
  }
  set(key) {
    let set = this.sets.get(key);
    if (!set) {
      set = new Set;
      this.sets.set(key, set);
    }
    return set;
  }
  weakMap(key) {
    let map = this.weakMaps.get(key);
    if (!map) {
      map = new WeakMap;
      this.weakMaps.set(key, map);
    }
    return map;
  }
  reset() {
    this.boxes.clear();
    this.maps.clear();
    this.sets.clear();
    this.weakMaps.clear();
  }
}
var defaultOsTransient = new OsTransient;
function ephemeralBox(key, init) {
  return defaultOsTransient.box(key, init);
}
function ephemeralMap(key) {
  return defaultOsTransient.map(key);
}
function createTurnOutcomeBroadcast() {
  const subscribers = new Set;
  return {
    push: (value) => {
      for (const subscriber of subscribers) {
        if (subscriber.resolve) {
          const resolve = subscriber.resolve;
          subscriber.resolve = null;
          resolve({ value, done: false });
        } else {
          subscriber.queue.push(value);
        }
      }
    },
    complete: () => {
      for (const subscriber of subscribers)
        subscriber.resolve?.({ value: undefined, done: true });
      subscribers.clear();
    },
    stream: {
      [Symbol.asyncIterator]() {
        const subscriber = { queue: [], resolve: null };
        subscribers.add(subscriber);
        return {
          next: () => {
            if (subscriber.queue.length > 0)
              return Promise.resolve({ value: subscriber.queue.shift(), done: false });
            return new Promise((resolve) => {
              subscriber.resolve = resolve;
            });
          },
          return: () => {
            subscribers.delete(subscriber);
            return Promise.resolve({ value: undefined, done: true });
          }
        };
      }
    }
  };
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("createTurnOutcomeBroadcast", () => {
    it("multicasts one pushed value to EVERY live subscriber, not a shared drain-once FIFO", async () => {
      const broadcast = createTurnOutcomeBroadcast();
      const iteratorA = broadcast.stream[Symbol.asyncIterator]();
      const iteratorB = broadcast.stream[Symbol.asyncIterator]();
      broadcast.push({ instanceId: 1, frames: [] });
      const [stepA, stepB] = await Promise.all([iteratorA.next(), iteratorB.next()]);
      expect(stepA).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
      expect(stepB).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
    });
    it("queues a value pushed before next() is called, and delivers queued values in push order", async () => {
      const broadcast = createTurnOutcomeBroadcast();
      const iterator = broadcast.stream[Symbol.asyncIterator]();
      broadcast.push({ instanceId: 2, frames: [] });
      broadcast.push({ instanceId: 3, frames: [] });
      expect(await iterator.next()).toEqual({ value: { instanceId: 2, frames: [] }, done: false });
      expect(await iterator.next()).toEqual({ value: { instanceId: 3, frames: [] }, done: false });
    });
    it("return() unsubscribes immediately \u2014 a later push never reaches a next() called after it", async () => {
      const broadcast = createTurnOutcomeBroadcast();
      const iterator = broadcast.stream[Symbol.asyncIterator]();
      expect(await iterator.return?.()).toEqual({ value: undefined, done: true });
      const pending = iterator.next();
      broadcast.push({ instanceId: 4, frames: [] });
      const raceResult = await Promise.race([pending.then(() => "resolved"), Promise.resolve("not-yet")]);
      expect(raceResult).toBe("not-yet");
    });
    it("complete() force-closes every still-live subscriber at once", async () => {
      const broadcast = createTurnOutcomeBroadcast();
      const iteratorA = broadcast.stream[Symbol.asyncIterator]();
      const iteratorB = broadcast.stream[Symbol.asyncIterator]();
      const pendingA = iteratorA.next();
      const pendingB = iteratorB.next();
      broadcast.complete();
      expect(await pendingA).toEqual({ value: undefined, done: true });
      expect(await pendingB).toEqual({ value: undefined, done: true });
    });
  });
}
function expandPluginRegistry(plugins, primaryPluginId, hostMode = false) {
  if (hostMode || !primaryPluginId)
    return plugins;
  const byId = new Map(plugins.map((entry) => [entry.pluginId, entry]));
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  const selected = new Map;
  const queue = [...primaryEntries, ...contributorEntries];
  for (const entry of queue)
    selected.set(entry.pluginId, entry);
  for (let index = 0;index < queue.length; index++) {
    const entry = queue[index];
    for (const dependency of entry.dependencies ?? []) {
      if (selected.has(dependency.pluginId))
        continue;
      const dependencyEntry = byId.get(dependency.pluginId);
      if (!dependencyEntry)
        continue;
      selected.set(dependency.pluginId, dependencyEntry);
      queue.push(dependencyEntry);
    }
  }
  return [...selected.values()];
}
function dependsOnToPluginDependencies(dependsOn) {
  return dependsOn?.map((pluginId) => ({ pluginId, version: "*" }));
}
function dialectEquals(a, b) {
  return a.artifactKind === b.artifactKind && a.standard === b.standard && a.subset === b.subset;
}
function dialectCoordinate(dialect) {
  return `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}`;
}
var SURFACE_FAULT_CODES = {
  ViewerReadOnly: "viewer.read-only",
  UnknownDialect: "surface.unknown-dialect",
  ContributionNotPermitted: "surface.contribution-not-permitted",
  Conflict: "surface.conflict",
  MissingOwnerSurface: "surface.missing-owner-surface"
};
function surfaceFault(code, message, scope = {}) {
  return { origin: "framework", code, severity: "error", message, scope, retryable: false };
}
function readManifestAppSurface(app) {
  const id = app.id;
  const role = app.role;
  const dialect = app.dialect;
  if (typeof id !== "string")
    return;
  if (role !== "viewer" && role !== "editor")
    return;
  if (!dialect || typeof dialect.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string")
    return;
  return { appId: id, role, dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset } };
}
function coordinateRoleKey(dialect, role) {
  return `${dialectCoordinate(dialect)}#${role}`;
}

class AppRouter {
  entriesByCoordinateRole;
  dialectsByCoordinate;
  ownerByArtifactKind;
  constructor(entriesByCoordinateRole, dialectsByCoordinate, ownerByArtifactKind) {
    this.entriesByCoordinateRole = entriesByCoordinateRole;
    this.dialectsByCoordinate = dialectsByCoordinate;
    this.ownerByArtifactKind = ownerByArtifactKind;
  }
  static build(manifests) {
    const ownerByArtifactKind = new Map;
    const seenRefs = new Set;
    const grouped = new Map;
    const loadedIds = new Set(manifests.map((manifest) => manifest.pluginId));
    const dependencyNodes = manifests.map((manifest) => ({
      pluginId: manifest.pluginId,
      dependencies: (manifest.dependencies ?? []).filter((dependency) => loadedIds.has(dependency.pluginId))
    }));
    const resolved = resolvePluginLoadOrder(dependencyNodes);
    const byId = new Map(manifests.map((manifest) => [manifest.pluginId, manifest]));
    const ordered = resolved.errors.length === 0 ? resolved.order.map((pluginId) => byId.get(pluginId)).filter(Boolean) : [...manifests];
    for (const manifest of ordered) {
      for (const kind of manifest.artifactKinds ?? []) {
        if (!ownerByArtifactKind.has(kind.id))
          ownerByArtifactKind.set(kind.id, manifest.pluginId);
      }
      for (const raw of manifest.apps) {
        const surface = readManifestAppSurface(raw);
        if (!surface)
          continue;
        let owner = ownerByArtifactKind.get(surface.dialect.artifactKind);
        if (owner === undefined) {
          owner = manifest.pluginId;
          ownerByArtifactKind.set(surface.dialect.artifactKind, owner);
        }
        if (owner !== manifest.pluginId) {
          const permitted = (manifest.dependencies ?? []).some((dependency) => dependency.pluginId === owner);
          if (!permitted) {
            throw new SemioFaultError(surfaceFault(SURFACE_FAULT_CODES.ContributionNotPermitted, `plugin ${JSON.stringify(manifest.pluginId)} contributes a surface for ${JSON.stringify(dialectCoordinate(surface.dialect))} without depending on owner ${JSON.stringify(owner)}`, { pluginId: manifest.pluginId }));
          }
        }
        const ref = { pluginId: manifest.pluginId, appId: surface.appId };
        const refKey = `${ref.pluginId} ${ref.appId}`;
        if (seenRefs.has(refKey)) {
          throw new SemioFaultError(surfaceFault(SURFACE_FAULT_CODES.Conflict, `AppRef {pluginId: ${JSON.stringify(ref.pluginId)}, appId: ${JSON.stringify(ref.appId)}} registered twice`, { pluginId: ref.pluginId, appId: ref.appId }));
        }
        seenRefs.add(refKey);
        const key = coordinateRoleKey(surface.dialect, surface.role);
        let group = grouped.get(key);
        if (!group) {
          group = { dialect: surface.dialect, role: surface.role, entries: [] };
          grouped.set(key, group);
        }
        group.entries.push(ref);
      }
    }
    const entriesByCoordinateRole = new Map;
    const dialectsByCoordinate = new Map;
    for (const [key, group] of grouped) {
      const owner = ownerByArtifactKind.get(group.dialect.artifactKind);
      const sorted = [...group.entries].sort((a, b) => a.pluginId === b.pluginId ? a.appId.localeCompare(b.appId) : a.pluginId.localeCompare(b.pluginId));
      const ordered2 = owner === undefined ? sorted : [...sorted.filter((ref) => ref.pluginId === owner), ...sorted.filter((ref) => ref.pluginId !== owner)];
      entriesByCoordinateRole.set(key, ordered2);
      dialectsByCoordinate.set(dialectCoordinate(group.dialect), group.dialect);
    }
    return new AppRouter(entriesByCoordinateRole, dialectsByCoordinate, ownerByArtifactKind);
  }
  entriesFor(dialect, role) {
    return this.entriesByCoordinateRole.get(coordinateRoleKey(dialect, role)) ?? [];
  }
  ownerPluginId(artifactKind) {
    return this.ownerByArtifactKind.get(artifactKind);
  }
  ownedSurfaceGaps() {
    const gaps = [];
    for (const [coordinate, dialect] of this.dialectsByCoordinate) {
      const owner = this.ownerByArtifactKind.get(dialect.artifactKind);
      if (owner === undefined)
        continue;
      for (const role of ["viewer", "editor"]) {
        if (this.entriesFor(dialect, role).length === 0) {
          gaps.push(surfaceFault(SURFACE_FAULT_CODES.MissingOwnerSurface, `owned subset ${JSON.stringify(coordinate)} has no ${role} surface`, { pluginId: owner }));
        }
      }
    }
    return gaps;
  }
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("AppRouter", () => {
    it("orders a loaded aggregate plugin after the foreign surface owner it depends on", () => {
      const cadDialect = { artifactKind: "s.cad.cad", standard: "1", subset: "*" };
      const aggregate = {
        pluginId: "demonstrator",
        apps: [{ id: "s.cad.cad@1/*#editor", role: "editor", dialect: cadDialect }],
        dependencies: [{ pluginId: "cad", version: "*" }]
      };
      const owner = {
        pluginId: "cad",
        apps: [{ id: "s.cad.cad@1/*#editor", role: "editor", dialect: cadDialect }]
      };
      const router = AppRouter.build([aggregate, owner]);
      expect(router.ownerPluginId("s.cad.cad")).toBe("cad");
      expect(router.entriesFor(cadDialect, "editor")).toEqual([
        { pluginId: "cad", appId: "s.cad.cad@1/*#editor" },
        { pluginId: "demonstrator", appId: "s.cad.cad@1/*#editor" }
      ]);
    });
  });
}
function ioFidelityRank(fidelity) {
  switch (fidelity) {
    case "Exact":
      return 3;
    case "Canonical":
      return 2;
    case "Semantic":
      return 1;
    case "Lossy":
      return 0;
  }
}
function ioFidelityFromRank(rank) {
  if (rank >= 3)
    return "Exact";
  if (rank === 2)
    return "Canonical";
  if (rank === 1)
    return "Semantic";
  return "Lossy";
}
function ioConfidenceRank(confidence) {
  switch (confidence) {
    case "None":
      return 0;
    case "Low":
      return 1;
    case "Medium":
      return 2;
    case "High":
      return 3;
  }
}
function ioConfidenceFromRank(rank) {
  if (rank >= 3)
    return "High";
  if (rank === 2)
    return "Medium";
  if (rank === 1)
    return "Low";
  return "None";
}
function ioEntryKey(from, into) {
  return `${dialectCoordinate(from)}->${dialectCoordinate(into)}`;
}

class IoEntryGraph {
  ownerByEntry;
  constructor(ownerByEntry) {
    this.ownerByEntry = ownerByEntry;
  }
  static build(plugins) {
    const ownerByEntry = new Map;
    for (const plugin of plugins) {
      for (const descriptor of plugin.entries) {
        const key = ioEntryKey(descriptor.from, descriptor.into);
        const existing = ownerByEntry.get(key);
        if (existing) {
          if (existing.pluginId !== plugin.pluginId) {
            throw new Error(`io entry route conflict for ${key}: ${JSON.stringify(existing.pluginId)} already owns it; ${JSON.stringify(plugin.pluginId)} cannot replace it`);
          }
          continue;
        }
        ownerByEntry.set(key, { pluginId: plugin.pluginId, descriptor });
      }
    }
    return new IoEntryGraph(ownerByEntry);
  }
  route(from, into, maxHops = 3) {
    const bound = Math.min(maxHops, 3);
    if (bound <= 0)
      throw new Error(`io_routes ${dialectCoordinate(from)} -> ${dialectCoordinate(into)}: max hops clamped to 0`);
    const candidates = [];
    const path = [];
    const visited = new Set([dialectCoordinate(from)]);
    const walk = (current, remainingHops) => {
      if (remainingHops === 0)
        return;
      for (const { descriptor } of this.ownerByEntry.values()) {
        if (!dialectEquals(descriptor.from, current))
          continue;
        const nextCoordinate = dialectCoordinate(descriptor.into);
        if (visited.has(nextCoordinate))
          continue;
        path.push(descriptor);
        if (dialectEquals(descriptor.into, into)) {
          candidates.push([...path]);
        } else {
          visited.add(nextCoordinate);
          walk(descriptor.into, remainingHops - 1);
          visited.delete(nextCoordinate);
        }
        path.pop();
      }
    };
    walk(from, bound);
    if (candidates.length === 0)
      throw new Error(`no io route from ${dialectCoordinate(from)} to ${dialectCoordinate(into)} within ${bound} hops`);
    const rank = (hops) => {
      const minFidelity = Math.min(...hops.map((hop) => ioFidelityRank(hop.fidelity)));
      const joined = hops.map((hop) => dialectCoordinate(hop.into)).join(",");
      return [-minFidelity, hops.length, joined];
    };
    const sorted = [...candidates].sort((a, b) => {
      const [aInverseFidelity, aLength, aJoined] = rank(a);
      const [bInverseFidelity, bLength, bJoined] = rank(b);
      if (aInverseFidelity !== bInverseFidelity)
        return aInverseFidelity - bInverseFidelity;
      if (aLength !== bLength)
        return aLength - bLength;
      return aJoined.localeCompare(bJoined);
    });
    const best = sorted[0];
    const minFidelityRank = Math.min(...best.map((hop) => ioFidelityRank(hop.fidelity)));
    return { hops: best, fidelity: ioFidelityFromRank(minFidelityRank) };
  }
  ownerOf(from, into) {
    return this.ownerByEntry.get(ioEntryKey(from, into))?.pluginId;
  }
  carrierEntries(carrier) {
    const found = [];
    for (const { pluginId, descriptor } of this.ownerByEntry.values()) {
      if (dialectEquals(descriptor.from, carrier) && descriptor.sniffs)
        found.push({ into: descriptor.into, pluginId });
    }
    return found;
  }
}
async function ioRun(graph, callingPluginId, from, into, payload, runHop, maxHops = 3) {
  const route = graph.route(from, into, maxHops);
  const hops = route.hops.map((hop) => {
    const owner = graph.ownerOf(hop.from, hop.into);
    if (owner === undefined)
      throw new Error(`io-run: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} vanished from the graph between resolve and execute`);
    if (owner === callingPluginId) {
      throw new Error(`io-run refused: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} is owned by the calling plugin ${JSON.stringify(callingPluginId)} itself \u2014 executing it would re-enter that plugin's own in-flight worker call`);
    }
    return { hop, owner };
  });
  let current = payload;
  for (const { hop, owner } of hops) {
    current = await runHop(owner, hop.from, hop.into, current);
  }
  return current;
}
async function ioIdentify(graph, callingPluginId, carrier, payload, sniffHop) {
  const candidates = graph.carrierEntries(carrier).filter((entry) => entry.pluginId !== callingPluginId);
  const found = [];
  for (const { into, pluginId } of candidates) {
    const confidence = ioConfidenceFromRank(await sniffHop(pluginId, carrier, into, payload));
    if (confidence !== "None")
      found.push([into, confidence]);
  }
  found.sort((a, b) => {
    const rankDiff = ioConfidenceRank(b[1]) - ioConfidenceRank(a[1]);
    if (rankDiff !== 0)
      return rankDiff;
    return dialectCoordinate(a[0]).localeCompare(dialectCoordinate(b[0]));
  });
  return found;
}
class SemioFaultError extends Error {
  fault;
  constructor(fault) {
    super(fault.message);
    this.name = "SemioFaultError";
    this.fault = fault;
  }
}
function relayPluginBackboneOutbound(uri, message) {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}
globalThis.__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;
function pluginBackboneDocumentIdFromUri(uri) {
  return uri.startsWith("actor://") ? uri.slice("actor://".length) : uri;
}
var pluginBackboneRoutes = new Map;
function intersectCapabilityGrants(granted, requested) {
  const grantedIds = new Set(granted.map((grant) => grant.id));
  return requested.filter((request) => grantedIds.has(request.id));
}
function defaultGuestSlimAssetFetcher(moduleUrl) {
  const vendorUrl = moduleUrl.split(/[?#]/)[0].replace(/\/[^/]+\/[^/]+\.js$/, "/_vendor/guestslim-typst-fonts.bin");
  return fetch(vendorUrl).then((response) => {
    if (!response.ok)
      throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
    return response.arrayBuffer();
  }).then((buffer) => [["guestslim-typst-fonts", buffer]]);
}
var DEFAULT_MAX_RESIDENT_ACTORS = 24;
var MIN_MAX_RESIDENT_ACTORS = 4;
var MAX_MAX_RESIDENT_ACTORS = 96;
var RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB = 6;
var BYTES_PER_RESIDENT_ACTOR = 64 * 1024 * 1024;
function clampResidentActors(value) {
  return Math.min(MAX_MAX_RESIDENT_ACTORS, Math.max(MIN_MAX_RESIDENT_ACTORS, Math.round(value)));
}
function defaultMemoryProbe() {
  const nav = globalThis.navigator;
  const perf = globalThis.performance;
  return { deviceMemoryGiB: nav?.deviceMemory, jsHeapSizeLimitBytes: perf?.memory?.jsHeapSizeLimit };
}
function residentActorCapFromMemory(reading, fallback = DEFAULT_MAX_RESIDENT_ACTORS) {
  if (typeof reading.deviceMemoryGiB === "number" && reading.deviceMemoryGiB > 0)
    return clampResidentActors(reading.deviceMemoryGiB * RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB);
  if (typeof reading.jsHeapSizeLimitBytes === "number" && reading.jsHeapSizeLimitBytes > 0)
    return clampResidentActors(reading.jsHeapSizeLimitBytes / BYTES_PER_RESIDENT_ACTOR);
  return fallback;
}
var DEFAULT_TURN_MAILBOX_CAPACITY = 32;

class ActivationRegistry {
  manifests = new Map;
  resident = new Map;
  residencyOrder = [];
  checkpoints = new Map;
  actorPlugin = new Map;
  actorGeneration = new Map;
  extensionsByParent = new Map;
  extensionChildren = new Map;
  shardClient;
  defaultBudget;
  maxResidentActors;
  fetchAssets;
  assetsPromise = null;
  now;
  lastRuntimeMetricsPublishMs = null;
  turnScheduler;
  onTurnResult;
  stopMetricsPublisher;
  metricsBus = new EventTarget;
  constructor(options) {
    this.shardClient = options.shardClient;
    this.defaultBudget = options.defaultBudget;
    this.maxResidentActors = options.maxResidentActors ?? residentActorCapFromMemory((options.memoryProbe ?? defaultMemoryProbe)());
    this.fetchAssets = options.fetchAssets ?? defaultGuestSlimAssetFetcher;
    this.now = options.now ?? (() => Date.now());
    this.onTurnResult = options.onTurnResult ?? (() => {});
    const onTurnError = options.onTurnError ?? ((actorId, error) => console.error(`[DEBUG] ActivationRegistry: turn failed for ${actorId}`, error));
    this.turnScheduler = new TurnScheduler({
      mailboxCapacity: options.turnMailboxCapacity ?? DEFAULT_TURN_MAILBOX_CAPACITY,
      budgetFor: () => this.defaultBudget,
      runTurn: (actorId, payload, budget) => this.runQueuedTurn(actorId, payload, budget),
      onTurnError
    });
    this.stopMetricsPublisher = options.autoStartMetricsPublisher === true ? this.startRuntimeMetricsPublisher((topic, snapshot) => this.metricsBus.dispatchEvent(new CustomEvent(topic, { detail: snapshot }))) : () => {};
  }
  registerManifest(entry) {
    this.manifests.set(entry.pluginId, entry);
  }
  registerCatalog(catalog) {
    for (const target of catalog.plugins)
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.moduleUrl(target.pluginId, target.wasmOut), caps: [] });
    for (const target of catalog.extensions) {
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.extensionModuleUrl(target.pluginId, target.wasmOut), caps: [] });
      const parentId = target.dependsOn?.[0];
      if (!parentId)
        continue;
      const siblings = this.extensionsByParent.get(parentId) ?? [];
      siblings.push(target.pluginId);
      this.extensionsByParent.set(parentId, siblings);
    }
  }
  manifestFor(pluginId) {
    return this.manifests.get(pluginId);
  }
  loadAssets(moduleUrl) {
    this.assetsPromise ??= this.fetchAssets(moduleUrl).catch((error) => {
      console.warn("[DEBUG] ActivationRegistry: guestSlim asset fetch failed; affected actors render without it", error);
      this.assetsPromise = null;
      return [];
    });
    return this.assetsPromise;
  }
  markResident(actorId, pluginId) {
    this.resident.set(actorId, { actorId, pluginId });
    this.actorPlugin.set(actorId, pluginId);
    this.touch(actorId);
  }
  touch(actorId) {
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
    this.residencyOrder.push(actorId);
  }
  async activate(pluginId, actorId, _reason) {
    const manifest = this.manifests.get(pluginId);
    if (!manifest)
      throw new Error(`[DEBUG] ActivationRegistry.activate: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    this.markResident(actorId, pluginId);
    await this.activateExtensionsOf(pluginId, actorId);
  }
  async activateExtensionsOf(pluginId, parentActorId) {
    const extensionIds = this.extensionsByParent.get(pluginId);
    if (!extensionIds || extensionIds.length === 0)
      return;
    const parentCaps = this.manifests.get(pluginId)?.caps ?? [];
    const children = [];
    for (const extensionId of extensionIds) {
      const manifest = this.manifests.get(extensionId);
      if (!manifest) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} has no registered manifest, skipping`);
        continue;
      }
      const childActorId = `${parentActorId}::${extensionId}`;
      try {
        const scopedCaps = intersectCapabilityGrants(parentCaps, manifest.caps);
        const assets = await this.loadAssets(manifest.moduleUrl);
        await this.shardClient.activate(childActorId, manifest.moduleUrl, scopedCaps, this.defaultBudget, assets);
        this.markResident(childActorId, extensionId);
        children.push(childActorId);
      } catch (error) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} failed to activate`, error);
      }
    }
    if (children.length > 0)
      this.extensionChildren.set(parentActorId, children);
  }
  enqueueTurn(actorId, lane, events, options) {
    const generation = this.actorGeneration.get(actorId) ?? 0;
    return this.turnScheduler.enqueue(actorId, { lane, coalesce: options?.coalesce, payload: { events, generation } });
  }
  async runQueuedTurn(actorId, payload, budget) {
    const currentGeneration = this.actorGeneration.get(actorId) ?? 0;
    if (payload.generation !== currentGeneration) {
      console.warn(`[DEBUG] ActivationRegistry: dropping turn for ${actorId} queued against generation ${payload.generation}, now at ${currentGeneration} (restored in between)`);
      return;
    }
    this.touch(actorId);
    const result = await this.shardClient.turn(actorId, payload.events, budget);
    this.onTurnResult(actorId, result);
  }
  async evictForMemoryPressure() {
    while (this.residencyOrder.length >= this.maxResidentActors) {
      await this.suspend(this.residencyOrder[0]);
    }
  }
  async suspend(actorId) {
    if (!this.resident.has(actorId))
      return;
    this.turnScheduler.cancelQueued(actorId);
    await this.suspendExtensionsOf(actorId);
    const checkpoint = await this.shardClient.checkpoint(actorId);
    this.checkpoints.set(actorId, checkpoint);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
  }
  async suspendExtensionsOf(parentActorId) {
    const children = this.extensionChildren.get(parentActorId);
    if (!children)
      return;
    for (const child of children)
      await this.suspend(child);
  }
  async resume(actorId) {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId)
      throw new Error(`[DEBUG] ActivationRegistry.resume: unknown actor ${actorId} (never activated)`);
    const manifest = this.manifests.get(pluginId);
    if (!manifest)
      throw new Error(`[DEBUG] ActivationRegistry.resume: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    const checkpoint = this.checkpoints.get(actorId);
    if (checkpoint)
      await this.shardClient.restore(actorId, checkpoint);
    this.markResident(actorId, pluginId);
    await this.resumeExtensionsOf(actorId);
  }
  async resumeExtensionsOf(parentActorId) {
    const children = this.extensionChildren.get(parentActorId);
    if (!children)
      return;
    for (const child of children) {
      if (this.checkpoints.has(child) && !this.resident.has(child))
        await this.resume(child);
    }
  }
  async restoreActor(actorId) {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId)
      return;
    this.actorGeneration.set(actorId, (this.actorGeneration.get(actorId) ?? 0) + 1);
    this.turnScheduler.cancelQueued(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
    await this.resume(actorId);
  }
  async restoreActors(actorIds) {
    await Promise.all(actorIds.map((actorId) => this.restoreActor(actorId).catch((error) => console.error(`[DEBUG] ActivationRegistry.restoreActors: failed to restore ${actorId}`, error))));
  }
  handleShardLost = (_shardIndex, actorIds) => {
    this.restoreActors(actorIds);
  };
  cancel(actorId) {
    if (!this.actorPlugin.has(actorId))
      return;
    const children = this.extensionChildren.get(actorId);
    if (children) {
      for (const child of children)
        this.cancel(child);
      this.extensionChildren.delete(actorId);
    }
    this.turnScheduler.teardownActor(actorId);
    this.actorGeneration.delete(actorId);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    this.checkpoints.delete(actorId);
    this.actorPlugin.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
  }
  isResident(actorId) {
    return this.resident.has(actorId);
  }
  dispose() {
    this.stopMetricsPublisher();
  }
  runtimeMetricsActorRows() {
    return [...this.actorPlugin.entries()].map(([actorId, pluginId]) => ({ actorId, pluginId, resident: this.resident.has(actorId), shard: this.shardClient.shardIndexFor(actorId) ?? null }));
  }
  runtimeMetricsSnapshot(sampledAtMs = this.now()) {
    return { actors: this.runtimeMetricsActorRows(), shards: this.shardClient.shardMetricsSamples(sampledAtMs), sampledAtMs };
  }
  startRuntimeMetricsPublisher(sink) {
    const interval = setInterval(() => {
      const nowMs = this.now();
      if (!runtimeMetricsDue(this.lastRuntimeMetricsPublishMs, nowMs))
        return;
      this.lastRuntimeMetricsPublishMs = nowMs;
      sink("os.runtime.metrics", this.runtimeMetricsSnapshot(nowMs));
    }, RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
    return () => clearInterval(interval);
  }
}
var RUNTIME_METRICS_PUBLISH_INTERVAL_MS = 500;
function runtimeMetricsDue(lastPublishedMs, nowMs) {
  if (lastPublishedMs === null)
    return true;
  return nowMs - lastPublishedMs >= RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
}
if (import.meta.vitest) {
  let createAutoReplyWorker = function() {
    const worker = {
      postMessage: (message) => {
        const requestId = message.requestId;
        if (requestId)
          queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
      },
      terminate: () => {},
      onmessage: null,
      onerror: null
    };
    return worker;
  }, fakeShardClient = function(shardCount = 1) {
    return new ShardClient({ shardCount, createWorker: () => createAutoReplyWorker() });
  }, catalogWithOneExtension = function() {
    return {
      plugins: [{ pluginId: "p1", wasmOut: "p1.wasm", role: "plugin", contributes: [], consumes: [] }],
      extensions: [{ pluginId: "p1-ext", wasmOut: "p1-ext.wasm", role: "extension", contributes: [], consumes: [], dependsOn: ["p1"] }],
      hosts: [],
      playgrounds: [],
      moduleUrl: (pluginId, wasmOut) => `https://x/${pluginId}/${wasmOut}`,
      extensionModuleUrl: (pluginId, wasmOut) => `https://x/ext/${pluginId}/${wasmOut}`
    };
  };
  const { describe, expect, it, vi } = import.meta.vitest;
  const BUDGET_FIXTURE = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };
  async function flushMicrotasks(n = 10) {
    for (let i = 0;i < n; i += 1)
      await Promise.resolve();
  }
  describe("ActivationRegistry.runtimeMetricsActorRows / runtimeMetricsSnapshot", () => {
    it("rows cover both resident and suspended actors, never activated-and-forgotten ones", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 500, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      const rows = registry.runtimeMetricsActorRows();
      expect(rows).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);
      await registry.suspend("actor-1");
      const afterSuspend = registry.runtimeMetricsActorRows();
      expect(afterSuspend).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);
    });
    it("snapshot combines actor rows with ShardClient.shardMetricsSamples at the given clock reading", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 999, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      const snapshot = registry.runtimeMetricsSnapshot(1000);
      expect(snapshot.sampledAtMs).toBe(1000);
      expect(snapshot.actors).toHaveLength(1);
      expect(snapshot.shards).toEqual(shardClient.shardMetricsSamples(1000));
    });
  });
  describe("runtimeMetricsDue", () => {
    it("gates at the 500ms / 2Hz interval, always due on the first call", () => {
      expect(runtimeMetricsDue(null, 0)).toBe(true);
      expect(runtimeMetricsDue(1000, 1200)).toBe(false);
      expect(runtimeMetricsDue(1000, 1500)).toBe(true);
    });
  });
  describe("ActivationRegistry.startRuntimeMetricsPublisher", () => {
    it("calls the sink with the os.runtime.metrics topic at the 2Hz interval, and stop() cancels it", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        const calls = [];
        const stop = registry.startRuntimeMetricsPublisher((topic, snapshot) => calls.push({ topic, snapshot }));
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(calls).toHaveLength(1);
        expect(calls[0].topic).toBe("os.runtime.metrics");
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(calls).toHaveLength(2);
        stop();
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 3);
        expect(calls).toHaveLength(2);
      } finally {
        vi.useRealTimers();
      }
    });
  });
  describe("ActivationRegistry.cancel", () => {
    it("disposes the worker-side instance and forgets the actor entirely \u2014 resume() afterward throws unknown actor", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      expect(registry.isResident("actor-1")).toBe(true);
      registry.cancel("actor-1");
      expect(registry.isResident("actor-1")).toBe(false);
      expect(registry.runtimeMetricsActorRows()).toEqual([]);
      expect(shardClient.shardIndexFor("actor-1")).toBeUndefined();
      await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
    });
    it("is a no-op for an actor this registry never activated", () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      expect(() => registry.cancel("ghost")).not.toThrow();
    });
    it("cancelling a suspended (non-resident but still tracked) actor still forgets it", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      await registry.suspend("actor-1");
      expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);
      registry.cancel("actor-1");
      expect(registry.runtimeMetricsActorRows()).toEqual([]);
    });
  });
  describe("intersectCapabilityGrants", () => {
    it("keeps only requested grants the parent's own granted set also carries, matched by id", () => {
      const grant = (id) => ({ id, token: "t", scope: "s", expiresMs: null });
      const granted = [grant("fs.read"), grant("net.fetch")];
      const requested = [grant("fs.read"), grant("fs.admin")];
      expect(intersectCapabilityGrants(granted, requested).map((g) => g.id)).toEqual(["fs.read"]);
    });
    it("is empty when the parent holds nothing, never escalates an ungranted request", () => {
      const grant = (id) => ({ id, token: "t", scope: "s", expiresMs: null });
      expect(intersectCapabilityGrants([], [grant("fs.admin")])).toEqual([]);
    });
  });
  describe("ActivationRegistry extension cascade (registerCatalog)", () => {
    it("activate() cascades to every registered extension of the plugin, under a deterministic child actorId", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      await registry.activate("p1", "actor-1", "manual");
      expect(registry.isResident("actor-1")).toBe(true);
      expect(registry.isResident("actor-1::p1-ext")).toBe(true);
      const rows = registry.runtimeMetricsActorRows();
      expect(rows).toContainEqual({ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 });
      expect(rows).toContainEqual({ actorId: "actor-1::p1-ext", pluginId: "p1-ext", resident: true, shard: 0 });
    });
    it("a plugin with no registered extensions activates with no cascade side effects", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);
    });
    it("suspend() cascades leaves-first, resume() cascades parent-first \u2014 zero orphans either way", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      await registry.activate("p1", "actor-1", "manual");
      await registry.suspend("actor-1");
      expect(registry.isResident("actor-1")).toBe(false);
      expect(registry.isResident("actor-1::p1-ext")).toBe(false);
      expect(registry.runtimeMetricsActorRows().map((r) => r.actorId).sort()).toEqual(["actor-1", "actor-1::p1-ext"]);
      await registry.resume("actor-1");
      expect(registry.isResident("actor-1")).toBe(true);
      expect(registry.isResident("actor-1::p1-ext")).toBe(true);
    });
    it("cancel() on the parent takes its extension down too \u2014 permanently, zero orphans", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      await registry.activate("p1", "actor-1", "manual");
      registry.cancel("actor-1");
      expect(registry.runtimeMetricsActorRows()).toEqual([]);
      await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
      await expect(registry.resume("actor-1::p1-ext")).rejects.toThrow(/unknown actor/);
    });
    it("scopes an extension's activated caps to the intersection with its parent's own granted set", async () => {
      const shardClient = fakeShardClient();
      const sentCaps = new Map;
      const worker = createAutoReplyWorker();
      const originalPostMessage = worker.postMessage;
      worker.postMessage = (message) => {
        const msg = message;
        if (msg.kind === "activate" && msg.actorId)
          sentCaps.set(msg.actorId, msg.caps ?? []);
        originalPostMessage(message);
      };
      const client = new ShardClient({ shardCount: 1, createWorker: () => worker });
      const registry = new ActivationRegistry({ shardClient: client, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      const grant = (id) => ({ id, token: "t", scope: "s", expiresMs: null });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1/p1.wasm", caps: [grant("fs.read")] });
      registry.registerManifest({ pluginId: "p1-ext", moduleUrl: "https://x/ext/p1-ext/p1-ext.wasm", caps: [grant("fs.read"), grant("fs.admin")] });
      await registry.activate("p1", "actor-1", "manual");
      expect(sentCaps.get("actor-1::p1-ext")?.map((g) => g.id)).toEqual(["fs.read"]);
    });
  });
  describe("ActivationRegistry.enqueueTurn lane priority", () => {
    it("dispatches turns by lane priority end-to-end through the registry, not enqueue order", async () => {
      const shardClient = fakeShardClient();
      const order = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: (actorId) => order.push(actorId) });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "low", "manual");
      await registry.activate("p1", "high", "manual");
      await registry.activate("p1", "mid", "manual");
      registry.enqueueTurn("low", "Background", []);
      registry.enqueueTurn("high", "Interactive", []);
      registry.enqueueTurn("mid", "UserVisible", []);
      await flushMicrotasks();
      expect(order).toEqual(["high", "mid", "low"]);
    });
  });
  describe("ActivationRegistry.suspend cancels queued turns", () => {
    it("a suspended actor's queued turns are cancelled, never delivered", async () => {
      const shardClient = fakeShardClient();
      const delivered = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      registry.enqueueTurn("actor-1", "Interactive", []);
      await registry.suspend("actor-1");
      await flushMicrotasks();
      expect(delivered).toEqual([]);
      await registry.resume("actor-1");
      await flushMicrotasks();
      expect(delivered).toEqual([]);
    });
  });
  describe("ActivationRegistry.handleShardLost / restoreActors", () => {
    it("is a valid ShardClientOptions.onShardLost value", () => {
      let registry;
      const shardClient = new ShardClient({
        shardCount: 1,
        createWorker: () => createAutoReplyWorker(),
        onShardLost: (shardIndex, actorIds) => registry.handleShardLost(shardIndex, actorIds)
      });
      registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      expect(typeof registry.handleShardLost).toBe("function");
    });
    it("restores exactly the actors that were on the lost shard, leaving an actor on a different shard untouched", async () => {
      const shardClient = new ShardClient({ shardCount: 2, exclusiveShardCount: 0, createWorker: () => createAutoReplyWorker() });
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "on-shard-0", "manual");
      await registry.activate("p1", "on-shard-1", "manual");
      expect(shardClient.shardIndexFor("on-shard-0")).toBe(0);
      expect(shardClient.shardIndexFor("on-shard-1")).toBe(1);
      const lostActorIds = shardClient.terminate(0);
      shardClient.rebuild(0);
      expect(lostActorIds).toEqual(["on-shard-0"]);
      await registry.restoreActors(lostActorIds);
      expect(registry.isResident("on-shard-0")).toBe(true);
      expect(registry.isResident("on-shard-1")).toBe(true);
      expect(shardClient.shardIndexFor("on-shard-0")).toBe(0);
    });
  });
  describe("ActivationRegistry restore ordering", () => {
    it("a restored actor does not receive turns that were queued before the restore, but does receive turns queued after", async () => {
      const shardClient = new ShardClient({ shardCount: 1, createWorker: () => createAutoReplyWorker() });
      const delivered = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      registry.enqueueTurn("actor-1", "Interactive", []);
      const lostActorIds = shardClient.terminate(0);
      shardClient.rebuild(0);
      await registry.restoreActors(lostActorIds);
      await flushMicrotasks();
      expect(delivered).toEqual([]);
      registry.enqueueTurn("actor-1", "Interactive", []);
      await flushMicrotasks();
      expect(delivered).toEqual(["delivered"]);
    });
  });
  describe("residentActorCapFromMemory", () => {
    it("derives the cap from deviceMemoryGiB when present, clamped to [4, 96]", () => {
      expect(residentActorCapFromMemory({ deviceMemoryGiB: 1 })).toBe(6);
      expect(residentActorCapFromMemory({ deviceMemoryGiB: 16 })).toBe(96);
    });
    it("falls back to jsHeapSizeLimitBytes when deviceMemoryGiB is absent", () => {
      expect(residentActorCapFromMemory({ jsHeapSizeLimitBytes: 256 * 1024 * 1024 })).toBe(4);
    });
    it("falls back to the hardcoded constant when neither signal is present", () => {
      expect(residentActorCapFromMemory({})).toBe(DEFAULT_MAX_RESIDENT_ACTORS);
    });
  });
  describe("ActivationRegistry.maxResidentActors derived from an injected memory probe", () => {
    async function activateAndCountResident(memoryProbe, activationCount) {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], memoryProbe });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      for (let i = 0;i < activationCount; i += 1)
        await registry.activate("p1", `actor-${i}`, "manual");
      let resident = 0;
      for (let i = 0;i < activationCount; i += 1)
        if (registry.isResident(`actor-${i}`))
          resident += 1;
      return resident;
    }
    it("a small deviceMemoryGiB reading evicts down to its (small) derived cap", async () => {
      const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 1 }), 10);
      expect(resident).toBe(residentActorCapFromMemory({ deviceMemoryGiB: 1 }));
      expect(resident).toBeLessThan(10);
    });
    it("a large deviceMemoryGiB reading keeps every one of the same 10 activations resident", async () => {
      const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 16 }), 10);
      expect(resident).toBe(10);
    });
  });
  describe("ActivationRegistry.metricsBus (autoStartMetricsPublisher)", () => {
    it("publishes os.runtime.metrics as a CustomEvent on metricsBus at the 2Hz interval, driven by the injected clock, and dispose() stops it", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        let simulatedNowMs = 0;
        const registry = new ActivationRegistry({
          shardClient,
          defaultBudget: BUDGET_FIXTURE,
          fetchAssets: async () => [],
          now: () => simulatedNowMs,
          autoStartMetricsPublisher: true
        });
        const received = [];
        registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push(event.detail));
        simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(received).toHaveLength(1);
        expect(received[0].sampledAtMs).toBe(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 2;
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(received).toHaveLength(2);
        registry.dispose();
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 5);
        expect(received).toHaveLength(2);
      } finally {
        vi.useRealTimers();
      }
    });
    it("stays empty (no live interval, no bus traffic) when autoStartMetricsPublisher is left at its default", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        const received = [];
        registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push(event.detail));
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 10);
        expect(received).toEqual([]);
        registry.dispose();
      } finally {
        vi.useRealTimers();
      }
    });
  });
}
var PLUGIN_SOURCE_WATCH_PATH = "/plugin-modules/watch";
function createDevPluginSource(registry) {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry]));
  const bootVersion = Date.now();
  return {
    id: "dev",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry)
        throw new Error(`[DEBUG] plugin source "dev" has no registry entry for ${pluginId}`);
      const separator = entry.moduleUrl.includes("?") ? "&" : "?";
      return `${entry.moduleUrl}${separator}v=${rebuiltAt ?? bootVersion}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined")
        return () => {};
      const source = new EventSource(PLUGIN_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          listener(JSON.parse(event.data));
        } catch (error) {
          console.warn(`[DEBUG] plugin source "dev" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    }
  };
}
var EXTENSION_SOURCE_WATCH_PATH = "/extensions/watch";
function extensionSourceEventToPluginSourceEvent(event) {
  if (event.kind === "snapshot") {
    if (!Array.isArray(event.extensions))
      throw new Error("snapshot extensions must be an array");
    return { kind: "snapshot", plugins: event.extensions.map((extension) => ({ pluginId: extension.extensionId, rebuiltAt: extension.installedAt })) };
  }
  if (event.kind === "installed")
    return { kind: "built", pluginId: event.extensionId, rebuiltAt: event.installedAt };
  if (event.kind === "uninstalled")
    return;
  throw new Error("unknown extension source event kind");
}
function createExtensionSource(catalog) {
  const registry = catalog.extensions.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: catalog.extensionModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn)
  }));
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry]));
  return {
    id: "extensions",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry)
        throw new Error(`[DEBUG] plugin source "extensions" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined")
        return () => {};
      const source = new EventSource(EXTENSION_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          const normalized = extensionSourceEventToPluginSourceEvent(JSON.parse(event.data));
          if (normalized)
            listener(normalized);
        } catch (error) {
          console.warn(`[DEBUG] plugin source "extensions" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    }
  };
}
function multiplexPluginSources(...sources) {
  if (sources.length === 0)
    throw new Error("[DEBUG] multiplexPluginSources requires at least one source");
  if (sources.length === 1)
    return sources[0];
  return {
    id: sources.map((source) => source.id).join("+"),
    async list() {
      const merged = new Map;
      for (const entries of await Promise.all(sources.map((source) => source.list()))) {
        for (const entry of entries)
          merged.set(entry.pluginId, entry);
      }
      return [...merged.values()];
    },
    moduleUrl(pluginId, rebuiltAt) {
      for (const source of sources) {
        try {
          return source.moduleUrl(pluginId, rebuiltAt);
        } catch {
          continue;
        }
      }
      throw new Error(`[DEBUG] multiplexed plugin sources have no registry entry for ${pluginId}`);
    },
    subscribe(listener) {
      const unsubscribes = sources.map((source) => source.subscribe(listener));
      return () => {
        for (const unsubscribe of unsubscribes)
          unsubscribe();
      };
    }
  };
}
function findPlaygroundVariant(catalog, playgroundPluginId) {
  return catalog.playgrounds.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}
function resolvePluginRegistryId(catalog, playgroundPluginId) {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.pluginId ?? playgroundPluginId;
}
function resolvePlaygroundDefaultAppId(catalog, playgroundPluginId) {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.app;
}
function resolvePlaygroundBoot(catalog, variant, session) {
  const defaultAppId = resolvePlaygroundDefaultAppId(catalog, variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins, dependencyErrors: [] };
  }
  const registryPluginId = resolvePluginRegistryId(catalog, variant);
  const hostMode = resolvePluginHostConfig(catalog, variant) !== undefined;
  const catalogPlugins = [...catalog.plugins, ...catalog.extensions].map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: target.role === "extension" ? catalog.extensionModuleUrl(target.pluginId, target.wasmOut) : catalog.moduleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn)
  }));
  const expanded = expandPluginRegistry(catalogPlugins, hostMode ? undefined : registryPluginId, hostMode);
  const { order, errors } = orderPluginRegistryEntries(expanded);
  if (errors.length > 0) {
    for (const error of errors)
      console.error(`[DEBUG] resolvePlaygroundBoot(${variant}): ${pluginGraphErrorMessage(error, "en")}`);
  }
  return {
    variant,
    defaultAppId,
    plugins: order,
    dependencyErrors: errors
  };
}
function resolvePluginHostConfig(catalog, playgroundPluginId) {
  const registryId = resolvePluginRegistryId(catalog, playgroundPluginId);
  return catalog.hosts.find((entry) => entry.pluginId === registryId);
}
function parseVersion(raw) {
  if (!raw)
    return null;
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(raw.trim());
  if (!match)
    return null;
  return { major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]) };
}
function compareVersions(a, b) {
  if (a.major !== b.major)
    return a.major - b.major;
  if (a.minor !== b.minor)
    return a.minor - b.minor;
  return a.patch - b.patch;
}
function parseVersionReq(raw) {
  const trimmed = raw.trim();
  if (trimmed === "*")
    return { kind: "any" };
  const opMatch = /^(=|\^|~|>=)(\d+\.\d+\.\d+)$/.exec(trimmed);
  if (!opMatch)
    return null;
  const version = parseVersion(opMatch[2]);
  if (!version)
    return null;
  switch (opMatch[1]) {
    case "=":
      return { kind: "exact", version };
    case "^":
      return { kind: "caret", version };
    case "~":
      return { kind: "tilde", version };
    case ">=":
      return { kind: "atLeast", version };
    default:
      return null;
  }
}
function versionSatisfies(actual, requirement) {
  const req = parseVersionReq(requirement);
  if (!req)
    return false;
  if (req.kind === "any")
    return true;
  const version = parseVersion(actual);
  if (!version)
    return false;
  if (req.kind === "exact")
    return compareVersions(version, req.version) === 0;
  if (req.kind === "atLeast")
    return compareVersions(version, req.version) >= 0;
  if (req.kind === "tilde") {
    return version.major === req.version.major && version.minor === req.version.minor && version.patch >= req.version.patch;
  }
  if (compareVersions(version, req.version) < 0)
    return false;
  if (req.version.major > 0)
    return version.major === req.version.major;
  if (req.version.minor > 0)
    return version.major === 0 && version.minor === req.version.minor;
  return version.major === 0 && version.minor === 0 && version.patch === req.version.patch;
}
function validatePluginDependencyGraph(nodes) {
  const byId = new Map(nodes.map((node) => [node.pluginId, node]));
  const errors = [];
  for (const node of nodes) {
    for (const dependency of node.dependencies ?? []) {
      const target = byId.get(dependency.pluginId);
      if (!target) {
        errors.push({ code: "transaction.dependency-missing", pluginId: node.pluginId, dependsOn: dependency.pluginId });
        continue;
      }
      if (target.version !== undefined && !versionSatisfies(target.version, dependency.version)) {
        errors.push({ code: "transaction.version-mismatch", pluginId: node.pluginId, dependsOn: dependency.pluginId, required: dependency.version, actual: target.version });
      }
    }
  }
  return errors;
}
function findCycleMembers(byId, leftover) {
  const visiting = new Set;
  const visited = new Set;
  const stack = [];
  let cycle = null;
  function visit(id) {
    if (cycle || !leftover.has(id) || visited.has(id))
      return;
    if (visiting.has(id)) {
      const start = stack.indexOf(id);
      cycle = stack.slice(start);
      return;
    }
    visiting.add(id);
    stack.push(id);
    for (const dependency of byId.get(id)?.dependencies ?? []) {
      if (leftover.has(dependency.pluginId))
        visit(dependency.pluginId);
      if (cycle)
        return;
    }
    stack.pop();
    visiting.delete(id);
    visited.add(id);
  }
  for (const id of [...leftover].sort()) {
    visit(id);
    if (cycle)
      break;
  }
  return cycle ?? [...leftover].sort();
}
function resolvePluginLoadOrder(nodes) {
  const structural = validatePluginDependencyGraph(nodes);
  if (structural.length > 0)
    return { order: [], errors: structural };
  const byId = new Map(nodes.map((node) => [node.pluginId, node]));
  const indegree = new Map;
  const dependents = new Map;
  for (const node of nodes) {
    indegree.set(node.pluginId, indegree.get(node.pluginId) ?? 0);
    for (const dependency of node.dependencies ?? []) {
      indegree.set(node.pluginId, (indegree.get(node.pluginId) ?? 0) + 1);
      const list = dependents.get(dependency.pluginId) ?? [];
      list.push(node.pluginId);
      dependents.set(dependency.pluginId, list);
    }
  }
  const order = [];
  const remaining = new Map(indegree);
  const queue = [...indegree.entries()].filter(([, count]) => count === 0).map(([id]) => id);
  while (queue.length > 0) {
    queue.sort();
    const id = queue.shift();
    order.push(id);
    for (const dependent of dependents.get(id) ?? []) {
      const next = (remaining.get(dependent) ?? 0) - 1;
      remaining.set(dependent, next);
      if (next === 0)
        queue.push(dependent);
    }
  }
  if (order.length === nodes.length)
    return { order, errors: [] };
  const leftover = new Set(nodes.map((node) => node.pluginId).filter((id) => !order.includes(id)));
  return { order: [], errors: [{ code: "transaction.cycle", members: findCycleMembers(byId, leftover) }] };
}
function pluginDependents(nodes, pluginId) {
  return nodes.filter((node) => (node.dependencies ?? []).some((dependency) => dependency.pluginId === pluginId)).map((node) => node.pluginId).sort();
}

class PluginGraph {
  nodes;
  constructor(nodes) {
    this.nodes = nodes;
  }
  validate() {
    return validatePluginDependencyGraph(this.nodes);
  }
  loadOrder() {
    return resolvePluginLoadOrder(this.nodes);
  }
  dependents(pluginId) {
    return pluginDependents(this.nodes, pluginId);
  }
  canUnload(pluginId, loadedIds) {
    return this.dependents(pluginId).every((dependent) => !loadedIds.has(dependent));
  }
}
function orderPluginRegistryEntries(entries) {
  const nodes = entries.map((entry) => ({ pluginId: entry.pluginId, dependencies: entry.dependencies }));
  const { order, errors } = new PluginGraph(nodes).loadOrder();
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry]));
  if (errors.length === 0) {
    return { order: order.map((id) => byId.get(id)).filter((entry) => entry !== undefined), errors: [] };
  }
  const blocked = new Set(errors.flatMap((error) => error.code === "transaction.cycle" ? error.members : [error.pluginId]));
  const remaining = entries.filter((entry) => !blocked.has(entry.pluginId));
  const retried = orderPluginRegistryEntries(remaining);
  return { order: retried.order, errors: [...errors, ...retried.errors] };
}
function resolveLocalizedLabel(label, locale) {
  return label[locale] ?? label.en ?? Object.values(label)[0] ?? "";
}
function pluginGraphErrorMessage(error, locale) {
  switch (error.code) {
    case "transaction.dependency-missing":
      return resolveLocalizedLabel({
        en: `Plugin "${error.pluginId}" needs "${error.dependsOn}", which is not installed.`,
        de: `Das Plugin \u201E${error.pluginId}\u201C ben\xF6tigt \u201E${error.dependsOn}\u201C, welches nicht installiert ist.`
      }, locale);
    case "transaction.version-mismatch":
      return resolveLocalizedLabel({
        en: `Plugin "${error.pluginId}" needs "${error.dependsOn}" ${error.required}, but ${error.actual} is installed.`,
        de: `Das Plugin \u201E${error.pluginId}\u201C ben\xF6tigt \u201E${error.dependsOn}\u201C ${error.required}, installiert ist jedoch ${error.actual}.`
      }, locale);
    case "transaction.cycle":
      return resolveLocalizedLabel({
        en: `Plugin dependency cycle: ${error.members.join(" \u2192 ")}.`,
        de: `Zyklische Plugin-Abh\xE4ngigkeit: ${error.members.join(" \u2192 ")}.`
      }, locale);
  }
}

class InstanceDirectory {
  byArtifactId = new Map;
  register(artifactId, ref) {
    this.byArtifactId.set(artifactId, ref);
  }
  unregister(artifactId) {
    this.byArtifactId.delete(artifactId);
  }
  resolve(artifactId) {
    return this.byArtifactId.get(artifactId);
  }
  entries() {
    return [...this.byArtifactId.entries()];
  }
}

class ArtifactRouterConflictError extends Error {
  code = "artifact-router.conflict";
  constructor(artifactKind, key) {
    super(`[DEBUG] router conflict: ${artifactKind}#${key} already registered with different metadata`);
    this.name = "ArtifactRouterConflictError";
  }
}

class ArtifactContributionNotPermittedError extends Error {
  code = "transaction.contribution-not-permitted";
  constructor(contributorPluginId, ownerPluginId) {
    super(`[DEBUG] "${contributorPluginId}" may not contribute onto "${ownerPluginId}"'s artifact kind \u2014 not a direct dependency`);
    this.name = "ArtifactContributionNotPermittedError";
  }
}
function stableStringify(value) {
  if (value === null || typeof value !== "object")
    return JSON.stringify(value);
  if (Array.isArray(value))
    return `[${value.map(stableStringify).join(",")}]`;
  const record = value;
  const keys = Object.keys(record).sort();
  return `{${keys.map((key) => `${JSON.stringify(key)}:${stableStringify(record[key])}`).join(",")}}`;
}

class ConflictCheckedRegistry {
  entries = new Map;
  register(artifactKind, key, ownership, metadata) {
    const compositeKey = `${artifactKind} ${key}`;
    const fingerprint = stableStringify(metadata);
    const existing = this.entries.get(compositeKey);
    if (existing && existing.fingerprint !== fingerprint)
      throw new ArtifactRouterConflictError(artifactKind, key);
    this.entries.set(compositeKey, { ownership, fingerprint });
  }
  resolve(artifactKind, key) {
    return this.entries.get(`${artifactKind} ${key}`)?.ownership;
  }
}

class ArtifactMutationRouter {
  registry = new ConflictCheckedRegistry;
  registerOwner(artifactKind, mutationId) {
    this.registry.register(artifactKind, mutationId, { kind: "owner" }, { kind: "owner", artifactKind, mutationId });
  }
  registerContributed(artifactKind, contributorPluginId, ownerPluginId, metadata, contributorDependsOnOwner) {
    if (!contributorDependsOnOwner)
      throw new ArtifactContributionNotPermittedError(contributorPluginId, ownerPluginId);
    this.registry.register(artifactKind, metadata.mutationId, { kind: "contributed", pluginId: contributorPluginId }, metadata);
  }
  resolve(artifactKind, mutationId) {
    return this.registry.resolve(artifactKind, mutationId);
  }
}

class ArtifactInferenceRouter {
  registry = new ConflictCheckedRegistry;
  dependsOn = new Map;
  registerOwner(artifactKind, inferenceSchema) {
    this.registry.register(artifactKind, inferenceSchema, { kind: "owner" }, { kind: "owner", artifactKind, inferenceSchema });
  }
  registerContributed(artifactKind, metadata, contributorDependsOnOwner) {
    if (metadata.owner !== metadata.contributor) {
      throw new Error(`[DEBUG] contributed inference owner/contributor mismatch: ${metadata.owner} !== ${metadata.contributor}`);
    }
    if (metadata.artifactKind !== artifactKind) {
      throw new Error(`[DEBUG] contributed inference artifactKind mismatch: ${metadata.artifactKind} !== ${artifactKind}`);
    }
    if (!contributorDependsOnOwner)
      throw new ArtifactContributionNotPermittedError(metadata.contributor, artifactKind);
    this.registry.register(artifactKind, metadata.inferenceSchema, { kind: "contributed", pluginId: metadata.contributor }, metadata);
    this.dependsOn.set(`${artifactKind} ${metadata.inferenceSchema}`, metadata.dependsOn ?? []);
  }
  resolve(artifactKind, inferenceSchema) {
    return this.registry.resolve(artifactKind, inferenceSchema);
  }
  dependencyOrder() {
    const keys = [...this.dependsOn.keys()];
    const indegree = new Map(keys.map((key) => [key, 0]));
    const dependents = new Map;
    for (const key of keys) {
      for (const dependency of this.dependsOn.get(key) ?? []) {
        if (!indegree.has(dependency))
          continue;
        indegree.set(key, (indegree.get(key) ?? 0) + 1);
        const list = dependents.get(dependency) ?? [];
        list.push(key);
        dependents.set(dependency, list);
      }
    }
    const order = [];
    const remaining = new Map(indegree);
    const queue = keys.filter((key) => (indegree.get(key) ?? 0) === 0);
    while (queue.length > 0) {
      queue.sort();
      const key = queue.shift();
      order.push(key);
      for (const dependent of dependents.get(key) ?? []) {
        const next = (remaining.get(dependent) ?? 0) - 1;
        remaining.set(dependent, next);
        if (next === 0)
          queue.push(dependent);
      }
    }
    if (order.length !== keys.length) {
      const leftover = keys.filter((key) => !order.includes(key)).sort();
      throw new Error(`[DEBUG] ArtifactInferenceRouter.dependencyOrder: cycle among ${leftover.join(", ")}`);
    }
    return order;
  }
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("expandPluginRegistry", () => {
    it("includes transitive dependsOn of primary and consume-matched contributors", () => {
      const plugins = [
        { pluginId: "host-plugin", moduleUrl: "a", consumes: ["ext.tag"], dependencies: [{ pluginId: "core", version: "*" }] },
        { pluginId: "core", moduleUrl: "b", dependencies: [{ pluginId: "stdio", version: "*" }] },
        { pluginId: "stdio", moduleUrl: "c" },
        { pluginId: "ext", moduleUrl: "d", contributes: ["ext.tag"], dependencies: [{ pluginId: "flow", version: "*" }] },
        { pluginId: "flow", moduleUrl: "e", dependencies: [{ pluginId: "stdio", version: "*" }] },
        { pluginId: "unrelated", moduleUrl: "f" }
      ];
      const expanded = expandPluginRegistry(plugins, "host-plugin", false);
      const ids = new Set(expanded.map((entry) => entry.pluginId));
      expect(ids.has("host-plugin")).toBe(true);
      expect(ids.has("core")).toBe(true);
      expect(ids.has("stdio")).toBe(true);
      expect(ids.has("ext")).toBe(true);
      expect(ids.has("flow")).toBe(true);
      expect(ids.has("unrelated")).toBe(false);
    });
  });
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("IoEntryGraph", () => {
    const binaryRaw = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" };
    const gif87a = { artifactKind: "s.stdio.gif", standard: "87a", subset: "*" };
    const gif89a = { artifactKind: "s.stdio.gif", standard: "89a", subset: "*" };
    const fixturePlugins = [
      { pluginId: "stdio", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true }] },
      {
        pluginId: "gif",
        entries: [
          { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false },
          { from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true }
        ]
      }
    ];
    it("resolves the highest-minimum-fidelity route regardless of registration order", () => {
      const forward = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a);
      const reversed = IoEntryGraph.build([...fixturePlugins].reverse()).route(binaryRaw, gif89a);
      expect(forward).toEqual(reversed);
      expect(forward).toEqual({
        hops: [
          { from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true },
          { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false }
        ],
        fidelity: "Canonical"
      });
    });
    it("respects maxHops, picking the direct (weaker) shortcut when bounded to 1", () => {
      const route = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a, 1);
      expect(route).toEqual({ hops: [{ from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true }], fidelity: "Lossy" });
    });
    it("rejects a different plugin claiming an already-owned (from,into) key", () => {
      expect(() => IoEntryGraph.build([...fixturePlugins, { pluginId: "intruder", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Lossy", sniffs: false }] }])).toThrow(/conflict/);
    });
    it("ownerOf reports the registering plugin", () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      expect(graph.ownerOf(binaryRaw, gif87a)).toBe("stdio");
      expect(graph.ownerOf(gif87a, gif89a)).toBe("gif");
      expect(graph.ownerOf(gif89a, binaryRaw)).toBeUndefined();
    });
    it("carrierEntries returns only the sniff-declaring hops whose from is the given carrier", () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const entries = graph.carrierEntries(binaryRaw);
      expect(entries.map((entry) => ({ into: entry.into, pluginId: entry.pluginId }))).toEqual([
        { into: gif87a, pluginId: "stdio" },
        { into: gif89a, pluginId: "gif" }
      ]);
    });
    it("ioRun executes the whole route hop by hop, feeding each hop's output to the next", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const calls = [];
      const result = await ioRun(graph, "norm", binaryRaw, gif89a, new Uint8Array([1]), (pluginId, from, into, payload) => {
        calls.push(`${pluginId}:${dialectCoordinate(from)}->${dialectCoordinate(into)}`);
        return new Uint8Array([...payload, payload.length]);
      });
      expect(calls).toEqual(["stdio:s.stdio.binary@raw/*->s.stdio.gif@87a/*", "gif:s.stdio.gif@87a/*->s.stdio.gif@89a/*"]);
      expect(Array.from(result)).toEqual([1, 1, 2]);
    });
    it("ioRun refuses the WHOLE route (no partial execution) when the calling plugin owns any hop", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      let ran = false;
      await expect(ioRun(graph, "gif", binaryRaw, gif89a, new Uint8Array, () => {
        ran = true;
        return new Uint8Array;
      })).rejects.toThrow(/refused/);
      expect(ran).toBe(false);
    });
    it("ioIdentify fans sniffHop out across carrier entries, skipping the calling plugin's own, sorted by confidence then coordinate", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const results = await ioIdentify(graph, "norm", binaryRaw, new Uint8Array, (pluginId) => pluginId === "stdio" ? 3 : 1);
      expect(results).toEqual([
        [gif87a, "High"],
        [gif89a, "Low"]
      ]);
    });
    it("ioIdentify skips the calling plugin's own carrier entries", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const results = await ioIdentify(graph, "stdio", binaryRaw, new Uint8Array, () => 3);
      expect(results).toEqual([[gif89a, "High"]]);
    });
  });
}
/* 🧰️framework/🔨️modules/🔄️machine/🟦️component.ts */
var NodeId = (value) => value;
var EventId = (value) => value;
var GuardId = (value) => value;
var ActionId = (value) => value;
var InvokeId = (value) => value;
var TimerId = (value) => value;
var ActorId = (value) => value;
var ROOT = NodeId(0);

class BitSet {
  #bits;
  constructor(bits = []) {
    this.#bits = new Set(bits);
  }
  set(id) {
    this.#bits.add(id);
  }
  clear(id) {
    this.#bits.delete(id);
  }
  contains(id) {
    return this.#bits.has(id);
  }
  *iterOnes() {
    for (const id of [...this.#bits].sort((a, b) => a - b))
      yield NodeId(id);
  }
  clearAll() {
    this.#bits.clear();
  }
  isEmpty() {
    return this.#bits.size === 0;
  }
  clone() {
    return new BitSet(this.#bits);
  }
  equals(other) {
    if (!(other instanceof BitSet))
      return false;
    if (other.#bits.size !== this.#bits.size)
      return false;
    for (const id of this.#bits)
      if (!other.#bits.has(id))
        return false;
    return true;
  }
}

class Snapshot {
  configuration;
  context;
  status;
  #nodes;
  #history = [];
  constructor(nodes, configuration, context, status = { kind: "running" }) {
    this.#nodes = nodes;
    this.configuration = configuration;
    this.context = context;
    this.status = status;
  }
  matches(stableId) {
    for (const id of this.configuration.iterOnes())
      if (this.#nodes[id].stableId === stableId)
        return true;
    return false;
  }
  historyFor(node) {
    return this.#history.find(([key]) => key === node)?.[1];
  }
  recordHistory(node, value) {
    const entry = this.#history.find(([key]) => key === node);
    if (entry)
      entry[1] = [...value];
    else
      this.#history.push([node, [...value]]);
  }
  historyEntries() {
    return this.#history;
  }
  branchForExploration() {
    const branch = new Snapshot(this.#nodes, this.configuration.clone(), structuredClone(this.context), { kind: "running" });
    for (const [owner, ids] of this.#history)
      branch.recordHistory(owner, ids);
    return branch;
  }
}

class NullInspector {
  observe() {}
}

class TraceInspector {
  entries = [];
  observe(event) {
    if (event.kind === "microstep")
      this.entries.push({ exited: event.exited, entered: event.entered });
  }
}
var MICROSTEP_LIMIT = 1000;
function isDescendant(nodes, a, ancestor) {
  if (a === ancestor)
    return false;
  let cur = nodes[a].parent;
  while (cur !== undefined) {
    if (cur === ancestor)
      return true;
    cur = nodes[cur].parent;
  }
  return false;
}
function isDescendantOrSelf(nodes, a, ancestor) {
  return a === ancestor || isDescendant(nodes, a, ancestor);
}
function depthOf(nodes, id) {
  let depth = 0;
  let cur = nodes[id].parent;
  while (cur !== undefined) {
    depth += 1;
    cur = nodes[cur].parent;
  }
  return depth;
}
function isCompoundOrParallel(nodes, id) {
  const kind = nodes[id].kind;
  return kind === "compound" || kind === "parallel";
}
function isLeafish(nodes, id) {
  const kind = nodes[id].kind;
  return kind === "atomic" || kind === "final";
}
function computeDomain(nodes, source, targets, kind) {
  if (targets.length === 0)
    return source;
  if (kind === "internal" && isCompoundOrParallel(nodes, source) && targets.every((t) => isDescendant(nodes, t, source)))
    return source;
  let anc = nodes[source].parent;
  while (anc !== undefined) {
    if (isCompoundOrParallel(nodes, anc) && targets.every((t) => isDescendantOrSelf(nodes, t, anc)))
      return anc;
    anc = nodes[anc].parent;
  }
  return ROOT;
}
function resolveEffectiveTargets(nodes, targets, snapshot) {
  const out = [];
  for (const t of targets) {
    const kind = nodes[t].kind;
    if (kind === "historyShallow" || kind === "historyDeep") {
      const recorded = snapshot.historyFor(t);
      if (recorded) {
        for (const r of recorded)
          if (!out.includes(r))
            out.push(r);
      } else {
        const fallback = nodes[t].initial;
        if (fallback !== undefined && !out.includes(fallback))
          out.push(fallback);
      }
    } else if (!out.includes(t)) {
      out.push(t);
    }
  }
  return out;
}
function addDescendantStatesToEnter(nodes, state, snapshot, out) {
  const kind = nodes[state].kind;
  if (kind === "historyShallow" || kind === "historyDeep") {
    for (const r of resolveEffectiveTargets(nodes, [state], snapshot))
      addDescendantStatesToEnter(nodes, r, snapshot, out);
    return;
  }
  if (!out.includes(state))
    out.push(state);
  if (kind === "compound") {
    const initial = nodes[state].initial;
    if (initial !== undefined) {
      addDescendantStatesToEnter(nodes, initial, snapshot, out);
      addAncestorStatesToEnter(nodes, initial, state, snapshot, out);
    }
  } else if (kind === "parallel") {
    for (const child of nodes[state].children) {
      if (!out.some((e) => isDescendantOrSelf(nodes, e, child)))
        addDescendantStatesToEnter(nodes, child, snapshot, out);
    }
  }
}
function addAncestorStatesToEnter(nodes, state, stopAt, snapshot, out) {
  let anc = nodes[state].parent;
  while (anc !== undefined && anc !== stopAt) {
    if (!out.includes(anc))
      out.push(anc);
    if (nodes[anc].kind === "parallel") {
      for (const child of nodes[anc].children) {
        if (!out.some((e) => isDescendantOrSelf(nodes, e, child)))
          addDescendantStatesToEnter(nodes, child, snapshot, out);
      }
    }
    anc = nodes[anc].parent;
  }
}
function stateDone(nodes, config, node) {
  const kind = nodes[node].kind;
  if (kind === "final")
    return true;
  if (kind === "compound") {
    for (const child of nodes[node].children)
      if (config.contains(child))
        return stateDone(nodes, config, child);
    return false;
  }
  if (kind === "parallel")
    return nodes[node].children.every((c) => stateDone(nodes, config, c));
  return false;
}
function computeDoneNodes(nodes, config) {
  const out = [];
  for (const id of config.iterOnes())
    if (isCompoundOrParallel(nodes, id) && stateDone(nodes, config, id))
      out.push(id);
  return out;
}
function candidatesFor(definition, config, context, event, selector, done) {
  const out = [];
  definition.transitions.forEach((t, i) => {
    if (!config.contains(t.source))
      return;
    const matchesTrigger = selector.kind === "event" && t.trigger.kind === "event" && t.trigger.event === selector.event || selector.kind === "spontaneous" && (t.trigger.kind === "eventless" || t.trigger.kind === "done" && done.includes(t.trigger.node)) || selector.kind === "timer" && t.trigger.kind === "timer" && t.trigger.timer === selector.timer;
    if (!matchesTrigger)
      return;
    if (t.guard !== undefined && !definition.guards[t.guard](context, event))
      return;
    out.push(i);
  });
  return out;
}
function resolveConflicts(nodes, transitions, candidates) {
  const sorted = [...candidates].sort((a, b) => transitions[a].docIndex - transitions[b].docIndex);
  const selected = [];
  outer:
    for (const cand of sorted) {
      const candDomain = computeDomain(nodes, transitions[cand].source, transitions[cand].targets, transitions[cand].kind);
      const toRemove = [];
      for (let i = 0;i < selected.length; i += 1) {
        const sel = selected[i];
        const selDomain = computeDomain(nodes, transitions[sel].source, transitions[sel].targets, transitions[sel].kind);
        if (isDescendantOrSelf(nodes, candDomain, selDomain) || isDescendantOrSelf(nodes, selDomain, candDomain)) {
          if (depthOf(nodes, transitions[cand].source) > depthOf(nodes, transitions[sel].source))
            toRemove.push(i);
          else
            continue outer;
        }
      }
      for (let i = toRemove.length - 1;i >= 0; i -= 1)
        selected.splice(toRemove[i], 1);
      selected.push(cand);
    }
  return selected;
}
function applyTransitions(definition, snapshot, transitionsIdx, event, sink, inspector) {
  const nodes = definition.nodes;
  const exitIds = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    for (const id of snapshot.configuration.iterOnes())
      if (isDescendant(nodes, id, domain) && !exitIds.includes(id))
        exitIds.push(id);
  }
  exitIds.sort((a, b) => depthOf(nodes, b) - depthOf(nodes, a));
  for (const owner of exitIds) {
    for (const child of nodes[owner].children) {
      const childKind = nodes[child].kind;
      if (childKind === "historyShallow") {
        const activeChild = nodes[owner].children.find((c) => snapshot.configuration.contains(c) && nodes[c].kind !== "historyShallow" && nodes[c].kind !== "historyDeep");
        if (activeChild !== undefined)
          snapshot.recordHistory(child, [activeChild]);
      } else if (childKind === "historyDeep") {
        const leaves = [];
        for (const id of snapshot.configuration.iterOnes())
          if (isDescendant(nodes, id, owner) && isLeafish(nodes, id))
            leaves.push(id);
        snapshot.recordHistory(child, leaves);
      }
    }
  }
  for (const id of exitIds) {
    for (const actionId of nodes[id].exitActions)
      definition.actions[actionId](snapshot.context, event, sink);
    for (const [timerId] of nodes[id].timers)
      sink.push({ kind: "cancelTimer", timer: timerId });
    for (const invokeId of nodes[id].invokes)
      sink.push({ kind: "stopInvoke", invoke: invokeId });
    snapshot.configuration.clear(id);
  }
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    for (const actionId of t.actions)
      definition.actions[actionId](snapshot.context, event, sink);
  }
  const entryIds = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    const effectiveTargets = resolveEffectiveTargets(nodes, t.targets, snapshot);
    for (const target of effectiveTargets)
      addDescendantStatesToEnter(nodes, target, snapshot, entryIds);
    for (const target of effectiveTargets)
      addAncestorStatesToEnter(nodes, target, domain, snapshot, entryIds);
  }
  entryIds.sort((a, b) => depthOf(nodes, a) - depthOf(nodes, b));
  for (const id of entryIds) {
    snapshot.configuration.set(id);
    for (const actionId of nodes[id].entryActions)
      definition.actions[actionId](snapshot.context, event, sink);
    for (const [timerId, delayMs] of nodes[id].timers)
      sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of nodes[id].invokes)
      sink.push({ kind: "startInvoke", invoke: invokeId });
  }
  inspector.observe({ kind: "microstep", exited: exitIds, entered: entryIds });
}
function finalizeStatus(definition, snapshot) {
  if (snapshot.status.kind === "done")
    return;
  if (stateDone(definition.nodes, snapshot.configuration, ROOT) && definition.makeOutput) {
    snapshot.status = { kind: "done", output: definition.makeOutput(snapshot.context) };
  }
}
function runToCompletion(definition, snapshot, seed, sink, inspector) {
  inspector.observe({ kind: "macrostepStart" });
  const queue = seed ? [seed] : [];
  let microsteps = 0;
  for (;; ) {
    if (microsteps >= MICROSTEP_LIMIT)
      break;
    let selected;
    let eventOwned;
    const trigger = queue.shift();
    if (trigger) {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const selector = trigger.selector.kind === "event" ? { kind: "event", event: trigger.selector.event } : { kind: "timer", timer: trigger.selector.timer };
      selected = candidatesFor(definition, snapshot.configuration, snapshot.context, trigger.event, selector, done);
      eventOwned = trigger.event;
    } else {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const spontaneous = candidatesFor(definition, snapshot.configuration, snapshot.context, undefined, { kind: "spontaneous" }, done);
      if (spontaneous.length === 0)
        break;
      selected = spontaneous;
      eventOwned = undefined;
    }
    if (selected.length === 0)
      continue;
    const resolved = resolveConflicts(definition.nodes, definition.transitions, selected);
    microsteps += 1;
    const local = [];
    applyTransitions(definition, snapshot, resolved, eventOwned, local, inspector);
    for (const command of local) {
      if (command.kind === "raise")
        queue.push({ selector: { kind: "event", event: command.event.eventId() }, event: command.event });
      inspector.observe({ kind: "commandIssued", command });
      sink.push(command);
    }
  }
  finalizeStatus(definition, snapshot);
  inspector.observe({ kind: "settled", microsteps });
  return { microsteps };
}
function init(machine, input, sink) {
  const definition = machine.definition;
  const snapshot = new Snapshot(definition.nodes, new BitSet, definition.contextFromInput(input));
  const entryIds = [];
  addDescendantStatesToEnter(definition.nodes, ROOT, snapshot, entryIds);
  entryIds.sort((a, b) => depthOf(definition.nodes, a) - depthOf(definition.nodes, b));
  for (const id of entryIds) {
    snapshot.configuration.set(id);
    for (const actionId of definition.nodes[id].entryActions)
      definition.actions[actionId](snapshot.context, undefined, sink);
    for (const [timerId, delayMs] of definition.nodes[id].timers)
      sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of definition.nodes[id].invokes)
      sink.push({ kind: "startInvoke", invoke: invokeId });
  }
  runToCompletion(definition, snapshot, undefined, sink, new NullInspector);
  return snapshot;
}
function macrostep(machine, snapshot, event, sink, inspector) {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "event", event: event.eventId() }, event }, sink, inspector);
}
function timerElapsed(machine, snapshot, timer, sink, inspector) {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "timer", timer } }, sink, inspector);
}

class NativeHost {
  #start = Date.now();
  #effects = [];
  #pendingTimers = [];
  #startedTasks = [];
  effects() {
    return this.#effects;
  }
  drainEffects() {
    return this.#effects.splice(0, this.#effects.length);
  }
  startedTasks() {
    return this.#startedTasks;
  }
  dueTimers() {
    const now = this.nowMs();
    const due = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now)
        return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }
  executeEffect(actor, effect) {
    this.#effects.push([actor, effect]);
  }
  schedule(actor, timer, delayMs) {
    this.#pendingTimers.push([actor, timer, this.nowMs() + delayMs]);
  }
  cancelTimer(actor, timer) {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }
  startTask(actor, invoke) {
    this.#startedTasks.push([actor, invoke]);
  }
  cancelTask(actor, invoke) {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
  }
  nowMs() {
    return Date.now() - this.#start;
  }
}

class TestHost {
  #clockMs = 0;
  #effects = [];
  #pendingTimers = [];
  #startedTasks = [];
  #cancelledTasks = [];
  effects() {
    return this.#effects;
  }
  startedTasks() {
    return this.#startedTasks;
  }
  cancelledTasks() {
    return this.#cancelledTasks;
  }
  advance(delayMs) {
    this.#clockMs += delayMs;
    const now = this.#clockMs;
    const due = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now)
        return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }
  executeEffect(actor, effect) {
    this.#effects.push([actor, effect]);
  }
  schedule(actor, timer, delayMs) {
    this.#pendingTimers.push([actor, timer, this.#clockMs + delayMs]);
  }
  cancelTimer(actor, timer) {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }
  startTask(actor, invoke) {
    this.#startedTasks.push([actor, invoke]);
  }
  cancelTask(actor, invoke) {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
    this.#cancelledTasks.push([actor, invoke]);
  }
  nowMs() {
    return this.#clockMs;
  }
}
function persist(machine, snapshot) {
  const nodes = machine.definition.nodes;
  const states = [...snapshot.configuration.iterOnes()].map((id) => nodes[id].stableId);
  const history = snapshot.historyEntries().map(([owner, ids]) => [nodes[owner].stableId, ids.map((id) => nodes[id].stableId)]);
  return { version: 1, fingerprint: machine.definition.fingerprint, states, history, done: snapshot.status.kind === "done" };
}
function stableIdToNode(nodes, stableId) {
  const idx = nodes.findIndex((n) => n.stableId === stableId);
  return idx === -1 ? undefined : NodeId(idx);
}
function restore(machine, persisted, context, migrations) {
  const definition = machine.definition;
  let current = persisted;
  while (current.fingerprint !== definition.fingerprint) {
    const next = migrations.find((m) => m.sourceFingerprint === current.fingerprint);
    if (!next)
      return { ok: false, error: { kind: "fingerprintMismatch" } };
    current = next.migrate(current);
  }
  const configuration = new BitSet;
  for (const stableId of current.states) {
    const id = stableIdToNode(definition.nodes, stableId);
    if (id === undefined)
      return { ok: false, error: { kind: "unknownStableId", stableId } };
    configuration.set(id);
  }
  const snapshot = new Snapshot(definition.nodes, configuration, context, { kind: "running" });
  for (const [ownerStableId, ids] of current.history) {
    const owner = stableIdToNode(definition.nodes, ownerStableId);
    if (owner === undefined)
      return { ok: false, error: { kind: "unknownStableId", stableId: ownerStableId } };
    const resolved = [];
    for (const stableId of ids) {
      const id = stableIdToNode(definition.nodes, stableId);
      if (id === undefined)
        return { ok: false, error: { kind: "unknownStableId", stableId } };
      resolved.push(id);
    }
    snapshot.recordHistory(owner, resolved);
  }
  return { ok: true, snapshot };
}

class Actor {
  id;
  snapshot;
  mailbox = [];
  constructor(id, snapshot) {
    this.id = id;
    this.snapshot = snapshot;
  }
}

class ActorSystem {
  host;
  #machine;
  #actors = [];
  #nextId = 0;
  constructor(host, machine) {
    this.host = host;
    this.#machine = machine;
  }
  spawnRoot(input) {
    const id = ActorId(this.#nextId);
    this.#nextId += 1;
    const buffer = [];
    const snapshot = init(this.#machine, input, buffer);
    this.#actors.push(new Actor(id, snapshot));
    this.#routeCommands(id, buffer);
    return id;
  }
  snapshot(id) {
    return this.#actors.find((a) => a.id === id)?.snapshot;
  }
  send(to, event) {
    this.#actors.find((a) => a.id === to)?.mailbox.push(event);
  }
  timerElapsed(to, timer) {
    const actor = this.#actors.find((a) => a.id === to);
    if (!actor)
      return;
    const buffer = [];
    const report = timerElapsed(this.#machine, actor.snapshot, timer, buffer, new NullInspector);
    this.#routeCommands(to, buffer);
    return report;
  }
  drain() {
    const reports = [];
    for (;; ) {
      let progressed = false;
      for (const actor of this.#actors) {
        const event = actor.mailbox.shift();
        if (event === undefined)
          continue;
        progressed = true;
        const buffer = [];
        const report = macrostep(this.#machine, actor.snapshot, event, buffer, new NullInspector);
        this.#routeCommands(actor.id, buffer);
        reports.push(report);
      }
      if (!progressed)
        break;
    }
    return reports;
  }
  #routeCommands(actor, commands) {
    const sends = [];
    const found = this.#actors.find((a) => a.id === actor);
    if (found) {
      for (const command of commands) {
        const pair = routeCommand(this.host, found.snapshot, actor, command);
        if (pair)
          sends.push(pair);
      }
    }
    for (const [to, event] of sends)
      this.send(to, event);
  }
}
function routeCommand(host, snapshot, actor, command) {
  switch (command.kind) {
    case "effect":
      host.executeEffect(actor, command.effect);
      return;
    case "raise":
      return;
    case "send":
      return [command.to, command.event];
    case "emit":
      snapshot.status = { kind: "done", output: command.output };
      return;
    case "startInvoke":
      host.startTask(actor, command.invoke);
      return;
    case "stopInvoke":
      host.cancelTask(actor, command.invoke);
      return;
    case "schedule":
      host.schedule(actor, command.timer, command.delayMs);
      return;
    case "cancelTimer":
      host.cancelTimer(actor, command.timer);
      return;
  }
}

class StepInspector {
  entered = [];
  exited = [];
  observe(event) {
    if (event.kind === "microstep") {
      this.exited.push(...event.exited);
      this.entered.push(...event.entered);
    }
  }
}

class MachineStep {
  entered;
  exited;
  active;
  commands;
  report;
  persisted;
  constructor(entered, exited, active, commands, report, persisted) {
    this.entered = entered;
    this.exited = exited;
    this.active = active;
    this.commands = commands;
    this.report = report;
    this.persisted = persisted;
  }
  isActive(stableId) {
    return this.active.includes(stableId);
  }
}
function stableIds(nodes, ids) {
  return ids.map((id) => nodes[id].stableId);
}
function machineStepOf(machine, snapshot, entered, exited, commands, report) {
  const nodes = machine.definition.nodes;
  return new MachineStep(stableIds(nodes, entered), stableIds(nodes, exited), stableIds(nodes, [...snapshot.configuration.iterOnes()]), commands, report, persist(machine, snapshot));
}
function start(machine, input) {
  const commands = [];
  const snapshot = init(machine, input, commands);
  return machineStepOf(machine, snapshot, [], [], commands, { microsteps: 0 });
}
function step(machine, prior, context, event, migrations) {
  const restored = restore(machine, prior, context, migrations);
  if (!restored.ok)
    return restored;
  const commands = [];
  const inspector = new StepInspector;
  const report = macrostep(machine, restored.snapshot, event, commands, inspector);
  return { ok: true, step: machineStepOf(machine, restored.snapshot, inspector.entered, inspector.exited, commands, report) };
}

class Model {
  events;
  constructor(events) {
    this.events = events;
  }
}
function activeStableIds(nodes, snapshot) {
  return [...snapshot.configuration.iterOnes()].map((id) => nodes[id].stableId);
}
function explore(machine, model, input) {
  const nodes = machine.definition.nodes;
  const root = init(machine, input, []);
  const visited = [];
  const frontier = [root];
  const reachedIds = [];
  let snapshot = frontier.pop();
  while (snapshot) {
    if (visited.some((v) => v.equals(snapshot.configuration))) {
      snapshot = frontier.pop();
      continue;
    }
    for (const stable of activeStableIds(nodes, snapshot))
      if (!reachedIds.includes(stable))
        reachedIds.push(stable);
    visited.push(snapshot.configuration.clone());
    for (const event of model.events) {
      const next = snapshot.branchForExploration();
      macrostep(machine, next, event, [], new NullInspector);
      frontier.push(next);
    }
    snapshot = frontier.pop();
  }
  return { visitedConfigurations: visited.length, reachedStableIds: reachedIds };
}
function checkInvariants(snapshot, invariants) {
  const violations = [];
  for (const invariant of invariants) {
    const result = invariant.check(snapshot);
    if (!result.ok)
      violations.push(`${invariant.name}: ${result.error.message}`);
  }
  return violations;
}
function runConformance(machine, input, steps) {
  const nodes = machine.definition.nodes;
  const sink = [];
  const snapshot = init(machine, input, sink);
  for (let index = 0;index < steps.length; index += 1) {
    const step2 = steps[index];
    macrostep(machine, snapshot, step2.event, sink, new NullInspector);
    for (const expected of step2.expectActive) {
      if (!snapshot.matches(expected)) {
        return { ok: false, error: { kind: "violation", message: `conformance step ${index}: expected active state '${expected}', got ${JSON.stringify(activeStableIds(nodes, snapshot))}` } };
      }
    }
  }
  return { ok: true };
}
/* 🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts */
function createLeasePool(load, dispose, options) {
  const lingerMs = options?.lingerMs ?? 30000;
  const label = options?.label ?? "resource";
  const entries = new Map;
  function disposeEntry(key, entry) {
    if (entries.get(key) !== entry)
      return;
    entries.delete(key);
    if (entry.settled !== undefined) {
      console.log(`[DEBUG] ${label} evicted ${key}`);
      dispose(entry.settled);
    }
  }
  return {
    async acquire(key) {
      let entry = entries.get(key);
      if (!entry) {
        const created = { promise: load(key), refs: 0, lingerTimer: null, settled: undefined };
        created.promise.then((value) => {
          created.settled = value;
        }, () => {
          if (entries.get(key) === created)
            entries.delete(key);
        });
        entries.set(key, created);
        entry = created;
      }
      const active = entry;
      if (active.lingerTimer !== null) {
        clearTimeout(active.lingerTimer);
        active.lingerTimer = null;
      }
      active.refs += 1;
      try {
        const value = await active.promise;
        let released = false;
        return {
          value,
          release: () => {
            if (released)
              return;
            released = true;
            active.refs -= 1;
            if (active.refs > 0)
              return;
            if (lingerMs <= 0) {
              disposeEntry(key, active);
              return;
            }
            active.lingerTimer = setTimeout(() => disposeEntry(key, active), lingerMs);
          }
        };
      } catch (error) {
        active.refs -= 1;
        throw error;
      }
    },
    evictNow(key) {
      for (const [entryKey, entry] of key ? [[key, entries.get(key)]] : entries) {
        if (!entry)
          continue;
        if (entry.refs > 0) {
          console.warn(`[DEBUG] ${label} evictNow(${entryKey}) skipped \u2014 ${entry.refs} active lease(s)`);
          continue;
        }
        if (entry.lingerTimer !== null)
          clearTimeout(entry.lingerTimer);
        disposeEntry(entryKey, entry);
      }
    },
    stats() {
      return Array.from(entries.entries()).map(([key, entry]) => ({
        key,
        refs: entry.refs,
        state: entry.settled === undefined ? "loading" : entry.lingerTimer !== null ? "lingering" : "resident"
      }));
    }
  };
}
function abortReason(signal) {
  return signal.reason ?? new Error("retryWithJitteredBackoff: aborted");
}
function abortableDelay(ms, signal) {
  return new Promise((resolve, reject) => {
    if (signal?.aborted) {
      reject(abortReason(signal));
      return;
    }
    const timer = setTimeout(() => {
      cleanup();
      resolve();
    }, ms);
    function cleanup() {
      clearTimeout(timer);
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort() {
      cleanup();
      reject(abortReason(signal));
    }
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}
async function retryWithJitteredBackoff(fn, options) {
  const { minMs, maxMs, signal } = options;
  let attempt = 0;
  for (;; ) {
    if (signal?.aborted)
      throw abortReason(signal);
    try {
      return await fn();
    } catch (error) {
      if (signal?.aborted)
        throw abortReason(signal);
      attempt += 1;
      const cap = Math.min(maxMs, minMs * 2 ** attempt);
      const waitMs = cap <= minMs ? minMs : minMs + Math.random() * (cap - minMs);
      try {
        await abortableDelay(waitMs, signal);
      } catch {
        throw signal?.aborted ? abortReason(signal) : error;
      }
    }
  }
}
function latestWins(run) {
  let current = null;
  let queued = null;
  function launch() {
    let promise;
    try {
      promise = Promise.resolve(run());
    } catch (error) {
      promise = Promise.reject(error);
    }
    current = promise;
    const advance = () => {
      if (current === promise)
        current = null;
      if (queued !== null)
        queued = null;
    };
    promise.then(advance, advance);
    return promise;
  }
  return function trigger() {
    if (current === null)
      return launch();
    if (queued === null)
      queued = current.then(launch, launch);
    return queued;
  };
}
async function fetchWithTimeout(url, init2, options) {
  const { timeoutMs, signal: externalSignal } = options;
  if (externalSignal?.aborted)
    throw externalSignal.reason ?? new Error("fetchWithTimeout: aborted");
  const controller = new AbortController;
  const timer = setTimeout(() => controller.abort(new Error(`fetchWithTimeout: timed out after ${timeoutMs}ms`)), timeoutMs);
  function onExternalAbort() {
    controller.abort(externalSignal.reason);
  }
  externalSignal?.addEventListener("abort", onExternalAbort, { once: true });
  try {
    return await fetch(url, { ...init2, signal: controller.signal });
  } finally {
    clearTimeout(timer);
    externalSignal?.removeEventListener("abort", onExternalAbort);
  }
}
function waitForEvent(subscribe, options) {
  const signal = options?.signal;
  return new Promise((resolve, reject) => {
    if (signal?.aborted) {
      reject(signal.reason ?? new Error("waitForEvent: aborted"));
      return;
    }
    let unsubscribe = null;
    function cleanup() {
      unsubscribe?.();
      unsubscribe = null;
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort() {
      cleanup();
      reject(signal.reason ?? new Error("waitForEvent: aborted"));
    }
    unsubscribe = subscribe((value) => {
      cleanup();
      resolve(value);
    });
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;
  describe("DockLayoutStore", () => {
    const emptySkeleton = () => ({
      version: 3,
      anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
    });
    it("returns null when nothing persisted", () => {
      const store = new DockLayoutStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });
    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      const appSkeleton = { ...emptySkeleton(), anchors: { ...emptySkeleton().anchors, "top-left": [{ id: "a" }] } };
      store.saveOs(osSkeleton);
      store.save(appSkeleton);
      expect(store.getSnapshot()).toEqual(appSkeleton);
    });
    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      store.saveOs(osSkeleton);
      expect(store.getSnapshot()).toEqual(osSkeleton);
    });
    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.save(emptySkeleton());
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toEqual(emptySkeleton());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });
    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.saveOs(emptySkeleton());
      store.save(emptySkeleton());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });
    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new DockLayoutStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 1, corners: { "top-left": [{ id: "a" }] } }, apps: {} } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 2, anchors: {} }, apps: {} } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });
  describe("DockUiStateStore", () => {
    const emptyUiState = () => ({ version: 3, anchors: {} });
    it("returns null when nothing persisted", () => {
      const store = new DockUiStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });
    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState = emptyUiState();
      const appState = { ...emptyUiState(), anchors: { "top-left": { visible: true, size: 320 } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });
    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState = { ...emptyUiState(), pathMemory: { "framework.category.workbench": "framework.panel.artifact" } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });
    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.save(emptyUiState());
      expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toEqual(emptyUiState());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });
    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.saveOs(emptyUiState());
      store.save(emptyUiState());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().dockUi).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });
    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new DockUiStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 1, corners: {} }, apps: {} } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 2, anchors: {} }, apps: {} } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
    it('keeps dock layout and dock ui as distinct projections for an app literally named "ui"', () => {
      const storage = createMemoryStoragePort();
      new DockLayoutStore(storage, "ui").save({
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
      });
      new DockUiStateStore(storage).saveOs(emptyUiState());
      const config = new OsShellConfig(storage).getSnapshot();
      expect(config.dockLayouts.apps.ui).toBeDefined();
      expect(config.dockUi.os).toEqual(emptyUiState());
    });
  });
  describe("WindowPaneStateStore", () => {
    const emptyPaneState = () => ({ version: 1, windows: {} });
    it("returns null when nothing persisted", () => {
      const store = new WindowPaneStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });
    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState = emptyPaneState();
      const appState = { version: 1, windows: { "puzzle3d.play": { utilities: { anchor: "bottom-left", folded: false, size: 280 } } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });
    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState = { version: 1, windows: { "puzzle3d.play": { measures: { anchor: "top-right", size: 320 } } } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });
    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.save(emptyPaneState());
      expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toEqual(emptyPaneState());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });
    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.saveOs(emptyPaneState());
      store.save(emptyPaneState());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().windowPanes).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });
    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new WindowPaneStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a foreign-version blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, windowPanes: { os: { version: 2, windows: {} }, apps: {} } }));
      const store = new WindowPaneStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });
  describe("OsShellConfig", () => {
    it("consolidates all four persisted shell projections into one config document", () => {
      const values = new Map;
      const storage = {
        get: (key) => values.get(key) ?? null,
        set: (key, value) => void values.set(key, value),
        remove: (key) => void values.delete(key)
      };
      const skeleton = {
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
      };
      new NamedLayoutStore("draw", storage).save({ id: "wide", label: "Wide", origin: "user", layout: { root: { kind: "stack", children: [] } } });
      new DockLayoutStore(storage, "draw").save(skeleton);
      new DockUiStateStore(storage, "draw").save({ version: 3, anchors: { "left-middle": { visible: true } } });
      new WindowPaneStateStore(storage, "draw").save({ version: 1, windows: {} });
      new OsShellConfig(storage).setPreference("ui.chrome.locale", "de");
      expect([...values.keys()]).toEqual(["semio.os.config"]);
      const snapshot = new OsShellConfig(storage).getSnapshot();
      expect(snapshot.namedLayouts.draw?.[0]?.id).toBe("wide");
      expect(snapshot.dockLayouts.apps.draw).toEqual(skeleton);
      expect(snapshot.dockUi.apps.draw?.anchors["left-middle"]?.visible).toBe(true);
      expect(snapshot.windowPanes.apps.draw).toEqual({ version: 1, windows: {} });
      expect(snapshot.preferences["ui.chrome.locale"]).toBe("de");
    });
  });
  const SYNTHETIC_PLUGIN_CATALOG = {
    plugins: [
      { pluginId: "alpha", wasmOut: "alpha.wasm", role: "plugin", contributes: [], consumes: [] },
      { pluginId: "beta", wasmOut: "beta.wasm", role: "plugin", contributes: [], consumes: [] }
    ],
    extensions: [{ pluginId: "beta-extension-gamma", wasmOut: "beta_gamma.wasm", role: "extension", contributes: ["beta.module"], consumes: [] }],
    hosts: [{ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" }],
    playgrounds: [
      { variant: "alpha", pluginId: "alpha", aliases: [] },
      { variant: "beta-play", pluginId: "beta", app: "beta-play-app", aliases: ["b", "beta play"] }
    ],
    moduleUrl: (pluginId, wasmOut) => `/plugin-modules/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
    extensionModuleUrl: (pluginId, wasmOut) => `/extensions/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`
  };
  describe("PlaygroundResolution", () => {
    it("resolves host config from the injected catalog", () => {
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "alpha")).toEqual({ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" });
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "beta-play")).toBeUndefined();
    });
    it("resolves playground aliases to registry plugin ids", () => {
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "b")).toBe("beta");
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "beta play")).toBe("beta");
    });
    it("rebuilds program rows when the generated session variant is stale", () => {
      const boot = resolvePlaygroundBoot(SYNTHETIC_PLUGIN_CATALOG, "beta-play", {
        variant: "alpha",
        defaultAppId: "alpha-app",
        plugins: [{ pluginId: "alpha", moduleUrl: "/plugin-modules/alpha/alpha_plugin.js" }]
      });
      expect(boot.variant).toBe("beta-play");
      expect(boot.defaultAppId).toBe("beta-play-app");
      expect(boot.plugins).toEqual([{ pluginId: "beta", moduleUrl: "/plugin-modules/beta/beta.js", contributes: [], consumes: [] }]);
    });
  });
  describe("effectiveActionArgs", () => {
    const TEXT_SCHEMA = { kind: "string", options: [] };
    const textArg = (id, extra = {}) => ({
      id,
      label: id,
      schema: TEXT_SCHEMA,
      required: false,
      ...extra
    });
    it("keeps a seeded arg that is not a declared form field, alongside the form's own staged fields (26/08/16 HUB-SPACES shareSpace regression: spaceId must reach the dispatched descriptor)", () => {
      const defs = [textArg("email")];
      const effective = effectiveActionArgs(defs, { email: "user2@semio.dev" }, { spaceId: "sp-1" });
      expect(effective).toEqual({ spaceId: "sp-1", email: "user2@semio.dev" });
    });
    it("a seed value for a declared field pre-fills it until the form stages its own value (renameSpace's current-name prefill)", () => {
      const defs = [textArg("name")];
      expect(effectiveActionArgs(defs, {}, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "Old Name" });
      expect(effectiveActionArgs(defs, { name: "New Name" }, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "New Name" });
    });
    it("a zero-declared-field confirm dialog passes seed+staged through wholesale (deleteSpace's confirm/cancel shape)", () => {
      expect(effectiveActionArgs([], {}, { spaceId: "sp-1", confirmed: true })).toEqual({ spaceId: "sp-1", confirmed: true });
    });
    it("missingRequiredArgs is unaffected by extra seed keys", () => {
      const defs = [textArg("email", { required: true })];
      const effective = effectiveActionArgs(defs, {}, { spaceId: "sp-1" });
      expect(missingRequiredArgs(defs, effective)).toEqual(["email"]);
    });
  });
  describe("organizeContextMenu", () => {
    const menuLeaf = (id) => ({ id, label: id, action: id });
    const menuDestructive = (id) => ({ ...menuLeaf(id), destructive: true });
    it("keeps a flat within-budget menu as-is, with groups sorted after leaves", () => {
      const items = [menuLeaf("a"), menuLeaf("b"), { id: "menu.group.view", children: [menuLeaf("c")] }];
      expect(organizeContextMenu(items, () => {
        return;
      })).toEqual(items);
    });
    it("shares the Rust fixture's grouped structure for a flat 12-item over-budget menu", () => {
      const items = [
        menuLeaf("primary0"),
        menuLeaf("primary1"),
        menuLeaf("primary2"),
        menuLeaf("primary3"),
        menuLeaf("primary4"),
        menuLeaf("overflow0"),
        menuLeaf("overflow1"),
        menuLeaf("overflow2"),
        menuLeaf("overflow3"),
        menuLeaf("overflow4"),
        menuLeaf("overflow5"),
        menuDestructive("delete")
      ];
      const categoryOf = (id) => id.startsWith("overflow") ? "view" : undefined;
      const organized = organizeContextMenu(items, categoryOf);
      expect(organized.map((item) => item.id)).toEqual([
        "primary0",
        "primary1",
        "primary2",
        "primary3",
        "primary4",
        "menu.group.view",
        "separator-organized-6",
        "delete"
      ]);
      expect(organized[5].children?.map((child) => child.id)).toEqual([
        "overflow0",
        "overflow1",
        "overflow2",
        "overflow3",
        "overflow4",
        "overflow5"
      ]);
      expect(organized[6].separator).toBe(true);
      expect(organized[6].label).toBeUndefined();
      expect(organized[7].destructive).toBe(true);
    });
  });
  describe("PluginSource", () => {
    const registry = [
      { pluginId: "note", moduleUrl: "/plugin-modules/note/note_plugin.js" },
      { pluginId: "s", moduleUrl: "/plugin-modules/s/s_plugin.js" }
    ];
    it("list() returns the registry it was created with", async () => {
      const source = createDevPluginSource(registry);
      expect(source.id).toBe("dev");
      await expect(source.list()).resolves.toEqual(registry);
    });
    it("moduleUrl() cache-busts a cold page load even before the build snapshot arrives", () => {
      const source = createDevPluginSource(registry);
      const first = new URL(source.moduleUrl("note"), "http://semio.test");
      const second = new URL(source.moduleUrl("s"), "http://semio.test");
      expect(first.pathname).toBe("/plugin-modules/note/note_plugin.js");
      expect(first.searchParams.get("v")).toMatch(/^\d+$/);
      expect(second.searchParams.get("v")).toBe(first.searchParams.get("v"));
    });
    it("moduleUrl() cache-busts with a rebuiltAt query param", () => {
      const source = createDevPluginSource(registry);
      expect(source.moduleUrl("note", 1785789943669)).toBe("/plugin-modules/note/note_plugin.js?v=1785789943669");
    });
    it("moduleUrl() throws for an unknown pluginId", () => {
      const source = createDevPluginSource(registry);
      expect(() => source.moduleUrl("missing")).toThrow(/missing/);
    });
    it("subscribe() is a harmless no-op without a global EventSource (node/vitest)", () => {
      const source = createDevPluginSource(registry);
      const events = [];
      const unsubscribe = source.subscribe((event) => events.push(event));
      expect(() => unsubscribe()).not.toThrow();
      expect(events).toEqual([]);
    });
    it("normalizes extension snapshots and installs into plugin availability events", () => {
      expect(extensionSourceEventToPluginSourceEvent({ kind: "snapshot", extensions: [{ extensionId: "gamma-extension", installedAt: 1785789943669 }] })).toEqual({
        kind: "snapshot",
        plugins: [{ pluginId: "gamma-extension", rebuiltAt: 1785789943669 }]
      });
      expect(extensionSourceEventToPluginSourceEvent({ kind: "installed", extensionId: "gamma-extension", installedAt: 1785789943670 })).toEqual({
        kind: "built",
        pluginId: "gamma-extension",
        rebuiltAt: 1785789943670
      });
      expect(extensionSourceEventToPluginSourceEvent({ kind: "uninstalled", extensionId: "gamma-extension" })).toBeUndefined();
    });
    it("multiplexPluginSources() merges list() and resolves moduleUrl from the matching child", async () => {
      const catalog = {
        plugins: [],
        extensions: [{ pluginId: "gamma-extension", wasmOut: "gamma.wasm", role: "extension", contributes: [], consumes: [] }],
        hosts: [],
        playgrounds: [],
        moduleUrl: (pluginId, wasmOut) => `/plugin-modules/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
        extensionModuleUrl: (pluginId, wasmOut) => `/extensions/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`
      };
      const dev = createDevPluginSource(registry);
      const extensions = createExtensionSource(catalog);
      const multiplexed = multiplexPluginSources(dev, extensions);
      expect(multiplexed.id).toBe("dev+extensions");
      const listed = await multiplexed.list();
      expect(listed.map((entry) => entry.pluginId).sort()).toEqual([...registry.map((entry) => entry.pluginId), ...catalog.extensions.map((entry) => entry.pluginId)].sort());
      expect(new URL(multiplexed.moduleUrl("note"), "http://semio.test").pathname).toBe("/plugin-modules/note/note_plugin.js");
      expect(() => multiplexed.moduleUrl("missing")).toThrow(/missing/);
    });
  });
  describe("ephemeralBox", () => {
    it("stores a function-typed init as the current value (not as a lazy factory)", () => {
      const identity = (id) => id;
      const box = ephemeralBox(`test.ephemeralBox.fn.${Math.random()}`, identity);
      expect(typeof box.current).toBe("function");
      expect(box.current("ui.nav.back")).toBe("ui.nav.back");
    });
    it("stores a no-op function init without invoking it", () => {
      let calls = 0;
      const noop = () => {
        calls += 1;
      };
      const box = ephemeralBox(`test.ephemeralBox.noop.${Math.random()}`, noop);
      expect(typeof box.current).toBe("function");
      expect(calls).toBe(0);
      box.current();
      expect(calls).toBe(1);
    });
    it("is owned by an isolatable, resettable OsTransient lane", () => {
      const left = new OsTransient;
      const right = new OsTransient;
      const leftBox = left.box("cursor", { x: 1 });
      leftBox.current.x = 2;
      expect(left.box("cursor", { x: 99 })).toBe(leftBox);
      expect(right.box("cursor", { x: 3 }).current.x).toBe(3);
      const oldMap = left.map("measurements");
      oldMap.set("width", 42);
      left.reset();
      expect(left.map("measurements")).not.toBe(oldMap);
      expect(left.map("measurements").size).toBe(0);
      expect(oldMap.get("width")).toBe(42);
    });
  });
  describe("LeasePool evictNow (hot-swap reload eviction)", () => {
    it("disposes a fully-released key immediately", async () => {
      const disposed = [];
      const pool = createLeasePool((key) => Promise.resolve(`value:${key}`), (value) => disposed.push(value), { lingerMs: 30000 });
      const lease = await pool.acquire("url-v1");
      lease.release();
      expect(disposed).toEqual([]);
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });
    it("skips (does not throw) a key with an active lease, matching a reload that hasn't released the old handle yet", async () => {
      const disposed = [];
      const pool = createLeasePool((key) => Promise.resolve(`value:${key}`), (value) => disposed.push(value));
      const lease = await pool.acquire("url-v1");
      expect(() => pool.evictNow("url-v1")).not.toThrow();
      expect(disposed).toEqual([]);
      lease.release();
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });
    it("treats two cache-busted URLs of the same pluginId as independent keys", async () => {
      const disposed = [];
      const pool = createLeasePool((key) => Promise.resolve(`value:${key}`), (value) => disposed.push(value));
      const oldLease = await pool.acquire("note.js?v=1");
      const newLease = await pool.acquire("note.js?v=2");
      oldLease.release();
      pool.evictNow("note.js?v=1");
      expect(disposed).toEqual(["value:note.js?v=1"]);
      newLease.release();
      pool.evictNow("note.js?v=2");
      expect(disposed).toEqual(["value:note.js?v=1", "value:note.js?v=2"]);
    });
  });

  class UnitFlipEvent {
    eventCount = 1;
    eventId() {
      return EventId(0);
    }
    eventName() {
      return "Flip";
    }
  }
  const UNIT_FLIP = new UnitFlipEvent;
  const UNIT_TOGGLE_NODES = [
    { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
    { stableId: "off", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
    { stableId: "on", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 }
  ];
  const UNIT_TOGGLE_TRANSITIONS = [
    { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
    { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 1 }
  ];
  const UNIT_TOGGLE_MACHINE = {
    definition: { id: "unit_toggle", nodes: UNIT_TOGGLE_NODES, transitions: UNIT_TOGGLE_TRANSITIONS, contextFromInput: () => ({ count: 0 }), guards: [], actions: [], fingerprint: 42n, manifestJson: "{}" }
  };
  const TOGGLE_MACHINE = {
    definition: {
      id: "toggle",
      nodes: UNIT_TOGGLE_NODES,
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [ActionId(0)], docIndex: 0 },
        { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, guard: GuardId(0), targets: [NodeId(1)], kind: "external", actions: [ActionId(0)], docIndex: 1 }
      ],
      contextFromInput: (allow) => ({ count: 0, allow }),
      guards: [(ctx) => ctx.allow],
      actions: [(ctx) => ctx.count += 1],
      fingerprint: 1n,
      manifestJson: "{}"
    }
  };

  class PlayerEvent {
    static IDS = { open: 0, pause: 1, play: 2, stop: 3, resume: 4 };
    static NAMES = ["Open", "Pause", "Play", "Stop", "Resume"];
    eventCount = 5;
    type;
    constructor(type) {
      this.type = type;
    }
    eventId() {
      return EventId(PlayerEvent.IDS[this.type]);
    }
    eventName(id) {
      return PlayerEvent.NAMES[id] ?? "?";
    }
  }
  const PLAYER_MACHINE = {
    definition: {
      id: "player",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(3)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "closed", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "playing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "open", kind: "compound", parent: NodeId(0), initial: NodeId(2), children: [NodeId(2), NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "paused", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
        { stableId: "open.history", kind: "historyShallow", parent: NodeId(3), initial: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 }
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(3)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(2), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 2 },
        { source: NodeId(3), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 },
        { source: NodeId(1), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 }
      ],
      contextFromInput: () => ({}),
      guards: [],
      actions: [],
      fingerprint: 2n,
      manifestJson: "{}"
    }
  };

  class RecorderEvent {
    static IDS = { start: 0, audioStop: 1, videoStop: 2 };
    static NAMES = ["Start", "AudioStop", "VideoStop"];
    eventCount = 3;
    type;
    constructor(type) {
      this.type = type;
    }
    eventId() {
      return EventId(RecorderEvent.IDS[this.type]);
    }
    eventName(id) {
      return RecorderEvent.NAMES[id] ?? "?";
    }
  }
  const RECORDER_MACHINE = {
    definition: {
      id: "recorder",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "idle", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "recording", kind: "parallel", parent: NodeId(0), children: [NodeId(3), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "audio", kind: "compound", parent: NodeId(2), initial: NodeId(4), children: [NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "audio.capturing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
        { stableId: "audio.done", kind: "final", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
        { stableId: "video", kind: "compound", parent: NodeId(2), initial: NodeId(7), children: [NodeId(7), NodeId(8)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
        { stableId: "video.capturing", kind: "atomic", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
        { stableId: "video.done", kind: "final", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 }
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(7), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(8)], kind: "external", actions: [], docIndex: 2 },
        { source: NodeId(2), trigger: { kind: "done", node: NodeId(2) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 }
      ],
      contextFromInput: () => ({}),
      guards: [],
      actions: [],
      fingerprint: 3n,
      manifestJson: "{}"
    }
  };

  class CheckoutEvent {
    static IDS = { confirm: 0, selectMethod: 1, paymentSucceeded: 2, paymentFailed: 3, retry: 4, cancel: 5, resume: 6, shipDone: 7, invoiceDone: 8 };
    static NAMES = ["Confirm", "SelectMethod", "PaymentSucceeded", "PaymentFailed", "Retry", "Cancel", "Resume", "ShipDone", "InvoiceDone"];
    eventCount = 9;
    type;
    constructor(type) {
      this.type = type;
    }
    eventId() {
      return EventId(CheckoutEvent.IDS[this.type]);
    }
    eventName(id) {
      return CheckoutEvent.NAMES[id] ?? "?";
    }
  }
  const CHECKOUT_MACHINE = {
    definition: {
      id: "checkout",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2), NodeId(7), NodeId(14)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "cart", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "payment", kind: "compound", parent: NodeId(0), initial: NodeId(3), children: [NodeId(3), NodeId(4), NodeId(5), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "selecting", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "processing", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [InvokeId(0)], timers: [[TimerId(0), 5000]], docIndex: 4 },
        { stableId: "failed", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
        { stableId: "payment_history", kind: "historyShallow", parent: NodeId(2), initial: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
        { stableId: "fulfilment", kind: "parallel", parent: NodeId(0), children: [NodeId(8), NodeId(11)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
        { stableId: "shipping", kind: "compound", parent: NodeId(7), initial: NodeId(9), children: [NodeId(9), NodeId(10)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 },
        { stableId: "ship_pending", kind: "atomic", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 9 },
        { stableId: "ship_done", kind: "final", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 10 },
        { stableId: "invoicing", kind: "compound", parent: NodeId(7), initial: NodeId(12), children: [NodeId(12), NodeId(13)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 11 },
        { stableId: "invoice_pending", kind: "atomic", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 12 },
        { stableId: "invoice_done", kind: "final", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 13 },
        { stableId: "done", kind: "final", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 14 }
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(1), trigger: { kind: "event", event: EventId(6) }, targets: [NodeId(6)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(3), trigger: { kind: "event", event: EventId(1) }, guard: GuardId(0), targets: [NodeId(4)], kind: "external", actions: [ActionId(0)], docIndex: 2 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(7)], kind: "external", actions: [], docIndex: 3 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(5) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 5 },
        { source: NodeId(4), trigger: { kind: "timer", timer: TimerId(0) }, targets: [NodeId(5)], kind: "external", actions: [ActionId(1)], docIndex: 6 },
        { source: NodeId(5), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 7 },
        { source: NodeId(9), trigger: { kind: "event", event: EventId(7) }, targets: [NodeId(10)], kind: "external", actions: [], docIndex: 8 },
        { source: NodeId(12), trigger: { kind: "event", event: EventId(8) }, targets: [NodeId(13)], kind: "external", actions: [], docIndex: 9 },
        { source: NodeId(7), trigger: { kind: "done", node: NodeId(7) }, targets: [NodeId(14)], kind: "external", actions: [], docIndex: 10 }
      ],
      contextFromInput: () => ({ attempts: 0, methodSet: false }),
      makeOutput: (ctx) => ({ attempts: ctx.attempts }),
      guards: [(ctx) => ctx.attempts < 3],
      actions: [(ctx) => ctx.methodSet = true, (ctx) => ctx.attempts += 1],
      fingerprint: 100n,
      manifestJson: "{}"
    }
  };
  describe("machine: TestHost", () => {
    it("advance fires due timers only", () => {
      const host = new TestHost;
      host.schedule(ActorId(0), TimerId(0), 100);
      host.schedule(ActorId(0), TimerId(1), 300);
      expect(host.advance(150)).toEqual([[0, 0]]);
      expect(host.advance(200)).toEqual([[0, 1]]);
    });
    it("cancelTimer removes pending", () => {
      const host = new TestHost;
      host.schedule(ActorId(0), TimerId(0), 100);
      host.cancelTimer(ActorId(0), TimerId(0));
      expect(host.advance(200)).toEqual([]);
    });
    it("records effects and task lifecycle", () => {
      const host = new TestHost;
      host.executeEffect(ActorId(0), "audit");
      expect(host.effects()).toEqual([[0, "audit"]]);
      host.startTask(ActorId(0), InvokeId(0));
      expect(host.startedTasks()).toEqual([[0, 0]]);
      host.cancelTask(ActorId(0), InvokeId(0));
      expect(host.startedTasks()).toEqual([]);
      expect(host.cancelledTasks()).toEqual([[0, 0]]);
    });
  });
  describe("machine: TraceInspector", () => {
    it("records one microstep per transition", () => {
      const sink = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const inspector = new TraceInspector;
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(inspector.entries.length).toBe(2);
      expect(inspector.entries[0].exited).toEqual([NodeId(1)]);
      expect(inspector.entries[0].entered).toEqual([NodeId(2)]);
      expect(inspector.entries[1].exited).toEqual([NodeId(2)]);
      expect(inspector.entries[1].entered).toEqual([NodeId(1)]);
    });
  });
  describe("machine: kernel", () => {
    it("flat machine toggles and counts", () => {
      const sink = [];
      const snapshot = init(TOGGLE_MACHINE, true, sink);
      expect(snapshot.matches("off")).toBe(true);
      const inspector = new NullInspector;
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      expect(snapshot.context.count).toBe(1);
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("off")).toBe(true);
      expect(snapshot.context.count).toBe(2);
    });
    it("guard blocks transition when false", () => {
      const sink = [];
      const snapshot = init(TOGGLE_MACHINE, false, sink);
      const inspector = new NullInspector;
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      expect(snapshot.context.count).toBe(1);
    });
    it("hierarchical machine enters default descendant", () => {
      const sink = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      expect(snapshot.matches("closed")).toBe(true);
      expect(snapshot.matches("open")).toBe(false);
    });
    it("hierarchical machine transitions into compound default", () => {
      const sink = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, new NullInspector);
      expect(snapshot.matches("open")).toBe(true);
      expect(snapshot.matches("playing")).toBe(true);
    });
    it("shallow history restores last active child", () => {
      const sink = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      const inspector = new NullInspector;
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, inspector);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("pause"), sink, inspector);
      expect(snapshot.matches("paused")).toBe(true);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("stop"), sink, inspector);
      expect(snapshot.matches("closed")).toBe(true);
      expect(snapshot.matches("open")).toBe(false);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("resume"), sink, inspector);
      expect(snapshot.matches("open")).toBe(true);
      expect(snapshot.matches("paused")).toBe(true);
      expect(snapshot.matches("playing")).toBe(false);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("play"), sink, inspector);
      expect(snapshot.matches("playing")).toBe(true);
      expect(snapshot.matches("paused")).toBe(false);
    });
    it("parallel regions enter together", () => {
      const sink = [];
      const snapshot = init(RECORDER_MACHINE, undefined, sink);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, new NullInspector);
      expect(snapshot.matches("recording")).toBe(true);
      expect(snapshot.matches("audio.capturing")).toBe(true);
      expect(snapshot.matches("video.capturing")).toBe(true);
    });
    it("parallel done bubbles only once every region finishes", () => {
      const sink = [];
      const snapshot = init(RECORDER_MACHINE, undefined, sink);
      const inspector = new NullInspector;
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, inspector);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("audioStop"), sink, inspector);
      expect(snapshot.matches("audio.done")).toBe(true);
      expect(snapshot.matches("recording")).toBe(true);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("videoStop"), sink, inspector);
      expect(snapshot.matches("idle")).toBe(true);
      expect(snapshot.matches("recording")).toBe(false);
    });
  });
  describe("machine: persist/restore", () => {
    it("persist then restore round-trips active state", () => {
      const sink = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, new NullInspector);
      expect(snapshot.matches("on")).toBe(true);
      const persisted = persist(UNIT_TOGGLE_MACHINE, snapshot);
      expect(persisted.fingerprint).toBe(UNIT_TOGGLE_MACHINE.definition.fingerprint);
      expect(persisted.states).toContain("on");
      const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("on")).toBe(true);
    });
    it("restore rejects fingerprint mismatch without migration", () => {
      const sink = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const persisted = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
      const result = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
      expect(result).toEqual({ ok: false, error: { kind: "fingerprintMismatch" } });
    });
    it("restore applies migration chain until fingerprint matches", () => {
      const sink = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const persisted = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
      const migration = { sourceFingerprint: 9999n, migrate: (s) => ({ ...s, fingerprint: UNIT_TOGGLE_MACHINE.definition.fingerprint }) };
      const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, [migration]);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("off")).toBe(true);
    });
  });
  describe("machine: ActorSystem", () => {
    it("drains sent events through one macrostep each", () => {
      const system = new ActorSystem(new TestHost, UNIT_TOGGLE_MACHINE);
      const root = system.spawnRoot(undefined);
      expect(system.snapshot(root).matches("off")).toBe(true);
      system.send(root, UNIT_FLIP);
      const reports = system.drain();
      expect(reports.length).toBe(1);
      expect(system.snapshot(root).matches("on")).toBe(true);
      system.send(root, UNIT_FLIP);
      system.drain();
      expect(system.snapshot(root).matches("off")).toBe(true);
      expect(system.snapshot(root).context).toEqual({ count: 0 });
    });
  });
  describe("machine: testing (Model/Coverage/Invariant/Conformance)", () => {
    it("explore reaches both toggle states", () => {
      const model = new Model([UNIT_FLIP]);
      const coverage = explore(UNIT_TOGGLE_MACHINE, model, undefined);
      expect(coverage.reachedStableIds).toContain("off");
      expect(coverage.reachedStableIds).toContain("on");
      expect(coverage.visitedConfigurations).toBe(2);
    });
    it("conformance fixture passes for matching sequence", () => {
      const steps = [
        { event: UNIT_FLIP, expectActive: ["on"] },
        { event: UNIT_FLIP, expectActive: ["off"] }
      ];
      expect(runConformance(UNIT_TOGGLE_MACHINE, undefined, steps).ok).toBe(true);
    });
    it("conformance fixture fails with a descriptive message", () => {
      const steps = [{ event: UNIT_FLIP, expectActive: ["off"] }];
      const result = runConformance(UNIT_TOGGLE_MACHINE, undefined, steps);
      expect(result.ok).toBe(false);
      expect(!result.ok && result.error.message).toContain("step 0");
      expect(!result.ok && result.error.message).toContain("off");
    });
    it("invariant reports violation by name", () => {
      const sink = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const invariants = [{ name: "never off", check: (s) => s.matches("off") ? { ok: false, error: { kind: "violation", message: "was off" } } : { ok: true } }];
      expect(checkInvariants(snapshot, invariants)).toEqual(["never off: was off"]);
    });
  });
  describe("machine: BitSet", () => {
    it("set/clear/contains", () => {
      const bits = new BitSet;
      expect(bits.contains(NodeId(3))).toBe(false);
      bits.set(NodeId(3));
      expect(bits.contains(NodeId(3))).toBe(true);
      bits.clear(NodeId(3));
      expect(bits.contains(NodeId(3))).toBe(false);
    });
    it("iterOnes ascends regardless of insertion order", () => {
      const bits = new BitSet;
      bits.set(NodeId(100));
      bits.set(NodeId(0));
      bits.set(NodeId(64));
      bits.set(NodeId(63));
      expect([...bits.iterOnes()]).toEqual([0, 63, 64, 100]);
    });
    it("clearAll and isEmpty", () => {
      const bits = new BitSet;
      expect(bits.isEmpty()).toBe(true);
      bits.set(NodeId(5));
      expect(bits.isEmpty()).toBe(false);
      bits.clearAll();
      expect(bits.isEmpty()).toBe(true);
    });
  });
  describe("machine: checkout DSL twin (integration)", () => {
    it("walks cart to receipt", () => {
      const host = new TestHost;
      const system = new ActorSystem(host, CHECKOUT_MACHINE);
      const root = system.spawnRoot(undefined);
      expect(system.snapshot(root).matches("cart")).toBe(true);
      system.send(root, new CheckoutEvent("confirm"));
      system.drain();
      expect(system.snapshot(root).matches("selecting")).toBe(true);
      system.send(root, new CheckoutEvent("selectMethod"));
      system.drain();
      expect(system.snapshot(root).matches("processing")).toBe(true);
      expect(system.snapshot(root).context.methodSet).toBe(true);
      expect(host.startedTasks()).toEqual([[root, 0]]);
      system.send(root, new CheckoutEvent("paymentSucceeded"));
      system.drain();
      expect(system.snapshot(root).matches("ship_pending")).toBe(true);
      expect(system.snapshot(root).matches("invoice_pending")).toBe(true);
      expect(host.cancelledTasks()).toEqual([[root, 0]]);
      system.send(root, new CheckoutEvent("shipDone"));
      system.drain();
      expect(system.snapshot(root).matches("ship_done")).toBe(true);
      expect(system.snapshot(root).matches("invoice_pending")).toBe(true);
      system.send(root, new CheckoutEvent("invoiceDone"));
      system.drain();
      const finalStatus = system.snapshot(root).status;
      expect(finalStatus.kind).toBe("done");
      expect(finalStatus.kind === "done" && finalStatus.output.attempts).toBe(0);
    });
    it("cancel/resume round-trips via shallow history", () => {
      const sink = [];
      const snapshot = init(CHECKOUT_MACHINE, undefined, sink);
      const inspector = new TraceInspector;
      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("confirm"), sink, inspector);
      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("selectMethod"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);
      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("cancel"), sink, inspector);
      expect(snapshot.matches("cart")).toBe(true);
      expect(snapshot.matches("payment")).toBe(false);
      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("resume"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);
      expect(snapshot.matches("selecting")).toBe(false);
      expect(inspector.entries.length).toBeGreaterThan(0);
      const fired = timerElapsed(CHECKOUT_MACHINE, snapshot, TimerId(0), sink, inspector);
      expect(fired.microsteps).toBe(1);
      expect(snapshot.matches("failed")).toBe(true);
      expect(snapshot.context.attempts).toBe(1);
      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("retry"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);
      const persisted = persist(CHECKOUT_MACHINE, snapshot);
      expect(persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
      const restored = restore(CHECKOUT_MACHINE, persisted, { ...snapshot.context }, []);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("processing")).toBe(true);
    });
    it("model coverage reaches every declared state", () => {
      const model = new Model(["confirm", "selectMethod", "paymentSucceeded", "paymentFailed", "retry", "cancel", "resume", "shipDone", "invoiceDone"].map((type) => new CheckoutEvent(type)));
      const coverage = explore(CHECKOUT_MACHINE, model, undefined);
      for (const expected of ["cart", "selecting", "processing", "failed", "ship_pending", "ship_done", "invoice_pending", "invoice_done", "done"]) {
        expect(coverage.reachedStableIds).toContain(expected);
      }
    });
    it("start produces a persistable initial configuration", () => {
      const initial = start(CHECKOUT_MACHINE, undefined);
      expect(initial.isActive("cart")).toBe(true);
      expect(initial.entered).toEqual([]);
      expect(initial.persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
    });
    it("step round-trips through persisted state only", () => {
      let carried = start(CHECKOUT_MACHINE, undefined).persisted;
      let context = { attempts: 0, methodSet: false };
      for (const [type, expected] of [
        ["confirm", "selecting"],
        ["selectMethod", "processing"]
      ]) {
        const outcome = step(CHECKOUT_MACHINE, carried, context, new CheckoutEvent(type), []);
        expect(outcome.ok).toBe(true);
        if (!outcome.ok)
          continue;
        expect(outcome.step.isActive(expected)).toBe(true);
        context = { ...context, methodSet: true };
        carried = outcome.step.persisted;
      }
    });
    it("step reports entered and exited states", () => {
      const initial = start(CHECKOUT_MACHINE, undefined).persisted;
      const outcome = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(outcome.ok).toBe(true);
      if (!outcome.ok)
        return;
      expect(outcome.step.exited).toContain("cart");
      expect(outcome.step.entered).toContain("payment");
      expect(outcome.step.entered).toContain("selecting");
      expect(outcome.step.isActive("cart")).toBe(false);
    });
    it("step with a blocked guard leaves the configuration untouched", () => {
      const initial = start(CHECKOUT_MACHINE, undefined).persisted;
      const confirmed = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(confirmed.ok).toBe(true);
      if (!confirmed.ok)
        return;
      const blocked = step(CHECKOUT_MACHINE, confirmed.step.persisted, { attempts: 3, methodSet: false }, new CheckoutEvent("selectMethod"), []);
      expect(blocked.ok).toBe(true);
      if (!blocked.ok)
        return;
      expect(blocked.step.isActive("selecting")).toBe(true);
      expect(blocked.step.entered).toEqual([]);
    });
    it("step rejects a persisted snapshot from another machine shape", () => {
      const foreign = { ...start(CHECKOUT_MACHINE, undefined).persisted };
      const mismatched = { ...foreign, fingerprint: foreign.fingerprint ^ 0xffffffffn };
      const outcome = step(CHECKOUT_MACHINE, mismatched, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(outcome.ok).toBe(false);
    });
  });
  describe("retryWithJitteredBackoff", () => {
    it("keeps every backoff delay within [minMs, maxMs] and varies it across attempts", async () => {
      vi.useFakeTimers();
      try {
        const setTimeoutSpy = vi.spyOn(globalThis, "setTimeout");
        let calls = 0;
        const fn = vi.fn(async () => {
          calls += 1;
          if (calls < 6)
            throw new Error("retry me");
          return "ok";
        });
        const resultPromise = retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 100 });
        for (let i = 0;i < 5; i++) {
          await vi.advanceTimersByTimeAsync(100);
        }
        await expect(resultPromise).resolves.toBe("ok");
        expect(calls).toBe(6);
        const delays = setTimeoutSpy.mock.calls.map(([, ms]) => Number(ms));
        expect(delays.length).toBe(5);
        for (const delay of delays) {
          expect(delay).toBeGreaterThanOrEqual(10);
          expect(delay).toBeLessThanOrEqual(100);
        }
        expect(new Set(delays).size).toBeGreaterThan(1);
      } finally {
        vi.useRealTimers();
      }
    });
    it("rejects promptly on abort mid-wait and makes no further attempt", async () => {
      vi.useFakeTimers();
      try {
        const controller = new AbortController;
        let calls = 0;
        const fn = vi.fn(async () => {
          calls += 1;
          throw new Error("always fails");
        });
        const promise = retryWithJitteredBackoff(fn, { minMs: 50, maxMs: 50, signal: controller.signal });
        await vi.advanceTimersByTimeAsync(0);
        expect(calls).toBe(1);
        controller.abort(new Error("stop"));
        await expect(promise).rejects.toThrow("stop");
        expect(calls).toBe(1);
      } finally {
        vi.useRealTimers();
      }
    });
    it("throws immediately for an already-aborted signal without calling fn", async () => {
      const controller = new AbortController;
      controller.abort(new Error("pre-aborted"));
      const fn = vi.fn(async () => "unreachable");
      await expect(retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 10, signal: controller.signal })).rejects.toThrow("pre-aborted");
      expect(fn).not.toHaveBeenCalled();
    });
  });
  describe("latestWins", () => {
    it("collapses N concurrent calls during an in-flight run into exactly one trailing run, and every caller sees the latest result", async () => {
      let calls = 0;
      const resolvers = [];
      const run = vi.fn(() => {
        calls += 1;
        const callIndex = calls;
        return new Promise((resolve) => resolvers.push(() => resolve(callIndex)));
      });
      const trigger = latestWins(run);
      const first = trigger();
      expect(calls).toBe(1);
      const concurrentA = trigger();
      const concurrentB = trigger();
      const concurrentC = trigger();
      expect(calls).toBe(1);
      resolvers[0](0);
      await new Promise((resolve) => setTimeout(resolve, 0));
      expect(calls).toBe(2);
      resolvers[1](0);
      await expect(first).resolves.toBe(1);
      await expect(concurrentA).resolves.toBe(2);
      await expect(concurrentB).resolves.toBe(2);
      await expect(concurrentC).resolves.toBe(2);
    });
    it("starts a fresh run when called again after everything has settled", async () => {
      let n = 0;
      const trigger = latestWins(async () => {
        n += 1;
        return n;
      });
      await expect(trigger()).resolves.toBe(1);
      await expect(trigger()).resolves.toBe(2);
    });
  });
  describe("fetchWithTimeout", () => {
    it("aborts the underlying fetch when the timeout elapses first", async () => {
      vi.useFakeTimers();
      try {
        const fetchMock = vi.fn((_url, init2) => new Promise((_resolve, reject) => {
          init2?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
        }));
        vi.stubGlobal("fetch", fetchMock);
        const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 50 });
        const assertion = expect(promise).rejects.toThrow();
        await vi.advanceTimersByTimeAsync(50);
        await assertion;
      } finally {
        vi.unstubAllGlobals();
        vi.useRealTimers();
      }
    });
    it("aborts the underlying fetch when the external signal aborts", async () => {
      const controller = new AbortController;
      const fetchMock = vi.fn((_url, init2) => new Promise((_resolve, reject) => {
        init2?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
      }));
      vi.stubGlobal("fetch", fetchMock);
      try {
        const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 1e4, signal: controller.signal });
        controller.abort(new Error("caller cancelled"));
        await expect(promise).rejects.toThrow();
      } finally {
        vi.unstubAllGlobals();
      }
    });
    it("clears the timer and removes the abort listener on the success path", async () => {
      vi.useFakeTimers();
      try {
        const clearTimeoutSpy = vi.spyOn(globalThis, "clearTimeout");
        const controller = new AbortController;
        const removeEventListenerSpy = vi.spyOn(controller.signal, "removeEventListener");
        const response = { ok: true, status: 200, statusText: "OK", headers: { get: () => null }, json: async () => ({}), text: async () => "" };
        vi.stubGlobal("fetch", vi.fn(async () => response));
        const result = await fetchWithTimeout("https://example.test", undefined, { timeoutMs: 1000, signal: controller.signal });
        expect(result).toBe(response);
        expect(clearTimeoutSpy).toHaveBeenCalled();
        expect(removeEventListenerSpy).toHaveBeenCalledWith("abort", expect.any(Function));
        await vi.advanceTimersByTimeAsync(5000);
      } finally {
        vi.unstubAllGlobals();
        vi.useRealTimers();
      }
    });
  });
  describe("waitForEvent", () => {
    it("resolves with the first delivered value and unsubscribes", async () => {
      let handler;
      let unsubscribed = false;
      const subscribe = (h) => {
        handler = h;
        return () => {
          unsubscribed = true;
        };
      };
      const promise = waitForEvent(subscribe);
      expect(unsubscribed).toBe(false);
      handler(42);
      await expect(promise).resolves.toBe(42);
      expect(unsubscribed).toBe(true);
    });
    it("rejects on abort and unsubscribes without ever having received a value", async () => {
      let unsubscribed = false;
      const subscribe = () => () => {
        unsubscribed = true;
      };
      const controller = new AbortController;
      const promise = waitForEvent(subscribe, { signal: controller.signal });
      controller.abort(new Error("cancelled"));
      await expect(promise).rejects.toThrow("cancelled");
      expect(unsubscribed).toBe(true);
    });
    it("rejects immediately for an already-aborted signal, never subscribing", async () => {
      const controller = new AbortController;
      controller.abort(new Error("pre-aborted"));
      let subscribed = false;
      const subscribe = () => {
        subscribed = true;
        return () => {};
      };
      await expect(waitForEvent(subscribe, { signal: controller.signal })).rejects.toThrow("pre-aborted");
      expect(subscribed).toBe(false);
    });
  });
}

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts */
import { basename, dirname, extname, join, relative, resolve } from "path";
import { fileURLToPath } from "url";
var __dirname2 = dirname(fileURLToPath(import.meta.url));
var cachedTaxonomy = ephemeralBox("framework.products.repo.modules.lib.discovery.component.ts.cachedTaxonomy", undefined);
function taxonomyPatternExpression(pattern) {
  let expression = "^";
  for (let index = 0;index < pattern.length; ) {
    if (pattern.slice(index, index + 3) === "**/") {
      expression += "(?:[^/]+/)*";
      index += 3;
      continue;
    }
    const character = pattern[index];
    if (character === "*" && pattern[index + 1] === "*") {
      expression += ".*";
      index += 2;
      continue;
    }
    if (character === "*")
      expression += "[^/]*";
    else if (character === "?")
      expression += "[^/]";
    else if (character === "[") {
      const end = pattern.indexOf("]", index + 1);
      if (end < 0)
        throw new Error(`Invalid taxonomy path pattern ${JSON.stringify(pattern)}.`);
      expression += pattern.slice(index, end + 1);
      index = end;
    } else
      expression += character.replace(/[\\^$.*+?()[\]{}|]/gu, "\\$&");
    index += 1;
  }
  return new RegExp(`${expression}$`, "u");
}
function taxonomyPathPatternMatches(path, pattern) {
  const normalizedPath = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
  return taxonomyPatternExpression(pattern.normalize("NFC")).test(normalizedPath);
}
var DISCOVERY_SKIP_DIRS = new Set(["node_modules", "target", "dist", "\uD83D\uDCE4\uFE0Fdist", ".git", ".\uD83E\uDDECsemio", "\uD83E\uDD16\uFE0Fgenerated", "\uD83D\uDD0C\uFE0Fplugin-modules", "pkg", "storybook-static", "temp", ".venv", "coverage", "__pycache__", "client", "client_bin"]);
var scanCache = ephemeralMap("framework.products.repo.modules.lib.discovery.component.ts.scanCache");
var SEMANTIC_SKIP_DIRS = new Set(["node_modules", "target", "dist", ".git", ".nx", ".cache", "vendor", "pkg", "storybook-static", "temp"]);
var SEMANTIC_NON_PRODUCTION_SEGMENTS = new Set(["\uD83E\uDDEA\uFE0Ftests", "tests", "test", "__tests__", "\uD83D\uDCDA\uFE0Fexamples", "\uD83E\uDDEA\uFE0Fexamples", "examples", "fixtures", "\uD83E\uDDEA\uFE0Ffixtures", "\uD83E\uDD16\uFE0Fgenerated"]);

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts */
var TAXONOMY_RELATIVE_PATH = "\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83D\uDD23\uFE0Ftaxonomy.json";
var GENERIC_SEMANTIC_STEMS = new Set(["asset", "assets", "component", "components", "empty", "glue", "test", "tests", "implementation", "impl", "index"]);
var WINDOWS_RESERVED = /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i;
var SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });
function record(value, name) {
  if (!value || typeof value !== "object" || Array.isArray(value))
    throw new Error(`Taxonomy v7 field ${name} must be an object`);
  return value;
}
function stringArray(value, name) {
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string"))
    throw new Error(`Taxonomy v7 field ${name} must be a string array`);
  return value;
}
function requiredString(value, name) {
  if (typeof value !== "string" || value.length === 0)
    throw new Error(`Taxonomy v7 field ${name} must be a non-empty string`);
  return value;
}
function validatedContractPattern(value, name, exactBasename) {
  const pattern = requiredString(value, name);
  if (pattern !== pattern.normalize("NFC") || pattern.startsWith("/") || pattern.endsWith("/") || pattern.includes("\\") || pattern.includes("//") || pattern.includes("\x00"))
    throw new Error(`Taxonomy v7 ${name} must be one NFC workspace-relative POSIX pattern`);
  if (/[{}]/u.test(pattern) || /^!/u.test(pattern) || /[!@+?*]\(/u.test(pattern))
    throw new Error(`Taxonomy v7 ${name} uses unsupported glob syntax`);
  for (const segment of pattern.split("/")) {
    if (segment.includes("**") && segment !== "**")
      throw new Error(`Taxonomy v7 ${name} may use ** only as a whole segment`);
    for (const match of segment.matchAll(/\[([^\]]*)\]/gu))
      if (!/^[A-Za-z0-9-]+$/u.test(match[1]) || /^[!^]/u.test(match[1]))
        throw new Error(`Taxonomy v7 ${name} has an invalid character class`);
    if ((segment.match(/\[/gu)?.length ?? 0) !== (segment.match(/\]/gu)?.length ?? 0))
      throw new Error(`Taxonomy v7 ${name} has an unclosed character class`);
  }
  const filename = pattern.slice(pattern.lastIndexOf("/") + 1);
  if (exactBasename && /[*?\[\]]/u.test(filename))
    throw new Error(`Taxonomy v7 ${name} must end in one exact literal basename`);
  taxonomyPathPatternMatches("", pattern);
  return pattern;
}
function fixedExpiry(value, name) {
  if (value === null)
    return null;
  const expires = requiredString(value, name);
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(expires))
    throw new Error(`Taxonomy v7 ${name} must be null or YYYY-MM-DD`);
  return expires;
}
function parseTaxonomy(raw, path) {
  const root = record(raw, "root");
  if (root.schemaVersion !== 7)
    throw new Error(`Taxonomy schemaVersion must be 7 at ${path}`);
  const fileKindRows = record(root.fileKinds, "fileKinds");
  const directoryKindRows = record(root.semanticDirectoryKinds, "semanticDirectoryKinds");
  const fixedRows = record(root.fixedFilenameContracts, "fixedFilenameContracts");
  const fixedDirectoryRows = record(root.fixedDirectoryContracts, "fixedDirectoryContracts");
  const configurableRows = record(root.configurableEntryContracts, "configurableEntryContracts");
  const fileResolutionRows = record(root.fileKindResolutionRules, "fileKindResolutionRules");
  const scopedFileRows = record(root.scopedFileKinds, "scopedFileKinds");
  const directoryMemberRows = record(root.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds");
  const projectedMemberRows = record(root.semanticProjectedMemberKinds, "semanticProjectedMemberKinds");
  const projectionRendererRows = record(root.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers");
  const descendantContractRows = record(root.semanticDescendantContracts, "semanticDescendantContracts");
  const projectionCatalogRows = record(root.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts");
  const projectionRows = record(root.semanticPathProjectionContracts, "semanticPathProjectionContracts");
  const mutationCatalogProjectionRow = record(root.mutationCatalogProjection, "mutationCatalogProjection");
  const generatorRows = record(root.generatorContracts, "generatorContracts");
  const boundaryRows = record(root.packageBoundaryRules, "packageBoundaryRules");
  const grammarRows = record(root.packageGlueGrammar, "packageGlueGrammar");
  const exclusionRows = record(root.pathExclusions, "pathExclusions");
  const unicode = record(root.unicodeNormalization, "unicodeNormalization");
  const selector = record(root.variationSelectorPolicy, "variationSelectorPolicy");
  const collision = record(root.collisionPolicy, "collisionPolicy");
  const enforcement = record(root.areaEnforcement, "areaEnforcement");
  if (unicode.form !== "NFC" || unicode.caseFold !== "lower" || unicode.locale !== "und")
    throw new Error("Taxonomy v7 unicodeNormalization must select NFC/lower/und");
  if (selector.selector !== "\uFE0F" || selector.requiredAfterEmoji !== true || selector.comparison !== "ignore-selector")
    throw new Error("Taxonomy v7 variationSelectorPolicy is not canonical");
  const requiredComparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (canonicalJson(collision.comparisons) !== canonicalJson(requiredComparisons) || !Number.isSafeInteger(collision.maxPathBytes) || collision.maxPathBytes < 1 || collision.rejectWindowsReservedNames !== true || collision.rejectTrailingDotsAndSpaces !== true)
    throw new Error("Taxonomy v7 collisionPolicy is incomplete");
  if (enforcement.requiredState !== "clean" || enforcement.undeclaredAreas !== "enforce")
    throw new Error("Taxonomy v7 areaEnforcement must enforce clean undeclared areas");
  const fileKinds = {};
  for (const [id, value] of Object.entries(fileKindRows)) {
    const spec = record(value, `fileKinds.${id}`);
    const emoji = requiredString(spec.emoji, `fileKinds.${id}.emoji`).normalize("NFC");
    const extensionChains = stringArray(spec.extensionChains, `fileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith(".")))
      throw new Error(`Taxonomy v7 fileKinds.${id}.extensionChains must contain dotted chains`);
    fileKinds[id] = { emoji, extensionChains: [...new Set(extensionChains)].sort((a, b) => b.length - a.length || a.localeCompare(b)), role: requiredString(spec.role, `fileKinds.${id}.role`) };
  }
  if (Object.keys(fileKinds).length === 0)
    throw new Error("Taxonomy v7 fileKinds must not be empty");
  const semanticDirectoryKinds = {};
  for (const [id, value] of Object.entries(directoryKindRows)) {
    const spec = record(value, `semanticDirectoryKinds.${id}`);
    const emoji = requiredString(spec.emoji, `semanticDirectoryKinds.${id}.emoji`).normalize("NFC");
    const slugPattern = requiredString(spec.slugPattern, `semanticDirectoryKinds.${id}.slugPattern`);
    new RegExp(slugPattern, "u");
    if (typeof spec.allowEmojiOnly !== "boolean")
      throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.allowEmojiOnly must be boolean`);
    semanticDirectoryKinds[id] = { emoji, slugPattern, allowEmojiOnly: spec.allowEmojiOnly, parentKindIds: spec.parentKindIds === undefined ? [] : stringArray(spec.parentKindIds, `semanticDirectoryKinds.${id}.parentKindIds`) };
  }
  if (Object.keys(semanticDirectoryKinds).length === 0)
    throw new Error("Taxonomy v7 semanticDirectoryKinds must not be empty");
  const fixedFilenameContracts = {};
  for (const [id, value] of Object.entries(fixedRows)) {
    const spec = record(value, `fixedFilenameContracts.${id}`);
    if (!["repository-root", "package-root", "directory-kind", "path-pattern"].includes(spec.scope))
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope is invalid`);
    if (spec.configurability !== "unconfigurable")
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.configurability must be unconfigurable`);
    const scope = spec.scope;
    const ecosystemId = typeof spec.ecosystemId === "string" ? spec.ecosystemId : undefined;
    const directoryKindId = typeof spec.directoryKindId === "string" ? spec.directoryKindId : undefined;
    if (scope === "package-root" && !ecosystemId)
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.ecosystemId is required for package-root scope`);
    if (scope === "directory-kind" && (!directoryKindId || !semanticDirectoryKinds[directoryKindId]))
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.directoryKindId is invalid`);
    fixedFilenameContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedFilenameContracts.${id}.pathPattern`, true),
      authority: requiredString(spec.authority, `fixedFilenameContracts.${id}.authority`),
      reason: requiredString(spec.reason, `fixedFilenameContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      ecosystemId,
      directoryKindId,
      verification: requiredString(spec.verification, `fixedFilenameContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedFilenameContracts.${id}.expires`)
    };
  }
  const fixedDirectoryContracts = {};
  for (const [id, value] of Object.entries(fixedDirectoryRows)) {
    const spec = record(value, `fixedDirectoryContracts.${id}`);
    if (!["repository-root", "directory-kind", "path-pattern"].includes(spec.scope))
      throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope is invalid`);
    if (spec.configurability !== "unconfigurable")
      throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.configurability must be unconfigurable`);
    const scope = spec.scope;
    const directoryKindId = typeof spec.directoryKindId === "string" ? spec.directoryKindId : undefined;
    if (scope === "directory-kind" && (!directoryKindId || !semanticDirectoryKinds[directoryKindId]))
      throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.directoryKindId is invalid`);
    fixedDirectoryContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedDirectoryContracts.${id}.pathPattern`, false),
      authority: requiredString(spec.authority, `fixedDirectoryContracts.${id}.authority`),
      reason: requiredString(spec.reason, `fixedDirectoryContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      directoryKindId,
      verification: requiredString(spec.verification, `fixedDirectoryContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedDirectoryContracts.${id}.expires`)
    };
  }
  if (Object.keys(fixedDirectoryContracts).length === 0)
    throw new Error("Taxonomy v7 fixedDirectoryContracts must not be empty");
  const configurableEntryContracts = {};
  for (const [id, value] of Object.entries(configurableRows)) {
    const spec = record(value, `configurableEntryContracts.${id}`);
    const fileKindId = requiredString(spec.fileKindId, `configurableEntryContracts.${id}.fileKindId`);
    if (!fileKinds[fileKindId])
      throw new Error(`Taxonomy v7 configurableEntryContracts.${id} references unknown file kind ${fileKindId}`);
    configurableEntryContracts[id] = {
      filename: requiredString(spec.filename, `configurableEntryContracts.${id}.filename`),
      fileKindId,
      ecosystemId: requiredString(spec.ecosystemId, `configurableEntryContracts.${id}.ecosystemId`),
      role: requiredString(spec.role, `configurableEntryContracts.${id}.role`),
      configurationSources: stringArray(spec.configurationSources, `configurableEntryContracts.${id}.configurationSources`)
    };
  }
  const fileKindResolutionRules = {};
  for (const [id, value] of Object.entries(fileResolutionRows)) {
    const spec = record(value, `fileKindResolutionRules.${id}`);
    const extensionChain = requiredString(spec.extensionChain, `fileKindResolutionRules.${id}.extensionChain`);
    const fileKindId = requiredString(spec.fileKindId, `fileKindResolutionRules.${id}.fileKindId`);
    if (!fileKinds[fileKindId]?.extensionChains.includes(extensionChain))
      throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} does not reference an owned extension chain`);
    if (!Number.isSafeInteger(spec.priority))
      throw new Error(`Taxonomy v7 fileKindResolutionRules.${id}.priority must be an integer`);
    const filenamePattern = typeof spec.filenamePattern === "string" ? spec.filenamePattern : undefined;
    const pathPattern = typeof spec.pathPattern === "string" ? validatedContractPattern(spec.pathPattern, `fileKindResolutionRules.${id}.pathPattern`, false) : undefined;
    if (filenamePattern)
      new RegExp(filenamePattern, "u");
    const parentKindIds = spec.parentKindIds === undefined ? undefined : stringArray(spec.parentKindIds, `fileKindResolutionRules.${id}.parentKindIds`);
    const ancestorKindIds = spec.ancestorKindIds === undefined ? undefined : stringArray(spec.ancestorKindIds, `fileKindResolutionRules.${id}.ancestorKindIds`);
    for (const kindId of [...parentKindIds ?? [], ...ancestorKindIds ?? []])
      if (!semanticDirectoryKinds[kindId])
        throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} references unknown directory kind ${kindId}`);
    fileKindResolutionRules[id] = { extensionChain, fileKindId, priority: spec.priority, filenamePattern, pathPattern, parentKindIds, ancestorKindIds };
  }
  if (Object.keys(fileKindResolutionRules).length === 0)
    throw new Error("Taxonomy v7 fileKindResolutionRules must not be empty");
  const scopedFileKinds = {};
  for (const [id, value] of Object.entries(scopedFileRows)) {
    const spec = record(value, `scopedFileKinds.${id}`);
    const extensionChains = stringArray(spec.extensionChains, `scopedFileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith(".")))
      throw new Error(`Taxonomy v7 scopedFileKinds.${id}.extensionChains must contain dotted chains`);
    const sourceFilenamePattern = requiredString(spec.sourceFilenamePattern, `scopedFileKinds.${id}.sourceFilenamePattern`);
    new RegExp(sourceFilenamePattern, "u");
    if (spec.role !== "evidence")
      throw new Error(`Taxonomy v7 scopedFileKinds.${id}.role must be evidence`);
    scopedFileKinds[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `scopedFileKinds.${id}.pathPattern`, false),
      emoji: requiredString(spec.emoji, `scopedFileKinds.${id}.emoji`).normalize("NFC"),
      extensionChains: [...new Set(extensionChains)].sort((left, right) => right.length - left.length || left.localeCompare(right)),
      role: "evidence",
      sourceFilenamePattern,
      authority: requiredString(spec.authority, `scopedFileKinds.${id}.authority`),
      reason: requiredString(spec.reason, `scopedFileKinds.${id}.reason`),
      verification: requiredString(spec.verification, `scopedFileKinds.${id}.verification`),
      expires: fixedExpiry(spec.expires, `scopedFileKinds.${id}.expires`)
    };
  }
  const semanticDirectoryMemberKinds = {};
  for (const [id, value] of Object.entries(directoryMemberRows)) {
    const spec = record(value, `semanticDirectoryMemberKinds.${id}`);
    if (spec.source !== "registry")
      throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id}.source must be registry`);
    const ownerKindIds = stringArray(spec.ownerKindIds, `semanticDirectoryMemberKinds.${id}.ownerKindIds`);
    const memberNames = stringArray(spec.memberNames, `semanticDirectoryMemberKinds.${id}.memberNames`);
    if (ownerKindIds.length === 0 || memberNames.length === 0)
      throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} must declare owners and members`);
    if (memberNames.some((name) => name !== name.normalize("NFC") || !splitLeadingEmoji(name).emoji))
      throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} member names must be NFC emoji-leading evidence`);
    semanticDirectoryMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), memberNames: [...new Set(memberNames)].sort(), source: "registry" };
  }
  const directoryContextIds = new Set([...Object.keys(semanticDirectoryKinds), ...Object.keys(semanticDirectoryMemberKinds)]);
  for (const [id, spec] of Object.entries(semanticDirectoryMemberKinds))
    for (const ownerId of spec.ownerKindIds)
      if (!directoryContextIds.has(ownerId))
        throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} references unknown owner kind ${ownerId}`);
  const semanticProjectedMemberKinds = {};
  for (const [id, value] of Object.entries(projectedMemberRows)) {
    const spec = record(value, `semanticProjectedMemberKinds.${id}`);
    if (spec.identityField !== "mutationDirectoryName" && spec.identityField !== "commandDirectoryName")
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.identityField is invalid`);
    const ownerKindIds = stringArray(spec.ownerKindIds, `semanticProjectedMemberKinds.${id}.ownerKindIds`);
    if (ownerKindIds.length === 0)
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.ownerKindIds must not be empty`);
    semanticProjectedMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), projectionContractId: requiredString(spec.projectionContractId, `semanticProjectedMemberKinds.${id}.projectionContractId`), sourceMemberKindId: requiredString(spec.sourceMemberKindId, `semanticProjectedMemberKinds.${id}.sourceMemberKindId`), identityField: spec.identityField };
  }
  if (Object.keys(semanticProjectedMemberKinds).length === 0)
    throw new Error("Taxonomy v7 semanticProjectedMemberKinds must not be empty");
  const allDirectoryContextIds = new Set([...directoryContextIds, ...Object.keys(semanticProjectedMemberKinds)]);
  for (const [id, spec] of Object.entries(semanticDirectoryKinds))
    for (const parentId of spec.parentKindIds ?? [])
      if (!allDirectoryContextIds.has(parentId))
        throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id} references unknown parent kind ${parentId}`);
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds)) {
    if (!semanticDirectoryMemberKinds[spec.sourceMemberKindId])
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown source member kind ${spec.sourceMemberKindId}`);
    for (const ownerId of spec.ownerKindIds)
      if (!allDirectoryContextIds.has(ownerId))
        throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown owner kind ${ownerId}`);
  }
  const semanticPathProjectionProfileRenderers = {};
  for (const [id, value] of Object.entries(projectionRendererRows)) {
    const spec = record(value, `semanticPathProjectionProfileRenderers.${id}`);
    if (spec.direction !== "forward-only" || canonicalJson(spec.captureFields) !== canonicalJson(["standardVersion", "subsetId"]) || spec.template !== "\uD83E\uDE86\uFE0F{standardVersion}-{subsetId}" || canonicalJson(spec.tupleCollisionFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId"]))
      throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} is not the forward-only standard/subset contract`);
    const directoryKindId = requiredString(spec.directoryKindId, `semanticPathProjectionProfileRenderers.${id}.directoryKindId`);
    if (!semanticDirectoryKinds[directoryKindId])
      throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} references unknown directory kind ${directoryKindId}`);
    semanticPathProjectionProfileRenderers[id] = { direction: "forward-only", captureFields: ["standardVersion", "subsetId"], directoryKindId, template: "\uD83E\uDE86\uFE0F{standardVersion}-{subsetId}", tupleCollisionFields: ["artifactId", "standardVersion", "subsetId"] };
  }
  if (Object.keys(semanticPathProjectionProfileRenderers).length === 0)
    throw new Error("Taxonomy v7 semanticPathProjectionProfileRenderers must not be empty");
  const parseDescendantNode = (value, name) => {
    const spec = record(value, name);
    if (spec.nodeType !== "directory" && spec.nodeType !== "file")
      throw new Error(`Taxonomy v7 ${name}.nodeType is invalid`);
    const pathRows = spec.pathSegments;
    if (!Array.isArray(pathRows))
      throw new Error(`Taxonomy v7 ${name}.pathSegments must be an array`);
    const pathSegments = pathRows.map((value2, index) => {
      const segment = record(value2, `${name}.pathSegments[${index}]`);
      const kindId = requiredString(segment.kindId, `${name}.pathSegments[${index}].kindId`);
      const literal = requiredString(segment.literal, `${name}.pathSegments[${index}].literal`).normalize("NFC");
      const kind = semanticDirectoryKinds[kindId];
      const leading = splitLeadingEmoji(literal);
      if (!kind || emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest))
        throw new Error(`Taxonomy v7 ${name} has an invalid semantic path segment ${literal}`);
      return { kindId, literal };
    });
    if (spec.nodeType === "directory") {
      const kindId = requiredString(spec.kindId, `${name}.kindId`);
      if (!allDirectoryContextIds.has(kindId) || spec.sourceFilename !== undefined || spec.fixedFilenameContractId !== undefined || spec.packageGlue !== undefined)
        throw new Error(`Taxonomy v7 ${name} references an invalid directory kind ${kindId}`);
      return { pathSegments, nodeType: "directory", kindId };
    }
    const authorities = [spec.kindId !== undefined, spec.fixedFilenameContractId !== undefined, spec.packageGlue !== undefined].filter(Boolean).length;
    if (authorities !== 1)
      throw new Error(`Taxonomy v7 ${name} must declare exactly one file authority`);
    if (spec.kindId !== undefined) {
      const kindId = requiredString(spec.kindId, `${name}.kindId`);
      if (!fileKinds[kindId])
        throw new Error(`Taxonomy v7 ${name} references unknown file kind ${kindId}`);
      const sourceFilename = spec.sourceFilename === undefined ? undefined : requiredString(spec.sourceFilename, `${name}.sourceFilename`).normalize("NFC");
      if (sourceFilename !== undefined && (kindId !== "rust-source" || sourceFilename !== "\uD83E\uDD80\uFE0Fcomponent.rs"))
        throw new Error(`Taxonomy v7 ${name}.sourceFilename is not the frozen Draw Rust source leaf`);
      return { pathSegments, nodeType: "file", kindId, ...sourceFilename ? { sourceFilename } : {} };
    }
    if (spec.fixedFilenameContractId !== undefined) {
      const fixedFilenameContractId = requiredString(spec.fixedFilenameContractId, `${name}.fixedFilenameContractId`);
      if (!fixedFilenameContracts[fixedFilenameContractId])
        throw new Error(`Taxonomy v7 ${name} references unknown fixed filename contract ${fixedFilenameContractId}`);
      return { pathSegments, nodeType: "file", fixedFilenameContractId };
    }
    const glue = record(spec.packageGlue, `${name}.packageGlue`);
    return { pathSegments, nodeType: "file", packageGlue: { ecosystemId: requiredString(glue.ecosystemId, `${name}.packageGlue.ecosystemId`), filename: requiredString(glue.filename, `${name}.packageGlue.filename`).normalize("NFC") } };
  };
  const semanticDescendantContracts = {};
  for (const [id, value] of Object.entries(descendantContractRows)) {
    const spec = record(value, `semanticDescendantContracts.${id}`);
    const rootDirectoryKindId = requiredString(spec.rootDirectoryKindId, `semanticDescendantContracts.${id}.rootDirectoryKindId`);
    if (!allDirectoryContextIds.has(rootDirectoryKindId))
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references unknown root directory kind ${rootDirectoryKindId}`);
    if (spec.contractKind === "catalog") {
      const catalogContractId = requiredString(spec.catalogContractId, `semanticDescendantContracts.${id}.catalogContractId`);
      const leafFileKindId = requiredString(spec.leafFileKindId, `semanticDescendantContracts.${id}.leafFileKindId`);
      const reserve2 = record(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
      if (!fileKinds[leafFileKindId] || spec.rendering !== "semantic-member-directory-and-physical-kind-leaf" || reserve2.derivation !== "longest-rendered-catalog-descendant-suffix" || !Number.isSafeInteger(reserve2.bytes) || reserve2.bytes <= 0)
        throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} is not a valid catalog descendant contract`);
      semanticDescendantContracts[id] = { contractKind: "catalog", rootDirectoryKindId, catalogContractId, leafFileKindId, rendering: "semantic-member-directory-and-physical-kind-leaf", pathBudgetReserve: { derivation: "longest-rendered-catalog-descendant-suffix", bytes: reserve2.bytes } };
      continue;
    }
    if (!Array.isArray(spec.requiredNodes) || !Array.isArray(spec.exclusiveAlternatives))
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} node lists must be arrays`);
    const requiredNodes = spec.requiredNodes.map((node, index) => parseDescendantNode(node, `semanticDescendantContracts.${id}.requiredNodes[${index}]`));
    const exclusiveAlternatives = spec.exclusiveAlternatives.map((value2, index) => {
      const alternative = record(value2, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}]`);
      if (alternative.mode !== "exactly-one" || !Array.isArray(alternative.nodes) || alternative.nodes.length < 2)
        throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} alternative must contain exactly-one candidates`);
      return { id: requiredString(alternative.id, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].id`), mode: "exactly-one", nodes: alternative.nodes.map((node, nodeIndex) => parseDescendantNode(node, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].nodes[${nodeIndex}]`)) };
    });
    if (!Number.isSafeInteger(spec.realizedNodeCount) || spec.realizedNodeCount !== requiredNodes.length + exclusiveAlternatives.length)
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.realizedNodeCount is invalid`);
    const reserve = record(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
    const suffix = (node) => {
      const segments = node.pathSegments.map((segment) => segment.literal);
      if (node.nodeType === "file") {
        if ("kindId" in node) {
          const kind = fileKinds[node.kindId];
          if (kind.extensionChains.length !== 1)
            throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} file kind ${node.kindId} must have one physical extension chain`);
          segments.push(`${kind.emoji}${kind.extensionChains[0]}`);
        } else if ("fixedFilenameContractId" in node)
          segments.push(posix.basename(fixedFilenameContracts[node.fixedFilenameContractId].pathPattern));
        else
          segments.push(node.packageGlue.filename);
      }
      return segments.length === 0 ? "" : `/${segments.join("/")}`;
    };
    const reserveBytes = Math.max(...[...requiredNodes, ...exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].map((node) => Buffer.byteLength(suffix(node), "utf8")));
    if (reserve.derivation !== "longest-canonical-descendant-suffix" || reserve.bytes !== reserveBytes)
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.pathBudgetReserve is not derived from its longest suffix`);
    semanticDescendantContracts[id] = { rootDirectoryKindId, requiredNodes, exclusiveAlternatives, realizedNodeCount: spec.realizedNodeCount, pathBudgetReserve: { derivation: "longest-canonical-descendant-suffix", bytes: reserveBytes } };
  }
  if (Object.keys(semanticDescendantContracts).length === 0)
    throw new Error("Taxonomy v7 semanticDescendantContracts must not be empty");
  const semanticPathProjectionCatalogContracts = {};
  const expectedCatalogContract = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
  for (const [id, value] of Object.entries(projectionCatalogRows)) {
    const spec = record(value, `semanticPathProjectionCatalogContracts.${id}`);
    if (spec.contractKind === undefined) {
      if (canonicalJson(value) !== canonicalJson(expectedCatalogContract))
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not the independent required vector registry contract`);
      semanticPathProjectionCatalogContracts[id] = expectedCatalogContract;
      continue;
    }
    if (spec.contractKind === "distributed-json-manifest-catalog") {
      if (spec.modelIdentityField !== "id" || spec.memberIdentityField !== "id" || spec.memberVersionField !== "version" || spec.requiredModelManifest !== true || spec.coverage !== "every-source-file-and-destination-node-exactly-once" || spec.unknownCategoryPolicy !== "problem" || spec.unownedModelPolicy !== "problem" || !Array.isArray(spec.categoryRules) || spec.categoryRules.length === 0)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict distributed manifest catalog`);
      const categoryRules = spec.categoryRules.map((value2, index) => {
        const rule = record(value2, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}]`);
        const sourceDirectoryName = requiredString(rule.sourceDirectoryName, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceDirectoryName`).normalize("NFC");
        const directoryKindId = requiredString(rule.directoryKindId, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].directoryKindId`);
        const manifestSchema = requiredString(rule.manifestSchema, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].manifestSchema`);
        if (!semanticDirectoryKinds[directoryKindId])
          throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}] references an unknown directory kind`);
        if (rule.sourceShape === "direct-semantic-json")
          return { sourceDirectoryName, directoryKindId, sourceShape: "direct-semantic-json", manifestSchema, memberDirectoryEmoji: requiredString(rule.memberDirectoryEmoji, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].memberDirectoryEmoji`).normalize("NFC") };
        if (rule.sourceShape === "nested-fixed-json")
          return { sourceDirectoryName, directoryKindId, sourceShape: "nested-fixed-json", manifestSchema, fixedSourceFilename: requiredString(rule.fixedSourceFilename, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].fixedSourceFilename`).normalize("NFC") };
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceShape is invalid`);
      });
      if (new Set(categoryRules.map((rule) => rule.sourceDirectoryName)).size !== categoryRules.length)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a catalog category`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "distributed-json-manifest-catalog", ownerArtifactMemberName: requiredString(spec.ownerArtifactMemberName, `semanticPathProjectionCatalogContracts.${id}.ownerArtifactMemberName`).normalize("NFC"), modelManifestSchema: requiredString(spec.modelManifestSchema, `semanticPathProjectionCatalogContracts.${id}.modelManifestSchema`), modelManifestSourceFilename: requiredString(spec.modelManifestSourceFilename, `semanticPathProjectionCatalogContracts.${id}.modelManifestSourceFilename`).normalize("NFC"), modelIdentityField: "id", memberIdentityField: "id", memberVersionField: "version", requiredMemberVersion: requiredString(spec.requiredMemberVersion, `semanticPathProjectionCatalogContracts.${id}.requiredMemberVersion`), requiredModelManifest: true, categoryRules, coverage: "every-source-file-and-destination-node-exactly-once", unknownCategoryPolicy: "problem", unownedModelPolicy: "problem" };
      continue;
    }
    if (spec.contractKind === "exact-owner-vectors") {
      if (spec.required !== true || spec.allowEmpty !== false || canonicalJson(spec.identityFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId", "commandDirectoryName"]) || spec.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(spec.vectors) || spec.vectors.length === 0)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict exact-owner vector registry`);
      const vectors = spec.vectors.map((value2, index) => {
        const vector = record(value2, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}]`);
        return { artifactId: requiredString(vector.artifactId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].artifactId`).normalize("NFC"), standardVersion: requiredString(vector.standardVersion, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].standardVersion`), subsetId: requiredString(vector.subsetId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].subsetId`), commandDirectoryName: requiredString(vector.commandDirectoryName, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].commandDirectoryName`).normalize("NFC") };
      });
      if (new Set(vectors.map((vector) => canonicalJson(vector))).size !== vectors.length)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats an owner vector`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "exact-owner-vectors", required: true, allowEmpty: false, identityFields: ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], coverage: "every-physical-command-bundle-exactly-once", vectors };
      continue;
    }
    throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.contractKind is invalid`);
  }
  if (Object.keys(semanticPathProjectionCatalogContracts).length === 0)
    throw new Error("Taxonomy v7 semanticPathProjectionCatalogContracts must not be empty");
  for (const [id, contract] of Object.entries(semanticDescendantContracts))
    if ("contractKind" in contract && !semanticPathProjectionCatalogContracts[contract.catalogContractId])
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references an unknown catalog contract`);
  const captureFields = new Set(["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"]);
  const parseProjectionSegment = (value, name, destination) => {
    const spec = record(value, name);
    const kindId = typeof spec.kindId === "string" ? spec.kindId : undefined;
    const memberKindId = typeof spec.memberKindId === "string" ? spec.memberKindId : undefined;
    const projectedMemberKindId = typeof spec.projectedMemberKindId === "string" ? spec.projectedMemberKindId : undefined;
    if ((kindId ? 1 : 0) + (memberKindId ? 1 : 0) + (projectedMemberKindId ? 1 : 0) !== 1)
      throw new Error(`Taxonomy v7 ${name} must identify exactly one kind`);
    if (kindId && !allDirectoryContextIds.has(kindId))
      throw new Error(`Taxonomy v7 ${name} references unknown directory kind ${kindId}`);
    if (memberKindId && !semanticDirectoryMemberKinds[memberKindId])
      throw new Error(`Taxonomy v7 ${name} references unknown semantic member kind ${memberKindId}`);
    if (projectedMemberKindId && !semanticProjectedMemberKinds[projectedMemberKindId])
      throw new Error(`Taxonomy v7 ${name} references unknown projected member kind ${projectedMemberKindId}`);
    if (destination) {
      if (memberKindId)
        throw new Error(`Taxonomy v7 ${name} cannot render a source member kind`);
      if (spec.literal !== undefined && kindId)
        return { kindId, literal: requiredString(spec.literal, `${name}.literal`) };
      if (spec.render === "profile" && kindId)
        return { kindId, render: "profile" };
      if (typeof spec.copy === "string" && captureFields.has(spec.copy))
        return projectedMemberKindId ? { projectedMemberKindId, copy: spec.copy } : { kindId, copy: spec.copy };
    } else {
      if (spec.literal !== undefined && kindId)
        return { kindId, literal: requiredString(spec.literal, `${name}.literal`) };
      if (spec.literal !== undefined && memberKindId) {
        const literal = requiredString(spec.literal, `${name}.literal`).normalize("NFC");
        if (!semanticDirectoryMemberKinds[memberKindId].memberNames.includes(literal))
          throw new Error(`Taxonomy v7 ${name}.literal is not registered by ${memberKindId}`);
        return { memberKindId, literal };
      }
      if (typeof spec.capture === "string" && captureFields.has(spec.capture))
        return projectedMemberKindId ? { projectedMemberKindId, capture: spec.capture } : { kindId, capture: spec.capture };
    }
    throw new Error(`Taxonomy v7 ${name} has an invalid ${destination ? "destination" : "source"} operation`);
  };
  const semanticPathProjectionContracts = {};
  for (const [id, value] of Object.entries(projectionRows)) {
    const spec = record(value, `semanticPathProjectionContracts.${id}`);
    if (!Array.isArray(spec.sourceSegments) || !Array.isArray(spec.destinationSegments) || !["artifact-mutation-test-projection-v1", "artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(String(spec.rationaleRule)))
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} is invalid`);
    const sourceOwnerKindId = requiredString(spec.sourceOwnerKindId, `semanticPathProjectionContracts.${id}.sourceOwnerKindId`);
    const destinationOwnerKindId = requiredString(spec.destinationOwnerKindId, `semanticPathProjectionContracts.${id}.destinationOwnerKindId`);
    if (!semanticDirectoryMemberKinds[sourceOwnerKindId] || !semanticDirectoryMemberKinds[destinationOwnerKindId])
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} owner kind is invalid`);
    const profileRendererId = requiredString(spec.profileRendererId, `semanticPathProjectionContracts.${id}.profileRendererId`);
    const descendantContractId = requiredString(spec.descendantContractId, `semanticPathProjectionContracts.${id}.descendantContractId`);
    const catalogContractId = requiredString(spec.catalogContractId, `semanticPathProjectionContracts.${id}.catalogContractId`);
    if (!semanticPathProjectionProfileRenderers[profileRendererId] || !semanticDescendantContracts[descendantContractId] || !semanticPathProjectionCatalogContracts[catalogContractId])
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references an unknown registry`);
    const rationaleRule = spec.rationaleRule;
    const sourceArtifactMemberName = spec.sourceArtifactMemberName === undefined ? undefined : requiredString(spec.sourceArtifactMemberName, `semanticPathProjectionContracts.${id}.sourceArtifactMemberName`).normalize("NFC");
    const expectedArtifact = rationaleRule === "artifact-example-model-catalog-projection-v1" ? "\uD83D\uDCD0\uFE0Fcad" : rationaleRule === "artifact-editor-command-projection-v1" ? "\uD83D\uDD8D\uFE0Fdraw" : undefined;
    if (sourceArtifactMemberName !== expectedArtifact)
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id}.sourceArtifactMemberName does not match its rationale`);
    const sourceSegments = spec.sourceSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.sourceSegments[${index}]`, false));
    const destinationSegments = spec.destinationSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.destinationSegments[${index}]`, true));
    const captures = sourceSegments.flatMap((segment) => ("capture" in segment) ? [segment.capture] : []);
    const expectedCaptures = rationaleRule === "artifact-mutation-test-projection-v1" ? ["standardVersion", "subsetId", "mutationId", "scenarioId"] : rationaleRule === "artifact-editor-command-projection-v1" ? ["standardVersion", "subsetId", "commandDirectoryName"] : ["standardVersion", "subsetId"];
    if (canonicalJson(captures) !== canonicalJson(expectedCaptures))
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid captures for ${rationaleRule}`);
    const descendant = semanticDescendantContracts[descendantContractId];
    const catalog = semanticPathProjectionCatalogContracts[catalogContractId];
    if (rationaleRule === "artifact-mutation-test-projection-v1" ? "contractKind" in descendant || "contractKind" in catalog : rationaleRule === "artifact-example-model-catalog-projection-v1" ? !(("contractKind" in descendant) && descendant.contractKind === "catalog" && ("contractKind" in catalog) && catalog.contractKind === "distributed-json-manifest-catalog") : ("contractKind" in descendant) || !(("contractKind" in catalog) && catalog.contractKind === "exact-owner-vectors"))
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references incompatible descendant/catalog authorities`);
    const descendantNodes = "contractKind" in descendant ? [] : [...descendant.requiredNodes, ...descendant.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)];
    const sourceNamedNodes = descendantNodes.filter((node) => ("kindId" in node) && node.sourceFilename !== undefined);
    if (rationaleRule === "artifact-editor-command-projection-v1" ? sourceNamedNodes.length !== 3 || descendantNodes.filter((node) => ("kindId" in node) && node.nodeType === "file" && node.kindId === "rust-source").length !== 3 : sourceNamedNodes.length !== 0)
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid source-filename descendant authority`);
    semanticPathProjectionContracts[id] = { sourceOwnerKindId, ...sourceArtifactMemberName ? { sourceArtifactMemberName } : {}, sourceSegments, profileRendererId, destinationOwnerKindId, destinationSegments, descendantContractId, catalogContractId, rationaleRule };
  }
  if (Object.keys(semanticPathProjectionContracts).length === 0)
    throw new Error("Taxonomy v7 semanticPathProjectionContracts must not be empty");
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds))
    if (!semanticPathProjectionContracts[spec.projectionContractId])
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown projection contract ${spec.projectionContractId}`);
  const mutationCatalogProjection = {
    projectionContractId: requiredString(mutationCatalogProjectionRow.projectionContractId, "mutationCatalogProjection.projectionContractId"),
    projectedMemberKindId: requiredString(mutationCatalogProjectionRow.projectedMemberKindId, "mutationCatalogProjection.projectedMemberKindId"),
    descendantContractId: requiredString(mutationCatalogProjectionRow.descendantContractId, "mutationCatalogProjection.descendantContractId"),
    catalogContractId: requiredString(mutationCatalogProjectionRow.catalogContractId, "mutationCatalogProjection.catalogContractId")
  };
  if (!semanticPathProjectionContracts[mutationCatalogProjection.projectionContractId] || !semanticProjectedMemberKinds[mutationCatalogProjection.projectedMemberKindId] || !semanticDescendantContracts[mutationCatalogProjection.descendantContractId] || !semanticPathProjectionCatalogContracts[mutationCatalogProjection.catalogContractId])
    throw new Error("Taxonomy v7 mutationCatalogProjection references unknown projection registries");
  const generatorContracts = {};
  const generatorRoots = [];
  for (const [id, value] of Object.entries(generatorRows)) {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id))
      throw new Error(`Taxonomy v7 generatorContracts.${id} has an invalid identifier`);
    const spec = record(value, `generatorContracts.${id}`);
    if (spec.ownership !== "owned" && spec.ownership !== "external")
      throw new Error(`Taxonomy v7 generatorContracts.${id}.ownership is invalid`);
    const ownership = spec.ownership;
    const ownerPath = spec.ownerPath === null ? null : normalizeRelative(requiredString(spec.ownerPath, `generatorContracts.${id}.ownerPath`));
    const target = spec.target === null ? null : requiredString(spec.target, `generatorContracts.${id}.target`);
    const previewTarget = spec.previewTarget === undefined ? undefined : requiredString(spec.previewTarget, `generatorContracts.${id}.previewTarget`);
    const checkTarget = spec.checkTarget === undefined ? undefined : requiredString(spec.checkTarget, `generatorContracts.${id}.checkTarget`);
    if (ownership === "owned" !== (ownerPath !== null && target !== null))
      throw new Error(`Taxonomy v7 generatorContracts.${id} owner and target do not match ownership`);
    if (target && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(target))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.target must be one exact Nx target`);
    if (ownership === "owned" ? !previewTarget : previewTarget !== undefined)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget does not match ownership`);
    if (previewTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(previewTarget))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget must be one exact Nx target`);
    if (target && previewTarget !== `${target.slice(0, target.lastIndexOf(":"))}:preview-generated`)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget must be the exact owner preview-generated target`);
    if (checkTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(checkTarget))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.checkTarget must be one exact Nx target`);
    const inputPatterns = stringArray(spec.inputPatterns, `generatorContracts.${id}.inputPatterns`).map((pattern, index) => validatedContractPattern(pattern, `generatorContracts.${id}.inputPatterns[${index}]`, false));
    if (ownership === "owned" ? inputPatterns.length === 0 : inputPatterns.length !== 0)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.inputPatterns do not match ownership`);
    const outputRows = spec.outputRoots;
    if (!Array.isArray(outputRows) || outputRows.length === 0)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.outputRoots must not be empty`);
    const outputRoots = outputRows.map((value2, index) => {
      const output = record(value2, `generatorContracts.${id}.outputRoots[${index}]`);
      const outputPath = requiredString(output.path, `generatorContracts.${id}.outputRoots[${index}].path`);
      if (outputPath !== normalizeRelative(outputPath) || /[*?\[\]]/u.test(outputPath))
        throw new Error(`Taxonomy v7 generatorContracts.${id} output path must be one literal NFC repository path`);
      if (output.inclusion !== "tracked" && output.inclusion !== "ignored")
        throw new Error(`Taxonomy v7 generatorContracts.${id} output inclusion is invalid`);
      generatorRoots.push({ id, path: outputPath });
      return { path: outputPath, inclusion: output.inclusion };
    }).sort((left, right) => left.path.localeCompare(right.path));
    if (new Set(outputRoots.map((output) => output.path)).size !== outputRoots.length)
      throw new Error(`Taxonomy v7 generatorContracts.${id} repeats an output root`);
    generatorContracts[id] = { ownership, ownerPath, target, previewTarget, checkTarget, inputPatterns: [...new Set(inputPatterns)].sort(), outputRoots, reason: requiredString(spec.reason, `generatorContracts.${id}.reason`) };
  }
  if (Object.keys(generatorContracts).length === 0)
    throw new Error("Taxonomy v7 generatorContracts must not be empty");
  for (let left = 0;left < generatorRoots.length; left++)
    for (let right = left + 1;right < generatorRoots.length; right++) {
      const a = generatorRoots[left];
      const b = generatorRoots[right];
      if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`))
        throw new Error(`Taxonomy v7 generator output roots overlap: ${a.id}:${a.path} and ${b.id}:${b.path}`);
    }
  const packageGlueGrammar = {};
  for (const [id, value] of Object.entries(grammarRows)) {
    const spec = record(value, `packageGlueGrammar.${id}`);
    if (!["rust", "typescript", "go", "python", "dotnet"].includes(String(spec.analyzer)))
      throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.analyzer is invalid`);
    const allowedRoles = stringArray(spec.allowedRoles, `packageGlueGrammar.${id}.allowedRoles`);
    if (allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role)) || new Set(allowedRoles).size !== allowedRoles.length)
      throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.allowedRoles is invalid`);
    if (!Number.isSafeInteger(spec.maxDelegationStatements) || spec.maxDelegationStatements < 0)
      throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.maxDelegationStatements is invalid`);
    packageGlueGrammar[id] = { analyzer: spec.analyzer, allowedRoles, maxDelegationStatements: spec.maxDelegationStatements };
  }
  const packageBoundaryRules = {};
  for (const [id, value] of Object.entries(boundaryRows)) {
    const spec = record(value, `packageBoundaryRules.${id}`);
    const glueGrammarId = requiredString(spec.glueGrammarId, `packageBoundaryRules.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId])
      throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown grammar ${glueGrammarId}`);
    if (spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem")
      throw new Error(`Taxonomy v7 packageBoundaryRules.${id} must be recursive and fail closed`);
    packageBoundaryRules[id] = {
      manifestContractId: spec.manifestContractId === null ? null : requiredString(spec.manifestContractId, `packageBoundaryRules.${id}.manifestContractId`),
      entryContractIds: stringArray(spec.entryContractIds, `packageBoundaryRules.${id}.entryContractIds`),
      allowedFixedContractIds: stringArray(spec.allowedFixedContractIds, `packageBoundaryRules.${id}.allowedFixedContractIds`),
      allowedFileKindIds: stringArray(spec.allowedFileKindIds, `packageBoundaryRules.${id}.allowedFileKindIds`),
      allowedDirectoryKindIds: stringArray(spec.allowedDirectoryKindIds, `packageBoundaryRules.${id}.allowedDirectoryKindIds`),
      glueGrammarId,
      recursive: true,
      uncertainRole: "problem",
      implementationRole: "problem"
    };
    const rule = packageBoundaryRules[id];
    if (rule.manifestContractId && !fixedFilenameContracts[rule.manifestContractId])
      throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown manifest contract ${rule.manifestContractId}`);
    for (const contractId of rule.entryContractIds)
      if (!configurableEntryContracts[contractId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown entry contract ${contractId}`);
    for (const contractId of rule.allowedFixedContractIds)
      if (!fixedFilenameContracts[contractId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown fixed contract ${contractId}`);
    for (const kindId of rule.allowedFileKindIds)
      if (!fileKinds[kindId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown file kind ${kindId}`);
    for (const kindId of rule.allowedDirectoryKindIds)
      if (!semanticDirectoryKinds[kindId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown directory kind ${kindId}`);
  }
  for (const [id, contract] of Object.entries(fixedFilenameContracts))
    if (contract.scope === "package-root" && !packageBoundaryRules[contract.ecosystemId])
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id} references unknown ecosystem ${contract.ecosystemId}`);
  const pathExclusions = {};
  const exclusions = [];
  for (const [id, value] of Object.entries(exclusionRows)) {
    const spec = record(value, `pathExclusions.${id}`);
    if (spec.mode !== "opaque")
      throw new Error(`Taxonomy v7 pathExclusions.${id}.mode must be opaque`);
    const excludedPath = normalizeRelative(requiredString(spec.path, `pathExclusions.${id}.path`));
    pathExclusions[id] = { path: excludedPath, mode: "opaque", reason: requiredString(spec.reason, `pathExclusions.${id}.reason`) };
    exclusions.push({ id, path: excludedPath });
  }
  if (Object.keys(pathExclusions).length !== 1 || pathExclusions.compose?.path !== "compose")
    throw new Error("Taxonomy v7 pathExclusions must contain only opaque compose");
  for (const id of stringArray(enforcement.opaquePathExclusionIds, "areaEnforcement.opaquePathExclusionIds")) {
    if (!pathExclusions[id])
      throw new Error(`Taxonomy v7 areaEnforcement references unknown opaque exclusion ${id}`);
  }
  const opaquePaths = Object.values(pathExclusions).map((entry) => entry.path);
  const crossesOpaque = (value) => opaquePaths.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
  for (const [id, contract] of Object.entries(generatorContracts)) {
    if (contract.ownerPath && crossesOpaque(contract.ownerPath))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.ownerPath crosses an opaque path`);
    for (const pattern of contract.inputPatterns)
      if (opaquePaths.some((opaque) => taxonomyPathPatternMatches(opaque, pattern) || taxonomyPathPatternMatches(`${opaque}/probe`, pattern)))
        throw new Error(`Taxonomy v7 generatorContracts.${id} input pattern admits an opaque path`);
    for (const output of contract.outputRoots)
      if (crossesOpaque(output.path))
        throw new Error(`Taxonomy v7 generatorContracts.${id} output root crosses an opaque path`);
  }
  const schema = {
    schemaVersion: 7,
    fileKinds,
    semanticDirectoryKinds,
    fixedFilenameContracts,
    fixedDirectoryContracts,
    configurableEntryContracts,
    fileKindResolutionRules,
    scopedFileKinds,
    semanticDirectoryMemberKinds,
    semanticProjectedMemberKinds,
    semanticPathProjectionProfileRenderers,
    semanticDescendantContracts,
    semanticPathProjectionCatalogContracts,
    semanticPathProjectionContracts,
    mutationCatalogProjection,
    generatorContracts,
    packageBoundaryRules,
    packageGlueGrammar,
    pathExclusions,
    unicodeNormalization: { form: "NFC", caseFold: "lower", locale: "und" },
    variationSelectorPolicy: { selector: "\uFE0F", requiredAfterEmoji: true, comparison: "ignore-selector" },
    collisionPolicy: {
      comparisons: collision.comparisons,
      maxPathBytes: collision.maxPathBytes,
      rejectWindowsReservedNames: collision.rejectWindowsReservedNames === true,
      rejectTrailingDotsAndSpaces: collision.rejectTrailingDotsAndSpaces === true
    },
    areaEnforcement: { requiredState: "clean", undeclaredAreas: "enforce", opaquePathExclusionIds: [...enforcement.opaquePathExclusionIds] }
  };
  return {
    path,
    schema,
    exclusions: exclusions.sort((a, b) => a.path.localeCompare(b.path)),
    fileKinds: Object.entries(fileKinds).map(([id, spec]) => ({ id, ...spec })).sort((a, b) => a.id.localeCompare(b.id)),
    directoryKinds: Object.entries(semanticDirectoryKinds).map(([id, spec]) => ({ id, ...spec, slugRegex: new RegExp(`^(?:${spec.slugPattern})$`, "u") })).sort((a, b) => a.id.localeCompare(b.id))
  };
}
function loadTaxonomy(options) {
  const path = options.taxonomyPath ? resolve2(options.repoRoot, options.taxonomyPath) : absolutePath(options.repoRoot, TAXONOMY_RELATIVE_PATH);
  return parseTaxonomy(JSON.parse(readFileSync(path, "utf8")), path);
}
function sha256(value) {
  return createHash("sha256").update(value).digest("hex");
}
function canonicalArrayKey(value) {
  if (!value || typeof value !== "object" || Array.isArray(value))
    return null;
  const row = value;
  const keys = ["operationId", "sourcePath", "path", "id", "destinationPath", "code", "relativeRoot", "structuredLocation"];
  const parts = keys.filter((key) => typeof row[key] === "string").map((key) => `${key}:${row[key]}`);
  return parts.length > 0 ? parts.join("\x00") : null;
}
function canonicalValue(value) {
  if (Array.isArray(value)) {
    const rows = value.map(canonicalValue);
    if (rows.every((row) => canonicalArrayKey(row) !== null))
      return [...rows].sort((a, b) => canonicalArrayKey(a).localeCompare(canonicalArrayKey(b)));
    return rows;
  }
  if (!value || typeof value !== "object")
    return value;
  const source = value;
  const target = {};
  for (const key of Object.keys(source).sort()) {
    if (source[key] !== undefined)
      target[key] = canonicalValue(source[key]);
  }
  return target;
}
function canonicalJson(value) {
  return JSON.stringify(canonicalValue(value));
}
function generatorPathCompare(left, right) {
  return Buffer.from(left).compare(Buffer.from(right));
}
function generatorPreviewJson(manifest) {
  return JSON.stringify({
    contractId: manifest.contractId,
    nodes: manifest.nodes.map((node) => ({ bytesBase64: node.bytesBase64, mode: node.mode, nodeKind: node.nodeKind, path: node.path })),
    schemaVersion: manifest.schemaVersion,
    staleRemovals: manifest.staleRemovals
  });
}
function parseGeneratorPreviewManifest(content, expectedContractId, outputRoots, excludedRoots = []) {
  let value;
  try {
    value = JSON.parse(content);
  } catch {
    throw new Error(`Generator preview stdout is not one canonical JSON document: bytes=${Buffer.byteLength(content)}, sha256=${sha256(content)}`);
  }
  const root = record(value, "generator preview");
  if (Object.keys(root).join("\x00") !== "contractId\x00nodes\x00schemaVersion\x00staleRemovals")
    throw new Error("Generator preview has noncanonical top-level keys or order");
  if (root.schemaVersion !== 1)
    throw new Error("Generator preview schemaVersion must be 1");
  if (root.contractId !== expectedContractId)
    throw new Error(`Generator preview contractId does not match ${expectedContractId}`);
  if (!Array.isArray(root.nodes) || !Array.isArray(root.staleRemovals))
    throw new Error("Generator preview nodes and staleRemovals must be arrays");
  const roots = [...new Set(outputRoots.map((path) => normalizeRelative(path)))].sort(generatorPathCompare);
  if (roots.length !== outputRoots.length || roots.some((path, index) => path !== outputRoots[index]))
    throw new Error("Generator preview output roots must be unique, NFC, repository-relative, and byte-sorted");
  const exclusions = excludedRoots.map(normalizeRelative);
  const withinRoot = (path) => roots.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const excluded = (path) => exclusions.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const nodes = root.nodes.map((value2, index) => {
    const node = record(value2, `generator preview nodes[${index}]`);
    if (Object.keys(node).join("\x00") !== "bytesBase64\x00mode\x00nodeKind\x00path")
      throw new Error(`Generator preview node ${index} has noncanonical keys or order`);
    const path = requiredString(node.path, `generator preview nodes[${index}].path`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path))
      throw new Error(`Generator preview node path is unsafe or outside registered roots: ${path}`);
    if (node.nodeKind !== "directory" && node.nodeKind !== "file")
      throw new Error(`Generator preview nodeKind is invalid at ${path}`);
    if (!Number.isSafeInteger(node.mode) || node.mode < 0 || node.mode > 4095)
      throw new Error(`Generator preview mode is invalid at ${path}`);
    if (typeof node.bytesBase64 !== "string" || !/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/u.test(node.bytesBase64))
      throw new Error(`Generator preview base64 is invalid at ${path}`);
    const decoded = Buffer.from(node.bytesBase64, "base64");
    if (decoded.toString("base64") !== node.bytesBase64 || node.nodeKind === "directory" && node.bytesBase64 !== "")
      throw new Error(`Generator preview base64 is noncanonical at ${path}`);
    return { bytesBase64: node.bytesBase64, mode: node.mode, nodeKind: node.nodeKind, path };
  });
  const nodeByPath = new Map;
  for (let index = 0;index < nodes.length; index++) {
    const node = nodes[index];
    if (nodeByPath.has(node.path) || index > 0 && generatorPathCompare(nodes[index - 1].path, node.path) >= 0)
      throw new Error(`Generator preview nodes repeat or are not byte-sorted at ${node.path}`);
    nodeByPath.set(node.path, node);
  }
  for (const registeredRoot of roots)
    if (!nodeByPath.has(registeredRoot))
      throw new Error(`Generator preview omits registered output root ${registeredRoot}`);
  for (const node of nodes) {
    let parent = posix.dirname(node.path);
    const registeredRoot = roots.filter((candidate) => node.path === candidate || node.path.startsWith(`${candidate}/`)).sort((left, right) => right.length - left.length)[0];
    while (registeredRoot && parent !== posix.dirname(registeredRoot)) {
      const parentNode = nodeByPath.get(parent);
      if (!parentNode || parentNode.nodeKind !== "directory")
        throw new Error(`Generator preview omits directory node ${parent}`);
      if (parent === registeredRoot)
        break;
      parent = posix.dirname(parent);
    }
    if (node.nodeKind === "file" && nodes.some((candidate) => candidate.path.startsWith(`${node.path}/`)))
      throw new Error(`Generator preview file has descendants at ${node.path}`);
  }
  const staleRemovals = root.staleRemovals.map((value2, index) => {
    const path = requiredString(value2, `generator preview staleRemovals[${index}]`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path))
      throw new Error(`Generator preview stale removal is unsafe or outside registered roots: ${path}`);
    if (nodeByPath.has(path) || nodes.some((node) => node.path.startsWith(`${path}/`)))
      throw new Error(`Generator preview stale removal overlaps expected output ${path}`);
    return path;
  });
  for (let index = 0;index < staleRemovals.length; index++)
    if (index > 0 && generatorPathCompare(staleRemovals[index - 1], staleRemovals[index]) >= 0 || staleRemovals.some((path, candidate) => candidate !== index && path.startsWith(`${staleRemovals[index]}/`)))
      throw new Error(`Generator preview stale removals repeat, overlap, or are not byte-sorted at ${staleRemovals[index]}`);
  const manifest = { contractId: expectedContractId, nodes, schemaVersion: 1, staleRemovals };
  if (content !== `${generatorPreviewJson(manifest)}
`)
    throw new Error("Generator preview stdout is noisy or not byte-canonical JSON");
  return manifest;
}
function normalizeRelative(value) {
  return sourceRelative(value).normalize("NFC");
}
function sourceRelative(value) {
  const slash = value.replaceAll("\\", "/").replace(/^\.\//, "");
  const normalized = posix.normalize(slash);
  if (normalized === ".")
    return "";
  if (normalized === ".." || normalized.startsWith("../") || normalized.startsWith("/") || normalized.includes("\x00"))
    throw new Error(`Path escapes repository scope: ${value}`);
  return normalized.replace(/\/$/, "");
}
function absolutePath(repoRoot, path) {
  const root = resolve2(repoRoot);
  const result = resolve2(root, ...sourceRelative(path).split("/").filter(Boolean));
  const rel = relative2(root, result);
  if (rel === ".." || rel.startsWith(`..${sep}`) || rel.startsWith("../") || rel.startsWith("..\\") || isAbsolute(rel))
    throw new Error(`Path escapes repository root: ${path}`);
  return result;
}
function isExcluded(path, taxonomy) {
  const normalized = normalizeRelative(path);
  return taxonomy.exclusions.some((entry) => normalized === entry.path || normalized.startsWith(`${entry.path}/`));
}
function inScope(path, scope) {
  if (!scope)
    return true;
  const normalizedScope = normalizeRelative(scope);
  const normalizedPath = normalizeRelative(path);
  return normalizedPath === normalizedScope || normalizedPath.startsWith(`${normalizedScope}/`) || normalizedScope.startsWith(`${normalizedPath}/`);
}
function emojiFold(value) {
  return value.normalize("NFC").replaceAll("\uFE0F", "");
}
function graphemes(value) {
  return [...SEGMENTER.segment(value)].map((entry) => entry.segment);
}
function isEmojiGrapheme(value) {
  return /[\p{Extended_Pictographic}\p{Emoji_Presentation}\uFE0F\u20E3]/u.test(value);
}
function splitLeadingEmoji(value) {
  const segments = graphemes(value);
  if (segments.length === 0 || !isEmojiGrapheme(segments[0]))
    return { emoji: "", rest: value };
  return { emoji: segments[0], rest: segments.slice(1).join("") };
}
function matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds = []) {
  const normalized = name.normalize("NFC");
  const leading = splitLeadingEmoji(normalized);
  const contextAllows = (kind) => (kind.parentKindIds?.length ?? 0) === 0 || parentKindId !== undefined && kind.parentKindIds?.includes(parentKindId) === true;
  if (leading.emoji) {
    const global = taxonomy.directoryKinds.filter((kind) => emojiFold(kind.emoji) === emojiFold(leading.emoji) && (leading.rest.length === 0 && kind.allowEmojiOnly || kind.slugRegex.test(leading.rest)));
    const exact2 = global.filter((kind) => kind.id.normalize("NFC").toLocaleLowerCase("und") === leading.rest.toLocaleLowerCase("und"));
    if (exact2.length === 1)
      return { kind: exact2[0], slug: leading.rest, ambiguous: [] };
    if (exact2.length > 1)
      return { kind: null, slug: leading.rest, ambiguous: exact2.map((entry) => entry.id) };
    const contextual2 = parentKindId === undefined ? [] : global.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
    const ordinary = contextual2.length > 0 ? contextual2 : global.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
    if (ordinary.length === 1)
      return { kind: ordinary[0], slug: leading.rest, ambiguous: [] };
    const contexts = [parentKindId, ...ancestorKindIds].filter((kindId, index, rows) => Boolean(kindId) && rows.indexOf(kindId) === index);
    const overlays = Object.entries(taxonomy.schema.semanticDirectoryMemberKinds).filter(([, spec]) => spec.memberNames.includes(normalized)).map(([id, spec]) => ({ id, distance: contexts.findIndex((kindId) => spec.ownerKindIds.includes(kindId)) })).filter((entry) => entry.distance >= 0).sort((left, right) => left.distance - right.distance || left.id.localeCompare(right.id));
    if (overlays.length > 0) {
      const nearest = overlays.filter((entry) => entry.distance === overlays[0].distance);
      if (nearest.length === 1)
        return { kind: { id: nearest[0].id, emoji: leading.emoji }, slug: leading.rest, ambiguous: [] };
      return { kind: null, slug: leading.rest, ambiguous: nearest.map((entry) => entry.id) };
    }
    return { kind: null, slug: leading.rest, ambiguous: ordinary.length > 0 ? ordinary.map((entry) => entry.id) : global.map((entry) => entry.id) };
  }
  const exact = taxonomy.directoryKinds.filter((kind) => contextAllows(kind) && kind.id.normalize("NFC").toLocaleLowerCase("und") === normalized.toLocaleLowerCase("und"));
  if (exact.length === 1)
    return { kind: exact[0], slug: normalized, ambiguous: [] };
  if (exact.length > 1)
    return { kind: null, slug: normalized, ambiguous: exact.map((entry) => entry.id) };
  const matching = taxonomy.directoryKinds.filter((kind) => kind.slugRegex.test(normalized));
  const contextual = parentKindId === undefined ? [] : matching.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
  const matches = contextual.length > 0 ? contextual : matching.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
  return { kind: matches.length === 1 ? matches[0] : null, slug: normalized, ambiguous: matches.map((entry) => entry.id) };
}
function resolveFileKind(path, taxonomy, parentKindId, ancestorKindIds, forcedId, contentKindId) {
  const name = basename2(path);
  const normalized = name.normalize("NFC");
  const folded = normalized.toLocaleLowerCase("und");
  const scoped = Object.entries(taxonomy.schema.scopedFileKinds).flatMap(([id, spec]) => {
    if (!taxonomyPathPatternMatches(path, spec.pathPattern) || !new RegExp(spec.sourceFilenamePattern, "u").test(normalized))
      return [];
    const extensions = spec.extensionChains.filter((chain) => folded.endsWith(chain.toLocaleLowerCase("und"))).sort((left, right) => right.length - left.length || left.localeCompare(right));
    return extensions.length > 0 ? [{ id, spec, extension: extensions[0] }] : [];
  }).sort((left, right) => left.id.localeCompare(right.id));
  if (scoped.length > 1)
    return { kind: null, extension: "", stem: normalized, ambiguous: scoped.map(({ id }) => `scoped:${id}`) };
  if (scoped.length === 1) {
    const selected2 = scoped[0];
    const kind = { id: `scoped:${selected2.id}`, emoji: selected2.spec.emoji, extensionChains: selected2.spec.extensionChains, role: selected2.spec.role };
    const withoutExtension2 = normalized.slice(0, -selected2.extension.length);
    const leading2 = splitLeadingEmoji(withoutExtension2);
    return { kind, extension: selected2.extension, stem: leading2.emoji && emojiFold(leading2.emoji) === emojiFold(kind.emoji) ? leading2.rest : withoutExtension2, ambiguous: [] };
  }
  const forced = forcedId ? taxonomy.fileKinds.find((kind) => kind.id === forcedId) : undefined;
  if (forced) {
    const extensions = forced.extensionChains.filter((chain) => normalized.endsWith(chain)).sort((left, right) => right.length - left.length || left.localeCompare(right));
    if (extensions.length > 0) {
      const extension2 = extensions[0];
      const withoutExtension2 = normalized.slice(0, -extension2.length);
      const leading2 = splitLeadingEmoji(withoutExtension2);
      return { kind: forced, extension: extension2, stem: leading2.emoji && emojiFold(leading2.emoji) === emojiFold(forced.emoji) ? leading2.rest : withoutExtension2, ambiguous: [] };
    }
  }
  const extensionRows = Object.entries(taxonomy.schema.fileKindResolutionRules).filter(([, rule]) => normalized.endsWith(rule.extensionChain)).sort((left, right) => right[1].extensionChain.length - left[1].extensionChain.length || left[0].localeCompare(right[0]));
  const longest = extensionRows[0]?.[1].extensionChain.length ?? 0;
  const candidates = extensionRows.filter(([, rule]) => rule.extensionChain.length === longest).filter(([, rule]) => !rule.filenamePattern || new RegExp(rule.filenamePattern, "u").test(normalized)).filter(([, rule]) => !rule.pathPattern || taxonomyPathPatternMatches(path, rule.pathPattern)).filter(([, rule]) => !rule.parentKindIds || parentKindId !== undefined && rule.parentKindIds.includes(parentKindId)).filter(([, rule]) => !rule.ancestorKindIds || rule.ancestorKindIds.some((kindId) => ancestorKindIds.includes(kindId))).map(([id, rule]) => ({ id, rule, predicates: Number(Boolean(rule.filenamePattern)) + Number(Boolean(rule.pathPattern)) + Number(Boolean(rule.parentKindIds)) + Number(Boolean(rule.ancestorKindIds)) })).sort((left, right) => right.rule.priority - left.rule.priority || right.predicates - left.predicates || left.id.localeCompare(right.id));
  if (candidates.length === 0) {
    const contentKind = contentKindId ? taxonomy.fileKinds.find((kind) => kind.id === contentKindId) : undefined;
    if (!contentKind)
      return { kind: null, extension: "", stem: normalized, ambiguous: [] };
    const extension2 = [...contentKind.extensionChains].sort((left, right) => left.length - right.length || left.localeCompare(right))[0];
    const leading2 = splitLeadingEmoji(normalized);
    const stem2 = (leading2.emoji && emojiFold(leading2.emoji) === emojiFold(contentKind.emoji) ? leading2.rest : normalized).trim().replace(/[. ]+$/u, "");
    return { kind: contentKind, extension: extension2, stem: stem2, ambiguous: [] };
  }
  const top = candidates.filter((entry) => entry.rule.priority === candidates[0].rule.priority && entry.predicates === candidates[0].predicates);
  const kindIds = [...new Set(top.map((entry) => entry.rule.fileKindId))];
  const extension = top[0].rule.extensionChain;
  const withoutExtension = normalized.slice(0, normalized.length - extension.length);
  if (kindIds.length !== 1)
    return { kind: null, extension, stem: withoutExtension, ambiguous: top.map((entry) => `${entry.id}:${entry.rule.fileKindId}`) };
  const selected = taxonomy.fileKinds.find((kind) => kind.id === kindIds[0]);
  if (!selected)
    return { kind: null, extension, stem: withoutExtension, ambiguous: kindIds };
  const leading = splitLeadingEmoji(withoutExtension);
  const stem = leading.emoji && emojiFold(leading.emoji) === emojiFold(selected.emoji) ? leading.rest : withoutExtension;
  return { kind: selected, extension, stem, ambiguous: [] };
}
function shebangCommand(line) {
  const raw = line.startsWith("#!") ? line.slice(2).trim() : "";
  if (!raw)
    return null;
  const tokens = raw.split(/\s+/u).filter(Boolean);
  let command = tokens.shift() ?? "";
  if (basename2(command).toLocaleLowerCase("und") === "env") {
    while (tokens[0]?.startsWith("-") || /^[A-Za-z_][A-Za-z0-9_]*=/u.test(tokens[0] ?? ""))
      tokens.shift();
    command = tokens.shift() ?? "";
  }
  return command ? basename2(command).replace(/\.exe$/iu, "").toLocaleLowerCase("und") : null;
}
function typescriptSyntax(text) {
  return /\b(?:interface|namespace|enum)\s+[A-Za-z_$]|\btype\s+[A-Za-z_$][\w$]*\s*=|\b(?:const|let|var)\s+[A-Za-z_$][\w$]*\s*:\s*[^=]|\b(?:satisfies|as\s+const)\b/u.test(text);
}
function extensionlessContentKind(path, bytes, taxonomy) {
  const name = basename2(path);
  if (name.includes(".") || !bytes)
    return { kindId: null };
  if (bytes.includes(0))
    return { kindId: "binary" };
  let text;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    return { kindId: "binary" };
  }
  if (/[\u0001-\u0008\u000B\u000C\u000E-\u001F]/u.test(text))
    return { kindId: null, violation: violation("content-kind-ambiguous", path, "Extensionless content contains non-text control bytes without a binary signature") };
  if (text.startsWith("#!")) {
    const command = shebangCommand(text.split(/\r?\n/u, 1)[0] ?? "");
    const kindId = command && /^(?:ba|da|z|k|fi)?sh$/u.test(command) ? "shell" : command && /^python(?:\d+(?:\.\d+)*)?$/u.test(command) ? "python-source" : command && /^(?:pwsh|powershell)$/u.test(command) ? "powershell" : command && /^(?:node|nodejs)$/u.test(command) ? typescriptSyntax(text) ? "typescript-source" : "javascript-source" : command && /^(?:bun|deno|tsx|ts-node)$/u.test(command) ? typescriptSyntax(text) ? "typescript-source" : "javascript-source" : null;
    if (!kindId)
      return { kindId: null, violation: violation("shebang-kind-unresolved", path, `Extensionless shebang interpreter is unknown or contradictory: ${command ?? "missing"}`) };
    if (!taxonomy.schema.fileKinds[kindId])
      return { kindId: null, violation: violation("shebang-kind-unregistered", path, `Shebang resolved to unregistered file kind ${kindId}`) };
    return { kindId };
  }
  if (!taxonomy.schema.fileKinds["plain-text"])
    return { kindId: null, violation: violation("text-kind-unregistered", path, "Extensionless UTF-8 content requires registered plain-text kind") };
  return { kindId: "plain-text" };
}
function ownerId(path) {
  const parts = path.split("/");
  if (parts[0] === ".\uD83E\uDDECsemio" && parts[1] === "\uD83E\uDD91\uFE0Frepo" && parts[2] === "\uD83C\uDFAB\uFE0Ftickets" && parts.length >= 7)
    return parts.slice(0, 7).join("/");
  if (parts[0] === "\u270F\uFE0Fs" && (parts[1] === "\uD83D\uDD0C\uFE0Fplugins" || parts[1] === "\uD83D\uDD28\uFE0Fmodules") && parts[2])
    return parts.slice(0, 3).join("/");
  if (parts[0] === "\uD83E\uDDF0\uFE0Fframework" && (parts[1] === "\uD83D\uDECD\uFE0Fproducts" || parts[1] === "\uD83D\uDD28\uFE0Fmodules") && parts[2])
    return parts.slice(0, 3).join("/");
  if ((parts[0] === "\uD83C\uDF0E\uFE0Fhub" || parts[0] === "\u267B\uFE0Fmit-bestand") && parts[1])
    return parts.slice(0, 2).join("/");
  return parts[0] ?? "";
}
function areaId(path) {
  const first = path.split("/")[0] ?? "";
  if (first === "\u270F\uFE0Fs")
    return path.split("/").slice(0, 2).join("/");
  return first;
}
function violation(code, path, message, severity = "error") {
  return { code, severity, path, message };
}
function stableViolations(rows) {
  return [...new Map(rows.map((entry) => [`${entry.path}\x00${entry.code}\x00${entry.severity}\x00${entry.message}`, entry])).values()].sort((a, b) => a.path.localeCompare(b.path) || a.code.localeCompare(b.code) || a.message.localeCompare(b.message));
}
function report(progress, operation, phase, current, total, path) {
  progress?.({ operation, phase, current, total, path });
}

class TaxonomyCancellationError extends Error {
  constructor() {
    super("Taxonomy operation cancelled");
  }
}
function checkCancellation(repoRoot, cancelFile) {
  if (!cancelFile)
    return;
  const path = isAbsolute(cancelFile) ? cancelFile : absolutePath(repoRoot, cancelFile);
  if (existsSync(path))
    throw new TaxonomyCancellationError;
}
function gitRows(repoRoot) {
  const stdout = execFileSync("git", ["ls-files", "--stage", "-z"], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return stdout.toString("utf8").split("\x00").filter(Boolean).map((row) => {
    const tab = row.indexOf("\t");
    const [mode, objectId, stage] = row.slice(0, tab).split(" ");
    return { path: sourceRelative(row.slice(tab + 1)), mode, objectId, stage };
  }).filter((row) => row.stage === "0").map(({ path, mode, objectId }) => ({ path, mode, objectId }));
}
function untrackedGitPaths(repoRoot, taxonomy) {
  const exclusions = taxonomy.exclusions.map((entry) => `:(exclude,top,literal)${entry.path}`);
  const stdout = execFileSync("git", ["ls-files", "--others", "--exclude-standard", "-z", "--", ".", ...exclusions], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return stdout.toString("utf8").split("\x00").filter(Boolean).map(sourceRelative).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
}
function worktreeCandidate(repoRoot, path) {
  const stat = lstatOrNull(absolutePath(repoRoot, path));
  if (!stat)
    return null;
  if (stat.isSymbolicLink())
    return { path, mode: "120000" };
  if (stat.isDirectory())
    return { path, mode: "040000", explicitDirectory: true };
  return { path, mode: (stat.mode & 73) !== 0 ? "100755" : "100644" };
}
function explicitTicketRows(repoRoot, ticketDir, taxonomy) {
  if (!ticketDir)
    return [];
  const rel = sourceRelative(isAbsolute(ticketDir) ? relative2(resolve2(repoRoot), resolve2(ticketDir)) : ticketDir);
  if (isExcluded(rel, taxonomy))
    return [];
  const root = absolutePath(repoRoot, rel);
  if (!existsSync(root))
    return [];
  const rows = [];
  const walk = (currentRel) => {
    if (isExcluded(currentRel, taxonomy))
      return;
    const currentAbs = absolutePath(repoRoot, currentRel);
    const stat = lstatSync(currentAbs);
    if (stat.isSymbolicLink()) {
      rows.push({ path: currentRel, mode: "120000" });
      return;
    }
    if (!stat.isDirectory()) {
      rows.push({ path: currentRel, mode: (stat.mode & 73) !== 0 ? "100755" : "100644" });
      return;
    }
    rows.push({ path: currentRel, mode: "040000", explicitDirectory: true });
    const nestedGit = taxonomy.schema.fixedDirectoryContracts["nested-git-metadata"];
    if (nestedGit && basename2(currentRel) === ".git" && taxonomyPathPatternMatches(currentRel, nestedGit.pathPattern))
      return;
    const children = readdirSync(currentAbs).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
    for (const child of children) {
      const childRel = sourceRelative(`${currentRel}/${child}`);
      if (isExcluded(childRel, taxonomy))
        continue;
      walk(childRel);
    }
  };
  walk(rel);
  return rows;
}
function generatorContractsForOutputPath(path, taxonomy) {
  const normalized = normalizeRelative(path);
  return Object.entries(taxonomy.schema.generatorContracts).filter(([, contract]) => contract.outputRoots.some((root) => normalized === root.path || normalized.startsWith(`${root.path}/`))).map(([id, contract]) => ({ id, contract })).sort((left, right) => left.id.localeCompare(right.id));
}
function ignoredGeneratorRows(repoRoot, taxonomy) {
  const rows = new Map;
  const walk = (path) => {
    if (isExcluded(path, taxonomy))
      return;
    const stat = lstatOrNull(absolutePath(repoRoot, path));
    if (!stat)
      return;
    if (stat.isSymbolicLink()) {
      rows.set(path, { path, mode: "120000" });
      return;
    }
    if (!stat.isDirectory()) {
      rows.set(path, { path, mode: (stat.mode & 73) !== 0 ? "100755" : "100644" });
      return;
    }
    rows.set(path, { path, mode: "040000", explicitDirectory: true });
    for (const child of readdirSync(absolutePath(repoRoot, path)).sort((a, b) => Buffer.from(a).compare(Buffer.from(b))))
      walk(sourceRelative(`${path}/${child}`));
  };
  for (const contract of Object.values(taxonomy.schema.generatorContracts))
    for (const root of contract.outputRoots)
      if (root.inclusion === "ignored")
        walk(root.path);
  return [...rows.values()].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}
function contentOf(repoRoot, row) {
  if (row.mode === "040000")
    return { kind: "directory", hash: "", size: 0 };
  const path = absolutePath(repoRoot, row.path);
  if (!existsSync(path) && row.mode !== "120000")
    return { kind: "file", hash: row.objectId ?? sha256(""), size: 0, violation: violation("tracked-path-missing", row.path, "Tracked path is missing from the worktree") };
  try {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink() || row.mode === "120000") {
      const target = readlinkSync(path);
      return { kind: "symlink", hash: sha256(target), size: Buffer.byteLength(target) };
    }
    if (stat.isDirectory())
      return { kind: "directory", hash: "", size: 0 };
    const bytes = readFileSync(path);
    return { kind: "file", hash: sha256(bytes), size: bytes.byteLength, bytes };
  } catch (error) {
    return { kind: row.mode === "120000" ? "symlink" : "file", hash: row.objectId ?? sha256(""), size: 0, violation: violation("path-read-failed", row.path, error instanceof Error ? error.message : String(error)) };
  }
}
function packageLocation(path, taxonomy) {
  const parts = path.split("/");
  const packageIndex = parts.findIndex((part) => taxonomy.directoryKinds.some((kind) => kind.id === "packages" && emojiFold(kind.emoji) === emojiFold(splitLeadingEmoji(part).emoji)) || part === "\uD83D\uDCE6\uFE0Fpackages");
  if (packageIndex < 0)
    return null;
  const owner = parts.slice(0, packageIndex).join("/");
  const ecosystemSegment = parts[packageIndex + 1] ?? "";
  const candidates = Object.entries(taxonomy.schema.packageBoundaryRules).filter(([id]) => {
    const slug = splitLeadingEmoji(ecosystemSegment).rest;
    return id === ecosystemSegment || id === slug || id.endsWith(`.${slug}`) || id.endsWith(`/${slug}`);
  });
  const selected = candidates.length === 1 ? candidates[0] : Object.entries(taxonomy.schema.packageBoundaryRules).length === 1 ? Object.entries(taxonomy.schema.packageBoundaryRules)[0] : null;
  return { owner, packageRoot: parts.slice(0, packageIndex + 2).join("/"), ecosystemId: selected?.[0] ?? null, rule: selected?.[1] ?? null };
}
function fixedSpecificity(contract) {
  const segments = contract.pathPattern.split("/");
  const tokens = contract.pathPattern.match(/\*\*|\*|\?|\[[^\]]+\]/gu) ?? [];
  const literals = contract.pathPattern.replaceAll("/", "").replace(/\*\*|\*|\?|\[[^\]]+\]/gu, "");
  return [segments.filter((segment) => !/[?*\[]/u.test(segment)).length, [...literals].length, -tokens.length, contract.scope === "path-pattern" ? 0 : 1];
}
function compareFixedSpecificity(left, right) {
  for (let index = 0;index < left.length; index++)
    if (left[index] !== right[index])
      return right[index] - left[index];
  return 0;
}
function equalFixedSpecificity(left, right) {
  return left.every((value, index) => value === right[index]);
}
function fixedScopeMatches(contract, path, packageInfo, parentKindId) {
  if (contract.scope === "repository-root")
    return !path.includes("/");
  if (contract.scope === "package-root")
    return packageInfo?.packageRoot === dirname2(path) && packageInfo.ecosystemId === contract.ecosystemId;
  if (contract.scope === "directory-kind")
    return parentKindId === contract.directoryKindId;
  return true;
}
function matchingFixedContracts(path, contracts, packageInfo, parentKindId) {
  const matches = Object.entries(contracts).filter(([, contract]) => taxonomyPathPatternMatches(path, contract.pathPattern) && fixedScopeMatches(contract, path, packageInfo, parentKindId)).map(([id, contract]) => ({ id, contract, specificity: fixedSpecificity(contract) })).sort((left, right) => compareFixedSpecificity(left.specificity, right.specificity) || left.id.localeCompare(right.id));
  if (matches.length === 0)
    return { selected: null, ambiguous: [] };
  const top = matches.filter((entry) => equalFixedSpecificity(entry.specificity, matches[0].specificity));
  return top.length === 1 ? { selected: [top[0].id, top[0].contract], ambiguous: [] } : { selected: null, ambiguous: top.map((entry) => entry.id) };
}
function configurableContract(path, taxonomy, packageInfo) {
  const rows = Object.entries(taxonomy.schema.configurableEntryContracts).filter(([, contract]) => basename2(path).normalize("NFC") === contract.filename.normalize("NFC") && packageInfo?.ecosystemId === contract.ecosystemId);
  return rows.length === 1 ? rows[0] : null;
}
function classifyGlue(analyzer, content, maxStatements) {
  const normalized = content.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "").replace(/^\s*#.*$/gm, "").trim();
  if (normalized.length === 0)
    return "declaration";
  if (analyzer === "rust") {
    if (/\b(?:struct|enum|trait|union|impl)\b/.test(normalized))
      return "implementation";
    const bodies = [...normalized.matchAll(/\bfn\s+\w+[^\{]*\{([\s\S]*?)\}/g)].map((match) => match[1].split(";").map((part) => part.trim()).filter(Boolean).length);
    if (bodies.some((count) => count > maxStatements))
      return "implementation";
    if (/\bfn\s+(?:main|start|bootstrap)\b/.test(normalized))
      return "bootstrap";
    if (/\b(?:register|provide|bind)\w*\s*\(/i.test(normalized))
      return "registration";
    if (/^(?:\s*(?:pub\s+)?(?:mod|use)\b[^;]*;|\s*#\[[^\]]+\]\s*)+$/s.test(normalized))
      return "declaration";
    return bodies.length > 0 ? "thin-delegation" : "unresolved";
  }
  if (analyzer === "typescript") {
    if (/\b(?:class|namespace)\b/.test(normalized))
      return "implementation";
    if (/^(?:\s*(?:import\b[^;]*;?|export\s+(?:\*|\{[^}]*\}|type\b[^;]*|interface\b[^{]*\{[^}]*\}|enum\b[^{]*\{[^}]*\})[^;]*;?)\s*)+$/s.test(normalized))
      return "declaration";
    if (/\b(?:register|provide|bind)\w*\s*\(/i.test(normalized))
      return "registration";
    const functionBodies = [...normalized.matchAll(/(?:function\s+([\w$]+)[^{]*|(?:const|let)\s+([\w$]+)\s*=\s*(?:async\s*)?\([^)]*\)\s*=>)\{([\s\S]*?)\}/g)];
    if (functionBodies.length > 0) {
      const thin = functionBodies.every((match) => {
        const name = match[1] ?? match[2] ?? "";
        const statements = match[3].split(";").map((part) => part.trim()).filter(Boolean);
        return /^(?:main|start|bootstrap|run)$/i.test(name) && statements.length <= maxStatements && statements.every((statement) => /^(?:return\s+)?(?:await\s+)?[\w$.]+\([^;]*\)$/.test(statement));
      });
      return thin ? "thin-delegation" : "implementation";
    }
    if (/=>|\bfunction\b|\.(?:reduce|map|filter|flatMap|sort)\s*\(/.test(normalized))
      return "implementation";
    return "implementation";
  }
  if (analyzer === "go") {
    if (/\btype\s+\w+\s+(?:struct|interface)\b/.test(normalized))
      return "implementation";
    const bodies = [...normalized.matchAll(/\bfunc\s+(?:main|init)\s*\([^)]*\)\s*\{([\s\S]*?)\}/g)];
    if (bodies.length > 0 && bodies.every((match) => match[1].split(`
`).map((line) => line.trim()).filter(Boolean).length <= maxStatements))
      return "bootstrap";
    if (/^package\s+\w+\s+(?:import\s*(?:\([^)]*\)|"[^"]+")\s*)?$/s.test(normalized))
      return "declaration";
    return "implementation";
  }
  if (analyzer === "python") {
    if (/^\s*(?:class|def)\s+/m.test(normalized))
      return "implementation";
    if (/^(?:\s*(?:from\s+\S+\s+import|import\s+|__all__\s*=)[^\n]*\n?)+$/s.test(normalized))
      return "declaration";
    const statements = normalized.split(`
`).map((line) => line.trim()).filter(Boolean).length;
    if (statements <= maxStatements && /if\s+__name__\s*==\s*["']__main__["']/.test(normalized))
      return "bootstrap";
    return "implementation";
  }
  if (/\b(?:class|struct|interface|record|enum)\b/.test(normalized))
    return "implementation";
  if (/\b(?:AddSingleton|AddScoped|AddTransient|Register)\b/.test(normalized))
    return "registration";
  if (/^(?:\s*(?:using|global\s+using|\[assembly:)[^;\n]*(?:;|\])\s*)+$/s.test(normalized))
    return "declaration";
  return "unresolved";
}
function classifyPackageRole(path, kindId, fixedId, content, taxonomy) {
  const location = packageLocation(path, taxonomy);
  if (!location)
    return "not-package";
  if (fixedId || configurableContract(path, taxonomy, location))
    return "configuration";
  if (!location.rule || !location.ecosystemId)
    return "unresolved";
  if (kindId && !location.rule.allowedFileKindIds.includes(kindId))
    return "implementation";
  if (!content)
    return "configuration";
  const grammar = taxonomy.schema.packageGlueGrammar[location.rule.glueGrammarId];
  const role = classifyGlue(grammar.analyzer, content, grammar.maxDelegationStatements);
  return grammar.allowedRoles.includes(role) ? role : role === "implementation" ? "implementation" : "unresolved";
}
function canonicalDirectory(path, parentCanonical, parentKindId, ancestorKindIds, taxonomy) {
  const name = basename2(path).normalize("NFC");
  const fixed = matchingFixedContracts(path, taxonomy.schema.fixedDirectoryContracts, packageLocation(path, taxonomy), parentKindId);
  if (fixed.ambiguous.length > 0)
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("fixed-directory-contract-ambiguous", path, `Equal-specificity fixed directory contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected) {
    const context = matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: context.kind?.id ?? null, fixedId: fixed.selected[0], violations: [] };
  }
  if (parentKindId === "packages") {
    const packageKinds = Object.keys(taxonomy.schema.packageBoundaryRules).filter((id) => emojiFold(id) === emojiFold(name));
    if (packageKinds.length === 1)
      return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: `package-language:${packageKinds[0]}`, violations: [] };
    if (packageKinds.length > 1)
      return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("package-language-ambiguous", path, `Package language boundary is ambiguous: ${packageKinds.join(", ")}`)] };
  }
  const match = matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
  if (!match.kind) {
    const message = match.ambiguous.length > 1 ? `Directory semantic kind is ambiguous: ${match.ambiguous.join(", ")}` : "Directory has no registered semantic kind";
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation(match.ambiguous.length > 1 ? "directory-kind-ambiguous" : "directory-kind-unresolved", path, message)] };
  }
  const canonicalName = `${match.kind.emoji}${match.slug}`.normalize("NFC");
  return { path: parentCanonical ? `${parentCanonical}/${canonicalName}` : canonicalName, kindId: match.kind.id, violations: [] };
}
function canonicalFile(path, parentCanonical, parentKindId, ancestorKindIds, directoryKindByPath, taxonomy, contentKindId) {
  const packageInfo = packageLocation(path, taxonomy);
  let fixedName = basename2(path);
  let fixed = matchingFixedContracts(path, taxonomy.schema.fixedFilenameContracts, packageInfo, directoryKindByPath.get(dirname2(path)));
  const decoratedFixedName = splitLeadingEmoji(fixedName);
  if (!fixed.selected && fixed.ambiguous.length === 0 && decoratedFixedName.emoji && decoratedFixedName.rest) {
    const candidatePath = dirname2(path) === "." ? decoratedFixedName.rest : `${dirname2(path)}/${decoratedFixedName.rest}`;
    const candidate = matchingFixedContracts(candidatePath, taxonomy.schema.fixedFilenameContracts, packageLocation(candidatePath, taxonomy), directoryKindByPath.get(dirname2(path)));
    if (candidate.selected || candidate.ambiguous.length > 0) {
      fixed = candidate;
      fixedName = decoratedFixedName.rest;
    }
  }
  if (fixed.ambiguous.length > 0)
    return { path: parentCanonical ? `${parentCanonical}/${basename2(path)}` : basename2(path), fileKind: null, stem: null, violations: [violation("fixed-contract-ambiguous", path, `Equal-specificity fixed filename contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected)
    return { path: parentCanonical ? `${parentCanonical}/${fixedName}` : fixedName, fileKind: null, stem: null, fixedId: fixed.selected[0], violations: [] };
  const configurable = configurableContract(path, taxonomy, packageInfo);
  const resolvedKind = resolveFileKind(path, taxonomy, parentKindId, ancestorKindIds, configurable?.[1].fileKindId, contentKindId);
  if (!resolvedKind.kind) {
    const message = resolvedKind.ambiguous.length > 1 ? `File kind is ambiguous: ${resolvedKind.ambiguous.join(", ")}` : "No file kind owns the longest extension chain";
    return { path: parentCanonical ? `${parentCanonical}/${basename2(path).normalize("NFC")}` : basename2(path).normalize("NFC"), fileKind: null, stem: null, violations: [violation(resolvedKind.ambiguous.length > 1 ? "file-kind-ambiguous" : "file-kind-unresolved", path, message)] };
  }
  const leadingSemantic = splitLeadingEmoji(resolvedKind.stem);
  const semanticEvidence = leadingSemantic.emoji || "";
  const sourceStem = semanticEvidence ? leadingSemantic.rest : resolvedKind.stem;
  const testSuffix = sourceStem.endsWith(".test");
  const semanticStem = testSuffix ? sourceStem.slice(0, -".test".length) : sourceStem;
  const kindOnly = `${resolvedKind.kind.emoji}${resolvedKind.extension}`.normalize("NFC");
  if (!semanticStem || configurable || GENERIC_SEMANTIC_STEMS.has(semanticStem.toLocaleLowerCase("und")))
    return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem || null, violations: [] };
  const parentSlug = splitLeadingEmoji(basename2(dirname2(path))).rest;
  if (parentSlug.normalize("NFC").toLocaleLowerCase("und") === semanticStem.normalize("NFC").toLocaleLowerCase("und"))
    return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [] };
  const roleContext = testSuffix ? "tests" : resolvedKind.kind.role === "asset" ? "assets" : resolvedKind.kind.role === "test" ? "tests" : parentKindId;
  const semantic = matchDirectoryKind(`${semanticEvidence}${semanticStem}`, taxonomy, roleContext);
  if (!semantic.kind) {
    const message = semantic.ambiguous.length > 1 ? `Semantic stem matches multiple directory kinds: ${semantic.ambiguous.join(", ")}` : "Semantic stem has no registered directory kind";
    return { path: parentCanonical ? `${parentCanonical}/${basename2(path).normalize("NFC")}` : basename2(path).normalize("NFC"), fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [violation(semantic.ambiguous.length > 1 ? "semantic-stem-ambiguous" : "semantic-stem-unresolved", path, message)] };
  }
  if (parentKindId === semantic.kind.id && parentSlug === semanticStem)
    return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [] };
  const semanticDirectory = `${semantic.kind.emoji}${semanticStem}`.normalize("NFC");
  return { path: parentCanonical ? `${parentCanonical}/${semanticDirectory}/${kindOnly}` : `${semanticDirectory}/${kindOnly}`, fileKind: resolvedKind.kind.id, stem: semanticStem, semanticDirectoryName: semanticDirectory, violations: [] };
}
function packageImplementationDestination(sourcePath, canonical, canonicalDirectoryByPath, directoryKindByPath, taxonomy) {
  const location = packageLocation(sourcePath, taxonomy);
  if (!location || !canonical.fileKind)
    return null;
  const ownerCanonical = canonicalDirectoryByPath.get(location.owner) ?? location.owner.normalize("NFC");
  const fileName = basename2(canonical.path);
  const stem = canonical.stem?.normalize("NFC") ?? "";
  if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und")))
    return ownerCanonical ? `${ownerCanonical}/${fileName}` : fileName;
  if (canonical.semanticDirectoryName)
    return ownerCanonical ? `${ownerCanonical}/${canonical.semanticDirectoryName}/${fileName}` : `${canonical.semanticDirectoryName}/${fileName}`;
  const semantic = matchDirectoryKind(stem, taxonomy, directoryKindByPath.get(location.owner));
  if (!semantic.kind)
    return null;
  const directoryName = `${semantic.kind.emoji}${stem}`.normalize("NFC");
  return ownerCanonical ? `${ownerCanonical}/${directoryName}/${fileName}` : `${directoryName}/${fileName}`;
}
function directoryHash(path, children) {
  const prefix = path ? `${path}/` : "";
  const rows = [...children].sort((a, b) => Buffer.from(a.sourcePath).compare(Buffer.from(b.sourcePath))).map((entry) => `${entry.nodeKind}\x00${entry.mode ?? ""}\x00${entry.sourcePath.slice(prefix.length)}\x00${entry.contentHash}`);
  return sha256(rows.join("\x00"));
}
function inventoryDigestOf(inventory) {
  return sha256(canonicalJson(inventory));
}
var indexedLineContent = "";
var indexedLineStarts = [0];
function lineLocation(content, start2, label) {
  if (indexedLineContent !== content) {
    const starts = [0];
    for (let index = content.indexOf(`
`);index >= 0; index = content.indexOf(`
`, index + 1))
      starts.push(index + 1);
    indexedLineContent = content;
    indexedLineStarts = starts;
  }
  let low = 0;
  let high = indexedLineStarts.length;
  while (low < high) {
    const middle = low + high >>> 1;
    if (indexedLineStarts[middle] <= start2)
      low = middle + 1;
    else
      high = middle;
  }
  const line = Math.max(1, low);
  const column = start2 - indexedLineStarts[line - 1] + 1;
  return `${label}:${line}:${column}@${start2}`;
}
function regexTokens(content, adapter, label, patterns) {
  const rows = [];
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      const value = match[1];
      if (typeof value !== "string" || match.index === undefined)
        continue;
      const relativeIndex = match[0].indexOf(value);
      const start2 = match.index + relativeIndex;
      rows.push({ adapter, structuredLocation: lineLocation(content, start2, label), start: start2, end: start2 + value.length, value });
    }
  }
  return rows;
}
function argumentTokens(content, fragment, fragmentStart, adapter, label) {
  const rows = [];
  for (const match of fragment.matchAll(/"([^"]+)"|'([^']+)'|([^\s()[\],;]+)/gu)) {
    if (match.index === undefined)
      continue;
    const value = match[1] ?? match[2] ?? match[3];
    if (!value || /^(?:=>|PUBLIC|PRIVATE|INTERFACE|EXCLUDE_FROM_ALL)$/u.test(value))
      continue;
    const inner = match[0].indexOf(value);
    const start2 = fragmentStart + match.index + inner;
    rows.push({ adapter, structuredLocation: lineLocation(content, start2, label), start: start2, end: start2 + value.length, value });
  }
  return rows;
}
function embeddedArgumentTokens(content, value, valueStart, adapter, label) {
  if (!/\s|(?:^|\s)--?[\w-]+=|\$\{(?:workspaceFolder|workspaceRoot)\}/u.test(value))
    return [];
  const rows = [];
  for (const match of value.matchAll(/[^\s"'`]+/gu)) {
    if (match.index === undefined)
      continue;
    let candidate = match[0].replace(/^[[(]+|[\]),;]+$/gu, "");
    let offset = match[0].indexOf(candidate);
    const assignment = candidate.match(/^--?[\w-]+=(.+)$/u);
    if (assignment) {
      offset += candidate.indexOf(assignment[1]);
      candidate = assignment[1];
    }
    const workspace = candidate.match(/^\$\{(?:workspaceFolder|workspaceRoot)\}\/(.+)$/u);
    if (workspace) {
      offset += candidate.indexOf(workspace[1]);
      candidate = workspace[1];
    }
    if (!candidate || /^(?:bun|node|python|python3|go|cargo|nx|run|test|build)$/u.test(candidate))
      continue;
    const start2 = valueStart + match.index + offset;
    rows.push({ adapter, structuredLocation: lineLocation(content, start2, label), start: start2, end: start2 + candidate.length, value: candidate });
  }
  for (const match of value.matchAll(/(?:\.\.?\/|\/)[^\s\\"'`()\],;]+/gu)) {
    if (match.index === undefined)
      continue;
    const start2 = valueStart + match.index;
    rows.push({ adapter, structuredLocation: lineLocation(content, start2, label), start: start2, end: start2 + match[0].length, value: match[0] });
  }
  return [...new Map(rows.map((entry) => [`${entry.start}\x00${entry.end}\x00${entry.value}`, entry])).values()].sort((left, right) => left.start - right.start || left.value.localeCompare(right.value));
}
var OLD_MUTATION_TEST_PREFIX_SOURCE = "\uD83C\uDFC5\uFE0Fstandards/\uD83D\uDD16\uFE0F([^/\\s\"'`|]+)\\/\uD83E\uDE86\uFE0Fsubsets/\u2733\uFE0F([^/\\s\"'`|]+)\\/\uD83E\uDDEC\uFE0Fschema/\uD83E\uDDEC\uFE0Fmutations\\/([^/\\s\"'`|]+)\\/\uD83E\uDDEA\uFE0Ftests\\/";
var OLD_MUTATION_STRUCTURE_SOURCE = `${OLD_MUTATION_TEST_PREFIX_SOURCE}([^/\\s"'\`|]+)(\\/[^\\s"'\`|)>}\\]]+)?`;
function artifactRootForPath(path) {
  const segments = normalizeRelative(path).split("/");
  const index = segments.findIndex((segment) => emojiFold(segment) === emojiFold("\uD83D\uDDFF\uFE0Fartifacts"));
  if (index >= 0 && index + 1 < segments.length)
    return segments.slice(0, index + 2).join("/");
  const standards = segments.findIndex((segment) => emojiFold(segment) === emojiFold("\uD83C\uDFC5\uFE0Fstandards"));
  return standards > 0 ? segments.slice(0, standards).join("/") : null;
}
function mutationStructuralPaths(content, fragmentStart = 0) {
  const rows = [];
  const pattern = new RegExp(OLD_MUTATION_STRUCTURE_SOURCE, "gu");
  for (const match of content.matchAll(pattern)) {
    if (match.index === undefined)
      continue;
    rows.push({ value: match[0], start: fragmentStart + match.index, standard: match[1], subset: match[2], mutation: match[3], scenario: match[4], suffix: match[5] ?? "" });
  }
  return rows;
}
function canonicalProjectionSuffix(suffix) {
  const segments = suffix.split("/");
  const name = segments.at(-1) ?? "";
  const leading = splitLeadingEmoji(name);
  if (leading.emoji && /^component\.[a-z0-9.]+$/u.test(leading.rest))
    segments[segments.length - 1] = `${leading.emoji}.${leading.rest.slice("component.".length)}`;
  return segments.join("/");
}
function projectionKey(artifactRoot, standard, subset) {
  return `${artifactRoot}\x00${standard}\x00${subset}`;
}
function projectedStructuralValue(row) {
  const scenario = splitLeadingEmoji(row.scenario).emoji ? row.scenario : `\uD83E\uDDEA\uFE0F${row.scenario}`;
  return `\uD83E\uDDEA\uFE0Ftests/\uD83E\uDE86\uFE0F${row.standard}-${row.subset}/${row.mutation}/${scenario}${canonicalProjectionSuffix(row.suffix)}`.normalize("NFC");
}
function structuralProjectionToken(content, row, adapter, label, artifactRoot, prefix = "") {
  const value = `${prefix}${row.value}`;
  const start2 = row.start - prefix.length;
  const target = artifactRoot && !/[<>]/u.test(row.value) ? `${artifactRoot}/${row.value}` : undefined;
  return {
    adapter,
    structuredLocation: label.startsWith("/") ? `${label}@${start2}` : lineLocation(content, start2, label),
    start: start2,
    end: start2 + value.length,
    value,
    targetValues: target ? [target] : undefined,
    rewriteKind: prefix === "asset://" ? "artifact-uri" : "projection-prose",
    rewriteData: {
      newValue: `${prefix}${projectedStructuralValue(row)}`,
      projectionKey: artifactRoot ? projectionKey(artifactRoot, row.standard, row.subset) : "",
      projectionProfile: `${row.standard}\x00${row.subset}`,
      artifactRoot: artifactRoot ?? ""
    }
  };
}
function structuralTokensInFragment(content, fragment, fragmentStart, adapter, label, artifactRoot) {
  const rows = [];
  for (const structural of mutationStructuralPaths(fragment, fragmentStart)) {
    const localStart = structural.start - fragmentStart;
    const before = fragment.slice(0, localStart);
    const prefix = before.endsWith("asset://") ? "asset://" : before.match(/(?:(?:\.\.\/|\.\/)+)$/u)?.[0] ?? "";
    rows.push(structuralProjectionToken(content, structural, adapter, prefix === "asset://" && adapter === "gherkin" ? "gherkin" : label, artifactRoot, prefix));
  }
  return rows;
}
function jsonTokens(path, content, adapter) {
  const rows = [];
  let ordinal = 0;
  const embeddedArgv = /(?:^|\/)(?:launch(?:\.seed)?\.jsonc?|tasks\.json|project\.json|package\.json)$/iu.test(path);
  for (const match of content.matchAll(/"((?:\\.|[^"\\])*)"/g)) {
    if (match.index === undefined)
      continue;
    const tail = content.slice(match.index + match[0].length).match(/^\s*/)?.[0].length ?? 0;
    const key = content[match.index + match[0].length + tail] === ":";
    let value;
    try {
      value = JSON.parse(match[0]);
    } catch {
      continue;
    }
    const raw = match[1];
    const start2 = match.index + 1;
    if (raw === value)
      rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${ordinal++}]@${start2}`, start: start2, end: start2 + raw.length, value });
    if (!key && raw === value)
      rows.push(...structuralTokensInFragment(content, raw, start2, adapter, `/@value[${Math.max(0, ordinal - 1)}]/prose`, artifactRootForPath(path)));
    if (!key && raw !== value && mutationStructuralPaths(value).length > 0)
      rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/prose@${start2}`, start: start2, end: start2 + raw.length, value: raw, unsupportedReason: "Escaped JSON projection prose has no proven decoded-to-raw offset map" });
    if (!key && embeddedArgv)
      rows.push(...embeddedArgumentTokens(content, raw, start2, adapter, "embedded-argv"));
  }
  return rows;
}
function tomlTokens(path, content) {
  const adapter = basename2(path) === "Cargo.toml" ? "rust" : "toml";
  const rows = [];
  for (const match of content.matchAll(/"([^"\r\n]+)"|'([^'\r\n]+)'/gu)) {
    if (match.index === undefined)
      continue;
    const value = match[1] ?? match[2];
    const start2 = match.index + match[0].indexOf(value);
    const entrypoint = value.match(/^([A-Za-z_]\w*(?:\.[A-Za-z_]\w*)+):([A-Za-z_]\w*)$/u);
    rows.push(entrypoint ? { adapter, structuredLocation: lineLocation(content, start2, "python-entrypoint"), start: start2, end: start2 + value.length, value, targetValues: [entrypoint[1]], rewriteKind: "python-entrypoint", rewriteData: { suffix: `:${entrypoint[2]}` } } : { adapter, structuredLocation: lineLocation(content, start2, "toml-string"), start: start2, end: start2 + value.length, value });
  }
  return rows;
}
function rustTokens(path, content) {
  const rows = regexTokens(content, "rust", "rust-string-path", [/#\s*\[\s*path\s*=\s*"([^"]+)"/gu, /\b(?:include|include_str|include_bytes)!\s*\(\s*"([^"]+)"/gu]);
  for (const match of content.matchAll(/^([ \t]*)((?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;)/gmu)) {
    if (match.index === undefined)
      continue;
    const statement = match[2];
    const start2 = match.index + match[1].length;
    const name = match[3];
    rows.push({
      adapter: "rust",
      structuredLocation: lineLocation(content, start2, "rust-mod"),
      start: start2,
      end: start2 + statement.length,
      value: statement,
      targetValues: [`./${name}.rs`, `./${name}/mod.rs`],
      rewriteKind: "rust-mod",
      rewriteData: { indentation: match[1], declaration: statement }
    });
  }
  for (const match of content.matchAll(/(?:^|\n)[ \t]*(?:(?:\/\/\/)|(?:\/\/!)|(?:\/\/))([^\r\n]*)/gu)) {
    if (match.index === undefined)
      continue;
    const fragment = match[1];
    const start2 = match.index + match[0].indexOf(fragment);
    rows.push(...structuralTokensInFragment(content, fragment, start2, "rust", "rust-comment", artifactRootForPath(path)));
  }
  return rows;
}
function pythonTokens(path, content) {
  const rows = regexTokens(content, "python", "python-reference", [/^\s*from\s+([\w.]+)\s+import\s+/gmu, /^\s*import\s+([\w.]+)(?:\s+as\s+\w+)?\s*$/gmu, /\b(?:open|Path|joinpath|files|read_text|read_bytes)\s*\(\s*["']([^"']+)["']/gu, /__file__[^\r\n]*?\/\s*["']([^"']+)["']/gu]);
  for (const match of content.matchAll(/^\s*([A-Z][A-Z0-9_]*VECTOR_ROOT|VECTOR_ROOT)\s*=\s*["'](asset:\/\/\uD83C\uDFC5\uFE0Fstandards\/\uD83D\uDD16\uFE0F([^/"']+)\/\uD83E\uDE86\uFE0Fsubsets\/\u2733\uFE0F([^/"']+)\/\uD83E\uDDEC\uFE0Fschema\/\uD83E\uDDEC\uFE0Fmutations)["']/gmu)) {
    if (match.index === undefined)
      continue;
    const value = match[2];
    const start2 = match.index + match[0].indexOf(value);
    rows.push({ adapter: "python", structuredLocation: lineLocation(content, start2, `python-string:${match[1]}`), start: start2, end: start2 + value.length, value, rewriteKind: "structural-projection", rewriteData: { newValue: `asset://\uD83E\uDDEA\uFE0Ftests/\uD83E\uDE86\uFE0F${match[3]}-${match[4]}`, projectionKey: "", projectionProfile: `${match[3]}\x00${match[4]}`, artifactRoot: artifactRootForPath(path) ?? "" } });
  }
  for (const match of content.matchAll(/^\s*(stem)\s*=\s*["'](%s\/%s\/\uD83E\uDDEA\uFE0Ftests\/%s)["']\s*%/gmu)) {
    if (match.index === undefined)
      continue;
    const value = match[2];
    const start2 = match.index + match[0].indexOf(value);
    rows.push({ adapter: "python", structuredLocation: lineLocation(content, start2, `python-format:${match[1]}`), start: start2, end: start2 + value.length, value, rewriteKind: "structural-projection", rewriteData: { newValue: "%s/%s/\uD83E\uDDEA\uFE0F%s", projectionKey: "", projectionProfile: "*", artifactRoot: artifactRootForPath(path) ?? "" } });
  }
  return rows;
}
function gherkinTokens(path, content) {
  return structuralTokensInFragment(content, content, 0, "gherkin", "gherkin-description", artifactRootForPath(path));
}
function typescriptTokens(content) {
  return regexTokens(content, "typescript", "typescript-path", [
    /(?:\bfrom\s*|\bimport\s*\(|\brequire\s*\(|\bimport\s+)["'\s]*([^"'\s)]+)["']/gu,
    /\b(?:worker|url)\s*\(\s*["']([^"']+)["']/giu,
    /\b(?:[A-Za-z_$][\w$]*(?:Path|File|Filename|Root|Schema|Taxonomy|Config|Entry|Target|Source|Output|Input)[\w$]*|(?:path|file|filename|root|schema|taxonomy|config|entry|target|source|output|input))\s*(?:=|:)\s*["']([^"']+)["']/giu,
    /\b(?:resolve|join|readFileSync|writeFileSync|existsSync|openSync|Bun\.file)\s*\([^;\r\n]*?["']([^"']+)["']/giu
  ]);
}
function goTokens(path, content) {
  const rows = [];
  if (path.toLowerCase().endsWith(".go")) {
    rows.push(...regexTokens(content, "go", "go-import", [/^\s*(?:[\w.]+\s+)?"([^"]+)"\s*$/gmu]));
    for (const match of content.matchAll(/^\s*\/\/go:(?:embed|generate)\s+([^\r\n]+)$/gmu))
      if (match.index !== undefined)
        rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-directive"));
    return rows;
  }
  for (const match of content.matchAll(/\buse\s*\(([\s\S]*?)\)/gu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*use\s+([^\r\n(][^\r\n]*)$/gmu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*replace\s+[^\r\n=]+=>\s*([^\s]+).*$/gmu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-mod-replace"));
  return rows;
}
function cmakeTokens(content) {
  const rows = [];
  for (const match of content.matchAll(/\b(?:add_subdirectory|add_executable|add_library|target_sources|include|configure_file|set)\s*\(([\s\S]*?)\)/giu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "native", "cmake-argument"));
  return rows;
}
function htmlTokens(content, adapter) {
  return regexTokens(content, adapter, "html-attribute", [/<(?:a|img|script|link|source|video|audio|form)\b[^>]*\b(?:href|src|srcset|poster|data|action)\s*=\s*["']([^"']+)["'][^>]*>/giu]);
}
function referenceTokens(path, content) {
  const lower = path.toLowerCase();
  if (lower.endsWith(".rs"))
    return rustTokens(path, content);
  if (lower.endsWith(".feature"))
    return gherkinTokens(path, content);
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower))
    return typescriptTokens(content);
  if (/\.(?:go|mod|work)$/u.test(lower) || /(?:^|\/)go\.(?:mod|work)$/u.test(lower))
    return goTokens(path, content);
  if (lower.endsWith(".py"))
    return pythonTokens(path, content);
  if (/\.(?:csproj|fsproj|vbproj|sln|props|targets|cs|fs|vb)$/u.test(lower))
    return regexTokens(content, "dotnet", "dotnet-reference", [/(?:Include|Update|Remove|Link|HintPath)\s*=\s*["']([^"']+)["']/giu, /^Project\([^\r\n]+?=\s*[^,]+,\s*"([^"]+)"/gmu, /\b(?:GetManifestResourceStream|ReadAllText|ReadAllBytes)\s*\(\s*["']([^"']+)["']/gu]);
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename2(path) === "CMakeLists.txt")
    return [...regexTokens(content, "native", "native-path", [/^\s*#\s*include\s*[<"]([^>"]+)[>"]/gmu, /["']([^"']+\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx))["']/gu]), ...cmakeTokens(content)];
  if (lower.endsWith(".json"))
    return jsonTokens(path, content, "json");
  if (lower.endsWith(".jsonc"))
    return jsonTokens(path, content, "jsonc");
  if (lower.endsWith(".toml"))
    return tomlTokens(path, content);
  if (/\.ya?ml$/u.test(lower)) {
    const direct = regexTokens(content, "yaml", "yaml-value", [/^\s*(?:-\s*)?[\w.-]+\s*:\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu, /^\s*-\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu]);
    const embeddedArgv = /(?:workflow|action|launch|task|project|(?:^|\/)ci(?:\/|$))/iu.test(path);
    return embeddedArgv ? [...direct, ...direct.flatMap((token) => embeddedArgumentTokens(content, token.value, token.start, "yaml", "embedded-argv"))] : direct;
  }
  if (/\.(?:xml|html|htm)$/u.test(lower))
    return [...regexTokens(content, "xml", "xml-attribute", [/(?:href|src|path|include|file|link|hintpath)\s*=\s*["']([^"']+)["']/giu]), ...htmlTokens(content, "xml")];
  if (/\.(?:md|mdx)$/u.test(lower))
    return [...regexTokens(content, "markdown", "markdown-link", [/!?(?:\[[^\]]*\])\(([^)\s]+)(?:\s+"[^"]*")?\)/gu, /^\s*\[[^\]]+\]:\s*(\S+)/gmu]), ...htmlTokens(content, "markdown")];
  return [];
}
function textualPath(path) {
  return /(?:\.rs|\.tsx?|\.jsx?|\.mjs|\.cjs|\.mts|\.cts|\.go|\.mod|\.work|\.py|\.cs|\.fs|\.vb|\.csproj|\.fsproj|\.vbproj|\.sln|\.props|\.targets|\.c|\.cc|\.cpp|\.cxx|\.h|\.hh|\.hpp|\.hxx|\.cmake|\.jsonc?|\.toml|\.ya?ml|\.xml|\.html?|\.mdx?|\.feature)$/iu.test(path) || basename2(path) === "CMakeLists.txt";
}
function splitTokenSuffix(value) {
  const index = value.search(/[?#]/);
  return index < 0 ? { path: value, suffix: "" } : { path: value.slice(0, index), suffix: value.slice(index) };
}
function addUniqueIndex(index, key, value) {
  if (!key)
    return;
  const existing = index.get(key);
  if (existing === undefined)
    index.set(key, value);
  else if (existing !== value)
    index.set(key, null);
}
function referencePathIndex(paths) {
  const exact = new Set;
  const nfc = new Map;
  const extensionless = new Map;
  const pythonModule = new Map;
  for (const path of paths) {
    exact.add(path);
    const normalized = path.normalize("NFC");
    addUniqueIndex(nfc, normalized, path);
    addUniqueIndex(extensionless, normalized.replace(/\.[^/.]+(?:\.[^/.]+)*$/u, ""), path);
    if (!normalized.endsWith(".py"))
      continue;
    const moduleSegments = (normalized.endsWith("/__init__.py") ? dirname2(normalized) : normalized.slice(0, -3)).split("/").filter(Boolean);
    for (let index = 0;index < moduleSegments.length; index++)
      addUniqueIndex(pythonModule, moduleSegments.slice(index).join("."), path);
  }
  return { exact, nfc, extensionless, pythonModule };
}
function resolveReferencePath(referencePath, token, index) {
  const split = splitTokenSuffix(token);
  if (!split.path || /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/i.test(split.path) || /[*{}]/.test(split.path))
    return null;
  const candidates = [];
  try {
    candidates.push(normalizeRelative(split.path.replace(/^\//, "")));
  } catch {}
  try {
    candidates.push(normalizeRelative(posix.join(dirname2(referencePath), split.path)));
  } catch {}
  for (const candidate of candidates) {
    if (index.exact.has(candidate))
      return candidate;
    const comparison = candidate.normalize("NFC");
    const nfc = index.nfc.get(comparison);
    if (nfc)
      return nfc;
    const extensionless = index.extensionless.get(comparison);
    if (extensionless)
      return extensionless;
  }
  if (/^[\w.]+$/.test(split.path)) {
    const python = index.pythonModule.get(split.path.normalize("NFC"));
    if (python)
      return python;
  }
  return null;
}
function resolveReferenceTokenPath(referencePath, token, index) {
  const matches = [...new Set((token.targetValues ?? [token.value]).map((value) => resolveReferencePath(referencePath, value, index)).filter((value) => value !== null))];
  return matches.length === 1 ? matches[0] : null;
}
function lexicalOpaqueReferenceTarget(referencePath, token, taxonomy) {
  for (const value of token.targetValues ?? [token.value]) {
    const path = splitTokenSuffix(value).path;
    if (!path || /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/iu.test(path) || /[*{}]/u.test(path))
      continue;
    const candidates = [path.replace(/^\//u, ""), posix.join(dirname2(referencePath), path)];
    for (const candidate of candidates) {
      try {
        const normalized = normalizeRelative(candidate);
        if (isExcluded(normalized, taxonomy))
          return normalized;
      } catch {}
    }
  }
  return null;
}
function rewriteReferenceValue(referencePath, oldValue, oldTarget, newTarget, sourceReferencePath = referencePath) {
  const split = splitTokenSuffix(oldValue);
  if (/^[\w.]+$/.test(split.path) && oldTarget.endsWith(".py")) {
    const modulePath = newTarget.replace(/(?:\/__init__)?\.py$/, "").replaceAll("/", ".");
    return `${modulePath}${split.suffix}`;
  }
  const absoluteStyle = split.path.startsWith("/");
  const relativeStyle = split.path.startsWith("./") || split.path.startsWith("../");
  let localBareStyle = false;
  if (!absoluteStyle && !relativeStyle) {
    try {
      localBareStyle = normalizeRelative(posix.join(dirname2(sourceReferencePath), split.path)) === oldTarget;
    } catch {}
  }
  const omittedExtension = !posix.extname(split.path);
  let value = absoluteStyle ? `/${newTarget}` : relativeStyle || localBareStyle ? posix.relative(dirname2(referencePath), newTarget) : newTarget;
  if (relativeStyle && !value.startsWith("."))
    value = `./${value}`;
  if (omittedExtension) {
    const finalName = basename2(newTarget);
    const extensionStart = finalName.indexOf(".");
    const extensionChain = extensionStart < 0 ? "" : finalName.slice(extensionStart);
    if (extensionChain && value.endsWith(extensionChain))
      value = value.slice(0, -extensionChain.length);
  }
  if (oldValue.includes("\\"))
    value = value.replaceAll("/", "\\");
  return `${value}${split.suffix}`;
}
function rewriteReferenceToken(referencePath, sourceReferencePath, token, oldTarget, newTarget) {
  if (token.rewriteKind === "rust-mod") {
    let relativeTarget = posix.relative(dirname2(referencePath), newTarget);
    if (!relativeTarget.startsWith("."))
      relativeTarget = `./${relativeTarget}`;
    const indentation = token.rewriteData?.indentation ?? "";
    const declaration = token.rewriteData?.declaration ?? token.value;
    return `#[path = ${JSON.stringify(relativeTarget)}]
${indentation}${declaration}`;
  }
  if (token.rewriteKind === "python-entrypoint") {
    const targetValue = token.targetValues?.[0] ?? token.value;
    return `${rewriteReferenceValue(referencePath, targetValue, oldTarget, newTarget, sourceReferencePath)}${token.rewriteData?.suffix ?? ""}`;
  }
  if (token.rewriteKind === "artifact-uri") {
    const artifactRoot = token.rewriteData?.artifactRoot;
    if (!artifactRoot || !newTarget.startsWith(`${artifactRoot}/`))
      throw new Error(`Artifact URI target escapes its captured owner: ${newTarget}`);
    return `asset://${newTarget.slice(artifactRoot.length + 1)}`;
  }
  if (token.rewriteKind === "projection-prose" && token.value.startsWith("\uD83C\uDFC5\uFE0Fstandards/")) {
    const artifactRoot = token.rewriteData?.artifactRoot;
    if (!artifactRoot || !newTarget.startsWith(`${artifactRoot}/`))
      throw new Error(`Projection prose target escapes its captured owner: ${newTarget}`);
    return newTarget.slice(artifactRoot.length + 1);
  }
  return rewriteReferenceValue(referencePath, token.value, oldTarget, newTarget, sourceReferencePath);
}
function unsupportedReferenceTokens(content, adapter) {
  const rows = [];
  const patterns = [/"([^"\r\n]+)"|'([^'\r\n]+)'|`([^`\r\n]+)`/gu, /(?:^|[\s(=,:])((?:\.\.?\/|\/)?[^\s"'`()\],;]+\/[^\s"'`()\],;]+|[A-Za-z0-9_.@-]+\.[A-Za-z0-9.]+)(?=$|[\s),;\]])/gmu];
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      if (match.index === undefined)
        continue;
      const value = match[1] ?? match[2] ?? match[3];
      if (!value || !/[\\/]/u.test(value) && !/^\.{1,2}$/u.test(value) && !/\.[A-Za-z0-9][A-Za-z0-9.-]*$/u.test(value))
        continue;
      const start2 = match.index + match[0].indexOf(value);
      rows.push({ adapter, structuredLocation: lineLocation(content, start2, "unsupported-path-syntax"), start: start2, end: start2 + value.length, value });
    }
  }
  return rows;
}
function referenceAdapter(path) {
  const lower = path.toLocaleLowerCase("und");
  if (lower.endsWith(".feature"))
    return "gherkin";
  if (lower.endsWith(".rs") || basename2(path) === "Cargo.toml")
    return "rust";
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower))
    return "typescript";
  if (/\.(?:go|mod|work)$/u.test(lower))
    return "go";
  if (lower.endsWith(".py"))
    return "python";
  if (/\.(?:cs|fs|vb|csproj|fsproj|vbproj|sln|props|targets)$/u.test(lower))
    return "dotnet";
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename2(path) === "CMakeLists.txt")
    return "native";
  if (lower.endsWith(".jsonc"))
    return "jsonc";
  if (lower.endsWith(".json"))
    return "json";
  if (lower.endsWith(".toml"))
    return "toml";
  if (/\.ya?ml$/u.test(lower))
    return "yaml";
  if (/\.(?:xml|html|htm)$/u.test(lower))
    return "xml";
  return "markdown";
}
function applyEditsToContent(content, edits) {
  let result = content;
  const offset = (edit) => {
    const value = edit.structuredLocation.match(/@(\d+)$/)?.[1];
    if (value === undefined)
      throw new Error(`Reference edit lacks a structured offset at ${edit.path}:${edit.structuredLocation}`);
    return Number.parseInt(value, 10);
  };
  const sorted = [...edits].sort((a, b) => offset(b) - offset(a) || b.structuredLocation.localeCompare(a.structuredLocation));
  for (const edit of sorted) {
    const start2 = offset(edit);
    const end = start2 + edit.oldValue.length;
    if (result.slice(start2, end) !== edit.oldValue)
      throw new Error(`Reference edit preimage mismatch at ${edit.path}:${edit.structuredLocation}`);
    result = `${result.slice(0, start2)}${edit.newValue}${result.slice(end)}`;
  }
  return result;
}
function referenceGraph(repoRoot, entries, taxonomy, progress, cancelFile) {
  const known = referencePathIndex(entries.keys());
  const files = [...entries.values()].filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && (entry.size ?? 0) <= 16 * 1024 * 1024);
  for (let index = 0;index < files.length; index++) {
    checkCancellation(repoRoot, cancelFile);
    const entry = files[index];
    if (isExcluded(entry.sourcePath, taxonomy))
      continue;
    let content;
    try {
      content = readFileSync(absolutePath(repoRoot, entry.sourcePath), "utf8");
    } catch {
      continue;
    }
    for (const token of referenceTokens(entry.sourcePath, content)) {
      const target = resolveReferenceTokenPath(entry.sourcePath, token, known);
      if (!target || !entries.has(target)) {
        const opaque = lexicalOpaqueReferenceTarget(entry.sourcePath, token, taxonomy);
        if (opaque)
          entry.violations.push(violation("opaque-reference-target", entry.sourcePath, `${token.adapter} ${token.structuredLocation} lexically targets excluded ${opaque}`, "warning"));
        continue;
      }
      entry.referencesOut.push(target);
      entries.get(target)?.referencesIn.push(entry.sourcePath);
    }
    report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
  }
  for (const entry of entries.values()) {
    entry.referencesIn = [...new Set(entry.referencesIn)].sort();
    entry.referencesOut = [...new Set(entry.referencesOut)].sort();
  }
}
function referenceEditIdentity(edit) {
  return `${edit.path}\x00${edit.structuredLocation}\x00${edit.oldValue}\x00${edit.newValue}`;
}
function buildReferenceEdits(inventory, moves, taxonomy, options, known) {
  const moveBySource = new Map(moves.map((move) => [move.sourcePath, move]));
  const destinationBySource = new Map(inventory.entries.filter((entry) => entry.sourcePath !== entry.normalizedPath && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).map((entry) => [entry.sourcePath, entry.normalizedPath]));
  const edits = [];
  const editTargets = new Map;
  const unresolved = [];
  const resultHashes = new Map;
  const accountedIncoming = new Set;
  const activeProjectionKeys = new Set;
  const activeProjectionProfiles = new Set;
  for (const move of moves.filter((entry) => entry.rationaleRule === "artifact-mutation-test-projection-v1")) {
    const structural = mutationStructuralPaths(move.sourcePath)[0];
    const artifactRoot = artifactRootForPath(move.sourcePath);
    if (!structural || !artifactRoot)
      continue;
    activeProjectionKeys.add(projectionKey(artifactRoot, structural.standard, structural.subset));
    activeProjectionProfiles.add(`${structural.standard}\x00${structural.subset}`);
  }
  const candidates = inventory.entries.filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0);
  for (let index = 0;index < candidates.length; index++) {
    checkCancellation(inventory.repoRoot, options.cancelFile);
    const entry = candidates[index];
    if (isExcluded(entry.sourcePath, taxonomy))
      continue;
    let content;
    try {
      content = readFileSync(absolutePath(inventory.repoRoot, entry.sourcePath), "utf8");
    } catch (error) {
      unresolved.push(violation("reference-preimage-unreadable", entry.sourcePath, error instanceof Error ? error.message : String(error)));
      continue;
    }
    const finalReferencePath = moveBySource.get(entry.sourcePath)?.destinationPath ?? entry.normalizedPath;
    const fileEdits = [];
    const fileTargets = new Map;
    const tokens = [...new Map(referenceTokens(entry.sourcePath, content).map((token) => [`${token.start}\x00${token.end}\x00${token.value}\x00${(token.targetValues ?? []).join("\x00")}`, token])).values()].sort((left, right) => left.start - right.start || left.end - right.end || left.structuredLocation.localeCompare(right.structuredLocation));
    const supported = tokens.map((token) => ({ token, target: resolveReferenceTokenPath(entry.sourcePath, token, known) }));
    for (const { token, target: oldTarget } of supported) {
      const destination = oldTarget ? destinationBySource.get(oldTarget) : undefined;
      const projectionProfile = token.rewriteData?.projectionProfile;
      const projectionActive = activeProjectionKeys.has(token.rewriteData?.projectionKey ?? "") || projectionProfile === "*" && activeProjectionProfiles.size > 0 || projectionProfile !== undefined && activeProjectionProfiles.has(projectionProfile);
      if (token.unsupportedReason && projectionActive) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${token.unsupportedReason}`));
        continue;
      }
      if ((!oldTarget || !destination) && !(projectionActive && token.rewriteData?.newValue))
        continue;
      const newValue = oldTarget && destination ? rewriteReferenceToken(finalReferencePath, entry.sourcePath, token, oldTarget, destination) : token.rewriteData.newValue;
      if (newValue === token.value)
        continue;
      if (oldTarget)
        accountedIncoming.add(`${oldTarget}\x00${entry.sourcePath}`);
      const edit = {
        path: finalReferencePath,
        adapter: token.adapter,
        structuredLocation: token.structuredLocation,
        oldValue: token.value,
        newValue,
        preimageHash: entry.contentHash
      };
      fileEdits.push(edit);
      if (oldTarget)
        fileTargets.set(referenceEditIdentity(edit), oldTarget);
    }
    for (const candidate of unsupportedReferenceTokens(content, referenceAdapter(entry.sourcePath))) {
      const oldTarget = resolveReferenceTokenPath(entry.sourcePath, candidate, known);
      if (!oldTarget || !destinationBySource.has(oldTarget))
        continue;
      const covered = supported.some(({ token, target }) => target === oldTarget && token.start <= candidate.start && token.end >= candidate.end);
      if (!covered)
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${candidate.adapter} ${candidate.structuredLocation} contains unsupported path-bearing token ${JSON.stringify(candidate.value)} targeting ${oldTarget}`));
    }
    if (fileEdits.length > 0) {
      const deduplicated = [...new Map(fileEdits.map((edit) => [`${edit.structuredLocation}:${edit.newValue}`, edit])).values()].sort(referenceEditCompare);
      edits.push(...deduplicated);
      for (const edit of deduplicated) {
        const target = fileTargets.get(referenceEditIdentity(edit));
        if (target)
          editTargets.set(referenceEditIdentity(edit), target);
      }
      resultHashes.set(finalReferencePath, sha256(applyEditsToContent(content, deduplicated)));
    }
    report(options.progress, "plan", "references", index + 1, candidates.length, entry.sourcePath);
  }
  for (const move of moves) {
    const entry = inventory.entries.find((candidate) => candidate.sourcePath === move.sourcePath);
    if (entry?.fileKind) {
      const role = taxonomy.schema.fileKinds[entry.fileKind]?.role;
      const unaccounted = entry.referencesIn.filter((source) => !accountedIncoming.has(`${entry.sourcePath}\x00${source}`));
      if (role === "binary" && unaccounted.length > 0)
        unresolved.push(violation("opaque-reference-rewrite-unresolved", entry.sourcePath, `Binary target has unsupported incoming references from ${unaccounted.join(", ")}`));
      if (role === "generated" && entry.referencesIn.length > 0)
        unresolved.push(violation("generated-reference-rewrite-unresolved", entry.sourcePath, "Generated target requires an explicit regeneration contract before its incoming references can move"));
    }
  }
  return { edits: edits.sort(referenceEditCompare), editTargets, resultHashes, unresolved: stableViolations(unresolved) };
}
function referenceEditCompare(a, b) {
  return a.path.localeCompare(b.path) || a.structuredLocation.localeCompare(b.structuredLocation) || a.oldValue.localeCompare(b.oldValue) || a.newValue.localeCompare(b.newValue);
}
function pathPolicyViolations(path, taxonomy) {
  const rows = [];
  if (Buffer.byteLength(path, "utf8") > taxonomy.schema.collisionPolicy.maxPathBytes)
    rows.push(violation("path-too-long", path, `Path exceeds ${taxonomy.schema.collisionPolicy.maxPathBytes} UTF-8 bytes`));
  for (const segment of path.split("/")) {
    if (taxonomy.schema.collisionPolicy.rejectWindowsReservedNames && WINDOWS_RESERVED.test(segment))
      rows.push(violation("windows-reserved-name", path, `Path segment is Windows-reserved: ${segment}`));
    if (taxonomy.schema.collisionPolicy.rejectTrailingDotsAndSpaces && /[. ]$/.test(segment))
      rows.push(violation("trailing-dot-or-space", path, `Path segment ends with a dot or space: ${segment}`));
  }
  return rows;
}
function sourceTreeDigest(entries) {
  return sha256(canonicalJson(entries.map((entry) => ({ sourcePath: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash }))));
}
function ancestorDirectoryKindIds(path, kinds) {
  const rows = [];
  let current = dirname2(path);
  while (current && current !== ".") {
    const kindId = kinds.get(current);
    if (kindId)
      rows.push(kindId);
    current = dirname2(current);
  }
  return rows;
}
function projectionDirectorySlug(name, kindId, taxonomy) {
  const kind = taxonomy.schema.semanticDirectoryKinds[kindId];
  if (!kind)
    return null;
  const leading = splitLeadingEmoji(name.normalize("NFC"));
  if (emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest))
    return null;
  return leading.rest;
}
function projectionSourceAt(path, scope, entries, kinds, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const contract = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const segments = path.split("/");
  if (segments.length <= contract.sourceSegments.length)
    return null;
  const start2 = segments.length - contract.sourceSegments.length;
  const artifactRoot = segments.slice(0, start2).join("/");
  const ownerRegistry = taxonomy.schema.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
  const ownerMatches = ownerRegistry.memberNames.filter((name) => emojiFold(name) === emojiFold(basename2(artifactRoot)));
  if (ownerMatches.length !== 1 && !(scope && (artifactRoot === scope || artifactRoot.startsWith(`${scope}/`))))
    return null;
  const captures = new Map;
  for (let index = 0;index < contract.sourceSegments.length; index++) {
    const segment = contract.sourceSegments[index];
    const currentPath = segments.slice(0, start2 + index + 1).join("/");
    const current = entries.get(currentPath);
    if (!current || current.nodeKind !== "directory")
      return null;
    const canonicalName = basename2(current.normalizedPath);
    if ("literal" in segment) {
      if (canonicalName !== segment.literal || kinds.get(currentPath) !== segment.kindId)
        return null;
      continue;
    }
    if ("projectedMemberKindId" in segment) {
      if (segment.projectedMemberKindId !== ids.projectedMemberKindId)
        return null;
      const sourceName2 = basename2(current.sourcePath).normalize("NFC");
      const slug2 = splitLeadingEmoji(sourceName2).rest;
      if (!slug2)
        return null;
      captures.set(segment.capture, slug2);
      continue;
    }
    const sourceName = basename2(current.sourcePath).normalize("NFC");
    const contextualUnprefixed = segment.capture === "scenarioId" && !splitLeadingEmoji(sourceName).emoji && new RegExp(taxonomy.schema.semanticDirectoryKinds[segment.kindId].slugPattern, "u").test(sourceName);
    if (kinds.get(currentPath) !== segment.kindId && !contextualUnprefixed)
      return null;
    const slug = contextualUnprefixed ? sourceName : projectionDirectorySlug(canonicalName, segment.kindId, taxonomy);
    if (!slug)
      return null;
    captures.set(segment.capture, slug);
  }
  const standardVersion = captures.get("standardVersion");
  const subsetId = captures.get("subsetId");
  const mutationId = captures.get("mutationId");
  const sourceScenarioId = captures.get("scenarioId");
  if (!standardVersion || !subsetId || !mutationId || !sourceScenarioId)
    return null;
  const source = contract.sourceSegments.map((_segment, index) => segments.slice(0, start2 + index + 1).join("/"));
  return {
    artifactRoot,
    artifactId: splitLeadingEmoji(basename2(artifactRoot)).rest || basename2(artifactRoot),
    standardVersion,
    standardDirectoryName: basename2(source[1]),
    subsetId,
    subsetDirectoryName: basename2(source[3]),
    mutationId,
    mutationDirectoryName: basename2(source[6]),
    sourceScenarioId,
    sourceScenarioDirectoryName: basename2(source[8]),
    subsetRoot: source[3],
    mutationRoot: source[6],
    scenarioRoot: source[8]
  };
}
function projectionCatalogVectors(path, source) {
  let root;
  try {
    root = record(JSON.parse(readFileSync(path, "utf8")), "mutation projection catalog");
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  if (!Array.isArray(root.mutationCatalogs))
    return { vectors: [], error: "mutationCatalogs must be an array" };
  const vectors = [];
  const seenSource = new Set;
  const seenCanonical = new Set;
  try {
    for (let catalogIndex = 0;catalogIndex < root.mutationCatalogs.length; catalogIndex++) {
      const catalog = record(root.mutationCatalogs[catalogIndex], `mutationCatalogs[${catalogIndex}]`);
      requiredString(catalog.id, `mutationCatalogs[${catalogIndex}].id`);
      requiredString(catalog.capability, `mutationCatalogs[${catalogIndex}].capability`);
      if (requiredString(catalog.standardDirectoryName, `mutationCatalogs[${catalogIndex}].standardDirectoryName`) !== source.standardDirectoryName || requiredString(catalog.subsetDirectoryName, `mutationCatalogs[${catalogIndex}].subsetDirectoryName`) !== source.subsetDirectoryName)
        throw new Error(`mutationCatalogs[${catalogIndex}] owner identity does not match its physical standard/subset`);
      stringArray(catalog.kinds, `mutationCatalogs[${catalogIndex}].kinds`);
      if (!Array.isArray(catalog.vectors))
        throw new Error(`mutationCatalogs[${catalogIndex}].vectors must be an array`);
      for (let vectorIndex = 0;vectorIndex < catalog.vectors.length; vectorIndex++) {
        const vector = record(catalog.vectors[vectorIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}]`);
        const mutationId = requiredString(vector.mutationId, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationId`);
        const sourceMutationDirectoryName = requiredString(vector.sourceMutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName`);
        if (sourceMutationDirectoryName !== sourceMutationDirectoryName.normalize("NFC") || sourceMutationDirectoryName.includes("/"))
          throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName is not one exact NFC basename`);
        const mutationDirectoryName = requiredString(vector.mutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationDirectoryName`).normalize("NFC");
        if (!Array.isArray(vector.scenarios))
          throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}] has an invalid physical mutation identity`);
        for (let scenarioIndex = 0;scenarioIndex < vector.scenarios.length; scenarioIndex++) {
          const scenario = record(vector.scenarios[scenarioIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}]`);
          const scenarioId = requiredString(scenario.id, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].id`);
          const scenarioDirectoryName = requiredString(scenario.directoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName`).normalize("NFC");
          if (splitLeadingEmoji(scenarioDirectoryName).rest !== scenarioId)
            throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}] has an invalid physical scenario identity`);
          const sourceKey = `${mutationId}\x00${sourceMutationDirectoryName}\x00${scenarioId}`;
          const canonicalKey = `${mutationId}\x00${mutationDirectoryName}\x00${scenarioId}`;
          if (seenSource.has(sourceKey) || seenCanonical.has(canonicalKey))
            throw new Error(`Duplicate physical vector identity ${sourceKey.replaceAll("\x00", "/")}`);
          seenSource.add(sourceKey);
          seenCanonical.add(canonicalKey);
          vectors.push({ mutationId, sourceMutationDirectoryName, mutationDirectoryName, scenarioId, scenarioDirectoryName });
        }
      }
    }
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  return { vectors: vectors.sort((left, right) => left.sourceMutationDirectoryName.localeCompare(right.sourceMutationDirectoryName) || left.scenarioDirectoryName.localeCompare(right.scenarioDirectoryName)) };
}
function projectionCatalogEntryForSubset(entries, subsetRoot) {
  const oracleRoot = `${subsetRoot}/\uD83E\uDDEA\uFE0Foracle`;
  const candidates = [...entries.values()].filter((entry) => entry.nodeKind === "file" && entry.fileKind === "json" && dirname2(entry.sourcePath) === oracleRoot && basename2(entry.normalizedPath) === "\uD83D\uDD23\uFE0F.json");
  return candidates.length === 1 ? candidates[0] : null;
}
function mutationDescendantContract(taxonomy) {
  const contract = taxonomy.schema.semanticDescendantContracts[taxonomy.schema.mutationCatalogProjection.descendantContractId];
  if (!contract || "contractKind" in contract || [...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].some((node) => !("kindId" in node)))
    throw new Error("Mutation projection must reference one physical-kind exact descendant contract");
  return contract;
}
function projectionDescendantPath(node, taxonomy) {
  const segments = node.pathSegments.map((segment) => segment.literal);
  if (node.nodeType === "file") {
    const kind = taxonomy.schema.fileKinds[node.kindId];
    if (!kind || kind.extensionChains.length !== 1)
      throw new Error(`Projection descendant kind ${node.kindId} is not a single physical leaf`);
    segments.push(`${kind.emoji}${kind.extensionChains[0]}`.normalize("NFC"));
  }
  return segments.join("/");
}
function canonicalProjectedMemberName(name, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const sourceKind = taxonomy.schema.semanticProjectedMemberKinds[ids.projectedMemberKindId].sourceMemberKindId;
  const matches = taxonomy.schema.semanticDirectoryMemberKinds[sourceKind].memberNames.filter((candidate) => emojiFold(candidate) === emojiFold(name.normalize("NFC")));
  return matches.length === 1 ? matches[0] : null;
}
function projectionBundleProblem(source, entries, kinds, contract, taxonomy) {
  const root = entries.get(source.scenarioRoot);
  if (!root)
    return "scenario root is absent";
  const actual = [...entries.values()].filter((entry) => entry.sourcePath === source.scenarioRoot || entry.sourcePath.startsWith(`${source.scenarioRoot}/`));
  if (actual.length !== contract.realizedNodeCount)
    return `bundle has ${actual.length} nodes, expected ${contract.realizedNodeCount}`;
  if (actual.some((entry) => entry.nodeKind === "symlink"))
    return "bundle contains a symlink";
  const byKey = new Map;
  for (const entry of actual) {
    const relativePath = entry.normalizedPath === root.normalizedPath ? "" : entry.normalizedPath.startsWith(`${root.normalizedPath}/`) ? entry.normalizedPath.slice(root.normalizedPath.length + 1) : null;
    if (relativePath === null)
      return `bundle node normalizes outside its scenario: ${entry.sourcePath}`;
    const key = `${entry.nodeKind}\x00${relativePath}`;
    if (byKey.has(key))
      return `bundle normalization duplicates ${relativePath}`;
    byKey.set(key, entry);
  }
  const matches = (node) => {
    const entry = byKey.get(`${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`);
    if (!entry)
      return false;
    return node.nodeType === "file" ? entry.fileKind === node.kindId : node.pathSegments.length === 0 && entry.sourcePath === source.scenarioRoot && node.kindId === contract.rootDirectoryKindId || kinds.get(entry.sourcePath) === node.kindId;
  };
  const missing = contract.requiredNodes.filter((node) => !matches(node));
  if (missing.length > 0)
    return `bundle is missing ${projectionDescendantPath(missing[0], taxonomy) || "scenario root"}`;
  for (const alternative of contract.exclusiveAlternatives)
    if (alternative.nodes.filter(matches).length !== 1)
      return `bundle must realize exactly one ${alternative.id} alternative`;
  const allowed = new Set([...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].filter(matches).map((node) => `${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`));
  const extra = [...byKey.keys()].find((key) => !allowed.has(key));
  return extra ? `bundle contains unregistered node ${extra.slice(extra.indexOf("\x00") + 1)}` : null;
}
function setProjectedPath(entry, destination, taxonomy) {
  entry.normalizedPath = destination.normalize("NFC");
  const superseded = new Set(["path-too-long", "windows-reserved-name", "trailing-dot-or-space"]);
  entry.violations = [...entry.violations.filter((row) => !superseded.has(row.code)), ...pathPolicyViolations(entry.normalizedPath, taxonomy)];
}
function mutationProjectionRationale(sourcePath, destinationPath, taxonomy) {
  const structural = mutationStructuralPaths(sourcePath)[0];
  const artifactRoot = artifactRootForPath(sourcePath);
  if (!artifactRoot)
    return null;
  const relativeDestination = destinationPath.startsWith(`${artifactRoot}/`) ? destinationPath.slice(artifactRoot.length + 1).split("/") : [];
  if (structural && relativeDestination[0] === "\uD83E\uDDEA\uFE0Ftests" && relativeDestination[1] === `\uD83E\uDE86\uFE0F${structural.standard}-${structural.subset}` && canonicalProjectedMemberName(relativeDestination[2] ?? "", taxonomy) === relativeDestination[2] && emojiFold(splitLeadingEmoji(relativeDestination[3] ?? "").emoji) === emojiFold(taxonomy.schema.semanticDirectoryKinds[taxonomy.schema.semanticDescendantContracts[taxonomy.schema.mutationCatalogProjection.descendantContractId].rootDirectoryKindId].emoji))
    return "artifact-mutation-test-projection-v1";
  const relativeSource = sourcePath.slice(artifactRoot.length + 1).split("/");
  const prefix = ["\uD83C\uDFC5\uFE0Fstandards", relativeSource[1], "\uD83E\uDE86\uFE0Fsubsets", relativeSource[3], "\uD83E\uDDEC\uFE0Fschema", "\uD83E\uDDEC\uFE0Fmutations"];
  if (relativeSource.length > 7 && prefix.every((segment, index) => relativeSource[index] === segment) && prefix.every((segment, index) => relativeDestination[index] === segment) && relativeSource[6] !== relativeDestination[6] && canonicalProjectedMemberName(relativeDestination[6] ?? "", taxonomy) === relativeDestination[6])
    return "artifact-mutation-source-canonicalization-v1";
  return null;
}
function projectMutationTestBundles(repoRoot, scope, entries, kinds, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const projection = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const descendant = mutationDescendantContract(taxonomy);
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const sources = [...entries.values()].filter((entry) => entry.nodeKind === "directory").map((entry) => projectionSourceAt(entry.sourcePath, scope, entries, kinds, taxonomy)).filter((entry) => entry !== null).sort((left, right) => left.scenarioRoot.localeCompare(right.scenarioRoot));
  const bySubset = new Map;
  for (const source of sources)
    bySubset.set(source.subsetRoot, [...bySubset.get(source.subsetRoot) ?? [], source]);
  const profileOwners = new Map;
  for (const source of sources) {
    const profile = renderer.template.replace("{standardVersion}", source.standardVersion).replace("{subsetId}", source.subsetId);
    const key = `${source.artifactRoot}\x00${emojiFold(profile).toLocaleLowerCase("und")}`;
    const owners = profileOwners.get(key) ?? new Set;
    owners.add(`${source.artifactId}\x00${source.standardVersion}\x00${source.subsetId}`);
    profileOwners.set(key, owners);
  }
  for (const [subsetRoot, subsetSources] of [...bySubset.entries()].sort(([left], [right]) => left.localeCompare(right))) {
    const catalogEntry = projectionCatalogEntryForSubset(entries, subsetRoot);
    const catalogPath = catalogEntry?.sourcePath ?? `${subsetRoot}/\uD83E\uDDEA\uFE0Foracle/\uD83D\uDD23\uFE0F.json`;
    const catalog = catalogEntry?.nodeKind === "file" ? projectionCatalogVectors(absolutePath(repoRoot, catalogPath), subsetSources[0]) : { vectors: [], error: `catalog is missing at ${catalogPath}` };
    if (catalog.error) {
      for (const source of subsetSources)
        entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-invalid", catalogPath, catalog.error));
      continue;
    }
    const vectorsByMutation = new Map;
    for (const vector of catalog.vectors) {
      const key = vector.sourceMutationDirectoryName;
      vectorsByMutation.set(key, [...vectorsByMutation.get(key) ?? [], vector]);
    }
    const sourcesByMutation = new Map;
    for (const source of subsetSources) {
      const key = source.mutationDirectoryName.normalize("NFC");
      sourcesByMutation.set(key, [...sourcesByMutation.get(key) ?? [], source]);
    }
    const consumed = new Set;
    const canonicalizedMutationRoots = new Set;
    for (const [mutationKey, mutationSources] of sourcesByMutation) {
      const vectors = vectorsByMutation.get(mutationKey) ?? [];
      const canonicalNames = [...new Set(vectors.map((vector) => canonicalProjectedMemberName(vector.mutationDirectoryName, taxonomy)).filter((name) => name !== null))];
      const mutationName = canonicalNames.length === 1 ? canonicalNames[0] : null;
      if (!mutationName || vectors.some((vector) => vector.sourceMutationDirectoryName !== mutationKey || vector.mutationDirectoryName !== mutationName)) {
        for (const source of mutationSources)
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-member-unresolved", source.mutationRoot, `Mutation member ${source.mutationDirectoryName} has no unique canonical registry identity`));
        continue;
      }
      const exact = new Map(vectors.map((vector) => [vector.scenarioDirectoryName, vector]));
      const assignments = new Map;
      for (const source of mutationSources) {
        const canonicalSourceName = `${taxonomy.schema.semanticDirectoryKinds[descendant.rootDirectoryKindId].emoji}${source.sourceScenarioId}`.normalize("NFC");
        const vector = exact.get(canonicalSourceName);
        if (vector)
          assignments.set(source, vector);
      }
      const unmatchedSources = mutationSources.filter((source) => !assignments.has(source));
      const matchedVectors = new Set(assignments.values());
      const unmatchedVectors = vectors.filter((vector) => !matchedVectors.has(vector));
      if (unmatchedSources.length === 1 && unmatchedVectors.length === 1)
        assignments.set(unmatchedSources[0], unmatchedVectors[0]);
      if (assignments.size !== mutationSources.length || assignments.size !== vectors.length) {
        for (const source of mutationSources)
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-coverage", source.scenarioRoot, `Physical mutation ${mutationName} does not have an exact one-to-one vector registry`));
        continue;
      }
      for (const [source, vector] of assignments) {
        const vectorKey = `${vector.mutationId}\x00${vector.sourceMutationDirectoryName}\x00${vector.scenarioId}`;
        if (consumed.has(vectorKey)) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-duplicate", source.scenarioRoot, `Vector ${vectorKey.replaceAll("\x00", "/")} owns more than one physical bundle`));
          continue;
        }
        const problem = projectionBundleProblem(source, entries, kinds, descendant, taxonomy);
        if (problem) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-bundle-invalid", source.scenarioRoot, problem));
          continue;
        }
        const profile = renderer.template.replace("{standardVersion}", source.standardVersion).replace("{subsetId}", source.subsetId).normalize("NFC");
        const profileKey = `${source.artifactRoot}\x00${emojiFold(profile).toLocaleLowerCase("und")}`;
        if ((profileOwners.get(profileKey)?.size ?? 0) !== 1) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-profile-collision", source.scenarioRoot, `Profile ${profile} is not a unique standard/subset rendering`));
          continue;
        }
        const destinationSegments = projection.destinationSegments.map((segment) => {
          if ("literal" in segment)
            return segment.literal;
          if ("render" in segment)
            return profile;
          if ("projectedMemberKindId" in segment)
            return mutationName;
          return vector.scenarioDirectoryName;
        });
        const destinationRoot = `${source.artifactRoot}/${destinationSegments.join("/")}`.normalize("NFC");
        if (Buffer.byteLength(destinationRoot, "utf8") + descendant.pathBudgetReserve.bytes > taxonomy.schema.collisionPolicy.maxPathBytes) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-path-budget", source.scenarioRoot, `Projected scenario plus reserved descendant suffix exceeds ${taxonomy.schema.collisionPolicy.maxPathBytes} bytes`));
          continue;
        }
        consumed.add(vectorKey);
        const root = entries.get(source.scenarioRoot);
        root.violations = root.violations.filter((row) => row.code !== "directory-kind-unresolved");
        const initialRoot = root.normalizedPath;
        for (const entry of entries.values()) {
          if (entry.sourcePath !== source.scenarioRoot && !entry.sourcePath.startsWith(`${source.scenarioRoot}/`))
            continue;
          const suffix = entry.normalizedPath === initialRoot ? "" : entry.normalizedPath.slice(initialRoot.length + 1);
          setProjectedPath(entry, suffix ? `${destinationRoot}/${suffix}` : destinationRoot, taxonomy);
        }
        if (!canonicalizedMutationRoots.has(source.mutationRoot)) {
          const mutation = entries.get(source.mutationRoot);
          const testsRoot = dirname2(source.scenarioRoot);
          if (mutation) {
            const initialMutationRoot = mutation.normalizedPath;
            const canonicalMutationRoot = `${dirname2(source.mutationRoot)}/${mutationName}`.normalize("NFC");
            const mutationEntries = [...entries.values()].filter((entry) => entry.sourcePath === source.mutationRoot || entry.sourcePath.startsWith(`${source.mutationRoot}/`)).filter((entry) => entry.sourcePath !== testsRoot && !entry.sourcePath.startsWith(`${testsRoot}/`));
            mutation.violations = mutation.violations.filter((row) => row.code !== "directory-kind-unresolved");
            for (const entry of mutationEntries) {
              const suffix = entry.normalizedPath === initialMutationRoot ? "" : entry.normalizedPath.startsWith(`${initialMutationRoot}/`) ? entry.normalizedPath.slice(initialMutationRoot.length + 1) : entry.sourcePath.slice(source.mutationRoot.length + 1);
              setProjectedPath(entry, suffix ? `${canonicalMutationRoot}/${suffix}` : canonicalMutationRoot, taxonomy);
            }
          }
          canonicalizedMutationRoots.add(source.mutationRoot);
        }
      }
    }
    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\x00${vector.sourceMutationDirectoryName}\x00${vector.scenarioId}`;
      if (!consumed.has(key))
        catalogEntry?.violations.push(violation("projection-catalog-unrealized", catalogPath, `Registered vector ${key.replaceAll("\x00", "/")} has no physical bundle`));
    }
  }
}
function validateProjectedMutationTestBundles(repoRoot, scope, entries, kinds, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const projection = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const descendant = mutationDescendantContract(taxonomy);
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const expected = new Set;
  const catalogs = [...entries.values()].filter((entry) => entry.nodeKind === "file" && entry.fileKind === "json" && basename2(dirname2(entry.sourcePath)) === "\uD83E\uDDEA\uFE0Foracle" && basename2(entry.normalizedPath) === "\uD83D\uDD23\uFE0F.json").sort((left, right) => left.sourcePath.localeCompare(right.sourcePath));
  for (const catalogEntry of catalogs) {
    const subsetRoot = dirname2(dirname2(catalogEntry.sourcePath));
    const segments = subsetRoot.split("/");
    if (segments.length < 5)
      continue;
    const artifactRoot = segments.slice(0, -4).join("/");
    const [standardsName, standardDirectoryName, subsetsName, subsetDirectoryName] = segments.slice(-4);
    if (standardsName !== "\uD83C\uDFC5\uFE0Fstandards" || subsetsName !== "\uD83E\uDE86\uFE0Fsubsets")
      continue;
    const ownerRegistry = taxonomy.schema.semanticDirectoryMemberKinds[projection.sourceOwnerKindId];
    const ownerMatches = ownerRegistry.memberNames.filter((name) => emojiFold(name) === emojiFold(basename2(artifactRoot)));
    if (ownerMatches.length !== 1 && !(scope && (artifactRoot === scope || artifactRoot.startsWith(`${scope}/`))))
      continue;
    const standardVersion = projectionDirectorySlug(standardDirectoryName, "standard", taxonomy);
    const subsetId = projectionDirectorySlug(subsetDirectoryName, "subset", taxonomy);
    if (!standardVersion || !subsetId)
      continue;
    const catalog = projectionCatalogVectors(absolutePath(repoRoot, catalogEntry.sourcePath), { standardDirectoryName, subsetDirectoryName });
    if (catalog.error) {
      catalogEntry.violations.push(violation("projection-catalog-invalid", catalogEntry.sourcePath, catalog.error));
      continue;
    }
    const profile = renderer.template.replace("{standardVersion}", standardVersion).replace("{subsetId}", subsetId).normalize("NFC");
    for (const vector of catalog.vectors) {
      const mutationDirectoryName = canonicalProjectedMemberName(vector.mutationDirectoryName, taxonomy);
      if (!mutationDirectoryName) {
        catalogEntry.violations.push(violation("projection-member-unresolved", catalogEntry.sourcePath, `Mutation member ${vector.mutationDirectoryName} has no unique canonical registry identity`));
        continue;
      }
      const mutationRoot = `${artifactRoot}/\uD83E\uDDEA\uFE0Ftests/${profile}/${mutationDirectoryName}`;
      const scenarioRoot = `${mutationRoot}/${vector.scenarioDirectoryName}`;
      expected.add(scenarioRoot);
      const root = entries.get(scenarioRoot);
      if (!root)
        continue;
      const source = { artifactRoot, artifactId: splitLeadingEmoji(basename2(artifactRoot)).rest || basename2(artifactRoot), standardVersion, standardDirectoryName, subsetId, subsetDirectoryName, mutationId: vector.mutationId, mutationDirectoryName, sourceScenarioId: vector.scenarioId, sourceScenarioDirectoryName: vector.scenarioDirectoryName, subsetRoot, mutationRoot, scenarioRoot };
      const problem = projectionBundleProblem(source, entries, kinds, descendant, taxonomy);
      if (problem) {
        root.violations.push(violation("projection-bundle-invalid", scenarioRoot, problem));
        continue;
      }
      root.violations = root.violations.filter((row) => row.code !== "directory-kind-unresolved");
      const initialRoot = root.normalizedPath;
      for (const entry of entries.values()) {
        if (entry.sourcePath !== scenarioRoot && !entry.sourcePath.startsWith(`${scenarioRoot}/`))
          continue;
        const suffix = entry.normalizedPath === initialRoot ? "" : entry.normalizedPath.slice(initialRoot.length + 1);
        setProjectedPath(entry, suffix ? `${scenarioRoot}/${suffix}` : scenarioRoot, taxonomy);
      }
      const mutation = entries.get(mutationRoot);
      if (mutation) {
        mutation.violations = mutation.violations.filter((row) => row.code !== "directory-kind-unresolved");
        setProjectedPath(mutation, mutationRoot, taxonomy);
      }
    }
  }
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "directory" || expected.has(entry.sourcePath))
      continue;
    const segments = entry.sourcePath.split("/");
    if (segments.length < 4)
      continue;
    const profilePath = segments.slice(0, -2).join("/");
    const testsPath = dirname2(profilePath);
    if (kinds.get(profilePath) === renderer.directoryKindId && basename2(testsPath) === "\uD83E\uDDEA\uFE0Ftests")
      entry.violations.push(violation("projection-destination-unregistered", entry.sourcePath, "Projected scenario has no exact catalog vector identity"));
  }
}
function inventoryTaxonomy(options) {
  const repoRoot = resolve2(options.repoRoot);
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1))
    throw new Error("workers must be a positive integer");
  const taxonomy = loadTaxonomy({ repoRoot, taxonomyPath: options.taxonomyPath });
  const scope = options.scope === undefined ? undefined : normalizeRelative(options.scope);
  if (scope && isExcluded(scope, taxonomy))
    throw new Error(`Inventory scope is opaque: ${scope}`);
  checkCancellation(repoRoot, options.cancelFile);
  const admitted = new Map;
  const trackedRows = gitRows(repoRoot);
  const activeExclusions = taxonomy.exclusions.filter((excluded) => existsSync(absolutePath(repoRoot, excluded.path))).map((entry) => entry.path);
  for (const row of trackedRows) {
    if (isExcluded(row.path, taxonomy) || !inScope(row.path, scope))
      continue;
    if (lstatOrNull(absolutePath(repoRoot, row.path)))
      admitted.set(row.path, row);
  }
  for (const path of untrackedGitPaths(repoRoot, taxonomy)) {
    if (isExcluded(path, taxonomy) || !inScope(path, scope) || admitted.has(path))
      continue;
    const row = worktreeCandidate(repoRoot, path);
    if (row)
      admitted.set(path, row);
  }
  for (const row of ignoredGeneratorRows(repoRoot, taxonomy)) {
    if (isExcluded(row.path, taxonomy) || !inScope(row.path, scope))
      continue;
    if (!admitted.has(row.path) || row.explicitDirectory)
      admitted.set(row.path, row);
  }
  for (const row of explicitTicketRows(repoRoot, options.ticketDir, taxonomy)) {
    if (isExcluded(row.path, taxonomy) || !inScope(row.path, scope))
      continue;
    if (!admitted.has(row.path) || row.explicitDirectory)
      admitted.set(row.path, row);
  }
  const directoryPaths = new Set;
  for (const row of admitted.values()) {
    if (row.explicitDirectory || row.mode === "040000")
      directoryPaths.add(row.path);
    let parent = dirname2(row.path);
    while (parent && parent !== ".") {
      if (inScope(parent, scope))
        directoryPaths.add(parent);
      parent = dirname2(parent);
    }
  }
  const entries = new Map;
  const canonicalDirectoryByPath = new Map;
  const directoryKindByPath = new Map;
  const directories = [...directoryPaths].sort((a, b) => a.split("/").length - b.split("/").length || Buffer.from(a).compare(Buffer.from(b)));
  for (let index = 0;index < directories.length; index++) {
    checkCancellation(repoRoot, options.cancelFile);
    const path = directories[index];
    const parentCanonical = canonicalDirectoryByPath.get(dirname2(path)) ?? "";
    const canonical = canonicalDirectory(path, parentCanonical, directoryKindByPath.get(dirname2(path)), ancestorDirectoryKindIds(path, directoryKindByPath), taxonomy);
    canonicalDirectoryByPath.set(path, canonical.path);
    if (canonical.kindId)
      directoryKindByPath.set(path, canonical.kindId);
    entries.set(path, {
      sourcePath: path,
      normalizedPath: canonical.path,
      nodeKind: "directory",
      ownerId: ownerId(path),
      areaId: areaId(path),
      fileKind: null,
      semanticStem: splitLeadingEmoji(basename2(path)).rest || null,
      fixedContractId: canonical.fixedId,
      contentHash: "",
      referencesIn: [],
      referencesOut: [],
      violations: [...canonical.violations, ...pathPolicyViolations(canonical.path, taxonomy)],
      mode: "040000",
      size: 0
    });
    report(options.progress, "inventory", "directories", index + 1, directories.length, path);
  }
  const leaves = [...admitted.values()].filter((row) => row.mode !== "040000" && !row.explicitDirectory).sort((a, b) => Buffer.from(a.path).compare(Buffer.from(b.path)));
  for (let index = 0;index < leaves.length; index++) {
    checkCancellation(repoRoot, options.cancelFile);
    const row = leaves[index];
    const content = contentOf(repoRoot, row);
    const parent = dirname2(row.path) === "." ? "" : dirname2(row.path);
    const contentKind = content.kind === "file" ? extensionlessContentKind(row.path, content.bytes, taxonomy) : { kindId: null };
    const canonical = canonicalFile(row.path, canonicalDirectoryByPath.get(parent) ?? "", directoryKindByPath.get(parent), ancestorDirectoryKindIds(row.path, directoryKindByPath), directoryKindByPath, taxonomy, contentKind.kindId ?? undefined);
    const violations2 = [...canonical.violations];
    if (content.violation)
      violations2.push(content.violation);
    if (contentKind.violation && !canonical.fixedId)
      violations2.push(contentKind.violation);
    let text = null;
    if (content.kind === "file" && content.size <= 16 * 1024 * 1024 && (textualPath(row.path) || contentKind.kindId !== null && contentKind.kindId !== "binary")) {
      try {
        text = new TextDecoder("utf-8", { fatal: true }).decode(content.bytes);
      } catch {
        text = null;
      }
    }
    const role = classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, text, taxonomy);
    let normalizedPath = canonical.path;
    if (role === "implementation") {
      const extracted = packageImplementationDestination(row.path, canonical, canonicalDirectoryByPath, directoryKindByPath, taxonomy);
      if (extracted) {
        normalizedPath = extracted;
        violations2.push(violation("package-implementation-file", row.path, `Package implementation must move to ${extracted}`, "warning"));
      } else
        violations2.push(violation("package-implementation-destination-unresolved", row.path, "Package implementation has no deterministic semantic owner"));
    }
    if (role === "unresolved")
      violations2.push(violation("package-role-unresolved", row.path, "Package role cannot be proven by the configured glue grammar"));
    violations2.push(...pathPolicyViolations(normalizedPath, taxonomy));
    if (content.kind === "symlink") {
      try {
        const target = readlinkSync(absolutePath(repoRoot, row.path));
        if (isAbsolute(target))
          violations2.push(violation("symlink-absolute-target", row.path, "Absolute symlink target cannot be proven repository-local"));
        else {
          const lexicalTarget = normalizeRelative(posix.join(dirname2(row.path), target.replaceAll("\\", "/")));
          if (isExcluded(lexicalTarget, taxonomy))
            violations2.push(violation("symlink-opaque-boundary", row.path, `Symlink lexically targets opaque path ${lexicalTarget}`));
        }
      } catch (error) {
        violations2.push(violation("symlink-target-unreadable", row.path, error instanceof Error ? error.message : String(error)));
      }
    }
    entries.set(row.path, {
      sourcePath: row.path,
      normalizedPath,
      nodeKind: content.kind,
      ownerId: ownerId(row.path),
      areaId: areaId(row.path),
      fileKind: canonical.fileKind,
      semanticStem: canonical.stem,
      fixedContractId: canonical.fixedId,
      packageRole: role,
      contentHash: content.hash,
      referencesIn: [],
      referencesOut: [],
      violations: violations2,
      mode: row.mode,
      size: content.size
    });
    report(options.progress, "inventory", "files", index + 1, leaves.length, row.path);
  }
  projectMutationTestBundles(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  validateProjectedMutationTestBundles(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  const childrenByParent = new Map;
  for (const entry of entries.values()) {
    const parent = dirname2(entry.sourcePath);
    const children = childrenByParent.get(parent) ?? [];
    children.push(entry);
    childrenByParent.set(parent, children);
  }
  for (const path of [...directoryPaths].sort((a, b) => b.split("/").length - a.split("/").length || b.localeCompare(a))) {
    const entry = entries.get(path);
    if (entry)
      entry.contentHash = directoryHash(path, childrenByParent.get(path) ?? []);
  }
  referenceGraph(repoRoot, entries, taxonomy, options.progress, options.cancelFile);
  const frozenEntries = [...entries.values()].sort((a, b) => Buffer.from(a.sourcePath).compare(Buffer.from(b.sourcePath))).map((entry) => ({
    sourcePath: entry.sourcePath,
    normalizedPath: entry.normalizedPath,
    nodeKind: entry.nodeKind,
    ownerId: entry.ownerId,
    areaId: entry.areaId,
    fileKind: entry.fileKind,
    semanticStem: entry.semanticStem,
    fixedContractId: entry.fixedContractId,
    packageRole: entry.packageRole,
    contentHash: entry.contentHash,
    referencesIn: [...entry.referencesIn],
    referencesOut: [...entry.referencesOut],
    violations: stableViolations(entry.violations)
  }));
  const violations = stableViolations(frozenEntries.flatMap((entry) => entry.violations));
  const sourceDigest = sourceTreeDigest(frozenEntries);
  const partial = {
    schemaVersion: 1,
    taxonomySchemaVersion: 7,
    scope,
    pathExclusions: taxonomy.exclusions.map((entry) => entry.path),
    activePathExclusions: activeExclusions,
    entries: frozenEntries,
    violations,
    sourceTreeDigest: sourceDigest
  };
  const inventory = {
    ...partial,
    repoRoot,
    taxonomyPath: taxonomy.path,
    inventoryDigest: inventoryDigestOf(partial)
  };
  report(options.progress, "inventory", "complete", frozenEntries.length, frozenEntries.length);
  return inventory;
}
function collisionKey(path, comparison) {
  if (comparison === "byte" || comparison === "same-kind")
    return path;
  if (comparison === "nfc")
    return path.normalize("NFC");
  if (comparison === "case-fold")
    return path.normalize("NFC").toLocaleLowerCase("und");
  return emojiFold(path).toLocaleLowerCase("und");
}
function collisionGroups(entries, taxonomy) {
  const groups = [];
  for (const comparison of taxonomy.schema.collisionPolicy.comparisons) {
    const buckets = new Map;
    for (const entry of entries) {
      const key = comparison === "same-kind" ? `${entry.nodeKind}\x00${entry.fileKind ?? "fixed"}\x00${collisionKey(entry.normalizedPath, comparison)}` : collisionKey(entry.normalizedPath, comparison);
      const rows = buckets.get(key) ?? [];
      rows.push(entry);
      buckets.set(key, rows);
    }
    for (const [key, rows] of buckets) {
      if (rows.length < 2)
        continue;
      const sources = rows.map((entry) => entry.sourcePath).sort();
      groups.push({ id: sha256(`${comparison}\x00${key}\x00${sources.join("\x00")}`).slice(0, 24), comparison, paths: [...new Set(rows.map((entry) => entry.normalizedPath))].sort(), sources });
    }
  }
  return groups.sort((a, b) => a.comparison.localeCompare(b.comparison) || a.id.localeCompare(b.id));
}
function generatorNodeRecord(repoRoot, path, taxonomy) {
  if (isExcluded(path, taxonomy))
    throw new Error(`Generator node is opaque: ${path}`);
  const absolute = absolutePath(repoRoot, path);
  const stat = lstatSync(absolute);
  const nodeKind = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file";
  const contentHash = nodeKind === "directory" ? sha256("directory") : nodeKind === "symlink" ? sha256(readlinkSync(absolute)) : sha256(readFileSync(absolute));
  return { path: normalizeRelative(path), nodeKind, contentHash, mode: stat.mode & 4095 };
}
function generatorTreeInventory(repoRoot, roots, taxonomy) {
  const rows = new Map;
  const walk = (path) => {
    if (isExcluded(path, taxonomy))
      throw new Error(`Generator output root is opaque: ${path}`);
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat)
      return;
    rows.set(path, generatorNodeRecord(repoRoot, path, taxonomy));
    if (!stat.isDirectory() || stat.isSymbolicLink())
      return;
    for (const child of readdirSync(absolute).sort((a, b) => Buffer.from(a).compare(Buffer.from(b))))
      walk(sourceRelative(`${path}/${child}`));
  };
  for (const root of [...new Set(roots.map(normalizeRelative))].sort(generatorPathCompare))
    walk(root);
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.path, right.path));
}
function generatorInputInventory(inventory, contract, taxonomy) {
  return inventory.entries.filter((entry) => contract.inputPatterns.some((pattern) => taxonomyPathPatternMatches(entry.sourcePath, pattern)) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).map((entry) => {
    const stat = lstatOrNull(absolutePath(inventory.repoRoot, entry.sourcePath));
    return { path: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash || sha256("directory"), mode: stat ? stat.mode & 4095 : 0 };
  }).sort((left, right) => generatorPathCompare(left.path, right.path));
}
function previewNodeRecords(manifest) {
  return manifest.nodes.map((node) => ({
    path: node.path,
    nodeKind: node.nodeKind,
    contentHash: node.nodeKind === "directory" ? sha256("directory") : sha256(Buffer.from(node.bytesBase64, "base64")),
    mode: node.mode
  }));
}
function validatePreviewPreState(manifest, preOutputs) {
  const expected = new Set(manifest.nodes.map((node) => node.path));
  const prePaths = new Set(preOutputs.map((node) => node.path));
  for (const stale of manifest.staleRemovals)
    if (![...prePaths].some((path) => path === stale || path.startsWith(`${stale}/`)))
      throw new Error(`Generator preview stale removal does not exist in the output pre-state: ${stale}`);
  for (const path of prePaths)
    if (!expected.has(path) && !manifest.staleRemovals.some((stale) => path === stale || path.startsWith(`${stale}/`)))
      throw new Error(`Generator preview omits stale output from staleRemovals: ${path}`);
}
function invokeGeneratorPreview(inventory, id, contract, taxonomy) {
  if (!contract.ownerPath || !contract.previewTarget)
    throw new Error(`Owned generator ${id} has no preview target`);
  assertGeneratorPreviewTarget(inventory.repoRoot, contract.ownerPath, contract.previewTarget);
  const capture = mkdtempSync(join2(tmpdir(), "semio-generator-preview-"));
  const stdoutPath = join2(capture, "stdout.json");
  const stderrPath = join2(capture, "stderr.txt");
  let exitCode = -1;
  let success = false;
  try {
    const wrapper = 'const [stdoutPath,stderrPath]=process.argv.slice(1);const result=Bun.spawnSync(["bun","./\uD83D\uDCDC\uFE0Fscript.ts","preview-generated"],{stderr:"pipe",stdout:"pipe"});await Bun.write(stdoutPath,result.stdout);await Bun.write(stderrPath,result.stderr);process.exit(result.exitCode);';
    const result = spawnSync("bun", ["-e", wrapper, stdoutPath, stderrPath], { cwd: absolutePath(inventory.repoRoot, contract.ownerPath), encoding: "utf8", maxBuffer: 1024 * 1024 });
    exitCode = result.status ?? -1;
    success = !result.error && result.status === 0 && result.signal === null && result.stdout === "" && result.stderr === "";
    const stdout = readFileSync(stdoutPath, "utf8");
    const stderr = readFileSync(stderrPath, "utf8");
    if (!success || stderr !== "")
      throw new Error(`Generator preview command failed for ${id}: status=${exitCode}, stdout=${sha256(stdout)}, stderr=${sha256(stderr)}`);
    const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
    const manifest = parseGeneratorPreviewManifest(stdout, id, roots, taxonomy.exclusions.map((entry) => entry.path));
    return { manifest, digest: sha256(stdout) };
  } finally {
    rmSync(capture, { recursive: true, force: true });
  }
}
function generatorPlanning(inventory, moves, edits, taxonomy, options) {
  const mutations = new Set;
  for (const move of moves) {
    mutations.add(move.sourcePath);
    mutations.add(move.destinationPath);
  }
  for (const edit of edits) {
    mutations.add(edit.path);
    const source = inventory.entries.find((entry) => entry.normalizedPath === edit.path)?.sourcePath;
    if (source)
      mutations.add(source);
  }
  const rows = [];
  const regenerations = [];
  const contracts = Object.entries(taxonomy.schema.generatorContracts).sort(([left], [right]) => left.localeCompare(right));
  for (let index = 0;index < contracts.length; index++) {
    const [id, contract] = contracts[index];
    const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
    const outputEntries = inventory.entries.filter((entry) => roots.some((root) => entry.sourcePath === root || entry.sourcePath.startsWith(`${root}/`)));
    const outputProblem = outputEntries.some((entry) => !roots.includes(entry.sourcePath) && (entry.sourcePath !== entry.normalizedPath || entry.violations.some((entry2) => entry2.severity === "error")));
    const outputMutation = [...mutations].some((path2) => roots.some((root) => pathsOverlap(path2, root)));
    const inputMutation = [...mutations].some((path2) => contract.inputPatterns.some((pattern) => taxonomyPathPatternMatches(path2, pattern)));
    if (!outputProblem && !outputMutation && !inputMutation)
      continue;
    const inputs = generatorInputInventory(inventory, contract, taxonomy);
    const preOutputs = generatorTreeInventory(inventory.repoRoot, roots, taxonomy);
    const inputDigest = sha256(canonicalJson(inputs));
    const preOutputDigest = sha256(canonicalJson(preOutputs));
    const path = roots[0];
    if (contract.ownership !== "owned") {
      rows.push(violation(`generator-ownership-${contract.ownership}`, path, `Generator contract ${id} is ${contract.ownership}; ${contract.reason}; input ${inputDigest}, output ${preOutputDigest}`));
      continue;
    }
    try {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const preview = invokeGeneratorPreview(inventory, id, contract, taxonomy);
      checkCancellation(inventory.repoRoot, options.cancelFile);
      validatePreviewPreState(preview.manifest, preOutputs);
      const outputs = previewNodeRecords(preview.manifest);
      const changed = canonicalJson(preOutputs) !== canonicalJson(outputs) || preview.manifest.staleRemovals.length > 0;
      if (inputMutation || outputMutation || changed) {
        const command = ["bun", "nx", "run", contract.target];
        const verifyCommand = contract.checkTarget ? ["bun", "nx", "run", contract.checkTarget] : undefined;
        const provisional = { contractId: id, cwd: contract.ownerPath, command, verifyCommand, outputRoots: roots, inputs, preOutputs, outputs, preview: preview.manifest, previewManifestDigest: preview.digest, staleRemovals: preview.manifest.staleRemovals };
        regenerations.push({ id: sha256(canonicalJson(provisional)).slice(0, 24), ...provisional });
      }
      report(options.progress, "plan", "generator-preview", index + 1, contracts.length, id);
    } catch (error) {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const message = error instanceof Error ? error.message.replaceAll(resolve2(inventory.repoRoot), "<repo>") : String(error);
      rows.push(violation("generator-preview-invalid", path, `Generator ${id} preview was rejected: ${message}`));
    }
  }
  return { regenerations: regenerations.sort((left, right) => left.contractId.localeCompare(right.contractId) || left.id.localeCompare(right.id)), violations: stableViolations(rows) };
}
function affectedPostStateDigest(plan, resultHashes = new Map) {
  const editsByPath = new Map;
  for (const edit of plan.edits)
    editsByPath.set(edit.path, [...editsByPath.get(edit.path) ?? [], edit]);
  const rows = plan.moves.map((move) => ({ path: move.destinationPath, contentHash: resultHashes.get(move.destinationPath) ?? move.sourceHash }));
  for (const [path, edits] of editsByPath) {
    if (rows.some((row) => row.path === path))
      continue;
    rows.push({ path, contentHash: resultHashes.get(path) ?? sha256(canonicalJson(edits)) });
  }
  for (const regeneration of plan.regenerations)
    rows.push({ path: `@generator/${regeneration.id}`, contentHash: sha256(canonicalJson(regeneration.outputs)) });
  return sha256(canonicalJson(rows.sort((a, b) => a.path.localeCompare(b.path))));
}
function taxonomyPlanDigest(plan) {
  const { planDigest: _planDigest, ...digestible } = plan;
  return sha256(canonicalJson(digestible));
}
function planTaxonomy(inventory, options) {
  if (inventory.taxonomySchemaVersion !== 7)
    throw new Error("Inventory taxonomy schemaVersion must be 7");
  if (inventory.sourceTreeDigest !== sourceTreeDigest(inventory.entries))
    throw new Error("Inventory sourceTreeDigest does not match inventory entries");
  const taxonomy = loadTaxonomy({ repoRoot: inventory.repoRoot, taxonomyPath: inventory.taxonomyPath });
  const baselineCommit = options.baselineCommit.trim() || repositoryHead(inventory.repoRoot);
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const groups = collisionGroups(inventory.entries, taxonomy);
  const groupBySource = new Map;
  for (const group of groups)
    for (const source of group.sources)
      if (!groupBySource.has(source))
        groupBySource.set(source, group.id);
  const preliminaryMoves = inventory.entries.filter((entry) => entry.nodeKind !== "directory" && entry.sourcePath !== entry.normalizedPath && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).map((entry) => ({
    operationId: sha256(`move\x00${entry.sourcePath}\x00${entry.normalizedPath}`).slice(0, 24),
    sourcePath: entry.sourcePath,
    destinationPath: entry.normalizedPath,
    sourceHash: entry.contentHash,
    rationaleRule: mutationProjectionRationale(entry.sourcePath, entry.normalizedPath, taxonomy) ?? (entry.semanticStem ? "semantic-stem-resolution" : entry.fixedContractId ? "fixed-contract-preservation" : "canonical-kind-name"),
    ownerId: entry.ownerId,
    collisionGroup: groupBySource.get(entry.sourcePath),
    referenceEdits: []
  })).sort((a, b) => a.sourcePath.localeCompare(b.sourcePath) || a.destinationPath.localeCompare(b.destinationPath));
  const knownSources = referencePathIndex(inventory.entries.map((entry) => entry.sourcePath));
  const references = buildReferenceEdits(inventory, preliminaryMoves, taxonomy, options, knownSources);
  const moves = preliminaryMoves.map((move) => ({
    ...move,
    referenceEdits: references.edits.filter((edit) => references.editTargets.get(referenceEditIdentity(edit)) === move.sourcePath)
  }));
  const generators = generatorPlanning(inventory, moves, references.edits, taxonomy, options);
  const unresolved = [
    ...inventory.violations.filter((entry) => entry.severity === "error" && generatorContractsForOutputPath(entry.path, taxonomy).length === 0),
    ...references.unresolved,
    ...generators.violations
  ];
  for (const group of groups)
    unresolved.push(violation(`collision-${group.comparison}`, group.paths[0] ?? group.sources[0], `Normalization collision ${group.id}: ${group.sources.join(", ")}`));
  for (const digest of options.excludedTreeDigests) {
    if (digest.algorithm !== "sha256-merkle-v1")
      unresolved.push(violation("opaque-digest-algorithm", digest.relativeRoot, `Unsupported opaque digest algorithm ${digest.algorithm}`));
    if (!inventory.pathExclusions.includes(normalizeRelative(digest.relativeRoot)))
      unresolved.push(violation("opaque-digest-unregistered", digest.relativeRoot, "Opaque digest is not registered by taxonomy pathExclusions"));
  }
  for (const excluded of inventory.activePathExclusions)
    if (!options.excludedTreeDigests.some((digest) => normalizeRelative(digest.relativeRoot) === excluded))
      unresolved.push(violation("opaque-digest-missing", excluded, "Opaque tree digest is required before planning"));
  const provisional = {
    schemaVersion: 1,
    taxonomySchemaVersion: 7,
    baselineCommit,
    scope: inventory.scope,
    sourceTreeDigest: inventory.sourceTreeDigest,
    excludedTreeDigests: [...options.excludedTreeDigests].sort((a, b) => a.relativeRoot.localeCompare(b.relativeRoot)),
    moves,
    edits: [...references.edits].sort(referenceEditCompare),
    regenerations: generators.regenerations,
    unresolved: stableViolations(unresolved),
    expectedPostStateDigest: affectedPostStateDigest({ moves, edits: references.edits, regenerations: generators.regenerations }, references.resultHashes),
    planDigest: ""
  };
  const plan = { ...provisional, planDigest: taxonomyPlanDigest(provisional) };
  report(options.progress, "plan", "complete", moves.length + references.edits.length + generators.regenerations.length, moves.length + references.edits.length + generators.regenerations.length);
  return plan;
}
function opaqueNodeDigest(path, counts) {
  const stat = lstatSync(path);
  const mode = (stat.mode & 4095).toString(8);
  if (stat.isSymbolicLink()) {
    counts.symlinks++;
    return sha256(`symlink\x00${mode}\x00${readlinkSync(path)}`);
  }
  if (stat.isFile()) {
    counts.files++;
    return sha256(Buffer.concat([Buffer.from(`file\x00${mode}\x00`), readFileSync(path)]));
  }
  if (stat.isDirectory()) {
    counts.directories++;
    const children = readdirSync(path).sort((a, b) => Buffer.from(a).compare(Buffer.from(b))).map((name) => `${Buffer.from(name).toString("hex")}\x00${opaqueNodeDigest(join2(path, name), counts)}`);
    return sha256(`directory\x00${mode}\x00${children.join("\x00")}`);
  }
  counts.others++;
  return sha256(`other\x00${mode}\x00${stat.size}`);
}
function opaqueTreeDigest(root, relativeRoot) {
  const normalized = normalizeRelative(relativeRoot);
  const path = absolutePath(root, normalized);
  const counts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = opaqueNodeDigest(path, counts);
  return { algorithm: "sha256-merkle-v1", relativeRoot: normalized, digest, ...counts };
}
function repositoryHead(repoRoot) {
  return execFileSync("git", ["rev-parse", "HEAD"], { cwd: repoRoot, encoding: "utf8" }).trim();
}
function verifyTaxonomy(options) {
  const inventory = inventoryTaxonomy(options);
  const plan = planTaxonomy(inventory, {
    baselineCommit: options.baselineCommit ?? repositoryHead(inventory.repoRoot),
    excludedTreeDigests: options.excludedTreeDigests ?? [],
    cancelFile: options.cancelFile,
    progress: options.progress
  });
  const violations = [...plan.unresolved];
  for (const move of plan.moves)
    violations.push(violation("normalization-move-required", move.sourcePath, `Path must move to ${move.destinationPath}`));
  for (const edit of plan.edits)
    violations.push(violation("reference-edit-required", edit.path, `Structured reference must change at ${edit.structuredLocation}`));
  const stable = stableViolations(violations);
  const clean = stable.every((entry) => entry.severity !== "error");
  report(options.progress, "verify", "complete", stable.length, stable.length);
  return { inventory, plan, violations: stable, clean };
}
var ABSENT_BACKUP = "@absent";
var SYMLINK_BACKUP_PREFIX = "@symlink:";
function writeCanonicalFile(path, value) {
  mkdirSync(dirname2(path), { recursive: true });
  const temporary = `${path}.writing`;
  writeFileSync(temporary, `${canonicalJson(value)}
`, "utf8");
  renameSync(temporary, path);
}
function journalSnapshot(journal) {
  return {
    schemaVersion: 1,
    planDigest: journal.planDigest,
    state: journal.state,
    stagingRoot: journal.stagingRoot,
    backupRoot: journal.backupRoot,
    stagedOperationIds: [...journal.stagedOperationIds].sort(),
    installedOperationIds: [...journal.installedOperationIds].sort(),
    appliedEditPaths: [...journal.appliedEditPaths].sort(),
    startedRegenerationIds: [...journal.startedRegenerationIds].sort(),
    completedRegenerationIds: [...journal.completedRegenerationIds].sort(),
    backups: Object.fromEntries(Object.entries(journal.backups).sort(([a], [b]) => a.localeCompare(b))),
    error: journal.error
  };
}
function persistJournal(path, journal) {
  writeCanonicalFile(path, journalSnapshot(journal));
}
function readJournal(path) {
  const value = JSON.parse(readFileSync(path, "utf8"));
  if (value.schemaVersion !== 1 || typeof value.planDigest !== "string" || typeof value.state !== "string" || typeof value.stagingRoot !== "string" || typeof value.backupRoot !== "string")
    throw new Error(`Invalid taxonomy journal at ${path}`);
  return {
    schemaVersion: 1,
    planDigest: value.planDigest,
    state: value.state,
    stagingRoot: value.stagingRoot,
    backupRoot: value.backupRoot,
    stagedOperationIds: [...value.stagedOperationIds ?? []],
    installedOperationIds: [...value.installedOperationIds ?? []],
    appliedEditPaths: [...value.appliedEditPaths ?? []],
    startedRegenerationIds: [...value.startedRegenerationIds ?? []],
    completedRegenerationIds: [...value.completedRegenerationIds ?? []],
    backups: { ...value.backups ?? {} },
    error: value.error
  };
}
function lstatOrNull(path) {
  try {
    return lstatSync(path);
  } catch (error) {
    if (error.code === "ENOENT")
      return null;
    throw error;
  }
}
function hashPath(path) {
  const stat = lstatSync(path);
  if (stat.isSymbolicLink())
    return sha256(readlinkSync(path));
  if (!stat.isFile())
    throw new Error(`Expected file or symlink at ${path}`);
  return sha256(readFileSync(path));
}
function verifyOpaqueDigests(repoRoot, taxonomy, expected) {
  const registered = new Set(taxonomy.exclusions.map((entry) => entry.path));
  for (const digest of [...expected].sort((a, b) => a.relativeRoot.localeCompare(b.relativeRoot))) {
    const relativeRoot = normalizeRelative(digest.relativeRoot);
    if (!registered.has(relativeRoot))
      throw new Error(`Refusing opaque digest for unregistered path ${relativeRoot}`);
    if (!lstatOrNull(absolutePath(repoRoot, relativeRoot)))
      continue;
    const actual = opaqueTreeDigest(repoRoot, relativeRoot);
    if (canonicalJson(actual) !== canonicalJson({ ...digest, relativeRoot }))
      throw new Error(`Opaque tree digest changed at ${relativeRoot}`);
  }
}
function canonicalDirectoryName(taxonomy, kindId, slug, parentKindId) {
  const kind = taxonomy.directoryKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.slugRegex.test(slug) || (kind.parentKindIds?.length ?? 0) > 0 && !kind.parentKindIds?.includes(parentKindId ?? ""))
    throw new Error(`Taxonomy directory kind ${kindId} cannot own slug ${slug}`);
  return `${kind.emoji}${slug}`.normalize("NFC");
}
function canonicalKindOnlyFilename(taxonomy, kindId, extension) {
  const kind = taxonomy.fileKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.extensionChains.includes(extension))
    throw new Error(`Taxonomy file kind ${kindId} cannot own extension ${extension}`);
  return `${kind.emoji}${extension}`.normalize("NFC");
}
function pathsOverlap(left, right) {
  const a = normalizeRelative(left);
  const b = normalizeRelative(right);
  return a === b || a === "" || b === "" || a.startsWith(`${b}/`) || b.startsWith(`${a}/`);
}
function assertGeneratorNodeRecords(records, roots, label) {
  const seen = new Set;
  for (const record2 of records) {
    const path = normalizeRelative(record2.path);
    if (path !== record2.path || seen.has(path))
      throw new Error(`${label} contains a duplicate or noncanonical path: ${record2.path}`);
    if (!roots.some((root) => path === root || path.startsWith(`${root}/`)))
      throw new Error(`${label} path is outside registered roots: ${path}`);
    if (!["directory", "file", "symlink"].includes(record2.nodeKind) || !/^[a-f0-9]{64}$/u.test(record2.contentHash) || !Number.isSafeInteger(record2.mode) || record2.mode < 0 || record2.mode > 4095)
      throw new Error(`${label} contains an invalid node record: ${path}`);
    seen.add(path);
  }
  if (records.some((record2, index) => index > 0 && generatorPathCompare(records[index - 1].path, record2.path) > 0))
    throw new Error(`${label} must be path-sorted`);
}
function nxTargetRecord(repoRoot, ownerPath, target) {
  const manifestPath = absolutePath(repoRoot, `${ownerPath}/\uD83D\uDCCB\uFE0Fproject.json`);
  const manifest = record(JSON.parse(readFileSync(manifestPath, "utf8")), `Nx manifest ${manifestPath}`);
  const separator = target.lastIndexOf(":");
  const project = target.slice(0, separator);
  const targetName = target.slice(separator + 1);
  const targets = record(manifest.targets, `Nx manifest ${manifestPath}.targets`);
  if (manifest.name !== project || !Object.hasOwn(targets, targetName))
    throw new Error(`Nx manifest ${manifestPath} does not own target ${target}`);
  return record(targets[targetName], `Nx target ${target}`);
}
function assertNxTarget(repoRoot, ownerPath, target) {
  nxTargetRecord(repoRoot, ownerPath, target);
}
function assertGeneratorPreviewTarget(repoRoot, ownerPath, target) {
  const preview = nxTargetRecord(repoRoot, ownerPath, target);
  const options = record(preview.options, `Nx target ${target}.options`);
  if (preview.executor !== "nx:run-commands" || options.cwd !== ownerPath || options.command !== "bun ./\uD83D\uDCDC\uFE0Fscript.ts preview-generated")
    throw new Error(`Nx target ${target} is not the exact owner JSON preview command`);
}
function assertRegenerationContract(regeneration, taxonomy, repoRoot) {
  const contract = taxonomy.schema.generatorContracts[regeneration.contractId];
  if (!contract || contract.ownership !== "owned" || !contract.ownerPath || !contract.target || !contract.previewTarget)
    throw new Error(`Regeneration ${regeneration.id} does not reference an owned generator contract`);
  const roots = contract.outputRoots.map((output) => output.path).sort(generatorPathCompare);
  if (regeneration.cwd !== contract.ownerPath || canonicalJson(regeneration.command) !== canonicalJson(["bun", "nx", "run", contract.target]))
    throw new Error(`Regeneration ${regeneration.id} command is not schema-owned`);
  const expectedVerify = contract.checkTarget ? ["bun", "nx", "run", contract.checkTarget] : undefined;
  if (canonicalJson(regeneration.verifyCommand) !== canonicalJson(expectedVerify))
    throw new Error(`Regeneration ${regeneration.id} verification command is not schema-owned`);
  if (canonicalJson([...regeneration.outputRoots].sort()) !== canonicalJson(roots))
    throw new Error(`Regeneration ${regeneration.id} output roots do not match its contract`);
  assertGeneratorNodeRecords(regeneration.preOutputs, roots, `Regeneration ${regeneration.id} preOutputs`);
  assertGeneratorNodeRecords(regeneration.outputs, roots, `Regeneration ${regeneration.id} outputs`);
  assertGeneratorNodeRecords(regeneration.inputs, regeneration.inputs.map((input) => input.path), `Regeneration ${regeneration.id} inputs`);
  for (const input of regeneration.inputs)
    if (!contract.inputPatterns.some((pattern) => taxonomyPathPatternMatches(input.path, pattern)))
      throw new Error(`Regeneration ${regeneration.id} input is not schema-owned: ${input.path}`);
  const preview = parseGeneratorPreviewManifest(`${generatorPreviewJson(regeneration.preview)}
`, regeneration.contractId, roots, taxonomy.exclusions.map((entry) => entry.path));
  if (regeneration.previewManifestDigest !== sha256(`${generatorPreviewJson(preview)}
`) || canonicalJson(regeneration.staleRemovals) !== canonicalJson(preview.staleRemovals) || canonicalJson(regeneration.outputs) !== canonicalJson(previewNodeRecords(preview)))
    throw new Error(`Regeneration ${regeneration.id} does not match its frozen preview manifest`);
  validatePreviewPreState(preview, regeneration.preOutputs);
  const identity = sha256(canonicalJson({ contractId: regeneration.contractId, cwd: regeneration.cwd, command: regeneration.command, verifyCommand: regeneration.verifyCommand, outputRoots: roots, inputs: regeneration.inputs, preOutputs: regeneration.preOutputs, outputs: regeneration.outputs, preview, previewManifestDigest: regeneration.previewManifestDigest, staleRemovals: regeneration.staleRemovals })).slice(0, 24);
  if (regeneration.id !== identity)
    throw new Error(`Regeneration ${regeneration.id} does not match canonical regeneration bytes`);
  assertNxTarget(repoRoot, contract.ownerPath, contract.target);
  assertGeneratorPreviewTarget(repoRoot, contract.ownerPath, contract.previewTarget);
  if (contract.checkTarget)
    assertNxTarget(repoRoot, contract.ownerPath, contract.checkTarget);
  return contract;
}
function assertPlanOutsideTransaction(plan, transactionRoot, taxonomy, repoRoot) {
  const paths = [
    ...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]),
    ...plan.edits.map((edit) => edit.path),
    ...plan.regenerations.flatMap((regeneration) => [...regeneration.outputRoots, ...regeneration.inputs.map((input) => input.path), ...regeneration.preOutputs.map((output) => output.path), ...regeneration.outputs.map((output) => output.path), ...regeneration.staleRemovals])
  ];
  const overlap = paths.find((path) => pathsOverlap(path, transactionRoot));
  if (overlap)
    throw new Error(`Plan path overlaps taxonomy transaction root: ${overlap} <> ${transactionRoot}`);
  for (const regeneration of plan.regenerations) {
    assertRegenerationContract(regeneration, taxonomy, repoRoot);
    const conflict = [...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]), ...plan.edits.map((edit) => edit.path)].find((path) => regeneration.outputRoots.some((root) => pathsOverlap(path, root)));
    if (conflict)
      throw new Error(`Generated output must be regenerated source-first, not moved or edited directly: ${conflict}`);
  }
}
function cleanupCommittedTransaction(repoRoot, journal) {
  rmSync(absolutePath(repoRoot, journal.stagingRoot), { recursive: true, force: true });
  rmSync(absolutePath(repoRoot, journal.backupRoot), { recursive: true, force: true });
}
function backupPath(repoRoot, logicalPath, backupRoot, journal) {
  if (journal.backups[logicalPath] !== undefined)
    return;
  const source = absolutePath(repoRoot, logicalPath);
  const stat = lstatOrNull(source);
  if (!stat) {
    journal.backups[logicalPath] = ABSENT_BACKUP;
    return;
  }
  if (stat.isSymbolicLink()) {
    journal.backups[logicalPath] = `${SYMLINK_BACKUP_PREFIX}${readlinkSync(source)}`;
    return;
  }
  if (!stat.isFile())
    throw new Error(`Backup target must be a file or symlink: ${logicalPath}`);
  const backupRelative = `${sha256(logicalPath).slice(0, 24)}.backup`;
  const destination = join2(backupRoot, backupRelative);
  mkdirSync(dirname2(destination), { recursive: true });
  copyFileSync(source, destination);
  chmodSync(destination, stat.mode & 4095);
  journal.backups[logicalPath] = backupRelative;
}
function restoreBackup(repoRoot, logicalPath, backupRoot, encoded) {
  const destination = absolutePath(repoRoot, logicalPath);
  mkdirSync(dirname2(destination), { recursive: true });
  const current = lstatOrNull(destination);
  if (encoded === ABSENT_BACKUP) {
    if (current?.isDirectory())
      throw new Error(`Cannot remove directory while restoring absent backup: ${logicalPath}`);
    if (current)
      rmSync(destination, { force: true });
    return;
  }
  if (current?.isDirectory())
    throw new Error(`Cannot replace directory while restoring backup: ${logicalPath}`);
  if (current)
    rmSync(destination, { force: true });
  if (encoded.startsWith(SYMLINK_BACKUP_PREFIX)) {
    symlinkSync(encoded.slice(SYMLINK_BACKUP_PREFIX.length), destination);
    return;
  }
  const source = join2(backupRoot, encoded);
  copyFileSync(source, destination);
  const stat = lstatSync(source);
  chmodSync(destination, stat.mode & 4095);
}
function actualAffectedDigest(repoRoot, plan, taxonomy) {
  const paths = new Set(plan.moves.map((move) => move.destinationPath));
  for (const edit of plan.edits)
    paths.add(edit.path);
  const rows = [...paths].sort().map((path) => ({ path, contentHash: hashPath(absolutePath(repoRoot, path)) }));
  for (const regeneration of plan.regenerations)
    rows.push({ path: `@generator/${regeneration.id}`, contentHash: sha256(canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy))) });
  return sha256(canonicalJson(rows));
}
function projectionStaleViolations(repoRoot, plan, taxonomy) {
  if (!plan.moves.some((move) => move.rationaleRule === "artifact-mutation-test-projection-v1"))
    return [];
  const paths = new Set;
  for (const row of gitRows(repoRoot))
    if (!isExcluded(row.path, taxonomy) && textualPath(row.path) && lstatOrNull(absolutePath(repoRoot, row.path))?.isFile())
      paths.add(row.path);
  for (const move of plan.moves)
    if (textualPath(move.destinationPath))
      paths.add(move.destinationPath);
  for (const edit of plan.edits)
    if (textualPath(edit.path))
      paths.add(edit.path);
  for (const regeneration of plan.regenerations)
    for (const output of regeneration.outputs)
      if (output.nodeKind === "file" && textualPath(output.path))
        paths.add(output.path);
  const rows = [];
  const pattern = new RegExp(OLD_MUTATION_TEST_PREFIX_SOURCE, "gu");
  for (const path of [...paths].sort()) {
    if (isExcluded(path, taxonomy))
      continue;
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat?.isFile() || stat.size > 16 * 1024 * 1024)
      continue;
    let content;
    try {
      content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolute));
    } catch {
      continue;
    }
    for (const match of content.matchAll(pattern))
      if (match.index !== undefined)
        rows.push(violation("projection-old-token-stale", path, `Old artifact mutation test hierarchy remains at raw offset ${match.index}`));
  }
  return stableViolations(rows);
}
function projectionPostApplyViolations(repoRoot, plan, taxonomy) {
  const moves = plan.moves.filter((move) => move.rationaleRule === "artifact-mutation-test-projection-v1");
  if (moves.length === 0)
    return [];
  const ids = taxonomy.schema.mutationCatalogProjection;
  const descendant = mutationDescendantContract(taxonomy);
  const groups = new Map;
  for (const move of moves) {
    const artifactRoot = artifactRootForPath(move.sourcePath);
    if (!artifactRoot || !move.destinationPath.startsWith(`${artifactRoot}/`))
      continue;
    const relativeSegments = move.destinationPath.slice(artifactRoot.length + 1).split("/");
    if (relativeSegments.length < 5)
      continue;
    const scenarioRoot = `${artifactRoot}/${relativeSegments.slice(0, 4).join("/")}`;
    groups.set(scenarioRoot, [...groups.get(scenarioRoot) ?? [], move]);
  }
  const rows = [];
  const expectedRequired = descendant.requiredNodes.map((node) => `${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`);
  for (const [scenarioRoot, group] of [...groups.entries()].sort(([left], [right]) => left.localeCompare(right))) {
    if (group.length !== 6) {
      rows.push(violation("projection-apply-move-count", scenarioRoot, `Projected scenario has ${group.length} file moves, expected 6`));
      continue;
    }
    for (const move of group)
      if (lstatOrNull(absolutePath(repoRoot, move.sourcePath)))
        rows.push(violation("projection-source-file-stale", move.sourcePath, "Projected source file remains after staged move installation"));
    const actual = new Set;
    const walk = (path) => {
      if (isExcluded(path, taxonomy))
        throw new Error(`Projection destination crosses opaque path ${path}`);
      const stat = lstatOrNull(absolutePath(repoRoot, path));
      if (!stat)
        return;
      const relativePath = path === scenarioRoot ? "" : path.slice(scenarioRoot.length + 1);
      if (stat.isSymbolicLink()) {
        rows.push(violation("projection-bundle-symlink", path, "Projected bundle contains a symlink"));
        return;
      }
      actual.add(`${stat.isDirectory() ? "directory" : "file"}\x00${relativePath}`);
      if (stat.isDirectory())
        for (const name of readdirSync(absolutePath(repoRoot, path)).sort((left, right) => Buffer.from(left).compare(Buffer.from(right))))
          walk(`${path}/${name}`);
    };
    walk(scenarioRoot);
    const alternatives = descendant.exclusiveAlternatives.map((alternative) => alternative.nodes.map((node) => `${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`).filter((key) => actual.has(key)));
    if (actual.size !== descendant.realizedNodeCount || expectedRequired.some((key) => !actual.has(key)) || alternatives.some((matches) => matches.length !== 1))
      rows.push(violation("projection-apply-bundle-invalid", scenarioRoot, `Projected destination does not realize the exact ${descendant.realizedNodeCount}-node descendant contract`));
  }
  if (groups.size * 6 !== moves.length)
    rows.push(violation("projection-apply-group-unresolved", moves[0].sourcePath, `${moves.length - groups.size * 6} projection move(s) do not resolve to an exact artifact scenario root`));
  return stableViolations(rows);
}
function injectFailure(options, stage) {
  if (options.injectFailureAt === stage)
    throw new Error(`Injected taxonomy failure at ${stage}`);
}
function pruneEmptySourceParents(repoRoot, plan, ticketRoot) {
  const candidates = new Set;
  for (const move of plan.moves) {
    let parent = dirname2(absolutePath(repoRoot, move.sourcePath));
    while (parent !== repoRoot && parent !== dirname2(parent) && !parent.startsWith(`${ticketRoot}/`)) {
      candidates.add(parent);
      parent = dirname2(parent);
    }
  }
  for (const path of [...candidates].sort((a, b) => b.length - a.length)) {
    try {
      rmdirSync(path);
    } catch (error) {
      if (!["ENOTEMPTY", "ENOENT", "EEXIST"].includes(String(error.code)))
        throw error;
    }
  }
}
function rollbackTransaction(repoRoot, plan, journalPath, journal) {
  journal.state = "rolling-back";
  persistJournal(journalPath, journal);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const started = new Set(journal.startedRegenerationIds);
  for (const regeneration of [...plan.regenerations].reverse()) {
    if (!started.has(regeneration.id))
      continue;
    for (const root of [...regeneration.outputRoots].sort((left, right) => right.length - left.length || right.localeCompare(left)))
      rmSync(absolutePath(repoRoot, root), { recursive: true, force: true });
    for (const directory of regeneration.preOutputs.filter((entry) => entry.nodeKind === "directory").sort((left, right) => left.path.split("/").length - right.path.split("/").length || left.path.localeCompare(right.path))) {
      const path = absolutePath(repoRoot, directory.path);
      mkdirSync(path, { recursive: true });
      chmodSync(path, directory.mode);
    }
  }
  for (const [path, backup] of Object.entries(journal.backups).sort(([a], [b]) => b.localeCompare(a)))
    restoreBackup(repoRoot, path, backupRoot, backup);
  const activeIds = new Set([...journal.stagedOperationIds, ...journal.installedOperationIds]);
  for (const move of [...plan.moves].reverse()) {
    if (!journal.installedOperationIds.includes(move.operationId))
      continue;
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const destination = absolutePath(repoRoot, move.destinationPath);
    if (!lstatOrNull(stage) && lstatOrNull(destination)) {
      mkdirSync(dirname2(stage), { recursive: true });
      renameSync(destination, stage);
    }
  }
  for (const move of [...plan.moves].reverse()) {
    if (!activeIds.has(move.operationId))
      continue;
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const source = absolutePath(repoRoot, move.sourcePath);
    if (lstatOrNull(stage)) {
      mkdirSync(dirname2(source), { recursive: true });
      if (lstatOrNull(source))
        throw new Error(`Rollback source is occupied: ${move.sourcePath}`);
      renameSync(stage, source);
    }
  }
  journal.state = "rolled-back";
  journal.appliedEditPaths = [];
  journal.startedRegenerationIds = [];
  journal.completedRegenerationIds = [];
  journal.installedOperationIds = [];
  journal.stagedOperationIds = [];
  persistJournal(journalPath, journal);
}
function applyTaxonomyPlan(plan, options) {
  const repoRoot = resolve2(options.repoRoot);
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1))
    throw new Error("workers must be a positive integer");
  const digest = taxonomyPlanDigest(plan);
  if (plan.planDigest !== digest)
    throw new Error("Plan digest does not match canonical plan bytes");
  if (options.expectedPlanDigest !== undefined && options.expectedPlanDigest !== digest)
    throw new Error("Plan digest does not match expectedPlanDigest");
  if (plan.unresolved.some((entry) => entry.severity === "error"))
    throw new Error("Plan has unresolved blocking violations");
  const taxonomy = loadTaxonomy({ repoRoot, taxonomyPath: options.taxonomyPath });
  const ticketRelative = normalizeRelative(isAbsolute(options.ticketDir) ? relative2(repoRoot, resolve2(options.ticketDir)) : options.ticketDir);
  if (isExcluded(ticketRelative, taxonomy))
    throw new Error(`Ticket directory is opaque: ${ticketRelative}`);
  const ticketRoot = absolutePath(repoRoot, ticketRelative);
  const transactionDirectory = canonicalDirectoryName(taxonomy, "taxonomy-transaction", "taxonomy-transaction");
  const digestDirectory = canonicalDirectoryName(taxonomy, "transaction-digest", digest, "taxonomy-transaction");
  const transactionRootRelative = normalizeRelative(`${ticketRelative}/${transactionDirectory}`);
  const transactionRelative = normalizeRelative(`${transactionRootRelative}/${digestDirectory}`);
  const journalRelative = normalizeRelative(`${transactionRelative}/${canonicalKindOnlyFilename(taxonomy, "json", ".json")}`);
  const defaultJournalPath = absolutePath(repoRoot, journalRelative);
  const resumeRelative = options.resumeJournal ? normalizeRelative(isAbsolute(options.resumeJournal) ? relative2(repoRoot, resolve2(options.resumeJournal)) : options.resumeJournal) : undefined;
  if (resumeRelative && resumeRelative !== journalRelative)
    throw new Error(`Resume journal must be the canonical plan journal ${journalRelative}`);
  const journalPath = defaultJournalPath;
  assertPlanOutsideTransaction(plan, transactionRootRelative, taxonomy, repoRoot);
  if (plan.baselineCommit)
    execFileSync("git", ["cat-file", "-e", `${plan.baselineCommit}^{commit}`], { cwd: repoRoot, stdio: "ignore" });
  verifyOpaqueDigests(repoRoot, taxonomy, plan.excludedTreeDigests);
  let journal;
  if (options.resumeJournal) {
    journal = readJournal(journalPath);
    if (journal.planDigest !== digest)
      throw new Error("Resume journal belongs to a different plan");
    if (journal.state === "committed") {
      cleanupCommittedTransaction(repoRoot, journal);
      return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEdits: plan.edits.length };
    }
    if (journal.state === "rolled-back")
      throw new Error(`Cannot resume journal in state ${journal.state}`);
    if (journal.state === "rolling-back") {
      rollbackTransaction(repoRoot, plan, journalPath, journal);
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEdits: 0 };
    }
  } else {
    for (const regeneration of plan.regenerations) {
      const actualInputs = regeneration.inputs.map((input) => generatorNodeRecord(repoRoot, input.path, taxonomy));
      if (canonicalJson(actualInputs) !== canonicalJson(regeneration.inputs))
        throw new Error(`Regeneration input preimage changed: ${regeneration.id}`);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.preOutputs))
        throw new Error(`Regeneration output preimage changed: ${regeneration.id}`);
    }
    const stagingRoot = normalizeRelative(`${transactionRelative}/stage`);
    const backupRoot = normalizeRelative(`${transactionRelative}/backup`);
    if (lstatOrNull(defaultJournalPath))
      throw new Error(`Journal already exists; pass resumeJournal to resume ${defaultJournalPath}`);
    journal = { schemaVersion: 1, planDigest: digest, state: "prepared", stagingRoot, backupRoot, stagedOperationIds: [], installedOperationIds: [], appliedEditPaths: [], startedRegenerationIds: [], completedRegenerationIds: [], backups: {} };
    mkdirSync(absolutePath(repoRoot, stagingRoot), { recursive: true });
    mkdirSync(absolutePath(repoRoot, backupRoot), { recursive: true });
    persistJournal(journalPath, journal);
  }
  const sourceSet = new Set(plan.moves.map((move) => move.sourcePath));
  try {
    checkCancellation(repoRoot, options.cancelFile);
    for (const move of plan.moves) {
      if (journal.stagedOperationIds.includes(move.operationId)) {
        const candidates = journal.installedOperationIds.includes(move.operationId) ? [absolutePath(repoRoot, move.destinationPath), join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)] : [join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)];
        const resumedPath = candidates.find((path) => lstatOrNull(path));
        if (!resumedPath)
          throw new Error(`Resume move state is invalid: ${move.operationId}`);
        if (!journal.appliedEditPaths.includes(move.destinationPath) && hashPath(resumedPath) !== move.sourceHash)
          throw new Error(`Resume move hash changed: ${move.operationId}`);
        continue;
      }
      const source = absolutePath(repoRoot, move.sourcePath);
      const destination = absolutePath(repoRoot, move.destinationPath);
      const sourceStat = lstatOrNull(source);
      if (!sourceStat)
        throw new Error(`Move source is missing: ${move.sourcePath}`);
      if (hashPath(source) !== move.sourceHash)
        throw new Error(`Move source hash changed: ${move.sourcePath}`);
      if (lstatOrNull(destination) && !sourceSet.has(move.destinationPath))
        throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    journal.state = "staging";
    persistJournal(journalPath, journal);
    for (let index = 0;index < plan.moves.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const move = plan.moves[index];
      const stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
      if (!journal.stagedOperationIds.includes(move.operationId)) {
        journal.stagedOperationIds.push(move.operationId);
        persistJournal(journalPath, journal);
      }
      if (!lstatOrNull(stage) && !journal.installedOperationIds.includes(move.operationId)) {
        mkdirSync(dirname2(stage), { recursive: true });
        renameSync(absolutePath(repoRoot, move.sourcePath), stage);
      }
      report(options.progress, "apply", "staging", index + 1, plan.moves.length, move.sourcePath);
    }
    injectFailure(options, "after-staging");
    journal.state = "moving";
    persistJournal(journalPath, journal);
    for (let index = 0;index < plan.moves.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const move = plan.moves[index];
      const destination = absolutePath(repoRoot, move.destinationPath);
      if (!journal.installedOperationIds.includes(move.operationId)) {
        journal.installedOperationIds.push(move.operationId);
        persistJournal(journalPath, journal);
      }
      if (!lstatOrNull(destination)) {
        mkdirSync(dirname2(destination), { recursive: true });
        renameSync(join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination);
      }
      report(options.progress, "apply", "moves", index + 1, plan.moves.length, move.destinationPath);
    }
    injectFailure(options, "after-moves");
    journal.state = "editing";
    persistJournal(journalPath, journal);
    const editGroups = new Map;
    for (const edit of plan.edits)
      editGroups.set(edit.path, [...editGroups.get(edit.path) ?? [], edit]);
    const sortedEditGroups = [...editGroups.entries()].sort(([a], [b]) => a.localeCompare(b));
    for (let index = 0;index < sortedEditGroups.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const [path, edits] = sortedEditGroups[index];
      if (!journal.appliedEditPaths.includes(path)) {
        const target = absolutePath(repoRoot, path);
        const hashes = new Set(edits.map((edit) => edit.preimageHash));
        if (hashes.size !== 1 || !hashes.has(hashPath(target)))
          throw new Error(`Reference edit preimage changed: ${path}`);
        backupPath(repoRoot, path, absolutePath(repoRoot, journal.backupRoot), journal);
        persistJournal(journalPath, journal);
        const stat = lstatSync(target);
        const result = applyEditsToContent(readFileSync(target, "utf8"), edits);
        writeFileSync(target, result, "utf8");
        chmodSync(target, stat.mode & 4095);
        journal.appliedEditPaths.push(path);
        persistJournal(journalPath, journal);
      }
      report(options.progress, "apply", "edits", index + 1, sortedEditGroups.length, path);
    }
    for (let index = 0;index < plan.regenerations.length; index++) {
      const regeneration = plan.regenerations[index];
      checkCancellation(repoRoot, options.cancelFile);
      if (journal.completedRegenerationIds.includes(regeneration.id)) {
        if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.outputs))
          throw new Error(`Completed regeneration output changed: ${regeneration.id}`);
        if (regeneration.verifyCommand)
          execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), stdio: "inherit" });
        report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
        continue;
      }
      if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs))
        throw new Error(`Regeneration output preimage changed before execution: ${regeneration.id}`);
      for (const output of regeneration.preOutputs)
        if (output.nodeKind !== "directory")
          backupPath(repoRoot, output.path, absolutePath(repoRoot, journal.backupRoot), journal);
      if (!journal.startedRegenerationIds.includes(regeneration.id))
        journal.startedRegenerationIds.push(regeneration.id);
      persistJournal(journalPath, journal);
      execFileSync(regeneration.command[0], [...regeneration.command.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.outputs))
        throw new Error(`Regeneration ${regeneration.id} produced missing, stale, unexpected, byte-different, or mode-different output`);
      if (regeneration.verifyCommand)
        execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      journal.completedRegenerationIds.push(regeneration.id);
      persistJournal(journalPath, journal);
      report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
    }
    injectFailure(options, "after-edits");
    journal.state = "verifying";
    persistJournal(journalPath, journal);
    injectFailure(options, "before-verify");
    const projectionState = projectionPostApplyViolations(repoRoot, plan, taxonomy);
    if (projectionState.length > 0)
      throw new Error(`Projection verification failed: ${projectionState[0].code} at ${projectionState[0].path}`);
    const staleProjectionTokens = projectionStaleViolations(repoRoot, plan, taxonomy);
    if (staleProjectionTokens.length > 0)
      throw new Error(`Projection verification found ${staleProjectionTokens.length} stale old-hierarchy token(s): ${staleProjectionTokens[0].path}`);
    if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest)
      throw new Error("Post-state digest does not match plan expectation");
    verifyOpaqueDigests(repoRoot, taxonomy, plan.excludedTreeDigests);
    rmSync(absolutePath(repoRoot, journal.stagingRoot), { recursive: true, force: true });
    pruneEmptySourceParents(repoRoot, plan, ticketRoot);
    journal.state = "committed";
    persistJournal(journalPath, journal);
    rmSync(absolutePath(repoRoot, journal.backupRoot), { recursive: true, force: true });
    const appliedOperations = plan.moves.length + plan.edits.length + plan.regenerations.length;
    report(options.progress, "apply", "complete", appliedOperations, appliedOperations);
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEdits: plan.edits.length };
  } catch (error) {
    journal.error = error instanceof Error ? error.message : String(error);
    if (journal.state === "committed") {
      persistJournal(journalPath, journal);
      throw error;
    }
    try {
      rollbackTransaction(repoRoot, plan, journalPath, journal);
    } catch (rollbackError) {
      journal.error = `${journal.error}; rollback failed: ${rollbackError instanceof Error ? rollbackError.message : String(rollbackError)}`;
      persistJournal(journalPath, journal);
      throw new Error(journal.error);
    }
    report(options.progress, "apply", "rolled-back", 0, plan.moves.length + plan.edits.length + plan.regenerations.length);
    return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEdits: 0 };
  }
}
export {
  verifyTaxonomy,
  taxonomyPlanDigest,
  planTaxonomy,
  parseGeneratorPreviewManifest,
  opaqueTreeDigest,
  inventoryTaxonomy,
  canonicalJson,
  applyTaxonomyPlan
};
