# Native Integrated Closure Audit

REFUSED at 2026-10-16T15:10:02.170224+00:00–2026-10-16T15:10:02.309696+00:00. Own sequential bounded nofollow FD reads/stamps/SHA compared 534 per-role rows (250 unique paths).

- reader: 164, `624c5e9ced4df2b25a738800c3cd755adb8044416474f0a20528940b790d9a84`.
- runtime: 245, `0fc13a625cf39fe4fb5bdde9c5c0bbd4d468cb643f11e43b8db0a460b34d978d`.
- compiler: 125, `7be7cfcfd403f558324bab62d45551829b7f9a331adedea81a57b4e59ad66b6d`.

Exact reader declaration: 164 entries; missing 0, extra 0. Problems: [{"role": "runtime", "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs", "expected": "829806bf1c5abdfc472e77a823c96c9931b2ee7d85992916e939f47c88b3f0fd", "actual": "68ca1400d1cd166cc748a663e1ee8b033c6fa2f74cac6d316ae93445880610a7"}, {"role": "runtime", "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs", "expected": "2d7dec5edc06657da904b1a57d291c5ba36e86cd873a99bc40d8b7f09c78e613", "actual": "f1ee05c507e58ecf7e08ab2f60cbbb44825938d49c42a4b65b08e7def3c8ed19"}].

Manifests store path/SHA, not capture stamps. Own pre/open/post named stamps are retained in the independent JSON. This is sequential current-byte evidence, not a simultaneous three-role tree, SDK provenance/use, physical or public workflow qualification. Earlier failed receipts remain historical.
