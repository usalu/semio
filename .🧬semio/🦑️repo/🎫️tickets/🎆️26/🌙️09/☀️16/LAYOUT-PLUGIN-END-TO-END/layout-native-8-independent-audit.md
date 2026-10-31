# Layout Native Eight Independent Audit

Read-only review after Rust reported session 49628 still live and past the fixed-image lease queue. The actual live log snapshot contains 167,282 bytes, SHA-256 08997ca48681af7baee78289d3e43f6574fc33cf4d7ea91c474de1a6e1208947.

The log confirms scheduler Source admission of 1/1, 28,834 bytes, followed by workspace selection. This is the scheduler wrapper, not the full selected Cargo graph. Initial image headroom requires 7.2 GiB with 23.6 GiB free, 19.6 GiB still expected from three running commands, no reservations ahead, and a 1.4 GiB floor. The owner announced freeing 4.6 GiB; no completed reclaim receipt was observed. At the next logged headroom observation, 25 seconds later, free space was 25.0 GiB and outstanding estimates were 19.3 GiB. This change cannot be attributed to that collector announcement alone.

Headroom admission, full selected Source admission, compilation and test execution remain unproved at this boundary. No terminal result or postprocessor condition is satisfied. Root's conditional release remains one attempt each for Dep's media, SDK and owned SQL checkers only after full Native8 EXIT0, the relevant owning PASS, finalized exact record sets and unchanged reviewed inputs. All final acceptance gates remain mandatory. The reviewer performed no runtime, Source edit, cache action or repeated scan.
# Selected Source Completion and Compile Boundary

A single bounded follow-up read of the live Native8 log observed 3,321,786 bytes, SHA-256 `ac79c2bcc7e9ced03c61decd35dc8a90cf20498376075977e7a007854c9e385c`. This is a live snapshot, not a finalized log hash. Actual Source completion is 916/916 units, 8,946 cumulative files and 67,992,933 bytes (line 2922). Model units 595 and 596 completed at lines 2280 and 2282; model-tests 597 and validation-tests 598 completed at lines 2284 and 2286. Their cumulative file/byte totals were respectively 4,239/46,351,325; 4,270/46,573,184; 4,287/46,651,266; and 4,299/46,803,970.

The same log contains actual compilation and its tail reaches plugin job fault publication and host configuration after a still-running report at 2,140 seconds. There are zero `Running unittests` and zero `test result:` boundaries in this snapshot. Source admission is proved for these selected owners; compiler success, terminal status, selected test results and finalized oracle records remain unproved. No duplicate runtime or source change was performed.

# Final Terminal Audit

The finalized `🗑️generated/layout-native-8/terminal.json` reports session 49628 terminal with outer exit 1, Cargo status 101 and null signal. An independent read matched the final log exactly: 6,775,534 bytes, SHA-256 `3c0d6ada450d9b69647fa52040527a26cbfd608306298d4a5c28a43c5ea78f87`.

The first diagnostic is `error: no rules expected \`=\`` at log line 82189, retained diff setters line 24 column 69. The same macro mismatch occurs at setter lines 26:79, 27:163 and 30:405. Each diagnostic identifies the `fields!` matcher `$kind:ty =>` and a generic type invocation with adjacent `>=>`. The final compiler diagnostic at log line 82249 is `could not compile semio-s-artifact-layout-layout-diff (lib) due to 4 previous errors`. The terminal tail records `OwnedCommandFailure` with status 101 and null signal. This is a compiler failure, not a selected test failure; no storage retry was observed in the terminal receipt.

The prepared canonical command retains all 19 unique selected packages, unfiltered `--lib`, `--features=testkit` and `--nocapture`, including the original 15 plus model, engine-json and both companion owners. Actual Source admission remains 916/916, 8,946 files and 67,992,933 bytes. The complete final log has zero `Running unittests` and zero `test result:` boundaries. The receipt has empty selected test/result arrays, zero media records and zero owned summaries/turns. No SDK success or record acceptance is established by this pre-test failure.

All three conditional postprocessors remain unused, with zero attempts and their Native8 success conditions unmet. Full Native19, actual owning records and browser acceptance remain mandatory. This audit performed no runtime, Source edit or retry. The eventual setter repair requires a separate current patch review and future authorized semantic verification.

