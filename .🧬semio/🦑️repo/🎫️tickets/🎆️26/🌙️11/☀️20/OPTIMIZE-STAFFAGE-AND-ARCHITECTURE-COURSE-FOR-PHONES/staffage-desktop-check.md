# Staffage Desktop Gallery Check

At 1280 × 800, the actual Staffage app opened the English archive in Chromium with its desktop layered overview. The root strip rendered 13 cards. Pressing Enter on the Humans link opened the Humans strip at `#/humans`. The gallery rendered all 28 Human cards from the current archive manifest. The desktop layout used the layered grid, fit the viewport horizontally and reported zero uncaught page errors.

The ticket’s `staffage-desktop-browser.mjs` drives the live app through the in-process Vite adapter. The count follows the authored manifest, which can change as assets are added.
