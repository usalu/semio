# Current Declared Production Manifest Independent Review

Bounded lexical authored manifest inventory (normal/build sections plus feature declarations) for current root/framework and selected normal Dev6/framework/kernel/WGPU identities. This is declared source evidence, not parsed Cargo resolution or compiled feature proof. Workspace dependency catalogs alone do not activate a runtime dependency; dev-dependencies remain separate.

```json
[
  {
    "path": "Cargo.toml",
    "sha256": "5a42e96244a2de1e04bc7ca800194ad5cf420212a8d787be2983f0ede0c055e9",
    "flagged": []
  },
  {
    "path": "🧰️framework/Cargo.toml",
    "sha256": "ef3cb54878fe4bf6c4c125a8546605c1a8f3daa3bff62f676a128a081d7b5714",
    "flagged": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "005a7ebc50745980673dbe0f9a481323dddfaa3793d40e8f8787ecd8797bb1aa",
    "flagged": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "1cc8e0e36176aaa6d2b171103de20c8d1ef24b228c957e0a35fbe5e8f0c7d0e5",
    "flagged": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "d9a18024ed498f56f87d35ddad0dce80bc3d6c952f6fe2c496caf2718a3895ce",
    "flagged": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "1a18190f2a82b3397835cdab23c47e984ea0ebbc849731e94973ae7ea1d030a6",
    "flagged": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "78bdeb74db5ef0e138def478c25203e1fb24eb32f7e2716c04ca2c9b60fde729",
    "flagged": [
      {
        "line": 30,
        "section": "[features]",
        "text": "mutation-testing = []"
      }
    ]
  },
  {
    "path": "🧰️framework/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "daba99b3c93c1fa37472d6bfcc19d6830cc34fbe3de3ec79a23b423647e7cc54",
    "flagged": []
  },
  {
    "path": "🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "01d307e20cffc1eb4335973c6b5ef74b268c6d0b9dc6b799f706328acbf4a027",
    "flagged": []
  },
  {
    "path": "🌎️hub/🧩️compositions/✒️writer/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "bbbc129f09f22c6ba53dba70a3b00e7338da0ebab679f3b82fdff2585afa09cd",
    "flagged": []
  },
  {
    "path": "🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "4c027192aa168cbf6beedb863428de6fb139f07b7e904d970849eed375731370",
    "flagged": []
  },
  {
    "path": "🌎️hub/🧩️compositions/🗒️note/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "36003faac51690ff92d9bdeab3ce51fef62abb9d347ffec124d82ac58a5afd8c",
    "flagged": []
  },
  {
    "path": "🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "ab2b9052c7492f34050a2bee5d6214e0715e441407cb786cfa50b9dfffc9c75d",
    "flagged": []
  },
  {
    "path": "🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/Cargo.toml",
    "sha256": "55e69bf4d4366a2370aa417724e6b9a7d7a259860c91b230bc1102fea3e9e51e",
    "flagged": []
  }
]
```

Flagged feature definitions require distinction from default/normal activation: merely declaring an empty mutation-testing opt-in is not a production request. No Cargo compile/metadata or full graph acquisition was executed.
