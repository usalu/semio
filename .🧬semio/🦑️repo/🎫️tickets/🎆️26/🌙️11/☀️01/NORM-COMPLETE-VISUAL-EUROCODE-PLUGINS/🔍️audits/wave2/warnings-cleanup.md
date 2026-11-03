# 🧹️ Warnings Cleanup Wave 2

Source: rustc logs `en1990.log` (q3035fceb016) and `en1995.log` (q29a019c8e36) in `⚡️cache/rust/fixed/m/u/l`, scoped to `✏️s/🔌️plugins/📕️norm/**` and `🧰️framework/🔨️modules/🌱️value/**`.
Applied by `🗑️generated/warnings-cleanup/apply.py --apply`: a line changes only when its current text equals the text quoted in the rustc diagnostic. Nothing was compiled (no cargo/rustc run).

Counts: 612 edits applied, 16 lines removed, 36 manual, 5 stale, 0 conflicts.

## Applied

| file:line | warning | fix |
|---|---|---|
| ✏️s/🔌️plugins/📕️norm/⚖️compliance/📐️quantities-clauses-and-text/🦀️.rs:148 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs:9 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs:5 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/⚖️compliance/🧱️core/🦀️.rs:4 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/⚖️compliance/🧱️core/🦀️.rs:3 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧬️schema/🎨️appearance/🦀️.rs:120 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧬️schema/🏷️annotations/🦀️.rs:49 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧬️schema/🏷️annotations/🦀️.rs:52 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧬️schema/🏷️annotations/🦀️.rs:53 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧬️schema/🏹️load-glyphs/🦀️.rs:89 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧬️schema/🏹️load-glyphs/🦀️.rs:90 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🕹️handle-kit/🦀️.rs:44 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:7 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:29 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:55 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:93 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:128 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:160 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:197 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:227 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:252 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:258 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:267 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:293 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/📏️dimensions/🦀️.rs:67 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/📏️dimensions/🦀️.rs:69 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/📏️dimensions/🦀️.rs:74 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/📏️dimensions/🦀️.rs:121 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/📏️dimensions/🦀️.rs:140 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔩️joints/🦀️.rs:7 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔩️joints/🦀️.rs:26 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔩️joints/🦀️.rs:61 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔩️joints/🦀️.rs:90 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔩️joints/🦀️.rs:111 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔩️joints/🦀️.rs:134 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🗺️layout/🦀️.rs:14 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:381 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:398 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:483 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:499 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:512 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:541 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:668 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:712 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🪵️profiles/🦀️.rs:109 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🪵️profiles/🦀️.rs:128 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🪵️profiles/🦀️.rs:140 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🪵️profiles/🦀️.rs:188 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🪵️profiles/🦀️.rs:201 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🪵️profiles/🦀️.rs:228 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️case-laws/🦀️.rs:85 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧱️foundation/🦀️.rs:5 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🦀️.rs:10 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🦀️.rs:9 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🦀️.rs:8 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/🧮️mechanics/🔩️fastening/🦀️.rs:118 | hidden lifetime parameters in types are deprecated | added `'_` lifetime |
| ✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/🦀️.rs:9 | unused import or extern crate | removed line |
| ✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/🦀️.rs:8 | unused import or extern crate | removed line |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/⚖️result/🦀️.rs:31 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/⚖️result/🦀️.rs:31 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📋️vector/🔑️identity/🦀️.rs:13 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📋️vector/🔑️identity/🦀️.rs:13 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🦀️.rs:26 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🦀️.rs:26 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🦀️.rs:26 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🦀️.rs:60 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🦀️.rs:60 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🦀️.rs:88 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🦀️.rs:88 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:102 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:118 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:155 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:156 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:160 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:162 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:162 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:162 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:162 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:179 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:185 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:57 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:60 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:62 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:123 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔐️mutex/🦀️.rs:12 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/📦️original/🦀️.rs:44 | use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs:21 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs:134 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:267 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:267 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:296 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:296 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:339 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:339 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:362 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:362 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:382 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:382 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:387 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:387 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:392 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:392 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:425 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:425 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:432 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:432 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:459 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:459 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:554 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:554 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:2 | unused import or extern crate | removed line |
| 🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs:137 | unused import: `format_ident` | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs:763 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs:78 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:81 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:81 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:83 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:86 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:86 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:106 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:111 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:115 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs:117 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:25 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:26 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:27 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:27 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:27 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:30 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🦀️.rs:42 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎮️splice/🦀️.rs:9 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎮️splice/🦀️.rs:11 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎮️splice/🦀️.rs:15 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎮️splice/🦀️.rs:45 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust/🦀️.rs:7 | unused import or extern crate | removed line |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:7 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:11 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:12 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:23 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:29 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:30 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:32 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:82 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:90 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:131 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:3 | unused import or extern crate | removed line |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🧬️clone/🦀️.rs:3 | unused import: `shared_issuance::SealedShared` | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/📦️packages/🦀️rust/🦀️.rs:7 | unused import: `NativeDecodeConstructionRecipient` | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/📦️packages/🦀️rust/🦀️.rs:9 | unused import: `original_vector_concat::OriginalVectorConcat` | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/🔑️named/🦀️.rs:22 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:733 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:734 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:741 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:743 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:750 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:753 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:122 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:122 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:124 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:124 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:132 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:136 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:138 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:138 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:138 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:138 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:138 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:138 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:157 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:157 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:159 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:159 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:159 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:163 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:178 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:178 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:178 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️requests/🦀️.rs:178 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📋️registry/🎫️slots/🦀️.rs:20 | use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs:63 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs:88 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs:76 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🗂️map/🦀️.rs:8 | bounds on generic parameters in type aliases are not enforced | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:34 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:34 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:35 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:35 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:35 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:35 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:35 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:35 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:24 | unused import or extern crate | removed line |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs:54 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/📽️projection/📦️packages/🦀️rust/🦀️.rs:2 | unused import or extern crate | removed line |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:101 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:101 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:173 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:173 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🧬️clone/🦀️.rs:31 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:70 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:70 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:69 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:69 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🎮️update/🎬️plan/🦀️.rs:7 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🎮️update/🎬️plan/🦀️.rs:7 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🎮️update/🎬️plan/🦀️.rs:7 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🦀️.rs:63 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:102 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:103 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:300 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:337 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:342 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:346 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:346 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:355 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:399 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:427 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:445 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:493 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:511 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛫️encode/🦀️.rs:27 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️backing/🦀️.rs:14 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️backing/🦀️.rs:14 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🔀️variant/🦀️.rs:13 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🧱️bulk/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🧱️bulk/🦀️.rs:110 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🧱️bulk/🦀️.rs:110 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:47 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:58 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/📦️box/🦀️.rs:14 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/📦️box/🦀️.rs:17 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/📦️box/🦀️.rs:17 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:56 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:56 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:62 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:79 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🦀️.rs:4 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/♻️retirement/🦀️.rs:12 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/♻️retirement/🦀️.rs:12 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:7 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:38 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:38 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:38 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:42 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:43 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:44 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:45 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:46 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:46 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:47 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:47 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:48 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:48 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:49 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:49 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:50 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:52 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:52 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:53 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:55 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:56 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:57 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:57 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:58 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:58 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:58 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:60 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:61 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:62 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:62 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:67 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:67 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:67 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:67 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:113 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:139 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:169 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:170 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:171 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:171 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:171 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:191 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️field/🦀️.rs:31 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️field/🦀️.rs:52 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:6 | unused import: `RetirementCursor` | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🦀️.rs:76 | use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:62 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:119 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:193 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:254 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:417 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:478 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️issuance-clone/🦀️.rs:20 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️issuance-clone/🦀️.rs:20 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:23 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:37 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:42 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:42 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:42 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:42 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:45 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:49 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:51 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:51 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:51 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:52 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:52 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:52 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:55 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:56 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:57 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:57 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:58 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:58 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:59 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:61 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:62 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:63 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:64 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:77 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:77 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:78 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:79 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:81 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:82 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:83 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:84 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:85 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:86 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:88 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:88 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:89 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs:90 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/♻️retirement/🦀️.rs:8 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/♻️retirement/🦀️.rs:15 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/♻️retirement/🦀️.rs:18 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/♻️retirement/🦀️.rs:19 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/♻️retirement/🦀️.rs:45 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:46 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:63 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:64 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:84 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:86 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:89 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:99 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:105 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:106 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:108 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:114 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:115 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:120 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:125 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:129 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:135 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:142 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:144 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:147 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:167 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:169 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:175 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:181 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:182 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:184 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:199 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:200 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:202 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:208 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:209 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:211 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:213 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:214 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:219 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:225 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:226 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:229 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:231 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:233 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:234 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:239 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:243 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:244 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:253 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:257 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:258 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:259 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:265 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:266 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:268 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:279 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:285 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:294 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:303 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:304 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:309 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:342 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:352 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:353 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:354 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:355 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:356 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:357 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs:360 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:25 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:26 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:32 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:37 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:38 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:44 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:45 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:55 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:67 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:68 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:69 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:70 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➖️remove/🦀️.rs:71 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/📦️packages/🦀️rust/🦀️.rs:4 | unused import or extern crate | removed line |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:46 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:53 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:56 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:57 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:66 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:68 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:69 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:75 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:76 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:76 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:77 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:78 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:78 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:79 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:84 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:88 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:90 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:93 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:96 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:104 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:105 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:110 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:111 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:113 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:120 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:136 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:138 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:140 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:146 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:147 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:149 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:158 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:163 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:169 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:181 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:186 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:192 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:206 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:207 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:208 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:215 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:219 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:234 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:235 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:235 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:238 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:243 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:257 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:258 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:259 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:260 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:31 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:32 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:75 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:77 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:78 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:79 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:80 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:81 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:82 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:84 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs:89 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:37 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:38 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:40 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:41 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:42 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:43 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:45 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:46 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:64 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:65 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:67 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:68 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:69 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:70 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:72 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:72 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:73 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:113 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:114 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:116 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:117 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:118 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:119 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:121 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:121 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:122 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔢️bounded-ord/🦀️.rs:123 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:71 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:73 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:73 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:73 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:106 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:45 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:101 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:103 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:103 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:103 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:104 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:104 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:104 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:140 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔗️alias/🦀️.rs:14 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔗️alias/🦀️.rs:14 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔗️alias/🦀️.rs:14 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔗️alias/🦀️.rs:15 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔗️alias/🦀️.rs:15 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔗️alias/🦀️.rs:15 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:46 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:48 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:48 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:48 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:49 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:49 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:49 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:70 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:74 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:76 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:76 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:76 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:77 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:77 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:77 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:97 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:84 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:86 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:86 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:86 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:87 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:87 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:87 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:124 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:93 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:95 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:95 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:95 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:96 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:96 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:96 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:126 | unnecessary qualification | applied rustc suggestion (qualification, import, bounds or deprecated name) |

