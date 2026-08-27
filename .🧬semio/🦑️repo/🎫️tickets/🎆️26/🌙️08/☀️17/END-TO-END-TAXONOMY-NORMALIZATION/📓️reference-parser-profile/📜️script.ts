import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = process.cwd();
const reportRoot = import.meta.dir;
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const sourcePath = join(repoRoot, library, "🧹️normalization/🟦️.ts");
const bundlePath = join(reportRoot, "🟨️profile.generated.js");
const hash = (value: string | Uint8Array): string => createHash("sha256").update(value).digest("hex");

if (process.argv[2] === "bundle") {
  const result = await Bun.build({
    entrypoints: [sourcePath],
    target: "bun",
    format: "esm",
    sourcemap: "inline",
    plugins: [{ name: "private-reference-profile", setup(build) {
      build.onLoad({ filter: /\.ts$/u }, (args) => resolve(args.path) === sourcePath ? { contents: `${readFileSync(sourcePath, "utf8")}\nexport { referencePathIndex, referenceTokens, resolveReferenceTokenPath };\n`, loader: "ts" } : undefined);
    } }],
  });
  if (!result.success || result.outputs.length !== 1) throw new Error(JSON.stringify(result.logs));
  await Bun.write(bundlePath, result.outputs[0]!);
  console.log("[DEBUG] profile bundle", JSON.stringify({ sourcePath, sourceSha256: hash(readFileSync(sourcePath)), bundlePath, bundleSha256: hash(readFileSync(bundlePath)) }));
} else if (process.argv[2] === "profile") {
  const { referencePathIndex, referenceTokens, resolveReferenceTokenPath } = await import(bundlePath);
  const sourcePaths = [`${library}/🔣️taxonomy.json`, "📜️script.ts", "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"];
  const candidates = Array.from({ length: 130000 }, (_, index) => `🧰️framework/🧪️tests/🧪️diagnostic-${index}/🟦️.ts`);
  const known = referencePathIndex([...candidates, ...sourcePaths]);
  const started = performance.now();
  for (const relativePath of sourcePaths) {
    const absolute = join(repoRoot, relativePath), stat = lstatSync(absolute);
    if (!stat.isFile() || stat.isSymbolicLink()) throw new Error(`Unsafe profile input: ${relativePath}`);
    const content = readFileSync(absolute, "utf8"), before = hash(content), rounds = [];
    for (let iteration = 0; iteration < 5; iteration++) {
      const start = performance.now(), tokens = referenceTokens(relativePath, content, known);
      let resolved = 0;
      for (const token of tokens) if (!token.unsupportedReason && resolveReferenceTokenPath(relativePath, token, known)) resolved++;
      rounds.push({ milliseconds: performance.now() - start, tokens: tokens.length, resolved });
      if (performance.now() - started > 45000) break;
    }
    if (hash(readFileSync(absolute)) !== before) throw new Error(`Profile input changed: ${relativePath}`);
    console.log("[DEBUG] reference profile input", JSON.stringify({ path: relativePath, bytes: stat.size, sha256: before, rounds }));
    if (performance.now() - started > 45000) break;
  }
  console.log("[DEBUG] reference profile complete", JSON.stringify({ milliseconds: performance.now() - started, candidateCount: known.exact.size }));
} else if (process.argv[2] === "analyze") {
  const path = join(reportRoot, "🔬️.cpuprofile"), bytes = readFileSync(path), data = JSON.parse(bytes.toString());
  const rows = data.nodes.map((node: { id: number; hitCount?: number; callFrame: { functionName: string; url: string; lineNumber: number } }) => ({ id: node.id, hits: node.hitCount ?? 0, ...node.callFrame }));
  const samples = new Map<number, number>();
  for (const id of data.samples ?? []) samples.set(id, (samples.get(id) ?? 0) + 1);
  console.log("[DEBUG] CPU profile", JSON.stringify({ bytes: bytes.byteLength, sha256: hash(bytes), durationMicroseconds: data.endTime - data.startTime, sampleCount: data.samples?.length, top: rows.map((row: { id: number }) => ({ ...row, samples: samples.get(row.id) ?? 0 })).sort((a: { samples: number }, b: { samples: number }) => b.samples - a.samples).slice(0, 25) }));
} else throw new Error("Expected bundle, profile or analyze");
