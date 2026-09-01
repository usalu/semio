# TypeScript Declaration Member Repair 63

## Stage A: Malformed-Source Boundary

This source packet changes only the new D TypeScriptDeclarationFacts region. The schema-first malformed12 cases were authored and executed before repair by the independent test owner, then mounted unchanged by root. Root's canonical desired RED was73PASS/12FAIL/1028assertions across85tests; the agent read all12 source strings, schema, controller, exact compiler diagnostic tuples and the retained actual RED. Initial28 expectations were not edited.

The first repair replaces unchecked member-tail skipping with grammar consumption of property/method tails, parameter annotations, generic constraints/defaults, enum initializers and separators. Declaration headers are validated before their facts are emitted. It is a syntactic subset, not TypeScript type resolution or provider/mutation identity.

Stage A passed canonical85, unchanged shared28 and ticket63's malformed12, as actually executed below. Root subsequently identified a valid-but-unsupported type predicate incorrectly labeled parse-error; that genuine RED is retained in [the diagnostic supplement](./📓️typescript-declaration-unsupported-diagnostic-63.md). Stage A alone is therefore not final diagnostic acceptance. The final conservative classification repair and reruns will be appended.

## Exact Source Footprint

The complete retained initial parser region is compared by TypeScript AST declaration/member text to the current source. No scanner change was made in this repair. The changed methods and source hashes below are actual captured values, not a git reconstruction.

```json
{
  "prewrite": {
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
    "outsideSha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423"
  },
  "after": {
    "source": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "708754",
      "modifiedNs": "1787878652832003606",
      "changedNs": "1787878652832003606",
      "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
      "bytes": 708754
    },
    "regionSha256": "5d70f9f6e8f422781d4daeea54dba95bac36b6e9650d68719cded6ef20ffcc09",
    "regionBytes": 52977,
    "firstLine": 6434,
    "lastLine": 7218,
    "outsideRegionSha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423",
    "outsideRegionBytes": 655775,
    "imports": 0,
    "forbiddenCallTargets": [],
    "astParseDiagnostics": []
  },
  "changes": {
    "source": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "708754",
      "modifiedNs": "1787878652832003606",
      "changedNs": "1787878652832003606",
      "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
      "bytes": 708754
    },
    "changes": [
      {
        "name": "TypeScriptDeclarationParser",
        "change": "changed"
      },
      {
        "name": "TypeScriptDeclarationParser.memberEnd",
        "change": "removed"
      },
      {
        "name": "TypeScriptDeclarationParser.members",
        "change": "changed"
      },
      {
        "name": "TypeScriptDeclarationParser.variables",
        "change": "changed"
      },
      {
        "name": "TypeScriptDeclarationParser.parse",
        "change": "changed"
      },
      {
        "name": "inspectTypeScriptDeclarationFacts",
        "change": "changed"
      },
      {
        "name": "TypeScriptDeclarationSyntaxError",
        "change": "added"
      },
      {
        "name": "TypeScriptDeclarationSyntaxError.constructor",
        "change": "added"
      },
      {
        "name": "TypeScriptDeclarationParser.invalid",
        "change": "added"
      },
      {
        "name": "TypeScriptDeclarationParser.typeEnd",
        "change": "added"
      },
      {
        "name": "TypeScriptDeclarationParser.typeAtom",
        "change": "added"
      },
      {
        "name": "TypeScriptDeclarationParser.typeParameters",
        "change": "added"
      },
      {
        "name": "TypeScriptDeclarationParser.parameters",
        "change": "added"
      },
      {
        "name": "TypeScriptDeclarationParser.expressionEnd",
        "change": "added"
      }
    ],
    "unchangedScanner": true
  }
}
```

At this stage removing exactly the new region and its separating newline reproduces the entire original655775-byte D preimage, SHA807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423. The new region has zero imports and no eval, Function or require invocation in the checked AST. Root has separately announced a later authorized taxonomy manifest-helper edit outside this region; any later outside-region drift must be recorded independently rather than erased or attributed to this parser packet.

## Actual Canonical85 Terminal and Stable Inputs

Full nofollow/full-ancestry/O_NOFOLLOW/fstat source captures bracketed the actual Bun/Nx command. All seven identities and hashes matched. The raw terminal is retained inside this receipt, including all85 names, strict region/oracle type checks, schema checks and actual diagnostics assertions.

