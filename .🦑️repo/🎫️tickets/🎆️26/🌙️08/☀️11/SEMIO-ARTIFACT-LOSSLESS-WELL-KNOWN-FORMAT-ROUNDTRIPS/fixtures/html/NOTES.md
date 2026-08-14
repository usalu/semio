# html/example.html — handcrafted HTML5 fixture

Handcrafted directly (no generator script needed — plain-text format), 1185 bytes, 26 lines.

## Structure present (all required elements)

- `<!DOCTYPE html>` — lowercase per HTML5 convention.
- `<html lang="en">` root.
- `<head>`: 2× `<meta>` (charset + viewport, both void elements), `<title>Semio Fixture: HTML5 Example</title>`, `<style>` block (2 CSS rules, `body` + `.highlight`).
- `<body>`: nested structure —
  - `<div id="content" class="wrapper">` wraps everything.
  - `<p>` containing a `<span class="highlight">` and a `<br>` void element right after inline text.
  - `<ul>` with 3× `<li>`, one containing `<a href="..." target="_blank">semio project</a>`, one containing `<img src="data:image/gif;base64,..." alt="...">` (void element, self-contained 1×1 pixel data URI — no external network fetch).
  - `<p disabled>` — a **valueless boolean attribute** (`disabled`) on a plain `<p>`, deliberately atypical usage to exercise the "attribute with no value" parser path.
  - HTML comment: `<!-- top-level structural fixture for the semio "html" format artifact (W0) -->`.
  - `<script>` inline block with one `console.log(...)` statement (prefixed `[DEBUG]` per repo convention for temp/log-style output, though here it's a permanent fixture comment, not a real debug log).

## Void elements present

`meta` (×2), `br`, `img` — all correctly self-closing (no closing tag, per HTML5 void-element rules), matching the plan's note: *"void-element set in encoder; well-formed-only ✳any (honest boundary)"*.

## Verification performed

Re-parsed with Python's stdlib `html.parser.HTMLParser` (independent of any code the eventual stdio codec will write) via a purpose-built verifier that:
1. Confirms the doctype string is exactly `<!doctype html>` (case-insensitive check on the leading bytes).
2. Tracks a tag stack: every non-void start tag pushes, every end tag must match the top of the stack (fails loudly on mismatch), and asserts the stack is **empty at EOF** (fully balanced, well-formed document).
3. Confirms both `br` and `img` were seen as void elements (not pushed onto the stack, no closing tag expected/present).
4. Confirms at least one HTML comment, at least one valueless attribute, and that `script`/`style`/`meta`/`title`/`div`/`p`/`ul`/`li`/`a` all appear.

Output: 19 tags seen, 4 void elements (`meta`,`meta`,`br`,`img`), 1 comment, 1 valueless attribute (`disabled` on `p`), empty stack at EOF.

→ **ALL HTML5 STRUCTURAL ASSERTIONS PASSED** (well-formed, balanced tags, void elements, comment, valueless attr, script+style blocks all present).
