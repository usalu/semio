# Accessible Node Disclosure

Draw's inspector node disclosure buttons appeared as unnamed buttons in the browser accessibility tree. The shared TreeItem rendered its label and chevron button without an accessible association or expanded state.

A language-neutral fixture covers default/property rows and English/German labels. Testing Library's independent accessible-name computation reproduced the failure in all four cases. The test also activates the disclosure with Enter and Space and verifies the row and button report the same expanded state.

The shared renderer now gives the visible label a React-host-generated unique id, references it with aria-labelledby from the disclosure button, and publishes aria-expanded on both row and button. This supports rich label nodes without converting them to an object string and preserves localized text. The button explicitly has type=button in both layouts.

The initial component validation was followed by verification in the actual Draw shell.

All 40 tests in the Tree component file passed after the change. This includes the four neutral disclosure cases, keyboard Enter/Space toggling, controlled branch behavior, selection and virtualized row checks already owned by that test file. The initial red run failed specifically because Testing Library could not find a named disclosure button in each of the four cases.

## Focused Keyboard Ownership

The browser exposed the new names and expanded states correctly, but Enter on the focused Anchor 2 button did not expand it. ShellHost admitted app-wide bindings from buttons and consumed Enter as Draw's finish-draft binding. The component-only test did not include that listener.

Both app-action and command shortcut handlers now use one shared focus guard. Native and ARIA widgets retain ordinary keys, including Shift navigation; modified chords remain available outside text editors. Inputs, selects, textareas, contenteditable and textbox/searchbox controls retain their editing shortcuts, and composition stays local. This removes the duplicate app-only editable-target predicate.

Sixteen language-neutral ownership cases cover button activation, modified undo/redo, Shift-Tab, links, tree/list/slider controls, plaintext contenteditable, and canvas/background shortcuts. Testing Library user-event independently exercises Enter/Space activation, modified shortcut routing, Tab focus, and text entry. The red run failed on the missing guard; the completed regression run passed all 21 tests in the OS command shortcut suite (13.9 seconds through Nx).

In the running Draw browser, Enter changed both Anchor 2's row and named button to expanded and exposed Open Contour, Close Contour, Delete Node, Split Segment and Convert to Curve. Space collapsed them; a second Enter reopened them. No console errors were reported. Screenshot: `🗑️generated/keyboard-disclosure-verified.png`.

The follow-up inspection found action leaf labels still rendered as passive spans inside click-only rows. Those actions need native keyboard activation too; separate English/German fixtures cover both default and property layouts.

Activatable leaf labels now render as type=button, retaining the row's existing click dispatch. Passive property labels remain spans and do not add tab stops. The four new activation cases initially failed to find the buttons; their Enter and Space checks now pass. The full shared Tree file passed all 48 tests (9.9 seconds through Nx), including three concurrently added disclosure schema/leaf tests. An initial passive-row assertion accidentally included an expandable child; the fixture now explicitly declares a non-expandable property, matching real inspector property rows.

The running browser now reports named buttons for Arrange and node operations. Enter on Convert to Curve produced the two expected Bézier handles and replaced the action with Straighten Segment. Command-Z while Straighten Segment was focused restored the original line and Convert to Curve in one step. No console errors were captured. Screenshot: `🗑️generated/keyboard-actions-verified.png`.

Replacing the conversion action removes the focused DOM node, returning focus to the document. This is a remaining workflow gap, distinct from activation; it must be handled before claiming complete keyboard editing.

## Continuation State

This goal turn made progress through the shared keyboard admission fix, native action-label buttons, passing regression suites and actual browser conversion/undo proof. The ticket and full editor goal remain open. Native node-row test 73342 and describe/materialize 79298 remain live; stable preview 76268 remains on port 6065. Poll those existing jobs instead of restarting. The current component was compiled before the node/gradient row changes, so it still needs a subsequent rebuild and explicit activation even when materialization finishes. The gradient-row test has not run yet. Browser tab 4 is retained with Orange Wedge restored and Anchor 2 expanded.
