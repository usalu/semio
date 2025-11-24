#!/usr/bin/env tsx
import { execSync } from "child_process";

const yak = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";
const version = process.argv[2] || "5.1.0-beta";

execSync(`"${yak}" yank semio ${version}`, { stdio: "inherit" });

console.log(`✅ Yanked semio ${version}`);