```json
{
  "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test \"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts\"",
  "before": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708754",
        "modifiedNs": "1787878652832003606",
        "changedNs": "1787878652832003606",
        "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
        "bytes": 708754
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "7884",
        "modifiedNs": "1787878489895812199",
        "changedNs": "1787878489895812199",
        "sha256": "ae2ae321e4ffe67e796e5877e5b553712bd326462e42aa169f27cdda9000dca8",
        "bytes": 7884
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926110",
        "mode": "33188",
        "size": "16781",
        "modifiedNs": "1787878027869531198",
        "changedNs": "1787878027869531198",
        "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
        "bytes": 16781
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926112",
        "mode": "33188",
        "size": "3757",
        "modifiedNs": "1787878027869831199",
        "changedNs": "1787878027869831199",
        "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
        "bytes": 3757
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926114",
        "mode": "33188",
        "size": "15589",
        "modifiedNs": "1787878489895441197",
        "changedNs": "1787878489895441197",
        "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
        "bytes": 15589
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928884",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878489703416097",
        "changedNs": "1787878489703416097",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928886",
        "mode": "33188",
        "size": "1564",
        "modifiedNs": "1787878489854494334",
        "changedNs": "1787878489854494334",
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      }
    }
  ],
  "after": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708754",
        "modifiedNs": "1787878652832003606",
        "changedNs": "1787878652832003606",
        "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
        "bytes": 708754
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "7884",
        "modifiedNs": "1787878489895812199",
        "changedNs": "1787878489895812199",
        "sha256": "ae2ae321e4ffe67e796e5877e5b553712bd326462e42aa169f27cdda9000dca8",
        "bytes": 7884
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926110",
        "mode": "33188",
        "size": "16781",
        "modifiedNs": "1787878027869531198",
        "changedNs": "1787878027869531198",
        "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
        "bytes": 16781
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926112",
        "mode": "33188",
        "size": "3757",
        "modifiedNs": "1787878027869831199",
        "changedNs": "1787878027869831199",
        "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
        "bytes": 3757
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926114",
        "mode": "33188",
        "size": "15589",
        "modifiedNs": "1787878489895441197",
        "changedNs": "1787878489895441197",
        "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
        "bytes": 15589
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928884",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878489703416097",
        "changedNs": "1787878489703416097",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928886",
        "mode": "33188",
        "size": "1564",
        "modifiedNs": "1787878489854494334",
        "changedNs": "1787878489854494334",
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      }
    }
  ],
  "stable": true,
  "terminal": {
    "chunk_id": "fd0d7c",
    "wall_time_seconds": 4.766999417,
    "exit_code": 0,
    "original_token_count": 1730,
    "output": "bun test v1.3.14 (0d9b296a)\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts:\n(pass) TypeScript declaration facts use the closed neutral schema [4.29ms]\n(pass) TypeScript declaration reference: comment-string-template [6.08ms]\n(pass) TypeScript declaration subject: comment-string-template [2.20ms]\n(pass) TypeScript declaration reference: local-same-name-mutation [0.28ms]\n(pass) TypeScript declaration subject: local-same-name-mutation [0.09ms]\n(pass) TypeScript declaration reference: imported-type-alias [0.95ms]\n(pass) TypeScript declaration subject: imported-type-alias [0.22ms]\n(pass) TypeScript declaration reference: literal-discriminated-union [0.39ms]\n(pass) TypeScript declaration subject: literal-discriminated-union [0.30ms]\n(pass) TypeScript declaration reference: exported-object-metadata [0.94ms]\n(pass) TypeScript declaration subject: exported-object-metadata [0.31ms]\n(pass) TypeScript declaration reference: nested-namespace-interface [0.37ms]\n(pass) TypeScript declaration subject: nested-namespace-interface [0.07ms]\n(pass) TypeScript declaration reference: type-only-reexport [0.57ms]\n(pass) TypeScript declaration subject: type-only-reexport [0.08ms]\n(pass) TypeScript declaration reference: conditional-mapped-computed [1.35ms]\n(pass) TypeScript declaration subject: conditional-mapped-computed [0.38ms]\n(pass) TypeScript declaration reference: enum-class-declarations [1.15ms]\n(pass) TypeScript declaration subject: enum-class-declarations [0.15ms]\n(pass) TypeScript declaration reference: unsupported-module-regions [0.68ms]\n(pass) TypeScript declaration subject: unsupported-module-regions [0.14ms]\n(pass) TypeScript declaration reference: regex-template-asi [0.90ms]\n(pass) TypeScript declaration subject: regex-template-asi [0.65ms]\n(pass) TypeScript declaration reference: escaped-identifier [0.46ms]\n(pass) TypeScript declaration subject: escaped-identifier [0.15ms]\n(pass) TypeScript declaration reference: tsx-jsx-expression [0.74ms]\n(pass) TypeScript declaration subject: tsx-jsx-expression [0.62ms]\n(pass) TypeScript declaration reference: malformed-parse [0.51ms]\n(pass) TypeScript declaration subject: malformed-parse [0.04ms]\n(pass) TypeScript declaration reference: valid-empty-source [0.05ms]\n(pass) TypeScript declaration subject: valid-empty-source\n(pass) TypeScript declaration reference: valid-comment-only-source [0.07ms]\n(pass) TypeScript declaration subject: valid-comment-only-source [0.01ms]\n(pass) TypeScript declaration reference: mixed-default-named-import [0.13ms]\n(pass) TypeScript declaration subject: mixed-default-named-import [0.11ms]\n(pass) TypeScript declaration reference: object-spread [0.23ms]\n(pass) TypeScript declaration subject: object-spread [0.06ms]\n(pass) TypeScript declaration reference: heritage-and-class-body [0.47ms]\n(pass) TypeScript declaration subject: heritage-and-class-body [0.11ms]\n(pass) TypeScript declaration reference: computed-type-literal [0.17ms]\n(pass) TypeScript declaration subject: computed-type-literal [0.09ms]\n(pass) TypeScript declaration reference: union-conditional-mapped-members [0.64ms]\n(pass) TypeScript declaration subject: union-conditional-mapped-members [0.23ms]\n(pass) TypeScript declaration reference: unsupported-primitive-type [0.10ms]\n(pass) TypeScript declaration subject: unsupported-primitive-type [0.03ms]\n(pass) TypeScript declaration reference: bodyless-ambient-module [0.18ms]\n(pass) TypeScript declaration subject: bodyless-ambient-module [0.13ms]\n(pass) TypeScript declaration reference: nested-template-regex-division-asi-comments [0.37ms]\n(pass) TypeScript declaration subject: nested-template-regex-division-asi-comments [0.14ms]\n(pass) TypeScript declaration reference: unicode-dotted-namespace [0.24ms]\n(pass) TypeScript declaration subject: unicode-dotted-namespace [0.07ms]\n(pass) TypeScript declaration reference: generic-conditional-argument [0.14ms]\n(pass) TypeScript declaration subject: generic-conditional-argument [0.11ms]\n(pass) TypeScript declaration reference: property-mapped-type [0.10ms]\n(pass) TypeScript declaration subject: property-mapped-type [0.07ms]\n(pass) TypeScript declaration reference: constructor-accessor-static-bodies [0.93ms]\n(pass) TypeScript declaration subject: constructor-accessor-static-bodies [0.13ms]\n(pass) TypeScript declaration inspector rejects an unspecified or unsupported language [0.09ms]\n(pass) TypeScript declaration grammar has strict standalone source types [486.28ms]\n(pass) TypeScript declaration compiler oracle has strict source types [276.85ms]\n(pass) TypeScript malformed declaration cases use the closed neutral schema [1.31ms]\n(pass) TypeScript malformed declaration reference: malformed-const-header [0.34ms]\n(pass) TypeScript malformed declaration subject: malformed-const-header [0.24ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member [0.79ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member [0.09ms]\n(pass) TypeScript malformed declaration reference: malformed-object-member [0.07ms]\n(pass) TypeScript malformed declaration subject: malformed-object-member [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-alias-generic-default [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-alias-generic-default [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-generic-default [0.02ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-generic-default [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-class-generic-default [0.06ms]\n(pass) TypeScript malformed declaration subject: malformed-class-generic-default [0.05ms]\n(pass) TypeScript malformed declaration reference: malformed-generic-constraint [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-generic-constraint [0.05ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member-separator [0.06ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member-separator [0.05ms]\n(pass) TypeScript malformed declaration reference: malformed-enum-initializer [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-enum-initializer [0.07ms]\n(pass) TypeScript malformed declaration reference: malformed-nested-const-header [0.05ms]\n(pass) TypeScript malformed declaration subject: malformed-nested-const-header [0.14ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-property-type [0.05ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-property-type [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-class-parameter-type [0.06ms]\n(pass) TypeScript malformed declaration subject: malformed-class-parameter-type [0.15ms]\n\n 85 pass\n 0 fail\n 1040 expect() calls\nRan 85 tests across 1 file. [1042.00ms]\n"
  }
}
```

