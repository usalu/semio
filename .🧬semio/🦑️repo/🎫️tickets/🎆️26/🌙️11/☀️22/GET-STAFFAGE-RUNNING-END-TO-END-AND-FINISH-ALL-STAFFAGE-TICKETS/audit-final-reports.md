# Audit of the Final Multiview Reports (2026-11-25)

Read-only audit by the coordinator's auditor. No file was edited except this report; no render, matrix, test or build was started.

Reports checked: `multiview-ML-1.md`, `multiview-MV-1.md`, `multiview-MC.md`, `multiview-MX.md` (all headed FINAL), and the older `multiview-MF.md`, `multiview-MO.md`, `multiview-MB-1.md`, `multiview-MB-2.md`, `multiview-MS.md`, `multiview-MR.md`.

## Method

- (a) Proof record: searched `🗑️generated/matrix/<BATCH>/` and `🧰️matrix/<BATCH>/` for each asset, and every `🗑️generated/matrix` log for `252 cells` or `matrix exit`. A record counts only if it ends with a summary line and shows no `fail` cell. Scan and gate records were looked for the same way.
- (b) Catalog file `S/🗂️catalog/<folder>/<id>/🔣️.json`: mtime (local time) against the record time. No report states a hash; the sha256 prefix in the tables is a reference for the next proof, not a comparison.
- Classes: `proven-current` (record ends, 0 fail, file not changed after it); `proven-but-file-changed-since`; `claimed-without-log` (clean claim, no record file); `open` (report states an open item, or the record shows fail, or the claim is superseded by an open state).

## Summary of the logs

- `🗑️generated/matrix/{ML-1, MF, MO, MB-1, MB-2, MS, MR, MX}` do not exist. The batch folders `🧰️matrix/{ML-1, MF, MO, MB-1, MB-2, MS, MR, MX}` hold scripts only. ML-1 and MC say in their own text that their logs were removed.
- `🗑️generated/matrix/MC` holds only `written.txt` (16:40). `🗑️generated/matrix/MV-1` holds only `sheets/`. No file in either folder records a matrix run.
- `🧰️matrix/MC/matrix.txt` is the one record for the MC batch: 22 lines (20 batch assets, `storage-crate-stack`, `glulam-beam`), each `exit 0`, status without `fail`, last written 11:26:48. It has no scan or gate record for the takeover.
- `🗑️generated/matrix/MX-2/proof/` (report IN WORK) holds `glulam-beam` and `steel-column` logs from 16:40: scan clean, matrix `252 ok, 0 fail`, `--fresh`. These two are the only proofs of those assets in the ticket after the MC and MX reports.
- Catalog mtimes in the 16:40 cluster (for example `used-tumble-dryer`, `used-harmonium`, `used-towel-radiator`, `used-wall-hung-gas-boiler`, `steel-column`, several MF and MO assets) were written after the reports' own proof windows. No report records why.

## Per batch

### ML-1 (FINAL): 6 clean, 1 open

Claim: 5 clean with all proofs plus `used-washing-machine`; open `used-harmonium` (21 fail, `paper-height`). Proof: takeover run 08:54 to 10:35. Record: none.

Open items stated in the report: `used-harmonium` 21 `paper-height` fail cells, asset not changed; `electric-fan-heater` `model-bounds` 3 of 4 heater cases fail (note in `requests.md`, not repaired).

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `cafe-chair` | claimed-without-log | NONE | 11-25 04:29:08 | `caf705584aa2` | MF had 14 fail cells; ML-1 repair; no log |
| `office-swivel-chair` | claimed-without-log | NONE | 11-25 04:29:14 | `7d8a36c27901` | MF had 70 fail cells; ML-1 repair; no log |
| `used-fridge-freezer` | claimed-without-log | NONE | 11-25 04:29:18 | `291208f9750a` | MF had 42 fail cells; ML-1 repair; no log |
| `used-tumble-dryer` | claimed-without-log | NONE | 11-25 16:40:44 | `ca9b35ba95f6` | file written after the ML-1 window; no log |
| `used-washing-machine` | claimed-without-log | NONE | 11-25 04:29:28 | `472dceec7acd` | MF check b repaired in ML-1; no log |
| `used-concert-harp` | claimed-without-log | NONE | 11-25 06:06:13 | `21a07cffba8b` | MO open (84 fail) superseded by ML-1; no log |
| `used-harmonium` | open | NONE | 11-25 16:40:46 | `d6c458319c4b` | 21 fail in ML-1 and MO; MX claims 0 fail with no log; file written after the ML-1 window |

