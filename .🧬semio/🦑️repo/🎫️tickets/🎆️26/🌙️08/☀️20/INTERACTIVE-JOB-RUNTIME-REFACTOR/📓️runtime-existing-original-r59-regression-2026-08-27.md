# Original Existing-Component Gate R59 — Scheduling Regression

Actual R59: **1 passed, 1 failed, 107 skipped**, 0.186s. The original zero-credit/source-preservation law passed unchanged. The original completed-copy law failed because its expected pending SetComponent was already moved to output pages before the component field had advanced.

The new separate empty-copy release phase exposed a scheduling order bug: the general pending-patch branch ran before that exact source field's final return phase. The patch now completes the empty retained component owner first, in its separate charged turn, leaving the pending patch structurally owned until the original field completion is observable. It does not change the original assertions, combine child and parent work, or restore same-turn root transfers.

Raw `🧪️member-runtime-existing-original-r59-native-2026-08-27.txt`, process 54173 exited 1. R60 is the unchanged original two-law rerun.
