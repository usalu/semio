# Block List Localized Accessible Presentation

The retained WGPU Block List now receives immutable localized scene chrome from the Shell for `Steps`, `Add Step`, and `Delete`, with English and German resolved through the existing locale authority. Its palette begins at the rail padding used by React. The WGPU painter no longer invents a `Palette` heading or an in-scene `No steps` message, and it does not put literal English copy into the scene painter.

The accepted scene produces bounded virtual accessibility buttons for Add Step, visible delete actions, and palette entries. Candidate controls are sealed and acknowledged with the presented frame; rejected candidates do not become interactive. Activation resolves the accepted key against the current retained Block List, exact host, controller, step, block, or palette kind before publishing the original action descriptor. A replaced controller or retired palette entry cannot replay the old accepted action.

The React Block List exposes its palette rows as named keyboard buttons. Enter and Space publish the same `addBlock` descriptor as pointer activation. Step and block delete controls now use the localized common-delete name. The retained canvas projection publishes localized button names, tabbability, actionability, and accepted rectangles through the existing accessibility mirror.

The strict shared fixture and schema are at `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🧩️block-list-presentation/🔣️.json` and `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🧩️block-list-presentation/🔣️.json`. It covers English and German chrome, the empty step body, one palette action, the absent invented copy, and exact add-step/add-block descriptors.

Validation:

- Actual React oracle `renders only the shared localized chrome and exposes palette keyboard activation`: one focused test passed after the final palette-button structure and shared-fixture path update. It validates the strict schema, switches the real UI locale between English and German, mounts the production `BlockListHost`, and verifies localized chrome, absent invented headings, pointer Add Step, and palette Enter/Space dispatch. Receipt: `🗑️generated/sol-block-list/react-3.log`.
- Native scene law `localized_empty_block_list_publishes_only_accepted_actionable_controls_and_rejects_a_retired_palette_entry` covers paint geometry, localization, accepted-frame control publication, exact descriptors, controller replacement, and palette retirement.
- Native interpreter law `accepted_block_list_buttons_publish_localized_actions_and_reject_a_retired_palette_entry` covers the final accessibility projection and production Activate route.
- The changed Rust sources parse under Rust 2021. Focused native execution is queued with the root-owned renderer test run.
