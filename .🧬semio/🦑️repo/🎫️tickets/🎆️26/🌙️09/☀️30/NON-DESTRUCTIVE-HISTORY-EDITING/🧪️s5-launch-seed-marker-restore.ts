/** 🪧️ Restores the two generator-authored marker comments of `.vscode/🧩️launch.seed.jsonc` that the 2026-10-04 merge resolution
 * (commit 5c7f51ee643) stripped (activation s4-7: `seed file … is missing the devLaunchers marker`). The comment bytes come from the
 * last commit that carried them (`git show 1011cc33cd1:.vscode/🧩️launch.seed.jsonc > 🗑️generated/coord/launch-seed-1011cc33cd1.jsonc`);
 * everything else stays byte-identical. `--check` reports without writing. */
const root = "/Users/ueli/Documents/semio";
const seed = `${root}/.vscode/🧩️launch.seed.jsonc`;
const check = process.argv.includes("--check");
const old = (await Bun.file(`${import.meta.dir}/🗑️generated/coord/launch-seed-1011cc33cd1.jsonc`).text()).split("\n");
/** 🔎️ Returns the comment block (with its leading blank lines) that precedes one top-level key in the old seed. */
function blockBefore(key: string): string[] {
  const at = old.indexOf(`  "${key}": {`);
  if (at === -1) throw new Error(`old seed has no ${key}`);
  let first = at;
  while (first > 0 && (old[first - 1]!.startsWith("  //") || old[first - 1] === "")) first--;
  const block = old.slice(first, at);
  if (!block.some(line => line.startsWith("  //"))) throw new Error(`old seed has no comment before ${key}`);
  return block;
}
const raw = await Bun.file(seed).text();
let next = raw;
let restored = 0;
for (const key of ["devLaunchers", "projectLaunchers"]) {
  const anchor = `\n  "${key}": {`;
  const at = next.indexOf(anchor);
  if (at === -1 || next.indexOf(anchor, at + 1) !== -1) throw new Error(`${key} anchor is not unique`);
  const block = blockBefore(key).join("\n");
  if (next.slice(0, at).endsWith(`\n${block}`)) continue;
  if (next.slice(Math.max(0, at - 200), at).includes("//")) throw new Error(`${key} already carries a different comment`);
  next = `${next.slice(0, at)}\n${block}${next.slice(at)}`;
  restored++;
}
const launch = await Bun.file(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`).text();
const literal = /const DEV_LAUNCHERS_MARKER =\s*\n\s*('(?:[^'\\]|\\.)*');/.exec(launch)?.[1];
if (!literal) throw new Error("DEV_LAUNCHERS_MARKER literal not found");
const marker = new Function(`return ${literal};`)() as string;
if (next.indexOf(marker) === -1 || next.indexOf(marker) !== next.lastIndexOf(marker)) throw new Error("restored seed does not carry exactly one devLaunchers marker");
if (JSON.stringify(Bun.JSONC.parse(next)) !== JSON.stringify(Bun.JSONC.parse(raw))) throw new Error("document value changed");
console.log(`${check ? "would restore" : "restored"} ${restored} marker comment block(s); marker present once`);
if (!check && restored > 0) {
  if ((await Bun.file(seed).text()) !== raw) throw new Error("seed changed while restoring; re-run");
  await Bun.write(seed, next);
}
