
> nx run @semio-tech/framework-plugin:test --args=exhaustive mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] plugin-runner-oracle cases=6
────────────
[32;1m Nextest run[0m ID [1mf611bb88-c6fb-4cc8-bd8b-d0d2e8eba485[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m1[0m test across [1m1[0m binary ([1m521[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner[0m

running 1 test
[DEBUG] mounted-resident-capacity fixed=534368 per=8388608 accepted=3 full=25700192 cap-plus-one=false exact-refusal=true restored=534368
test component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner[0m
────────────
[32;1m     Summary[0m [   0.018s] [1m1[0m test run: [1m1[0m [32;1mpassed[0m, [1m521[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-eOc74O[0m



 NX   Successfully ran target test for project @semio-tech/framework-plugin


