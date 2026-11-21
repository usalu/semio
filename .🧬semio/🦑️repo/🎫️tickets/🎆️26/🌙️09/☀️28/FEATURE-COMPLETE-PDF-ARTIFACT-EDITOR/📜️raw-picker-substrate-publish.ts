/** 🧾 Stage the verified Rust picker and Store substrate with exact workspace lock updates. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const generated = join(import.meta.dir, "🗑️generated");
const sha = (body: Buffer | string) => createHash("sha256").update(body).digest("hex");
type Overlay = { preimages: Record<string, string | null>; candidate: Record<string, string> };
type PathRow = { path: string; before: string | null; after: string; body: string };
const stages = [
    "raw-store-atomic-overlay.json",
    "raw-spr-group-overlay.json",
    "raw-spr-transport-overlay.json",
    "raw-picker-mounted-overlay.json",
];
const proof = JSON.parse(readFileSync(join(generated, "raw-picker-mounted-measurement.json"), "utf8")) as {
    source: string;
    checks: { os: string; changed: boolean; admitted: boolean }[];
};
const expected = new Map([
    ["raw-store-atomic-overlay.json", "cc6089ee7154376e59c62d31171026b1b00b37c3e0980a2997f25d3ba6784c0f"],
    ["raw-spr-group-overlay.json", "347faae75f34965289ebecb55b3492ed3f77c41abf319613670ffc85c6096cea"],
    ["raw-spr-transport-overlay.json", "44ca3e50acc69c6917a3f8cc52f6a4664bf727dfda64feb38fb2a2c3cabbe696"],
    ["raw-picker-mounted-overlay.json", proof.source],
]);
assert.equal(new Set(proof.checks.map(row => row.os)).size, 4);
assert.ok(proof.checks.every(row => !row.changed || row.admitted));

const rows = new Map<string, PathRow>();
const live = (file: string) => existsSync(file) ? sha(readFileSync(file)) : null;
for (const stage of stages) {
    const raw = readFileSync(join(generated, stage));
    assert.equal(sha(raw), expected.get(stage), stage);
    const overlay = JSON.parse(raw.toString()) as Overlay;
    for (const [path, body] of Object.entries(overlay.candidate)) {
        if (stage === "raw-spr-transport-overlay.json" && path.endsWith(".ts")) continue;
        assert.ok(!rows.has(path), path);
        const before = overlay.preimages[path];
        assert.ok(before === null || /^[0-9a-f]{64}$/.test(before), path);
        rows.set(path, { path, before, after: sha(body), body });
    }
}
const gate = [...rows.values()].find(row => row.body.includes("RAW_PICKER_ENABLED") && /picker-work/.test(row.path));
assert.ok(gate && /const RAW_PICKER_ENABLED:\s*bool\s*=\s*false/.test(gate.body), "Original picker remains gated during substrate installation");

type Manifest = {
    package: { name: string };
    dependencies?: Record<string, { package?: string } | string>;
};
const dependencies = new Map<string, string[]>();
for (const row of rows.values()) {
    if (!row.path.endsWith("/Cargo.toml")) continue;
    assert.equal(live(row.path), row.before, row.path);
    const prev = Bun.TOML.parse(readFileSync(row.path, "utf8")) as Manifest;
    const next = Bun.TOML.parse(row.body) as Manifest;
    assert.equal(prev.package.name, next.package.name);
    const added = Object.entries(next.dependencies ?? {}).filter(([alias]) => !(alias in (prev.dependencies ?? {})))
        .map(([alias, spec]) => typeof spec === "string" ? alias : spec.package ?? alias);
    assert.ok(!Object.keys(prev.dependencies ?? {}).some(alias => !(alias in (next.dependencies ?? {}))), row.path);
    if (added.length) dependencies.set(next.package.name, added);
}

const scan = Bun.spawnSync(["repo", "tools", "exec", "--light", "git", "--", "-c", "core.quotePath=false",
    "ls-files", "-z", "Cargo.lock", "**/Cargo.lock"], { stdout: "pipe", stderr: "inherit" });
