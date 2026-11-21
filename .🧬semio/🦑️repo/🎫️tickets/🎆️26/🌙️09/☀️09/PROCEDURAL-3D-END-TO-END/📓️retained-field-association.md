# Retained field association

The shared interpreter now associates a Field with its published control. Its renderer resolves the control's real address even when the author uses a key such as `origin.x.input`. The same association supplies inherited names, error text and descriptions in ordinary and staged controls. Explicit control names take precedence. Read-only and busy single-child fields render their label without an inactive target.

The native accessibility projection inherits the name of a field's sole control from its actual parent. Explicit native names and the designated control in a compound field retain their existing precedence.

## Verification

- The English and German regression tests first failed because the numeric input had no accessible name. Both pass with the retained interpreter. They cover input replacement, authored names, required and invalid state, description and error text, a labelled slider, and a read-only field.
- All 120 editable-control tests passed. The targeted Vitest run also passed the existing staged Form render and live control test. Forty-two additional Bun tests passed with 108 assertions, including AJV and independent Field markup checks.
- Chromium confirmed that clicking the visible label focuses the numeric control with a different key in the short Generate pane. The text, slider and toggle ownership checks still passed in both locales.
- The pinned compiler built the live WGPU accessibility projection module with cached dependencies. Its independent published-document harness passed the same fixture in both locales for inherited numeric and slider names and explicit override. The official native projection suite remains pending.
- An exploratory broad document-store run failed fixture paths that derive from the working directory, several default timeouts, and one paged continuation assertion. Its staged form test passed again when run directly. The goal's canonical linked and mounted product runs remain pending under the storage gate. The workspace reported 0.27 GiB free.

## Form publication capacity

The shared contract allocates 128 records per mounted UI body. The current Form and its field stack each consume one record. A body containing 64 scalar questions therefore needs at least 130 records. Vector groups, headings and media have variable per-field record and height costs. This needs a shared viewport that seats a bounded interval of whole fields, preserves form and keyboard semantics, and can re-request fields as their sizes or visibility change. The current tree window assumes uniform row extents within a section.