### MV-1 (FINAL): 14 plants clean

Claim: 14 of 14 plants clean in scan and matrix (`0 fail`), proof lines pasted in the report. Record: none in the ticket. The proof lines in the report are text, not a file.

Open items stated: wind-pose drawing quality of the `carry.ts` plates (no fail cell, judged on sheets; heaviest on `black-alder`); `black-locust` `ld2` group drift (reported tier, 2350 mm default pose); reported tier (group drift) on eight plants; `rowan` and `saucer-magnolia` handed to MV-3.

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `apple-tree` | claimed-without-log | NONE | 11-24 20:18:57 | `315c575c2df5` | no log in ticket |
| `bamboo-clump` | claimed-without-log | NONE | 11-24 19:49:12 | `5470742b36f9` | no log in ticket |
| `black-alder` | claimed-without-log | NONE | 11-24 19:30:02 | `12ad6a769bd9` | no log in ticket |
| `black-locust` | claimed-without-log | NONE | 11-25 10:19:20 | `f8c93712678f` | no log in ticket |
| `common-ash` | claimed-without-log | NONE | 11-24 20:20:54 | `7d7a65f5fcf3` | no log in ticket |
| `common-lilac` | claimed-without-log | NONE | 11-24 18:45:30 | `ee79ceca2e9d` | no log in ticket |
| `common-reed` | claimed-without-log | NONE | 11-24 19:47:33 | `30aa684b566b` | no log in ticket |
| `crab-apple` | claimed-without-log | NONE | 11-24 20:19:10 | `35e409410af3` | no log in ticket |
| `dawn-redwood` | claimed-without-log | NONE | 11-24 20:20:00 | `fd18ffdc6ad1` | no log in ticket |
| `english-oak` | claimed-without-log | NONE | 11-25 09:53:54 | `cc5d67aca678` | no log in ticket |
| `european-beech` | claimed-without-log | NONE | 11-24 20:20:30 | `14cae481e1db` | no log in ticket |
| `field-maple` | claimed-without-log | NONE | 11-24 19:25:49 | `d13f2e89c9f1` | no log in ticket |
| `fig-tree` | claimed-without-log | NONE | 11-24 20:19:28 | `d448c44e1e27` | no log in ticket |
| `hawthorn` | claimed-without-log | NONE | 11-24 19:20:59 | `99395b1f7299` | no log in ticket |

### MC (FINAL): 20 batch assets clean, 2 leftovers

Claim: 20 of 20 batch assets clean (scan, gate, matrix 0 fail); leftovers `storage-crate-stack` (clean), `glulam-beam` (clean, matrix 252 ok), `steel-column` (open, 21 cells).
Record: `🧰️matrix/MC/matrix.txt` (11:26:48). The scan and gate of the takeover have no file record; the predecessor's `after/scan.json` predates the takeover and was not used.

Open items stated: `steel-column` 21 `derived-view-in-band` cells until an allowlist line is merged; `used-towel-radiator` small round parts flat (no finding); `reclaimed-precast-stair` bare soffit face (no finding); reported tier rose for `forklift`, `city-bus`, `motor-scooter`; `forklift` does not print at 1:20 (before the batch).

