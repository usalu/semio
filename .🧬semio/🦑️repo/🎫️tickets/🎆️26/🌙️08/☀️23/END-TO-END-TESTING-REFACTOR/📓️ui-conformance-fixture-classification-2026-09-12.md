# UI Conformance Fixture Classification — 2026-09-12

The 147-file conformance corpus is used only by test-gated Rust and renderer tests. It belongs in the UI contract owner's fixtures, outside cases and product examples.

## baseline Runtime

The actual conformanceCorpusSelfTests export passed all 62 catalog cases using strict Ajv, exact directory-role comparison and expected-case identities.

## Exact Moves

```json
[
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/⌨️shortcut/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/⌨️shortcut/🎯️expect.json",
    "sha256": "1d1aa2cf961341e9127587d5da8c35e445ef5500a6580dc0a19ea5c56eddaac2"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/⌨️shortcut/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/⌨️shortcut/📸️snapshot.json",
    "sha256": "b48530fb0cb512d389a8f372e91090ca7b590c59f2c3f84b68b9ca391182ec1c"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/🏷️labelled/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/🏷️labelled/🎯️expect.json",
    "sha256": "c22cfa4c9cef82349164cca1f086855623772c1b1c667c0663900403f70c29ff"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/🏷️labelled/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/🏷️labelled/📸️snapshot.json",
    "sha256": "b7b7391fbc676839e651837e829acb255fe48ed9dfac291039ceeef81d7896f3"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/💬️described/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/💬️described/🎯️expect.json",
    "sha256": "bbed4d65bb3ad1da101ce97a39af12727a3ec3b0941b582981408ec7fcd6f899"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/💬️described/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/💬️described/📸️snapshot.json",
    "sha256": "e70876a958a7ba3b6d19af173ab90efaa0c09101998fe81eadb3660ee529aba8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/📢️live-region/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/📢️live-region/🎯️expect.json",
    "sha256": "fc60280804a7493ade330a32c5e37ac084785750c2677f6afdbd27292b24563c"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/📢️live-region/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/📢️live-region/📸️snapshot.json",
    "sha256": "786b6b757033340e71ad31e46d597bc43d90af45b2fc683ccbfda39d0656fc01"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/🖼️decorative-image/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/🖼️decorative-image/🎯️expect.json",
    "sha256": "c9174255314245238e8a0366480fac44719060a0180e26e9d31818c7300b05f3"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/♿️accessibility/🖼️decorative-image/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/♿️accessibility/🖼️decorative-image/📸️snapshot.json",
    "sha256": "c125847b3bf8c23f27d2a1295e4083061e36a42c54bc2733494c8dcc500c7526"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📇️catalog.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📇️catalog.json",
    "sha256": "0fcfcec10e7d9b37c97dcfbb0ff2bae45a0e613202940e01a7afc201022e8f83"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🍃️leaf/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🍃️leaf/🎯️expect.json",
    "sha256": "9647d048e3a2542bcda33e818d333f0d039171ed1258c17969cebb3228441f04"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🍃️leaf/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🍃️leaf/📸️snapshot.json",
    "sha256": "ccb68546e3d87f5be641fb36beb6e63291b25e40720ad80fd1c9386836c48027"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/📍️absolute/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/📍️absolute/🎯️expect.json",
    "sha256": "9a9bdb6140ccc0a3cc0811a6c719414136c79186a052af741746d84cf267ac7d"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/📍️absolute/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/📍️absolute/📸️snapshot.json",
    "sha256": "e67e7cb5ce80fe27dad19d0200071bf5d1f6feac4ae02739a8d645d8fd25bcfd"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/📚️stack/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/📚️stack/🎯️expect.json",
    "sha256": "d8d72d2a85f9a2042b3608156072b1937ce3cd267e7d19f2094533e819d63ecd"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/📚️stack/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/📚️stack/📸️snapshot.json",
    "sha256": "2888eaef65d4955262e74e7856c753b73224ec835c0b9614fffd4fdde28436c4"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/📜️scroll/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/📜️scroll/🎯️expect.json",
    "sha256": "bdb982171f1a679c52fc7fc684bc330505b6dbe783d08ab51e7257ab363659ab"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/📜️scroll/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/📜️scroll/📸️snapshot.json",
    "sha256": "59d763ada9d3c783a7d61dc5df7d73718a94aed2f18041d10d9db44309373171"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🕸️grid/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🕸️grid/🎯️expect.json",
    "sha256": "58ac9b8be32b2de21e48ec57ab537bae86242757705e741ba5e5663969d8646a"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🕸️grid/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🕸️grid/📸️snapshot.json",
    "sha256": "bb94284b0b026693d380c6ee8a770e2041e8e01431dbc05a546d1dedcb1b3991"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🪆️nesting/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🪆️nesting/🎯️expect.json",
    "sha256": "2dd68e2279e9f7c81334170dc44ce3c6a915ea205f2bd6870f9c8668b15d885d"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🪆️nesting/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🪆️nesting/📸️snapshot.json",
    "sha256": "5b6c597ba6f1725b86374545acf9c79b0d18222ab071bcd4da1818bee61966da"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🪟️overlay/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🪟️overlay/🎯️expect.json",
    "sha256": "1188280e3daeaae67b3f29524e7b56dd47c2ac11b32aadb9ea9aef36a7960172"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/📐️layout/🪟️overlay/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/📐️layout/🪟️overlay/📸️snapshot.json",
    "sha256": "c46c322b031ce0fa8c5c2e0a6957391e9f2ee5b4a60eae0fd2694663c13eae4d"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/☑️form-with-validation/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/☑️form-with-validation/🎯️expect.json",
    "sha256": "6296cd3c8204a66db756dc55c1715546ed9e8a766f33b4b41841caa17de99365"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/☑️form-with-validation/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/☑️form-with-validation/📸️snapshot.json",
    "sha256": "c891f5ffe7db627efd141f53264b459b232d0aa77e46f53115262214f4fa8a38"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/🌳️tree-nested-sections/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/🌳️tree-nested-sections/🎯️expect.json",
    "sha256": "de5fa4312cdff4b073a6782b8b9c9425116368aaf65239abe490ea9bbfd5739a"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/🌳️tree-nested-sections/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/🌳️tree-nested-sections/📸️snapshot.json",
    "sha256": "466aa4c05d97cb681371330494ef156ea132b374042a34a9d20dfd4a663a0b1d"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/💭️dialog/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/💭️dialog/🎯️expect.json",
    "sha256": "f6364923cd297a4f9986d9d28269a42ae11ccfd6558b4fb1b0246fcdd381098f"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/💭️dialog/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/💭️dialog/📸️snapshot.json",
    "sha256": "2cd7b8eecf7ff592f5f513fb7820a50affb0201758bf1b2f4f60523bdcb2d27b"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/🗺️surface-embedded/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/🗺️surface-embedded/🎯️expect.json",
    "sha256": "e05d1ee3644826d0224f7e7bd5909d5071ac27af750000bd95ea41a93af50779"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/🗺️surface-embedded/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/🗺️surface-embedded/📸️snapshot.json",
    "sha256": "60650b6dae7a8d96b9bee5422333f1b64a570a7831b086086076c217b78290b1"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/🧰️toolbar/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/🧰️toolbar/🎯️expect.json",
    "sha256": "851421bca8e7fcca55380c9a0eb1ceededdb8e3ac8abbfbf48bd14c64bba935c"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/🧰️toolbar/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🖥️composite/🧰️toolbar/📸️snapshot.json",
    "sha256": "87f443f44644eec44aa6d445c659c6ad5e82c44f26559605dd73193167387c2b"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/👶️quota-children/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/👶️quota-children/🎯️expect.json",
    "sha256": "74da0e7ecaa2b495868024ec61d1d97159e0feae4544afb0d8a3fbd88e527b50"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/👶️quota-children/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/👶️quota-children/📸️snapshot.json",
    "sha256": "96abd1405d75fc421dc89a1005f481c9756110b47d58d50887664415529784d6"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/👶️quota-children/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/👶️quota-children/🩹️patch.json",
    "sha256": "182c96b590d72e476d4dd5edf2225734e2901cb9c7d459d8f249a3f7bdfee322"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/💾️quota-patch-bytes/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/💾️quota-patch-bytes/🎯️expect.json",
    "sha256": "cc7a47e6c28823413b865469997fe33e9656d20018c429dd0bb1060b887864ba"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/💾️quota-patch-bytes/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/💾️quota-patch-bytes/📸️snapshot.json",
    "sha256": "96abd1405d75fc421dc89a1005f481c9756110b47d58d50887664415529784d6"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/💾️quota-patch-bytes/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/💾️quota-patch-bytes/🩹️patch.json",
    "sha256": "b39ce8e686d1d3bf72c5cf9dc11c9ade11b52f7e22f344ae1539eccc426bac12"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/📏️quota-depth/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/📏️quota-depth/🎯️expect.json",
    "sha256": "465a91a056ae98306c88de685d89e995009aec897886b94c7acd304a25091d8d"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/📏️quota-depth/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/📏️quota-depth/📸️snapshot.json",
    "sha256": "06ef2f48e4b836d02794e433a46400cd1a2a44131f6c150e131de97c5ff3d9c5"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/📏️quota-depth/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/📏️quota-depth/🩹️patch.json",
    "sha256": "84d5fb9e0f21c640bd3494d96df7e74875e6b0570566cba6725587d490d2c6b6"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔁️cycle/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔁️cycle/🎯️expect.json",
    "sha256": "365d29b30ab21bee742d838b4a18ad423493cf1836561cc4498d26a30d4a4a21"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔁️cycle/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔁️cycle/📸️snapshot.json",
    "sha256": "2a8c845a56b5e4b6d6d3ad9108123c5da35263271baf5f0efaeced1a8ceb6de5"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔁️cycle/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔁️cycle/🩹️patch.json",
    "sha256": "9e476408e513bb772f42d1c8b71e33e33ec5bad4c52d3f13954be30821087392"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔗️dangling-child/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔗️dangling-child/🎯️expect.json",
    "sha256": "678381abc2ca9247914e8ae27bd0087ed68dfcd530fe49a676b5fdc5e39ab92b"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔗️dangling-child/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔗️dangling-child/📸️snapshot.json",
    "sha256": "96abd1405d75fc421dc89a1005f481c9756110b47d58d50887664415529784d6"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔗️dangling-child/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔗️dangling-child/🩹️patch.json",
    "sha256": "2056d79ac6111bedf8e0ae82f82ed775f71e01a3bbeb9fa00f3de1160ae1a2a3"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔢️quota-nodes/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔢️quota-nodes/🎯️expect.json",
    "sha256": "b466c56d0cad5c68dd29ea335ce1b0e621d8f2384dcecf093d86839904f49613"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔢️quota-nodes/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔢️quota-nodes/📸️snapshot.json",
    "sha256": "29fbad6958e754ece9c09617d49dd377b58ef4fa012f983eba509d26eda23e90"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔢️quota-nodes/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔢️quota-nodes/🩹️patch.json",
    "sha256": "ebd07bfcdfa94298fad67722cd081eab39ac65dd067b48284009d7cf3ca61b15"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔤️quota-text-bytes/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔤️quota-text-bytes/🎯️expect.json",
    "sha256": "de4973a980d95ecfb01e8559072dadc1d398534b57b0ff46bf602936a1d60239"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔤️quota-text-bytes/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔤️quota-text-bytes/📸️snapshot.json",
    "sha256": "96abd1405d75fc421dc89a1005f481c9756110b47d58d50887664415529784d6"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🔤️quota-text-bytes/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🔤️quota-text-bytes/🩹️patch.json",
    "sha256": "4542ad860657932d9b6d098a7068a54d7e436241658440a6978b332fa7abf257"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🕰️stale-base-revision/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🕰️stale-base-revision/🎯️expect.json",
    "sha256": "63915b9487cf6346885c517d5cf93876437d24d24a627efd245c7aa5f2b33196"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🕰️stale-base-revision/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🕰️stale-base-revision/📸️snapshot.json",
    "sha256": "96abd1405d75fc421dc89a1005f481c9756110b47d58d50887664415529784d6"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🕰️stale-base-revision/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🕰️stale-base-revision/🩹️patch.json",
    "sha256": "7853d80e65172f029dbcbc0c4713dc69ab2b3223598165f3a72b0fb0b32794a3"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🗝️duplicate-sibling-key/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🗝️duplicate-sibling-key/🎯️expect.json",
    "sha256": "4b86efd1e1a717c4c0c8d801340b46ce2934b9e31d1faca5867adc4f44fa6e35"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🗝️duplicate-sibling-key/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🗝️duplicate-sibling-key/📸️snapshot.json",
    "sha256": "658d677884d4fb8eaddc630bc5c1138e48ddf9cbafbcc5eefaa75083a5310002"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🗝️duplicate-sibling-key/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🗝️duplicate-sibling-key/🩹️patch.json",
    "sha256": "3090b1bf57b6da18615304be973b296bd089bfaa449af3bae86fb60c72b2abfe"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🧮️quota-patch-ops/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🧮️quota-patch-ops/🎯️expect.json",
    "sha256": "0a4e5d92c08a90fd230a1a46f7ad6e0d8e6eeaf6c3c4781fe1429113b507abba"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🧮️quota-patch-ops/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🧮️quota-patch-ops/📸️snapshot.json",
    "sha256": "96abd1405d75fc421dc89a1005f481c9756110b47d58d50887664415529784d6"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🚫️rejection/🧮️quota-patch-ops/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🚫️rejection/🧮️quota-patch-ops/🩹️patch.json",
    "sha256": "8401bd5a047df05f49cbaecd62687c1d3ff7f28bf4d8900b6e15aa27347fc854"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⌨️input/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⌨️input/🎯️expect.json",
    "sha256": "d438d16a0969ba058061b0c524429d9e971545afe2e0d6ed913248c2ccfada84"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⌨️input/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⌨️input/📸️snapshot.json",
    "sha256": "9990c05d72aaeb4f34fb9a4ce8a5d75976deaeac3e5ea32ee09d66a533206e12"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⏳️state-activity-loading/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⏳️state-activity-loading/🎯️expect.json",
    "sha256": "b9a619c7f38477c3c86d14352ef916a6bdd4bf97b701bf368c71661a316eacc1"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⏳️state-activity-loading/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⏳️state-activity-loading/📸️snapshot.json",
    "sha256": "405f313fa60937f394294a31ed30b890adac43aa5dc1f790a782d9d061570d43"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⏸️state-activity-waiting/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⏸️state-activity-waiting/🎯️expect.json",
    "sha256": "4f78b5a1d45a66d559541406f5df8503cef25062221a17ddacd50cea2383c604"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⏸️state-activity-waiting/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⏸️state-activity-waiting/📸️snapshot.json",
    "sha256": "9c6f1338f4f7a36e2aa40f9368f8a97a1f6068a90c05927f950ec2320eb133cc"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⛔️state-disabled/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⛔️state-disabled/🎯️expect.json",
    "sha256": "61dd1c799cd94742497935ee7aa88d4a743c7ae75c72b6ea002fc29d68e18d12"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/⛔️state-disabled/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/⛔️state-disabled/📸️snapshot.json",
    "sha256": "628343a79733279009132e8cdfc65497661008c591fe3cbf41f056e1966eb635"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/➖️separator/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/➖️separator/🎯️expect.json",
    "sha256": "9c700bbf64d7d08f9803e454f4f7c4833d7b73402801765e42983a6bb20be160"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/➖️separator/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/➖️separator/📸️snapshot.json",
    "sha256": "92941db119479366aa98e352bfe13db675c55682c2bee9a569495c1d323355a9"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🌳️tree/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🌳️tree/🎯️expect.json",
    "sha256": "5bba856f960272d50b49a9a5bf5d92a8e72e8fe3d2722cac5302db8381fc96d1"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🌳️tree/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🌳️tree/📸️snapshot.json",
    "sha256": "69249a9048a936b3418b282870e79142bd9a0084fc986bb71317d80f97ae8046"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🍽️state-with-menu/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🍽️state-with-menu/🎯️expect.json",
    "sha256": "c444507c739af4fd16f510946b9b2320a07c64522c15f876609e2d50eea876d8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🍽️state-with-menu/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🍽️state-with-menu/📸️snapshot.json",
    "sha256": "08a6ce3fc84a53cc1fb8c9eeb77c35a5c9af03e04d8b68609dbc423f246d5a94"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🎉️state-transition-celebrating/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🎉️state-transition-celebrating/🎯️expect.json",
    "sha256": "2d1e9731c51bb6823c19194994284423d011e55487199e3c892035b391c674fa"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🎉️state-transition-celebrating/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🎉️state-transition-celebrating/📸️snapshot.json",
    "sha256": "39b08e5c95aabe8acbdb217aaa54052ed59232b1b969cf396fb5dd19de1eff8a"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🎚️slider/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🎚️slider/🎯️expect.json",
    "sha256": "84b99447c60df72287b541be4fdf746c16a405377ec9ab3f713bfc9288bedb1b"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🎚️slider/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🎚️slider/📸️snapshot.json",
    "sha256": "2c13c41da9673388d93a530bc6ea96fc13207b4e40a994ae5f858f8ec6c0e1d7"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🎴️icon-select/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🎴️icon-select/🎯️expect.json",
    "sha256": "429151fc611ab1707211191f8857af4919ae03b45f8947e55aba0a0c33b14893"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🎴️icon-select/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🎴️icon-select/📸️snapshot.json",
    "sha256": "bcda9f1fce0616a4e95e472ac8b393653bf98fb060c0e1faa9f8cfadb3b2596d"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🏁️state-activity-finished/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🏁️state-activity-finished/🎯️expect.json",
    "sha256": "fbf29434a45628b8982a40c8943d78644dec9143f684cbe64e842c2cb4de4983"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🏁️state-activity-finished/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🏁️state-activity-finished/📸️snapshot.json",
    "sha256": "299a79af01b0da14059bca33322a5a1f52e9ced2346c095467020fd03eabc60f"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/👋️state-transition-introducing/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/👋️state-transition-introducing/🎯️expect.json",
    "sha256": "7b4449547d38e7e85a99805d66dbc81041031c62536d294fb7f99c825051f5d1"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/👋️state-transition-introducing/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/👋️state-transition-introducing/📸️snapshot.json",
    "sha256": "8aee9ecb089b13f5bbe8ea467a7bf82e84e0bde9b77e15e815d694f8a60709fa"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/💍️ring/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/💍️ring/🎯️expect.json",
    "sha256": "4f71abecfdf56ccf22370eb4627818b7cfec7e946f5ee56463d4b19a19e9f67a"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/💍️ring/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/💍️ring/📸️snapshot.json",
    "sha256": "d471e0ab8bd423962d870046d97e4700faa381fa48c759a1ae61c9c24f02520b"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/📋️key-value-list/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📋️key-value-list/🎯️expect.json",
    "sha256": "ebeb30c90f28807dd9b495bf36bc8caa609cca7056c3f8f7ad581b1bd05137e3"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/📋️key-value-list/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📋️key-value-list/📸️snapshot.json",
    "sha256": "7892409f00ebaa2a8d3b83222f2a9a3c066e321b003296a915a05d254e102b68"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/📝️text/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📝️text/🎯️expect.json",
    "sha256": "f3d5781ba4c8892baccc71a6e2148d2da189fbf95dbdf20f68503e072964c184"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/📝️text/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📝️text/📸️snapshot.json",
    "sha256": "4f2843d427ef7e1002a7db4243864a1de6b7f4d34f1160d0247c16db294e8aa4"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔀️toggle/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔀️toggle/🎯️expect.json",
    "sha256": "ddb344593dd6b81f4d9de15d8767f165d76cde92c69aadce6aab0ba069fa9a13"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔀️toggle/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔀️toggle/📸️snapshot.json",
    "sha256": "a11cb400cc0dbe0254000026345b7aa9b069d54b00648ebcc6d39aaae56832fe"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔌️extension/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔌️extension/🎯️expect.json",
    "sha256": "ef4402376b928dd8b4c6d58ec130d0af719ad6957302a61a3e793b6f2c7f78a9"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔌️extension/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔌️extension/📸️snapshot.json",
    "sha256": "d699b2dd2f79ac5c86fcf63c60c37607b79d35c8346c7671d9c6a77766a8023a"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔘️button/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔘️button/🎯️expect.json",
    "sha256": "9c43b18c7488e6559bc90e5d64aea4032bb0dbaded9bfca095c7fa1d5bf9702f"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔘️button/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔘️button/📸️snapshot.json",
    "sha256": "c61365ce8c31a4af8955f24d1de5c8cc38e9173e216a0b6079eb3811fe06943c"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔢️number-stepper/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔢️number-stepper/🎯️expect.json",
    "sha256": "9563da71325e96a226bcb9c63e4cb77ebc6bf3258fa4e1702628a48c11efb5ad"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔢️number-stepper/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔢️number-stepper/📸️snapshot.json",
    "sha256": "3771e66341b8178c8eb84e310200aa662b5a3515daf568d07fafc6128b552712"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔽️select/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔽️select/🎯️expect.json",
    "sha256": "387a25fae43ef65a1e7b72b3d2db7c257cb8af312650e125c631b8d1bfbfbaa7"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🔽️select/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🔽️select/📸️snapshot.json",
    "sha256": "dd966773d8633689054ed0a3e2159d69781cf38197f3dc0e20386128c3328776"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🖼️image/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🖼️image/🎯️expect.json",
    "sha256": "71a968f38d391c105a59fc415aafc758f28201f4b989120b06d18916a7d8dcf5"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🖼️image/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🖼️image/📸️snapshot.json",
    "sha256": "ee2e09aff10a84b136835415ebbaee7ba411ee8d8af4a7dcd0b98de7a9876b99"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🗺️surface/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🗺️surface/🎯️expect.json",
    "sha256": "3fdbef12cada5f53d5fc1c980ec6538bba13a98a39aee21fb82d61ca3ad89cd5"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🗺️surface/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🗺️surface/📸️snapshot.json",
    "sha256": "ba8c2d0401a4de57e64f8c9bdea8018032412f200601b7d6846c0f447561dc34"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🧺️container/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🧺️container/🎯️expect.json",
    "sha256": "159b00fee4625cf14c8b177fdf50d5ddb474b516b1577ff311a0161f3b83d24b"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧩️component/🧺️container/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🧺️container/📸️snapshot.json",
    "sha256": "279872df874dff523390e5a1781cef19497b276dbd3668f460d665f89258157e"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/⏳️set-activity/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/⏳️set-activity/🎯️expect.json",
    "sha256": "62594e622aaaa7b205f7afcb38eca5ec061b506f0d9448dea045fca4671250eb"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/⏳️set-activity/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/⏳️set-activity/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/⏳️set-activity/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/⏳️set-activity/🩹️patch.json",
    "sha256": "7d6e7e1894f87b7f92ab0406ef9b7f4ffe7b046959d12f0cd472cfbd7ab58607"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/♿️set-accessibility/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/♿️set-accessibility/🎯️expect.json",
    "sha256": "92b12527fda56f3585736a01c226114d0857b1fa06b21f254b5edbb9b867e542"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/♿️set-accessibility/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/♿️set-accessibility/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/♿️set-accessibility/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/♿️set-accessibility/🩹️patch.json",
    "sha256": "152761df6a0d1c899a41b1cc550fc9f4eaf5100193e5a95cc38d669dbbef953f"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🌱️set-root/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🌱️set-root/🎯️expect.json",
    "sha256": "71e2416bbfbda836d072bfc685b37e1dffb9be51cb2af0d871b4f782762a0156"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🌱️set-root/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🌱️set-root/📸️snapshot.json",
    "sha256": "2ea40ab46f7249faafaff167583a691937259f70db83169f0a2e53306294a3a2"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🌱️set-root/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🌱️set-root/🩹️patch.json",
    "sha256": "ffa389d38769f6803a70e364b48af8717e38c1b634e3f7a6d9d092767b4fffca"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🍽️set-menu/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🍽️set-menu/🎯️expect.json",
    "sha256": "b53fbe7506fab83884b553d94750951660018c8aa996b31c8b78d58f76805f6a"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🍽️set-menu/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🍽️set-menu/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🍽️set-menu/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🍽️set-menu/🩹️patch.json",
    "sha256": "69cd048a905286e90adfb8b4faedc2128f0cc6fda08b78b5e1138ba0d5a7d5a2"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🎨️set-style/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🎨️set-style/🎯️expect.json",
    "sha256": "add3814ae083b0208061e2cf799fc8323a4a7aa72dff7370b03427429536f9d3"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🎨️set-style/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🎨️set-style/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🎨️set-style/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🎨️set-style/🩹️patch.json",
    "sha256": "f6ca700e10308eaa269a5a8f739a42bac27b5e53f853fe2a24b0a7fdfc219f4e"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/👶️set-children/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/👶️set-children/🎯️expect.json",
    "sha256": "0217884f3e99db50c507b40e675bb3c775f01e19afe45afb106962fbcd56aabb"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/👶️set-children/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/👶️set-children/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/👶️set-children/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/👶️set-children/🩹️patch.json",
    "sha256": "4260d0c424bb7847c6848a1d7ad654f044b134216c7806ac3b0b3fe4b3b17b48"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/📐️set-layout/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/📐️set-layout/🎯️expect.json",
    "sha256": "2dae32e35d49f081785cb133367811bca8206ace5549702e5bfb3fef48d52d1e"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/📐️set-layout/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/📐️set-layout/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/📐️set-layout/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/📐️set-layout/🩹️patch.json",
    "sha256": "e63116701d4029345623671441048dc537642c95339f0270c9abbf4627fdd863"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/📥️upsert/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/📥️upsert/🎯️expect.json",
    "sha256": "ffc330a0377c9c3a7de897e54dedc6a943c836f0a7153c5fee0d5532bf3dc055"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/📥️upsert/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/📥️upsert/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/📥️upsert/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/📥️upsert/🩹️patch.json",
    "sha256": "8d7f2c288cd24eae9f9bb43648911a83340b43067b44c89fa0024a797f22c140"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🔀️reorder-children/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🔀️reorder-children/🎯️expect.json",
    "sha256": "7ad509426da404140c598ecaddc62dec8a9740bc56fa1f28e90467dd81d3e8a4"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🔀️reorder-children/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🔀️reorder-children/📸️snapshot.json",
    "sha256": "c3d833f1e598ab87ba77600f2f456e7b49ddfb6d3e408c75151319ffb8149861"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🔀️reorder-children/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🔀️reorder-children/🩹️patch.json",
    "sha256": "151961dbe7025f09fa333a36f2525e0d1fb9dfeae58ae0ff0728177b228a67a7"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🔗️set-bindings/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🔗️set-bindings/🎯️expect.json",
    "sha256": "9bcd0a31bad5b21d1dab5c3d6ac4faa223c8d59b32a9c1a142b6cd7dde0e1cce"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🔗️set-bindings/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🔗️set-bindings/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🔗️set-bindings/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🔗️set-bindings/🩹️patch.json",
    "sha256": "c7fd76d288dc01830462b93f36f23001bdca060c8e3d8b80b7e229b94284257a"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🧩️set-component/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🧩️set-component/🎯️expect.json",
    "sha256": "add28d46d0c58998d02c24e3d6e2abe7da7da36601118ff93a5b2c134a49dd87"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🧩️set-component/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🧩️set-component/📸️snapshot.json",
    "sha256": "8975e8fa0eb211f1c5f5828d9417ce1f488e8a0a033b6e98a1ebd2ab9a80f1b8"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🧩️set-component/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🧩️set-component/🩹️patch.json",
    "sha256": "567ec68ccee19462ce470ae230aaf1eb0cd23dba175bae238fcf20a5b75a1614"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🪓️remove-subtree/🎯️expect.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🪓️remove-subtree/🎯️expect.json",
    "sha256": "611eb00030ed1ae0a29b373a40b96e23d9e9ed084f9a1deb789af5a8a5656d40"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🪓️remove-subtree/📸️snapshot.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🪓️remove-subtree/📸️snapshot.json",
    "sha256": "186b732cc0e868e896bc4f2d19f5af01d37c7b499dff87b314e87664c54d5ebd"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🩹️patch/🪓️remove-subtree/🩹️patch.json",
    "newPath": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🩹️patch/🪓️remove-subtree/🩹️patch.json",
    "sha256": "69967d2fd7376f6374a59f892bde8e7a6f4431d6f4e081921794c7961be64da6"
  }
]
```

