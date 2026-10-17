/** 🚧️ Areas owned by running slices or by the parent. An edge is skipped when its importer or its target lies in one of them. */
import { nfc } from "./model.ts";

export const SKIP_PREFIXES: { prefix: string; slice: string }[] = [
  { prefix: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/", slice: "rhu2 (renderer TS)" },
  { prefix: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/", slice: "pi1 (plugin)" },
  { prefix: "✏️s/🔌️plugins/🌀️procedural/", slice: "sf3 (procedural)" },
  { prefix: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", slice: "parent (library)" },
].map(x => ({ prefix: nfc(x.prefix), slice: x.slice }));

export function skipReason(path: string): string | null {
  const f = nfc(path) + "/";
  for (const s of SKIP_PREFIXES) if (f.startsWith(s.prefix)) return s.slice;
  return null;
}

export function crossScope(file: string, target: string): string | null {
  const top = (p: string) => nfc(p).split("/")[0];
  const f = top(file), t = top(target);
  if ((f === "✏️s" || f === "🧰️framework") && t === "🌎️hub") return "cross-scope (the importer's Bun scope has no link to the hub packages)";
  return null;
}

export const edgeSkip = (e: { file: string; resolved: string }) => skipReason(e.file) ?? skipReason(e.resolved) ?? crossScope(e.file, e.resolved);
