# Mutation Source File-Facts Mount 64

## Implemented Boundary

The root now exports the pure owned `MutationTaxonomySourceFileFact` and `mutationTaxonomySourceFileFacts`. The function composes the existing complete-admission regular-file selector with the existing taxonomy suffix classifier, retaining every admitted regular non-gitlink row, raw spelling, UTF-8 byte order, and explicit null kind/role for unknown or tied suffixes. The taxonomy argument is mandatory. It does not load taxonomy, traverse source paths, parse declarations, infer mutation ownership, or widen SourceIndex yet.

The neutral canonical fixture retains 40 cases, covering all26 current source extension chains plus full role vocabulary, raw NFD, executable files, nonregular and gitlink exclusion, unknown JSX and a synthetic ambiguous suffix. A real pure-projector Compose rejection is tested before poisoned observations access; no physical Compose path is read. Test-only minimatch is the independent suffix reference and Ajv validates the closed schema. The pure oracle is typechecked with exact zero diagnostics. The earlier seven-error filtering test was rejected and removed, not treated as strict success.

## Actual Named RED And GREEN

The new Nx target was first executed against unchanged root SHA5eb9cbfff… with the corrected test/oracle. It exited1 with5reference passes and exactly4 missing-export failures,47assertions,2.37s. This RED is export absence, not a behavior mismatch. The full colored output remains in the tool event; the following extract records its observed terminal result without claiming a separately saved raw transcript:

```text
error: missing mutationTaxonomySourceFileFacts export
(fail) mutation source-file facts subject rejects projected opaque Compose before poisoned observations
(fail) mutation source-file facts subject matches reference: current
(fail) mutation source-file facts subject matches reference: synthetic-generated
(fail) mutation source-file facts subject matches reference: synthetic-tie
 5 pass
 4 fail
 47 expect() calls
Ran 9 tests across 1 file. [2.37s]
NX Running target test-mutation-source-file-facts for project @semio-tech/repo-lib failed
```

After the narrow root mount, the same named target passed9/9,53assertions,1.77s; all8selected source endpoint tuples were identical. The reference routing then actually passed5/5 with4subject cases filtered,47assertions,1.72s; all13selected source/test endpoint tuples were identical. These are finite fixture results, not real admission, whole-package, provider identity, all-language parser, native, or exhaustive mutation readiness.

Final rerun after the explicit test-dependency declaration also passed9/9,53assertions,2.10s with13selected tuples unchanged:

```text
bun ./node_modules/nx/bin/nx.js run @semio-tech/repo-lib:test-mutation-source-file-facts --skipNxCache

> nx run @semio-tech/repo-lib:test-mutation-source-file-facts

> bun ./📜️script.ts test mutation-source-file-facts

bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:
✓ mutation source-file facts vectors are closed and cover the registered source chains [33.67ms]
✓ mutation source-file facts reference oracle has strict standalone types [1619.56ms]
✓ mutation source-file facts independent suffix reference: current [17.23ms]
✓ mutation source-file facts independent suffix reference: synthetic-generated [0.50ms]
✓ mutation source-file facts independent suffix reference: synthetic-tie [0.40ms]
✓ mutation source-file facts subject rejects projected opaque Compose before poisoned observations [92.18ms]
✓ mutation source-file facts subject matches reference: current [16.70ms]
✓ mutation source-file facts subject matches reference: synthetic-generated [2.53ms]
✓ mutation source-file facts subject matches reference: synthetic-tie [2.31ms]

 9 pass
 0 fail
 53 expect() calls
Ran 9 tests across 1 file. [2.10s]



 NX   Successfully ran target test-mutation-source-file-facts for project @semio-tech/repo-lib



```

Reference routing output:

```text
bun ./node_modules/nx/bin/nx.js run @semio-tech/repo-lib:test-mutation-source-file-facts --skipNxCache -- reference

> nx run @semio-tech/repo-lib:test-mutation-source-file-facts reference

> bun ./📜️script.ts test mutation-source-file-facts reference

bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:
✓ mutation source-file facts vectors are closed and cover the registered source chains [63.63ms]
✓ mutation source-file facts reference oracle has strict standalone types [1400.83ms]
✓ mutation source-file facts independent suffix reference: current [17.59ms]
✓ mutation source-file facts independent suffix reference: synthetic-generated [0.50ms]
✓ mutation source-file facts independent suffix reference: synthetic-tie [0.40ms]

 5 pass
 4 filtered out
 0 fail
 47 expect() calls
Ran 5 tests across 1 file. [1.72s]



 NX   Successfully ran target test-mutation-source-file-facts for project @semio-tech/repo-lib



```

