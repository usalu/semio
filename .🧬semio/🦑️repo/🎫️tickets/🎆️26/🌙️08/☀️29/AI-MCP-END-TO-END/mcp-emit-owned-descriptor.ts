import { mkdir, mkdtemp, readFile, rm, stat, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { stripCoreNamesV1 } from "./mcp-native-wasm-names.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";

const knownOwners = ["dag", "reasoning", "remodel", "trinity"];
const [owner, emitterPath] = process.argv.slice(2);
if (!knownOwners.includes(owner) || !emitterPath) throw Error("Expected one descriptor owner and the owned emitter executable");

const generated = join(import.meta.dirname, "🗑️generated");
const emitter = resolve(emitterPath);
const original = join(generated, "native-guests/wasm32-wasip2/debug", `semio_hub_${owner}_component.wasm`);
const staging = join(generated, "descriptor-refresh");
const maximumBytes = 512 * 1024 ** 2;
const jsonName = "🔣️.json";
const packName = "🛂️.descriptor.semio";

/** 🧾️ Hashes emitted evidence without retaining the entire component in memory. */
async function sha256(path: string): Promise<string> {
  const hash = new Bun.CryptoHasher("sha256");
  for await (const bytes of Bun.file(path).stream()) hash.update(bytes);
  return hash.digest("hex");
}

await mkdir(staging, { recursive: true });
const temporary = await mkdtemp(join(staging, "guest-"));
try {
  let component = original;
  const length = (await stat(original)).size;
  console.log(`[DEBUG] ${owner} exact authored component ${length} bytes SHA-256 ${await sha256(original)}`);
  if (length > maximumBytes) {
    component = join(temporary, "component.wasm");
    const removed = await stripCoreNamesV1(original, component, maximumBytes);
    console.log(`[DEBUG] Verified retained core sections after removing ${removed} symbol-name bytes, SHA-256 ${await sha256(component)}`);
  }
  console.log(`[DEBUG] Owned emitter SHA-256 ${await sha256(emitter)}`);
  const child = Bun.spawn([emitter, "describe", component, "--out", temporary], {
    stdout: "inherit", stderr: "inherit", env: { ...process.env, SEMIO_BUILD_MODE: "prototype" },
  });
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
  const exit = await child.exited;
  if (exit !== 0) throw Error(`Descriptor emitter exited ${exit} for ${owner}`);

  const [json, pack] = await Promise.all([
    readFile(join(temporary, jsonName)), readFile(join(temporary, packName)),
  ]);
  const descriptor = verifySemanticDescriptorPairBytesV1(json, pack);
  if (descriptor.packageId !== `semio:${owner}`) throw Error(`Unexpected component package ${descriptor.packageId}`);
  const selected = join(staging, owner);
  await mkdir(selected, { recursive: true });
  await writeFile(join(selected, jsonName), json);
  await writeFile(join(selected, packName), pack);
  console.log(`[DEBUG] Verified semantic pair ${descriptor.semanticSha256} at ${selected}`);
} finally {
  await rm(temporary, { recursive: true, force: true });
}
