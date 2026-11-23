import {readFileSync, mkdirSync, writeFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {renderPlaygroundSessionTypeScript} from "@semio-tech/plugin-registry/playground/session";
import {parseDeployedRegistryEntryV1} from "@semio-tech/plugin-registry/discovery/schema";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const catalog = join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
const variant = process.argv[2] ?? "s";
const keep = new Set((process.argv[3] ?? process.env.FULLS_PLUGINS ?? "space,stdio,stdio-office").split(","));
const entries = (JSON.parse(readFileSync(join(catalog, "🔌️plugins.json"), "utf8")) as unknown[]).map(v => parseDeployedRegistryEntryV1(v)).filter(e => keep.has(e.pluginId));
const playgrounds = JSON.parse(readFileSync(join(catalog, "🎠️playgrounds.json"), "utf8"));
const source = renderPlaygroundSessionTypeScript(variant, {entries, playgrounds} as never);
const out = join(repoRoot, ".cache/b/p/m/browser/prototype/sessions", variant, "🎮️playground-session/🟦️.ts");
mkdirSync(join(out, ".."), {recursive: true});
writeFileSync(out, source);
console.log("[DEBUG] wrote", out, "\n" + source);
