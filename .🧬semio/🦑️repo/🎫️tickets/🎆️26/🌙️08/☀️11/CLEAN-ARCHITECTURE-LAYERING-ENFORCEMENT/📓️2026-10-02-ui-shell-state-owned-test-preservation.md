# 🐚️ UI ShellState Explicit Owned Test Axes

## 📐️ Declared Input Contract Before Implementation

The current UI neutral contract has explicit Locale and Terminology axes and refuses missing authority in production. The owned ShellState constructor receives four ordered inputs: plugins, plugin filter, Locale, Terminology. Test setup supplies its original typed axis where one is already in scope; setups without an axis explicitly select neutral Locale::En and Terminology::Native to preserve original fixture expectations. No constructor default, locale fallback, production bootstrap or assertion changes belong to this cohort.

The exact whole physical repository test roster has42 Rust /🧪️tests/ files and161 actual ShellState::new call expressions. Two locale-bearing chrome dialog loops preserve their local `locale`;159 other setups explicitly choose neutral En/Native. The same first two argument expressions, every assertion, fixture include and feature gate must survive byte-identically. Only third/fourth constructor arguments may be inserted. Full fresh originals were closed before edits in 📓️2026-10-02-ui-shell-state-owned-test-original-inputs.md.

## 🔴️ Actual Pre-Implementation Source RED

Observed 2026-10-02T12:51:13.718Z. The actual current161 calls each have two arguments and therefore fail the declared four-input source contract. Tokenized Rust call scope and existing paired delimiter parser identified exact physical call expressions; root-wide rg confirmed there are no additional physical Rust /🧪️tests/ owners outside this42-file roster. This is real source RED, not native compilation or synthetic layout validation. UI High owns the registered language-neutral contract and actual constructor implementation; all original mounted native cohorts remain required.

| Source | Original SHA256 | Calls |
| --- | --- | --- |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs | 57f914a0e7329d1e4cad139412ddefedfd0ef20e7aa57944a52ac4ce4c059495 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs | 85451fb025669c0495940c20533e7e5e3be354a01d452546c4e331d2b742f478 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs | fff1e72cf59647587010806c998cab12bda22accd370dfd0a96e60fe19b05b22 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs | b6fc996dcd5f8957c0fc0e7a482d0734ba8188fc16cd5440a9f1be4670f44ebc | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs | 694032182c23998be77ae65c7c3f3151058b37fdeb43212e09db09b4a821be2a | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs | 92f3b09ac5ec10df0c2164da11e60b1e2aad4a71efabb83fb3cc899e58bdfa03 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs | 9a0c5948940c40b5959889226239d0c818fb25b11429d4cf2a45f33e062a3c91 | 3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs | e0cc6efbbeb6f49bcea6995fad2541c28b3223b55f34b521c74d9c752d3f1a22 | 5 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs | cd200363846f1f7130ba2242fb14fe49e05e144a5b994808d962fbd44acfcd99 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs | 7a4bc8ea689689979a3b9978c92ce3094b65db99d1d5f4acf4f65f7d21405924 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs | 4484b4a7f7d99072f8578f559c75fa8c0dcc9afa8445ad037209d853e37ac4f9 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs | c88a226bb5d9d369f176b7c5bbb05613a41bd5c70ca56f7f1c3777fd702a420f | 3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs | eb6b6c978d9851465d1fce59a383e2e42bb923f9a04903fbcd230a58669791c4 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🚗️driver-editor/🦀️.rs | 38a80366c13e86eab84c70c6f6bcc6ca06437bad95953307112f2bd4275af56d | 3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs | 9a6af92a5a0ceb824fdf32ceeee8b7058c6e72da3474f1bb0dcdc025f84d5456 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️native-keyboard-ring/🦀️.rs | d8379b2a2f66ff0948c77b34ade3a7ee2f225e925d71f9188fa27fbfdb27e075 | 3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs | bd608300e6a259e87e4fb418a6a2434a9e7d638f5f983ae85f66f23e9018f400 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs | d9bb6f9ac8d186b2b00eff3285629d25bbd77a76e637ee8c119f1b2d6646def0 | 3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs | 1f2489d2d23681f3ae5e3bf4f34cff70c843e1e92ac62951e5ad9cab1c281970 | 7 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs | 3447df4be1dd10a96da76dd26cdaec079ea4c77247a709d6947d74fd0a62c1f4 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs | 83276de192786fd36c37a3b46a51bf54badb30c45fa5edaf519bd47d57b96b06 | 3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs | 475fcda31ba4c1f04e2627adfb5c71fc71146ca2a007c154c44fd3f0eb3bc001 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs | cefdacd26cd121d7406875d10b057ff0158f34fa0a3ea440726f6331baf4614b | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs | 81b4a1c373b95cca2a578756269f4b1e1b0755acde0bb89ac48b716cbf40876e | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-agent-overlays/🦀️.rs | a7f7927cbd35547c4d2130e06564fe4132b17de0b9abe5ace65317d66b1289f3 | 21 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs | 8639a6fbcf30e9245571b9aaf4a2a7382b7327f8b931efe684b7825adda90317 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs | f685dd26eb0e78aec0d6b8e27be942e756aa2fd8352967c629d98c49fa79d475 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs | 9247d75871a011e9f5ba36fc789038b50f1f8a5f3a6945bedad2f924c086cc69 | 6 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs | 5d3f04be55cec731f060aac3cad9202e1402eeb876d37576b50b234aa3dc9e34 | 11 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs | 28d741a33ddb05269ad4b2529d260b097c3bc1cdb3c3c75ce96f07a905e26794 | 5 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs | ab1a9bce16645b085fc4f1e3ef4984541bcc3f4350f0d23e38a89a5fbc5a4b94 | 8 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs | d688bbc06c5affc331a82b0b6b88ca66e282b5f69bbc13300116662068735aba | 11 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs | 9966a4f98bc194e55befc1201e6028a33b6ed764684577bba053f3de8b311c42 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs | da62225ada6677b42d7902111238fe2d0315e1e74a89eccf7b5cb9f59da8b356 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs | 43d1ae22f25a6741f05af294d3589460f0d905bd9d24982859a8b7cb829c49f7 | 20 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs | cf7c158b515fe64c9fe1543e1fa7443137f261cff954dbf113533b45b6f6c310 | 8 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs | 4ebacf1a89ecb4835f9779adb8cdfa5763b186596cb6f86f0352e210645428c1 | 5 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-media-frames/🦀️.rs | 72a6945e963febba501c512f74bfe189add252935a60960a258d277d2d849e6c | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs | 10a873d0138e6316d3cca8db981723d1e08e8c18fd0f331d8e6fd8bbe0d27aa3 | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs | 833e8bfed48db975bffbd175c0905f0ca72591e89a4ac6f252fb59c44399b410 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs | d6ef97bd54503235290d823ead489b166339831371caaa3654639be599595634 | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs | 2137a70c4dceb29d6bae24dc736ecd77a5de2a2fcca1c2d9a0e9b4b979a4e21a | 1 |

