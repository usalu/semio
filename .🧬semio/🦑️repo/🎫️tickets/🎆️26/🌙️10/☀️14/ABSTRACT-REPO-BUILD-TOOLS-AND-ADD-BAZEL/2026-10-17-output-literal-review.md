# Output Style And Literal Exec Review

Read-only four-root Source review plus new style DATA; no runtime, protected installed SDK, Native/store access or expensive suite. Parent/agent reported gates are not reviewer reruns.

Literal exec selects program=first forwarded token and args=remaining tokens with literal cwd. Backend bypasses authored interpolation for program/cwd, shell-quotes program and every literal argument separately, while authored command templates retain existing interpolation. Neutral execution interface admits exclusive nonempty program and configuration cannot change program/command form. No material literal placeholder defect identified in inspected current Source. Per-command forward flag fix preserves argument vector before individual suppression.

Output static path requires finite outputlimits and actual control, counts raw bytes/lines beforebuffering, uses separate per-stream StringDecoder and invokes readiness onLine before buffering. Stream renderer tracks beginning-of-line acrosschunks and decoder preserves splitUTF8. Backend supplies requested renderer at actual outer Bazel/service command boundary; service onLine readiness remains live even when displaystatic. Child nonzero/signal outcome is checked before deliveryFailure, preserving child primary on simultaneous static sinkfailure. No stamp/control policy change is involved.

One concrete coverage/robustness concern sent to Bazel agent: explicit stream and stream-without-prefixes still pipe to destination without error listener/pipeline, while static writehelper reports deliveryfailure. Async destination error during stream could become unhandled instead of retained terminal outcome; new seven-case corpus covers sinkfailure only for static. Raw default path already had the same underlying pipe behavior, so this is not claimed as newly introduced regression or an executed failure. Add bounded async sinkerror cases for both stream modes and childstatus7 before asserting all-style delivery provenance. No production fix prescribed absent reproduced law.

Observed hashes:
- owned-execution/🟦️.ts:38679e6257e2d9c587556fc726df53e64677733212e2fc4d3823fc5b9b82c6e2
- Bazel/📜️script.ts:3533b43b1d8867ebe5058180aa5f4352a3dc38c077a73e2557505b479ae423cc
- build/🧭️selection/🟦️.ts:7bd65d56ed23a98b1d2b1afda6d12c5518b8a33d1a01fd5b948a9f7d97869e6d
- build/🕸️graph/🟦️.ts:2d82e4a278b16736a6c66526e43f140fb83780b7aec2706941bd8ba1c36b4a6a

These direct byte observations are not complete transitive FD freshness holds. Current Nx adapter Source was inspected; installed Nx runtime comparison remains owned by agent's genuine oracle gates.
