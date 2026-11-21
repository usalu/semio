# Tools Hard-Cap Grace Decision Evidence

Ready records a closed `toolsGraceDenial` code when the opted-in stop is a physical tools hard-cap timeout. The grace decision uses the existing eligibility check, projected clock, ordinary progress age, protected sample, and counter series. The code identifies the first gate that blocked continuation at the existing decision point.

- `not-eligible` records that the flight or window failed the existing eligibility checks.
- `budget-exhausted` records the end of the absolute fifth interval.
- `progress-quiet` records that the ordinary progress clock exceeded its quiet window.
- `sample-missing`, `sample-unqualified`, and `sample-stale` distinguish the observer's available checkpoint state and its freshness.
- `series-unqualified` records a counter reset or mismatched current series. `series-needs-advances` records the absence of enough fresh verified increases.

The code uses the existing atomic stop record and the same exact inflight custody as the deadline verdict. Its closed schema covers the durable inflight record and the mirrored terminal timing. Adoption reuses a valid terminal record on the same path that recovers the deadline. The field contains only one enum value. Live sampling, deadlines, grace limits, and I/O cadence retain their existing settings.

The test-first private tools flight reached its absolute maximum and reproduced a missing reason. The regression exercises real protected standing, normal observer adoption, the hard cap, exact stop data observed by the runner on SIGTERM, terminal receipt recovery, and its private mirrored timing. SQLite independently models decision precedence and boundaries. Ajv verifies the closed enum, valid scope, and invalid scope rejection. The final run passed 15 tests and 469 assertions, including adjacent grace and stage-completion contracts. The exact transcripts are `🗑️generated/ready-tools-denial-red.log` and `🗑️generated/ready-tools-denial-green.log`.

The historical m02 refusal remains undetermined. Its older mirror has no grace decision evidence. No live workers were operated.
