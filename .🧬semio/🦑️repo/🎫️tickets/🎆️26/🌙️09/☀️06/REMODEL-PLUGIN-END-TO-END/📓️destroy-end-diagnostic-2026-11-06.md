# Destroy And End Diagnostic — 2026-11-06

The repeated full Bun relay crash remained unexplained after deterministic close draining. Root examined a narrower hypothesis: the relay's close callbacks call end on a peer that a simultaneous teardown may already have destroyed.

Before changing production test control, Root authored the bounded loopback diagnostic 🧪️destroy-end-probe-2026-11-06.mjs inside this ticket. It runs 128 sequential real TCP connections, destroys both peers, invokes each peer's end only after the paired close callback, and waits for both actual close events. It has a 15-second AbortSignal deadline and explicit socket/server cleanup. No sleeps, pin change, external service or fabricated state are used.

Normal admitted repo tools exec Node and Bun both returned terminal exit 0 with identical output: cycles 128 and endOnDestroyedCalls 256. Retained logs are destroy-end-node-2026-11-06.log and destroy-end-bun-2026-11-06.log in the generated directory.

This does not reproduce the proposed simple destroyed-peer end failure, so it does not justify a causal relay repair. It also does not explain or erase the prior complex Bun relay suite crashes. Current drain Node/Chromium narrow successes remain separately scoped. No source implementation or runtime pin was changed from this diagnostic.
