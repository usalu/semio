import fs from "node:fs";
import crypto from "node:crypto";
export const R = "/Users/ueli/Documents/semio/";
export const sha = (text) => crypto.createHash("sha256").update(text, "utf8").digest("hex");
const state = new Map();
export function load(path) {
  if (!state.has(path)) {
    const exists = fs.existsSync(R + path);
    const text = exists ? fs.readFileSync(R + path, "utf8") : null;
    state.set(path, { before: text, after: text, isNew: !exists });
  }
  return state.get(path).after;
}
export function set(path, text) { load(path); state.get(path).after = text; }
export function create(path, text) {
  if (fs.existsSync(R + path)) throw Error("exists: " + path);
  state.set(path, { before: null, after: text, isNew: true });
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
export function entries() { return [...state.entries()].filter(([, v]) => v.before !== v.after); }
