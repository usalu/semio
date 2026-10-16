import { readFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const productRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🖥️server";
const fixture = JSON.parse(readFileSync(resolve(productRoot, "🧪️tests/🧫️fixtures/🔌️wire/🔣️.json"), "utf8")) as { routes: unknown[]; errors: { kind: string }[] };
function analyse(routerSource: string) {
  const baseRouter = routerSource.slice(routerSource.indexOf("fn base_router<I: ServerInstance>"));
  const block = baseRouter.slice(0, baseRouter.indexOf("\n}"));
  let group = "core";
  const mounted = block.split("\n").flatMap((line) => {
    const condition = /^\s*if (?:groups\.(\w+)|\w+) \{$/u.exec(line.trimEnd());
    if (condition) group = condition[1] ?? "documents";
    if (!line.includes('.route("')) return [];
    const path = /\.route\("([^"]+)"/u.exec(line)![1];
    const verbs = line.slice(line.indexOf(path) + path.length);
    return [...verbs.matchAll(/\b(get|post|put|head)\(/gu)].map(([, verb]) => ({ method: verb.toUpperCase(), path, group }));
  });
  const gateway = routerSource.slice(routerSource.indexOf("pub fn kind(&self) -> &'static str"));
  const kinds = [...gateway.slice(0, gateway.indexOf("\n    }")).matchAll(/=> "(\w+)"/gu)].map(([, kind]) => kind);
  return { mounted, kinds };
}
const oldSource = readFileSync(join(import.meta.dir, "orig/gateway.rs.txt"), "utf8");
const newSource = readModuleTreeSource(resolve(productRoot, "🔨️modules/📡️gateway/🦀️.rs"));
const a = analyse(oldSource), b = analyse(newSource);
console.log("routes old", a.mounted.length, "new", b.mounted.length, "equal", JSON.stringify(a.mounted) === JSON.stringify(b.mounted), "equals fixture", JSON.stringify(b.mounted) === JSON.stringify(fixture.routes));
console.log("kinds old", a.kinds.length, "new", b.kinds.length, "equal", JSON.stringify(a.kinds) === JSON.stringify(b.kinds), "equals fixture", JSON.stringify(b.kinds) === JSON.stringify(fixture.errors.map((e) => e.kind)));
