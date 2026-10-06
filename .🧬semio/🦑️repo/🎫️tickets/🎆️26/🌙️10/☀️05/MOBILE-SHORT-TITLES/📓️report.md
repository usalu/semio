# 📱 Mobile Short Titles

## Goal

Mobile navbars and swipe hints lack room for long titles. Every page now carries an explicit short title used below the tablet breakpoint (768 px), while the full title stays available to assistive technology.

## Pattern

- **`ResponsiveLabel`** (`@semio-tech/ui-react`): shows `short` below `md`, full title from `md` up; `title` attribute keeps the full name for pointer hover.
- **`LayeredPane.shortLabel`**: swipe neighbour hints use the short label; waiting/failed copy still uses the full `label`.
- **`ShellBrand.shortWindowTitle`**: editor/demo shells show the short form in the mobile navbar.
- **Quiz data**: optional `short` (`ShortText`) on `Catalog` and `Quiz`, projected through `CatalogView`.

## Quiz site

| Place | Full (example) | Short (example) |
|---|---|---|
| Catalog navbar | Architecture and Technology Quizzes | A&T Quizzes |
| Catalog navbar (de) | Quizze zu Architektur und Technologie | A&T Quizze |
| Physics quiz page | Physical Understanding | Physics |
| Energy demand quiz | Energy Demand | Demand |
| How it works | How it works / So funktioniert's | How / So |
| Leaderboard | Leaderboard / Rangliste | Board / Liste |
| Settings | Settings / Einstellungen | Prefs / Einst. |

Document tab titles use the catalog short name (`Leaderboard · A&T Quizzes`).

## Play & demonstrator

- Play runtime catalog: `shortLabel` on panes with long names (e.g. Procedural 3D → Proc 3D); `playPaneBrand` sets `shortWindowTitle`.
- Demonstrator brands: `shortWindowTitle` such as `EmB · Agg` for `Entwerfen mit Bestand · Aggregator`; layered swipe hints derive the trailing segment.

## Tests

- `@semio-tech/quiz-react` navigation suite: 40/40 passed.
- `@semio-tech/quiz` read-views suite: 28/28 passed.
