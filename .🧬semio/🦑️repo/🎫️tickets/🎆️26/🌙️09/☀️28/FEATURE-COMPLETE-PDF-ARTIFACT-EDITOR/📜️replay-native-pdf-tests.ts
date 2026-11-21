import { createHash } from "node:crypto";
import { chmodSync, constants, copyFileSync, createReadStream, existsSync, readdirSync, mkdirSync, openSync, closeSync, unlinkSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = process.cwd();
const pdf = resolve(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf");
const ticket = dirname(fileURLToPath(import.meta.url));
const output = resolve(ticket, "🗑️generated");
const scratch = resolve(output, "rt");
const prefix = ".cache/b/r/m/u";
const custom = process.argv[2] === "--published";
const current = process.argv[2] === "--current-product";
const epoch = custom ? process.argv[3] : current ? "current-product" : "all-fixtures";
const lane = custom ? process.argv[4] : current ? "p87f2a5" : "pf748d4";
if (!/^[a-z0-9-]+$/.test(epoch) || !/^p[0-9a-f]{6}$/.test(lane)) throw Error("Expected one governed Cargo log and lane");
const importedEvents = (await Bun.file(resolve(root, prefix, "x", lane + ".jsonl")).text()).trim().split("\n")
    .map(line => JSON.parse(line)).filter(event => event.kind === "publish");
const published = new Set(importedEvents.map(event => event.key));
const cargoLog = await Bun.file(resolve(output, "pdf-editor-native-gate-" + epoch + ".log")).stat();
const { casObjectPathV1, casRootV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts"));
const cargoEnv = Bun.TOML.parse(await Bun.file(resolve(root, ".cargo/config.toml")).text()).env;
const stack = process.env.RUST_MIN_STACK ?? String(cargoEnv.RUST_MIN_STACK);
const separator = process.argv.indexOf("--", 2);
const selectionStart = custom ? 5 : current ? 3 : 2;
const selected = new Set(process.argv.slice(selectionStart, separator < 0 ? undefined : separator));
const testArgs = separator < 0 ? [] : process.argv.slice(separator + 1);
const suites = custom ? await discover() : current ? [
    ["page_editor_tests", "fd/fd5093d148cfe498a8375a2af16c2ec24569cae03856f0fcc5152c736a2320fe/001795205189477-529618.json"],
    ["viewer", "51/51616fe1f755f621df6c6f19f651adffb651141d9cc5fc70903b0098ce755dfe/001795205256098-f8dd94.json"],
] : [
    ["viewer", "fe/fe8beb8a5284a568512c34ed1537a17f68f5171c78efc1d582a9cfcbadf11cef/001795198476812-abcee1.json"],
    ["page_editor_tests", "8e/8ed8842a01685b8ce1cf88067514d5c1cc90ddd51684ecbcdc1e84293505a9b6/001795198502239-4abd0b.json"],
    ["pdf_snapshot_value_tests", "69/69b01ea1ba1cbe97dfeae094afc6ba0bc60f051d690df9f0afda2a5a4dcd3803/001795198657698-eadd66.json"],
    ["graph_projection", "82/82d3ecf3801af80fdbd41aebd71a5b85878ab20b4d3997a6441ce0a4bea768af/001795198673671-6b0385.json"],
];

async function discover(): Promise<string[][]> {
    const selectedSuites = [...selected];
    if (!selectedSuites.length) throw Error("An explicit published PDF test suite is required");
    const result = new Map<string, string>();
    for (const key of published) {
        if (typeof key !== "string" || !/^[0-9a-f]{64}$/.test(key)) continue;
        const directory = resolve(root, prefix, "e", key.slice(0, 2), key);
        if (!existsSync(directory)) continue;
        for (const filename of readdirSync(directory)) {
            const relative = [key.slice(0, 2), key, filename].join("/");
            if (!filename.endsWith(".json")) continue;
            const row = await Bun.file(resolve(directory, filename)).json();
            if (row.createdMs < cargoLog.birthtimeMs || row.createdMs > cargoLog.mtimeMs) continue;
            const name = selectedSuites.find(suite => row.crate === "semio_s_artifact_stdio_pdf_" + suite);
            if (!name || !row.outputs?.some(out => out.executable && new RegExp("^" + row.crate + "-[0-9a-f]+$").test(out.name))) continue;
            if (result.has(name)) throw Error("More than one linked executable for " + name);
            result.set(name, relative);
        }
    }
    if (result.size !== selectedSuites.length) throw Error("Expected all selected compiled Cargo artifacts in the exact publication lane");
    return [...result];
}

async function digest(path: string) {
    if (!(await Bun.file(path).exists())) return null;
    const hash = createHash("sha256");
    for await (const chunk of createReadStream(path)) hash.update(chunk);
    return hash.digest("hex");
}

mkdirSync(scratch, { recursive: true });
for (const [suite, receipt] of suites) {
    if (selected.size && !selected.has(suite)) continue;
    console.log("[DEBUG] Verifying " + suite);
    const entry = await Bun.file(resolve(root, prefix, "e", receipt)).json();
    const name = "semio_s_artifact_stdio_pdf_" + suite;
    if (entry.crate !== name || !published.has(entry.key) || entry.createdMs < cargoLog.birthtimeMs || entry.createdMs > cargoLog.mtimeMs) {
        throw Error("Compilation receipt does not match " + suite);
    }
    for (const input of entry.dependencies) {
        if (await digest(resolve(pdf, input.path)) !== input.sha256) throw Error("Source changed: " + input.path);
    }
    const artifact = entry.outputs.find(row => row.executable && new RegExp("^" + name + "-[0-9a-f]+$").test(row.name));
    if (!artifact) throw Error("Linked test executable is absent: " + suite);
    const casPath = casObjectPathV1(casRootV1(), artifact.sha256);
    const executable = resolve(scratch, suite);
    copyFileSync(casPath, executable, constants.COPYFILE_FICLONE);
    try {
        if (await digest(executable) !== artifact.sha256) throw Error("Executable digest differs from receipt: " + suite);
        chmodSync(executable, 0o700);
        const logfile = openSync(resolve(output, "native-replay-" + ((current || custom) ? epoch + "-" : "") + suite + (testArgs.length ? "-focused" : "") + ".log"), "w");
        try {
            const child = Bun.spawn([executable, "--test-threads=1", ...testArgs], {
                cwd: dirname(resolve(pdf, entry.dependencies[0].path)),
                env: { ...process.env, RUST_MIN_STACK: stack, TMPDIR: scratch, TMP: scratch, TEMP: scratch },
                stdout: logfile,
                stderr: logfile,
            });
            const abort = () => child.kill("SIGTERM");
            process.once("SIGINT", abort);
            process.once("SIGTERM", abort);
            console.log("[DEBUG] Running " + suite);
            try {
                const exit = await child.exited;
                console.log("[DEBUG] " + suite + " exited " + exit);
                if (exit !== 0) process.exitCode = 1;
            } finally {
                process.off("SIGINT", abort);
                process.off("SIGTERM", abort);
            }
        } finally {
            closeSync(logfile);
        }
    } finally {
        unlinkSync(executable);
    }
}
