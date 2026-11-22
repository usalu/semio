import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, normalize, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { blake3 } from "@noble/hashes/blake3.js";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const build = resolve(ticket, "🗑️generated/w/b/wasm32-unknown-unknown/prototype/build");
const store = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/";
const worker = "🧰️framework/🔨️modules/🧵️job/👷️worker/";
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
const puzzle2d = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/";
const puzzle = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/";
const mutationRoot = resolve(root, puzzle, "🧬️schema/🧬️mutations");
const mutationLeaves = readdirSync(mutationRoot, { recursive: true, withFileTypes: true })
  .filter(entry => entry.isFile() && entry.name === "🦀️.rs")
  .map(entry => resolve(entry.parentPath, entry.name))
  .filter(path => readFileSync(path, "utf8").includes("dsl::MutationLeaf"))
  .map(path => relative(root, path))
  .sort();
const mutation2dRoot = resolve(root, puzzle2d, "🧬️schema/🧬️mutations");
const mutation2dLeaves = readdirSync(mutation2dRoot, { recursive: true, withFileTypes: true })
  .filter(entry => entry.isFile() && entry.name === "🦀️.rs" && entry.parentPath !== mutation2dRoot && !entry.parentPath.includes("🧪️tests"))
  .map(entry => resolve(entry.parentPath, entry.name))
  .filter(path => readFileSync(path, "utf8").includes("DslRecord"))
  .map(path => relative(root, path))
  .sort();
const sources = new Set([
  "🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs",
  "🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs",
  "🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs",
  "🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs",
  "🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs",
  puzzle2d + "🧬️schema/🧬️mutations/🦀️.rs",
  ...mutation2dLeaves,
  puzzle2d + "🧬️schema/🧱️records/🦀️.rs",
  puzzle2d + "🚪️io/💾️binary/🧬️mutations/🦀️.rs",
  puzzle2d + "✏️editor/🧠️artifact-editor/🦀️.rs",
  puzzle2d + "✏️editor/🪟️window/🦀️.rs",
  "🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🦀️.rs",
  store + "🧵️canonical-edit/🧳️source/🦀️.rs",
  store + "🎮️one-item/📬️publication/📥️lifted/🧵️operation-wire/🧳️receiving/🦀️.rs",
  store + "🎮️one-item/📬️publication/📥️lifted/🧵️operation-wire/🧳️receiving/🔢️binary/🦀️.rs",
  store + "🧵️operation-wire/🧳️replacement/🦀️.rs",
  store + "🧵️operation-wire/📦️output/🦀️.rs",
  store + "🧵️operation-wire/📤️edit/🦀️.rs",
  store + "🧵️operation-wire/🧳️original/🦀️.rs",
  store + "🧵️operation-wire/📜️history/🦀️.rs",
  store + "🧵️operation-wire/📜️history/📤️batch/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️play-app/🦀️.rs",
  store + "🏗️initialization/🔁️replay/🦀️.rs",
  store + "🏗️initialization/🔁️replay/🧬️native/🦀️.rs",
  store + "🔁️replay/🎮️operation/🌱️derived/🦀️.rs",
  store + "🔁️replay/🎮️operation/🧬️native/🔌️drivers/🦀️.rs",
  store + "🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs",
  store + "🧬️snapshot-clone/🦀️.rs",
  store + "🧬️snapshot-clone/♻️custody/🦀️.rs",
  store + "🧬️snapshot-clone/✏️config-apply/🦀️.rs",
  worker + "🎟️authority/🦀️.rs",
  worker + "🧵️worker-job-session/🦀️.rs",
  worker + "🕳️retirement-slots-and-nodes/🦀️.rs",
  plugin + "👷️job/🦀️.rs",
  plugin + "🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs",
  plugin + "🪟️window-kits/🌍️world3d/🦀️.rs",
  plugin + "🪟️window-kits/🌍️world3d/☀️sun/🦀️.rs",
  plugin + "🪟️window/🎚️config/📥️retained/📦️typed-pack-load/🛬️native/🦀️.rs",
  plugin + "🪟️window/🎚️config/🧬️preparation/🔁️replace/🦀️.rs",
  plugin + "🧵️retained-command/🔄️full-operation/🦀️.rs",
  plugin + "⏪️time-travel/🧵️instance/🧹️owners-and-retirement/🦀️.rs",
  plugin + "⏪️time-travel/🗄️store-state/🦀️.rs",
  plugin + "🧵️instance/📄️docs/📥️staging/🚪️ingress/🦀️.rs",
  plugin + "🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs",
  plugin + "🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs",
  plugin + "🧵️instance/🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs",
  puzzle + "✏️editor/👆️gestures/🩹️patches/🦀️.rs",
  puzzle + "✏️editor/👆️gestures/🎛️selection-and-weights/🦀️.rs",
  puzzle + "✏️editor/👆️gestures/🧲️proximity-connect/🦀️.rs",
  puzzle + "✏️editor/🎚️config/🦀️.rs",
  puzzle + "✏️editor/🪟️window/🦀️.rs",
  puzzle + "✏️editor/✍️artifact-editor/🦀️.rs",
  puzzle + "✏️editor/🔧️play-app-retained-commands-and-tool-proofs/🦀️.rs",
  puzzle + "✏️editor/🧵️retained/🦀️.rs",
  puzzle + "✏️editor/🧰️kit/📚️example-and-brush-mesh/🦀️.rs",
  puzzle + "🚪️io/💾️binary/🧬️mutations/🦀️.rs",
  puzzle + "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio",
  puzzle + "🧬️schema/🎞️play-snapshot/🦀️.rs",
  puzzle + "🧬️schema/🧱️document-records/🦀️.rs",
  puzzle + "🧬️schema/🧬️mutations/🦀️.rs",
  puzzle + "🧬️schema/🧬️mutations/🔣️.json",
  puzzle + "🧬️schema/🔣️.json",
  puzzle + "🧬️schema/🧬️mutations/🔀️splice-scene-rows/🦀️.rs",
  puzzle + "🧬️schema/🧬️mutations/🔀️splice-scene-rows/🧬️schema/🔣️.json",
  puzzle + "✏️editor/🖌️session/📌️paste/🦀️.rs",
  puzzle + "✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs",
  puzzle + "🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs",
  puzzle + "🧬️schema/🦀️.rs",
  puzzle + "🧬️schema/📸️snapshot/🦀️.rs",
  puzzle + "🧬️schema/🔺️diff/🦀️.rs",
  ...mutationLeaves,
]);

