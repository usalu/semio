# Active Chrome

The archive chrome was using an underline for the current page and an inverted foreground fill for a selected segment, a pressed button, and a selected pose or clip card.

The design system uses the primary color for the current page and `bg-active-base` with `text-active-foreground` for a pressed or selected control. `bg-active-base` is the primary color. Emphasized fill stays for hover, not for the pressed state.

- Current navigation, including the home link and the links inside the settings menu, uses `text-primary` and `aria-current="page"`.
- A button with `aria-pressed` and a selected segment use the active background.
- A primary action uses `bg-primary`.
- A selected pose or clip card uses the active background.
- The category load action uses the primary color.