assert.equal(scan.exitCode, 0);
const locks = scan.stdout.toString().split("\0").filter(Boolean);
assert.ok(locks.some(file => file === "Cargo.lock"));
const packageName = (text: string) => /^\s*name = "([^"]+)"\n/m.exec(text)?.[1];
const dependenciesIn = (section: string) => {
    const block = /dependencies = \[\n([\s\S]*?)\n\]/.exec(section);
    return block ? [...block[1]!.matchAll(/^ "([^"]+)",$/gm)].map(row => row[1]!) : [];
};
const canonical = new Map<string, string>();
for (const section of readFileSync("Cargo.lock", "utf8").split("[[package]]").slice(1)) {
    const name = packageName(section);
    if (name?.startsWith("semio-") && !/^source = /m.test(section)) {
        assert.ok(!canonical.has(name), name);
        canonical.set(name, section);
    }
}
let lockCount = 0;
const newPackages: Record<string, string[]> = {};
const recognized = new Set<string>();
for (const path of locks) {
    const before = readFileSync(path, "utf8");
    const sections = before.split("[[package]]");
    const local = new Map<string, number>();
    for (const section of sections.slice(1)) {
        const name = packageName(section);
        assert.ok(name, path);
        local.set(name, (local.get(name) ?? 0) + 1);
    }
    const additional = new Map<string, string>();
    function requireLocal(name: string) {
        if (local.has(name)) return;
        const section = canonical.get(name);
        assert.ok(section, path + " lacks the exact locked package " + name);
        local.set(name, 1);
        additional.set(name, section);
        for (const dependency of dependenciesIn(section)) {
            const existing = dependency.split(" ")[0]!;
            if (!local.has(existing)) {
                assert.equal(dependency, existing, path + " requires a specific missing version " + dependency);
                requireLocal(existing);
            }
        }
    }
    for (const [target, added] of dependencies) {
        if (local.has(target)) for (const dependency of added) requireLocal(dependency);
    }
    for (const [name, section] of [...additional.entries()].sort(([left], [right]) => left.localeCompare(right))) {
        const place = sections.findIndex((current, index) => index > 0 && packageName(current)! > name);
        if (place < 0) sections.push(section);
        else sections.splice(place, 0, section);
    }
    if (additional.size) newPackages[path] = [...additional.keys()].sort();
    const updated = sections.map((section, index) => {
        if (index === 0) return section;
        const target = packageName(section)!;
        const added = dependencies.get(target);
        if (!added) return section;
        assert.equal(local.get(target), 1, target);
        recognized.add(target);
        const block = /dependencies = \[\n([\s\S]*?)\n\]/.exec(section);
        assert.ok(block, path + " " + target);
        const entries = dependenciesIn(section);
        const count = entries.length;
        for (const dependency of added) {
            if (entries.some(entry => entry === dependency || entry.startsWith(dependency + " "))) continue;
            assert.equal(local.get(dependency), 1, path + " " + target + " → " + dependency);
            const place = entries.findIndex(entry => entry > dependency);
            if (place < 0) entries.push(dependency);
            else entries.splice(place, 0, dependency);
        }
        if (entries.length === count) return section;
        const content = "dependencies = [\n" + entries.map(entry => ' "' + entry + '",').join("\n") + "\n]";
        return section.replace(block[0], content);
    }).join("[[package]]");
    if (updated !== before) {
        rows.set(path, { path, before: sha(before), after: sha(updated), body: updated });
        lockCount++;
    }
}
assert.deepEqual([...recognized].sort(), [...dependencies.keys()].sort());
for (const row of rows.values()) {
    assert.ok(resolve(row.path).normalize("NFD").length <= 256, row.path);
    assert.equal(live(row.path), row.before, row.path);
}
const receipt = { stages: [...expected], lockCount, newPackages, files: [...rows.values()].map(({ path, before, after }) => ({ path, before, after })) };
writeFileSync(join(generated, "raw-picker-substrate-receipt.json"), JSON.stringify(receipt, null, 2) + "\n");
if (process.argv.includes("--apply")) {
    const scratch = join(generated, "raw-substrate-install");
    mkdirSync(scratch, { recursive: true });
    assert.ok([...rows.values()].every(row => live(row.path) === row.before));
    const ordered = [...rows.values()].sort((a, b) => Number(a.before !== null) - Number(b.before !== null));
    for (const [i, row] of ordered.entries()) {
        const pending = join(scratch, `source-${process.pid}-${i}`);
        writeFileSync(pending, row.body, { flag: "wx", mode: existsSync(row.path) ? statSync(row.path).mode & 0o777 : 0o644 });
        assert.equal(live(row.path), row.before, row.path);
        mkdirSync(dirname(row.path), { recursive: true });
        renameSync(pending, row.path);
        assert.equal(live(row.path), row.after, row.path);
    }
    writeFileSync(join(generated, "raw-picker-substrate-installed.json"), JSON.stringify(receipt, null, 2) + "\n");
}
console.log("[DEBUG] Guarded picker substrate and workspace locks", rows.size, lockCount);
