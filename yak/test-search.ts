#!/usr/bin/env tsx
import { execSync } from "child_process";

const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" search --source https://test.yak.rhino3d.com --all --prerelease semio`, { stdio: "inherit" });
