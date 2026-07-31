# Introduction Body Hover Emphasizes Content

## Intent
Emphasize only the introduction body paragraph under the pointer — not the whole body, sibling paragraphs, chips, or checklist.

## Root cause (Aggregator)
Welcome step body is one string with `\n\n` between two visual paragraphs, rendered as a single `<p whitespace-pre-line>`. Hovering either paragraph emphasized the entire `<p>`.

## Change
- `splitIntroductionBodyParagraphs` splits body on blank lines
- `UIIntroduction` renders each chunk as `[data-slot="introduction-body-paragraph"]`
- CSS: `[data-slot="introduction-body-paragraph"]:hover` sets emphasized color
- Vitest covers split helper + two-paragraph markup + CSS contract

## Follow-ups
1. Dev: content only, not chips
2. Dev: only the paragraph where the cursor is
3. Dev: Aggregator still highlighted both — fixed by splitting blank-line paragraphs
