# Summary

Sketchpad app modules, state machine wiring, and shared app surfaces for Home, Kit, Design, Type, Quality, Docs, and Feedback.

# Docs

## elements.tsx

`Table` supports row-level hover callbacks for app hover state dispatch.

## Home.tsx

Home app hover state is stored in the Sketchpad state machine and updated via hover commands for table rows.

## Kit.tsx

Kit app hover state covers all artifact kinds and is updated via table and diagram hover dispatch.

## Sketchpad.tsx

Home command hooks forward hover events, including clear, into the Sketchpad state machine.

# Specs

## State Management

App hover and selection state MUST be managed by the Sketchpad state machine.

## Toolbar

The toolbar is a floating panel positioned at the bottom center of the canvas. Each app registers toolbar sections.

- **Home app**: Filter toggles for kit kinds (temporary, local, remote) with action buttons to create new kits
- **Kit app**: Filter toggles for artifact kinds (designs, types, qualities, ports, tags, concepts, files, folders, authors) with action buttons to create new artifacts
- **Design app**: Selection tools (normal, additive, subtractive) and lasso tools (rectangular, freeform)
- **Type app**: Selection tools and connector creation tool
- **Feedback app**: Send button to submit feedback form

Toolbar panel visibility defaults to true for all apps in default state creation.

## Interaction State

Hover and selection feedback across Home, Kit, Design, Type, Quality, Docs, and Feedback is driven by the app state machine.

Hover and selection highlights MUST be consistent across tables, lists, and diagrams.

## Borders

- Element border kind (hover color)
- Window border kind (normal border color)
- Window spacing: 1-unit gap between windows and 1-unit margin to canvas edge
- Base canvas uses the base background surface; windows, panels, and temporary UI surfaces use their respective background levels
- Exactly one window is active in a multi-window layout; the active window surface uses an active background tint
- Table views use the active window surface background
- Global Sketchpad shell is wrapped in base level so Navbar/Footer resolve base background
- Panels are rendered under panel level so panel surfaces resolve panel background
- Window chrome controls MUST be rendered as Action UI elements
- Window frames use inset overlay strokes so all four edges remain visible with clipped layouts

## Windows

Sketchpad apps MUST render inside a multi-window workspace.

Each app MUST define a set of window kinds and a default window layout.

Window layouts MUST be persisted per app as JSON strings.

The active window MUST be tracked for focus-sensitive UI.

Window chrome MUST expose action controls for open-in-new-window, maximize/minimize, and close.

## [👤semio📚js🗃️sketchpad💻kittsx](semiorepo://section/semio/js/sketchpad/Kit.tsx)

Selection:
    - Designs
    - Types
    - Folders
    - Files
    - Ports

Filters:
    - Designs
    - Types
    - Folders
    - Files
    - Ports


## [👤semio📚js🗃️sketchpad💻kittsx🔖table](semiorepo://section/semio/js/sketchpad/Kit.tsx/Internal%20State%20Management/Canvas)

- {{design-row}}
- {{type-row}}
- {{port-row}}
- {{folder-row*}} # folder
    - {{file-row}} # files inside the folder

Currently I have

- file1
- file2

But it should be:

- folder
    - file1
    - file2

## [👤semio📚js🗃️sketchpad💻designtsx🔖panels](semiorepo://section/semio/js/sketchpad/Design.tsx/Panels/Details)

Home screen:


Once Piece, Once Connection selected:
- {{piece-details}}
- {{connection-details}}
- {{kit-details}}

Once Piece selected:
- {{piece-details}}
- {{kit-details}}


Piece Details Section:
```yaml
Piece: # section,
  Type: "{{piece-type-select}}" # input tree item, only show types that can replaced the type (e.g. all used connectors must exist)
  Id: "{{piece-id-input}}" # input tree item
  Description: "{{piece-description-text-area}}" # input tree item
  Attributes:
    - name: "{{attribute-name-input}}" # input tree item
      value: "{{attribute-value-input}}" # input tree item
  Plane: # collection tree item, only show section when
    Origin: # collection tree item
        X: "{{origin-x-stepper}}" # input tree item
        Y: "{{origin-y-stepper}}" # input tree item
        Z: "{{origin-z-stepper}}" # input tree item
    X-Axis:
      X: "{{x-axis-x-stepper}}"
      Y: "{{x-axis-y-stepper}}"
      Z: "{{x-axis-z-stepper}}"
    Y-Axis:
        X: "{{y-axis-x-stepper}}"
        Y: "{{y-axis-y-stepper}}"
        Z: "{{y-axis-z-stepper}}"
Parent Connection:
  Translation:
    Gap: "{{gap-slider}}"
    Shift: "{{shift-slider}}"
    Rise: "{{rise-slider}}"
  Orientation:
    Rotation: "{{rotation-slider}}"
    Inversion: "{{inversion-slider}}"
```

Kit Details Section
```yaml
Kit: # section,
  Name: "{{kit-name}}"
```