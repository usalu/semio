import { Database } from "bun:sqlite";

const content = await Bun.file(process.argv[2]).text();
const lines = content.split("\n");
const header = lines.findIndex(line => line.startsWith("parts [") && line.includes("part-2d:REC"));
const end = lines.findIndex((line, index) => index > header && line === "}");
const db = new Database(":memory:");
const calculation = db.query("select ? as parts, ? as rowBytes, ? * ? as minimumBacking, 1048576 as mountedRelease, (? * ?) - 1048576 as excess");
const rows = end - header - 1;
const width = Number(process.argv[3]);
const result = calculation.get(rows, width, rows, width, rows, width) as { parts: number; excess: number };
if (result.parts !== 2880 || result.excess <= 0) throw new Error(JSON.stringify(result));
console.log(JSON.stringify(result));
