// 🔎 Looser pass: any text line mentioning the last three segments of a path tax2 renamed (quoted or not), excluding the ticket folder. usage: bun dangling2.ts <pairs.tsv>
import {execFileSync} from "node:child_process";
import {readFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio";
const pairs = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t") as [string, string]);
const tails = new Map<string, string>();
for (const [f] of pairs) tails.set(f.split("/").slice(-3).join("/"), f);
const terms = [...tails.keys()];
for (let i = 0; i < terms.length; i += 10) {
  const args = ["grep", "--untracked", "-I", "-F", "-n"];
  for (const t of terms.slice(i, i + 10)) args.push("-e", t);
  args.push("--", ".", ":!.🧬semio", ":!node_modules", ":!*/node_modules/*", ":!*/target/*", ":!*/📤️dist/*");
  let text = ""; try { text = execFileSync("git", args, {cwd: R, maxBuffer: 1 << 30, stdio: ["ignore", "pipe", "ignore"]}).toString(); } catch (e: any) { text = e.stdout?.toString() ?? ""; }
  for (const line of text.split("\n").filter(Boolean)) console.log(line.slice(0, 260));
}
