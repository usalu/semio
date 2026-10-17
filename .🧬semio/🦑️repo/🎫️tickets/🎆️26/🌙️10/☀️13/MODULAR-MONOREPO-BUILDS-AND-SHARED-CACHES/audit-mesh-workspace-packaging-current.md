# Independent Mesh Workspace Packaging Audit

Audited report SHA256 ee7a06a87a58e11726bc5b8e07c25ae7ca1b9e137dcd0a71737edab9c9bacd59. All 117 frozen Source/log/report rows matched before inspection; see audit-mesh-workspace-packaging-before.md.

Source inspection findings: No discrepancy in the bounded literal Source checks. Independently parsed all 31 live locks and checked 218 owner entries against actual manifest direct dependency rosters, local version 0.1.0, absent registry identity for first-party owners, and named dependency entry existence. All 22 archived related locks match their recorded census hashes and are not live substitutions. Production graph among declared owners is acyclic.

### semio-framework-3d

Production: semio-framework-3d-brep-vector, semio-framework-3d-collision, semio-framework-3d-mesh-clipping, semio-framework-3d-mesh-contract, semio-framework-3d-mesh-document, semio-framework-3d-mesh-reconstruction, semio-framework-3d-rigid, semio-framework-mesh-engine, semio-framework-value, semio-framework-value-derive. Development: none. Literal references: semio-framework-3d-brep-vector, semio-framework-3d-collision, semio-framework-3d-mesh-clipping, semio-framework-3d-mesh-contract, semio-framework-3d-mesh-document, semio-framework-3d-mesh-reconstruction, semio-framework-3d-rigid, semio-framework-mesh-engine, semio-framework-value, semio-framework-value-derive. Library closure: 7 files, 211324 bytes, 2028 nonblank lines; 0 explicit integration targets.

### semio-framework-3d-rigid

Production: semio-framework-value, semio-framework-value-derive. Development: none. Literal references: semio-framework-value-derive. Library closure: 2 files, 10714 bytes, 262 nonblank lines; 0 explicit integration targets.

### semio-framework-3d-collision

Production: semio-framework-3d-rigid, semio-framework-value, semio-framework-value-derive. Development: parry3d. Literal references: parry3d, semio-framework-3d-rigid, semio-framework-value-derive. Library closure: 2 files, 50317 bytes, 1000 nonblank lines; 0 explicit integration targets.

### semio-framework-3d-mesh-contract

Production: semio-framework-value, semio-framework-value-derive. Development: serde. Literal references: semio-framework-value, semio-framework-value-derive, serde. Library closure: 1 files, 7108 bytes, 147 nonblank lines; 0 explicit integration targets.

### semio-framework-3d-mesh-document

Production: semio-framework-3d-mesh-contract, semio-framework-mesh-engine, semio-framework-pack-json, semio-framework-value, semio-framework-value-derive. Development: parry3d, semio-framework-3d-testing-support, serde, serde_json. Literal references: parry3d, semio-framework-3d-mesh-contract, semio-framework-3d-testing-support, semio-framework-mesh-engine, semio-framework-pack-json, semio-framework-value, semio-framework-value-derive, serde, serde_json. Library closure: 25 files, 242824 bytes, 3181 nonblank lines; 0 explicit integration targets.

### semio-framework-3d-mesh-clipping

Production: semio-framework-3d-mesh-contract, semio-framework-mesh-engine, semio-framework-value. Development: serde_json. Literal references: semio-framework-3d-mesh-contract, semio-framework-mesh-engine, semio-framework-value, serde_json. Library closure: 2 files, 11726 bytes, 153 nonblank lines; 0 explicit integration targets.

### semio-framework-3d-mesh-reconstruction

Production: semio-framework-3d-mesh-contract, semio-framework-3d-mesh-document, semio-framework-mesh-engine, semio-framework-value. Development: semio-framework-3d-testing-support, serde_json. Literal references: semio-framework-3d-mesh-contract, semio-framework-3d-mesh-document, semio-framework-3d-testing-support, semio-framework-mesh-engine, semio-framework-value, serde_json. Library closure: 3 files, 111263 bytes, 900 nonblank lines; 0 explicit integration targets.

### semio-framework-3d-mesh-tests

Production: none. Development: parry3d, semio-framework-3d, semio-framework-3d-mesh-reconstruction, semio-framework-3d-testing-support, semio-framework-async-macros, semio-framework-mesh-engine, semio-framework-pack-json, semio-framework-trace, semio-framework-value, serde_json. Literal references: parry3d, semio-framework-3d, semio-framework-3d-mesh-reconstruction, semio-framework-3d-testing-support, semio-framework-async-macros, semio-framework-mesh-engine, semio-framework-pack-json, semio-framework-value, serde_json. Library closure: 1 files, 86 bytes, 1 nonblank lines; 5 explicit integration targets.

## External Identity

Framework lock retains the following exact existing identities (registry authenticity and prior history are not independently verified):

```json
[
  {
    "name": "parry3d",
    "version": "0.17.6",
    "source": "registry+https://github.com/rust-lang/crates.io-index",
    "checksum": "6aeb9659a05b1783fb2e9bc94f48225ae5b40817eb45b62569c0e4dd767a6e51"
  },
  {
    "name": "serde",
    "version": "1.0.229",
    "source": "registry+https://github.com/rust-lang/crates.io-index",
    "checksum": "4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba"
  },
  {
    "name": "serde_json",
    "version": "1.0.154",
    "source": "registry+https://github.com/rust-lang/crates.io-index",
    "checksum": "e7e9cc8b1b85264074fbcc02a88680c4096b1e47df8f739dceb03bf482f04bd6"
  }
]
```

## Evidence and Limits

Read actual packaging laws: Bun.TOML and independently installed smol-toml are compared on each manifest/lock; Ajv validates the authored schema/corpus; SQLite bidirectional EXCEPT checks exact incidence, and recursive SQL plus independent JavaScript traversal check cycles. These remain real consumed Source assertions through the existing clipping test import. Their retained terminal logs establish the reported selected Source runs; this audit did not rerun tests. The wrapper mesh branch retains --strict, --noUncheckedIndexedAccess, own import.meta.path and existing target/module/type roots. Both editor catalogs and wrapper are frozen identities. Library closures retain the 262144-byte / 32-file / 4000-line limits; explicit test roots remain distinct files and test library is only a domain docstring. No source copying or raised limit appears in this packaging cut; historical preservation is bounded by frozen identities, not a whole-repository proof.

Rigid and collision explicitly alias semio_framework_value_derive as value_derive and both declare Value plus ValueDerive; value(crate) names canonical ::semio_framework_value. Collision declares rigid and dev-only parry3d; integration tests declare MeshEngine directly. Literal imports and macro tokens are not expanded Rust resolution.

No Cargo/rustc/metadata/build, network, provider/native/cache operation, OS sampling, git mutation, production edit, lifecycle mutation or test execution was performed. Cache-disabled Bun was used only for file hashing, parsing and inspection. Cargo resolution, feature/cfg/macro closure, Rust type checking and physical compiler admission remain unperformed and unclaimed.

Final identity inspection again matched all 117 frozen rows; before/after inventory content is identical. The report itself still matches its supplied SHA256. Retained final prototype log explicitly records 15 pass / 0 fail / 5096 expectations; terminal metadata separately records prototype/release green3 and types2 exit 0. Empty type logs provide no standalone completion proof.
