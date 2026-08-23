🧪️ Verification record — ticket 26/08/23/END-TO-END-TESTING-REFACTOR
Every line below is a command that was actually executed and its actual output.

────────────────────────────────────────────────────────────────────────────────
1. Discovery (owner-root, compose-excluded in the library)
   $ bun ./📜️script.ts test discover
   test-framework-products-repo-modules-test-4e9a61-host-protocol-parity  …/🧪️test/🧪️tests/host-protocol-parity   [rust,typescript,go,python,dotnet]
   test-s-plugins-stdio-artifacts-pdf-fc3e39-create-minimal-pdf           …/📄️pdf/🧪️tests/create-minimal-pdf      [rust]
   test-s-plugins-stdio-artifacts-pdf-fc3e39-edit-existing-pdf            …/📄️pdf/🧪️tests/edit-existing-pdf       [rust]
   [discover] 3 test case(s)

2. Contract phase
   $ bun ./📜️script.ts test contract
   0 high-priority breach(es) across 0 rule(s)      exit 0

3. Five-language host conformance + pairwise parity
   $ bun ./📜️script.ts parity quick        (in the testing domain)
   [test] level=quick cases=1 executed=15 passed=15 failed=0 errored=0 parity=30/30

   The 30 comparisons are C(5,2)=10 implementation pairs × 3 scenarios. Projections
   are byte-identical across five INDEPENDENTLY written SHA-256 + fixture-resolver
   implementations (hand-written Rust, Go crypto/sha256, Node crypto, Python hashlib,
   .NET System.Security.Cryptography):
     rust:       {"vectorDigest":"7087e4e006b72ee5b808b9f622732600","literalDigest":"96e272947bd23fe166b7da2ff2269e41",…}
     typescript: {"vectorDigest":"7087e4e006b72ee5b808b9f622732600","literalDigest":"96e272947bd23fe166b7da2ff2269e41",…}
     go:         {"vectorDigest":"7087e4e006b72ee5b808b9f622732600","literalDigest":"96e272947bd23fe166b7da2ff2269e41",…}
     python:     {"vectorDigest":"7087e4e006b72ee5b808b9f622732600","literalDigest":"96e272947bd23fe166b7da2ff2269e41",…}
     dotnet:     {"vectorDigest":"7087e4e006b72ee5b808b9f622732600","literalDigest":"96e272947bd23fe166b7da2ff2269e41",…}

4. Nx per-case project generation and execution
   $ bun nx show project test-framework-products-repo-modules-test-4e9a61-host-protocol-parity --json
   targets: lint, test, test-contract, test-exhaustive, test-long, test-oracle,
            test-parity, test-quick, test-subject
   tags:    type:test, owner:…/🧪️test, impl:🦀️component.rs, impl:🟦️component.ts,
            impl:🐹️component.go, impl:🐍️component.py, impl:🔷️component.cs
   $ bun nx run test-framework-products-repo-modules-test-4e9a61-host-protocol-parity:test-quick
   [test] level=quick cases=1 executed=15 passed=15 failed=0 errored=0 parity=30/30
   NX   Successfully ran target test-quick

   $ bun nx show projects            212 projects, 3 of them generated test cases
   No hand-authored 📋️project.json exists for any test case.

5. Platform self-tests
   $ bun test ./🧪️index.test.ts
   31 pass  0 fail  522 expect() calls

6. Toolchains (a missing tool fails setup, never a skip)
   $ bun ./📜️script.ts test doctor
   typescript: 1.3.14 | rust: cargo 1.99.0-nightly | go: go1.25.0 darwin/arm64
   python: Python 3.9.6 | dotnet: 10.0.300

7. Oracle chain (pdf-writer creation + lopdf editing, both projected by the
   independent lopdf reader)
   create      -> {"version":"1.7","pageCount":2,"pages":[{"mediaBox":[0,0,595,842],…},{…}],"metadata":{"title":"Original Title","author":"Original Author"},"parsedByIndependentReader":true}
   metadata    -> {…,"metadata":{"title":"Replaced Title","author":"Replaced Author"},…}
   delete-page -> {"version":"1.7","pageCount":1,…}

8. Dependency gates
   $ bun ./📜️script.ts verify dependencies
   baseline: 206 third-party dependenc(y/ies); current: 206.  clean — no new dependencies.
   $ bun ./📜️script.ts dependency          (in the testing domain)
   [dependency] ecosystems=4 entries=206 production-reachable=154 test-oracle=2
   [dependency] test-oracle rust:lopdf@0.44 (lopdf)
   [dependency] test-oracle rust:pdf-writer@0.15 (pdf-writer)
   Baseline by ecosystem: go=60 js=66 python=15 rust=65 dotnet=0
   Baseline by class: production-runtime=107 production-build=60 repository-tooling=40
                      test-runner=18 test-oracle=2
   Both oracles are productionReachable=false.

9. Clean safety
   $ bun ./📜️script.ts clean test --dry
   removals=25 files=114 bytes=496698   (work 9, hosts 6, results 9, reports 1)
   skipped-unmarked = 0; protected .🧬semio/🦑️repo/⚡️cache
   Self-test plants a sentinel file in an UNMARKED sibling directory and asserts it
   survives; asserts every removal path starts with .🧬semio/🦑️repo/⚡️cache/tests/
   and contains no compose/ segment; asserts markOutputDir() outside the cache throws.
   $ bun ./📜️script.ts clean coverage --dry   → 5 coverage roots, nothing else.

10. Launch configuration
   .vscode/launch.json now parses (274 configurations). A PRE-EXISTING structural
   corruption at HEAD — the "🧹clean" entry spliced into the middle of
   "⚖️gate🦀️zero-warnings🖥️native" — was repaired; both entries are restored.
   14 new 🧪️test… entries in group 4_gate.

────────────────────────────────────────────────────────────────────────────────
BLOCKED, NOT BY THIS WORK
   The PDF pilot's SUBJECT phase cannot compile. `semio-s-plugin-stdio` (the owner of
   decode_pdf/encode_pdf) transitively needs `semio-framework-os-kernel`, which fails
   with 179 errors:

     🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
       → 162 references to `semio_framework_job::`
     🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml
       → does NOT declare semio-framework-job

   Introduced by commit 9d7cabfd9c (2026-08-23 10:55:52 +0200), i.e. a concurrent
   session's in-flight refactor, two minutes after this ticket opened. It fails in any
   workspace, not only in the generated host's. The pilot's oracle chain is verified
   working (section 7); both PDF cases go green as soon as that manifest edge lands.