## Actual Shared28 and Ticket12 Receipts

The current shared57 controller consumes the canonical schema/vectors/test-only oracle and captures five inputs. Its older four-input receipts remain historical and are not relabeled. No schema, vector, oracle, controller, package, N, S, launch or registry source was changed by this owner.

```json
{
  "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧫️run-Ud8nGJ/🔣️result.json",
  "fingerprint": {
    "device": "16777230",
    "inode": "134929585",
    "mode": "33188",
    "size": "31552",
    "modifiedNs": "1787878705528539528",
    "changedNs": "1787878705528539528",
    "sha256": "faaaa4e23e6f9eef278cc7337c24378229884d671628330609ec14e814ead4fb",
    "bytes": 31552
  },
  "mode": "subject",
  "oracle": {
    "typescript": "5.9.3",
    "passed": 28,
    "total": 28
  },
  "sourceSubject": {
    "mode": "subject",
    "status": "passed",
    "passed": 28,
    "total": 28
  },
  "before": {
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json": {
      "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
      "bytes": 3757
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json": {
      "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
      "bytes": 16781
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts": {
      "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
      "bytes": 15589
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
      "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
      "bytes": 708754
    },
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts": {
      "sha256": "3ae174f1c9bb50b8dce421ac9e604e0a4735d564b39cfb07e180b8f18e650e7f",
      "bytes": 9911
    }
  },
  "after": {
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json": {
      "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
      "bytes": 3757
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json": {
      "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
      "bytes": 16781
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts": {
      "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
      "bytes": 15589
    },
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
      "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
      "bytes": 708754
    },
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts": {
      "sha256": "3ae174f1c9bb50b8dce421ac9e604e0a4735d564b39cfb07e180b8f18e650e7f",
      "bytes": 9911
    }
  },
  "stable": true
}
```

