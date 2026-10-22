# Native Manifest Parser Boundary

The current native-manifests MJS owner lazily resolves existing @iarna/toml plus its package version through createRequire. readToml uses the parser-version namespace in memoizedFileRecord and retains observed Source SHA, byte length, bounded source line and underlying parse cause on refusal. Any replacement must preserve valid Cargo TOML and refusal semantics, parser identity freshness and actual Node/Bun consumers. No production Source was changed during this inspection.

Current [Bun TOML documentation](https://bun.com/docs/runtime/toml) describes the newer full-conformance parser. The official [Bun1.4 breaking-change record](https://github.com/oven-sh/bun/issues/28792) explicitly describes the previous parser accepting duplicate keys and rejecting valid date/time values. The pinned repository runtime is1.3.14, so using its global TOML parser as a blind replacement would not preserve the existing manifest boundary. Do not infer1.3.14 behavior from current1.4documentation or upgrade pins as a hidden gate bypass.

A bounded follow-up must establish an existing first-party conformant parse port or a proper schema-first implementation and all consumers. A minimal dependency command kernel should meanwhile avoid importing the full graph/native inference subtree when that command does not use it. The current1172-file syntax projection is conservative DATA, not actual Bun scanner/runtime admission. No actual parser, compiler or subprocess was executed for this research.

## Current Public Pin Qualification

Fresh root package read during currentlock shows packageManager bun@1.4.2, engines.bun>=1.2.0 and node24.15.0. The earlier description of1.3.14 as the workspace pin is superseded;1.3.14 is the observed ambient/bootstrap version from the original failure and peer isolated probe. Current managed executable identity has not been independently executed here. Do not apply the older-version parser inference as a claim about1.4.2. No parser change was made, and the separate Node/Nx MJS compatibility/interface requirement still needs actual proof before replacing the existing narrow TOML owner.
