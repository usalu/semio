# Lane Status

| Lane | Wave | Status | Gate | Evidence |
|---|---|---|---|---|
| wave0-scaffold | 0 | done | cargo check OK; cargo test running under CLT SDK | stubs+glue+lint+launch+prompts |
| 1-bvh | 1 | done | FROZEN | 🧪lane-1-bvh-test-quick-run-1.txt |
| 1-primitives | 1 | done | FROZEN | 🧪lane-1-primitives-test-quick-run-1.txt (11 ok) |
| 1-measure | 1 | done | FROZEN | 🧪lane-1-measure-test-quick-run-1.txt |
| 1-tessellate | 1 | done | FROZEN | 🧪lane-1-tessellate-test-quick-run-1.txt; 🧾lane-1-tessellate-scope-note.txt |
| 1-oracle | 1 | done | `cargo test -p semio-s-3d --lib oracle` | lane-1-oracle-test-quick-run-1.txt |
| 1-int-cc | 1 | done | cargo test -p semio-s-3d --lib int_cc (4 passed) | 🧪lane-1-int-cc-test-quick-run-1.txt; 🧾lane-1-int-cc-scope-note.txt |
| 2-int-cs | 2 | done | FROZEN | wave2-run-3 |
| 2-int-ss | 2 | done | cargo test -p semio-s-3d --lib brep::int_ss:: (4 passed) | 🧪lane-2-int-ss-test-quick-run-1.txt; 🧾lane-2-int-ss-scope-note.txt |
| 2-sweep | 2 | done | FROZEN | 🧪lane-2-sweep-test-quick-run-1.txt; 🧾lane-2-sweep-scope-note.txt |
| 2-sew | 2 | done | FROZEN | wave2-run-3 |
| 2-step | 2 | done | FROZEN | wave2-run-3 |
| 2-mesh-io | 2 | done | FROZEN | wave2-run-3 |
| 3-classify | 3 | done | FROZEN | 🧪lane-3-classify-test-quick-run-1.txt (7 ok); 🧾lane-3-classify-scope-note.txt |
| 3-imprint | 3 | done | cargo test -p semio-s-3d --lib brep::imprint:: (3 passed) | 🧪lane-3-imprint-test-quick-run-1.txt; 🧾lane-3-imprint-scope-note.txt |
| 3-heal | 3 | done | FROZEN | 🧪lane-3-heal-test-quick-run-1.txt (5 ok); 🧾lane-3-heal-scope-note.txt |
| 4-boolean | 4 | done | 6 passed; FROZEN | 🧪lane-4-boolean-test-quick-run-1.txt |
| 5-offset | 5 | done | 4 passed; FROZEN | 🧪lane-5-offset-test-quick-run-1.txt |
| 5-blend | 5 | done | 4 passed; FROZEN | 🧪lane-5-blend-test-quick-run-1.txt |
| 6-flip | 6 | pending | | |
| 7-harden | 7 | pending | | |
