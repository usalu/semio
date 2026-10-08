import { readFileSync, writeFileSync } from "node:fs";
export const edit = (file: string, pairs: [string, string][]) => {
  let s = readFileSync(file, "utf8");
  for (const [from, to] of pairs) { if (!s.includes(from)) throw new Error(`${file}: missing ${from}`); s = s.replace(from, to); }
  writeFileSync(file, s);
};
