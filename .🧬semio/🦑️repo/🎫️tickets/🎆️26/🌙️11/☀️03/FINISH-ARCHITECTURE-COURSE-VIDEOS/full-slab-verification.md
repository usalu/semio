> Historical v2 proof for the previous model input. The current model SHA starts bd43e558 and requires the separate full-slab-v4 gate and 20-input freeze. The former v2 reuse instructions below are historical; follow FULL-SUITE-PLAN.md and slab-cache-v4-validation.md for current work. Do not relabel old model-key entries.

# Full Native Slab Verification

Render Python3.12; dedicated full-slab-v2 cache; frozen19source/schema/manifest inputs. Each recorded row passed actual warm-hit detection, typed exact cold/warm equality and SHA-256 equality. All ten full-model slabs passed; validator session79144 exited0. All19frozen source/schema/manifest hashes matched again after completion.

| Slab | Cold seconds | Warm seconds | Typed geometry SHA-256 |
|---|---:|---:|---|
| SL-1 | 60.891 | 34.484 | b2c76435d641b0984ad0f0c7cec3823f0c02438b78868dfa7abd2f09a9460c79 |
| SL-R | 9181.953 | 45.828 | 6ff2fe59a66135c9ac39ea943291b4e32862505508f7a48fca14bd6673d976c4 |
| SL-KIOSK | 852.547 | 29.922 | 6160e1dbb2fe99375e3fd3815e79b86a92f48d29c05a0bd25ce2e3dba81ebb65 |
| SL-CELLAR | 45.937 | 35.344 | c8465c2de424fd99e5e7ab82eb9e140bf8565b42a39c88848baa306eee261068 |
| SL-GROUND-CELLAR | 46.312 | 33.532 | 54b69cdaa865fd6173fff1a5d5cb2b1486bb77114ea9e90b4a0de4fe8d2883a4 |
| COVER-HOIST | 41.735 | 31.796 | 39d1b3879df15f08385feb0f306706a03abab0d2944792b64484813f8024aacd |
| COVER-COAL | 44.485 | 30.797 | 579a06c322094a88a256564df273a09c4148d7ae2dc8b89e6cee73253dda2ec2 |
| SL-GROUND | 50.64 | 34.422 | a615d8395aa425a7f8d52c4897a128ff00830aa7f5bb0a9c8f0dfc3e65628b8e |
| SL-STAIR-CANOPY | 58.828 | 32.656 | 65ae4171cc6aa84b875a10617a0f1c1b78589ea5ce4a259e2b596b3e2c222d97 |
| SL-STAIR-WEATHER | 53.359 | 34.422 | 55eced92a57927910d96f934ca1f47268754a14122ef84128c241a1e6912ba47 |

The full native gate is complete. Subsequent BIM rendering and canonical tests must reuse SEMIO_BIM_SLAB_CACHE pointing to this ticket generated/full-slab-v2 directory; source fingerprints remain authoritative.

## Production Cache Freeze Recheck

At 2026-11-07T13:26:35.752Z, a direct SHA-256 recheck of all 19 frozen model/cache files found 0 differences. This checks input identity only; the ten native cold/warm equality results remain recorded in the table above.
