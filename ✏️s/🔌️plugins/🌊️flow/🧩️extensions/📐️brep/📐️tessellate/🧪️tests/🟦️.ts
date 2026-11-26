import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv";

test("original tessellation keeps strict business source until complete canonical handoff", () => {
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
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
  expect(implementation).toContain("flow_extension_sdk::admit_evaluation_failure");
  const guest = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  const normal = guest.slice(guest.indexOf("fn invoke(&mut self, capability:"), guest.indexOf("fn inference_context", guest.indexOf("fn invoke(&mut self, capability:")));
  expect(normal).toContain('"tessellate" => self.context.tessellate_raw(request, cx)');
  expect(normal).not.toContain("tessellate_step_envelope_json");
});

test("the tessellation invocation is compiled by its own library", () => {
  const source = new URL("../🦀️.rs", import.meta.url);
  const manifest = new URL("../📦️packages/🦀️rust/Cargo.toml", import.meta.url);
  const extensionManifest = new URL("../../📦️packages/🦀️rust/Cargo.toml", import.meta.url);
  const library = Bun.TOML.parse(readFileSync(manifest, "utf8"));
  const extension = Bun.TOML.parse(readFileSync(extensionManifest, "utf8"));
  const workspace = Bun.TOML.parse(readFileSync(new URL("../../Cargo.toml", import.meta.url), "utf8"));
  expect(library.package.metadata.semio.role).toBe("library");
  expect(workspace.workspace.members).toContain("📐️tessellate/📦️packages/🦀️rust");
  expect(new URL(library.lib.path, manifest).href).toBe(source.href);
  expect(new URL(extension.dependencies[library.package.name].path + "/Cargo.toml", extensionManifest).href).toBe(manifest.href);
  expect(library.dependencies[extension.package.name]).toBeUndefined();
  const guest = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  expect(guest).toContain("use semio_s_plugin_flow_brep_tessellation as tessellation;");
  expect(guest).not.toContain('📐️tessellate/🦀️.rs');
});


test("Three.js reproduces the authored fillet bounds and minimum surface detail", async () => {
  const law = JSON.parse(readFileSync(new URL("./🔣️curved-mesh.json", import.meta.url), "utf8"));
  const { RoundedBoxGeometry } = await import("three/addons/geometries/RoundedBoxGeometry.js");
  const [width, depth, height] = law.box;
  const geometry = new RoundedBoxGeometry(width, depth, height, 4, law.radius);
  geometry.translate(width / 2, depth / 2, height / 2);
  geometry.computeBoundingBox();
  const bounds = geometry.boundingBox!;
  expect(bounds.min.toArray().every(x => Math.abs(x) <= law.tolerance)).toBe(true);
  expect(bounds.max.toArray().every((x, i) => Math.abs(x - law.box[i]) <= law.tolerance)).toBe(true);
  expect(geometry.attributes.position!.count / 3).toBeGreaterThanOrEqual(law.minimumTriangles);
  geometry.dispose();
});
