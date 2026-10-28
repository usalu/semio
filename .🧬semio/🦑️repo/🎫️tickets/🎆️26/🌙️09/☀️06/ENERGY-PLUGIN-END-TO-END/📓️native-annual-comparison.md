# Native Annual BESTEST Numerical Comparison

All four native cases completed the real production batch driver with 30 warmup days and 8,760 hourly result samples, six numerical timesteps per hour. EnergyPlus 25.2.0 hourly SQL from the committed exchanged fixtures is the independent comparison. Both producers use the same official Denver EPW. No tolerance was changed for this run.

| Case | Metric | Native | EnergyPlus | Difference | Limit | Result |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| 600 | Temperature RMSE K | 0.104069 | 0.000000 | +0.104069 | 2 | PASS |
| 600 | Annual heating kWh | 4358.947937 | 4324.744191 | +0.79% | 20% | PASS |
| 600 | Peak heating kW | 3.208487 | 3.204159 | +0.14% | 25% | PASS |
| 600 | Annual cooling kWh | 6067.257783 | 6044.058559 | +0.38% | 10% | PASS |
| 600 | Peak cooling kW | 6.334380 | 6.352444 | -0.28% | 25% | PASS |
| 900 | Temperature RMSE K | 0.020516 | 0.000000 | +0.020516 | 2 | PASS |
| 900 | Annual heating kWh | 1674.211592 | 1661.137877 | +0.79% | 20% | PASS |
| 900 | Peak heating kW | 2.689871 | 2.687473 | +0.09% | 25% | PASS |
| 900 | Annual cooling kWh | 2491.617050 | 2498.161384 | -0.26% | 10% | PASS |
| 900 | Peak cooling kW | 3.044990 | 3.041487 | +0.12% | 25% | PASS |
| 600FF | Temperature RMSE K | 0.559791 | 0.000000 | +0.559791 | 2 | PASS |
| 600FF | minC | -12.774986 | -12.560575 | -0.214411 | 2.5 | PASS |
| 600FF | maxC | 64.756515 | 63.920004 | +0.836511 | 2.5 | PASS |
| 600FF | meanC | 24.886125 | 24.960640 | -0.074515 | descriptive | descriptive |
| 900FF | Temperature RMSE K | 0.057791 | 0.000000 | +0.057791 | 2 | PASS |
| 900FF | minC | 1.134930 | 1.246663 | -0.111733 | 2.5 | PASS |
| 900FF | maxC | 44.325917 | 44.299161 | +0.026756 | 2.5 | PASS |
| 900FF | meanC | 25.094471 | 25.133316 | -0.038846 | descriptive | descriptive |

The exact reproducible comparison is `🧪️compare-native-bestest.py`; it reads actual native result JSON and EnergyPlus SQLite records, rejects missing annual hours, and independently integrates every controlled heating/cooling hourly trace back to the native annual meter. Temperature comparisons use EnergyPlus hourly means against native hourly output.

Simulation regression suites after adding native hourly power series: sim-unit 29 passed; restore 5 passed; wire 5 passed. The BESTEST projection suite previously passed all 16 tests. Progress, cancellation, input identity, packet custody, finite caller fuel and terminal retirement remain exercised by the real caller harness. The harness uses the first-party testing authority only to supply genuine memory receipts; it does not substitute the numerical kernel.
