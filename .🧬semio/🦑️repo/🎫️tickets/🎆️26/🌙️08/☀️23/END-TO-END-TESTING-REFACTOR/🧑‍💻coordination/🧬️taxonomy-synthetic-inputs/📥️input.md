# Synthetic Input Relocation Input

JCO guest and browser-host stand-ins, and the scale actor are synthetic executable inputs; assertion programs remain direct canonical tests. Build output remains inside the source fixture package.

```json
{
  "mappings": [
    [
      "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest",
      "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest"
    ],
    [
      "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript",
      "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host"
    ],
    [
      "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust",
      "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale"
    ],
    [
      "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🟦️typescript/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🟦️.ts"
    ]
  ],
  "moves": [
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/🧩️component/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/🧩️component/🦀️.rs",
      "sha256": "508b94baf161c85826d149503520a307ee227fdb60e2d604a937d976def16b1a"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/Cargo.toml",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.toml",
      "sha256": "e1dd5aa3e494e4632c8c730c5a4d573a8ad84dc1d1659f0161da11d4884a6b7f"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/Cargo.lock",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "15f877b56093fdbcc4a9aa06512583a989684068e944202f46885c4eca0986c8"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs",
      "sha256": "e3173e78ce5ec51c9d0226f5a87fe99c29206f9247f50bee119e1a6786bb2a55"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit",
      "sha256": "6f51deddddd46b999edf52bb32c88a6318637a83f61143e804853faae2a7752a"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/📜️script.ts",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/📜️script.ts",
      "sha256": "06ceb510595efdc90b93bd18131308240cf09b4f5c41b5a9496f5ac15fea17c2"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🌐️.html",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🌐️.html",
      "sha256": "12ff558404f118ff7d360ea501debb7a56742c7e98017e46d47182e55c62404d"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/io.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/io.js",
      "sha256": "807450ada8d995eba2332429f4e852ef5ac14d8ebcf1f41ecafdd23e98939262"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/filesystem.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/filesystem.js",
      "sha256": "7e54ae7f11d6df9d7f541c10c7a88b9af97078ff09fe5e708a789d18db1c24c3"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/random.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/random.js",
      "sha256": "990e1f6119aaed184ee44491afe81d0735f4208cd73267c4620fa8ac53ee8e90"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/index.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/index.js",
      "sha256": "5a678d2657d51266936e22759cd88335ccc625a9f91ac5a4af244ef975005799"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/config.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/config.js",
      "sha256": "40c60a11b1841f461436b11a2ff3318feac4a496bb18a0688a3980e15cff181d"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/sockets.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/sockets.js",
      "sha256": "50e97906c4d2894917eb6e5c060afd175c3132e31589b8f75682c19a120e486b"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/clocks.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/clocks.js",
      "sha256": "f8222196a0759af782acbb67595b9e0d02cc464f1155e391d8ae2a59bdb1245a"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/cli.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/cli.js",
      "sha256": "7d1729463727266e0816418e14e1e8c9700eda8c54b68ab6e6cae54eeba98c5a"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/environment.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/environment.js",
      "sha256": "4b9daf18432ded5e09367ead23abcb5e85bb49ae6fa3b25bc24b8d6ddc7bf1e1"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/http.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/http.js",
      "sha256": "0f8a9410638050dc64f20ca34823f4e9949c7ae110e0bd8f15af95529b18d642"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🖥️host-shim.js",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🖥️host-shim.js",
      "sha256": "7d8bcedd26a3f7e1ab9c06a04fd49b5312e43e9444448cd97d2cdcda64bd0b06"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🦀️.rs",
      "sha256": "4053eb4883f5ea0b8477f7d496c63205b30794ba8652d5b260761d70f0db24d9"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/🎭️profile/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🎭️profile/🦀️.rs",
      "sha256": "ff09faa8e92b3efaf63c43f2721d565477c9936e292ebde4fe45a0951fa4c1ad"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/Cargo.toml",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/Cargo.toml",
      "sha256": "4f819d9619f17e0f9d0d578f6ddbcf995ae6101bd6724fd0000f2b284305d53f"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/dist/component/.nx-artifact.json",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/.nx-artifact.json",
      "sha256": "41566c2327e18a859523a79c0dfe493e0a138adedc8556c1d193db89c179456a"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm",
      "sha256": "0ecf6554bfb276e75a6c56588e4875a11ab3499c200e9b42c760b90d81629c3c"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/📋️project.json",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json",
      "sha256": "e9fbeef9dee68e00c52ed994dabcc828116bb6aad375b5d0c0d72d6d381c686d"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/📜️script.ts",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts",
      "sha256": "12b39487f27bf3f7fcdeb4122905ce045f89f20e4068f18919f418e302a569b0"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/🦀️.rs",
      "sha256": "0bb64b6f50cda4416c1fad3d6fd4b32b118f11e0fb08fa4e0ddca04c6bf6253e"
    },
    {
      "from": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🟦️typescript/🟦️.ts",
      "to": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🟦️.ts",
      "sha256": "7e8a088cc75557df23b98a57a67468ab6f2535810334911be779b7b0d80d4a07"
    }
  ]
}
```
