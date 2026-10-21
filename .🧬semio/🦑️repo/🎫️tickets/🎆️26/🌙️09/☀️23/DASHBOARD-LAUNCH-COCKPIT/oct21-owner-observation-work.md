# Owner Observation Work

Read-only latency audit found two local cargoTargets invocations in one Cargo owner observation. This is a local manifest probe and three target DTOs, not a whole-workspace Cargo walk. Current code captures that result once and reuses it at the same merge boundary. Authored targets and component targets retain precedence; physical source reads, selected/base admission, revisions, generated dependencies and fresh-attempt rechecks remain.

Four schema-first neutral cases exercise the complete actual discovery function through explicit observation ports, count one local target construction and one physical declaration read, compare files/dependencies/dependency-inputs, and check merge precedence independently with SQLite. Source0 mounts the law. Low accepted source; actual bodies and speedup remain unverified.

The repeated inference passes in80585 corresponded to actual observed-fact invalidations. Registry index allocation was evaluated but deferred: the admitted registry is cloned rather than frozen, and a mutable index cache would add an unmeasured second invalidation contract. No blind cache or source-epoch reuse was added.
