#!/usr/bin/env node
import fs from "fs";
import path from "path";

const ROOT = "/Users/ueli/Documents/semio";
const OS = path.join(ROOT, "🧰️framework/🛍️products/💻️os");
const CORE_LIB = path.join(ROOT, "🧰️framework/📦️packages/🦀️rust/📦️lib.rs");

const store = fs.readFileSync(path.join(OS, "🔨️modules/🏪️store/🦀️component.rs"), "utf8");
const sprCore = fs.readFileSync(path.join(OS, "🔨️modules/📡️spr/