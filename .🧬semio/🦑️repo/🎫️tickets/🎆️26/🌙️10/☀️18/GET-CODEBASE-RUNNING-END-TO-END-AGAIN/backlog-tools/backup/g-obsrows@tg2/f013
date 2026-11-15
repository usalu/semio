import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { readIssuedScriptProcessEnvelopeV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withBuildTestInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import { readGenuineBirthPolicyV1, selectGenuineBirthV1 } from "../../🟦️.ts";

const live = process.env.SEMIO_GENUINE_PROBE === "1";

/** 🧫️ Observes the genuine birth only when the harness law launched this file; plain runs skip it. */
test.skipIf(!live)("probe birth carries the declared repository request", () => withBuildTestInvocationV1(async original => {
  const birth = selectGenuineBirthV1(process.env.SEMIO_GENUINE_BIRTH), capabilities = readIssuedScriptProcessEnvelopeV1(original).capabilities as { arguments: string[]; tool: string };
  expect(capabilities.arguments).toEqual(["repo", ...birth.arguments]);
  expect(readGenuineBirthPolicyV1().births.map(row => row.id)).toContain(birth.id);
  expect(birth.sdk).toBe(process.env.SEMIO_NX_TOOLING_IDENTITY !== undefined);
}));

/** 🧾️ The prepared execution inventory reaches the file exactly when the birth declares its preparation, and its digest is the independent sha256 of the file. */
test.skipIf(!live)("probe inventory reaches the file exactly when the birth prepares it", () => {
  const birth = selectGenuineBirthV1(process.env.SEMIO_GENUINE_BIRTH), path = process.env.SEMIO_TARGET_EXECUTION_INVENTORY_FILE, digest = process.env.SEMIO_TARGET_EXECUTION_INVENTORY_SHA256;
  if (birth.preparation === undefined) { expect(path).toBeUndefined(); expect(digest).toBeUndefined(); return; }
  if (path === undefined || digest === undefined) throw Error("Prepared execution inventory required");
  const bytes = readFileSync(path);
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(digest);
  expect(JSON.parse(bytes.toString("utf8")).value.producer).toMatch(/^[a-f0-9]{64}$/u);
});

test.skipIf(!live)("probe a.b(c)+[d]", () => { expect(1).toBe(1); });

test.skipIf(!live)("probe a.b(c)+[d]x", () => { expect(1).toBe(1); });
