# Owned TypeScript Declaration Parser 61

## Current Status

Initial source implementation, not final acceptance. Only D's new `//#region 🟦️TypeScriptDeclarationFacts` was added (lines6434–7015 at the initial source hash). D is held unchanged for independent ticket62 probes. No N/S/Rust/registry/metadata/package/canonical-test changes were made by this owner. No Cargo/rustc, global scan, candidate-source evaluation or nested-repository access occurred.

The unchanged schema-first ticket57 controller genuinely passed all28 compiler-oracle and actual-subject cases through Bun/Nx. That does not close general lexical/parser correctness: the read-only member review below found two false-complete malformed-member cases and one mislabeled valid index signature. These remain open pending schema-first regression ownership and the D hold release.

## Source Boundary

- Before source: `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423`,655775bytes.
- Initial source: `2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e`,697928bytes.
- New region: `8a631938a652283026e53e0b731addaafdf56aab58aa157b274afeca709dd207`.
- Removing exactly the newly inserted region and its separating newline reproduces the original whole-D hash and655775bytes. Existing imports/declarations are byte-for-byte unchanged.

The public API owns its result/diagnostic/span types, rejects invalid language/source inputs with TypeError, and parses provided strings only. Lexical tokens retain raw UTF16 source coordinates separately from decoded identifier names. Literal scanning is atomic; delimiter pairing is iterative. Ambiguous recovery or call-stack exhaustion returns explicit incomplete evidence, never a success-shaped empty record. There is no eval/new Function, runtime compiler dependency, N import cycle, IO inside the API or global fact cache. Member/type summaries do not resolve symbols, mutation identity, providers or values.

## Test-First Evidence

Root/presence authored and ran the reviewed schema/reference28 packet before any D write. The retained actual missing-export RED at `🧪️off-facet-typescript-declaration-census-57/🧫️run-wW51Zq` was followed by the source splice and actual subject GREEN below. The earlier25 RED `run-6uNrfx` was also read and verified against original D. Original14 case structural fingerprints remain checked independently of oracle output by the frozen controller.

