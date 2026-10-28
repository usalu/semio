# Native Caption Regression

The first bilingual repair batch passed the unchanged real caption_bar regression in 278.390 seconds.

The remaining 15 canonical caption cases passed in 152.250 seconds. Each case checks both EN and DE and reproduces the prior overflow.

- `🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` / `Beat9_WaermestromFormel` / `power`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` / `Beat3_LueftungslastImSommer` / `intro`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` / `Beat3_LueftungslastImSommer` / `sens`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` / `Beat1_SolarIrradiance` / `chart`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` / `Beat5_SolarCoolingLoad` / `intro`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` / `Beat2_VolumeFlowEquation` / `formula`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` / `Beat3_IsolateAirflow` / `substitute`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` / `Beat3_IsolateAirflow` / `qvr`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` / `Beat3_IsolateAirflow` / `load`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` / `Beat3_IsolateAirflow` / `compare`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` / `Beat1_LastZuerstSenken` / `reserve`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` / `Beat2_SorptionsKuehlung` / `intro`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` / `Beat2_SorptionsKuehlung` / `nachteil`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` / `Beat3_KomfortStrategie` / `yes`
- `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` / `Beat3_KomfortStrategie` / `reserve`

## Opaque Envelope Scope Restoration

The English opaque-envelope qualifier was restored in the canonical narration, portable case, and retained repair input. The targeted source/contract test and original English/German caption_bar oracle passed in 17.110 seconds; the old English overflow remains reproduced. No broad test repeat was needed after this one-word data correction.

## Caption Count Reconciliation

The portable contract contains **39 canonical scene/key cases and 44 previous-language strings**; five keys have both EN and DE previous strings. This is **43 overflows from the all-source caption sweep plus one earlier Physics comparison failure**. It is not a claim that the sweep found 44 failures.

The retained records establish the partition: `caption-repairs.json` contains 42 unique language-specific sweep repairs; the separately registered ventilation `Beat3_LatenteLast` / `dx` / EN case is the 43rd. The remaining previous string is Physics `Beat4_Waermepumpe` / `compare2` / DE, whose earlier three-line failure is still recorded in `🗑️generated/smoke/results.json`. Every retained repair's previous text matches its portable case. The original caption oracle has validated all 44 previous strings across the 39 keys.

The later **Opaque envelope** qualifier restoration changes one English narration datum and its matching portable case/repair input. It adds no new overflow occurrence or canonical key. The targeted original caption oracle check establishes the restored wording still fits.
