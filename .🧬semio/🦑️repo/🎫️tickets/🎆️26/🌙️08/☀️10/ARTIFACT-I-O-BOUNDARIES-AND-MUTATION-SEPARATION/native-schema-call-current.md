# Current Native Calls in Canonical TypeScript Schema

Independent TypeScript AST census of 3016 schema sources finds 107 JSON parse/stringify calls. These are review candidates, not automatic boundary conclusions. Tests, fixtures, probes and examples are excluded.

- 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts:77

```typescript
JSON.stringify(failure.ours)
```

- 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts:77

```typescript
JSON.stringify(failure.oracle)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts:9

```typescript
JSON.stringify(discovery.issues)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/🏷️export-identity/🟦️.ts:16

```typescript
JSON.parse(source.text)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️json-document/🟦️.ts:10

```typescript
JSON.parse(value.slice(0, -1).trim())
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️json-document/🟦️.ts:11

```typescript
JSON.stringify(key)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🔣️json-schema/🟦️.ts:45

```typescript
JSON.parse(text)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🟦️typescript/🟦️.ts:34

```typescript
JSON.stringify([sourceId, name, exported])
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:15

```typescript
JSON.parse(readFileSync(join(repoRoot, path), "utf8"))
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:302

```typescript
JSON.stringify(row)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:373

```typescript
JSON.stringify(row.widget)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:452

```typescript
JSON.stringify(pointer)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:516

```typescript
JSON.stringify(id ?? null)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:536

```typescript
JSON.stringify(under)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:542

```typescript
JSON.stringify(census ? report.census : listed ? report.inputs : { diagnostics: report.diagnostics, census: report.census, multiline: report.multiline }, null, 2)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:593

```typescript
JSON.stringify(outcome.invariant)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:593

```typescript
JSON.stringify(id)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:644

```typescript
JSON.stringify(value, (_, entry: unknown) => (isRecord(entry) ? Object.fromEntries(Object.entries(entry).sort(([left], [right]) => left.localeCompare(right))) : entry))
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:665

```typescript
JSON.stringify(wireName)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:665

```typescript
JSON.stringify(keys)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:666

```typescript
JSON.stringify(layout.tag)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:666

```typescript
JSON.stringify(wireName)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:666

```typescript
JSON.stringify(value[layout.tag] ?? keys)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:669

```typescript
JSON.stringify(layout.tag)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:669

```typescript
JSON.stringify(layout.content)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:669

```typescript
JSON.stringify(extra)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:721

```typescript
JSON.parse(step.docString)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1016

```typescript
JSON.stringify(key)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1027

```typescript
JSON.stringify(key)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1135

```typescript
JSON.stringify(documents.get(declared))
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1135

```typescript
JSON.stringify(document)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1145

```typescript
JSON.stringify(documents.get(document.$id))
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1145

```typescript
JSON.stringify(document)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1173

```typescript
JSON.stringify(candidate.layout)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1173

```typescript
JSON.stringify(candidates[0]!.layout)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1188

```typescript
JSON.stringify(finding.pointer)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1221

```typescript
JSON.stringify(tag)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1221

```typescript
JSON.stringify(wireName)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1243

```typescript
JSON.parse(readFileSync(join(repoRoot, fixture), "utf8"))
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1309

```typescript
JSON.stringify(entry.kind)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1313

```typescript
JSON.stringify(entry.kind)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1355

```typescript
JSON.stringify(census ? report.census : report, null, 2)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1696

```typescript
JSON.stringify(site.texts)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1716

```typescript
JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:2112

```typescript
JSON.stringify(site.text)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:2141

```typescript
JSON.stringify(missing)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:2141

```typescript
JSON.stringify(extra)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:2157

```typescript
JSON.stringify(scope)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:2161

```typescript
JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:2538

```typescript
JSON.stringify(census ? report.census : report, null, 2)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:2592

```typescript
JSON.stringify({ diagnostics, fixtures: reports }, null, 2)
```

- 🧰️framework/🔨️modules/🧬️schema/🟦️.ts:307

```typescript
JSON.stringify([kind.id, kind.emoji, kind.iconId, kind.label, kind.filterable])
```

- 🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts:53

```typescript
JSON.stringify(key)
```

- 🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts:54

```typescript
JSON.stringify(value)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📥️source/🟦️.ts:16