Command actually executed:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun <ticket>/🧪️off-facet-typescript-declaration-census-57/📜️script.ts subject
```

Exit0, actual subject28/28 and TypeScript5.9.3 reference28/28, all4 inputs stable:

```json
{
  "receipt": {
    "device": "16777230",
    "inode": "134925571",
    "mode": "33188",
    "size": "31098",
    "modifiedNs": "1787877923113610096",
    "changedNs": "1787877923113610096",
    "sha256": "03d945a576330f97f63dd5391ff8f9e0b62221f512c59fe22cae9f29b0605e22",
    "bytes": 31098
  },
  "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧫️run-I8RKCa/🔣️result.json",
  "mode": "subject",
  "oracle": {
    "passed": 28,
    "total": 28,
    "typescript": "5.9.3"
  },
  "subject": {
    "mode": "subject",
    "status": "passed",
    "passed": 28,
    "total": 28
  },
  "before": {
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧬️schema/🔣️.json": {
      "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
      "bytes": 3757
    },
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🔣️.json": {
      "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
      "bytes": 16781
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
      "sha256": "2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e",
      "bytes": 697928
    },
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts": {
      "sha256": "06f7c6ad496081380dd6c743a20eafebdcd6c4f283b733e588452c6b09ea6aca",
      "bytes": 22460
    }
  },
  "after": {
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧬️schema/🔣️.json": {
      "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
      "bytes": 3757
    },
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🔣️.json": {
      "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
      "bytes": 16781
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
      "sha256": "2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e",
      "bytes": 697928
    },
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts": {
      "sha256": "06f7c6ad496081380dd6c743a20eafebdcd6c4f283b733e588452c6b09ea6aca",
      "bytes": 22460
    }
  },
  "stable": true,
  "source": {
    "fingerprint": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "697928",
      "modifiedNs": "1787877914078086346",
      "changedNs": "1787877914078086346",
      "sha256": "2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e",
      "bytes": 697928
    },
    "regionSha256": "8a631938a652283026e53e0b731addaafdf56aab58aa157b274afeca709dd207",
    "startLine": 6434,
    "endLine": 7015,
    "outsideRegionSha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423"
  }
}
```

## Open Member-Grammar Review

These are read-only executions of the actual exported API against three small neutral source strings, alongside actual TypeScript parse diagnostics. The first two desired completeness values were authored before execution; they are genuine observed mismatches, not a passing oracle. They have not yet been mounted as canonical schema-owned goldens, and do not increase the accepted28 count.

The valid index signature has no computed property name in TypeScript; the current owned member reader conflates its bracketed parameter syntax with a computed name. It must either represent that grammar accurately or report an honest unsupported reason, not computed-property evidence.

```json
{
  "kind": "actual read-only new-D syntax review",
  "sourceBefore": {
    "device": "16777230",
    "inode": "109708023",
    "mode": "33188",
    "size": "697928",
    "modifiedNs": "1787877914078086346",
    "changedNs": "1787877914078086346",
    "sha256": "2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e",
    "bytes": 697928
  },
  "sourceAfter": {
    "device": "16777230",
    "inode": "109708023",
    "mode": "33188",
    "size": "697928",
    "modifiedNs": "1787877914078086346",
    "changedNs": "1787877914078086346",
    "sha256": "2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e",
    "bytes": 697928
  },
  "stable": true,
  "results": [
    {
      "id": "malformed-interface-member",
      "source": "export interface Shape { value string }",
      "expectedCompleteness": "incomplete",
      "actual": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "interface",
            "name": "Shape",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 39
            },
            "structure": {
              "form": "object",
              "members": [
                "value"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      },
      "compilerParseDiagnostics": [
        {
          "start": 25,
          "length": 5,
          "message": "Property or signature expected."
        },
        {
          "start": 38,
          "length": 1,
          "message": "Declaration or statement expected."
        }
      ]
    },
    {
      "id": "malformed-object-member",
      "source": "export const metadata = { kind value };",
      "expectedCompleteness": "incomplete",
      "actual": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "variable",
            "name": "metadata",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 13,
              "end": 38
            },
            "structure": {
              "form": "object",
              "members": [
                "kind"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      },
      "compilerParseDiagnostics": [
        {
          "start": 31,
          "length": 5,
          "message": "',' expected."
        }
      ]
    },
    {
      "id": "type-index-signature",
      "source": "export interface Shape { [key: string]: number }",
      "expectedCompleteness": null,
      "actual": {
        "completeness": "incomplete",
        "declarations": [
          {
            "kind": "interface",
            "name": "Shape",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 48
            },
            "structure": {
              "form": "object",
              "members": [],
              "unresolved": "computed-property"
            }
          }
        ],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unresolved-computed-property",
            "span": {
              "start": 0,
              "end": 48
            }
          }
        ]
      },
      "compilerParseDiagnostics": []
    }
  ]
}
```

## Pending Review

Await independent ticket62 stress/lexical receipt and exact source release, then root-owned regression data before repairing the member grammar. Root owns canonical test/oracle/package integration. Final source hashes, new-region-only proof and actual rerun results will be appended; no general parser/census/mutation completeness claim is made from28 cases.

## Member Repair Follow-Up

The original member REDs above are now retained historical evidence. The exact independent12-case schema/RED, source repair, actual canonical85/shared28/ticket12 GREEN and unchanged-region proof are recorded in [Member Repair63](./📓️typescript-declaration-member-repair-63.md). A subsequent valid-but-unsupported diagnostic issue is retained separately; Stage A is not generalized compiler or final diagnostic acceptance.

The conservative diagnostic follow-up is now complete in that report: canonical88/shared28/malformed12 actual GREEN at D8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae, with the valid predicate retained as unsupported rather than falsely labeled a syntax error. The source packet is released for root review; the stated finite-coverage and index-signature precision limits remain explicit.
