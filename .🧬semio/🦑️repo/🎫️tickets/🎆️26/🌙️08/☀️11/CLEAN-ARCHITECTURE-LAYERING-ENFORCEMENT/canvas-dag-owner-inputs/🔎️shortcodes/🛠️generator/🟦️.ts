import { posix, resolve, relative } from "node:path";
import { readdirSync } from "node:fs";

/** 🔖️ Extracts the original literal ASCII suffix after its authored non-ASCII prefix. */
export function iconSourceIdentity(name: string): string {
  if (!name.endsWith(".svg")) throw Error("icon source is not SVG");
  const stem = name.slice(0, -4);
  let start = stem.length;
  while (start) {
    const code = stem.charCodeAt(start - 1);
    if (!(code >= 65 && code <= 90 || code >= 97 && code <= 122 || code >= 48 && code <= 57 || code === 95 || code === 45)) break;
    start--;
  }
  if (!start || start === stem.length) throw Error("icon identity lacks an authored prefix and literal suffix: " + name);
  return stem.slice(start);
}

/** 📁️ Reads the exact authored icon identities without following linked or generated inputs. */
export function collectIconShortcodeSources(base: string, repoRoot: string, signal?: AbortSignal, onProgress?: (directory: string, entries: number) => void): IconShortcodeSource[] {
  const found = new Map<string, string>(), pending = [base];
  while (pending.length) {
    signal?.throwIfAborted();
    const current = pending.pop()!;
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      if (current === base && entry.name === "🤖️generated") continue;
      if (entry.isSymbolicLink()) throw Error("linked icon source is not admitted");
      const path = resolve(current, entry.name);
      if (entry.isDirectory()) pending.push(path);
      else if (entry.isFile() && entry.name.endsWith(".svg")) {
        const key = iconSourceIdentity(entry.name);
        if (found.has(key)) throw Error("duplicate icon identity: " + key + " first=" + found.get(key) + " current=" + path);
        found.set(key, relative(repoRoot, path).replaceAll("\\", "/"));
      }
    }
    onProgress?.(current, found.size);
  }
  return [...found].map(([key, path]) => ({ key, path }));
}

/** 🔣️ One original source asset bound directly by its owned shortcode key. */
export interface IconShortcodeSource { key: string; path: string }
/** 🗂️ Complete owner-local static tables and their exact original asset paths. */
export interface IconShortcodeGenerationInput { directory: string; emoji: Record<string, string>; themed: IconShortcodeSource[]; catalog: IconShortcodeSource[] }
/** 📃️ Generated Rust bindings retain exact source assets without copying them. */
export interface IconShortcodeGeneration { source: string; keys: { emoji: string[]; themed: string[]; catalog: string[] }; bindings: { tier: string; key: string; path: string; literal: string }[] }

/** 🛠️ Generates deterministic UTF-8-keyed Rust tables using direct defining asset paths. */
export function generateIconShortcodeTables(input: IconShortcodeGenerationInput, signal?: AbortSignal): IconShortcodeGeneration {
  const literal = (text: string) => JSON.stringify(text).replace(/\\u([a-f0-9]{4})/gu, "\\u{$1}"), bindings: IconShortcodeGeneration["bindings"] = [], keys = { emoji: [] as string[], themed: [] as string[], catalog: [] as string[] };
  const table = (name: string, entries: [string, string][], tier: keyof typeof keys, asset: boolean) => {
    const unique = new Set<string>();
    entries.sort(([left], [right]) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
    return "const " + name + ": &[(&str, &str)] = &[\n" + entries.map(([key, value]) => {
      signal?.throwIfAborted();
      if (unique.has(key)) throw Error("duplicate shortcode identity: " + tier + ":" + key);
      unique.add(key); keys[tier].push(key);
      if (!asset) return "    (" + literal(key) + ", " + literal(value) + "),";
      const path = posix.normalize(value), relative = posix.relative(input.directory, path);
      if (path !== value || path.startsWith("/") || path === ".." || path.startsWith("../") || !relative) throw Error("invalid defining asset path: " + value);
      const text = literal(relative); bindings.push({ tier, key, path, literal: text });
      return "    (" + literal(key) + ", include_str!(" + text + ")),";
    }).join("\n") + "\n];\n";
  };
  signal?.throwIfAborted();
const source = table("EMOJI", Object.entries(input.emoji), "emoji", false) + table("THEMED", input.themed.map(row => [row.key, row.path]), "themed", true) + table("CATALOG", input.catalog.map(row => [row.key, row.path]), "catalog", true) + "pub(super) fn icon_shortcode_resolve(code: &str) -> Option<IconShortcodeMatch<'static>> { resolve_icon_shortcode(code, [EMOJI, THEMED, CATALOG]) }\n";
  return { source, keys, bindings };
}
