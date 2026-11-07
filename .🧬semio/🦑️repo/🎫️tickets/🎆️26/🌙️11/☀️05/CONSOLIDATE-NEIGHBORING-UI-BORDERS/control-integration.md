# Control Border Integration

Input and textarea register their containing frames so native void controls can retain their own fallback border while the compositor paints shared edges. The stepper registers its frame and three adjacent cells; its middle cell includes the value and optional unit, so the two internal seams have a single owner. Ribbon zones, groups, toolbars and the navbar control row no longer insert spacing between neighboring controls. Native Checkbox has no custom border to register; Button already uses ButtonGroup's registered frame. The Tabs root gap separates the tab list from its content, not neighboring tab controls.

The language-neutral `control-border.cases.json` specifies frame edges and cell count. The component test validates its schema with Ajv and checks rendered registration and stepper cell adjacency.
