#!/usr/bin/env tsx
import { execSync } from "child_process";

const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" login`, { stdio: "inherit" });

console.log("✅ Logged in to Yak");
