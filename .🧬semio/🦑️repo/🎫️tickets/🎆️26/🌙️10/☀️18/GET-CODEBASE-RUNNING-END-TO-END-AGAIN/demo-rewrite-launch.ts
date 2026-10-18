/** 🔁️ One-shot rewrite of stale launch routes (`workspace:dev -- <variant>`, `SEMIO_BUILD_MODE=ship`) in launch.json and the seed. Usage: bun demo-rewrite-launch.ts <rows.json> <apply|check> <file...> */
import { readFileSync, writeFileSync } from "node:fs";

const [rowsPath, mode, ...files] = process.argv.slice(2);
const rows: Record<string, { aliases: string[] }> = JSON.parse(readFileSync(rowsPath!, "utf8"));
const names = new Map<string, string>();
for (const [variant, row] of Object.entries(rows)) {
  names.set(variant, variant);
  for (const alias of row.aliases) names.set(alias, variant);
}
const PLAY_PORTS = ["CAD_JS_RENDERER_PLAY_PORT", "PUZZLE_3D_PLAY_PORT", "PUZZLE_5D_PLAY_PORT", "SHOOTING_PLAY_PORT"];
const OWNER = "@semio-tech/framework-os-dev";

function resolve(args: string[]): { variant: string; extra: string[] } {
  for (let n = args.length; n > 0; n--) {
    const variant = names.get(args.slice(0, n).join(" "));
    if (variant) return { variant, extra: args.slice(n) };
  }
  throw Error("Unknown playground variant: " + args.join(" "));
}

function route(args: string[], renderer: string | undefined): { command: string; extra: string[]; variant?: string } {
  if (args[0] === "mcp") {
    const rest = args.slice(1).join(" ");
    if (rest === "") return { command: "workspace:dev-mcp", extra: [] };
    if (rest === "repo") return { command: "workspace:dev-mcp-repo", extra: [] };
    if (rest === "stdio cursor" || rest === "stdio os") return { command: "workspace:dev-mcp -- " + rest, extra: [] };
    if (rest === "http os") return { command: "@semio-tech/framework-os-mcp-rs:dev", extra: [] };
    throw Error("Unmapped mcp route: " + args.join(" "));
  }
  if (renderer !== "react" && renderer !== "wgpu") throw Error("Renderer required for " + args.join(" "));
  const { variant, extra } = resolve(args);
  return { command: `${OWNER}:dev-${variant}-${renderer}-prototype --skip-cache --configuration=prototype`, extra, variant };
}

let total = 0;
for (const file of files) {
  const lines = readFileSync(file, "utf8").split("\n");
  const out: string[] = [];
  let devCount = 0, shipCount = 0;
  for (let i = 0; i < lines.length; i++) {
    const match = /^(\s*)"command": "bun repo run workspace:dev -- ([^"]*)"(,?)$/.exec(lines[i]!);
    if (!match) {
      if (/^\s*"SEMIO_BUILD_MODE": "ship",?$/.test(lines[i]!)) { out.push(lines[i]!.replace('"ship"', '"release"')); shipCount++; } else out.push(lines[i]!);
      continue;
    }
    const indent = match[1]!, open = " ".repeat(indent.length - 2) + "{", close = new RegExp("^" + " ".repeat(indent.length - 2) + "},?$");
    let start = out.length - 1;
    while (start >= 0 && out[start] !== open) start--;
    let end = i + 1;
    while (end < lines.length && !close.test(lines[end]!)) end++;
    if (start < 0 || end >= lines.length) throw Error("Block bounds not found at line " + (i + 1));
    const block = lines.slice(i + 1, end), head = out.slice(start + 1);
    const text = [...head, ...block].join("\n");
    const renderer = /"SEMIO_RENDERER": "(\w+)"/.exec(text)?.[1];
    const target = route(match[2]!.split(" "), renderer);
    out.push(`${indent}"command": "bun repo run ${target.command}"${match[3]}`);
    for (let j = i + 1; j < end; j++) {
      let line = lines[j]!;
      for (const key of PLAY_PORTS) if (line.includes(`"${key}"`)) line = line.replace(`"${key}"`, '"S_OS_PORT"');
      lines[j] = line;
      out.push(line);
      if (target.variant && ["puzzle3d", "puzzle5d"].includes(target.variant) && target.extra.join(" ") === "fixture concrete" && /^\s*"SEMIO_RENDERER": /.test(line)) {
        out.push(line.replace(/"SEMIO_RENDERER": "\w+"/, '"SEMIO_DEFAULT_EXAMPLE": "concrete-forest"'));
      }
    }
    i = end - 1;
    devCount++;
  }
  total += devCount + shipCount;
  console.log(`${file}: workspace:dev entries ${devCount}, ship modes ${shipCount}`);
  if (mode === "apply") writeFileSync(file, out.join("\n"));
}
console.log("total edits", total);
