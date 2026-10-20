import { stripComments, read, rel } from "./closure.ts";
export const WRITE = /\b(writeFileSync|writeFile|appendFileSync|appendFile|writeSync|Bun\.write|mkdirSync|mkdir|cpSync|cp|copyFileSync|copyFile|renameSync|rename|rmSync|rm|unlinkSync|unlink|symlinkSync|symlink|truncateSync|createWriteStream|ensureDirSync|writeJsonSync|writeTextFileSync|linkSync|mkdtempSync|mkdtemp|utimesSync|chmodSync|openSync)\s*[(<]/g;
export type Site = { file: string; line: number; prim: string; text: string };
const cache = new Map<string, Site[]>();
export function sitesOf(abs: string): Site[] {
  let s = cache.get(abs);
  if (s) return s;
  s = [];
  const src = stripComments(read(abs));
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    WRITE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = WRITE.exec(lines[i]!))) {
      if (["cp", "rm", "rename", "mkdir"].includes(m[1]!) && !/\b(fs|promises|fsp|Fs)\.\w*$|(?:^|[^.\w])(cp|rm|rename|mkdir)\(/.test(lines[i]!.slice(0, m.index + m[0].length))) continue;
      s.push({ file: rel(abs), line: i + 1, prim: m[1]!, text: lines[i]!.trim().slice(0, 200) });
    }
  }
  cache.set(abs, s);
  return s;
}