Status changes since MC: `glulam-beam` was changed by MX to a merged bar (matrix 7 fail, MX at 11:50) and later proven by MX-2 (252 ok, 16:40). `steel-column` is 252 ok in MX-2 (16:40).

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `reclaimed-cast-iron-column` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:26 | `050fc636d70a` | matrix record only |
| `reclaimed-precast-stair` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:27 | `fa038c731727` | matrix record only |
| `reclaimed-trapezoidal-sheet-stack` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:27 | `b355338fb48c` | matrix record only |
| `sectional-garage-door` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:27 | `cb3a0c758092` | matrix record only |
| `sliding-door` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:27 | `5fa4df1f5aed` | matrix record only |
| `used-air-water-heat-pump-monobloc` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:27 | `98b786d13b85` | matrix record only |
| `used-forced-draught-oil-burner` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:27 | `af311085d159` | matrix record only |
| `used-pellet-boiler` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:27 | `b955ec03b22d` | matrix record only |
| `diesel-generator` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:28 | `75048eedacc5` | matrix record only |
| `evaporative-humidifier` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:28 | `0f021bfcd971` | matrix record only |
| `forklift` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:28 | `e598d6f108bc` | matrix record only |
| `scroll-compressor` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:28 | `5c1954fefdab` | matrix record only |
| `cargo-bike` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:28 | `93d3591a50a8` | matrix record only |
| `city-bicycle` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:28 | `7955fa15f4ee` | matrix record only |
| `city-bus` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:29 | `9429feb90819` | matrix record only |
| `city-car` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:29 | `adbe0e1660f2` | matrix record only |
| `motor-scooter` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:29 | `1142f2f128d6` | matrix record only |
| `vintage-car-1929` | proven-current | MC matrix.txt 11:26 | 11-25 02:20:29 | `63ca6d298500` | matrix record only |
| `used-towel-radiator` | proven-but-file-changed-since | MC matrix.txt 11:26 | 11-25 16:40:46 | `1c8f43e5cfdf` | file written after the record; hash not comparable |
| `used-wall-hung-gas-boiler` | proven-but-file-changed-since | MC matrix.txt 11:26 | 11-25 16:40:46 | `c3a149565d10` | file written after the record; hash not comparable |
| `storage-crate-stack` | proven-current | MC matrix.txt 11:26 | 11-25 09:28:58 | `2ca778486b87` | leftover; matrix 252 ok; MB-2 had 28 band copy and 28 language-geometry cells |
| `glulam-beam` | proven-current | MX-2 glulam-beam-matrix.log 16:40 | 11-25 11:34:03 | `57bfedfc6eda` | MC record (11:26) predates the 11:34 change; MX had 7 fail; MX-2 252 ok fresh |
| `steel-column` | proven-current | MX-2 steel-column-matrix.log 16:40 | 11-25 16:40:01 | `cc7c76cd5ca6` | MC, MB-1 and MX open claims superseded by the MX-2 record |

### MX (FINAL): shared code, 1 clean asset

Claim: `used-harmonium` clean (fresh matrix 231 ok, 21 unavailable, 0 fail, scan clean). Record: none in the ticket. Open per MX: `steel-column` 21 cells and `glulam-beam` 7 cells until the coordinator merges `🧰️matrix/MX/allowlist.json`. Both are now proven by MX-2 records (see MC table).

Other open items stated by MX: pocket rule of `🧊️catalog-silhouette` (not changed); `lateralScale` of revolved drawing solids; a declared section compared with the whole body; `pregnant-person`, `tour-guide`, `kneeling-gardener` to re-measure.

### MF (older, no FINAL): 20 assets, 16 matrix-clean claimed

Claim: scan 20 of 20 clean; matrix 16 of 20 without a `fail` cell; 4 failing (`cafe-chair`, `office-swivel-chair`, `used-fridge-freezer`, `used-tumble-dryer`) repaired in ML-1. Record: none.

Open items stated: `electric-fan-heater` `model-bounds` heater 3 of 4 cases fail; `used-hinged-wardrobe` 1:20 print gap (fails as before the batch); `used-washing-machine` `carried` front rear feet gap (later repaired in ML-1); open strokes on tilting bones stay flat; sheets of 15 assets not opened.

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `electric-fan-heater` | open | NONE | 11-24 18:10:16 | `9e1ff9f606a6` | model-bounds heater 3 of 4 cases fail |
| `used-hinged-wardrobe` | open | NONE | 11-24 18:09:54 | `40e6e284b0b7` | 1:20 print gap open |
| `gas-hob` | claimed-without-log | NONE | 11-24 18:10:17 | `235a725adcde` | no log |
| `used-bedside-table` | claimed-without-log | NONE | 11-25 16:40:45 | `abca80fb93c5` | no log |
| `used-dishwasher` | claimed-without-log | NONE | 11-25 16:40:44 | `ed6c77afc005` | no log |
| `used-electric-cooker` | claimed-without-log | NONE | 11-25 16:40:44 | `766f148e4f70` | no log |
| `used-electric-guitar-and-combo` | claimed-without-log | NONE | 11-24 18:10:33 | `62cea70dd302` | no log |
| `used-electronic-drum-kit` | claimed-without-log | NONE | 11-24 18:10:43 | `5694d9c2c847` | no log |
| `used-electronic-organ` | claimed-without-log | NONE | 11-25 16:40:44 | `7638b434e009` | no log |
| `used-high-chair` | claimed-without-log | NONE | 11-25 16:40:45 | `6db7c57d2e49` | no log |
| `used-kitchen-base-cabinet` | claimed-without-log | NONE | 11-24 18:09:58 | `9aae2fdbbb03` | no log |
| `used-microwave-oven` | claimed-without-log | NONE | 11-25 16:40:44 | `c8899ba09dc3` | no log |
| `used-park-armchair` | claimed-without-log | NONE | 11-24 18:10:02 | `9733f8ff6e26` | no log |
| `used-recliner-chair` | claimed-without-log | NONE | 11-24 18:10:05 | `2b9f68269b2b` | no log |
| `used-wingback-armchair` | claimed-without-log | NONE | 11-24 18:10:15 | `30d36bb1dca2` | no log |

