// 🏷️ Lists the full test names (module path + fn) and item-position macro invocations reachable from one Rust test root, walking inline modules, #[path] mounts and include! like rustc. usage: bun names.ts <root file> [--orig]; --orig reads files from tc4-tools/orig (the snapshot before the split) where they exist. Read-only.
import {readFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {rustAttributes, rustStringValue, rustTokenPairs, rustTokens, rustVisibility} from "@semio-tech/compiler-syntax-rust-ts";
import {selectRustSourceV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio/";
const HERE = import.meta.dir;
const [rootFile, flag] = process.argv.slice(2) as [string, string | undefined];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const cfg = [...hostCfg, "debug_assertions", "test"];
const snapshot = new Map<string, string>();
if (flag === "--orig") for (const line of readFileSync(join(HERE, "orig/index.tsv"), "utf8").split("\n").filter(Boolean)) { const [key, , path] = line.split("\t"); snapshot.set(resolve(ROOT, path!), join(HERE, "orig", key!)); }
const read = (path: string): string => readFileSync(snapshot.get(path) ?? path, "utf8");
const out: string[] = [];

const walkFile = (path: string, modulePath: string[], dir: string): void => {
  const selected = selectRustSourceV1(read(path), cfg), tokens = rustTokens(selected), pairs = rustTokenPairs(tokens);
  const scope = (start: number, end: number, mods: string[], base: string, fileScope: boolean): void => {
    for (let index = start; index < end;) {
      if (tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") { index = pairs.get(index + 2)! + 1; continue; }
      const attrs = rustAttributes(tokens, pairs, index, end), visibility = rustVisibility(tokens, pairs, attrs.next);
      let at = visibility.next;
      const isTest = attrs.ranges.some(([a, b]) => { const words = tokens.slice(a, b).map(t => t.text).join(""); return /(^|::)(async_)?test(\(|$)/u.test(words) || words === "test"; });
      while (["async", "unsafe", "const", "extern", "default"].includes(tokens[at]?.text ?? "") && tokens[at + 1]?.text !== "{") at++;
      const word = tokens[at]?.text;
      if (word === "mod" && tokens[at + 1]?.kind === "identifier") {
        const name = tokens[at + 1]!.text.replace(/^r#/u, ""), boundary = at + 2;
        const mounts = attrs.ranges.filter(([a]) => tokens[a]?.text === "path").map(([a]) => rustStringValue(tokens[a + 2]));
        const mount = mounts[0] ?? undefined;
        if (tokens[boundary]?.text === "{") {
          const close = pairs.get(boundary)!;
          scope(boundary + 1, close, [...mods, name], resolve(base, mount ?? name), false);
          index = close + 1; continue;
        }
        if (tokens[boundary]?.text === ";") {
          const candidates = mount === undefined ? [join(base, name + ".rs"), join(base, name, "mod.rs")] : [resolve(fileScope ? dirname(path) : base, mount)];
          const present = candidates.filter(c => { try { read(c); return true; } catch { return false; } });
          if (present.length !== 1) throw Error("module " + name + " in " + path + ": " + present.length);
          const child = present[0]!;
          walkFile(child, [...mods, name], mount === undefined && child.endsWith(name + ".rs") ? join(dirname(child), name) : dirname(child));
          index = boundary + 1; continue;
        }
      }
      if (word === "fn" && tokens[at + 1]?.kind === "identifier") {
        if (isTest) out.push([...mods, tokens[at + 1]!.text].join("::"));
      } else if (tokens[at]?.kind === "identifier" && tokens[at + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[at + 2]?.text ?? "")) {
        const close = pairs.get(at + 2)!;
        if (word === "include" && tokens[at + 3]?.kind !== undefined && rustStringValue(tokens[at + 3]) !== null && close === at + 4) {
          const target = resolve(dirname(path), rustStringValue(tokens[at + 3])!);
          walkFile(target, mods, dirname(target));
        } else out.push("MACRO " + mods.join("::") + "::" + word + "!");
        index = close + 1; if (tokens[index]?.text === ";") index++; continue;
      }
      const boundary = (() => { for (let i = at; i < end; i++) { if (tokens[i]!.text === ";") return i; if (tokens[i]!.text === "{") return i; const c = pairs.get(i); if (c !== undefined && c > i && tokens[i]!.text !== "{") i = c; } return -1; })();
      if (boundary < 0) break;
      index = (tokens[boundary]!.text === "{" ? pairs.get(boundary)! : boundary) + 1;
    }
  };
  scope(0, tokens.length, modulePath, dir, true);
};
walkFile(resolve(rootFile), [], dirname(resolve(rootFile)));
console.log(out.join("\n"));