function linkedWasmUnits() {
  const nodes = [];
  const byReceipt = new Map();
  const host = resolve(build, "../../../prototype/build");
  for (const [target, directory] of [["host", host], ["wasm", build]]) {
    for (const owner of readdirSync(directory)) {
      for (const key of readdirSync(resolve(directory, owner))) {
        const fingerprints = resolve(directory, owner, key, "fingerprint");
        if (!existsSync(fingerprints)) continue;
        for (const name of readdirSync(fingerprints).filter(name => name.endsWith(".json"))) {
          const kind = name.slice(0, -5);
          const marker = resolve(fingerprints, kind);
          if (!existsSync(marker)) continue;
          const signature = readFileSync(marker, "utf8").trim();
          if (!/^[0-9a-f]{16}$/.test(signature)) continue;
          const metadata = JSON.parse(readFileSync(resolve(fingerprints, name), "utf8"), (_, value, context) =>
            typeof value === "number" && !Number.isSafeInteger(value) ? context.source : value
          );
          if (!Array.isArray(metadata.deps)) continue;
          const node = { target, owner, key, kind, timestamp: statSync(marker).mtimeMs, dependencies: metadata.deps };
          nodes.push(node);
          const receipt = Buffer.from(signature, "hex").readBigUInt64LE().toString();
          const candidates = byReceipt.get(receipt) ?? [];
          candidates.push(node);
          byReceipt.set(receipt, candidates);
        }
      }
    }
  }

  const component = nodes
    .filter(node => node.target === "wasm" && node.owner === "semio-hub-puzzle-component")
    .sort((a, b) => b.timestamp - a.timestamp)[0];
  if (!component) throw new Error("The linked Hub component has no completed Cargo fingerprint");

  const reached = new Set([component]);
  const pending = [component];
  for (const node of pending) {
    for (const [, name, , receipt] of node.dependencies) {
      const candidates = byReceipt.get(String(receipt));
      const dependency = candidates?.find(candidate => candidate.kind === "lib-" + name) ?? candidates?.[0];
      if (!dependency || reached.has(dependency)) continue;
      reached.add(dependency);
      pending.push(dependency);
    }
  }
  return new Set(pending.filter(node => node.target === "wasm").map(node => node.owner + "/" + node.key));
}

const checks = new Map([...sources].map(source => {
  const absolute = resolve(root, source);
  const bytes = readFileSync(absolute);
  const checksum = Buffer.from(blake3(bytes)).toString("hex");
  return [absolute, { source, checksum, length: bytes.length, owners: [] }];
}));
const ownerNames = /framework-value-(snapshot-read|retirement|persistent)|framework-pack-(json|record-retained)|framework-job-worker|framework-replication-edit|store-(one-item-publication|snapshot-clone|initialization-replay|replay-operation|operation-wire|engine)|plugin-(document-media|window-kits|window-config|job|command|retained-command|time-travel|instance-(commands|docs|stores|typed-ops))|puzzle-[235]d/;
const checksumLine = /^# checksum:blake3=([0-9a-f]+) file_len:(\d+) (.*)$/;
for (const owner of readdirSync(build).filter(name => ownerNames.test(name))) {
  for (const key of readdirSync(resolve(build, owner))) {
    const unit = resolve(build, owner, key);
    const out = resolve(unit, "out");
    if (!existsSync(out)) continue;
    for (const name of readdirSync(out).filter(name => name.endsWith(".d"))) {
      const artifact = statSync(resolve(out, "lib" + name.slice(0, -2) + ".rlib"), { throwIfNoEntry: false });
      if (!artifact) continue;
      const lines = readFileSync(resolve(out, name), "utf8").split(/\r?\n/);
      const invoked = statSync(resolve(unit, "fingerprint/invoked.timestamp"), { throwIfNoEntry: false });
      for (const line of lines) {
        const record = checksumLine.exec(line);
        if (!record) continue;
        const [, checksum, length, path] = record;
        const check = checks.get(normalize(path.replaceAll("\\ ", " ")));
        if (!check) continue;
        check.owners.push({
          owner,
          key,
          built: artifact.mtime.toISOString(),
          invoked: invoked?.mtime.toISOString(),
          exact: checksum === check.checksum && Number(length) === check.length,
        });
      }
    }
  }
}

const records = [...checks.values()];
const requireComplete = process.argv.includes("--require-complete");
if (requireComplete) {
  const linked = linkedWasmUnits();
  for (const record of records) {
    const owners = record.owners.filter(owner => linked.has(owner.owner + "/" + owner.key));
    record.linkedOwners = owners.map(owner => owner.owner + "/" + owner.key);
    record.linkedExact = owners.length > 0 && owners.every(owner => owner.exact);
  }
  if (records.some(record => !record.linkedExact)) process.exitCode = 1;
}
console.log(JSON.stringify(records, null, 2));
