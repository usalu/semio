# Rotating appliance geometry and performance checks

The three live catalog assets each have four digital scale models and three available paper representations. All 12 authored volume scales were independently compared with their ticket authoring documents after publication.

| Appliance | Live SHA-256 | Valid GLB scenes | GLB bytes | Paper scale and denominator combinations | Clip instants across four scales |
|---|---|---:|---:|---:|---:|
| duct-fan | `11237b05486324cef30c6567cf89ae887b1f0c66cfde4b765385c9347c563824` | 12 | 1,372,476 | 333 | 384 |
| decentralised-hrv-fan | `a17fb615698ee618acbe9ade98d61e2f883e48504eecfa8b8d5e09c46dfde2ec` | 12 | 1,708,608 | 333 | 408 |
| circulation-pump | `7a93dd4fd8fd5b78b5d43d80da9eac2119476b7df59a741ef09b6f0902754c82` | 12 | 2,108,504 | 333 | 564 |

## Five-blade ventilation rotors

Each model retains five twisted, tapered blades at all four digital scales. The inlet, outlet, motor, 8 mm drive shaft, 18 mm fixed bearing and support struts remain distinct. The shaft turns in the fixed bush with 0.05 mm radial running clearance. The actual tessellated blade vertices and inscribed housing bore give the following minimum gaps across all four scales.

| Appliance | Real parts retained | Minimum tip-to-housing gap | Minimum blade-to-strut gap |
|---|---|---:|---:|
| Duct fan | 30 mm inlet ring, 60 mm spigot, 34 mm radius hub, four 8 × 16 mm struts | 0.576 mm | 19.357 mm |
| Reversing HRV fan | 75 mm inlet ring, 220 mm wall sleeve, 28 mm radius hub, three 8 × 12 mm struts | 1.576 mm | 14.533 mm |

The duct fan now turns at its two depicted running speeds during the six-minute operating clip. The HRV fan reverses both rotor rotation and the active air emitter. Its regenerative core has 37 open hexagonal passages through its 100 mm length. All 1,628 sampled channel positions were clear over the four scales.

## Wet-rotor circulating pump

The terminal casing measures 20 × 60 × 66 mm, with a 16 mm saddle and a 14 mm lid. The 25 mm clear-bore pipe tails and hollow union nuts lead to open casting ports. The 62 mm impeller has six backward-curved vanes, a 2.5 mm back plate and a covered eye. A pierced cast divider separates the inlet plenum from the impeller discharge. The offset casting and both port necks clear the wheel by at least 3 mm at the pipe ends. The 2,016 mesh-containment probes follow the open pipe bores, inlet bend, suction eye, and the outlet side across all four scales. An independent solid-interior witness identifies the intact diaphragm outside the eye. These are geometric checks of the modeled passages. Flow-rate calibration would require hydraulic performance measurements.

An 8 mm ceramic shaft runs through two ceramic radial bushes. Separate fixed seats and a carbon thrust washer support the rotor. Its stainless can surrounds the rotating drum with 0.8 mm radial clearance. The pointer is bound to the drive shaft. The geometry follows the construction materials in the pump manufacturer’s [UPS2 sectional drawing and specification, page 6](https://api.grundfos.com/literature/Grundfosliterature-5173526.pdf). Internal component sizes and material-property values are modeling estimates.

## Ordinary, structural and thermal checks

The complete source-schema, drawing, 3D body, thermal coverage, structure and print validator returned zero findings for each live entry. The GLB mesh audit and independent binary scene reader passed all 36 presentation scenes. Mesh vertex witnesses were compared with the independently transformed rest positions for every clip and scale. The fixed assemblies held their position, and the moving witnesses described the expected rotary travel.

Each authored paper band was checked at every integer denominator within its declared interval, including both representations at shared endpoints. All three models keep 1:500 available digitally and unavailable for paper. The paper sections show the sectioned stock, including the impeller, casing wall and carbon thrust ring.

All nine named operating states were solved at the reference 21 °C room and a common 14 °C shaded room with a 0.3 m/s air speed. At reference conditions the duct motor surface resolves to 21.00, 39.20 and 54.22 °C for off, low and high. The pump motor resolves to 21.00, 26.46 and 60.00 °C. The HRV core resolves to 21.00 °C in extract and −3.34 °C when supplied from its −5 °C outside-air boundary. These are results of the authored heat-transfer model.

The replay inputs are `rotor-dimensions.py`, `rotor-motion.ts`, `rotor-climate.ts`, `rotor-paper.ts`, `r3-validate.ts` and `digital-scenes.ts`. The individual construction references are recorded in [the duct-fan audit](r3-appliances-duct-fan.md), [the reversing-fan audit](r3-appliances-decentralised-hrv-fan.md) and [the pump audit](r3-appliances-circulation-pump.md).
