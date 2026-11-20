# Particle-Path Arrow Inventory (Lüftung)

## Goal

Hide (not delete) arrows that mark air-particle direction across all ventilation
video scenes. Keep moving particles visible.

## Mechanism

Shared helper `flow_guides` in
`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py` draws faint
tipped streamlines beside particle streams. Every Lüftung chapter imports and
plays it via `Create` / `FadeIn`.

## Chapters that draw particle-path guides

| Chapter | Uses `flow_guides` | Extra air-direction `Arrow` with particles |
|---|---|---|
| `❓️need` | yes | no (q-arrows are heat flux) |
| `📦️volume` | yes | no (axes / conversion) |
| `🌡️heat-flow` | yes (`_guides`) | no |
| `🌟️natural` | yes | `inlet_arrow` / `outlet_arrow` on stack beat |
| `⚙️mechanical` | yes | duct / heat arrows (ports / WRG, not particle paths) |
| `🤝️handoff` | yes | series-chain UI arrows |
| `🗺️roadmap` | no | — |

## Approach

1. Add `visible` to `flow_guides` (default `True` so heating/cooling unchanged).
2. In each ventilation chapter, rebind `flow_guides` with `visible=False`.
3. On the natural stack beat, set `inlet_arrow` / `outlet_arrow` opacity to 0
   (code stays; labels remain).
4. Language-agnostic case: invisible guides keep geometry; particles still
   animate.
