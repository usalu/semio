# Gitlink 58 Source-Admission Splice Review

## One Concrete Finding

[Projector lines 2484–2485](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:2484) test `!inScope(...)` before diagnosing `repository-boundary-descendant`. The fence set is derived before scope filtering, but a supplied contradictory descendant outside the selected scope is silently omitted.

The agreed contract says supplied descendants below an index boundary are contradictory admission and must be diagnosed rather than silently filtered. This source ordering leaves that obligation uncovered when the requested scope is unrelated.

Handcrafted input:

```json
{
  "scope": "owned",
  "opaquePrefixes": [],
  "generatorOutputRoots": [],
  "candidates": [
    {
      "sourcePath": "foreign/module",
      "observedKind": "directory",
      "worktreeMode": "040000",
      "explicitDirectory": true,
      "origins": [
        "tracked"
      ],
      "indexEntries": [
        {
          "stage": 0,
          "mode": "160000",
          "objectId": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
      ],
      "unsafeAncestor": false
    },
    {
      "sourcePath": "foreign/module/file.rs",
      "observedKind": "file",
      "worktreeMode": "100644",
      "explicitDirectory": false,
      "origins": [
        "nonignored-untracked"
      ],
      "indexEntries": [],
      "unsafeAncestor": false
    }
  ]
}
```

By direct source control flow, the current function reaches an empty `complete` result: `owned` is not inside a fence, neither candidate is in scope, no group or descendant diagnostic is created, and the final diagnostic list is empty. Under the agreed no-silent-descendant rule it must reject with `repository-boundary-descendant` for `foreign/module/file.rs`; it may still keep the scoped observation list empty.

This is **source evidence, not an additional executed counterexample**. The approved actual30 replay did run and passed; none of those 30 cases combines an unrelated scope with supplied descendants. No extra subject run or production change was made during this review.

Minimal root-owned correction: perform the descendant consistency diagnosis for every safe supplied candidate before the scope filter. Add a neutral scoped-out descendant regression before the correction, plus an ordinary scoped-out row control. Preserve physical spelling and scoped observation projection. A full-inventory or apply change is not needed to address this specific pure gap.

## Remaining Bounded Review

No other premature candidate probe was identified in the inspected admission splice under its captured-index contract:

- `sourceAdmissionPrepareOptions` runs lexical/root-ancestry checks, captures index rows and derives conservative 160000 fences before taxonomy-file observation. Scope/ticket allow exact boundary roots but reject descendants; taxonomy/cancellation reject exact roots as well.
- `sourceAdmissionGitRows` keeps stage, mode and OID from strict NUL/fatal UTF-8 framing. Every 160000 row fences traversal, including nonzero/conflicting stages.
- The private prepared value carries one captured index authority to the collector. `inventoryTaxonomySources` does not independently re-enumerate index membership.
- `sourceAdmissionCheckCancellation` guards the path before its lstat.
- The collector checks every declared generator output root against the fences before its walks. It guards admitted paths before their final physical observation.
- `sourceAdmissionWalk` checks a path against fences before lstat and stops at an exact fence before readdir. Absent targets are not fabricated into directories; final index-driven observation supplies the true absent tuple.
- `sourceAdmissionUntrackedRows` adds fence paths to both Git exclusion patterns and literal pathspec exclusions; strict byte framing, single terminal directory-marker handling and raw spelling are preserved. The collector retains the marker check against the actual final directory observation.
- The pure tag requires a single deduplicated stage-zero160000 index record, tracked origin, a consistent safe physical tuple, and actual directory or absence. Mixed stages, contradictory identity, file/symlink/other/unsafe/unobserved remain null-tagged and rejected. Existing opaque/unsafe physical suppression remains present.
- NFC is used for containment/scope comparison; source observations still group by raw path. The reviewed NFD/NFC scope and raw-distinct cases passed in actual30.

These are read-only call-order and predicate findings, not fresh IO execution or proof of Git's filesystem traversal behavior. Root's IO/59 laws remain the authority for those runtime obligations. This review deliberately excludes generic future plan/apply work.

## Source Binding

The review started at whole N `aece45f7980f07b393f23e2b0b3cacf7cd1aa8d857d2a63998f7361410a703be` and ended at `d6922221a330e285cbc31232a90e30ece0991d08e90d904598cb352267585a2a`. All 24 selected source-admission declarations were checked by TypeScript AST name and exact declaration-text SHA-256 and were unchanged; whole-N drift is separate from this slice review.

Projector declaration remained `df769355c0b1be62b002cdfd5ef55deb7c791927247b6b496f861981ee024460`. Initial and endpoint reads used lexical Compose exclusion, complete nofollow ancestry and O_NOFOLLOW/fstat identity checks.

