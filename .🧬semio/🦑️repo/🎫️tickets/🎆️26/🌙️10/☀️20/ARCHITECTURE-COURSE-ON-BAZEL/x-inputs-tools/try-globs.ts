import {readdirSync} from "node:fs";
import {join} from "node:path";
import {declaredModuleGlobsV1} from "./staged/declared-inputs.block.mjs";
const readdir = (d: string) => { try { return readdirSync(d, {withFileTypes: true}).map(e => ({name: e.name, directory: e.isDirectory(), file: e.isFile()})); } catch { return undefined; } };
const root = "/Users/ueli/Documents/semio", project = "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript";
const out = declaredModuleGlobsV1(["default", "{workspaceRoot}/🎓️teaching/🏛️architecture/📚️course/🧪️tests/**/🟦️.ts", "{projectRoot}/../../🏗️builder/**/*.ts", "!{projectRoot}/../../🧪️tests/🧪️deploy/**/*.ts"], root, project, {readdir, Glob: Bun.Glob, join});
console.log(out.length, out.slice(0, 3));
