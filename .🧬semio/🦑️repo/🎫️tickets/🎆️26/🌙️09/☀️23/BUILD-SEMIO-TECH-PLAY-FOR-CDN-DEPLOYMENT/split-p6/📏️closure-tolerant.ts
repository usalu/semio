/** 🩹️ Copy of the canonical closure reader that records generated includes instead of refusing (measuring only). */
import { selectRustSourceV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { createHash } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";
import { inspectRustCompileReferences, rustAttributes, rustFindTopLevel, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokens, rustVisibility } from "../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

/** 📎️ Cfg-selected authored input; compiler dep-info must still qualify generated ownership. */
export interface RustSourceInputV1 { readonly path: string; readonly digest: string; readonly bytes: number; readonly activeBytes: number; readonly activeLines: number; readonly kind: "source" | "embedded"; }

/** 🧭️ Resolves authored production mounts with exact cfg and refuses missing or escaped ownership. */
export const generated: string[] = [];
export async function readRustSourceClosureV1(entries: readonly string[], root: string, cfg: readonly string[], read: (path: string) => Promise<Uint8Array | null>): Promise<readonly RustSourceInputV1[]> {
  const files = new Map<string, RustSourceInputV1>(), visited = new Set<string>(), decoder = new TextDecoder();
  const visit = async (path: string, base = dirname(path), embedded = false): Promise<void> => {
    const name = relative(root, path).replaceAll("\\", "/");
    if (name === ".." || name.startsWith("../")) throw Error("Rust source escaped declared workspace: " + path);
    const key = path + "\0" + base;
    if (visited.has(key)) return;
    visited.add(key);
    const bytes = await read(path);
    if (bytes === null) throw Error("Missing Rust source input: " + name);
    const selected = embedded ? "" : selectRustSourceV1(decoder.decode(bytes), cfg);
    files.set(name, { path: name, digest: createHash("sha256").update(bytes).digest("hex"), bytes: bytes.byteLength, activeBytes: embedded ? bytes.byteLength : Buffer.byteLength(selected), activeLines: embedded ? 0 : selected.split("\n").length, kind: embedded ? "embedded" : "source" });
    if (embedded) return;
    for (const reference of inspectRustCompileReferences(selected)) {
      if (reference.kind === "path") continue;
      if (reference.base || reference.expansion) { generated.push(name); continue; }
      await visit(resolve(dirname(path), reference.path), dirname(path), reference.kind !== "include");
    }
    const tokens = rustTokens(selected), pairs = rustTokenPairs(tokens);
    const scope = async (start: number, end: number, moduleBase: string, fileScope: boolean): Promise<void> => {
      for (let index = start; index < end;) {
        const attrs = rustAttributes(tokens, pairs, index, end), visibility = rustVisibility(tokens, pairs, attrs.next), at = visibility.next;
        if (tokens[at]?.text === "mod" && tokens[at + 1]?.kind === "identifier") {
          const name = tokens[at + 1]!.text.replace(/^r#/u, ""), boundary = at + 2;
          const mounts = attrs.ranges.filter(([a]) => tokens[a]?.text === "path").map(([a, b]) => b === a + 3 && tokens[a + 1]?.text === "=" ? rustStringValue(tokens[a + 2]) : null);
          if (mounts.length > 1 || mounts.includes(null)) throw Error("Ambiguous Rust source mount: " + name);
          const mount = mounts[0] ?? undefined;
          if (tokens[boundary]?.text === "{") {
            const close = pairs.get(boundary)!;
            await scope(boundary + 1, close, resolve(moduleBase, mount ?? name), false);
            index = close + 1; continue;
          }
          if (tokens[boundary]?.text === ";") {
            const candidates = mount === undefined ? [join(moduleBase, name + ".rs"), join(moduleBase, name, "mod.rs")] : [resolve(fileScope ? dirname(path) : moduleBase, mount)];
            const present: string[] = [];
            for (const candidate of candidates) if (await read(candidate) !== null) present.push(candidate);
            if (present.length !== 1) throw Error("Missing or ambiguous Rust module: " + name);
            const child = present[0]!;
            await visit(child, mount === undefined && child.endsWith(name + ".rs") ? join(dirname(child), name) : dirname(child));
            index = boundary + 1; continue;
          }
        }
        const boundary = rustFindTopLevel(tokens, pairs, at, end, new Set([";", "{"]));
        if (boundary < 0) break;
        index = (pairs.get(boundary) ?? boundary) + 1;
      }
    };
    await scope(0, tokens.length, base, true);
  };
  for (const entry of entries) await visit(resolve(root, entry));
  return [...files.values()].sort((a, b) => a.path.localeCompare(b.path));
}

/** 🎛️ Evaluates one Rust conditional predicate against the complete compiler cfg snapshot. */
