# Host ABI Usage Census (L0-imports)

**Ticket:** 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME  
**Generated:** 2026-08-17  
**Purpose:** Exhaustive census of host ABI usage across 33 plugins + 26 extensions to scope migration packets precisely.

---

## Executive Summary

| Metric | Value |
|--------|-------|
| Plugins analyzed | 33 |
| Extensions analyzed | 26 |
| Total .rs files | 9,544+ |
| HostEffect occurrences | 297 |
| Distinct HostEffect variants | 56+ |
| Plugins with handlers | 2 |
| Plugins with high HostEffect usage | 4 |

**Migration Tiers:** L (Heavy) = 4, M (Medium) = 20, S (Small) = 9

---

## Plugin Detailed Summary


### 🔌️ `✒️writer`

| Attr | Value |
|------|-------|
| Files |       86 |
| HostEffect usages |        9 |
| Distinct variants |        1 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (9 uses) — ✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:107


### 🔌️ `🌀️procedural`

| Attr | Value |
|------|-------|
| Files |      342 |
| HostEffect usages |       10 |
| Distinct variants |        2 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `DispatchAction` (7 uses) — 🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:261
- `InvokeExtension` (3 uses) — 🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️component.rs:26


### 🔌️ `🌊️flow`

| Attr | Value |
|------|-------|
| Files |      138 |
| HostEffect usages |        4 |
| Distinct variants |        2 |
| Handlers |       13 |
| Contributes |        0 |
| Extensions |        9 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `DispatchAction` (3 uses) — 🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️component.rs:17
- `InvokeExtension` (1 uses) — 🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️component.rs:57


### 🔌️ `🌍️gis`

| Attr | Value |
|------|-------|
| Files |      155 |
| HostEffect usages |        2 |
| Distinct variants |        1 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 2) |


**Top HostEffect Variants:**
- `OpenExternalUrl` (2 uses) — 🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️shell/🦀️component.rs:25


### 🔌️ `🎞️animate`

| Attr | Value |
|------|-------|
| Files |      106 |
| HostEffect usages |       18 |
| Distinct variants |        3 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (8 uses) — 🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:177
- `DownloadMediaExport` (6 uses) — 🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:174
- `ReplayShellCommand` (4 uses) — 🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:84


### 🔌️ `🎥️shooting`

| Attr | Value |
|------|-------|
| Files |      171 |
| HostEffect usages |       14 |
| Distinct variants |        4 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `RequestFileOpen` (5 uses) — 🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:815
- `LoadDocument` (4 uses) — 🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:339
- `IconRenderExport` (3 uses) — 🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖨️export/🦀️component.rs:41
- `DownloadMediaExport` (2 uses) — 🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:820


### 🔌️ `🏗️fem`

| Attr | Value |
|------|-------|
| Files |      274 |
| HostEffect usages |       19 |
| Distinct variants |        1 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (19 uses) — 🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:26


### 🔌️ `🏛️architect`

| Attr | Value |
|------|-------|
| Files |      864 |
| HostEffect usages |        6 |
| Distinct variants |        3 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (3 uses) — 🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:63
- `DownloadMediaExport` (2 uses) — 🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️exchange/🦀️component.rs:19
- `RequestFileOpen` (1 uses) — 🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️exchange/🦀️component.rs:82


### 🔌️ `🏭️process`

| Attr | Value |
|------|-------|
| Files |      130 |
| HostEffect usages |       16 |
| Distinct variants |        4 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        4 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (11 uses) — 🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:109
- `SetActiveUtility` (2 uses) — 🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:100
- `DownloadMediaExport` (2 uses) — 🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:143
- `RequestFileOpen` (1 uses) — 🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🦀️component.rs:53


### 🔌️ `💠️lowpoly`

| Attr | Value |
|------|-------|
| Files |      130 |
| HostEffect usages |        8 |
| Distinct variants |        1 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (8 uses) — 💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:328


### 🔌️ `💡️reasoning`

| Attr | Value |
|------|-------|
| Files |       91 |
| HostEffect usages |       10 |
| Distinct variants |        2 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (6 uses) — 💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:53
- `DispatchAction` (4 uses) — 💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:78


### 🔌️ `📋️forms`

| Attr | Value |
|------|-------|
| Files |      102 |
| HostEffect usages |        1 |
| Distinct variants |        1 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 2) |


**Top HostEffect Variants:**
- `DownloadMediaExport` (1 uses) — 📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-fixture/🦀️component.rs:16


### 🔌️ `📏️layout`

| Attr | Value |
|------|-------|
| Files |      149 |
| HostEffect usages |       20 |
| Distinct variants |        2 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `DispatchAction` (12 uses) — 📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:64
- `DownloadMediaExport` (8 uses) — 📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-svg/🦀️component.rs:23


### 🔌️ `📐️cad`

