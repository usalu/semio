# 📝️ haiku-consumer-audit: Course Call Sites (Handedness, Mirror, Sizing), Relayed by the Coordinator

This is a static audit. It covers all 33 files and their 169 call sites. Asset checks were sampled. `MAN` is the cast manifest `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json`.

## Mirrored casts with asymmetric content

The manifest flag is invisible at the call site, because Python `staffage()` defaults to `mirror=False`.

| Where | Cast and asset | Risk |
|---|---|---|
| `MAN:27`; `🎯️optimization/…/🏠️room:1350` | `optimization-room-cooler` (split-ac-indoor-unit) | The unit has a single display, a single wall rest and one jet outlet; the mirror moves them to the other side. |
| `MAN:35,36`; `🌬️ventilation/…/🪟️natural:231` | `ventilation-natural-radiator-on` and `-off` (panel-radiator) | The radiator has one valve head and one supply pipe; the mirror moves the valve to the other side. |
| `MAN:54`; physics fundamentals 425 and 443; heating solar-gains 823–989 | `shared-blind-lowered` | The mirror reverses the sun-facing slat cut-off. |
| `MAN:41`; ventilation volume 424 | `ventilation-volume-hood` | Probably symmetric; low risk. |
| ventilation volume 420 | `shared-office-worker-pathlines`, `mirror=True` ×3 | Probably symmetric; check the hands in the typing clip. |
| optimization room 588 | `shared-radiator-winter`, `mirror=True` | Valve side. |
| heating calculation 1226–1227 | `shared-radiator-thermostat`, second instance | Valve side; the mirror is deliberate, but the wall side is unverified. |
| ventilation need 159 | `shared-person-pathlines`, alternating mirror | standing-adult is symmetric; low risk. |

## Sizing by magic constants (front and plan casts)

| Where | Cast | Issue |
|---|---|---|
| physics fundamentals 1759, 1914 | `shared-radiator-front-winter` | `unit=1.2` and `fe_metre` instead of a size derived from the cast's height |
| heating conduction 1034 | `shared-radiator-thermostat-front` | Magic `/1.75` |
| demand practice 154, certificate 263, primary-energy 144 | `shared-radiator-front-winter` | Magic `/2.5` room height |
| demand recap 205; heating fundamentals 123 | `shared-radiator-front-winter` | `_metre(H)` and `_ROOM_METRE` are unverified |
| optimization extrema 440 | `optimization-extrema-cooler` (front) | Fixed `unit=.8` |
| optimization experiments 904–905 | window (front) beside occupant (elevation) | Both use `unit=.5`, and the icon mixes views |
| bim information 212–214, 343 | `bim-information-car` (plan) | Non-uniform `stretch`, which breaks the real dimensions |
| excel program 368 | `excel-program-occupant` (plan) | Rotated by −π/2 |

## Coordinator rule D19

- A mirror on an asset with lateral asymmetry needs a reason in the guard's allowlist. Lateral asymmetry means anchors or parts with a non-zero side offset (`z`) that matters, or single-sided parts.
- Every mirror must be visible at the call site, through an explicit parameter or a cast id suffix.
- Every cast is sized from its real dimensions, using `heightM`, `boundsM` and the scene's metre, with no magic constants.
- Non-uniform stretching of a cast is forbidden; the guard rejects it.
- A rotation of a plan cast is allowed only as a documented physical yaw in the scene.
