# Chromium Office XLSX codec ownership

`prove-office-browser-codec.ts` invokes the actual Office guest in repository-pinned Chromium with JSPI. The local fixture serves the existing production-transpiled module in `🗑️generated/office-web` and generates its bridge with `pluginComponentBridgeSource`. The real kernel bootstrap issues the operation from the existing finite composition fixture and checks each receipt. The guest closes the original source and the bootstrap reports zero live sources. The descriptor returned through the same bridge matches the native emitted semantic Pack in `🗑️generated/office-emitted-replay` byte for byte. The probe exited 0.

| Browser bridge operation | Selector | Additional input |
| --- | --- | --- |
| `pack-schema-hash` | `stdio.xlsx` | |
| `pack-schema-hash` | `s.stdio.xlsx` | |
| `genesis` | `stdio.xlsx` | `artifact-0123456789abcdef0123456789abcdef` |
| `genesis` | `s.stdio.xlsx` | `artifact-0123456789abcdef0123456789abcdef` |

Each real guest answer contains a fault with code `plugin.internal`, origin `plugin`, and message `artifact codec schema resolves more than one app of the same role`. The test passes when it reproduces that failure. The guest returned no XLSX Pack/SPR from genesis, so this fixture cannot make a paired `print-mirror` call.

## Ownership rule and prerequisites

The Office descriptor supplies an editor and viewer for each of `s.stdio.xlsx@ecma-376/*`, `s.stdio.xlsx@ecma-376/strict`, and `s.stdio.xlsx@ecma-376/transitional`. Every app uses `stdio.xlsx` as its document schema. The guest's `artifact_codec_candidates` matches the native registered schema or the declared artifact kind. Its `artifact_codec_owner` currently refuses the second candidate of either role. The native artifact definition separately declares a single XLSX native codec identity with the structural Pack schema hash `b81858b598a8d0bdcbcc36b48291e9d3a3f638adf4cac479e5ebce714c9a1a17`.

The existing [neutral most-general-dialect law](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🧫️fixtures/🌳️most-general-dialect/🔣️.json) names this same guest defect and calls out XLSX. A whole-standard dialect covers named subsets of that same kind and standard. The unique uncovered declaration supplies the codec for the generic selector. Equal duplicate declarations, competing standards, unrelated kinds and incomparable subsets still have multiple uncovered candidates. The Hub trusted catalog already implements the rule and checks the fixture both forwards and backwards. The probe independently evaluated all 13 neutral cases with SQLite and confirmed that the whole-standard editor is the unique uncovered declaration in the captured Office manifest. The guest can apply the same rule per role, with the existing editor preference. The existing guest test exercises app-free codecs under distinct registered schemas and misses overlapping declarations.

The guest resolver now uses the shared framework selector separately for editors and viewers and prefers the selected editor. Candidate admission still matches exactly the registered document schema or the declared artifact kind. The existing SQLite operations still parse their explicit dialect coordinate separately. The guest declaration test drives all 13 neutral rows through registered editor and viewer candidates, in both orders, and repeats matching-kind rows through the kind selector. The pinned Rust parser and scoped whitespace check passed. A standalone pinned Rust check compiled the exact production guest selector and shared helper with minimal type fixtures and passed all 13 cases in both orders and roles. The full guest test requires the managed build.

The fresh Chromium check requires a rebuilt Office component, its matching native descriptor pair, and production transpilation. It compares the returned schema hash to the native factory declaration, checks byte identity of Pack/SPR and the printed mirror for the generic kind and schema, and checks the requested document identity in the operations text. A real XLSX picker import and export requires managed browser-product admission and session activation. The separate Chromium workbook oracle reads the positive XLSX workbook independently of the actor.

```sh
repo tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/prove-office-browser-codec.ts
```

A fresh capture uses its own generated directories.

```sh
repo tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/prove-office-browser-codec.ts --fresh MODULE_DIRECTORY DESCRIPTOR_DIRECTORY
```

## Shared dialect selection

Pinned Chromium exercised the production-transpiled Office guest and generated kernel bridge. Both the `stdio.xlsx` document schema and the `s.stdio.xlsx` artifact kind reached the guest and returned its structured `plugin.internal` ownership fault. XLSX registers Base, Strict and Transitional with the same schema in each surface role. The previous guest resolver rejected the second candidate before it considered the dialect.

