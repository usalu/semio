/** 🔬️ Probes which Ajv accessor keeps the `data is T` predicate that narrows an `unknown` fixture. */
type Shape = Readonly<{ limits: Readonly<{ files: number }> }>;
export async function probe(): Promise<void> {
  const assert: typeof import("node:assert/strict") = (await import("node:assert/strict")).default;
  const { default: Ajv } = await import("ajv");
  const ajv = new Ajv({ strict: true });
  const byGetSchema = ajv.getSchema<Shape>("#/$defs/Shape")!;
  const a: unknown = JSON.parse("{}");
  assert(byGetSchema(a), "get-schema");
  const viaGetSchema: number = a.limits.files;
  const byCompile = ajv.compile<Shape>({ $ref: "#/$defs/Shape" });
  const b: unknown = JSON.parse("{}");
  assert(byCompile(b), "compile");
  const viaCompile: number = b.limits.files;
  console.log(viaGetSchema, viaCompile);
}
