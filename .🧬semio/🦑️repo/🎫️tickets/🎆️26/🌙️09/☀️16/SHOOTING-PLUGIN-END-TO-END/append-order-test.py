"""🧪 Appends one bun test that checks drawing field order and emblem ownership."""
from pathlib import Path

test = next(p for p in Path("/home/deck/semio/✏️s/🔌️plugins/🎥️shooting").rglob("*.ts") if p.parent.name.endswith("tests") and "snapshot" in p.as_posix() and "independent" not in p.as_posix() and p.parent.parent.name.endswith("snapshot"))
schema = test.parents[4] / "\U0001f9ec\ufe0fschema"
print("test", test)
print("schema exists", schema.exists(), schema)
graphql = next(schema.glob("*.graphql")).name
proto = next(schema.glob("*.proto")).name
artifact = next(path.name for path in schema.glob("*.json") if path.parent == schema and "$defs" in path.read_text()[:200] or '"$defs"' in path.read_text())
lines = [
    "",
    'test("Shooting drawing fields share one order and every document facet owns emblem", async () => {',
    f'  const artifact = await Bun.file(new URL("../../../../{schema.name}/{artifact}", import.meta.url)).json();',
    "  const drawing = Object.keys(artifact.$defs.ShootingDrawing.properties);",
    "  const facets = [",
    f'    "../../../../{schema.name}",',
    f'    "../../../../{schema.name}/{(schema / "\U0001f4f8\ufe0fsnapshot").name}",',
    f'    "../../../../{schema.name}/{(schema / "\U0001f9ec\ufe0fmutations").name}",',
    f'    "../../../../{schema.name}/{(schema / "\U0001f53adiff").name}",',
    "  ];",
    "  for (const facet of facets) {",
    f'    const document = parse(await Bun.file(new URL(`${{facet}}/{graphql}`, import.meta.url)).text());',
    '    const type = document.definitions.find(definition => definition.kind === Kind.OBJECT_TYPE_DEFINITION && definition.name.value === "ShootingDrawing");',
    "    expect(type?.kind).toBe(Kind.OBJECT_TYPE_DEFINITION);",
    "    if (type?.kind === Kind.OBJECT_TYPE_DEFINITION) expect(type.fields?.map(field => field.name.value)).toEqual(drawing);",
    "  }",
    "  const roots = [",
    f'    ["../../../../{schema.name}", "ShootingArtifact"],',
    f'    ["../../../../{schema.name}/{(schema / "\U0001f4f8\ufe0fsnapshot").name}", "ShootingSnapshot"],',
    f'    ["../../../../{schema.name}/{(schema / "\U0001f53adiff").name}", "ShootingDiff"],',
    f'    ["../../../../{schema.name}/{(schema / "\U0001f9ec\ufe0fmutations").name}", "ShootingMutation"],',
    "  ] as const;",
    "  for (const [facet, name] of roots) {",
    f'    const document = parse(await Bun.file(new URL(`${{facet}}/{graphql}`, import.meta.url)).text());',
    "    const type = document.definitions.find(definition => definition.kind === Kind.OBJECT_TYPE_DEFINITION && definition.name.value === name);",
    "    expect(type?.kind).toBe(Kind.OBJECT_TYPE_DEFINITION);",
    '    if (type?.kind === Kind.OBJECT_TYPE_DEFINITION) expect(type.fields?.some(field => field.name.value === "emblem")).toBe(true);',
    f'    const proto = await Bun.file(new URL(`${{facet}}/{proto}`, import.meta.url)).text();',
    '    expect(proto).toContain("emblem");',
    "  }",
    "});",
    "",
]
text = test.read_text()
if "every document facet owns emblem" not in text:
    test.write_text(text + "\n".join(lines))
print("appended", test.stat().st_size)
