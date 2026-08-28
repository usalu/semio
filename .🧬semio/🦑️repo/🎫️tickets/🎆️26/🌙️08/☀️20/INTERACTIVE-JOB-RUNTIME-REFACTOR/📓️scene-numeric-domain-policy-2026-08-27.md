# Scene Numeric Domain Policy

The generic ScenePack codec preserves the IEEE f64 bit domain, including NaN, infinities and negative zero. It requires the F64 tag for floating fields; integer-tag coercion is not part of the wire contract. Scene geometry admission is a separate schema rule: the typed projection rejects nonfinite geometry fields. Opaque JSON strings remain unparsed at this stage, so this does not certify embedded JSON geometry.

Unsigned/signed integer narrowing must be exact. Native `forward_to_deserialize_num` previously used unchecked `as` conversions. The12 shared numeric vectors exposed u8=256 being accepted; child native R4 was0PASS/1FAIL and stopped at that first case. Two conversion arms now use `TryFrom`, rejecting overflow and negative-to-unsigned instead of wrapping. No float codec behavior changed. Child native R5 passed the12-vector test; full native Scene R6 passed96tests/0failed, including the19 generic vectors and15valid/6hostile typed-schema oracle.

`usize` is an owning-host admission width, not a different wire encoding. Both32-bit and64-bit profiles retain the same exact u64 arena scalar. Projection and paired binding constructors now require an explicit `{usizeBits:32|64}` profile and capture its value once. Reuse also checks that width. The eventual live bridge must obtain it from its actual owning host capability; no packet field or JavaScript Number heuristic may choose it. The current APIs are not mounted, so live capability provenance is still pending.

Renderer R5:4PASS/574skipped/578total,8.05s,start17:18:51. The numeric test validates strictAjv fixture,12 generic arena reads with NodeBuffer floating-bit oracle, and22 typed scene admissions across both host widths (the standalone i64 case is generic-only because no catalog scene field is signed i64). Profile mutation after cursor construction does not change admission. Native execution used the host's actual usize width; it is not a Wasm32 runtime test.

## Native Finite-Geometry Admission Follow-On

The native wire decoder must not acquire a blanket nonfinite ban. Proposed exact schema counterpart: a borrowed `SceneGeometryCursor` produced by explicit implementations for all15 SceneDoc types. Canvas contributes3f64 fields; NodeGraph contributes4 per node and3 optional viewport fields; Paint contributes2; Board contributes2; the other11 have no direct f64 fields. One cursor advance checks one scalar with explicit item/byte accounting, and cancellation releases only bounded borrowed iterator frames. This avoids hiding a whole NodeGraph scan behind `decode_pack` or introducing a success-by-default geometry trait.

Native finite admission is not yet implemented/mounted. All-default/unknown-field parity expansion is also still pending. Existing15/6 parity does not claim these additional edge laws. Native declarations/Flow catalogue width types were not changed.
