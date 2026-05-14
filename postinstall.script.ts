#!/usr/bin/env bun
/** ⚡ Ensures optional lightningcss native binary exists for the current platform. */
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = import.meta.dir;
const pkgPath = join(root, "node_modules", "lightningcss", "package.json");
if (!existsSync(pkgPath)) process.exit(0);

const { version } = JSON.parse(readFileSync(pkgPath, "utf8")) as {
  version: string;
};

const report = process.report?.getReport?.() as
  | { header?: { glibcVersionRuntime?: string } }
  | undefined;
const libc =
  process.platform === "linux"
    ? report?.header?.glibcVersionRuntime
      ? "gnu"
      : "musl"
    : "";
const key = [process.platform, process.arch, libc].filter(Boolean).join("/");
const pkgByKey: Record<string, string> = {
  "win32/x64": "lightningcss-win32-x64-msvc",
  "win32/arm64": "lightningcss-win32-arm64-msvc",
  "darwin/x64": "lightningcss-darwin-x64",
  "darwin/arm64": "lightningcss-darwin-arm64",
  "linux/x64/gnu": "lightningcss-linux-x64-gnu",
  "linux/x64/musl": "lightningcss-linux-x64-musl",
  "linux/arm64/gnu": "lightningcss-linux-arm64-gnu",
  "linux/arm64/musl": "lightningcss-linux-arm64-musl",
};
const platformPkg = pkgByKey[key];
if (!platformPkg) process.exit(0);
if (existsSync(join(root, "node_modules", platformPkg))) process.exit(0);

const spec = `${platformPkg}@${version}`;
execFileSync("bun", ["add", "--no-save", spec], { cwd: root, stdio: "inherit" });
