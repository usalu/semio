# React Census Reconciliation

The initial804-test census reported17 failures. Source audit confirmed stale stylesheet/tutorial paths, an obsolete fullscreen slot selector, and a nested picker selecting terrain while its own schema permitted only map. Root corrected those test inputs without weakening production UIDialog validation. The next full census ran804:799 passed,5 failed.

The five included a Diagram publication-generation assertion (undefined), a remaining retired dialog-box CSS selector, the Actions toggle test using its old element after the pane remounted, a conditional Panel controls selector with no authored controls, and one WindowChrome geometry test. Root corrected the dialog selector to dialog-content, reacquired the live Actions toggle for its second click, and explicitly asserts that the Panel has no unused controls while retaining its glass/content checks.

The four focused chrome/introduction tests now pass (4 passed,561 outside selection), including the untouched WindowChrome geometry law. The full census must rerun to determine whether that measurement failure was shared-test contamination. Diagram remains an independently observed publication check requiring a current full receipt; no Diagram production source was changed.

Census3 passed803/804; only the unmodified WindowChrome measurement law failed. Diagram passed this run without any source or test relaxation. The measurement contamination is concrete: an earlier utility-bar test assigned HTMLElement.prototype.getBoundingClientRect, then restored its inherited function by assignment. That leaves an own property shadowing all later Element.prototype spies. Its finally now restores the original property descriptor or removes the temporary own property, preserving the prototype chain. Census4 is running against this test-harness repair.

Census4 passed all804 tests across28 files. The geometry test remains unchanged, proving the exact prototype cleanup resolved the shared-suite failure. This gate includes the six mounted ShellSearch tests and updated independent-inset composition. No React product behavior was changed by root in this reconciliation packet.
