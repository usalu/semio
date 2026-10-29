# Material Manifest Adjacency Audit

Cheap production pure-function validation; no material downloads, compiler, images or heavy jobs opened.

```json
{
  "distributionLayouts": [
    {
      "target": "aarch64-apple-darwin",
      "layout": {
        "store": "tools",
        "directory": "/fixture/print/tectonic/aarch64-apple-darwin"
      }
    },
    {
      "target": "x86_64-apple-darwin",
      "error": "Closed Print artifact layout required"
    },
    {
      "target": "aarch64-unknown-linux-musl",
      "layout": {
        "store": "tools",
        "directory": "/fixture/print/tectonic/aarch64-unknown-linux-musl"
      }
    },
    {
      "target": "x86_64-unknown-linux-musl",
      "error": "Closed Print artifact layout required"
    },
    {
      "target": "arm-unknown-linux-musleabihf",
      "layout": {
        "store": "tools",
        "directory": "/fixture/print/tectonic/arm-unknown-linux-musleabihf"
      }
    },
    {
      "target": "i686-unknown-linux-gnu",
      "layout": {
        "store": "tools",
        "directory": "/fixture/print/tectonic/i686-unknown-linux-gnu"
      }
    },
    {
      "target": "x86_64-pc-windows-msvc",
      "error": "Closed Print artifact layout required"
    }
  ],
  "bundle": {
    "files": 500,
    "bytes": 19856791,
    "maxFile": 4408061,
    "maxName": 45
  },
  "fonts": [
    {
      "output": "Anta-Regular.ttf",
      "bytes": 74304,
      "magic": "00010000",
      "sha256": "9f3b9723a50c1244d49906b1892b3aad43876ab9a2de80288db47821b0c7b298"
    },
    {
      "output": "ShareTechMono-Regular.ttf",
      "bytes": 43272,
      "magic": "00010000",
      "sha256": "9ceab1f87414829af259c0f537573ae03ef7dd3147c0b27a36a1a0beb6732677"
    },
    {
      "output": "NotoEmoji-Regular.ttf",
      "bytes": 1982596,
      "magic": "00010000",
      "sha256": "de6c18832938afc99caf132b39d6a30a19bac7f2e812e28db2535b4608d27551"
    },
    {
      "output": "NotoSansCJKsc-Regular.otf",
      "bytes": 16437364,
      "magic": "4f54544f",
      "sha256": "2c76254f6fc379fddfce0a7e84fb5385bb135d3e399294f6eeb6680d0365b74b"
    }
  ],
  "fontBytes": 18537536
}
```

## Findings

The known target-pattern defect also affects Intel macOS and native Windows x64, not only current Linux x64. Four other declared distributions pass current layout validation. Core owns the narrow schema fix and independent vectors. Every declared compressed archive is below the32MiB material limit. Production printBundlePlanV1 successfully validates all500 pinned file ranges,19,856,791 aggregate bytes and maximum4,408,061-byte single file, within32MiB aggregate/16MiB per-file/512-file bounds. It enforces unique normalized names, reserved-name exclusions, offsets and archive endpoints.

All four tracked fonts exist with valid TrueType/OpenType magic and unique ASCII output names;18,537,536 aggregate bytes fit the32MiB font budget and four files fit maximum16. The manifest source paths are included in the private source snapshot. No additional deterministic manifest/policy mismatch found in these pure checks. Archive contents, HTTPS responses and actual downloaded identity remain runtime checks; this audit did not fetch or decode compiler archives.

## Independent Target Fix Review

Reviewed narrow artifact schema pattern and neutral layout corpus/test. New target grammar allows underscore only between nonempty lowercase alphanumeric groups inside hyphen-separated segments, retaining at least one hyphen, max64units and closed object fields. It still cannot express a separator, dot, colon, absolute path, traversal or drive prefix. It does not widen namespace/id/source path or directory/path limits.

Neutral corpus now includes all seven actual manifest targets, with exact Unix expected slots; existing adapter verifies AJV acceptance, owned layout, independent SQLite concatenation and Windows join. Invalid rows add leading/doubled/trailing underscore, doubled hyphen, slash and uppercase cases. The new test iterates the real distribution manifest through tectonicDistributionV1, requires a matching neutral row and verifies exact target slot. This closes the actual-data mismatch across all declared hosts without adding unbounded pathability. Root reported7tests/177assertions passing; reviewer did not rerun tests or launch heavy work.
