# Current Retired Schema Reference Audit

Read-only current-source audit of authored framework contracts and runtime source against the runtime retired-schema maps. Parsed 666 contract JSON files, examined 3821 references, and checked 29 retired schema paths/identities. Local JSON Pointers and file references were resolved. Static relative TypeScript schema imports were checked for existing files. Test/fixture payloads and generated/dependency trees were excluded from the runtime corpus scan.

0 parse/local-reference errors and 9 retired-runtime-reference or missing-static-schema-import findings. This is source evidence, not a runtime test claim.

```json
{
  "errors": [],
  "hits": [
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema/🔣️.json"
    },
    {
      "kind": "retiredPathLiteral",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json",
      "retiredPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema/🔣️.json"
    }
  ]
}
```
