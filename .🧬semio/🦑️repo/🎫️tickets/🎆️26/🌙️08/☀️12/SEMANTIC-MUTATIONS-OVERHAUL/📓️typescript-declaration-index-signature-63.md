# TypeScript Index Signature Diagnostic Boundary 63

## Scope and Schema-First RED

Only the canonical unsupported sibling schema/vector and D's owned TypeScriptDeclarationFacts member boundary are in scope. The predicate case is preserved verbatim; original28 and malformed12 data/tests are unchanged. No index-signature grammar or new diagnostic/API is introduced. The authored second valid-but-unsupported source is `export interface Shape { [key: string]: number }`; it requires compiler parse diagnostics[], owned incomplete output, and forbids both parse-error and unresolved-computed-property.

The unsupported schema now owns a nonempty unique closed-enum forbidden-code array. Strict Ajv independently accepts both actual cases and rejects unknown/duplicate/empty code arrays. The independent TypeScript parser identifies an IndexSignature, not a computed property.

Actual canonical desired RED: Nx1,89PASS/1FAIL/1070assertions across90tests in984ms. The sole failure is the prohibited unresolved-computed-property result. All9 source/asset captures are stable. Whole D is3839040a… with previously announced peer changes outside our region; our pre-repair region remainsde35f121… unchanged. No unrelated source is restored or overwritten.

