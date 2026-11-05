/** 🧾️ Compares every registry package version in current affected locks with one immutable Git revision. */
const fs = require("node:fs");
const cp = require("node:child_process");
const { promisify } = require("node:util");
const crypto = require("node:crypto");
const exec = promisify(cp.execFile);
const repoBinary = process.platform === "win32" ? "repo.exe" : "repo";
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END";
const digest = value => crypto.createHash("sha256").update(value).digest("hex");
const registry = text => text.split("[[package]]").slice(1).filter(block => !block.match(/^\nname = "([^"]+)"/)?.[1]?.startsWith("semio-")).map(block => digest(block.trim())).sort();
(async () => {
    const { stdout: revisionText } = await exec(repoBinary, ["tools", "exec", "--light", "git", "--", "rev-parse", "HEAD"], { encoding: "utf8" });
    const revision = revisionText.trim();
    if (!/^[0-9a-f]{40}$/.test(revision)) throw Error("Expected exact immutable Git revision");
    const { stdout: filesText } = await exec(repoBinary, ["tools", "exec", "--light", "git", "--", "ls-files", "-z", "--", "**/Cargo.lock", "Cargo.lock"], { encoding: "utf8", maxBuffer: 16 << 20 });
    const files = filesText.split("\0").filter(file => file.endsWith("Cargo.lock")).filter(file => file === "🧰️framework/Cargo.lock" || fs.readFileSync(file, "utf8").includes('name = "semio-s-plugin-wfc-engine-job"'));
    const rows = await Promise.all(files.map(async file => {
        const { stdout: original } = await exec(repoBinary, ["tools", "exec", "--light", "git", "--", "show", revision + ":" + file], { encoding: "utf8", maxBuffer: 16 << 20 });
        const current = fs.readFileSync(file, "utf8");
        const before = registry(original), after = registry(current);
        return { file, registryBlocksBefore: before.length, registryBlocksAfter: after.length, allRegistryVersionsUnchanged: JSON.stringify(before) === JSON.stringify(after), currentSha256: digest(current) };
    }));
    const report = { revision, capturedAt: new Date().toISOString(), rows };
    fs.writeFileSync(ticket + "/🗑️generated/registry-lock-audit-current.json", JSON.stringify(report, null, 2) + "\n");
    const unchanged = rows.filter(row => row.allRegistryVersionsUnchanged).length;
    fs.appendFileSync(ticket + "/run-r2-lock-audit.md", "\n## Exhaustive Registry Version Comparison\n\nAt " + report.capturedAt + ", compared raw non-Semio package block multisets, including duplicate names/versions, against immutable Git revision " + revision + ". Unchanged in " + unchanged + "/" + rows.length + " affected lock files. Details are in 🗑️generated/registry-lock-audit-current.json. This strengthens the initial name-keyed assertion; internal-row-only mutation was already enforced by construction.\n\nThe framework member lock is 🧰️framework/Cargo.lock. An initial orphan headless row added to repository-root Cargo.lock was removed exactly; the repository root has no corresponding manifest dependency. External headless consumers omit serde_json dev; external WFC job/oracle rows omit their workspace-only test dependencies. Cargo --locked verification remains authoritative.\n");
    console.log("[DEBUG] all-version registry comparison: " + unchanged + "/" + rows.length + " unchanged");
    if (unchanged !== rows.length) console.log("[DEBUG] concurrent registry differences recorded; no registry edits performed");
})().catch(error => { console.error(error); process.exitCode = 1; });
