# Checkbox Paint Parity

The sealed paired runtime21 Window Options view showed WGPU drawing a wide accent rectangle around checked controls while React draws a compact checkbox. In native retained paint, the checkbox's checked state is carried in `presence.selected`; after painting the compact control, the generic presence phase treats that same state as an outer selection outline across its entire allocation.

The shared retained-toggle fixture now includes a 104×24 allocation, a 9.6-pixel control side, both checked states and zero painted selection-outline width. The actual styled React Interpreter is mounted through React Testing Library, its real Tailwind CSS is compiled, and Chromium measures each checked state. This oracle passed 3/3 (`checkbox-react19.log`, Nx exit 0, 8.5 seconds). Chromium reports `outline-width: 3px` with `outline-style: none`, so the oracle measures painted width rather than asserting the unused CSS width is zero. The temporary diagnostic log was removed.

The native production-frame law ran RED before the repair (`checkbox-paint-red2.log`, Nx exit 1 in 4m50s): the unchecked case passed, and the checked case failed because its paint outlined the 104-pixel allocation. The first invocation had rejected an unsupported `--nocapture` position before any assertions and supplies no regression evidence.

The repair now suppresses only the checkbox's generic selected outline. The compact checkbox continues painting checkedness; other presence channels and ButtonToggle selected behavior retain their existing paths. Both the production retained cursor and the immediate test dispatcher use the same selection predicate, without cloning presence or its peer collection. The focused native GREEN rerun is active (`checkbox-paint-green1.log`).

The first GREEN invocation stopped before assertions after 11m02s because the concurrent Tree selection stamp passed a section vector to an item-slice helper (E0308). Its current source now iterates `section.items`. Full UI11 has started on that coherent source and includes the checkbox law, so a separate duplicate focused run is unnecessary. Native success and a rebuilt Window Options browser result remain pending.
# Integrated Native Receipt

Full UI11 passed716/716 with zero skipped, Nextest10.145s, Nx20m27s, exit0. The checked-checkbox paint law is included. This supersedes the pre-assertion GREEN1 dependency failure; a fresh physical Options checkbox check remains due.
