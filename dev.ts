#!/usr/bin/env bun
/** 💻 Default dev entry — Semio desktop shell. */
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run", "@semio/desktop:dev"], { cwd: import.meta.dir, stdio: "inherit" });
