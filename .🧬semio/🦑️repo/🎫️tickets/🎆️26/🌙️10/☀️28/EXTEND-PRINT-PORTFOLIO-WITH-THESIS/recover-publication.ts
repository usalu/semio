import { test } from "bun:test";
import assert from "node:assert/strict";
import { readFile, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
const checkout = process.cwd(), product = join(checkout, "🧰️framework/🛍️products/📓️print"), artifacts = resolve(checkout, process.env.SEMIO_TEST_ARTIFACT_DIR!);
const { privateRepositoryWorkspaceV1 } = await import(join(checkout, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🟦️.ts"));
const { declareThesisTestBirth } = await import(join(product, "🧪️tests/🎓️thesis/🎟️birth/🟦️.ts"));
const { withBuildTestInvocationV1 } = await import("@semio-tech/process/testing/invocation");
const { createIssuedResourceControlV1 } = await import("@semio-tech/process/routing");
const { readNativeCallerAuthorityV1 } = await import("@semio-tech/repo-lib/artifacts/native-orchestration/invocation");
const { withPrintExecutionV1 } = await import(join(product, "📦️artifacts/🎛️operation/🟦️.ts"));
const { printTemplatePdfNames, withPublishedPrintDocumentV1 } = await import(join(product, "🔨️modules/🖨️tectonic-template-compilation/🟦️.ts"));
const { readPrintMaterialFileV1 } = await import(join(product, "📦️artifacts/📥️material/📤️producer/🟦️.ts"));
const fixture = (await import(join(product, "🧪️tests/🎓️thesis/🔣️.json"))).default;
const repository = await privateRepositoryWorkspaceV1();
assert.equal(repository.owned, false);
assert.equal(repository.root, resolve(checkout, ".cache/test-sandbox/rIlIoQ1"));
await repository.within(async () => declareThesisTestBirth(fixture.runners.publication));
const hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const snapshot = JSON.parse(await readFile(join(dirname(import.meta.path), "🗑️generated/publication/source-snapshot.json"), "utf8"));
const recheck = async () => {
  for (const [path, expected] of Object.entries(snapshot.sourceSha256)) assert.equal(hash(await readFile(join(checkout, path))), expected);
  for (const [path, expected] of Object.entries(snapshot.consumedSha256)) assert.equal(hash(await readFile(join(repository.root, path))), expected);
};
test("recover already compiled private thesis PDFs through owned readers", () => repository.within(() => withBuildTestInvocationV1(async invocation => {
  assert.equal(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot, repository.root);
  await recheck();
  const phaseControl = createIssuedResourceControlV1(invocation, "print:thesis:publication:recovery", 0), files: unknown[] = [];
  await withPrintExecutionV1({ workspace: repository.root, invocation, phaseControl, kind: "document" }, async execution => {
    for (const row of fixture.publications) await withPublishedPrintDocumentV1(execution, "print", row.id, "@semio-tech/print:build-" + row.id, async artifact => {
      const names = Object.values(printTemplatePdfNames(row.id + ".tex"));
      assert.deepEqual([...artifact.files].sort(), [...names].sort());
      for (const name of names) {
        const bytes = await readPrintMaterialFileV1(join(artifact.path, name), 67108864, artifact.control);
        assert.equal(new TextDecoder().decode(bytes.subarray(0, 5)), "%PDF-");
        const file = "private-" + name, target = join(artifacts, file);
        assert.ok(target.length <= 256);
        await writeFile(target, bytes);
        files.push({ file, bytes: bytes.byteLength, sha256: hash(bytes) });
        console.log("[print] Recovered " + name);
      }
      await artifact.recheck();
    });
  });
  await recheck();
  await writeFile(join(artifacts, "recovery-proof.json"), JSON.stringify({ kind: "owned-private-publication-recovery", canonicalWorkspaceProduct: false, privateRoot: repository.root, files }, null, 2) + "\n");
})), fixture.runners.publication.maximumElapsedMilliseconds);
