import assert from "node:assert/strict";
import { createServer } from "node:http";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { readFile } from "node:fs/promises";

const load = (path: string) => import(pathToFileURL(resolve(path)).href);
const { courseAssignmentPdfDocumentsV1 } = await load("🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📃️documents/🟦️.ts");
const { readCourseAssignmentPdfsV1, assignmentPdfBytes } = await load("🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📖️assignments/🟦️.ts");
const { verifyParametrizationAssignmentContentV1 } = await load("🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/🧪️tests/🟦️.ts");
const fixture = JSON.parse(await readFile("🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🧫️cases.json", "utf8"));
for (const row of fixture.publications) {
  if (row.expected) assert.deepEqual(courseAssignmentPdfDocumentsV1(row.value), row.expected);
  else assert.throws(() => courseAssignmentPdfDocumentsV1(row.value), new RegExp(row.error ?? "Conflicting"));
}
verifyParametrizationAssignmentContentV1();
const { printArtifactNxOutputV1 } = await load("🧰️framework/🛍️products/📓️print/📦️artifacts/🟦️.ts");
const project = JSON.parse(await readFile("🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json", "utf8"));
const catalog = JSON.parse(await readFile("🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json", "utf8"));
assert.ok(catalog.documents.every((row: { id: string }) => !row.id.startsWith("parametrization-")));
for (const language of ["en", "de"]) {
  const target = project.targets["build-parametrization-" + language];
  assert.deepEqual(target.outputs, [printArtifactNxOutputV1(process.cwd(), { version: 1, kind: "document", mode: "prototype", namespace: "course", id: "architecture-parametric-parametrization-" + language })]);
  for (const prerequisite of ["fonts", "deps-tex", "deps-tectonic", "generate"]) assert.ok(target.dependsOn.includes("@semio-tech/print:" + prerequisite));
  for (const consumer of ["build", "dev-site"]) assert.ok(project.targets[consumer].dependsOn.includes("@teaching/architecture-course:build-parametrization-" + language));
}
await assert.rejects(() => readCourseAssignmentPdfsV1({}), /transport required/);
assert.throws(() => assignmentPdfBytes(new TextEncoder().encode("<html>")));
assert.throws(() => assignmentPdfBytes(new Uint8Array(16 * 1024 * 1024 + 1)));
const requests: string[] = [], bytes = new TextEncoder().encode("%PDF-1.7\nassignment\n%%EOF"), secret = "a".repeat(64);
const server = createServer((request, response) => {
  assert.equal(request.headers.authorization, "Bearer " + secret);
  requests.push(request.url!);
  response.setHeader("content-length", bytes.length);
  response.end(bytes);
});
await new Promise<void>(accept => server.listen(0, "127.0.0.1", accept));
try {
  const address = server.address();
  assert.ok(address && typeof address !== "string");
  const documents = await readCourseAssignmentPdfsV1({ SEMIO_VITE_RUNTIME_CHANNELS_V1: JSON.stringify([{ name: "course-assignment-pdfs", endpoint: "http://127.0.0.1:" + address.port + "/pdfs", secret }]) });
  const names = ["architecture-parametric-parametrization-en.pdf", "architecture-parametric-parametrization-de.pdf"];
  assert.deepEqual(requests, names.map(name => "/pdfs/" + name));
  assert.deepEqual([...documents.keys()], names);
  for (const actual of documents.values()) assert.deepEqual(actual, bytes);
} finally { await new Promise<void>(accept => { server.close(() => accept()); server.closeAllConnections(); }); }
console.log("Course semantic, producer and authenticated localized PDF transport contracts passed");
