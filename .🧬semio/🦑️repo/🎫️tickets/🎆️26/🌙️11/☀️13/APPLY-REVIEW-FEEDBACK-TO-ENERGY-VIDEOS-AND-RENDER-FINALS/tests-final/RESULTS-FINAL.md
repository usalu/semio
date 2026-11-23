# Test results on the final sources (2026-11-14)

All 66 modules of `modules.txt` exit 0. The main pass (`run-all.sh`) ran before the round 4 fixes; the second table re-ran the modules touched by those fixes and the five modules that need a different launch (the `repo` build with `--light` on PATH via `run-one-path.sh`, or profile `preview` with language `en` for the visuals integration module).

## Main pass

| Exit | Time | Module |
| --- | --- | --- |
| 0 | 3s | `🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🧪️tests.🐍️` |
| 0 | 3s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️` |
| 0 | 7s | `🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.🧪️tests.🐍️` |
| 0 | 0s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️numbers` |
| 0 | 4s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️transitions` |
| 0 | 9s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️lod` |
| 0 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🌍️captions.🐍️` |
| 0 | 21s | `🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.📐️system-sizing.🧪️tests.🐍️` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️` |
| 0 | 11s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️course_staffage` |
| 0 | 26s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️` |
| 0 | 7s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️icons` |
| 0 | 3s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️labels` |
| 0 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️media_staffage` |
| 0 | 10s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️scenes` |
| 0 | 9s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️section_consumers` |
| 0 | 5s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️sections` |
| 0 | 117s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only` |
| 0 | 4s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️transforms` |
| 0 | 6s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.📟️readout.🐍️` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🧪️tests.🐍️` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎞️playback.🧪️tests.🐍️` |
| 0 | 193s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🎨️semio-style.🐍️` |
| 1 | 6s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🧪️tests.🐍️` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📤️export.🎞️course.🧪️tests.🐍️` |
| 0 | 7s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📤️export.🧪️tests.🎬️e2e.🐍️` |
| 0 | 28s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.👥️audience.🧪️tests.🐍️` |
| 0 | 5s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📤️export.🧪️tests.🐍️` |
| 1 | 210s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration` |
| 0 | 28s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🧪️tests.🐍️` |
| 0 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.💾️persisted.🐍️` |
| 0 | 0s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.📚️closure.🐍️` |
| 1 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🕰️motion.🐍️` |
| 1 | 0s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🖱️hover.🐍️` |
| 0 | 0s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🧹️staging.🐍️` |
| 1 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🪡️axis.🐍️` |
| 0 | 10s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📐️geometry.🧪️tests.🐍️` |
| 0 | 42s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🎥️camera.🐍️` |
| 0 | 209s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖼️posters.🧪️tests.🐍️` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🐍️` |
| 0 | 318s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🐍️` |
| 0 | 15s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🧩️chapters.🐍️pointer` |
| 0 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.⏱️purity.🐍️` |
| 0 | 434s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🧪️tests.🐍️` |
| 0 | 3s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🖼️stream.🐍️` |
| 0 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🪵️diagnostics.🐍️` |
| 0 | 0s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🧪️tests.🐍️` |
| 0 | 9s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🧪️tests.🐍️` |
| 0 | 11s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧪️tests.🛡️guard.🐍️` |
| 0 | 3s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧪️tests.🧮️tex.🐍️` |
| 0 | 8s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧪️tests.🪟️caption-layout.🐍️` |
| 0 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🧪️tests.🗣️lexicon.🐍️` |
| 0 | 17s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🧪️tests.🐍️` |
| 0 | 19s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🧪️tests.📑️heading.🐍️` |
| 0 | 4s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🗺️roadmap.🧪️tests.🐍️` |
| 0 | 8s | `🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🧪️tests.☀️sun.🐍️` |
| 0 | 4s | `🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🏭️primary-energy.🧪️tests.🐍️chain` |
| 0 | 4s | `🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🏭️primary-energy.🧪️tests.🐍️scale` |
| 0 | 4s | `🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🔁️recap.🧪️tests.🧮️balance.🐍️` |
| 0 | 4s | `🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🧪️tests.🔗️chain.🐍️` |
| 0 | 5s | `🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🧪️tests.🛡️guard.🐍️` |
| 0 | 8s | `🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧱️conduction.🧪️tests.🐍️` |
| 0 | 10s | `🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️` |
| 0 | 403s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🧩️chapters.🐍️` |
| 0 | 386s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️` |

Non-zero exits in this table are launch problems, cleared in the re-run below.

## Re-run after round 4

| Exit | Time | Module |
| --- | --- | --- |
| 0 | 13s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🧪️tests.🐍️` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🕰️motion.🐍️` |
| 0 | 15s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🖱️hover.🐍️` |
| 0 | 1s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🪡️axis.🐍️` |
| 0 | 3s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️` |
| 0 | 5s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️` |
| 0 | 0s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️numbers` |
| 0 | 3s | `🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️transitions` |
| 0 | 2s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🗺️roadmap.🧪️tests.🐍️` |
| 0 | 8s | `🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️` |
| 0 | 205s | `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration` |
