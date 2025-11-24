#!/usr/bin/env tsx
import { execSync } from "child_process";

const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
const packageFile = process.argv[2] || "semio-2.1.0-any-win.yak";

execSync(`"${yak}" push --source https://test.yak.rhino3d.com ${packageFile}`, { stdio: "inherit" });

console.log(`✅ Pushed ${packageFile} to test server`);
