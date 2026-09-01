# Sync Demo Source-Structure Verifier Receipt

Newly executed ticket-only source evidence; no Rust/native/provenance execution.

- Mode: reference
- Result: FAIL
- Checks: 21/21
- Run: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧫️run-9jca2c

The repository lexer and any public D facts belong to the same parser family. Ajv and jsonc-parser validate JSON contracts, not Rust semantics. Whole-file drift is separate from scoped owner stability.

Complete receipt:

```json
{
  "version": 1,
  "mode": "reference",
  "status": "FAIL",
  "scope": "source-structure preparation only",
  "root": "/Users/ueli/Documents/semio",
  "run": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧫️run-9jca2c",
  "cancelled": false,
  "failure": "Error: Neutral replacement is not exactly one occurrence.\n    at altered (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/📜️script.ts:512:84)\n    at main (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/📜️script.ts:585:29)",
  "passed": 21,
  "total": 21,
  "neutralCases": 14,
  "expectedNeutralCases": 25,
  "checks": [
    {
      "name": "schema:contract",
      "pass": true,
      "detail": null
    },
    {
      "name": "schema:vectors",
      "pass": true,
      "detail": null
    },
    {
      "name": "schema:missing-contract-field-rejected",
      "pass": true
    },
    {
      "name": "schema:unknown-vector-field-rejected",
      "pass": true
    },
    {
      "name": "jsonc-parser:matches-native-json",
      "pass": true
    },
    {
      "name": "source-case:mounted-scoped-source",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": true,
          "includes": []
        },
        "actual": {
          "accepted": true,
          "failures": [],
          "ownerFingerprint": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:unrelated-prefix-is-not-owner-drift",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": true,
          "includes": [],
          "sameOwnerAs": "mounted-scoped-source"
        },
        "actual": {
          "accepted": true,
          "failures": [],
          "ownerFingerprint": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "scope-invariance:unrelated-prefix-is-not-owner-drift",
      "pass": true,
      "detail": {
        "sameOwnerAs": "mounted-scoped-source",
        "previous": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
        "actual": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d"
      }
    },
    {
      "name": "source-case:unrelated-suffix-is-not-owner-drift",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": true,
          "includes": [],
          "sameOwnerAs": "mounted-scoped-source"
        },
        "actual": {
          "accepted": true,
          "failures": [],
          "ownerFingerprint": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "scope-invariance:unrelated-suffix-is-not-owner-drift",
      "pass": true,
      "detail": {
        "sameOwnerAs": "mounted-scoped-source",
        "previous": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
        "actual": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d"
      }
    },
    {
      "name": "source-case:premount-is-source-red",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "canonical-file-missing",
            "private-mount",
            "retired-concrete-owner",
            "consumer-joins"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "canonical-file-missing",
            "consumer-joins",
            "descriptor-provenance-source",
            "direct-roster",
            "invalid-json-schema",
            "leaf-hooks",
            "private-mount",
            "retired-concrete-owner"
          ],
          "ownerFingerprint": "db42424c39940d201643739526211345e23f725066029d6c9d70e2fbb3bffdf9",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:missing-aggregate",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "canonical-file-missing"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "canonical-file-missing",
            "descriptor-provenance-source",
            "direct-roster"
          ],
          "ownerFingerprint": "1a9f60568ca57753c4f50c9940acc1220d0c41dd21589f7d09b9c9b4f9aab088",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:missing-payload-schema",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "canonical-file-missing"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "canonical-file-missing",
            "invalid-json-schema"
          ],
          "ownerFingerprint": "aac595c2ed74ce40c01403023063e10a8474245cb36ca451b8f41263e46cb8a9",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:wrong-inline-anchor",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "private-mount"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "private-mount"
          ],
          "ownerFingerprint": "9d2d534ba95e3e65a37e5b13145241f9928b99628e5ead788976862d79d64d8a",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:public-tests-owner",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "private-mount"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "private-mount"
          ],
          "ownerFingerprint": "9d2d534ba95e3e65a37e5b13145241f9928b99628e5ead788976862d79d64d8a",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:comment-only-codec",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "unchanged-active-chunks"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "unchanged-active-chunks"
          ],
          "ownerFingerprint": "92c6065886856608dd4d51ca9a65c70f36e7e02bf90ac2181f6ab42590de00d5",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:string-only-codec",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "unchanged-active-chunks"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "unchanged-active-chunks"
          ],
          "ownerFingerprint": "92c6065886856608dd4d51ca9a65c70f36e7e02bf90ac2181f6ab42590de00d5",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:duplicate-active-codec",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "unchanged-active-chunks"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "unchanged-active-chunks"
          ],
          "ownerFingerprint": "92c6065886856608dd4d51ca9a65c70f36e7e02bf90ac2181f6ab42590de00d5",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:intrinsic-visibility-widened",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "unchanged-active-chunks"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "unchanged-active-chunks"
          ],
          "ownerFingerprint": "2cf59e12c3313de33117105d7e32646b4ec5343f218ac515759226ca9dba066f",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:optional-snapshot-regression",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "unchanged-active-chunks"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "unchanged-active-chunks"
          ],
          "ownerFingerprint": "2cf59e12c3313de33117105d7e32646b4ec5343f218ac515759226ca9dba066f",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    },
    {
      "name": "source-case:nested-intrinsic-owner",
      "pass": true,
      "detail": {
        "expected": {
          "accepted": false,
          "includes": [
            "unchanged-active-chunks"
          ]
        },
        "actual": {
          "accepted": false,
          "failures": [
            "unchanged-active-chunks"
          ],
          "ownerFingerprint": "2cf59e12c3313de33117105d7e32646b4ec5343f218ac515759226ca9dba066f",
          "proof": {
            "sourceOnly": true,
            "nativeExecuted": false,
            "rustCompilerExecuted": false,
            "provenanceExecuted": false
          }
        },
        "schemaErrors": null
      }
    }
  ],
  "outputs": [
    {
      "id": "mounted-scoped-source",
      "basis": "mounted",
      "result": {
        "accepted": true,
        "failures": [],
        "ownerFingerprint": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2484
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 173,
                "end": 3234,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3235,
                "end": 4488,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4489,
                "end": 6865,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1538,
                "body": 1542,
                "end": 1577
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1579,
                "body": 1583,
                "end": 1682
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1684,
                "body": 1688,
                "end": 1787
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1789,
                "body": 1793,
                "end": 1898
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1915,
                "body": 1919,
                "end": 2045
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2047,
                "body": 2051,
                "end": 2091
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2093,
                "body": 2097,
                "end": 2135
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2137,
                "body": 2141,
                "end": 2181
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2183,
                "body": 2187,
                "end": 2227
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2230,
                "body": 2234,
                "end": 2313
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2315,
                "body": 2319,
                "end": 2398
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2400,
                "body": 2404,
                "end": 2483
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1548,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1564,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1614,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1663,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1719,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1768,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1813,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1856,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1885,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1933,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1978,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2021,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2065,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2111,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2155,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2201,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2248,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2287,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2333,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2372,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2418,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2457,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "unrelated-prefix-is-not-owner-drift",
      "basis": "mounted",
      "result": {
        "accepted": true,
        "failures": [],
        "ownerFingerprint": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2484
          }
        ],
        "headers": [
          {
            "start": 27,
            "end": 199,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 200,
                "end": 3261,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3262,
                "end": 4515,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4516,
                "end": 6892,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1538,
                "body": 1542,
                "end": 1577
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1579,
                "body": 1583,
                "end": 1682
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1684,
                "body": 1688,
                "end": 1787
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1789,
                "body": 1793,
                "end": 1898
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1915,
                "body": 1919,
                "end": 2045
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2047,
                "body": 2051,
                "end": 2091
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2093,
                "body": 2097,
                "end": 2135
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2137,
                "body": 2141,
                "end": 2181
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2183,
                "body": 2187,
                "end": 2227
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2230,
                "body": 2234,
                "end": 2313
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2315,
                "body": 2319,
                "end": 2398
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2400,
                "body": 2404,
                "end": 2483
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1548,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1564,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1614,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1663,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1719,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1768,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1813,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1856,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1885,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1933,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1978,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2021,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2065,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2111,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2155,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2201,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2248,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2287,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2333,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2372,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2418,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2457,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "unrelated-suffix-is-not-owner-drift",
      "basis": "mounted",
      "result": {
        "accepted": true,
        "failures": [],
        "ownerFingerprint": "399ac791292cfcbeb25e58b5ba41e32d2a3195a00713bc9fd6a12838b5f9128d",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2484
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 173,
                "end": 3234,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3235,
                "end": 4488,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4489,
                "end": 6865,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1538,
                "body": 1542,
                "end": 1577
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1579,
                "body": 1583,
                "end": 1682
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1684,
                "body": 1688,
                "end": 1787
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1789,
                "body": 1793,
                "end": 1898
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1915,
                "body": 1919,
                "end": 2045
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2047,
                "body": 2051,
                "end": 2091
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2093,
                "body": 2097,
                "end": 2135
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2137,
                "body": 2141,
                "end": 2181
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2183,
                "body": 2187,
                "end": 2227
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2230,
                "body": 2234,
                "end": 2313
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2315,
                "body": 2319,
                "end": 2398
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2400,
                "body": 2404,
                "end": 2483
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1548,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1564,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1614,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1663,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1719,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1768,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1813,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1856,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1885,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1933,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1978,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2021,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2065,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2111,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2155,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2201,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2248,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2287,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2333,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2372,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2418,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2457,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "premount-is-source-red",
      "basis": "premount",
      "result": {
        "accepted": false,
        "failures": [
          "canonical-file-missing",
          "consumer-joins",
          "descriptor-provenance-source",
          "direct-roster",
          "invalid-json-schema",
          "leaf-hooks",
          "private-mount",
          "retired-concrete-owner"
        ],
        "ownerFingerprint": "db42424c39940d201643739526211345e23f725066029d6c9d70e2fbb3bffdf9",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": false,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN { n }",
                "DemoMutation::SetN { n: 0 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN { n: 5 }",
                "DemoMutation::SetN { n: 0 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN { n: 6 }",
                "DemoMutation::SetN { n: 5 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN { n: 42 }",
                "DemoMutation::SetN { n: 0 }",
                "DemoMutation::SetN { n: 42 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN { n: 1 }",
                "DemoMutation::SetN { n: 42 }",
                "DemoMutation::SetN { n: 1 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN { n: 7 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN { n }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN { n: 5 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN { n: 1 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN { n: 1 }",
                "DemoMutation::SetN { n: 2 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN { n: 1 }",
                "DemoMutation::SetN { n: 2 }"
              ],
              "valid": false
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN { n: 1 }",
                "DemoMutation::SetN { n: 2 }"
              ],
              "valid": false
            }
          ],
          "constructorCount": 24,
          "unownedConstructorCount": 2,
          "retired": [
            "enum",
            "manual-mutation-impl"
          ],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": null
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": null
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": null
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": null
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": null
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": null
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": null
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": null
            }
          ],
          "direct": false,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": false
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": false
            }
          ],
          "descriptor": null
        },
        "roots": [
          {
            "token": 29966,
            "body": 29968,
            "end": 44912
          }
        ],
        "headers": [],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 187574,
                "end": 190635,
                "parent": 29968
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 190839,
                "end": 192092,
                "parent": 29968
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 192092,
                "end": 194468,
                "parent": 29968
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN { n }",
              "DemoMutation::SetN { n: 0 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 33030,
                "body": 33044,
                "end": 33193
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 5 }",
              "DemoMutation::SetN { n: 0 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 33906,
                "body": 33910,
                "end": 35347
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 6 }",
              "DemoMutation::SetN { n: 5 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 35349,
                "body": 35355,
                "end": 35520
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 42 }",
              "DemoMutation::SetN { n: 0 }",
              "DemoMutation::SetN { n: 42 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 36677,
                "body": 36681,
                "end": 36909
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 1 }",
              "DemoMutation::SetN { n: 42 }",
              "DemoMutation::SetN { n: 1 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 37374,
                "body": 37378,
                "end": 38023
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 7 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 39161,
                "body": 39165,
                "end": 39651
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN { n }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 39659,
                "body": 39663,
                "end": 40182
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 5 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 40190,
                "body": 40194,
                "end": 40642
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 1 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 40650,
                "body": 40654,
                "end": 40960
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 1 }",
              "DemoMutation::SetN { n: 2 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 42743,
                "body": 42747,
                "end": 43134
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 1 }",
              "DemoMutation::SetN { n: 2 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 43154,
                "body": 43158,
                "end": 43611
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN { n: 1 }",
              "DemoMutation::SetN { n: 2 }"
            ],
            "valid": false,
            "ranges": [
              {
                "start": 43631,
                "body": 43635,
                "end": 44342
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 32898,
            "expression": "DemoMutation::SetN { n }"
          },
          {
            "token": 32939,
            "expression": "DemoMutation::SetN { n: snapshot.n }"
          },
          {
            "token": 33067,
            "expression": "DemoMutation::SetN { n }"
          },
          {
            "token": 33080,
            "expression": "DemoMutation::SetN { n: 0 }"
          },
          {
            "token": 34341,
            "expression": "DemoMutation::SetN { n: 5 }"
          },
          {
            "token": 34387,
            "expression": "DemoMutation::SetN { n: 0 }"
          },
          {
            "token": 35436,
            "expression": "DemoMutation::SetN { n: 6 }"
          },
          {
            "token": 35482,
            "expression": "DemoMutation::SetN { n: 5 }"
          },
          {
            "token": 36749,
            "expression": "DemoMutation::SetN { n: 42 }"
          },
          {
            "token": 36789,
            "expression": "DemoMutation::SetN { n: 0 }"
          },
          {
            "token": 36899,
            "expression": "DemoMutation::SetN { n: 42 }"
          },
          {
            "token": 37549,
            "expression": "DemoMutation::SetN { n: 1 }"
          },
          {
            "token": 37754,
            "expression": "DemoMutation::SetN { n: 42 }"
          },
          {
            "token": 37794,
            "expression": "DemoMutation::SetN { n: 1 }"
          },
          {
            "token": 39496,
            "expression": "DemoMutation::SetN { n: 7 }"
          },
          {
            "token": 39857,
            "expression": "DemoMutation::SetN { n }"
          },
          {
            "token": 40519,
            "expression": "DemoMutation::SetN { n: 5 }"
          },
          {
            "token": 40846,
            "expression": "DemoMutation::SetN { n: 1 }"
          },
          {
            "token": 42835,
            "expression": "DemoMutation::SetN { n: 1 }"
          },
          {
            "token": 42885,
            "expression": "DemoMutation::SetN { n: 2 }"
          },
          {
            "token": 43347,
            "expression": "DemoMutation::SetN { n: 1 }"
          },
          {
            "token": 43434,
            "expression": "DemoMutation::SetN { n: 2 }"
          },
          {
            "token": 43844,
            "expression": "DemoMutation::SetN { n: 1 }"
          },
          {
            "token": 43931,
            "expression": "DemoMutation::SetN { n: 2 }"
          }
        ],
        "retired": [
          {
            "kind": "enum",
            "token": 31994
          },
          {
            "kind": "manual-mutation-impl",
            "token": 32853
          }
        ],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": false,
            "occurrences": []
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": false,
            "occurrences": []
          }
        ],
        "schemaResults": null,
        "schemaError": "Error: Missing JSON surface: intrinsicSchema"
      }
    },
    {
      "id": "missing-aggregate",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "canonical-file-missing",
          "descriptor-provenance-source",
          "direct-roster"
        ],
        "ownerFingerprint": "1a9f60568ca57753c4f50c9940acc1220d0c41dd21589f7d09b9c9b4f9aab088",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": null
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": false,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2484
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 173,
                "end": 3234,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3235,
                "end": 4488,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4489,
                "end": 6865,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1538,
                "body": 1542,
                "end": 1577
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1579,
                "body": 1583,
                "end": 1682
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1684,
                "body": 1688,
                "end": 1787
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1789,
                "body": 1793,
                "end": 1898
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1915,
                "body": 1919,
                "end": 2045
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2047,
                "body": 2051,
                "end": 2091
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2093,
                "body": 2097,
                "end": 2135
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2137,
                "body": 2141,
                "end": 2181
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2183,
                "body": 2187,
                "end": 2227
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2230,
                "body": 2234,
                "end": 2313
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2315,
                "body": 2319,
                "end": 2398
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2400,
                "body": 2404,
                "end": 2483
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1548,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1564,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1614,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1663,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1719,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1768,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1813,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1856,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1885,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1933,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1978,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2021,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2065,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2111,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2155,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2201,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2248,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2287,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2333,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2372,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2418,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2457,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "missing-payload-schema",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "canonical-file-missing",
          "invalid-json-schema"
        ],
        "ownerFingerprint": "aac595c2ed74ce40c01403023063e10a8474245cb36ca451b8f41263e46cb8a9",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": null
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": null
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2484
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 173,
                "end": 3234,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3235,
                "end": 4488,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4489,
                "end": 6865,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1538,
                "body": 1542,
                "end": 1577
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1579,
                "body": 1583,
                "end": 1682
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1684,
                "body": 1688,
                "end": 1787
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1789,
                "body": 1793,
                "end": 1898
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1915,
                "body": 1919,
                "end": 2045
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2047,
                "body": 2051,
                "end": 2091
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2093,
                "body": 2097,
                "end": 2135
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2137,
                "body": 2141,
                "end": 2181
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2183,
                "body": 2187,
                "end": 2227
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2230,
                "body": 2234,
                "end": 2313
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2315,
                "body": 2319,
                "end": 2398
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2400,
                "body": 2404,
                "end": 2483
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1548,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1564,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1614,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1663,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1719,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1768,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1813,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1856,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1885,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1933,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1978,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2021,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2065,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2111,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2155,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2201,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2248,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2287,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2333,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2372,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2418,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2457,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": null,
        "schemaError": "Error: Missing JSON surface: payloadSchema"
      }
    },
    {
      "id": "wrong-inline-anchor",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "private-mount"
        ],
        "ownerFingerprint": "9d2d534ba95e3e65a37e5b13145241f9928b99628e5ead788976862d79d64d8a",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": false,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2484
          }
        ],
        "headers": [],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 177,
                "end": 3238,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3239,
                "end": 4492,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4493,
                "end": 6869,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1538,
                "body": 1542,
                "end": 1577
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1579,
                "body": 1583,
                "end": 1682
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1684,
                "body": 1688,
                "end": 1787
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1789,
                "body": 1793,
                "end": 1898
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1915,
                "body": 1919,
                "end": 2045
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2047,
                "body": 2051,
                "end": 2091
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2093,
                "body": 2097,
                "end": 2135
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2137,
                "body": 2141,
                "end": 2181
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2183,
                "body": 2187,
                "end": 2227
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2230,
                "body": 2234,
                "end": 2313
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2315,
                "body": 2319,
                "end": 2398
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2400,
                "body": 2404,
                "end": 2483
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1548,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1564,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1614,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1663,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1719,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1768,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1813,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1856,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1885,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1933,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1978,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2021,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2065,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2111,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2155,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2201,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2248,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2287,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2333,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2372,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2418,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2457,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "public-tests-owner",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "private-mount"
        ],
        "ownerFingerprint": "9d2d534ba95e3e65a37e5b13145241f9928b99628e5ead788976862d79d64d8a",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": false,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 14,
            "body": 16,
            "end": 2485
          }
        ],
        "headers": [],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 177,
                "end": 3238,
                "parent": 16
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3239,
                "end": 4492,
                "parent": 16
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4493,
                "end": 6869,
                "parent": 16
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1539,
                "body": 1543,
                "end": 1578
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1580,
                "body": 1584,
                "end": 1683
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1685,
                "body": 1689,
                "end": 1788
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1790,
                "body": 1794,
                "end": 1899
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1916,
                "body": 1920,
                "end": 2046
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2048,
                "body": 2052,
                "end": 2092
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2094,
                "body": 2098,
                "end": 2136
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2138,
                "body": 2142,
                "end": 2182
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2184,
                "body": 2188,
                "end": 2228
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2231,
                "body": 2235,
                "end": 2314
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2316,
                "body": 2320,
                "end": 2399
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2401,
                "body": 2405,
                "end": 2484
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1549,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1565,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1615,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1664,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1720,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1769,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1814,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1857,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1886,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1934,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1979,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2022,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2066,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2112,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2156,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2202,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2249,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2288,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2334,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2373,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2419,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2458,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "comment-only-codec",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "unchanged-active-chunks"
        ],
        "ownerFingerprint": "92c6065886856608dd4d51ca9a65c70f36e7e02bf90ac2181f6ab42590de00d5",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": false
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 1926
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 173,
                "end": 3234,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3235,
                "end": 4488,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": false,
            "occurrences": []
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 980,
                "body": 984,
                "end": 1019
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1021,
                "body": 1025,
                "end": 1124
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1126,
                "body": 1130,
                "end": 1229
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1231,
                "body": 1235,
                "end": 1340
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1357,
                "body": 1361,
                "end": 1487
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1489,
                "body": 1493,
                "end": 1533
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1535,
                "body": 1539,
                "end": 1577
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1579,
                "body": 1583,
                "end": 1623
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1625,
                "body": 1629,
                "end": 1669
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1672,
                "body": 1676,
                "end": 1755
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1757,
                "body": 1761,
                "end": 1840
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1842,
                "body": 1846,
                "end": 1925
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 990,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1006,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1056,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1105,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1161,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1210,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1255,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1298,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1327,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1375,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1420,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1463,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1507,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 1553,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1597,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1643,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1690,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1729,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 1775,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1814,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 1860,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1899,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "string-only-codec",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "unchanged-active-chunks"
        ],
        "ownerFingerprint": "92c6065886856608dd4d51ca9a65c70f36e7e02bf90ac2181f6ab42590de00d5",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": false
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 1934
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 173,
                "end": 3234,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3235,
                "end": 4488,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": false,
            "occurrences": []
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 988,
                "body": 992,
                "end": 1027
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1029,
                "body": 1033,
                "end": 1132
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1134,
                "body": 1138,
                "end": 1237
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1239,
                "body": 1243,
                "end": 1348
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1365,
                "body": 1369,
                "end": 1495
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1497,
                "body": 1501,
                "end": 1541
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1543,
                "body": 1547,
                "end": 1585
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1587,
                "body": 1591,
                "end": 1631
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1633,
                "body": 1637,
                "end": 1677
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1680,
                "body": 1684,
                "end": 1763
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1765,
                "body": 1769,
                "end": 1848
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1850,
                "body": 1854,
                "end": 1933
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 998,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1014,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1064,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1113,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1169,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1218,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1263,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1306,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1335,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1383,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1428,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1471,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1515,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 1561,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1605,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1651,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1698,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1737,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 1783,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1822,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 1868,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1907,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "duplicate-active-codec",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "unchanged-active-chunks"
        ],
        "ownerFingerprint": "92c6065886856608dd4d51ca9a65c70f36e7e02bf90ac2181f6ab42590de00d5",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": true
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": false
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 3042
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": true,
            "occurrences": [
              {
                "start": 173,
                "end": 3234,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3235,
                "end": 4488,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": false,
            "occurrences": [
              {
                "start": 4489,
                "end": 6865,
                "parent": 15
              },
              {
                "start": 6866,
                "end": 9242,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2096,
                "body": 2100,
                "end": 2135
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2137,
                "body": 2141,
                "end": 2240
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2242,
                "body": 2246,
                "end": 2345
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2347,
                "body": 2351,
                "end": 2456
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2473,
                "body": 2477,
                "end": 2603
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2605,
                "body": 2609,
                "end": 2649
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2651,
                "body": 2655,
                "end": 2693
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2695,
                "body": 2699,
                "end": 2739
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2741,
                "body": 2745,
                "end": 2785
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2788,
                "body": 2792,
                "end": 2871
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2873,
                "body": 2877,
                "end": 2956
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2958,
                "body": 2962,
                "end": 3041
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 2106,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2122,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 2172,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2221,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 2277,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 2326,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2371,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2414,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 2443,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2491,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2536,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2579,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2623,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2669,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2713,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2759,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2806,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2845,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2891,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2930,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2976,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 3015,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "intrinsic-visibility-widened",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "unchanged-active-chunks"
        ],
        "ownerFingerprint": "2cf59e12c3313de33117105d7e32646b4ec5343f218ac515759226ca9dba066f",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": false
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2488
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": false,
            "occurrences": []
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3246,
                "end": 4499,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4500,
                "end": 6876,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1542,
                "body": 1546,
                "end": 1581
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1583,
                "body": 1587,
                "end": 1686
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1688,
                "body": 1692,
                "end": 1791
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1793,
                "body": 1797,
                "end": 1902
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1919,
                "body": 1923,
                "end": 2049
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2051,
                "body": 2055,
                "end": 2095
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2097,
                "body": 2101,
                "end": 2139
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2141,
                "body": 2145,
                "end": 2185
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2187,
                "body": 2191,
                "end": 2231
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2234,
                "body": 2238,
                "end": 2317
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2319,
                "body": 2323,
                "end": 2402
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2404,
                "body": 2408,
                "end": 2487
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1552,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1568,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1618,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1667,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1723,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1772,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1817,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1860,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1889,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1937,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1982,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2025,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2069,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2115,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2159,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2205,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2252,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2291,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2337,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2376,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2422,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2461,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "optional-snapshot-regression",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "unchanged-active-chunks"
        ],
        "ownerFingerprint": "2cf59e12c3313de33117105d7e32646b4ec5343f218ac515759226ca9dba066f",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": false
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2487
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": false,
            "occurrences": []
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3243,
                "end": 4496,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4497,
                "end": 6873,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1541,
                "body": 1545,
                "end": 1580
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1582,
                "body": 1586,
                "end": 1685
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1687,
                "body": 1691,
                "end": 1790
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1792,
                "body": 1796,
                "end": 1901
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1918,
                "body": 1922,
                "end": 2048
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2050,
                "body": 2054,
                "end": 2094
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2096,
                "body": 2100,
                "end": 2138
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2140,
                "body": 2144,
                "end": 2184
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2186,
                "body": 2190,
                "end": 2230
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2233,
                "body": 2237,
                "end": 2316
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2318,
                "body": 2322,
                "end": 2401
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2403,
                "body": 2407,
                "end": 2486
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1551,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1567,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1617,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1666,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1722,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1771,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1816,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1859,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1888,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1936,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1981,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2024,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2068,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2114,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2158,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2204,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2251,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2290,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2336,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2375,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2421,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2460,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    },
    {
      "id": "nested-intrinsic-owner",
      "basis": "mounted",
      "result": {
        "accepted": false,
        "failures": [
          "unchanged-active-chunks"
        ],
        "ownerFingerprint": "2cf59e12c3313de33117105d7e32646b4ec5343f218ac515759226ca9dba066f",
        "proof": {
          "sourceOnly": true,
          "nativeExecuted": false,
          "rustCompilerExecuted": false,
          "provenanceExecuted": false
        }
      },
      "detail": {
        "projection": {
          "mount": true,
          "chunks": [
            {
              "id": "snapshot-and-diff",
              "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
              "valid": false
            },
            {
              "id": "op-text",
              "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
              "valid": true
            },
            {
              "id": "op-binary",
              "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
              "valid": true
            }
          ],
          "consumers": [
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_operation_envelope",
              "observed": [
                "DemoMutation::SetN(SetN { n })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })",
                "DemoMutation::SetN(SetN { n: 0 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "sample_wire_envelope_for_fixtures",
              "observed": [
                "DemoMutation::SetN(SetN { n: 6 })",
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
              "observed": [
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 0 })",
                "DemoMutation::SetN(SetN { n: 42 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "folder_external_edit_delivers_remote_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 42 })",
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "two_hosts_converge_through_hub",
              "observed": [
                "DemoMutation::SetN(SetN { n: 7 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "reconnect_since_catch_up_replays_backlog",
              "observed": [
                "DemoMutation::SetN(SetN { n })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "detach_drains_pending_outbound_operations",
              "observed": [
                "DemoMutation::SetN(SetN { n: 5 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests",
                "actor_tests"
              ],
              "function": "command_outcome_accepted_fires_after_hub_ack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            },
            {
              "modulePath": [
                "tests"
              ],
              "function": "folder_text_storage_round_trips_pack",
              "observed": [
                "DemoMutation::SetN(SetN { n: 1 })",
                "DemoMutation::SetN(SetN { n: 2 })"
              ],
              "valid": true
            }
          ],
          "constructorCount": 22,
          "unownedConstructorCount": 0,
          "retired": [],
          "canonicalFiles": [
            {
              "role": "intrinsicSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️schema/🔣️.json",
              "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05"
            },
            {
              "role": "domainVectors",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🔣️.json",
              "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2"
            },
            {
              "role": "domainSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧪️tests/🧬️schema/🔣️.json",
              "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25"
            },
            {
              "role": "aggregateRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs",
              "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e"
            },
            {
              "role": "aggregateSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔣️.json",
              "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a"
            },
            {
              "role": "leafRust",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs",
              "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4"
            },
            {
              "role": "descriptor",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json",
              "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd"
            },
            {
              "role": "payloadSchema",
              "canonicalPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
              "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1"
            }
          ],
          "direct": true,
          "hooks": [
            {
              "name": "timestamp",
              "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
              "valid": true
            },
            {
              "name": "capability",
              "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
              "valid": true
            }
          ],
          "descriptor": {
            "schemaVersion": 1,
            "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
            "semanticKind": "set-n",
            "displayName": "Set N",
            "emoji": "🔢️",
            "aggregateVariant": "SetN",
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": "set-n",
            "binaryTag": 0,
            "invertibility": "explicit-mutation",
            "diffParticipation": "apply-only",
            "outcomeClasses": [
              "applied"
            ],
            "composition": "atomic",
            "requiredLanguageSurfaces": [
              "rust",
              "json-schema",
              "text",
              "binary"
            ]
          }
        },
        "roots": [
          {
            "token": 13,
            "body": 15,
            "end": 2488
          }
        ],
        "headers": [
          {
            "start": 0,
            "end": 172,
            "token": 0,
            "parent": null
          }
        ],
        "chunks": [
          {
            "id": "snapshot-and-diff",
            "sha256": "bc0577e23351e81d189de9452fcf3b0daa307e8232be46b8853f476b36f85fc8",
            "valid": false,
            "occurrences": [
              {
                "start": 190,
                "end": 3251,
                "parent": 38
              }
            ]
          },
          {
            "id": "op-text",
            "sha256": "68bf06f92a741872e0801b0d8956e439fea854fa45cf06facfa63287b7760597",
            "valid": true,
            "occurrences": [
              {
                "start": 3258,
                "end": 4511,
                "parent": 15
              }
            ]
          },
          {
            "id": "op-binary",
            "sha256": "eade2db8824fdfab07cb78358856e3735d712235c01abd7ae4eb84c370be7540",
            "valid": true,
            "occurrences": [
              {
                "start": 4512,
                "end": 6888,
                "parent": 15
              }
            ]
          }
        ],
        "consumers": [
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_operation_envelope",
            "expected": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1542,
                "body": 1546,
                "end": 1581
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "wire_fixtures_stay_byte_identical_across_rust_and_ts",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })",
              "DemoMutation::SetN(SetN { n: 0 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1583,
                "body": 1587,
                "end": 1686
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "sample_wire_envelope_for_fixtures",
            "expected": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 6 })",
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1688,
                "body": 1692,
                "end": 1791
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "op_envelope_from_stored_edit_round_trips_through_ingest",
            "expected": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 0 })",
              "DemoMutation::SetN(SetN { n: 42 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1793,
                "body": 1797,
                "end": 1902
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "folder_external_edit_delivers_remote_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 42 })",
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 1919,
                "body": 1923,
                "end": 2049
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "two_hosts_converge_through_hub",
            "expected": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 7 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2051,
                "body": 2055,
                "end": 2095
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "reconnect_since_catch_up_replays_backlog",
            "expected": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2097,
                "body": 2101,
                "end": 2139
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "detach_drains_pending_outbound_operations",
            "expected": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 5 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2141,
                "body": 2145,
                "end": 2185
              }
            ]
          },
          {
            "modulePath": [
              "tests",
              "actor_tests"
            ],
            "function": "command_outcome_accepted_fires_after_hub_ack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2187,
                "body": 2191,
                "end": 2231
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_event_log_storage_round_trips_undo_position_through_pack_spr",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2234,
                "body": 2238,
                "end": 2317
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_dsl_and_appends_ops",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2319,
                "body": 2323,
                "end": 2402
              }
            ]
          },
          {
            "modulePath": [
              "tests"
            ],
            "function": "folder_text_storage_round_trips_pack",
            "expected": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "observed": [
              "DemoMutation::SetN(SetN { n: 1 })",
              "DemoMutation::SetN(SetN { n: 2 })"
            ],
            "valid": true,
            "ranges": [
              {
                "start": 2404,
                "body": 2408,
                "end": 2487
              }
            ]
          }
        ],
        "allConstructors": [
          {
            "token": 1552,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 1568,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1618,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1667,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1723,
            "expression": "DemoMutation::SetN(SetN { n: 6 })"
          },
          {
            "token": 1772,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 1817,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1860,
            "expression": "DemoMutation::SetN(SetN { n: 0 })"
          },
          {
            "token": 1889,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 1937,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 1982,
            "expression": "DemoMutation::SetN(SetN { n: 42 })"
          },
          {
            "token": 2025,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2069,
            "expression": "DemoMutation::SetN(SetN { n: 7 })"
          },
          {
            "token": 2115,
            "expression": "DemoMutation::SetN(SetN { n })"
          },
          {
            "token": 2159,
            "expression": "DemoMutation::SetN(SetN { n: 5 })"
          },
          {
            "token": 2205,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2252,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2291,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2337,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2376,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          },
          {
            "token": 2422,
            "expression": "DemoMutation::SetN(SetN { n: 1 })"
          },
          {
            "token": 2461,
            "expression": "DemoMutation::SetN(SetN { n: 2 })"
          }
        ],
        "retired": [],
        "hooks": [
          {
            "name": "timestamp",
            "sha256": "6f7a9804224c5ba48c0836b5b0e4b884c2409cf6c11e7649ee95af953c3ef970",
            "valid": true,
            "occurrences": [
              {
                "start": 1069,
                "end": 1164,
                "token": 243,
                "parent": 96
              }
            ]
          },
          {
            "name": "capability",
            "sha256": "cf48fdcfac981d80c56a23f22a656790022bb1e47142261bfe26d3844f37163c",
            "valid": true,
            "occurrences": [
              {
                "start": 1166,
                "end": 1231,
                "token": 262,
                "parent": 96
              }
            ]
          }
        ],
        "schemaResults": [
          {
            "id": "snapshot-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "snapshot-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "snapshot-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-array",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "diff-omitted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-null",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "diff-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "payload-missing",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "payload-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-flat",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-same-zero",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-negative",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-min",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-max",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-unknown-accepted",
            "expected": true,
            "actual": true,
            "pass": true,
            "errors": null
          },
          {
            "id": "envelope-lowercase",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-kebab",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-unknown-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/operation",
                "schemaPath": "#/properties/operation/const",
                "keyword": "const",
                "params": {
                  "allowedValue": "SetN"
                },
                "message": "must be equal to constant"
              }
            ]
          },
          {
            "id": "envelope-missing-operation",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "operation"
                },
                "message": "must have required property 'operation'"
              }
            ]
          },
          {
            "id": "envelope-missing-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null-n",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "envelope-wrapper-not-flat",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/required",
                "keyword": "required",
                "params": {
                  "missingProperty": "n"
                },
                "message": "must have required property 'n'"
              }
            ]
          },
          {
            "id": "envelope-null",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              },
              {
                "instancePath": "",
                "schemaPath": "#/type",
                "keyword": "type",
                "params": {
                  "type": "object"
                },
                "message": "must be object"
              }
            ]
          },
          {
            "id": "snapshot-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "snapshot-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "snapshot-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "snapshot-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "diff-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "diff-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf/1/type",
                "keyword": "type",
                "params": {
                  "type": "null"
                },
                "message": "must be null"
              },
              {
                "instancePath": "/n",
                "schemaPath": "#/properties/n/anyOf",
                "keyword": "anyOf",
                "params": {},
                "message": "must match a schema in anyOf"
              }
            ]
          },
          {
            "id": "payload-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "payload-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "payload-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "payload-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-boolean",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-string",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-fraction",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/type",
                "keyword": "type",
                "params": {
                  "type": "integer"
                },
                "message": "must be integer"
              }
            ]
          },
          {
            "id": "aggregate-underflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/minimum",
                "keyword": "minimum",
                "params": {
                  "comparison": ">=",
                  "limit": -2147483648
                },
                "message": "must be >= -2147483648"
              }
            ]
          },
          {
            "id": "aggregate-overflow",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          },
          {
            "id": "aggregate-large",
            "expected": false,
            "actual": false,
            "pass": true,
            "errors": [
              {
                "instancePath": "/n",
                "schemaPath": "https://semio.tech/schema/store-sync-demo/intrinsic/1#/$defs/i32/maximum",
                "keyword": "maximum",
                "params": {
                  "comparison": "<=",
                  "limit": 2147483647
                },
                "message": "must be <= 2147483647"
              }
            ]
          }
        ]
      }
    }
  ],
  "helperEvidence": {
    "declarations": [
      {
        "name": "rustIdentifierPart",
        "text": "function rustIdentifierPart(character: string): boolean {\n  return character === \"_\" || /[\\p{L}\\p{N}]/u.test(character);\n}",
        "sha256": "30b7c311249717b21c945ccba782aca4d869f4dcf79046e39c98606aeaff338f"
      },
      {
        "name": "rustTokens",
        "text": "function rustTokens(source: string): RustToken[] {\n  const tokens: RustToken[] = [];\n  const punctuation = [\"::\", \"=>\", \"->\", \"..=\", \"...\", \"..\", \"&&\", \"||\", \"<=\", \">=\", \"==\", \"!=\", \"<<=\", \">>=\", \"<<\", \">>\"];\n  let index = 0;\n  while (index < source.length) {\n    const start = index;\n    const character = source[index]!;\n    if (/\\s/u.test(character)) {\n      index += 1;\n      continue;\n    }\n    if (source.startsWith(\"//\", index)) {\n      index = source.indexOf(\"\\n\", index + 2);\n      if (index < 0) break;\n      continue;\n    }\n    if (source.startsWith(\"/*\", index)) {\n      let depth = 1;\n      index += 2;\n      while (index < source.length && depth > 0) {\n        if (source.startsWith(\"/*\", index)) {\n          depth += 1;\n          index += 2;\n        } else if (source.startsWith(\"*/\", index)) {\n          depth -= 1;\n          index += 2;\n        } else index += 1;\n      }\n      continue;\n    }\n    const rawPrefix = source.startsWith(\"br\", index) ? 2 : source.startsWith(\"r\", index) ? 1 : 0;\n    if (rawPrefix > 0) {\n      let cursor = index + rawPrefix;\n      while (source[cursor] === \"#\") cursor += 1;\n      if (source[cursor] === '\"') {\n        const hashes = cursor - index - rawPrefix;\n        const suffix = `\"${\"#\".repeat(hashes)}`;\n        const close = source.indexOf(suffix, cursor + 1);\n        index = close < 0 ? source.length : close + suffix.length;\n        tokens.push({ kind: \"string\", text: source.slice(start, index), start, end: index });\n        continue;\n      }\n    }\n    if (character === '\"' || (character === \"b\" && source[index + 1] === '\"')) {\n      index += character === \"b\" ? 2 : 1;\n      while (index < source.length) {\n        if (source[index] === \"\\\\\") index += 2;\n        else if (source[index] === '\"') {\n          index += 1;\n          break;\n        } else index += 1;\n      }\n      tokens.push({ kind: \"string\", text: source.slice(start, index), start, end: index });\n      continue;\n    }\n    if (character === \"'\" && source[index + 2] === \"'\") {\n      index += 3;\n      tokens.push({ kind: \"string\", text: source.slice(start, index), start, end: index });\n      continue;\n    }\n    if (source.startsWith(\"r#\", index) && source[index + 2] && rustIdentifierPart(source[index + 2]!) && !/[0-9]/u.test(source[index + 2]!)) {\n      index += 2;\n      while (index < source.length) {\n        const next = String.fromCodePoint(source.codePointAt(index)!);\n        if (!rustIdentifierPart(next)) break;\n        index += next.length;\n      }\n      tokens.push({ kind: \"identifier\", text: source.slice(start, index), start, end: index });\n      continue;\n    }\n    if (rustIdentifierPart(character) && !/[0-9]/u.test(character)) {\n      index += character.length;\n      while (index < source.length) {\n        const next = String.fromCodePoint(source.codePointAt(index)!);\n        if (!rustIdentifierPart(next)) break;\n        index += next.length;\n      }\n      tokens.push({ kind: \"identifier\", text: source.slice(start, index), start, end: index });\n      continue;\n    }\n    if (/[0-9]/u.test(character)) {\n      index += 1;\n      while (index < source.length && /[\\p{L}\\p{N}_.]/u.test(source[index]!)) index += 1;\n      tokens.push({ kind: \"number\", text: source.slice(start, index), start, end: index });\n      continue;\n    }\n    const operator = punctuation.find((candidate) => source.startsWith(candidate, index));\n    index += operator?.length ?? 1;\n    tokens.push({ kind: \"punctuation\", text: operator ?? character, start, end: index });\n  }\n  return tokens;\n}",
        "sha256": "97e3357050e10498fe2da3e8bd17aad458bc1f937caa7dd66341d2fd37adf075"
      },
      {
        "name": "rustTokenPairs",
        "text": "function rustTokenPairs(tokens: readonly RustToken[]): ReadonlyMap<number, number> {\n  const pairs = new Map<number, number>();\n  const stack: { readonly index: number; readonly token: string }[] = [];\n  const closeFor: Readonly<Record<string, string>> = { \"(\": \")\", \"[\": \"]\", \"{\": \"}\" };\n  for (let index = 0; index < tokens.length; index += 1) {\n    const text = tokens[index]!.text;\n    if (closeFor[text]) stack.push({ index, token: text });\n    else if (text === \")\" || text === \"]\" || text === \"}\") {\n      const open = stack.at(-1);\n      if (open && closeFor[open.token] === text) {\n        stack.pop();\n        pairs.set(open.index, index);\n        pairs.set(index, open.index);\n      }\n    }\n  }\n  return pairs;\n}",
        "sha256": "3b104af66510903dd27261025846b262b165e66e138a2095ce57fdb9b43891cb"
      }
    ],
    "closureSha256": "25c227591e313fc7370129f83676571b9796d2e4644c07f77429134d227ff368",
    "emittedSha256": "9e6b8e4b671a20008b48527b4af7f8b0ec463cea1a316fd385db5bac99f29dd1",
    "globals": [
      "Map",
      "String"
    ],
    "execution": "captured-repository-lexer-only",
    "independentRustOracle": false,
    "candidateRustExecuted": false
  },
  "wholeFileDrift": [
    {
      "role": "sync",
      "first": "62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6",
      "final": null,
      "wholeFileChanged": null
    },
    {
      "role": "discovery",
      "first": "5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956",
      "final": null,
      "wholeFileChanged": null
    }
  ],
  "observedDFromContract": "5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956",
  "first": [
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/📜️script.ts",
      "sha256": "f0d8784a40fa815f4651b427f1429779f4d1e2fa3796bfe314328b5424d3b0ea",
      "bytes": 43857,
      "identity": {
        "dev": "16777230",
        "ino": "135017631",
        "mode": "33188",
        "size": "43857",
        "mtimeMs": 1787894106934.2676,
        "ctimeMs": 1787894106934.2676
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75",
          "identity": {
            "dev": "16777230",
            "ino": "135016215",
            "mode": "16877",
            "size": "288",
            "mtimeMs": 1787895311265.9126,
            "ctimeMs": 1787895311265.9126
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/📜️script.ts",
          "identity": {
            "dev": "16777230",
            "ino": "135017631",
            "mode": "33188",
            "size": "43857",
            "mtimeMs": 1787894106934.2676,
            "ctimeMs": 1787894106934.2676
          }
        }
      ],
      "role": "controller"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧬️schema/🔣️.json",
      "sha256": "29d4ffeac496bae621e43fa01c6bddce0a926677f598e7d951c54d63d3c15795",
      "bytes": 13024,
      "identity": {
        "dev": "16777230",
        "ino": "135016217",
        "mode": "33188",
        "size": "13024",
        "mtimeMs": 1787894017675.6597,
        "ctimeMs": 1787894017675.6597
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75",
          "identity": {
            "dev": "16777230",
            "ino": "135016215",
            "mode": "16877",
            "size": "288",
            "mtimeMs": 1787895311265.9126,
            "ctimeMs": 1787895311265.9126
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧬️schema",
          "identity": {
            "dev": "16777230",
            "ino": "135016216",
            "mode": "16877",
            "size": "96",
            "mtimeMs": 1787893310129.1858,
            "ctimeMs": 1787893310129.1858
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧬️schema/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135016217",
            "mode": "33188",
            "size": "13024",
            "mtimeMs": 1787894017675.6597,
            "ctimeMs": 1787894017675.6597
          }
        }
      ],
      "role": "schema"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🔣️contract.json",
      "sha256": "140b1773d791f4f04a5349183b64d0668794a8136aaff008460d59e50483a038",
      "bytes": 33549,
      "identity": {
        "dev": "16777230",
        "ino": "135016218",
        "mode": "33188",
        "size": "33549",
        "mtimeMs": 1787894017675.0789,
        "ctimeMs": 1787894017675.0789
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75",
          "identity": {
            "dev": "16777230",
            "ino": "135016215",
            "mode": "16877",
            "size": "288",
            "mtimeMs": 1787895311265.9126,
            "ctimeMs": 1787895311265.9126
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🔣️contract.json",
          "identity": {
            "dev": "16777230",
            "ino": "135016218",
            "mode": "33188",
            "size": "33549",
            "mtimeMs": 1787894017675.0789,
            "ctimeMs": 1787894017675.0789
          }
        }
      ],
      "role": "contract"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🔣️.json",
      "sha256": "71af094f9d7ad5da7466ed8f72a1a4c26065cb72180ebb29f0d92a9ec1164ab8",
      "bytes": 39619,
      "identity": {
        "dev": "16777230",
        "ino": "135016219",
        "mode": "33188",
        "size": "39619",
        "mtimeMs": 1787893626448.1611,
        "ctimeMs": 1787893626448.1611
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75",
          "identity": {
            "dev": "16777230",
            "ino": "135016215",
            "mode": "16877",
            "size": "288",
            "mtimeMs": 1787895311265.9126,
            "ctimeMs": 1787895311265.9126
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135016219",
            "mode": "33188",
            "size": "39619",
            "mtimeMs": 1787893626448.1611,
            "ctimeMs": 1787893626448.1611
          }
        }
      ],
      "role": "vectors"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧫️run-zAwXM9/📄️input-12/🦀️component.rs",
      "sha256": "62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6",
      "bytes": 267795,
      "identity": {
        "dev": "16777230",
        "ino": "135010902",
        "mode": "33188",
        "size": "267795",
        "mtimeMs": 1787892253671.619,
        "ctimeMs": 1787892253671.619
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧫️run-zAwXM9",
          "identity": {
            "dev": "16777230",
            "ino": "135010878",
            "mode": "16832",
            "size": "1024",
            "mtimeMs": 1787892324977.9114,
            "ctimeMs": 1787892324977.9114
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧫️run-zAwXM9/📄️input-12",
          "identity": {
            "dev": "16777230",
            "ino": "135010901",
            "mode": "16877",
            "size": "96",
            "mtimeMs": 1787892253671.5388,
            "ctimeMs": 1787892253671.5388
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧫️run-zAwXM9/📄️input-12/🦀️component.rs",
          "identity": {
            "dev": "16777230",
            "ino": "135010902",
            "mode": "33188",
            "size": "267795",
            "mtimeMs": 1787892253671.619,
            "ctimeMs": 1787892253671.619
          }
        }
      ],
      "role": "before-image"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "sha256": "5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956",
      "bytes": 712446,
      "identity": {
        "dev": "16777230",
        "ino": "109708023",
        "mode": "33188",
        "size": "712446",
        "mtimeMs": 1787880280382.397,
        "ctimeMs": 1787880280382.397
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework",
          "identity": {
            "dev": "16777230",
            "ino": "68444954",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787784599006.1006,
            "ctimeMs": 1787784599006.1006
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products",
          "identity": {
            "dev": "16777230",
            "ino": "68445404",
            "mode": "16877",
            "size": "320",
            "mtimeMs": 1787645968797.1858,
            "ctimeMs": 1787645968797.1858
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "68445457",
            "mode": "16877",
            "size": "416",
            "mtimeMs": 1787875316293.2292,
            "ctimeMs": 1787875316293.2292
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules",
          "identity": {
            "dev": "16777230",
            "ino": "68445458",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787475225160.7244,
            "ctimeMs": 1787475225160.7244
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",
          "identity": {
            "dev": "16777230",
            "ino": "68457148",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787856739843.06,
            "ctimeMs": 1787856739843.06
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery",
          "identity": {
            "dev": "16777230",
            "ino": "90135279",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787820510295.5476,
            "ctimeMs": 1787820510295.5476
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
          "identity": {
            "dev": "16777230",
            "ino": "109708023",
            "mode": "33188",
            "size": "712446",
            "mtimeMs": 1787880280382.397,
            "ctimeMs": 1787880280382.397
          }
        }
      ],
      "role": "discovery"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs",
      "sha256": "62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6",
      "bytes": 267795,
      "identity": {
        "dev": "16777230",
        "ino": "126682926",
        "mode": "33188",
        "size": "267795",
        "mtimeMs": 1787886214218.119,
        "ctimeMs": 1787886214218.119
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework",
          "identity": {
            "dev": "16777230",
            "ino": "68444954",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787784599006.1006,
            "ctimeMs": 1787784599006.1006
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products",
          "identity": {
            "dev": "16777230",
            "ino": "68445404",
            "mode": "16877",
            "size": "320",
            "mtimeMs": 1787645968797.1858,
            "ctimeMs": 1787645968797.1858
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os",
          "identity": {
            "dev": "16777230",
            "ino": "68445405",
            "mode": "16877",
            "size": "448",
            "mtimeMs": 1787256546470.0427,
            "ctimeMs": 1787256546470.0427
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules",
          "identity": {
            "dev": "16777230",
            "ino": "68445406",
            "mode": "16877",
            "size": "896",
            "mtimeMs": 1787314120845.061,
            "ctimeMs": 1787314120845.061
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store",
          "identity": {
            "dev": "16777230",
            "ino": "70425863",
            "mode": "16877",
            "size": "576",
            "mtimeMs": 1787878760007.007,
            "ctimeMs": 1787878760007.007
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync",
          "identity": {
            "dev": "16777230",
            "ino": "70425879",
            "mode": "16877",
            "size": "96",
            "mtimeMs": 1787678826984.3086,
            "ctimeMs": 1787678826984.3086
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs",
          "identity": {
            "dev": "16777230",
            "ino": "126682926",
            "mode": "33188",
            "size": "267795",
            "mtimeMs": 1787886214218.119,
            "ctimeMs": 1787886214218.119
          }
        }
      ],
      "role": "sync"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json",
      "sha256": "db1c30ab7f19ab9a0f46539c71a427ba6ce51789c5c7904ea4d93dd9ea488aee",
      "bytes": 2406,
      "identity": {
        "dev": "16777230",
        "ino": "130951445",
        "mode": "33188",
        "size": "2406",
        "mtimeMs": 1787805396156.062,
        "ctimeMs": 1787805396156.062
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework",
          "identity": {
            "dev": "16777230",
            "ino": "68444954",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787784599006.1006,
            "ctimeMs": 1787784599006.1006
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products",
          "identity": {
            "dev": "16777230",
            "ino": "68445404",
            "mode": "16877",
            "size": "320",
            "mtimeMs": 1787645968797.1858,
            "ctimeMs": 1787645968797.1858
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "68445457",
            "mode": "16877",
            "size": "416",
            "mtimeMs": 1787875316293.2292,
            "ctimeMs": 1787875316293.2292
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules",
          "identity": {
            "dev": "16777230",
            "ino": "68445458",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787475225160.7244,
            "ctimeMs": 1787475225160.7244
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",
          "identity": {
            "dev": "16777230",
            "ino": "68457148",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787856739843.06,
            "ctimeMs": 1787856739843.06
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json",
          "identity": {
            "dev": "16777230",
            "ino": "130951445",
            "mode": "33188",
            "size": "2406",
            "mtimeMs": 1787805396156.062,
            "ctimeMs": 1787805396156.062
          }
        }
      ],
      "role": "descriptor-authority"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️schema/🔣️.json",
      "sha256": "e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05",
      "bytes": 723,
      "identity": {
        "dev": "16777230",
        "ino": "135009764",
        "mode": "33188",
        "size": "723",
        "mtimeMs": 1787891745518.8914,
        "ctimeMs": 1787891745518.8914
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️schema",
          "identity": {
            "dev": "16777230",
            "ino": "135009763",
            "mode": "16877",
            "size": "96",
            "mtimeMs": 1787891745518.8225,
            "ctimeMs": 1787891745518.8225
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️schema/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135009764",
            "mode": "33188",
            "size": "723",
            "mtimeMs": 1787891745518.8914,
            "ctimeMs": 1787891745518.8914
          }
        }
      ],
      "role": "reference-intrinsicSchema"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧪️tests/🔣️.json",
      "sha256": "1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2",
      "bytes": 31687,
      "identity": {
        "dev": "16777230",
        "ino": "135009766",
        "mode": "33188",
        "size": "31687",
        "mtimeMs": 1787891745524.4246,
        "ctimeMs": 1787891745524.4246
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧪️tests",
          "identity": {
            "dev": "16777230",
            "ino": "135009765",
            "mode": "16877",
            "size": "128",
            "mtimeMs": 1787891745528.3948,
            "ctimeMs": 1787891745528.3948
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧪️tests/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135009766",
            "mode": "33188",
            "size": "31687",
            "mtimeMs": 1787891745524.4246,
            "ctimeMs": 1787891745524.4246
          }
        }
      ],
      "role": "reference-domainVectors"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧪️tests/🧬️schema/🔣️.json",
      "sha256": "13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25",
      "bytes": 12358,
      "identity": {
        "dev": "16777230",
        "ino": "135009768",
        "mode": "33188",
        "size": "12358",
        "mtimeMs": 1787891745528.577,
        "ctimeMs": 1787891745528.577
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧪️tests",
          "identity": {
            "dev": "16777230",
            "ino": "135009765",
            "mode": "16877",
            "size": "128",
            "mtimeMs": 1787891745528.3948,
            "ctimeMs": 1787891745528.3948
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧪️tests/🧬️schema",
          "identity": {
            "dev": "16777230",
            "ino": "135009767",
            "mode": "16877",
            "size": "96",
            "mtimeMs": 1787891745528.4854,
            "ctimeMs": 1787891745528.4854
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧪️tests/🧬️schema/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135009768",
            "mode": "33188",
            "size": "12358",
            "mtimeMs": 1787891745528.577,
            "ctimeMs": 1787891745528.577
          }
        }
      ],
      "role": "reference-domainSchema"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🦀️.rs",
      "sha256": "ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e",
      "bytes": 593,
      "identity": {
        "dev": "16777230",
        "ino": "135009999",
        "mode": "33188",
        "size": "593",
        "mtimeMs": 1787891985586.1592,
        "ctimeMs": 1787891985586.1592
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations",
          "identity": {
            "dev": "16777230",
            "ino": "135009769",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868553.364,
            "ctimeMs": 1787891868553.364
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🦀️.rs",
          "identity": {
            "dev": "16777230",
            "ino": "135009999",
            "mode": "33188",
            "size": "593",
            "mtimeMs": 1787891985586.1592,
            "ctimeMs": 1787891985586.1592
          }
        }
      ],
      "role": "reference-aggregateRust"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔣️.json",
      "sha256": "233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a",
      "bytes": 410,
      "identity": {
        "dev": "16777230",
        "ino": "135009770",
        "mode": "33188",
        "size": "410",
        "mtimeMs": 1787891745531.7827,
        "ctimeMs": 1787891745531.7827
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations",
          "identity": {
            "dev": "16777230",
            "ino": "135009769",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868553.364,
            "ctimeMs": 1787891868553.364
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135009770",
            "mode": "33188",
            "size": "410",
            "mtimeMs": 1787891745531.7827,
            "ctimeMs": 1787891745531.7827
          }
        }
      ],
      "role": "reference-aggregateSchema"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n/🦀️.rs",
      "sha256": "4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4",
      "bytes": 16335,
      "identity": {
        "dev": "16777230",
        "ino": "135010000",
        "mode": "33188",
        "size": "16335",
        "mtimeMs": 1787892249913.5933,
        "ctimeMs": 1787892249913.5933
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations",
          "identity": {
            "dev": "16777230",
            "ino": "135009769",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868553.364,
            "ctimeMs": 1787891868553.364
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n",
          "identity": {
            "dev": "16777230",
            "ino": "135009771",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868558.7324,
            "ctimeMs": 1787891868558.7324
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n/🦀️.rs",
          "identity": {
            "dev": "16777230",
            "ino": "135010000",
            "mode": "33188",
            "size": "16335",
            "mtimeMs": 1787892249913.5933,
            "ctimeMs": 1787892249913.5933
          }
        }
      ],
      "role": "reference-leafRust"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n/🔣️.json",
      "sha256": "f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd",
      "bytes": 615,
      "identity": {
        "dev": "16777230",
        "ino": "135009772",
        "mode": "33188",
        "size": "615",
        "mtimeMs": 1787891745534.5942,
        "ctimeMs": 1787891745534.5942
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations",
          "identity": {
            "dev": "16777230",
            "ino": "135009769",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868553.364,
            "ctimeMs": 1787891868553.364
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n",
          "identity": {
            "dev": "16777230",
            "ino": "135009771",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868558.7324,
            "ctimeMs": 1787891868558.7324
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135009772",
            "mode": "33188",
            "size": "615",
            "mtimeMs": 1787891745534.5942,
            "ctimeMs": 1787891745534.5942
          }
        }
      ],
      "role": "reference-descriptor"
    },
    {
      "missing": false,
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
      "sha256": "0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1",
      "bytes": 433,
      "identity": {
        "dev": "16777230",
        "ino": "135009774",
        "mode": "33188",
        "size": "433",
        "mtimeMs": 1787891745537.4167,
        "ctimeMs": 1787891745537.4167
      },
      "ancestry": [
        {
          "path": "/",
          "identity": {
            "dev": "16777230",
            "ino": "2",
            "mode": "16877",
            "size": "704",
            "mtimeMs": 1782354543000,
            "ctimeMs": 1782354543000
          }
        },
        {
          "path": "/Users",
          "identity": {
            "dev": "16777230",
            "ino": "18439",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1784313113831.2966,
            "ctimeMs": 1784313113831.2966
          }
        },
        {
          "path": "/Users/ueli",
          "identity": {
            "dev": "16777230",
            "ino": "35146",
            "mode": "16872",
            "size": "2112",
            "mtimeMs": 1787881251268.7556,
            "ctimeMs": 1787881251268.7556
          }
        },
        {
          "path": "/Users/ueli/Documents",
          "identity": {
            "dev": "16777230",
            "ino": "35195",
            "mode": "16832",
            "size": "736",
            "mtimeMs": 1786014490563.1807,
            "ctimeMs": 1786014490563.1807
          }
        },
        {
          "path": "/Users/ueli/Documents/semio",
          "identity": {
            "dev": "16777230",
            "ino": "587164",
            "mode": "16877",
            "size": "2336",
            "mtimeMs": 1787869387893.6753,
            "ctimeMs": 1787869387893.6753
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio",
          "identity": {
            "dev": "16777230",
            "ino": "108371909",
            "mode": "16877",
            "size": "256",
            "mtimeMs": 1787779222832.9695,
            "ctimeMs": 1787779222832.9695
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo",
          "identity": {
            "dev": "16777230",
            "ino": "594692",
            "mode": "16877",
            "size": "480",
            "mtimeMs": 1787662782726.758,
            "ctimeMs": 1787843526225.8823
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets",
          "identity": {
            "dev": "16777230",
            "ino": "598031",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787842524601.2095,
            "ctimeMs": 1787842524601.2095
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26",
          "identity": {
            "dev": "16777230",
            "ino": "598782",
            "mode": "16877",
            "size": "352",
            "mtimeMs": 1787842524596.3254,
            "ctimeMs": 1787842524596.3254
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08",
          "identity": {
            "dev": "16777230",
            "ino": "72734504",
            "mode": "16877",
            "size": "832",
            "mtimeMs": 1787842524599.337,
            "ctimeMs": 1787842524599.337
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12",
          "identity": {
            "dev": "16777230",
            "ino": "96993403",
            "mode": "16877",
            "size": "512",
            "mtimeMs": 1787786981281.8425,
            "ctimeMs": 1787786981281.8425
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL",
          "identity": {
            "dev": "16777230",
            "ino": "130716741",
            "mode": "16877",
            "size": "13824",
            "mtimeMs": 1787895055866.4937,
            "ctimeMs": 1787895055866.4937
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74",
          "identity": {
            "dev": "16777230",
            "ino": "135009761",
            "mode": "16877",
            "size": "224",
            "mtimeMs": 1787892253626.0037,
            "ctimeMs": 1787892253626.0037
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate",
          "identity": {
            "dev": "16777230",
            "ino": "135009762",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891745531.629,
            "ctimeMs": 1787891745531.629
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations",
          "identity": {
            "dev": "16777230",
            "ino": "135009769",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868553.364,
            "ctimeMs": 1787891868553.364
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n",
          "identity": {
            "dev": "16777230",
            "ino": "135009771",
            "mode": "16877",
            "size": "160",
            "mtimeMs": 1787891868558.7324,
            "ctimeMs": 1787891868558.7324
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n/🧬️schema",
          "identity": {
            "dev": "16777230",
            "ino": "135009773",
            "mode": "16877",
            "size": "96",
            "mtimeMs": 1787891745537.3594,
            "ctimeMs": 1787891745537.3594
          }
        },
        {
          "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-candidate-74/🧪️candidate/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json",
          "identity": {
            "dev": "16777230",
            "ino": "135009774",
            "mode": "33188",
            "size": "433",
            "mtimeMs": 1787891745537.4167,
            "ctimeMs": 1787891745537.4167
          }
        }
      ],
      "role": "reference-payloadSchema"
    }
  ],
  "final": [],
  "nativeExecuted": false,
  "rustCompilerExecuted": false,
  "generatedProvenanceExecuted": false,
  "externalRustOracleUsed": false,
  "libraries": {
    "Ajv": "strict schema reference only",
    "jsoncParser": "strict JSON structure reference only",
    "TypeScript": "captured TypeScript helper AST/closure only"
  },
  "filesystem": {
    "lexicalComposeExclusion": true,
    "fullAncestryChecks": true,
    "noFollowFlagAvailable": true,
    "exclusiveEvidenceWrites": true,
    "atomicDirectoryRaceImmunityClaimed": false
  },
  "limitations": [
    "Captured D token spans are same-family source evidence, not independent Rust syntax, cfg activation, semantic binding, macro expansion or provenance execution.",
    "Neutral mounted fixtures are authored source-token contexts, not executable Rust programs.",
    "Source acceptance requires the reviewed exact eight candidate files; it does not grant taxonomy admission or prove the native privacy/module/derive join.",
    "Unchanged protected chunks and ordered constructor expressions do not certify unrelated Sync function bodies or global source completeness.",
    "Whole-D and whole-Sync changes are recorded separately; exact helper bodies and the scoped source projection are the acceptance boundary."
  ],
  "progress": [
    "[DEBUG] Capturing reviewed verifier inputs; Rust and provenance execution are disabled.",
    "[DEBUG] Evaluating 25 authored source-token cases with the captured D lexer."
  ]
}
```
