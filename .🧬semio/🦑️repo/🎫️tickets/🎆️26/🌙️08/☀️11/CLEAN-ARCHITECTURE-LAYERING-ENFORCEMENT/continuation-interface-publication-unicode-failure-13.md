# Publication Unicode Failure in Whole 13

Actual source-bound publication refused because published membership differed from prepared pages. Whole13 snapshot Cargo.toml contains 2 replacement characters; current physical root Cargo.toml contains 0. Exact new source bytes and immutable before snapshot remain retained.

```json
[
  {
    "offset": 13312,
    "context": "s/🦀️rust\" }\nsemio-framework-trace = { path = \"🧰️framework/��️modules/⏱️trace/📦️packages/🦀️rust\" }\nsemio-framework-jo"
  },
  {
    "offset": 13313,
    "context": "/🦀️rust\" }\nsemio-framework-trace = { path = \"🧰️framework/��️modules/⏱️trace/📦️packages/🦀️rust\" }\nsemio-framework-job"
  }
]
```

The prepared Pages writer uses code points for authored values, but copying retained source uses one UTF-16 code unit at a time and can flush between the high and low halves of a surrogate pair. The physical writer independently UTF-8 encodes each page, so a split pair becomes two replacement characters. New neutral page-boundary publication laws and independent Node UTF-8 oracle must establish the actual fix before adoption; the successful small original tests do not prove this boundary.
