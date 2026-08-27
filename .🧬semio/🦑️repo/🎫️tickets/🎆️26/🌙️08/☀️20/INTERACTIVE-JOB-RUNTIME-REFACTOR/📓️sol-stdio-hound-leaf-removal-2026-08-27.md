# Sol Stdio Hound Leaf Removal — 2026-08-27

## Result

The isolated stdio test-oracle no longer owns or calls `hound`. The WAVE capability now uses one dependency-free owned RIFF/PCM16 boundary that is independent from the stdio subject codec.

This was the smallest genuinely self-contained next leaf:

- exactly two Rust source owners called `hound`;
- its supported surface is bounded to integer PCM16 RIFF/WAVE;
- existing coverage includes canonical creation, retuning, a 96,000-sample real fixture, five exhaustive mutation rows, inverse properties, and identity round-trip;
- unlike DEFLATE or OBJ, replacing it did not require owning an unrelated compression or geometry grammar.

## Frozen language-neutral evidence

Added `🧫️stdio-wav-pcm16-oracle-golden-v1.json` beside this report. It freezes the pre-removal Hound 3.5.1 evidence:

- canonical mono PCM16 bytes for `[-32768, -1, 0, 1, 32767]`;
- the canonical semantic projection;
- SHA-256, dimensions, and projection SHA-256 for the committed 192,044-byte, 96,000-sample real fixture;
- projection SHA-256 values for `no-mutation`, `set-snapshot`, `set-fmt`, `set-data`, and `set-other-chunks`;
- eight hostile wire inputs covering magic, truncation, non-PCM encoding, zero channels, byte-rate and block-alignment disagreement, incomplete frames, and chunk overrun.

The dependency baseline was not edited.

## Owned implementation

Updated `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔊️audio/🦀️component.rs` with:

- owned `PcmWavFormat` and `PcmWav` types;
- checked RIFF size and chunk traversal;
- strict PCM format 1, positive channel/rate, 16-bit depth, byte-rate, block-alignment, complete-frame, duplicate-chunk, padding, and overrun validation;
- canonical little-endian PCM16 encoding;
- ordered opaque auxiliary-chunk preservation;
- four unit tests covering the frozen byte golden, hostile signed sample lanes, odd auxiliary-chunk padding, all eight malformed inputs, incomplete frames, and invalid fourcc values.

Updated the subset mutation oracle at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` so every mutation, inverse, identity rewrite, and projection goes through the shared owned boundary. The former duplicated Hound-plus-hand-splice path was removed.

The creation/retune and mutation adapters keep their existing repository interfaces. No compatibility API was added.

## Honest test contract

Removed both live Hound registry entries. The root contribution now records `frozen-hound-pcm16` as a no-oracle decision backed by:

- `language-neutral-goldens`;
- `owned-independent-boundary`;
- `hostile-wire-fixtures`.

The previously live differential retune and mutation scenarios are now conformance scenarios. Round-trip and inverse-property modes remain unchanged. This avoids representing first-party source as an external oracle distribution and avoids keeping a registry identity for a package that is no longer linked.

## Manifest and lock ownership

Updated `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml`:

- removed `dep:hound` from the `oracles` feature;
- removed `hound = { version = "3", optional = true }`;
- optional external oracle count is now 24.

Updated the crate-local Cargo lock:

- removed the `hound 3.5.1` package block;
- removed `hound` from the `semio-s-plugin-stdio-test-oracle` dependency array.

That lock is intentionally ignored by the repository's existing `**/📦️packages/**/Cargo.lock` rule; the on-disk generated lock is nevertheless internally consistent with the source manifest. No root lock or dependency baseline was rewritten.

## Direct Bun and source evidence

Passed:

```text
language-neutral PCM16 parity probe
  canonicalBytes: 54
  realSamples: 96000
  differentialRows: 5
  hostileCases: 8
  liveHoundOwnership: false

feature parser and decision coverage probe
  create/retune: round-trip, round-trip, conformance
  mutate: 5 conformance rows, 5 property rows, 1 round-trip
  both capabilities covered by frozen-hound-pcm16

optional manifest inventory
  24 identities
  hasHound: false

focused git diff --check
  exit 0

bun ./📜️script.ts verify dependencies summary --format json
  exit 0
```

Fresh dependency totals after this leaf:

| Measure | Before this leaf | After | Delta |
| --- | ---: | ---: | ---: |
| Rust literal external | 76 | 75 | -1 |
| JavaScript corrected external | 66 | 66 | 0 |
| Total raw identities | 162 | 161 | -1 |
| Total literal external identities | 157 | 156 | -1 |
| Production-reachable identities | 107 | 106 | -1 |

The summary reports zero oracle conflicts, zero toolchain conflicts, and zero unauthorized toolchain rows.

## Deferred evidence and blocker

No Cargo, Nx, rustfmt, or modifying Git command was run. Rust compilation and the four new unit tests remain unverified until the coordinator explicitly releases the Store compiler lease. A focused Bun registry suite entered the repository's full contribution-discovery traversal; its result is not claimed here unless it completes with an observed exit status.
