# Dependency Wait Terminal Failures — 2026-11-05

Root re-polled its actual handles at approximately 16:16 UTC. Full Remodel Vitest 97244 and the prepared React host 64120 both returned terminal exit 1. Their retained logs, vitest-fresh.log and host-fresh.log, report Dependency build lease wait exceeded its declared maximum. No Vitest body, prepared host readiness or UI behavior is proved.

This is a real terminal failure of those two consumers, not an observation timeout. It does not prove that the shared producer failed or stopped. Descriptor 62456 and registered browser-startup 7387 continued reporting the same shared dependency-build wait at the inspected instant. The read-only audit is examining current producer facts and its actual work/wait mechanism. No peer process was killed, no lock removed and no limit raised.

The failed consumers have not been blindly restarted against the same unresolved prerequisite. Their required full scopes remain in the completion audit. A current prepared host and full Vitest verdict are still required. Root native workspace 19262, Hub library 87775 and WASM component 38749 were authoritatively re-polled live during this check; no successful verdict is inferred.

Other aligned work continues: the real TCP and Chromium worker relay proofs passed; official saved-artifact reconnect integration and strict accounting were authored, and the plugin-owned twelve-scenario browser acceptance driver is under source review. Its remaining unexecuted scenarios cannot pass. These are distinct proofs and do not substitute for the failed frontend scopes.
