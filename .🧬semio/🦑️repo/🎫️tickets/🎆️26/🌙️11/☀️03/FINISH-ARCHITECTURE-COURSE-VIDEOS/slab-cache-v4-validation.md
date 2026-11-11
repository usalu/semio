# BIM native slab validation, corrected bilingual model

Gate session 60843 uses model JSON SHA-256 bd43e5585fce0eec30332c4eb8b953451e89d84af8ee09f385139bf10aa926e6 and the 19 unchanged producer/cache inputs in generated cache-freeze-v4.json. All 20 matched before launch. Its distinct cache is 🗑️generated/full-slab-v4; neither v2 nor incomplete v3 records are reused.

The retained full-slab-validation.py enumerates all ten native IfcSlab entries and checks exact typed cold/warm equality, digest equality and a real persistent warm-cache hit. A passing exit and post-run recheck of all 20 frozen inputs are required. The optional v2 digest comparison is additional evidence and does not replace this computation.

| Slab | Cold seconds | Warm seconds | Warm hits | Exact digest | v2 comparison |
| --- | ---: | ---: | ---: | --- | --- |
| SL-1 | 39.657 | 22.109 | 1 | b2c76435d641b0984ad0f0c7cec3823f0c02438b78868dfa7abd2f09a9460c79 | matches v2 |
| SL-R | 6031.500 | 32.750 | 1 | 6ff2fe59a66135c9ac39ea943291b4e32862505508f7a48fca14bd6673d976c4 | matches v2 |
| SL-KIOSK | 537.859 | 21.922 | 1 | 6160e1dbb2fe99375e3fd3815e79b86a92f48d29c05a0bd25ce2e3dba81ebb65 | matches v2 |
| SL-CELLAR | 33.469 | 29.797 | 1 | c8465c2de424fd99e5e7ab82eb9e140bf8565b42a39c88848baa306eee261068 | matches v2 |
| SL-GROUND-CELLAR | 41.562 | 28.891 | 1 | 54b69cdaa865fd6173fff1a5d5cb2b1486bb77114ea9e90b4a0de4fe8d2883a4 | matches v2 |
| COVER-HOIST | 38.641 | 30.031 | 1 | 39d1b3879df15f08385feb0f306706a03abab0d2944792b64484813f8024aacd | matches v2 |
| COVER-COAL | 37.563 | 28.515 | 1 | 579a06c322094a88a256564df273a09c4148d7ae2dc8b89e6cee73253dda2ec2 | matches v2 |
| SL-GROUND | 38.828 | 27.704 | 1 | a615d8395aa425a7f8d52c4897a128ff00830aa7f5bb0a9c8f0dfc3e65628b8e | matches v2 |
| SL-STAIR-CANOPY | 44.484 | 23.750 | 1 | 65ae4171cc6aa84b875a10617a0f1c1b78589ea5ce4a259e2b596b3e2c222d97 | matches v2 |
| SL-STAIR-WEATHER | 35.781 | 23.328 | 1 | 55eced92a57927910d96f934ca1f47268754a14122ef84128c241a1e6912ba47 | matches v2 |

Gate session 60843 exited 0. All ten cold/warm comparisons and persistent hits passed; all 20 frozen input hashes matched after exit, and all ten typed geometry digests match the historical v2 table.

The corrected-model gate passed.
