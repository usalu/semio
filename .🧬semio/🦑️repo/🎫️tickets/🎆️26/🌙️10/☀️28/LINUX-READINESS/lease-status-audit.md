# Lease Status Contract Repair

The actual workspace10173 run reached the owned Nx workspace test leaf and exposed two stale lease-status fixture assumptions. The production runLeasesCliV1 API and JSON publisher are correct and unchanged.

## Causal Red Evidence

The focused suite reproduced one passing and two failing tests. The command helper passed an empty environment to runLeasesCliV1. The current language contract correctly refused to choose a default language, leaving no output; the test then obscured that refusal by parsing the empty output as JSON.

Adding an assertion on the captured command error before JSON parsing exposed the exact leading cause: the bilingual "No language selected for texts" refusal. The four-argument API still correctly forwards environment, directory and write callback to runLeasesCommandV1. No typed Original is needed for this existing read-only status API.

The second fixture hardcoded 4096 MiB as a minimum admission floor. Current canonical policy instead sets a 1536 MiB minimum, a 4096 MiB maximum and 1250 basis points of host memory. The actual law is:

`min(total, max(1536, min(4096, floor(total * 1250 / 10000))))`

The old assertion contradicted that current authored policy; no admission guard or production floor was changed.

## Fixture Correction and Independent Checks

The closed neutral command corpus now explicitly selects English. Additional cases select German for JSON and pass an empty environment to verify the existing bilingual no-language refusal and absence of output. The helper checks command errors before parsing JSON.

The corpus declares the current floor policy, and a new test asserts exact equality with the production policy fields. Five neutral host-size vectors cover a host smaller than the nominal minimum, the minimum region, proportional region and maximum region. Python independently computes the same law and is compared with the actual hostFloorMegabytesV1 implementation.

The actual live admission result is validated with the existing canonical closed Admission schema, including referenced grant and waiter shapes, plus the authored floor bounds. The test no longer duplicates an incomplete approximation of that schema.

A further current-contract mismatch was reproduced red: canonical AdmissionGrant permits zero jobs for quiet grants, while the old Python oracle rejected them. Three neutral grant cases now compare Ajv's canonical grant validation and the Python oracle: zero and one jobs are valid; negative jobs remain invalid. The oracle was corrected to the existing zero-job contract.

Temporary lease roots now require the caller's explicit SEMIO_TEST_ARTIFACT_DIR and remain inside its generated output directory. No machine-global registry entry, peer process or shared lease was changed.

## Verification

The final focused run completed0: 5 tests, 82 assertions, 288 ms. Full output was streamed directly to `🗑️generated/lease-status-final.log`.

Only four adjacent test files changed under `🏗️build/🔒️leases/⌨️command/🧪️tests/🧮️status`: the TypeScript runner, neutral corpus, closed corpus schema and independent Python oracle. Production status, language selection, scheduling/admission and output code remain unchanged.

The full workspace gate was not retried by this agent. Root owns the next actual gate and the separate Nx diagnostic.