```typescript
JSON.parse(bytes.toString("utf8"))
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🏃️execution/🟦️.ts:18

```typescript
JSON.stringify({ contractId: GENERATOR_ID, nodes, schemaVersion: 1, staleRemovals: [] })
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:7

```typescript
JSON.stringify(kind.id)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:7

```typescript
JSON.stringify(kind.emoji)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:7

```typescript
JSON.stringify(kind.iconId)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:7

```typescript
JSON.stringify(kind.label)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:42

```typescript
JSON.stringify(kind.id)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:42

```typescript
JSON.stringify(kind.emoji)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:42

```typescript
JSON.stringify(kind.iconId)
```

- 🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts:42

```typescript
JSON.stringify(kind.label)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/🟦️.ts:323

```typescript
JSON.stringify(expected)
```

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/🟦️.ts:590

```typescript
JSON.stringify(G3_EVENT_FIELDS)
```

- 🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts:70

```typescript
JSON.stringify(values)
```

- 🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts:98

```typescript
JSON.stringify(allowed)
```

- ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:758

```typescript
JSON.stringify(value)
```

- ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:812

```typescript
JSON.stringify(value)
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️contract/🟦️.ts:4

```typescript
JSON.stringify(result.value??null)
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️contract/🟦️.ts:4

```typescript
JSON.stringify(vector.value)
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️contract/🟦️.ts:4

```typescript
JSON.stringify(inferGltfAxisAlignedBounds(context).value)
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️contract/🟦️.ts:4

```typescript
JSON.stringify(result.value)
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧪️contract/🟦️.ts:7

```typescript
JSON.stringify(result.value ?? null)
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧪️contract/🟦️.ts:7

```typescript
JSON.stringify(vector.value)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts:100

```typescript
JSON.stringify(node.const)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts:101

```typescript
JSON.stringify(node.enum)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:64

```typescript
JSON.stringify(node.const)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:64

```typescript
JSON.stringify(value)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:64

```typescript
JSON.stringify(node.const)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:65

```typescript
JSON.stringify(option)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:65

```typescript
JSON.stringify(value)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:65

```typescript
JSON.stringify(node.enum)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:86

```typescript
JSON.stringify(item)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧬️schema/🛂️validation/🟦️.ts:15

```typescript
JSON.parse(readFileSync(DEV_SCHEMA_URL, "utf8"))
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts:17

```typescript
JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"))
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts:33

```typescript
JSON.stringify(left)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts:33

```typescript
JSON.stringify(right)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts:46

```typescript
JSON.stringify(node.const)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts:47

```typescript
JSON.stringify(node.enum)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts:64

```typescript
JSON.stringify(item)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧬️schema/🟦️.ts:154

```typescript
JSON.stringify(ordered)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧬️schema/🟦️.ts:162

```typescript
JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes))
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🟦️.ts:14

```typescript
JSON.stringify(descriptorChannel)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🟦️.ts:22

```typescript
JSON.parse(source)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts:164

```typescript
JSON.stringify(value, (_key, item: unknown) => (typeof item === "bigint" ? item.toString() : item instanceof Uint8Array ? `<${item.byteLength} bytes>` : item))
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🟦️.ts:22

```typescript
JSON.stringify(entry.name)
```

- ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts:73

```typescript
JSON.stringify(key)
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts:11048

```typescript
JSON.stringify(schema["const"])
```

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🟦️.ts:37

```typescript
JSON.stringify(value.id)
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️plan/🎚️config/🧬️schema/🟦️.ts:39

```typescript
JSON.stringify(next[key])
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️plan/🎚️config/🧬️schema/🟦️.ts:39

```typescript
JSON.stringify(base[key])
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️world/🎚️config/🧬️schema/🟦️.ts:85

```typescript
JSON.stringify(next[key])
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️world/🎚️config/🧬️schema/🟦️.ts:85

```typescript
JSON.stringify(base[key])
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️section/🎚️config/🧬️schema/🟦️.ts:45

```typescript
JSON.stringify(next[key])
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️section/🎚️config/🧬️schema/🟦️.ts:45

```typescript
JSON.stringify(base[key])
```
