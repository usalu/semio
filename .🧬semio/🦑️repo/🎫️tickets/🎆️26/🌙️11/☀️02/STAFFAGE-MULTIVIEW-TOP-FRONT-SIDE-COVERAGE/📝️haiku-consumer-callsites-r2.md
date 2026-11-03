# 📝️ haiku-consumer-audit: Course Call Sites, Second Pass (relayed by the coordinator)

This is a static audit against D19, comparing with `📝️haiku-consumer-callsites.md`. `…` stands for the chapter path under `🎓️teaching/🏛️architecture/⚡️energy`.

## Fixed

- The four hidden manifest mirrors are now visible as `-mirrored` cast ids:
  - room cooler (`🏠️room:1350`);
  - natural-ventilation radiators (`🪟️natural:231`);
  - volume hood (`📦️volume:424`);
  - venetian blind (`🧲️fundamentals:425`, `☀️solar-gains:823`).
- extrema: `unit` is derived from `staffage_height` (`📉️extrema:440`).
- BIM car: the scaling is uniform (`📊️information:213,343`).
- The clip key includes `cast_key` through `casts_in` (`🎬️clip/⚙️runtime:436-437`), and this is tested.

## Open

| Item | Where |
|---|---|
| Python `mirror=True` without a stated reason | `🏠️room:588`, `📦️volume:420`, `❓️need:159`, `🧮️calculation:1226-1227` |
| Magic sizing (`/ 2.5`, `unit=1.2`, `/1.75`, `unit=.5`, `unit=0.8`, `width=0.5`) | `🧲️fundamentals:90,123,128,1572,1759,1912,2201`; `🧱️conduction:1034`; `🧪️practice:154`; `🏷️certificate:257,263,846`; `🏭️primary-energy:144`; `🔁️recap:193-195`; `📉️experiments:904-905`; `🧮️calculation:1206-1207`; `👥️internal-gains:133,143,249,582,588,618,621,681,910`; `⚖️regulations:423,864`; `♨️system-losses:461`; `⚡️demand-power:253` |
| No named storey-height constant anywhere in the course | — |
| Plan rotation not documented as a yaw | `📋️program:368` |
| Crate casts named by f-string, so `casts_in` misses them (stale-render risk) | `📦️volume:407-408`, manifest `:43-45` |
