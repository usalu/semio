# Layout dependency metadata

The Puzzle Hub build stopped while parsing the Layout mutation decisions manifest. Cargo reference `e7012b7f` recorded an inherited development dependency on `semio-framework-value-clone-source-testing`, which the Layout workspace does not declare.

The live mutation decisions manifest already declared the testing crate by a relative path when checked. Its workspace is `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml`. The dependency resolves to `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧪️testing/📦️packages/🦀️rust/Cargo.toml`, whose package name matches the declaration. The decisions manifest is at `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🔎️decisions/📦️packages/🦀️rust/Cargo.toml`.

The Layout workspace lock at `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.lock` already included the testing package. Its decisions member now records the declared development dependency on that package.

Full locked offline Cargo metadata resolved both Layout and Puzzle Hub through the repository's Cargo launcher. The Layout graph confirms that Layout decisions depends on the local clone-source testing package for development under every target. The Puzzle Hub graph includes Layout decisions and all nine of its production dependencies.
