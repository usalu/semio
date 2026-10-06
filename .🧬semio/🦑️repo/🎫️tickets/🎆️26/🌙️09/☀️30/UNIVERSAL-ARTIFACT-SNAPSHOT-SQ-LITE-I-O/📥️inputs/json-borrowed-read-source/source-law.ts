
test("neutral borrowed-read fixture retains original Source syntax and independent jsonc-parser lexemes", async () => {
  const fixture = JSON.parse(read("🧫️fixtures/🫳️read-source.json")) as {source: string; duplicate: string; invalidUtf8: number[]; largeStringBytes: number; decodeAllocationBytes: number; stepUnits: number};
  const validate = new Ajv({strict: true}).compile(JSON.parse(read("🧬️schema/🫳️read-source/🔣️.json")));
  expect(validate(fixture)).toBe(true);
  expect(validate({...fixture, stepUnits: 0})).toBe(false);
  const {decodeJsonSyntax} = await import("../../📥️decode/🟦️.ts");
  const operation = () => ({maximumBytes: fixture.decodeAllocationBytes, maximumNodes: 4096, maximumDepth: 128, chunk: 256, cancelled: () => false, progress: (_: unknown) => {}, yield: async () => {}});
  const syntax = (node: Node, text: string): unknown => {
    if (node.type === "number") return {kind: "number", text: text.slice(node.offset, node.offset + node.length)};
    if (node.type === "null") return {kind: "null"};
    if (node.type === "string" || node.type === "boolean") return {kind: node.type, value: node.value};
    if (node.type === "array") return {kind: "array", items: (node.children ?? []).map(child => syntax(child, text))};
    return {kind: "object", members: (node.children ?? []).map(child => ({name: child.children![0]!.value, value: syntax(child.children![1]!, text)}))};
  };
  const errors: ParseError[] = [], tree = parseTree(fixture.source, errors, {disallowComments: true, allowTrailingComma: false});
  expect(errors).toEqual([]);
  expect(await decodeJsonSyntax(fixture.source, JsonMemberPolicy.Reject, operation())).toEqual(syntax(tree!, fixture.source));
  await expect(decodeJsonSyntax(fixture.duplicate, JsonMemberPolicy.Reject, operation())).rejects.toMatchObject({code: "duplicate-member"});
  expect(() => new TextDecoder("utf-8", {fatal: true}).decode(Uint8Array.from(fixture.invalidUtf8))).toThrow();
  const large = JSON.stringify("x".repeat(fixture.largeStringBytes));
  expect(await decodeJsonSyntax(large, JsonMemberPolicy.Reject, operation())).toEqual({kind: "string", value: JSON.parse(large)});
  console.log("[DEBUG] neutral original8194 source and exact numeric lexemes match first-party Source syntax, jsonc-parser and host JSON; Native paged retention remains a separate owning receipt");
});
