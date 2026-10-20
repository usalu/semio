# Explicit Dashboard Language Choice

The prior authored fallback silently selected English when no supported system or explicit setting existed. The new schema records choices, autonyms and bilingual instructions without a default. Preference projection now exposes a null language until a supported system locale, journal event, environment override or explicit flag resolves it. Existing layer precedence remains.

Interactive startup renders the language choice before normal windows, daemon connection or command discovery. No row is initially selected, and Enter does not accept an absent selection. Numeric keys, arrows, Tab and primary pointer selection choose an autonym; confirmation publishes the existing local or explicitly shared preference event. The bounded journal publication runs off the UI thread with bilingual progress and settles truthfully after acceptance. Cancelling before confirmation returns without creating an event. Reopening replays the explicit event.

Unresolved noninteractive help renders both catalog languages. Internal daemon serving remains language-neutral and emits bilingual failures when unresolved; other noninteractive lifecycle wording requires a resolved or explicit language. Public effective preferences never fabricate a language.

Schema and neutral cases preceded implementation. Intl.Locale remains the independent system-locale oracle, and SQLite independently predicts chooser transitions. Native laws cover the same transitions, unresolved projection and genuine journal reopen. Source and native suite execution remain pending; no runtime/UI pass is claimed. Independent Source review is requested.

## Exact Changed Paths

- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/⚙️preferences/🌐️language/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🌐️language/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/🌐️language/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/⚙️preferences/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🌐️language/🟦️.ts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/⚙️preferences/🥒️.feature
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🌐️language/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🌐️language/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️language-choice/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⌨️controls/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📋️panes/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/⌨️command/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️labels/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🎹️keymap-and-launcher/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🔗️open-ready/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🖼️window-focus-and-sessions/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md

## Verification Reachability

The previously authored language Source suite had no dispatcher route. The new exact Source leaf `repo run @semio-tech/repo-dashboard-rs:test-language --output=stream` dispatches before installed-dashboard checks, declares its actual runtime module explicitly and keeps all inputs as freshness observations. It is appended to the separate retained Source-extra plan; the original26 roster stays intact. Low accepted the bounded chooser design/source; actual Source/native/PTY bodies remain pending.

Additional changed paths:
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts

Explicit launcher test language setup additionally changed:
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🚀️launcher/🧪️tests/🔬️unit/🦀️.rs

Low accepted the bounded chooser Source slice. No actual Source/native/PTY test body has run for this change.

## Installed PTY Acceptance Authored

Two actual installed-binary journeys now drive the existing portable-pty/vt100 oracle: unresolved startup has no selected row; Enter publishes nothing; selecting then cancelling creates no journal and exits. A second journey confirms Deutsch, parses exactly one durable preference event independently, detaches and reopens under the same unsupported system environment, and requires German controls without a repeated prompt or journal mutation. These journeys are authored but not executed.

- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🥒️.feature

## Cancellable Publication Boundary

The existing writer can wait up to two seconds for another journal writer. The new cancellable port checks before effects, during that unchanged bounded OS-lock wait and after projection before its first publication mutation. Once compaction/truncation or event writing begins it settles that commit; a late cancellation cannot falsely report an already committed event as cancelled. The chooser keeps its worker responsive, requests cancellation with Esc, and awaits the authoritative result. No preference is applied before a durable success. New shared cut cases have an independent SQLite output, and the actual native law checks pre-cancel no-file, lock-wait cancellation/no event, and durable post-commit reopening. These bodies remain pending.
