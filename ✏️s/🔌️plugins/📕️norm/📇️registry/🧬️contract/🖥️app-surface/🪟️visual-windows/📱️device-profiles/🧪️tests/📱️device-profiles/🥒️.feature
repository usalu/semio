@capability-device-profiles
@oracle-framework-window-layout
Feature: Device profiles adapt the visual windows to desktop, mobile and tablet
  Profiles are cross-checked against the framework window layout produced by `visual_layout`.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Priority and budgets: Given the three device classes When their profiles are read Then desktop has the largest budgets, then tablet, then mobile, and mobile debounces live evaluation the longest
  - Touch targets and handle input: Given a touch device When a handle is hit-tested Then its hit area is at least 44 px, hover is off, and mobile moves handles by tap-to-nudge
  - Layout agreement: Given the framework layout of each device class Then mobile is one tabbed stack, tablet two panes and desktop five windows
  - Classification: Given a width and touch capability When the device class is derived Then widths below 600 are mobile, touch below 1024 is tablet and everything else desktop
  - Layouts are user-customisable local-only preferences: Given the default layout preferences When the user resizes the dock, hides the checks and diagrams windows, reorders the tab bars and pulls up the sheet Then the desktop dock, the mobile tab bar and the tablet side pane follow, the viewport is never hidden, shares are renormalised, any stored value is sanitised and the preference round-trips through the value codec as persisted local-only
  - Mobile is one pane with a tab bar and a bottom sheet: Given the mobile device class When the layout is resolved Then the model, properties, checks and diagrams are tabs of one stack and the properties sheet peeks, half-opens or fills the screen over the model tab while something is selected
  - Tablet is two panes: Given the tablet device class When the layout is resolved Then the viewport and a tabbed side pane share the width
  - Touch gestures go through the interaction domain: Given the interaction definition generated for an artifact When tap, long-press, long-press drag, double tap, drag, pinch and nudge buttons are mapped Then selection intents use only declared methods and merge modes, one finger orbits, two pan, a pinch zooms within limits, handles drag freely on tablets and nudge by buttons on mobile
