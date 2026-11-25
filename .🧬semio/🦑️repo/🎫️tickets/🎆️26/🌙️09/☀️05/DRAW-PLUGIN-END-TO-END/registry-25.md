# Registry And Publication Verification — 2026-11-25

- `repo test-file --lang=en -- <registry discovery test>`: discovery file passed 13 tests and 129 assertions, including independent AJV contract and Node stage-vector checks. This is registry fixture coverage, not a fresh Draw component descriptor emission.
- `repo test-file --lang=en -- <registry publication test>`: exited 0, 12 tests and 26 assertions passed, including independent AJV rejection. This exercises neutral publication authority fixtures, not the Draw-specific publication audit.
- The combined discovery, descriptor-verification and publication test invocation progressed to descriptor verification after discovery passed; the descriptor process was still active at 64 seconds with no test result. The descriptor suite inventories the canonical schema scopes. Publication in this combined invocation had not yet started; it was separately run as above.
- `repo run @semio-tech/draw-js:publication-authority-audit --lang=en` and `repo run @semio-tech/draw-js:browser-oracle-check --lang=en` were waiting at capacity admission after 50 seconds. The host reported memory pressure and swapping; neither target body had executed. These are not passes or failures. The oracle check is the browser acceptance oracle and does not itself prove a real browser journey.
- No fresh Draw descriptor/component pair or third-party drawing carrier oracle is established by these results.

Both Draw targets were subsequently admitted after roughly 67 seconds. The audit reached project graph planning and was waiting for a slice worker; the oracle was waiting for a resident service slot. No target body or result was observed at that point. The descriptor invocation remained active at 84 seconds.

A direct `repo ./📜️script.ts publication-authority-audit` from the Draw TypeScript package exited 1 before the audit: the script rejected the missing Original parent Script process invocation. The registered target remains the valid route.

The combined invocation subsequently exited 0: discovery 13 pass, descriptor-verification 21 pass, publication 12 pass, no failures or skips. Descriptor-verification took 150.2 seconds. Its fixture-based descriptor admission and schema inventory are not an emitted Draw descriptor comparison.

`repo tools exec bun -- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️check.mjs` exited 0 under 2048 MB admission: all five PDF oracle cases passed (same PDF, metadata only, edited rectangle, empty page, malformed document). The registered browser-oracle target was cancelled with SIGINT while still waiting for graph planning; it did not execute. This direct check runs the same oracle script, not a served browser journey.

The registered publication target was cancelled while in graph-slice planning, before its body. A direct `repo tools exec bun -- -e` invocation of the identical exported `proveDrawPublicationAuthority` function with the same package and subset roots exited 0, admitted at 1921 MB: 69 independent AJV field-patch cases, React Scheduler yield/resume oracle, DrawingPlayApp 38 routes, five hostile mutations and four mutation admission cases. This proves that function on the observed source, not the registered target's graph or a compiled guest.
