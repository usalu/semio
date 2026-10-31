# Funding Model Source Review

Root read the new journal integration and funding replacement model while its owning tests were pending. Ordinary reserved/committed/rebudgeted paths now explicitly reject creation or use of replacement associations. The reducer retains predecessor plus exact delta on reservation, removes only delta on abort, and atomically replaces predecessor and delta with the target commitment on settlement. Expected sequence and matching immutable association DATA are checked. This review does not establish live issuer/custody or green tests.

One concrete pending invariant finding was sent to the defining owner: requireNativeStorageFundingStateV1 currently permits settled row.bytes lower than funding.targetBytes. Ordinary rebudget on funding rows is rejected, and no valid event in this model decreases a settled target; thus a checkpoint could assert target36 with charge1. Root requested exact settled charge equality or a separately owned shrink transition, plus hostile checkpoint coverage. Root also requested checking a settled association with its predecessor still retained, which does not match the atomic settlement result. No production edit or assertion relaxation was made by root.

The defining owner retains the fix, neutral/third-party regression and strict check work. Source readiness and actual funding transition remain unproved until those receipts and final Native bridge/frontdoor integration exist.

## Issuer DATA Capture Finding

Root also read the private issuer factory source. Its bounded capture currently builds a plain result object using indexed assignment of external own keys. An own JSON __proto__ key can therefore modify the result prototype and cease to be an own field before closed-schema admission. Root reported this as a potential closed-DATA bypass, requesting safe own-property construction and actual JSON.parse hostile request/allocation/policy vectors against independent Ajv. It is not claimed as an observed live authority bypass. No root production edit or runtime claim was made.

The defining owner acknowledged exact settled charge equality and settled predecessor exclusion, and will author hostile checkpoint/funding-smuggling regressions before changing the model gates. Those new checks have not yet passed.