## Not changed (manual)

Unused variables need a decision (prefix `_` only where the value is truly unused). Dead code needs removal across call sites. Multi-name unused imports and `as` renames need a manual pass.

| file:line | warning | status |
|---|---|---|
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🦀️.rs:26 | unused variable: `body` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:5 | unused imports: `marker::PhantomData` and `sync::Arc` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:3 | unused import: `RetainedCloneProgress` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🌳️intrinsic/🦀️.rs:2 | unused import: `ValueRefusalKind` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🌐️wire/🦀️.rs:2 | unused import: `ValueRefusalKind` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎮️splice/🦀️.rs:3 | unused import: `admit_retained_clone_progress` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust/🦀️.rs:5 | unused imports: `RetainedEquality` and `ToValue` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust/🦀️.rs:6 | unused imports: `DslValue`, `FromValue`, `ToValue`, and `ValueShape` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🧳️read/🦀️.rs:3 | unused import: `RetainedCloneStep` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/📦️packages/🦀️rust/🦀️.rs:5 | unused imports: `DslValue` and `Number` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:926 | unused import: `ValueRefusalKind` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs:63 | unused variable: `body` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs:4 | unused import: `add_receipt` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs:5 | unused import: `paged_value_index as paged_index` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:147 | struct `RegistryAuthority` is never constructed | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🧬️clone/🦀️.rs:31 | unused variable: `body` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🔗️read/🧬️clone/🦀️.rs:28 | field `original` is never read | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:35 | function `node` is never used | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:51 | function `balanced` is never used | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛫️encode/🦀️.rs:27 | unused variable: `body` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🔀️variant/🦀️.rs:2 | unused import: `NativeSchemaLeaf` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🧱️bulk/🦀️.rs:30 | method `empty` is never used | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📋️list/🦀️.rs:2 | unused import: `receiving_child_body_budget` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🗂️map/🦀️.rs:2 | unused imports: `NativeDefault` and `add_receipt` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🗂️map/🦀️.rs:5 | unused import: `size_of` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🗂️map/🦀️.rs:3 | unused import: `retirement::RetireOwned` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs:20 | function `receiving_child_body_budget` is never used | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/📦️storage/🦀️.rs:4 | unused import: `MaybeUninit` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧺️set/🦀️.rs:2 | unused imports: `NativeDefault` and `add_receipt` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧺️set/🦀️.rs:5 | unused import: `size_of` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:3 | unused import: `size_of` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:6 | unused imports: `RetireOwned` and `RetirementStep` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🦀️.rs:29 | associated function `source_demands` is never used | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️projection/🦀️.rs:4 | unused import: `RetainedCloneStep` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:28 | unused variable: `body` | not changed (needs item-level decision) |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🦀️.rs:4 | unused import: `ErasedSnapshotRetirement` | not changed (needs item-level decision) |

## Stale (skipped)

| file:line | warning | status |
|---|---|---|
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️case-laws/🦀️.rs:6 | unnecessary qualification | STALE: source no longer matches build log, skipped |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️case-laws/🦀️.rs:9 | unnecessary qualification | STALE: source no longer matches build log, skipped |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️case-laws/🦀️.rs:9 | hidden lifetime parameters in types are deprecated | LT-MISMATCH: source no longer matches build log, skipped |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️case-laws/🦀️.rs:85 | unnecessary qualification | STALE: source no longer matches build log, skipped |
| ✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️case-laws/🦀️.rs:85 | unnecessary qualification | STALE: source no longer matches build log, skipped |
