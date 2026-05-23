#!/usr/bin/env bun
/**
 * 🧭 Yak distribution router: `bun ./script.ts <build|publish|setup|test> [segments…]`.
 */
import { execFileSync, execSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
const verb = segs[0];
const yakWin8 = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
const yakWin7 = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";

if (verb === "build") {
  const distDir = join(cwd, "dist");
  for (const f of [join(distDir, "semio_512x512.png"), join(distDir, "manifest.yml")]) {
    if (existsSync(f)) rmSync(f);
  }
  if (!existsSync(distDir)) mkdirSync(distDir);
  copyFileSync(join(cwd, "..", "..", "..", "..", "..", "assets", "icons", "semio_512x512.png"), join(distDir, "semio_512x512.png"));
  copyFileSync(join(cwd, "manifest.yml"), join(distDir, "manifest.yml"));
  const yak =
    process.platform === "win32" ? yakWin8 : process.platform === "darwin" ? "/Applications/Rhino 8.app/Contents/Resources/bin/yak" : "yak";
  execFileSync(yak, ["build", "--platform", "win"], { cwd: distDir, stdio: "inherit" });
  console.log("✅ Yak package built");
} else if (verb === "publish" && segs[1] === "yank") {
  const version = segs[2] || "5.1.0-beta";
  execSync(`"${yakWin7}" yank semio ${version}`, { stdio: "inherit" });
  console.log(`✅ Yanked semio ${version}`);
} else if (verb === "publish" && segs[1] === "unyank") {
  const version = segs[2] || "5.1.0-beta";
  execSync(`"${yakWin7}" unyank semio ${version}`, { stdio: "inherit" });
  console.log(`✅ Unyanked semio ${version}`);
} else if (verb === "publish") {
  const dist = join(cwd, "dist");
  const manifestContent = readFileSync(join(dist, "manifest.yml"), "utf-8");
  const versionMatch = manifestContent.match(/version:\s*(.+)/);
  if (!versionMatch) throw new Error("Could not find version in manifest.yml");
  const version = versionMatch[1].trim();
  const buildName = `semio-${version}-rh8_10-win.yak`;
  execSync(`"${yakWin8}" push ${buildName}`, { cwd: dist, stdio: "inherit" });
  console.log("✅ Yak package published");
} else if (verb === "setup" && segs[1] === "login") {
  execSync(`"${yakWin8}" login`, { stdio: "inherit" });
  console.log("✅ Logged in to Yak");
} else if (verb === "test" && segs[1] === "push") {
  const packageFile = segs[2] || "semio-2.1.0-any-win.yak";
  execSync(`"${yakWin8}" push --source https://test.yak.rhino3d.com ${packageFile}`, { stdio: "inherit" });
  console.log(`✅ Pushed ${packageFile} to test server`);
} else if (verb === "test" && segs[1] === "search") {
  execSync(`"${yakWin8}" search --source https://test.yak.rhino3d.com --all --prerelease semio`, { stdio: "inherit" });
} else {
  console.error("usage: bun ./script.ts <build|publish|publish yank|publish unyank|setup login|test push|test search> …");
  process.exit(1);
}
