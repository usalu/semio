# Test Layout Audit Closure Manifest

This manifest records the durable, read-only audit inputs and reports retained for review. The temporary output paths listed below contain only transcribed compiler, scanner, and test-run captures owned by this audit. They are removed after this manifest is written. No shared generated lane is included.

~~~json
{
  "retainedInputs": [
    "🧑‍💻final-layout-audit/📜️script.ts",
    "🧑‍💻rust-module-path-oracle/📜️script.ts",
    "🧑‍💻rust-module-path-oracle/🏛️default-non-mod/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️default-non-mod/outer/🧪️tests/🔬️default/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️dot-parent/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️dot-parent/🧪️tests/🔬️dot/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️empty-parent/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️empty-parent/🧪️tests/🔬️empty/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️mixed-absent/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️mixed-absent/reassigned-scope/🧪️tests/🔬️mixed-absent/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️mixed-present/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️mixed-present/named-scope/🪪️physical-directory",
    "🧑‍💻rust-module-path-oracle/🏛️mixed-present/reassigned-scope/🧪️tests/🔬️mixed-present/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️named-parent/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/🏛️named-parent/renamed-scope/🧪️tests/🔬️named/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/📦️crate/lib.rs",
    "🧑‍💻rust-module-path-oracle/📦️crate/🏛️owner/🦀️.rs",
    "🧑‍💻rust-module-path-oracle/📦️crate/🏛️owner/outer/🧪️tests/🔬️imported/🦀️.rs"
  ],
  "retainedReports": [
    "📓️test-layout-final-runner-audit-2026-09-08.md",
    "📓️rust-inline-module-compiler-oracle-2026-09-08.md",
    "📓️test-layout-final-current-audit-2026-09-08.md",
    "📓️test-layout-layering-baseline-audit-2026-09-08.md",
    "📓️test-layout-audit-closure-manifest-2026-09-08.md",
    "📓️test-layout-final-closeout-audit-2026-09-08.md"
  ],
  "removedGeneratedOutputs": [
    "🗑️generated/audit-terra/final-layout-bun-nx-2026-09-08.log",
    "🗑️generated/audit-terra/final-layout-source-scan-2026-09-08.json",
    "🗑️generated/rust-inline-module-oracle/"
  ]
}
~~~

The final-current report transcribes the focused source audit and runner samples. The compiler-oracle report transcribes all compiler outcomes and traversal totals. The layering report carries the source-origin evidence and post-edit verification.
