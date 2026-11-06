# Hub Escrow Dependency Recheck

Root tool60548 actually ended with exit1 during cargo fetch, before the Hub test suite: job-context inherited semio-framework-job-return-escrow-publication was absent from the workspace dependency table at that invocation. The final Cargo diagnostic is retained in generated/hub-native-dependency.log.

Root inspected current source rather than replaying an old repair. Framework Cargo.toml now already declares that key with the canonical job outcome/publication/escrow package path. Two independent existing TOML parsers agree on the job-context manifest, workspace root and target manifest, and the target package name is exactly semio-framework-job-return-escrow-publication. This concurrent repair was preserved; Root made no duplicate source edit.

A normal-admitted complete Hub manifest-closure audit46721 is running with generated/hub-closure-current.log. One fresh Hub suite87775 uses the same semio-hub-remodel --lib scope with normal requested one job; output generated/hub-escrow-current.log. No earlier failed attempt is counted as a pass, and no healthy peer or native job was canceled. Actual closure and native verdicts remain required.

Actual complete closure46721 completed exit0:154 members,3692 transitive manifests,two independent parsers,empty counts and issues. Root independently inspected the retained output. This proves current inherited keys, paths and package names across that audited closure; it is not Cargo compilation or a Hub native test pass. Fresh Hub87775 remains active.
