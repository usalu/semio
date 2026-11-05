/** 🧾️ Captures WFC worked-file provenance without changing source or git state. */
const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const child = require("node:child_process");
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END";
const inventory = ticket + "/run-r2-file-inventory.md";
const report = ticket + "/run-r2-provenance.md";
const scriptPath = ticket + "/wfc-provenance.cjs";
const generated = ticket + "/🗑️generated";
const outputName = process.argv[2];
if (!outputName || !outputName.endsWith(".json") || !/^[A-Za-z0-9][A-Za-z0-9._-]*$/.test(outputName)) throw Error("Explicit new JSON provenance filename required");
const manifest = generated + "/" + outputName;
if (fs.existsSync(manifest)) throw Error("Provenance output already exists: " + manifest);
const scopes = ["Cargo.lock", "🧰️framework/Cargo.lock", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧪️tests/⏱️deadlines.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧪️tests/📬️responses.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧪️tests/🗃️planning.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧪️tests/🧵️streams.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧬️schema/⏱️deadlines.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧬️schema/📬️responses.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧬️schema/🗃️planning.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧬️schema/🧵️streams.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧬️schema/📬️output.json", "🧰️framework/Cargo.toml", "🧰️framework/🔨️modules/🧵️job/🏃️headless", "✏️s/🔌️plugins/🀄️wfc", "🌎️hub/🧩️compositions/🀄️wfc", ticket, ...["🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🟦️.ts","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧬️schema/🔣️.json","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧪️tests/🔣️.json","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧪️tests/🟦️.test.ts","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🧪️tests/👓️oracle.cjs","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/📥️runtime/🟦️.ts"]];
const previous = fs.existsSync(inventory) ? fs.readFileSync(inventory, "utf8") : "";
const worked = new Map(previous.split("\n").filter(row => row.startsWith("- [")).map(row => [row.slice(7), row.slice(3, 5).replaceAll("_", " ")]));
const command = "repo tools exec --light git -- status --porcelain=v1 -z --untracked-files=all -- " + scopes.map(scope => '"' + scope + '"').join(" ");
const status = child.execSync(command, { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 });
const current = new Map(status.split("\0").filter(row => /^(?:[ MADRCU?!]{2}) /.test(row)).map(row => [row.slice(3), row.slice(0, 2)]));
for (const [file, state] of current) worked.set(file, state);
if (!worked.has("Cargo.lock")) worked.set("Cargo.lock", " M");
for (const file of [inventory, report, scriptPath]) if (!worked.has(file)) worked.set(file, "??");
for (const file of worked.keys()) if (file.includes("/🗑️generated/")) worked.delete(file);
const timestamp = new Date().toISOString();
const rows = [...worked].sort(([a], [b]) => a.localeCompare(b)).map(([file, priorStatus]) => {
  const absolute = path.resolve(file);
  const exists = fs.existsSync(file);
  const state = current.get(file) ?? (exists ? "==" : "D ");
  const kind = priorStatus.includes("D") && !exists ? "Removed" : priorStatus.includes("A") || priorStatus === "??" ? "Created" : "Updated";
  return { path: file, status: state, priorStatus, kind, exists, absoluteUtf16: absolute.length };
});
let body = "# WFC Ticket File Inventory\n\nUnion of the previous worked-file inventory and current scoped git porcelain, captured " + timestamp + ". Generated outputs excluded. Cleaned/committed files remain in this worked list (status ==). No index or source mutation.\n";
for (const kind of ["Created", "Updated", "Removed"]) {
  const group = rows.filter(row => row.kind === kind);
  body += "\n## " + kind + " (" + group.length + ")\n\n" + group.map(row => "- [" + row.status.replaceAll(" ", "_") + "] " + row.path).join("\n") + "\n";
}
fs.writeFileSync(inventory, body);
const contextPaths = ["Cargo.toml","Cargo.lock","✏️s/Cargo.lock","🌎️hub/Cargo.lock","🧰️framework/Cargo.lock","✏️s/🔌️plugins/🀄️wfc/Cargo.lock","✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.lock",...["◻️2d","🔲️grid2d","🖼️bitmap","🧊️3d","🧱️grid3d"].map(kind=>"✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/"+kind+"/Cargo.lock")];
const inputRows = [...rows,...contextPaths.filter(file=>!rows.some(row=>row.path===file)).map(file=>({path:file,exists:fs.existsSync(file),status:"=="}))];
const inputs = inputRows.filter(row => /\.(rs|toml|lock|json|ts|tsx|semio|wit|proto|graphql|cjs|py)$/i.test(row.path)).map(row => {
  if (!row.exists) return { path: row.path, exists: false, status: row.status };
  const before = fs.statSync(row.path, { bigint: true });
  const bytes = fs.readFileSync(row.path);
  const after = fs.statSync(row.path, { bigint: true });
  if (before.size !== after.size || before.mtimeNs !== after.mtimeNs) throw Error("Input changed during capture: " + row.path);
  return { path: row.path, exists: true, status: row.status, bytes: bytes.length, mtimeNs: String(after.mtimeNs), sha256: crypto.createHash("sha256").update(bytes).digest("hex") };
});
const errors = [];
let json = 0;
for (const row of rows.filter(row => row.exists)) {
  const bytes = fs.readFileSync(row.path);
  const textInput = /\.(rs|toml|lock|json|ts|tsx|wit|proto|graphql|cjs|js|py|md|css|html|svg|yml|yaml|txt)$/i.test(row.path) || (row.path.endsWith(".semio") && !row.path.endsWith(".descriptor.semio"));
  if (textInput && bytes.includes(0)) errors.push("NUL " + row.path);
  if (row.path.endsWith(".json")) { json++; try { JSON.parse(bytes.toString("utf8")); } catch (error) { errors.push("JSON " + row.path + " " + error.message); } }
  if (row.absoluteUtf16 > 256) errors.push("Path " + row.path);
}
const maximum = rows.reduce((a, b) => a.absoluteUtf16 > b.absoluteUtf16 ? a : b);
fs.mkdirSync(generated, { recursive: true });
const output = { schema: "semio.wfc.verification-provenance/v1", capturedAt: timestamp, workspace: path.resolve("."), workedFiles: rows.length, inputs, checks: { json, errors, maximumAbsoluteUtf16: maximum.absoluteUtf16, maximumPath: path.resolve(maximum.path) } };
const bytes = JSON.stringify(output, null, 2) + "\n";
fs.writeFileSync(manifest, bytes, { flag: "wx" });
const digest = crypto.createHash("sha256").update(bytes).digest("hex");
fs.writeFileSync(report, "# WFC Verification Provenance\n\nCaptured " + timestamp + " using retained input wfc-provenance.cjs through repo. Worked list is the union of prior inventory and current status, preserving files committed meanwhile.\n\n" + rows.length + " worked files; " + inputs.length + " source/input hashes; " + json + " JSON files parsed; " + errors.length + " validation errors. Maximum absolute path " + maximum.absoluteUtf16 + " UTF16 units.\n\nGenerated hash manifest: "+manifest+". Manifest SHA256: " + digest + ". Per-file SHA256, exact byte size and mtimeNs exist only in that generated manifest.\n\nThis snapshot is evidence for later comparison, not a verification pass. Cargo.lock is captured exactly as observed; a launcher refresh must be compared explicitly and must not be silently accepted as matching source. Run this input again only to create a deliberately refreshed snapshot after preserving the earlier generated manifest for comparison.\n");
console.log(JSON.stringify({ timestamp, workedFiles: rows.length, inputs: inputs.length, json, errors, maximumAbsoluteUtf16: maximum.absoluteUtf16, manifest, manifestSha256: digest }));
