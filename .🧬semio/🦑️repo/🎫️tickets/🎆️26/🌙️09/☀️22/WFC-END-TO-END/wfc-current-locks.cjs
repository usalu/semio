/** 🔒️ Reconciles WFC-owned lock carriers with current internal job manifests without changing registry packages. */
const fs = require("node:fs");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END";
const parse = text => new Map(text.split("[[package]]").slice(1).map(block => [block.match(/^\nname = "([^"]+)"/)?.[1], "[[package]]" + block]));
const normalHeadless = '[[package]]\nname = "semio-framework-job-headless"\nversion = "0.1.0"\ndependencies = [\n "semio-framework-async",\n "semio-framework-job",\n "semio-framework-job-payload",\n "semio-framework-job-worker",\n "semio-framework-job-worker-authority",\n "semio-framework-value",\n]\n\n';
const memberHeadless = normalHeadless.replace(' "semio-framework-value",\n', ' "semio-framework-value",\n "serde_json",\n');
let repoLock = fs.readFileSync("Cargo.lock", "utf8");
const orphan = parse(repoLock).get("semio-framework-job-headless");
if (orphan && !fs.readFileSync("Cargo.toml", "utf8").includes("semio-framework-job-headless")) {
    if (orphan.trim() !== memberHeadless.trim()) throw Error("Unexpected concurrent repository-root headless row");
    repoLock = repoLock.replace(orphan, "");
    fs.writeFileSync("Cargo.lock", repoLock);
}
const canonical = parse(repoLock);
canonical.set("semio-framework-job-headless", normalHeadless);
const frameworkFile = "🧰️framework/Cargo.lock";
let frameworkLock = fs.readFileSync(frameworkFile, "utf8");
if (!parse(frameworkLock).has("semio-framework-job-headless")) {
    const next = frameworkLock.indexOf('[[package]]\nname = "semio-framework-job-payload"');
    if (next < 0) throw Error("Framework member headless insertion frontier missing");
    const updated = frameworkLock.slice(0, next) + memberHeadless + frameworkLock.slice(next);
    if (fs.readFileSync(frameworkFile, "utf8") !== frameworkLock) throw Error("Concurrent framework lock change");
    fs.writeFileSync(frameworkFile, updated);
}
const paths = cp.execSync("repo tools exec --light git -- ls-files -z -- **/Cargo.lock Cargo.lock", { encoding: "utf8", maxBuffer: 16 << 20 }).split("\0").filter(file => file.endsWith("Cargo.lock"));
const sha = text => crypto.createHash("sha256").update(text).digest("hex");
const report = [];
const replace = (text, name, block) => {
    const marker = '[[package]]\nname = "' + name + '"\n';
    const start = text.indexOf(marker);
    if (start < 0) {
        const rows = [...text.matchAll(/\[\[package\]\]\nname = "([^"]+)"/g)];
        const next = rows.find(row => row[1] > name);
        const index = next ? next.index : text.length;
        return text.slice(0, index) + block.replace(/\s*$/, "\n\n") + text.slice(index);
    }
    const next = text.indexOf("[[package]]", start + marker.length);
    return text.slice(0, start) + block.replace(/\s*$/, "\n\n") + text.slice(next < 0 ? text.length : next);
};
const dependencies = block => [...(block.match(/dependencies = \[\n([\s\S]*?)\n\]/)?.[1] || "").matchAll(/ "([^"]+)"/g)].map(row => row[1]);
const editDependencies = (text, name, add, remove) => {
    const block = parse(text).get(name);
    if (!block) return text;
    const deps = dependencies(block).filter(dep => !remove.includes(dep));
    for (const dep of add) if (!deps.includes(dep)) deps.push(dep);
    deps.sort();
    const replacement = "dependencies = [\n" + deps.map(dep => ' "' + dep + '",').join("\n") + "\n]";
    const updated = block.replace(/dependencies = \[\n[\s\S]*?\n\]/, replacement);
    return replace(text, name, updated);
};
for (const file of paths) {
    const original = fs.readFileSync(file, "utf8");
    if (!original.includes('name = "semio-s-plugin-wfc-engine-job"')) continue;
    let text = original;
    const changed = [];
    for (const [name] of parse(original)) {
        if (!name?.startsWith("semio-framework-job") || !canonical.has(name)) continue;
        const block = canonical.get(name);
        if (parse(text).get(name).trim() !== block.trim()) { text = replace(text, name, block); changed.push(name); }
    }
    const ownWfc = file === "✏️s/🔌️plugins/🀄️wfc/Cargo.lock";
    text = editDependencies(text, "semio-s-plugin-wfc-engine-job", ownWfc ? ["semio-framework-job-headless", "semio-framework-trace"] : ["semio-framework-job-headless"], ["semio-framework-async", "semio-framework-job-worker", "semio-framework-job-worker-authority", ...(ownWfc ? [] : ["semio-framework-trace", "semio-s-plugin-wfc-engine-oracle", "serde_json"])]);
    text = editDependencies(text, "semio-s-plugin-wfc-engine-oracle", ownWfc ? ["semio-framework-job-testing-caller", "semio-framework-value-testing-allocator", "semio-framework-value"] : ["semio-framework-job-testing-caller", "semio-framework-value-testing-allocator"], ["semio-s-plugin-wfc-engine-job", "semio-framework-value-derive", ...(ownWfc ? [] : ["semio-framework-value", "serde_json"])]);
    let added = true;
    while (added) {
        added = false;
        const present = parse(text);
        for (const [name, block] of present) {
            if (!name?.startsWith("semio-framework-job") && !["semio-s-plugin-wfc-engine-job", "semio-s-plugin-wfc-engine-oracle"].includes(name) && !changed.includes(name)) continue;
            for (const dep of dependencies(block)) {
                const depName = dep.split(" ")[0];
                if (present.has(depName)) continue;
                if (!depName.startsWith("semio-") || !canonical.has(depName)) throw Error("Unresolved non-internal locked dependency: " + file + ":" + dep);
                text = replace(text, depName, canonical.get(depName));
                changed.push(depName);
                added = true;
            }
        }
    }
    const registry = input => input.split("[[package]]").slice(1).filter(block => !block.match(/^\nname = "([^"]+)"/)?.[1]?.startsWith("semio-")).map(block => sha(block.trim())).sort();
    if (JSON.stringify(registry(original)) !== JSON.stringify(registry(text))) throw Error("Registry package multiset changed: " + file);
    if (text === original) continue;
    if (fs.readFileSync(file, "utf8") !== original) throw Error("Concurrent lock carrier changed: " + file);
    fs.writeFileSync(file, text);
    if (fs.readFileSync(file, "utf8") !== text) throw Error("Lock write verification failed: " + file);
    report.push({ file, updatedInternalJobRows: changed, registryPackagesChanged: 0 });
}
fs.mkdirSync(ticket + "/🗑️generated", { recursive: true });
fs.writeFileSync(ticket + "/🗑️generated/current-job-locks.json", JSON.stringify(report, null, 2) + "\n");
fs.appendFileSync(ticket + "/run-r2-native.md", "\n## Internal Job Lock Carriers\n\nReconciled " + report.length + " WFC-owned Cargo.lock carriers with current internal job/context/payload rows and the declared engine-job/oracle edges. Added missing internal original-caller/context rows where required. Registry package blocks were verified unchanged. This is source graph reconciliation, not a native pass; Cargo --locked must verify the final graph. Input procedure: wfc-current-locks.cjs; generated details: 🗑️generated/current-job-locks.json.\n");
console.log("[DEBUG] reconciled " + report.length + " WFC lock carriers; registry package changes=0");
