import fs from "fs";
import path from "path";

const TICKET = fs.readFileSync("/tmp/os-ticket-path.txt","utf8").trim();
const FW = fs.readFileSync("/tmp/fw-path.txt","utf8").trim();
const OS = fs.readFileSync("/tmp/os-path.txt","utf8").trim();
const root = fs.readFileSync("Cargo.toml","utf8");

const keepAdds = [
  `${OS}/📦️packages/🦀️rust`,
  `${OS}/🖥️host/📦️packages/🦀️rust`,
  `${OS}/🔨️modules/🛢️db/📦️packages/🦀️rust`,
  `${OS}/🔨️modules/🔌️plugin/📦️packages/🦀️rust`,
  `${OS}/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust`,
  `${OS}/🔨️modules/🏃️run/📦️packages/🦀️rust`,
  `${OS}/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust`,
  `${OS}/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust`,
];

const lines = root.split("\n");
const out = [];
const removedMembers = [];
const removedDeps = [];
let inMembers = false;
let membersClosed = false;

for (let i = 0; i < lines.length; i++) {
  let line = lines[i];

  // Strip member entries pointing at deleted implementations (os or compiler)
  const memberMatch = line.match(/^\s*"([^"]+)",?\s*$/);
  if (memberMatch && (memberMatch[1].includes("⚡️implementations"))) {
    removedMembers.push(memberMatch[1]);
    continue;
  }

  // Fix workspace.dependencies paths that still point at os implementations
  if (line.includes("path = ") && line.includes("🛍️products/💻️os/") && line.includes("⚡️implementations")) {
    removedDeps.push(line.trim());
    // Retarget known aliases
    if (line.includes("semio-framework-plugin")) {
      line = `semio-framework-plugin = { path = "${OS}/🔨️modules/🔌️plugin/📦️packages/🦀️rust" }`;
    } else if (line.includes("semio-framework-os =")) {
      line = `semio-framework-os = { path = "${OS}/🖥️host/📦️packages/🦀️rust" }`;
    } else if (line.includes("semio-framework-os-kernel-db")) {
      line = line.replace(/path = "[^"]+"/, `path = "${OS}/🔨️modules/🛢️db/📦️packages/🦀️rust"`);
      // db-* leaf aliases → facade package
      line = line.replace(/semio-framework-os-kernel-db-[a-z0-9-]+/, "semio-framework-os-kernel-db");
    } else if (line.includes("semio-framework-os-kernel-neural")) {
      line = line.replace(/path = "[^"]+"/, `path = "${OS}/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust"`);
    } else if (
      line.includes("semio-framework-os-kernel-flow") ||
      line.includes("semio-framework-os-kernel-infinite") ||
      line.includes("semio-framework-os-kernel-playbook") ||
      line.includes("semio-framework-os-kernel-workflow") ||
      line.includes("semio-framework-os-kernel-space") ||
      line.includes("semio-s-kernel-flow")
    ) {
      // Fold into host or drop — retarget to host for now for path validity
      const name = line.match(/^([A-Za-z0-9_-]+)\s*=/)?.[1];
      line = `${name} = { path = "${OS}/🖥️host/📦️packages/🦀️rust", package = "semio-framework-os" }`;
    } else {
      // default: kernel
      const name = line.match(/^([A-Za-z0-9_-]+)\s*=/)?.[1];
      if (name) line = `${name} = { path = "${OS}/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }`;
      else continue;
    }
  }

  out.push(line);
}

let body = out.join("\n");
// Ensure Shape V2 package members are present
const missing = [];
for (const m of keepAdds) {
  if (!body.includes(`"${m}"`)) missing.push(m);
}
if (missing.length) {
  body = body.replace(
    /(\[workspace\][\s\S]*?members\s*=\s*\[)/,
    (m) => m + "\n" + missing.map((x) => `    "${x}",`).join("\n"),
  );
}

const patched = path.join(TICKET, "🧪Cargo.toml.patched-root");
fs.writeFileSync(patched, body);
fs.writeFileSync(path.join(TICKET, "🧪registrar-removed-members.txt"), removedMembers.join("\n")+"\n");
fs.writeFileSync(path.join(TICKET, "🧪registrar-rewritten-deps.txt"), removedDeps.join("\n")+"\n");
console.log("removedMembers="+removedMembers.length);
console.log("rewroteDepsSeen="+removedDeps.length);
console.log("addedMissingMembers="+missing.length);
console.log("wrote "+patched);
