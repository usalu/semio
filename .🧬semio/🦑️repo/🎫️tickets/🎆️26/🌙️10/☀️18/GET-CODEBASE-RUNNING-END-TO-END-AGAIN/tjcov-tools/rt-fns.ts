import { instance } from "./rt-next.ts";
const [from, to] = process.argv.slice(2).map(Number);
const re = /\n( *)(?:pub(?:\([a-z]+\))? )?(?:async )?fn ([a-z_0-9]+)/g;
for (const m of instance.matchAll(re)) if (m.index! >= from! && m.index! <= to!) console.log(m.index, m[1]!.length, m[2]);