### MO (older): 17 objects, 15 clean claimed

Claim: 15 clean in scan and matrix; open: `used-concert-harp` (84 fail, later repaired in ML-1) and `used-harmonium` (21 fail). Record: none.

Open items stated: `used-harmonium` `paper-height` (asset not changed); `used-concert-harp` contact `shoulder` (later repaired); reported-tier drift for `charcoal-grill`, `used-football-goal`, `used-cello`; flat strokes on coarse bands.

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `charcoal-grill` | claimed-without-log | NONE | 11-24 18:38:14 | `ac259460696c` | no log |
| `stepladder` | claimed-without-log | NONE | 11-24 18:38:14 | `5f7a174a2191` | no log |
| `used-basketball-hoop` | claimed-without-log | NONE | 11-24 18:38:14 | `74a0199d4479` | no log |
| `used-bicycle-shelter` | claimed-without-log | NONE | 11-24 18:38:15 | `f1895e43843b` | no log |
| `used-cello` | claimed-without-log | NONE | 11-24 18:38:15 | `61e73754ff15` | no log |
| `used-classical-guitar` | claimed-without-log | NONE | 11-24 18:38:15 | `4d3069b39369` | no log |
| `used-djembe-and-cajon` | claimed-without-log | NONE | 11-25 16:40:44 | `5f3f0a8ec778` | no log |
| `used-double-bass` | claimed-without-log | NONE | 11-24 18:38:16 | `85d39f0264ec` | no log |
| `used-double-swing` | claimed-without-log | NONE | 11-24 18:38:16 | `5c2843e750b3` | no log |
| `used-drum-kit` | claimed-without-log | NONE | 11-24 18:38:17 | `18273bab27bb` | no log |
| `used-football-goal` | claimed-without-log | NONE | 11-24 18:38:17 | `4a4d17e519b9` | no log |
| `used-spring-rocker` | claimed-without-log | NONE | 11-25 16:40:47 | `7ef32429608b` | no log |
| `used-viola` | claimed-without-log | NONE | 11-25 16:40:48 | `054d8a05b73f` | no log |
| `used-water-play-table` | claimed-without-log | NONE | 11-24 18:38:18 | `a66c6f7b26d9` | no log |
| `used-wc-pan-cistern` | claimed-without-log | NONE | 11-25 16:40:48 | `c32d16064aba` | no log |

### MB-1 (older): 19 band-copy assets, 12 clean claimed

Claim: 12 clean (scan and matrix 0 fail); 7 keep 168 fail cells. Record: none.

