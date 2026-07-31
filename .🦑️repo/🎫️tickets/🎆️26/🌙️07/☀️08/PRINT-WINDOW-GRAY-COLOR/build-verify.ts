#!/usr/bin/env bun
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { arch, platform } from "node:os";
import { join } from "node:path";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const printRoot = join(repoRoot, "print");
const texDir = join(printRoot, "tex");
const tectonicVersion = "0.15.0";
const target = `${platform()}-${arch() === "arm64" ? "aarch64" : arch()}`;
const ext = platform() === "win32" ? ".exe" : "";
const tectonic = join(repoRoot, `.repo/cache/tectonic/${tectonicVersion}/tectonic${ext}`);
if (!existsSync(tectonic)) throw new Error(`missing tectonic: ${tectonic}`);
mkdirSync(ticketDir, { recursive: true });
const build = spawnSync(tectonic, ["--keep-logs", "--synctex", `-Z`, `search-path=${texDir}`, "--outdir", ticketDir, "verify.tex"], {
  cwd: ticketDir,
  stdio: "inherit",
  env: { ...process.env, TEXINPUTS: `${texDir}:` },
});
if (build.status !== 0) process.exit(build.status ?? 1);
