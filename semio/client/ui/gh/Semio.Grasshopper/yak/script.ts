#!/usr/bin/env bun
/** 🧭 Yak distribution router: `bun ./script.ts <build|publish|setup|test> [segments…]`. */
import { execFileSync, execSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../../repo/lib/js/src/bundle-script.ts";

const yakWin8 = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
const yakWin7 = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";

class BuildScript extends BundleScript {
  run(): void {
    const distDir = join(this.root, "dist");
    for (const f of [join(distDir, "semio_512x512.png"), join(distDir, "manifest.yml")]) {
      if (existsSync(f)) rmSync(f);
    }
    if (!existsSync(distDir)) mkdirSync(distDir);
    copyFileSync(join(this.root, "..", "..", "..", "..", "..", "assets", "icons", "semio_512x512.png"), join(distDir, "semio_512x512.png"));
    copyFileSync(join(this.root, "manifest.yml"), join(distDir, "manifest.yml"));
    const yak =
      process.platform === "win32" ? yakWin8 : process.platform === "darwin" ? "/Applications/Rhino 8.app/Contents/Resources/bin/yak" : "yak";
    execFileSync(yak, ["build", "--platform", "win"], { cwd: distDir, stdio: "inherit" });
    console.log("✅ Yak package built");
  }
}

class PublishScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] === "yank") {
      const version = segments[1] || "5.1.0-beta";
      execSync(`"${yakWin7}" yank semio ${version}`, { stdio: "inherit" });
      console.log(`✅ Yanked semio ${version}`);
      return;
    }
    if (segments[0] === "unyank") {
      const version = segments[1] || "5.1.0-beta";
      execSync(`"${yakWin7}" unyank semio ${version}`, { stdio: "inherit" });
      console.log(`✅ Unyanked semio ${version}`);
      return;
    }
    const dist = join(this.root, "dist");
    const manifestContent = readFileSync(join(dist, "manifest.yml"), "utf-8");
    const versionMatch = manifestContent.match(/version:\s*(.+)/);
    if (!versionMatch) throw new Error("Could not find version in manifest.yml");
    const version = versionMatch[1]!.trim();
    const buildName = `semio-${version}-rh8_10-win.yak`;
    execSync(`"${yakWin8}" push ${buildName}`, { cwd: dist, stdio: "inherit" });
    console.log("✅ Yak package published");
  }
}

class SetupScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] !== "login") {
      console.error("usage: bun ./script.ts setup login");
      process.exit(1);
    }
    execSync(`"${yakWin8}" login`, { stdio: "inherit" });
    console.log("✅ Logged in to Yak");
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] === "push") {
      const packageFile = segments[1] || "semio-2.1.0-any-win.yak";
      execSync(`"${yakWin8}" push --source https://test.yak.rhino3d.com ${packageFile}`, { stdio: "inherit" });
      console.log(`✅ Pushed ${packageFile} to test server`);
      return;
    }
    if (segments[0] === "search") {
      execSync(`"${yakWin8}" search --source https://test.yak.rhino3d.com --all --prerelease semio`, { stdio: "inherit" });
      return;
    }
    console.error("usage: bun ./script.ts test push|test search …");
    process.exit(1);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("build", BuildScript)
  .register("publish", PublishScript)
  .register("setup", SetupScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
