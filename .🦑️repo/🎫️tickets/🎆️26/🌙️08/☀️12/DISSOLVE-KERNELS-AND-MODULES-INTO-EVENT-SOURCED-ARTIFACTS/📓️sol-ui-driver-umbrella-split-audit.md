# UI Driver Umbrella Split Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🚗️UiDriver/🟦️component.tsx`
- Source SHA-256: `9baffb7eb658388153924a625bb1f8229b9f7f197f680803e290d5b7d9f78e16`
- Source state: clean, 272 lines

## Finding

`🚗️UiDriver` combines several independent responsibilities and is not a maximally specific UI element:

- driver configuration contract, built-in configurations, validation, canonical encoding, and resolution;
- browser storage of active and custom driver configurations;
- ambient React/non-React active-driver context and provider;
- native drag arming;
- product label-id resolution and control-id caption formatting.

## Consumer Evidence and Disposition

### Driver Configuration

The driver contract and resolution are used independently by protected renderer Shell, ShellHost, ChromePanels, ShellHelpers, and framework UI behavior. Retain as a specific shared UI module. `parseUiDriver` and `serializeUiDriver` have no production terminal outside the current owner/glue and must not remain public solely for tests or package assembly; retain them privately only if storage requires them.

### Driver Storage

Independent consumers include renderer Shell and ShellHost, while ambient context also reads storage. Retain as a specific shared UI module that owns the two storage keys and read/write behavior behind the repository `StoragePort` interface.

### Active Driver Context

Independent consumers include framework Label and protected renderer World3dHost, with additional active use through framework React application behavior. Retain as a specific shared UI module. Keep the provider and ambient resolver with this responsibility. `useUiDriverTooltips` has no production consumer and should be deleted.

### Native Drag Arming

Independent consumers include Table, PanelTabBar, Panel, and renderer BlockListHost. Retain `useNativeDragArm` as a specific shared interaction module. The related `useUiDriverDragSurface` query belongs with active-driver context because it derives directly from that configuration.

### Control Label Resolution

Shared resolution/caption behavior is used by framework Label and renderer ChromePanels. Retain only the semantically shared resolver and caption functions in a specific module. `panelKindFromPanelToggleControlId`, `isInternalChromeControlId`, and `humanizeEngagementStepId` have only the Label component as a production consumer and should move private into Label. `humanizeControlSegment` is an internal dependency of the shared caption function and stays private in that module.

## Required Ownership

Delete the `UiDriver` element identity after moving qualified capabilities to UI-owner modules and one-consumer functions into Label. Rewire direct consumers and the React assembly surface without a forwarding compatibility component. This execution must wait until the active ClassNames split finishes because Table, PanelTabBar, Panel, and Label are in that lease's writable closure.
