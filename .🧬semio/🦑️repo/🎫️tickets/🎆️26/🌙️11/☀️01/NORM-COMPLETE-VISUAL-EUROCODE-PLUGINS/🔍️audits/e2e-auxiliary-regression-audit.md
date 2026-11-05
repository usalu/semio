# Narrow Auxiliary Regression Execution

2026-11-04. Attempted only the existing inherit real-child test with Bun --test-name-pattern through repo tools. No cargo, new test code, forged invocation or large-stream test was launched.

The initial light tool invocation reached Bun but failed before any real auxiliary child: Original parent Script process invocation required. A normal tools invocation first failed EROFS writing the default host-capacity queue. Retried with the exact shared ticket SEMIO_HOST_CAPACITY_DIRECTORY used by the fleet. It waited on ordinary capacity, was admitted at 2048 MB, then reached Bun and failed identically before the worker: zero pass, nine filtered out, two failures (the inherit test and harness cleanup), 139 ms. This is a test ingress failure, not reproduction of EPERM or evidence that the auxiliary pipe works. The normal tools run is terminal; no test process remains live.

The official route is `repo ./🧰️framework/🔨️modules/🏃️process/📜️script.ts test owned-execution`. Its authored suite row expands the whole owned-execution tree with a 120000 ms budget. The suite runner supplies genuine script birth and test sandbox; the raw test file does not declare a standalone birth. No narrower authored suite exists for auxiliary inherit, and no caller-controlled test-name filter is present in that official route. Running it would include the expressly excluded large-stream tests, so it was not launched.

Next safe choice: an authorized narrow authored test route or an approved broad owned-execution suite through the existing official script, subject to its ordinary source admission. Do not manufacture an invocation envelope to make the raw test pass.