Open items stated (5 still open; `steel-column` and `glulam-beam` are now proven by MX-2): `etics-brick-wall-12` (35), `etics-brick-wall-30` (35), `solid-brick-wall-1960` (42), `spiral-stair` (14), `stair-flight` (14). Requests are in `requests.md`.

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `bookshelf` | claimed-without-log | NONE | 11-24 12:41:35 | `d762400bfbf4` | no log |
| `curtain-wall-bay` | claimed-without-log | NONE | 11-24 12:41:44 | `3ff9646bbe78` | no log |
| `dining-table` | claimed-without-log | NONE | 11-24 12:41:36 | `a84001c7301b` | no log |
| `duct-fan` | claimed-without-log | NONE | 11-24 12:41:36 | `f580036585d9` | no log |
| `external-venetian-blind` | claimed-without-log | NONE | 11-24 12:41:45 | `184bb50d50ee` | no log |
| `hot-water-cylinder` | claimed-without-log | NONE | 11-24 12:41:55 | `11f2c083b56a` | no log |
| `kitchen-extractor-hood` | claimed-without-log | NONE | 11-24 12:58:24 | `70ee9d25706d` | no log |
| `park-bench` | claimed-without-log | NONE | 11-24 12:47:22 | `a59692831525` | no log |
| `reclaimed-steel-column` | claimed-without-log | NONE | 11-24 12:41:45 | `2056564d8103` | no log |
| `single-bed` | claimed-without-log | NONE | 11-24 12:44:07 | `053ba4785dbc` | no log |
| `used-crt-television` | claimed-without-log | NONE | 11-24 12:41:36 | `2ebe2a65682e` | no log |
| `used-trench-convector` | claimed-without-log | NONE | 11-24 12:41:45 | `bd165275e746` | no log |
| `etics-brick-wall-12` | open | NONE | 11-24 12:41:44 | `e28481941770` | 35 fail cells, open |
| `etics-brick-wall-30` | open | NONE | 11-24 12:41:44 | `0750b0e60dec` | 35 fail cells, open |
| `solid-brick-wall-1960` | open | NONE | 11-24 12:41:44 | `e5823f933d8e` | 42 fail cells, open |
| `spiral-stair` | open | NONE | 11-24 12:43:11 | `4c6f3f289e2c` | 14 fail cells, open |
| `stair-flight` | open | NONE | 11-24 12:41:45 | `cded97dbad7b` | 14 fail cells, open |

### MB-2 (older): 18 band-copy assets

