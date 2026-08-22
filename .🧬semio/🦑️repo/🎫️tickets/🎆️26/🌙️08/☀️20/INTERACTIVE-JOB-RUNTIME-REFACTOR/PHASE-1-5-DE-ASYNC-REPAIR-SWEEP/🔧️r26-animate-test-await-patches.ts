import { realpathSync } from "node:fs";

const root = process.cwd();
const errors = await Bun.file(process.argv[2]).text();
const edits = new Map<string, Map<number, Set<number>>>();
for (const row of errors.split("\n")) {
  const match = row.match(/^(.*):(\d+):(\d+): error\[E0599\]: no method named \`(expect|is_err)\` found for opaque type \`impl Future/);
  if (!match) continue;
  const path = realpathSync(`${root}/${match[1]}`);
  const line = Number(match[2]);
  const column = Number(match[3]);
  const lines = (await Bun.file(path).text()).split("\n");
  const source = lines[line - 1];
  const needle = `.${match[4]}`;
  const offsets = [...source.matchAll(new RegExp(`\\${needle}`, "g"))].map((item) => item.index ?? 0);
  const offset = offsets.reduce((best, value) => Math.abs(value + 2 - column) < Math.abs(best + 2 - column) ? value : best);
  if (source.slice(0, offset).endsWith(".await")) continue;
  if (!edits.has(path)) edits.set(path, new Map());
  if (!edits.get(path)!.has(line)) edits.get(path)!.set(line, new Set());
  edits.get(path)!.get(line)!.add(offset);
}
const patches: string[] = [];
for (const [path, lines] of edits) {
  const source = (await Bun.file(path).text()).split("\n");
  const hunks: string[] = [];
  for (const [line, offsets] of [...lines].sort(([left], [right]) => left - right)) {
    const oldLine = source[line - 1];
    let newLine = oldLine;
    for (const offset of [...offsets].sort((a, b) => b - a)) newLine = `${newLine.slice(0, offset)}.await${newLine.slice(offset)}`;
    hunks.push(`@@
-${oldLine}
+${newLine}`);
  }
  patches.push(`*** Begin Patch
*** Update File: ${path}
${hunks.join("\n")}
*** End Patch`);
}
process.stdout.write(JSON.stringify(patches));