## 🟢️ Exact Source Transformation Receipt

Applied161 argument insertions across42 fresh owned test sources. Two locale loop values remain unchanged and are passed into the constructor;159 explicit En/Native test setup choices retain original expectations. Every complete source reconstructs its closed original SHA after removing only its declared inserted axes; no original assertion, first/second argument, fixture, scenario or feature token changed.

```json
{
  "at": "2026-10-02T12:53:13.033Z",
  "rows": [
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs",
      "originalSha256": "57f914a0e7329d1e4cad139412ddefedfd0ef20e7aa57944a52ac4ce4c059495",
      "currentSha256": "e44c36b479d7422e2dc3e2af081273f0698d5c05ab0ce417ba8bf139acdabb3c",
      "edits": [
        {
          "start": 9436,
          "end": 9436,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"frame-candidate-retirement\".to_string())",
          "originalLine": 177
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs",
      "originalSha256": "85451fb025669c0495940c20533e7e5e3be354a01d452546c4e331d2b742f478",
      "currentSha256": "ac4d3964197d2093565f20940472035da945100da6672196e4768e3b53089ec1",
      "edits": [
        {
          "start": 100741,
          "end": 100741,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"maintenance-owner\".to_string())",
          "originalLine": 1635
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs",
      "originalSha256": "fff1e72cf59647587010806c998cab12bda22accd370dfd0a96e60fe19b05b22",
      "currentSha256": "2dc788fe493fd1b2064897765287e41835c9c72aa9710aa85a665e787cd34aea",
      "edits": [
        {
          "start": 3768,
          "end": 3768,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"component-asset-close\".to_string())",
          "originalLine": 83
        },
        {
          "start": 39693,
          "end": 39693,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"closed-fixture\".to_string())",
          "originalLine": 677
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs",
      "originalSha256": "b6fc996dcd5f8957c0fc0e7a482d0734ba8188fc16cd5440a9f1be4670f44ebc",
      "currentSha256": "e70f6630d1e70eff9d442684504aa6207c7133a788b7ffa9681b5acd0123ab3f",
      "edits": [
        {
          "start": 3913,
          "end": 3913,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 81
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs",
      "originalSha256": "694032182c23998be77ae65c7c3f3151058b37fdeb43212e09db09b4a821be2a",
      "currentSha256": "c0e6a71f31584c9650338f36d73fd0fb880af6910918a4d643381cefe288d53d",
      "edits": [
        {
          "start": 6790,
          "end": 6790,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 133
        },
        {
          "start": 39462,
          "end": 39462,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(vec![], \"test\".into())",
          "originalLine": 723
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs",
      "originalSha256": "92f3b09ac5ec10df0c2164da11e60b1e2aad4a71efabb83fb3cc899e58bdfa03",
      "currentSha256": "bf59bb47b0113aab7484e1e55b35e02404218d49cd3d2f4b9c6ec1e89f6ee3da",
      "edits": [
        {
          "start": 831,
          "end": 831,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 19
        },
        {
          "start": 4297,
          "end": 4297,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 87
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs",
      "originalSha256": "9a0c5948940c40b5959889226239d0c818fb25b11429d4cf2a45f33e062a3c91",
      "currentSha256": "697a8faa5a175f4479ce7427542585356f0884eb268c332b5c295b21cbdd0344",
      "edits": [
        {
          "start": 3097,
          "end": 3097,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 54
        },
        {
          "start": 30496,
          "end": 30496,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 442
        },
        {
          "start": 35638,
          "end": 35638,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 506
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs",
      "originalSha256": "e0cc6efbbeb6f49bcea6995fad2541c28b3223b55f34b521c74d9c752d3f1a22",
      "currentSha256": "20b0ea7a1906c1d5a81780cb5b002100d1e3341685adc3128cdac44239e13c0c",
      "edits": [
        {
          "start": 718,
          "end": 718,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 11
        },
        {
          "start": 8345,
          "end": 8345,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(vec![bridge], \"space\".into())",
          "originalLine": 125
        },
        {
          "start": 18668,
          "end": 18668,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 266
        },
        {
          "start": 23155,
          "end": 23155,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 337
        },
        {
          "start": 82511,
          "end": 82511,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1218
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs",
      "originalSha256": "cd200363846f1f7130ba2242fb14fe49e05e144a5b994808d962fbd44acfcd99",
      "currentSha256": "64ddb4c4d92d4d8b0b2517e12ec6209f6e2729696bd372bca76695dbaf0920c9",
      "edits": [
        {
          "start": 10248,
          "end": 10248,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 175
        },
        {
          "start": 18588,
          "end": 18588,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 297
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs",
      "originalSha256": "7a4bc8ea689689979a3b9978c92ce3094b65db99d1d5f4acf4f65f7d21405924",
      "currentSha256": "82510aee0207910555f7a5708028760b42c4f6ecdccf2dc6067aaf7a35e3c34b",
      "edits": [
        {
          "start": 2670,
          "end": 2670,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 43
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs",
      "originalSha256": "4484b4a7f7d99072f8578f559c75fa8c0dcc9afa8445ad037209d853e37ac4f9",
      "currentSha256": "8aae99ab9c75b42c2475687fd857b7cdbc93eee40b0a7f59eafe35b65ea247a0",
      "edits": [
        {
          "start": 1695,
          "end": 1695,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 26
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs",
      "originalSha256": "c88a226bb5d9d369f176b7c5bbb05613a41bd5c70ca56f7f1c3777fd702a420f",
      "currentSha256": "ed7e00a88fdb3a5965a26c8e3e0a8d4f0a456a5ddc218864b48ec664c0ccb61e",
      "edits": [
        {
          "start": 9837,
          "end": 9837,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 154
        },
        {
          "start": 13745,
          "end": 13745,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 194
        },
        {
          "start": 17015,
          "end": 17015,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 236
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs",
      "originalSha256": "eb6b6c978d9851465d1fce59a383e2e42bb923f9a04903fbcd230a58669791c4",
      "currentSha256": "768e821e5ed76e4d87f8e0aa0254e59e8f877f3ad247e5c52cea3d607d717d8b",
      "edits": [
        {
          "start": 4550,
          "end": 4550,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 85
        },
        {
          "start": 14509,
          "end": 14509,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 240
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🚗️driver-editor/🦀️.rs",
      "originalSha256": "38a80366c13e86eab84c70c6f6bcc6ca06437bad95953307112f2bd4275af56d",
      "currentSha256": "e53e323270c38560ef75c09de7dad957d84410e8693c87f3a231c2dbf8ef883c",
      "edits": [
        {
          "start": 2005,
          "end": 2005,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 46
        },
        {
          "start": 3525,
          "end": 3525,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 73
        },
        {
          "start": 5814,
          "end": 5814,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 111
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs",
      "originalSha256": "9a6af92a5a0ceb824fdf32ceeee8b7058c6e72da3474f1bb0dcdc025f84d5456",
      "currentSha256": "09c1e94e06e67b1e08bc2ce73547cbcedfa340692f8a2291c78d75e30aa96b05",
      "edits": [
        {
          "start": 49443,
          "end": 49443,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"chrome-width-law\".into())",
          "originalLine": 692
        },
        {
          "start": 50715,
          "end": 50715,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"task-manager-footer-law\".into())",
          "originalLine": 713
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️native-keyboard-ring/🦀️.rs",
      "originalSha256": "d8379b2a2f66ff0948c77b34ade3a7ee2f225e925d71f9188fa27fbfdb27e075",
      "currentSha256": "c6a360eefb06d5a396b4fee61957d6717f9589034b85b574c75ce8ed0aea8ada",
      "edits": [
        {
          "start": 1504,
          "end": 1504,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 24
        },
        {
          "start": 3602,
          "end": 3602,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 55
        },
        {
          "start": 4652,
          "end": 4652,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 71
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs",
      "originalSha256": "bd608300e6a259e87e4fb418a6a2434a9e7d638f5f983ae85f66f23e9018f400",
      "currentSha256": "6b1411d79b338e8e6d661180b1c47164309dbe0270c2d4dd9b80e418d8233917",
      "edits": [
        {
          "start": 3222,
          "end": 3222,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 52
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs",
      "originalSha256": "d9bb6f9ac8d186b2b00eff3285629d25bbd77a76e637ee8c119f1b2d6646def0",
      "currentSha256": "c634bc62cb31e03e19a8bb98d66eb63fdc51cb4b775ecb90e3eef9d39fbe0ace",
      "edits": [
        {
          "start": 6197,
          "end": 6197,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 108
        },
        {
          "start": 9649,
          "end": 9649,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 162
        },
        {
          "start": 20414,
          "end": 20414,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 318
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs",
      "originalSha256": "1f2489d2d23681f3ae5e3bf4f34cff70c843e1e92ac62951e5ad9cab1c281970",
      "currentSha256": "eff15946eeba13c6e4d50f5f90e688abfe35cc5d54f1c6b629991c98f2f29ad7",
      "edits": [
        {
          "start": 6956,
          "end": 6956,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 129
        },
        {
          "start": 10529,
          "end": 10529,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 191
        },
        {
          "start": 11157,
          "end": 11157,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 204
        },
        {
          "start": 11891,
          "end": 11891,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 217
        },
        {
          "start": 18587,
          "end": 18587,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 313
        },
        {
          "start": 21125,
          "end": 21125,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 353
        },
        {
          "start": 44379,
          "end": 44379,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 670
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs",
      "originalSha256": "3447df4be1dd10a96da76dd26cdaec079ea4c77247a709d6947d74fd0a62c1f4",
      "currentSha256": "713468b82d5df37de7a491c87317832f3182706f960952869b46d30af3aec190",
      "edits": [
        {
          "start": 3476,
          "end": 3476,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(vec![], \"dpi-law\".into())",
          "originalLine": 66
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs",
      "originalSha256": "83276de192786fd36c37a3b46a51bf54badb30c45fa5edaf519bd47d57b96b06",
      "currentSha256": "e6f7144805623dc8461580ac18d4ea6a2797c96fe96443d06292a19933aabf2c",
      "edits": [
        {
          "start": 4305,
          "end": 4305,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 77
        },
        {
          "start": 11955,
          "end": 11955,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 196
        },
        {
          "start": 48576,
          "end": 48576,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 702
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs",
      "originalSha256": "475fcda31ba4c1f04e2627adfb5c71fc71146ca2a007c154c44fd3f0eb3bc001",
      "currentSha256": "0769b61b5459b9c54ef48d0487f948186c06cc9a4907105518e2eea77eab8b53",
      "edits": [
        {
          "start": 2444,
          "end": 2444,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"s\".into())",
          "originalLine": 40
        },
        {
          "start": 3167,
          "end": 3167,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"s\".into())",
          "originalLine": 53
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs",
      "originalSha256": "cefdacd26cd121d7406875d10b057ff0158f34fa0a3ea440726f6331baf4614b",
      "currentSha256": "2ffb662e05cce575cf7017a0b75249bab94f9f2a9197194e8786432ddb5fca4b",
      "edits": [
        {
          "start": 7713,
          "end": 7713,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 127
        },
        {
          "start": 9332,
          "end": 9332,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 152
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs",
      "originalSha256": "81b4a1c373b95cca2a578756269f4b1e1b0755acde0bb89ac48b716cbf40876e",
      "currentSha256": "39df7cd0b1d5a18c84eb53fa9984c283c87205b983b1272d4e54d4ad386d1abe",
      "edits": [
        {
          "start": 87,
          "end": 87,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 4
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-agent-overlays/🦀️.rs",
      "originalSha256": "a7f7927cbd35547c4d2130e06564fe4132b17de0b9abe5ace65317d66b1289f3",
      "currentSha256": "a91b8fa366857d2d7bef42a106dde1087c32ce4837316c19d5d90d85bd30c52b",
      "edits": [
        {
          "start": 3095,
          "end": 3095,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 54
        },
        {
          "start": 4114,
          "end": 4114,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 71
        },
        {
          "start": 7405,
          "end": 7405,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 135
        },
        {
          "start": 7979,
          "end": 7979,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 147
        },
        {
          "start": 8675,
          "end": 8675,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 157
        },
        {
          "start": 10197,
          "end": 10197,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 184
        },
        {
          "start": 11055,
          "end": 11055,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 197
        },
        {
          "start": 11818,
          "end": 11818,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 208
        },
        {
          "start": 12462,
          "end": 12462,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 220
        },
        {
          "start": 15785,
          "end": 15785,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 278
        },
        {
          "start": 16160,
          "end": 16160,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 286
        },
        {
          "start": 17194,
          "end": 17194,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 299
        },
        {
          "start": 17941,
          "end": 17941,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 309
        },
        {
          "start": 18654,
          "end": 18654,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 318
        },
        {
          "start": 19561,
          "end": 19561,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 331
        },
        {
          "start": 20646,
          "end": 20646,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 351
        },
        {
          "start": 22642,
          "end": 22642,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 380
        },
        {
          "start": 22852,
          "end": 22852,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 386
        },
        {
          "start": 23157,
          "end": 23157,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 393
        },
        {
          "start": 23680,
          "end": 23680,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 403
        },
        {
          "start": 24406,
          "end": 24406,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 415
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs",
      "originalSha256": "8639a6fbcf30e9245571b9aaf4a2a7382b7327f8b931efe684b7825adda90317",
      "currentSha256": "94aa7128999d919cb83d84510b09c5452126c25fc95cafcd31fa88a7b83a1259",
      "edits": [
        {
          "start": 2678,
          "end": 2678,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 58
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs",
      "originalSha256": "f685dd26eb0e78aec0d6b8e27be942e756aa2fd8352967c629d98c49fa79d475",
      "currentSha256": "d0eff5f4483bba880a344eb64fdac3a98aec990b17243432b8037f8e8e6eb5ff",
      "edits": [
        {
          "start": 48747,
          "end": 48747,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 732
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs",
      "originalSha256": "9247d75871a011e9f5ba36fc789038b50f1f8a5f3a6945bedad2f924c086cc69",
      "currentSha256": "8c987d7a1fc1fb33f15a282534c76f6a7dc30dfd1dcae64fcaea14a413830893",
      "edits": [
        {
          "start": 111,
          "end": 111,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 5
        },
        {
          "start": 35640,
          "end": 35640,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(plugins.clone(), variant.clone())",
          "originalLine": 599
        },
        {
          "start": 35691,
          "end": 35691,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(plugins, variant)",
          "originalLine": 600
        },
        {
          "start": 48097,
          "end": 48097,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(plugins, variant)",
          "originalLine": 780
        },
        {
          "start": 56345,
          "end": 56345,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(plugins, variant)",
          "originalLine": 911
        },
        {
          "start": 67835,
          "end": 67835,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(plugins, variant)",
          "originalLine": 1081
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs",
      "originalSha256": "5d3f04be55cec731f060aac3cad9202e1402eeb876d37576b50b234aa3dc9e34",
      "currentSha256": "af83e94e571863776e6137a6ca2cecb98d60206857f7729455c7d59de2eb1515",
      "edits": [
        {
          "start": 2918,
          "end": 2918,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 66
        },
        {
          "start": 6366,
          "end": 6366,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 111
        },
        {
          "start": 7777,
          "end": 7777,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 131
        },
        {
          "start": 13028,
          "end": 13028,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 196
        },
        {
          "start": 19472,
          "end": 19472,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 278
        },
        {
          "start": 29555,
          "end": 29555,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 398
        },
        {
          "start": 34064,
          "end": 34064,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 458
        },
        {
          "start": 36867,
          "end": 36867,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 491
        },
        {
          "start": 38861,
          "end": 38861,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 520
        },
        {
          "start": 41242,
          "end": 41242,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 552
        },
        {
          "start": 42971,
          "end": 42971,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 571
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs",
      "originalSha256": "28d741a33ddb05269ad4b2529d260b097c3bc1cdb3c3c75ce96f07a905e26794",
      "currentSha256": "9d70d4d574042a62d3ca2dd0713694947425a544bd36af11f76e0207b4746027",
      "edits": [
        {
          "start": 5900,
          "end": 5900,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"generation3d\".into())",
          "originalLine": 113
        },
        {
          "start": 6577,
          "end": 6577,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"generation3d\".into())",
          "originalLine": 126
        },
        {
          "start": 8052,
          "end": 8052,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"generation3d\".into())",
          "originalLine": 147
        },
        {
          "start": 9608,
          "end": 9608,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"generation3d\".into())",
          "originalLine": 172
        },
        {
          "start": 10815,
          "end": 10815,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"generation3d\".into())",
          "originalLine": 193
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs",
      "originalSha256": "ab1a9bce16645b085fc4f1e3ef4984541bcc3f4350f0d23e38a89a5fbc5a4b94",
      "currentSha256": "13606fa220c63d07d9e935fd0034cbc5ee7a484ce27430e7ec3a41c615a23e69",
      "edits": [
        {
          "start": 5956,
          "end": 5956,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 182
        },
        {
          "start": 11296,
          "end": 11296,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 272
        },
        {
          "start": 13676,
          "end": 13676,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 310
        },
        {
          "start": 16629,
          "end": 16629,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 348
        },
        {
          "start": 20290,
          "end": 20290,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 397
        },
        {
          "start": 21346,
          "end": 21346,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 416
        },
        {
          "start": 22672,
          "end": 22672,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 439
        },
        {
          "start": 32530,
          "end": 32530,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 573
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs",
      "originalSha256": "d688bbc06c5affc331a82b0b6b88ca66e282b5f69bbc13300116662068735aba",
      "currentSha256": "928b4c31a29539daed2816a6cb2f84b63dd99ed2f8431ac88e6c5088324c381c",
      "edits": [
        {
          "start": 21055,
          "end": 21055,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 370
        },
        {
          "start": 24495,
          "end": 24495,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 430
        },
        {
          "start": 46942,
          "end": 46942,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 738
        },
        {
          "start": 81637,
          "end": 81637,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1283
        },
        {
          "start": 83182,
          "end": 83182,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1301
        },
        {
          "start": 88291,
          "end": 88291,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1364
        },
        {
          "start": 89928,
          "end": 89928,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1386
        },
        {
          "start": 91789,
          "end": 91789,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1409
        },
        {
          "start": 93006,
          "end": 93006,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1426
        },
        {
          "start": 155223,
          "end": 155223,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 2244
        },
        {
          "start": 156984,
          "end": 156984,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 2272
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs",
      "originalSha256": "9966a4f98bc194e55befc1201e6028a33b6ed764684577bba053f3de8b311c42",
      "currentSha256": "d2cc59e6b0e0e5df33bb5f9bd1fabc715a18a313fa6f0f0548cf9fd58425e0b5",
      "edits": [
        {
          "start": 2699,
          "end": 2699,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(vec![], \"test\".into())",
          "originalLine": 39
        },
        {
          "start": 10836,
          "end": 10836,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(vec![], \"test\".into())",
          "originalLine": 166
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs",
      "originalSha256": "da62225ada6677b42d7902111238fe2d0315e1e74a89eccf7b5cb9f59da8b356",
      "currentSha256": "c062476096e91de1ffc7f03f1ea6eef77752c99b3751f6452a9e17316e3df791",
      "edits": [
        {
          "start": 791,
          "end": 791,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 19
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs",
      "originalSha256": "43d1ae22f25a6741f05af294d3589460f0d905bd9d24982859a8b7cb829c49f7",
      "currentSha256": "12c008402dda22c8b072540cf9cddac5a4fd225bf5466b305bc01d1426dd909d",
      "edits": [
        {
          "start": 3448,
          "end": 3448,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 70
        },
        {
          "start": 5231,
          "end": 5231,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 108
        },
        {
          "start": 7887,
          "end": 7887,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 145
        },
        {
          "start": 18564,
          "end": 18564,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 296
        },
        {
          "start": 23104,
          "end": 23104,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 363
        },
        {
          "start": 24726,
          "end": 24726,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 389
        },
        {
          "start": 25721,
          "end": 25721,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 404
        },
        {
          "start": 27678,
          "end": 27678,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 434
        },
        {
          "start": 29360,
          "end": 29360,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 456
        },
        {
          "start": 31937,
          "end": 31937,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 482
        },
        {
          "start": 34784,
          "end": 34784,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 516
        },
        {
          "start": 45058,
          "end": 45058,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 673
        },
        {
          "start": 47077,
          "end": 47077,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 695
        },
        {
          "start": 52375,
          "end": 52375,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 770
        },
        {
          "start": 57652,
          "end": 57652,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 846
        },
        {
          "start": 58506,
          "end": 58506,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 857
        },
        {
          "start": 64973,
          "end": 64973,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 955
        },
        {
          "start": 68367,
          "end": 68367,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1003
        },
        {
          "start": 74088,
          "end": 74088,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1093
        },
        {
          "start": 78362,
          "end": 78362,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 1155
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs",
      "originalSha256": "cf7c158b515fe64c9fe1543e1fa7443137f261cff954dbf113533b45b6f6c310",
      "currentSha256": "2511a83c3c80de3e3d5d9d49995e18c1215a392a449bdbd982f9c043d1efa93c",
      "edits": [
        {
          "start": 5687,
          "end": 5687,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 126
        },
        {
          "start": 7736,
          "end": 7736,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 170
        },
        {
          "start": 14376,
          "end": 14376,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 279
        },
        {
          "start": 16769,
          "end": 16769,
          "previous": "",
          "current": ", locale, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "locale",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "original typed locale loop",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 314
        },
        {
          "start": 19848,
          "end": 19848,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 348
        },
        {
          "start": 22610,
          "end": 22610,
          "previous": "",
          "current": ", locale, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "locale",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "original typed locale loop",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 380
        },
        {
          "start": 25358,
          "end": 25358,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 415
        },
        {
          "start": 31231,
          "end": 31231,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 526
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs",
      "originalSha256": "4ebacf1a89ecb4835f9779adb8cdfa5763b186596cb6f86f0352e210645428c1",
      "currentSha256": "c881b01363e5ebeae5090b5cd5e780a149160ed1964901c93f496f53f2ef5fc7",
      "edits": [
        {
          "start": 1806,
          "end": 1806,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 26
        },
        {
          "start": 5265,
          "end": 5265,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 89
        },
        {
          "start": 6033,
          "end": 6033,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 100
        },
        {
          "start": 7044,
          "end": 7044,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 116
        },
        {
          "start": 10255,
          "end": 10255,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 163
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-media-frames/🦀️.rs",
      "originalSha256": "72a6945e963febba501c512f74bfe189add252935a60960a258d277d2d849e6c",
      "currentSha256": "1cb29949e83d92172625e827ecbed5a6b4abd40192ac9be3a17420b353ffeaf5",
      "edits": [
        {
          "start": 180,
          "end": 180,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 6
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs",
      "originalSha256": "10a873d0138e6316d3cca8db981723d1e08e8c18fd0f331d8e6fd8bbe0d27aa3",
      "currentSha256": "b66877a158725bef0d5a6f29207b1a91e789971ecd53475f02606ac0e33241b8",
      "edits": [
        {
          "start": 12794,
          "end": 12794,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 213
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs",
      "originalSha256": "833e8bfed48db975bffbd175c0905f0ca72591e89a4ac6f252fb59c44399b410",
      "currentSha256": "7b6ee1685237a694a4a9f0f9f23feaa3ceb683d5edefeea96b9e8f9a5a8f13a2",
      "edits": [
        {
          "start": 6212,
          "end": 6212,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(vec![], \"test\".into())",
          "originalLine": 103
        },
        {
          "start": 8296,
          "end": 8296,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(vec![], \"test\".into())",
          "originalLine": 137
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs",
      "originalSha256": "d6ef97bd54503235290d823ead489b166339831371caaa3654639be599595634",
      "currentSha256": "81d49fdbb879c27b3d6f26e514c32673963b601e95a49d83ad0da77565462015",
      "edits": [
        {
          "start": 27055,
          "end": 27055,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 419
        },
        {
          "start": 36369,
          "end": 36369,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), String::new())",
          "originalLine": 548
        }
      ],
      "normalizedEquivalent": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs",
      "originalSha256": "2137a70c4dceb29d6bae24dc736ecd77a5de2a2fcca1c2d9a0e9b4b979a4e21a",
      "currentSha256": "929bc3cdc9eae42338189f88913df04da9936d9865a501e403926f88f9399949",
      "edits": [
        {
          "start": 82481,
          "end": 82481,
          "previous": "",
          "current": ", semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native",
          "kind": "ShellState-explicit-test-axis",
          "locale": "semio_framework_ui_locale::Locale::En",
          "terminology": "semio_framework_ui_locale::Terminology::Native",
          "authority": "explicit owned test setup",
          "originalCall": "ShellState::new(Vec::new(), \"text-editor-runtime-space\".into())",
          "originalLine": 1325
        }
      ],
      "normalizedEquivalent": true
    }
  ]
}
```

