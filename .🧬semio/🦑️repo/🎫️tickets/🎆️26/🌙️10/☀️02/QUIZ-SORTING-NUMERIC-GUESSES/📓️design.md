# Numeric guesses for sorting tasks

Ticket `2026/10/02/QUIZ-SORTING-NUMERIC-GUESSES`. Request: for every quiz question that sorts numerically, let the
learner add a numerical guess, show the unit automatically and reorder accordingly.

## Scope

Only `sorting` tasks sort numerically (today: `powers` and `energies` of the physics quiz, both logarithmic and
SI-prefixed). Matching assigns fixed value cards, classification has no numbers; neither changes.

## Contract (schema-first)

`SortingAnswer` gains optional `guesses: { <item id>: number }`, the guess in the quantity's **base unit** (never
SI-prefixed). Rules, enforced identically by the TypeScript and Rust twins and a Python reference:

1. keys name sheet items, values are finite numbers, positive on a logarithmic scale;
2. the guessed items appear in `order` in non-decreasing guess order (ties allowed, unguessed items unconstrained).

So an answer can never show a guess that contradicts its order. Drafts in the thinking room (`ThinkingAnswer`) may carry
`guesses` too: finite numbers, at most `THINKING_LIMIT` entries, no cross-field rule. Guesses never enter scoring.

## Interaction (react target, `↕️sorting`)

- Per item a text field `Guess for <item>`; placeholder `e.g. 2 kW` (unit shown from the first look).
- While typing, a preview `= 500 kg` shows what the text reads as in the unit scaled with the best SI prefix.
- Commit on Enter or leaving the field. After commit the field shows the formatted quantity; text left unchanged
  after that commit does not re-commit (a rounded display must not overwrite an exact guess). Escape reverts.
- Reorder rule `ordered(order, guesses)`: the guessed items sort by guess (stable) **among the places they occupy**;
  unguessed items stay where they are. Reordering on commit rather than per keystroke keeps focus and the row under
  the learner's hand stable; focus follows the field (Enter) or the control tabbed to (blur).
- Moving a guessed item by hand (buttons, drag) removes its guess, so guesses and order never disagree. Clearing the
  text removes the guess. An unreadable text stays flagged (`aria-invalid`, `role="alert"`) until fixed or emptied.
- A guess counts as an answer (the "keep this order" step is no longer needed once something is guessed).
- Results get a "Your guess" column when the learner guessed.

## Parsing (`📏️quantity`, `parseQuantity`)

Inverse of `formatQuantity`: number with optional exponent, optional SI prefix and unit (`2 kW`, `5 M`, `3,5 MWh`,
`1e3 kW`). Locale decimal mark; a lone `.`/`,` elsewhere is a decimal mark unless it groups three digits (locale group
mark between a one-to-three digit head and three digits); spaces/apostrophes group by three. Prefix case-sensitive
(`mW` ≠ `MW`), unit case-insensitive, `u` reads as `µ`, `K` as `k`. Value assembled as `Number("<digits>e<exp>")` so
`1.5 kW` is exactly 1500.

## Tests

- `🧫️fixtures/📐️quantity-formatting` `parses` vectors + `Intl.NumberFormat` round trips (third-party oracle).
- `🧪️tests/⌨️task-keyboard`: guess ordering, preview, focus, invalid text, hand move drops guess, unchanged text.
- `🧪️tests/🗳️crowd-answers`: thinking drafts with guesses, checked against ajv from the JSON schema.
- `🧪️tests/⚖️partial-credit-scoring`: guesses do not change a score.
- `🧪️tests/✅️answer-validation`: shared vectors from the Python reference, run by TypeScript and Rust.