```json
{
  "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🧫️runs/b2702911-f789-48e9-87ee-316a3f95d57c/🔣️result.json",
  "fingerprint": {
    "device": "16777230",
    "inode": "134929588",
    "mode": "33188",
    "size": "6115",
    "modifiedNs": "1787878705663662831",
    "changedNs": "1787878705663662831",
    "sha256": "b67a3d06bf0cc7cbc1bfa129681cdc9d4549955f7b17c648b0aa95a8737cd16d",
    "bytes": 6115
  },
  "receipt": {
    "schemaVersion": 1,
    "mode": "check",
    "typescript": "5.9.3",
    "before": {
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🧬️schema/🔣️.json": {
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json": {
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
        "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
        "bytes": 708754
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/📜️script.ts": {
        "sha256": "13dea255920fe7883bb10e2968b67fed6edb7680c71eb60a321a7c4192ebbf79",
        "bytes": 7716
      }
    },
    "after": {
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🧬️schema/🔣️.json": {
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json": {
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
        "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
        "bytes": 708754
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/📜️script.ts": {
        "sha256": "13dea255920fe7883bb10e2968b67fed6edb7680c71eb60a321a7c4192ebbf79",
        "bytes": 7716
      }
    },
    "results": [
      {
        "id": "malformed-const-header",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 15,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-member",
        "compilerDiagnostics": [
          {
            "code": 1131,
            "start": 25,
            "length": 5
          },
          {
            "code": 1128,
            "start": 38,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-object-member",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 31,
            "length": 5
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-alias-generic-default",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 20,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-generic-default",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 25,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-class-generic-default",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 21,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-generic-constraint",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 26,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-member-separator",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 31,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-enum-initializer",
        "compilerDiagnostics": [
          {
            "code": 1109,
            "start": 19,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-nested-const-header",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 36,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-property-type",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 33,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-class-parameter-type",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 31,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      }
    ],
    "failures": [],
    "stable": true
  }
}
```

## Actual Owned Diagnostics

This additional read-only observation invoked the same current D export on the unchanged authored12 strings to preserve the diagnostic codes and raw half-open UTF16 coordinates omitted by the compact ticket controller. It adds no new golden or accepted test count. No recovered declaration or alias was emitted for any of these12 malformed declarations.

```json
{
  "before": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708754",
        "modifiedNs": "1787878652832003606",
        "changedNs": "1787878652832003606",
        "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
        "bytes": 708754
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134927662",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878305450757306",
        "changedNs": "1787878305450757306",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    }
  ],
  "after": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708754",
        "modifiedNs": "1787878652832003606",
        "changedNs": "1787878652832003606",
        "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
        "bytes": 708754
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134927662",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878305450757306",
        "changedNs": "1787878305450757306",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    }
  ],
  "stable": true,
  "actual": [
    {
      "id": "malformed-const-header",
      "source": "export const x y = {};",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 15,
              "end": 16
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-member",
      "source": "export interface Shape { value string }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 31,
              "end": 37
            }
          }
        ]
      }
    },
    {
      "id": "malformed-object-member",
      "source": "export const metadata = { kind value };",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 31,
              "end": 36
            }
          }
        ]
      }
    },
    {
      "id": "malformed-alias-generic-default",
      "source": "export type Box<T = > = T;",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 20,
              "end": 21
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-generic-default",
      "source": "export interface Box<T = > { value: T }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 25,
              "end": 26
            }
          }
        ]
      }
    },
    {
      "id": "malformed-class-generic-default",
      "source": "export class Box<T = > { value!: T; }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 21,
              "end": 22
            }
          }
        ]
      }
    },
    {
      "id": "malformed-generic-constraint",
      "source": "export type Box<T extends = string> = T;",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 26,
              "end": 27
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-member-separator",
      "source": "export interface I { x: string y: number }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 31,
              "end": 32
            }
          }
        ]
      }
    },
    {
      "id": "malformed-enum-initializer",
      "source": "export enum E { A =, B }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unsupported-recovery-suffix",
            "span": {
              "start": 19,
              "end": 24
            }
          }
        ]
      }
    },
    {
      "id": "malformed-nested-const-header",
      "source": "export namespace N { export const x y = {}; }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 36,
              "end": 37
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-property-type",
      "source": "export interface I { readonly x: ; }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 33,
              "end": 34
            }
          }
        ]
      }
    },
    {
      "id": "malformed-class-parameter-type",
      "source": "export class C { method(value: ) {} }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 31,
              "end": 32
            }
          }
        ]
      }
    }
  ]
}
```

## Explicit Limits

- Passing finite neutral28/malformed12 cases does not prove general TypeScript compiler parity, complete source census, symbol resolution or mutation identity.
- The separately observed valid index-signature diagnostic remains an incomplete but imprecise computed-property classification. This packet does not claim it represents an actual computed property and does not silently change that unreviewed contract.

