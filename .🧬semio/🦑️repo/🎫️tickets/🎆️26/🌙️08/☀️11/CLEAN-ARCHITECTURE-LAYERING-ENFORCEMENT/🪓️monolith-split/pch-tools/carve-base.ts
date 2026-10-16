// 🕰️ Read-only: for each artifact, finds the commit that first added a carved crate and prints its parent, the last pre-carve tree.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const git = async (...args: string[]) => (await Bun.$`git --no-optional-locks -c core.quotepath=false -c core.fsmonitor=false ${args}`.cwd(ROOT).text()).trim();
for (const artifact of process.argv.slice(2)) {
  const tracked = (await git("ls-files", "--", artifact)).split("\n").filter(p => p.endsWith("/📦️packages/🦀️rust/Cargo.toml") && existsSync(join(ROOT, p)));
  const facadePath = `${artifact}/📦️packages/🦀️rust/Cargo.toml`;
  if (!tracked.includes(facadePath)) { console.log(`${artifact}\tNO-FACADE`); continue; }
  const facade = (Bun.TOML.parse(readFileSync(join(ROOT, facadePath), "utf8")) as any).package.name as string;
  const carved = tracked.filter(p => p !== facadePath && ((Bun.TOML.parse(readFileSync(join(ROOT, p), "utf8")) as any).package?.name ?? "").startsWith(`${facade}-`));
  if (!carved.length) { console.log(`${artifact}\tNO-CARVED`); continue; }
  const lines = (await git("log", "--diff-filter=A", "--format=%H %ct", "--", ...carved)).split("\n").filter(Boolean).map(l => l.split(" "));
  const first = lines.sort((a, b) => Number(a[1]) - Number(b[1]))[0]!;
  const parent = await git("rev-parse", `${first[0]}^`);
  console.log([artifact.split("/").slice(-1)[0], facade, carved.length, first[0]!.slice(0, 11), parent.slice(0, 11)].join("\t"));
}