Claim: 5 clean; 11 clean except band copies entered in `allowlist.json` (pending the coordinator's merge); 2 in-band views pending. `storage-crate-stack` is among the 11 and was closed by MC. Record: none.

Open items stated: allowlist merge pending (10 assets with their own fail cells); `tealight` and `used-bollard-light` in-band views pending; `saguaro-cactus` matrix not finished (25-minute limit).

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `used-extendable-dining-table` | claimed-without-log | NONE | 11-24 12:53:59 | `f9143f62269d` | no log |
| `sorption-wheel` | claimed-without-log | NONE | 11-24 12:57:34 | `c6f5c88b2aa5` | no log |
| `patio-heater` | claimed-without-log | NONE | 11-24 12:59:35 | `ecddaa07debc` | no log |
| `used-sandpit-digger` | claimed-without-log | NONE | 11-24 13:04:19 | `07b2a1ed4d3a` | no log |
| `used-climbing-net-pyramid` | claimed-without-log | NONE | 11-24 13:09:16 | `ab87b949fb07` | no log |
| `bicycle-stand` | open | NONE | 11-24 17:17:10 | `92f8db492971` | allowlist pending; fail cells remain |
| `heat-pump-outdoor-unit` | open | NONE | 11-24 13:03:00 | `ef88bcd69fd2` | allowlist pending; fail cells remain |
| `plate-heat-exchanger` | open | NONE | 11-24 13:03:20 | `04e1de760db8` | allowlist pending; fail cells remain |
| `solar-thermal-collector` | open | NONE | 11-24 13:03:37 | `0cc9c722c7e2` | allowlist pending; fail cells remain |
| `saguaro-cactus` | open | NONE | 11-22 22:32:11 | `466841083586` | allowlist pending; fail cells remain |
| `used-coffee-table` | open | NONE | 11-24 13:09:31 | `5a4dfc7c30b0` | allowlist pending; fail cells remain |
| `used-folding-bookcase-ladder` | open | NONE | 11-22 16:20:08 | `9caf63d6d99a` | allowlist pending; fail cells remain |
| `used-shoe-cabinet` | open | NONE | 11-24 17:17:20 | `faa7d8e6d40a` | allowlist pending; fail cells remain |
| `used-tv-sideboard` | open | NONE | 11-24 17:17:20 | `0361cc966ab9` | allowlist pending; fail cells remain |
| `used-wall-unit` | open | NONE | 11-24 13:09:16 | `3709c991aea5` | allowlist pending; fail cells remain |
| `tealight` | open | NONE | 11-24 17:17:10 | `4fe1e5b907d6` | in-band allowlist pending |
| `used-bollard-light` | open | NONE | 11-24 12:53:59 | `f349ac62d8a5` | in-band allowlist pending |

### MS (older): 12 assets, 2 clean claimed

Claim: `decentralised-hrv-fan` and `european-larch` clean (scan); `pendant-lamp` clean except the allowlisted front view. Record: none. The batch is scan-only (no matrix).

Open items stated: `ceiling-diffuser`, `used-park-lantern`, `used-playground-roundabout`, `waste-bin`, `boxwood-ball` (allowlist `front` until merged); `norway-spruce`, `windmill-palm` (allowlist proposed, not decided); `saucer-magnolia` (1:500 front open); `rowan` (rest open; moved to MV-3).

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `decentralised-hrv-fan` | claimed-without-log | NONE | 11-24 12:47:47 | `3c98ef1aed1e` | scan clean claim; no log |
| `european-larch` | claimed-without-log | NONE | 11-24 12:59:42 | `abca3166cbb4` | scan clean claim; no log |
| `pendant-lamp` | open | NONE | 11-24 12:43:56 | `7a1dcd93b59d` | 3 allowlisted front findings |
| `ceiling-diffuser` | open | NONE | 11-21 12:45:23 | `5641270bbbca` | open per MS |
| `used-park-lantern` | open | NONE | 11-22 21:15:26 | `03f193d37479` | open per MS |
| `used-playground-roundabout` | open | NONE | 11-23 14:44:31 | `8e6c08aa5d30` | open per MS |
| `waste-bin` | open | NONE | 11-22 23:34:23 | `ed532b3372b0` | open per MS |
| `boxwood-ball` | open | NONE | 11-22 21:14:35 | `b0a903b60f4b` | open per MS |
| `norway-spruce` | open | NONE | 11-23 00:36:49 | `c09053d4c555` | open per MS |
| `windmill-palm` | open | NONE | 11-22 23:58:43 | `eb6ae26e964b` | open per MS |
| `saucer-magnolia` | open | NONE | 11-25 03:02:06 | `1bd876adc084` | open per MS |
| `rowan` | open | NONE | 11-25 02:56:33 | `8e8022955a2f` | open per MS |

### MR (older): 2 reference people, clean claimed

Claim: `standing-adult` and `walking-commuter` with 0 fail (matrix 252 cells each, 706 s and 1,278 s). Record: none. The matrix logs of the MH batches were searched; none covers these two (only vitest logs mention them).

Open items stated: reported-tier group drift (not failures); `fit.ts` has no op for a flat elevation shape to solid.

| Asset | Class | Record | Catalog mtime | sha256 (12) | Note |
|---|---|---|---|---|---|
| `standing-adult` | claimed-without-log | NONE | 11-24 12:49:29 | `7234c81d590a` | no log |
| `walking-commuter` | claimed-without-log | NONE | 11-24 12:54:43 | `3408b186b167` | no log |

## Assets that need a fresh proof

- Whole batch ML-1 (6), MV-1 (14), MO (15), MB-1 clean (12), MB-2 clean (5), MS clean (2), MR (2), MF clean (13 others), `used-concert-harp`, `cafe-chair`, `office-swivel-chair`, `used-fridge-freezer`, `used-washing-machine`, `used-tumble-dryer`: no record file. Run the proof again and keep the log under the batch folder.
- Changed after their record: `used-towel-radiator`, `used-wall-hung-gas-boiler` (MC record 11:26, file 16:40); `used-tumble-dryer` and `used-harmonium` (file 16:40, after the ML-1 window).
- Open, needs repair or a merged allowlist: `used-harmonium` (21 fail `paper-height`, unless the MX claim is re-proven with a log), `electric-fan-heater` (model-bounds), `used-hinged-wardrobe` (1:20 print), the 10 MB-2 assets with allowlist pending, `tealight`, `used-bollard-light`, the 5 MB-1 assets (`etics-brick-wall-12`, `etics-brick-wall-30`, `solid-brick-wall-1960`, `spiral-stair`, `stair-flight`), `pendant-lamp` and the 9 MS assets.
- Current by record but MX-2 is IN WORK: `glulam-beam` and `steel-column` (MX-2 logs, 16:40). Confirm when MX-2 closes.

## Counts

| Class | Assets |
|---|---|
| proven-current | 21 |
| proven-but-file-changed-since | 2 |
| claimed-without-log | 69 |
| open | 30 |
| total unique assets | 122 |
