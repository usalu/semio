# TXT UTF-8 Any Changed-File Packet

This packet lists this bounded TXT lane's intended direct writes. It excludes
unrelated concurrent checkout changes and never includes `compose/**`.

## Semantic Owners

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔨️modules/🧬️mutation-support/🦀️component.rs`
- `.../🧬️schema/🧬️mutations/🦀️component.rs`
- `.../🧬️schema/🧬️mutations/{✏️set-trailing-newline,✏️set-line-ending,📥️insert-line,🗑️remove-line,✏️set-line}/🦀️component.rs`
- The ten corresponding leaf `📝️text/🦀️component.rs` and
  `💾️binary/🦀️component.rs` files.

## Root Carrier and Facet Mirrors

- `.../🧬️schema/🧬️mutations/📝️text/{🦀️component.rs,📖️component.grammar.semio,🅰️component.g4}`
- `.../🧬️schema/🧬️mutations/💾️binary/{🦀️component.rs,📡️component.protocol.semio,🔠️component.abnf,🥋️component.ksy,🌶️component.spicy}`
- `.../🧬️schema/🧬️mutations/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`

## TXT Consumers and Test Owners

- `.../✏️editor/🦀️component.rs`
- `.../🧬️schema/🔺️diff/🦀️component.rs`
- `.../🧪️oracle/🦀️component.rs`
- `.../🧪️oracle/🔣️component.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🧪️tests/mutate-txt-utf-8/🦀️component.rs`
- `.../📚️examples/🎬️demo/🦀️component.rs` (removal of the stale whole-enum
  byte constant only).

## Ticket Evidence

- `📓️txt-utf-8-any-semantic-closure.md`
- `📓️txt-utf-8-any-changed-file-packet.md`
- `🧪️txt-production-leaf-runtime/🧪️metadata-retry.log` (root-produced mounted
  actual-source runtime evidence)

Status: source formatting, schema/static probes, and scoped mounted production
runtime are green. Full registered STDIO/Nx acceptance remains owned by the
root lane.
