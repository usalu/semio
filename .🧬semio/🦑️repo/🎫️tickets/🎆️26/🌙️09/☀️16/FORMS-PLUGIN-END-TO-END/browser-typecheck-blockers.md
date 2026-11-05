# Text Editor Closure Typecheck Blockers

The last strict closure check exited 2. These are the exact diagnostics; no changed TextEditor-path diagnostic appeared. This check predates the final readonly settings union and is not a passing full typecheck.

```text
🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts(96,3): error TS2353: Object literal may only specify known properties, and 'rowControlFocus' does not exist in type 'Pick<{ readonly nav: { readonly back: UiLabelValue; readonly forward: UiLabelValue; readonly up: UiLabelValue; }; readonly search: { readonly toggle: UiLabelValue; ... 5 more ...; readonly category: { ...; }; }; ... 42 more ...; readonly nullableInput: { ...; }; }, "search" | ... 8 more ... | "panelToggle">'.
🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts(389,3): error TS2353: Object literal may only specify known properties, and 'rowControlFocus' does not exist in type 'Pick<{ readonly nav: { readonly back: UiLabelValue; readonly forward: UiLabelValue; readonly up: UiLabelValue; }; readonly search: { readonly toggle: UiLabelValue; ... 5 more ...; readonly category: { ...; }; }; ... 42 more ...; readonly nullableInput: { ...; }; }, "search" | ... 8 more ... | "panelToggle">'.
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts(332,9): error TS2322: Type '{ type: "container"; role: "group" | "form" | "section" | "toolbar" | "sheet" | "plain" | "field"; label: string | null; description: string | null; required: boolean | null; error: string | null; defaultOpen: boolean | null; dropOverlay: DropOverlaySpec | null; }' is not assignable to type 'RetainedUiComponent'.
  Type '{ type: "container"; role: "group" | "form" | "section" | "toolbar" | "sheet" | "plain" | "field"; label: string | null; description: string | null; required: boolean | null; error: string | null; defaultOpen: boolean | null; dropOverlay: DropOverlaySpec | null; }' is not assignable to type '{ type: "container"; } & ContainerProps'.
    Type '{ type: "container"; role: "group" | "form" | "section" | "toolbar" | "sheet" | "plain" | "field"; label: string | null; description: string | null; required: boolean | null; error: string | null; defaultOpen: boolean | null; dropOverlay: DropOverlaySpec | null; }' is not assignable to type 'ContainerProps'.
      Types of property 'role' are incompatible.
        Type '"group" | "form" | "section" | "toolbar" | "sheet" | "plain" | "field"' is not assignable to type 'ContainerRole'.
          Type '"sheet"' is not assignable to type 'ContainerRole'.
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️.ts(96,96): error TS2322: Type '"unnamedControl"' is not assignable to type '"cycle" | "orphanChild" | "duplicateSiblingKey" | "nodeQuota" | "depthQuota" | "danglingRoot" | "sectionNested" | "nonFiniteNumber" | "invalidTreeInlineToolbar" | "invalidTreeSectionHeaderToolbar" | "invalidTreeDetail" | "invalidRowTarget" | "invalidSnaps" | "invalidNumberRange"'.
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️.ts(155,92): error TS2322: Type '"unnamedControl"' is not assignable to type '"cycle" | "orphanChild" | "duplicateSiblingKey" | "nodeQuota" | "depthQuota" | "danglingRoot" | "sectionNested" | "nonFiniteNumber" | "invalidTreeInlineToolbar" | "invalidTreeSectionHeaderToolbar" | "invalidTreeDetail" | "invalidRowTarget" | "invalidSnaps" | "invalidNumberRange"'.
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx(523,74): error TS2367: This comparison appears to be unintentional because the types '"standard"' and '"touch"' have no overlap.
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx(524,102): error TS2367: This comparison appears to be unintentional because the types '"standard"' and '"touch"' have no overlap.
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx(525,78): error TS2367: This comparison appears to be unintentional because the types '"standard"' and '"touch"' have no overlap.
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx(528,76): error TS2367: This comparison appears to be unintentional because the types '"standard"' and '"touch"' have no overlap.
```

## Resolution Ownership

Both shell translation objects already include rowControlFocus and windowFocus, and UiTranslationSchema declares them. Their satisfies Pick key lists omitted those existing properties; the lists now include them. The remaining seven diagnostics come from a stale generated framework-manifest UI contract: owned Rust schema projection rows already include ContainerRole sheet, TreePresentation touch and unnamedControl. They require `repo run @semio-tech/ui-contract-rs:generate`; generated output must not be hand patched. A fresh full closure typecheck is required afterward.
