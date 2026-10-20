import fs from "node:fs";
import crypto from "node:crypto";
export const R = "/Users/ueli/Documents/semio/";
export const sha = (text) => crypto.createHash("sha256").update(text, "utf8").digest("hex");
export const GROUPS = ["tokio", "json", "wasm", "react"];
const state = new Map();
let group = GROUPS[0];
export function setGroup(name) { if (!GROUPS.includes(name)) throw Error("unknown group " + name); group = name; }
export function load(path) {
  if (!state.has(path)) {
    const exists = fs.existsSync(R + path);
    const text = exists ? fs.readFileSync(R + path, "utf8") : null;
    state.set(path, { before: text, after: text, isNew: !exists, steps: new Map(), alt: new Set() });
  }
  return state.get(path).after;
}
export function set(path, text) { load(path); const entry = state.get(path); entry.after = text; entry.steps.set(group, text); }
export function create(path, text) {
  if (fs.existsSync(R + path)) throw Error("exists: " + path);
  if (state.has(path)) throw Error("created twice: " + path);
  state.set(path, { before: null, after: text, isNew: true, steps: new Map([[group, text]]), alt: new Set() });
}
export function edit(path, oldText, newText) {
  const text = load(path);
  const count = text.split(oldText).length - 1;
  if (count !== 1) throw Error(`anchor occurs ${count} times in ${path}: ${oldText.slice(0, 80)}`);
  set(path, text.replace(oldText, () => newText));
}
export function editAll(path, oldText, newText, expected) {
  const text = load(path);
  const count = text.split(oldText).length - 1;
  if (count !== expected) throw Error(`expected ${expected} occurrences, found ${count} in ${path}: ${oldText.slice(0, 80)}`);
  set(path, text.split(oldText).join(newText));
}
export function replaceBetween(path, start, end, newText) {
  const text = load(path);
  const from = text.indexOf(start);
  if (from < 0 || text.indexOf(start, from + 1) >= 0) throw Error(`start anchor not unique in ${path}: ${start.slice(0, 60)}`);
  const to = text.indexOf(end, from);
  if (to < 0) throw Error(`end anchor missing in ${path}: ${end.slice(0, 60)}`);
  set(path, text.slice(0, from) + newText + text.slice(to + end.length));
}
export function entries() { return [...state.entries()].filter(([, v]) => v.before !== v.after); }
export function stepsOf(path) { return [...state.get(path).steps.entries()].map(([g, after]) => ({ group: g, after })).sort((a, b) => GROUPS.indexOf(a.group) - GROUPS.indexOf(b.group)); }
export function replaceFrom(path, start, newText) {
  const text = load(path);
  const from = text.indexOf(start);
  if (from < 0 || text.indexOf(start, from + 1) >= 0) throw Error(`start anchor not unique in ${path}: ${start.slice(0, 60)}`);
  set(path, text.slice(0, from) + newText);
}
export function rebase(path, text) {
  load(path);
  const entry = state.get(path);
  entry.alt.add(sha(text));
  set(path, text);
}
export function altOf(path) { return [...state.get(path).alt]; }
