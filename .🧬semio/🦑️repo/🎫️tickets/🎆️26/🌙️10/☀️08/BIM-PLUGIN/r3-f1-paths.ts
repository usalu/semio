/** 🧭️ Locates the BIM model subset on disk without typing a single emoji path: every directory is found by suffix. */
import { readdirSync } from "node:fs";
import { join } from "node:path";

export const em = (...points: number[]) => String.fromCodePoint(...points) + "️";
export const repo = join(import.meta.dir, "../../../../../../..");
const sub = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);

export const plugin = sub(sub(join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))!), "plugins"), "bim");
export const artifact = sub(sub(plugin, "artifacts"), "model");
export const subsets = sub(sub(artifact, "standards"), "1");
export const subset = join(sub(subsets, "subsets"), readdirSync(sub(subsets, "subsets")).find((n) => n.endsWith("any"))!);
export const child = (parent: string, suffix: string) => sub(parent, suffix);
export const schema = child(subset, "schema");
export const mutations = child(schema, "mutations");
export const fixtures = child(subset, "fixtures");

export const RS = em(0x1f980) + ".rs";
export const JSONF = em(0x1f523) + ".json";
export const TSF = em(0x1f7e6) + ".ts";
export const GQLF = em(0x1f517) + ".graphql";
export const PROTOF = em(0x1f6f0) + ".proto";
export const rel = (path: string) => path.slice(repo.length + 1).replaceAll("\\", "/");
