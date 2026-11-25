IN WORK

# 🧭️ Multiview Lane MU: the 145 Assets No Batch Owns (2026-11-25)

State of 20:30, third fixer of the lane. A result below counts only where its log ran to the end; "claimed" marks what a predecessor wrote without a log, "not run" what did not run.

List `🧰️matrix/batches/MU.tsv` (written by `🧰️matrix/MU/list.py`: 375 catalog assets minus every asset of `batches/M*.tsv` minus `electric-fan-heater`): components 37, objects 31, vegetation 22, appliances 21, furniture 20, machinery 8, vehicles 3, humans 2, animals 1. Tools `🧰️matrix/MU/`, repairs `🧰️matrix/MU/ops/<id>.json` (written by `make/*.py` where a writer exists) applied by `fix.ts`.

## 1. Measured

- Authors' gate `T/🔎️scan.ts` over the 123 assets that are not plants (`chain/scan-rigid.sh`, 20:05): 123 clean, 0 hard findings. Over the 22 plants: not run yet (`chain/trees-rest.sh`).
- Matrix per asset (`matrix1.sh`, records `🗑️generated/matrix/MU/matrix-rigid` and `matrix-trees`): running, see section 3.

## 2. Repaired assets

| Asset | Before | Repair | State |
|---|---|---|---|
| `used-electric-bass-and-cabinet` | fixer's gate 120+ hard (flat castor wheels) | 16 wheel shapes solid (`ops`) | landed 16:25: scan clean, validate 0, prints, check ✓; matrix 18:xx without fail; suites not run |
| `reclaimed-cobblestone-bag` | fixer's gate hard findings in the transition reinstalled > stored | `make/reclaimed-cobblestone-bag.py` | landed 16:25: scan clean, validate 0, prints, check ✓; matrix without fail; suites not run |
| `reclaimed-paving-slab-pallet` | same kind | `make/reclaimed-paving-slab-pallet.py` | landed 16:28, same proofs; suites not run |
| `excavator` | 3 hard (1:500 does not print, `bucket-link` 0.18 mm) | `make/excavator.py` | landed 17:37: scan clean, validate 0, check ✓, matrix 19:03 175 ok, 56 group-drift, 21 range-end, 0 fail; the range end 1:20 does not print (`mirror-arm-right-2` 0.193 mm < 0.2 mm) as before the repair; suites not run |
| `delivery-van` | fixer's gate 86 hard (flat tyres on the spinning wheel bones: `sync-height` 220 mm, `anchor-landmark` 259 mm) | `make/wheels.py`: tyres carried by the axle bones | landed 20:2x: scan clean, validate 0, check ✓; the range end 1:150 does not print (`tail-pipe` 0.141 mm) as before; matrix and suites pending |
| `electric-kick-scooter` | fixer's gate 8 hard (1:200 rear wheel, 108 mm) | `make/wheels.py` | landed: scan clean, validate 0, prints, check ✓; matrix and suites pending |
| `used-timpani-pair` | fixer's gate 3 hard (1:500 mallets, 52 mm) | `make/wheels.py`: tyres on the castor bones, mallets solid at 1:500 | landed: scan clean, validate 0, prints, check ✓; matrix and suites pending |
| `lavender-bush` | matrix 7 fail `derived-view-in-band` (1:500 symbol, front equals side, 0.0197) | `ops/lavender-bush.json`: front of the symbol with four spikes over the 0.76 m face | landed: scan clean, validate 0, prints, check ✓; matrix and suites pending |
| `running-child` | matrix 14 fail `sync-bone-height` (1:500 right forearm, 23.4 mm against 21.9 mm) | `ops/running-child.json`: the side elevation takes the body of revolution of front and plan | trial clean (gate 0 hard, validate unchanged, silhouette side 0.9460 > 0.9399); landing pending |
| `sunbather-on-towel` | matrix 70 fail (`sync-bone-height` 56, `sync-height` 56, `sync-lateral` 28) | `make/sunbather-on-towel.py` | in work |
| `used-bentwood-dining-chairs` | fixer's gate 120+ hard (the two upper chairs turn by 180 degrees) | `make/used-bentwood-dining-chairs.py` | in work: the catalog file is the untouched original again (the write of 17:33 had 4 validate issues, check ✗ and a failing silhouette and was taken back at 19:02) |

## 3. Open

Filled at the end.
