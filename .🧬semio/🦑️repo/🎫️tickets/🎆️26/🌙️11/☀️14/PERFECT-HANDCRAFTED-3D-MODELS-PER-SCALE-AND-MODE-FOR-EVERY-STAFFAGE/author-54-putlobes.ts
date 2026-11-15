import { readFileSync } from "node:fs";
import { openCtx, saveLists, type Json } from "./kit-lib.ts";
const [spec, sc, file] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const extra = process.argv[5] ? (JSON.parse(readFileSync(process.argv[5], "utf8")) as Json[]) : [];
const c = openCtx(spec, sc);
const lobes: Json[] = JSON.parse(readFileSync(file, "utf8")).solids;
saveLists(c, { body: [...extra, ...lobes] });
console.log(extra.length + lobes.length, "body solids");
