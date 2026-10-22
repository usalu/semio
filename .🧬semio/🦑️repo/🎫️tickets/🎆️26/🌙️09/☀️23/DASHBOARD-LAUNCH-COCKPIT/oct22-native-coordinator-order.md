# Native Retirement Coordinator Lease Order

Source frozen for independent review on 2026-10-22. No producer, evaluated product Source, subprocess, actual reset or unchanged retry was executed by this repair lane.

## Narrow Resource Relation

The authored policy now classifies `native-whole-retirement:` as generation-scoped Native storage coordination at rank 5, before image rank 6. Its only new allowed edge is coordinator to image, with the closed schema-owned binding `{kind: "same-directory-suffix", fromPrefix: "native-whole-retirement:", toPrefix: "native-fixed-image:"}`. The generic matcher requires exact lease-directory equality, both declared resource prefixes and equal nonempty suffixes. It contains no Native-specific branch. A foreign image, foreign lease catalog, journal/persistent-state/other image prefix, unclassified resource or reverse image-to-coordinator request receives no exemption. Existing capacity edges remain unbound and unchanged. Grace 15000 ms, polling 250 ms, self-deadlock, cycle, fairness and all existing defaults are preserved.

## Actual Lifetime Trace

The two coordinator call paths are terminal retirement after the actual image has closed, and resume before any canonical image acquisition. The whole coordinator spans the same canonical image opening guards, factory shared custody, maintenance EX, normal close, backend EX reacquisition and detach. Actual Native factory guards in storage/fixed and persistent cleanup request only `native-fixed-image:<options.directory>` from the same opening lease directory. Resume checks the real journal directly while holding actual image EX; it does not acquire a `native-storage:<backing identity>` lease. Initial domain reservation/journal recovery precedes close and coordinator acquisition. No reverse ordinary image-to-coordinator acquisition exists on the selected actual reset lifecycle. Normal factory close joins accepted image operations and releases its reader before terminal coordinator acquisition; the new policy grants no reverse edge.

## Schema and Neutral Coverage

Schema was updated first with the closed optional generic edge binding, followed by a closed Native coordinator-case contract and twelve language-neutral cases before implementation. The existing law `Native durable terminal continuation matches SQLite exact remaining-subset relation` is extended; all five Native law names and the existing `@semio-tech/framework-process:test-native-storage-platform` mount remain unchanged. The SQLite oracle independently classifies exact/longest-prefix classes and evaluates allowed binding relations. The authored law compares those results with actual production `classifyLeaseResourceV1`, `checkHoldAndWaitV1` and `checkLeaseOrderV1`.

Cases cover same image after grace, Windows identity, foreign image/catalog, journal/persistent/unclassified prefixes, reverse acquisition, empty suffix, unrelated Graph, unchanged grace and the existing capacity-to-Graph edge.

## Qualification

Executed only static/data checks: strict no-emit TypeScript reports zero semantic diagnostics for the generic order implementation and Native law leaf; both parse without syntax diagnostics and every finite direct relative import resolves. Ajv accepts the current closed policy and all twelve new closed cases. An independent SQLite DATA-only oracle agrees with all twelve authored expected outcomes. No production TypeScript was evaluated for these checks. Actual policy acquisition behavior and Native laws remain pending their genuine owned repository routes; static acceptance is not runtime passage.

## Frozen Changed Source

| Path | SHA-256 | Bytes |
| --- | --- | ---: |
| 🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts | 29b811107fd1f319f445e9192ef00cf77e5c9c72e1ff34bdd9d84ec8fa44ce0e | 25666 |
| 🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json | 223f2da7bc9f44e14d4728f1cc4f24278bd029e66408b4bd862f883905f493d8 | 8496 |
| 🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧬️schema/🔣️.json | 441218defeeb207dc3b4f82136bb1eb44d27009bec09983e86206af30b8cdbe0 | 14244 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🧬️schema/🔣️.json | bc8b24a656f8757620d8b76ff4f1f8fd5143682151425a6393b6f85a019b24c8 | 5197 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🧪️tests/🔣️.json | 4d55f19dfaecc7bf4a481fc5067f3b19f3b8a420e5f8a0f327ea001a203f6fa4 | 8871 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🧪️tests/🟦️.ts | da726d655833b9bd8bf24306a1aecf0599b8e3de598458e8baf9c63b8fbed612 | 12691 |
