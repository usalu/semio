# 🧯️ Remaining Rust failures A: mounted-layout and shell chrome-4

## Results
| crate | before | after |
| --- | --- | --- |
| `semio-ui-wgpu-mounted-layout` | 23/24 | 24/24 |
| `semio-renderer-wgpu-shell-tests-chrome-4` | 98/102 | 102/102 |
| `semio-ui-wgpu-paint-laws` | 110/110 | 110/110 |

## 1. `mounted_layout_worker_runs_on_the_original_caller_turn`
- Root cause: the fixture `close_owner` asserted the caller's total turn receipt (`self.receipt()`, a delta of cumulative spend) equals the close step's progress. Total spend also contains the span held for the Source's reserved return (1 item, 118 bytes), which the job contract deliberately excludes from the fresh step receipt (`StepContext::fresh_retained_progress_since`, `original_worker_current_progress`). So the product was right and the fixture compared against the wrong quantity.
- Fix: `close_owner` measures `fresh_retained_progress_since` inside its turn and asserts that equals `step.progress()`.
- File: `🧰️framework/🔨️modules/🧵️job/👷️worker/🧪️testing/🦀️.rs`.

## 2. chrome-4, three "no accepting lifetime registration" tests
- Root cause: the UI value arena now needs an actual registered Source lifetime. Tests that touch the arena must own a `UiArenaFixture` (the established pattern of the other UI crates); these three did not. `theme_host_mutations_...` additionally left its retained Theme documents alive at fixture close ("descendants remain borrowed").
- Fix: each of the three tests starts with `let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();`; the crate gains the dev-dependency `semio-ui-contract-values` (feature `testkit`); the theme-publication test retires its panel documents through `retire_one_surface_document`, drops the shell and returns the value pages before the fixture closes.
- Files: `🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/{♿️accessible-names-and-activation,🔁️theme-tokens-and-leaves,🎚️theme-publication-and-sections}/🦀️.rs`, `🌴️shell-tests-chrome-4/📦️packages/🦀️rust/Cargo.toml`.

## 3. `a_discarded_chrome_walk_cannot_exhaust_successor_accessible_names`
- Root cause: chrome-walk setup now clears the accessible-name set incrementally (one `pop_first` per step, setup step 23), so one `render_chrome_step` turn no longer empties it. The law (a discarded walk leaves no names behind) is unchanged.
- Fix: the test drives turns until the set is empty, bounded by `SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY + 64`, then asserts empty.
- File: `🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🧭️settings-strip-and-keybindings/🦀️.rs`.

## Notes
- Another agent repeatedly deleted `🗑️generated`; the last runs logged to the session scratchpad instead.
- Transient breakage from other agents' edits (`AuthoringWallet::self_funded`) cleared by itself.