```json
{
  "first": {
    "device": "16777230",
    "inode": "129693956",
    "mode": "33188",
    "size": "901708",
    "modifiedNs": "1787875238237201097",
    "changedNs": "1787875238237201097",
    "sha256": "aece45f7980f07b393f23e2b0b3cacf7cd1aa8d857d2a63998f7361410a703be",
    "bytes": 901708
  },
  "endpoint": {
    "device": "16777230",
    "inode": "129693956",
    "mode": "33188",
    "size": "901994",
    "modifiedNs": "1787875447848472880",
    "changedNs": "1787875447848472880",
    "sha256": "d6922221a330e285cbc31232a90e30ece0991d08e90d904598cb352267585a2a",
    "bytes": 901994
  },
  "declarations": [
    {
      "name": "sourceAdmissionSafePath",
      "sha256": "62dd9e376e3656cee2fe2bfae51f39174d15e9bebab7fbc5f1185338d089a9fd",
      "stable": true
    },
    {
      "name": "sourceAdmissionOpaque",
      "sha256": "afdcf174a6931f5c4d6e51dc0a67fe8d5fe7f19d515b57f1fd9352e2de88b34b",
      "stable": true
    },
    {
      "name": "sourceAdmissionRecord",
      "sha256": "be299c61fd5208e526774227071a278d4af1797d01a25d0977ed06dfe7e95cdc",
      "stable": true
    },
    {
      "name": "sourceAdmissionInputShape",
      "sha256": "4f5cb5d528e2fc771be1fa5676e4d5b7dc2154e64affa14ab10b65729e576ee2",
      "stable": true
    },
    {
      "name": "sourceAdmissionPhysicalConsistent",
      "sha256": "89828695ff549ec10220674921e3cae7a6bf5911056eccff5b27fc8f2b9ed844",
      "stable": true
    },
    {
      "name": "sourceAdmissionRepositoryFences",
      "sha256": "e0c2d146b9c6feab777b039b15779c3a8887fe0aa3ae16a74d57ecbded437e5d",
      "stable": true
    },
    {
      "name": "sourceAdmissionContainingRepository",
      "sha256": "475db138f83107caf57846b2007b6a93de3f18c462b9df68759300c64b966b22",
      "stable": true
    },
    {
      "name": "sourceAdmissionAssertRepositoryPath",
      "sha256": "f82fd4710cdbf0ad2ce372f31ef18f5344ace430b4c86463b89b972b32df488a",
      "stable": true
    },
    {
      "name": "projectTaxonomySourceAdmission",
      "sha256": "df769355c0b1be62b002cdfd5ef55deb7c791927247b6b496f861981ee024460",
      "stable": true
    },
    {
      "name": "SourceAdmissionUnsafeAncestorError",
      "sha256": "07034c2ce1211729bf0769011cbb401997c562fbd721abf97011ca62d68fde74",
      "stable": true
    },
    {
      "name": "SourceAdmissionPreparedOptions",
      "sha256": "c6c7be5a540f17be4a574c27ada74412c2cec781219f59b14e0dbba789d890f1",
      "stable": true
    },
    {
      "name": "sourceAdmissionAssertLexical",
      "sha256": "1171d502f73903b675628774ece5fa39735a04185de8b52647e7c5a560657105",
      "stable": true
    },
    {
      "name": "sourceAdmissionDirectoryChain",
      "sha256": "f68f994847144ccbf183910ad49293b82b82b47b07375a7564bc9ab7e0d77e57",
      "stable": true
    },
    {
      "name": "sourceAdmissionLstat",
      "sha256": "0dd4dede3b75510587c17151c746a382255bd289e79f150381fb4bbccb1c1b64",
      "stable": true
    },
    {
      "name": "sourceAdmissionPrepareOptions",
      "sha256": "912522e6423f98607fbdb3762eb225e430ff2a2ba5e7ac0eb1d1d518dc82e9c0",
      "stable": true
    },
    {
      "name": "sourceAdmissionCheckCancellation",
      "sha256": "c23a9cd993e046c2ef31f31482ea17792a2c51be76ea4d87766394ad56b46483",
      "stable": true
    },
    {
      "name": "sourceAdmissionGitRecords",
      "sha256": "fa3668d3e8dbe61b458daa93906a1966832a81cf2207b8c964a4ed3316bf8225",
      "stable": true
    },
    {
      "name": "sourceAdmissionGitExclusions",
      "sha256": "57ac49ba06eb202a05278ebc85190ba9f6970c116d1ede350af8a4e85e433755",
      "stable": true
    },
    {
      "name": "sourceAdmissionGitRows",
      "sha256": "4673485835802e49aedbf24ace95384bf376393a30eea5a936ab207d9c8113b7",
      "stable": true
    },
    {
      "name": "sourceAdmissionUntrackedRows",
      "sha256": "4ae03388af7445431ba8f62a6faab5d0d938486a3f592c72f3327f0ee5da72b6",
      "stable": true
    },
    {
      "name": "sourceAdmissionWalk",
      "sha256": "4e9eafe22e00be4b222fa7d3ff2391c517eba5614627f743cb6c14fb23aff286",
      "stable": true
    },
    {
      "name": "sourceAdmissionObservation",
      "sha256": "1ea2920ca5f9beebee17678e5a8e40c93354ff53997604fb62ad05e2701cc412",
      "stable": true
    },
    {
      "name": "collectTaxonomySourceAdmission",
      "sha256": "c9f54e359d94539345716402c198386b1eb0c294bd7180369c68c5eca804ac2a",
      "stable": true
    },
    {
      "name": "inventoryTaxonomySources",
      "sha256": "b0f570edd9a8381f377449f8210a42091724a6829bc6108d0376004370810449",
      "stable": true
    }
  ]
}
```

