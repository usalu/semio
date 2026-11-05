# Independent ACK Corpus Oracle

Added permanent Bun test FullOp/🧪️tests/📬️ack/🟦️.ts, using existing test-only XState and Ajv. No runtime dependency or production source changed. Ajv validates the complete permanent schema/corpus; one XState machine per13rows models AwaitingAck, PublishingRetained, RetiringRetained, PublishingEmpty and Retired. Exact five-field token matching drives ACK. Advancement requires accepted ACK, matching original source and all currencies funded. Wrong source refuses without releasing custody. Derived10expected fields are compared exactly with each neutral expected object. Artifact uses its own initial token generation1/sequence0/attempt1, independently of base initial token.

Executed SEMIO_LANGUAGE=en repo tools exec --light bun -- test ./<permanent test path>. Result14pass0fail,15expect calls,189ms across1file. An initial operand without ./ was refused by repo before Bun started; corrected explicit path ran normally.

This result validates an independent state-machine oracle and fixture schema. It does not claim Rust physical retirement, actual TypedCtx routing or native editor execution.

Latest directpublisher helper source now takes every returned ownership receipt, checks both grants, and posts only the unposted retained delta. Document precisephase loop uses this helper. Reported remaining fuel parity detail: production pump consumes one fuel when unit itself did not, whereas the direct fixture helper currently only reconciles retained bytes. Preserve the same original source and charge fuel once.
