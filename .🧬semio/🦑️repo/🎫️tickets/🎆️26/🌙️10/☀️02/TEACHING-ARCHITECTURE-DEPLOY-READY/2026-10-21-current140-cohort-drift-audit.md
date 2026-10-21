# Current 140-Row Cohort Drift Audit

Observed 2026-10-21T01:22:22.866Z. Read-only current metadata/content observations of historical83 Ticket+57 Source rows; no new seal/policy or execution. All140 observations succeeded, no refusal/read issue. Each leaf was bounded ordinary single-link, ancestors checked nonsymlink directories, opened O_NOFOLLOW/nonblocking, descriptor/path device+inode and revision facts checked around read. This is per-file observation, not an atomic whole capture or custody proof.

Twelve rows differ from historical pins (two Ticket, ten Source); remaining128 match historical bytes/hash. Complete current/historical differences and metadata:

```json
[
  {
    "kind": "Ticket",
    "path": "📦️fresh/combined.schema.json",
    "bytes": 106837,
    "sha256": "517bc59594031bec16321431133dccfff308b42c75e2b948bd7ba5af5f1409f6",
    "dev": "16777229",
    "ino": "447579077",
    "mtime": "2026-10-21T00:57:57.265Z",
    "historical": {
      "bytes": 106187,
      "sha256": "0b284a4121debc644948ba77743793690aab03ef127e01f4d2b05fc75ca8d71b"
    }
  },
  {
    "kind": "Ticket",
    "path": "📦️fresh/combined.cases.json",
    "bytes": 24614,
    "sha256": "cb409f7875c7f6d1c667a7180df4d906a376248c0c81209b8991d09f628b91d3",
    "dev": "16777229",
    "ino": "447579078",
    "mtime": "2026-10-21T00:57:57.267Z",
    "historical": {
      "bytes": 22682,
      "sha256": "351658ab90c8296cde8b2dac1645ff8cdc69ca7bd24d9aa1a429914258774164"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs",
    "bytes": 37282,
    "sha256": "5eb20e1d0252a2d1756dd8df67ab11dec7d41303aca42cd6018039e290417ddc",
    "dev": "16777229",
    "ino": "451613145",
    "mtime": "2026-10-21T00:43:28.167Z",
    "historical": {
      "bytes": 37273,
      "sha256": "b7802a9f54b5e52b34a4ea17bac72a2db69d3f40f5c0bfceb310fd57cfee0dc0"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs",
    "bytes": 3858,
    "sha256": "1b5d361493190e3521269ecdb5f172ea32a47c8d8464b022f994428f128bc152",
    "dev": "16777229",
    "ino": "379113746",
    "mtime": "2026-10-21T01:15:39.170Z",
    "historical": {
      "bytes": 3800,
      "sha256": "31a02c99a3216a8208cf3afd51efd39ba3d4df981798aa7b7f906619e870b95a"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs",
    "bytes": 8795,
    "sha256": "610fd892040131dd79f3b66d1c0fd4b3bafcb794c22883660913502a3a721cb6",
    "dev": "16777229",
    "ino": "380862593",
    "mtime": "2026-10-21T01:15:39.169Z",
    "historical": {
      "bytes": 8090,
      "sha256": "deb1f14855dc0db7576d5d9daec003d691492745ff8c0aa393160f316300fe3d"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",
    "bytes": 33736,
    "sha256": "14d778b9323d66f7786c7bf43a3f99545252cce88910827e88732e4b0ad68657",
    "dev": "16777229",
    "ino": "452355868",
    "mtime": "2026-10-21T01:05:57.630Z",
    "historical": {
      "bytes": 33466,
      "sha256": "bed212e7c8bb1f6a3b996588f2544aee311ea96a4fb763ca76ea65b52c5859db"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts",
    "bytes": 105458,
    "sha256": "8cf030c4dde4d14ccf1b50cce09da679bb5cf087fae6156634ed04d8d1b94df2",
    "dev": "16777229",
    "ino": "452822511",
    "mtime": "2026-10-21T01:14:48.045Z",
    "historical": {
      "bytes": 96795,
      "sha256": "ab2cb5d9e9843d7a27e38b7e2bc7644029dcda553b7bbb8902ff560f30b12048"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json",
    "bytes": 181615,
    "sha256": "71a3cd5f22ab3875fe0fe48d49b659817c4103c4b9abb7a16ee8b2f4d533f1df",
    "dev": "16777229",
    "ino": "451613143",
    "mtime": "2026-10-21T01:14:48.035Z",
    "historical": {
      "bytes": 165635,
      "sha256": "cf8888db4eb5e51f5a28eb87a4a15733fe66e89ec657fb47daecfca3cadcc357"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json",
    "bytes": 28716,
    "sha256": "9e447637457b6a5dd8bac4836248c62b8a47d9bc661ec1a15740a0633e5903bb",
    "dev": "16777229",
    "ino": "452326292",
    "mtime": "2026-10-21T01:06:31.398Z",
    "historical": {
      "bytes": 26414,
      "sha256": "65b34dd758cb9429b2e08689d8e419170f66b6b301548ab7bf7abaa8377a3b15"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json",
    "bytes": 161468,
    "sha256": "43ba79cb42ad00c18a76ba347b5ad38fdf1679971d5fb24f690cd1ea67259f70",
    "dev": "16777229",
    "ino": "436568655",
    "mtime": "2026-10-21T01:06:31.401Z",
    "historical": {
      "bytes": 149000,
      "sha256": "92d43d8138a58573354deaf40047a740a47f0879ecb7077e58c343976cd074d2"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts",
    "bytes": 60780,
    "sha256": "3eec7d9b97621426d3cab52c8b815a690f8464ec2a4af83bb1d416797678908b",
    "dev": "16777229",
    "ino": "436568652",
    "mtime": "2026-10-21T01:06:31.402Z",
    "historical": {
      "bytes": 53576,
      "sha256": "4707e8f48fd2b42e710bdeb86bce6b83c754ae1a4b1a20fd0f495bd506e1c8ec"
    }
  },
  {
    "kind": "Source",
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts",
    "bytes": 53695,
    "sha256": "311556b2a0ff76660098928dfd50e53bef78916b0fc8192415c357ff30381c8d",
    "dev": "16777229",
    "ino": "452434908",
    "mtime": "2026-10-21T00:53:09.875Z",
    "historical": {
      "bytes": 52179,
      "sha256": "dd74e70a87e5862ab9eb564810e3757308836e1564ff42ec364af51c976d9352"
    }
  }
]
```

The revision reader is currently8795 bytes/SHA-256 `610fd892040131dd79f3b66d1c0fd4b3bafcb794c22883660913502a3a721cb6`, versus historical8090/deb1. Progress schema/corpus/test/runtime drift; progress feature remains historical-matching. Combined schema/cases also drift; revision schema itself matches historical. No blanket refresh is performed. Historical manifest remains dispatch evidence and is not current authority. Comparison with Root-reviewed current report snapshots follows separately if available.

## Comparison With Available Current Reviews

All five progress observations match the exact current cohort documented in progress2-input-custody and progress-fourteen-slot-independent-review (schema28716/9e4476, corpus161468/43ba79, test60780/3eec7d, runtime53695/311556, feature4754/06a433). Assembly37282/5eb20 and script-inputs33736/14d778 also match progress-import-resource-peer-review. Current SourceDemand105458/8cf030 and graphfixture181615/71a3cd are newer than that report snapshot105097/7b80b9 and180862/772f21; exact newer-byte review must be joined separately rather than assumed from the older report. Reader8795/610fd and loader3858/1b5d36 are current physical observations, with no test execution claim.
