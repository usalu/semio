import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const root = process.argv[2] ?? ".";
const skip = (path: string): boolean =>
  /🧭️journeys|🗺️coverage/.test(path) || /[\\/](target|node_modules)[\\/]/.test(path);

const walk = (dir: string, out: string[] = []): string[] => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (skip(path)) continue;
    if (statSync(path).isDirectory()) walk(path, out);
    else if (name.endsWith(".rs")) out.push(path);
  }
  return out;
};

const emojiOf = (text: string): string | undefined => {
  const first = text.trim().split(/\s+/)[0] ?? "";
  if (!/^[#*0-9]️?⃣|\p{Extended_Pictographic}|\p{Regional_Indicator}/u.test(first)) return undefined;
  return first.replace(/[︎️]/g, "");
};

let problems = 0;
for (const file of walk(root)) {
  const lines = readFileSync(file, "utf8").split(/\r?\n/);
  const seen = new Map<string, number>();
  let inBlock = false;
  lines.forEach((line, index) => {
    const match = /^\s*\/\/[\/!](?!\/)\s?(.*)$/.exec(line);
    if (!match) {
      inBlock = false;
      return;
    }
    if (inBlock) return;
    inBlock = true;
    const text = match[1] ?? "";
    if (text.trim() === "") return;
    const emoji = emojiOf(text);
    const shown = relative(".", file).replaceAll("\\", "/");
    if (!emoji) {
      console.log(`MISSING ${shown}:${index + 1} | ${text.slice(0, 90)}`);
      problems++;
      return;
    }
    const earlier = seen.get(emoji);
    if (earlier !== undefined) {
      console.log(`DUP ${shown}:${index + 1} ${emoji} | ${text.slice(0, 90)} (first ${earlier})`);
      problems++;
    } else seen.set(emoji, index + 1);
  });
}
console.log(problems === 0 ? "clean" : `${problems} problems`);
process.exit(problems === 0 ? 0 : 1);