The later authorized [Index Signature Diagnostic Boundary63](./📓️typescript-declaration-index-signature-63.md) now closes that specific limitation with a second schema-first unsupported case, actual89/1 RED, conservative member guard and actual90/0 GREEN. The earlier source outcomes above remain historical rather than being rewritten.
- Unknown grammar and lexical recovery remain incomplete; call-stack exhaustion cannot become complete. No new runtime compiler dependency, candidate execution or fact cache was introduced.
- No Cargo/rustc, global traversal, nested repository access, cleanup, shared Git mutation or launch changes occurred.

## Final Stage B: Conservative Diagnostic Classification

Root mounted a separate canonical valid-but-unsupported fixture before correction. The exact TypeScript type-predicate source had no compiler parse diagnostics but received owned parse-error[42,44) at `is`. Root's actual canonical RED retained87PASS/1FAIL across88tests. The narrow repair does not parse or expand predicates: unsupported subset tails now use the existing incomplete recovery diagnostic, while an explicitly absent required type token retains parse-error. This also preserves the original zero-width missing-type semicolon law verbatim.

The final source is released for parent review at D8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae. Actual canonical88PASS/0FAIL/1054assertions/948ms, shared57 actual28/28 plus compiler28/28, and exact malformed12/12 all passed with stable captured inputs. No further D write is planned by this owner. This is acceptance of these finite source laws, not all TypeScript syntax or a declaration/mutation census.

Final source capture still reproduces outside-region807e at this observation. This is an endpoint fact, not a hold or restriction on the announced independent taxonomy edit. The final new-region hash remains the independent identity for this packet.

```json
{
  "beforeDiagnosticRepair": {
    "source": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "708754",
      "modifiedNs": "1787878652832003606",
      "changedNs": "1787878652832003606",
      "sha256": "3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520",
      "bytes": 708754
    },
    "region": "5d70f9f6e8f422781d4daeea54dba95bac36b6e9650d68719cded6ef20ffcc09",
    "outside": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423"
  },
  "final": {
    "source": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "708893",
      "modifiedNs": "1787879272425427366",
      "changedNs": "1787879272425427366",
      "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
      "bytes": 708893
    },
    "regionSha256": "de35f121e6e4a210d4685e2feddebc46f07f19ec78b6798fe47be56d92dd1cd4",
    "regionBytes": 53116,
    "firstLine": 6434,
    "lastLine": 7218,
    "outsideRegionSha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423",
    "outsideRegionBytes": 655775
  }
}
```

### Complete Canonical88 Receipt

