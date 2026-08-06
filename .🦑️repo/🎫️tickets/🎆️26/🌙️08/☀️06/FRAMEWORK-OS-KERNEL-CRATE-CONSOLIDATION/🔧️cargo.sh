#!/usr/bin/env bash
# 🧪️ Pre-registrar cargo runner: temp root member for semio-framework-os-kernel, then restore.
set -euo pipefail
cd /Users/ueli/Documents/semio
TICKET=$(cat /tmp/os-ticket-path.txt)
ROOT=Cargo.toml
MEMBER='    "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",'
cp "$ROOT" "$TICKET/🧪root-Cargo.toml.bak"
trap 'cp "$TICKET/🧪root-Cargo.toml.bak" "$ROOT"' EXIT

export TICKET MEMBER
node <<'NODE'
const fs = require("fs");
const member = process.env.MEMBER;
let root = fs.readFileSync("Cargo.toml", "utf8");
if (root.includes("💻️os/📦️packages/🦀️rust")) {
  console.log("member already present");
  process.exit(0);
}
const needle = "framework/🔨️modules/🧮️math/📦️packages/🦀️rust\",";
const idx = root.indexOf(needle);
if (idx >= 0) {
  const lineEnd = root.indexOf("\n", idx);
  root = root.slice(0, lineEnd + 1) + member + "\n" + root.slice(lineEnd + 1);
} else {
  root = root.replace(/members\s*=\s*\[/, (m) => m + "\n" + member);
}
fs.writeFileSync("Cargo.toml", root);
console.log("inserted member");
NODE

DEVELOPER_DIR=/Library/Developer/CommandLineTools \
  CARGO_TARGET_DIR="$PWD/$TICKET/🧪target" \
  cargo "$@"
