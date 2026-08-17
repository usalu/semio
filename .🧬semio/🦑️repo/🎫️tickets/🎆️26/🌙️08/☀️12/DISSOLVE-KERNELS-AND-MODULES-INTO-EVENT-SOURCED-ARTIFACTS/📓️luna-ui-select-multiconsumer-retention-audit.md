# UI Select Multiconsumer Retention Audit

## Snapshot

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Definition: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🟦️component.tsx`, SHA-256 `6d62c59c9ecae8c94345a2a063ed9edb8c73aff908b89bc80b2a20f9214aef39`, clean.
- Story: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🧪️story.tsx`, SHA-256 `7ccd2fabdd4a0d9b8661649a033429951a32ff02464e16be49c5195870cf5a73`, clean and excluded from production consumer count.
- Shared React barrel after the concurrent Avatar registrar: SHA-256 `fdd7e8ec24ea5288b386bab04f2627d81194712e2461860e8e2abcead71a4a23`.

## Independent Active Production Consumers

1. UI `IconSelector`.
2. UI `Canvas`.
3. UI `Navbar`.
4. UI `Tree`.
5. OS renderer `ChromePanels`.
6. OS renderer `Interpreter`.
7. OS renderer `ShellHelpers`.
8. CAD editor renderer.
9. The UI React `EngagementControlView` implementation.

The OS renderer and CAD consumers are protected, active production surfaces. Story and generated/schema occurrences do not increase the count.

## Ownership and Disposition

The definition owns one coherent compound Select interaction and keeps the Radix adapter behind repository-owned components. `@radix-ui/react-select` remains a valid private dependency. Retain at the framework UI Select owner: the reverse closure substantially exceeds two independent production components, and neither inline, delete, nor responsibility split is justified.
