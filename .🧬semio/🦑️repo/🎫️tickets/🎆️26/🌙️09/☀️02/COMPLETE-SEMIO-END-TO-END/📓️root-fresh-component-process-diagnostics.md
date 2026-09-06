# Fresh Component Process Diagnostics

## Outcome

The fresh producer now uses the existing first-party bounded process-tree owner. Cargo commands request JSON diagnostics; stdout/stderr are captured under a unique trace and the terminal outcome is written before failure publication. A thrown error has at most 6,000 diagnostic characters plus fixed metadata, not the full compiler transcript. This corrects observability and child-process retirement; it does not yet establish why GIS Cargo failed.

The caller freezes an optional diagnostics root in its build control. Ticket runs retain exclusive traces under the ticket generated directory, outside component target/stage cleanup. The producer does not read an ambient artifact-root variable or fall back to Cargo/catalog directories. Non-ticket development calls without evidence authority use an ephemeral private capture, remove it after settlement, and return only the bounded summary. Existing component/core/descriptor wiping and stage cleanup remain unchanged. Windows subprocesses retain hidden-window behavior.

## Verification

- `os-hub:browser-actor-gis-describe-check` session 78311: fixture placement setup error; corrected the two new neutral JSON files to the describe-domain fixture directory.
- Session 82224: intended TDD RED, successful subprocess produced no retained trace.
- Session 47128: GREEN, eight real subprocess laws and nine descriptor normalization laws.
- Session 45976: GREEN after caller-root refinement, eight real subprocess laws, AJV validation, three independent fast-deep-equal transcript comparisons, and nine descriptor normalization laws. Explicit evidence: `🗑️generated/fresh-process-laws-KYiBBR`.
- Process cases: success, selected compiler diagnostic, non-JSON stderr failure, spawn failure, in-flight cancellation, deadline, 64 MiB aggregate output limit, and pre-cancellation. The latter creates no trace; failures cannot advance the completion checkpoint. All launched processes settled within the 6-second law bound.
- Each law passes a deliberately wrong ambient artifact directory; the explicit control root still owns its retained trace.
- Root Hub native session 27152 remains live. The earlier genuine GIS session 38704 remains BUILD RED before actor derivation/Chromium. Its exact compiler cause is still unqualified; the next native run uses the new capture.

## Scope and Follow-Up

The fresh native GIS gate remains the required real component → derived child → guest describe test. These subprocess laws do not substitute for that gate, Hub artifact admission, Session activation, rendering, effects, Map mutation, durable commit, or collaboration.

The independently reviewed audit is [diagnostics containment](📓️terra-fresh-component-process-diagnostics-containment-audit.md). Remaining qualification includes the next genuine Cargo transcript and the production exact-Session activation owner. This goal and ticket remain open.

## Current Native Retry And Regression

Scoped format70231/write and7262/check, owned-scope diff-check, and bootstrap source2192 are GREEN. The source target now runs the eight subprocess laws alongside the eleven retained-staging laws.

Fresh native GIS65957 is BUILD ACTIVE. Its first actual Cargo trace is `🗑️generated/gis-child-real/fresh-process-HwPaH5`, a sibling outside the temporary native work/target/stage root. The first inspected capture contains compiler progress and no error-level records or outcome yet. This is not a passing build.

The next production implementation is exact-WebSocket Session activation; root fully read `📓️terra-backbone-session-browser-actor-activation-current.md`. No activation source change has landed in this pass.