```json
{
  "subjectAndReference": {
    "results": [
      {
        "id": "actual-two-case-vectors",
        "passed": true
      },
      {
        "id": "unknown-diagnostic",
        "passed": true
      },
      {
        "id": "duplicate-diagnostic",
        "passed": true
      },
      {
        "id": "empty-diagnostics",
        "passed": true
      },
      {
        "id": "original-predicate-preserved",
        "passed": true
      }
    ],
    "source": "export interface Shape { [key: string]: number }",
    "compilerParseDiagnostics": [],
    "compilerMemberKind": "IndexSignature",
    "compilerIsIndexSignature": true,
    "facts": {
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
    "before": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "710999",
      "modifiedNs": "1787879421607893873",
      "changedNs": "1787879421607893873",
      "sha256": "3839040a84d446af409bfecc5d29bbdaaa690397d6a17fc27da02d9abd2eeaab",
      "bytes": 710999
    },
    "after": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "710999",
      "modifiedNs": "1787879421607893873",
      "changedNs": "1787879421607893873",
      "sha256": "3839040a84d446af409bfecc5d29bbdaaa690397d6a17fc27da02d9abd2eeaab",
      "bytes": 710999
    },
    "stable": true
  },
  "before": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "710999",
        "modifiedNs": "1787879421607893873",
        "changedNs": "1787879421607893873",
        "sha256": "3839040a84d446af409bfecc5d29bbdaaa690397d6a17fc27da02d9abd2eeaab",
        "bytes": 710999
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "9579",
        "modifiedNs": "1787879347724724320",
        "changedNs": "1787879347724724320",
        "sha256": "aefd06016fb1a3d28071f9d85ea786b30cc2f4fc33cabe6768d4ce85b4357fcf",
        "bytes": 9579
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
        "size": "719",
        "modifiedNs": "1787879564266850819",
        "changedNs": "1787879564266850819",
        "sha256": "a530c6260921657b35cec95c3bd0be818e049173c101aefd1d0cfebe5d243188",
        "bytes": 719
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931941",
        "mode": "33188",
        "size": "1300",
        "modifiedNs": "1787879564266461234",
        "changedNs": "1787879564266461234",
        "sha256": "fa36e55eaa1441897fb167dd8eb603958cfe054899e6db02e6ed824e58456acf",
        "bytes": 1300
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
        "size": "710999",
        "modifiedNs": "1787879421607893873",
        "changedNs": "1787879421607893873",
        "sha256": "3839040a84d446af409bfecc5d29bbdaaa690397d6a17fc27da02d9abd2eeaab",
        "bytes": 710999
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "9579",
        "modifiedNs": "1787879347724724320",
        "changedNs": "1787879347724724320",
        "sha256": "aefd06016fb1a3d28071f9d85ea786b30cc2f4fc33cabe6768d4ce85b4357fcf",
        "bytes": 9579
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
        "size": "719",
        "modifiedNs": "1787879564266850819",
        "changedNs": "1787879564266850819",
        "sha256": "a530c6260921657b35cec95c3bd0be818e049173c101aefd1d0cfebe5d243188",
        "bytes": 719
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931941",
        "mode": "33188",
        "size": "1300",
        "modifiedNs": "1787879564266461234",
        "changedNs": "1787879564266461234",
        "sha256": "fa36e55eaa1441897fb167dd8eb603958cfe054899e6db02e6ed824e58456acf",
        "bytes": 1300
      }
    }
  ],
  "stable": true,
  "terminal": {
    "chunk_id": "be73eb",
    "wall_time_seconds": 7.813687583,
    "exit_code": 1,
    "original_token_count": 2435,
    "output": "bun test v1.3.14 (0d9b296a)\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts:\n(pass) TypeScript declaration facts use the closed neutral schema [7.14ms]\n(pass) TypeScript declaration reference: comment-string-template [6.24ms]\n(pass) TypeScript declaration subject: comment-string-template [2.46ms]\n(pass) TypeScript declaration reference: local-same-name-mutation [0.29ms]\n(pass) TypeScript declaration subject: local-same-name-mutation [0.09ms]\n(pass) TypeScript declaration reference: imported-type-alias [0.95ms]\n(pass) TypeScript declaration subject: imported-type-alias [0.21ms]\n(pass) TypeScript declaration reference: literal-discriminated-union [0.39ms]\n(pass) TypeScript declaration subject: literal-discriminated-union [0.45ms]\n(pass) TypeScript declaration reference: exported-object-metadata [0.74ms]\n(pass) TypeScript declaration subject: exported-object-metadata [0.29ms]\n(pass) TypeScript declaration reference: nested-namespace-interface [0.32ms]\n(pass) TypeScript declaration subject: nested-namespace-interface [0.06ms]\n(pass) TypeScript declaration reference: type-only-reexport [0.25ms]\n(pass) TypeScript declaration subject: type-only-reexport [0.05ms]\n(pass) TypeScript declaration reference: conditional-mapped-computed [2.15ms]\n(pass) TypeScript declaration subject: conditional-mapped-computed [0.33ms]\n(pass) TypeScript declaration reference: enum-class-declarations [0.88ms]\n(pass) TypeScript declaration subject: enum-class-declarations [1.85ms]\n(pass) TypeScript declaration reference: unsupported-module-regions [3.10ms]\n(pass) TypeScript declaration subject: unsupported-module-regions [0.33ms]\n(pass) TypeScript declaration reference: regex-template-asi [2.05ms]\n(pass) TypeScript declaration subject: regex-template-asi [0.71ms]\n(pass) TypeScript declaration reference: escaped-identifier [0.71ms]\n(pass) TypeScript declaration subject: escaped-identifier [0.24ms]\n(pass) TypeScript declaration reference: tsx-jsx-expression [1.00ms]\n(pass) TypeScript declaration subject: tsx-jsx-expression [0.51ms]\n(pass) TypeScript declaration reference: malformed-parse [0.57ms]\n(pass) TypeScript declaration subject: malformed-parse [0.04ms]\n(pass) TypeScript declaration reference: valid-empty-source [0.04ms]\n(pass) TypeScript declaration subject: valid-empty-source [0.03ms]\n(pass) TypeScript declaration reference: valid-comment-only-source [0.10ms]\n(pass) TypeScript declaration subject: valid-comment-only-source [0.02ms]\n(pass) TypeScript declaration reference: mixed-default-named-import [0.15ms]\n(pass) TypeScript declaration subject: mixed-default-named-import [0.06ms]\n(pass) TypeScript declaration reference: object-spread [0.36ms]\n(pass) TypeScript declaration subject: object-spread [0.07ms]\n(pass) TypeScript declaration reference: heritage-and-class-body [0.50ms]\n(pass) TypeScript declaration subject: heritage-and-class-body [0.11ms]\n(pass) TypeScript declaration reference: computed-type-literal [0.18ms]\n(pass) TypeScript declaration subject: computed-type-literal [0.07ms]\n(pass) TypeScript declaration reference: union-conditional-mapped-members [0.71ms]\n(pass) TypeScript declaration subject: union-conditional-mapped-members [0.24ms]\n(pass) TypeScript declaration reference: unsupported-primitive-type [0.09ms]\n(pass) TypeScript declaration subject: unsupported-primitive-type [0.03ms]\n(pass) TypeScript declaration reference: bodyless-ambient-module [0.15ms]\n(pass) TypeScript declaration subject: bodyless-ambient-module [0.03ms]\n(pass) TypeScript declaration reference: nested-template-regex-division-asi-comments [0.34ms]\n(pass) TypeScript declaration subject: nested-template-regex-division-asi-comments [0.13ms]\n(pass) TypeScript declaration reference: unicode-dotted-namespace [0.25ms]\n(pass) TypeScript declaration subject: unicode-dotted-namespace [0.07ms]\n(pass) TypeScript declaration reference: generic-conditional-argument [0.16ms]\n(pass) TypeScript declaration subject: generic-conditional-argument [0.10ms]\n(pass) TypeScript declaration reference: property-mapped-type [0.22ms]\n(pass) TypeScript declaration subject: property-mapped-type [0.10ms]\n(pass) TypeScript declaration reference: constructor-accessor-static-bodies [1.21ms]\n(pass) TypeScript declaration subject: constructor-accessor-static-bodies [0.13ms]\n(pass) TypeScript declaration inspector rejects an unspecified or unsupported language [0.10ms]\n(pass) TypeScript declaration grammar has strict standalone source types [434.02ms]\n(pass) TypeScript declaration compiler oracle has strict source types [254.63ms]\n(pass) TypeScript malformed declaration cases use the closed neutral schema [1.12ms]\n(pass) TypeScript malformed declaration reference: malformed-const-header [0.32ms]\n(pass) TypeScript malformed declaration subject: malformed-const-header [0.19ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member [0.87ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member [0.08ms]\n(pass) TypeScript malformed declaration reference: malformed-object-member [0.06ms]\n(pass) TypeScript malformed declaration subject: malformed-object-member [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-alias-generic-default [0.04ms]\n(pass) TypeScript malformed declaration subject: malformed-alias-generic-default [0.08ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-generic-default [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-generic-default [0.05ms]\n(pass) TypeScript malformed declaration reference: malformed-class-generic-default [0.08ms]\n(pass) TypeScript malformed declaration subject: malformed-class-generic-default [0.05ms]\n(pass) TypeScript malformed declaration reference: malformed-generic-constraint [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-generic-constraint [0.05ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member-separator [0.05ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member-separator [0.05ms]\n(pass) TypeScript malformed declaration reference: malformed-enum-initializer [0.02ms]\n(pass) TypeScript malformed declaration subject: malformed-enum-initializer [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-nested-const-header [0.04ms]\n(pass) TypeScript malformed declaration subject: malformed-nested-const-header [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-property-type [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-property-type [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-class-parameter-type [0.04ms]\n(pass) TypeScript malformed declaration subject: malformed-class-parameter-type [0.03ms]\n(pass) TypeScript unsupported declaration cases use the closed neutral schema [0.38ms]\n(pass) TypeScript unsupported declaration reference: valid-type-predicate-is-not-a-proven-syntax-error [0.04ms]\n(pass) TypeScript unsupported declaration subject: valid-type-predicate-is-not-a-proven-syntax-error [0.09ms]\n(pass) TypeScript unsupported declaration reference: valid-index-signature-is-not-a-computed-property [0.03ms]\n147 |     const facts = inspector()(row.source, row.language);\n148 |     expect(validateFacts(facts), JSON.stringify(validateFacts.errors)).toBe(true);\n149 |     coordinateBounds(row.source, facts);\n150 |     expect(facts.completeness).toBe(row.expected.completeness);\n151 |     expect(facts.diagnostics.length).toBeGreaterThan(0);\n152 |     for (const diagnostic of facts.diagnostics) expect(row.expected.forbiddenDiagnosticCodes).not.toContain(diagnostic.code);\n                                                                                                        ^\nerror: expect(received).not.toContain(expected)\n\nExpected to not contain: \"unresolved-computed-property\"\nReceived: [ \"parse-error\", \"unresolved-computed-property\" ]\n\n      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts:152:99)\n(fail) TypeScript unsupported declaration subject: valid-index-signature-is-not-a-computed-property [0.27ms]\n\n1 tests failed:\n(fail) TypeScript unsupported declaration subject: valid-index-signature-is-not-a-computed-property [0.27ms]\n\n 89 pass\n 1 fail\n 1070 expect() calls\nRan 90 tests across 1 file. [984.00ms]\n55 |     }\n56 |     const projects = getProjects(projectGraph, nxArgs);\n57 |     const projectsToRun = (0, get_command_projects_1.getCommandProjects)(projectGraph, projects, nxArgs);\n58 |     projectsToRun.forEach((projectName) => {\n59 |         const command = argv.reduce((cmd, arg) => cmd + `\"${arg}\" `, '').trim();\n60 |         (0, child_process_1.execSync)(command, {\n                                 ^\nerror: Command failed: \"bun\" \"test\" \"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts\"\n signal: null,\n status: 1,\n output: [ null, null, null ],\n    pid: 57623,\n stdout: null,\n stderr: null,\n\n      at genericNodeError (node:child_process:998:13)\n      at checkExecSyncError (node:child_process:458:27)\n      at execSync (node:child_process:278:31)\n      at <anonymous> (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:29)\n      at forEach (1:11)\n      at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)\n      at async <anonymous> (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:75)\n\n"
  }
}
```

The production repair follows this retained desired RED.

## Narrow Repair and Final GREEN

The only D change in this follow-up is a bracketed-member guard in Parser.members. In type/class member position, an identifier immediately followed by a colon inside `[...]` is retained as unsupported recovery, not mislabeled as a computed property. This deliberately does not parse, expand or resolve an index signature. Existing computed-property fixtures remain exact and pass.

The final actual output is incomplete with no declaration/alias facts and unsupported-recovery-suffix[25,48). It contains neither parse-error nor unresolved-computed-property. The original predicate row remains byte-for-byte equal, independently checked using jsonc-parser node offsets; original28/malformed12 full asset hashes are unchanged.

Final canonical90PASS/0FAIL/1065assertions/914ms, all9 source/asset identities stable. The five fewer assertions than the RED arise from the rejected index-signature declaration no longer being emitted, so coordinate checks cover one diagnostic instead of both a declaration and a diagnostic. Strict standalone grammar/oracle checks and original computed-property cases pass.

The canonical90 run itself was stable at whole De07f1839… and711203bytes. A later final-proof read observed whole D663a38f5… and711937bytes, with outside-regionfec86dd7… rather than pre-repairba1e6e0d…. That outside-region endpoint drift is separate from the successful stable test run and was not restored or retested for a desired outcome. Reversing only the exact guard inside the current region reproduces pre-repair regionde35f121…, proving the narrow parser change independently. Reversing it in the later whole D producesa065858c… (not3839040a…) because unrelated outside-region bytes changed after the test. No historical807e outside image is asserted current.

```json
{
  "prepatchSource": {
    "device": "16777230",
    "inode": "109708023",
    "mode": "33188",
    "size": "710999",
    "modifiedNs": "1787879421607893873",
    "changedNs": "1787879421607893873",
    "sha256": "3839040a84d446af409bfecc5d29bbdaaa690397d6a17fc27da02d9abd2eeaab",
    "bytes": 710999
  },
  "prepatchRegionSha256": "de35f121e6e4a210d4685e2feddebc46f07f19ec78b6798fe47be56d92dd1cd4",
  "prepatchOutsideRegionSha256": "ba1e6e0d0110745c9f91c96ff21527c14d27b3f066d6fba286599f3326971911",
  "final": {
    "source": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "711937",
      "modifiedNs": "1787879656882819181",
      "changedNs": "1787879656882819181",
      "sha256": "663a38f56aa750833b1b8acb379b40b6e43dcab45aa91843c3466c80cc4d2c09",
      "bytes": 711937
    },
    "regionSha256": "1d655b87e4947defca6d0a1e84354870b47d5f8655c8f0764b26f60b9c877b11",
    "regionBytes": 53320,
    "firstLine": 6469,
    "lastLine": 7257,
    "outsideRegionSha256": "fec86dd7221001f160e31c8a1925d40193b3ee0878cec00ad0345cf5b1c1ef42",
    "outsideRegionBytes": 658615,
    "exactNarrowPatchOccurrences": 1,
    "reconstructedPrepatchSha256": "a065858cce5555a24864948247a5d9ff9b4c6f04076c5059f927cf613494c2de",
    "reconstructedPrepatchBytes": 711733,
    "predicateRawBytesPreserved": true,
    "predicateRawSha256": "d46331810cf7892b41cb3e26096c20bbada71801229329d374b2a0ed1112acbd",
    "actual": {
      "completeness": "incomplete",
      "declarations": [],
      "aliases": [],
      "diagnostics": [
        {
          "code": "unsupported-recovery-suffix",
          "span": {
            "start": 25,
            "end": 48
          }
        }
      ]
    }
  }
}
```

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
        "size": "711203",
        "modifiedNs": "1787879629575071304",
        "changedNs": "1787879629575071304",
        "sha256": "e07f1839bdf8ba48dcac340dd91abafa183211ff16bd024c70a72e7849507d1c",
        "bytes": 711203
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "9579",
        "modifiedNs": "1787879347724724320",
        "changedNs": "1787879347724724320",
        "sha256": "aefd06016fb1a3d28071f9d85ea786b30cc2f4fc33cabe6768d4ce85b4357fcf",
        "bytes": 9579
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
        "size": "719",
        "modifiedNs": "1787879564266850819",
        "changedNs": "1787879564266850819",
        "sha256": "a530c6260921657b35cec95c3bd0be818e049173c101aefd1d0cfebe5d243188",
        "bytes": 719
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931941",
        "mode": "33188",
        "size": "1300",
        "modifiedNs": "1787879564266461234",
        "changedNs": "1787879564266461234",
        "sha256": "fa36e55eaa1441897fb167dd8eb603958cfe054899e6db02e6ed824e58456acf",
        "bytes": 1300
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
        "size": "711203",
        "modifiedNs": "1787879629575071304",
        "changedNs": "1787879629575071304",
        "sha256": "e07f1839bdf8ba48dcac340dd91abafa183211ff16bd024c70a72e7849507d1c",
        "bytes": 711203
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts",
      "fingerprint": {
        "device": "16777230",
        "inode": "134924386",
        "mode": "33188",
        "size": "9579",
        "modifiedNs": "1787879347724724320",
        "changedNs": "1787879347724724320",
        "sha256": "aefd06016fb1a3d28071f9d85ea786b30cc2f4fc33cabe6768d4ce85b4357fcf",
        "bytes": 9579
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
        "size": "719",
        "modifiedNs": "1787879564266850819",
        "changedNs": "1787879564266850819",
        "sha256": "a530c6260921657b35cec95c3bd0be818e049173c101aefd1d0cfebe5d243188",
        "bytes": 719
      }
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️unsupported/🧬️schema/🔣️.json",
      "fingerprint": {
        "device": "16777230",
        "inode": "134931941",
        "mode": "33188",
        "size": "1300",
        "modifiedNs": "1787879564266461234",
        "changedNs": "1787879564266461234",
        "sha256": "fa36e55eaa1441897fb167dd8eb603958cfe054899e6db02e6ed824e58456acf",
        "bytes": 1300
      }
    }
  ],
  "stable": true,
  "terminal": {
    "chunk_id": "7011a4",
    "wall_time_seconds": 3.6710800839999997,
    "exit_code": 0,
    "original_token_count": 1862,
    "output": "bun test v1.3.14 (0d9b296a)\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🟦️.ts:\n(pass) TypeScript declaration facts use the closed neutral schema [2.30ms]\n(pass) TypeScript declaration reference: comment-string-template [5.57ms]\n(pass) TypeScript declaration subject: comment-string-template [2.16ms]\n(pass) TypeScript declaration reference: local-same-name-mutation [0.26ms]\n(pass) TypeScript declaration subject: local-same-name-mutation [0.09ms]\n(pass) TypeScript declaration reference: imported-type-alias [0.90ms]\n(pass) TypeScript declaration subject: imported-type-alias [0.22ms]\n(pass) TypeScript declaration reference: literal-discriminated-union [0.38ms]\n(pass) TypeScript declaration subject: literal-discriminated-union [0.22ms]\n(pass) TypeScript declaration reference: exported-object-metadata [0.62ms]\n(pass) TypeScript declaration subject: exported-object-metadata [0.26ms]\n(pass) TypeScript declaration reference: nested-namespace-interface [0.35ms]\n(pass) TypeScript declaration subject: nested-namespace-interface [0.06ms]\n(pass) TypeScript declaration reference: type-only-reexport [0.27ms]\n(pass) TypeScript declaration subject: type-only-reexport [0.05ms]\n(pass) TypeScript declaration reference: conditional-mapped-computed [1.09ms]\n(pass) TypeScript declaration subject: conditional-mapped-computed [0.35ms]\n(pass) TypeScript declaration reference: enum-class-declarations [0.86ms]\n(pass) TypeScript declaration subject: enum-class-declarations [0.20ms]\n(pass) TypeScript declaration reference: unsupported-module-regions [0.75ms]\n(pass) TypeScript declaration subject: unsupported-module-regions [0.13ms]\n(pass) TypeScript declaration reference: regex-template-asi [0.61ms]\n(pass) TypeScript declaration subject: regex-template-asi [0.62ms]\n(pass) TypeScript declaration reference: escaped-identifier [0.25ms]\n(pass) TypeScript declaration subject: escaped-identifier [0.13ms]\n(pass) TypeScript declaration reference: tsx-jsx-expression [0.77ms]\n(pass) TypeScript declaration subject: tsx-jsx-expression [0.52ms]\n(pass) TypeScript declaration reference: malformed-parse [0.43ms]\n(pass) TypeScript declaration subject: malformed-parse [0.03ms]\n(pass) TypeScript declaration reference: valid-empty-source [0.04ms]\n(pass) TypeScript declaration subject: valid-empty-source\n(pass) TypeScript declaration reference: valid-comment-only-source [0.20ms]\n(pass) TypeScript declaration subject: valid-comment-only-source [0.04ms]\n(pass) TypeScript declaration reference: mixed-default-named-import [0.19ms]\n(pass) TypeScript declaration subject: mixed-default-named-import [0.07ms]\n(pass) TypeScript declaration reference: object-spread [0.27ms]\n(pass) TypeScript declaration subject: object-spread [0.06ms]\n(pass) TypeScript declaration reference: heritage-and-class-body [0.35ms]\n(pass) TypeScript declaration subject: heritage-and-class-body [0.10ms]\n(pass) TypeScript declaration reference: computed-type-literal [0.16ms]\n(pass) TypeScript declaration subject: computed-type-literal [0.11ms]\n(pass) TypeScript declaration reference: union-conditional-mapped-members [0.24ms]\n(pass) TypeScript declaration subject: union-conditional-mapped-members [0.16ms]\n(pass) TypeScript declaration reference: unsupported-primitive-type [0.06ms]\n(pass) TypeScript declaration subject: unsupported-primitive-type [0.05ms]\n(pass) TypeScript declaration reference: bodyless-ambient-module [0.11ms]\n(pass) TypeScript declaration subject: bodyless-ambient-module [0.02ms]\n(pass) TypeScript declaration reference: nested-template-regex-division-asi-comments [0.26ms]\n(pass) TypeScript declaration subject: nested-template-regex-division-asi-comments [0.09ms]\n(pass) TypeScript declaration reference: unicode-dotted-namespace [0.25ms]\n(pass) TypeScript declaration subject: unicode-dotted-namespace [0.13ms]\n(pass) TypeScript declaration reference: generic-conditional-argument [0.29ms]\n(pass) TypeScript declaration subject: generic-conditional-argument [0.10ms]\n(pass) TypeScript declaration reference: property-mapped-type [0.12ms]\n(pass) TypeScript declaration subject: property-mapped-type [0.06ms]\n(pass) TypeScript declaration reference: constructor-accessor-static-bodies [0.95ms]\n(pass) TypeScript declaration subject: constructor-accessor-static-bodies [0.11ms]\n(pass) TypeScript declaration inspector rejects an unspecified or unsupported language [0.09ms]\n(pass) TypeScript declaration grammar has strict standalone source types [376.67ms]\n(pass) TypeScript declaration compiler oracle has strict source types [300.21ms]\n(pass) TypeScript malformed declaration cases use the closed neutral schema [1.09ms]\n(pass) TypeScript malformed declaration reference: malformed-const-header [0.29ms]\n(pass) TypeScript malformed declaration subject: malformed-const-header [0.19ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member [0.74ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member [0.07ms]\n(pass) TypeScript malformed declaration reference: malformed-object-member [0.05ms]\n(pass) TypeScript malformed declaration subject: malformed-object-member [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-alias-generic-default [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-alias-generic-default [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-generic-default [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-generic-default [0.04ms]\n(pass) TypeScript malformed declaration reference: malformed-class-generic-default [0.06ms]\n(pass) TypeScript malformed declaration subject: malformed-class-generic-default [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-generic-constraint [0.12ms]\n(pass) TypeScript malformed declaration subject: malformed-generic-constraint [0.15ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-member-separator [0.09ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-member-separator [0.06ms]\n(pass) TypeScript malformed declaration reference: malformed-enum-initializer [0.03ms]\n(pass) TypeScript malformed declaration subject: malformed-enum-initializer [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-nested-const-header [0.05ms]\n(pass) TypeScript malformed declaration subject: malformed-nested-const-header [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-interface-property-type [0.02ms]\n(pass) TypeScript malformed declaration subject: malformed-interface-property-type [0.03ms]\n(pass) TypeScript malformed declaration reference: malformed-class-parameter-type [0.04ms]\n(pass) TypeScript malformed declaration subject: malformed-class-parameter-type [0.03ms]\n(pass) TypeScript unsupported declaration cases use the closed neutral schema [0.37ms]\n(pass) TypeScript unsupported declaration reference: valid-type-predicate-is-not-a-proven-syntax-error [0.04ms]\n(pass) TypeScript unsupported declaration subject: valid-type-predicate-is-not-a-proven-syntax-error [0.09ms]\n(pass) TypeScript unsupported declaration reference: valid-index-signature-is-not-a-computed-property [0.02ms]\n(pass) TypeScript unsupported declaration subject: valid-index-signature-is-not-a-computed-property [0.03ms]\n\n 90 pass\n 0 fail\n 1065 expect() calls\nRan 90 tests across 1 file. [914.00ms]\n"
  }
}
```