The artifact-reference owner now supplies the same selection to the guest and to Hub creation and default opening. A whole-standard declaration covers named subsets of its own artifact kind and standard. Exactly one uncovered declaration wins in each role. An uncovered tie still yields the structured ambiguity fault. The guest prefers the editor and then the viewer. Native SQLite selection continues to use its exact dialect coordinate.

Pinned Rust compiled the production selection module. SQLite independently calculated the same outcome for all 13 existing language-neutral cases in both declaration orders. The guest law uses the same fixture and admits candidates through actual plug-in registrations. The guest selector also passed a standalone pinned Rust extraction over the fixture, in editor and viewer roles. The full registered guest law awaits its normal native test executable. The negative Chromium replay passed. The positive mode turned the same old fault into a failing assertion, confirming the positive check actually reaches the codec.

```sh
repo tools exec --light bun -- "$ticket/verify-dialect-selection.ts"
repo tools exec --light bun -- "$ticket/prove-office-browser-codec.ts"
repo tools exec --light bun -- "$ticket/prove-office-browser-codec.ts" --fresh "$module_dir" "$descriptor_dir"
```

The fresh `repo cargo` build has its own WASIp2 Cargo log and live artifact-capture watcher. The local proof regenerates the native descriptor, verifies the emitted pair, runs the production browser transpiler, then checks schema hash, Pack/SPR and mirror through the actual Chromium actor. Product publication still goes through the managed Office component target and its tool receipts. English and German editor journeys, the Hub I/O matrix and the standalone pane still await managed activation.

### Fresh guest codec verdict

The managed Cargo WASIp2 link produced a fresh 198,371,810-byte guest with SHA-256 `29c9a312482dd4cb90d2ad42aa3bdfeff02bbfd5a2bc5e739ffc1294d87458ee`. Cargo emitted its successful compiler-artifact and build-finished records. The current prototype native descriptor emitter, verified against its own retained compiler receipt, emitted semantic digest `29f7fd8f689bcf37cd9a5c030f6c786ef23589e9e21891b4ece0010ecb127e55`. The repository pair verifier and Ajv passed.

Pinned Chromium loaded the transpiled guest through the generated kernel bridge. `pack-schema-hash("stdio.xlsx")` reached the selected Base editor and returned `plugin.internal` with `artifact codec schema has no structural record specification`. This is one step later than the previous role ambiguity and leaves the structural hash verdict open. The fresh probe exited 1. The diagnostic mode checks genesis independently of the hash result and follows with a mirror call if it receives a pair.

The repo wrapper automatically requested another Cargo snapshot because a peer edited its profile constraints after the successful compiler run. That superseded retry was retired after the genuine guest and descriptor were captured. The next component build needs the corrected structural registration.

### Verified guest genesis and mirror

Pinned Chromium ran the captured fresh Office guest through the production bridge and compared both accepted identities. `stdio.xlsx` and `s.stdio.xlsx` each produced a 1,448-byte Pack and 247-byte SPR. Their SHA-256 values matched across identities. The DSL and operations mirrors likewise had matching digests. The operations header was `doc artifact-0123456789abcdef0123456789abcdef schema=stdio.xlsx`. The descriptor matched the native emitter, and the original Source closed with zero live sources.

| Output | SHA-256 |
| --- | --- |
| Pack | `5c53471d3672b797f600c60c9986775a2d54caea3619794702b4f77d21d73cf6` |
| SPR | `c8f033f7646cc2a23b25f9b93122031f7e19eb0d791562178eabd799c5ea5894` |
| DSL mirror | `400ee862611fd8fb4254a505656026de3d64ff8b31a416b9d0fef56ed9e03061` |
| Operations mirror | `28a6e45998f79b7a1ba9f9836f053c450c261dff13a67dfd2e2a3dfe18343474` |

Both `pack-schema-hash` calls returned the same structural-spec refusal. The targeted repair connects the guest codec to the authored protocol already used by the native factory.

### Mounted edit and Hub routing

The mounted XLSX editor's SetCell command produces the typed XLSX mutation under the live document revision. The configured Office XLSX row in the Hub I/O matrix edits A1 through this surface and exercises the file picker and natural export. The verified Office-hosted XLSX target receives the exact `stdio.native.xlsx.v1` binding from Stdio after native schema and hash verification. Bound Hub operation apply and Check In use the linked codec's concrete decode, reduction, pair-print and close paths. Guest genesis continues to execute in the Office component for this target. These routing facts come from source inspection. The managed journey and Hub I/O execution supply their own outcomes.