```json
{
  "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test \"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts\"",
  "before": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708893",
        "modifiedNs": "1787879272425427366",
        "changedNs": "1787879272425427366",
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "9582",
        "modifiedNs": "1787879190548509491",
        "changedNs": "1787879190548509491",
        "sha256": "655a4b7bab22dc5d4dc01b0b47a356df912a50dfb4d4ba7dd3724337ee627c51",
        "bytes": 9582
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926110",
        "mode": "33188",
        "size": "16781",
        "modifiedNs": "1787878027869531198",
        "changedNs": "1787878027869531198",
        "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
        "bytes": 16781
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926112",
        "mode": "33188",
        "size": "3757",
        "modifiedNs": "1787878027869831199",
        "changedNs": "1787878027869831199",
        "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
        "bytes": 3757
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926114",
        "mode": "33188",
        "size": "15589",
        "modifiedNs": "1787878489895441197",
        "changedNs": "1787878489895441197",
        "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
        "bytes": 15589
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928884",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878489703416097",
        "changedNs": "1787878489703416097",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928886",
        "mode": "33188",
        "size": "1564",
        "modifiedNs": "1787878489854494334",
        "changedNs": "1787878489854494334",
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931942",
        "mode": "33188",
        "size": "367",
        "modifiedNs": "1787879190548173114",
        "changedNs": "1787879190548173114",
        "sha256": "9530fbe461f9b19c4a71c508dca0182ddb8e630618b3ebead39139f2827145a4",
        "bytes": 367
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931941",
        "mode": "33188",
        "size": "1125",
        "modifiedNs": "1787879190547954614",
        "changedNs": "1787879190547954614",
        "sha256": "4ffca85ee710fddbd7d6b7de321840f30705e697fe963f5db4c27d77df5da98f",
        "bytes": 1125
      }
    }
  ],
  "after": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708893",
        "modifiedNs": "1787879272425427366",
        "changedNs": "1787879272425427366",
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "9582",
        "modifiedNs": "1787879190548509491",
        "changedNs": "1787879190548509491",
        "sha256": "655a4b7bab22dc5d4dc01b0b47a356df912a50dfb4d4ba7dd3724337ee627c51",
        "bytes": 9582
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926110",
        "mode": "33188",
        "size": "16781",
        "modifiedNs": "1787878027869531198",
        "changedNs": "1787878027869531198",
        "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
        "bytes": 16781
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926112",
        "mode": "33188",
        "size": "3757",
        "modifiedNs": "1787878027869831199",
        "changedNs": "1787878027869831199",
        "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
        "bytes": 3757
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134926114",
        "mode": "33188",
        "size": "15589",
        "modifiedNs": "1787878489895441197",
        "changedNs": "1787878489895441197",
        "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
        "bytes": 15589
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928884",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878489703416097",
        "changedNs": "1787878489703416097",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️malformed/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134928886",
        "mode": "33188",
        "size": "1564",
        "modifiedNs": "1787878489854494334",
        "changedNs": "1787878489854494334",
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931942",
        "mode": "33188",
        "size": "367",
        "modifiedNs": "1787879190548173114",
        "changedNs": "1787879190548173114",
        "sha256": "9530fbe461f9b19c4a71c508dca0182ddb8e630618b3ebead39139f2827145a4",
        "bytes": 367
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931941",
        "mode": "33188",
        "size": "1125",
        "modifiedNs": "1787879190547954614",
        "changedNs": "1787879190547954614",
        "sha256": "4ffca85ee710fddbd7d6b7de321840f30705e697fe963f5db4c27d77df5da98f",
        "bytes": 1125
      }
    }
  ],
  "stable": true,
  "terminal": {
    "chunk_id": "e28785",
    "wall_time_seconds": 4.347907209,
    "exit_code": 0,
    "original_token_count": 1807,
    "output": "bun test v1.3.14 (0d9b296a)\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts:\n(pass) TypeScript declaration facts use the closed neutral schema [5.20ms]\n(pass) TypeScript declaration reference: comment-string-template [5.43ms]\n(pass) TypeScript declaration subject: comment-string-template [2.18ms]\n(pass) TypeScript declaration reference: local-same-name-mutation [0.25ms]\n(pass) TypeScript declaration subject: local-same-name-mutation [0.09ms]\n(pass) TypeScript declaration reference: imported-type-alias [0.98ms]\n(pass) TypeScript declaration subject: imported-type-alias [0.35ms]\n(pass) TypeScript declaration reference: literal-discriminated-union [0.55ms]\n(pass) TypeScript declaration subject: literal-discriminated-union [0.22ms]\n(pass) TypeScript declaration reference: exported-object-metadata [0.66ms]\n(pass) TypeScript declaration subject: exported-object-metadata [0.29ms]\n(pass) TypeScript declaration reference: nested-namespace-interface [0.29ms]\n(pass) TypeScript declaration subject: nested-namespace-interface [0.07ms]\n(pass) TypeScript declaration reference: type-only-reexport [0.26ms]\n(pass) TypeScript declaration subject: type-only-reexport [0.05ms]\n(pass) TypeScript declaration reference: conditional-mapped-computed [1.02ms]\n(pass) TypeScript declaration subject: conditional-mapped-computed [0.32ms]\n(pass) TypeScript declaration reference: enum-class-declarations [0.88ms]\n(pass) TypeScript declaration subject: enum-class-declarations [0.12ms]\n(pass) TypeScript declaration reference: unsupported-module-regions [0.60ms]\n(pass) TypeScript declaration subject: unsupported-module-regions [0.12ms]\n(pass) TypeScript declaration reference: regex-template-asi [0.60ms]\n(pass) TypeScript declaration subject: regex-template-asi [0.56ms]\n(pass) TypeScript declaration reference: escaped-identifier [0.23ms]\n(pass) TypeScript declaration subject: escaped-identifier [0.12ms]\n(pass) TypeScript declaration reference: tsx-jsx-expression [0.62ms]\n(pass) TypeScript declaration subject: tsx-jsx-expression [0.48ms]\n(pass) TypeScript declaration reference: malformed-parse [0.36ms]\n(pass) TypeScript declaration subject: malformed-parse [0.03ms]\n(pass) TypeScript declaration reference: valid-empty-source [0.03ms]\n(pass) TypeScript declaration subject: valid-empty-source\n(pass) TypeScript declaration reference: valid-comment-only-source [0.07ms]\n(pass) TypeScript declaration subject: valid-comment-only-source [0.01ms]\n(pass) TypeScript declaration reference: mixed-default-named-import [0.11ms]\n(pass) TypeScript declaration subject: mixed-default-named-import [0.05ms]\n(pass) TypeScript declaration reference: object-spread [0.19ms]\n(pass) TypeScript declaration subject: object-spread [0.05ms]\n(pass) TypeScript declaration reference: heritage-and-class-body [0.54ms]\n(pass) TypeScript declaration subject: heritage-and-class-body [0.12ms]\n(pass) TypeScript declaration reference: computed-type-literal [0.19ms]\n(pass) TypeScript declaration subject: computed-type-literal [0.11ms]\n(pass) TypeScript declaration reference: union-conditional-mapped-members [0.39ms]\n(pass) TypeScript declaration subject: union-conditional-mapped-members [0.19ms]\n(pass) TypeScript declaration reference: unsupported-primitive-type [0.07ms]\n(pass) TypeScript declaration subject: unsupported-primitive-type [0.03ms]\n(pass) TypeScript declaration reference: bodyless-ambient-module [0.13ms]\n(pass) TypeScript declaration subject: bodyless-ambient-module [0.03ms]\n(pass) TypeScript declaration reference: nested-template-regex-division-asi-comments [0.65ms]\n(pass) TypeScript declaration subject: nested-template-regex-division-asi-comments [0.13ms]\n(pass) TypeScript declaration reference: unicode-dotted-namespace [0.23ms]\n(pass) TypeScript declaration subject: unicode-dotted-namespace [0.07ms]\n(pass) TypeScript declaration reference: generic-conditional-argument [0.13ms]\n(pass) TypeScript declaration subject: generic-conditional-argument [0.12ms]\n(pass) TypeScript declaration reference: property-mapped-type [0.10ms]\n(pass) TypeScript declaration subject: property-mapped-type [0.08ms]\n(pass) TypeScript declaration reference: constructor-accessor-static-bodies [1.30ms]\n(pass) TypeScript declaration subject: constructor-accessor-static-bodies [0.12ms]\n(pass) TypeScript declaration inspector rejects an unspecified or unsupported language [0.09ms]\n(pass) TypeScript declaration grammar has strict standalone source types [400.69ms]\n(pass) TypeScript declaration compiler oracle has strict source types [230.38ms]\n(pass) TypeScript malformed declaration cases use the closed neutral schema [1.12ms]\n(pass) TypeScript malformed declaration reference: malformed-const-header [0.27ms]\n(pass) TypeScript malformed declaration subject: malformed-const-header [0.18ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member [0.75ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-object-member [0.05ms]\n(pass) TypeScript malformed declaration subject: malformed-object-member [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-alias-generic-default [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-alias-generic-default [0.07ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-generic-default [0.02ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-generic-default [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-class-generic-default [0.06ms]\n(pass) TypeScript malformed declaration subject: malformed-class-generic-default [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-generic-constraint [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-generic-constraint [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member-separator [0.04ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member-separator [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-enum-initializer [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-enum-initializer [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-nested-const-header [0.04ms]\n(pass) TypeScript malformed declaration subject: malformed-nested-const-header [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-property-type [0.02ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-property-type [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-class-parameter-type [0.04ms]\n(pass) TypeScript malformed declaration subject: malformed-class-parameter-type [0.03ms]\n(pass) TypeScript unsupported declaration cases use the closed neutral schema [0.31ms]\n(pass) TypeScript unsupported declaration reference: valid-type-predicate-is-not-a-proven-syntax-error [0.03ms]\n(pass) TypeScript unsupported declaration subject: valid-type-predicate-is-not-a-proven-syntax-error [0.10ms]\n\n 88 pass\n 0 fail\n 1054 expect() calls\nRan 88 tests across 1 file. [948.00ms]\n"
  }
}
```

