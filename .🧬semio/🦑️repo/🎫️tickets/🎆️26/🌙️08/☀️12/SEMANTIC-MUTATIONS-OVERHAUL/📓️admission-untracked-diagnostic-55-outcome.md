# Exact Untracked Git Diagnostic 55

## Outcome

The one authorized Git call returned successfully in 4.405 seconds: 53,426 bytes and 320 NUL-framed UTF-8 records. The actual extracted `sourceAdmissionUntrackedPaths` helper threw `Git untracked output has an invalid source path`.

Exactly three records failed the actual `sourceAdmissionSafePath` predicate. Each fails only because its terminal slash creates an empty final segment. No other unsafe-path predicate matched any record. The independent reason classification agreed with the actual source predicate for all 320 records. No returned candidate was read, statted, followed, trimmed, filtered, or opened.

This identifies directory-shaped Git records, not evidence of unsafe filenames. Their names coincide with the isolated physical-integration fixture roots reported by the parent. This diagnostic deliberately does not verify their nested repository contents or physical type. It is an instrumented original-loader diagnostic, not an unmodified public-wrapper replay, complete admission roster, or production release.

## Exact Rejected Records

Offsets and lengths are raw UTF-8 byte positions excluding the terminating NUL.

| Record Index | Byte Offset | Byte Length | Exact Decoded Record |
| --- | --- | --- | --- |
| 20 | 3168 | 169 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-physical-integration-53/🧫️run-CtgJYd/🧪️fixture/` |
| 23 | 3677 | 169 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-physical-integration-53/🧫️run-Sjc4gE/🧪️fixture/` |
| 26 | 4186 | 169 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-physical-integration-53/🧫️run-o21R2r/🧪️fixture/` |

For all three: empty final segment index `10`; no leading slash, drive prefix, backslash, control character, lossy UTF-8 roundtrip, or dot/dot-dot segment. Original bytes equal UTF-8 re-encoding of the decoded text. Complete hexadecimal values and escaped strings are retained in the receipt. The terminal bytes are `666978747572652f` (`fixture/`).

## Exact Instrumentation

The controller captured current N, D, taxonomy, and itself with lexical any-case Compose exclusion before traversal, nofollow ancestry checks, descriptor validation, and SHA-256 fingerprints. TypeScript AST extraction selected the actual `sourceAdmissionSafePath`, `sourceAdmissionGitRecords`, `sourceAdmissionGitExclusions`, `sourceAdmissionUntrackedPaths`, and their actual `sourceAdmissionByteCompare` dependency.

N was exposed only in memory by appending a private-loader export and module-identity witness. Its original module URL, file path, and import directory were verified unchanged; the original file was not edited. The actual N `loadTaxonomy` consumed bytes equal to the captured taxonomy. It yielded exactly `compose` and `temp/compose` exclusions. The controller called the actual exported `taxonomyScopedGitPathspec(undefined, ["compose", ...loaded.exclusions.map(value => value.path)])`; no D-loader substitute or handwritten fallback exclusion list was used.

The extracted helper retained its source behavior. Only its `execFileSync` binding was instrumented to preserve command arguments and returned bytes, with an explicit 30-second timeout. Exactly one real Git invocation occurred:

```json
{
  "command": "git",
  "args": [
    "ls-files",
    "--others",
    "--exclude-standard",
    "--exclude=[cC][oO][mM][pP][oO][sS][eE]",
    "--exclude=/compose",
    "--exclude=/temp/compose",
    "-z",
    "--",
    ".",
    ":(exclude,top,literal)compose",
    ":(exclude,top,literal)temp/compose",
    ":(exclude,icase,glob)**/compose",
    ":(exclude,icase,glob)**/compose/**"
  ],
  "originalOptions": {
    "cwd": "/Users/ueli/Documents/semio",
    "encoding": "buffer",
    "maxBuffer": 268435456
  },
  "effectiveOptions": {
    "cwd": "/Users/ueli/Documents/semio",
    "encoding": "buffer",
    "maxBuffer": 268435456,
    "timeout": 30000,
    "killSignal": "SIGKILL"
  }
}
```

Git returned exit 0 without signal or timeout. Its raw stdout SHA-256 is `fde3be9741e4432e86de0e782e592035eaab59574c9ee26b33fc115c924d61c9`. No successful stderr capture is claimed: the synchronous API returned stdout only.

## Source Endpoints and Deliberate Incomplete Receipt

| Input | First SHA-256 | Final Observation |
| --- | --- | --- |
| controller | `6abe63e12f47256cdad74d819de94f00aafe948218a6e72be99b4364dcbf82bf` | Final skipped: source also returned as an untracked candidate |
| N | `0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce` | Exact first/final match |
| D | `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423` | Exact first/final match |
| taxonomy | `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce` | Exact first/final match |

N, D, and taxonomy had exact descriptor and content matches before import, before Git, and after Git. The controller matched before import and before Git, but Git also returned its newly authored path. Its final filesystem recapture was therefore skipped to obey the no-candidate-probe boundary.

Accordingly the controller exited 1 and the raw receipt records `captureComplete:false` and `sourceStable:false`. This is an unavailable controller endpoint, **not an observed source change**. There was no retry, second Git invocation, full inventory rerun, or source restoration.

Additional instrumentation hashes:

- Appended export text: `df9881c60fc730507875d70ecff032f1347dced76c04a4b348f1ff8812ae51e2`.
- In-memory N plus appended text: `f9c6fecb35b395e84e2c46a3878fb87c752d2dd34e520953150fc7b4310944cc`.
- Original N module load count: 1; identity preserved: true.

The historical failed roster54 taxonomy remains its historical value. This newly executed diagnostic used actual taxonomy `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce`; it was not pinned or restored.

## Retained Evidence

- Newly authored controller: [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/📜️script.ts).
- Complete structured receipt: [🔣️receipt.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/🔣️receipt.json).
- Run Markdown with complete receipt: [📓️receipt.md](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/📓️receipt.md).
- Unique complete sibling receipt: [ticket receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55-2026-08-27T22-56-13-885Z-🧫️run-SOCd6O.md).
- Exact returned bytes: [🔣️git-stdout.bin](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/🔣️git-stdout.bin).
- First captured source bytes are retained in the run as `🧬️controller.source.txt`, `🧬️N.source.txt`, `🧬️D.source.txt`, and `🧬️taxonomy.source.txt`; no post-Git candidate-source reads were performed to prepare this report.

This evidence is newly executed after the earlier unrelated admission-owner loss. Nothing was recreated from missing run directories.

## Boundary for Parent Review

The demonstrated mismatch is between Git's returned terminal-slash directory records and a strict file-path predicate that rejects every empty segment. Production policy and fixture changes remain parent-owned: no record was silently normalized, dropped, or admitted. An exact future regression should preserve these bytes and distinguish the enumeration-level directory marker from malformed file paths; the diagnostic does not authorize loosening `sourceAdmissionSafePath`.

No N, D, S, P, canonical fixture, launch, or production source edits were made. No Cargo/native test or content/mutation completeness claim is made. The bounded one-call task is complete, with the endpoint limitation retained explicitly.

