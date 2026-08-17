# UI Class Names Umbrella Split Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️ClassNames/🟦️component.tsx`
- Source SHA-256: `417bbf652b46de8dbb5ba63c559bffd1e0e6f143b3c34110a4b636a18c612bfe`
- Source state: clean, 269 lines
- React barrel SHA-256 at coordinator recheck: `0f8def42b5703b2ab00bd31f6e7b242e334ea9f60fdd9a5d35c1a88fdf8fa401`

## Finding

`🏷️ClassNames` is not one maximally specific semantic component. It combines class-name composition, interaction presentation, form-control presentation, surface fills, borders, menu-item presentation, shell-floor fill ownership, status borders, chrome-control presentation, Slider-private presentation, and Table-private presentation.

The direct active consumer closure proves these dispositions:

- Class-name composition: dozens of independent UI consumers; retain as a shared module.
- Form-control presentation: independent consumers include Input, Textarea, Select, Command, Stepper, and ActionGroup; retain as a shared module.
- Surface presentation: independent consumers include Surface, Canvas, Layout, Dialog, UIDialog, Panel, Footer, Navbar, Tree, and renderer hosts; retain as a shared module.
- Interaction presentation: independent consumers include Tabs, Collapsible, ActionGroup, Canvas, PanelTabBar, Tree, and renderer EventFeedHost; retain as a shared module.
- Status-border presentation: independent consumers include Slider, Scene, Diagram, Skeletons, renderer Interpreter, and ShellHost; retain as a shared module.
- Chrome-control presentation: independent consumers include ActionGroup and Canvas, with other direct chrome consumers; retain as a shared module.
- Menu-item presentation: Command, Select, and ActionGroup; retain as a shared module.
- Border presentation: Command, Select, Input, Textarea, ButtonGroup, Table, Panel, Stepper, and renderer consumers; retain as a shared module.
- Shell-floor presentation: Canvas, Footer, and Navbar; retain as a shared module.
- Table row presentation: one production component, Table; inline into Table.
- Slider presentation: one production component, Slider. Its apparent second use was only the public barrel; inline into Slider.

The React barrel and package indexes are glue and do not count as production consumers. Tests, stories, and generated surfaces are excluded.

## Required Semantic Ownership

Delete the `ClassNames` element identity. Create specific shared modules at the UI owner for class-name composition, form-control presentation, surface presentation, interaction presentation, status-border presentation, chrome-control presentation, menu-item presentation, border presentation, and shell-floor presentation. Inline the Table and Slider leaf presentation into their only components. Rewire every direct import and assembly export without a forwarding compatibility module.

The class-name composition public signature must use a repository-owned class-value contract rather than exposing `clsx`'s external `ClassValue` type.