## Exact Authored Consumer Paths

```json
[
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️conformance-corpus/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️conformance-unit/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx"
]
```

## verify Runtime

The actual conformanceCorpusSelfTests export passed all 62 catalog cases using strict Ajv, exact directory-role comparison and expected-case identities.

The old root is absent; all 147 relocated file hashes equal their retained pre-move hashes.

## Native Consumer Runtime

```json
{
  "status": 0,
  "args": [
    "cargo",
    "test",
    "--locked",
    "--offline",
    "--manifest-path",
    "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml",
    "--all-features",
    "--lib",
    "--",
    "conformance::",
    "--nocapture"
  ],
  "result": [
    "test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.08s"
  ],
  "tail": "   |\nhelp: remove the unnecessary path segments\n   |\n11 -     assert!(!owner.open_into(&mut surface, crate::UiDocumentAssemblyIdentity { generation: 117, revision: UiRevision(4), root: Some(UiNodeId(41)), layout_epoch: 0 }, 1, 0).unwrap().progressed);\n11 +     assert!(!owner.open_into(&mut surface, UiDocumentAssemblyIdentity { generation: 117, revision: UiRevision(4), root: Some(UiNodeId(41)), layout_epoch: 0 }, 1, 0).unwrap().progressed);\n   |\n\nwarning: unnecessary qualification\n  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📃️document/🎟️assembly/🧪️tests/🎟️assembly/🦀️.rs:13:46\n   |\n13 |     let step = owner.open_into(&mut surface, crate::UiDocumentAssemblyIdentity { generation: 117, revision: UiRevision(4), root: Som...\n   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: remove the unnecessary path segments\n   |\n13 -     let step = owner.open_into(&mut surface, crate::UiDocumentAssemblyIdentity { generation: 117, revision: UiRevision(4), root: Some(UiNodeId(41)), layout_epoch: 0 }, 1, 32768).unwrap();\n13 +     let step = owner.open_into(&mut surface, UiDocumentAssemblyIdentity { generation: 117, revision: UiRevision(4), root: Some(UiNodeId(41)), layout_epoch: 0 }, 1, 32768).unwrap();\n   |\n\nwarning: unnecessary qualification\n   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📃️document/🎟️assembly/🧪️tests/🎟️assembly/🦀️.rs:142:65\n    |\n142 |     let open_blocked = matches!(blocked.open_into(&mut surface, crate::UiDocumentAssemblyIdentity { generation: 117, revision: UiRe...\n    |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    |\nhelp: remove the unnecessary path segments\n    |\n142 -     let open_blocked = matches!(blocked.open_into(&mut surface, crate::UiDocumentAssemblyIdentity { generation: 117, revision: UiRevision(4), root: Some(UiNodeId(41)), layout_epoch: 0 }, 1, 32768), Err(error) if error.kind == UiDocumentAssemblyErrorKind::Contended);\n142 +     let open_blocked = matches!(blocked.open_into(&mut surface, UiDocumentAssemblyIdentity { generation: 117, revision: UiRevision(4), root: Some(UiNodeId(41)), layout_epoch: 0 }, 1, 32768), Err(error) if error.kind == UiDocumentAssemblyErrorKind::Contended);\n    |\n\nwarning: `semio-framework-ui-contract` (lib test) generated 7 warnings (run `cargo fix --lib -p semio-framework-ui-contract --tests` to apply 7 suggestions)\n    Finished `test` profile [unoptimized] target(s) in 19.36s\n     Running unittests 🦀️.rs (.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-ui-contract/4112ad685442ed42/out/semio_framework_ui_contract-4112ad685442ed42)\n"
}
```
