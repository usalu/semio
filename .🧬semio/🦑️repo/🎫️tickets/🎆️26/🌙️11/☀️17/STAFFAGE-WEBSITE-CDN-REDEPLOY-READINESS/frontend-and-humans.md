# Staffage keyboard access and human asset validation

Desktop Display settings now moves keyboard focus to the selected text size. Tab reaches the selected motion setting, and Escape closes the panel and restores the trigger. On phones, focus starts at the first archive navigation link in the same panel.

Section and floor plan placements now use native buttons. Keyboard focus shows an outline and preserves the current selection until Enter or Space. Clicking selects the placement too. A cancelled pointer gesture restores the committed position without recording an edit.

The new regression cases reproduced the desktop focus, focus-driven playground selection and cancelled drag defects before the fixes. All 42 focused React cases passed afterwards, including the settings focus order, drag, viewpoint and phone layout cases. React TypeScript passed. The full React run before the three catalog fixes passed 298 of 299 cases. The sole failure was the whole-catalog readiness check, which saw 121 ready records and 31 invalid records in that snapshot. The tests ran through the workspace-pinned Bun because repository admission was waiting on host capacity.

The 1:500 Elderly with Cane drawing retains the cane’s 76 mm local shaft thickness and uses a rearward offset, giving a measured forward extent of 0.506 m against the declared 0.500 m. The left hand structural loft now uses 75 mm sections to meet the 1:500 feature limit.

The 1:200 and 1:500 thermal head or neck lofts for Person with Umbrella and Tour Guide each had coincident authored sections. Redundant sections were removed. Person with Umbrella’s 1:500 thermal neck spans 75 mm between the adjoining areas. Tour Guide’s 1:500 wrist begins 10 mm farther under the sleeve and spans 40 mm. The authored asset validator now accepts Elderly with Cane and Person with Umbrella. Tour Guide’s only remaining finding from the targeted run was the independently tracked 1:700–1:1000 drawing band gap.

The prosthesis socket in Runner with Blade Prosthesis declared both a single thermal region and a list of thermal regions in its 1:500 elevation, front and plan drawings. The three singular declarations duplicated the first item in the corresponding list and were removed. The ordered socket and suspension liner list is intact in every view. The production asset validator reproduced three `region-ambiguous` findings before the edit and returned zero findings afterwards. A structural scan of all 152 catalog asset documents found no simultaneous `region` and `regions` declarations after the edit.