## 🌐️ Locale Resolution Selected Input Contract and RED

The actual locale resolver now receives a selected typed Locale as third input. Four existing source calls lack that input (real source RED4/4). Tests explicitly provide neutral En while preserving the lock/store/host precedence assertions and every expected string. The final absent-host expected en derives from this declared caller input; the historical diagnostic string remains unchanged. No unsupported-language fallback is added.

```json
{
  "at": "2026-10-02T12:53:56.684Z",
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs",
  "sha256": "2ffb662e05cce575cf7017a0b75249bab94f9f2a9197194e8786432ddb5fca4b",
  "edits": [
    {
      "start": 15252,
      "end": 15252,
      "previous": "",
      "current": ", semio_framework_ui_locale::Locale::En",
      "kind": "selected-locale-test-axis",
      "originalCall": "resolve_locale_id(None, &stored(None))"
    },
    {
      "start": 15372,
      "end": 15372,
      "previous": "",
      "current": ", semio_framework_ui_locale::Locale::En",
      "kind": "selected-locale-test-axis",
      "originalCall": "resolve_locale_id(None, &stored(Some(OsUiLocale::En)))"
    },
    {
      "start": 15509,
      "end": 15509,
      "previous": "",
      "current": ", semio_framework_ui_locale::Locale::En",
      "kind": "selected-locale-test-axis",
      "originalCall": "resolve_locale_id(Some(\"en\".to_string()), &stored(Some(OsUiLocale::De)))"
    },
    {
      "start": 15627,
      "end": 15627,
      "previous": "",
      "current": ", semio_framework_ui_locale::Locale::En",
      "kind": "selected-locale-test-axis",
      "originalCall": "resolve_locale_id(None, &stored(None))"
    }
  ]
}
```

