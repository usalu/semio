# GIS2D Presence Direct Packet Independent Review 48

## Scope and evidence boundary

This was a read-only source review of the camera-only presence aggregate, direct leaf, Rust and cross-language schemas, neutral vectors, native test source, and `🧪️gis2d-presence-direct-47/📜️script.ts`. No production or test source/controller was changed, no `compose` path was accessed, and no Cargo, rustc, or native test execution occurred.

The previous controller receipt records `1011` source/reference assertions with `46` behavioral assertions. That establishes only the controller's JavaScript/Ajv reference model; it is not native acceptance.

The reviewed source snapshot hashes are:

| Input | SHA-256 |
| --- | --- |
| Presence component | `52b39bdf16be84c81c2658ebedc03f639c771bea8d4189d65a56eee3ebc8b9d9` |
| Sparse diff | `250f3ee91326857c10bdf827a0b44ae3100ea5f01f54bc581bfd90525273b5b3` |
| Mutation aggregate | `21c603943b1b59a614708c5b2cdadff45c8512e21984160ededb6e6324c1830e` |
| Set-camera leaf | `865b1c93a1388862f9f9b8fdc115a5bddb05d3c631e36ee947f95d6f2facb65f` |
| Native test module | `e22371ea12877ca8bf82588ad9d5f3002add7cd2bbe894808adf4b51a3ca452a` |
| Domain vectors | `b9d9ab0e5564e6e373505c7a72fac255ecd51323baeee6134d256a5dbc036bd6` |
| Controller | `225bb6a23434fb55164b067b10fab9f8da2db2111b549d36dfcd4145a747a123` |

## Observed sound joins

- The aggregate has exactly `SetCamera(SetCamera)`, uses generic `DslOps`, and does not retain a manual prefix/strip path.
- State and payload serde are strict and require `cameraJson`; the JSON schemas agree for those two values.
- The direct leaf's forward diff, inverse target, `set-camera` opcode, tag `0`, and sparse ordered application are internally consistent for the one currently supported operation.
- The state/payload/aggregate schemas reject the former selection and hover fields, and the Proto sidecar retains `camera_json = 2`.

## Blocking native-test defect

`presence/🧪️tests/🧬️mutations/🦀️.rs:19` reads `fixture["envelope"]["valid"][0]` in `assert_set_camera_leaf`. The reviewed domain vector and its adjacent schema have only `state`, `payload`, `aggregate`, `diff`, and `laws`; `envelope` is both absent and forbidden by the fixture schema's `additionalProperties: false`.

Consequently, each test that invokes `assert_set_camera_leaf` reaches `Null` and then panics at `payload.as_object_mut().expect("envelope object")` before exercising descriptor, text, binary, apply, or inverse assertions. This affects both the aggregate test named `direct_payload_metadata_text_binary_and_inverse_match_neutral_fixture` and the leaf-local delegated test of the same name. The controller validates the fixture but never validates the native helper's `envelope` dependency, so its 1011 assertions do not reveal this failure.

Repair direction: make the native helper consume the existing `aggregate.valid` operation case, or add a schema-declared `envelope` section and validate it through the controller. Do not invent a second payload representation.

## Diff null/missing observation

`Gis2dPresenceDelta.camera_json` is `Option<String>` and the diff schema admits both an omitted `cameraJson` and `cameraJson: null`. Serde deserializes both to `None`, and `apply` skips both. Therefore the current implementation deliberately normalizes missing and null to the same identity step. The vector has both as valid, but native code merely checks deserialization acceptance; it does not assert that serializing either form and applying it has the declared identity effect. If semantic distinction is required, this data type cannot represent it. If equivalence is intended, a native round-trip/apply assertion must say so explicitly.

## Incomplete codec rejection coverage

The authored native rejection evidence is only `parse_op("camera cameraJson \"{}\"")` and `decode_op(&[])`. There are no neutral text/binary malformed vectors and no native probes for a wrong known tag, truncated tag/payload, malformed length/body, or a valid binary with a trailing byte. The controller validates schemas and source strings, not actual `OpText`/`OpBinary` rejection behavior.

## Capture limitation

The controller first-hashes and rereads its listed files, but its `nofollow` routine begins at the repository root and only lstat-checks descendant segments. It does not lstat/check the workspace root itself before traversing descendants. This is a capture-hardening gap, not evidence of a present symlink.

## Native status

Native tests remain unexecuted. In particular, no native RED has been observed for the defect above; it is a direct source/data-flow finding. The packet should not be scheduled as native-ready until the vector/helper mismatch and codec/null coverage are resolved.
