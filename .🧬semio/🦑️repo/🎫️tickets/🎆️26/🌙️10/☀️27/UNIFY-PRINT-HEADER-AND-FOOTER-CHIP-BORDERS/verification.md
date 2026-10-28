# Print Chrome Border Verification

The header and footer now use three-sided metadata chips joined to one continuous shared border, matching the chapter construction. Header chips open downward; footer chips open upward. Removed the spacing that detached boxes from their rule. Chips retain the existing fonts, paints, configurable icon sizes, metadata zones and mirrored placement. Long running titles resize inside the chip so their borders retain their height.

The chip body is normalized to zero depth before placement. This ensures its side strokes meet the shared rule even though the background box adds depth for padding.

## Verification

- The language-neutral JSON and Gherkin cases cover both border orientations and long labels.
- The focused checks failed before implementation at the closed-box assertion, then passed after implementation.
- Independent mathjs evaluation agrees with the declared body, open-chip and complete-bar geometry.
- Regression checks preserve the author, logo, project, center-label and label-computation bindings.
- The focused verification is integrated into the existing quick print pipeline.
- Read-only diff review confirms the change is confined to page chrome and the quick-pipeline test registration.
- `repo run @semio-tech/print:test --lang=en --exclude-dependencies -- quick` failed before the suite could execute: repository source admission cannot stat `C:/git/semio/.cache/b/l/fixed.dir`.
- The authored-script entry point also failed before tests ran: `@semio-tech/framework-transient` cannot be resolved from the UI theme module.
- Consequently, full print tests and rendered-PDF visual verification remain unverified. These are repository setup failures outside this styling change.

## Reproduction

Run the retained `verify.ts` through `repo tools exec bun -- <ticket-folder>/verify.ts` for the focused source and geometry checks. Run the normal print quick suite once repository setup is operational.

No AGENTS files or git state were modified. No generated ticket outputs remain.

## Ticket Closure

MCP ticket_close could not complete: its default path failed while collecting git diff status outside a repository. Explicit dated and returned identity paths were rejected as invalid. The follow-up successfully closed the ticket with management integration disabled.
