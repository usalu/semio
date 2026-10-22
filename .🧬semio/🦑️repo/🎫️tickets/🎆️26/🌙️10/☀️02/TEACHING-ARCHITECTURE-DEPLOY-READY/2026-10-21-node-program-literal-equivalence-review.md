# Node Program Literal Equivalence Review

Root statically inspected Node owner8991bytes/SHA-256`dcc896b84dc53761bc694ddc0779e58d3fc13d2292f093474bab0be09125a1f3` against the held prior authored9015-byte owner. The new String.raw program text is exactly `await (` plus the prior capturedNodeMainV1 function declaration AST text plus `)();`. It has7749UTF8bytes/SHA-256`20a1e2b4c904d5e8c8bfb9c8bb51ddcf25e50c92cc58ad63cda97371f191326b`.

The raw program contains no template interpolation. Both ownerSource and raw program have zero TypeScript AST parse diagnostics. This uses Source text and static AST only; no Source/helper or child program is evaluated by tooling. All manifest/hash/bootstrap/hook/import/readiness behavior inside the authored program is byte-for-byte unchanged. The material change is the serialization boundary: the parent receives a literal program instead of stringifying an already-transformed executable function.

Actual Native17628 confirms the prior enclosing Bun redirect leaked its module alias before Node hooks began. A direct host-MJS probe cannot establish the repair; the forthcoming actual captured-parent→Node regression and new Native run remain necessary. No runtime pass, prefix acceptance or redeploy artifact is claimed. Prospective Source280 policy is revised by45000ms for that existing Node law, with both old/new policy proposals preserved.
