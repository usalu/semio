# Right Chip Implementation

The teaching presentation keeps the existing silhouette layout and moves its single localized view chip from the bottom left to the bottom right. The chip contains the existing maximize-2 icon beside View / Ansehen. The whole overview card remains its single native button, and the chip remains a noninteractive label inside it.

The expanded video window uses one cap-right native button containing Restore / Wiederherstellen and the existing minimize-2 icon. Its class comes from the shared overviewCardChipClass so the label and icon occupy one auto-width silhouette chip. Pointer pressed feedback and restore behavior remain connected to that same button.

The shared OverviewCardOpenChip accepts an optional React node icon. Callers that do not supply it retain the existing chevron and hover transition. Other product callers are unchanged. The teaching icon wrapper exposes its icon name through data-presentation-icon and hides decorative icons from assistive technology.

Files changed:

- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx
- 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx

Validation: the test agent established the expected red failure against the unchanged production UI in both languages: the chip began at x=6.390625 and was required to appear past x=266.6640625. Production mutation followed that red result. Focused browser verification is owned by the right-chip test agent; its final result is recorded separately in the ticket.

Final validation: all eight focused invocations passed after RED, the seven-second encoded preview passed independent geometry/native-event/decode/boundary verification, and the final visual audit passed.
