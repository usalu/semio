# Semantic New-File Existence Readback Correction

Read-only absolute-path capture at 2026-10-06T15:07:41.531744+00:00. Cwd: /Users/ueli/Documents/semio. All 19 semantic Rust files are absent, have null before images, and correctly match those before images. There is no SHA for an absent file.

| Owner | Exists | Before null | SHA | Matches Before |
| --- | --- | --- | --- | --- |
| 🔤️text | False | True | absent | True |
| 🔊️audio | False | True | absent | True |
| 🎬️video | False | True | absent | True |
| 📦️object | False | True | absent | True |
| 🔢️value | False | True | absent | True |
| 📊️table | False | True | absent | True |
| 🌊️flow | False | True | absent | True |
| 🖼️image | False | True | absent | True |
| 🎞️animation | False | True | absent | True |
| 🕸️graph | False | True | absent | True |
| 🔺️mesh | False | True | absent | True |
| 🧰️kit | False | True | absent | True |
| 🏛️model | False | True | absent | True |
| 📐️cad | False | True | absent | True |
| 📑️document | False | True | absent | True |
| 📽️presentation | False | True | absent | True |
| 🖊️drawing | False | True | absent | True |
| 🧊️brep | False | True | absent | True |
| ✉️base | False | True | absent | True |

The previous report's assertion that 19 paths concurrently changed was incorrect. Its diagnostic loop assigned empty string to an absent file, then compared that empty string against the actual null before image. An earlier loop excluded absent paths when counting inequality; these two inconsistent absence rules produced the misleading narrative. The later “60 of 79 matching” count likewise counted only existing matching files. This is an audit implementation error, not evidence of transient production changes.

The commands used for this turn read production paths with pathlib. They wrote only ticket Markdown reports. No held after image was written to production. This resumed context records earlier syntax checks as stdin rustfmt with emitted stdout; this correction does not claim a shell-history audit of preceding compacted turns. No runtime success is claimed.
