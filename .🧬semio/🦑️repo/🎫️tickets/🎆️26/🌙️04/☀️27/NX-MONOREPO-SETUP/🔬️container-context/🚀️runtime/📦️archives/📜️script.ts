import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, createWriteStream } from "node:fs";
import { join, resolve } from "node:path";
import { Readable } from "node:stream";
import { pipeline } from "node:stream/promises";
import { spawnSync } from "node:child_process";

const workspace = process.cwd(), fixture = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🔣️.json"), "utf8"));
const output = resolve(import.meta.dir, "../../../🗑️generated/container-runtime-archives");
assert.ok(output.includes("NX-MONOREPO-SETUP/🗑️generated/"));
mkdirSync(output, { recursive: true });
const controller = new AbortController(), interrupt = () => controller.abort();
process.once("SIGINT", interrupt); process.once("SIGTERM", interrupt);
try {
  for (const row of fixture.platforms) for (const tool of ["bun", "node"]) {
    const binary = tool === "bun" ? `bun-linux-${row.bunArchive}/bun` : `node-v${fixture.node}-linux-${row.nodeArchive}/bin/node`;
    const name = tool === "bun" ? `bun-linux-${row.bunArchive}.zip` : `node-v${fixture.node}-linux-${row.nodeArchive}.tar.xz`;
    const url = tool === "bun" ? `https://github.com/oven-sh/bun/releases/download/bun-v${fixture.bun}/${name}` : `https://nodejs.org/dist/v${fixture.node}/${name}`;
    const signal = AbortSignal.any([controller.signal, AbortSignal.timeout(180000)]), hash = createHash("sha256"), archive = join(output, name);
    const response = await fetch(url, { signal });
    assert.ok(response.ok && response.body);
    let size = 0;
    const source = Readable.fromWeb(response.body as any);
    source.on("data", (bytes: Buffer) => { size += bytes.length; if (size > 150 * 1024 * 1024) source.destroy(new Error("Archive too large")); else hash.update(bytes); });
    const progress = setInterval(() => console.log(`[DEBUG] ${name} download bytes=${size}`), 10000);
    try { await pipeline(source, createWriteStream(archive), { signal }); } finally { clearInterval(progress); }
    const digest = hash.digest("hex");
    assert.equal(digest, row[`${tool}Sha256`]);
    const oracle = spawnSync("shasum", ["-a", "256", archive], { encoding: "utf8", timeout: 30000 });
    assert.equal(oracle.status, 0, oracle.stderr); assert.equal(oracle.stdout.slice(0, 64), digest);
    const listing = spawnSync(tool === "bun" ? "unzip" : "tar", tool === "bun" ? ["-l", archive, binary] : ["-tJf", archive, binary], { encoding: "utf8", timeout: 30000 });
    assert.equal(listing.status, 0, listing.stderr); assert.ok(listing.stdout.includes(binary));
    console.log(`[DEBUG] ${name} bytes=${size} sha256=${digest} native shasum and archive member PASS`);
  }
} finally { process.off("SIGINT", interrupt); process.off("SIGTERM", interrupt); }