### Final Shared Controller Receipts

```json
[
  {
    "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧫️run-vi8oAC/🔣️result.json",
    "fingerprint": {
      "device": "16777230",
      "inode": "134932654",
      "mode": "33188",
      "size": "31552",
      "modifiedNs": "1787879319307249945",
      "changedNs": "1787879319307249945",
      "sha256": "f00d15e9056bb9000280fcb250ee30b3f4ec74d6effa1e496f027aa8545165c2",
      "bytes": 31552
    },
    "mode": "subject",
    "oracle": {
      "typescript": "5.9.3",
      "passed": 28,
      "total": 28
    },
    "sourceSubject": {
      "mode": "subject",
      "status": "passed",
      "passed": 28,
      "total": 28
    },
    "before": {
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json": {
        "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
        "bytes": 3757
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json": {
        "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
        "bytes": 16781
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts": {
        "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
        "bytes": 15589
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts": {
        "sha256": "3ae174f1c9bb50b8dce421ac9e604e0a4735d564b39cfb07e180b8f18e650e7f",
        "bytes": 9911
      }
    },
    "after": {
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧬️schema/🔣️.json": {
        "sha256": "22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774",
        "bytes": 3757
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🔣️.json": {
        "sha256": "d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48",
        "bytes": 16781
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts": {
        "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa",
        "bytes": 15589
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts": {
        "sha256": "3ae174f1c9bb50b8dce421ac9e604e0a4735d564b39cfb07e180b8f18e650e7f",
        "bytes": 9911
      }
    },
    "stable": true
  },
  {
    "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🧫️runs/86cd0491-d5e1-4098-8c36-03964af913f9/🔣️result.json",
    "fingerprint": {
      "device": "16777230",
      "inode": "134932657",
      "mode": "33188",
      "size": "6115",
      "modifiedNs": "1787879319443612460",
      "changedNs": "1787879319443612460",
      "sha256": "1d5d8432bcf847873c5ff92d1ef6b633af8e568759b769eb5585de77dd22d22b",
      "bytes": 6115
    },
    "mode": "check",
    "results": [
      {
        "id": "malformed-const-header",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 15,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-member",
        "compilerDiagnostics": [
          {
            "code": 1131,
            "start": 25,
            "length": 5
          },
          {
            "code": 1128,
            "start": 38,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-object-member",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 31,
            "length": 5
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-alias-generic-default",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 20,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-generic-default",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 25,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-class-generic-default",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 21,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-generic-constraint",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 26,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-member-separator",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 31,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-enum-initializer",
        "compilerDiagnostics": [
          {
            "code": 1109,
            "start": 19,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-nested-const-header",
        "compilerDiagnostics": [
          {
            "code": 1005,
            "start": 36,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-interface-property-type",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 33,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      },
      {
        "id": "malformed-class-parameter-type",
        "compilerDiagnostics": [
          {
            "code": 1110,
            "start": 31,
            "length": 1
          }
        ],
        "completeness": "incomplete",
        "inferredDeclarationNames": [],
        "diagnosticCount": 1,
        "passed": true
      }
    ],
    "before": {
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🧬️schema/🔣️.json": {
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json": {
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/📜️script.ts": {
        "sha256": "13dea255920fe7883bb10e2968b67fed6edb7680c71eb60a321a7c4192ebbf79",
        "bytes": 7716
      }
    },
    "after": {
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🧬️schema/🔣️.json": {
        "sha256": "2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc",
        "bytes": 1564
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json": {
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      },
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts": {
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      },
      ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/📜️script.ts": {
        "sha256": "13dea255920fe7883bb10e2968b67fed6edb7680c71eb60a321a7c4192ebbf79",
        "bytes": 7716
      }
    },
    "stable": true
  }
]
```

