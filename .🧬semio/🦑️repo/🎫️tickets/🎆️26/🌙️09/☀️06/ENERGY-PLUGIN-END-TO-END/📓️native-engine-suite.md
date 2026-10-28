# Native Energy Engine Suite

All 55 selected production energy engine packages passed their native library tests: 290 tests, zero failures. The package list is the energy engine set that does not depend on the separately compiling Store replay/application graph. Selection includes numerical physics, kernel, simulation, restore, wire, retirement and BESTEST.

The suite exposed two genuine issues now fixed: zero-depth retirement denied ownership too late, and the run dispatcher consumed the only fuel before funded checkpoint page admission. The one-fuel/four-fuel restore chronology regression now completes in 0.05 seconds and matches final commit bytes; its initial checkpoint loop has an explicit finite progress ceiling.

Production scheduling now delegates a wire page birth its original work unit, and each other wire/publication rung consumes its own unit. No extra fuel is minted. Native hourly/meter readers borrow their original live owners and follow the final Results move without cloning.

Run the retained ticket script `🧪️engine-suite.py`. This result is not a full editor/application or browser verification.