## 🧪️ Current Full Cohort Source Receipt

Observed 2026-10-02T12:54:45.073Z. Actual42 source files /165 owned axis edits (161 ShellState setup calls plus4 resolver selected inputs). Every fresh current source is exactly its closed original body with only its declared constructor/selected-axis additions;4319 original assertion macros remain. Full physical repo Rust test census has161 four-argument ShellState calls and4 three-argument resolver calls, zero arity deficits. Source checks 292. Ordered source row SHA256 e47b778cf75de85d5bf483097e1d645cd66118d2a7ced7496c67f38358c5014c. No native/compiler route was executed here; original mounted renderer and shell law cohorts remain required. UI High owns direct provider manifest closure and registered neutral source/fixture tests.

```json
{
  "at": "2026-10-02T12:54:45.073Z",
  "files": 42,
  "edits": 165,
  "shellCalls": 161,
  "resolveCalls": 4,
  "assertionMacros": 4319,
  "checks": 292,
  "deficits": [],
  "nativeExecuted": false,
  "rows": [
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs",
      "originalSha256": "57f914a0e7329d1e4cad139412ddefedfd0ef20e7aa57944a52ac4ce4c059495",
      "currentSha256": "e44c36b479d7422e2dc3e2af081273f0698d5c05ab0ce417ba8bf139acdabb3c",
      "ownEdits": 1,
      "originalAssertions": 44,
      "currentAssertions": 44,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs",
      "originalSha256": "85451fb025669c0495940c20533e7e5e3be354a01d452546c4e331d2b742f478",
      "currentSha256": "ac4d3964197d2093565f20940472035da945100da6672196e4768e3b53089ec1",
      "ownEdits": 1,
      "originalAssertions": 405,
      "currentAssertions": 405,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs",
      "originalSha256": "fff1e72cf59647587010806c998cab12bda22accd370dfd0a96e60fe19b05b22",
      "currentSha256": "2dc788fe493fd1b2064897765287e41835c9c72aa9710aa85a665e787cd34aea",
      "ownEdits": 2,
      "originalAssertions": 72,
      "currentAssertions": 72,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs",
      "originalSha256": "b6fc996dcd5f8957c0fc0e7a482d0734ba8188fc16cd5440a9f1be4670f44ebc",
      "currentSha256": "e70f6630d1e70eff9d442684504aa6207c7133a788b7ffa9681b5acd0123ab3f",
      "ownEdits": 1,
      "originalAssertions": 32,
      "currentAssertions": 32,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs",
      "originalSha256": "694032182c23998be77ae65c7c3f3151058b37fdeb43212e09db09b4a821be2a",
      "currentSha256": "c0e6a71f31584c9650338f36d73fd0fb880af6910918a4d643381cefe288d53d",
      "ownEdits": 2,
      "originalAssertions": 273,
      "currentAssertions": 273,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs",
      "originalSha256": "92f3b09ac5ec10df0c2164da11e60b1e2aad4a71efabb83fb3cc899e58bdfa03",
      "currentSha256": "bf59bb47b0113aab7484e1e55b35e02404218d49cd3d2f4b9c6ec1e89f6ee3da",
      "ownEdits": 2,
      "originalAssertions": 31,
      "currentAssertions": 31,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs",
      "originalSha256": "9a0c5948940c40b5959889226239d0c818fb25b11429d4cf2a45f33e062a3c91",
      "currentSha256": "697a8faa5a175f4479ce7427542585356f0884eb268c332b5c295b21cbdd0344",
      "ownEdits": 3,
      "originalAssertions": 102,
      "currentAssertions": 102,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs",
      "originalSha256": "e0cc6efbbeb6f49bcea6995fad2541c28b3223b55f34b521c74d9c752d3f1a22",
      "currentSha256": "20b0ea7a1906c1d5a81780cb5b002100d1e3341685adc3128cdac44239e13c0c",
      "ownEdits": 5,
      "originalAssertions": 244,
      "currentAssertions": 244,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs",
      "originalSha256": "cd200363846f1f7130ba2242fb14fe49e05e144a5b994808d962fbd44acfcd99",
      "currentSha256": "64ddb4c4d92d4d8b0b2517e12ec6209f6e2729696bd372bca76695dbaf0920c9",
      "ownEdits": 2,
      "originalAssertions": 74,
      "currentAssertions": 74,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs",
      "originalSha256": "7a4bc8ea689689979a3b9978c92ce3094b65db99d1d5f4acf4f65f7d21405924",
      "currentSha256": "82510aee0207910555f7a5708028760b42c4f6ecdccf2dc6067aaf7a35e3c34b",
      "ownEdits": 1,
      "originalAssertions": 47,
      "currentAssertions": 47,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs",
      "originalSha256": "4484b4a7f7d99072f8578f559c75fa8c0dcc9afa8445ad037209d853e37ac4f9",
      "currentSha256": "8aae99ab9c75b42c2475687fd857b7cdbc93eee40b0a7f59eafe35b65ea247a0",
      "ownEdits": 1,
      "originalAssertions": 14,
      "currentAssertions": 14,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs",
      "originalSha256": "c88a226bb5d9d369f176b7c5bbb05613a41bd5c70ca56f7f1c3777fd702a420f",
      "currentSha256": "ed7e00a88fdb3a5965a26c8e3e0a8d4f0a456a5ddc218864b48ec664c0ccb61e",
      "ownEdits": 3,
      "originalAssertions": 58,
      "currentAssertions": 58,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs",
      "originalSha256": "eb6b6c978d9851465d1fce59a383e2e42bb923f9a04903fbcd230a58669791c4",
      "currentSha256": "768e821e5ed76e4d87f8e0aa0254e59e8f877f3ad247e5c52cea3d607d717d8b",
      "ownEdits": 2,
      "originalAssertions": 43,
      "currentAssertions": 43,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🚗️driver-editor/🦀️.rs",
      "originalSha256": "38a80366c13e86eab84c70c6f6bcc6ca06437bad95953307112f2bd4275af56d",
      "currentSha256": "e53e323270c38560ef75c09de7dad957d84410e8693c87f3a231c2dbf8ef883c",
      "ownEdits": 3,
      "originalAssertions": 25,
      "currentAssertions": 25,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs",
      "originalSha256": "9a6af92a5a0ceb824fdf32ceeee8b7058c6e72da3474f1bb0dcdc025f84d5456",
      "currentSha256": "09c1e94e06e67b1e08bc2ce73547cbcedfa340692f8a2291c78d75e30aa96b05",
      "ownEdits": 2,
      "originalAssertions": 138,
      "currentAssertions": 138,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️native-keyboard-ring/🦀️.rs",
      "originalSha256": "d8379b2a2f66ff0948c77b34ade3a7ee2f225e925d71f9188fa27fbfdb27e075",
      "currentSha256": "c6a360eefb06d5a396b4fee61957d6717f9589034b85b574c75ce8ed0aea8ada",
      "ownEdits": 3,
      "originalAssertions": 11,
      "currentAssertions": 11,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs",
      "originalSha256": "bd608300e6a259e87e4fb418a6a2434a9e7d638f5f983ae85f66f23e9018f400",
      "currentSha256": "6b1411d79b338e8e6d661180b1c47164309dbe0270c2d4dd9b80e418d8233917",
      "ownEdits": 1,
      "originalAssertions": 8,
      "currentAssertions": 8,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs",
      "originalSha256": "d9bb6f9ac8d186b2b00eff3285629d25bbd77a76e637ee8c119f1b2d6646def0",
      "currentSha256": "c634bc62cb31e03e19a8bb98d66eb63fdc51cb4b775ecb90e3eef9d39fbe0ace",
      "ownEdits": 3,
      "originalAssertions": 40,
      "currentAssertions": 40,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs",
      "originalSha256": "1f2489d2d23681f3ae5e3bf4f34cff70c843e1e92ac62951e5ad9cab1c281970",
      "currentSha256": "eff15946eeba13c6e4d50f5f90e688abfe35cc5d54f1c6b629991c98f2f29ad7",
      "ownEdits": 7,
      "originalAssertions": 133,
      "currentAssertions": 133,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs",
      "originalSha256": "3447df4be1dd10a96da76dd26cdaec079ea4c77247a709d6947d74fd0a62c1f4",
      "currentSha256": "713468b82d5df37de7a491c87317832f3182706f960952869b46d30af3aec190",
      "ownEdits": 1,
      "originalAssertions": 10,
      "currentAssertions": 10,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs",
      "originalSha256": "83276de192786fd36c37a3b46a51bf54badb30c45fa5edaf519bd47d57b96b06",
      "currentSha256": "e6f7144805623dc8461580ac18d4ea6a2797c96fe96443d06292a19933aabf2c",
      "ownEdits": 3,
      "originalAssertions": 206,
      "currentAssertions": 206,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs",
      "originalSha256": "475fcda31ba4c1f04e2627adfb5c71fc71146ca2a007c154c44fd3f0eb3bc001",
      "currentSha256": "0769b61b5459b9c54ef48d0487f948186c06cc9a4907105518e2eea77eab8b53",
      "ownEdits": 2,
      "originalAssertions": 35,
      "currentAssertions": 35,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs",
      "originalSha256": "cefdacd26cd121d7406875d10b057ff0158f34fa0a3ea440726f6331baf4614b",
      "currentSha256": "9711ee756e01e7e0ca40e938b79179930ac71dc48b1ea7da5a7929e031bb0bd5",
      "ownEdits": 6,
      "originalAssertions": 57,
      "currentAssertions": 57,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs",
      "originalSha256": "81b4a1c373b95cca2a578756269f4b1e1b0755acde0bb89ac48b716cbf40876e",
      "currentSha256": "39df7cd0b1d5a18c84eb53fa9984c283c87205b983b1272d4e54d4ad386d1abe",
      "ownEdits": 1,
      "originalAssertions": 65,
      "currentAssertions": 65,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-agent-overlays/🦀️.rs",
      "originalSha256": "a7f7927cbd35547c4d2130e06564fe4132b17de0b9abe5ace65317d66b1289f3",
      "currentSha256": "a91b8fa366857d2d7bef42a106dde1087c32ce4837316c19d5d90d85bd30c52b",
      "ownEdits": 21,
      "originalAssertions": 91,
      "currentAssertions": 91,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs",
      "originalSha256": "8639a6fbcf30e9245571b9aaf4a2a7382b7327f8b931efe684b7825adda90317",
      "currentSha256": "94aa7128999d919cb83d84510b09c5452126c25fc95cafcd31fa88a7b83a1259",
      "ownEdits": 1,
      "originalAssertions": 161,
      "currentAssertions": 161,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs",
      "originalSha256": "f685dd26eb0e78aec0d6b8e27be942e756aa2fd8352967c629d98c49fa79d475",
      "currentSha256": "d0eff5f4483bba880a344eb64fdac3a98aec990b17243432b8037f8e8e6eb5ff",
      "ownEdits": 1,
      "originalAssertions": 175,
      "currentAssertions": 175,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs",
      "originalSha256": "9247d75871a011e9f5ba36fc789038b50f1f8a5f3a6945bedad2f924c086cc69",
      "currentSha256": "8c987d7a1fc1fb33f15a282534c76f6a7dc30dfd1dcae64fcaea14a413830893",
      "ownEdits": 6,
      "originalAssertions": 97,
      "currentAssertions": 97,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs",
      "originalSha256": "5d3f04be55cec731f060aac3cad9202e1402eeb876d37576b50b234aa3dc9e34",
      "currentSha256": "af83e94e571863776e6137a6ca2cecb98d60206857f7729455c7d59de2eb1515",
      "ownEdits": 11,
      "originalAssertions": 98,
      "currentAssertions": 98,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs",
      "originalSha256": "28d741a33ddb05269ad4b2529d260b097c3bc1cdb3c3c75ce96f07a905e26794",
      "currentSha256": "9d70d4d574042a62d3ca2dd0713694947425a544bd36af11f76e0207b4746027",
      "ownEdits": 5,
      "originalAssertions": 36,
      "currentAssertions": 36,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs",
      "originalSha256": "ab1a9bce16645b085fc4f1e3ef4984541bcc3f4350f0d23e38a89a5fbc5a4b94",
      "currentSha256": "13606fa220c63d07d9e935fd0034cbc5ee7a484ce27430e7ec3a41c615a23e69",
      "ownEdits": 8,
      "originalAssertions": 58,
      "currentAssertions": 58,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs",
      "originalSha256": "d688bbc06c5affc331a82b0b6b88ca66e282b5f69bbc13300116662068735aba",
      "currentSha256": "928b4c31a29539daed2816a6cb2f84b63dd99ed2f8431ac88e6c5088324c381c",
      "ownEdits": 11,
      "originalAssertions": 406,
      "currentAssertions": 406,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs",
      "originalSha256": "9966a4f98bc194e55befc1201e6028a33b6ed764684577bba053f3de8b311c42",
      "currentSha256": "d2cc59e6b0e0e5df33bb5f9bd1fabc715a18a313fa6f0f0548cf9fd58425e0b5",
      "ownEdits": 2,
      "originalAssertions": 30,
      "currentAssertions": 30,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs",
      "originalSha256": "da62225ada6677b42d7902111238fe2d0315e1e74a89eccf7b5cb9f59da8b356",
      "currentSha256": "c062476096e91de1ffc7f03f1ea6eef77752c99b3751f6452a9e17316e3df791",
      "ownEdits": 1,
      "originalAssertions": 12,
      "currentAssertions": 12,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs",
      "originalSha256": "43d1ae22f25a6741f05af294d3589460f0d905bd9d24982859a8b7cb829c49f7",
      "currentSha256": "12c008402dda22c8b072540cf9cddac5a4fd225bf5466b305bc01d1426dd909d",
      "ownEdits": 20,
      "originalAssertions": 191,
      "currentAssertions": 191,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs",
      "originalSha256": "cf7c158b515fe64c9fe1543e1fa7443137f261cff954dbf113533b45b6f6c310",
      "currentSha256": "2511a83c3c80de3e3d5d9d49995e18c1215a392a449bdbd982f9c043d1efa93c",
      "ownEdits": 8,
      "originalAssertions": 134,
      "currentAssertions": 134,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs",
      "originalSha256": "4ebacf1a89ecb4835f9779adb8cdfa5763b186596cb6f86f0352e210645428c1",
      "currentSha256": "c881b01363e5ebeae5090b5cd5e780a149160ed1964901c93f496f53f2ef5fc7",
      "ownEdits": 5,
      "originalAssertions": 32,
      "currentAssertions": 32,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-media-frames/🦀️.rs",
      "originalSha256": "72a6945e963febba501c512f74bfe189add252935a60960a258d277d2d849e6c",
      "currentSha256": "1cb29949e83d92172625e827ecbed5a6b4abd40192ac9be3a17420b353ffeaf5",
      "ownEdits": 1,
      "originalAssertions": 22,
      "currentAssertions": 22,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs",
      "originalSha256": "10a873d0138e6316d3cca8db981723d1e08e8c18fd0f331d8e6fd8bbe0d27aa3",
      "currentSha256": "b66877a158725bef0d5a6f29207b1a91e789971ecd53475f02606ac0e33241b8",
      "ownEdits": 1,
      "originalAssertions": 86,
      "currentAssertions": 86,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs",
      "originalSha256": "833e8bfed48db975bffbd175c0905f0ca72591e89a4ac6f252fb59c44399b410",
      "currentSha256": "7b6ee1685237a694a4a9f0f9f23feaa3ceb683d5edefeea96b9e8f9a5a8f13a2",
      "ownEdits": 2,
      "originalAssertions": 15,
      "currentAssertions": 15,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs",
      "originalSha256": "d6ef97bd54503235290d823ead489b166339831371caaa3654639be599595634",
      "currentSha256": "81d49fdbb879c27b3d6f26e514c32673963b601e95a49d83ad0da77565462015",
      "ownEdits": 2,
      "originalAssertions": 100,
      "currentAssertions": 100,
      "exactOriginalProjection": true
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs",
      "originalSha256": "2137a70c4dceb29d6bae24dc736ecd77a5de2a2fcca1c2d9a0e9b4b979a4e21a",
      "currentSha256": "929bc3cdc9eae42338189f88913df04da9936d9865a501e403926f88f9399949",
      "ownEdits": 1,
      "originalAssertions": 408,
      "currentAssertions": 408,
      "exactOriginalProjection": true
    }
  ],
  "digest": "e47b778cf75de85d5bf483097e1d645cd66118d2a7ced7496c67f38358c5014c"
}
```
