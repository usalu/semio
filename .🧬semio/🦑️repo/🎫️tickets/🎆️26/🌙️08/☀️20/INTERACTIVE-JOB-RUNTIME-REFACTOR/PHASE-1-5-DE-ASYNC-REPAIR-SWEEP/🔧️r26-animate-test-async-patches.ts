import { realpathSync } from "node:fs";

const root = process.cwd();
const errors = await Bun.file(process.argv[2]).text();
const paths = new Set(
  errors
    .split("\n")
    .filter((line) => line.includes("error["))
    .map((line) => line.slice(0, line.indexOf(":")))
    .filter(Boolean)
    .map((path) => realpathSync(`${root}/${path}`)),
);
const patches: string[] = [];
for (const path of paths) {
  const source = await Bun.file(path).text();
  const hunks: string[] = [];
  for (const match of source.matchAll(/^([\t ]*)#\[test\]\n([\t ]*)fn ([^\n]+)$/gm)) {
    hunks.push(`@@\n-${match[1]}#[test]\n-${match[2]}fn ${match[3]}\n+${match[1]}#[semio_framework_async_macros::async_test]\n+${match[2]}async fn ${match[3]}`);
  }
  if (hunks.length > 0) patches.push(`*** Begin Patch\n*** Update File: ${path}\n${hunks.join("\n")}\n*** End Patch`);
}
process.stdout.write(JSON.stringify(patches));
