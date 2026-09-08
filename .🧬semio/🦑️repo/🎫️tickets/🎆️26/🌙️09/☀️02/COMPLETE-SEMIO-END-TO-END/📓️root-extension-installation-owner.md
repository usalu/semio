# Extension Installation Owner and Renderer Compile Closure

The renderer compile exposed a real extension packaging omission: `ExtensionPackageManifestRecord` requires the authored emoji-prefixed installation directory, but the shared Cargo package producer dropped it. The parser now derives that owner from the existing `extension/📦️packages/🦀️rust/Cargo.toml` layout, and the package writer passes it into the actual manifest. No guessed emoji, compatibility fallback or migrated assets were added.

The new neutral fixture covers Flow Math, Process Wood and CAD AEC Building. The registered repo-library test first failed65297 because `directoryName` was absent, then passed15692 with20 assertions, three independent @iarna/toml comparisons and one hostile schema rejection. The DEBUG result was read. No Cargo/component packaging or installation runtime proof is claimed from this parser test.

The other two repo-library compiler sites were narrowed without changing their workflows: the existing bounded synchronous CLI retry uses standard Atomics.wait instead of an undeclared Bun global, and the SVG timeline callback explicitly accepts number. No modifying Git command was run. The actual retry/capture paths were not exercised by this packet.

Renderer87580 passed its full registered typecheck after these changes and the fleet's tutorial fixes. This is compilation, not full frontend functionality. Tutorial selection still required real record/replay integration and was explicitly reassigned. AgentBridge4227 passed31 again after its command began awaiting the test runner promise. Focused PluginRuntime21422 passed10 tests, with54 skipped. All claims remain narrower than the full OS/Hub/AI collaboration goal.
