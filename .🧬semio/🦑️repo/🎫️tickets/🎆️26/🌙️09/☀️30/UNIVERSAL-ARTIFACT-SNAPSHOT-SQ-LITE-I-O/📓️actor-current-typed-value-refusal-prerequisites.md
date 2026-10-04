# Actor Current Typed Value Refusal Prerequisites

Root’s actual root-authentic-csv9.log compiler diagnostics identify 20 errors from 16 real constructor sites: 12 in Actor lifetime and four in Actor root (format! constructor sites each produced both E0308 and E0061). These are preassert compiler prerequisites, not SQLite feature failures.

Each actual site is an ordinary typed input-shape/value refusal: missing or unexpected object/field/variant, noncanonical lifecycle generation, invalid request sequence or invalid ActorId/ShardId map key. Each now explicitly constructs InvalidValue from the canonical semio_framework_value authority. Existing message prose and under(field) propagation are unchanged. No generic prose classifier, default refusal kind or compatibility constructor was introduced. Both files already have the real direct Value dependency.

Both changed files passed rustfmt parser-only readback. No Cargo or native runtime test was launched by this executor; Root remains sole native verification authority.
