# Component Runtime Grant R37 RED

R36 failed before native execution due to a malformed newly authored comparison schema; corrected without production behavior changes. R37 is the actual semantic RED: 0 passed, 1 failed, 134 skipped, 0.034 seconds. Exact retained sources were fully closed before the assertion; no Drop guard was weakened. The new test uses neutral fixture `runtimeWorkGrant=4096` and physical allocation grant32768.

```text
15:[DEBUG] fixed-list-page-oracle checks=35
23:[DEBUG] component-copy-real-grant inline=3096 work-max=3096 complete=false
25:thread 'action::component_copy_tests::retained_component_copy_surface_advances_under_real_4096_work_grant' (5293739) panicked at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🧪️component.rs:88:5:
26:valid Surface must progress under the actual runtime 4096-byte work grant
46:     Summary [   0.034s] 1 test run: 0 passed, 1 failed, 134 skipped

```