## Release

This packet is complete and released for parent review. Production write scope was one D member-grammar guard; canonical write scope was exactly the unsupported sibling schema/vector. No canonical TS test, P/root/N, other D declaration, launch, Cargo/native, Git, cleanup or traversal change was made. No new diagnostic API or broader predicate/index-signature implementation is claimed. The formerly documented index-signature false diagnostic is now closed by an actual schema-first negative/positive regression, while broader syntax/census/provider limits remain unchanged.

## Post-Run Region-Only Proof

The additional capture below binds the narrow parser region independently of the later outside-region drift. No subsequent production source write or test rerun was made by this owner.

```json
{
  "source": {
    "device": "16777230",
    "inode": "109708023",
    "mode": "33188",
    "size": "711937",
    "modifiedNs": "1787879656882819181",
    "changedNs": "1787879656882819181",
    "sha256": "663a38f56aa750833b1b8acb379b40b6e43dcab45aa91843c3466c80cc4d2c09",
    "bytes": 711937
  },
  "regionSha256": "1d655b87e4947defca6d0a1e84354870b47d5f8655c8f0764b26f60b9c877b11",
  "reconstructedPrepatchRegionSha256": "de35f121e6e4a210d4685e2feddebc46f07f19ec78b6798fe47be56d92dd1cd4",
  "outsideRegionSha256": "fec86dd7221001f160e31c8a1925d40193b3ee0878cec00ad0345cf5b1c1ef42"
}
```