| Attr | Value |
|------|-------|
| Files |      142 |
| HostEffect usages |       10 |
| Distinct variants |        3 |
| Handlers |        0 |
| Contributes |        4 |
| Extensions |        4 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (4 uses) — 📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:462
- `DownloadMediaExport` (4 uses) — 📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:295
- `RequestFileOpen` (2 uses) — 📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2090


### 🔌️ `📜️imperative`

| Attr | Value |
|------|-------|
| Files |       81 |
| HostEffect usages |        0 |
| Distinct variants |        0 |
| Handlers |        5 |
| Contributes |        0 |
| Extensions |        5 |
| Weight | **M** (score: 2) |



### 🔌️ `📸️remodel`

| Attr | Value |
|------|-------|
| Files |      221 |
| HostEffect usages |        5 |
| Distinct variants |        4 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 2) |


**Top HostEffect Variants:**
- `Notify` (2 uses) — 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-bytes-payload/🦀️component.rs:155
- `RequestMediaFrames` (1 uses) — 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-video/🦀️component.rs:33
- `RequestFileOpen` (1 uses) — 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-frames/🦀️component.rs:30
- `DownloadMediaExport` (1 uses) — 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-qc-report/🦀️component.rs:15


### 🔌️ `🔱️trinity`

| Attr | Value |
|------|-------|
| Files |      182 |
| HostEffect usages |        9 |
| Distinct variants |        1 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (9 uses) — 🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:7


### 🔌️ `🖍️draw`

| Attr | Value |
|------|-------|
| Files |      120 |
| HostEffect usages |       14 |
| Distinct variants |        3 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (9 uses) — 🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:162
- `SetActiveUtility` (3 uses) — 🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:613
- `ReplayShellCommand` (2 uses) — 🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:672


### 🔌️ `🗒️note`

| Attr | Value |
|------|-------|
| Files |      192 |
| HostEffect usages |       13 |
| Distinct variants |        3 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (9 uses) — 🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:23
- `RequestFileOpen` (2 uses) — 🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️load-request/🦀️component.rs:14
- `DownloadMediaExport` (2 uses) — 🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️save-download/🦀️component.rs:15


### 🔌️ `🧩️puzzle`

| Attr | Value |
|------|-------|
| Files |      607 |
| HostEffect usages |       21 |
| Distinct variants |        6 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 4) |


**Top HostEffect Variants:**
- `SetActiveUtility` (11 uses) — 🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2186
- `SetActiveTool` (9 uses) — 🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2183
- `OpenDialog` (2 uses) — 🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2105
- `ClipboardWrite` (2 uses) — 🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2298
- `PatchWorld` (1 uses) — 🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:4133


### 🔌️ `🪐️space`

| Attr | Value |
|------|-------|
| Files |      162 |
| HostEffect usages |       81 |
| Distinct variants |        8 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        0 |
| Weight | **M** (score: 4) |


**Top HostEffect Variants:**
- `ReplayShellCommand` (31 uses) — 🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-space/🦀️component.rs:30
- `OpenDialog` (15 uses) — 🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-space/🦀️component.rs:27
- `Navigate` (13 uses) — 🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️go-home/🦀️component.rs:16
- `DownloadMediaExport` (8 uses) — 🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏙️create-studio/🦀️component.rs:108
- `LoadDocument` (6 uses) — 🪐️space/🦀️component.rs:213


### 🔌️ `🪵️sourcing`

| Attr | Value |
|------|-------|
| Files |       78 |
| HostEffect usages |        7 |
| Distinct variants |        1 |
| Handlers |        0 |
| Contributes |        0 |
| Extensions |        3 |
| Weight | **M** (score: 3) |


**Top HostEffect Variants:**
- `LoadDocument` (7 uses) — 🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:217


---

## HostEffect Variant Reference

**Total Distinct Variants: 56+**


