# Architecture Assignment Print Integration

## Intended Boundary

Keep assignment.cls, semio-assignment.sty, generic assignment schema/preamble inference, native Diagram/Transitions kinds, shared font/material preparation, and source staging in the Print product. Move all parametrization course mains, localized content, geometry, icon manifests, domain projection/runtime code, and domain tests to the architecture Parametric topic's assignment leaf. Remove both parametrization entries from the generic Print template catalog.

The existing publishPrintArtifact document intent already supports a workspace-relative sourceRoot and an explicit complete source list. The course-owned producer should call that API directly using the relocated authored source rather than introducing an adapter or registering course content as a Print template. Course dev/build/deploy should depend on that producer and consume its retained publication through the existing authenticated in-memory assignment PDF transport.

## Inspected Constraints

The Print document intent and artifact layout currently restrict namespace to print/report/probe. A course namespace must be added consistently in schema/type/layout validation. The generic publisher currently derives receipt ownership as @semio-tech/{namespace}:build-{id}; this does not match the actual @teaching/architecture-course project. Root coordination will settle a correct explicit generic publication identity before mutation.

Source staging already maps semantic Unicode paths to portable compiler names, rewrites captured TeX references, retains source hashes, and rejects changed input. It should remain the single source projection mechanism. Publication already prepares pinned Tectonic, bundled TeX and fonts, compiles under retained control, validates PDF signatures, and writes mode-specific artifact slots. No hand-run compiler fallback or checked-in generated output is appropriate.

The architecture course currently names parametrization-en/de under namespace print and depends on @semio-tech/print:build-parametrization-en/de. Its PDF consumer hardcodes print. Those declarations, dependencies and consumer identities need to change together. The course script has an import before its shebang and will need the interpreter directive restored to the first line when its target router is edited.

## Resource Work

Inspect the existing vector resource conversion interface and the canonical authored SVG sources. Compiler inputs should derive resource PDFs through the repository-owned converter interface and retain only authored sources in version control. Do not move checked-in generated PDFs to the course. Preserve the approved native output and font preparation.

No production mutation has occurred at this investigation stage. A language-neutral boundary case and independent schema validation will precede implementation. Root owns the ticket, canonical path decision and cross-agent ownership.
