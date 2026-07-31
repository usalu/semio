import assert from "assert";

function parseRepoEvents(output) {
  const lines = output
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  return lines.map((line) => JSON.parse(line));
}

function extractRepoResult(events) {
  let lastResult = null;
  for (const event of events) {
    if (event.kind === "error" && event.error?.fatal) {
      throw new Error(event.error.message ?? "Repo command failed");
    }
    if (event.kind === "result") {
      lastResult = event.result ?? event.data ?? null;
    }
  }
  if (lastResult && typeof lastResult === "object" && !Array.isArray(lastResult)) {
    if ("data" in lastResult) {
      return lastResult;
    }
  }
  return { data: lastResult };
}

console.log("Testing extractRepoResult fix...");

const testOutput = '{"kind":"result","result":{"data":{"breachs":[{"id":"v1","summary":"Test breach"}]}}}';
const events = parseRepoEvents(testOutput);
const result = extractRepoResult(events);

assert.ok(result.data, "Should have data field");
assert.ok(result.data.breachs, "Should have breachs");
assert.strictEqual(result.data.breachs.length, 1, "Should have 1 breach");
assert.strictEqual(result.data.breachs[0].id, "v1", "Should have correct breach id");

console.log("✓️ All tests passed!");
console.log("Extracted result:", JSON.stringify(result, null, 2));
