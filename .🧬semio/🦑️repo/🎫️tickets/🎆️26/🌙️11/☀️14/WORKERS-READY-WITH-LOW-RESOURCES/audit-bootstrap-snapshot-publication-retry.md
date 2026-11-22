# Bootstrap Snapshot Publication Retry

The current replica marked its snapshot as shared before starting its asynchronous publish. The publisher uses an explicit result for a not-yet-sealed local snapshot, for an already verified destination, and for a temporary write failure. Only thrown errors cleared the shared flag. A non-throwing refusal therefore prevented every later retry in that process.

The loop now records success only after a seal is published or the existing destination has been verified. One publication stays in flight at a time, outside the replica's ordered sync turn. A refusal is reported once per changed reason and can retry under the existing healthy-store and mature-code checks. The retry delay follows the existing poll and initial-publication settings within the schema's 30-second normal ceiling. The exact snapshot verifier and atomic final publication retain their original custody.

A private fixture temporarily gives the ordinary store code directory the wrong file type. The production publisher refuses it. Restoring the directory path then lets the running daemon publish the same release without restarting. The regression checks the refusal, the exact resulting seal and every installed file hash through Node's independent crypto implementation. The ordinary healthy publication test originally timed out with the same unchecked result. The timing-sensitive watchdog fixture also now leaves its dummy process blocked longer than the recorded watchdog and stack-sampling window, then checks the typed stop receipt.

The first regression run encountered a JSON escaping error in the newly declared policy. The schema was corrected before the red run and native validation. After implementation, the three focused tests for the healthy publication, refusal recovery and blocked-loop stop passed with 25 assertions. The real coordinator's concurrent outbound journal delay is a separate observation and is not attributed to this bootstrap snapshot publication branch.

The final direct boot suite passed all 23 tests with 185 assertions, including the exact temporary store-directory refusal, local seal publication, fallback and process stops.
