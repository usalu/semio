# Matrix Numbering Fixture Order

Actual matrix 83411 compiled its first complete case and then failed the expected THESIS-MAIN-DEPTH=3 transcript assertion. The log was read but not retained before assertion/owned probe cleanup; its actual failed value is unknown and must not be reported as observed.

Authoritative source establishes a fixture ordering problem: the fixture configured secnumdepth=3 before begin{document}; existing semio-window.sty sets its all-level Print startup depth in both AfterEndPreamble and AtBeginDocument. The thesis title suspends the active depth after document startup, and makemainmatter restores that saved value. This is not evidence that thesis save/restore lost a caller setting active at suspension.

Moved the fixture selection immediately after begin{document}, before title/frontmatter. Its existing neutral expected numbering vectors now test the intended caller-active depth across front/main/back matter transitions. No production LaTeX change, default change or weakened assertion was made. The actual matrix remains the behavioral gate. The canonical live build remains valid and running.

Sibling thesis_contract owns retaining bounded log/register transcript facts through held probe.read before assertions, with actual marker text in failure output. No compiler duplicate launched for diagnosis.
