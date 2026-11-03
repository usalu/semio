# r2 rl: reconcile the "Known leftovers" table (read-only verification)

Scope: each row of the table in `summary-r2-draft.md` (lines 7-25), checked against the current code and the newest r2 reports. No source files were edited. Cast ids were checked against `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` (98 casts).

## Reconciled table

| Item | Verdict | Evidence |
|---|---|---|
| TTS key unset, OI-110/111 | Owner/infra | `r2-tt-report.md:19` (no key, `🔊️speech/🐍️.py:272-273` returns None), `:28-30` open; rulings "Owner or infrastructure" |
| Docker daemon stopped, OI-84 | Owner/infra | `r2-px-report.md:5`, `:35`; rulings "Owner or infrastructure" |
| Linux vitest needs sudo bootstrap | Owner/infra | `r2-lx-report.md:9` (no passwordless sudo), `:32-33` (owner action) |
| Energy series German on screen | Ruled (by design) | `r2-tx-report.md:10`, `:25`. Not in `r2-coordinator-rulings.md`, so the ruling rests on the tx report only |
| `repo dependencies ensure` refuses, d3-format unlinked | Owner/infra | `r2-ty-report.md:15`, `:25` (repo infra, left alone) |
| Gendered titles | Ruled (as authored) | `r2-coordinator-rulings.md` "Gendered person titles ... stay as authored". The bc suggestion `Jogger:in` (`r2-bc-report.md:23`) is superseded |
| BIM geometry over 2 h, OI-01/02/80/81/118 | Still open | `r2-rb-report.md:14`, `:19` (`ModelCase.setUpClass` builds geometry, over 2 h per probe; "not fixed here"); `r2-br-report.md:28-32` (one suite still running). No `r2-bv-report.md` exists |
| OI-04 energy_tank | Ruled (keep) | `r2-coordinator-rulings.md` OI-04 |
| Nakagin memory | Still open | `r2-xl3-report.md:3`, `:45-47`: full tower drawing about 5.1 GB, above the 5 GB guard; target not met |
| OI-10/11 cast memory and frame budget | Still open | `r2-sz-report.md:42-44` (oak and volume figures above 200 KiB; OI-11 undecided); `r2-pf-report.md:46` |
| OI-23 kettle mass | Fixed | `🔌️appliances/electric-kettle/🔣️.json:51` `massKg` 1.22 (base 0.55 + body 0.45 + handle 0.12 + lid 0.10). Filled 2.22 kg in en/de text (`:43-44`) is consistent. `r2-kr-report.md` OI-23 section. Thermal solve does not read `massKg` |
| OI-27 | Ruled (no change) | `r2-coordinator-rulings.md` OI-27 |
| OI-112 | Fixed (per report, not re-read in the clip code) | `r2-cl-report.md:7` (solar-fed regeneration coil) |
| OI-113 toaster | Still open (text defect) | Numbers fixed: casing 95 W on 0.24 m2 (about +40 K, about 60 degC) per `r2-ap-report.md:20-21`. But the de text `🔌️appliances/toaster/🔣️.json:43` still says "545 W als heiße Schlitzluft" while en `:42` says 630 W. Sum 95+25+150+545 = 815, not 900. Fix the de sentence |
| OI-114 pumpjack skid and motor | Fixed | `🏗️machinery/oil-pumpjack/🔣️.json:141-155` skid node 850 kg; `:302-316` motor 250 kg (was 1100). Total 5400+850+2400+2900+350+3200+2600+180+250 = 18130 matches `:54` `massKg` |
| OI-134 HRV emitter reference | Fixed | `🔌️appliances/decentralised-hrv-fan/🔣️.json:1910-1911` (en and de): both emitters are relative to 21 degC room air, not outdoor air |
| OI-124/125 test imports, oracle docstrings | Owner/infra | rulings "Owner or infrastructure" (OI-124 to 126) |
| OI-126 untracked files | Owner/infra | rulings; commit only on request |
| OI-115/131 renders re-checked | Still open (unverified) | No `r2-cp3-report.md` exists. No r2 report cites OI-115 or OI-131 |
| Guard is name-based | Still open (limit documented) | `r2-hz-report.md:10` (cannot see primitive helpers); proposals in its section 4 |
| `_WATT_DEVICES` orphans | Fixed | `🎨️visuals/🐍️.py:252-270`: `_WATT_DEVICES` has only laptop, toaster, heater, person, seated, sun. No bulb, monitor or vacuum. Manifest has no `shared-watt-bulb|monitor|vacuum` rows. `🧪️tests/🐍️icons.py:63` iterates the current `_WATT_CASTS`. `audit-final-static-r2.md:9` line refs (322-345) are stale |

## Specific checks

- **Manifest casts unused by every clip (orphans):** 98 manifest casts. A literal scan of clip and visual code (excluding tests) finds 85. The other 13 are not orphans: their ids are built dynamically.
  - `cooling-internal-gains-{sleeper,seated-worker,walker,jogger,standing,laptop,laptop-closed,server-rack}` are built as `_CAST + name` in `⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:93,116,166`, and `lamp-dimmed` via `_lamp(..., "lamp-dimmed")` at `:687,:916`. `office-chair` is used at `:146`.
  - `ventilation-volume-crates-{low,half,high}` are built as an f-string at `⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py:406-407`.
  - Result: **0 orphans**.
- **Clip references missing from the manifest:** a literal scan of `⚡️energy` and `🏢️nakagin` for the nine manifest prefixes finds 80 ids. All are in the manifest except:
  - `cooling-internal-gains-`, a dynamic prefix (see above). Not a missing cast.
  - `excel-tab`, `shared-array-increments`, `shared-array-offset`, `shared-array-root`, `shared-dim`. These occur in no architecture source outside venv and cache directories, so they are not clip references.
  - Result: **0 missing**.
- **Kettle `massKg`:** 1.22 in the catalog, not mirrored in the manifest. The thermal solve does not use it.

## Open items for the owner or coordinator

1. Fix the toaster de text (`🔌️appliances/toaster/🔣️.json:43`, 545 W to 630 W) so en and de agree and sum to 900 W.
2. Nakagin full-tower memory (5.1 GB against the 5 GB guard) and BIM `setUpClass` cost remain open, as recorded in `r2-xl3-report.md` and `r2-rb-report.md`.
3. Render re-check for OI-115/131 has no report. Either run it or record it as not done.
4. Update the stale line refs in `audit-final-static-r2.md:9` (or the summary) so they do not claim the watt orphans are still present.
