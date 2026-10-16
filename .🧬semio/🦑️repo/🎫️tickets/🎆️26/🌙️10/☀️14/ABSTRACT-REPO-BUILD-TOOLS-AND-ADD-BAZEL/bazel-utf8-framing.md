# Bazel Control UTF-8 Framing

All three actual TCP control paths now initialize native streaming UTF-8 decoding before any data handler: the parent rendezvous server, received-action cancellation client and prebirth client. `pending+=chunk.toString()` now receives complete decoded strings. Existing Buffer.byteLength(pending) control-message checks remain unchanged; malformed/oversize messages still refuse. Runtime uses only the system Socket decoder through a small own structural interface, with no external runtime dependency or Node class exposed in its API.

The language-neutral fixture contains literal emoji/CJK/German JSON strings and byte cuts inside multibyte codepoints. A real loopback TCP socket sends the first fragment, waits for the server's acknowledgement, then sends the remainder. Thus the cut cannot be hidden by coalescing both writes into one application chunk. Streaming TextDecoder supplies an independent decode oracle and Ajv validates the payload contract. The actual production decoder initialization is used on the server, and Source checks cover all three production paths.

The initial test had an incorrect shared-helper import; bazel-utf8-framing-red.log is setup failure only. After fixing that path, semantic RED29275 reproduced actual corruption: expected a🧪界z, received a����界z. Baseline extraction added a no-op socket setup without changing previous decoding semantics or removing an existing guard. Adding setEncoding('utf8') fixes that real split-byte failure.

Root current combined87090 terminal0:14pass/9skip/379assertions/5.76seconds, bazel-snapshot-readonly-green.log. This includes the framing law and concurrent snapshot/primary-cleanup laws. No current entire backend matrix or SDK success is inferred from this Source gate.

The ticket Bazel factory now authors an emoji in its policy owner BEFORE Original birth. Later actual action/launcher handshakes therefore carry genuine Unicode Original wire, with unchanged policy/deadline throughout their lifetime. Fresh actual Cache/Refusals remain queued behind Root's SDK slot; historical ASCII runtime receipts are not relabeled as that proof.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️framing/🟦️.ts` SHA256 `c2ba3bb0f7596426f8d6914873d9177e0f20265f3b296bc3c62b44a72f76c69e`, absolute UTF16 length136.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️framing/🔣️.json` SHA256 `7e1bce4d13d3d2fdf6f05c38b5e5dfde3f2cc6120450a94f71bd4bf0579ee2f2`, absolute UTF16 length138.
