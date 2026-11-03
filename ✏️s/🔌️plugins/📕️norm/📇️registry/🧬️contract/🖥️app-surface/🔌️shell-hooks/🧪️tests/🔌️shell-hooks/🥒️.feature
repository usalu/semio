@capability-shell-hooks
@oracle-accesskit-consumer-tree
Feature: The shell asks the norm plugin for its layout, its keys and its names

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - The layout follows the device and the user's stored preferences: Given the shell opens an editor of "<variant>" in a window <width> pixels wide with touch "<touch>" When the shell asks the plugin on start Then the plugin answers the named layout "<layout>" Examples (3 rows: en1993-1440-false-norm-visual-desktop, en1993-900-true-norm-visual-tablet, en1993-390-true-norm-visual-mobile)
  - A phone shows the properties in a bottom sheet while something is selected: Given the shell opens an editor of "en1993" in a window 390 pixels wide with touch "true" And something is selected When the shell asks the plugin on start Then the answer carries a bottom sheet labelled "Properties" with a handle of at least 44 pixels And the properties inspector is wrapped in a container of role "sheet" announced as "complementary"
  - The window is resized and the input switches to touch: Given the shell has asked the plugin on start at 1440 pixels When the window becomes 390 pixels wide Then the plugin is asked again because of a resize When the last input becomes touch Then the plugin is asked again because of the input modality And nothing is asked while neither changes
  - The plugin sees the key before the shell defaults: Given the active window is "<window>" When the key "<key>" is pressed with ctrl "<ctrl>" Then the plugin answers "<answer>" Examples (4 rows: norm-viewport-arrowleft-false-norm-shortcut-orbit-left, norm-handle-arrowleft-false-norm-handle-arrow-left-normal, norm-global-z-true-norm-shortcut-undo, norm-viewport-q-false-ignored)
  - The viewport label and the screen reader read the same name: Given a scene with a member "B1" at 64 % utilisation that passes When the scene is lowered in English Then the viewport instance of "B1" is labelled "Entity B1, utilisation 64 %, passes" And the scene list has an option with the same name at position 1 And the platform accessibility tree receives that option with its role and name
  - The platform preferences reach the plugin once per change: Given the shell has a plugin with hooks When the host reports forced colors "true", more contrast "false", dark "true" and reduced motion "true" Then the plugin hears the four answers And reporting the same four again tells it nothing And the viewport paint draws every grip in the strongest tone with a cross at the focused one
  - The keyboard walks the grips of the selection and cycles inside one entity: Given the viewport shows grips "a.length", "a.height" of entity "a" and "b.length" of entity "b" When Enter is pressed Then the grip "a.length" has the keyboard And Tab moves to "a.height" And Control with Arrow Right wraps to "a.length" inside entity "a" without reaching "b" And a plain Arrow Right nudges the focused grip And Tab past the last grip hands the key back to the shell And Escape cancels the open change and returns the keyboard to the viewport

  Scenario: A phone edits handles with nudge buttons and a finger held on the model opens the Edit menu
    Given the viewport of an editor on a phone with a grip selected
    When the shell asks the touch profile at 390 pixels
    Then one finger orbits and the hit edge is 44 pixels
    When a finger is held on the model and lifted in place
    Then the Edit menu opens with Copy, Paste, Duplicate, Delete and Select all in English and German
    And every grip of the selection has a minus and a plus button that commit one schema step in one set-fields edit
    When the shell asks the touch profile at 900 pixels
    Then one finger drags handles and the nudge buttons are still offered

  Scenario: The outliner is a tab of the phone and tablet layouts
    Given the layout of an editor on a phone and on a tablet
    Then the outliner, model, properties, checks and diagrams windows are all tabs
    And a selection in the list shows the properties in the bottom sheet of the phone