First post-mount full output (Nx's historical flaky-task label follows the deliberate changed-source RED/GREEN; no intermittent identical-source result is inferred):

```text

> nx run @semio-tech/repo-lib:test-mutation-source-file-facts

> bun ./📜️script.ts test mutation-source-file-facts

bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:
✓ mutation source-file facts vectors are closed and cover the registered source chains [33.93ms]
✓ mutation source-file facts reference oracle has strict standalone types [1361.92ms]
✓ mutation source-file facts independent suffix reference: current [16.70ms]
✓ mutation source-file facts independent suffix reference: synthetic-generated [0.48ms]
✓ mutation source-file facts independent suffix reference: synthetic-tie [0.43ms]
✓ mutation source-file facts subject rejects projected opaque Compose before poisoned observations [133.35ms]
✓ mutation source-file facts subject matches reference: current [20.19ms]
✓ mutation source-file facts subject matches reference: synthetic-generated [2.35ms]
✓ mutation source-file facts subject matches reference: synthetic-tie [2.24ms]

 9 pass
 0 fail
 53 expect() calls
Ran 9 tests across 1 file. [1.77s]



 NX   Successfully ran target test-mutation-source-file-facts for project @semio-tech/repo-lib



 NX   Nx detected a flaky task

  @semio-tech/repo-lib:test-mutation-source-file-facts

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

## Exact Source Preservation

Removing only the authored additions **in memory** reproduces all five exact pre-edit source hashes below. No source was restored or written by this check. S includes only one classifier import and the SourceFileFacts region; P includes the one test route, target, script, test import, and two dev dependency declarations. Existing parser, source admission, SourceIndex, option flow, metadata, and other package test regions are unchanged by this packet.

```json
{
  "sourceWithoutAuthoredAddition": "5eb9cbfff2f505be52eef456cb6c26a310622f0fabff291a5277306c47d779e4",
  "ownedRegionHash": "9af6f057dd5f65c3c0d0dde2c41456b491dfb07f47795ffdf0c4998c2b1db230",
  "packageScriptWithoutAddition": "fcae555a1a3aab5ac29216075803aac8c6feec8b14329bd2483d5412bcdc1b7d",
  "projectWithoutAddition": "cdd9cf079bf900f7539ea65c77a83f5e4a164626b691955b13a72fc65e157331",
  "packageWithoutAdditions": "ee524ae11df31331cb151e024191c0826b15a3ea15c73c92ffa6d0ed57ebcfd1",
  "indexWithoutAddition": "6519c832d011c814ad8ace8e068ea364921847b96bbd006eb1adc297d501fc9d"
}
```

The following thirteen final inputs had equal device/inode/size/mtimeNs/ctimeNs/SHA tuples before and after the final run. Ancestry was checked for symlinks before each read, with file identity checked across the read; this is selected-endpoint stability, not an atomic ancestry lease or whole graph proof.

```json
[
  {
    "path": "/Users/ueli/Documents/semio/📜️script.ts",
    "tuple": [
      "16777230",
      "116368958",
      "2832588",
      "1787880624627463098",
      "1787880624627463098"
    ],
    "sha256": "fdb34f8e4a9d1696915dc18d804876ed80a1f46c6d09d365f92b24914c5a991d"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
    "tuple": [
      "16777230",
      "109708023",
      "712446",
      "1787880280382396863",
      "1787880280382396863"
    ],
    "sha256": "5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts",
    "tuple": [
      "16777230",
      "129693956",
      "909774",
      "1787879247312533114",
      "1787879247312533114"
    ],
    "sha256": "e4942657a1f54b834528da9e66ecddcd06b3e0689e20864dd5aa6d2ea5352001"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
    "tuple": [
      "16777230",
      "129218887",
      "386042",
      "1787871076158192537",
      "1787871076158192537"
    ],
    "sha256": "84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
    "tuple": [
      "16777230",
      "90135993",
      "35172",
      "1787880587658810179",
      "1787880587658810179"
    ],
    "sha256": "8638565c1829c5ef67e0dd770ab698a60f7423d4a342d2ac7230b8bb6e1f173d"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json",
    "tuple": [
      "16777230",
      "90135994",
      "16271",
      "1787880587659218597",
      "1787880587659218597"
    ],
    "sha256": "91949cfd4a5334b208ce42867d11d2a3b03d35008f1d722c05600d5d6fd32ccb"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json",
    "tuple": [
      "16777230",
      "90135385",
      "2387",
      "1787880836031435182",
      "1787880836031435182"
    ],
    "sha256": "6ce50a15cff3662760e8e1b0e2b2a21fe41f20d22f8fe62983b4a3b9859e9b19"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts",
    "tuple": [
      "16777230",
      "109727533",
      "544409",
      "1787880587661117980",
      "1787880587661117980"
    ],
    "sha256": "1ae0bcfe51f4053d33dc0c268635b675e39e6be59c3b1dafb8c820870fbd54f0"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts",
    "tuple": [
      "16777230",
      "134934357",
      "8309",
      "1787880471041259594",
      "1787880471041259594"
    ],
    "sha256": "98e236ebffef51934046b2b58915e0995926a8afc45cb56cbf61c60d7031ce0d"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🧪️oracle/🟦️.ts",
    "tuple": [
      "16777230",
      "134940109",
      "3999",
      "1787880471040796259",
      "1787880471040796259"
    ],
    "sha256": "5f743f901d2c1245bdcc6e4548e46e11c30c50babd0a36e7429771504ba17309"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🔣️.json",
    "tuple": [
      "16777230",
      "134934057",
      "10947",
      "1787879945615049393",
      "1787879945615049393"
    ],
    "sha256": "a27830505bf1a7b55a2b8f21fab684fd14f23a288a373e2a0b6c7d8004d18d73"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🧬️schema/🔣️.json",
    "tuple": [
      "16777230",
      "134934056",
      "1672",
      "1787879945616003188",
      "1787879945616003188"
    ],
    "sha256": "7c35594804d05e0defc7759f203bdf5aacccd70e148b0baebed5c4518ffb1b3a"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-declaration-facts/🧪️oracle/🟦️.ts",
    "tuple": [
      "16777230",
      "134926114",
      "15589",
      "1787878489895441197",
      "1787878489895441197"
    ],
    "sha256": "32a3f4ba0656633012ed584b55f2460eeab35cf078b7ba1d129e27680f82d6fa"
  }
]
```

## Direct Test Dependency Declaration

The oracle initially resolved transitive tooling. The package now explicitly declares test-only Ajv8.20.0 and minimatch9.0.3, exactly the installed and already locked objects. The corresponding existing P workspace dev-dependency tuple was narrowly updated in bun.lock; every other parsed lock tuple, including all package resolution/integrity objects, remains identical. No install, resolution refresh, lifecycle script, or runtime API change was run. A fresh-machine or whole-workspace frozen install has not been verified by this packet.

```json
{
  "before": {
    "lockHash": "59ca8ef14e6fdcb3909e2cba89654659963e49ed5f0ceb6f36bc6b46d1694f82",
    "workspace": {
      "name": "@semio-tech/repo-lib",
      "version": "1.0.0",
      "devDependencies": {
        "@types/node": "^22.19.6",
        "markdown-it": "14.3.0",
        "typescript": "^5.9.3"
      }
    },
    "packageDev": {
      "@types/node": "^22.19.6",
      "markdown-it": "14.3.0",
      "typescript": "^5.9.3"
    },
    "outsideHash": "d37327771479207d489241d276a3b6c7772a8aa7101f92571d96234c55731fa2",
    "packages": {
      "ajv": [
        "ajv@8.20.0",
        "",
        {
          "dependencies": {
            "fast-deep-equal": "^3.1.3",
            "fast-uri": "^3.0.1",
            "json-schema-traverse": "^1.0.0",
            "require-from-string": "^2.0.2"
          }
        },
        "sha512-Thbli+OlOj+iMPYFBVBfJ3OmCAnaSyNn4M1vz9T6Gka5Jt9ba/HIR56joy65tY6kx/FCF5VXNB819Y7/GUrBGA=="
      ],
      "minimatch": [
        "minimatch@9.0.3",
        "",
        {
          "dependencies": {
            "brace-expansion": "^2.0.1"
          }
        },
        "sha512-RHiac9mvaRw0x3AYRgDC1CxAP7HTcNrrECeA8YYJeWnpo+2Q5CegtZjaotWTWxDG3UeGA1coE05iH1mPjT/2mg=="
      ]
    }
  },
  "after": {
    "lockHash": "44d62deb3fa16c06854ae535ea23599258357a7b38cb53a3984f7a29d0707b75",
    "workspace": {
      "name": "@semio-tech/repo-lib",
      "version": "1.0.0",
      "devDependencies": {
        "@types/node": "^22.19.6",
        "ajv": "8.20.0",
        "markdown-it": "14.3.0",
        "minimatch": "9.0.3",
        "typescript": "^5.9.3"
      }
    },
    "packageDev": {
      "@types/node": "^22.19.6",
      "ajv": "8.20.0",
      "markdown-it": "14.3.0",
      "minimatch": "9.0.3",
      "typescript": "^5.9.3"
    },
    "outsideHash": "d37327771479207d489241d276a3b6c7772a8aa7101f92571d96234c55731fa2",
    "packages": {
      "ajv": [
        "ajv@8.20.0",
        "",
        {
          "dependencies": {
            "fast-deep-equal": "^3.1.3",
            "fast-uri": "^3.0.1",
            "json-schema-traverse": "^1.0.0",
            "require-from-string": "^2.0.2"
          }
        },
        "sha512-Thbli+OlOj+iMPYFBVBfJ3OmCAnaSyNn4M1vz9T6Gka5Jt9ba/HIR56joy65tY6kx/FCF5VXNB819Y7/GUrBGA=="
      ],
      "minimatch": [
        "minimatch@9.0.3",
        "",
        {
          "dependencies": {
            "brace-expansion": "^2.0.1"
          }
        },
        "sha512-RHiac9mvaRw0x3AYRgDC1CxAP7HTcNrrECeA8YYJeWnpo+2Q5CegtZjaotWTWxDG3UeGA1coE05iH1mPjT/2mg=="
      ]
    }
  }
}
```

## Next Required Work

SourceIndex still selects content using its old suffix predicate; this new pure classifier has not silently broadened capture. A schema-first source coverage/capture cutover remains required. The output-ticket versus explicit-authored-input conflation also remains open for a separate tested caller change; independently admitted output-path candidates must remain admitted and assignment-ledger identity is separate from ancestor traversal. No full mutation census, direct-leaf completion, or goal completion is claimed.