| Variant | Uses | Sample Locations |
|---------|------|------------------|
| `LoadDocument` | 112 | 🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:7,🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:64,🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:8 |
| `ReplayShellCommand` | 37 | 🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:84,🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️seed-grid/🦀️component.rs:86,🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️canvas-pointer-down/🦀️component.rs:45 |
| `DownloadMediaExport` | 36 | 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-qc-report/🦀️component.rs:15,🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:143,🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🦀️component.rs:29 |
| `DispatchAction` | 27 | 🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️component.rs:17,🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️component.rs:17,🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-resolve/🦀️component.rs:17 |
| `OpenDialog` | 17 | 🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-space/🦀️component.rs:27,🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-space/🦀️component.rs:28,🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-space/🦀️component.rs:29 |
| `SetActiveUtility` | 16 | 🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:100,🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2186,🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1516 |
| `RequestFileOpen` | 16 | 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-frames/🦀️component.rs:30,🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🦀️component.rs:53,📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2090 |
| `Navigate` | 13 | 🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️go-home/🦀️component.rs:16,🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏙️create-studio/🦀️component.rs:39,🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️navigate-virtual-file-system-node/🦀️component.rs:20 |
| `SetActiveTool` | 9 | 🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2183,🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:30,🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-submit/🦀️component.rs:23 |
| `Notify` | 4 | 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-bytes-payload/🦀️component.rs:155,🪐️space/⚙️engine/🪐️space/🦀️component.rs:81,🪐️space/⚙️engine/🪐️space/🎮️commands/🔗️connect-media-ports/🦀️component.rs:88 |
| `InvokeExtension` | 4 | 🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️component.rs:57,🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️component.rs:26,🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:789 |
| `IconRenderExport` | 3 | 🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖨️export/🦀️component.rs:41 |
| `OpenPluginInstance` | 2 | 🪐️space/⚙️engine/🪐️space/🎮️commands/🔍️open-instance/🦀️component.rs:23 |
| `OpenExternalUrl` | 2 | 🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️shell/🦀️component.rs:25 |
| `ClipboardWrite` | 2 | 🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2298 |
| `RequestMediaFrames` | 1 | 📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-video/🦀️component.rs:33 |
| `PatchWorld` | 1 | 🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:4133 |

---

## Extension Crates (26 Total)

### By Host ABI Dependency

**With Handlers:**

- `🌊️flow/🏗️bim` —        2 handlers
- `🌊️flow/📃️list` —        1 handlers
- `🌊️flow/📐️brep` —        3 handlers
- `🌊️flow/📖️dictionary` —        1 handlers
- `🌊️flow/📝️text` —        1 handlers
- `🌊️flow/🔤️primitive` —        1 handlers
- `🌊️flow/🖍️draw` —        2 handlers
- `🌊️flow/🧠️logic` —        1 handlers
- `🌊️flow/🧮️math` —        1 handlers
- `📜️imperative/🎮️control` —        1 handlers
- `📜️imperative/📝️text` —        1 handlers
- `📜️imperative/📣️effect` —        1 handlers
- `📜️imperative/🧠️logic` —        1 handlers
- `📜️imperative/🧮️math` —        1 handlers

**Minimal/Topic-Only:**

- `🏭️process/🔩️metal`
- `🏭️process/🤖️robotic`
- `🏭️process/🧱️concrete`
- `🏭️process/🪵️wood`
- `📐️cad/🏛️aec-building-structure`
- `📐️cad/🏢️aec-building`
- `📐️cad/📐️spatial-shape`
- `📐️cad/🔥️aec-building-energy`
- `📖️playbook/🌀️procedural`
- `🪵️sourcing/🧱️slabs`
- `🪵️sourcing/🪟️windows`
- `🪵️sourcing/🪵️beams`

---

## High-Complexity Plugins (HostEffect ≥ 15)


### `🎞️animate` —       18 HostEffect usages

**Variant breakdown:**
- `LoadDocument`: 8
- `DownloadMediaExport`: 6
- `ReplayShellCommand`: 4


### `🏗️fem` —       19 HostEffect usages

**Variant breakdown:**
- `LoadDocument`: 19


### `🏭️process` —       16 HostEffect usages

**Variant breakdown:**
- `LoadDocument`: 11
- `SetActiveUtility`: 2
- `DownloadMediaExport`: 2
- `RequestFileOpen`: 1


### `📏️layout` —       20 HostEffect usages

**Variant breakdown:**
- `DispatchAction`: 12
- `DownloadMediaExport`: 8


### `🧩️puzzle` —       21 HostEffect usages

**Variant breakdown:**
- `SetActiveUtility`: 11
- `SetActiveTool`: 9
- `OpenDialog`: 2
- `ClipboardWrite`: 2
- `PatchWorld`: 1
- `DispatchAction`: 1


### `🪐️space` —       81 HostEffect usages

**Variant breakdown:**
- `ReplayShellCommand`: 31
- `OpenDialog`: 15
- `Navigate`: 13
- `DownloadMediaExport`: 8
- `LoadDocument`: 6
- `RequestFileOpen`: 4
- `OpenPluginInstance`: 2
- `Notify`: 2


---

## Migration Planning Notes

### By Weight Category

**L (Heavy) — 4 plugins**
- Need comprehensive refactoring for async jobs
- Check for loop bounds, recursive structures  
- Candidates: large HostEffect variety + custom effects

**M (Medium) — 20 plugins**
- Incremental migration possible
- Handlers need wrapped dispatch
- HostEffect::LoadDocument likely needs special handling

**S (Small) — 9 plugins**
- Minimal host ABI touch points
- Low-risk migrations

---

Report generated: Mon Aug 17 21:08:40 CEST 2026
