# Wire Budget Audit

Pure bounded Bun measurement and source inspection; no production edits or native build.

Current actual browser public serialization is JSON.stringify (window-invocation/🟦️.ts:80,89). Native JSON text writer explicitly passes non-ASCII through unescaped (text-writer/🦀️.rs:108); canonical byte writer does likewise. public_invocation_char_cost intentionally reserves five per BMP non-ASCII character so per-string admission works for either escaped or raw encoders; it is not actual serialized byte length. A two-byte BMP character has cost5 but actual UTF8 wire2. Applying its conservative cost to the entire body as if every character is Unicode-escaped would overestimate current wire size.

Measured adversarial admitted transport shapes:

```json
[
  {
    "name": "Patch",
    "wireBytes": 116804,
    "asciiEscapedWireBytes": 116804,
    "chunkCounts": [
      65,
      null
    ],
    "maxChunkCost": 512
  },
  {
    "name": "SetTry",
    "wireBytes": 77307,
    "asciiEscapedWireBytes": 77307,
    "chunkCounts": [
      256,
      256
    ],
    "maxChunkCost": 64
  }
]
```

Patch measurement: 64 identifiers ×128 one-byte control characters =8192UTF8 identity aggregate; field and paramKey128 controls each; valid JSON source32769 tabs +0 =32770UTF8 bytes in65chunks. Metadata controls outer-escape to six wire bytes each, valid source tabs outer-escape to two. SetTry measurement: key/window512 controls each, paramKey128 controls, optionValue512 controls, windowKind forms-try; each valid JSON source16384 bytes (tabs +0 and tabs +{}), split into256chunks each. Input transport metadata excluded because valueJsonChunks cannot mix it. These examples test transport admission only; target leases and question semantics require live fixtures.

Conservative actual-encoder bound: valid JSON source characters can at most double in the enclosing JSON string (quotes, backslashes, tabs/newlines); raw other control characters cannot occur in valid JSON. Patch source therefore<=65540 bytes, bounded metadata (8192+128+128) ×6<=50688, plus65chunk delimiters,64identifier delimiters, and fixed addressed-envelope overhead; comfortably below131072. SetTry sources<=65536, allowed simultaneous metadata<=1664×6 plus forms-try, up to512chunk delimiters and fixed envelope; also below131072. Per512UTF8 page, worst public_invocation_char_cost<=2560 (all one-byte controls), below4096. Two-byte BMP pages cost at most1280. Metadata512UTF8 also costs<=2560.

No concrete128KiB mismatch established for the specified bounded transport values with actual current serialization. Arbitrary Unicode-escaping external encoders can produce larger equivalent bodies; that is not the current serializer and does not establish a current producer failure.
