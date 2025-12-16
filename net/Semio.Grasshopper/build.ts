#!/usr/bin/env tsx
import { execSync } from "child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, rmSync } from "fs";
import { join } from "path";

const cwd = __dirname;


execSync("tsx ./build-value-lists.ts", { cwd, stdio: "inherit" });

const msbuild = "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\MSBuild\\Current\\Bin\\MSBuild.exe";


execSync(`"${msbuild}" Semio.sln /t:Clean`, { cwd, stdio: "inherit" });
execSync(`"${msbuild}" Semio.sln /p:Configuration=Debug`, { cwd, stdio: "inherit" });


const yakDistFolder = join(cwd, "..", "..", "yak", "dist");
if (existsSync(yakDistFolder)) {
  rmSync(yakDistFolder, { recursive: true });
}
mkdirSync(yakDistFolder, { recursive: true });

const binFolder = join(cwd, "bin", "Debug", "net48");
const files = readdirSync(binFolder);
for (const file of files) {
  copyFileSync(join(binFolder, file), join(yakDistFolder, file));
}

console.log("✅ Grasshopper build complete");
