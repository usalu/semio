# Generation form page admission

The shared form renderer now plans contiguous pages using real mounted record costs. Each scalar or read-only field costs its container and control. A coordinate group costs its group plus each axis field and input. A step heading is charged with its first visible field on each page. Conditional visibility is resolved before seating the fields. The renderer constructs only the requested page and preserves the original question keys and typed generation bindings.

A form that fits inside the complete 128-record body still renders as one page. Multi-page rendering reserves room for navigation and repeats authored step headings where needed. A page request beyond the visible form clamps to its last page. A field whose entire control cannot fit produces an explicit admission error.

## Verification

- The pinned Rust witness ran the production page planner with nine language-neutral cases, including 70 questions, mixed vector and scalar costs, repeated headings, an empty form, a stale page and an oversized field.
- AJV validated the fixture and an independent Lodash-backed minimum contiguous partition agreed with all nine results.
- The registered Generation3d composition test projects 70 original Flow slider widgets into both semantic pages. It checks original question addresses, bindings and mounted record counts. The updated shared form and both app routes passed Rust source parsing. The linked composition test remains unexecuted.
- Host-local page selection and accessible navigation have since been connected in both form windows. The [navigation report](📓️generation-form-navigation.md) records the mounted Chromium and scoped native verification.
- The workspace had 0.358 GiB free at the verification check. The full native link remains below its 5.5 GiB admission reserve.
