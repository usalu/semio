# Held Chart Paged Operation Producer

Read the exact held pair in `📥️inputs/operation-byte-pages/chart-paged-producer-held-pair.json`. Its Chart path is the actual existing operation codec owner. The ordinary Text print/parse, ordinary binary encode/decode and the exact `[1,1]` header remain byte-for-byte unchanged in the pair.

The new required sink method uses the actual derived specification producer and controlled Record producer for `ChangeChartValue`, passes the same caller NativeEncodeControl into header writing and the canonical Core Record body sink, and preserves typed PackRefusal conversion. It supplies the existing ordinary default Record encode options, consistent with the current ordinary operation method whose signature also has no options parameter.

This is a static readback of held text only. The proposed body API, operation output interface, bounded page accounting, controlled owner creation, partial-output cleanup and reader/caller closure must be provided and tested together by the operation-byte owner. No compile, runtime, allocation, byte-equality or cancellation credit is claimed for this future revision. The frozen Print Native 23/23 receipt applies to the earlier ordinary codec revision.
