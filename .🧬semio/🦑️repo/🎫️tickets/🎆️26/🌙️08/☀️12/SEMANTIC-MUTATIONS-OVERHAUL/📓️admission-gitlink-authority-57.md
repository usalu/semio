# Gitlink Admission Authority Audit

## Decision

The current rejection matches the existing admission semantics. It is not contradicted by an existing schema-owned Gitlink exception. The canonical schema accepts `160000` as an index fact so it can be retained; the current projector and authored neutral fixture explicitly reject its admission as authored source.

For a complete repository-observation roster, the appropriate future design is an explicit, index-authorized, retained terminal Gitlink boundary—not an ordinary authored directory. That requires a reviewed schema/consumer change. Keep the current rejection until that contract and its tests exist. Do not demote the diagnostic, add a path exclusion, change the index, or use the observed path as an allowlist entry.

## Verified Current Authority

Read-only calls to the actual D fixed-contract matchers, with the actual captured taxonomy supplied explicitly, returned:

| Supplied Path String | Existing Fixed Contract |
| --- | --- |
| `♻️mit-bestand/recherche` | None |
| `♻️mit-bestand/recherche/.git` | `nested-git-metadata` |
| `.gitmodules` | `gitmodules` |

These were in-memory path matches, not filesystem probes. Current D taxonomy validation returned no problems.

The taxonomy's `gitmodules` contract at line 8923 governs the root filename used for submodule discovery. Its `nested-git-metadata` contract at line 9254 governs names matching `**/.git`, specifically retained repository metadata. D's `FixedDirectoryContract` and matching code (lines 494 and 2237–2289) define naming/scope authority, not authored-source admission or a foreign repository root classification. The metadata contract does not match the Gitlink root itself and does not license its descendants. No root or nested `.gitmodules`/`.git` content was opened.

The current canonical source-admission schema has no boundary/disposition member. An actual Ajv check rejects adding `repositoryBoundary` to the retained observation as an unknown property. Thus a boundary cannot honestly be introduced solely by changing the diagnostic or the top-level status.

The registered `mit-bestand` member list also has no `recherche` entry, but this is not used as the rejection reason or as a proposed admission test. Authority must be mode/identity-driven, not a hardcoded path list.

## Actual Observed Tuple and Current Pure Behavior

The retained roster receipt has exactly one completed observation, schema-valid but rejected; the second attempt was cooperatively cancelled, with no stable pair. Its only blocking diagnostic is `nonregular-node`. The target tuple is:

```json
{
  "sourcePath": "♻️mit-bestand/recherche",
  "observedKind": "directory",
  "worktreeMode": "040000",
  "explicitDirectory": true,
  "origins": [
    "tracked"
  ],
  "indexEntries": [
    {
      "mode": "160000",
      "objectId": "92036c7ca0149b43ddea28db8c8e516f983fe718",
      "stage": 0
    }
  ],
  "generatorOutputs": []
}
```

A read-only call to the actual N projector, using this tuple with `unsafeAncestor:false` and empty unrelated scope/generator inputs, reproduced exactly the retained tuple plus the existing Gitlink rejection. Both the input and result passed the current actual JSON Schema through Ajv. This is a single pure diagnostic check, not another filesystem observation.

The existing neutral case `inconsistent-physical-facts-and-gitlink-are-both-rejected` was also invoked unchanged and matched its authored expectation exactly. It deliberately contains an invalid physical `file/160000` tuple; that negative must remain rejected. A future valid boundary case must use actual physical `directory/040000/explicitDirectory:true`, with `160000` only in the Git index entry.

## Exact Gaps a Boundary Contract Must Close

1. **N2443–2504 projector:** line 2482 rejects every Gitlink index entry as `nonregular-node`. The current result retains the physical and index facts but has no non-authored boundary variant.
2. **N2772–2798 walker:** it stops at a matching `.git` metadata directory only. It does not receive the full index-derived Gitlink boundary set. An explicit-ticket or declared ignored-generator walk overlapping a Gitlink root can therefore enumerate that root before the final projector rejects it. This is source evidence; no such traversal was executed in this audit.
3. **N2815–2857 collector:** the full stage-aware index is already available before the two explicit walks. That is the existing place to derive exact terminal boundaries from unambiguous stage-zero `160000` identities and carry them to the existing walk. No new filesystem roots or nested Git invocation is needed.
4. **N6057–6066 full-inventory consumer:** it treats every successful physical directory observation as an authored `CandidatePath`. Merely making the Gitlink diagnostic nonblocking would classify `recherche` as an authored directory and feed canonical-directory/planning logic. The boundary must remain represented in the result and explicitly excluded from authored classification/decoding—not silently discarded from the roster.
5. **N310–340 and canonical source-admission schema:** introduce a closed boundary representation before code. Preserve observed kind, worktree mode, directory flag, all origin/index identities, conflicts, and no-state facts. A Gitlink is an index ownership boundary, not a new physical filesystem kind.

