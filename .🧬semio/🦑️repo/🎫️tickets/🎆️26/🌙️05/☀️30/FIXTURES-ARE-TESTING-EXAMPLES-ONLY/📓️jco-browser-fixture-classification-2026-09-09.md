# JCO Browser Fixture Classification Follow-up

Independent coordinator inspection found handwritten worker assertions, a browser page, host shim and vendored preview shims mixed with generated guest examples. They are executable test/support code. The assertions now occupy a canonical OS test case; the page and shims belong to JCO testkit. Generated guest examples remain fixtures; their imports now reach the physical testkit support. Every initial move preserved SHA-256. Runtime validation follows below.

## Runtime Evidence

The actual Bun server returned HTTP 500 at `/` before the repair: a prior path rewrite had changed its root-page comparison into a parent traversal string, so it attempted to stream a directory. The repaired server maps `/` to its testkit HTML and serves only the three explicit test/example/support owners. A public Bun/Nx check fetched the actual HTML, canonical worker, guest example and four support imports successfully. Its port is configurable for isolated validation; the registered default remains 8846.

The in-app browser then ran the relocated worker in Chromium. Its visible page and captured console both reported all four scenarios passing: S1 returned 42 as a Promise; S2 returned 777 while the 5 ms interval ticked 17 times; S3 returned 1; S4 returned five body chunks. The final browser message was `overall=true`, with four successful verdicts. Browser logs were read at 2026-09-09T10:38:59Z.

The real `bun nx run semio-jcoprobe-guest:check` also completed successfully after the runtime taxonomy catalog correction. Cargo checked the package at its canonical testkit location and exited zero. This closes the earlier JCO verification gap recorded in the framework report.

Retained verification input: `🧑‍💻coordination/🧪️jco-browser/📜️script.ts`. Generated logs: coordinator/jco-browser-red.log, jco-browser-green.log, jcoprobe-check-final.log. The initial probe itself had a variable-shadowing error before the corrected red run; only the corrected HTTP-500 run is used as pre-fix evidence.

## Preserved Moves

```json
[
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🧵️worker.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jco-callback/🟨️.mjs",
    "sha256": "47fad221af7ba4013f0c8f781417f7c7f0aa8acadeb6e53e1fe2a813a3585885"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🌐️.html",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🌐️.html",
    "sha256": "95808f16fa2bcc0ec67d1c318cec6e13271c72563f7c7e1b2af5dc8bf7df35e3"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🖥️host-shim.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🖥️host-shim.js",
    "sha256": "7d8bcedd26a3f7e1ab9c06a04fd49b5312e43e9444448cd97d2cdcda64bd0b06"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/io.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/io.js",
    "sha256": "807450ada8d995eba2332429f4e852ef5ac14d8ebcf1f41ecafdd23e98939262"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/filesystem.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/filesystem.js",
    "sha256": "7e54ae7f11d6df9d7f541c10c7a88b9af97078ff09fe5e708a789d18db1c24c3"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/random.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/random.js",
    "sha256": "990e1f6119aaed184ee44491afe81d0735f4208cd73267c4620fa8ac53ee8e90"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/index.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/index.js",
    "sha256": "5a678d2657d51266936e22759cd88335ccc625a9f91ac5a4af244ef975005799"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/config.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/config.js",
    "sha256": "40c60a11b1841f461436b11a2ff3318feac4a496bb18a0688a3980e15cff181d"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/sockets.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/sockets.js",
    "sha256": "50e97906c4d2894917eb6e5c060afd175c3132e31589b8f75682c19a120e486b"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/clocks.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/clocks.js",
    "sha256": "f8222196a0759af782acbb67595b9e0d02cc464f1155e391d8ae2a59bdb1245a"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/cli.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/cli.js",
    "sha256": "7d1729463727266e0816418e14e1e8c9700eda8c54b68ab6e6cae54eeba98c5a"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/environment.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/environment.js",
    "sha256": "4b9daf18432ded5e09367ead23abcb5e85bb49ae6fa3b25bc24b8d6ddc7bf1e1"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/http.js",
    "new": "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/http.js",
    "sha256": "0f8a9410638050dc64f20ca34823f4e9949c7ae110e0bd8f15af95529b18d642"
  }
]
```

## Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧩️jco-classification/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🌐️.html",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🖥️host-shim.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/cli.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/clocks.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/config.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/environment.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/filesystem.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/http.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/index.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/io.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/random.js",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🪞️preview2-shim/sockets.js",
  "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jco-callback/🟨️.mjs",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/⚡️out-jspi-explicit/jcoprobe.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/jcoprobe.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🌐️.html",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🖥️host-shim.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🧵️worker.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/cli.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/clocks.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/config.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/environment.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/filesystem.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/http.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/index.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/io.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/random.js",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/🪞️preview2-shim/sockets.js",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
]
```