### Final Actual Owned Diagnostics and Predicate Outcome

These full returned facts are actual read-only invocations against the unchanged authored inputs, not new expected outputs. The valid predicate receives unsupported-recovery-suffix[42,52), never parse-error; its compiler parse diagnostics remain empty. The six explicit missing-type cases retain parse-error. All12 malformed sources remain incomplete with no recovered declaration or alias facts. The original28 exact-output suite, including the original malformed type alias, independently remains GREEN.

```json
{
  "before": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708893",
        "modifiedNs": "1787879272425427366",
        "changedNs": "1787879272425427366",
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134927662",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878305450757306",
        "changedNs": "1787878305450757306",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    }
  ],
  "after": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "708893",
        "modifiedNs": "1787879272425427366",
        "changedNs": "1787879272425427366",
        "sha256": "8572639aa8642ac37fd47135d38c37d87be91280b964068cbda1f99caf57c5ae",
        "bytes": 708893
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/🔣️vectors.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134927662",
        "mode": "33188",
        "size": "3658",
        "modifiedNs": "1787878305450757306",
        "changedNs": "1787878305450757306",
        "sha256": "d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462",
        "bytes": 3658
      }
    }
  ],
  "stable": true,
  "actual": [
    {
      "id": "malformed-const-header",
      "source": "export const x y = {};",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unsupported-recovery-suffix",
            "span": {
              "start": 15,
              "end": 22
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-member",
      "source": "export interface Shape { value string }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unsupported-recovery-suffix",
            "span": {
              "start": 31,
              "end": 39
            }
          }
        ]
      }
    },
    {
      "id": "malformed-object-member",
      "source": "export const metadata = { kind value };",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unsupported-recovery-suffix",
            "span": {
              "start": 31,
              "end": 39
            }
          }
        ]
      }
    },
    {
      "id": "malformed-alias-generic-default",
      "source": "export type Box<T = > = T;",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 20,
              "end": 21
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-generic-default",
      "source": "export interface Box<T = > { value: T }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 25,
              "end": 26
            }
          }
        ]
      }
    },
    {
      "id": "malformed-class-generic-default",
      "source": "export class Box<T = > { value!: T; }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 21,
              "end": 22
            }
          }
        ]
      }
    },
    {
      "id": "malformed-generic-constraint",
      "source": "export type Box<T extends = string> = T;",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 26,
              "end": 27
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-member-separator",
      "source": "export interface I { x: string y: number }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unsupported-recovery-suffix",
            "span": {
              "start": 31,
              "end": 42
            }
          }
        ]
      }
    },
    {
      "id": "malformed-enum-initializer",
      "source": "export enum E { A =, B }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unsupported-recovery-suffix",
            "span": {
              "start": 19,
              "end": 24
            }
          }
        ]
      }
    },
    {
      "id": "malformed-nested-const-header",
      "source": "export namespace N { export const x y = {}; }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unsupported-recovery-suffix",
            "span": {
              "start": 36,
              "end": 45
            }
          }
        ]
      }
    },
    {
      "id": "malformed-interface-property-type",
      "source": "export interface I { readonly x: ; }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 33,
              "end": 34
            }
          }
        ]
      }
    },
    {
      "id": "malformed-class-parameter-type",
      "source": "export class C { method(value: ) {} }",
      "facts": {
        "completeness": "incomplete",
        "declarations": [],
        "aliases": [],
        "diagnostics": [
          {
            "code": "parse-error",
            "span": {
              "start": 31,
              "end": 32
            }
          }
        ]
      }
    }
  ],
  "predicate": {
    "source": "export type Predicate = (x: unknown) => x is string;",
    "facts": {
      "completeness": "incomplete",
      "declarations": [],
      "aliases": [],
      "diagnostics": [
        {
          "code": "unsupported-recovery-suffix",
          "span": {
            "start": 42,
            "end": 52
          }
        }
      ]
    },
    "compilerParseDiagnostics": []
  }
}
```