The preferred schema direction is a distinct retained-boundary record/tag, derived from the exact Git-index identity and consumed explicitly by full inventory. The final API field placement should be fixed in that schema packet; the temporary unknown-property Ajv probe above is evidence of the missing current contract, not a proposed compatibility field.

## Exact First Neutral Test to Author

Author `stage-zero-gitlink-directory-is-retained-terminal-boundary` in the existing canonical admission owner, using the exact tuple above as input. Its proposed semantic expectation is:

```json
{
  "classification": "retained-gitlink-boundary",
  "authoredSource": false,
  "traversal": "terminal",
  "observation": {
    "sourcePath": "♻️mit-bestand/recherche",
    "observedKind": "directory",
    "worktreeMode": "040000",
    "explicitDirectory": true,
    "origins": [
      "tracked"
    ],
    "indexEntries": [
      {
        "mode": "160000",
        "objectId": "92036c7ca0149b43ddea28db8c8e516f983fe718",
        "stage": 0
      }
    ],
    "generatorOutputs": []
  },
  "effects": {
    "nestedLstat": 0,
    "nestedReaddir": 0,
    "nestedContentRead": 0,
    "authoredDirectoryClassification": 0,
    "authoredLeafDecode": 0
  }
}
```

This is a **proposed neutral expectation**, not a schema currently accepted by production and not a passing test claim. Fix the canonical boundary schema first, validate it independently with Ajv, then execute the actual projector/collector and exact full-inventory consumer against it. The current implementation must produce a genuine RED: it has no boundary representation and rejects the directory.

The case must prove all of the following together:

- The same physical/index tuple and object ID survive; no fake file mode, fabricated directory, dropped origin, or lost index identity.
- The record is visibly non-authored and terminal. A completed admission result is justified by that typed distinction, not by deleting a diagnostic.
- A virtual existing-authority walk under an already admitted parent stops at the index-proven Gitlink before any child `lstat`, `readdir`, or content read. Exercise both existing ignored-generator and explicit-ticket joins; expected nested-call counts are zero.
- The exact full-inventory consumer retains boundary information while making zero authored directory-classification, leaf-decoding, or normalization/planning calls for the boundary.
- Repeating the same index/physical facts under a different safe path gives the same boundary semantics; this prevents a path exception.

Required adjacent negatives are: missing stage-zero authority, nonzero/conflicting stages, contradictory duplicate index identities, file/symlink/nonregular physical targets, and unsafe/unobserved targets. Keep the current artificial `file/160000` negative unchanged. Preserve the existing 12 strict physical-path cases and all projector no-state rules. In particular, absence remains `absent/null/false`, and unsafe observation remains `unobserved/null/false`; neither becomes a present directory. The first positive is intentionally limited to an actually observed directory. Any later acceptance of index-only absent Gitlinks needs its own explicit schema rule rather than inference from this test.

## Evidence and Limits

- [Roster outcome](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-current-roster-56-outcome.md).
- [Complete retained roster receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-current-roster-54/🧫️run-1KTP5R/🔣️receipt.json).
- Current N, D, taxonomy, canonical schema, and canonical vector inputs matched exact first/final guarded fingerprints during the pure audit.
- The 170,130,121-byte receipt was read with nofollow/descriptor validation and hashed; it was not regenerated.
- No candidate path was statted, opened, followed, or traversed. No nested repository command, full/global rerun, shared index change, new root, skip list, status coercion, or production/test/controller edit occurred. Only this audit Markdown was authored.

| Captured Input | SHA-256 |
| --- | --- |
| N | `34ca6ab7cdf9bee2738766d88d463be76541c405666f52fe6a59c272e3a9588f` |
| D | `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423` |
| taxonomy | `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce` |
| schema | `ed117e588c2aa2e1a0622455ab4710cec623725dce829af6ddb4a5f6328bb1a5` |
| canonical | `14a4b2cf5adcb8d09bf0cb481c7c693be1d3a817a81b28431edfae91ab2cdf91` |
| receipt | `8e345da58dc0669c0075c0927b6e376c9a327572d774602ad1f950bd26d1be1e` |

The present roster rejection remains authoritative. This audit recommends the smallest schema-first boundary feature and its required tests; it does not authorize or claim that feature has been implemented.

