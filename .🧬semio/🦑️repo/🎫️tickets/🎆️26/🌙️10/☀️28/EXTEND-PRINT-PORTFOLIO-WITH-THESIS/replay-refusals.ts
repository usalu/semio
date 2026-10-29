import { test } from "bun:test";
import assert from "node:assert/strict";
import { readFile, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";
import { withBuildTestInvocationV1 } from "@semio-tech/process/testing/invocation";
import { createIssuedResourceControlV1, scriptInvocationEnvironment } from "@semio-tech/process/routing";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution/capture";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts/native-orchestration/invocation";
import { privateRepositoryWorkspaceV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🟦️.ts";
import { declareThesisTestBirth } from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🎟️birth/🟦️.ts";
import { withPrintExecutionV1, withPrintScratchV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/🎛️operation/🟦️.ts";
import { readPrintMaterialFileV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/📤️producer/🟦️.ts";
import { printCompilationIntentV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📋️intent/🟦️.ts";
import { readPrintCompilerRefusalCaptureV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📥️refusal/🟦️.ts";
import { thesisPreamble } from "../../../../../../../🧰️framework/🛍️products/📓️print/🧾️template/🎓️thesis/🟦️.ts";
import fixture from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🔣️.json";
import refusalSchema from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📥️refusal/🧬️schema/🔣️.json";
const checkout = process.cwd(), product = join(checkout, "🧰️framework/🛍️products/📓️print"), artifacts = resolve(checkout, process.env.SEMIO_TEST_ARTIFACT_DIR!);
const repository = await privateRepositoryWorkspaceV1();
assert.equal(repository.owned, false);
assert.equal(repository.root, resolve(checkout, ".cache/test-sandbox/rsLAcRv"));
await repository.within(async () => declareThesisTestBirth(fixture.runners.rendering));
const digest = (bytes: Uint8Array | string): string => createHash("sha256").update(bytes).digest("hex");
const snapshot = JSON.parse(await readFile(join(dirname(import.meta.path), "🗑️generated/rendering/source-snapshot.json"), "utf8"));
const recheck = async () => {
  for (const [path, expected] of Object.entries(snapshot.sourceSha256)) assert.equal(digest(await readFile(join(checkout, path))), expected);
  for (const [path, expected] of Object.entries(snapshot.consumedSha256)) assert.equal(digest(await readFile(join(repository.root, path))), expected);
};
const renderingPath = join(product, "🧪️tests/🎓️thesis/🖨️rendering/🟦️.ts"), renderingText = await readFile(renderingPath, "utf8"), parsed = ts.createSourceFile(renderingPath, renderingText, ts.ScriptTarget.Latest, true);
const bodies: ts.NoSubstitutionTemplateLiteral[] = [];
const visit = (node: ts.Node): void => {
  if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.name.text === "body" && node.initializer && ts.isTaggedTemplateExpression(node.initializer)) {
    assert.equal(node.initializer.tag.getText(parsed), "String.raw");
    assert.ok(ts.isNoSubstitutionTemplateLiteral(node.initializer.template));
    bodies.push(node.initializer.template);
  }
  ts.forEachChild(node, visit);
};
visit(parsed); assert.equal(bodies.length, 1);
const body = bodies[0]!.getText(parsed).slice(1, -1).replace(/\r\n?/g, "\n");
const source = thesisPreamble({ language: "en", theme: "light", degree: "master", title: "Thesis Fixture", author: "Ada Researcher", university: "Example University", awardedDegree: "Master of Science", programme: "Research Programme", studentId: "12345", supervisor: "Supervisor Example", reviewers: "Reviewer Example", submission: "2026-10-28", place: "Berlin", keywords: "Research Keywords", declarationText: "Authored declaration evidence." }) + body;
const count = process.env.SEMIO_THESIS_NEGATIVE_COUNT ?? "1";
assert.ok(count === "1" || count === "6");
test("owned exact negative compiler inspector capture", () => repository.within(() => withBuildTestInvocationV1(async invocation => {
  assert.equal(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot, repository.root);
  await recheck();
  const phaseControl = createIssuedResourceControlV1(invocation, "print:thesis:negative-replay", 0);
  await withPrintExecutionV1({ workspace: repository.root, invocation, phaseControl, kind: "probe" }, async execution => {
    for (const row of fixture.rendering.invalid.slice(0, Number(count))) await withPrintScratchV1(execution, "compiler-refusal", async scratch => {
      const mutated = "remove" in row ? source.replace(row.remove!, "") : source.replace(row.replace![0]!, row.replace![1]!);
      const invalid = "documentType" in row ? mutated.replace("type=thesis", "type=" + row.documentType) : mutated;
      assert.notEqual(invalid, source);
      const intent = printCompilationIntentV1({ version: 1, kind: "probe", name: "thesis.tex", source: invalid });
      const bytes = Buffer.from(JSON.stringify({ version: 1, intent })), policy = refusalSchema.definitions.Policy.const;
      assert.ok(bytes.length <= policy.maximumRequestBytes);
      const path = join(scratch.directory, "request.json"), requestSha256 = digest(bytes);
      checkResourceLeaseControl(scratch.control);
      await writeFile(path, bytes, { flag: "wx", signal: scratch.control.signal });
      const transport = readNativeCallerAuthorityV1(scratch.invocation.capabilities).transport;
      const observation = () => ({ parentOriginalCancelled: invocation.control.signal.aborted, parentScratchCancelled: scratch.control.signal.aborted, parentRemainingMilliseconds: invocation.control.remainingMilliseconds() });
      const before = observation();
      const captured = await captureOwnedCommandResultV1(process.execPath, [join(product, "🔨️modules/🖨️tectonic-template-compilation/📥️refusal/📜️script.ts"), path, requestSha256], scratch.workspace, { control: scratch.control, env: scriptInvocationEnvironment(scratch.invocation, scratch.environment), output: { maximumBytes: Math.min(policy.maximumResultBytes, transport.maximumBytes), maximumLines: Math.min(policy.maximumResultLines, transport.maximumLines) } });
      const after = observation(), target = join(artifacts, row.id + ".json");
      assert.ok(target.length <= 256);
      checkResourceLeaseControl(scratch.control);
      await writeFile(target, JSON.stringify({ kind: "owned-print-refusal-capture", canonicalWorkspaceProduct: false, case: row.id, requestSha256, sourceSha256: digest(invalid), renderingSourceSha256: digest(renderingText), before, after, capture: captured }, null, 2) + "\n", { signal: scratch.control.signal });
      checkResourceLeaseControl(scratch.control);
      assert.equal(digest(await readPrintMaterialFileV1(path, policy.maximumRequestBytes, scratch.control)), requestSha256);
      const report = readPrintCompilerRefusalCaptureV1(captured, requestSha256);
      assert.ok(report.log.replace(/\s+/g, "").toLowerCase().includes(row.message.replace(/\s+/g, "").toLowerCase()), row.id + " exact expected compiler refusal");
      console.log("[print] Owned negative replay " + row.id + " PASS");
    });
  });
  await recheck();
})), fixture.runners.rendering.maximumElapsedMilliseconds);
