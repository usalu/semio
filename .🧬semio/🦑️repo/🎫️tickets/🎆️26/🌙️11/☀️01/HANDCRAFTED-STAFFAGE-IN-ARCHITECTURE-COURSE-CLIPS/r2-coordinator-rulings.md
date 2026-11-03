# Coordinator rulings on round-2 open items

Rulings on the open items in `r2-open-items.md` that the open-items ledger left to the coordinator or to judgement.

## No change (ruled)

| OI | Ruling |
|---|---|
| OI-04 | Keep `energy_tank` in practice. It is the abstract "Quelle" source gauge of the Raum, Zähler and Quelle balance, not a building store, so the abstract-gauge rule applies. |
| OI-21 | 1.48 m for a 200 L cylinder with a 0.60 m jacket is inside the real product range (about 1.2–1.5 m). Accepted. |
| OI-27 | 300 W of sensible and latent gain from a 38 °C shower spray into the room is plausible. Kept. |
| OI-41 | Thermometers stay diagram glyphs (round-1 ruling). |
| OI-46, OI-47 | Nakagin podium furniture and the single pose of the Excel plan occupants: round-1 rulings stand. |
| OI-54 | No pen-series policy exists in the archive rules, so only outliers are snapped. |
| OI-73 | A native German review cannot be done by agents. Recorded as a limit, not a defect. |
| OI-90 | `round.sh` already skips modules 27, 33, 34 and 35, which have no tests. |
| OI-93 | JSON case files are the language-agnostic tests that AGENTS.md asks for; a `.feature` is not required. |
| OI-116 | A 0.5 m vent jet at 1.1 L/s from a slot vent is physically plausible. |
| OI-117, OI-120, OI-122 | Plausible as designed. |
| OI-129 | A search facet is a new feature outside this ticket. |

## Dispatched (fix)

| OI | Ruling | Agent |
|---|---|---|
| OI-34 | Move the wall out of the panel-radiator asset. An asset is the device, and the wall belongs to the scene. | ap |
| OI-48 | Move the evaporative humidifier to 🏗️machinery, next to the plate HX and the sorption wheel. | ap |
| OI-113 | The toaster casing must stay cool-touch (about 60 °C). Lower the casing share of the heat output. | ap |
| OI-114 | Split the pumpjack skid from the motor. A 30 kW motor weighs about 250 kg. | ap |
| OI-134 | State the HRV emitter reference (room or outdoor air) and keep the emitters consistent with it. | ap |
| OI-87, OI-123, OI-133 | Render-review the remaining touched assets, improve the weak drawings, and fix the plan dimension labels that show height. | av |
| OI-67, OI-112, OI-121, OI-132 | Clip polish: the volume dimension overlap, a solar-fed regeneration coil, `staffage_height`, and the conduction Beat4 height dimension. | cl |
| OI-128 | Offer the `span` range mode in the React viewer. | rv2 |
| OI-23 | Kettle dry mass of 1.22 kg. | kr |
| OI-83, OI-85, OI-86, OI-91, OI-115, OI-119, OI-131 | Verification. | lx, xl, cr2, rr/tr/pr, ne |
| OI-01, OI-02, OI-80, OI-81, OI-118 | BIM. | br |
| OI-74, OI-109, OI-127 | Summary and contract wording at close. | coordinator |

## Later rulings

- **Thermal span:** the unit that must share one `thermalSpanK` is the Scene class: what is on screen together, not the clip file. Each scene uses the smallest span that covers the hottest cast shown with the others. The default is 25 K. Examples: the physics kettle and device beats use 78 K; the comfort and internal-gains scenes use 25 K.
- **Narration numbers:** the number stated on screen or in the narration is the lesson. The cast runs at it through its state or load. The text is corrected in both en and de only where the number is physically wrong for the depicted device.
- **Oak feature measure:** speed it up with exact techniques only. Never lower the raster cap or change measured widths.
- **Nakagin tower:** build the geometry once and recolour it in place per variant, with pixel-identical output.
- **Depiction language:** textless depictions carry no language at all. Only text-rendering callers pass one.
- **Gendered person titles** in the catalog stay as authored (persona).
- **Tests:** a skip is allowed only for a missing external tool that the repo does not provision. Bun is provisioned by `repo`, so a test must not silently skip without it.
- **Platform tests:** a test of an operating-system feature that does not exist on the host may skip on that platform. Examples are POSIX signals on Windows and job objects on POSIX. Each platform must have its own equivalent test. This covers the cast-export SIGTERM test and the `integration.py` platform guards (`r2-sk-report.md`). A missing tool that `repo` downloads and provisions fails loudly. A `system` tool that `repo` only registers and cannot provision, such as ffmpeg (`r2-ff-report.md`), may skip with a visible reason that names `SEMIO_TOOL_FFMPEG`.

## Owner or infrastructure (cannot be done in this ticket)

- OI-110 and OI-111: no TTS key is configured, and the speech tooling lives outside the staffage scope.
- OI-84: the devcontainer cannot run because the Docker daemon is stopped.
- OI-100 to OI-102, OI-106, OI-124 to OI-126, OI-130, OI-135: repository infrastructure and git.
