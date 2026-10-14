import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

test("original tessellation keeps strict business source until complete canonical handoff", () => {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, earlyTerminalPayload: true })).toBe(false);
  const exact = fixture.source.match(/"nodeHash":(\d+)/)[1];
  expect(BigInt(exact)).toBe((1n << 64n) - 3n);
  const source = JSON.parse(fixture.source);
  expect(Object.keys(source).sort()).toEqual(["handle", "nodeHash", "tolerance", "windowId", "windowKindId"]);
  for (const copy of fixture.copyGrants) {
    const bytes = new TextEncoder().encode(fixture.source);
    const destination: number[] = [];
    for (let at = 0; at < bytes.length; at += copy) destination.push(...bytes.subarray(at, at + copy));
    expect(new TextDecoder().decode(new Uint8Array(destination))).toBe(fixture.source);
  }
  const implementation = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(implementation).toContain("OwnedProjectedPackWireCursor");
  expect(implementation).not.toContain("ArtifactCanonicalJsonTreeWireCursor");
  expect(implementation).toContain("self.completed=Some(ready.source)");
  expect(implementation).toContain("closing-tessellation-output-source");
  expect(implementation).toContain("tessellate_step");
  expect(implementation).toContain("admit_original_domain_failure");
  const guest = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  const normal = guest.slice(guest.indexOf("fn invoke(&self, capability:"), guest.indexOf("fn inference_context", guest.indexOf("fn invoke(&self, capability:")));
  expect(normal).toContain('"tessellate" => self.context.tessellate_raw(request, cx)');
  expect(normal).not.toContain("tessellate_step_envelope_json");
  console.log("[DEBUG] Ajv, independent JSON/BigInt and UTF8 chunks preserve original request5 and reject early terminal publication; native ownership qualification is separate");
});
